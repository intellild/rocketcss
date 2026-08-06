use super::*;

impl<R: Unpin, D: Unpin, K> StyleSheet<'_, R, D, K> {
    pub(super) fn is_valid_direct_rule_context(&self, context: DirectRuleContext<R>) -> bool {
        self.rules
            .get(context.rule)
            .is_some_and(|rule| rule.live && rule.parent == context.parent)
    }

    pub(super) fn resolve_direct_rule_window(
        &self,
        context: DirectRuleContext<R>,
    ) -> Result<DirectRuleWindow<R>, MutationError<R>> {
        if !self.is_valid_direct_rule_context(context) {
            return Err(MutationError::<R>::InvalidRuleTopology(context.rule));
        }
        self.direct_rule_windows(context.parent)?
            .find(|candidate| candidate.rule == context.rule)
            .ok_or(MutationError::<R>::InvalidRuleTopology(context.rule))
    }
}

impl<R, D: Unpin, K> StyleSheet<'_, R, D, K>
where
    R: RuleIdReferences<R> + Unpin,
    K: RuleIdReferences<R> + Copy + Eq + std::hash::Hash,
{
    /// Inserts a new direct sibling after `after` at its final Radix ID.
    ///
    /// The physical insertion anchor is the tail of `after`'s complete
    /// subtree plus any retired arena entries before the next live
    /// rule. This preserves global lexical preorder without reusing IDs.
    #[cfg(test)]
    pub(super) fn insert_rule_after(
        &mut self,
        context: DirectRuleContext<R>,
        payload: R,
    ) -> Result<InsertedRule<R>, MutationError<R>> {
        let context = self.resolve_direct_rule_window(context)?;
        let parent = context.parent;
        if !self
            .rules
            .can_insert_between(context.insertion_anchor, context.storage_before)
        {
            return Err(MutationError::<R>::LocalRuleCapacityExhausted(
                context.insertion_anchor,
            ));
        }

        self.authored_declaration_append = None;
        let result = self.rules.insert_between(
            context.insertion_anchor,
            context.storage_before,
            RuleRecord {
                payload,
                parent,
                descendant_count: 0,
                nested_rule_count: 0,
                subtree_last: None,
                declaration_block: None,
                live: true,
            },
        );
        self.repair_rule_id_remaps(&result.remaps);

        let context = context.remapped_with(|id| remap_rule_id(id, &result.remaps));
        let parent = parent.map(|id| remap_rule_id(id, &result.remaps));
        let inserted = result.id;
        let direct_parent = parent;
        let mut ancestor = parent;
        while let Some(id) = ancestor {
            let rule = self
                .rules
                .get_mut(id)
                .expect("an insertion ancestor remains resolvable after ID repair");
            rule.descendant_count += 1;
            if Some(id) == direct_parent {
                rule.nested_rule_count += 1;
            }
            if rule.subtree_last.unwrap_or(id) == context.insertion_anchor {
                rule.subtree_last = Some(inserted);
            }
            ancestor = rule.parent;
        }

        Ok(InsertedRule { rule: result })
    }

    pub(super) fn repair_rule_id_remaps(&mut self, remaps: &[RadixIdRemap<RuleId<R>>]) {
        if remaps.is_empty() {
            return;
        }
        self.rules.for_each_enumerated_mut(|_, rule| {
            rule.parent = rule.parent.map(|id| remap_rule_id(id, remaps));
            rule.subtree_last = rule.subtree_last.map(|id| remap_rule_id(id, remaps));
            rule.payload.remap_rule_ids(remaps);
        });

        self.declaration_blocks.for_each_enumerated_mut(|_, block| {
            let DeclarationBlockOwner::<R>::Rule(owner) = &mut block.owner;
            *owner = remap_rule_id(*owner, remaps);
        });
        for key in self.effective_keys.iter_mut() {
            key.remap_rule_ids(remaps);
        }
        self.effective_key_ids.clear();
        for (id, key) in self.effective_keys.iter_enumerated() {
            self.effective_key_ids.insert(*key, id);
        }

        for value in self.context_values.iter_mut() {
            value.representative = remap_rule_id(value.representative, remaps);
        }
        for bucket in self.context_value_buckets.values_mut() {
            for state in bucket {
                state.representative = remap_rule_id(state.representative, remaps);
            }
        }
        for layer in self.layer_contexts.iter_mut() {
            layer.occurrence = remap_rule_id(layer.occurrence, remaps);
        }
        self.layer_context_ids.clear();
        for (id, layer) in self.layer_contexts.iter_enumerated() {
            self.layer_context_ids.insert(
                LayerContextKey {
                    parent: layer.parent,
                    occurrence: layer.occurrence,
                },
                id,
            );
        }
    }

    pub(super) fn first_declaration_block_after_rule(
        &self,
        edge: DirectRuleEdge<R>,
    ) -> Result<DeclarationBlockId<R>, MutationError<R>> {
        let after = edge.left();
        let before_or_at = edge.right();
        self.rules
            .get(before_or_at)
            .ok_or(MutationError::<R>::UnknownRule(before_or_at))?;
        if !self.is_valid_direct_rule_edge(edge) {
            return Err(MutationError::<R>::InvalidRuleTopology(after));
        }
        let bridge = self.resolve_direct_rule_window(edge.left_context())?.bridge;
        let bridge_ids = self
            .rules
            .ids_in_range(bridge)
            .ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
        for rule in bridge_ids.chain(std::iter::once(before_or_at)) {
            let record = self
                .rules
                .get(rule)
                .ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
            if let Some(block) = record.declaration_block {
                return Ok(block);
            }
        }
        Err(MutationError::<R>::InvalidRuleTopology(after))
    }
}

