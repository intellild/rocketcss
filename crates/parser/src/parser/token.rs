use std::borrow::Cow;

use rocketcss_ast::{Token as ValueToken, Unit, match_ignore_ascii_case};
use rocketcss_common::StringPool;

use crate::{Span, Token};

use super::length::parse_length_unit_name;

pub(crate) fn decode_token<'i>(
    kind: Token,
    span: Span,
    source: &'i str,
    string_pool: &mut StringPool,
) -> ValueToken<'i> {
    let raw = &source[span.start as usize..span.end as usize];
    match kind {
        Token::Ident => ValueToken::Ident(string_pool.intern(&decode_name(raw))),
        Token::AtKeyword => ValueToken::AtKeyword(string_pool.intern(&decode_name(&raw[1..]))),
        Token::Hash => ValueToken::Hash(string_pool.intern(&decode_name(&raw[1..]))),
        Token::IDHash => ValueToken::IdHash(string_pool.intern(&decode_name(&raw[1..]))),
        Token::QuotedString => ValueToken::String(string_pool.intern(&decode_string(raw))),
        Token::UnquotedUrl => ValueToken::UnquotedUrl(string_pool.intern(&decode_url(raw))),
        Token::Delim => ValueToken::Delim(raw),
        Token::Number => ValueToken::Number(parse_number(raw)),
        Token::Percentage => ValueToken::Percentage(parse_number(&raw[..raw.len() - 1]) / 100.0),
        Token::Dimension => {
            let number_end = numeric_prefix_len(raw);
            let unit = decode_name(&raw[number_end..]);
            let value = parse_number(&raw[..number_end]);
            if let Some(unit) = parse_unit(&unit) {
                ValueToken::Dimension { unit, value }
            } else {
                ValueToken::UnknownDimension {
                    unit: string_pool.intern(&unit),
                    value,
                }
            }
        }
        Token::WhiteSpace => ValueToken::WhiteSpace(raw),
        Token::Comment => ValueToken::Comment(
            raw.strip_prefix("/*")
                .and_then(|value| value.strip_suffix("*/"))
                .unwrap_or_else(|| raw.strip_prefix("/*").unwrap_or(raw)),
        ),
        Token::Colon => ValueToken::Colon,
        Token::Semicolon => ValueToken::Semicolon,
        Token::Comma => ValueToken::Comma,
        Token::IncludeMatch => ValueToken::IncludeMatch,
        Token::DashMatch => ValueToken::DashMatch,
        Token::PrefixMatch => ValueToken::PrefixMatch,
        Token::SuffixMatch => ValueToken::SuffixMatch,
        Token::SubstringMatch => ValueToken::SubstringMatch,
        Token::CDO => ValueToken::Cdo,
        Token::CDC => ValueToken::Cdc,
        Token::Function => {
            let open = function_opening(raw);
            ValueToken::Function(string_pool.intern(&decode_name(&raw[..open])))
        }
        Token::ParenthesisBlock => ValueToken::ParenthesisBlock,
        Token::SquareBracketBlock => ValueToken::SquareBracketBlock,
        Token::CurlyBracketBlock => ValueToken::CurlyBracketBlock,
        Token::BadUrl => ValueToken::BadUrl(string_pool.intern(&decode_url(raw))),
        Token::BadString => ValueToken::BadString(string_pool.intern(&decode_string(raw))),
        Token::CloseParenthesis => ValueToken::CloseParenthesis,
        Token::CloseSquareBracket => ValueToken::CloseSquareBracket,
        Token::CloseCurlyBracket => ValueToken::CloseCurlyBracket,
    }
}

fn parse_unit(unit: &str) -> Option<Unit> {
    if let Some(unit) = parse_length_unit_name(unit) {
        Some(Unit::Length(unit))
    } else {
        match_ignore_ascii_case!(
            unit,
            "deg" => Some(Unit::Deg),
            "rad" => Some(Unit::Rad),
            "grad" => Some(Unit::Grad),
            "turn" => Some(Unit::Turn),
            "s" => Some(Unit::Seconds),
            "ms" => Some(Unit::Milliseconds),
            "hz" => Some(Unit::Hertz),
            "khz" => Some(Unit::Kilohertz),
            "dpi" => Some(Unit::Dpi),
            "dpcm" => Some(Unit::Dpcm),
            "dppx" => Some(Unit::Dppx),
            "x" => Some(Unit::ResolutionX),
            "fr" => Some(Unit::Flex),
            _ => None,
        )
    }
}

fn decode_name(raw: &str) -> Cow<'_, str> {
    crate::unescape(raw)
}

fn decode_string(raw: &str) -> Cow<'_, str> {
    let Some(quote) = raw.as_bytes().first().copied() else {
        return Cow::Borrowed(raw);
    };
    let mut value = &raw[1..];
    if value.as_bytes().last() == Some(&quote) {
        value = &value[..value.len() - 1];
    }
    decode_name(value)
}

fn decode_url(raw: &str) -> Cow<'_, str> {
    let open = function_opening(raw);
    let mut value = raw[open + 1..].trim_matches(css_whitespace);
    if let Some(without_close) = value.strip_suffix(')') {
        value = without_close.trim_end_matches(css_whitespace);
    }
    decode_name(value)
}

fn function_opening(raw: &str) -> usize {
    let mut position = 0;
    while position < raw.len() {
        match raw.as_bytes()[position] {
            b'(' => return position,
            b'\\' => position = crate::escape::parse_escape(raw, position).end,
            byte if byte.is_ascii() => position += 1,
            _ => {
                position += raw[position..].chars().next().unwrap().len_utf8();
            }
        }
    }
    raw.len().saturating_sub(1)
}

pub(crate) fn numeric_prefix_len(raw: &str) -> usize {
    let bytes = raw.as_bytes();
    let mut position = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));

    while bytes.get(position).is_some_and(u8::is_ascii_digit) {
        position += 1;
    }

    if bytes.get(position) == Some(&b'.') && bytes.get(position + 1).is_some_and(u8::is_ascii_digit)
    {
        position += 1;
        while bytes.get(position).is_some_and(u8::is_ascii_digit) {
            position += 1;
        }
    }

    if matches!(bytes.get(position), Some(b'e' | b'E')) {
        let exponent = position;
        position += 1;
        if matches!(bytes.get(position), Some(b'+' | b'-')) {
            position += 1;
        }
        let digits = position;
        while bytes.get(position).is_some_and(u8::is_ascii_digit) {
            position += 1;
        }
        if digits == position {
            position = exponent;
        }
    }

    position
}

fn parse_number(raw: &str) -> f32 {
    raw.parse()
        .expect("the tokenizer produced a valid CSS number")
}

fn css_whitespace(value: char) -> bool {
    matches!(value, ' ' | '\t' | '\n' | '\r' | '\u{c}')
}

#[cfg(test)]
mod tests {
    use super::numeric_prefix_len;

    #[test]
    fn finds_dimension_unit() {
        for (value, expected) in [
            ("10px", 2),
            ("-1.5e+2rem", 7),
            (".25turn", 3),
            ("1e\\66 oo", 1),
        ] {
            assert_eq!(numeric_prefix_len(value), expected);
        }
    }
}
