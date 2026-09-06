use super::*;

pub(in crate::parser) fn parse_view_transition_contents_into<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    mut push: impl FnMut(
        &mut Compiler<'i>,
        ViewTransitionProperty<'i>,
    ) -> Result<(), ParseError<'i, ParserError<'i>>>,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
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

            let property = if descriptor.eq_ignore_ascii_case("navigation") {
                let value = value
                    .first()
                    .and_then(|token| token_ident(input.ast_context(), token))
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                ViewTransitionProperty::Navigation(match_ignore_ascii_case!(
                    value,
                    "auto" => Navigation::Auto,
                    "none" => Navigation::None,
                    _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
                ))
            } else if descriptor.eq_ignore_ascii_case("types") {
                let mut idents = allocator.vec();
                for token in &value {
                    if let TokenOrValue::Token(id) = token
                        && let rocketcss_ast::Token::Ident(ident) = input.ast_context().node(*id)
                    {
                        idents.push(ident);
                    } else if !matches!(token, TokenOrValue::Token(token) if matches!(ValueToken::from_ast(input.ast_context().node(*token), input.ast_context()), ValueToken::WhiteSpace(_)))
                    {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                }
                let types = if idents.len() == 1
                    && input
                        .ast_context()
                        .str(idents[0])
                        .eq_ignore_ascii_case("none")
                {
                    NoneOrCustomIdentList::None
                } else if idents.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                } else {
                    NoneOrCustomIdentList::Idents(store_vec(idents, input))
                };
                ViewTransitionProperty::Types(store_node(types, input))
            } else {
                let value = store_vec(value, input);
                ViewTransitionProperty::Custom(store_node(
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
            Ok::<_, ParseError<'i, ParserError<'i>>>(property)
        })();

        match result {
            Ok(property) => push(input, property)?,
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}
