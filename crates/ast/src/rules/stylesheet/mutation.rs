use super::*;

#[derive(Clone, Copy)]
struct RuleListCursor<R> {
    after: RuleId<R>,
    direct_before: Option<RuleId<R>>,
    source_anchor: RuleId<R>,
    source_before: Option<RuleId<R>>,
}

/// A validated direct-rule editing context with a stable source-order cursor.
pub struct RuleListEditor<'sheet, 'ast, R: Unpin, D, K> {
    list: RuleListId,
    parent: Option<RuleId<R>>,
    cursor: RuleListCursor<R>,
    stylesheet: &'sheet mut StyleSheet<'ast, R, D, K>,
}

impl<'sheet, 'ast, R, D, K> RuleListEditor<'sheet, 'ast, R, D, K>
where
    R: RuleIdReferences<R> + Unpin,
    K: RuleIdReferences<R> + Copy + Eq + std::hash::Hash,
{
    /// Inserts one direct sibling through this prevalidated context.
    pub fn try_insert(self, payload: R) -> Result<RadixInsertResult<RuleId<R>>, MutationError<R>> {
        let Self {
            list,
            parent,
            cursor,
            stylesheet,
        } = self;
        let record = RuleRecord {
            payload,
            parent,
            parent_list: list,
            previous_sibling: Some(cursor.after),
            next_sibling: cursor.direct_before,
            previous_in_source: Some(cursor.source_anchor),
            next_in_source: cursor.source_before,
            child_list: None,
            declaration_block: None,
            revision: 0,
            live: true,
        };
        let result = match stylesheet
            .rules
            .entry_between(cursor.source_anchor, cursor.source_before)
        {
            Some(entry) => match entry.try_insert(record) {
                Ok(id) => RadixInsertResult {
                    id,
                    remaps: std::vec::Vec::new(),
                },
                Err(error) => rebalance_insert(
                    &mut stylesheet.rules,
                    cursor.source_anchor,
                    cursor.source_before,
                    error.value,
                )
                .map_err(|_| {
                    MutationError::<R>::LocalRuleCapacityExhausted(cursor.source_anchor)
                })?,
            },
            None => rebalance_insert(
                &mut stylesheet.rules,
                cursor.source_anchor,
                cursor.source_before,
                record,
            )
            .map_err(|_| MutationError::<R>::LocalRuleCapacityExhausted(cursor.source_anchor))?,
        };
        stylesheet.repair_rule_id_remaps(&result.remaps);

        let after = remap_rule_id(cursor.after, &result.remaps);
        let direct_before = cursor
            .direct_before
            .map(|id| remap_rule_id(id, &result.remaps));
        let source_anchor = remap_rule_id(cursor.source_anchor, &result.remaps);
        let source_before = cursor
            .source_before
            .map(|id| remap_rule_id(id, &result.remaps));
        stylesheet
            .rules
            .get_mut(after)
            .expect("the insertion transaction repaired the previous sibling")
            .next_sibling = Some(result.id);
        if let Some(before) = direct_before {
            stylesheet
                .rules
                .get_mut(before)
                .expect("the insertion transaction repaired the next sibling")
                .previous_sibling = Some(result.id);
        } else {
            stylesheet.rule_lists.get_mut(list).last = Some(result.id);
        }
        stylesheet.rule_lists.get_mut(list).live_len += 1;
        stylesheet
            .rules
            .get_mut(source_anchor)
            .expect("the insertion transaction repaired the source anchor")
            .next_in_source = Some(result.id);
        if let Some(before) = source_before {
            stylesheet
                .rules
                .get_mut(before)
                .expect("the insertion transaction repaired the source successor")
                .previous_in_source = Some(result.id);
        } else {
            stylesheet.last_rule_in_source = Some(result.id);
        }
        Ok(result)
    }
}

#[derive(Clone, Copy)]
struct DeclarationBlockCursor<R> {
    after: DeclarationBlockId<R>,
    before: Option<DeclarationBlockId<R>>,
}

