//! Incremental cross-rule scheduler over the compiler-owned Radix AST.
//!
//! Declaration summaries are published at the end of each local block pass.
//! Finalization consumes that publication tape plus the final context remap;
//! ownership, effective context, source identity, and adjacency stay
//! authoritative in the AST rather than being reconstructed into Nano records.

use rocketcss_ast::{
    Declaration, EqIgnoringTombstones, Span,
    radix_ast::{
        Compilation, CssRulePayload, DeclarationBlockId, DeclarationBlockOwner, DeclarationPayload,
        EffectiveKeyId, MutationError, NestingRulePayload, RuleId, StyleRulePayload,
    },
};
use rocketcss_common::{
    Allocator, RadixIdRemap, RadixInsertResult,
    prelude::{HashMap, HashSet, Vec},
};
use std::{cmp::Reverse, collections::VecDeque};

use super::declaration_ir::{CompactPropertyKey, DeclarationIrStore, MovementDomain};
use super::partial_selector::materialize_selector_union;

pub(crate) struct CrossRuleBuilder<'arena, 'ast> {
    state: CrossRuleState<'arena, 'ast>,
}

impl<'arena, 'ast> CrossRuleBuilder<'arena, 'ast> {
    pub(super) fn new(compilation: &Compilation<'ast>, allocator: &'arena Allocator) -> Self {
        Self {
            state: CrossRuleState::new_in(compilation, allocator),
        }
    }

    pub(super) fn publish_block(
        &mut self,
        compilation: &Compilation<'ast>,
        block: DeclarationBlockId,
    ) -> Result<(), MutationError> {
        self.state.publish_block(compilation, block)
    }

    pub(super) fn finalize(&mut self, key_remaps: &[EffectiveKeyId]) {
        self.state.finalize_published_blocks(key_remaps);
    }
}

pub(super) fn stabilize_with_builder<'arena, 'ast>(
    mut builder: CrossRuleBuilder<'arena, 'ast>,
    compilation: &mut Compilation<'ast>,
    preserve_selector_compatibility: bool,
) -> Result<(), MutationError> {
    builder
        .state
        .run(compilation, preserve_selector_compatibility)?;
    builder.state.finish(compilation);
    Ok(())
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

#[derive(Debug)]
struct SameSelectorCandidateList<'arena> {
    pending: VecDeque<Candidate>,
    queued: HashSet<'arena, Candidate>,
}

impl<'arena> SameSelectorCandidateList<'arena> {
    fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            pending: VecDeque::new(),
            queued: HashSet::new_in(allocator),
        }
    }

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

#[derive(Debug)]
struct PartialMergeCandidateList<'arena> {
    pending: Vec<'arena, Reverse<Candidate>>,
    queued: HashSet<'arena, Candidate>,
}

impl<'arena> PartialMergeCandidateList<'arena> {
    fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            pending: allocator.vec(),
            queued: HashSet::new_in(allocator),
        }
    }

    fn push(&mut self, candidate: Candidate) {
        if self.queued.insert(candidate) {
            self.pending.push(Reverse(candidate));
            self.sift_up(self.pending.len() - 1);
        }
    }

    fn pop(&mut self) -> Option<Candidate> {
        let candidate = self.pending.first()?.0;
        let last = self.pending.pop().expect("the heap head exists");
        if !self.pending.is_empty() {
            self.pending[0] = last;
            self.sift_down(0);
        }
        self.queued.remove(&candidate);
        Some(candidate)
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    fn remap_blocks(&mut self, remaps: &[RadixIdRemap<DeclarationBlockId>]) {
        for Reverse(candidate) in self.pending.iter_mut() {
            candidate.remap_blocks(remaps);
        }
        for index in (0..self.pending.len() / 2).rev() {
            self.sift_down(index);
        }
        self.queued.clear();
        for candidate in &self.pending {
            self.queued.insert(candidate.0);
        }
    }

    fn sift_up(&mut self, mut index: usize) {
        while index != 0 {
            let parent = (index - 1) / 2;
            if self.pending[parent].0 <= self.pending[index].0 {
                break;
            }
            self.pending.swap(parent, index);
            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left = index * 2 + 1;
            let Some(right) = left.checked_add(1) else {
                break;
            };
            let mut smallest = index;
            if self
                .pending
                .get(left)
                .is_some_and(|candidate| candidate.0 < self.pending[smallest].0)
            {
                smallest = left;
            }
            if self
                .pending
                .get(right)
                .is_some_and(|candidate| candidate.0 < self.pending[smallest].0)
            {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            self.pending.swap(index, smallest);
            index = smallest;
        }
    }
}

#[derive(Debug)]
struct DeclarationOverrideCandidateList<'arena> {
    pending: VecDeque<EffectiveKeyId>,
    queued: HashSet<'arena, EffectiveKeyId>,
}

