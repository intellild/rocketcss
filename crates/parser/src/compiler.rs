use rocketcss_allocator::{Allocator, GhostToken, StringPool};
use rocketcss_ast::StyleSheet;

use crate::{Error, ParserOptions, parser::stylesheet::parse_with_string_pool};

/// Shared state for parsing CSS into one arena-owned compilation.
pub struct Compiler<'alloc> {
    allocator: &'alloc Allocator,
    string_pool: &'alloc StringPool<'alloc>,
    source: &'alloc str,
    source_map_url: Option<&'alloc str>,
}

impl<'alloc> Compiler<'alloc> {
    pub fn new(allocator: &'alloc Allocator) -> Self {
        Self {
            allocator,
            string_pool: allocator.alloc(StringPool::new_in(allocator)),
            source: "",
            source_map_url: None,
        }
    }

    pub fn parse<'ghost>(
        &mut self,
        source: &'alloc str,
        token: &mut GhostToken<'ghost>,
        options: ParserOptions<'alloc>,
    ) -> Result<StyleSheet<'alloc, 'ghost>, Error<'alloc>> {
        let parsed =
            parse_with_string_pool(source, self.allocator, self.string_pool, token, options)?;
        self.source = options.filename;
        self.source_map_url = parsed.source_map_url;
        Ok(parsed.stylesheet)
    }

    #[inline]
    pub fn string_pool(&self) -> &'alloc StringPool<'alloc> {
        self.string_pool
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
