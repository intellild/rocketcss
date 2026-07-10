//! CSS tokenizer and parser infrastructure.

macro_rules! match_byte {
    ($value:expr, $($rest:tt)*) => {
        match $value {
            $($rest)+
        }
    };
}

mod escape;
mod tokenizer;

pub use escape::unescape;
pub use rs_css_ast::Span;
pub use tokenizer::{ParserState, SourceLocation, SourcePosition, Token, TokenAndSpan, Tokenizer};