/// A validated declaration-block editing context bound to its owner rule.
pub struct DeclarationBlockEditor<'sheet, 'ast, R: Unpin, D, K> {
    owner: RuleId<R>,
    effective_key: EffectiveKeyId,
    cursor: DeclarationBlockCursor<R>,
    stylesheet: &'sheet mut StyleSheet<'ast, R, D, K>,
}

impl<'sheet, 'ast, R: Unpin, D, K> DeclarationBlockEditor<'sheet, 'ast, R, D, K> {
    /// Inserts and binds the empty synthesized block through this context.
    pub fn try_insert(self) -> Result<RadixInsertResult<DeclarationBlockId<R>>, MutationError<R>> {
        let Self {
            owner,
            effective_key,
            cursor,
            stylesheet,
        } = self;
        let block = DeclarationBlock::<R> {
            declarations: DeclarationList::Range(DeclarationRange {
                start: stylesheet.declarations.len() as u32,
                len: 0,
            }),
            owner: DeclarationBlockOwner::<R>::Rule(owner),
            effective_key,
            revision: 0,
            live: true,
        };
        let result = match stylesheet
            .declaration_blocks
            .entry_between(cursor.after, cursor.before)
        {
            Some(entry) => match entry.try_insert(block) {
                Ok(id) => RadixInsertResult {
                    id,
                    remaps: std::vec::Vec::new(),
                },
                Err(error) => rebalance_insert(
                    &mut stylesheet.declaration_blocks,
                    cursor.after,
                    cursor.before,
                    error.value,
                )
                .map_err(|_| {
                    MutationError::<R>::LocalDeclarationBlockCapacityExhausted(cursor.after)
                })?,
            },
            None => rebalance_insert(
                &mut stylesheet.declaration_blocks,
                cursor.after,
                cursor.before,
                block,
            )
            .map_err(|_| {
                MutationError::<R>::LocalDeclarationBlockCapacityExhausted(cursor.after)
            })?,
        };
        if !result.remaps.is_empty() {
            let rule_ids = stylesheet
                .rules
                .iter_enumerated()
                .map(|(id, _)| id)
                .collect::<std::vec::Vec<_>>();
            for rule in rule_ids {
                let rule = stylesheet
                    .rules
                    .get_mut(rule)
                    .expect("an enumerated rule remains resolvable");
                rule.declaration_block = rule
                    .declaration_block
                    .map(|id| remap_declaration_block_id(id, &result.remaps));
            }
        }
        stylesheet
            .rules
            .get_mut(owner)
            .expect("the synthesized block owner was validated before commit")
            .declaration_block = Some(result.id);
        Ok(result)
    }
}