impl<'arena> DeclarationOverrideCandidateList<'arena> {
    fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            pending: VecDeque::new(),
            queued: HashSet::new_in(allocator),
        }
    }

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

#[derive(Debug)]
struct MinifyScratch<'arena> {
    history: Vec<'arena, DeclarationBlockId>,
    left_declarations: Vec<'arena, rocketcss_ast::radix_ast::DeclarationId>,
    right_declarations: Vec<'arena, rocketcss_ast::radix_ast::DeclarationId>,
    left_residual: Vec<'arena, rocketcss_ast::radix_ast::DeclarationId>,
    right_residual: Vec<'arena, rocketcss_ast::radix_ast::DeclarationId>,
    common: Vec<'arena, CommonDeclaration>,
    matched_right: HashSet<'arena, rocketcss_ast::radix_ast::DeclarationId>,
    matched_left: HashSet<'arena, rocketcss_ast::radix_ast::DeclarationId>,
    affected_blocks: HashSet<'arena, DeclarationBlockId>,
    previous_by_property: HashMap<
        'arena,
        CompactPropertyKey,
        (DeclarationBlockId, rocketcss_ast::radix_ast::DeclarationId),
    >,
}

impl<'arena> MinifyScratch<'arena> {
    fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            history: allocator.vec(),
            left_declarations: allocator.vec(),
            right_declarations: allocator.vec(),
            left_residual: allocator.vec(),
            right_residual: allocator.vec(),
            common: allocator.vec(),
            matched_right: HashSet::new_in(allocator),
            matched_left: HashSet::new_in(allocator),
            affected_blocks: HashSet::new_in(allocator),
            previous_by_property: HashMap::new_in(allocator),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PublishedBlock {
    block: DeclarationBlockId,
    effective_key: EffectiveKeyId,
    revision: u32,
    previous_style_block: Option<(DeclarationBlockId, EffectiveKeyId, u32)>,
}

struct CrossRuleState<'arena, 'ast> {
    allocator: &'arena Allocator,
    declaration_ir: DeclarationIrStore<'arena, 'ast>,
    // The authored common case is one occurrence per key. Keep that one ID
    // inline in the arena vector and grow it only when S2 needs a history.
    histories: HashMap<'arena, EffectiveKeyId, Vec<'arena, DeclarationBlockId>>,
    published_blocks: Vec<'arena, PublishedBlock>,
    direct_style_edges: Vec<'arena, Candidate>,
    same_selector_candidates: SameSelectorCandidateList<'arena>,
    declaration_override_candidates: DeclarationOverrideCandidateList<'arena>,
    partial_merge_candidates: PartialMergeCandidateList<'arena>,
    scratch: MinifyScratch<'arena>,
    published_block_count: usize,
}

impl<'arena, 'ast> CrossRuleState<'arena, 'ast> {
    fn new_in(compilation: &Compilation<'ast>, allocator: &'arena Allocator) -> Self {
        let declaration_capacity = compilation.declarations_in_source_order().len();
        let block_capacity = compilation.declaration_block_count();
        Self {
            allocator,
            declaration_ir: DeclarationIrStore::new_in(
                allocator,
                declaration_capacity,
                block_capacity,
            ),
            histories: HashMap::with_capacity_in(block_capacity, allocator),
            published_blocks: Vec::with_capacity_in(block_capacity, allocator),
            direct_style_edges: Vec::with_capacity_in(block_capacity, allocator),
            same_selector_candidates: SameSelectorCandidateList::new_in(allocator),
            declaration_override_candidates: DeclarationOverrideCandidateList::new_in(allocator),
            partial_merge_candidates: PartialMergeCandidateList::new_in(allocator),
            scratch: MinifyScratch::new_in(allocator),
            published_block_count: 0,
        }
    }

