use crate::{
    CssRule, DeclarationBlockId, DeclarationBlockStore, DefaultAtRule, RuleId, RuleListId,
    RuleStore, SelectorListId, Visit, VisitMut,
};
use rocketcss_common::{DenseId, GhostToken};

/// Shared compilation-store access carried through immutable AST traversal.
pub struct VisitContext<'token, 'ast, 'ghost> {
    token: &'token GhostToken<'ghost>,
    declaration_blocks: Option<&'token DeclarationBlockStore<'ast>>,
    rules: Option<&'token RuleStore<'ast>>,
}

impl<'token, 'ast, 'ghost> VisitContext<'token, 'ast, 'ghost> {
    #[inline]
    pub fn new(token: &'token GhostToken<'ghost>) -> Self {
        Self {
            token,
            declaration_blocks: None,
            rules: None,
        }
    }

    #[inline]
    pub fn new_with_declaration_blocks(
        token: &'token GhostToken<'ghost>,
        declaration_blocks: &'token DeclarationBlockStore<'ast>,
    ) -> Self {
        Self {
            token,
            declaration_blocks: Some(declaration_blocks),
            rules: None,
        }
    }

    #[inline]
    pub fn new_with_stores(
        token: &'token GhostToken<'ghost>,
        declaration_blocks: &'token DeclarationBlockStore<'ast>,
        rules: &'token RuleStore<'ast>,
    ) -> Self {
        Self {
            token,
            declaration_blocks: Some(declaration_blocks),
            rules: Some(rules),
        }
    }

    pub(crate) fn visit_rule_list<V: ?Sized + crate::Visitor<'ast, 'ghost>>(
        &self,
        list: RuleListId,
        visitor: &mut V,
    ) {
        let rules = self
            .rules
            .expect("rule-list traversal requires a compilation store");
        for (_, rule) in rules.children(list) {
            rule.visit(visitor, self);
        }
    }

    pub(crate) fn visit_selector_list<V: ?Sized + crate::Visitor<'ast, 'ghost>>(
        &self,
        list: SelectorListId,
        visitor: &mut V,
    ) {
        let rules = self
            .rules
            .expect("selector traversal requires a compilation store");
        visitor.visit_selector_list(rules.selectors(list), self);
    }

    #[inline]
    pub fn with_declaration_block<R>(
        &self,
        id: DeclarationBlockId,
        f: impl FnOnce(&crate::DeclarationBlock<'ast>, &Self) -> R,
    ) -> R {
        let blocks = self
            .declaration_blocks
            .expect("declaration block traversal requires a compilation store");
        f(blocks.get(id), self)
    }

    pub(crate) fn visit_declaration_block<V: ?Sized + crate::Visitor<'ast, 'ghost>>(
        &self,
        id: DeclarationBlockId,
        visitor: &mut V,
    ) {
        let blocks = self
            .declaration_blocks
            .expect("declaration block traversal requires a compilation store");
        blocks.block(id).visit(visitor, self);
        for (declaration, _) in blocks.block_iter(id) {
            declaration.visit(visitor, self);
        }
        visitor.visit_declaration_block_id(id, self);
    }

    #[inline]
    pub const fn token(&self) -> &'token GhostToken<'ghost> {
        self.token
    }
}

/// Unique compilation-store access carried through mutable AST traversal.
pub struct VisitMutContext<'token, 'ast, 'ghost> {
    marker: std::marker::PhantomData<&'token mut GhostToken<'ghost>>,
    declaration_blocks: DeclarationBlockVisitState<'ast>,
    rules: Option<*mut RuleStore<'ast>>,
    current_rule: Option<RuleId>,
    traverse_rule_lists: bool,
}

enum DeclarationBlockVisitState<'ast> {
    Unavailable,
    Available(*mut DeclarationBlockStore<'ast>),
    Borrowed,
}

struct DeclarationBlockScopeReset<'ast>(*mut DeclarationBlockVisitState<'ast>);
struct RuleStoreScopeReset<'ast>(*mut Option<*mut RuleStore<'ast>>);

struct RulePayloadRestore<'ast> {
    rules: *mut RuleStore<'ast>,
    rule: RuleId,
    payload: Option<CssRule<'ast>>,
}

impl Drop for RulePayloadRestore<'_> {
    fn drop(&mut self) {
        let payload = self
            .payload
            .take()
            .expect("a detached rule payload is restored exactly once");
        // SAFETY: the context holds the exclusive RuleStore borrow for the
        // traversal, and this guard restores the same slot before another
        // rule payload is detached or the context is released.
        *unsafe { &mut *self.rules }.get_mut(self.rule) = payload;
    }
}

struct CurrentRuleReset(*mut Option<RuleId>, Option<RuleId>);

