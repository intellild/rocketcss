//! CSS tokenizer and parser infrastructure.

macro_rules! match_byte {
    ($value:expr, $($rest:tt)*) => {
        match $value {
            $($rest)+
        }
    };
}

mod compiler;
mod escape;
mod parser;
pub mod prelude;
mod tokenizer;

pub use compiler::Compiler;
pub use escape::unescape;
pub use parser::stylesheet::parse;
pub use parser::{
    BasicParseError, BasicParseErrorKind, Delimiter, Delimiters, Error, Parse, ParseError,
    ParseErrorKind, ParseUntilErrorBehavior, ParserError, ParserOptions, ParserState,
};
pub use rocketcss_ast::{Span, Token as ValueToken};
pub use tokenizer::{
    SourceLocation, SourcePosition, Token, TokenAndSpan, Tokenizer, TokenizerState,
};

#[cfg(test)]
mod tests {
    use rocketcss_ast::{DUMMY_SP, match_ignore_ascii_case};
    use rocketcss_common::Allocator;

    use crate::Compiler;

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

    #[test]
    fn failed_speculative_parses_roll_back_node_allocations() {
        let allocator = Allocator::new();
        let mut compiler = Compiler::new_with_source("", &allocator);

        for value in 0_u8..4 {
            let result: Result<(), ()> = compiler.try_parse(|compiler| {
                compiler.ast_context_mut().alloc_node(value, DUMMY_SP);
                Err(())
            });
            assert_eq!(result, Err(()));
        }

        let committed = compiler.ast_context_mut().alloc_node(4_u8, DUMMY_SP);
        assert_eq!(committed.index(), 0);
        assert_eq!(*compiler.ast_context().node(committed), 4);
    }
}
