//! Incremental cross-rule scheduler over the compiler-owned Radix AST.
//!
//! Initialization reads declaration-block records once. Ownership, effective
//! context, source identity, and adjacency stay authoritative in the AST
//! rather than being reconstructed into Nano-specific records.

use rocketcss_ast::{
    Declaration, EqIgnoringTombstones, Span,
    radix_ast::{
        Compilation, CssRulePayload, DeclarationBlockId, DeclarationBlockOwner, DeclarationPayload,
        EffectiveKeyId, MutationError, NestingRulePayload, RuleId, StyleRulePayload,
    },
};
use rocketcss_common::{RadixIdRemap, RadixInsertResult};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use super::declaration_ir::{
    CompactPropertyKey, DeclarationIrClassifier, MovementDomain, PropertyBloom,
};
use super::partial_selector::materialize_selector_union;

pub(super) fn stabilize<'ast>(
    compilation: &mut Compilation<'ast>,
    preserve_selector_compatibility: bool,
) -> Result<(), MutationError> {
    let mut state = CrossRuleState::from_compilation(compilation)?;
    state.run(compilation, preserve_selector_compatibility)?;
    state.finish(compilation);
    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct DeclarationOccurrenceIr {
    property_key: Option<CompactPropertyKey>,
    movement_domain: Option<MovementDomain>,
    live: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct DeclarationBlockIr {
    live_count: u32,
    property_bloom: PropertyBloom,
}

#[derive(Debug, Default)]
struct DeclarationIrStore<'ast> {
    classifier: DeclarationIrClassifier<'ast>,
    occurrences: std::vec::Vec<Option<DeclarationOccurrenceIr>>,
    blocks: FxHashMap<DeclarationBlockId, DeclarationBlockIr>,
}

impl<'ast> DeclarationIrStore<'ast> {
    fn freeze_block(
        &mut self,
        compilation: &Compilation<'ast>,
        block: DeclarationBlockId,
    ) -> Result<(), MutationError> {
        let mut summary = DeclarationBlockIr::default();
        for (declaration, record) in compilation.declaration_occurrences_in_block(block)? {
            let (property_key, movement_domain, live) = match record.payload() {
                DeclarationPayload::Property(value) => {
                    let live = !matches!(value, Declaration::Tombstone);
                    let property_key = live
                        .then(|| self.classifier.property_key(value, record.is_important()))
                        .flatten();
                    let movement_domain = live
                        .then(|| self.classifier.movement_domain(value))
                        .flatten();
                    (property_key, movement_domain, live)
                }
                DeclarationPayload::FontFace(_)
                | DeclarationPayload::FontPaletteValues(_)
                | DeclarationPayload::ViewTransition(_)
                | DeclarationPayload::FontFeature(_)
                | DeclarationPayload::PropertyRule(_) => (None, None, true),
            };
            if live {
                summary.live_count = summary
                    .live_count
                    .checked_add(1)
                    .expect("declaration count exceeds u32::MAX");
            }
            if let Some(key) = property_key {
                summary.property_bloom.insert(key);
            }
            if self.occurrences.len() <= declaration.index() {
                self.occurrences.resize(declaration.index() + 1, None);
            }
            self.occurrences[declaration.index()] = Some(DeclarationOccurrenceIr {
                property_key,
                movement_domain,
                live,
            });
        }
        self.blocks.insert(block, summary);
        Ok(())
    }

    #[cfg(test)]
    fn occurrence_count(&self) -> usize {
        self.occurrences.iter().flatten().count()
    }

    #[cfg(test)]
    fn matchable_count(&self) -> usize {
        self.occurrences
            .iter()
            .flatten()
            .filter(|occurrence| occurrence.property_key.is_some())
            .count()
    }

    #[cfg(test)]
    fn live_count(&self) -> usize {
        self.occurrences
            .iter()
            .flatten()
            .filter(|occurrence| occurrence.live)
            .count()
    }

    #[cfg(test)]
    fn movement_domain_count(&self) -> usize {
        self.occurrences
            .iter()
            .flatten()
            .filter(|occurrence| occurrence.movement_domain.is_some())
            .count()
    }

    fn compose(&mut self, left: DeclarationBlockId, right: DeclarationBlockId) {
        let left = self
            .blocks
            .remove(&left)
            .expect("an S1 endpoint has initialized declaration IR");
        let right = self
            .blocks
            .get_mut(&right)
            .expect("an S1 endpoint has initialized declaration IR");
        right.live_count = right
            .live_count
            .checked_add(left.live_count)
            .expect("declaration count exceeds u32::MAX");
        right.property_bloom.union_with(left.property_bloom);
    }

    fn occurrence(
        &self,
        declaration: rocketcss_ast::radix_ast::DeclarationId,
    ) -> Option<&DeclarationOccurrenceIr> {
        self.occurrences.get(declaration.index())?.as_ref()
    }

    fn live_declarations(
        &self,
        compilation: &Compilation<'_>,
        block: DeclarationBlockId,
    ) -> Result<SmallVec<[rocketcss_ast::radix_ast::DeclarationId; 8]>, MutationError> {
        Ok(compilation
            .declaration_occurrences_in_block(block)?
            .filter_map(|(declaration, _)| {
                self.occurrence(declaration)
                    .is_some_and(|occurrence| occurrence.live)
                    .then_some(declaration)
            })
            .collect())
    }

    fn mark_dead(
        &mut self,
        block: DeclarationBlockId,
        declaration: rocketcss_ast::radix_ast::DeclarationId,
    ) {
        let occurrence = self.occurrences[declaration.index()]
            .as_mut()
            .expect("an authored declaration has initialized IR");
        if !occurrence.live {
            return;
        }
        occurrence.live = false;
        self.blocks
            .get_mut(&block)
            .expect("a live block has initialized IR")
            .live_count -= 1;
    }

    fn block_live_count(&self, block: DeclarationBlockId) -> u32 {
        self.blocks
            .get(&block)
            .map_or(0, |summary| summary.live_count)
    }

    fn property_bloom(&self, block: DeclarationBlockId) -> PropertyBloom {
        self.blocks
            .get(&block)
            .map_or(PropertyBloom::default(), |summary| summary.property_bloom)
    }

    fn repair_block_remaps(&mut self, remaps: &[RadixIdRemap<DeclarationBlockId>]) {
        if remaps.is_empty() {
            return;
        }
        self.blocks = std::mem::take(&mut self.blocks)
            .into_iter()
            .map(|(block, summary)| (remap_block_id(block, remaps), summary))
            .collect();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Candidate {
    left: DeclarationBlockId,
    right: DeclarationBlockId,
    left_revision: u32,
    right_revision: u32,
    same_effective_key: bool,
    may_share_declaration: bool,
}

#[derive(Debug, Default)]
struct SameSelectorCandidateList {
    pending: VecDeque<Candidate>,
    queued: FxHashSet<Candidate>,
}

impl SameSelectorCandidateList {
    fn push(&mut self, candidate: Candidate) {
        if self.queued.insert(candidate) {
            self.pending.push_back(candidate);
        }
    }

    fn pop(&mut self) -> Option<Candidate> {
        let candidate = self.pending.pop_front()?;
        self.queued.remove(&candidate);
        Some(candidate)
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    fn remap_blocks(&mut self, remaps: &[RadixIdRemap<DeclarationBlockId>]) {
        for candidate in &mut self.pending {
            candidate.remap_blocks(remaps);
        }
        self.queued.clear();
        self.queued.extend(self.pending.iter().copied());
    }
}

#[derive(Debug, Default)]
struct PartialMergeCandidateList {
    pending: BinaryHeap<Reverse<Candidate>>,
    queued: FxHashSet<Candidate>,
}

impl PartialMergeCandidateList {
    fn push(&mut self, candidate: Candidate) {
        if self.queued.insert(candidate) {
            self.pending.push(Reverse(candidate));
        }
    }

    fn pop(&mut self) -> Option<Candidate> {
        let candidate = self.pending.pop()?.0;
        self.queued.remove(&candidate);
        Some(candidate)
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    fn remap_blocks(&mut self, remaps: &[RadixIdRemap<DeclarationBlockId>]) {
        let mut repaired = BinaryHeap::with_capacity(self.pending.len());
        for Reverse(mut candidate) in std::mem::take(&mut self.pending).into_vec() {
            candidate.remap_blocks(remaps);
            repaired.push(Reverse(candidate));
        }
        self.pending = repaired;
        self.queued.clear();
        self.queued
            .extend(self.pending.iter().map(|candidate| candidate.0));
    }
}

#[derive(Debug, Default)]
struct DeclarationOverrideCandidateList {
    pending: VecDeque<EffectiveKeyId>,
    queued: FxHashSet<EffectiveKeyId>,
}

impl DeclarationOverrideCandidateList {
    fn push(&mut self, key: EffectiveKeyId) {
        if self.queued.insert(key) {
            self.pending.push_back(key);
        }
    }

    fn pop(&mut self) -> Option<EffectiveKeyId> {
        let key = self.pending.pop_front()?;
        self.queued.remove(&key);
        Some(key)
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

#[derive(Debug, Default)]
struct CrossRuleState<'ast> {
    declaration_ir: DeclarationIrStore<'ast>,
    // The authored common case is one occurrence per key. Keep that one ID
    // inline and allocate a repeated-key history only when S2 can use it.
    histories: FxHashMap<EffectiveKeyId, SmallVec<[DeclarationBlockId; 1]>>,
    direct_style_edges: std::vec::Vec<Candidate>,
    same_selector_candidates: SameSelectorCandidateList,
    declaration_override_candidates: DeclarationOverrideCandidateList,
    partial_merge_candidates: PartialMergeCandidateList,
    block_scan_count: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct S2Stats {
    declarations_removed: usize,
    empty_rules_retired: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct S3Stats {
    reused_left_commits: usize,
    allocated_shared_commits: usize,
    rejected_no_common: usize,
    rejected_unsafe_movement: usize,
    rejected_capacity: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SchedulerStats {
    s1_commits: usize,
    s2: S2Stats,
    s3: S3Stats,
}

#[derive(Clone, Copy, Debug)]
struct CommonDeclaration {
    left: rocketcss_ast::radix_ast::DeclarationId,
    right: rocketcss_ast::radix_ast::DeclarationId,
    right_order: usize,
}

impl<'ast> CrossRuleState<'ast> {
    fn from_compilation(compilation: &Compilation<'ast>) -> Result<Self, MutationError> {
        let mut state = Self::default();
        for (block_id, block) in compilation.declaration_blocks_in_source_order() {
            if !block.is_live() {
                continue;
            }
            state.block_scan_count += 1;
            state
                .histories
                .entry(block.effective_key())
                .or_default()
                .push(block_id);
            state.declaration_ir.freeze_block(compilation, block_id)?;

            let DeclarationBlockOwner::Rule(owner) = block.owner();
            let owner = compilation
                .rule(owner)
                .ok_or(MutationError::InvalidRuleTopology(owner))?;
            if !is_style_owner(owner.payload()) {
                continue;
            }
            let Some(previous_rule_id) = owner.previous_sibling() else {
                continue;
            };
            let previous_rule = compilation
                .rule(previous_rule_id)
                .ok_or(MutationError::InvalidRuleTopology(previous_rule_id))?;
            if !is_style_owner(previous_rule.payload()) {
                continue;
            }
            let Some(previous_block_id) = previous_rule.declaration_block() else {
                return Err(MutationError::InvalidRuleTopology(previous_rule_id));
            };
            let previous_block = compilation
                .declaration_block(previous_block_id)
                .ok_or(MutationError::UnknownDeclarationBlock(previous_block_id))?;
            state.enqueue_edge_candidate(Candidate {
                left: previous_block_id,
                right: block_id,
                left_revision: previous_block.revision(),
                right_revision: block.revision(),
                same_effective_key: previous_block.effective_key() == block.effective_key(),
                may_share_declaration: state
                    .declaration_ir
                    .property_bloom(previous_block_id)
                    .may_share_declaration(state.declaration_ir.property_bloom(block_id)),
            });
        }
        let dirty_keys = state
            .histories
            .iter()
            .filter_map(|(&key, history)| (history.len() > 1).then_some(key))
            .collect::<SmallVec<[_; 8]>>();
        for key in dirty_keys {
            state.declaration_override_candidates.push(key);
        }
        Ok(state)
    }

    fn run(
        &mut self,
        compilation: &mut Compilation<'ast>,
        preserve_selector_compatibility: bool,
    ) -> Result<SchedulerStats, MutationError> {
        let mut stats = SchedulerStats::default();
        loop {
            stats.s1_commits += self.run_s1(compilation)?;
            let s2 = self.run_s2_exact(compilation)?;
            stats.s2.declarations_removed += s2.declarations_removed;
            stats.s2.empty_rules_retired += s2.empty_rules_retired;
            if !self.same_selector_candidates.is_empty() {
                continue;
            }

            let s3 = self.run_s3(compilation, preserve_selector_compatibility)?;
            stats.s3.reused_left_commits += s3.reused_left_commits;
            stats.s3.allocated_shared_commits += s3.allocated_shared_commits;
            stats.s3.rejected_no_common += s3.rejected_no_common;
            stats.s3.rejected_unsafe_movement += s3.rejected_unsafe_movement;
            stats.s3.rejected_capacity += s3.rejected_capacity;

            if self.same_selector_candidates.is_empty()
                && self.declaration_override_candidates.is_empty()
                && self.partial_merge_candidates.is_empty()
            {
                return Ok(stats);
            }
        }
    }

    /// Terminal S5 boundary for the currently implemented exact-only model.
    ///
    /// S1-S3 commit `Range`/`Local4`/`Overflow` representations atomically.
    /// Complex partially-live effects return `NoChange`, so there is no S4
    /// deferred plan to materialize yet. Consuming `self` tears down every
    /// merge-only queue, history, summary, and revision sidecar. Debug builds
    /// also prove that the committed AST is structurally complete.
    fn finish(self, _compilation: &Compilation<'ast>) {
        debug_assert!(self.same_selector_candidates.is_empty());
        debug_assert!(self.declaration_override_candidates.is_empty());
        debug_assert!(self.partial_merge_candidates.is_empty());
        #[cfg(debug_assertions)]
        debug_assert_eq!(_compilation.validate_ast(), Ok(()));
    }

    fn run_s1(&mut self, compilation: &mut Compilation<'ast>) -> Result<usize, MutationError> {
        let mut commits = 0;
        while let Some(candidate) = self.same_selector_candidates.pop() {
            let Some((left_rule, right_rule, key)) = validate_s1(compilation, candidate) else {
                continue;
            };
            let merged =
                compilation.merge_adjacent_rule_declaration_blocks(left_rule, right_rule)?;
            debug_assert_eq!(merged.effective_key, key);
            self.declaration_ir
                .compose(merged.retired_block, merged.retained_block);
            let history = self
                .histories
                .get_mut(&key)
                .expect("an initialized live block has a key history");
            history.retain(|block| *block != merged.retired_block);
            commits += 1;

            let retained = compilation
                .rule(merged.retained_rule)
                .expect("the retained S1 rule remains live");
            for (left, right) in [
                retained
                    .previous_sibling()
                    .map(|previous| (previous, merged.retained_rule)),
                retained
                    .next_sibling()
                    .map(|next| (merged.retained_rule, next)),
            ]
            .into_iter()
            .flatten()
            {
                if let Some(edge) = self.edge_candidate(compilation, left, right) {
                    self.enqueue_edge_candidate(edge);
                }
            }
            self.declaration_override_candidates.push(key);
        }
        Ok(commits)
    }

    fn run_s2_exact(
        &mut self,
        compilation: &mut Compilation<'ast>,
    ) -> Result<S2Stats, MutationError> {
        let mut stats = S2Stats::default();
        while let Some(key) = self.declaration_override_candidates.pop() {
            let Some(history) = self.histories.get(&key).cloned() else {
                continue;
            };
            let mut affected_blocks = FxHashSet::default();
            let mut previous_by_property: FxHashMap<
                CompactPropertyKey,
                (DeclarationBlockId, rocketcss_ast::radix_ast::DeclarationId),
            > = FxHashMap::default();
            for block in history {
                let declarations = compilation
                    .declaration_occurrences_in_block(block)?
                    .map(|(id, _)| id)
                    .collect::<std::vec::Vec<_>>();
                for declaration in declarations {
                    let Some(property_key) = self
                        .declaration_ir
                        .occurrence(declaration)
                        .filter(|occurrence| occurrence.live)
                        .and_then(|occurrence| occurrence.property_key)
                    else {
                        continue;
                    };
                    if let Some(&(previous_block, previous)) =
                        previous_by_property.get(&property_key)
                        && declarations_are_exactly_equal(compilation, previous, declaration)
                    {
                        compilation.replace_declaration(
                            previous_block,
                            previous,
                            DeclarationPayload::Property(Declaration::Tombstone),
                        )?;
                        self.declaration_ir.mark_dead(previous_block, previous);
                        affected_blocks.insert(previous_block);
                        stats.declarations_removed += 1;
                    }
                    previous_by_property.insert(property_key, (block, declaration));
                }
            }

            for block in affected_blocks {
                self.declaration_ir.freeze_block(compilation, block)?;
                let Some(block_record) = compilation.declaration_block(block) else {
                    continue;
                };
                let DeclarationBlockOwner::Rule(owner) = block_record.owner();
                let block_key = block_record.effective_key();
                if self.declaration_ir.block_live_count(block) == 0
                    && compilation
                        .rule(owner)
                        .is_some_and(|rule| rule.child_list().is_none())
                {
                    let retired = compilation.retire_rule(owner)?;
                    self.remove_history_occurrence(block_key, block);
                    if let (Some(previous), Some(next)) = (retired.previous, retired.next)
                        && let Some(edge) = self.edge_candidate(compilation, previous, next)
                    {
                        self.enqueue_edge_candidate(edge);
                    }
                    stats.empty_rules_retired += 1;
                } else {
                    self.enqueue_rule_incident_edges(compilation, owner);
                }
            }
            if !self.same_selector_candidates.is_empty() {
                break;
            }
        }
        Ok(stats)
    }

    fn run_s3(
        &mut self,
        compilation: &mut Compilation<'ast>,
        preserve_selector_compatibility: bool,
    ) -> Result<S3Stats, MutationError> {
        let mut stats = S3Stats::default();
        while let Some(candidate) = self.partial_merge_candidates.pop() {
            let Some(endpoints) = validate_s3(compilation, candidate) else {
                continue;
            };
            let left_declarations = self
                .declaration_ir
                .live_declarations(compilation, candidate.left)?;
            let right_declarations = self
                .declaration_ir
                .live_declarations(compilation, candidate.right)?;
            if left_declarations.is_empty() || right_declarations.is_empty() {
                continue;
            }

            let mut matched_right = FxHashSet::default();
            let mut common = SmallVec::<[CommonDeclaration; 8]>::new();
            for &left in &left_declarations {
                let Some(left_ir) = self.declaration_ir.occurrence(left) else {
                    continue;
                };
                let Some(property_key) = left_ir.property_key else {
                    continue;
                };
                if let Some((right_order, &right)) =
                    right_declarations.iter().enumerate().find(|(_, right)| {
                        !matched_right.contains(&**right)
                            && self
                                .declaration_ir
                                .occurrence(**right)
                                .is_some_and(|right| right.property_key == Some(property_key))
                            && declarations_have_equal_effect(compilation, left, **right)
                    })
                {
                    matched_right.insert(right);
                    common.push(CommonDeclaration {
                        left,
                        right,
                        right_order,
                    });
                }
            }
            if common.is_empty() {
                stats.rejected_no_common += 1;
                continue;
            }
            let matched_left = common
                .iter()
                .map(|common| common.left)
                .collect::<FxHashSet<_>>();
            let left_residual = left_declarations
                .iter()
                .copied()
                .filter(|declaration| !matched_left.contains(declaration))
                .collect::<SmallVec<[_; 8]>>();
            let right_residual = right_declarations
                .iter()
                .copied()
                .filter(|declaration| !matched_right.contains(declaration))
                .collect::<SmallVec<[_; 8]>>();
            if !radix_partial_movement_is_safe(
                &common,
                &left_residual,
                &right_residual,
                &self.declaration_ir,
            ) {
                stats.rejected_unsafe_movement += 1;
                continue;
            }

            let Some(selectors) = materialize_selector_union(
                compilation
                    .selector_value(endpoints.left_selector)
                    .expect("a validated selector value remains resolvable")
                    .selectors(),
                compilation
                    .selector_value(endpoints.right_selector)
                    .expect("a validated selector value remains resolvable")
                    .selectors(),
                preserve_selector_compatibility,
            ) else {
                continue;
            };
            let selector_value = compilation.intern_selector_value(
                selectors,
                endpoints.selector_kind,
                endpoints.vendor_prefix,
            )?;
            let Some(shared_key) = compilation.intern_selector_union_effective_key(
                endpoints.left_key,
                endpoints.right_key,
                selector_value,
            )?
            else {
                continue;
            };

            if left_residual.is_empty() {
                compilation.replace_rule_selector_value(endpoints.left_rule, selector_value)?;
                let left_payload = compilation
                    .rule_mut(endpoints.left_rule)
                    .expect("the reused S3 endpoint remains live")
                    .payload_mut();
                match left_payload {
                    CssRulePayload::Style(payload) => payload.span = endpoints.span,
                    CssRulePayload::Nesting(payload) => payload.span = endpoints.span,
                    _ => unreachable!("the S3 selector owner was validated"),
                }

                for declaration in &common {
                    compilation.replace_declaration(
                        candidate.right,
                        declaration.right,
                        DeclarationPayload::Property(Declaration::Tombstone),
                    )?;
                    self.declaration_ir
                        .mark_dead(candidate.right, declaration.right);
                }
                self.declaration_ir
                    .freeze_block(compilation, candidate.right)?;
                debug_assert_eq!(
                    compilation
                        .declaration_block(candidate.left)
                        .expect("the reused S3 block remains live")
                        .effective_key(),
                    shared_key
                );
                self.remove_history_occurrence(endpoints.left_key, candidate.left);
                self.insert_history_occurrence(shared_key, candidate.left);

                let retain_right = self.declaration_ir.block_live_count(candidate.right) != 0
                    || compilation
                        .rule(endpoints.right_rule)
                        .is_some_and(|rule| rule.child_list().is_some());
                if !retain_right {
                    compilation.retire_rule(endpoints.right_rule)?;
                    self.remove_history_occurrence(endpoints.right_key, candidate.right);
                }
                self.publish_incident_edges(
                    compilation,
                    endpoints.left_rule,
                    retain_right.then_some(endpoints.right_rule),
                );
                self.declaration_override_candidates
                    .push(endpoints.left_key);
                self.declaration_override_candidates
                    .push(endpoints.right_key);
                self.declaration_override_candidates.push(shared_key);
                stats.reused_left_commits += 1;
                break;
            }

            let before_block = first_block_after_rule_in_source(
                compilation,
                endpoints.left_rule,
                endpoints.right_rule,
            )?;
            if !compilation.can_insert_declaration_block_between(
                candidate.left,
                Some(before_block),
                common.len(),
            ) {
                stats.rejected_capacity += 1;
                continue;
            }
            let payload = match endpoints.selector_kind {
                rocketcss_ast::radix_ast::SelectorFrameKind::Style => {
                    CssRulePayload::Style(StyleRulePayload {
                        span: endpoints.span,
                        selector_value,
                        vendor_prefix: endpoints.vendor_prefix,
                    })
                }
                rocketcss_ast::radix_ast::SelectorFrameKind::Nesting => {
                    CssRulePayload::Nesting(NestingRulePayload {
                        span: endpoints.span,
                        selector_value,
                    })
                }
            };
            let rule_result = match compilation.insert_rule_after(endpoints.left_rule, payload) {
                Ok(result) => result,
                Err(
                    MutationError::LocalRuleCapacityExhausted(_)
                    | MutationError::PrimaryRuleCapacityExhausted,
                ) => {
                    stats.rejected_capacity += 1;
                    continue;
                }
                Err(error) => return Err(error),
            };
            let left_rule = remap_rule_id(endpoints.left_rule, &rule_result.remaps);
            let right_rule = remap_rule_id(endpoints.right_rule, &rule_result.remaps);
            let shared_rule = rule_result.id;
            let block_result = compilation.insert_declaration_block_between(
                candidate.left,
                Some(before_block),
                shared_rule,
                shared_key,
            )?;
            self.repair_block_remaps(&block_result);
            let left_block = remap_block_id(candidate.left, &block_result.remaps);
            let right_block = remap_block_id(candidate.right, &block_result.remaps);
            let shared_block = block_result.id;

            for declaration in &common {
                let important = compilation
                    .declaration(declaration.left)
                    .ok_or(MutationError::UnknownDeclaration(declaration.left))?
                    .is_important();
                let moved = compilation.replace_declaration(
                    left_block,
                    declaration.left,
                    DeclarationPayload::Property(Declaration::Tombstone),
                )?;
                compilation.replace_declaration(
                    right_block,
                    declaration.right,
                    DeclarationPayload::Property(Declaration::Tombstone),
                )?;
                self.declaration_ir.mark_dead(left_block, declaration.left);
                self.declaration_ir
                    .mark_dead(right_block, declaration.right);
                compilation.append_declaration(shared_block, moved, important)?;
            }
            self.declaration_ir.freeze_block(compilation, left_block)?;
            self.declaration_ir.freeze_block(compilation, right_block)?;
            self.declaration_ir
                .freeze_block(compilation, shared_block)?;
            self.insert_history_occurrence(shared_key, shared_block);

            let retain_right = self.declaration_ir.block_live_count(right_block) != 0
                || compilation
                    .rule(right_rule)
                    .is_some_and(|rule| rule.child_list().is_some());
            if !retain_right {
                compilation.retire_rule(right_rule)?;
                self.remove_history_occurrence(endpoints.right_key, right_block);
            }
            self.publish_incident_edges(
                compilation,
                shared_rule,
                retain_right.then_some(right_rule),
            );
            self.declaration_override_candidates
                .push(endpoints.left_key);
            self.declaration_override_candidates
                .push(endpoints.right_key);
            self.declaration_override_candidates.push(shared_key);
            debug_assert_eq!(
                compilation
                    .rule(shared_rule)
                    .and_then(|rule| rule.previous_sibling()),
                Some(left_rule)
            );
            stats.allocated_shared_commits += 1;
            break;
        }
        Ok(stats)
    }

    fn remove_history_occurrence(&mut self, key: EffectiveKeyId, block: DeclarationBlockId) {
        let remove_key = if let Some(history) = self.histories.get_mut(&key) {
            history.retain(|candidate| *candidate != block);
            history.is_empty()
        } else {
            false
        };
        if remove_key {
            self.histories.remove(&key);
        }
    }

    fn insert_history_occurrence(&mut self, key: EffectiveKeyId, block: DeclarationBlockId) {
        let history = self.histories.entry(key).or_default();
        let index = history.binary_search(&block).unwrap_or_else(|index| index);
        if history.get(index) != Some(&block) {
            history.insert(index, block);
        }
    }

    fn publish_incident_edges(
        &mut self,
        compilation: &Compilation<'_>,
        left: RuleId,
        right: Option<RuleId>,
    ) {
        let previous = compilation
            .rule(left)
            .and_then(|rule| rule.previous_sibling());
        for edge in [
            previous.and_then(|previous| self.edge_candidate(compilation, previous, left)),
            compilation
                .rule(left)
                .and_then(|rule| rule.next_sibling())
                .and_then(|next| self.edge_candidate(compilation, left, next)),
            right.and_then(|right| {
                compilation
                    .rule(right)
                    .and_then(|rule| rule.next_sibling())
                    .and_then(|next| self.edge_candidate(compilation, right, next))
            }),
        ]
        .into_iter()
        .flatten()
        {
            self.enqueue_edge_candidate(edge);
        }
    }

    fn enqueue_rule_incident_edges(&mut self, compilation: &Compilation<'_>, rule: RuleId) {
        let Some(record) = compilation.rule(rule) else {
            return;
        };
        for edge in [
            record
                .previous_sibling()
                .and_then(|previous| self.edge_candidate(compilation, previous, rule)),
            record
                .next_sibling()
                .and_then(|next| self.edge_candidate(compilation, rule, next)),
        ]
        .into_iter()
        .flatten()
        {
            self.enqueue_edge_candidate(edge);
        }
    }

    fn enqueue_edge_candidate(&mut self, edge: Candidate) {
        self.direct_style_edges.push(edge);
        if edge.same_effective_key {
            self.same_selector_candidates.push(edge);
        } else if edge.may_share_declaration {
            self.partial_merge_candidates.push(edge);
        }
    }

    fn repair_block_remaps(&mut self, result: &RadixInsertResult<DeclarationBlockId>) {
        if result.remaps.is_empty() {
            return;
        }
        self.declaration_ir.repair_block_remaps(&result.remaps);
        for history in self.histories.values_mut() {
            for block in history {
                *block = remap_block_id(*block, &result.remaps);
            }
        }
        for edge in &mut self.direct_style_edges {
            edge.remap_blocks(&result.remaps);
        }
        self.same_selector_candidates.remap_blocks(&result.remaps);
        self.partial_merge_candidates.remap_blocks(&result.remaps);
    }

    fn edge_candidate(
        &self,
        compilation: &Compilation<'_>,
        left: RuleId,
        right: RuleId,
    ) -> Option<Candidate> {
        let left_rule = compilation.rule(left)?;
        let right_rule = compilation.rule(right)?;
        if !left_rule.is_live()
            || !right_rule.is_live()
            || left_rule.next_sibling() != Some(right)
            || right_rule.previous_sibling() != Some(left)
            || !is_style_owner(left_rule.payload())
            || !is_style_owner(right_rule.payload())
        {
            return None;
        }
        let left = left_rule.declaration_block()?;
        let right = right_rule.declaration_block()?;
        let left_block = compilation.declaration_block(left)?;
        let right_block = compilation.declaration_block(right)?;
        Some(Candidate {
            left,
            right,
            left_revision: left_block.revision(),
            right_revision: right_block.revision(),
            same_effective_key: left_block.effective_key() == right_block.effective_key(),
            may_share_declaration: self
                .declaration_ir
                .property_bloom(left)
                .may_share_declaration(self.declaration_ir.property_bloom(right)),
        })
    }
}

impl Candidate {
    fn remap_blocks(&mut self, remaps: &[RadixIdRemap<DeclarationBlockId>]) {
        self.left = remap_block_id(self.left, remaps);
        self.right = remap_block_id(self.right, remaps);
    }
}

fn is_style_owner(payload: &CssRulePayload<'_>) -> bool {
    matches!(
        payload,
        CssRulePayload::Style(_) | CssRulePayload::Nesting(_)
    )
}

fn first_block_after_rule_in_source(
    compilation: &Compilation<'_>,
    left: RuleId,
    right: RuleId,
) -> Result<DeclarationBlockId, MutationError> {
    let mut current = compilation
        .rule(left)
        .ok_or(MutationError::UnknownRule(left))?
        .next_in_source();
    while let Some(rule) = current {
        let record = compilation
            .rule(rule)
            .ok_or(MutationError::InvalidRuleTopology(left))?;
        if let Some(block) = record.declaration_block() {
            return Ok(block);
        }
        if rule == right {
            break;
        }
        current = record.next_in_source();
    }
    Err(MutationError::InvalidRuleTopology(left))
}

fn remap_rule_id(id: RuleId, remaps: &[RadixIdRemap<RuleId>]) -> RuleId {
    remaps
        .iter()
        .find_map(|remap| (remap.old == id).then_some(remap.new))
        .unwrap_or(id)
}

fn remap_block_id(
    id: DeclarationBlockId,
    remaps: &[RadixIdRemap<DeclarationBlockId>],
) -> DeclarationBlockId {
    remaps
        .iter()
        .find_map(|remap| (remap.old == id).then_some(remap.new))
        .unwrap_or(id)
}

fn validate_s1(
    compilation: &Compilation<'_>,
    candidate: Candidate,
) -> Option<(RuleId, RuleId, EffectiveKeyId)> {
    let left_block = compilation.declaration_block(candidate.left)?;
    let right_block = compilation.declaration_block(candidate.right)?;
    if !left_block.is_live()
        || !right_block.is_live()
        || left_block.revision() != candidate.left_revision
        || right_block.revision() != candidate.right_revision
        || left_block.effective_key() != right_block.effective_key()
    {
        return None;
    }
    let DeclarationBlockOwner::Rule(left) = left_block.owner();
    let DeclarationBlockOwner::Rule(right) = right_block.owner();
    let left_rule = compilation.rule(left)?;
    let right_rule = compilation.rule(right)?;
    if left_rule.child_list().is_some()
        || left_rule.next_sibling() != Some(right)
        || right_rule.previous_sibling() != Some(left)
        || !is_style_owner(left_rule.payload())
        || !is_style_owner(right_rule.payload())
    {
        return None;
    }
    Some((left, right, left_block.effective_key()))
}

fn declarations_are_exactly_equal(
    compilation: &Compilation<'_>,
    left: rocketcss_ast::radix_ast::DeclarationId,
    right: rocketcss_ast::radix_ast::DeclarationId,
) -> bool {
    let Some(left) = compilation.declaration(left) else {
        return false;
    };
    let Some(right) = compilation.declaration(right) else {
        return false;
    };
    left.is_important() == right.is_important() && left.payload() == right.payload()
}

#[derive(Clone, Copy)]
struct RadixS3Endpoints {
    left_rule: RuleId,
    right_rule: RuleId,
    left_key: EffectiveKeyId,
    right_key: EffectiveKeyId,
    left_selector: rocketcss_ast::radix_ast::SelectorValueId,
    right_selector: rocketcss_ast::radix_ast::SelectorValueId,
    selector_kind: rocketcss_ast::radix_ast::SelectorFrameKind,
    vendor_prefix: rocketcss_ast::VendorPrefix,
    span: Span,
}

fn validate_s3(compilation: &Compilation<'_>, candidate: Candidate) -> Option<RadixS3Endpoints> {
    let left_block = compilation.declaration_block(candidate.left)?;
    let right_block = compilation.declaration_block(candidate.right)?;
    if !left_block.is_live()
        || !right_block.is_live()
        || left_block.revision() != candidate.left_revision
        || right_block.revision() != candidate.right_revision
        || left_block.effective_key() == right_block.effective_key()
    {
        return None;
    }
    let DeclarationBlockOwner::Rule(left_rule_id) = left_block.owner();
    let DeclarationBlockOwner::Rule(right_rule_id) = right_block.owner();
    let left_rule = compilation.rule(left_rule_id)?;
    let right_rule = compilation.rule(right_rule_id)?;
    if left_rule.child_list().is_some()
        || left_rule.next_sibling() != Some(right_rule_id)
        || right_rule.previous_sibling() != Some(left_rule_id)
    {
        return None;
    }
    let (left_selector, left_span) = match left_rule.payload() {
        CssRulePayload::Style(payload) => (payload.selector_value, payload.span),
        CssRulePayload::Nesting(payload) => (payload.selector_value, payload.span),
        _ => return None,
    };
    let (right_selector, right_span) = match right_rule.payload() {
        CssRulePayload::Style(payload) => (payload.selector_value, payload.span),
        CssRulePayload::Nesting(payload) => (payload.selector_value, payload.span),
        _ => return None,
    };
    let left_value = compilation.selector_value(left_selector)?;
    let right_value = compilation.selector_value(right_selector)?;
    if left_selector == right_selector
        || left_value.kind() != right_value.kind()
        || left_value.vendor_prefix() != right_value.vendor_prefix()
    {
        return None;
    }
    Some(RadixS3Endpoints {
        left_rule: left_rule_id,
        right_rule: right_rule_id,
        left_key: left_block.effective_key(),
        right_key: right_block.effective_key(),
        left_selector,
        right_selector,
        selector_kind: left_value.kind(),
        vendor_prefix: left_value.vendor_prefix(),
        span: Span::new(left_span.start, right_span.end),
    })
}

fn declarations_have_equal_effect(
    compilation: &Compilation<'_>,
    left: rocketcss_ast::radix_ast::DeclarationId,
    right: rocketcss_ast::radix_ast::DeclarationId,
) -> bool {
    let Some(left) = compilation.declaration(left) else {
        return false;
    };
    let Some(right) = compilation.declaration(right) else {
        return false;
    };
    if left.is_important() != right.is_important() {
        return false;
    }
    match (left.payload(), right.payload()) {
        (DeclarationPayload::Property(left), DeclarationPayload::Property(right)) => {
            left.eq_ignoring_tombstones(right)
        }
        _ => false,
    }
}

fn radix_partial_movement_is_safe(
    common: &[CommonDeclaration],
    left_residual: &[rocketcss_ast::radix_ast::DeclarationId],
    right_residual: &[rocketcss_ast::radix_ast::DeclarationId],
    declaration_ir: &DeclarationIrStore<'_>,
) -> bool {
    let common_domains = common
        .iter()
        .map(|common| declaration_ir.occurrence(common.left)?.movement_domain)
        .collect::<Option<SmallVec<[_; 8]>>>();
    if left_residual.is_empty() && right_residual.is_empty() {
        let Some(common_domains) = common_domains else {
            return common
                .windows(2)
                .all(|pair| pair[0].right_order < pair[1].right_order);
        };
        return radix_common_effect_order_is_safe(common, &common_domains);
    }
    let Some(common_domains) = common_domains else {
        return false;
    };
    let residual_domains = left_residual
        .iter()
        .chain(right_residual)
        .map(|&declaration| declaration_ir.occurrence(declaration)?.movement_domain)
        .collect::<Option<SmallVec<[_; 8]>>>();
    let Some(residual_domains) = residual_domains else {
        return false;
    };
    if common_domains.iter().any(|common| {
        residual_domains
            .iter()
            .any(|residual| common.overlaps(residual))
    }) {
        return false;
    }
    radix_common_effect_order_is_safe(common, &common_domains)
}

fn radix_common_effect_order_is_safe(
    common: &[CommonDeclaration],
    common_domains: &[MovementDomain],
) -> bool {
    for left in 0..common.len() {
        for right in left + 1..common.len() {
            if common[left].right_order > common[right].right_order
                && common_domains[left].overlaps(&common_domains[right])
            {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
    use rocketcss_common::Allocator;
    use rocketcss_parser::{Compiler, ParserOptions};

    use super::*;

    trait ParseTestCompilation<'ast> {
        fn parse_test_compilation(
            &mut self,
            source: &'ast str,
            options: ParserOptions<'ast>,
        ) -> Result<rocketcss_ast::Compilation<'ast>, rocketcss_parser::Error<'ast>>;
    }

    impl<'ast> ParseTestCompilation<'ast> for Compiler<'ast> {
        fn parse_test_compilation(
            &mut self,
            source: &'ast str,
            options: ParserOptions<'ast>,
        ) -> Result<rocketcss_ast::Compilation<'ast>, rocketcss_parser::Error<'ast>> {
            rocketcss_common::GhostToken::scope(|mut token| self.parse(source, &mut token, options))
        }
    }

    #[test]
    fn initializes_histories_ir_and_direct_edges_in_one_block_scan() {
        let allocator = Allocator::new();
        let compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{color:red}a{color:blue}@media print{a{color:red}a{color:blue}}b{color:red}b{color:blue}a{color:green}",
                ParserOptions::default(),
            )
            .unwrap();
        let state = CrossRuleState::from_compilation(&compilation).unwrap();

        assert_eq!(state.block_scan_count, 7);
        assert_eq!(state.declaration_ir.occurrence_count(), 7);
        assert_eq!(state.declaration_ir.live_count(), 7);
        assert_eq!(state.declaration_ir.matchable_count(), 7);
        assert_eq!(state.declaration_ir.movement_domain_count(), 7);
        assert_eq!(state.histories.len(), 3);
        assert_eq!(
            state
                .direct_style_edges
                .iter()
                .filter(|edge| edge.same_effective_key)
                .count(),
            3
        );
        assert_eq!(
            state
                .direct_style_edges
                .iter()
                .filter(|edge| !edge.same_effective_key)
                .count(),
            1
        );
        assert_eq!(
            state
                .direct_style_edges
                .iter()
                .filter(|edge| !edge.same_effective_key && edge.may_share_declaration)
                .count(),
            1
        );
        for (&key, blocks) in &state.histories {
            assert!(blocks.iter().all(|&block| {
                compilation
                    .declaration_block(block)
                    .is_some_and(|block| block.effective_key() == key)
            }));
        }
    }

    #[test]
    fn nested_declaration_segments_join_history_without_becoming_style_edges() {
        let allocator = Allocator::new();
        let compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{width:1px;& b{height:2px}width:3px}",
                ParserOptions::default(),
            )
            .unwrap();
        let state = CrossRuleState::from_compilation(&compilation).unwrap();

        assert_eq!(state.block_scan_count, 3);
        assert_eq!(state.declaration_ir.occurrence_count(), 3);
        assert!(state.direct_style_edges.is_empty());
        assert!(state.histories.values().any(|history| history.len() == 2));
    }

    #[test]
    fn s1_commits_directly_and_enqueues_the_newly_exposed_ast_edge() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{color:red}a{width:1px}a{height:2px}b{color:blue}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        assert_eq!(state.run_s1(&mut compilation).unwrap(), 2);
        let root = compilation.stylesheet().root_rules();
        let live = compilation
            .rules_in_list(root)
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>();
        assert_eq!(live.len(), 2);
        let retained = compilation
            .rule(live[0])
            .unwrap()
            .declaration_block()
            .unwrap();
        assert_eq!(
            compilation.declarations_in_block(retained).unwrap().count(),
            3
        );
        assert!(
            state
                .histories
                .values()
                .any(|history| history.as_slice() == [retained])
        );
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_tombstones_only_exact_old_declarations_in_the_same_ast_history() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{color:red;width:1px}b{color:red}a{color:red;width:2px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        let stats = state.run_s2_exact(&mut compilation).unwrap();
        assert_eq!(
            stats,
            S2Stats {
                declarations_removed: 1,
                empty_rules_retired: 0,
            }
        );
        let first_block = compilation
            .declaration_blocks_in_source_order()
            .next()
            .unwrap()
            .0;
        let declarations = compilation
            .declarations_in_block(first_block)
            .unwrap()
            .collect::<std::vec::Vec<_>>();
        assert!(matches!(
            declarations[0].payload(),
            DeclarationPayload::Property(Declaration::Tombstone)
        ));
        assert!(!matches!(
            declarations[1].payload(),
            DeclarationPayload::Property(Declaration::Tombstone)
        ));
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_retires_an_empty_leaf_rule_without_rebuilding_topology() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{color:red}b{width:1px}a{color:red}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        let stats = state.run_s2_exact(&mut compilation).unwrap();
        assert_eq!(stats.declarations_removed, 1);
        assert_eq!(stats.empty_rules_retired, 1);
        assert_eq!(
            compilation
                .rules_in_list(compilation.stylesheet().root_rules())
                .unwrap()
                .count(),
            2
        );
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_exposes_and_queues_only_the_new_local_s1_edge() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{color:red}b{width:1px}a{height:2px}b{width:1px}a{padding:0}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        assert_eq!(state.run_s1(&mut compilation).unwrap(), 0);
        let s2 = state.run_s2_exact(&mut compilation).unwrap();
        assert_eq!(s2.empty_rules_retired, 1);
        assert_eq!(state.run_s1(&mut compilation).unwrap(), 1);

        assert_eq!(
            compilation
                .rules_in_list(compilation.stylesheet().root_rules())
                .unwrap()
                .count(),
            3
        );
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_keeps_importance_context_mismatches_and_unparsed_values() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{color:red!important;unknown:x}a{color:red;unknown:x}",
                ParserOptions {
                    error_recovery: true,
                    ..ParserOptions::default()
                },
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        assert_eq!(
            state.run_s2_exact(&mut compilation).unwrap(),
            S2Stats::default()
        );
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s3_reuses_an_exhausted_left_block_with_order_independent_ir() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation(
                    "a{color:red;width:1px}b{width:1px;color:red;height:2px}",
                    options,
                )
                .unwrap();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            assert_eq!(
                state.run_s3(&mut compilation, true).unwrap(),
                S3Stats {
                    reused_left_commits: 1,
                    allocated_shared_commits: 0,
                    rejected_no_common: 0,
                    rejected_unsafe_movement: 0,
                    rejected_capacity: 0,
                }
            );
            let actual = compilation
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let expected = Compiler::new(&allocator)
                .parse("a,b{color:red;width:1px}b{height:2px}", &mut token, options)
                .unwrap()
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(compilation.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s3_inserts_a_shared_rule_and_block_at_their_final_radix_ids() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation("a{color:red;width:1px}b{color:red;height:2px}", options)
                .unwrap();
            let authored_declarations = compilation.declarations_in_source_order().count();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            assert_eq!(
                state.run_s3(&mut compilation, true).unwrap(),
                S3Stats {
                    reused_left_commits: 0,
                    allocated_shared_commits: 1,
                    rejected_no_common: 0,
                    rejected_unsafe_movement: 0,
                    rejected_capacity: 0,
                }
            );
            assert_eq!(
                compilation
                    .rules_in_list(compilation.stylesheet().root_rules())
                    .unwrap()
                    .count(),
                3
            );
            assert_eq!(
                compilation.declarations_in_source_order().count(),
                authored_declarations + 1
            );
            assert!(
                state
                    .histories
                    .values()
                    .all(|history| { history.windows(2).all(|pair| pair[0] < pair[1]) })
            );
            let actual = compilation
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let expected = Compiler::new(&allocator)
                .parse(
                    "a{width:1px}a,b{color:red}b{height:2px}",
                    &mut token,
                    options,
                )
                .unwrap()
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(compilation.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s3_insertion_retires_an_exhausted_right_endpoint_locally() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation("a{width:1px;color:red}b{color:red}", options)
                .unwrap();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            assert_eq!(
                state.run_s3(&mut compilation, true).unwrap(),
                S3Stats {
                    reused_left_commits: 0,
                    allocated_shared_commits: 1,
                    rejected_no_common: 0,
                    rejected_unsafe_movement: 0,
                    rejected_capacity: 0,
                }
            );
            assert_eq!(
                compilation
                    .rules_in_list(compilation.stylesheet().root_rules())
                    .unwrap()
                    .count(),
                2
            );
            let actual = compilation
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let expected = Compiler::new(&allocator)
                .parse("a{width:1px}a,b{color:red}", &mut token, options)
                .unwrap()
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(compilation.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn scheduler_returns_new_local_work_to_s1_and_s2_before_the_next_s3_edge() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation("a{color:red}b{color:red}a,b{width:1px}", options)
                .unwrap();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            let stats = state.run(&mut compilation, true).unwrap();
            assert_eq!(stats.s3.reused_left_commits, 1);
            assert_eq!(stats.s1_commits, 1);
            let actual = compilation
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let expected = Compiler::new(&allocator)
                .parse("a,b{color:red;width:1px}", &mut token, options)
                .unwrap()
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(compilation.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn scheduler_inserts_s3_history_in_semantic_order_for_s2() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation(
                    "a,b{width:1px}c{display:block}a{width:1px}b{width:1px}",
                    options,
                )
                .unwrap();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            let stats = state.run(&mut compilation, true).unwrap();
            assert_eq!(stats.s3.reused_left_commits, 1);
            assert_eq!(stats.s2.declarations_removed, 1);
            let actual = compilation
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let expected = Compiler::new(&allocator)
                .parse("c{display:block}a,b{width:1px}", &mut token, options)
                .unwrap()
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(compilation.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn scheduler_uses_the_s3_heap_for_overlapping_partial_edges() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation("a{color:red}b{color:red;width:1px}c{width:1px}", options)
                .unwrap();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            let stats = state.run(&mut compilation, true).unwrap();
            assert_eq!(stats.s3.reused_left_commits, 2);
            let actual = compilation
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let expected = Compiler::new(&allocator)
                .parse("a,b{color:red}b,c{width:1px}", &mut token, options)
                .unwrap()
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(compilation.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s1_after_s3_commits_a_noncontiguous_local4_representation() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation("a{width:1px;color:red}b{color:red}a,b{padding:0}", options)
                .unwrap();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            let stats = state.run(&mut compilation, true).unwrap();
            assert_eq!(stats.s3.allocated_shared_commits, 1);
            assert_eq!(stats.s1_commits, 1);
            assert!(
                compilation
                    .declaration_blocks_in_source_order()
                    .any(|(_, block)| block.is_live()
                        && matches!(
                            block.declarations(),
                            rocketcss_ast::radix_ast::DeclarationList::Local4(_)
                        ))
            );
            let actual = compilation
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let expected = Compiler::new(&allocator)
                .parse("a{width:1px}a,b{color:red;padding:0}", &mut token, options)
                .unwrap()
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(compilation.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s1_after_s3_commits_a_complete_arena_overflow_representation() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation(
                    "a{width:1px;color:red}b{color:red}a,b{padding:0;height:2px;opacity:.5}",
                    options,
                )
                .unwrap();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            let stats = state.run(&mut compilation, true).unwrap();
            assert_eq!(stats.s3.allocated_shared_commits, 1);
            assert_eq!(stats.s1_commits, 1);
            assert!(
                compilation
                    .declaration_blocks_in_source_order()
                    .any(|(_, block)| block.is_live()
                        && matches!(
                            block.declarations(),
                            rocketcss_ast::radix_ast::DeclarationList::Overflow(_)
                        ))
            );
            let actual = compilation
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let expected = Compiler::new(&allocator)
                .parse(
                    "a{width:1px}a,b{color:red;padding:0;height:2px;opacity:.5}",
                    &mut token,
                    options,
                )
                .unwrap()
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(compilation.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s3_rejects_reordered_overlapping_effect_domains() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{width:1px;min-width:2px}b{min-width:2px;width:1px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        assert_eq!(state.direct_style_edges.len(), 1);
        assert!(!state.direct_style_edges[0].same_effective_key);
        assert!(state.direct_style_edges[0].may_share_declaration);

        assert_eq!(
            state.run_s3(&mut compilation, true).unwrap(),
            S3Stats {
                reused_left_commits: 0,
                allocated_shared_commits: 0,
                rejected_no_common: 0,
                rejected_unsafe_movement: 1,
                rejected_capacity: 0,
            }
        );
        assert_eq!(compilation.validate_ast(), Ok(()));
    }
}
