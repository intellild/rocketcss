use super::{
    media::{parse_import, parse_media_list, parse_supports_condition},
    properties::{CssWideValueHint, parse_declaration_with_css_wide_hint},
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
pub(super) fn parse_rule_list<'i, 'ghost>(
    input: &mut Compiler<'i>,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<RuleListId, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let rules = input.begin_rule_list();
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
                let rule_id = input.reserve_rule(rules);
                let result = if depth > 0
                    && matches_ignore_case(
                        &name,
                        &["import", "namespace", "charset", "custom-media"],
                    ) {
                    Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())))
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
                    input.with_current_rule(rule_id, |input| {
                        parse_at_rule(input, token, options, depth, &start, name, false)
                    })
                };
                Some((rule_id, result))
            }
            ValueToken::Cdo | ValueToken::Cdc | ValueToken::Semicolon => None,
            _ => {
                input.reset(&start);
                let rule_id = input.reserve_rule(rules);
                let result = input.with_current_rule(rule_id, |input| {
                    parse_qualified_rule(input, token, options, depth, &start)
                });
                Some((rule_id, result))
            }
        };

        let Some((rule_id, result)) = result else {
            continue;
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
                input.finish_rule(rule_id, rule);
            }
            Err(_) if options.error_recovery => {
                input.finish_rule(rule_id, CssRule::Custom(DefaultAtRule));
                recover_rule(input)
            }
            Err(error) => return Err(error),
        }
    }

    Ok(rules)
}

