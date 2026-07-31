use rocketcss_ast::{
    Atom, Compilation, CssRule, Declaration, DeclarationBlockId, DeclarationBlockStore,
    DeclarationId, RuleId, RuleListId, RuleStore, SelectorList, SelectorListId,
};
use rocketcss_common::{GhostToken, StringPool};

use crate::{
    Error, ParserOptions,
    parser::{ParserCursor, stylesheet::parse_stylesheet},
};

/// Shared state for parsing CSS into one compilation-owned flat IR.
pub struct Compiler<'alloc> {
    pub(crate) string_pool: StringPool,
    pub(crate) cursor: ParserCursor<'alloc>,
    declaration_blocks: DeclarationBlockStore<'alloc>,
    rules: RuleStore<'alloc>,
    current_rule: Option<RuleId>,
    source: &'alloc str,
    source_map_url: Option<&'alloc str>,
}

impl<'alloc> Compiler<'alloc> {
    pub fn new() -> Self {
        Self {
            string_pool: StringPool::new(),
            cursor: ParserCursor::new(""),
            declaration_blocks: DeclarationBlockStore::new(),
            rules: RuleStore::new(),
            current_rule: None,
            source: "",
            source_map_url: None,
        }
    }

    /// Creates a compiler positioned at `source` for parsing an individual CSS value.
    pub fn new_with_source(source: &'alloc str) -> Self {
        Self {
            string_pool: StringPool::new(),
            cursor: ParserCursor::new(source),
            declaration_blocks: DeclarationBlockStore::new(),
            rules: RuleStore::new(),
            current_rule: None,
            source: "",
            source_map_url: None,
        }
    }

    pub fn parse<'ghost>(
        &mut self,
        source: &'alloc str,
        token: &mut GhostToken<'ghost>,
        options: ParserOptions<'alloc>,
    ) -> Result<Compilation<'alloc>, Error<'alloc>> {
        self.cursor = ParserCursor::new(source);
        self.declaration_blocks = DeclarationBlockStore::new();
        self.rules = RuleStore::new();
        self.current_rule = None;
        let stylesheet = parse_stylesheet(self, token, options)?;
        self.source = options.filename;
        self.source_map_url = self.cursor.source_map_url;
        Ok(Compilation::new(
            stylesheet,
            std::mem::take(&mut self.string_pool),
            std::mem::take(&mut self.declaration_blocks),
            std::mem::take(&mut self.rules),
            options.origin,
        ))
    }

    #[inline]
    pub fn string_pool(&self) -> &StringPool {
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

    #[inline]
    pub(crate) fn begin_declaration_block(&mut self) -> DeclarationBlockId {
        self.declaration_blocks.begin_block()
    }

    #[inline]
    pub(crate) fn push_declaration(
        &mut self,
        block: DeclarationBlockId,
        declaration: Declaration<'alloc>,
        important: bool,
    ) -> DeclarationId {
        self.declaration_blocks
            .push_declaration(block, declaration, important)
    }

    #[inline]
    pub(crate) fn declaration_block_is_empty(&self, id: DeclarationBlockId) -> bool {
        self.declaration_blocks.block(id).is_empty()
    }

    #[inline]
    pub(crate) fn begin_rule_list(&mut self) -> RuleListId {
        self.rules.begin_list(self.current_rule)
    }

    #[inline]
    pub(crate) fn push_selector_list(&mut self, selectors: SelectorList<'alloc>) -> SelectorListId {
        self.rules.push_selector_list(selectors)
    }

    #[inline]
    pub(crate) fn first_rule(&self, list: RuleListId) -> Option<RuleId> {
        self.rules.children(list).next().map(|(id, _)| id)
    }

    pub(crate) fn reserve_rule(&mut self, list: RuleListId) -> RuleId {
        self.rules.reserve(list)
    }

    #[inline]
    pub(crate) fn finish_rule(&mut self, id: RuleId, rule: CssRule<'alloc>) {
        self.rules.finish(id, rule);
    }

    #[inline]
    pub(crate) fn rule_mut(&mut self, id: RuleId) -> &mut CssRule<'alloc> {
        self.rules.get_mut(id)
    }

    pub(crate) fn with_current_rule<T>(
        &mut self,
        rule: RuleId,
        parse: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let parent = self.current_rule.replace(rule);
        let result = parse(self);
        self.current_rule = parent;
        result
    }

    pub(crate) fn with_source<T>(
        &mut self,
        source: &'alloc str,
        parse: impl FnOnce(&mut Self) -> T,
    ) -> T {
        let parent = std::mem::replace(&mut self.cursor, ParserCursor::new(source));
        let result = parse(self);
        self.cursor = parent;
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

impl Default for Compiler<'_> {
    fn default() -> Self {
        Self::new()
    }
}
