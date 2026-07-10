/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;
use std::ops::{BitOr, Range};

use rs_css_allocator::Allocator;
use rs_css_ast::Token as ValueToken;

use crate::tokenizer::TokenizerState;
use crate::{SourceLocation, SourcePosition, Span, Token, TokenAndSpan, Tokenizer};

/// A capture of the parser position and pending nested-block state.
#[derive(Debug, Clone)]
pub struct ParserState {
    tokenizer: TokenizerState,
    at_start_of: Option<BlockType>,
}

impl ParserState {
    #[inline]
    pub fn position(&self) -> SourcePosition {
        self.tokenizer.position()
    }

    #[inline]
    pub fn source_location(&self) -> SourceLocation {
        self.tokenizer.source_location()
    }
}

/// Controls whether a failed delimited parse consumes the rest of its input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseUntilErrorBehavior {
    Consume,
    Stop,
}

/// Fundamental errors produced by the parser infrastructure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BasicParseErrorKind<'i> {
    UnexpectedToken(TokenAndSpan),
    EndOfInput,
    AtRuleInvalid(&'i str),
    AtRuleBodyInvalid,
    QualifiedRuleInvalid,
}

impl fmt::Display for BasicParseErrorKind<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken(token) => write!(formatter, "unexpected token: {token:?}"),
            Self::EndOfInput => formatter.write_str("unexpected end of input"),
            Self::AtRuleInvalid(name) => write!(formatter, "invalid @ rule encountered: '@{name}'"),
            Self::AtRuleBodyInvalid => formatter.write_str("invalid @ rule body encountered"),
            Self::QualifiedRuleInvalid => formatter.write_str("invalid qualified rule encountered"),
        }
    }
}

/// A fundamental parse error and its source location.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicParseError<'i> {
    pub kind: BasicParseErrorKind<'i>,
    pub location: SourceLocation,
}

impl fmt::Display for BasicParseError<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(formatter)
    }
}

impl std::error::Error for BasicParseError<'_> {}

/// Either an infrastructure error or an error produced by a grammar parser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseErrorKind<'i, E> {
    Basic(BasicParseErrorKind<'i>),
    Custom(E),
}

impl<E: fmt::Display> fmt::Display for ParseErrorKind<'_, E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic(error) => error.fmt(formatter),
            Self::Custom(error) => error.fmt(formatter),
        }
    }
}

/// An extensible parse error and its source location.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError<'i, E> {
    pub kind: ParseErrorKind<'i, E>,
    pub location: SourceLocation,
}

impl<'i, E> ParseError<'i, E> {
    pub fn basic(self) -> BasicParseError<'i> {
        match self.kind {
            ParseErrorKind::Basic(kind) => BasicParseError {
                kind,
                location: self.location,
            },
            ParseErrorKind::Custom(_) => panic!("custom parse error is not a basic parse error"),
        }
    }

    pub fn into<U>(self) -> ParseError<'i, U>
    where
        E: Into<U>,
    {
        ParseError {
            kind: match self.kind {
                ParseErrorKind::Basic(error) => ParseErrorKind::Basic(error),
                ParseErrorKind::Custom(error) => ParseErrorKind::Custom(error.into()),
            },
            location: self.location,
        }
    }
}

impl<E: fmt::Display> fmt::Display for ParseError<'_, E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(formatter)
    }
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for ParseError<'_, E> {}

impl<'i, E> From<BasicParseError<'i>> for ParseError<'i, E> {
    fn from(error: BasicParseError<'i>) -> Self {
        Self {
            kind: ParseErrorKind::Basic(error.kind),
            location: error.location,
        }
    }
}

impl SourceLocation {
    #[inline]
    pub fn new_basic_error<'i>(self, kind: BasicParseErrorKind<'i>) -> BasicParseError<'i> {
        BasicParseError {
            kind,
            location: self,
        }
    }

    #[inline]
    pub fn new_error<'i, E>(self, kind: BasicParseErrorKind<'i>) -> ParseError<'i, E> {
        ParseError {
            kind: ParseErrorKind::Basic(kind),
            location: self,
        }
    }

    #[inline]
    pub fn new_custom_error<'i, E1: Into<E2>, E2>(self, error: E1) -> ParseError<'i, E2> {
        ParseError {
            kind: ParseErrorKind::Custom(error.into()),
            location: self,
        }
    }
}

