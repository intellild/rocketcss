use super::{
    color::{parse_hex_color, validate_rgb_function},
    stylesheet::check_depth,
};
use crate::prelude::*;

mod animation;
mod background;
mod box_model;
mod font;
mod multicol;

pub(super) use animation::{parse_animation_list, parse_comma_separated, value_contains_comment};
pub(super) use font::parse_font_family_list;

pub(super) fn single_token<'a, 'i>(value: &'a [TokenOrValue<'i>]) -> Option<&'a ValueToken<'i>> {
    if let [TokenOrValue::Token(token)] = value {
        Some(token)
    } else {
        None
    }
}

pub(super) fn collect_tokens<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<Vec<'i, TokenOrValue<'i>>, ParseError<'i, ParserError<'i>>> {
    collect_tokens_impl(input, allocator, depth, false)
}

pub(super) fn collect_custom_property_tokens<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<Vec<'i, TokenOrValue<'i>>, ParseError<'i, ParserError<'i>>> {
    collect_tokens_impl(input, allocator, depth, true)
}

fn collect_tokens_impl<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    depth: usize,
    parse_embedded_values: bool,
) -> Result<Vec<'i, TokenOrValue<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut tokens = allocator.vec();

    loop {
        let state = input.state();
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        match token {
            ValueToken::Function(name) => {
                let arguments = input.parse_nested_block(|input| {
                    collect_tokens_impl(input, allocator, depth + 1, parse_embedded_values)
                })?;
                let mut function = Function::new(name, arguments);
                validate_rgb_function(&mut function);
                let function = allocator.boxed(function);
                if parse_embedded_values
                    && function.kind().is_color()
                    && (!matches!(function.kind(), KnownFunction::Rgb | KnownFunction::Rgba)
                        || function.is_valid_rgb())
                {
                    tokens.push(TokenOrValue::Color(
                        allocator.boxed(CssColor::Function(function)),
                    ));
                } else {
                    tokens.push(TokenOrValue::Function(function));
                }
            }
            ValueToken::Hash(value) if parse_embedded_values => {
                if let Some(color) = parse_hex_color(value) {
                    tokens.push(TokenOrValue::Color(allocator.boxed(CssColor::Rgba(color))));
                } else {
                    tokens.push(TokenOrValue::Token(
                        allocator.boxed(ValueToken::Hash(value)),
                    ));
                }
            }
            ValueToken::IdHash(value) if parse_embedded_values => {
                if let Some(color) = parse_hex_color(value) {
                    tokens.push(TokenOrValue::Color(allocator.boxed(CssColor::Rgba(color))));
                } else {
                    tokens.push(TokenOrValue::Token(
                        allocator.boxed(ValueToken::IdHash(value)),
                    ));
                }
            }
            ValueToken::UnquotedUrl(url) => {
                tokens.push(TokenOrValue::Url(allocator.boxed(Url {
                    span: input.current_token_span().unwrap_or_default(),
                    url,
                })));
            }
            ValueToken::Ident(name) if name.starts_with("--") => {
                tokens.push(TokenOrValue::DashedIdent(name));
            }
            opening @ (ValueToken::ParenthesisBlock
            | ValueToken::SquareBracketBlock
            | ValueToken::CurlyBracketBlock) => {
                let closing = match opening {
                    ValueToken::ParenthesisBlock => ValueToken::CloseParenthesis,
                    ValueToken::SquareBracketBlock => ValueToken::CloseSquareBracket,
                    ValueToken::CurlyBracketBlock => ValueToken::CloseCurlyBracket,
                    _ => unreachable!(),
                };
                tokens.push(TokenOrValue::Token(allocator.boxed(opening)));
                let nested = input.parse_nested_block(|input| {
                    collect_tokens_impl(input, allocator, depth + 1, parse_embedded_values)
                })?;
                tokens.extend(nested);
                tokens.push(TokenOrValue::Token(allocator.boxed(closing)));
            }
            ValueToken::BadUrl(_)
            | ValueToken::BadString(_)
            | ValueToken::CloseParenthesis
            | ValueToken::CloseSquareBracket
            | ValueToken::CloseCurlyBracket => {
                let token = input.current_token().unwrap_or_else(|| {
                    crate::TokenAndSpan::new(crate::Token::BadString, Span::default())
                });
                input.reset(&state);
                return Err(input.new_custom_error(ParserError::UnexpectedToken(token)));
            }
            token => tokens.push(TokenOrValue::Token(allocator.boxed(token))),
        }
    }

    Ok(tokens)
}

pub(super) fn remove_important(value: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let Some(important_index) = previous_non_whitespace(value, value.len()) else {
        return false;
    };
    if !token_ident(&value[important_index])
        .is_some_and(|name| name.eq_ignore_ascii_case("important"))
    {
        trim_trailing_whitespace(value);
        return false;
    }
    let Some(bang_index) = previous_non_whitespace(value, important_index) else {
        trim_trailing_whitespace(value);
        return false;
    };
    if !matches!(&value[bang_index], TokenOrValue::Token(token) if matches!(**token, ValueToken::Delim("!")))
    {
        trim_trailing_whitespace(value);
        return false;
    }
    value.remove(important_index);
    value.remove(bang_index);
    trim_trailing_whitespace(value);
    true
}

pub(super) fn previous_non_whitespace(value: &[TokenOrValue<'_>], before: usize) -> Option<usize> {
    (0..before).rev().find(|index| {
        !matches!(&value[*index], TokenOrValue::Token(token) if matches!(**token, ValueToken::WhiteSpace(_) | ValueToken::Comment(_)))
    })
}

pub(super) fn trim_trailing_whitespace(value: &mut Vec<'_, TokenOrValue<'_>>) {
    while matches!(value.last(), Some(TokenOrValue::Token(token)) if matches!(**token, ValueToken::WhiteSpace(_)))
    {
        value.pop();
    }
}

pub(super) fn trim_leading_whitespace(value: &mut Vec<'_, TokenOrValue<'_>>) {
    while matches!(value.first(), Some(TokenOrValue::Token(token)) if matches!(**token, ValueToken::WhiteSpace(_)))
    {
        value.remove(0);
    }
}

pub(super) fn token_ident<'i>(value: &TokenOrValue<'i>) -> Option<&'i str> {
    match value {
        TokenOrValue::Token(token) => match **token {
            ValueToken::Ident(name) => Some(name),
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn css_wide_keyword(value: &str) -> Option<CSSWideKeyword> {
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

pub(super) fn matches_ignore_case(value: &str, expected: &[&str]) -> bool {
    expected.iter().any(|item| value.eq_ignore_ascii_case(item))
}