impl<'ast, R, D, K> StyleSheet<'ast, R, D, K>
where
    R: RuleIdReferences<R> + Unpin,
    K: RuleIdReferences<R> + Copy + Eq + std::hash::Hash,
{
    /// Inserts a new direct sibling after `after` at its final Radix ID.
    ///
    /// The physical insertion anchor is the tail of `after`'s complete
    /// subtree plus any retired source-chain entries before the next live
    /// rule. This preserves global lexical preorder without reusing IDs.
    pub fn insert_rule_after(
        &mut self,
        after: RuleId<R>,
        payload: R,
    ) -> Result<RadixInsertResult<RuleId<R>>, MutationError<R>> {
        self.rule_list_editor_after(after)?.try_insert(payload)
    }

    /// Resolves all direct-sibling and source-order context for one insertion.
    pub fn rule_list_editor_after(
        &mut self,
        after: RuleId<R>,
    ) -> Result<RuleListEditor<'_, 'ast, R, D, K>, MutationError<R>> {
        let after_record = self
            .rules
            .get(after)
            .ok_or(MutationError::<R>::UnknownRule(after))?;
        if !after_record.live {
            return Err(MutationError::<R>::RetiredRule(after));
        }
        let parent = after_record.parent;
        let list = after_record.parent_list;
        let direct_before = after_record.next_sibling;
        let list_record = self
            .rule_lists
            .try_get(list)
            .ok_or(MutationError::<R>::UnknownRuleList(list))?;
        if direct_before.is_none() && list_record.last != Some(after) {
            return Err(MutationError::<R>::InvalidRuleTopology(after));
        }
        if direct_before.is_some_and(|before| {
            self.rules.get(before).is_none_or(|before| {
                !before.live || before.previous_sibling != Some(after) || before.parent_list != list
            })
        }) {
            return Err(MutationError::<R>::InvalidRuleTopology(after));
        }

        let logical_tail = self
            .subtree_tail(after)
            .ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
        let storage_before = self.next_after_subtree(after);
        if direct_before.is_some() && storage_before != direct_before {
            return Err(MutationError::<R>::InvalidRuleTopology(after));
        }
        let mut anchor = logical_tail;
        loop {
            let next = self
                .rules
                .get(anchor)
                .ok_or(MutationError::<R>::InvalidRuleTopology(after))?
                .next_in_source;
            if next == storage_before {
                break;
            }
            let next = next.ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
            if self.rules.get(next).is_none_or(|rule| rule.live) {
                return Err(MutationError::<R>::InvalidRuleTopology(after));
            }
            anchor = next;
        }
        Ok(RuleListEditor {
            list,
            parent,
            cursor: RuleListCursor {
                after,
                direct_before,
                source_anchor: anchor,
                source_before: storage_before,
            },
            stylesheet: self,
        })
    }

    fn repair_rule_id_remaps(&mut self, remaps: &[RadixIdRemap<RuleId<R>>]) {
        if remaps.is_empty() {
            return;
        }
        self.first_rule_in_source = self
            .first_rule_in_source
            .map(|id| remap_rule_id(id, remaps));
        self.last_rule_in_source = self.last_rule_in_source.map(|id| remap_rule_id(id, remaps));
        for list in self.rule_lists.iter_mut() {
            list.parent = list.parent.map(|id| remap_rule_id(id, remaps));
            list.first = list.first.map(|id| remap_rule_id(id, remaps));
            list.last = list.last.map(|id| remap_rule_id(id, remaps));
        }

        let rule_ids = self
            .rules
            .iter_enumerated()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>();
        for id in rule_ids {
            let rule = self
                .rules
                .get_mut(id)
                .expect("an enumerated rule ID remains resolvable");
            rule.parent = rule.parent.map(|id| remap_rule_id(id, remaps));
            rule.previous_sibling = rule.previous_sibling.map(|id| remap_rule_id(id, remaps));
            rule.next_sibling = rule.next_sibling.map(|id| remap_rule_id(id, remaps));
            rule.previous_in_source = rule.previous_in_source.map(|id| remap_rule_id(id, remaps));
            rule.next_in_source = rule.next_in_source.map(|id| remap_rule_id(id, remaps));
            rule.payload.remap_rule_ids(remaps);
        }

        let block_ids = self
            .declaration_blocks
            .iter_enumerated()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>();
        for id in block_ids {
            let block = self
                .declaration_blocks
                .get_mut(id)
                .expect("an enumerated block ID remains resolvable");
            let DeclarationBlockOwner::<R>::Rule(owner) = &mut block.owner;
            *owner = remap_rule_id(*owner, remaps);
        }
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
}

impl<'ast, R: Unpin, D, K> StyleSheet<'ast, R, D, K> {
    fn declaration_list_for_ids(
        &mut self,
        declarations: &[DeclarationId],
    ) -> Result<DeclarationList, MutationError<R>> {
        let contiguous = declarations
            .windows(2)
            .all(|pair| pair[0].index() + 1 == pair[1].index());
        if contiguous {
            let start = declarations.first().map_or(0, |id| id.index() as u32);
            return Ok(DeclarationList::Range(DeclarationRange {
                start,
                len: declarations.len() as u32,
            }));
        }
        if let Some(local) = LocalPropertySet::from_ids(declarations) {
            return Ok(DeclarationList::Local4(local));
        }
        let mut overflow = self.allocator.vec();
        for &declaration in declarations {
            overflow.push(declaration);
        }
        let overflow = self
            .declaration_overflows
            .try_push(overflow)
            .map_err(|_| MutationError::<R>::DeclarationOverflowCapacityExhausted)?;
        Ok(DeclarationList::Overflow(overflow))
    }

    /// Appends a transformed declaration to any live block representation.
    ///
    /// Authored parsing continues to use `append_declaration`, which rejects a
    /// range closed by nested syntax. This transaction is for synthesized
    /// declarations: it retains a contiguous range when possible, uses
    /// `Local4` for a small non-contiguous sequence, and atomically promotes a
    /// fifth local occurrence to the arena-backed complete overflow list.
    pub fn append_transformed_declaration(
        &mut self,
        block: DeclarationBlockId<R>,
        payload: D,
        important: bool,
    ) -> Result<DeclarationId, MutationError<R>> {
        let block_record = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::<R>::UnknownDeclarationBlock(block));
        }
        let revision = block_record.revision.wrapping_add(1);
        let declaration = self
            .declarations
            .try_next_id()
            .map_err(|_| MutationError::<R>::DeclarationCapacityExhausted)?;
        let mut declarations = self
            .declaration_ids_in_block(block)?
            .collect::<std::vec::Vec<_>>();
        declarations.push(declaration);
        let representation = self.declaration_list_for_ids(&declarations)?;
        let inserted = self
            .declarations
            .try_push(DeclarationRecord { payload, important })
            .map_err(|_| MutationError::<R>::DeclarationCapacityExhausted)?;
        debug_assert_eq!(inserted, declaration);
        let block = self
            .declaration_blocks
            .get_mut(block)
            .expect("the transformed declaration owner was validated");
        block.declarations = representation;
        block.revision = revision;
        Ok(inserted)
    }

    /// Preflights the remaining fallible storage operations for a synthesized
    /// block before its owner rule is inserted.
    pub fn can_insert_declaration_block_between(
        &mut self,
        after: DeclarationBlockId<R>,
        before: Option<DeclarationBlockId<R>>,
        declaration_count: usize,
    ) -> bool {
        if self
            .declarations
            .len()
            .checked_add(declaration_count)
            .is_none_or(|len| len > u32::MAX as usize)
        {
            return false;
        }
        self.declaration_blocks
            .entry_between(after, before)
            .is_some()
            || build_rebalance_plan(&self.declaration_blocks, after, before).is_some()
    }

    /// Inserts a synthesized declaration block at its final semantic block ID
    /// and binds it to an already inserted live owner rule.
    pub fn insert_declaration_block_between(
        &mut self,
        after: DeclarationBlockId<R>,
        before: Option<DeclarationBlockId<R>>,
        owner: RuleId<R>,
        effective_key: EffectiveKeyId,
    ) -> Result<RadixInsertResult<DeclarationBlockId<R>>, MutationError<R>> {
        self.declaration_block_editor(after, before, owner, effective_key)?
            .try_insert()
    }

    /// Resolves the owner and source-order context for one synthesized block.
    pub fn declaration_block_editor(
        &mut self,
        after: DeclarationBlockId<R>,
        before: Option<DeclarationBlockId<R>>,
        owner: RuleId<R>,
        effective_key: EffectiveKeyId,
    ) -> Result<DeclarationBlockEditor<'_, 'ast, R, D, K>, MutationError<R>> {
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
        Ok(DeclarationBlockEditor {
            owner,
            effective_key,
            cursor: DeclarationBlockCursor { after, before },
            stylesheet: self,
        })
    }
}

