use super::*;

pub(in crate::parser) fn parse_property_rule_descriptors_into<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    mut push: impl FnMut(&mut Compiler<'i>, PropertyRuleDescriptor<'i>),
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut has_syntax = false;
    let mut syntax_is_universal = false;
    let mut has_inherits = false;
    let mut has_initial_value = false;

    loop {
        let descriptor = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, allocator, depth + 1)
            })?;
            let _ = input.try_parse(Compiler::expect_semicolon);
            if remove_important(input.ast_context(), &mut value) {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            trim_leading_whitespace(input.ast_context(), &mut value);

            let parsed = if descriptor.eq_ignore_ascii_case("syntax") {
                let [TokenOrValue::Token(token)] = value.as_slice() else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                let rocketcss_ast::Token::String(value) = input.ast_context().node(*token) else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                let syntax = parse_syntax_string(input, value)?;
                syntax_is_universal = matches!(syntax, SyntaxString::Universal);
                has_syntax = true;
                PropertyRuleDescriptor::Syntax(store_node(syntax, input))
            } else if descriptor.eq_ignore_ascii_case("inherits") {
                let Some(value) = value
                    .first()
                    .and_then(|token| token_ident(input.ast_context(), token))
                else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                let inherits = match_ignore_ascii_case!(
                    value,
                    "true" => true,
                    "false" => false,
                    _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
                );
                has_inherits = true;
                PropertyRuleDescriptor::Inherits(inherits)
            } else if descriptor.eq_ignore_ascii_case("initial-value") {
                has_initial_value = true;
                let value = store_vec(value, input);
                PropertyRuleDescriptor::InitialValue(store_node(
                    ParsedComponent::TokenList(value),
                    input,
                ))
            } else {
                let value = store_vec(value, input);
                PropertyRuleDescriptor::Unknown(store_node(
                    CustomProperty {
                        name: store_node(
                            CustomPropertyName::Unknown(input.add_str(descriptor)),
                            input,
                        ),
                        value,
                    },
                    input,
                ))
            };
            push(input, parsed);
            Ok::<_, ParseError<'i, ParserError<'i>>>(())
        })();
        if let Err(error) = result {
            if options.error_recovery {
                recover_declaration(input);
            } else {
                return Err(error);
            }
        }
    }

    if !has_syntax || !has_inherits || (!syntax_is_universal && !has_initial_value) {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok(())
}

pub(in crate::parser) fn parse_syntax_string<'i>(
    input: &mut Compiler<'i>,
    range: rocketcss_ast::AstStr<'i>,
) -> Result<SyntaxString<'i>, ParseError<'i, ParserError<'i>>> {
    let value = input.ast_context().str(range);
    if value == "*" {
        return Ok(SyntaxString::Universal);
    }
    let mut kinds = input.allocator().vec();
    for raw_component in value.split('|') {
        let raw_component = raw_component.trim();
        let (component, multiplier) = if let Some(component) = raw_component.strip_suffix('+') {
            (component.trim_end(), Multiplier::Space)
        } else if let Some(component) = raw_component.strip_suffix('#') {
            (component.trim_end(), Multiplier::Comma)
        } else {
            (raw_component, Multiplier::None)
        };
        let kind = match_ignore_ascii_case!(
            component,
            "<length>" => SyntaxComponentKind::Length,
            "<number>" => SyntaxComponentKind::Number,
            "<percentage>" => SyntaxComponentKind::Percentage,
            "<length-percentage>" => SyntaxComponentKind::LengthPercentage,
            "<string>" => SyntaxComponentKind::String,
            "<color>" => SyntaxComponentKind::Color,
            "<image>" => SyntaxComponentKind::Image,
            "<url>" => SyntaxComponentKind::Url,
            "<integer>" => SyntaxComponentKind::Integer,
            "<angle>" => SyntaxComponentKind::Angle,
            "<time>" => SyntaxComponentKind::Time,
            "<resolution>" => SyntaxComponentKind::Resolution,
            "<transform-function>" => SyntaxComponentKind::TransformFunction,
            "<transform-list>" => SyntaxComponentKind::TransformList,
            "<custom-ident>" => SyntaxComponentKind::CustomIdent,
            _ => if !component.is_empty()
                && component
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
            {
                SyntaxComponentKind::Literal(input.ast_context().string_pool().slice(range, component.as_ptr() as usize - value.as_ptr() as usize, component.as_ptr() as usize - value.as_ptr() as usize + component.len()))
            } else {
                return Err(
                    crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue)
                );
            },
        );
        kinds.push((kind, multiplier));
    }
    if kinds.is_empty() {
        return Err(crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue));
    }
    let mut components = input.allocator().vec();
    for (kind, multiplier) in kinds {
        components.push(SyntaxComponent {
            kind: store_node(kind, input),
            multiplier,
        });
    }
    Ok(SyntaxString::Components(store_vec(components, input)))
}