pub(super) fn parse_group_rule_body<'i, 'ghost>(
    input: &mut Compiler<'i>,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
    in_style_rule: bool,
) -> Result<RuleListId, ParseError<'i, ParserError<'i>>> {
    if !in_style_rule {
        return parse_rule_list(input, token, options, depth);
    }

    let start = input.state();
    let (declarations, rules) = parse_style_contents(input, token, options, depth, true)?;
    if !input.declaration_block_is_empty(declarations) {
        let span = span_from(&start, input.position());
        let leading_id = input.first_rule(rules).expect("reserved leading rule");
        let leading = input.rule_mut(leading_id);
        *leading = CssRule::NestedDeclarations(rocketcss_ast::NestedDeclarationsRule {
            declarations,
            span,
        });
    }
    Ok(rules)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn parse_at_rule<'i, 'ghost>(
    input: &mut Compiler<'i>,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &ParserState,
    name: Atom<'i>,
    in_style_rule: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    if in_style_rule
        && matches_ignore_case(
            &name,
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
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
    }
    let prelude_start = input.position();
    let prelude = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| collect_tokens(input, depth + 1),
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
        Ok(_) => {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        Err(error) => return Err(error.into()),
    };

    let rule = if name.eq_ignore_ascii_case("import") {
        if matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        parse_import(input, raw_prelude, start, input.position())?
    } else if name.eq_ignore_ascii_case("media") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let query = parse_media_list(input, raw_prelude)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::Media(MediaRule {
            span: span_from(start, input.position()),
            query,
            rules,
        })
    } else if name.eq_ignore_ascii_case("supports") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::Supports(SupportsRule {
            condition: std::boxed::Box::new(parse_supports_condition(raw_prelude)),
            span: span_from(start, input.position()),
            rules,
        })
    } else if name.eq_ignore_ascii_case("starting-style") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::StartingStyle(StartingStyleRule::new(
            span_from(start, input.position()),
            rules,
        ))
    } else if name.eq_ignore_ascii_case("font-face") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let properties = input
            .parse_nested_block(|input| parse_font_face_contents(input, options, depth + 1))?;
        CssRule::FontFace(rocketcss_ast::FontFaceRule {
            span: span_from(start, input.position()),
            properties,
        })
    } else if name.eq_ignore_ascii_case("charset") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let encoding = parse_charset(input, raw_prelude)?;
        CssRule::Charset(CharsetRule {
            span: span_from(start, input.position()),
            encoding,
        })
    } else if name.eq_ignore_ascii_case("namespace") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let (prefix, url) = parse_namespace(input, raw_prelude)?;
        CssRule::Namespace(NamespaceRule {
            span: span_from(start, input.position()),
            prefix,
            url,
        })
    } else if name.eq_ignore_ascii_case("layer") {
        let mut names = parse_layer_names(input, raw_prelude)?;
        if matches!(ending, Ending::Block) {
            if names.len() > 1 {
                return Err(
                    input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into()))
                );
            }
            let layer_name = names.pop();
            let rules = input.parse_nested_block(|input| {
                parse_group_rule_body(input, token, options, depth + 1, in_style_rule)
            })?;
            CssRule::LayerBlock(LayerBlockRule {
                span: span_from(start, input.position()),
                name: layer_name,
                rules,
            })
        } else {
            if names.is_empty() {
                return Err(
                    input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into()))
                );
            }
            CssRule::LayerStatement(LayerStatementRule {
                span: span_from(start, input.position()),
                names,
            })
        }
    } else if name.eq_ignore_ascii_case("custom-media") {
        if !matches!(ending, Ending::Semicolon | Ending::None) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let (custom_name, query) = parse_custom_media(input, raw_prelude)?;
        CssRule::CustomMedia(rocketcss_ast::CustomMediaRule {
            span: span_from(start, input.position()),
            name: custom_name,
            query,
        })
    } else if matches_ignore_case(
        &name,
        &[
            "keyframes",
            "-webkit-keyframes",
            "-moz-keyframes",
            "-o-keyframes",
            "-ms-keyframes",
        ],
    ) {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let keyframes_name = parse_keyframes_name(input, raw_prelude)?;
        let keyframes =
            input.parse_nested_block(|input| parse_keyframe_list(input, options, depth + 1))?;
        CssRule::Keyframes(KeyframesRule {
            keyframes,
            span: span_from(start, input.position()),
            name: std::boxed::Box::new(keyframes_name),
            vendor_prefix: at_rule_vendor_prefix(&name),
        })
    } else if name.eq_ignore_ascii_case("counter-style") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let counter_name = parse_single_ident(input, raw_prelude)?;
        let declarations =
            input.parse_nested_block(|input| parse_declaration_block(input, options, depth + 1))?;
        CssRule::CounterStyle(rocketcss_ast::CounterStyleRule {
            declarations,
            span: span_from(start, input.position()),
            name: counter_name,
        })
    } else if matches_ignore_case(&name, &["viewport", "-ms-viewport"]) {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let declarations =
            input.parse_nested_block(|input| parse_declaration_block(input, options, depth + 1))?;
        CssRule::Viewport(rocketcss_ast::ViewportRule {
            declarations,
            span: span_from(start, input.position()),
            vendor_prefix: at_rule_vendor_prefix(&name),
        })
    } else if name.eq_ignore_ascii_case("position-try") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let position_name = parse_single_ident(input, raw_prelude)?;
        if !position_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let declarations =
            input.parse_nested_block(|input| parse_declaration_block(input, options, depth + 1))?;
        CssRule::PositionTry(rocketcss_ast::PositionTryRule {
            span: span_from(start, input.position()),
            name: position_name,
            declarations,
        })
    } else if name.eq_ignore_ascii_case("-moz-document") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        validate_moz_document_prelude(input, raw_prelude)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::MozDocument(rocketcss_ast::MozDocumentRule::new(
            span_from(start, input.position()),
            rules,
        ))
    } else if name.eq_ignore_ascii_case("container") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let (container_name, condition) = parse_container_prelude(input, raw_prelude)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::Container(rocketcss_ast::ContainerRule {
            condition,
            span: span_from(start, input.position()),
            name: container_name,
            rules,
        })
    } else if name.eq_ignore_ascii_case("scope") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let (scope_start, scope_end) = parse_scope_prelude(input, raw_prelude, depth + 1)?;
        let rules = input.parse_nested_block(|input| {
            parse_group_rule_body(input, token, options, depth + 1, in_style_rule)
        })?;
        CssRule::Scope(ScopeRule {
            span: span_from(start, input.position()),
            rules,
            scope_end,
            scope_start,
        })
    } else if name.eq_ignore_ascii_case("page") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let selectors = parse_page_selectors(input, raw_prelude)?;
        let (declarations, rules) =
            input.parse_nested_block(|input| parse_page_body(input, options, depth + 1))?;
        CssRule::Page(PageRule {
            declarations,
            span: span_from(start, input.position()),
            rules,
            selectors,
        })
    } else if name.eq_ignore_ascii_case("font-palette-values") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let palette_name = parse_single_ident(input, raw_prelude)?;
        if !palette_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let properties = input
            .parse_nested_block(|input| parse_font_palette_contents(input, options, depth + 1))?;
        CssRule::FontPaletteValues(FontPaletteValuesRule {
            span: span_from(start, input.position()),
            name: palette_name,
            properties,
        })
    } else if name.eq_ignore_ascii_case("font-feature-values") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let family_names = parse_family_names(input, raw_prelude)?;
        let rules = input
            .parse_nested_block(|input| parse_font_feature_subrules(input, options, depth + 1))?;
        CssRule::FontFeatureValues(FontFeatureValuesRule {
            span: span_from(start, input.position()),
            name: family_names,
            rules,
        })
    } else if name.eq_ignore_ascii_case("property") {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let property_name = parse_single_ident(input, raw_prelude)?;
        if !property_name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let mut property = input.parse_nested_block(|input| {
            parse_property_rule(input, options, depth + 1, property_name)
        })?;
        property.span = span_from(start, input.position());
        CssRule::Property(property)
    } else if name.eq_ignore_ascii_case("view-transition") {
        if !raw_prelude.is_empty() || !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let properties = input.parse_nested_block(|input| {
            parse_view_transition_contents(input, options, depth + 1)
        })?;
        CssRule::ViewTransition(ViewTransitionRule {
            span: span_from(start, input.position()),
            properties,
        })
    } else if name.eq_ignore_ascii_case("nest") && in_style_rule {
        if !matches!(ending, Ending::Block) {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into())));
        }
        let selectors = parse_selector_string(input, raw_prelude, depth + 1)?;
        let selectors = input.push_selector_list(selectors);
        let (declarations, rules) = input.parse_nested_block(|input| {
            parse_style_contents(input, token, options, depth + 1, false)
        })?;
        let span = span_from(start, input.position());
        CssRule::Nesting(NestingRule {
            span,
            style: StyleRule::new(declarations, span, rules, selectors, VendorPrefix::NONE),
        })
    } else {
        let block = if matches!(ending, Ending::Block) {
            Some(input.parse_nested_block(|input| collect_tokens(input, depth + 1))?)
        } else {
            None
        };
        CssRule::Unknown(UnknownAtRule {
            block,
            span: span_from(start, input.position()),
            name,
            prelude,
        })
    };

    Ok(rule)
}

