use super::{at_rule::parse_group_at_rule, *};

pub(super) fn parse_style_rule<'ast>(
    input: &mut Compiler<'ast>,
    list: RuleListId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    options: &ParserOptions<'ast>,
    depth: usize,
    start: &ParserState,
) -> Result<ConcreteRuleId<'ast>, ParseError<'ast, ParserError<'ast>>> {
    let allocator = input.allocator();
    let selectors = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
        if options.error_recovery {
            parse_selector_list_with_recovery(input, allocator, depth + 1)
        } else {
            parse_selector_list(input, allocator, depth + 1)
        }
    })?;
    input.expect_curly_bracket_block()?;
    let selector_value = input
        .ast_context_mut()
        .intern_selector_value(selectors, SelectorFrameKind::Style, VendorPrefix::NONE)
        .map_err(|error| mutation_error(input, error))?;

    // Allocate the owner before descending so primary IDs follow lexical
    // preorder rather than recursive-return order.
    let rule = input
        .ast_context_mut()
        .append_rule(
            list,
            CssRulePayload::Style(StyleRulePayload {
                selector_value,
                vendor_prefix: VendorPrefix::NONE,
            }),
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
    let declarations = input
        .ast_context_mut()
        .append_declaration_block(DeclarationBlockOwner::Rule(rule), key)
        .map_err(|error| mutation_error(input, error))?;

    input.parse_nested_block(|input| {
        parse_mixed_style_contents(
            input,
            rule,
            key,
            context,
            Some((rule, declarations)),
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

#[allow(clippy::too_many_arguments)]
pub(super) fn parse_mixed_style_contents<'ast>(
    input: &mut Compiler<'ast>,
    owner_rule: ConcreteRuleId<'ast>,
    effective_key: EffectiveKeyId<'ast>,
    context: ConcreteEffectiveContext<'ast>,
    mut active_segment: Option<(ConcreteRuleId<'ast>, ConcreteDeclarationBlockId<'ast>)>,
    options: &ParserOptions<'ast>,
    depth: usize,
) -> Result<(), ParseError<'ast, ParserError<'ast>>> {
    check_depth(input, depth)?;
    let allocator = input.allocator();

    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => *token,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        let result = match token {
            ValueToken::Semicolon => continue,
            ValueToken::AtKeyword(name) => {
                let child_list = ensure_child_list(input, owner_rule)
                    .map_err(|error| mutation_error(input, error))?;
                parse_group_at_rule(input, child_list, context, options, depth, &start, name)
                    .map(|_| active_segment = None)
            }
            ValueToken::Ident(name) => {
                let has_colon = input.try_parse(Compiler::expect_colon).is_ok();
                let scan = scan_rule_body(input, !name.starts_with("--"));
                if has_colon
                    && (name.starts_with("--")
                        || scan.delimiter != Some(RuleBodyDelimiter::CurlyBracket))
                {
                    parse_declaration_with_css_wide_hint(
                        input,
                        allocator,
                        name,
                        depth,
                        scan.css_wide_hint(),
                    )
                    .and_then(|(declaration, important)| {
                        if active_segment.is_none() {
                            let child_list = ensure_child_list(input, owner_rule)
                                .map_err(|error| mutation_error(input, error))?;
                            let span = span_from(&start, input.position());
                            let nested_rule = input
                                .ast_context_mut()
                                .append_rule_with_span(
                                    child_list,
                                    CssRulePayload::NestedDeclarations(NestedDeclarationsPayload),
                                    span,
                                )
                                .map_err(|error| mutation_error(input, error))?;
                            let block = input
                                .ast_context_mut()
                                .append_declaration_block(
                                    DeclarationBlockOwner::Rule(nested_rule),
                                    effective_key,
                                )
                                .map_err(|error| mutation_error(input, error))?;
                            active_segment = Some((nested_rule, block));
                        }
                        let (active_rule, active_block) = active_segment
                            .expect("a declaration always has an active syntax segment");
                        input
                            .ast_context_mut()
                            .append_declaration(
                                active_block,
                                DeclarationPayload::Property(declaration),
                                important,
                            )
                            .map_err(|error| mutation_error(input, error))?;
                        if Some(active_rule) != context.style_rule() {
                            let end = input.position();
                            let mut span = input
                                .ast_context_mut()
                                .rule_span(active_rule)
                                .expect("the active declaration segment remains live");
                            span.end = end.byte_index() as u32;
                            input
                                .ast_context_mut()
                                .set_rule_span(active_rule, span)
                                .map_err(|error| mutation_error(input, error))?;
                        }
                        Ok(())
                    })
                } else if !has_colon && scan.delimiter != Some(RuleBodyDelimiter::CurlyBracket) {
                    Err(input.new_custom_error(ParserError::InvalidDeclaration))
                } else {
                    input.reset(&start);
                    let child_list = ensure_child_list(input, owner_rule)
                        .map_err(|error| mutation_error(input, error))?;
                    parse_style_rule(input, child_list, context, options, depth, &start)
                        .map(|_| active_segment = None)
                }
            }
            _ => {
                input.reset(&start);
                let child_list = ensure_child_list(input, owner_rule)
                    .map_err(|error| mutation_error(input, error))?;
                parse_style_rule(input, child_list, context, options, depth, &start)
                    .map(|_| active_segment = None)
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
    Ok(())
}

pub(super) fn ensure_child_list<'ast>(
    input: &mut Compiler<'ast>,
    owner: ConcreteRuleId<'ast>,
) -> Result<RuleListId<'ast>, rocketcss_ast::ConcreteMutationError<'ast>> {
    if let Some(list) = input
        .ast_context_mut()
        .rule(owner)
        .expect("the current style rule remains live")
        .child_list()
    {
        Ok(list)
    } else {
        input.ast_context_mut().create_child_list(owner)
    }
}
