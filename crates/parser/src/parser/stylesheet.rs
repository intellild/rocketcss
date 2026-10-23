use rocketcss_ast_next::Compilation;
use rocketcss_common::{Allocator, GhostToken};

use crate::{Compiler, Error, ParserOptions};

/// Parses a stylesheet into the flattened AST.
pub fn parse<'i, 'ghost>(
    source: &'i str,
    allocator: &'i Allocator,
    token: &mut GhostToken<'ghost>,
    options: ParserOptions<'i>,
) -> Result<Compilation<'i>, Error<'i>> {
    Compiler::new(allocator).parse(source, token, options)
}
