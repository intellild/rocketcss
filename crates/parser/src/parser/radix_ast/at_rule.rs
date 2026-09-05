use super::{
    style::{ensure_child_list, parse_mixed_style_contents},
    *,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn parse_group_at_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    if name.eq_ignore_ascii_case("media") {
        parse_media_rule(input, list, context, options, depth, start, name)
    } else if name.eq_ignore_ascii_case("supports") {
        parse_supports_rule(input, list, context, options, depth, start, name)
    } else if name.eq_ignore_ascii_case("starting-style") {
        parse_starting_style_rule(input, list, context, options, depth, start, name)
    } else if name.eq_ignore_ascii_case("layer") {
        parse_layer_rule(input, list, context, options, depth, start, name)
    } else if name.eq_ignore_ascii_case("container") {
        parse_container_rule(input, list, context, options, depth, start, name)
    } else if name.eq_ignore_ascii_case("scope") {
        parse_scope_rule(input, list, context, options, depth, start, name)
    } else if name.eq_ignore_ascii_case("-moz-document") {
        parse_moz_document_rule(input, list, context, options, depth, start, name)
    } else if name.eq_ignore_ascii_case("counter-style") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_counter_style_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-face") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_face_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-palette-values") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_palette_values_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("view-transition") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_view_transition_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("viewport") || name.eq_ignore_ascii_case("-ms-viewport") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_viewport_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("position-try") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_position_try_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("import")
        || name.eq_ignore_ascii_case("charset")
        || name.eq_ignore_ascii_case("namespace")
        || name.eq_ignore_ascii_case("custom-media")
    {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_top_level_statement_rule(input, list, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("keyframes")
        || name.eq_ignore_ascii_case("-webkit-keyframes")
        || name.eq_ignore_ascii_case("-moz-keyframes")
        || name.eq_ignore_ascii_case("-o-keyframes")
        || name.eq_ignore_ascii_case("-ms-keyframes")
    {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_keyframes_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("page") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_page_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-feature-values") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_feature_values_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("property") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_property_rule(input, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("nest") && context.style_rule().is_some() {
        parse_nesting_rule(input, list, context, options, depth, start, name)
    } else {
        parse_unknown_at_rule(input, list, depth, start, name)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AtRuleEnding {
    None,
    Semicolon,
    Block,
}

#[allow(clippy::too_many_arguments)]
fn parse_media_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let query = parse_media_list(input, raw_prelude)?;
    let rule = input
        .ast_context_mut()
        .append_rule(list, CssRulePayload::Media(MediaRulePayload { query }))
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, rule, context, options, depth, Some(raw_prelude))?;

    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

#[allow(clippy::too_many_arguments)]
fn parse_layer_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let (raw_prelude, ending) = parse_at_rule_header(input, depth, name)?;
    let mut names = parse_layer_names(raw_prelude, input.allocator())?;
    if ending == AtRuleEnding::Block {
        if names.len() > 1 {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let rule = input
            .ast_context_mut()
            .append_rule(
                list,
                CssRulePayload::LayerBlock(LayerBlockRulePayload { name: names.pop() }),
            )
            .map_err(|error| mutation_error(input, error))?;
        parse_group_rule_contents(input, rule, context, options, depth, None)?;
        let end = input.position();
        input
            .ast_context_mut()
            .set_rule_span(rule, span_from(start, end))
            .map_err(|error| mutation_error(input, error))?;
        Ok(rule)
    } else {
        if names.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let span = span_from(start, input.position());
        input
            .ast_context_mut()
            .append_rule_with_span(
                list,
                CssRulePayload::LayerStatement(LayerStatementRulePayload { names }),
                span,
            )
            .map_err(|error| mutation_error(input, error))
    }
}

#[allow(clippy::too_many_arguments)]
fn parse_container_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let (container_name, condition) = parse_container_prelude(input, raw_prelude)?;
    let rule = input
        .ast_context_mut()
        .append_rule(
            list,
            CssRulePayload::Container(ContainerRulePayload {
                name: container_name,
                condition,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, rule, context, options, depth, Some(raw_prelude))?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

#[allow(clippy::too_many_arguments)]
fn parse_scope_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let (scope_start, scope_end) = parse_scope_prelude(input, raw_prelude, depth + 1)?;
    let rule = input
        .ast_context_mut()
        .append_rule(
            list,
            CssRulePayload::Scope(ScopeRulePayload {
                scope_start,
                scope_end,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, rule, context, options, depth, None)?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

#[allow(clippy::too_many_arguments)]
fn parse_moz_document_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    validate_moz_document_prelude(raw_prelude, input.allocator())?;
    let rule = input
        .ast_context_mut()
        .append_rule(list, CssRulePayload::MozDocument(MozDocumentRulePayload))
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, rule, context, options, depth, None)?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_unknown_at_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let allocator = input.allocator();
    let prelude = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| collect_tokens(input, allocator, depth + 1),
    )?;
    let block = match input.next() {
        Ok(ValueToken::Semicolon) => None,
        Ok(ValueToken::CurlyBracketBlock) => {
            Some(input.parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?)
        }
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => None,
        Ok(_) => return Err(input.new_custom_error(ParserError::InvalidAtRule(name))),
        Err(error) => return Err(error.into()),
    };
    let span = span_from(start, input.position());
    input
        .ast_context_mut()
        .append_rule_with_span(
            list,
            CssRulePayload::Unknown(UnknownAtRulePayload {
                name,
                prelude,
                block,
            }),
            span,
        )
        .map_err(|error| mutation_error(input, error))
}

fn parse_top_level_statement_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let (prelude, ending) = parse_at_rule_header(input, depth, name)?;
    if ending == AtRuleEnding::Block {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let allocator = input.allocator();
    let span = span_from(start, input.position());
    let payload = if name.eq_ignore_ascii_case("import") {
        CssRulePayload::Import(parse_import_rule(input, prelude)?)
    } else if name.eq_ignore_ascii_case("charset") {
        CssRulePayload::Charset(CharsetRule {
            encoding: parse_charset(prelude, allocator)?,
        })
    } else if name.eq_ignore_ascii_case("namespace") {
        let (prefix, url) = parse_namespace(prelude, allocator)?;
        CssRulePayload::Namespace(NamespaceRule { prefix, url })
    } else {
        let (custom_name, query) = parse_custom_media(input, prelude)?;
        CssRulePayload::CustomMedia(CustomMediaRule {
            name: custom_name,
            query,
        })
    };
    input
        .ast_context_mut()
        .append_rule_with_span(list, payload, span)
        .map_err(|error| mutation_error(input, error))
}

fn parse_keyframes_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_keyframes_name(prelude, input.allocator())?;
    let name = store_node(name, input);
    let rule = input
        .ast_context_mut()
        .append_rule(
            list,
            CssRulePayload::Keyframes(KeyframesRulePayload {
                name,
                vendor_prefix: at_rule_vendor_prefix(at_rule_name),
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    let frames = input
        .ast_context_mut()
        .create_child_list(rule)
        .map_err(|error| mutation_error(input, error))?;
    input
        .parse_nested_block(|input| parse_keyframe_list_into(input, frames, options, depth + 1))?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_keyframe_list_into<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    check_depth(input, depth)?;
    let allocator = input.allocator();
    loop {
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }
        let parsed = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
            input.parse_comma_separated(parse_keyframe_selector)
        });
        input.expect_curly_bracket_block()?;
        if parsed.is_err() {
            input.parse_nested_block(|input| {
                while input.next_including_whitespace_and_comments().is_ok() {}
                Ok::<_, ParseError<'ast, ParserError<'ast>>>(())
            })?;
            continue;
        }
        let mut selectors = allocator.vec();
        selectors.extend(parsed?);
        let frame = append_declaration_owner(
            input,
            list,
            CssRulePayload::Keyframe(KeyframePayload { selectors }),
        )?;
        parse_declaration_owner_body(input, frame, options, depth)?;
    }
    Ok(())
}

fn parse_font_feature_values_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let family_names = parse_family_names(prelude, input.allocator())?;
    let rule = input
        .ast_context_mut()
        .append_rule(
            list,
            CssRulePayload::FontFeatureValues(FontFeatureValuesRulePayload { name: family_names }),
        )
        .map_err(|error| mutation_error(input, error))?;
    let subrules = input
        .ast_context_mut()
        .create_child_list(rule)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        parse_font_feature_subrules_into(input, subrules, options, depth + 1)
    })?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_font_feature_subrules_into<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    check_depth(input, depth)?;
    let allocator = input.allocator();
    loop {
        let start = input.state();
        let name = match input.next() {
            Ok(ValueToken::AtKeyword(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidRule)),
            Err(error) => return Err(error.into()),
        };
        let kind = font_feature_subrule_type(name)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidAtRule(name)))?;
        if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let subrule = append_declaration_owner(
            input,
            list,
            CssRulePayload::FontFeatureSubrule(FontFeatureSubrulePayload { name: kind }),
        )?;
        let block = input
            .ast_context_mut()
            .rule(subrule)
            .and_then(|record| record.declaration_block())
            .expect("a font-feature subrule block is bound before descriptors");
        let mut sink_error = None;
        input.parse_nested_block(|input| {
            parse_font_feature_declarations_into(
                input,
                allocator,
                options,
                depth + 1,
                |input, declaration| {
                    if sink_error.is_none()
                        && let Err(error) = input.ast_context_mut().append_declaration(
                            block,
                            DeclarationPayload::FontFeature(declaration),
                            false,
                        )
                    {
                        sink_error = Some(error);
                    }
                    Ok(())
                },
            )
        })?;
        if let Some(error) = sink_error {
            return Err(mutation_error(input, error));
        }
        let end = input.position();
        input
            .ast_context_mut()
            .set_rule_span(subrule, span_from(&start, end))
            .map_err(|error| mutation_error(input, error))?;
    }
    Ok(())
}

