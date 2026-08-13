use super::*;

impl<'ast, R: Unpin, D, K> RadixCompilation<'ast, R, D, K> {
    /// Appends a new rule record and links it as the direct sibling after
    /// `after`. Dense identity is independent from semantic source order.
    pub fn insert_rule_after(
        &mut self,
        after: RuleId<'ast, R>,
        payload: R,
    ) -> Result<RuleId<'ast, R>, MutationError<'ast, R>> {
        let after_record = self
            .rules
            .try_get(after)
            .ok_or(MutationError::UnknownRule(after))?;
        if !after_record.live {
            return Err(MutationError::RetiredRule(after));
        }
        let parent = after_record.parent;
        let list = after_record.parent_list;
        let direct_before = after_record.next_sibling;
        let list_record = self
            .rule_lists
            .try_get(list)
            .ok_or(MutationError::UnknownRuleList(list))?;
        if direct_before.is_none() && list_record.last != Some(after) {
            return Err(MutationError::InvalidRuleTopology(after));
        }
        if direct_before.is_some_and(|before| {
            self.rules.try_get(before).is_none_or(|before| {
                !before.live || before.previous_sibling != Some(after) || before.parent_list != list
            })
        }) {
            return Err(MutationError::InvalidRuleTopology(after));
        }

        let logical_tail = self
            .subtree_tail(after)
            .ok_or(MutationError::InvalidRuleTopology(after))?;
        let storage_before = self.next_after_subtree(after);
        if direct_before.is_some() && storage_before != direct_before {
            return Err(MutationError::InvalidRuleTopology(after));
        }
        let mut anchor = logical_tail;
        loop {
            let next = self
                .rules
                .try_get(anchor)
                .ok_or(MutationError::InvalidRuleTopology(after))?
                .next_in_source;
            if next == storage_before {
                break;
            }
            let next = next.ok_or(MutationError::InvalidRuleTopology(after))?;
            if self.rules.try_get(next).is_none_or(|rule| rule.live) {
                return Err(MutationError::InvalidRuleTopology(after));
            }
            anchor = next;
        }
        if !self.rules.has_capacity_for(1) {
            return Err(MutationError::RuleCapacityExhausted);
        }
        let source_order_id = self.source_order_id_between(Some(anchor), storage_before)?;
        let inserted = self
            .rules
            .try_push(RuleRecord {
                payload,
                source_order_id,
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
            .map_err(|_| MutationError::RuleCapacityExhausted)?;
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

impl<'ast, R: Unpin, D, K> RadixCompilation<'ast, R, D, K> {
    /// Returns whether a terminal transform can allocate `additional`
    /// declaration records without publishing a partial mutation.
    #[inline]
    pub fn can_insert_transformed_declarations(&self, additional: usize) -> bool {
        self.declarations.has_capacity_for(additional)
    }

    /// Replaces one declaration at its exact chain position with a nonempty
    /// sequence and bumps the owning block revision once.
    ///
    /// Membership, topology, count overflow, and capacity are validated before
    /// any payload or link is changed. The original declaration ID is reused
    /// for the first replacement, which keeps authored positions stable and
    /// permits non-`Clone` payloads to move through the transaction.
    pub fn replace_declaration_with_sequence(
        &mut self,
        block: DeclarationBlockId<'ast, R>,
        declaration: DeclarationId<'ast>,
        replacements: std::vec::Vec<(D, bool)>,
    ) -> Result<D, MutationError<'ast, R>> {
        let block_record = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::UnknownDeclarationBlock(block));
        }
        if replacements.is_empty() {
            return Err(MutationError::InvalidDeclarationChain(block));
        }

        let first = block_record.first_declaration;
        let last = block_record.last_declaration;
        let declaration_count = block_record.declaration_count;
        let revision = block_record.revision.wrapping_add(1);
        let additional = replacements.len() - 1;
        let additional_u32 =
            u32::try_from(additional).map_err(|_| MutationError::InvalidDeclarationChain(block))?;
        let new_count = declaration_count
            .checked_add(additional_u32)
            .ok_or(MutationError::InvalidDeclarationChain(block))?;

        let mut current = first;
        let mut remaining = declaration_count;
        let mut found_successor = None;
        let mut observed_last = None;
        while let Some(current_id) = current {
            if remaining == 0 {
                return Err(MutationError::InvalidDeclarationChain(block));
            }
            let record = self
                .declarations
                .try_get(current_id)
                .ok_or(MutationError::UnknownDeclaration(current_id))?;
            if current_id == declaration {
                found_successor = Some(record.next_in_block);
            }
            observed_last = Some(current_id);
            current = record.next_in_block;
            remaining -= 1;
        }
        if remaining != 0 || observed_last != last {
            return Err(MutationError::InvalidDeclarationChain(block));
        }
        let successor = found_successor.ok_or(MutationError::UnknownDeclaration(declaration))?;
        if !self.declarations.has_capacity_for(additional) {
            return Err(MutationError::DeclarationCapacityExhausted);
        }

        let mut replacements = replacements.into_iter();
        let (first_payload, first_important) = replacements
            .next()
            .expect("a nonempty replacement sequence was validated");
        let original = {
            let record = self
                .declarations
                .try_get_mut(declaration)
                .expect("the replacement origin was validated before commit");
            record.important = first_important;
            std::mem::replace(&mut record.payload, first_payload)
        };

        let mut tail = declaration;
        for (payload, important) in replacements {
            let inserted = self.declarations.push(DeclarationRecord {
                payload,
                next_in_block: None,
                important,
            });
            self.declarations
                .try_get_mut(tail)
                .expect("a committed replacement tail remains resolvable")
                .next_in_block = Some(inserted);
            tail = inserted;
        }
        self.declarations
            .try_get_mut(tail)
            .expect("the final replacement tail remains resolvable")
            .next_in_block = successor;

        let block_record = self
            .declaration_blocks
            .try_get_mut(block)
            .expect("the replacement owner was validated before commit");
        block_record.declaration_count = new_count;
        if last == Some(declaration) {
            block_record.last_declaration = Some(tail);
        }
        block_record.revision = revision;
        Ok(original)
    }

    /// Rewrites a non-`Clone` payload into a sequence after one complete
    /// validation and capacity preflight.
    ///
    /// `placeholder` exists only while `rewrite` owns the original payload.
    /// The callback is infallible and must return exactly `additional + 1`
    /// records. The final block revision advances once, regardless of the
    /// number of declarations emitted.
    pub fn rewrite_declaration_with_sequence(
        &mut self,
        block: DeclarationBlockId<'ast, R>,
        declaration: DeclarationId<'ast>,
        additional: usize,
        placeholder: D,
        rewrite: impl FnOnce(D, bool) -> std::vec::Vec<(D, bool)>,
    ) -> Result<(), MutationError<'ast, R>> {
        let block_record = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::UnknownDeclarationBlock(block));
        }
        let mut current = block_record.first_declaration;
        let mut remaining = block_record.declaration_count;
        let mut contains_origin = false;
        let mut observed_last = None;
        while let Some(current_id) = current {
            if remaining == 0 {
                return Err(MutationError::InvalidDeclarationChain(block));
            }
            let record = self
                .declarations
                .try_get(current_id)
                .ok_or(MutationError::UnknownDeclaration(current_id))?;
            contains_origin |= current_id == declaration;
            observed_last = Some(current_id);
            current = record.next_in_block;
            remaining -= 1;
        }
        if remaining != 0 || observed_last != block_record.last_declaration {
            return Err(MutationError::InvalidDeclarationChain(block));
        }
        if !contains_origin {
            return Err(MutationError::UnknownDeclaration(declaration));
        }
        let final_revision = block_record.revision.wrapping_add(1);
        if !self.declarations.has_capacity_for(additional) {
            return Err(MutationError::DeclarationCapacityExhausted);
        }
        let important = self
            .declarations
            .try_get(declaration)
            .ok_or(MutationError::UnknownDeclaration(declaration))?
            .important;

        let original = self.replace_declaration(block, declaration, placeholder)?;
        let replacements = rewrite(original, important);
        assert_eq!(
            replacements.len(),
            additional + 1,
            "a declaration rewrite must emit its preflighted sequence length"
        );
        if self
            .replace_declaration_with_sequence(block, declaration, replacements)
            .is_err()
        {
            unreachable!("a preflighted declaration rewrite cannot fail during commit");
        }
        self.declaration_blocks
            .try_get_mut(block)
            .expect("the rewrite owner remains live through terminal commit")
            .revision = final_revision;
        Ok(())
    }

    /// Appends a transformed declaration to any live block chain.
    ///
    /// Authored parsing continues to use `append_declaration`, whose nonempty
    /// append frontier is closed by nested syntax. This transaction is for
    /// synthesized declarations and therefore ignores allocation adjacency.
    pub fn append_transformed_declaration(
        &mut self,
        block: DeclarationBlockId<'ast, R>,
        payload: D,
        important: bool,
    ) -> Result<DeclarationId<'ast>, MutationError<'ast, R>> {
        let block_record = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::UnknownDeclarationBlock(block));
        }
        let first = block_record.first_declaration;
        let last = block_record.last_declaration;
        let declaration_count = block_record.declaration_count;
        let revision = block_record.revision.wrapping_add(1);
        if first.is_none() != last.is_none() || (first.is_none() != (declaration_count == 0)) {
            return Err(MutationError::InvalidDeclarationChain(block));
        }
        if let Some(last) = last
            && self
                .declarations
                .try_get(last)
                .ok_or(MutationError::UnknownDeclaration(last))?
                .next_in_block
                .is_some()
        {
            return Err(MutationError::InvalidDeclarationChain(block));
        }
        if !self.declarations.has_capacity_for(1) {
            return Err(MutationError::DeclarationCapacityExhausted);
        }
        let inserted = self.declarations.push(DeclarationRecord {
            payload,
            next_in_block: None,
            important,
        });
        if let Some(last) = last {
            self.declarations
                .try_get_mut(last)
                .expect("the transformed tail was validated")
                .next_in_block = Some(inserted);
        } else {
            self.declaration_blocks
                .try_get_mut(block)
                .expect("the transformed declaration owner was validated")
                .first_declaration = Some(inserted);
        }
        let block_record = self
            .declaration_blocks
            .try_get_mut(block)
            .expect("the transformed declaration owner was validated");
        block_record.declaration_count = declaration_count
            .checked_add(1)
            .expect("a block cannot contain u32::MAX declarations");
        block_record.last_declaration = Some(inserted);
        block_record.revision = revision;
        Ok(inserted)
    }

    /// Preflights the remaining fallible storage operations for a synthesized
    /// block before its owner rule is inserted.
    pub fn can_insert_declaration_block(&self, declaration_count: usize) -> bool {
        self.declaration_blocks.has_capacity_for(1)
            && self.declarations.has_capacity_for(declaration_count)
    }

    /// Inserts a synthesized declaration block at its final semantic block ID
    /// and binds it to an already inserted live owner rule.
    pub fn insert_declaration_block(
        &mut self,
        owner: RuleId<'ast, R>,
        effective_key: EffectiveKeyId<'ast>,
    ) -> Result<DeclarationBlockId<'ast, R>, MutationError<'ast, R>> {
        let owner_record = self
            .rules
            .try_get(owner)
            .ok_or(MutationError::UnknownRule(owner))?;
        if !owner_record.live {
            return Err(MutationError::RetiredRule(owner));
        }
        if owner_record.declaration_block.is_some() {
            return Err(MutationError::DeclarationBlockAlreadyExists(owner));
        }
        if self.effective_keys.try_get(effective_key).is_none() {
            return Err(MutationError::UnknownEffectiveKey(effective_key));
        }
        let block = self
            .declaration_blocks
            .try_push(DeclarationBlockRecord {
                first_declaration: None,
                last_declaration: None,
                declaration_count: 0,
                owner: DeclarationBlockOwner::Rule(owner),
                effective_key,
                revision: 0,
                live: true,
            })
            .map_err(|_| MutationError::DeclarationBlockCapacityExhausted)?;
        self.rules
            .try_get_mut(owner)
            .expect("the synthesized block owner was validated before commit")
            .declaration_block = Some(block);
        Ok(block)
    }
}