/// Tokenizer storage and the one-token decoded-value cache used by [`Parser`].
pub struct ParserInput<'i> {
    tokenizer: Tokenizer<'i>,
    source: &'i str,
    allocator: &'i Allocator,
    cached_token: Option<CachedToken<'i>>,
    source_map_url: Option<&'i str>,
    source_url: Option<&'i str>,
}

struct CachedToken<'i> {
    lexical: TokenAndSpan,
    value: ValueToken<'i>,
    end_state: TokenizerState,
}

impl<'i> ParserInput<'i> {
    /// Creates parser input. Escaped values are allocated in `allocator` only when consumed.
    pub fn new(source: &'i str, allocator: &'i Allocator) -> Self {
        Self {
            tokenizer: Tokenizer::new(source),
            source,
            allocator,
            cached_token: None,
            source_map_url: None,
            source_url: None,
        }
    }

    fn cached_value(&self) -> &ValueToken<'i> {
        &self.cached_token.as_ref().unwrap().value
    }

    fn cached_lexical(&self) -> TokenAndSpan {
        self.cached_token.as_ref().unwrap().lexical
    }

    fn observe_comment(&mut self, token: TokenAndSpan) {
        let raw = &self.source[token.span.start as usize..token.span.end as usize];
        let contents = raw
            .strip_prefix("/*")
            .and_then(|value| value.strip_suffix("*/"))
            .unwrap_or_else(|| raw.strip_prefix("/*").unwrap_or(raw));
        for (directive, source_map) in [
            ("# sourceMappingURL=", true),
            ("@ sourceMappingURL=", true),
            ("# sourceURL=", false),
            ("@ sourceURL=", false),
        ] {
            if let Some(value) = contents.strip_prefix(directive) {
                let value = value
                    .split([' ', '\t', '\u{c}', '\r', '\n'])
                    .next()
                    .unwrap_or_default();
                if source_map {
                    self.source_map_url = Some(value);
                } else {
                    self.source_url = Some(value);
                }
                break;
            }
        }
    }
}

/// A CSS parser with nested-block handling, backtracking, and delayed token decoding.
pub struct Parser<'i, 't> {
    input: &'t mut ParserInput<'i>,
    at_start_of: Option<BlockType>,
    stop_before: Delimiters,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlockType {
    Parenthesis,
    SquareBracket,
    CurlyBracket,
}

impl BlockType {
    fn opening(token: Token) -> Option<Self> {
        match token {
            Token::Function | Token::ParenthesisBlock => Some(Self::Parenthesis),
            Token::SquareBracketBlock => Some(Self::SquareBracket),
            Token::CurlyBracketBlock => Some(Self::CurlyBracket),
            _ => None,
        }
    }

    fn closing(token: Token) -> Option<Self> {
        match token {
            Token::CloseParenthesis => Some(Self::Parenthesis),
            Token::CloseSquareBracket => Some(Self::SquareBracket),
            Token::CloseCurlyBracket => Some(Self::CurlyBracket),
            _ => None,
        }
    }
}

/// A set of top-level delimiters used by `parse_until_*`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Delimiters {
    bits: u8,
}

#[allow(non_upper_case_globals, non_snake_case)]
pub mod Delimiter {
    use super::Delimiters;

    pub const None: Delimiters = Delimiters { bits: 0 };
    pub const CurlyBracketBlock: Delimiters = Delimiters { bits: 1 << 1 };
    pub const Semicolon: Delimiters = Delimiters { bits: 1 << 2 };
    pub const Bang: Delimiters = Delimiters { bits: 1 << 3 };
    pub const Comma: Delimiters = Delimiters { bits: 1 << 4 };
}

#[allow(non_upper_case_globals, non_snake_case)]
mod ClosingDelimiter {
    use super::Delimiters;

    pub const CloseCurlyBracket: Delimiters = Delimiters { bits: 1 << 5 };
    pub const CloseSquareBracket: Delimiters = Delimiters { bits: 1 << 6 };
    pub const CloseParenthesis: Delimiters = Delimiters { bits: 1 << 7 };
}

impl BitOr for Delimiters {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}

impl Delimiters {
    fn contains(self, other: Self) -> bool {
        self.bits & other.bits != 0
    }

    fn from_byte(byte: Option<u8>) -> Self {
        match byte {
            Some(b'{') => Delimiter::CurlyBracketBlock,
            Some(b';') => Delimiter::Semicolon,
            Some(b'!') => Delimiter::Bang,
            Some(b',') => Delimiter::Comma,
            Some(b'}') => ClosingDelimiter::CloseCurlyBracket,
            Some(b']') => ClosingDelimiter::CloseSquareBracket,
            Some(b')') => ClosingDelimiter::CloseParenthesis,
            _ => Delimiter::None,
        }
    }
}

