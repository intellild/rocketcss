use super::*;

impl<R: Unpin, D, K> RadixCompilation<'_, R, D, K> {
    /// Appends a new rule record and links it as the direct sibling after
    /// `after`. Dense identity is independent from semantic source order.
    pub fn insert_rule_after(
        &mut self,
        after: RuleId<R>,
        payload: R,
    ) -> Result<RuleId<R>, MutationError<R>> {
        let after_record = self
            .rules
            .try_get(after)
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
            self.rules.try_get(before).is_none_or(|before| {
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
                .try_get(anchor)
                .ok_or(MutationError::<R>::InvalidRuleTopology(after))?
                .next_in_source;
            if next == storage_before {
                break;
            }
            let next = next.ok_or(MutationError::<R>::InvalidRuleTopology(after))?;
            if self.rules.try_get(next).is_none_or(|rule| rule.live) {
                return Err(MutationError::<R>::InvalidRuleTopology(after));
            }
            anchor = next;
        }
        let inserted = self
            .rules
            .try_push(RuleRecord {
                payload,
                parent,
                parent_list: list,
                previous_sibling: Some(after),
                next_sibling: direct_before,
                previous_in_source: Some(anchor),
                next_in_source: storage_before,
                child_list: None,
                declaration_block: None,
                revision: 0,
                live: true,
            })
            .map_err(|_| MutationError::<R>::RuleCapacityExhausted)?;
        self.rules
            .try_get_mut(after)
            .expect("the validated previous sibling remains resolvable")
            .next_sibling = Some(inserted);
        if let Some(before) = direct_before {
            self.rules
                .try_get_mut(before)
                .expect("the validated next sibling remains resolvable")
                .previous_sibling = Some(inserted);
        } else {
            self.rule_lists.get_mut(list).last = Some(inserted);
        }
        self.rule_lists.get_mut(list).live_len += 1;
        self.rules
            .try_get_mut(anchor)
            .expect("the validated source anchor remains resolvable")
            .next_in_source = Some(inserted);
        if let Some(before) = storage_before {
            self.rules
                .try_get_mut(before)
                .expect("the validated source successor remains resolvable")
                .previous_in_source = Some(inserted);
        } else {
            self.last_rule_in_source = Some(inserted);
        }
        Ok(inserted)
    }
}

impl<R: Unpin, D, K> RadixCompilation<'_, R, D, K> {
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
            .try_get(block)
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
            .try_get_mut(block)
            .expect("the transformed declaration owner was validated");
        block.declarations = representation;
        block.revision = revision;
        Ok(inserted)
    }

    /// Preflights the remaining fallible storage operations for a synthesized
    /// block before its owner rule is inserted.
    pub fn can_insert_declaration_block(&self, declaration_count: usize) -> bool {
        self.declaration_blocks.try_next_id().is_ok()
            && self
                .declarations
                .len()
                .checked_add(declaration_count)
                .is_some_and(|len| len <= u32::MAX as usize)
    }

    /// Inserts a synthesized declaration block at its final semantic block ID
    /// and binds it to an already inserted live owner rule.
    pub fn insert_declaration_block(
        &mut self,
        owner: RuleId<R>,
        effective_key: EffectiveKeyId,
    ) -> Result<DeclarationBlockId<R>, MutationError<R>> {
        let owner_record = self
            .rules
            .try_get(owner)
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
        let block = self
            .declaration_blocks
            .try_push(DeclarationBlockRecord::<R> {
                declarations: DeclarationList::Range(DeclarationRange {
                    start: self.declarations.len() as u32,
                    len: 0,
                }),
                owner: DeclarationBlockOwner::<R>::Rule(owner),
                effective_key,
                revision: 0,
                live: true,
            })
            .map_err(|_| MutationError::<R>::DeclarationBlockCapacityExhausted)?;
        self.rules
            .try_get_mut(owner)
            .expect("the synthesized block owner was validated before commit")
            .declaration_block = Some(block);
        Ok(block)
    }
}

impl<R: Unpin, D, K> RadixCompilation<'_, R, D, K> {
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
            .try_get(block)
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
            .try_get_mut(block)
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
            .try_get(left)
            .ok_or(MutationError::<R>::UnknownRule(left))?;
        let right_rule = self
            .rules
            .try_get(right)
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
                .and_then(|id| self.rules.try_get(id).map(|rule| (id, rule)))
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
            .try_get(left_block_id)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(left_block_id))?;
        let right_block = self
            .declaration_blocks
            .try_get(right_block_id)
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
                .try_get(bridge)
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
            .try_get_mut(left_block_id)
            .expect("the retired block remains a source tombstone")
            .declarations = DeclarationList::Range(DeclarationRange { start: 0, len: 0 });
        for bridge in bridge_blocks {
            self.declaration_blocks
                .try_get_mut(bridge)
                .expect("a validated bridge block remains a source tombstone")
                .declarations = DeclarationList::Range(DeclarationRange { start: 0, len: 0 });
        }
        let retained_block = self
            .declaration_blocks
            .try_get_mut(right_block_id)
            .expect("the retained block was validated before commit");
        retained_block.declarations = merged_declaration_list;
        retained_block.revision = retained_block.revision.wrapping_add(1);
        let retained_rule = self
            .rules
            .try_get_mut(right)
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
            .try_get(id)
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
                    .try_get(previous)
                    .is_none_or(|previous| previous.next_sibling != Some(id) || !previous.live)
            })
            || next.is_some_and(|next| {
                self.rules
                    .try_get(next)
                    .is_none_or(|next| next.previous_sibling != Some(id) || !next.live)
            })
        {
            return Err(MutationError::<R>::InvalidRuleTopology(id));
        }
        if declaration_block.is_some_and(|block| {
            self.declaration_blocks.try_get(block).is_none_or(|block| {
                !block.live || block.owner != DeclarationBlockOwner::<R>::Rule(id)
            })
        }) {
            return Err(MutationError::<R>::InvalidRuleTopology(id));
        }

        if let Some(previous) = previous {
            self.rules
                .try_get_mut(previous)
                .expect("the previous direct sibling was validated")
                .next_sibling = next;
        } else {
            self.rule_lists.get_mut(list).first = next;
        }
        if let Some(next) = next {
            self.rules
                .try_get_mut(next)
                .expect("the next direct sibling was validated")
                .previous_sibling = previous;
        } else {
            self.rule_lists.get_mut(list).last = previous;
        }
        self.rule_lists.get_mut(list).live_len -= 1;

        let rule = self
            .rules
            .try_get_mut(id)
            .expect("the retiring rule was validated");
        rule.previous_sibling = None;
        rule.next_sibling = None;
        rule.live = false;
        rule.revision = rule.revision.wrapping_add(1);
        if let Some(block) = declaration_block {
            let block = self
                .declaration_blocks
                .try_get_mut(block)
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
            .try_get_mut(rule_id)
            .ok_or(ConcreteMutationError::UnknownRule(rule_id))?;
        if !rule.live {
            return Err(ConcreteMutationError::RetiredRule(rule_id));
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
        block: ConcreteDeclarationBlockId<'ast>,
        declaration_id: DeclarationId,
    ) -> Result<(&mut DeclarationPayload<'ast>, bool), ConcreteMutationError<'ast>> {
        let block_record = self
            .declaration_blocks
            .try_get(block)
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
            .try_get_mut(declaration_id)
            .ok_or(ConcreteMutationError::UnknownDeclaration(declaration_id))?;
        let important = declaration.important;
        self.declaration_blocks
            .try_get_mut(block)
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