impl<R: Unpin, D, K> StyleSheet<'_, R, D, K> {
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
            .try_get_mut(declaration)
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
    /// publishes any mutation. Retired source-chain blocks between the live
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
        if left_rule.child_list.is_some() {
            return Err(MutationError::<R>::RuleHasChildren(left));
        }
        if left_rule.next_sibling != Some(right)
            || right_rule.previous_sibling != Some(left)
            || left_rule.parent != right_rule.parent
            || left_rule.parent_list != right_rule.parent_list
        {
            return Err(MutationError::<R>::InvalidRuleTopology(left));
        }
        let mut bridge_blocks = std::vec::Vec::new();
        let mut source_cursor = left_rule.next_in_source;
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
            source_cursor = source_rule.1.next_in_source;
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
        let mut merged_declarations = self
            .declaration_ids_in_block(left_block_id)?
            .collect::<std::vec::Vec<_>>();
        for &bridge in &bridge_blocks {
            let bridge_block = self
                .declaration_blocks
                .get(bridge)
                .ok_or(MutationError::<R>::UnknownDeclarationBlock(bridge))?;
            if bridge_block.live {
                return Err(MutationError::<R>::InvalidRuleTopology(left));
            }
            merged_declarations.extend(self.declaration_ids_in_block(bridge)?);
        }
        merged_declarations.extend(self.declaration_ids_in_block(right_block_id)?);
        let merged_declaration_list = self.declaration_list_for_ids(&merged_declarations)?;

        self.retire_rule(left)?;
        self.declaration_blocks
            .get_mut(left_block_id)
            .expect("the retired block remains a source tombstone")
            .declarations = DeclarationList::Range(DeclarationRange { start: 0, len: 0 });
        for bridge in bridge_blocks {
            self.declaration_blocks
                .get_mut(bridge)
                .expect("a validated bridge block remains a source tombstone")
                .declarations = DeclarationList::Range(DeclarationRange { start: 0, len: 0 });
        }
        let retained_block = self
            .declaration_blocks
            .get_mut(right_block_id)
            .expect("the retained block was validated before commit");
        retained_block.declarations = merged_declaration_list;
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

    /// Unlinks one live leaf rule while retaining its source-chain tombstone.
    ///
    /// Parsed primary IDs and inserted sibling IDs are never reused. The
    /// declaration block is retired in the same transaction, while its range
    /// continues to own the corresponding source-tape occurrences.
    pub fn retire_rule(&mut self, id: RuleId<R>) -> Result<RetiredRule<R>, MutationError<R>> {
        let rule = self
            .rules
            .get(id)
            .ok_or(MutationError::<R>::UnknownRule(id))?;
        if !rule.live {
            return Err(MutationError::<R>::RetiredRule(id));
        }
        if rule.child_list.is_some() {
            return Err(MutationError::<R>::RuleHasChildren(id));
        }
        let list = rule.parent_list;
        let previous = rule.previous_sibling;
        let next = rule.next_sibling;
        let declaration_block = rule.declaration_block;

        let list_record = self
            .rule_lists
            .try_get(list)
            .ok_or(MutationError::<R>::UnknownRuleList(list))?;
        if list_record.live_len == 0
            || previous.is_none() && list_record.first != Some(id)
            || next.is_none() && list_record.last != Some(id)
            || previous.is_some_and(|previous| {
                self.rules
                    .get(previous)
                    .is_none_or(|previous| previous.next_sibling != Some(id) || !previous.live)
            })
            || next.is_some_and(|next| {
                self.rules
                    .get(next)
                    .is_none_or(|next| next.previous_sibling != Some(id) || !next.live)
            })
        {
            return Err(MutationError::<R>::InvalidRuleTopology(id));
        }
        if declaration_block.is_some_and(|block| {
            self.declaration_blocks.get(block).is_none_or(|block| {
                !block.live || block.owner != DeclarationBlockOwner::<R>::Rule(id)
            })
        }) {
            return Err(MutationError::<R>::InvalidRuleTopology(id));
        }

        if let Some(previous) = previous {
            self.rules
                .get_mut(previous)
                .expect("the previous direct sibling was validated")
                .next_sibling = next;
        } else {
            self.rule_lists.get_mut(list).first = next;
        }
        if let Some(next) = next {
            self.rules
                .get_mut(next)
                .expect("the next direct sibling was validated")
                .previous_sibling = previous;
        } else {
            self.rule_lists.get_mut(list).last = previous;
        }
        self.rule_lists.get_mut(list).live_len -= 1;

        let rule = self
            .rules
            .get_mut(id)
            .expect("the retiring rule was validated");
        rule.previous_sibling = None;
        rule.next_sibling = None;
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
            list,
            previous,
            next,
            declaration_block,
        })
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
        rule.revision = rule.revision.wrapping_add(1);
        Ok(())
    }

    /// Resolves one authored declaration-tape occurrence through its owner for
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
        let revision = block_record.revision.wrapping_add(1);
        if !self
            .declaration_ids_in_block(block)?
            .any(|candidate| candidate == declaration_id)
        {
            return Err(StyleSheetMutationError::UnknownDeclaration(declaration_id));
        }
        let declaration = self
            .declarations
            .try_get_mut(declaration_id)
            .ok_or(StyleSheetMutationError::UnknownDeclaration(declaration_id))?;
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
    /// The returned reference borrows the whole stylesheet, so Rust prevents
    /// callers from inserting into any store until that local mutation ends.
    /// The block revision is bumped before the reference is exposed.
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