impl<'i, 't> Parser<'i, 't> {
    pub fn new(input: &'t mut ParserInput<'i>) -> Self {
        Self {
            input,
            at_start_of: None,
            stop_before: Delimiter::None,
        }
    }

    #[inline]
    pub fn allocator(&self) -> &'i Allocator {
        self.input.allocator
    }

    pub fn is_exhausted(&mut self) -> bool {
        let state = self.state();
        let exhausted = self.expect_exhausted().is_ok();
        if !exhausted {
            self.reset(&state);
        }
        exhausted
    }

    pub fn expect_exhausted(&mut self) -> Result<(), BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next() {
            Err(BasicParseError {
                kind: BasicParseErrorKind::EndOfInput,
                ..
            }) => Ok(()),
            Err(error) => Err(error),
            Ok(_) => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    #[inline]
    pub fn position(&self) -> SourcePosition {
        self.input.tokenizer.position()
    }

    #[inline]
    pub fn current_source_location(&self) -> SourceLocation {
        self.input.tokenizer.current_source_location()
    }

    #[inline]
    pub fn current_line(&self) -> &'i str {
        self.input.tokenizer.current_source_line()
    }

    #[inline]
    pub fn current_source_map_url(&self) -> Option<&'i str> {
        self.input.source_map_url
    }

    #[inline]
    pub fn current_source_url(&self) -> Option<&'i str> {
        self.input.source_url
    }

    #[inline]
    pub fn current_token_span(&self) -> Option<Span> {
        self.input
            .cached_token
            .as_ref()
            .map(|token| token.lexical.span)
    }

    #[inline]
    pub fn current_token(&self) -> Option<TokenAndSpan> {
        self.input.cached_token.as_ref().map(|token| token.lexical)
    }

    #[inline]
    pub fn new_basic_error(&self, kind: BasicParseErrorKind<'i>) -> BasicParseError<'i> {
        self.current_source_location().new_basic_error(kind)
    }

    #[inline]
    pub fn new_error<E>(&self, kind: BasicParseErrorKind<'i>) -> ParseError<'i, E> {
        self.current_source_location().new_error(kind)
    }

    #[inline]
    pub fn new_custom_error<E1: Into<E2>, E2>(&self, error: E1) -> ParseError<'i, E2> {
        self.current_source_location().new_custom_error(error)
    }

    pub fn new_error_for_next_token<E>(&mut self) -> ParseError<'i, E> {
        let location = self.current_source_location();
        match self.next() {
            Ok(_) => location.new_error(BasicParseErrorKind::UnexpectedToken(
                self.input.cached_lexical(),
            )),
            Err(error) => error.into(),
        }
    }

    #[inline]
    pub fn state(&self) -> ParserState {
        ParserState {
            tokenizer: self.input.tokenizer.state(),
            at_start_of: self.at_start_of,
        }
    }

    #[inline]
    pub fn reset(&mut self, state: &ParserState) {
        self.input.tokenizer.reset(&state.tokenizer);
        self.at_start_of = state.at_start_of;
    }

    pub fn try_parse<F, T, E>(&mut self, parse: F) -> Result<T, E>
    where
        F: FnOnce(&mut Parser<'i, 't>) -> Result<T, E>,
    {
        let start = self.state();
        let result = parse(self);
        if result.is_err() {
            self.reset(&start);
        }
        result
    }

    #[inline]
    pub fn slice(&self, range: Range<SourcePosition>) -> &'i str {
        self.input.tokenizer.slice(range)
    }

    #[inline]
    pub fn slice_from(&self, start: SourcePosition) -> &'i str {
        self.input.tokenizer.slice_from(start)
    }

    pub fn skip_whitespace(&mut self) {
        if let Some(block) = self.at_start_of.take() {
            consume_until_end_of_block(block, &mut self.input.tokenizer);
        }
        loop {
            if !matches!(
                self.input.tokenizer.next_byte(),
                Some(b' ' | b'\t' | b'\n' | b'\r' | b'\x0c' | b'/')
            ) {
                break;
            }
            let state = self.input.tokenizer.state();
            let Ok(token) = self.input.tokenizer.next() else {
                break;
            };
            match token.token {
                Token::WhiteSpace => {}
                Token::Comment => self.input.observe_comment(token),
                _ => {
                    self.input.tokenizer.reset(&state);
                    break;
                }
            }
        }
    }

    pub fn skip_cdc_and_cdo(&mut self) {
        if let Some(block) = self.at_start_of.take() {
            consume_until_end_of_block(block, &mut self.input.tokenizer);
        }
        self.input.tokenizer.skip_cdc_and_cdo();
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<&ValueToken<'i>, BasicParseError<'i>> {
        self.skip_whitespace();
        self.next_including_whitespace_and_comments()
    }

    pub fn next_including_whitespace(&mut self) -> Result<&ValueToken<'i>, BasicParseError<'i>> {
        loop {
            self.next_including_whitespace_and_comments()?;
            if !matches!(self.input.cached_value(), ValueToken::Comment(_)) {
                return Ok(self.input.cached_value());
            }
        }
    }

    pub fn next_including_whitespace_and_comments(
        &mut self,
    ) -> Result<&ValueToken<'i>, BasicParseError<'i>> {
        if let Some(block) = self.at_start_of.take() {
            consume_until_end_of_block(block, &mut self.input.tokenizer);
        }

        let byte = self.input.tokenizer.next_byte();
        if self.stop_before.contains(Delimiters::from_byte(byte)) {
            return Err(self.new_basic_error(BasicParseErrorKind::EndOfInput));
        }

        let start = self.input.tokenizer.position();
        let use_cache = self
            .input
            .cached_token
            .as_ref()
            .is_some_and(|cached| cached.lexical.span.start as usize == start.byte_index());

        if use_cache {
            let end_state = self.input.cached_token.as_ref().unwrap().end_state.clone();
            self.input.tokenizer.reset(&end_state);
        } else {
            let lexical = self
                .input
                .tokenizer
                .next()
                .map_err(|()| self.new_basic_error(BasicParseErrorKind::EndOfInput))?;
            let value = crate::value::decode_token(
                lexical.token,
                lexical.span,
                self.input.source,
                self.input.allocator,
            );
            if lexical.token == Token::Comment {
                self.input.observe_comment(lexical);
            }
            self.input.cached_token = Some(CachedToken {
                lexical,
                value,
                end_state: self.input.tokenizer.state(),
            });
        }

        let lexical = self.input.cached_lexical();
        self.at_start_of = BlockType::opening(lexical.token);
        Ok(self.input.cached_value())
    }

    pub fn parse_entirely<F, T, E>(&mut self, parse: F) -> Result<T, ParseError<'i, E>>
    where
        F: FnOnce(&mut Parser<'i, 't>) -> Result<T, ParseError<'i, E>>,
    {
        let result = parse(self)?;
        self.expect_exhausted()?;
        Ok(result)
    }

    pub fn parse_comma_separated<F, T, E>(
        &mut self,
        parse_one: F,
    ) -> Result<Vec<T>, ParseError<'i, E>>
    where
        F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        self.parse_comma_separated_internal(parse_one, false)
    }

    pub fn parse_comma_separated_ignoring_errors<F, T, E>(&mut self, parse_one: F) -> Vec<T>
    where
        E: 'i,
        F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        self.parse_comma_separated_internal(parse_one, true)
            .unwrap_or_else(|_| unreachable!())
    }

    fn parse_comma_separated_internal<F, T, E>(
        &mut self,
        mut parse_one: F,
        ignore_errors: bool,
    ) -> Result<Vec<T>, ParseError<'i, E>>
    where
        F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        let mut values = Vec::with_capacity(1);
        loop {
            self.skip_whitespace();
            match self.parse_until_before(Delimiter::Comma, &mut parse_one) {
                Ok(value) => values.push(value),
                Err(error) if !ignore_errors => return Err(error),
                Err(_) => {}
            }
            match self.next() {
                Err(_) => return Ok(values),
                Ok(ValueToken::Comma) => {}
                Ok(_) => unreachable!(),
            }
        }
    }

    pub fn parse_nested_block<F, T, E>(&mut self, parse: F) -> Result<T, ParseError<'i, E>>
    where
        F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        parse_nested_block(self, parse)
    }

    pub fn parse_until_before<F, T, E>(
        &mut self,
        delimiters: Delimiters,
        parse: F,
    ) -> Result<T, ParseError<'i, E>>
    where
        F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        parse_until_before(self, delimiters, ParseUntilErrorBehavior::Consume, parse)
    }

    pub fn parse_until_after<F, T, E>(
        &mut self,
        delimiters: Delimiters,
        parse: F,
    ) -> Result<T, ParseError<'i, E>>
    where
        F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        parse_until_after(self, delimiters, ParseUntilErrorBehavior::Consume, parse)
    }

    pub fn expect_whitespace(&mut self) -> Result<&'i str, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next_including_whitespace()? {
            ValueToken::WhiteSpace(value) => Ok(value),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_ident(&mut self) -> Result<&'i str, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::Ident(value) => Ok(value),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_ident_matching(&mut self, expected: &str) -> Result<(), BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::Ident(value) if value.eq_ignore_ascii_case(expected) => Ok(()),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_string(&mut self) -> Result<&'i str, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::String(value) => Ok(value),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_ident_or_string(&mut self) -> Result<&'i str, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::Ident(value) | ValueToken::String(value) => Ok(value),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_url(&mut self) -> Result<&'i str, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::UnquotedUrl(value) => Ok(value),
            ValueToken::Function(name) if name.eq_ignore_ascii_case("url") => self
                .parse_nested_block(|input| {
                    let value = input.expect_string()?;
                    input.expect_exhausted()?;
                    Ok::<_, ParseError<'i, ()>>(value)
                })
                .map_err(ParseError::basic),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_url_or_string(&mut self) -> Result<&'i str, BasicParseError<'i>> {
        let state = self.state();
        match self.expect_url() {
            Ok(value) => Ok(value),
            Err(_) => {
                self.reset(&state);
                self.expect_string()
            }
        }
    }

    pub fn expect_number(&mut self) -> Result<f32, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::Number(value) => Ok(*value),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_integer(&mut self) -> Result<i32, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::Number(_) => {
                let span = self.input.cached_lexical().span;
                let raw = &self.input.source[span.start as usize..span.end as usize];
                if raw.contains(['.', 'e', 'E']) {
                    return Err(
                        location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                            self.input.cached_lexical(),
                        )),
                    );
                }
                let value = raw.parse::<f64>().unwrap();
                Ok(value.clamp(i32::MIN as f64, i32::MAX as f64) as i32)
            }
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_percentage(&mut self) -> Result<f32, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::Percentage(value) => Ok(*value),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_colon(&mut self) -> Result<(), BasicParseError<'i>> {
        self.expect_token(|token| matches!(token, ValueToken::Colon))
    }

    pub fn expect_semicolon(&mut self) -> Result<(), BasicParseError<'i>> {
        self.expect_token(|token| matches!(token, ValueToken::Semicolon))
    }

    pub fn expect_comma(&mut self) -> Result<(), BasicParseError<'i>> {
        self.expect_token(|token| matches!(token, ValueToken::Comma))
    }

    pub fn expect_delim(&mut self, expected: char) -> Result<(), BasicParseError<'i>> {
        self.expect_token(|token| {
            matches!(token, ValueToken::Delim(value) if value.starts_with(expected) && value.len() == expected.len_utf8())
        })
    }

    pub fn expect_curly_bracket_block(&mut self) -> Result<(), BasicParseError<'i>> {
        self.expect_token(|token| matches!(token, ValueToken::CurlyBracketBlock))
    }

    pub fn expect_square_bracket_block(&mut self) -> Result<(), BasicParseError<'i>> {
        self.expect_token(|token| matches!(token, ValueToken::SquareBracketBlock))
    }

    pub fn expect_parenthesis_block(&mut self) -> Result<(), BasicParseError<'i>> {
        self.expect_token(|token| matches!(token, ValueToken::ParenthesisBlock))
    }

    pub fn expect_function(&mut self) -> Result<&'i str, BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::Function(name) => Ok(name),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_function_matching(&mut self, expected: &str) -> Result<(), BasicParseError<'i>> {
        let location = self.current_source_location();
        match self.next()? {
            ValueToken::Function(name) if name.eq_ignore_ascii_case(expected) => Ok(()),
            _ => Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            ),
        }
    }

    pub fn expect_no_error_token(&mut self) -> Result<(), BasicParseError<'i>> {
        loop {
            match self.next_including_whitespace_and_comments() {
                Ok(
                    ValueToken::Function(_)
                    | ValueToken::ParenthesisBlock
                    | ValueToken::SquareBracketBlock
                    | ValueToken::CurlyBracketBlock,
                ) => self
                    .parse_nested_block(|input| {
                        input.expect_no_error_token()?;
                        Ok::<_, ParseError<'i, ()>>(())
                    })
                    .map_err(ParseError::basic)?,
                Ok(
                    ValueToken::BadUrl(_)
                    | ValueToken::BadString(_)
                    | ValueToken::CloseParenthesis
                    | ValueToken::CloseSquareBracket
                    | ValueToken::CloseCurlyBracket,
                ) => {
                    return Err(self.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                        self.input.cached_lexical(),
                    )));
                }
                Ok(_) => {}
                Err(BasicParseError {
                    kind: BasicParseErrorKind::EndOfInput,
                    ..
                }) => return Ok(()),
                Err(error) => return Err(error),
            }
        }
    }

    fn expect_token(
        &mut self,
        expected: impl FnOnce(&ValueToken<'i>) -> bool,
    ) -> Result<(), BasicParseError<'i>> {
        let location = self.current_source_location();
        if expected(self.next()?) {
            Ok(())
        } else {
            Err(
                location.new_basic_error(BasicParseErrorKind::UnexpectedToken(
                    self.input.cached_lexical(),
                )),
            )
        }
    }
}