impl<'ast, R: Unpin, D, K> RadixCompilation<'ast, R, D, K> {
    /// Replaces one declaration payload through its owning block and bumps the
    /// block revision used by incremental Nano candidates.
    pub fn replace_declaration(
        &mut self,
        block: DeclarationBlockId<'ast, R>,
        declaration: DeclarationId<'ast>,
        replacement: D,
    ) -> Result<D, MutationError<'ast, R>> {
        let block_record = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::UnknownDeclarationBlock(block));
        }
        let revision = block_record.revision.wrapping_add(1);
        if !self.declaration_chain_contains(block, block_record, declaration)? {
            return Err(MutationError::UnknownDeclaration(declaration));
        }
        let record = self
            .declarations
            .try_get_mut(declaration)
            .ok_or(MutationError::UnknownDeclaration(declaration))?;
        let previous = std::mem::replace(record.payload_mut(), replacement);
        self.declaration_blocks
            .try_get_mut(block)
            .expect("the declaration owner was validated before commit")
            .revision = revision;
        Ok(previous)
    }

    /// Mutates an iterator-created occurrence without rescanning membership.
    ///
    /// The token's block must remain live. Merging into that same retained
    /// block preserves the token because declaration chains only grow; retiring
    /// its block invalidates the token.
    #[doc(hidden)]
    pub fn validated_declaration_mut(
        &mut self,
        occurrence: DeclarationOccurrence<'ast, R>,
    ) -> Result<(&mut D, bool), MutationError<'ast, R>> {
        let block = occurrence.block;
        let declaration = occurrence.declaration;
        let block_record = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::UnknownDeclarationBlock(block));
        }
        let revision = block_record.revision.wrapping_add(1);
        let declaration_record = self
            .declarations
            .try_get_mut(declaration)
            .ok_or(MutationError::UnknownDeclaration(declaration))?;
        let important = declaration_record.important;
        self.declaration_blocks
            .try_get_mut(block)
            .expect("the validated occurrence block remains resolvable")
            .revision = revision;
        Ok((&mut declaration_record.payload, important))
    }

    /// Folds one direct leaf rule's declaration chain into its right sibling
    /// and retires the left rule in the same transaction.
    ///
    /// Semantic callers must additionally decide whether these rule kinds are
    /// mergeable. This storage transaction proves adjacency, equal AST-owned
    /// EffectiveKeys, unique block ownership, and valid chain endpoints before it
    /// publishes any mutation. Retired source-chain blocks between the live
    /// endpoints are absorbed as well; semantic callers are responsible for
    /// retiring those owners only after all of their occurrences are dead.
    pub fn merge_adjacent_rule_declaration_blocks(
        &mut self,
        left: RuleId<'ast, R>,
        right: RuleId<'ast, R>,
    ) -> Result<MergedAdjacentRuleBlocks<'ast, R>, MutationError<'ast, R>> {
        let left_rule = self
            .rules
            .try_get(left)
            .ok_or(MutationError::UnknownRule(left))?;
        let right_rule = self
            .rules
            .try_get(right)
            .ok_or(MutationError::UnknownRule(right))?;
        if !left_rule.live {
            return Err(MutationError::RetiredRule(left));
        }
        if !right_rule.live {
            return Err(MutationError::RetiredRule(right));
        }
        if left_rule.child_list.is_some() {
            return Err(MutationError::RuleHasChildren(left));
        }
        if left_rule.next_sibling != Some(right)
            || right_rule.previous_sibling != Some(left)
            || left_rule.parent != right_rule.parent
            || left_rule.parent_list != right_rule.parent_list
        {
            return Err(MutationError::InvalidRuleTopology(left));
        }
        let mut bridge_blocks = std::vec::Vec::new();
        let mut source_cursor = left_rule.next_in_source;
        while source_cursor != Some(right) {
            let source_rule = source_cursor
                .and_then(|id| self.rules.try_get(id).map(|rule| (id, rule)))
                .ok_or(MutationError::InvalidRuleTopology(left))?;
            if source_rule.1.live {
                return Err(MutationError::InvalidRuleTopology(source_rule.0));
            }
            if let Some(block) = source_rule.1.declaration_block {
                bridge_blocks.push(block);
            }
            source_cursor = source_rule.1.next_in_source;
        }
        let left_block_id = left_rule
            .declaration_block
            .ok_or(MutationError::InvalidRuleTopology(left))?;
        let right_block_id = right_rule
            .declaration_block
            .ok_or(MutationError::InvalidRuleTopology(right))?;
        let left_block = self
            .declaration_blocks
            .try_get(left_block_id)
            .ok_or(MutationError::UnknownDeclarationBlock(left_block_id))?;
        let right_block = self
            .declaration_blocks
            .try_get(right_block_id)
            .ok_or(MutationError::UnknownDeclarationBlock(right_block_id))?;
        if !left_block.live
            || !right_block.live
            || left_block.owner != DeclarationBlockOwner::Rule(left)
            || right_block.owner != DeclarationBlockOwner::Rule(right)
            || left_block.effective_key != right_block.effective_key
        {
            return Err(MutationError::InvalidRuleTopology(left));
        }
        let effective_key = left_block.effective_key;
        let mut block_chains = std::vec::Vec::new();
        block_chains.push((
            left_block_id,
            left_block.first_declaration,
            left_block.last_declaration,
            left_block.declaration_count,
        ));
        for &bridge in &bridge_blocks {
            let bridge_block = self
                .declaration_blocks
                .try_get(bridge)
                .ok_or(MutationError::UnknownDeclarationBlock(bridge))?;
            if bridge_block.live {
                return Err(MutationError::InvalidRuleTopology(left));
            }
            block_chains.push((
                bridge,
                bridge_block.first_declaration,
                bridge_block.last_declaration,
                bridge_block.declaration_count,
            ));
        }
        block_chains.push((
            right_block_id,
            right_block.first_declaration,
            right_block.last_declaration,
            right_block.declaration_count,
        ));

        let mut joins = std::vec::Vec::new();
        let mut merged_first = None;
        let mut merged_last = None;
        let mut merged_count = 0_u32;
        for &(block_id, first, last, count) in &block_chains {
            if count == 0 {
                if first.is_some() || last.is_some() {
                    return Err(MutationError::InvalidDeclarationChain(block_id));
                }
                continue;
            }
            let (Some(first), Some(last)) = (first, last) else {
                return Err(MutationError::InvalidDeclarationChain(block_id));
            };
            self.declarations
                .try_get(first)
                .ok_or(MutationError::UnknownDeclaration(first))?;
            let last_record = self
                .declarations
                .try_get(last)
                .ok_or(MutationError::UnknownDeclaration(last))?;
            if last_record.next_in_block.is_some() {
                return Err(MutationError::InvalidDeclarationChain(block_id));
            }
            merged_count = merged_count
                .checked_add(count)
                .ok_or(MutationError::InvalidDeclarationChain(block_id))?;
            let Some(previous_last) = merged_last else {
                merged_first = Some(first);
                merged_last = Some(last);
                continue;
            };
            joins.push((previous_last, first));
            merged_last = Some(last);
        }

        self.retire_rule(left)?;
        for (tail, next) in joins {
            let tail_record = self
                .declarations
                .try_get_mut(tail)
                .expect("all declaration joins were validated before commit");
            tail_record.next_in_block = Some(next);
        }
        for &(retired_block, _, _, _) in &block_chains[..block_chains.len().saturating_sub(1)] {
            let retired_block = self
                .declaration_blocks
                .try_get_mut(retired_block)
                .expect("a validated retired block remains a source tombstone");
            retired_block.first_declaration = None;
            retired_block.last_declaration = None;
            retired_block.declaration_count = 0;
            retired_block.live = false;
            retired_block.revision = retired_block.revision.wrapping_add(1);
        }
        let retained_block = self
            .declaration_blocks
            .try_get_mut(right_block_id)
            .expect("the retained block was validated before commit");
        retained_block.first_declaration = merged_first;
        retained_block.last_declaration = merged_last;
        retained_block.declaration_count = merged_count;
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
    /// declaration block is retired in the same transaction, while its chain
    /// continues to own the corresponding declaration occurrences.
    pub fn retire_rule(
        &mut self,
        id: RuleId<'ast, R>,
    ) -> Result<RetiredRule<'ast, R>, MutationError<'ast, R>> {
        let rule = self
            .rules
            .try_get(id)
            .ok_or(MutationError::UnknownRule(id))?;
        if !rule.live {
            return Err(MutationError::RetiredRule(id));
        }
        if rule.child_list.is_some() {
            return Err(MutationError::RuleHasChildren(id));
        }
        let list = rule.parent_list;
        let previous = rule.previous_sibling;
        let next = rule.next_sibling;
        let declaration_block = rule.declaration_block;

        let list_record = self
            .rule_lists
            .try_get(list)
            .ok_or(MutationError::UnknownRuleList(list))?;
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
            return Err(MutationError::InvalidRuleTopology(id));
        }
        if declaration_block.is_some_and(|block| {
            self.declaration_blocks
                .try_get(block)
                .is_none_or(|block| !block.live || block.owner != DeclarationBlockOwner::Rule(id))
        }) {
            return Err(MutationError::InvalidRuleTopology(id));
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
    /// Runs non-structural mutations through compact handles branded to one
    /// live declaration block. The higher-ranked scope prevents handles from
    /// escaping the callback.
    ///
    /// ```compile_fail
    /// use rocketcss_ast::{
    ///     Compilation, ConcreteDeclarationBlockId, ScopedDeclarationHandle,
    /// };
    ///
    /// fn escape<'ast>(
    ///     compilation: &mut Compilation<'ast>,
    ///     block: ConcreteDeclarationBlockId<'ast>,
    /// ) -> ScopedDeclarationHandle<'static, 'ast> {
    ///     compilation
    ///         .with_declaration_block_mutations(block, |scope| {
    ///             scope.declaration_handles().unwrap().next().unwrap()
    ///         })
    ///         .unwrap()
    /// }
    /// ```
    pub fn with_declaration_block_mutations<T>(
        &mut self,
        block: ConcreteDeclarationBlockId<'ast>,
        mutate: impl for<'scope> FnOnce(DeclarationBlockMutationScope<'scope, 'ast>) -> T,
    ) -> Result<T, ConcreteMutationError<'ast>> {
        let block_record = self
            .declaration_blocks
            .try_get(block)
            .ok_or(ConcreteMutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(ConcreteMutationError::UnknownDeclarationBlock(block));
        }
        self.declaration_chain_start_from_record(block, block_record)?;
        Ok(mutate(DeclarationBlockMutationScope {
            compilation: self,
            block,
        }))
    }

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
        declaration_id: DeclarationId<'ast>,
    ) -> Result<(&mut DeclarationPayload<'ast>, bool), ConcreteMutationError<'ast>> {
        let block_record = self
            .declaration_blocks
            .try_get(block)
            .ok_or(ConcreteMutationError::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(ConcreteMutationError::UnknownDeclarationBlock(block));
        }
        let revision = block_record.revision.wrapping_add(1);
        if !self.declaration_chain_contains(block, block_record, declaration_id)? {
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

    /// Resolves an iterator-created occurrence without rescanning block
    /// membership. Structural mutations preserve tokens for the retained block
    /// and invalidate tokens whose block has been retired.
    #[doc(hidden)]
    pub fn validated_declaration_occurrence_mut(
        &mut self,
        occurrence: ConcreteDeclarationOccurrence<'ast>,
    ) -> Result<(&mut DeclarationPayload<'ast>, bool), ConcreteMutationError<'ast>> {
        self.validated_declaration_mut(occurrence)
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
        declaration_id: DeclarationId<'ast>,
    ) -> Result<(&mut crate::Declaration<'ast>, bool), ConcreteMutationError<'ast>> {
        let (payload, important) = self.declaration_occurrence_mut(block, declaration_id)?;
        let DeclarationPayload::Property(payload) = payload else {
            return Err(ConcreteMutationError::UnknownDeclaration(declaration_id));
        };
        Ok((payload, important))
    }

    /// Mutates a property through an iterator-created occurrence token.
    pub fn validated_property_declaration_mut(
        &mut self,
        occurrence: ConcreteDeclarationOccurrence<'ast>,
    ) -> Result<(&mut crate::Declaration<'ast>, bool), ConcreteMutationError<'ast>> {
        let declaration_id = occurrence.declaration;
        let (payload, important) = self.validated_declaration_occurrence_mut(occurrence)?;
        let DeclarationPayload::Property(payload) = payload else {
            return Err(ConcreteMutationError::UnknownDeclaration(declaration_id));
        };
        Ok((payload, important))
    }
}

impl<'scope, 'ast> DeclarationBlockMutationScope<'scope, 'ast> {
    /// Iterates compact handles for this scope's block in semantic order.
    pub fn declaration_handles(
        &self,
    ) -> Result<
        impl ExactSizeIterator<Item = ScopedDeclarationHandle<'scope, 'ast>> + '_,
        ConcreteMutationError<'ast>,
    > {
        Ok(self
            .compilation
            .declaration_ids_in_block(self.block)?
            .map(|declaration| ScopedDeclarationHandle {
                declaration,
                marker: std::marker::PhantomData,
            }))
    }

    /// Resolves one handle for immutable inspection.
    #[inline]
    pub fn declaration(
        &self,
        handle: ScopedDeclarationHandle<'scope, 'ast>,
    ) -> Option<&DeclarationRecord<'ast, DeclarationPayload<'ast>>> {
        self.compilation.declaration(handle.declaration)
    }

    /// Mutates one property declaration without rescanning membership.
    #[inline]
    pub fn property_declaration_mut(
        &mut self,
        handle: ScopedDeclarationHandle<'scope, 'ast>,
    ) -> Result<(&mut crate::Declaration<'ast>, bool), ConcreteMutationError<'ast>> {
        self.compilation
            .validated_property_declaration_mut(DeclarationOccurrence {
                block: self.block,
                declaration: handle.declaration,
            })
    }

    #[inline]
    pub fn allocator(&self) -> &'ast Allocator {
        self.compilation.allocator()
    }
}
