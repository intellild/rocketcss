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
    let mut tokens = input.allocator().vec();

    loop {
        let state = input.state();
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        let span = input
            .current_token_span()
            .map(from_span)
            .unwrap_or(token_span(input));

        match token {
            OldToken::Function(name) => {
                let arguments = input.parse_nested_block(|input| {
                    collect_tokens_impl(input, depth + 1, parse_embedded_values)
                })?;
                let function = Function::new(name, arguments, input.ast_mut(), span);
                validate_rgb_function(input.ast_mut(), function);
                let kind = function.kind(input.ast());
                let valid = function
                    .flags(input.ast())
                    .contains(FunctionFlags::VALID_RGB);
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
            OldToken::UnquotedUrl(url) => {
                let url = input.ast_mut().alloc_url(UrlData { url }, span);
                tokens.push(
                    input
                        .ast_mut()
                        .alloc_token_or_value(TokenOrValueData::Url(url), span),
                );
            }
            OldToken::Ident(name) if name.starts_with("--") => {
                tokens.push(
                    input
                        .ast_mut()
                        .alloc_token_or_value(TokenOrValueData::DashedIdent(name), span),
                );
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
            opening @ (OldToken::ParenthesisBlock
            | OldToken::SquareBracketBlock
            | OldToken::CurlyBracketBlock) => {
                let closing = match opening {
                    OldToken::ParenthesisBlock => OldToken::CloseParenthesis,
                    OldToken::SquareBracketBlock => OldToken::CloseSquareBracket,
                    OldToken::CurlyBracketBlock => OldToken::CloseCurlyBracket,
                    _ => unreachable!(),
                };
                let token = alloc_token(input.ast_mut(), opening, span);
                tokens.push(
                    input
                        .ast_mut()
                        .alloc_token_or_value(TokenOrValueData::Token(token), span),
                );
                let nested = input.parse_nested_block(|input| {
                    collect_tokens_impl(input, depth + 1, parse_embedded_values)
                })?;
                tokens.extend(nested.iter().copied());
                let token = alloc_token(input.ast_mut(), closing, span);
                tokens.push(
                    input
                        .ast_mut()
                        .alloc_token_or_value(TokenOrValueData::Token(token), span),
                );
            }
            OldToken::BadUrl(_)
            | OldToken::BadString(_)
            | OldToken::CloseParenthesis
            | OldToken::CloseSquareBracket
            | OldToken::CloseCurlyBracket => {
                let token = input.current_token().unwrap_or_else(|| {
                    crate::TokenAndSpan::new(crate::Token::BadString, crate::Span::default())
                });
                input.reset(&state);
                return Err(input.new_custom_error(ParserError::UnexpectedToken(token)));
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

    Ok(tokens.into_slice())
}

/// Removes a trailing `!important` from a token list.
pub(crate) fn remove_important<'i>(
    ast: &Ast<'i>,
    value: &'i [TokenOrValue<'i>],
) -> (bool, &'i [TokenOrValue<'i>]) {
    let mut value = value.to_vec();
    let Some(important_index) = previous_non_whitespace(ast, &value, value.len()) else {
        return (false, ast.alloc_slice(&value));
    };
    if !token_ident(ast, &value[important_index])
        .is_some_and(|name| name.eq_ignore_ascii_case("important"))
    {
        trim_trailing_whitespace(ast, &mut value);
        return (false, ast.alloc_slice(&value));
    }
    let Some(bang_index) = previous_non_whitespace(ast, &value, important_index) else {
        trim_trailing_whitespace(ast, &mut value);
        return (false, ast.alloc_slice(&value));
    };
    if !matches!(value[bang_index].data(ast), TokenOrValueData::Token(token) if matches!(token.data(ast), TokenData::Delim("!")))
    {
        trim_trailing_whitespace(ast, &mut value);
        return (false, ast.alloc_slice(&value));
    }
    value.remove(important_index);
    value.remove(bang_index);
    trim_trailing_whitespace(ast, &mut value);
    (true, ast.alloc_slice(&value))
}

fn previous_non_whitespace<'i>(
    ast: &Ast<'i>,
    value: &[TokenOrValue<'i>],
    before: usize,
) -> Option<usize> {
    (0..before).rev().find(|index| {
        !matches!(value[*index].data(ast), TokenOrValueData::Token(token) if matches!(token.data(ast), TokenData::WhiteSpace(_) | TokenData::Comment(_)))
    })
}

fn is_whitespace<'i>(ast: &Ast<'i>, value: &TokenOrValue<'i>) -> bool {
    matches!(
        value.data(ast),
        TokenOrValueData::Token(token) if matches!(token.data(ast), TokenData::WhiteSpace(_))
    )
}

fn trim_trailing_whitespace<'i>(ast: &Ast<'i>, value: &mut Vec<TokenOrValue<'i>>) {
    while value.last().is_some_and(|token| is_whitespace(ast, token)) {
        value.pop();
    }
}

pub(crate) fn trim_leading_whitespace<'i>(
    ast: &Ast<'i>,
    value: &'i [TokenOrValue<'i>],
) -> &'i [TokenOrValue<'i>] {
    let mut start = 0;
    while value
        .get(start)
        .is_some_and(|token| is_whitespace(ast, token))
    {
        start += 1;
    }
    ast.alloc_slice(&value[start..])
}

pub(crate) fn token_ident<'i>(ast: &Ast<'i>, value: &TokenOrValue<'i>) -> Option<&'i str> {
    match value.data(ast) {
        TokenOrValueData::Token(token) => match token.data(ast) {
            TokenData::Ident(name) => Some(name),
            _ => None,
        },
        _ => None,
    }
}

pub(crate) fn css_wide_keyword(value: &str) -> Option<CSSWideKeyword> {
    match_ignore_ascii_case!(
        value,
        "initial" => Some(CSSWideKeyword::Initial),
        "inherit" => Some(CSSWideKeyword::Inherit),
        "unset" => Some(CSSWideKeyword::Unset),
        "revert" => Some(CSSWideKeyword::Revert),
        "revert-layer" => Some(CSSWideKeyword::RevertLayer),
        _ => None,
    )
}

pub(crate) fn token_values_contain_opaque<'i>(ast: &Ast<'i>, values: &[TokenOrValue<'i>]) -> bool {
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