impl<R: Unpin, D: Unpin, K> StyleSheet<'_, R, D, K> {
    pub(super) fn can_insert_declaration_range_between(
        &self,
        after: Option<DeclarationId>,
        before: Option<DeclarationId>,
        len: u32,
    ) -> bool {
        if len == 0 {
            return true;
        }
        match after {
            Some(after) => self
                .declarations
                .can_insert_stable_range_between(after, before, len),
            None => {
                before.is_none()
                    && self.declarations.is_empty()
                    && self.declarations.can_push_primary_range(len)
            }
        }
    }

    /// Refreshes the current block-local portion of an append cursor after a
    /// payload-only declaration or effective-key mutation.
    #[doc(hidden)]
    fn refresh_declaration_append_context(
        &self,
        context: DeclarationAppendContext<R>,
    ) -> Result<DeclarationAppendContext<R>, MutationError<R>> {
        let block = context.position.block;
        let record = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?;
        if !record.live {
            return Err(MutationError::<R>::UnknownDeclarationBlock(block));
        }
        let after = if record.declarations.is_empty() {
            context.position.previous_non_empty_tail
        } else {
            Some(record.declarations.last_id())
        };
        Ok(DeclarationAppendContext {
            position: DeclarationBlockPosition {
                live: record.live,
                declarations: record.declarations,
                ..context.position
            },
            after,
            before: context.before,
        })
    }

    /// Inserts a complete transformed declaration batch at the semantic end of
    /// `block`. The batch is preflighted before the declaration arena or block
    /// range is changed, and no existing declaration ID is relabeled.
    pub fn insert_transformed_declarations_at_block_end<Values>(
        &mut self,
        block: DeclarationBlockId<R>,
        declarations: Values,
    ) -> Result<(), MutationError<R>>
    where
        Values: IntoIterator<Item = (D, bool)>,
        Values::IntoIter: ExactSizeIterator,
    {
        let context = self.declaration_append_context(block)?;
        self.insert_transformed_declarations_with_context(context, declarations)
            .map(|_| ())
    }

    /// Context-consuming form used by local mutation schedulers.
    #[doc(hidden)]
    pub(super) fn insert_transformed_declarations_with_context<Values>(
        &mut self,
        context: DeclarationAppendContext<R>,
        declarations: Values,
    ) -> Result<DeclarationAppendContext<R>, MutationError<R>>
    where
        Values: IntoIterator<Item = (D, bool)>,
        Values::IntoIter: ExactSizeIterator,
    {
        let declarations = declarations.into_iter();
        let len = u32::try_from(declarations.len())
            .map_err(|_| MutationError::<R>::DeclarationCapacityExhausted)?;
        let block = context.position.block;
        let block_record = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::<R>::UnknownDeclarationBlock(block));
        }
        if block_record.declarations != context.position.declarations {
            return Err(MutationError::<R>::NonContiguousDeclarationRange(block));
        }
        if len == 0 {
            return Ok(context);
        }
        let existing = block_record.declarations;
        if !self.can_insert_declaration_range_between(context.after, context.before, len) {
            return Err(MutationError::<R>::DeclarationCapacityExhausted);
        }
        let records =
            declarations.map(|(payload, important)| DeclarationRecord { payload, important });
        self.authored_declaration_append = None;
        let inserted = match context.after {
            Some(after) => {
                self.declarations
                    .insert_stable_range_between(after, context.before, records)
            }
            None => self.declarations.push_primary_range(records),
        };
        let block = self
            .declaration_blocks
            .get_mut(block)
            .expect("the transformed declaration owner was validated");
        if existing.is_empty() {
            block.declarations = inserted;
        } else {
            debug_assert_eq!(block.declarations, existing);
            block.declarations.extend(inserted);
        }
        Ok(DeclarationAppendContext {
            position: DeclarationBlockPosition {
                declarations: block.declarations,
                previous_non_empty_tail: Some(block.declarations.last_id()),
                ..context.position
            },
            after: Some(block.declarations.last_id()),
            before: context.before,
        })
    }

    /// Inserts a synthesized declaration block at its final semantic block ID
    /// and binds it to an already inserted live owner rule.
    #[cfg(test)]
    pub(crate) fn insert_declaration_block_between(
        &mut self,
        after: DeclarationBlockId<R>,
        before: Option<DeclarationBlockId<R>>,
        owner: RuleId<R>,
        effective_key: EffectiveKeyId,
    ) -> Result<rocketcss_common::RadixInsertResult<DeclarationBlockId<R>>, MutationError<R>> {
        let owner_record = self
            .rules
            .get(owner)
            .ok_or(MutationError::<R>::UnknownRule(owner))?;
        if !owner_record.live {
            return Err(MutationError::<R>::RetiredRule(owner));
        }
        if owner_record.declaration_block.is_some() {
            return Err(MutationError::<R>::DeclarationBlockAlreadyExists(owner));
        }
        if self.effective_keys.try_get(effective_key).is_none() {
            return Err(MutationError::<R>::UnknownEffectiveKey(effective_key));
        }
        if !self.declaration_blocks.can_insert_between(after, before) {
            return Err(MutationError::<R>::LocalDeclarationBlockCapacityExhausted(
                after,
            ));
        }

        self.authored_declaration_append = None;
        let result = self.declaration_blocks.insert_between(
            after,
            before,
            DeclarationBlock::<R> {
                declarations: DeclarationList::empty(),
                owner: DeclarationBlockOwner::<R>::Rule(owner),
                effective_key,
                live: true,
            },
        );
        if !result.remaps.is_empty() {
            self.rules.for_each_enumerated_mut(|_, rule| {
                rule.declaration_block = rule
                    .declaration_block
                    .map(|id| remap_declaration_block_id(id, &result.remaps));
            });
        }
        self.rules
            .get_mut(owner)
            .expect("the synthesized block owner was validated before commit")
            .declaration_block = Some(result.id);
        Ok(result)
    }
}

