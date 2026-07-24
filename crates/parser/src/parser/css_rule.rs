use super::{
    media::{parse_import, parse_media_list, parse_supports_condition},
    properties::parse_declaration,
    rules::*,
    selector::{parse_selector_list, parse_selector_list_with_recovery, parse_selector_string},
    stylesheet::{check_depth, recover_declaration, recover_rule, span_from},
    values::{collect_tokens, matches_ignore_case},
};
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum TopLevelState {
    Start,
    Layers,
    Imports,
    Namespaces,
    Body,
}

/// Parses a top-level or nested CSS rule list.
pub(super) fn parse_rule_list<'i, 't, 'ghost>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, CssRule<'i, 'ghost>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut rules = allocator.vec();
    let mut top_level_state = TopLevelState::Start;

    loop {
        let start = input.state();
        let css_token = match input.next() {
            Ok(css_token) => css_token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        let result = match css_token {
            ValueToken::AtKeyword(name) => {
                if depth > 0
                    && matches_ignore_case(
                        name,
                        &["import", "namespace", "charset", "custom-media"],
                    )
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
                    parse_at_rule(input, allocator, token, options, depth, &start, name, false)
                }
            }
            ValueToken::Cdo | ValueToken::Cdc | ValueToken::Semicolon => continue,
            _ => {
                input.reset(&start);
                parse_qualified_rule(input, allocator, token, options, depth, &start)
            }
        };

        match result {
            Ok(rule) => {
                if depth == 0 {
                    top_level_state = match &rule {
                        CssRule::Charset(_) => top_level_state,
                        CssRule::Import(_) => TopLevelState::Imports,
                        CssRule::Namespace(_) => TopLevelState::Namespaces,
                        CssRule::LayerStatement(_) if top_level_state <= TopLevelState::Layers => {
                            TopLevelState::Layers
                        }
                        _ => TopLevelState::Body,
                    };
                }
                rules.push(rule)
            }
            Err(_) if options.error_recovery => recover_rule(input),
            Err(error) => return Err(error),
        }
    }

    Ok(rules)
}