    #[cfg(test)]
    fn publish_all_blocks(&mut self, compilation: &Compilation<'ast>) -> Result<(), MutationError> {
        for (block_id, block) in compilation.declaration_blocks_in_source_order() {
            if !block.is_live() {
                continue;
            }
            self.publish_block(compilation, block_id)?;
        }
        Ok(())
    }

    fn publish_block(
        &mut self,
        compilation: &Compilation<'ast>,
        block: DeclarationBlockId,
    ) -> Result<(), MutationError> {
        self.declaration_ir.freeze_block(compilation, block)?;
        let block_record = compilation
            .declaration_block(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?;
        let DeclarationBlockOwner::Rule(owner) = block_record.owner();
        let owner_record = compilation
            .rule(owner)
            .ok_or(MutationError::InvalidRuleTopology(owner))?;
        let previous_style_block = if is_style_owner(owner_record.payload())
            && let Some(previous) = owner_record.previous_sibling()
        {
            let previous_record = compilation
                .rule(previous)
                .ok_or(MutationError::InvalidRuleTopology(previous))?;
            if is_style_owner(previous_record.payload()) {
                let previous_block = previous_record
                    .declaration_block()
                    .ok_or(MutationError::InvalidRuleTopology(previous))?;
                let previous_block_record = compilation
                    .declaration_block(previous_block)
                    .ok_or(MutationError::UnknownDeclarationBlock(previous_block))?;
                Some((
                    previous_block,
                    previous_block_record.effective_key(),
                    previous_block_record.revision(),
                ))
            } else {
                None
            }
        } else {
            None
        };
        self.published_blocks.push(PublishedBlock {
            block,
            effective_key: block_record.effective_key(),
            revision: block_record.revision(),
            previous_style_block,
        });
        Ok(())
    }

    fn finalize_published_blocks(&mut self, key_remaps: &[EffectiveKeyId]) {
        self.published_block_count = 0;
        for index in 0..self.published_blocks.len() {
            let mut published = self.published_blocks[index];
            if !key_remaps.is_empty() {
                published.effective_key = key_remaps[published.effective_key.index()];
                if let Some((block, effective_key, revision)) = published.previous_style_block {
                    published.previous_style_block =
                        Some((block, key_remaps[effective_key.index()], revision));
                }
                self.published_blocks[index] = published;
            }
            self.published_block_count += 1;
            if self.insert_history_occurrence(published.effective_key, published.block) {
                self.declaration_override_candidates
                    .push(published.effective_key);
            }
            if let Some((previous_block, previous_key, previous_revision)) =
                published.previous_style_block
            {
                self.enqueue_edge_candidate(Candidate {
                    left: previous_block,
                    right: published.block,
                    left_revision: previous_revision,
                    right_revision: published.revision,
                    same_effective_key: previous_key == published.effective_key,
                    may_share_declaration: self
                        .declaration_ir
                        .property_bloom(previous_block)
                        .may_share_declaration(self.declaration_ir.property_bloom(published.block)),
                });
            }
        }
    }

    #[cfg(test)]
    fn from_compilation<'minify>(
        compilation: &Compilation<'ast>,
    ) -> Result<CrossRuleState<'minify, 'ast>, MutationError>
    where
        'ast: 'minify,
    {
        let mut state = CrossRuleState::new_in(compilation, compilation.allocator());
        state.publish_all_blocks(compilation)?;
        state.finalize_published_blocks(&[]);
        Ok(state)
    }
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

impl<'arena, 'ast> CrossRuleState<'arena, 'ast> {
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
            let Some(history) = self.histories.get(&key) else {
                continue;
            };
            self.scratch.history.clear();
            self.scratch.history.extend(history.iter().copied());
            self.scratch.affected_blocks.clear();
            self.scratch.previous_by_property.clear();
            for &block in &self.scratch.history {
                let declaration_count = compilation.declaration_ids_in_block(block)?.len();
                for index in 0..declaration_count {
                    let declaration = compilation.declaration_id_at_in_block(block, index)?;
                    let Some(property_key) = self
                        .declaration_ir
                        .occurrence(declaration)
                        .filter(|occurrence| occurrence.live)
                        .and_then(|occurrence| occurrence.property_key)
                    else {
                        continue;
                    };
                    if let Some(&(previous_block, previous)) =
                        self.scratch.previous_by_property.get(&property_key)
                        && declarations_are_exactly_equal(compilation, previous, declaration)
                    {
                        compilation.replace_declaration(
                            previous_block,
                            previous,
                            DeclarationPayload::Property(Declaration::Tombstone),
                        )?;
                        self.declaration_ir.mark_dead(previous_block, previous);
                        self.scratch.affected_blocks.insert(previous_block);
                        stats.declarations_removed += 1;
                    }
                    self.scratch
                        .previous_by_property
                        .insert(property_key, (block, declaration));
                }
            }

