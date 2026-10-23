//! At-rule parsing into the flattened rule tree.

use super::super::{
    css_rule::scan_rule_body,
    media::{parse_import_rule, parse_media_list, parse_supports_condition},
    properties::parse_declaration_with_css_wide_hint,
    selector::parse_selector_string,
    values::collect_tokens,
};
use super::{
    descriptors::{
        at_rule_vendor_prefix, font_feature_subrule_type, page_margin_box, parse_charset,
        parse_container_prelude, parse_custom_media, parse_family_names, parse_font_face_contents,
        parse_font_feature_declarations, parse_font_palette_contents, parse_keyframe_selector,
        parse_keyframes_name, parse_layer_names, parse_namespace, parse_page_selectors,
        parse_property_rule_descriptors, parse_scope_prelude, parse_single_ident,
        parse_view_transition_contents, recover_declaration, validate_moz_document_prelude,
    },
    parse_group_rule_contents, parse_mixed_style_contents, *,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum AtRuleEnding {
    None,
    Semicolon,
    Block,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn parse_group_at_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
    style_context: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    if name.eq_ignore_ascii_case("media") {
        parse_media_rule(input, options, depth, start, name, style_context)
    } else if name.eq_ignore_ascii_case("supports") {
        parse_supports_rule(input, options, depth, start, name, style_context)
    } else if name.eq_ignore_ascii_case("starting-style") {
        parse_starting_style_rule(input, options, depth, start, name, style_context)
    } else if name.eq_ignore_ascii_case("layer") {
        parse_layer_rule(input, options, depth, start, name, style_context)
    } else if name.eq_ignore_ascii_case("container") {
        parse_container_rule(input, options, depth, start, name, style_context)
    } else if name.eq_ignore_ascii_case("scope") {
        parse_scope_rule(input, options, depth, start, name, style_context)
    } else if name.eq_ignore_ascii_case("-moz-document") {
        parse_moz_document_rule(input, options, depth, start, name, style_context)
    } else if name.eq_ignore_ascii_case("counter-style") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_counter_style_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-face") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_face_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-palette-values") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_palette_values_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("view-transition") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_view_transition_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("viewport") || name.eq_ignore_ascii_case("-ms-viewport") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_viewport_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("position-try") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_position_try_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("import")
        || name.eq_ignore_ascii_case("charset")
        || name.eq_ignore_ascii_case("namespace")
        || name.eq_ignore_ascii_case("custom-media")
    {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_top_level_statement_rule(input, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("keyframes")
        || name.eq_ignore_ascii_case("-webkit-keyframes")
        || name.eq_ignore_ascii_case("-moz-keyframes")
        || name.eq_ignore_ascii_case("-o-keyframes")
        || name.eq_ignore_ascii_case("-ms-keyframes")
    {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_keyframes_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("page") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_page_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-feature-values") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_feature_values_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("property") {
        if style_context {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_property_rule(input, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("nest") && style_context {
        parse_nesting_rule(input, options, depth, start, name)
    } else {
        parse_unknown_at_rule(input, depth, start, name)
    }
}

fn parse_media_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
    style_context: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let query = parse_media_list(input, raw_prelude)?;
    let children = parse_group_rule_contents(input, options, depth, style_context)?;
    let span = span_from(start, input.position());
    let media = input
        .ast_mut()
        .alloc_media_rule(MediaRuleData { query }, span);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Media(media),
        None,
        children,
        span,
    ))
}

fn parse_supports_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
    style_context: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let condition = parse_supports_condition(input, raw_prelude);
    let children = parse_group_rule_contents(input, options, depth, style_context)?;
    let span = span_from(start, input.position());
    let supports = input
        .ast_mut()
        .alloc_supports_rule(SupportsRuleData { condition }, span);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Supports(supports),
        None,
        children,
        span,
    ))
}

fn parse_starting_style_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
    style_context: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let children = parse_group_rule_contents(input, options, depth, style_context)?;
    let span = span_from(start, input.position());
    let starting = input
        .ast_mut()
        .alloc_starting_style_rule(StartingStyleRuleData {}, span);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::StartingStyle(starting),
        None,
        children,
        span,
    ))
}