impl<R: Unpin, D: Unpin, K> StyleSheet<'_, R, D, K> {
    /// Replaces one declaration payload through its owning block.
    pub fn replace_declaration(
        &mut self,
        block: DeclarationBlockId<R>,
        declaration: DeclarationId,
        replacement: D,
    ) -> Result<D, MutationError<R>> {
        let block_record = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::<R>::UnknownDeclarationBlock(block));
        }
        if !block_record.declarations.contains(declaration) {
            return Err(MutationError::<R>::UnknownDeclaration(declaration));
        }
        let record = self
            .declarations
            .get_mut(declaration)
            .ok_or(MutationError::<R>::UnknownDeclaration(declaration))?;
        let previous = std::mem::replace(record.payload_mut(), replacement);
        Ok(previous)
    }

    /// Folds one direct leaf rule's declaration range into its right sibling
    /// and retires the left rule in the same transaction.
    ///
    /// Semantic callers must additionally decide whether these rule kinds are
    /// mergeable. This storage transaction uses the edge's already-proven
    /// source bridge to preserve declaration-block order, and validates equal
    /// AST-owned EffectiveKeys and unique block ownership before it publishes
    /// any mutation. Retired source-order blocks between the live endpoints
    /// are absorbed as well; semantic callers are responsible for retiring
    /// those owners only after all of their occurrences are dead.
    pub(super) fn merge_adjacent_rule_declaration_blocks(
        &mut self,
        edge: DirectRuleEdge<R>,
    ) -> Result<MergedAdjacentRuleBlocks<R>, MutationError<R>> {
        let right = edge.right();
        let right_block = self
            .rules
            .get(right)
            .ok_or(MutationError::<R>::UnknownRule(right))?
            .declaration_block
            .ok_or(MutationError::<R>::InvalidRuleTopology(right))?;
        let append = self.declaration_append_context(right_block)?;
        self.merge_adjacent_rule_declaration_blocks_with_context(edge, append)
    }

    /// Context-consuming form for schedulers that already retain the right
    /// block's append cursor.
    #[doc(hidden)]
    fn merge_adjacent_rule_declaration_blocks_with_context(
        &mut self,
        edge: DirectRuleEdge<R>,
        append: DeclarationAppendContext<R>,
    ) -> Result<MergedAdjacentRuleBlocks<R>, MutationError<R>> {
        let left = edge.left();
        let right = edge.right();
        if !self.is_valid_direct_rule_edge(edge) {
            return Err(MutationError::<R>::InvalidRuleTopology(left));
        }
        let left_context = self.resolve_direct_rule_window(edge.left_context())?;
        let left_rule = self
            .rules
            .get(left)
            .ok_or(MutationError::<R>::UnknownRule(left))?;
        let right_rule = self
            .rules
            .get(right)
            .ok_or(MutationError::<R>::UnknownRule(right))?;
        if !left_rule.live {
            return Err(MutationError::<R>::RetiredRule(left));
        }
        if !right_rule.live {
            return Err(MutationError::<R>::RetiredRule(right));
        }
        if self.has_nested_rules(left)? {
            return Err(MutationError::<R>::RuleHasChildren(left));
        }
        if left_rule.parent != right_rule.parent {
            return Err(MutationError::<R>::InvalidRuleTopology(left));
        }
        let mut bridge_blocks = std::vec::Vec::new();
        let bridge_ids = self
            .rules
            .ids_in_range(left_context.bridge)
            .ok_or(MutationError::<R>::InvalidRuleTopology(left))?;
        for source_id in bridge_ids {
            let source_rule = self
                .rules
                .get(source_id)
                .ok_or(MutationError::<R>::InvalidRuleTopology(left))?;
            if source_rule.live {
                return Err(MutationError::<R>::InvalidRuleTopology(source_id));
            }
            if let Some(block) = source_rule.declaration_block {
                bridge_blocks.push(block);
            }
        }
        let left_block_id = left_rule
            .declaration_block
            .ok_or(MutationError::<R>::InvalidRuleTopology(left))?;
        let right_block_id = right_rule
            .declaration_block
            .ok_or(MutationError::<R>::InvalidRuleTopology(right))?;
        let left_block = self
            .declaration_blocks
            .get(left_block_id)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(left_block_id))?;
        let right_block = self
            .declaration_blocks
            .get(right_block_id)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(right_block_id))?;
        if append.block() != right_block_id
            || append.position.declarations != right_block.declarations
        {
            return Err(MutationError::<R>::NonContiguousDeclarationRange(
                right_block_id,
            ));
        }
        if !left_block.live
            || !right_block.live
            || left_block.owner != DeclarationBlockOwner::<R>::Rule(left)
            || right_block.owner != DeclarationBlockOwner::<R>::Rule(right)
            || left_block.effective_key != right_block.effective_key
        {
            return Err(MutationError::<R>::InvalidRuleTopology(left));
        }
        let effective_key = left_block.effective_key;
        let mut merged_declarations = DeclarationList::empty();
        let mut merge_range = |range: DeclarationList| {
            if range.is_empty() {
                return;
            }
            if merged_declarations.is_empty() {
                merged_declarations = range;
            } else {
                merged_declarations.extend(range);
            }
        };
        merge_range(left_block.declarations);
        for &bridge in &bridge_blocks {
            let bridge_block = self
                .declaration_blocks
                .get(bridge)
                .ok_or(MutationError::<R>::UnknownDeclarationBlock(bridge))?;
            if bridge_block.live {
                return Err(MutationError::<R>::InvalidRuleTopology(left));
            }
            merge_range(bridge_block.declarations);
        }
        merge_range(right_block.declarations);

        self.retire_rule(edge.left_context())?;
        self.declaration_blocks
            .get_mut(left_block_id)
            .expect("the retired block remains a source tombstone")
            .declarations
            .clear();
        for bridge in bridge_blocks {
            self.declaration_blocks
                .get_mut(bridge)
                .expect("a validated bridge block remains a source tombstone")
                .declarations
                .clear();
        }
        let retained_block = self
            .declaration_blocks
            .get_mut(right_block_id)
            .expect("the retained block was validated before commit");
        retained_block.declarations = merged_declarations;
        let declaration_append = self.refresh_declaration_append_context(append)?;
        Ok(MergedAdjacentRuleBlocks::<R> {
            retired_rule: left,
            retired_block: left_block_id,
            retained_rule: right,
            retained_block: right_block_id,
            effective_key,
            declaration_append,
        })
    }

    /// Retires one live rule without live nested rules while retaining its
    /// source-order tombstone.
    ///
    /// Parsed primary IDs and inserted sibling IDs are never reused. The
    /// declaration block is retired in the same transaction, while its range
    /// continues to own the corresponding arena occurrences.
    pub(super) fn retire_rule(
        &mut self,
        context: DirectRuleContext<R>,
    ) -> Result<(), MutationError<R>> {
        let context = self.resolve_direct_rule_window(context)?;
        let id = context.rule;
        let rule = self
            .rules
            .get(id)
            .ok_or(MutationError::<R>::UnknownRule(id))?;
        if !rule.live {
            return Err(MutationError::<R>::RetiredRule(id));
        }
        if self.has_nested_rules(id)? {
            return Err(MutationError::<R>::RuleHasChildren(id));
        }
        let declaration_block = rule.declaration_block;
        if declaration_block.is_some_and(|block| {
            self.declaration_blocks.get(block).is_none_or(|block| {
                !block.live || block.owner != DeclarationBlockOwner::<R>::Rule(id)
            })
        }) {
            return Err(MutationError::<R>::InvalidRuleTopology(id));
        }

        let rule = self
            .rules
            .get_mut(id)
            .expect("the retiring rule was validated");
        rule.live = false;
        if let Some(parent) = context.parent {
            let parent = self
                .rules
                .get_mut(parent)
                .expect("a retiring rule's validated parent remains resolvable");
            parent.nested_rule_count = parent
                .nested_rule_count
                .checked_sub(1)
                .expect("a live direct child contributes to its parent's count");
        }
        if let Some(block) = declaration_block {
            let block = self
                .declaration_blocks
                .get_mut(block)
                .expect("the retiring declaration block was validated");
            block.live = false;
        }
        Ok(())
    }
}