            self.scratch.history.clear();
            for &block in &self.scratch.affected_blocks {
                self.scratch.history.push(block);
            }
            for index in 0..self.scratch.history.len() {
                let block = self.scratch.history[index];
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
            self.declaration_ir.live_declarations(
                compilation,
                candidate.left,
                &mut self.scratch.left_declarations,
            )?;
            self.declaration_ir.live_declarations(
                compilation,
                candidate.right,
                &mut self.scratch.right_declarations,
            )?;
            if self.scratch.left_declarations.is_empty()
                || self.scratch.right_declarations.is_empty()
            {
                continue;
            }

            self.scratch.matched_right.clear();
            self.scratch.matched_left.clear();
            self.scratch.common.clear();
            for &left in &self.scratch.left_declarations {
                let Some(left_ir) = self.declaration_ir.occurrence(left) else {
                    continue;
                };
                let Some(property_key) = left_ir.property_key else {
                    continue;
                };
                if left_ir.movement_domain.is_some_and(|domain| {
                    has_opaque_domain_conflict(
                        &self.declaration_ir,
                        domain,
                        &self.scratch.left_declarations,
                    ) || has_opaque_domain_conflict(
                        &self.declaration_ir,
                        domain,
                        &self.scratch.right_declarations,
                    )
                }) {
                    continue;
                }
                let mut matched = false;
                if let Some(indexed) = self
                    .declaration_ir
                    .property_candidates(candidate.right, property_key)
                {
                    for indexed in indexed {
                        let right = indexed.declaration;
                        let right_order = indexed.order;
                        if self.scratch.matched_right.contains(&right)
                            || !self.scratch.right_declarations.contains(&right)
                            || !declarations_have_equal_effect(compilation, left, right)
                        {
                            continue;
                        }
                        self.scratch.matched_right.insert(right);
                        self.scratch.matched_left.insert(left);
                        self.scratch.common.push(CommonDeclaration {
                            left,
                            right,
                            right_order,
                        });
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    for (right_order, &right) in self.scratch.right_declarations.iter().enumerate()
                    {
                        if self.scratch.matched_right.contains(&right)
                            || self
                                .declaration_ir
                                .occurrence(right)
                                .is_none_or(|right| right.property_key != Some(property_key))
                            || !declarations_have_equal_effect(compilation, left, right)
                        {
                            continue;
                        }
                        self.scratch.matched_right.insert(right);
                        self.scratch.matched_left.insert(left);
                        self.scratch.common.push(CommonDeclaration {
                            left,
                            right,
                            right_order,
                        });
                        break;
                    }
                }
            }
            if self.scratch.common.is_empty() {
                stats.rejected_no_common += 1;
                continue;
            }
            self.scratch.left_residual.clear();
            self.scratch.right_residual.clear();
            for &declaration in &self.scratch.left_declarations {
                if !self.scratch.matched_left.contains(&declaration) {
                    self.scratch.left_residual.push(declaration);
                }
            }
            for &declaration in &self.scratch.right_declarations {
                if !self.scratch.matched_right.contains(&declaration) {
                    self.scratch.right_residual.push(declaration);
                }
            }
            if !radix_partial_movement_is_safe(
                &self.scratch.common,
                &self.scratch.left_residual,
                &self.scratch.right_residual,
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
                self.allocator,
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

            if self.scratch.left_residual.is_empty() {
                compilation.replace_rule_selector_value_in(
                    endpoints.left_rule,
                    selector_value,
                    self.allocator,
                )?;
                let left_payload = compilation
                    .rule_mut(endpoints.left_rule)
                    .expect("the reused S3 endpoint remains live")
                    .payload_mut();
                match left_payload {
                    CssRulePayload::Style(payload) => payload.span = endpoints.span,
                    CssRulePayload::Nesting(payload) => payload.span = endpoints.span,
                    _ => unreachable!("the S3 selector owner was validated"),
                }

                for declaration in &self.scratch.common {
                    compilation.replace_declaration(
                        candidate.right,
                        declaration.right,
                        DeclarationPayload::Property(Declaration::Tombstone),
                    )?;
                    self.declaration_ir
                        .mark_dead(candidate.right, declaration.right);
                }
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
                self.scratch.common.len(),
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

            for declaration in &self.scratch.common {
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
                let moved = compilation.append_declaration(shared_block, moved, important)?;
                self.declaration_ir.publish_synthesized_declaration(
                    compilation,
                    shared_block,
                    moved,
                )?;
            }
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

    fn insert_history_occurrence(
        &mut self,
        key: EffectiveKeyId,
        block: DeclarationBlockId,
    ) -> bool {
        if !self.histories.contains_key(&key) {
            self.histories.insert(key, self.allocator.vec());
        }
        let history = self
            .histories
            .get_mut(&key)
            .expect("the history was inserted above");
        let index = history.binary_search(&block).unwrap_or_else(|index| index);
        if history.get(index) != Some(&block) {
            history.insert(index, block);
        }
        history.len() == 2
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
        for block in &mut self.published_blocks {
            block.block = remap_block_id(block.block, &result.remaps);
            if let Some((previous, key, revision)) = block.previous_style_block {
                block.previous_style_block =
                    Some((remap_block_id(previous, &result.remaps), key, revision));
            }
        }
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

fn has_opaque_domain_conflict(
    declaration_ir: &DeclarationIrStore<'_, '_>,
    domain: MovementDomain,
    declarations: &[rocketcss_ast::radix_ast::DeclarationId],
) -> bool {
    declarations.iter().any(|declaration| {
        let Some(occurrence) = declaration_ir.occurrence(*declaration) else {
            return false;
        };
        occurrence.property_key.is_none()
            && occurrence
                .movement_domain
                .is_some_and(|opaque_domain| domain.overlaps(&opaque_domain))
    })
}

fn radix_partial_movement_is_safe(
    common: &[CommonDeclaration],
    left_residual: &[rocketcss_ast::radix_ast::DeclarationId],
    right_residual: &[rocketcss_ast::radix_ast::DeclarationId],
    declaration_ir: &DeclarationIrStore<'_, '_>,
) -> bool {
    if left_residual.is_empty() && right_residual.is_empty() {
        if common.iter().any(|common| {
            declaration_ir
                .occurrence(common.left)
                .and_then(|occurrence| occurrence.movement_domain)
                .is_none()
        }) {
            return common
                .windows(2)
                .all(|pair| pair[0].right_order < pair[1].right_order);
        }
        return radix_common_effect_order_is_safe(common, declaration_ir);
    }
    for common in common {
        let Some(common_domain) = declaration_ir
            .occurrence(common.left)
            .and_then(|occurrence| occurrence.movement_domain)
        else {
            return false;
        };
        for &residual in left_residual.iter().chain(right_residual) {
            let Some(residual_domain) = declaration_ir
                .occurrence(residual)
                .and_then(|occurrence| occurrence.movement_domain)
            else {
                return false;
            };
            if common_domain.overlaps(&residual_domain) {
                return false;
            }
        }
    }
    radix_common_effect_order_is_safe(common, declaration_ir)
}

fn radix_common_effect_order_is_safe(
    common: &[CommonDeclaration],
    declaration_ir: &DeclarationIrStore<'_, '_>,
) -> bool {
    for left in 0..common.len() {
        for right in left + 1..common.len() {
            if common[left].right_order > common[right].right_order {
                let Some(left_domain) = declaration_ir
                    .occurrence(common[left].left)
                    .and_then(|occurrence| occurrence.movement_domain)
                else {
                    return false;
                };
                let Some(right_domain) = declaration_ir
                    .occurrence(common[right].left)
                    .and_then(|occurrence| occurrence.movement_domain)
                else {
                    return false;
                };
                if left_domain.overlaps(&right_domain) {
                    return false;
                }
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
    fn finalizes_histories_ir_and_direct_edges_from_published_metadata() {
        let allocator = Allocator::new();
        let compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                "a{color:red}a{color:blue}@media print{a{color:red}a{color:blue}}b{color:red}b{color:blue}a{color:green}",
                ParserOptions::default(),
            )
            .unwrap();
        let state = CrossRuleState::from_compilation(&compilation).unwrap();

        assert_eq!(state.published_block_count, 7);
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

        assert_eq!(state.published_block_count, 3);
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