fn parse_layer_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
    style_context: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let (raw_prelude, ending) = parse_at_rule_header(input, depth, name)?;
    let names = parse_layer_names(input, raw_prelude)?;
    if ending == AtRuleEnding::Block {
        if names.len() > 1 {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let layer_name = names.first().copied();
        let children = parse_group_rule_contents(input, options, depth, style_context)?;
        let span = span_from(start, input.position());
        let layer = input
            .ast_mut()
            .alloc_layer_block_rule(LayerBlockRuleData { name: layer_name }, span);
        Ok(alloc_rule(
            input,
            CssRulePayloadData::LayerBlock(layer),
            None,
            children,
            span,
        ))
    } else {
        if names.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let span = span_from(start, input.position());
        let layer = input
            .ast_mut()
            .alloc_layer_statement_rule(LayerStatementRuleData { names }, span);
        Ok(alloc_rule(
            input,
            CssRulePayloadData::LayerStatement(layer),
            None,
            Vec::new(),
            span,
        ))
    }
}

fn parse_container_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
    style_context: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let (container_name, condition) = parse_container_prelude(input, raw_prelude)?;
    let children = parse_group_rule_contents(input, options, depth, style_context)?;
    let span = span_from(start, input.position());
    let container = input.ast_mut().alloc_container_rule(
        ContainerRuleData {
            name: container_name,
            condition,
        },
        span,
    );
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Container(container),
        None,
        children,
        span,
    ))
}

fn parse_scope_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
    style_context: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let (scope_start, scope_end) = parse_scope_prelude(input, raw_prelude, depth + 1)?;
    let children = parse_group_rule_contents(input, options, depth, style_context)?;
    let span = span_from(start, input.position());
    let scope = input.ast_mut().alloc_scope_rule(
        ScopeRuleData {
            scope_start,
            scope_end,
        },
        span,
    );
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Scope(scope),
        None,
        children,
        span,
    ))
}

fn parse_moz_document_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
    style_context: bool,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    validate_moz_document_prelude(input, raw_prelude)?;
    let children = parse_group_rule_contents(input, options, depth, style_context)?;
    let span = span_from(start, input.position());
    let rule = input
        .ast_mut()
        .alloc_moz_document_rule(MozDocumentRuleData {}, span);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::MozDocument(rule),
        None,
        children,
        span,
    ))
}

fn parse_unknown_at_rule<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| collect_tokens(input, depth + 1),
    )?;
    let block = match input.next() {
        Ok(OldToken::Semicolon) => None,
        Ok(OldToken::CurlyBracketBlock) => {
            Some(input.parse_nested_block(|input| collect_tokens(input, depth + 1))?)
        }
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => None,
        Ok(_) => return Err(input.new_custom_error(ParserError::InvalidAtRule(name))),
        Err(error) => return Err(error.into()),
    };
    let span = span_from(start, input.position());
    let unknown = input.ast_mut().alloc_unknown_at_rule(
        UnknownAtRuleData {
            name,
            prelude,
            block,
        },
        span,
    );
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Unknown(unknown),
        None,
        Vec::new(),
        span,
    ))
}

fn parse_top_level_statement_rule<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let (prelude, ending) = parse_at_rule_header(input, depth, name)?;
    if ending == AtRuleEnding::Block {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let span = span_from(start, input.position());
    let data = if name.eq_ignore_ascii_case("import") {
        let import = parse_import_rule(prelude, input, start, input.position())?;
        CssRulePayloadData::Import(import)
    } else if name.eq_ignore_ascii_case("charset") {
        let encoding = parse_charset(input, prelude)?;
        let charset = input
            .ast_mut()
            .alloc_charset_rule(CharsetRuleData { encoding }, span);
        CssRulePayloadData::Charset(charset)
    } else if name.eq_ignore_ascii_case("namespace") {
        let (prefix, url) = parse_namespace(input, prelude)?;
        let namespace = input
            .ast_mut()
            .alloc_namespace_rule(NamespaceRuleData { prefix, url }, span);
        CssRulePayloadData::Namespace(namespace)
    } else {
        let (custom_name, query) = parse_custom_media(input, prelude)?;
        let custom_media = input.ast_mut().alloc_custom_media_rule(
            CustomMediaRuleData {
                name: custom_name,
                query,
            },
            span,
        );
        CssRulePayloadData::CustomMedia(custom_media)
    };
    Ok(alloc_rule(input, data, None, Vec::new(), span))
}

fn parse_keyframes_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    at_rule_name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_keyframes_name(input, prelude)?;
    let frames =
        input.parse_nested_block(|input| parse_keyframe_list_into(input, options, depth + 1))?;
    let span = span_from(start, input.position());
    let keyframes = input.ast_mut().alloc_keyframes_rule(
        KeyframesRuleData {
            name,
            vendor_prefix: at_rule_vendor_prefix(at_rule_name),
        },
        span,
    );
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Keyframes(keyframes),
        None,
        frames,
        span,
    ))
}