pub(super) fn parse_group_rule_body<'i, 't, 'ghost>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
    in_style_rule: bool,
) -> Result<Vec<'i, CssRule<'i, 'ghost>>, ParseError<'i, ParserError<'i>>> {
    if !in_style_rule {
        return parse_rule_list(input, allocator, token, options, depth);
    }

    let start = input.state();
    let (declarations, mut rules) = parse_style_contents(input, allocator, token, options, depth)?;
    if !declarations.is_empty() {
        rules.insert(
            0,
            CssRule::NestedDeclarations(allocator.boxed(rocketcss_ast::NestedDeclarationsRule {
                declarations: allocator.alloc_ghost(declarations),
                span: span_from(&start, input.position()),
            })),
        );
    }
    Ok(rules)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn parse_at_rule<'i, 't, 'ghost>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &ParserState,
    name: &'i str,
    in_style_rule: bool,
) -> Result<CssRule<'i, 'ghost>, ParseError<'i, ParserError<'i>>> {
    if in_style_rule
        && matches_ignore_case(
            name,
            &[
                "import",
                "namespace",
                "charset",
                "custom-media",
                "font-face",
                "font-feature-values",
                "font-palette-values",
                "counter-style",
                "keyframes",
                "-webkit-keyframes",
                "-moz-keyframes",
                "-o-keyframes",
                "-ms-keyframes",
                "page",
                "property",
                "position-try",
                "viewport",
                "-ms-viewport",
                "view-transition",
            ],
        )
    {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let prelude_start = input.position();
    let prelude = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| collect_tokens(input, allocator, depth + 1),
    )?;
    let prelude_end = input.position();
    let raw_prelude = input.slice(prelude_start..prelude_end).trim();

    enum Ending {
        None,
        Semicolon,
        Block,
    }

    let ending = match input.next() {
        Ok(ValueToken::Semicolon) => Ending::Semicolon,
        Ok(ValueToken::CurlyBracketBlock) => Ending::Block,
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => Ending::None,
        Ok(_) => return Err(input.new_custom_error(ParserError::InvalidAtRule(name))),
        Err(error) => return Err(error.into()),
    };

    let rule = if name.eq_ignore_ascii_case("import") {
        if matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        parse_import(raw_prelude, allocator, start, input.position())?
    } else if name.eq_ignore_ascii_case("media") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let query = parse_media_list(raw_prelude, allocator)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::Media(allocator.boxed(MediaRule {
            span: span_from(start, input.position()),
            query,
            rules,
        }))
    } else if name.eq_ignore_ascii_case("supports") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::Supports(allocator.boxed(SupportsRule {
            condition: allocator.boxed(parse_supports_condition(raw_prelude)),
            span: span_from(start, input.position()),
            rules,
        }))
    } else if name.eq_ignore_ascii_case("starting-style") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::StartingStyle(allocator.boxed(StartingStyleRule {
            span: span_from(start, input.position()),
            rules,
        }))
    } else if name.eq_ignore_ascii_case("font-face") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let properties = input.parse_nested_block(|input| {
            parse_font_face_contents(input, allocator, options, depth + 1)
        })?;
        CssRule::FontFace(allocator.boxed(rocketcss_ast::FontFaceRule {
            span: span_from(start, input.position()),
            properties,
        }))
    } else if name.eq_ignore_ascii_case("charset") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let encoding = parse_charset(raw_prelude, allocator)?;
        CssRule::Charset(allocator.boxed(CharsetRule {
            span: span_from(start, input.position()),
            encoding,
        }))
    } else if name.eq_ignore_ascii_case("namespace") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let (prefix, url) = parse_namespace(raw_prelude, allocator)?;
        CssRule::Namespace(allocator.boxed(NamespaceRule {
            span: span_from(start, input.position()),
            prefix,
            url,
        }))
    } else if name.eq_ignore_ascii_case("layer") {
        let mut names = parse_layer_names(raw_prelude, allocator)?;
        if matches!(ending, Ending::Block) {
            if names.len() > 1 {
                return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
            }
            let layer_name = names.pop();
            let rules = input.parse_nested_block(|input| {
                parse_group_rule_body(input, allocator, token, options, depth + 1, in_style_rule)
            })?;
            CssRule::LayerBlock(allocator.boxed(LayerBlockRule {
                span: span_from(start, input.position()),
                name: layer_name,
                rules,
            }))
        } else {
            if names.is_empty() {
                return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
            }
            CssRule::LayerStatement(allocator.boxed(LayerStatementRule {
                span: span_from(start, input.position()),
                names,
            }))
        }
    } else if name.eq_ignore_ascii_case("custom-media") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let (custom_name, query) = parse_custom_media(raw_prelude, allocator)?;
        CssRule::CustomMedia(allocator.boxed(rocketcss_ast::CustomMediaRule {
            span: span_from(start, input.position()),
            name: custom_name,
            query,
        }))
    } else if matches_ignore_case(
        name,
        &[
            "keyframes",
            "-webkit-keyframes",
            "-moz-keyframes",
            "-o-keyframes",
            "-ms-keyframes",
        ],
    ) {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let keyframes_name = parse_keyframes_name(raw_prelude, allocator)?;
        let keyframes = input.parse_nested_block(|input| {
            parse_keyframe_list(input, allocator, options, depth + 1)
        })?;
        CssRule::Keyframes(allocator.boxed(KeyframesRule {
            keyframes,
            span: span_from(start, input.position()),
            name: allocator.boxed(keyframes_name),
            vendor_prefix: at_rule_vendor_prefix(name),
        }))
    } else if name.eq_ignore_ascii_case("counter-style") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let counter_name = parse_single_ident(raw_prelude, allocator)?;
        let declarations = input.parse_nested_block(|input| {
            parse_declaration_block(input, allocator, options, depth + 1)
        })?;
        CssRule::CounterStyle(allocator.boxed(rocketcss_ast::CounterStyleRule {
            declarations: allocator.alloc_ghost(declarations),
            span: span_from(start, input.position()),
            name: counter_name,
        }))
    } else if matches_ignore_case(name, &["viewport", "-ms-viewport"]) {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let declarations = input.parse_nested_block(|input| {
            parse_declaration_block(input, allocator, options, depth + 1)
        })?;
        CssRule::Viewport(allocator.boxed(rocketcss_ast::ViewportRule {
            declarations: allocator.alloc_ghost(declarations),
            span: span_from(start, input.position()),
            vendor_prefix: at_rule_vendor_prefix(name),
        }))
    } else if name.eq_ignore_ascii_case("position-try") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let position_name = parse_single_ident(raw_prelude, allocator)?;
        if !position_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let declarations = input.parse_nested_block(|input| {
            parse_declaration_block(input, allocator, options, depth + 1)
        })?;
        CssRule::PositionTry(allocator.boxed(rocketcss_ast::PositionTryRule {
            span: span_from(start, input.position()),
            name: position_name,
            declarations: allocator.alloc_ghost(declarations),
        }))
    } else if name.eq_ignore_ascii_case("-moz-document") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        validate_moz_document_prelude(raw_prelude, allocator)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::MozDocument(allocator.boxed(rocketcss_ast::MozDocumentRule {
            span: span_from(start, input.position()),
            rules,
        }))
    } else if name.eq_ignore_ascii_case("container") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let (container_name, condition) = parse_container_prelude(raw_prelude, allocator)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::Container(allocator.boxed(rocketcss_ast::ContainerRule {
            condition,
            span: span_from(start, input.position()),
            name: container_name,
            rules,
        }))
    } else if name.eq_ignore_ascii_case("scope") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let (scope_start, scope_end) = parse_scope_prelude(raw_prelude, allocator, depth + 1)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, allocator, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::Scope(allocator.boxed(ScopeRule {
            span: span_from(start, input.position()),
            rules,
            scope_end,
            scope_start,
        }))
    } else if name.eq_ignore_ascii_case("page") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let selectors = parse_page_selectors(raw_prelude, allocator)?;
        let (declarations, rules) = input
            .parse_nested_block(|input| parse_page_body(input, allocator, options, depth + 1))?;
        CssRule::Page(allocator.boxed(PageRule {
            declarations: allocator.alloc_ghost(declarations),
            span: span_from(start, input.position()),
            rules,
            selectors,
        }))
    } else if name.eq_ignore_ascii_case("font-palette-values") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let palette_name = parse_single_ident(raw_prelude, allocator)?;
        if !palette_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let properties = input.parse_nested_block(|input| {
            parse_font_palette_contents(input, allocator, options, depth + 1)
        })?;
        CssRule::FontPaletteValues(allocator.boxed(FontPaletteValuesRule {
            span: span_from(start, input.position()),
            name: palette_name,
            properties,
        }))
    } else if name.eq_ignore_ascii_case("font-feature-values") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let family_names = parse_family_names(raw_prelude, allocator)?;
        let rules = input.parse_nested_block(|input| {
            parse_font_feature_subrules(input, allocator, options, depth + 1)
        })?;
        CssRule::FontFeatureValues(allocator.boxed(FontFeatureValuesRule {
            span: span_from(start, input.position()),
            name: family_names,
            rules,
        }))
    } else if name.eq_ignore_ascii_case("property") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let property_name = parse_single_ident(raw_prelude, allocator)?;
        if !property_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let mut property = input.parse_nested_block(|input| {
            parse_property_rule(input, allocator, options, depth + 1, property_name)
        })?;
        property.span = span_from(start, input.position());
        CssRule::Property(allocator.boxed(property))
    } else if name.eq_ignore_ascii_case("view-transition") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let properties = input.parse_nested_block(|input| {
            parse_view_transition_contents(input, allocator, options, depth + 1)
        })?;
        CssRule::ViewTransition(allocator.boxed(ViewTransitionRule {
            span: span_from(start, input.position()),
            properties,
        }))
    } else if name.eq_ignore_ascii_case("nest") && in_style_rule {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let selectors = parse_selector_string(raw_prelude, allocator, depth + 1)?;
        let (declarations, rules) = input.parse_nested_block(|input| {
            parse_style_contents(input, allocator, token, options, depth + 1)
        })?;
        let span = span_from(start, input.position());
        CssRule::Nesting(allocator.boxed(NestingRule {
            span,
            style: allocator.alloc_ghost(StyleRule::new(
                allocator.alloc_ghost(declarations),
                span,
                rules,
                selectors,
                VendorPrefix::NONE,
            )),
        }))
    } else {
        let block = if matches!(ending, Ending::Block) {
            Some(input.parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?)
        } else {
            None
        };
        CssRule::Unknown(allocator.boxed(UnknownAtRule {
            block,
            span: span_from(start, input.position()),
            name,
            prelude,
        }))
    };

    Ok(rule)
}