pub fn parse_until_before<'i: 't, 't, F, T, E>(
    parser: &mut Parser<'i, 't>,
    delimiters: Delimiters,
    error_behavior: ParseUntilErrorBehavior,
    parse: F,
) -> Result<T, ParseError<'i, E>>
where
    F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
{
    let delimiters = parser.stop_before | delimiters;
    let result;
    {
        let mut delimited = Parser {
            input: parser.input,
            at_start_of: parser.at_start_of.take(),
            stop_before: delimiters,
        };
        result = delimited.parse_entirely(parse);
        if error_behavior == ParseUntilErrorBehavior::Stop && result.is_err() {
            return result;
        }
        if let Some(block) = delimited.at_start_of {
            consume_until_end_of_block(block, &mut delimited.input.tokenizer);
        }
    }

    loop {
        if delimiters.contains(Delimiters::from_byte(parser.input.tokenizer.next_byte())) {
            break;
        }
        match parser.input.tokenizer.next() {
            Ok(token) => {
                if let Some(block) = BlockType::opening(token.token) {
                    consume_until_end_of_block(block, &mut parser.input.tokenizer);
                }
            }
            Err(()) => break,
        }
    }
    result
}

pub fn parse_until_after<'i: 't, 't, F, T, E>(
    parser: &mut Parser<'i, 't>,
    delimiters: Delimiters,
    error_behavior: ParseUntilErrorBehavior,
    parse: F,
) -> Result<T, ParseError<'i, E>>
where
    F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
{
    let result = parse_until_before(parser, delimiters, error_behavior, parse);
    if error_behavior == ParseUntilErrorBehavior::Stop && result.is_err() {
        return result;
    }
    let next = parser.input.tokenizer.next_byte();
    if next.is_some() && !parser.stop_before.contains(Delimiters::from_byte(next)) {
        parser.input.tokenizer.advance(1);
        if next == Some(b'{') {
            consume_until_end_of_block(BlockType::CurlyBracket, &mut parser.input.tokenizer);
        }
    }
    result
}