fn parse_keyframe_list_into<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<CssRule<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut frames = Vec::new();
    loop {
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }
        let start = input.state();
        let parsed = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
            input.parse_comma_separated(parse_keyframe_selector)
        });
        input.expect_curly_bracket_block()?;
        if parsed.is_err() {
            input.parse_nested_block(|input| {
                while input.next_including_whitespace_and_comments().is_ok() {}
                Ok::<_, ParseError<'i, ParserError<'i>>>(())
            })?;
            continue;
        }
        let selectors = input.ast().alloc_slice(&parsed?);
        let declarations = input.parse_nested_block(|input| {
            parse_standard_declaration_contents(input, options, depth + 1)
        })?;
        let span = span_from(&start, input.position());
        let keyframe = input
            .ast_mut()
            .alloc_keyframe(KeyframeData { selectors }, span);
        let declarations = alloc_declaration_block(input, declarations);
        frames.push(alloc_rule(
            input,
            CssRulePayloadData::Keyframe(keyframe),
            Some(declarations),
            Vec::new(),
            span,
        ));
    }
    Ok(frames)
}

fn parse_font_feature_values_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let family_names = parse_family_names(input, prelude)?;
    let children = input
        .parse_nested_block(|input| parse_font_feature_subrules_into(input, options, depth + 1))?;
    let span = span_from(start, input.position());
    let values = input
        .ast_mut()
        .alloc_font_feature_values_rule(FontFeatureValuesRuleData { name: family_names }, span);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::FontFeatureValues(values),
        None,
        children,
        span,
    ))
}

fn parse_font_feature_subrules_into<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<CssRule<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut subrules = Vec::new();
    loop {
        let start = input.state();
        let name = match input.next() {
            Ok(OldToken::AtKeyword(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidRule)),
            Err(error) => return Err(error.into()),
        };
        let kind = font_feature_subrule_type(name)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidAtRule(name)))?;
        if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let properties = input.parse_nested_block(|input| {
            parse_font_feature_declarations(input, options, depth + 1)
        })?;
        let declarations: Vec<_> = properties
            .into_iter()
            .map(|declaration| {
                alloc_declaration_entry(
                    input,
                    DeclarationPayloadData::FontFeature(declaration),
                    false,
                )
            })
            .collect();
        let span = span_from(&start, input.position());
        let subrule = input
            .ast_mut()
            .alloc_font_feature_subrule_rule(FontFeatureSubruleRuleData { name: kind }, span);
        let declarations = alloc_declaration_block(input, declarations);
        subrules.push(alloc_rule(
            input,
            CssRulePayloadData::FontFeatureSubrule(subrule),
            Some(declarations),
            Vec::new(),
            span,
        ));
    }
    Ok(subrules)
}

fn parse_page_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let selectors = parse_page_selectors(input, prelude)?;
    let items = input.parse_nested_block(|input| parse_page_contents(input, options, depth + 1))?;
    let mut owner_declarations = Vec::new();
    let children = build_page_children(input, items, &mut owner_declarations);
    let span = span_from(start, input.position());
    let page = input
        .ast_mut()
        .alloc_page_rule(PageRuleData { selectors }, span);
    let declarations = alloc_declaration_block(input, owner_declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Page(page),
        Some(declarations),
        children,
        span,
    ))
}

enum PageItem<'i> {
    Declaration(DeclarationEntry<'i>),
    Margin(CssRule<'i>),
}

fn parse_page_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<PageItem<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut items = Vec::new();
    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, OldToken::Semicolon) {
            continue;
        }
        let result = match token {
            OldToken::Ident(name) => {
                input.expect_colon()?;
                let scan = scan_rule_body(input, !name.starts_with("--"));
                parse_declaration_with_css_wide_hint(input, name, depth, scan.css_wide_hint()).map(
                    |(declaration, important)| {
                        let entry = alloc_declaration_entry(
                            input,
                            DeclarationPayloadData::Property(declaration),
                            important,
                        );
                        items.push(PageItem::Declaration(entry));
                    },
                )
            }
            OldToken::AtKeyword(name) => {
                let margin_box = page_margin_box(name)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidAtRule(name)))?;
                if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
                }
                let declarations = input.parse_nested_block(|input| {
                    parse_standard_declaration_contents(input, options, depth + 1)
                })?;
                let span = span_from(&start, input.position());
                let margin = input
                    .ast_mut()
                    .alloc_page_margin_rule(PageMarginRuleData { margin_box }, span);
                let declarations = alloc_declaration_block(input, declarations);
                let rule = alloc_rule(
                    input,
                    CssRulePayloadData::PageMargin(margin),
                    Some(declarations),
                    Vec::new(),
                    span,
                );
                items.push(PageItem::Margin(rule));
                Ok(())
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
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