pub(super) fn parse_qualified_rule<'i, 't, 'ghost>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &ParserState,
) -> Result<CssRule<'i, 'ghost>, ParseError<'i, ParserError<'i>>> {
    let selectors = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
        if options.error_recovery {
            parse_selector_list_with_recovery(input, allocator, depth + 1)
        } else {
            parse_selector_list(input, allocator, depth + 1)
        }
    })?;
    input.expect_curly_bracket_block()?;
    let (declarations, rules) = input.parse_nested_block(|input| {
        parse_style_contents(input, allocator, token, options, depth + 1)
    })?;

    Ok(CssRule::Style(allocator.alloc_ghost(StyleRule::new(
        allocator.alloc_ghost(declarations),
        span_from(start, input.position()),
        rules,
        selectors,
        VendorPrefix::NONE,
    ))))
}

type StyleContents<'i, 'ghost> = (DeclarationBlock<'i>, Vec<'i, CssRule<'i, 'ghost>>);

pub(super) fn parse_style_contents<'i, 't, 'ghost>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<StyleContents<'i, 'ghost>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut declarations = DeclarationBlock::new(allocator);
    let mut rules = allocator.vec();

    loop {
        let start = input.state();
        let css_token = match input.next() {
            Ok(css_token) => css_token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        let result = match css_token {
            ValueToken::Semicolon => continue,
            ValueToken::AtKeyword(name) => {
                parse_at_rule(input, allocator, token, options, depth, &start, name, true)
                    .map(|rule| Some((false, rule)))
            }
            ValueToken::Ident(name) => {
                let has_colon = input.try_parse(Parser::expect_colon).is_ok();
                let next_delimiter = next_rule_body_delimiter(input);
                if has_colon
                    && (name.starts_with("--")
                        || next_delimiter != Some(RuleBodyDelimiter::CurlyBracket))
                {
                    parse_declaration(input, allocator, name, depth).map(
                        |(declaration, important)| {
                            if rules.is_empty() {
                                declarations.push(declaration, important);
                            } else if let Some(CssRule::NestedDeclarations(rule)) = rules.last_mut()
                            {
                                rule.declarations
                                    .as_ref()
                                    .borrow_mut(token)
                                    .push(declaration, important);
                            } else {
                                let mut nested = DeclarationBlock::new(allocator);
                                nested.push(declaration, important);
                                rules.push(CssRule::NestedDeclarations(allocator.boxed(
                                    NestedDeclarationsRule {
                                        declarations: allocator.alloc_ghost(nested),
                                        span: DUMMY_SP,
                                    },
                                )));
                            }
                            None
                        },
                    )
                } else if !has_colon && next_delimiter != Some(RuleBodyDelimiter::CurlyBracket) {
                    Err(input.new_custom_error(ParserError::InvalidDeclaration))
                } else {
                    input.reset(&start);
                    parse_qualified_rule(input, allocator, token, options, depth, &start)
                        .map(|rule| Some((true, rule)))
                }
            }
            _ => {
                input.reset(&start);
                parse_qualified_rule(input, allocator, token, options, depth, &start)
                    .map(|rule| Some((true, rule)))
            }
        };

        match result {
            Ok(Some((_, rule))) => rules.push(rule),
            Ok(None) => {}
            Err(_) if options.error_recovery => {
                input.reset(&start);
                recover_declaration(input);
            }
            Err(error) => return Err(error),
        }
    }

    Ok((declarations, rules))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RuleBodyDelimiter {
    Semicolon,
    CurlyBracket,
}

fn next_rule_body_delimiter(input: &mut Parser<'_, '_>) -> Option<RuleBodyDelimiter> {
    let state = input.state();
    let _: Result<(), ParseError<'_, ()>> = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| {
            while input.next().is_ok() {}
            Ok(())
        },
    );
    let delimiter = match input.next() {
        Ok(ValueToken::Semicolon) => Some(RuleBodyDelimiter::Semicolon),
        Ok(ValueToken::CurlyBracketBlock) => Some(RuleBodyDelimiter::CurlyBracket),
        _ => None,
    };
    input.reset(&state);
    delimiter
}
