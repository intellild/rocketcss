use rocketcss_ast::{Atom, StyleSheet};
use rocketcss_common::{Allocator, GhostToken, StringPool};

use crate::{
    Error, ParserOptions,
    parser::{DeclarationTokenReplay, ParserCursor},
};

/// Shared state for parsing CSS into one arena-owned stylesheet.
pub struct Compiler<'alloc> {
    pub(crate) allocator: &'alloc Allocator,
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
    ) -> Result<StyleSheet<'alloc>, Error<'alloc>> {
        let stylesheet = self.parse_stylesheet(source, options)?;
        self.source = options.filename;
        self.source_map_url = self.cursor.source_map_url;
        Ok(stylesheet)
    }

    #[inline]
    pub fn allocator(&self) -> &'alloc Allocator {
        self.allocator
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