fn build_page_children<'i>(
    input: &mut Compiler<'i>,
    items: Vec<PageItem<'i>>,
    owner_declarations: &mut Vec<DeclarationEntry<'i>>,
) -> Vec<CssRule<'i>> {
    let mut children = Vec::new();
    let mut current: Option<Vec<DeclarationEntry<'i>>> = None;
    let mut is_first = true;
    for item in items {
        match item {
            PageItem::Declaration(declaration) => {
                current.get_or_insert_with(Vec::new).push(declaration);
            }
            PageItem::Margin(rule) => {
                if let Some(declarations) = current.take() {
                    if is_first {
                        owner_declarations.extend(declarations);
                    } else if !declarations.is_empty() {
                        children.push(alloc_page_declarations_rule(input, declarations));
                    }
                }
                is_first = false;
                children.push(rule);
            }
        }
    }
    if let Some(declarations) = current.take() {
        if is_first {
            owner_declarations.extend(declarations);
        } else if !declarations.is_empty() {
            children.push(alloc_page_declarations_rule(input, declarations));
        }
    }
    children
}

fn alloc_page_declarations_rule<'i>(
    input: &mut Compiler<'i>,
    declarations: Vec<DeclarationEntry<'i>>,
) -> CssRule<'i> {
    let declarations = alloc_declaration_block(input, declarations);
    let payload = with_token_span!(input, |span| input
        .ast_mut()
        .alloc_page_declarations_rule(PageDeclarationsRuleData {}, span));
    alloc_rule(
        input,
        CssRulePayloadData::PageDeclarations(payload),
        Some(declarations),
        Vec::new(),
        token_span(input),
    )
}

fn parse_nesting_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let selectors = parse_selector_string(input, prelude, depth + 1)?;
    let items =
        input.parse_nested_block(|input| parse_mixed_style_contents(input, options, depth + 1))?;
    let mut owner_declarations = Vec::new();
    let children = build_children(input, items, Some(&mut owner_declarations));
    let span = span_from(start, input.position());
    let nesting = input
        .ast_mut()
        .alloc_nesting_rule(NestingRuleData { selectors }, span);
    let declarations = alloc_declaration_block(input, owner_declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Nesting(nesting),
        Some(declarations),
        children,
        span,
    ))
}

fn parse_property_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    at_rule_name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(input, prelude)?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let (descriptors, pushed) = input
        .parse_nested_block(|input| parse_property_rule_descriptors(input, options, depth + 1))?;
    let declarations: Vec<_> = pushed
        .into_iter()
        .map(|descriptor| {
            alloc_declaration_entry(
                input,
                DeclarationPayloadData::PropertyRule(descriptor),
                false,
            )
        })
        .collect();
    let span = span_from(start, input.position());
    let property = input.ast_mut().alloc_property_rule(
        PropertyRuleData {
            name,
            syntax: descriptors.syntax,
            inherits: descriptors.inherits,
            initial_value: descriptors.initial_value,
        },
        span,
    );
    let declarations = alloc_declaration_block(input, declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Property(property),
        Some(declarations),
        Vec::new(),
        span,
    ))
}

fn parse_font_face_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let properties =
        input.parse_nested_block(|input| parse_font_face_contents(input, options, depth + 1))?;
    let declarations: Vec<_> = properties
        .into_iter()
        .map(|property| {
            alloc_declaration_entry(input, DeclarationPayloadData::FontFace(property), false)
        })
        .collect();
    let span = span_from(start, input.position());
    let font_face = input
        .ast_mut()
        .alloc_font_face_rule(FontFaceRuleData {}, span);
    let declarations = alloc_declaration_block(input, declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::FontFace(font_face),
        Some(declarations),
        Vec::new(),
        span,
    ))
}

