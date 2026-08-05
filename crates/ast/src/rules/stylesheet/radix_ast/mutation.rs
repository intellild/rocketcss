use super::*;

impl<R, D: Unpin, K> RadixCompilation<'_, R, D, K>
where
    R: RuleIdReferences<R> + Unpin,
    K: RuleIdReferences<R> + Copy + Eq + std::hash::Hash,
{
    /// Inserts a new direct sibling after `after` at its final Radix ID.
    ///
    /// The physical insertion anchor is the tail of `after`'s complete
    /// subtree plus any retired arena entries before the next live
    /// rule. This preserves global lexical preorder without reusing IDs.
    pub fn insert_rule_after(
        &mut self,
        after: RuleId<R>,
        payload: R,
    ) -> Result<RadixInsertResult<RuleId<R>>, MutationError<R>> {
        let after_record = self
            .rules
            .get(after)
            .ok_or(MutationError::<R>::UnknownRule(after))?;
        if !after_record.live {
            return Err(MutationError::<R>::RetiredRule(after));
        }
        let parent = after_record.parent;
        let direct_before = self.next_sibling(after);

        let logical_tail = self
            .subtree_tail(after)
            .ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
        let storage_before =
            direct_before.or_else(|| parent.and_then(|parent| self.next_after_subtree(parent)));
        let mut anchor = logical_tail;
        loop {
            let next = self.rules.next_id(anchor);
            if next == storage_before {
                break;
            }
            let next = next.ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
            if self.rules.get(next).is_none_or(|rule| rule.live) {
                return Err(MutationError::<R>::InvalidRuleTopology(after));
            }
            anchor = next;
        }
        if !self.rules.can_insert_between(anchor, storage_before) {
            return Err(MutationError::<R>::LocalRuleCapacityExhausted(anchor));
        }

        let result = self.rules.insert_between(
            anchor,
            storage_before,
            RuleRecord {
                payload,
                parent,
                nested_rule_count: 0,
                declaration_block: None,
                revision: 0,
                live: true,
            },
        );
        self.repair_rule_id_remaps(&result.remaps);

        let mut ancestor = parent.map(|id| remap_rule_id(id, &result.remaps));
        while let Some(id) = ancestor {
            let rule = self
                .rules
                .get_mut(id)
                .expect("an insertion ancestor remains resolvable after ID repair");
            rule.nested_rule_count += 1;
            ancestor = rule.parent;
        }
        Ok(result)
    }

    fn repair_rule_id_remaps(&mut self, remaps: &[RadixIdRemap<RuleId<R>>]) {
        if remaps.is_empty() {
            return;
        }
        self.rules.for_each_enumerated_mut(|_, rule| {
            rule.parent = rule.parent.map(|id| remap_rule_id(id, remaps));
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

    fn first_declaration_block_after_rule(
        &self,
        after: RuleId<R>,
        before_or_at: RuleId<R>,
    ) -> Result<DeclarationBlockId<R>, MutationError<R>> {
        self.rules
            .get(before_or_at)
            .ok_or(MutationError::<R>::UnknownRule(before_or_at))?;
        if self.next_sibling(after) != Some(before_or_at) {
            return Err(MutationError::<R>::InvalidRuleTopology(after));
        }
        let tail = self
            .subtree_tail(after)
            .ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
        let mut current = self.rules.next_id(tail);
        while let Some(rule) = current {
            let record = self
                .rules
                .get(rule)
                .ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
            if let Some(block) = record.declaration_block {
                return Ok(block);
            }
            if rule == before_or_at {
                break;
            }
            current = self.rules.next_id(rule);
        }
        Err(MutationError::<R>::InvalidRuleTopology(after))
    }

    /// Inserts one synthesized rule and its declaration block as a single
    /// preflighted AST transaction.
    ///
    /// `before_or_at` bounds the source-order search for the next block. The
    /// caller supplies semantic rule/block endpoints; physical subtree and
    /// tombstone placement stays private to the AST.
    pub fn insert_rule_with_declaration_block_after(
        &mut self,
        after_rule: RuleId<R>,
        before_or_at_rule: RuleId<R>,
        after_block: DeclarationBlockId<R>,
        payload: R,
        effective_key: EffectiveKeyId,
        additional_declarations: usize,
    ) -> Result<InsertedRuleWithDeclarationBlock<R>, MutationError<R>> {
        if self.effective_keys.try_get(effective_key).is_none() {
            return Err(MutationError::<R>::UnknownEffectiveKey(effective_key));
        }
        let before_block =
            self.first_declaration_block_after_rule(after_rule, before_or_at_rule)?;
        let additional_declarations = u32::try_from(additional_declarations)
            .map_err(|_| MutationError::<R>::DeclarationCapacityExhausted)?;
        let (after_declaration, before_declaration) =
            self.declaration_neighbors_at_block_end(after_block)?;
        if !self.can_insert_declaration_range_between(
            after_declaration,
            before_declaration,
            additional_declarations,
        ) {
            return Err(MutationError::<R>::DeclarationCapacityExhausted);
        }
        if !self
            .declaration_blocks
            .can_insert_between(after_block, Some(before_block))
        {
            return Err(MutationError::<R>::LocalDeclarationBlockCapacityExhausted(
                after_block,
            ));
        }

        let rule = self.insert_rule_after(after_rule, payload)?;
        let declaration_block = self.insert_declaration_block_between(
            after_block,
            Some(before_block),
            rule.id,
            effective_key,
        )?;
        Ok(InsertedRuleWithDeclarationBlock {
            rule,
            declaration_block,
        })
    }
}

impl<R: Unpin, D: Unpin, K> RadixCompilation<'_, R, D, K> {
    fn declaration_neighbors_at_block_end(
        &self,
        target: DeclarationBlockId<R>,
    ) -> Result<(Option<DeclarationId>, Option<DeclarationId>), MutationError<R>> {
        let mut previous = None;
        let mut found = false;
        for (block_id, block) in self.declaration_blocks.iter_enumerated() {
            if block_id == target {
                found = true;
                if !block.declarations.is_empty() {
                    previous = Some(
                        self.declarations
                            .last_id(block.declarations)
                            .ok_or(MutationError::<R>::NonContiguousDeclarationRange(target))?,
                    );
                }
                continue;
            }
            if block.declarations.is_empty() {
                continue;
            }
            if found {
                return Ok((previous, Some(block.declarations.start_id())));
            }
            previous = Some(
                self.declarations
                    .last_id(block.declarations)
                    .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block_id))?,
            );
        }
        if found {
            Ok((previous, None))
        } else {
            Err(MutationError::<R>::UnknownDeclarationBlock(target))
        }
    }

    fn can_insert_declaration_range_between(
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
        let declarations = declarations.into_iter();
        let len = u32::try_from(declarations.len())
            .map_err(|_| MutationError::<R>::DeclarationCapacityExhausted)?;
        let block_record = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::<R>::UnknownDeclarationBlock(block));
        }
        if len == 0 {
            return Ok(());
        }
        let existing = block_record.declarations;
        let revision = block_record.revision.wrapping_add(len);
        let (after, before) = self.declaration_neighbors_at_block_end(block)?;
        if !self.can_insert_declaration_range_between(after, before, len) {
            return Err(MutationError::<R>::DeclarationCapacityExhausted);
        }
        let records =
            declarations.map(|(payload, important)| DeclarationRecord { payload, important });
        let inserted = match after {
            Some(after) => self
                .declarations
                .insert_stable_range_between(after, before, records),
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
            block.declarations.extend_by(len);
        }
        block.revision = revision;
        Ok(())
    }

    /// Inserts a synthesized declaration block at its final semantic block ID
    /// and binds it to an already inserted live owner rule.
    pub(crate) fn insert_declaration_block_between(
        &mut self,
        after: DeclarationBlockId<R>,
        before: Option<DeclarationBlockId<R>>,
        owner: RuleId<R>,
        effective_key: EffectiveKeyId,
    ) -> Result<RadixInsertResult<DeclarationBlockId<R>>, MutationError<R>> {
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

        let result = self.declaration_blocks.insert_between(
            after,
            before,
            DeclarationBlockRecord::<R> {
                declarations: DeclarationList::empty(),
                owner: DeclarationBlockOwner::<R>::Rule(owner),
                effective_key,
                revision: 0,
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

impl<R: Unpin, D: Unpin, K> RadixCompilation<'_, R, D, K> {
    /// Replaces one declaration payload through its owning block and bumps the
    /// block revision used by incremental Nano candidates.
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
        let revision = block_record.revision.wrapping_add(1);
        if !self
            .declaration_ids_in_block(block)?
            .any(|candidate| candidate == declaration)
        {
            return Err(MutationError::<R>::UnknownDeclaration(declaration));
        }
        let record = self
            .declarations
            .get_mut(declaration)
            .ok_or(MutationError::<R>::UnknownDeclaration(declaration))?;
        let previous = std::mem::replace(record.payload_mut(), replacement);
        self.declaration_blocks
            .get_mut(block)
            .expect("the declaration owner was validated before commit")
            .revision = revision;
        Ok(previous)
    }

    /// Folds one direct leaf rule's declaration range into its right sibling
    /// and retires the left rule in the same transaction.
    ///
    /// Semantic callers must additionally decide whether these rule kinds are
    /// mergeable. This storage transaction proves adjacency, equal AST-owned
    /// EffectiveKeys, unique block ownership, and contiguous ranges before it
    /// publishes any mutation. Retired source-order blocks between the live
    /// endpoints are absorbed as well; semantic callers are responsible for
    /// retiring those owners only after all of their occurrences are dead.
    pub fn merge_adjacent_rule_declaration_blocks(
        &mut self,
        left: RuleId<R>,
        right: RuleId<R>,
    ) -> Result<MergedAdjacentRuleBlocks<R>, MutationError<R>> {
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
        if self.next_sibling(left) != Some(right) || left_rule.parent != right_rule.parent {
            return Err(MutationError::<R>::InvalidRuleTopology(left));
        }
        let mut bridge_blocks = std::vec::Vec::new();
        let mut source_cursor = self.rules.next_id(left);
        while source_cursor != Some(right) {
            let source_rule = source_cursor
                .and_then(|id| self.rules.get(id).map(|rule| (id, rule)))
                .ok_or(MutationError::<R>::InvalidRuleTopology(left))?;
            if source_rule.1.live {
                return Err(MutationError::<R>::InvalidRuleTopology(source_rule.0));
            }
            if let Some(block) = source_rule.1.declaration_block {
                bridge_blocks.push(block);
            }
            source_cursor = self.rules.next_id(source_rule.0);
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
        let mut previous_non_empty = None;
        let mut merge_range = |block_id: DeclarationBlockId<R>, range: DeclarationList| {
            if range.is_empty() {
                return Ok(());
            }
            if let Some(previous) = previous_non_empty
                && !self.declarations.ranges_are_adjacent(previous, range)
            {
                return Err(MutationError::<R>::NonContiguousDeclarationRange(block_id));
            }
            if merged_declarations.is_empty() {
                merged_declarations = range;
            } else {
                merged_declarations.extend_by(range.len());
            }
            previous_non_empty = Some(range);
            Ok(())
        };
        merge_range(left_block_id, left_block.declarations)?;
        for &bridge in &bridge_blocks {
            let bridge_block = self
                .declaration_blocks
                .get(bridge)
                .ok_or(MutationError::<R>::UnknownDeclarationBlock(bridge))?;
            if bridge_block.live {
                return Err(MutationError::<R>::InvalidRuleTopology(left));
            }
            merge_range(bridge, bridge_block.declarations)?;
        }
        merge_range(right_block_id, right_block.declarations)?;

        self.retire_rule(left)?;
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
        retained_block.revision = retained_block.revision.wrapping_add(1);
        let retained_rule = self
            .rules
            .get_mut(right)
            .expect("the retained rule was validated before commit");
        retained_rule.revision = retained_rule.revision.wrapping_add(1);

        Ok(MergedAdjacentRuleBlocks::<R> {
            retired_rule: left,
            retired_block: left_block_id,
            retained_rule: right,
            retained_block: right_block_id,
            effective_key,
        })
    }

    /// Retires one live rule without live nested rules while retaining its
    /// source-order tombstone.
    ///
    /// Parsed primary IDs and inserted sibling IDs are never reused. The
    /// declaration block is retired in the same transaction, while its range
    /// continues to own the corresponding arena occurrences.
    pub fn retire_rule(&mut self, id: RuleId<R>) -> Result<RetiredRule<R>, MutationError<R>> {
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
        let previous = self.previous_sibling(id);
        let next = self.next_sibling(id);
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
        rule.revision = rule.revision.wrapping_add(1);
        if let Some(block) = declaration_block {
            let block = self
                .declaration_blocks
                .get_mut(block)
                .expect("the retiring declaration block was validated");
            block.live = false;
            block.revision = block.revision.wrapping_add(1);
        }
        Ok(RetiredRule {
            id,
            previous,
            next,
            declaration_block,
        })
    }
}

impl<'ast> Compilation<'ast> {
    /// Runs a scoped, non-structural transform over one live rule payload.
    #[doc(hidden)]
    pub fn transform_rule_payload(
        &mut self,
        rule_id: ConcreteRuleId<'ast>,
        transform: impl FnOnce(&mut CssRulePayload<'ast>),
    ) -> Result<(), ConcreteMutationError<'ast>> {
        let rule = self
            .rules
            .get_mut(rule_id)
            .ok_or(ConcreteMutationError::UnknownRule(rule_id))?;
        if !rule.live {
            return Err(ConcreteMutationError::RetiredRule(rule_id));
        }
        transform(&mut rule.payload);
        rule.revision = rule.revision.wrapping_add(1);
        Ok(())
    }

    /// Resolves one declaration-arena occurrence through its owner for
    /// a scoped local transform.
    #[doc(hidden)]
    pub fn declaration_occurrence_mut(
        &mut self,
        block: ConcreteDeclarationBlockId<'ast>,
        declaration_id: DeclarationId,
    ) -> Result<(&mut DeclarationPayload<'ast>, bool), ConcreteMutationError<'ast>> {
        let block_record = self
            .declaration_blocks
            .get(block)
            .ok_or(ConcreteMutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(ConcreteMutationError::UnknownDeclarationBlock(block));
        }
        let revision = block_record.revision.wrapping_add(1);
        if !self
            .declaration_ids_in_block(block)?
            .any(|candidate| candidate == declaration_id)
        {
            return Err(ConcreteMutationError::UnknownDeclaration(declaration_id));
        }
        let declaration = self
            .declarations
            .get_mut(declaration_id)
            .ok_or(ConcreteMutationError::UnknownDeclaration(declaration_id))?;
        let important = declaration.important;
        self.declaration_blocks
            .get_mut(block)
            .expect("the declaration owner was validated before mutation")
            .revision = revision;
        Ok((&mut declaration.payload, important))
    }

    /// Resolves one property declaration through its owning block for a
    /// scoped, in-place local transform.
    ///
    /// The returned reference borrows the whole compilation, so Rust prevents
    /// callers from inserting into any store until that local mutation ends.
    /// The block revision is bumped before the reference is exposed.
    pub fn property_declaration_mut(
        &mut self,
        block: ConcreteDeclarationBlockId<'ast>,
        declaration_id: DeclarationId,
    ) -> Result<(&mut crate::Declaration<'ast>, bool), ConcreteMutationError<'ast>> {
        let (payload, important) = self.declaration_occurrence_mut(block, declaration_id)?;
        let DeclarationPayload::Property(payload) = payload else {
            return Err(ConcreteMutationError::UnknownDeclaration(declaration_id));
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
fn remap_declaration_block_id<P>(
    id: DeclarationBlockId<P>,
    remaps: &[RadixIdRemap<DeclarationBlockId<P>>],
) -> DeclarationBlockId<P> {
    remaps
        .iter()
        .find_map(|remap| (remap.old == id).then_some(remap.new))
        .unwrap_or(id)
}