impl Drop for CurrentRuleReset {
    fn drop(&mut self) {
        // SAFETY: the guard does not outlive the VisitMutContext field and
        // restores the previous nested traversal state during unwinding.
        unsafe { *self.0 = self.1 };
    }
}

impl Drop for DeclarationBlockScopeReset<'_> {
    fn drop(&mut self) {
        // SAFETY: the guard never outlives the `VisitMutContext` field it points
        // to, and the callback cannot move that context while it is borrowed.
        unsafe { *self.0 = DeclarationBlockVisitState::Unavailable };
    }
}

impl Drop for RuleStoreScopeReset<'_> {
    fn drop(&mut self) {
        // SAFETY: the guard points to the context field that created it.
        unsafe { *self.0 = None };
    }
}

impl<'token, 'ast, 'ghost> VisitMutContext<'token, 'ast, 'ghost> {
    #[inline]
    pub fn new(_token: &'token mut GhostToken<'ghost>) -> Self {
        Self {
            marker: std::marker::PhantomData,
            declaration_blocks: DeclarationBlockVisitState::Unavailable,
            rules: None,
            current_rule: None,
            traverse_rule_lists: true,
        }
    }

    #[inline]
    pub fn new_with_declaration_blocks(
        _token: &'token mut GhostToken<'ghost>,
        declaration_blocks: &'token mut DeclarationBlockStore<'ast>,
    ) -> Self {
        Self {
            marker: std::marker::PhantomData,
            declaration_blocks: DeclarationBlockVisitState::Available(declaration_blocks),
            rules: None,
            current_rule: None,
            traverse_rule_lists: true,
        }
    }

    #[inline]
    pub fn new_with_stores(
        _token: &'token mut GhostToken<'ghost>,
        declaration_blocks: &'token mut DeclarationBlockStore<'ast>,
        rules: &'token mut RuleStore<'ast>,
    ) -> Self {
        Self {
            marker: std::marker::PhantomData,
            declaration_blocks: DeclarationBlockVisitState::Available(declaration_blocks),
            rules: Some(rules),
            current_rule: None,
            traverse_rule_lists: true,
        }
    }

    #[inline]
    pub fn new_with_stores_flat(
        _token: &'token mut GhostToken<'ghost>,
        declaration_blocks: &'token mut DeclarationBlockStore<'ast>,
        rules: &'token mut RuleStore<'ast>,
    ) -> Self {
        Self {
            marker: std::marker::PhantomData,
            declaration_blocks: DeclarationBlockVisitState::Available(declaration_blocks),
            rules: Some(rules),
            current_rule: None,
            traverse_rule_lists: false,
        }
    }

    pub(crate) fn visit_rule_list<V: ?Sized + crate::VisitorMut<'ast, 'ghost>>(
        &mut self,
        list: RuleListId,
        visitor: &mut V,
    ) {
        if !self.traverse_rule_lists {
            return;
        }
        let rules = self
            .rules
            .expect("rule-list traversal requires a compilation store");
        // Collect IDs before callbacks. Payload visitors may edit fields but
        // cannot grow or structurally rewrite the rule store.
        let ids = unsafe { &*rules }
            .children(list)
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>();
        for id in ids {
            self.visit_rule(id, visitor);
        }
    }

    pub(crate) fn visit_selector_list<V: ?Sized + crate::VisitorMut<'ast, 'ghost>>(
        &mut self,
        list: SelectorListId,
        visitor: &mut V,
    ) {
        let rules = self
            .rules
            .expect("selector traversal requires a compilation store");
        // SAFETY: structural rule growth is unavailable during visitor
        // callbacks. Selector ranges are fixed-size slots in the same store.
        visitor.visit_selector_list(unsafe { &mut *rules }.selectors_mut(list), self);
    }

    pub fn visit_rule<V: ?Sized + crate::VisitorMut<'ast, 'ghost>>(
        &mut self,
        id: RuleId,
        visitor: &mut V,
    ) {
        let rules = self
            .rules
            .expect("rule traversal requires a compilation store");
        let payload = std::mem::replace(
            unsafe { &mut *rules }.get_mut(id),
            CssRule::Custom(DefaultAtRule),
        );
        let mut restore = RulePayloadRestore {
            rules,
            rule: id,
            payload: Some(payload),
        };
        let previous = self.current_rule.replace(id);
        let _current_reset = CurrentRuleReset(&mut self.current_rule, previous);
        restore
            .payload
            .as_mut()
            .expect("the detached payload is available during traversal")
            .visit_mut(visitor, self);
    }

    #[inline]
    pub fn current_rule(&self) -> RuleId {
        self.current_rule
            .expect("rule-local traversal has a current RuleId")
    }