fn parse_font_palette_values_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    at_rule_name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(input, prelude)?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let properties =
        input.parse_nested_block(|input| parse_font_palette_contents(input, options, depth + 1))?;
    let declarations: Vec<_> = properties
        .into_iter()
        .map(|property| {
            alloc_declaration_entry(
                input,
                DeclarationPayloadData::FontPaletteValues(property),
                false,
            )
        })
        .collect();
    let span = span_from(start, input.position());
    let rule = input
        .ast_mut()
        .alloc_font_palette_values_rule(FontPaletteValuesRuleData { name }, span);
    let declarations = alloc_declaration_block(input, declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::FontPaletteValues(rule),
        Some(declarations),
        Vec::new(),
        span,
    ))
}

fn parse_view_transition_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let properties = input
        .parse_nested_block(|input| parse_view_transition_contents(input, options, depth + 1))?;
    let declarations: Vec<_> = properties
        .into_iter()
        .map(|property| {
            alloc_declaration_entry(
                input,
                DeclarationPayloadData::ViewTransition(property),
                false,
            )
        })
        .collect();
    let span = span_from(start, input.position());
    let rule = input
        .ast_mut()
        .alloc_view_transition_rule(ViewTransitionRuleData {}, span);
    let declarations = alloc_declaration_block(input, declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::ViewTransition(rule),
        Some(declarations),
        Vec::new(),
        span,
    ))
}

fn parse_counter_style_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    at_rule_name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(input, prelude)?;
    let declarations = input.parse_nested_block(|input| {
        parse_standard_declaration_contents(input, options, depth + 1)
    })?;
    let span = span_from(start, input.position());
    let rule = input
        .ast_mut()
        .alloc_counter_style_rule(CounterStyleRuleData { name }, span);
    let declarations = alloc_declaration_block(input, declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::CounterStyle(rule),
        Some(declarations),
        Vec::new(),
        span,
    ))
}

fn parse_viewport_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let declarations = input.parse_nested_block(|input| {
        parse_standard_declaration_contents(input, options, depth + 1)
    })?;
    let span = span_from(start, input.position());
    let rule = input.ast_mut().alloc_viewport_rule(
        ViewportRuleData {
            vendor_prefix: at_rule_vendor_prefix(name),
        },
        span,
    );
    let declarations = alloc_declaration_block(input, declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::Viewport(rule),
        Some(declarations),
        Vec::new(),
        span,
    ))
}

fn parse_position_try_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    start: &crate::ParserState,
    at_rule_name: &'i str,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(input, prelude)?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let declarations = input.parse_nested_block(|input| {
        parse_standard_declaration_contents(input, options, depth + 1)
    })?;
    let span = span_from(start, input.position());
    let rule = input
        .ast_mut()
        .alloc_position_try_rule(PositionTryRuleData { name }, span);
    let declarations = alloc_declaration_block(input, declarations);
    Ok(alloc_rule(
        input,
        CssRulePayloadData::PositionTry(rule),
        Some(declarations),
        Vec::new(),
        span,
    ))
}

fn parse_standard_declaration_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<DeclarationEntry<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut declarations = Vec::new();
    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, OldToken::Semicolon) {
            continue;
        }
        let result = match token {
            OldToken::Ident(name) => {
                input.expect_colon()?;
                let scan = scan_rule_body(input, !name.starts_with("--"));
                parse_declaration_with_css_wide_hint(input, name, depth, scan.css_wide_hint()).map(
                    |(declaration, important)| {
                        let entry = alloc_declaration_entry(
                            input,
                            DeclarationPayloadData::Property(declaration),
                            important,
                        );
                        declarations.push(entry);
                    },
                )
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
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
    Ok(declarations)
}

fn parse_group_rule_prelude<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
    name: &'i str,
) -> Result<&'i str, ParseError<'i, ParserError<'i>>> {
    let (prelude, ending) = parse_at_rule_header(input, depth, name)?;
    if ending == AtRuleEnding::Block {
        Ok(prelude)
    } else {
        Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
    }
}

fn parse_at_rule_header<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
    name: &'i str,
) -> Result<(&'i str, AtRuleEnding), ParseError<'i, ParserError<'i>>> {
    let prelude_start = input.position();
    let _prelude = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| collect_tokens(input, depth + 1),
    )?;
    let raw_prelude = input.slice(prelude_start..input.position()).trim();
    let ending = match input.next() {
        Ok(OldToken::Semicolon) => AtRuleEnding::Semicolon,
        Ok(OldToken::CurlyBracketBlock) => AtRuleEnding::Block,
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => AtRuleEnding::None,
        Ok(_) => return Err(input.new_custom_error(ParserError::InvalidAtRule(name))),
        Err(error) => return Err(error.into()),
    };
    Ok((raw_prelude, ending))
}
