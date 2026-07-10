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
mod stylesheet;
mod tokenizer;
mod traits;
mod value;

pub use escape::unescape;
pub use parser::{
    BasicParseError, BasicParseErrorKind, Delimiter, Delimiters, ParseError, ParseErrorKind,
    ParseUntilErrorBehavior, Parser, ParserInput, ParserState,
};
pub use rs_css_ast::{Span, Token as ValueToken};
pub use stylesheet::parse;
pub use tokenizer::{
    SourceLocation, SourcePosition, Token, TokenAndSpan, Tokenizer, TokenizerState,
};
pub use traits::{Error, Parse, ParserError, ParserOptions};