fn build_rebalance_plan<T: Unpin, I: RadixIdKey>(
    arena: &TypedRadixIndexArena<'_, T, I>,
    after: I,
    before: Option<I>,
) -> Option<RadixRebalancePlan<I>> {
    const SIBLING_LIMIT: u16 = 1023;

    if after.is_overflow() || arena.len() == u32::MAX as usize {
        return None;
    }
    let mut neighbors = arena.ids();
    while neighbors.next()? != after {}
    if neighbors.next() != before {
        return None;
    }

    let primary = I::from_primary_index(after.primary_index());
    let mut live = std::vec::Vec::new();
    let mut window = arena.ids().skip_while(|id| *id != primary);
    if window.next()? != primary {
        return None;
    }
    for id in window {
        if id.primary_index() != primary.primary_index() || id.is_primary() {
            break;
        }
        live.push(id);
    }
    let insertion_offset = live.partition_point(|id| id.sibling_key() <= after.sibling_key());
    let final_len = live.len().checked_add(1)?;

    let available = (1..=SIBLING_LIMIT)
        .filter(|&key| {
            arena.sibling_slot_state(primary, key) != Some(RadixSiblingSlotState::Retired)
        })
        .collect::<std::vec::Vec<_>>();
    if final_len > available.len() {
        return None;
    }

    let mut old_slots = live.into_iter().map(Some).collect::<std::vec::Vec<_>>();
    old_slots.insert(insertion_offset, None);
    let assignments = old_slots
        .into_iter()
        .enumerate()
        .map(|(index, old)| {
            let available_index = (index + 1) * available.len() / (final_len + 1);
            RadixAssignment {
                old,
                new: I::from_parts(primary.primary_index(), available[available_index]),
            }
        })
        .collect::<std::vec::Vec<_>>();
    let remaps = assignments
        .iter()
        .filter_map(|assignment| {
            assignment.old.and_then(|old| {
                (old != assignment.new).then_some(RadixIdRemap {
                    old,
                    new: assignment.new,
                })
            })
        })
        .collect();
    Some(RadixRebalancePlan {
        window: RadixRebalanceWindow {
            left_primary: primary,
            right_primary: before.filter(|id| id.is_primary()),
            insertion_offset: insertion_offset as u32,
        },
        assignments,
        remaps,
        replacement_ranges: std::vec::Vec::new(),
    })
}

