use rocketcss_ast::{AstContext, AstVec, Atom, NodeId};
use rocketcss_common::{Allocator, GhostToken, StringPool, vec::Vec};

use crate::{
    Error, ParserOptions,
    parser::{DeclarationTokenReplay, ParserCursor},
};

/// Shared state for parsing CSS into one arena-owned compilation.
pub struct Compiler<'alloc> {
    pub(crate) allocator: &'alloc Allocator,
    pub(crate) compilation: AstContext<'alloc>,
    pub(crate) string_pool: StringPool<'alloc>,
    pub(crate) cursor: ParserCursor<'alloc>,
    pub(crate) replay: DeclarationTokenReplay<'alloc>,
    source: &'alloc str,
    source_map_url: Option<&'alloc str>,
}

impl<'alloc> Compiler<'alloc> {
    pub fn new(allocator: &'alloc Allocator) -> Self {
        Self {
            allocator,
            compilation: AstContext::new_in(allocator),
            string_pool: StringPool::new_in(allocator),
            cursor: ParserCursor::new(""),
            replay: DeclarationTokenReplay::new(allocator),
            source: "",
            source_map_url: None,
        }
    }

    /// Creates a compiler positioned at `source` for parsing an individual CSS value.
    pub fn new_with_source(source: &'alloc str, allocator: &'alloc Allocator) -> Self {
        Self {
            allocator,
            compilation: AstContext::new_in(allocator),
            string_pool: StringPool::new_in(allocator),
            cursor: ParserCursor::new(source),
            replay: DeclarationTokenReplay::new(allocator),
            source: "",
            source_map_url: None,
        }
    }

    pub fn parse<'ghost>(
        &mut self,
        source: &'alloc str,
        _token: &mut GhostToken<'ghost>,
        options: ParserOptions<'alloc>,
    ) -> Result<AstContext<'alloc>, Error<'alloc>> {
        let compilation = self.parse_compilation(source, options)?;
        self.source = options.filename;
        self.source_map_url = self.cursor.source_map_url;
        Ok(compilation)
    }

    #[inline]
    pub fn allocator(&self) -> &'alloc Allocator {
        self.allocator
    }

    /// Returns the AST context that owns every node allocated by this parser.
    #[inline]
    pub fn ast_context(&self) -> &AstContext<'alloc> {
        &self.compilation
    }

    /// Returns the AST context that owns every node allocated by this parser.
    #[inline]
    pub fn ast_context_mut(&mut self) -> &mut AstContext<'alloc> {
        &mut self.compilation
    }

    /// Finishes value parsing and transfers ownership of the node context to the caller.
    #[inline]
    pub fn into_ast_context(self) -> AstContext<'alloc> {
        self.compilation
    }

    #[inline]
    pub fn string_pool(&self) -> &StringPool<'alloc> {
        &self.string_pool
    }

    #[inline]
    pub fn intern(&mut self, value: &str) -> Atom<'alloc> {
        self.string_pool.intern(value)
    }

    #[inline]
    pub fn intern_ascii_lowercase(&mut self, value: &str) -> Atom<'alloc> {
        self.string_pool.intern_ascii_lowercase(value)
    }

    pub(crate) fn with_source<T>(
        &mut self,
        source: &'alloc str,
        parse: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let parent = std::mem::replace(&mut self.cursor, ParserCursor::new(source));
        let saved_replay = std::mem::replace(
            &mut self.replay,
            DeclarationTokenReplay::new(self.allocator),
        );
        let result = parse(self);
        self.cursor = parent;
        self.replay = saved_replay;
        result
    }

    #[inline]
    pub fn source(&self) -> &'alloc str {
        self.source
    }

    #[inline]
    pub fn source_map_url(&self) -> Option<&'alloc str> {
        self.source_map_url
    }
}

/// Stores a parsed value after its construction has released any temporary compiler borrow.
#[inline]
pub(crate) fn store_node<'alloc, T: 'alloc + rocketcss_ast::AstNodeStorage<'alloc>>(
    value: T,
    input: &mut Compiler<'alloc>,
) -> NodeId<'alloc, T> {
    let span = input.current_token_span().unwrap_or_default();
    input.ast_context_mut().alloc_node(value, span)
}

/// Commits a completed construction-time list to the AST context.
#[inline]
pub(crate) fn store_vec<'alloc, T: 'alloc + Unpin + rocketcss_ast::ExtraDataCompact<'alloc>>(
    values: Vec<'alloc, T>,
    input: &mut Compiler<'alloc>,
) -> AstVec<'alloc, T> {
    input.ast_context_mut().alloc_vec(values)
}

/// Stores each parsed node, then commits their dense IDs as one persistent range.
#[inline]
pub(crate) fn store_node_vec<'alloc, T: 'alloc + Unpin + rocketcss_ast::AstNodeStorage<'alloc>>(
    values: Vec<'alloc, T>,
    input: &mut Compiler<'alloc>,
) -> AstVec<'alloc, NodeId<'alloc, T>> {
    let mut ids = input.ast_context().allocator().vec();
    for value in values {
        ids.push(store_node(value, input));
    }
    store_vec(ids, input)
}
