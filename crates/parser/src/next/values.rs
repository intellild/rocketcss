//! Token collection into the flattened value tree.

use super::{
    color::{parse_hex_color, validate_rgb_function},
    prelude::*,
    token::alloc_token,
};

const MAX_NESTING_DEPTH: usize = 500;

pub(crate) fn collect_tokens<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
) -> Result<&'i [TokenOrValue<'i>], ParseError<'i, ParserError<'i>>> {
    collect_tokens_impl(input, depth, false)
}

pub(crate) fn collect_custom_property_tokens<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
) -> Result<&'i [TokenOrValue<'i>], ParseError<'i, ParserError<'i>>> {
    collect_tokens_impl(input, depth, true)
}

fn collect_tokens_impl<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
    parse_embedded_values: bool,
) -> Result<&'i [TokenOrValue<'i>], ParseError<'i, ParserError<'i>>> {
    if depth > MAX_NESTING_DEPTH {
        return Err(input.new_custom_error(ParserError::MaximumNestingDepth));
    }
    let mut tokens = Vec::new();
    let span = DUMMY_SP;

    loop {
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        match token {
            OldToken::Function(name) => {
                let arguments = input.parse_nested_block(|input| {
                    collect_tokens_impl(input, depth + 1, parse_embedded_values)
                })?;
                let function = Function::new(name, arguments, input.ast_mut(), span);
                validate_rgb_function(input.ast_mut(), function);
                let kind = function.kind(input.ast());
                let valid = function.flags(input.ast()).contains(FunctionFlags::VALID_RGB);
                let value = if parse_embedded_values
                    && kind.is_color()
                    && (!matches!(kind, KnownFunction::Rgb | KnownFunction::Rgba) || valid)
                {
                    TokenOrValueData::Color(
                        input
                            .ast_mut()
                            .alloc_css_color(CssColorData::Function(function), span),
                    )
                } else {
                    TokenOrValueData::Function(function)
                };
                tokens.push(input.ast_mut().alloc_token_or_value(value, span));
            }
            OldToken::Hash(value) if parse_embedded_values => {
                if let Some(color) = parse_hex_color(value) {
                    let rgba = input.ast_mut().alloc_rgba(color, span);
                    let color = input
                        .ast_mut()
                        .alloc_css_color(CssColorData::Rgba(rgba), span);
                    let value = input
                        .ast_mut()
                        .alloc_token_or_value(TokenOrValueData::Color(color), span);
                    tokens.push(value);
                } else {
                    let token = alloc_token(input.ast_mut(), OldToken::Hash(value), span);
                    tokens.push(
                        input
                            .ast_mut()
                            .alloc_token_or_value(TokenOrValueData::Token(token), span),
                    );
                }
            }
            OldToken::IdHash(value) if parse_embedded_values => {
                if let Some(color) = parse_hex_color(value) {
                    let rgba = input.ast_mut().alloc_rgba(color, span);
                    let color = input
                        .ast_mut()
                        .alloc_css_color(CssColorData::Rgba(rgba), span);
                    let value = input
                        .ast_mut()
                        .alloc_token_or_value(TokenOrValueData::Color(color), span);
                    tokens.push(value);
                } else {
                    let token = alloc_token(input.ast_mut(), OldToken::IdHash(value), span);
                    tokens.push(
                        input
                            .ast_mut()
                            .alloc_token_or_value(TokenOrValueData::Token(token), span),
                    );
                }
            }
            token => {
                let token = alloc_token(input.ast_mut(), token, span);
                tokens.push(
                    input
                        .ast_mut()
                        .alloc_token_or_value(TokenOrValueData::Token(token), span),
                );
            }
        }
    }

    Ok(input.ast().alloc_slice(&tokens))
}

pub(crate) fn token_values_contain_opaque<'i>(
    ast: &Ast<'i>,
    values: &[TokenOrValue<'i>],
) -> bool {
    values.iter().any(|value| match value.data(ast) {
        TokenOrValueData::Token(token) => matches!(token.data(ast), TokenData::Comment(_)),
        TokenOrValueData::Var(_) | TokenOrValueData::Env(_) => true,
        TokenOrValueData::Function(function) => {
            function.kind(ast).is_variable()
                || function.arguments(ast).iter().any(|argument| {
                    token_values_contain_opaque(ast, std::slice::from_ref(argument))
                })
        }
        _ => false,
    })
}