pub fn parse_nested_block<'i: 't, 't, F, T, E>(
    parser: &mut Parser<'i, 't>,
    parse: F,
) -> Result<T, ParseError<'i, E>>
where
    F: for<'tt> FnOnce(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
{
    let block = parser.at_start_of.take().expect(
        "parse_nested_block must follow a function or an opening parenthesis, bracket, or brace",
    );
    let closing = match block {
        BlockType::CurlyBracket => ClosingDelimiter::CloseCurlyBracket,
        BlockType::SquareBracket => ClosingDelimiter::CloseSquareBracket,
        BlockType::Parenthesis => ClosingDelimiter::CloseParenthesis,
    };
    let result;
    {
        let mut nested = Parser {
            input: parser.input,
            at_start_of: None,
            stop_before: closing,
        };
        result = nested.parse_entirely(parse);
        if let Some(block) = nested.at_start_of {
            consume_until_end_of_block(block, &mut nested.input.tokenizer);
        }
    }
    consume_until_end_of_block(block, &mut parser.input.tokenizer);
    result
}

fn consume_until_end_of_block(block: BlockType, tokenizer: &mut Tokenizer<'_>) {
    let mut stack = Vec::with_capacity(16);
    stack.push(block);
    while let Ok(token) = tokenizer.next() {
        if BlockType::closing(token.token) == stack.last().copied() {
            stack.pop();
            if stack.is_empty() {
                return;
            }
        }
        if let Some(opening) = BlockType::opening(token.token) {
            stack.push(opening);
        }
    }
}
