pub(crate) use crate::lexer::{LengthUnit as OldLengthUnit, Token as OldToken, Unit as OldUnit};
pub(crate) use crate::match_ignore_ascii_case;
pub(crate) use crate::parser::{
    BasicParseErrorKind, Delimiter, Error, Parse, ParseError, ParserError, ParserOptions,
};
pub(crate) use crate::prelude::LexicalToken;
pub(crate) use crate::{Compiler, Tokenizer};
pub(crate) use rocketcss_ast_next::*;

/// Converts a parser position into a flattened-AST span.
#[inline]
pub(crate) fn span_from(start: &crate::ParserState, end: crate::SourcePosition) -> Span {
    Span::new(
        start.position().byte_index() as u32,
        end.byte_index() as u32,
    )
}

/// Converts a tokenizer span into a flattened-AST span.
#[inline]
pub(crate) fn from_span(span: crate::Span) -> Span {
    Span::new(span.start, span.end)
}

/// Returns the span of the most recently consumed token.
#[inline]
pub(crate) fn token_span<'i>(input: &Compiler<'i>) -> Span {
    input
        .current_token_span()
        .map(from_span)
        .unwrap_or(DUMMY_SP)
}

/// Evaluates the token span before running an allocation expression.
macro_rules! with_token_span {
    ($input:ident, |$span:ident| $body:expr) => {{
        let $span = token_span($input);
        $body
    }};
}
pub(crate) use with_token_span;

/// Enforces the shared maximum selector/value nesting depth.
#[inline]
pub(crate) fn check_depth<'i>(
    input: &Compiler<'i>,
    depth: usize,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    if depth > MAX_NESTING_DEPTH {
        Err(input.new_custom_error(ParserError::MaximumNestingDepth))
    } else {
        Ok(())
    }
}

pub(crate) const MAX_NESTING_DEPTH: usize = 500;