    pub fn with_selector_list<R>(
        &mut self,
        id: SelectorListId,
        callback: impl FnOnce(&mut [crate::Selector<'ast>]) -> R,
    ) -> R {
        let rules = self
            .rules
            .expect("selector mutation requires a compilation store");
        // SAFETY: visitor callbacks cannot grow or structurally rewrite the
        // rule store, and the selector list owns a fixed dense range.
        callback(unsafe { &mut *rules }.selectors_mut(id))
    }

    #[inline]
    pub fn rule_store(&self) -> &RuleStore<'ast> {
        // SAFETY: rule payloads are detached before callbacks run. Consumers
        // may inspect already-restored preorder predecessors but cannot mutate
        // or grow the store through this shared view.
        unsafe {
            &*self
                .rules
                .expect("rule inspection requires a compilation store")
        }
    }

    #[inline]
    pub fn with_declaration_block<R>(
        &mut self,
        id: DeclarationBlockId,
        f: impl FnOnce(&mut crate::DeclarationBlock<'ast>, &mut Self) -> R,
    ) -> R {
        let DeclarationBlockVisitState::Available(blocks) = std::mem::replace(
            &mut self.declaration_blocks,
            DeclarationBlockVisitState::Borrowed,
        ) else {
            panic!("declaration block traversal requires an available compilation store");
        };
        // SAFETY: `Available` is installed only from a live exclusive borrow.
        // Replacing it with `Borrowed` prevents nested access while `f` runs.
        let blocks = unsafe { &mut *blocks };
        let result = f(blocks.get_mut(id), self);
        self.declaration_blocks = DeclarationBlockVisitState::Available(blocks);
        result
    }

    pub(crate) fn visit_declaration_block<V: ?Sized + crate::VisitorMut<'ast, 'ghost>>(
        &mut self,
        id: DeclarationBlockId,
        visitor: &mut V,
    ) {
        let DeclarationBlockVisitState::Available(blocks) = std::mem::replace(
            &mut self.declaration_blocks,
            DeclarationBlockVisitState::Borrowed,
        ) else {
            panic!("declaration block traversal requires an available compilation store");
        };
        // SAFETY: the state stores a live exclusive borrow and remains marked
        // `Borrowed` until every header/declaration callback has completed.
        let blocks = unsafe { &mut *blocks };
        blocks.block_mut(id).visit_mut(visitor, self);
        let declaration_ids = blocks
            .block(id)
            .ranges()
            .iter()
            .flat_map(|range| range.as_usize_range())
            .map(|index| {
                crate::DeclarationId::from_index(index)
                    .expect("a declaration block range contains valid IDs")
            })
            .collect::<std::vec::Vec<_>>();
        for declaration in declaration_ids {
            blocks.declaration_mut(declaration).visit_mut(visitor, self);
        }
        self.declaration_blocks = DeclarationBlockVisitState::Available(blocks);
        visitor.visit_declaration_block_id(id, self);
    }

    pub fn with_declaration_block_store<R>(
        &mut self,
        callback: impl FnOnce(&mut DeclarationBlockStore<'ast>, &mut Self) -> R,
    ) -> R {
        let DeclarationBlockVisitState::Available(blocks) = std::mem::replace(
            &mut self.declaration_blocks,
            DeclarationBlockVisitState::Borrowed,
        ) else {
            panic!("declaration block access requires an available compilation store");
        };
        // SAFETY: the state stores a live exclusive borrow and remains marked
        // `Borrowed` until the callback returns.
        let blocks = unsafe { &mut *blocks };
        let result = callback(blocks, self);
        self.declaration_blocks = DeclarationBlockVisitState::Available(blocks);
        result
    }

    pub fn with_declaration_blocks<R>(
        &mut self,
        declaration_blocks: &mut DeclarationBlockStore<'ast>,
        callback: impl FnOnce(&mut Self) -> R,
    ) -> R {
        assert!(
            matches!(
                self.declaration_blocks,
                DeclarationBlockVisitState::Unavailable
            ),
            "declaration block store is already available"
        );
        self.declaration_blocks = DeclarationBlockVisitState::Available(declaration_blocks);
        let _reset = DeclarationBlockScopeReset(&mut self.declaration_blocks);
        callback(self)
    }

    pub fn with_stores<R>(
        &mut self,
        declaration_blocks: &mut DeclarationBlockStore<'ast>,
        rules: &mut RuleStore<'ast>,
        callback: impl FnOnce(&mut Self) -> R,
    ) -> R {
        assert!(self.rules.is_none(), "rule store is already available");
        self.rules = Some(rules);
        let _rules_reset = RuleStoreScopeReset(&mut self.rules);
        self.with_declaration_blocks(declaration_blocks, callback)
    }
}