fn parse_page_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let selectors = parse_page_selectors(prelude, input.allocator())?;
    let page = input
        .ast_context_mut()
        .append_rule(list, CssRulePayload::Page(PageRulePayload { selectors }))
        .map_err(|error| mutation_error(input, error))?;
    let key = input
        .ast_context_mut()
        .append_effective_key(ConcreteEffectiveContext::<'ast>::isolated(page))
        .map_err(|error| mutation_error(input, error))?;
    let block = input
        .ast_context_mut()
        .append_declaration_block(DeclarationBlockOwner::Rule(page), key)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        parse_page_contents(input, page, key, Some((page, block)), options, depth + 1)
    })?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(page, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(page)
}

fn parse_page_contents<'ast>(
    input: &mut Compiler<'ast>,
    page: ConcreteRuleId<'ast>,
    effective_key: EffectiveKeyId<'ast>,
    mut active_segment: Option<(ConcreteRuleId<'ast>, ConcreteDeclarationBlockId<'ast>)>,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    check_depth(input, depth)?;
    let allocator = input.allocator();
    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, ValueToken::Semicolon) {
            continue;
        }
        let result = match token {
            ValueToken::Ident(name) => {
                input.expect_colon()?;
                let scan = scan_rule_body(input, !name.starts_with("--"));
                parse_declaration_with_css_wide_hint(
                    input,
                    allocator,
                    name,
                    depth,
                    scan.css_wide_hint(),
                )
                .and_then(|(declaration, important)| {
                    if active_segment.is_none() {
                        let children = ensure_child_list(input, page)
                            .map_err(|error| mutation_error(input, error))?;
                        let span = span_from(&start, input.position());
                        let declarations = input
                            .ast_context_mut()
                            .append_rule_with_span(
                                children,
                                CssRulePayload::PageDeclarations(PageDeclarationsPayload),
                                span,
                            )
                            .map_err(|error| mutation_error(input, error))?;
                        let block = input
                            .ast_context_mut()
                            .append_declaration_block(
                                DeclarationBlockOwner::Rule(declarations),
                                effective_key,
                            )
                            .map_err(|error| mutation_error(input, error))?;
                        active_segment = Some((declarations, block));
                    }
                    let (segment, block) = active_segment
                        .expect("a page declaration always has an active syntax segment");
                    input
                        .ast_context_mut()
                        .append_declaration(
                            block,
                            DeclarationPayload::Property(declaration),
                            important,
                        )
                        .map_err(|error| mutation_error(input, error))?;
                    if segment != page {
                        let end = input.position();
                        let mut span = input
                            .ast_context_mut()
                            .rule_span(segment)
                            .expect("the page declaration segment remains live");
                        span.end = end.byte_index() as u32;
                        input
                            .ast_context_mut()
                            .set_rule_span(segment, span)
                            .map_err(|error| mutation_error(input, error))?;
                    }
                    Ok(())
                })
            }
            ValueToken::AtKeyword(name) => {
                let margin_box = page_margin_box(name)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidAtRule(name)))?;
                if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
                }
                let children =
                    ensure_child_list(input, page).map_err(|error| mutation_error(input, error))?;
                let margin = append_declaration_owner(
                    input,
                    children,
                    CssRulePayload::PageMargin(PageMarginPayload { margin_box }),
                )?;
                parse_declaration_owner_body(input, margin, options, depth)?;
                let end = input.position();
                input
                    .ast_context_mut()
                    .set_rule_span(margin, span_from(&start, end))
                    .map_err(|error| mutation_error(input, error))?;
                active_segment = None;
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
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn parse_nesting_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let selectors = parse_selector_string(input, prelude, depth + 1)?;
    let selector_value = input
        .ast_context_mut()
        .intern_selector_value(selectors, SelectorFrameKind::Nesting, VendorPrefix::NONE)
        .map_err(|error| mutation_error(input, error))?;
    let rule = input
        .ast_context_mut()
        .append_rule(
            list,
            CssRulePayload::Nesting(NestingRulePayload { selector_value }),
        )
        .map_err(|error| mutation_error(input, error))?;
    let context = input
        .ast_context_mut()
        .enter_selector_context(context, rule)
        .map_err(|error| mutation_error(input, error))?;
    let key = input
        .ast_context_mut()
        .append_effective_key(context.effective_key())
        .map_err(|error| mutation_error(input, error))?;
    let block = input
        .ast_context_mut()
        .append_declaration_block(DeclarationBlockOwner::Rule(rule), key)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        parse_mixed_style_contents(
            input,
            rule,
            key,
            context,
            Some((rule, block)),
            options,
            depth + 1,
        )
    })?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_property_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(prelude, input.allocator())?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let rule = append_declaration_owner(
        input,
        list,
        CssRulePayload::Property(PropertyRulePayload {
            name,
            syntax: None,
            inherits: None,
            initial_value: None,
        }),
    )?;
    let block = input
        .ast_context_mut()
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a property rule block is bound before parsing descriptors");
    let allocator = input.allocator();
    let mut syntax = None;
    let mut inherits = None;
    let mut initial_value = None;
    let mut sink_error = None;
    input.parse_nested_block(|input| {
        parse_property_rule_descriptors_into(
            input,
            allocator,
            options,
            depth + 1,
            |input, descriptor| {
                if sink_error.is_some() {
                    return;
                }
                let kind = match &descriptor {
                    PropertyRuleDescriptor::Syntax(_) => 0,
                    PropertyRuleDescriptor::Inherits(_) => 1,
                    PropertyRuleDescriptor::InitialValue(_) => 2,
                    PropertyRuleDescriptor::Unknown(_) => 3,
                };
                match input.ast_context_mut().append_declaration(
                    block,
                    DeclarationPayload::PropertyRule(descriptor),
                    false,
                ) {
                    Ok(id) => match kind {
                        0 => syntax = Some(id),
                        1 => inherits = Some(id),
                        2 => initial_value = Some(id),
                        _ => {}
                    },
                    Err(error) => sink_error = Some(error),
                }
            },
        )
    })?;
    if let Some(error) = sink_error {
        return Err(mutation_error(input, error));
    }
    let end = input.position();
    let payload = input
        .ast_context_mut()
        .rule_mut(rule)
        .expect("a parsed property rule remains live")
        .payload_mut();
    let CssRulePayload::Property(payload) = payload else {
        unreachable!("the allocated payload remains a property rule")
    };
    payload.syntax = syntax;
    payload.inherits = inherits;
    payload.initial_value = initial_value;
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_font_face_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let rule =
        append_declaration_owner(input, list, CssRulePayload::FontFace(FontFaceRulePayload))?;
    let block = input
        .ast_context_mut()
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a font-face block is bound before parsing descriptors");
    let allocator = input.allocator();
    let mut sink_error = None;
    input.parse_nested_block(|input| {
        parse_font_face_contents_into(input, allocator, options, depth + 1, |input, property| {
            if sink_error.is_none()
                && let Err(error) = input.ast_context_mut().append_declaration(
                    block,
                    DeclarationPayload::FontFace(property),
                    false,
                )
            {
                sink_error = Some(error);
            }
            Ok(())
        })
    })?;
    if let Some(error) = sink_error {
        return Err(mutation_error(input, error));
    }
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_font_palette_values_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(prelude, input.allocator())?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let rule = append_declaration_owner(
        input,
        list,
        CssRulePayload::FontPaletteValues(FontPaletteValuesRulePayload { name }),
    )?;
    let block = input
        .ast_context_mut()
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a font-palette-values block is bound before parsing descriptors");
    let allocator = input.allocator();
    let mut sink_error = None;
    input.parse_nested_block(|input| {
        parse_font_palette_contents_into(input, allocator, options, depth + 1, |input, property| {
            if sink_error.is_none()
                && let Err(error) = input.ast_context_mut().append_declaration(
                    block,
                    DeclarationPayload::FontPaletteValues(property),
                    false,
                )
            {
                sink_error = Some(error);
            }
            Ok(())
        })
    })?;
    if let Some(error) = sink_error {
        return Err(mutation_error(input, error));
    }
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_view_transition_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let rule = append_declaration_owner(
        input,
        list,
        CssRulePayload::ViewTransition(ViewTransitionRulePayload),
    )?;
    let block = input
        .ast_context_mut()
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a view-transition block is bound before parsing descriptors");
    let allocator = input.allocator();
    let mut sink_error = None;
    input.parse_nested_block(|input| {
        parse_view_transition_contents_into(
            input,
            allocator,
            options,
            depth + 1,
            |input, property| {
                if sink_error.is_none()
                    && let Err(error) = input.ast_context_mut().append_declaration(
                        block,
                        DeclarationPayload::ViewTransition(property),
                        false,
                    )
                {
                    sink_error = Some(error);
                }
                Ok(())
            },
        )
    })?;
    if let Some(error) = sink_error {
        return Err(mutation_error(input, error));
    }
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_counter_style_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(prelude, input.allocator())?;
    let rule = append_declaration_owner(
        input,
        list,
        CssRulePayload::CounterStyle(CounterStyleRulePayload { name }),
    )?;
    parse_declaration_owner_body(input, rule, options, depth)?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_viewport_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let rule = append_declaration_owner(
        input,
        list,
        CssRulePayload::Viewport(ViewportRulePayload {
            vendor_prefix: at_rule_vendor_prefix(name),
        }),
    )?;
    parse_declaration_owner_body(input, rule, options, depth)?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_position_try_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(prelude, input.allocator())?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let rule = append_declaration_owner(
        input,
        list,
        CssRulePayload::PositionTry(PositionTryRulePayload { name }),
    )?;
    parse_declaration_owner_body(input, rule, options, depth)?;
    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn append_declaration_owner<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    payload: CssRulePayload<'ast>,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let rule = input
        .ast_context_mut()
        .append_rule(list, payload)
        .map_err(|error| mutation_error(input, error))?;
    let key = input
        .ast_context_mut()
        .append_effective_key(ConcreteEffectiveContext::<'ast>::isolated(rule))
        .map_err(|error| mutation_error(input, error))?;
    input
        .ast_context_mut()
        .append_declaration_block(DeclarationBlockOwner::Rule(rule), key)
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_declaration_owner_body<'ast>(
    input: &mut Compiler<'ast>,
    rule: ConcreteRuleId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    let block = input
        .ast_context_mut()
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a declaration owner is bound before its body is parsed");
    input.parse_nested_block(|input| {
        parse_standard_declaration_contents(input, block, options, depth + 1)
    })
}

