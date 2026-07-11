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

pub use escape::unescape;
pub use parser::stylesheet::parse;
pub use parser::{
    BasicParseError, BasicParseErrorKind, Delimiter, Delimiters, Error, Parse, ParseError,
    ParseErrorKind, ParseUntilErrorBehavior, Parser, ParserError, ParserInput, ParserOptions,
    ParserState,
};
pub use rocketcss_ast::{Span, Token as ValueToken};
pub use tokenizer::{
    SourceLocation, SourcePosition, Token, TokenAndSpan, Tokenizer, TokenizerState,
};

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
