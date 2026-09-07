use super::values::css_wide_keyword;
use crate::prelude::*;
use std::borrow::Cow;

use rocketcss_ast::{Unit, match_ignore_ascii_case};
use rocketcss_common::Allocator;

use crate::{Span, Token};

use super::length::parse_length_unit_name;

/// A decoded parser token borrowing transient source or escape storage.
/// Persistent tokens are constructed only when this value enters the AST.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueToken<'a> {
    Ident(&'a str),
    AtKeyword(&'a str),
    Hash(&'a str),
    IdHash(&'a str),
    /// A hexadecimal color hash normalized during minification.
    MinifiedHash(&'a str),
    String(&'a str),
    /// A quoted font family that can be serialized as identifiers in place.
    UnquotedFont(&'a str),
    UnquotedUrl(&'a str),
    Delim(&'a str),
    Number(f32),
    Percentage(f32),
    Dimension {
        unit: Unit,
        value: f32,
    },
    UnknownDimension {
        unit: &'a str,
        value: f32,
    },
    WhiteSpace(&'a str),
    Comment(&'a str),
    Colon,
    Semicolon,
    Comma,
    IncludeMatch,
    DashMatch,
    PrefixMatch,
    SuffixMatch,
    SubstringMatch,
    Cdo,
    Cdc,
    Function(&'a str),
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl(&'a str),
    BadString(&'a str),
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}

impl<'a> ValueToken<'a> {
    pub(crate) fn into_ast<'ast>(
        self,
        ast: &mut rocketcss_ast::AstContext<'ast>,
    ) -> rocketcss_ast::Token<'ast> {
        match self {
            Self::Ident(value) => rocketcss_ast::Token::Ident(ast.add_str(value)),
            Self::AtKeyword(value) => rocketcss_ast::Token::AtKeyword(ast.add_str(value)),
            Self::Hash(value) => rocketcss_ast::Token::Hash(ast.add_str(value)),
            Self::IdHash(value) => rocketcss_ast::Token::IdHash(ast.add_str(value)),
            Self::MinifiedHash(value) => rocketcss_ast::Token::MinifiedHash(ast.add_str(value)),
            Self::String(value) => rocketcss_ast::Token::String(ast.add_str(value)),
            Self::UnquotedFont(value) => rocketcss_ast::Token::UnquotedFont(ast.add_str(value)),
            Self::UnquotedUrl(value) => rocketcss_ast::Token::UnquotedUrl(ast.add_str(value)),
            Self::Delim(value) => rocketcss_ast::Token::Delim(ast.add_str(value)),
            Self::WhiteSpace(value) => rocketcss_ast::Token::WhiteSpace(ast.add_str(value)),
            Self::Comment(value) => rocketcss_ast::Token::Comment(ast.add_str(value)),
            Self::Function(value) => rocketcss_ast::Token::Function(ast.add_str(value)),
            Self::BadUrl(value) => rocketcss_ast::Token::BadUrl(ast.add_str(value)),
            Self::BadString(value) => rocketcss_ast::Token::BadString(ast.add_str(value)),
            Self::Number(value) => rocketcss_ast::Token::Number(value),
            Self::Percentage(value) => rocketcss_ast::Token::Percentage(value),
            Self::Colon => rocketcss_ast::Token::Colon,
            Self::Semicolon => rocketcss_ast::Token::Semicolon,
            Self::Comma => rocketcss_ast::Token::Comma,
            Self::IncludeMatch => rocketcss_ast::Token::IncludeMatch,
            Self::DashMatch => rocketcss_ast::Token::DashMatch,
            Self::PrefixMatch => rocketcss_ast::Token::PrefixMatch,
            Self::SuffixMatch => rocketcss_ast::Token::SuffixMatch,
            Self::SubstringMatch => rocketcss_ast::Token::SubstringMatch,
            Self::Cdo => rocketcss_ast::Token::Cdo,
            Self::Cdc => rocketcss_ast::Token::Cdc,
            Self::ParenthesisBlock => rocketcss_ast::Token::ParenthesisBlock,
            Self::SquareBracketBlock => rocketcss_ast::Token::SquareBracketBlock,
            Self::CurlyBracketBlock => rocketcss_ast::Token::CurlyBracketBlock,
            Self::CloseParenthesis => rocketcss_ast::Token::CloseParenthesis,
            Self::CloseSquareBracket => rocketcss_ast::Token::CloseSquareBracket,
            Self::CloseCurlyBracket => rocketcss_ast::Token::CloseCurlyBracket,
            Self::Dimension { unit, value } => rocketcss_ast::Token::Dimension { unit, value },
            Self::UnknownDimension { unit, value } => rocketcss_ast::Token::UnknownDimension {
                unit: ast.add_str(unit),
                value,
            },
        }
    }
    pub(crate) fn from_ast<'ast>(
        value: rocketcss_ast::Token<'ast>,
        ast: &'a rocketcss_ast::AstContext<'ast>,
    ) -> Self {
        match value {
            rocketcss_ast::Token::Ident(value) => Self::Ident(ast.str(value)),
            rocketcss_ast::Token::AtKeyword(value) => Self::AtKeyword(ast.str(value)),
            rocketcss_ast::Token::Hash(value) => Self::Hash(ast.str(value)),
            rocketcss_ast::Token::IdHash(value) => Self::IdHash(ast.str(value)),
            rocketcss_ast::Token::MinifiedHash(value) => Self::MinifiedHash(ast.str(value)),
            rocketcss_ast::Token::String(value) => Self::String(ast.str(value)),
            rocketcss_ast::Token::UnquotedFont(value) => Self::UnquotedFont(ast.str(value)),
            rocketcss_ast::Token::UnquotedUrl(value) => Self::UnquotedUrl(ast.str(value)),
            rocketcss_ast::Token::Delim(value) => Self::Delim(ast.str(value)),
            rocketcss_ast::Token::WhiteSpace(value) => Self::WhiteSpace(ast.str(value)),
            rocketcss_ast::Token::Comment(value) => Self::Comment(ast.str(value)),
            rocketcss_ast::Token::Function(value) => Self::Function(ast.str(value)),
            rocketcss_ast::Token::BadUrl(value) => Self::BadUrl(ast.str(value)),
            rocketcss_ast::Token::BadString(value) => Self::BadString(ast.str(value)),
            rocketcss_ast::Token::Number(value) => Self::Number(value),
            rocketcss_ast::Token::Percentage(value) => Self::Percentage(value),
            rocketcss_ast::Token::Colon => Self::Colon,
            rocketcss_ast::Token::Semicolon => Self::Semicolon,
            rocketcss_ast::Token::Comma => Self::Comma,
            rocketcss_ast::Token::IncludeMatch => Self::IncludeMatch,
            rocketcss_ast::Token::DashMatch => Self::DashMatch,
            rocketcss_ast::Token::PrefixMatch => Self::PrefixMatch,
            rocketcss_ast::Token::SuffixMatch => Self::SuffixMatch,
            rocketcss_ast::Token::SubstringMatch => Self::SubstringMatch,
            rocketcss_ast::Token::Cdo => Self::Cdo,
            rocketcss_ast::Token::Cdc => Self::Cdc,
            rocketcss_ast::Token::ParenthesisBlock => Self::ParenthesisBlock,
            rocketcss_ast::Token::SquareBracketBlock => Self::SquareBracketBlock,
            rocketcss_ast::Token::CurlyBracketBlock => Self::CurlyBracketBlock,
            rocketcss_ast::Token::CloseParenthesis => Self::CloseParenthesis,
            rocketcss_ast::Token::CloseSquareBracket => Self::CloseSquareBracket,
            rocketcss_ast::Token::CloseCurlyBracket => Self::CloseCurlyBracket,
            rocketcss_ast::Token::Dimension { unit, value } => Self::Dimension { unit, value },
            rocketcss_ast::Token::UnknownDimension { unit, value } => Self::UnknownDimension {
                unit: ast.str(unit),
                value,
            },
        }
    }
}

pub(crate) fn decode_token<'i>(
    kind: Token,
    span: Span,
    source: &'i str,
    allocator: &'i Allocator,
) -> ValueToken<'i> {
    let raw = &source[span.start as usize..span.end as usize];
    match kind {
        Token::Ident => ValueToken::Ident(decode_name(raw, allocator)),
        Token::AtKeyword => ValueToken::AtKeyword(decode_name(&raw[1..], allocator)),
        Token::Hash => ValueToken::Hash(decode_name(&raw[1..], allocator)),
        Token::IDHash => ValueToken::IdHash(decode_name(&raw[1..], allocator)),
        Token::QuotedString => ValueToken::String(decode_string(raw, allocator)),
        Token::UnquotedUrl => ValueToken::UnquotedUrl(decode_url(raw, allocator)),
        Token::Delim => ValueToken::Delim(raw),
        Token::Number => ValueToken::Number(parse_number(raw)),
        Token::Percentage => ValueToken::Percentage(parse_number(&raw[..raw.len() - 1]) / 100.0),
        Token::Dimension => {
            let number_end = numeric_prefix_len(raw);
            let unit = decode_name(&raw[number_end..], allocator);
            let value = parse_number(&raw[..number_end]);
            if let Some(unit) = parse_unit(unit) {
                ValueToken::Dimension { unit, value }
            } else {
                ValueToken::UnknownDimension { unit, value }
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
            // consume_ident_like includes the opening parenthesis in the span.
            // Quoted URLs return Function without advancing past that parenthesis.
            debug_assert!(raw.ends_with('('));
            ValueToken::Function(decode_name(&raw[..raw.len() - 1], allocator))
        }
        Token::ParenthesisBlock => ValueToken::ParenthesisBlock,
        Token::SquareBracketBlock => ValueToken::SquareBracketBlock,
        Token::CurlyBracketBlock => ValueToken::CurlyBracketBlock,
        Token::BadUrl => ValueToken::BadUrl(decode_url(raw, allocator)),
        Token::BadString => ValueToken::BadString(decode_string(raw, allocator)),
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

fn decode_name<'i>(raw: &'i str, allocator: &'i Allocator) -> &'i str {
    store(crate::unescape(raw), allocator)
}

fn decode_string<'i>(raw: &'i str, allocator: &'i Allocator) -> &'i str {
    let Some(quote) = raw.as_bytes().first().copied() else {
        return raw;
    };
    let mut value = &raw[1..];
    if value.as_bytes().last() == Some(&quote) {
        value = &value[..value.len() - 1];
    }
    decode_name(value, allocator)
}

fn decode_url<'i>(raw: &'i str, allocator: &'i Allocator) -> &'i str {
    let open = function_opening(raw);
    let mut value = raw[open + 1..].trim_matches(css_whitespace);
    if let Some(without_close) = value.strip_suffix(')') {
        value = without_close.trim_end_matches(css_whitespace);
    }
    decode_name(value, allocator)
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

fn store<'i>(value: Cow<'i, str>, allocator: &'i Allocator) -> &'i str {
    match value {
        Cow::Borrowed(value) => value,
        Cow::Owned(value) => allocator.alloc_str(&value),
    }
}

fn css_whitespace(value: char) -> bool {
    matches!(value, ' ' | '\t' | '\n' | '\r' | '\u{c}')
}

impl<'i> Parse<'i> for AnimationName<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
            if ident.eq_ignore_ascii_case("none") {
                return Ok(Self::None);
            }
            // Custom idents exclude CSS-wide keywords and `default`.
            if css_wide_keyword(ident).is_some() || ident.eq_ignore_ascii_case("default") {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            return Ok(Self::Ident(input.add_str(ident)));
        }
        let value = input.expect_string()?;
        Ok(Self::String(input.add_str(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::numeric_prefix_len;

    #[test]
    fn function_spans_end_at_the_unescaped_opening_parenthesis() {
        let allocator = rocketcss_common::Allocator::new();
        for (source, expected) in [
            ("calc(1 + 2)", "calc"),
            (r"f\(name(1)", "f(name"),
            (r"f\28 name(1)", "f(name"),
            ("函数(1)", "函数"),
            ("url(  \"a.png\")", "url"),
            ("URL(\r\n'a.png')", "URL"),
            (r#"u\72 l( "a.png")"#, "url"),
            ("f\0n(", "f\u{fffd}n"),
        ] {
            let mut tokenizer = crate::tokenizer::Tokenizer::new(source);
            let lexical = tokenizer.next().unwrap();
            assert_eq!(lexical.token, crate::Token::Function);
            let raw = &source[lexical.span.start as usize..lexical.span.end as usize];
            assert!(raw.ends_with('('));
            assert_eq!(super::function_opening(raw), raw.len() - 1);
            let value = super::decode_token(lexical.token, lexical.span, source, &allocator);
            assert!(matches!(value, super::ValueToken::Function(name) if name == expected));
        }
    }

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