impl<'ast> StyleSheet<'ast> {
    /// Runs a scoped, non-structural transform over one live rule payload.
    #[doc(hidden)]
    pub fn transform_rule_payload(
        &mut self,
        rule_id: CssRuleId<'ast>,
        transform: impl FnOnce(&mut CssRule<'ast>),
    ) -> Result<(), StyleSheetMutationError<'ast>> {
        let rule = self
            .rules
            .get_mut(rule_id)
            .ok_or(StyleSheetMutationError::UnknownRule(rule_id))?;
        if !rule.live {
            return Err(StyleSheetMutationError::RetiredRule(rule_id));
        }
        transform(&mut rule.payload);
        Ok(())
    }

    /// Resolves one declaration-arena occurrence through its owner for
    /// a scoped local transform.
    #[doc(hidden)]
    pub fn declaration_occurrence_mut(
        &mut self,
        block: CssDeclarationBlockId<'ast>,
        declaration_id: DeclarationId,
    ) -> Result<(&mut CssDeclaration<'ast>, bool), StyleSheetMutationError<'ast>> {
        let block_record = self
            .declaration_blocks
            .get(block)
            .ok_or(StyleSheetMutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(StyleSheetMutationError::UnknownDeclarationBlock(block));
        }
        if !block_record.declarations.contains(declaration_id) {
            return Err(StyleSheetMutationError::UnknownDeclaration(declaration_id));
        }
        let declaration = self
            .declarations
            .get_mut(declaration_id)
            .ok_or(StyleSheetMutationError::UnknownDeclaration(declaration_id))?;
        let important = declaration.important;
        Ok((&mut declaration.payload, important))
    }

    /// Resolves one property declaration through its owning block for a
    /// scoped, in-place local transform.
    ///
    /// The returned reference borrows the whole stylesheet, so Rust prevents
    /// callers from inserting into any store until that local mutation ends.
    pub fn property_declaration_mut(
        &mut self,
        block: CssDeclarationBlockId<'ast>,
        declaration_id: DeclarationId,
    ) -> Result<(&mut crate::Declaration<'ast>, bool), StyleSheetMutationError<'ast>> {
        let (payload, important) = self.declaration_occurrence_mut(block, declaration_id)?;
        let CssDeclaration::Property(payload) = payload else {
            return Err(StyleSheetMutationError::UnknownDeclaration(declaration_id));
        };
        Ok((payload, important))
    }
}

#[inline]
fn remap_rule_id<P>(id: RuleId<P>, remaps: &[RadixIdRemap<RuleId<P>>]) -> RuleId<P> {
    remaps
        .iter()
        .find_map(|remap| (remap.old == id).then_some(remap.new))
        .unwrap_or(id)
}

#[inline]
pub(super) fn remap_declaration_block_id<P>(
    id: DeclarationBlockId<P>,
    remaps: &[RadixIdRemap<DeclarationBlockId<P>>],
) -> DeclarationBlockId<P> {
    remaps
        .iter()
        .find_map(|remap| (remap.old == id).then_some(remap.new))
        .unwrap_or(id)
}