fn rebalance_insert<T: Unpin, I: RadixIdKey>(
    arena: &mut TypedRadixIndexArena<'_, T, I>,
    after: I,
    before: Option<I>,
    value: T,
) -> Result<RadixInsertResult<I>, T> {
    let Some(plan) = build_rebalance_plan(arena, after, before) else {
        return Err(value);
    };
    debug_assert_eq!(
        plan.window.insertion_offset as usize,
        plan.assignments
            .iter()
            .position(|assignment| assignment.old.is_none())
            .unwrap()
    );
    debug_assert!(
        plan.window
            .right_primary
            .is_none_or(|right| right.primary_index() > plan.window.left_primary.primary_index())
    );
    debug_assert!(plan.replacement_ranges.is_empty());

    let mut existing = std::vec::Vec::with_capacity(plan.assignments.len().saturating_sub(1));
    for old in plan
        .assignments
        .iter()
        .filter_map(|assignment| assignment.old)
    {
        existing.push(
            arena
                .remove_sibling(old)
                .expect("a validated rebalance assignment names a live sibling"),
        );
    }

    let mut existing = existing.into_iter();
    let mut inserted = Some(value);
    let inserted_id = plan
        .assignments
        .iter()
        .find(|assignment| assignment.old.is_none())
        .expect("one rebalance assignment is the insertion")
        .new;
    {
        let mut entry = arena
            .sibling_entry(plan.window.left_primary)
            .expect("a validated rebalance window has a primary anchor");
        for assignment in &plan.assignments {
            let value = if assignment.old.is_some() {
                existing
                    .next()
                    .expect("every old assignment has one removed value")
            } else {
                inserted.take().expect("the plan inserts exactly one value")
            };
            let id = entry
                .try_insert(assignment.new.sibling_key(), value)
                .unwrap_or_else(|_| panic!("a validated rebalance plan is fully applicable"));
            debug_assert_eq!(id, assignment.new);
        }
    }
    debug_assert!(existing.next().is_none());
    Ok(RadixInsertResult {
        id: inserted_id,
        remaps: plan.remaps,
    })
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

#[cfg(test)]
mod rebalance_tests {
    use super::*;
    use rocketcss_common::{Allocator, RadixId, TypedRadixIndexArena};

    #[test]
    fn plan_failure_is_atomic_until_a_tombstone_is_explicitly_reclaimed() {
        let allocator = Allocator::new();
        let mut arena = TypedRadixIndexArena::<_, RadixId<u16>>::new_in(&allocator);
        let primary = arena.push_primary(0);
        let next_primary = arena.push_primary(20_000);
        let mut retired = None;
        {
            let mut entry = arena.sibling_entry(primary).unwrap();
            for key in 1..=1023 {
                let id = entry.try_insert(key, key).unwrap();
                if key == 500 {
                    retired = Some(id);
                }
            }
        }
        let retired = retired.unwrap();
        assert_eq!(arena.retire_sibling(retired), Some(500));
        let before_ids = arena.ids().collect::<std::vec::Vec<_>>();
        let before_values = arena.iter().copied().collect::<std::vec::Vec<_>>();
        let before_len = arena.len();
        let before = before_ids[1];

        assert_eq!(
            rebalance_insert(&mut arena, primary, Some(before), 9_999).unwrap_err(),
            9_999
        );
        assert_eq!(arena.ids().collect::<std::vec::Vec<_>>(), before_ids);
        assert_eq!(
            arena.iter().copied().collect::<std::vec::Vec<_>>(),
            before_values
        );
        assert_eq!(arena.len(), before_len);

        assert!(arena.reclaim_retired_sibling(retired));
        let result = rebalance_insert(&mut arena, primary, Some(before), 9_999).unwrap();
        assert!(!result.remaps.is_empty());
        assert_eq!(arena.get(next_primary), Some(&20_000));
        let values = arena.iter().copied().collect::<std::vec::Vec<_>>();
        assert_eq!(values[0..3], [0, 9_999, 1]);
        assert_eq!(values.last(), Some(&20_000));
    }
}