fn parse_standard_declaration_contents<'ast>(
    input: &mut Compiler<'ast>,
    block: ConcreteDeclarationBlockId<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    check_depth(input, depth)?;
    let allocator = input.allocator();
    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, ValueToken::Semicolon) {
            continue;
        }
        let result = match token {
            ValueToken::Ident(name) => {
                input.expect_colon()?;
                let scan = scan_rule_body(input, !name.starts_with("--"));
                parse_declaration_with_css_wide_hint(
                    input,
                    allocator,
                    name,
                    depth,
                    scan.css_wide_hint(),
                )
                .and_then(|(declaration, important)| {
                    input
                        .ast_context_mut()
                        .append_declaration(
                            block,
                            DeclarationPayload::Property(declaration),
                            important,
                        )
                        .map(|_| ())
                        .map_err(|error| mutation_error(input, error))
                })
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
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn parse_supports_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let condition = parse_supports_condition(raw_prelude);
    let rule = input
        .ast_context_mut()
        .append_rule(
            list,
            CssRulePayload::Supports(SupportsRulePayload { condition }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, rule, context, options, depth, Some(raw_prelude))?;

    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

#[allow(clippy::too_many_arguments)]
fn parse_starting_style_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let rule = input
        .ast_context_mut()
        .append_rule(
            list,
            CssRulePayload::StartingStyle(StartingStyleRulePayload),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, rule, context, options, depth, None)?;

    let end = input.position();
    input
        .ast_context_mut()
        .set_rule_span(rule, span_from(start, end))
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_group_rule_prelude<'ast>(
    input: &mut Compiler<'ast>,
    depth: usize,
    name: &'ast str,
) -> Result<&'ast str, ParseError<'ast, ParserError<'ast>>> {
    let (prelude, ending) = parse_at_rule_header(input, depth, name)?;
    if ending == AtRuleEnding::Block {
        Ok(prelude)
    } else {
        Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
    }
}

fn parse_at_rule_header<'ast>(
    input: &mut Compiler<'ast>,
    depth: usize,
    name: &'ast str,
) -> Result<(&'ast str, AtRuleEnding), ParseError<'ast, ParserError<'ast>>> {
    let allocator = input.allocator();
    let prelude_start = input.position();
    let _prelude = input.parse_until_before(
        Delimiter::Semicolon | Delimiter::CurlyBracketBlock,
        |input| collect_tokens(input, allocator, depth + 1),
    )?;
    let raw_prelude = input.slice(prelude_start..input.position()).trim();
    let ending = match input.next() {
        Ok(ValueToken::Semicolon) => AtRuleEnding::Semicolon,
        Ok(ValueToken::CurlyBracketBlock) => AtRuleEnding::Block,
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => AtRuleEnding::None,
        Ok(_) => return Err(input.new_custom_error(ParserError::InvalidAtRule(name))),
        Err(error) => return Err(error.into()),
    };
    Ok((raw_prelude, ending))
}

fn parse_group_rule_contents<'ast>(
    input: &mut Compiler<'ast>,
    rule: ConcreteRuleId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    source_key: Option<&'ast str>,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    let children = input
        .ast_context_mut()
        .create_child_list(rule)
        .map_err(|error| mutation_error(input, error))?;
    let context = input
        .ast_context_mut()
        .enter_wrapper_context_with_source_key(context, rule, source_key)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        if context.style_rule().is_some() {
            let key = input
                .ast_context_mut()
                .append_effective_key(context.effective_key())
                .map_err(|error| mutation_error(input, error))?;
            parse_mixed_style_contents(input, rule, key, context, None, options, depth + 1)
        } else {
            parse_rule_list(input, children, context, options, depth + 1)
        }
    })
}