pub(super) fn parse_qualified_rule<'i, 'ghost>(
    input: &mut Compiler<'i>,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &ParserState,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let selectors = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
        if options.error_recovery {
            parse_selector_list_with_recovery(input, depth + 1)
        } else {
            parse_selector_list(input, depth + 1)
        }
    })?;
    let selectors = input.push_selector_list(selectors);
    input.expect_curly_bracket_block()?;
    let (declarations, rules) = input.parse_nested_block(|input| {
        parse_style_contents(input, token, options, depth + 1, false)
    })?;

    Ok(CssRule::Style(StyleRule::new(
        declarations,
        span_from(start, input.position()),
        rules,
        selectors,
        VendorPrefix::NONE,
    )))
}

type StyleContents = (DeclarationBlockId, RuleListId);

pub(super) fn parse_style_contents<'i, 'ghost>(
    input: &mut Compiler<'i>,
    token: &mut GhostToken<'ghost>,
    options: &ParserOptions<'i>,
    depth: usize,
    leading_as_nested_rule: bool,
) -> Result<StyleContents, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let declarations = input.begin_declaration_block();
    let rules = input.begin_rule_list();
    if leading_as_nested_rule {
        let leading = input.reserve_rule(rules);
        input.finish_rule(leading, CssRule::Custom(DefaultAtRule));
    }
    let mut encountered_nested_rule = false;
    let mut trailing_declarations = None;

    loop {
        let start = input.state();
        let css_token = match input.next() {
            Ok(css_token) => css_token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        let mut reserved_rule = None;
        let result = match css_token {
            ValueToken::Semicolon => continue,
            ValueToken::AtKeyword(name) => {
                let rule_id = input.reserve_rule(rules);
                reserved_rule = Some(rule_id);
                input
                    .with_current_rule(rule_id, |input| {
                        parse_at_rule(input, token, options, depth, &start, name, true)
                    })
                    .map(|rule| Some((rule_id, rule)))
            }
            ValueToken::Ident(name) => {
                let has_colon = input.try_parse(Compiler::expect_colon).is_ok();
                let scan = scan_rule_body(input, !name.starts_with("--"));
                if has_colon
                    && (name.starts_with("--")
                        || scan.delimiter != Some(RuleBodyDelimiter::CurlyBracket))
                {
                    parse_declaration_with_css_wide_hint(input, name, depth, scan.css_wide_hint())
                        .map(|(declaration, important)| {
                            if !encountered_nested_rule {
                                input.push_declaration(declarations, declaration, important);
                            } else if let Some(nested) = trailing_declarations {
                                input.push_declaration(nested, declaration, important);
                            } else {
                                let nested = input.begin_declaration_block();
                                input.push_declaration(nested, declaration, important);
                                let rule_id = input.reserve_rule(rules);
                                input.finish_rule(
                                    rule_id,
                                    CssRule::NestedDeclarations(NestedDeclarationsRule {
                                        declarations: nested,
                                        span: DUMMY_SP,
                                    }),
                                );
                                trailing_declarations = Some(nested);
                            }
                            None
                        })
                } else if !has_colon && scan.delimiter != Some(RuleBodyDelimiter::CurlyBracket) {
                    Err(input.new_custom_error(ParserError::InvalidDeclaration))
                } else {
                    input.reset(&start);
                    let rule_id = input.reserve_rule(rules);
                    reserved_rule = Some(rule_id);
                    input
                        .with_current_rule(rule_id, |input| {
                            parse_qualified_rule(input, token, options, depth, &start)
                        })
                        .map(|rule| Some((rule_id, rule)))
                }
            }
            _ => {
                input.reset(&start);
                let rule_id = input.reserve_rule(rules);
                reserved_rule = Some(rule_id);
                input
                    .with_current_rule(rule_id, |input| {
                        parse_qualified_rule(input, token, options, depth, &start)
                    })
                    .map(|rule| Some((rule_id, rule)))
            }
        };

        match result {
            Ok(Some((rule_id, rule))) => {
                input.finish_rule(rule_id, rule);
                encountered_nested_rule = true;
                trailing_declarations = None;
            }
            Ok(None) => {}
            Err(_) if options.error_recovery => {
                if let Some(rule_id) = reserved_rule {
                    input.finish_rule(rule_id, CssRule::Custom(DefaultAtRule));
                    encountered_nested_rule = true;
                    trailing_declarations = None;
                }
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

struct RuleBodyScan<'i> {
    delimiter: Option<RuleBodyDelimiter>,
    css_wide_candidate: Option<Atom<'i>>,
}

impl<'i> RuleBodyScan<'i> {
    fn css_wide_hint(&self) -> CssWideValueHint<'i> {
        self.css_wide_candidate
            .clone()
            .map_or(CssWideValueHint::NotCssWide, CssWideValueHint::Candidate)
    }
}

fn drain_rule_body(input: &mut Compiler<'_>) {
    while input.next().is_ok() {}
}

fn scan_single_ident_value<'i>(input: &mut Compiler<'i>) -> Option<Atom<'i>> {
    let ident = match input.next() {
        Ok(ValueToken::Ident(value)) => value.clone(),
        Ok(_) => {
            drain_rule_body(input);
            return None;
        }
        Err(_) => return None,
    };

    match input.next() {
        Err(_) => Some(ident),
        Ok(ValueToken::Delim("!")) => {
            let is_important = matches!(
                input.next(),
                Ok(ValueToken::Ident(value)) if value.eq_ignore_ascii_case("important")
            );
            let is_exhausted = input.next().is_err();
            if is_important && is_exhausted {
                Some(ident)
            } else {
                drain_rule_body(input);
                None
            }
        }
        Ok(_) => {
            drain_rule_body(input);
            None
        }
    }
}

// This single pass serves both nested-rule disambiguation and declaration-value
// classification. A future byte/SIMD scanner must fall back for escapes and
// comments so the decoded candidate and lossless behavior stay unchanged.
fn scan_rule_body<'i>(input: &mut Compiler<'i>, scan_css_wide: bool) -> RuleBodyScan<'i> {
    let state = input.state();
    let mut css_wide_candidate = None;
    let _: Result<(), ParseError<'_, ()>> = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| {
            if scan_css_wide {
                css_wide_candidate = scan_single_ident_value(input);
            } else {
                drain_rule_body(input);
            }
            Ok(())
        },
    );
    let delimiter = match input.next() {
        Ok(ValueToken::Semicolon) => Some(RuleBodyDelimiter::Semicolon),
        Ok(ValueToken::CurlyBracketBlock) => Some(RuleBodyDelimiter::CurlyBracket),
        _ => None,
    };
    let css_wide_candidate = css_wide_candidate.filter(|_| !input.saw_comments_since(&state));
    input.reset(&state);
    RuleBodyScan {
        delimiter,
        css_wide_candidate,
    }
}
