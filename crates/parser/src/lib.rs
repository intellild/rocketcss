//! CSS tokenizer and parser infrastructure.

macro_rules! match_byte {
    ($value:expr, $($rest:tt)*) => {
        match $value {
            $($rest)+
        }
    };
}

macro_rules! match_ignore_ascii_case {
    (
        $value:expr,
        $($($expected:literal)|+ => $result:expr,)+
        _ => $fallback:expr $(,)?
    ) => {{
        let value = $value;
        $(
            if $(value.eq_ignore_ascii_case($expected))||+ {
                $result
            } else
        )+
        {
            $fallback
        }
    }};
}

mod escape;
mod parser;
pub mod prelude;
mod tokenizer;

use rocketcss_allocator::Allocator;
use rocketcss_ast::StyleSheet;

pub use escape::unescape;
pub use parser::{
    BasicParseError, BasicParseErrorKind, Delimiter, Delimiters, Error, Parse, ParseError,
    ParseErrorKind, ParseUntilErrorBehavior, Parser as TokenParser, ParserError, ParserInput,
    ParserOptions, ParserState,
};
pub use rocketcss_ast::{Span, Token as ValueToken};
pub use tokenizer::{
    SourceLocation, SourcePosition, Token, TokenAndSpan, Tokenizer, TokenizerState,
};

/// A stylesheet parser whose inputs share one lifetime.
///
/// Keeping the source, allocator, and options together makes it possible for
/// every reference in the returned AST to use the same lifetime.
pub struct Parser<'a> {
    source: &'a str,
    allocator: &'a Allocator,
    options: ParserOptions<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, allocator: &'a Allocator, options: ParserOptions<'a>) -> Self {
        Self {
            source,
            allocator,
            options,
        }
    }

    pub fn parse(&self) -> Result<StyleSheet<'a>, Error<'a>> {
        parser::stylesheet::parse(self.source, self.allocator, self.options)
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn allocator(&self) -> &'a Allocator {
        self.allocator
    }

    pub fn options(&self) -> &ParserOptions<'a> {
        &self.options
    }
}

/// Parses a stylesheet using a temporary [`Parser`].
pub fn parse<'a>(
    source: &'a str,
    allocator: &'a Allocator,
    options: ParserOptions<'a>,
) -> Result<StyleSheet<'a>, Error<'a>> {
    Parser::new(source, allocator, options).parse()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ascii_case_match_evaluates_input_once_and_supports_aliases() {
        let mut evaluations = 0;
        let result = match_ignore_ascii_case!(
            {
                evaluations += 1;
                "ScReEn"
            },
            "all" => 0,
            "print" | "screen" => 1,
            _ => 2,
        );

        assert_eq!(evaluations, 1);
        assert_eq!(result, 1);
    }
}
