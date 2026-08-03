use super::{
    style::{ensure_child_list, parse_mixed_style_contents},
    *,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn parse_group_at_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    if name.eq_ignore_ascii_case("media") {
        parse_media_rule(
            input,
            compilation,
            list,
            context,
            options,
            depth,
            start,
            name,
        )
    } else if name.eq_ignore_ascii_case("supports") {
        parse_supports_rule(
            input,
            compilation,
            list,
            context,
            options,
            depth,
            start,
            name,
        )
    } else if name.eq_ignore_ascii_case("starting-style") {
        parse_starting_style_rule(
            input,
            compilation,
            list,
            context,
            options,
            depth,
            start,
            name,
        )
    } else if name.eq_ignore_ascii_case("layer") {
        parse_layer_rule(
            input,
            compilation,
            list,
            context,
            options,
            depth,
            start,
            name,
        )
    } else if name.eq_ignore_ascii_case("container") {
        parse_container_rule(
            input,
            compilation,
            list,
            context,
            options,
            depth,
            start,
            name,
        )
    } else if name.eq_ignore_ascii_case("scope") {
        parse_scope_rule(
            input,
            compilation,
            list,
            context,
            options,
            depth,
            start,
            name,
        )
    } else if name.eq_ignore_ascii_case("-moz-document") {
        parse_moz_document_rule(
            input,
            compilation,
            list,
            context,
            options,
            depth,
            start,
            name,
        )
    } else if name.eq_ignore_ascii_case("counter-style") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_counter_style_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-face") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_face_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-palette-values") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_palette_values_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("view-transition") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_view_transition_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("viewport") || name.eq_ignore_ascii_case("-ms-viewport") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_viewport_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("position-try") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_position_try_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("import")
        || name.eq_ignore_ascii_case("charset")
        || name.eq_ignore_ascii_case("namespace")
        || name.eq_ignore_ascii_case("custom-media")
    {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_top_level_statement_rule(input, compilation, list, depth, start, name)
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
            parse_keyframes_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("page") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_page_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("font-feature-values") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_font_feature_values_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("property") {
        if context.style_rule().is_some() {
            Err(input.new_custom_error(ParserError::InvalidAtRule(name)))
        } else {
            parse_property_rule(input, compilation, list, options, depth, start, name)
        }
    } else if name.eq_ignore_ascii_case("nest") && context.style_rule().is_some() {
        parse_nesting_rule(
            input,
            compilation,
            list,
            context,
            options,
            depth,
            start,
            name,
        )
    } else {
        parse_unknown_at_rule(input, compilation, list, depth, start, name)
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
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let query = parse_media_list(raw_prelude, input.allocator())?;
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::Media(MediaRulePayload {
                span: span_from(start, input.position()),
                query,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, compilation, rule, context, options, depth)?;

    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed media rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::Media(payload) = payload else {
        unreachable!("the allocated payload remains a media rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

#[allow(clippy::too_many_arguments)]
fn parse_layer_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let (raw_prelude, ending) = parse_at_rule_header(input, depth, name)?;
    let mut names = parse_layer_names(raw_prelude, input.allocator())?;
    if ending == AtRuleEnding::Block {
        if names.len() > 1 {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        let rule = compilation
            .append_rule(
                list,
                CssRulePayload::LayerBlock(LayerBlockRulePayload {
                    span: span_from(start, input.position()),
                    name: names.pop(),
                }),
            )
            .map_err(|error| mutation_error(input, error))?;
        parse_group_rule_contents(input, compilation, rule, context, options, depth)?;
        let end = input.position();
        let payload = compilation
            .rule_mut(rule)
            .expect("a parsed layer block remains live while its body is parsed")
            .payload_mut();
        let CssRulePayload::LayerBlock(payload) = payload else {
            unreachable!("the allocated payload remains a layer block")
        };
        payload.span = span_from(start, end);
        Ok(rule)
    } else {
        if names.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
        }
        compilation
            .append_rule(
                list,
                CssRulePayload::LayerStatement(LayerStatementRulePayload {
                    span: span_from(start, input.position()),
                    names,
                }),
            )
            .map_err(|error| mutation_error(input, error))
    }
}

#[allow(clippy::too_many_arguments)]
fn parse_container_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let (container_name, condition) = parse_container_prelude(raw_prelude, input.allocator())?;
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::Container(ContainerRulePayload {
                span: span_from(start, input.position()),
                name: container_name,
                condition,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, compilation, rule, context, options, depth)?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed container rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::Container(payload) = payload else {
        unreachable!("the allocated payload remains a container rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

#[allow(clippy::too_many_arguments)]
fn parse_scope_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let (scope_start, scope_end) = parse_scope_prelude(input, raw_prelude, depth + 1)?;
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::Scope(ScopeRulePayload {
                span: span_from(start, input.position()),
                scope_start,
                scope_end,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, compilation, rule, context, options, depth)?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed scope rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::Scope(payload) = payload else {
        unreachable!("the allocated payload remains a scope rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

#[allow(clippy::too_many_arguments)]
fn parse_moz_document_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    validate_moz_document_prelude(raw_prelude, input.allocator())?;
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::MozDocument(MozDocumentRulePayload {
                span: span_from(start, input.position()),
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, compilation, rule, context, options, depth)?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed moz-document rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::MozDocument(payload) = payload else {
        unreachable!("the allocated payload remains a moz-document rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_unknown_at_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
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
    compilation
        .append_rule(
            list,
            CssRulePayload::Unknown(UnknownAtRulePayload {
                span: span_from(start, input.position()),
                name,
                prelude,
                block,
            }),
        )
        .map_err(|error| mutation_error(input, error))
}

fn parse_top_level_statement_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let (prelude, ending) = parse_at_rule_header(input, depth, name)?;
    if ending == AtRuleEnding::Block {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let allocator = input.allocator();
    let span = span_from(start, input.position());
    let payload = if name.eq_ignore_ascii_case("import") {
        CssRulePayload::Import(parse_import_rule(
            prelude,
            allocator,
            start,
            input.position(),
        )?)
    } else if name.eq_ignore_ascii_case("charset") {
        CssRulePayload::Charset(CharsetRule {
            span,
            encoding: parse_charset(prelude, allocator)?,
        })
    } else if name.eq_ignore_ascii_case("namespace") {
        let (prefix, url) = parse_namespace(prelude, allocator)?;
        CssRulePayload::Namespace(NamespaceRule { span, prefix, url })
    } else {
        let (custom_name, query) = parse_custom_media(prelude, allocator)?;
        CssRulePayload::CustomMedia(CustomMediaRule {
            span,
            name: custom_name,
            query,
        })
    };
    compilation
        .append_rule(list, payload)
        .map_err(|error| mutation_error(input, error))
}

fn parse_keyframes_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_keyframes_name(prelude, input.allocator())?;
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::Keyframes(KeyframesRulePayload {
                span: span_from(start, input.position()),
                name: input.allocator().boxed(name),
                vendor_prefix: at_rule_vendor_prefix(at_rule_name),
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    let frames = compilation
        .create_child_list(rule)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        parse_keyframe_list_into(input, compilation, frames, options, depth + 1)
    })?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed keyframes rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::Keyframes(payload) = payload else {
        unreachable!("the allocated payload remains a keyframes rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_keyframe_list_into<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
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
            compilation,
            list,
            CssRulePayload::Keyframe(KeyframePayload { selectors }),
        )?;
        parse_declaration_owner_body(input, compilation, frame, options, depth)?;
    }
    Ok(())
}

fn parse_font_feature_values_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let family_names = parse_family_names(prelude, input.allocator())?;
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::FontFeatureValues(FontFeatureValuesRulePayload {
                span: span_from(start, input.position()),
                name: family_names,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    let subrules = compilation
        .create_child_list(rule)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        parse_font_feature_subrules_into(input, compilation, subrules, options, depth + 1)
    })?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed font-feature-values rule remains live")
        .payload_mut();
    let CssRulePayload::FontFeatureValues(payload) = payload else {
        unreachable!("the allocated payload remains a font-feature-values rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_font_feature_subrules_into<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
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
            compilation,
            list,
            CssRulePayload::FontFeatureSubrule(FontFeatureSubrulePayload {
                span: span_from(&start, input.position()),
                name: kind,
            }),
        )?;
        let block = compilation
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
                |declaration| {
                    if sink_error.is_none()
                        && let Err(error) = compilation.append_declaration(
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
        let payload = compilation
            .rule_mut(subrule)
            .expect("a parsed font-feature subrule remains live")
            .payload_mut();
        let CssRulePayload::FontFeatureSubrule(payload) = payload else {
            unreachable!("the allocated payload remains a font-feature subrule")
        };
        payload.span = span_from(&start, end);
    }
    Ok(())
}

fn parse_page_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let selectors = parse_page_selectors(prelude, input.allocator())?;
    let page = compilation
        .append_rule(
            list,
            CssRulePayload::Page(PageRulePayload {
                span: span_from(start, input.position()),
                selectors,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    let key = compilation
        .append_effective_key(EffectiveContext::isolated(page))
        .map_err(|error| mutation_error(input, error))?;
    let block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(page), key)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        parse_page_contents(
            input,
            compilation,
            page,
            key,
            Some((page, block)),
            options,
            depth + 1,
        )
    })?;
    let end = input.position();
    let payload = compilation
        .rule_mut(page)
        .expect("a parsed page rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::Page(payload) = payload else {
        unreachable!("the allocated payload remains a page rule")
    };
    payload.span = span_from(start, end);
    Ok(page)
}

fn parse_page_contents<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    page: RuleId,
    effective_key: EffectiveKeyId,
    mut active_segment: Option<(RuleId, DeclarationBlockId)>,
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
                        let children = ensure_child_list(compilation, page)
                            .map_err(|error| mutation_error(input, error))?;
                        let declarations = compilation
                            .append_rule(
                                children,
                                CssRulePayload::PageDeclarations(PageDeclarationsPayload {
                                    span: span_from(&start, input.position()),
                                }),
                            )
                            .map_err(|error| mutation_error(input, error))?;
                        let block = compilation
                            .append_declaration_block(
                                DeclarationBlockOwner::Rule(declarations),
                                effective_key,
                            )
                            .map_err(|error| mutation_error(input, error))?;
                        active_segment = Some((declarations, block));
                    }
                    let (segment, block) = active_segment
                        .expect("a page declaration always has an active syntax segment");
                    compilation
                        .append_declaration(
                            block,
                            DeclarationPayload::Property(declaration),
                            important,
                        )
                        .map_err(|error| mutation_error(input, error))?;
                    if segment != page {
                        let end = input.position();
                        let payload = compilation
                            .rule_mut(segment)
                            .expect("the page declaration segment remains live")
                            .payload_mut();
                        let CssRulePayload::PageDeclarations(payload) = payload else {
                            unreachable!("post-margin declarations use a page segment")
                        };
                        payload.span.end = end.byte_index() as u32;
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
                let children = ensure_child_list(compilation, page)
                    .map_err(|error| mutation_error(input, error))?;
                let margin = append_declaration_owner(
                    input,
                    compilation,
                    children,
                    CssRulePayload::PageMargin(PageMarginPayload {
                        span: span_from(&start, input.position()),
                        margin_box,
                    }),
                )?;
                parse_declaration_owner_body(input, compilation, margin, options, depth)?;
                let end = input.position();
                let payload = compilation
                    .rule_mut(margin)
                    .expect("a parsed page margin remains live")
                    .payload_mut();
                let CssRulePayload::PageMargin(payload) = payload else {
                    unreachable!("the allocated payload remains a page margin")
                };
                payload.span = span_from(&start, end);
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
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, name)?;
    let selectors = parse_selector_string(input, prelude, depth + 1)?;
    let selector_value = compilation
        .intern_selector_value(selectors, SelectorFrameKind::Nesting, VendorPrefix::NONE)
        .map_err(|error| mutation_error(input, error))?;
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::Nesting(NestingRulePayload {
                span: span_from(start, input.position()),
                selector_value,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    let context = compilation
        .enter_selector_context(context, rule)
        .map_err(|error| mutation_error(input, error))?;
    let key = compilation
        .append_effective_key(context.effective_key())
        .map_err(|error| mutation_error(input, error))?;
    let block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(rule), key)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        parse_mixed_style_contents(
            input,
            compilation,
            rule,
            key,
            context,
            Some((rule, block)),
            options,
            depth + 1,
        )
    })?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed nesting rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::Nesting(payload) = payload else {
        unreachable!("the allocated payload remains a nesting rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_property_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(prelude, input.allocator())?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let rule = append_declaration_owner(
        input,
        compilation,
        list,
        CssRulePayload::Property(PropertyRulePayload {
            span: span_from(start, input.position()),
            name,
            syntax: None,
            inherits: None,
            initial_value: None,
        }),
    )?;
    let block = compilation
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a property rule block is bound before parsing descriptors");
    let allocator = input.allocator();
    let mut syntax = None;
    let mut inherits = None;
    let mut initial_value = None;
    let mut sink_error = None;
    input.parse_nested_block(|input| {
        parse_property_rule_descriptors_into(input, allocator, options, depth + 1, |descriptor| {
            if sink_error.is_some() {
                return;
            }
            let kind = match &descriptor {
                PropertyRuleDescriptor::Syntax(_) => 0,
                PropertyRuleDescriptor::Inherits(_) => 1,
                PropertyRuleDescriptor::InitialValue(_) => 2,
                PropertyRuleDescriptor::Unknown(_) => 3,
            };
            match compilation.append_declaration(
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
        })
    })?;
    if let Some(error) = sink_error {
        return Err(mutation_error(input, error));
    }
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed property rule remains live")
        .payload_mut();
    let CssRulePayload::Property(payload) = payload else {
        unreachable!("the allocated payload remains a property rule")
    };
    payload.span = span_from(start, end);
    payload.syntax = syntax;
    payload.inherits = inherits;
    payload.initial_value = initial_value;
    Ok(rule)
}

fn parse_font_face_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let rule = append_declaration_owner(
        input,
        compilation,
        list,
        CssRulePayload::FontFace(FontFaceRulePayload {
            span: span_from(start, input.position()),
        }),
    )?;
    let block = compilation
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a font-face block is bound before parsing descriptors");
    let allocator = input.allocator();
    let mut sink_error = None;
    input.parse_nested_block(|input| {
        parse_font_face_contents_into(input, allocator, options, depth + 1, |property| {
            if sink_error.is_none()
                && let Err(error) = compilation.append_declaration(
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
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed font-face rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::FontFace(payload) = payload else {
        unreachable!("the allocated payload remains a font-face rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_font_palette_values_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(prelude, input.allocator())?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let rule = append_declaration_owner(
        input,
        compilation,
        list,
        CssRulePayload::FontPaletteValues(FontPaletteValuesRulePayload {
            span: span_from(start, input.position()),
            name,
        }),
    )?;
    let block = compilation
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a font-palette-values block is bound before parsing descriptors");
    let allocator = input.allocator();
    let mut sink_error = None;
    input.parse_nested_block(|input| {
        parse_font_palette_contents_into(input, allocator, options, depth + 1, |property| {
            if sink_error.is_none()
                && let Err(error) = compilation.append_declaration(
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
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed font-palette-values rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::FontPaletteValues(payload) = payload else {
        unreachable!("the allocated payload remains a font-palette-values rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_view_transition_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let rule = append_declaration_owner(
        input,
        compilation,
        list,
        CssRulePayload::ViewTransition(ViewTransitionRulePayload {
            span: span_from(start, input.position()),
        }),
    )?;
    let block = compilation
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a view-transition block is bound before parsing descriptors");
    let allocator = input.allocator();
    let mut sink_error = None;
    input.parse_nested_block(|input| {
        parse_view_transition_contents_into(input, allocator, options, depth + 1, |property| {
            if sink_error.is_none()
                && let Err(error) = compilation.append_declaration(
                    block,
                    DeclarationPayload::ViewTransition(property),
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
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed view-transition rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::ViewTransition(payload) = payload else {
        unreachable!("the allocated payload remains a view-transition rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_counter_style_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(prelude, input.allocator())?;
    let rule = append_declaration_owner(
        input,
        compilation,
        list,
        CssRulePayload::CounterStyle(CounterStyleRulePayload {
            span: span_from(start, input.position()),
            name,
        }),
    )?;
    parse_declaration_owner_body(input, compilation, rule, options, depth)?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed counter-style rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::CounterStyle(payload) = payload else {
        unreachable!("the allocated payload remains a counter-style rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_viewport_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let rule = append_declaration_owner(
        input,
        compilation,
        list,
        CssRulePayload::Viewport(ViewportRulePayload {
            span: span_from(start, input.position()),
            vendor_prefix: at_rule_vendor_prefix(name),
        }),
    )?;
    parse_declaration_owner_body(input, compilation, rule, options, depth)?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed viewport rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::Viewport(payload) = payload else {
        unreachable!("the allocated payload remains a viewport rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn parse_position_try_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    at_rule_name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let prelude = parse_group_rule_prelude(input, depth, at_rule_name)?;
    let name = parse_single_ident(prelude, input.allocator())?;
    if !name.starts_with("--") {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(at_rule_name)));
    }
    let rule = append_declaration_owner(
        input,
        compilation,
        list,
        CssRulePayload::PositionTry(PositionTryRulePayload {
            span: span_from(start, input.position()),
            name,
        }),
    )?;
    parse_declaration_owner_body(input, compilation, rule, options, depth)?;
    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed position-try rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::PositionTry(payload) = payload else {
        unreachable!("the allocated payload remains a position-try rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

fn append_declaration_owner<'ast>(
    input: &Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    payload: CssRulePayload<'ast>,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let rule = compilation
        .append_rule(list, payload)
        .map_err(|error| mutation_error(input, error))?;
    let key = compilation
        .append_effective_key(EffectiveContext::isolated(rule))
        .map_err(|error| mutation_error(input, error))?;
    compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(rule), key)
        .map_err(|error| mutation_error(input, error))?;
    Ok(rule)
}

fn parse_declaration_owner_body<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    rule: RuleId,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    let block = compilation
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("a declaration owner is bound before its body is parsed");
    input.parse_nested_block(|input| {
        parse_standard_declaration_contents(input, compilation, block, options, depth + 1)
    })
}

fn parse_standard_declaration_contents<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    block: DeclarationBlockId,
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
                    compilation
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
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    let raw_prelude = parse_group_rule_prelude(input, depth, name)?;
    let condition = parse_supports_condition(raw_prelude);
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::Supports(SupportsRulePayload {
                span: span_from(start, input.position()),
                condition,
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, compilation, rule, context, options, depth)?;

    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed supports rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::Supports(payload) = payload else {
        unreachable!("the allocated payload remains a supports rule")
    };
    payload.span = span_from(start, end);
    Ok(rule)
}

#[allow(clippy::too_many_arguments)]
fn parse_starting_style_rule<'ast>(
    input: &mut Compiler<'ast>,
    compilation: &mut Compilation<'ast>,
    list: RuleListId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
    name: &'ast str,
) -> Result<RuleId, ParseError<'ast, ParserError<'ast>>> {
    if !parse_group_rule_prelude(input, depth, name)?.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidAtRule(name)));
    }
    let rule = compilation
        .append_rule(
            list,
            CssRulePayload::StartingStyle(StartingStyleRulePayload {
                span: span_from(start, input.position()),
            }),
        )
        .map_err(|error| mutation_error(input, error))?;
    parse_group_rule_contents(input, compilation, rule, context, options, depth)?;

    let end = input.position();
    let payload = compilation
        .rule_mut(rule)
        .expect("a parsed starting-style rule remains live while its body is parsed")
        .payload_mut();
    let CssRulePayload::StartingStyle(payload) = payload else {
        unreachable!("the allocated payload remains a starting-style rule")
    };
    payload.span = span_from(start, end);
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
    compilation: &mut Compilation<'ast>,
    rule: RuleId,
    context: EffectiveContext,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    let children = compilation
        .create_child_list(rule)
        .map_err(|error| mutation_error(input, error))?;
    let context = compilation
        .enter_wrapper_context(context, rule)
        .map_err(|error| mutation_error(input, error))?;
    input.parse_nested_block(|input| {
        if context.style_rule().is_some() {
            let key = compilation
                .append_effective_key(context.effective_key())
                .map_err(|error| mutation_error(input, error))?;
            parse_mixed_style_contents(
                input,
                compilation,
                rule,
                key,
                context,
                None,
                options,
                depth + 1,
            )
        } else {
            parse_rule_list(input, compilation, children, context, options, depth + 1)
        }
    })
}
