use std::fmt;

use rocketcss_allocator::Allocator;

use super::{ParseError, Parser, ParserInput};
use crate::{SourceLocation, TokenAndSpan};

/// Grammar-level parser errors, modeled after lightningcss' parser errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParserError<'i> {
    InvalidRule,
    InvalidDeclaration,
    InvalidSelector,
    InvalidValue,
    InvalidAtRule(&'i str),
    UnexpectedImportRule,
    UnexpectedNamespaceRule,
    UnexpectedToken(TokenAndSpan),
    MaximumNestingDepth,
}

impl fmt::Display for ParserError<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRule => formatter.write_str("invalid CSS rule"),
            Self::InvalidDeclaration => formatter.write_str("invalid CSS declaration"),
            Self::InvalidSelector => formatter.write_str("invalid selector"),
            Self::InvalidValue => formatter.write_str("invalid value"),
            Self::InvalidAtRule(name) => write!(formatter, "invalid at-rule: @{name}"),
            Self::UnexpectedImportRule => formatter.write_str(
                "@import rules must precede all rules aside from @charset and initial @layer statements",
            ),
            Self::UnexpectedNamespaceRule => formatter.write_str(
                "@namespace rules must precede all rules aside from @charset, @import, and initial @layer statements",
            ),
            Self::UnexpectedToken(token) => write!(formatter, "unexpected token: {token:?}"),
            Self::MaximumNestingDepth => formatter.write_str("maximum nesting depth exceeded"),
        }
    }
}

impl std::error::Error for ParserError<'_> {}

/// A parse error with filename and source location.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error<'i> {
    pub kind: ParserError<'i>,
    pub filename: &'i str,
    pub location: SourceLocation,
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at {}:{}:{}",
            self.kind, self.filename, self.location.line, self.location.column
        )
    }
}

impl std::error::Error for Error<'_> {}

/// CSS parsing options shared by stylesheet and value parsers.
#[derive(Clone, Copy, Debug, Default)]
pub struct ParserOptions<'i> {
    pub filename: &'i str,
    pub error_recovery: bool,
}

/// A lightningcss-style trait for values parsed from CSS syntax.
pub trait Parse<'i>: Sized {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>>;

    fn parse_string(
        source: &'i str,
        allocator: &'i Allocator,
    ) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut input = ParserInput::new(source, allocator);
        let mut parser = Parser::new(&mut input);
        parser.parse_entirely(Self::parse)
    }
}

impl<'i, T: Parse<'i>> Parse<'i> for Option<T> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        Ok(input.try_parse(T::parse).ok())
    }
}
