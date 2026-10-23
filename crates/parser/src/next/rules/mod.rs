//! Rule tree construction for the flattened AST.

use super::{
    css_rule::{RuleBodyDelimiter, scan_rule_body},
    prelude::*,
    properties::parse_declaration_with_css_wide_hint,
    selector::{parse_selector_list, parse_selector_list_with_recovery},
};

mod at_rule;
mod descriptors;

pub(crate) use descriptors::{recover_declaration, recover_rule};

pub(crate) use at_rule::parse_group_at_rule;

#[derive(Clone, Copy)]
pub(crate) enum BodyItem<'i> {
    Declaration(DeclarationEntry<'i>),
    Rule(CssRule<'i>),
}

impl<'ast> Compiler<'ast> {
    /// Parses a stylesheet directly into the flattened rule tree.
    pub(crate) fn parse_compilation(
        &mut self,
        source: &'ast str,
        options: ParserOptions<'ast>,
    ) -> Result<rocketcss_ast_next::Compilation<'ast>, Error<'ast>> {
        self.cursor = crate::parser::ParserCursor::new(source);
        self.replay.reset_for_new_source();
        self.ast = Ast::with_capacity_in(self.allocator, source.len() / 4 + 1);
        let mut license_comments = Vec::new();
        let mut state = self.state();
        while let Ok(token) = self.next_including_whitespace_and_comments() {
            match token {
                OldToken::WhiteSpace(_) => {}
                OldToken::Comment(comment) if comment.starts_with('!') => {
                    license_comments.push(*comment);
                }
                _ => break,
            }
            state = self.state();
        }
        self.reset(&state);

        let rules = parse_rule_list(self, &options, 0)
            .map_err(|error| into_error(error, options.filename))?;
        let license_comments = self.ast().alloc_slice(&license_comments);
        let style_sheet = self.ast_mut().alloc_style_sheet(
            StyleSheetData {
                rules,
                license_comments,
            },
            DUMMY_SP,
        );
        let allocator = self.allocator;
        let ast = std::mem::replace(&mut self.ast, Ast::new_in(allocator));
        Ok(rocketcss_ast_next::Compilation::new(ast, style_sheet))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TopLevelState {
    Start,
    Layers,
    Imports,
    Namespaces,
    Body,
}

pub(crate) fn parse_rule_list<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<&'i [CssRule<'i>], ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut rules = Vec::new();
    let mut top_level_state = TopLevelState::Start;
    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        let result = match token {
            OldToken::Cdo | OldToken::Cdc | OldToken::Semicolon => continue,
            OldToken::AtKeyword(name) => {
                if depth > 0
                    && (name.eq_ignore_ascii_case("import")
                        || name.eq_ignore_ascii_case("namespace")
                        || name.eq_ignore_ascii_case("charset")
                        || name.eq_ignore_ascii_case("custom-media"))
                {
                    Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
                } else if depth == 0
                    && name.eq_ignore_ascii_case("import")
                    && top_level_state > TopLevelState::Imports
                {
                    Err(input.new_custom_error(ParserError::UnexpectedImportRule))
                } else if depth == 0
                    && name.eq_ignore_ascii_case("namespace")
                    && top_level_state > TopLevelState::Namespaces
                {
                    Err(input.new_custom_error(ParserError::UnexpectedNamespaceRule))
                } else {
                    parse_group_at_rule(input, options, depth, &start, name, false)
                }
            }
            _ => {
                input.reset(&start);
                parse_style_rule(input, options, depth, &start)
            }
        };
        match result {
            Ok(rule) => {
                if depth == 0 {
                    top_level_state = match rule.payload(input.ast()).data(input.ast()) {
                        CssRulePayloadData::Charset(_) => top_level_state,
                        CssRulePayloadData::Import(_) => TopLevelState::Imports,
                        CssRulePayloadData::Namespace(_) => TopLevelState::Namespaces,
                        CssRulePayloadData::LayerStatement(_)
                            if top_level_state <= TopLevelState::Layers =>
                        {
                            TopLevelState::Layers
                        }
                        _ => TopLevelState::Body,
                    };
                }
                rules.push(rule);
            }
            Err(_) if options.error_recovery => recover_rule(input),
            Err(error) => return Err(error),
        }
    }
    Ok(input.ast().alloc_slice(&rules))
}

pub(crate) fn parse_style_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let selectors = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
        if options.error_recovery {
            parse_selector_list_with_recovery(input, depth + 1)
        } else {
            parse_selector_list(input, depth + 1)
        }
    })?;
    input.expect_curly_bracket_block()?;
    let items =
        input.parse_nested_block(|input| parse_mixed_style_contents(input, options, depth + 1))?;
    let mut owner_declarations = Vec::new();
    let children = build_children(input, items, Some(&mut owner_declarations));
    let declarations = alloc_declaration_block(input, owner_declarations);
    let span = span_from(start, input.position());
    let style = input.ast_mut().alloc_style_rule(
        StyleRuleData {
            selectors,
            vendor_prefix: VendorPrefix::NONE,
        },
        span,
    );
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Style(style),
        Some(declarations),
        children,
        span,
    ))
}

