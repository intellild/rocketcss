//! CSS tokenizer and parser infrastructure.

macro_rules! match_byte {
    ($value:expr, $($rest:tt)*) => {
        match $value {
            $($rest)+
        }
    };
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
pub use rs_css_ast::{Span, Token as ValueToken};
pub use tokenizer::{
    SourceLocation, SourcePosition, Token, TokenAndSpan, Tokenizer, TokenizerState,
};
