use super::stylesheet::check_depth;
use crate::prelude::*;

pub(super) fn single_token<'a, 'i>(value: &'a [TokenOrValue<'i>]) -> Option<&'a ValueToken<'i>> {
    if let [TokenOrValue::Token(token)] = value {
        Some(token)
    } else {
        None
    }
}

pub(super) fn collect_tokens<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
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
                let arguments = input
                    .parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?;
                tokens.push(TokenOrValue::Function(
                    allocator.boxed(Function::new(name, arguments)),
                ));
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
                let nested = input
                    .parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?;
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
    value.truncate(bang_index);
    trim_trailing_whitespace(value);
    true
}

pub(super) fn previous_non_whitespace(value: &[TokenOrValue<'_>], before: usize) -> Option<usize> {
    (0..before).rev().find(|index| {
        !matches!(&value[*index], TokenOrValue::Token(token) if matches!(**token, ValueToken::WhiteSpace(_) | ValueToken::Comment(_)))
    })
}

pub(super) fn trim_trailing_whitespace(value: &mut Vec<'_, TokenOrValue<'_>>) {
    while matches!(value.last(), Some(TokenOrValue::Token(token)) if matches!(**token, ValueToken::WhiteSpace(_) | ValueToken::Comment(_)))
    {
        value.pop();
    }
}

pub(super) fn trim_leading_whitespace(value: &mut Vec<'_, TokenOrValue<'_>>) {
    while matches!(value.first(), Some(TokenOrValue::Token(token)) if matches!(**token, ValueToken::WhiteSpace(_) | ValueToken::Comment(_)))
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

pub(super) fn ascii_lowercase<'i>(value: &'i str, allocator: &'i Allocator) -> &'i str {
    if value.bytes().all(|byte| !byte.is_ascii_uppercase()) {
        value
    } else {
        allocator.alloc_str(&value.to_ascii_lowercase())
    }
}

pub(super) fn matches_ignore_case(value: &str, expected: &[&str]) -> bool {
    expected.iter().any(|item| value.eq_ignore_ascii_case(item))
}