pub(crate) fn parse_mixed_style_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<BodyItem<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut items = Vec::new();

    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        let result = match token {
            OldToken::Semicolon => continue,
            OldToken::AtKeyword(name) => {
                parse_group_at_rule(input, options, depth, &start, name, true)
                    .map(|rule| items.push(BodyItem::Rule(rule)))
            }
            OldToken::Ident(name) => {
                let has_colon = input.try_parse(Compiler::expect_colon).is_ok();
                let scan = scan_rule_body(input, !name.starts_with("--"));
                if has_colon
                    && (name.starts_with("--")
                        || scan.delimiter != Some(RuleBodyDelimiter::CurlyBracket))
                {
                    parse_declaration_with_css_wide_hint(input, name, depth, scan.css_wide_hint())
                        .map(|(declaration, important)| {
                            let entry = alloc_declaration_entry(
                                input,
                                DeclarationPayloadData::Property(declaration),
                                important,
                            );
                            items.push(BodyItem::Declaration(entry));
                        })
                } else if !has_colon && scan.delimiter != Some(RuleBodyDelimiter::CurlyBracket) {
                    Err(input.new_custom_error(ParserError::InvalidDeclaration))
                } else {
                    input.reset(&start);
                    parse_style_rule(input, options, depth, &start)
                        .map(|rule| items.push(BodyItem::Rule(rule)))
                }
            }
            _ => {
                input.reset(&start);
                parse_style_rule(input, options, depth, &start)
                    .map(|rule| items.push(BodyItem::Rule(rule)))
            }
        };

        match result {
            Ok(()) => {}
            Err(_) if options.error_recovery => {
                input.reset(&start);
                recover_declaration(input);
            }
            Err(error) => return Err(error),
        }
    }
    Ok(items)
}

pub(crate) fn build_children<'i>(
    input: &mut Compiler<'i>,
    items: Vec<BodyItem<'i>>,
    mut owner: Option<&mut Vec<DeclarationEntry<'i>>>,
) -> Vec<CssRule<'i>> {
    let mut children = Vec::new();
    let mut current: Option<Vec<DeclarationEntry<'i>>> = None;
    let mut is_first = owner.is_some();

    for item in items {
        match item {
            BodyItem::Declaration(declaration) => {
                current.get_or_insert_with(Vec::new).push(declaration);
            }
            BodyItem::Rule(rule) => {
                if let Some(declarations) = current.take() {
                    if is_first {
                        if let Some(owner) = owner.as_mut() {
                            owner.extend(declarations);
                        }
                    } else if !declarations.is_empty() {
                        children.push(alloc_nested_declarations_rule(input, declarations));
                    }
                }
                is_first = false;
                children.push(rule);
            }
        }
    }

    if let Some(declarations) = current.take() {
        if is_first {
            if let Some(owner) = owner.as_mut() {
                owner.extend(declarations);
            }
        } else if !declarations.is_empty() {
            children.push(alloc_nested_declarations_rule(input, declarations));
        }
    }
    children
}

pub(crate) fn alloc_declaration_entry<'i>(
    input: &mut Compiler<'i>,
    payload: DeclarationPayloadData<'i>,
    important: bool,
) -> DeclarationEntry<'i> {
    let payload = with_token_span!(input, |span| input
        .ast_mut()
        .alloc_declaration_payload(payload, span));
    DeclarationEntry { payload, important }
}

pub(crate) fn alloc_declaration_block<'i>(
    input: &mut Compiler<'i>,
    entries: Vec<DeclarationEntry<'i>>,
) -> DeclarationBlock<'i> {
    let mut declarations = Vec::with_capacity(entries.len());
    let mut important = Vec::with_capacity(entries.len());
    for entry in entries {
        declarations.push(entry.payload);
        important.push(entry.important);
    }
    let declarations = input.ast().alloc_slice(&declarations);
    let important = input.ast().alloc_slice(&important);
    with_token_span!(input, |span| input.ast_mut().alloc_declaration_block(
        DeclarationBlockData {
            declarations,
            important
        },
        span,
    ))
}

fn alloc_nested_declarations_rule<'i>(
    input: &mut Compiler<'i>,
    declarations: Vec<DeclarationEntry<'i>>,
) -> CssRule<'i> {
    let declarations = alloc_declaration_block(input, declarations);
    let payload = with_token_span!(input, |span| input
        .ast_mut()
        .alloc_nested_declarations_rule(NestedDeclarationsRuleData {}, span));
    alloc_rule(
        input,
        CssRulePayloadData::NestedDeclarations(payload),
        Some(declarations),
        Vec::new(),
        token_span(input),
    )
}

pub(crate) fn alloc_rule<'i>(
    input: &mut Compiler<'i>,
    payload: CssRulePayloadData<'i>,
    declarations: Option<DeclarationBlock<'i>>,
    children: Vec<CssRule<'i>>,
    span: Span,
) -> CssRule<'i> {
    let payload = input.ast_mut().alloc_css_rule_payload(payload, span);
    let children = input.ast().alloc_slice(&children);
    input.ast_mut().alloc_css_rule(
        CssRuleData {
            payload,
            declarations,
            children,
        },
        span,
    )
}

pub(crate) fn parse_group_rule_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    style_context: bool,
) -> Result<Vec<CssRule<'i>>, ParseError<'i, ParserError<'i>>> {
    input.parse_nested_block(|input| {
        if style_context {
            let items = parse_mixed_style_contents(input, options, depth + 1)?;
            Ok(build_children(input, items, None))
        } else {
            Ok(parse_rule_list(input, options, depth + 1)?.to_vec())
        }
    })
}

pub(crate) fn into_error<'i>(
    error: ParseError<'i, ParserError<'i>>,
    filename: &'i str,
) -> Error<'i> {
    let kind = match error.kind {
        crate::ParseErrorKind::Custom(error) => error,
        crate::ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(token)) => {
            ParserError::UnexpectedToken(token)
        }
        crate::ParseErrorKind::Basic(BasicParseErrorKind::AtRuleInvalid(name)) => {
            ParserError::InvalidAtRule(name)
        }
        crate::ParseErrorKind::Basic(_) => ParserError::InvalidRule,
    };
    Error {
        kind,
        filename,
        location: error.location,
    }
}
