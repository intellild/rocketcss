//! Incremental cross-rule scheduler over the stylesheet AST.
//!
//! Declaration summaries are published at the end of each local block pass.
//! Finalization consumes that publication tape plus the final context remap;
//! ownership, effective context, source identity, and adjacency stay
//! authoritative in the AST rather than being reconstructed into Nano records.

use rocketcss_ast::{
    CssDeclaration, CssDeclarationBlockId as DeclarationBlockId, CssRule, CssRuleId as RuleId,
    Declaration, DeclarationBlockOwner, DirectRuleContext, DirectRuleEdge, EffectiveKeyId,
    EqIgnoringTombstones, NestingRule, RuleMutationDelta, Span, StyleRule, StyleSheet,
    StyleSheetMutationError as MutationError,
};
use rocketcss_common::{
    Allocator, RadixIdRemap, RadixInsertResult,
    prelude::{HashMap, HashSet, Vec},
};
use std::{cmp::Reverse, collections::VecDeque};

use super::declaration_ir::{CompactPropertyKey, DeclarationIrStore, MovementDomain};
use super::partial_selector::materialize_selector_union;

type RuleEdge<'ast> = DirectRuleEdge<CssRule<'ast>>;
type RuleContext<'ast> = DirectRuleContext<CssRule<'ast>>;
type RuleDelta<'ast> = RuleMutationDelta<CssRule<'ast>>;

pub(crate) struct CrossRuleBuilder<'arena, 'ast> {
    state: CrossRuleState<'arena, 'ast>,
}

impl<'arena, 'ast> CrossRuleBuilder<'arena, 'ast> {
    pub(super) fn new(stylesheet: &StyleSheet<'ast>, allocator: &'arena Allocator) -> Self {
        Self {
            state: CrossRuleState::new_in(stylesheet, allocator),
        }
    }

    pub(super) fn publish_block(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
        block: DeclarationBlockId<'ast>,
    ) -> Result<(), MutationError<'ast>> {
        self.state.publish_block(stylesheet, block)
    }

    pub(super) fn finalize(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
        key_remaps: &[EffectiveKeyId],
    ) -> Result<(), MutationError<'ast>> {
        self.state.finalize_published_blocks(stylesheet, key_remaps)
    }
}

pub(super) fn stabilize_with_builder<'arena, 'ast>(
    mut builder: CrossRuleBuilder<'arena, 'ast>,
    stylesheet: &mut StyleSheet<'ast>,
    preserve_selector_compatibility: bool,
) -> Result<(), MutationError<'ast>> {
    builder
        .state
        .run(stylesheet, preserve_selector_compatibility)?;
    builder.state.finish(stylesheet);
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Candidate<'ast> {
    edge: RuleEdge<'ast>,
    left: DeclarationBlockId<'ast>,
    right: DeclarationBlockId<'ast>,
    left_revision: u32,
    right_revision: u32,
    same_effective_key: bool,
    may_share_declaration: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AffectedRule<'ast> {
    parent: Option<RuleId<'ast>>,
    owner: RuleId<'ast>,
    block: DeclarationBlockId<'ast>,
    effective_key: EffectiveKeyId,
    retire: bool,
}

#[derive(Debug)]
struct SameSelectorCandidateList<'arena, 'ast> {
    pending: VecDeque<Candidate<'ast>>,
    queued: HashSet<'arena, Candidate<'ast>>,
}

impl<'arena, 'ast> SameSelectorCandidateList<'arena, 'ast> {
    fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            pending: VecDeque::new(),
            queued: HashSet::new_in(allocator),
        }
    }

    fn push(&mut self, candidate: Candidate<'ast>) {
        if self.queued.insert(candidate) {
            self.pending.push_back(candidate);
        }
    }

    fn pop(&mut self) -> Option<Candidate<'ast>> {
        let candidate = self.pending.pop_front()?;
        self.queued.remove(&candidate);
        Some(candidate)
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    fn remap_blocks(&mut self, remaps: &[RadixIdRemap<DeclarationBlockId<'ast>>]) {
        for candidate in &mut self.pending {
            candidate.remap_blocks(remaps);
        }
        self.queued.clear();
        self.queued.extend(self.pending.iter().copied());
    }

    fn remap_rules(&mut self, remaps: &[RadixIdRemap<RuleId<'ast>>]) {
        for candidate in &mut self.pending {
            candidate.remap_rules(remaps);
        }
        self.queued.clear();
        self.queued.extend(self.pending.iter().copied());
    }
}

#[derive(Debug)]
struct PartialMergeCandidateList<'arena, 'ast> {
    pending: Vec<'arena, Reverse<Candidate<'ast>>>,
    queued: HashSet<'arena, Candidate<'ast>>,
}

impl<'arena, 'ast> PartialMergeCandidateList<'arena, 'ast> {
    fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            pending: allocator.vec(),
            queued: HashSet::new_in(allocator),
        }
    }

    fn push(&mut self, candidate: Candidate<'ast>) {
        if self.queued.insert(candidate) {
            self.pending.push(Reverse(candidate));
            self.sift_up(self.pending.len() - 1);
        }
    }

    fn pop(&mut self) -> Option<Candidate<'ast>> {
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

    fn remap_blocks(&mut self, remaps: &[RadixIdRemap<DeclarationBlockId<'ast>>]) {
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

    fn remap_rules(&mut self, remaps: &[RadixIdRemap<RuleId<'ast>>]) {
        for Reverse(candidate) in self.pending.iter_mut() {
            candidate.remap_rules(remaps);
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
struct MinifyScratch<'arena, 'ast> {
    history: Vec<'arena, DeclarationBlockId<'ast>>,
    left_declarations: Vec<'arena, rocketcss_ast::DeclarationId>,
    right_declarations: Vec<'arena, rocketcss_ast::DeclarationId>,
    left_residual: Vec<'arena, rocketcss_ast::DeclarationId>,
    right_residual: Vec<'arena, rocketcss_ast::DeclarationId>,
    common: Vec<'arena, CommonDeclaration>,
    matched_right: HashSet<'arena, rocketcss_ast::DeclarationId>,
    matched_left: HashSet<'arena, rocketcss_ast::DeclarationId>,
    affected_blocks: HashSet<'arena, DeclarationBlockId<'ast>>,
    affected_rules: Vec<'arena, AffectedRule<'ast>>,
    rule_contexts: Vec<'arena, RuleContext<'ast>>,
    mutation_deltas: Vec<'arena, RuleDelta<'ast>>,
    previous_by_property: HashMap<
        'arena,
        CompactPropertyKey,
        (DeclarationBlockId<'ast>, rocketcss_ast::DeclarationId),
    >,
}

impl<'arena, 'ast> MinifyScratch<'arena, 'ast> {
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
            affected_rules: allocator.vec(),
            rule_contexts: allocator.vec(),
            mutation_deltas: allocator.vec(),
            previous_by_property: HashMap::new_in(allocator),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PublishedBlock<'ast> {
    block: DeclarationBlockId<'ast>,
    effective_key: EffectiveKeyId,
}

struct CrossRuleState<'arena, 'ast> {
    allocator: &'arena Allocator,
    declaration_ir: DeclarationIrStore<'arena, 'ast>,
    // The authored common case is one occurrence per key. Keep that one ID
    // inline in the arena vector and grow it only when S2 needs a history.
    histories: HashMap<'arena, EffectiveKeyId, Vec<'arena, DeclarationBlockId<'ast>>>,
    published_blocks: Vec<'arena, PublishedBlock<'ast>>,
    direct_style_edges: Vec<'arena, Candidate<'ast>>,
    same_selector_candidates: SameSelectorCandidateList<'arena, 'ast>,
    declaration_override_candidates: DeclarationOverrideCandidateList<'arena>,
    partial_merge_candidates: PartialMergeCandidateList<'arena, 'ast>,
    scratch: MinifyScratch<'arena, 'ast>,
    published_block_count: usize,
}

impl<'arena, 'ast> CrossRuleState<'arena, 'ast> {
    fn new_in(stylesheet: &StyleSheet<'ast>, allocator: &'arena Allocator) -> Self {
        let declaration_capacity = stylesheet.declarations_in_source_order().len();
        let block_capacity = stylesheet.declaration_block_count();
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
    fn publish_all_blocks(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
    ) -> Result<(), MutationError<'ast>> {
        for (block_id, block) in stylesheet.declaration_blocks_in_source_order() {
            if !block.is_live() {
                continue;
            }
            self.publish_block(stylesheet, block_id)?;
        }
        Ok(())
    }

    fn publish_block(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
        block: DeclarationBlockId<'ast>,
    ) -> Result<(), MutationError<'ast>> {
        self.declaration_ir.freeze_block(stylesheet, block)?;
        let block_record = stylesheet
            .declaration_block(block)
            .ok_or(MutationError::<'ast>::UnknownDeclarationBlock(block))?;
        self.published_blocks.push(PublishedBlock {
            block,
            effective_key: block_record.effective_key(),
        });
        Ok(())
    }

    fn finalize_published_blocks(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
        key_remaps: &[EffectiveKeyId],
    ) -> Result<(), MutationError<'ast>> {
        self.published_block_count = 0;
        for index in 0..self.published_blocks.len() {
            let mut published = self.published_blocks[index];
            if !key_remaps.is_empty() {
                published.effective_key = key_remaps[published.effective_key.index()];
                self.published_blocks[index] = published;
            }
            self.published_block_count += 1;
            if self.insert_history_occurrence(published.effective_key, published.block) {
                self.declaration_override_candidates
                    .push(published.effective_key);
            }
        }
        self.enqueue_initial_edges(stylesheet)
    }

    fn enqueue_initial_edges(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
    ) -> Result<(), MutationError<'ast>> {
        let first = stylesheet.root_rules().next().map(|(rule, _)| rule);
        self.enqueue_rule_list_edges(stylesheet, stylesheet.root_rule_edges(), first)
    }

    fn enqueue_rule_list_edges(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
        edges: impl Iterator<Item = RuleEdge<'ast>>,
        first: Option<RuleId<'ast>>,
    ) -> Result<(), MutationError<'ast>> {
        let mut final_right = None;
        for edge in edges {
            self.enqueue_nested_rule_list_edges(stylesheet, edge.left())?;
            if let Some(candidate) = self.edge_candidate_from_edge(stylesheet, edge) {
                self.enqueue_edge_candidate(candidate);
            }
            final_right = Some(edge.right());
        }
        if let Some(rule) = final_right.or(first) {
            self.enqueue_nested_rule_list_edges(stylesheet, rule)?;
        }
        Ok(())
    }

    fn enqueue_nested_rule_list_edges(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
        parent: RuleId<'ast>,
    ) -> Result<(), MutationError<'ast>> {
        let first = stylesheet
            .nested_rules(parent)?
            .next()
            .map(|(rule, _)| rule);
        self.enqueue_rule_list_edges(stylesheet, stylesheet.nested_rule_edges(parent)?, first)
    }

    #[cfg(test)]
    fn from_stylesheet<'minify>(
        stylesheet: &StyleSheet<'ast>,
    ) -> Result<CrossRuleState<'minify, 'ast>, MutationError<'ast>>
    where
        'ast: 'minify,
    {
        let mut state = CrossRuleState::new_in(stylesheet, stylesheet.allocator());
        state.publish_all_blocks(stylesheet)?;
        state.finalize_published_blocks(stylesheet, &[])?;
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
    left: rocketcss_ast::DeclarationId,
    right: rocketcss_ast::DeclarationId,
    right_order: usize,
}

impl<'arena, 'ast> CrossRuleState<'arena, 'ast> {
    fn run(
        &mut self,
        stylesheet: &mut StyleSheet<'ast>,
        preserve_selector_compatibility: bool,
    ) -> Result<SchedulerStats, MutationError<'ast>> {
        let mut stats = SchedulerStats::default();
        loop {
            stats.s1_commits += self.run_s1(stylesheet)?;
            let s2 = self.run_s2_exact(stylesheet)?;
            stats.s2.declarations_removed += s2.declarations_removed;
            stats.s2.empty_rules_retired += s2.empty_rules_retired;
            if !self.same_selector_candidates.is_empty() {
                continue;
            }

            let s3 = self.run_s3(stylesheet, preserve_selector_compatibility)?;
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
    /// S1-S3 commit one semantic `RadixRange` per declaration block.
    /// Complex partially-live effects return `NoChange`, so there is no S4
    /// deferred plan to materialize yet. Consuming `self` tears down every
    /// merge-only queue, history, summary, and revision sidecar. Debug builds
    /// also prove that the committed AST is structurally complete.
    fn finish(self, _stylesheet: &StyleSheet<'ast>) {
        debug_assert!(self.same_selector_candidates.is_empty());
        debug_assert!(self.declaration_override_candidates.is_empty());
        debug_assert!(self.partial_merge_candidates.is_empty());
        #[cfg(debug_assertions)]
        debug_assert_eq!(_stylesheet.validate_ast(), Ok(()));
    }

    fn run_s1(&mut self, stylesheet: &mut StyleSheet<'ast>) -> Result<usize, MutationError<'ast>> {
        let mut commits = 0;
        while let Some(candidate) = self.same_selector_candidates.pop() {
            let Some(key) = validate_s1(stylesheet, candidate) else {
                continue;
            };
            let merged = stylesheet.merge_adjacent_rule_declaration_blocks(candidate.edge)?;
            debug_assert_eq!(merged.effective_key, key);
            self.declaration_ir
                .compose(merged.retired_block, merged.retained_block);
            let history = self
                .histories
                .get_mut(&key)
                .expect("an initialized live block has a key history");
            history.retain(|block| *block != merged.retired_block);
            commits += 1;
            self.enqueue_mutation_delta(stylesheet, merged.delta);
            self.declaration_override_candidates.push(key);
        }
        Ok(commits)
    }

    fn run_s2_exact(
        &mut self,
        stylesheet: &mut StyleSheet<'ast>,
    ) -> Result<S2Stats, MutationError<'ast>> {
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
                let declaration_count = stylesheet.declaration_ids_in_block(block)?.len();
                for index in 0..declaration_count {
                    let declaration = stylesheet.declaration_id_at_in_block(block, index)?;
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
                        && declarations_are_exactly_equal(stylesheet, previous, declaration)
                    {
                        stylesheet.replace_declaration(
                            previous_block,
                            previous,
                            CssDeclaration::Property(Declaration::Tombstone),
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

            self.scratch.affected_rules.clear();
            for &block in &self.scratch.affected_blocks {
                let block_record = stylesheet
                    .declaration_block(block)
                    .ok_or(MutationError::<'ast>::UnknownDeclarationBlock(block))?;
                let DeclarationBlockOwner::Rule(owner) = block_record.owner();
                let owner_record = stylesheet
                    .rule(owner)
                    .ok_or(MutationError::<'ast>::UnknownRule(owner))?;
                self.scratch.affected_rules.push(AffectedRule {
                    parent: owner_record.parent(),
                    owner,
                    block,
                    effective_key: block_record.effective_key(),
                    retire: self.declaration_ir.block_live_count(block) == 0
                        && !stylesheet.has_nested_rules(owner)?,
                });
            }
            self.scratch
                .affected_rules
                .sort_unstable_by_key(|affected| (affected.parent, affected.owner));

            let mut group_start = 0;
            while group_start < self.scratch.affected_rules.len() {
                let parent = self.scratch.affected_rules[group_start].parent;
                let mut group_end = group_start + 1;
                while group_end < self.scratch.affected_rules.len()
                    && self.scratch.affected_rules[group_end].parent == parent
                {
                    group_end += 1;
                }

                self.scratch.rule_contexts.clear();
                if let Some(parent) = parent {
                    for context in stylesheet.nested_rule_contexts(parent)? {
                        let affected = self.scratch.affected_rules[group_start..group_end]
                            .binary_search_by_key(&context.rule(), |affected| affected.owner)
                            .is_ok();
                        if affected {
                            self.scratch.rule_contexts.push(context);
                        }
                    }
                } else {
                    for context in stylesheet.root_rule_contexts() {
                        let affected = self.scratch.affected_rules[group_start..group_end]
                            .binary_search_by_key(&context.rule(), |affected| affected.owner)
                            .is_ok();
                        if affected {
                            self.scratch.rule_contexts.push(context);
                        }
                    }
                }
                if self.scratch.rule_contexts.len() != group_end - group_start {
                    return Err(MutationError::<'ast>::InvalidRuleTopology(
                        self.scratch.affected_rules[group_start].owner,
                    ));
                }

                self.scratch.mutation_deltas.clear();
                for context_index in 0..self.scratch.rule_contexts.len() {
                    let context = self.scratch.rule_contexts[context_index];
                    let affected_index = self.scratch.affected_rules[group_start..group_end]
                        .binary_search_by_key(&context.rule(), |affected| affected.owner)
                        .expect("the direct-rule pass selected only affected owners")
                        + group_start;
                    let affected = self.scratch.affected_rules[affected_index];
                    let delta = if affected.retire {
                        let retired = stylesheet.retire_rule(context)?;
                        self.remove_history_occurrence(affected.effective_key, affected.block);
                        stats.empty_rules_retired += 1;
                        retired.delta
                    } else {
                        stylesheet.rule_incident_edges(context)?
                    };
                    self.scratch.mutation_deltas.push(delta);
                }
                for delta_index in 0..self.scratch.mutation_deltas.len() {
                    self.enqueue_mutation_delta(
                        stylesheet,
                        self.scratch.mutation_deltas[delta_index],
                    );
                }
                group_start = group_end;
            }
            if !self.same_selector_candidates.is_empty() {
                break;
            }
        }
        Ok(stats)
    }

    fn run_s3(
        &mut self,
        stylesheet: &mut StyleSheet<'ast>,
        preserve_selector_compatibility: bool,
    ) -> Result<S3Stats, MutationError<'ast>> {
        let mut stats = S3Stats::default();
        while let Some(candidate) = self.partial_merge_candidates.pop() {
            let Some(endpoints) = validate_s3(stylesheet, candidate) else {
                continue;
            };
            self.declaration_ir.live_declarations(
                stylesheet,
                candidate.left,
                &mut self.scratch.left_declarations,
            )?;
            self.declaration_ir.live_declarations(
                stylesheet,
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
                            || !declarations_have_equal_effect(stylesheet, left, right)
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
                            || !declarations_have_equal_effect(stylesheet, left, right)
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
                stylesheet
                    .selector_value(endpoints.left_selector)
                    .expect("a validated selector value remains resolvable")
                    .selectors(),
                stylesheet
                    .selector_value(endpoints.right_selector)
                    .expect("a validated selector value remains resolvable")
                    .selectors(),
                preserve_selector_compatibility,
                self.allocator,
            ) else {
                continue;
            };
            let selector_value = stylesheet.intern_selector_value(
                selectors,
                endpoints.selector_kind,
                endpoints.vendor_prefix,
            )?;
            let Some(shared_key) = stylesheet.intern_selector_union_effective_key(
                endpoints.left_key,
                endpoints.right_key,
                selector_value,
            )?
            else {
                continue;
            };

            if self.scratch.left_residual.is_empty() {
                let (_, selector_delta) = stylesheet.replace_rule_selector_value_in_edge(
                    candidate.edge,
                    selector_value,
                    self.allocator,
                )?;
                let left_payload = stylesheet
                    .rule_mut(endpoints.left_rule)
                    .expect("the reused S3 endpoint remains live")
                    .payload_mut();
                match left_payload {
                    CssRule::Style(payload) => payload.span = endpoints.span,
                    CssRule::Nesting(payload) => payload.span = endpoints.span,
                    _ => unreachable!("the S3 selector owner was validated"),
                }

                for declaration in &self.scratch.common {
                    stylesheet.replace_declaration(
                        candidate.right,
                        declaration.right,
                        CssDeclaration::Property(Declaration::Tombstone),
                    )?;
                    self.declaration_ir
                        .mark_dead(candidate.right, declaration.right);
                }
                debug_assert_eq!(
                    stylesheet
                        .declaration_block(candidate.left)
                        .expect("the reused S3 block remains live")
                        .effective_key(),
                    shared_key
                );
                self.remove_history_occurrence(endpoints.left_key, candidate.left);
                self.insert_history_occurrence(shared_key, candidate.left);

                let retain_right = self.declaration_ir.block_live_count(candidate.right) != 0
                    || stylesheet.has_nested_rules(endpoints.right_rule)?;
                self.scratch.mutation_deltas.clear();
                self.scratch.mutation_deltas.push(selector_delta);
                self.scratch
                    .mutation_deltas
                    .push(stylesheet.rule_incident_edges(candidate.edge.right_context())?);
                if !retain_right {
                    let retired = stylesheet.retire_rule(candidate.edge.right_context())?;
                    self.scratch.mutation_deltas.push(retired.delta);
                    self.remove_history_occurrence(endpoints.right_key, candidate.right);
                }
                for delta_index in 0..self.scratch.mutation_deltas.len() {
                    self.enqueue_mutation_delta(
                        stylesheet,
                        self.scratch.mutation_deltas[delta_index],
                    );
                }
                self.declaration_override_candidates
                    .push(endpoints.left_key);
                self.declaration_override_candidates
                    .push(endpoints.right_key);
                self.declaration_override_candidates.push(shared_key);
                stats.reused_left_commits += 1;
                break;
            }

            let payload = match endpoints.selector_kind {
                rocketcss_ast::SelectorFrameKind::Style => CssRule::Style(StyleRule {
                    span: endpoints.span,
                    selector_value,
                    vendor_prefix: endpoints.vendor_prefix,
                }),
                rocketcss_ast::SelectorFrameKind::Nesting => CssRule::Nesting(NestingRule {
                    span: endpoints.span,
                    selector_value,
                }),
            };
            let insertion = match stylesheet.insert_rule_with_declaration_block_after(
                candidate.edge,
                candidate.left,
                payload,
                shared_key,
                self.scratch.common.len(),
            ) {
                Ok(result) => result,
                Err(
                    MutationError::<'ast>::LocalRuleCapacityExhausted(_)
                    | MutationError::<'ast>::PrimaryRuleCapacityExhausted
                    | MutationError::<'ast>::LocalDeclarationBlockCapacityExhausted(_)
                    | MutationError::<'ast>::DeclarationCapacityExhausted,
                ) => {
                    stats.rejected_capacity += 1;
                    continue;
                }
                Err(error) => return Err(error),
            };
            let insertion_delta = insertion.delta;
            let rule_result = insertion.rule;
            let right_rule = remap_rule_id(endpoints.right_rule, &rule_result.remaps);
            let shared_rule = rule_result.id;
            self.repair_rule_remaps(&rule_result);
            let block_result = insertion.declaration_block;
            self.repair_block_remaps(&block_result);
            let left_block = remap_block_id(candidate.left, &block_result.remaps);
            let right_block = remap_block_id(candidate.right, &block_result.remaps);
            let shared_block = block_result.id;

            let mut moved_declarations = std::vec::Vec::with_capacity(self.scratch.common.len());
            for declaration in &self.scratch.common {
                let important = stylesheet
                    .declaration(declaration.left)
                    .ok_or(MutationError::<'ast>::UnknownDeclaration(declaration.left))?
                    .is_important();
                let moved = stylesheet.replace_declaration(
                    left_block,
                    declaration.left,
                    CssDeclaration::Property(Declaration::Tombstone),
                )?;
                stylesheet.replace_declaration(
                    right_block,
                    declaration.right,
                    CssDeclaration::Property(Declaration::Tombstone),
                )?;
                self.declaration_ir.mark_dead(left_block, declaration.left);
                self.declaration_ir
                    .mark_dead(right_block, declaration.right);
                moved_declarations.push((moved, important));
            }
            stylesheet
                .insert_transformed_declarations_at_block_end(shared_block, moved_declarations)?;
            self.declaration_ir.freeze_block(stylesheet, shared_block)?;
            self.insert_history_occurrence(shared_key, shared_block);

            let retain_right = self.declaration_ir.block_live_count(right_block) != 0
                || stylesheet.has_nested_rules(right_rule)?;
            self.scratch.mutation_deltas.clear();
            self.scratch.mutation_deltas.push(insertion_delta);
            if !retain_right {
                let right_context = insertion_delta
                    .edges()
                    .find(|edge| edge.left() == shared_rule && edge.right() == right_rule)
                    .map(|edge| edge.right_context())
                    .ok_or(MutationError::<'ast>::InvalidRuleTopology(right_rule))?;
                let retired = stylesheet.retire_rule(right_context)?;
                self.scratch.mutation_deltas.push(retired.delta);
                self.remove_history_occurrence(endpoints.right_key, right_block);
            }
            for delta_index in 0..self.scratch.mutation_deltas.len() {
                self.enqueue_mutation_delta(stylesheet, self.scratch.mutation_deltas[delta_index]);
            }
            self.declaration_override_candidates
                .push(endpoints.left_key);
            self.declaration_override_candidates
                .push(endpoints.right_key);
            self.declaration_override_candidates.push(shared_key);
            stats.allocated_shared_commits += 1;
            break;
        }
        Ok(stats)
    }

    fn remove_history_occurrence(&mut self, key: EffectiveKeyId, block: DeclarationBlockId<'ast>) {
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
        block: DeclarationBlockId<'ast>,
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

    fn enqueue_mutation_delta(&mut self, stylesheet: &StyleSheet<'ast>, delta: RuleDelta<'ast>) {
        for edge in delta.edges() {
            if let Some(candidate) = self.edge_candidate_from_edge(stylesheet, edge) {
                self.enqueue_edge_candidate(candidate);
            }
        }
    }

    fn enqueue_edge_candidate(&mut self, edge: Candidate<'ast>) {
        self.direct_style_edges.push(edge);
        if edge.same_effective_key {
            self.same_selector_candidates.push(edge);
        } else if edge.may_share_declaration {
            self.partial_merge_candidates.push(edge);
        }
    }

    fn repair_block_remaps(&mut self, result: &RadixInsertResult<DeclarationBlockId<'ast>>) {
        if result.remaps.is_empty() {
            return;
        }
        self.declaration_ir.repair_block_remaps(&result.remaps);
        for block in &mut self.published_blocks {
            block.block = remap_block_id(block.block, &result.remaps);
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

    fn repair_rule_remaps(&mut self, result: &RadixInsertResult<RuleId<'ast>>) {
        if result.remaps.is_empty() {
            return;
        }
        for edge in &mut self.direct_style_edges {
            edge.remap_rules(&result.remaps);
        }
        self.same_selector_candidates.remap_rules(&result.remaps);
        self.partial_merge_candidates.remap_rules(&result.remaps);
    }

    fn edge_candidate_from_edge(
        &self,
        stylesheet: &StyleSheet<'ast>,
        edge: RuleEdge<'ast>,
    ) -> Option<Candidate<'ast>> {
        if !stylesheet.is_valid_direct_rule_edge(edge) {
            return None;
        }
        let left_rule_id = edge.left();
        let right_rule_id = edge.right();
        let left_rule = stylesheet.rule(left_rule_id)?;
        let right_rule = stylesheet.rule(right_rule_id)?;
        if !left_rule.is_live()
            || !right_rule.is_live()
            || !is_style_owner(left_rule.payload())
            || !is_style_owner(right_rule.payload())
        {
            return None;
        }
        let left = left_rule.declaration_block()?;
        let right = right_rule.declaration_block()?;
        let left_block = stylesheet.declaration_block(left)?;
        let right_block = stylesheet.declaration_block(right)?;
        Some(Candidate {
            edge,
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

impl<'ast> Candidate<'ast> {
    fn remap_blocks(&mut self, remaps: &[RadixIdRemap<DeclarationBlockId<'ast>>]) {
        self.left = remap_block_id(self.left, remaps);
        self.right = remap_block_id(self.right, remaps);
    }

    fn remap_rules(&mut self, remaps: &[RadixIdRemap<RuleId<'ast>>]) {
        self.edge = self.edge.remapped(remaps);
    }
}

fn is_style_owner(payload: &CssRule<'_>) -> bool {
    matches!(payload, CssRule::Style(_) | CssRule::Nesting(_))
}

fn remap_rule_id<'ast>(id: RuleId<'ast>, remaps: &[RadixIdRemap<RuleId<'ast>>]) -> RuleId<'ast> {
    remaps
        .iter()
        .find_map(|remap| (remap.old == id).then_some(remap.new))
        .unwrap_or(id)
}

fn remap_block_id<'ast>(
    id: DeclarationBlockId<'ast>,
    remaps: &[RadixIdRemap<DeclarationBlockId<'ast>>],
) -> DeclarationBlockId<'ast> {
    remaps
        .iter()
        .find_map(|remap| (remap.old == id).then_some(remap.new))
        .unwrap_or(id)
}

fn validate_s1<'ast>(
    stylesheet: &StyleSheet<'ast>,
    candidate: Candidate<'ast>,
) -> Option<EffectiveKeyId> {
    if !stylesheet.is_valid_direct_rule_edge(candidate.edge) {
        return None;
    }
    let left_block = stylesheet.declaration_block(candidate.left)?;
    let right_block = stylesheet.declaration_block(candidate.right)?;
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
    if left != candidate.edge.left() || right != candidate.edge.right() {
        return None;
    }
    let left_rule = stylesheet.rule(left)?;
    let right_rule = stylesheet.rule(right)?;
    if stylesheet.has_nested_rules(left).ok()?
        || !is_style_owner(left_rule.payload())
        || !is_style_owner(right_rule.payload())
    {
        return None;
    }
    Some(left_block.effective_key())
}

fn declarations_are_exactly_equal(
    stylesheet: &StyleSheet<'_>,
    left: rocketcss_ast::DeclarationId,
    right: rocketcss_ast::DeclarationId,
) -> bool {
    let Some(left) = stylesheet.declaration(left) else {
        return false;
    };
    let Some(right) = stylesheet.declaration(right) else {
        return false;
    };
    left.is_important() == right.is_important() && left.payload() == right.payload()
}

#[derive(Clone, Copy)]
struct RadixS3Endpoints<'ast> {
    left_rule: RuleId<'ast>,
    right_rule: RuleId<'ast>,
    left_key: EffectiveKeyId,
    right_key: EffectiveKeyId,
    left_selector: rocketcss_ast::SelectorValueId,
    right_selector: rocketcss_ast::SelectorValueId,
    selector_kind: rocketcss_ast::SelectorFrameKind,
    vendor_prefix: rocketcss_ast::VendorPrefix,
    span: Span,
}

fn validate_s3<'ast>(
    stylesheet: &StyleSheet<'ast>,
    candidate: Candidate<'ast>,
) -> Option<RadixS3Endpoints<'ast>> {
    if !stylesheet.is_valid_direct_rule_edge(candidate.edge) {
        return None;
    }
    let left_block = stylesheet.declaration_block(candidate.left)?;
    let right_block = stylesheet.declaration_block(candidate.right)?;
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
    if left_rule_id != candidate.edge.left() || right_rule_id != candidate.edge.right() {
        return None;
    }
    let left_rule = stylesheet.rule(left_rule_id)?;
    let right_rule = stylesheet.rule(right_rule_id)?;
    if stylesheet.has_nested_rules(left_rule_id).ok()? {
        return None;
    }
    let (left_selector, left_span) = match left_rule.payload() {
        CssRule::Style(payload) => (payload.selector_value, payload.span),
        CssRule::Nesting(payload) => (payload.selector_value, payload.span),
        _ => return None,
    };
    let (right_selector, right_span) = match right_rule.payload() {
        CssRule::Style(payload) => (payload.selector_value, payload.span),
        CssRule::Nesting(payload) => (payload.selector_value, payload.span),
        _ => return None,
    };
    let left_value = stylesheet.selector_value(left_selector)?;
    let right_value = stylesheet.selector_value(right_selector)?;
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
    stylesheet: &StyleSheet<'_>,
    left: rocketcss_ast::DeclarationId,
    right: rocketcss_ast::DeclarationId,
) -> bool {
    let Some(left) = stylesheet.declaration(left) else {
        return false;
    };
    let Some(right) = stylesheet.declaration(right) else {
        return false;
    };
    if left.is_important() != right.is_important() {
        return false;
    }
    match (left.payload(), right.payload()) {
        (CssDeclaration::Property(left), CssDeclaration::Property(right)) => {
            left.eq_ignoring_tombstones(right)
        }
        _ => false,
    }
}

fn has_opaque_domain_conflict(
    declaration_ir: &DeclarationIrStore<'_, '_>,
    domain: MovementDomain,
    declarations: &[rocketcss_ast::DeclarationId],
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
    left_residual: &[rocketcss_ast::DeclarationId],
    right_residual: &[rocketcss_ast::DeclarationId],
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

    trait ParseTestStyleSheet<'ast> {
        fn parse_test_stylesheet(
            &mut self,
            source: &'ast str,
            options: ParserOptions<'ast>,
        ) -> Result<rocketcss_ast::StyleSheet<'ast>, rocketcss_parser::Error<'ast>>;
    }

    impl<'ast> ParseTestStyleSheet<'ast> for Compiler<'ast> {
        fn parse_test_stylesheet(
            &mut self,
            source: &'ast str,
            options: ParserOptions<'ast>,
        ) -> Result<rocketcss_ast::StyleSheet<'ast>, rocketcss_parser::Error<'ast>> {
            rocketcss_common::GhostToken::scope(|mut token| self.parse(source, &mut token, options))
        }
    }

    #[test]
    fn finalizes_histories_ir_and_direct_edges_from_published_metadata() {
        let allocator = Allocator::new();
        let stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{color:red}a{color:blue}@media print{a{color:red}a{color:blue}}b{color:red}b{color:blue}a{color:green}",
                ParserOptions::default(),
            )
            .unwrap();
        let state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

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
                stylesheet
                    .declaration_block(block)
                    .is_some_and(|block| block.effective_key() == key)
            }));
        }
        assert!(state.direct_style_edges.iter().all(|candidate| {
            stylesheet.rule(candidate.edge.left()).unwrap().parent()
                == stylesheet.rule(candidate.edge.right()).unwrap().parent()
        }));
    }

    #[test]
    fn nested_declaration_segments_join_history_without_becoming_style_edges() {
        let allocator = Allocator::new();
        let stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{width:1px;& b{height:2px}width:3px}",
                ParserOptions::default(),
            )
            .unwrap();
        let state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

        assert_eq!(state.published_block_count, 3);
        assert_eq!(state.declaration_ir.occurrence_count(), 3);
        assert!(state.direct_style_edges.is_empty());
        assert!(state.histories.values().any(|history| history.len() == 2));
    }

    #[test]
    fn s1_commits_directly_and_enqueues_the_newly_exposed_ast_edge() {
        let allocator = Allocator::new();
        let mut stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{color:red}a{width:1px}a{height:2px}b{color:blue}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

        assert_eq!(state.run_s1(&mut stylesheet).unwrap(), 2);
        let live = stylesheet
            .root_rules()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>();
        assert_eq!(live.len(), 2);
        let retained = stylesheet
            .rule(live[0])
            .unwrap()
            .declaration_block()
            .unwrap();
        assert_eq!(
            stylesheet.declarations_in_block(retained).unwrap().count(),
            3
        );
        assert!(
            state
                .histories
                .values()
                .any(|history| history.as_slice() == [retained])
        );
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    #[test]
    fn topology_revision_discards_a_candidate_after_insertion() {
        let allocator = Allocator::new();
        let mut stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet("a{color:red}a{color:blue}", ParserOptions::default())
            .unwrap();
        let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();
        let candidate = state.same_selector_candidates.pop().unwrap();

        stylesheet
            .insert_rule_after(
                candidate.edge.left_context(),
                CssRule::NestedDeclarations(rocketcss_ast::NestedDeclarationsRule {
                    span: Span::new(0, 0),
                }),
            )
            .unwrap();

        assert!(!stylesheet.is_valid_direct_rule_edge(candidate.edge));
        assert_eq!(validate_s1(&stylesheet, candidate), None);
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_batches_consecutive_empty_rules_in_one_parent_pass() {
        let allocator = Allocator::new();
        let mut stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{color:red}a{width:1px}a{color:red;width:1px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

        let stats = state.run_s2_exact(&mut stylesheet).unwrap();

        assert_eq!(stats.declarations_removed, 2);
        assert_eq!(stats.empty_rules_retired, 2);
        assert_eq!(stylesheet.root_rules().count(), 1);
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_tombstones_only_exact_old_declarations_in_the_same_ast_history() {
        let allocator = Allocator::new();
        let mut stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{color:red;width:1px}b{color:red}a{color:red;width:2px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

        let stats = state.run_s2_exact(&mut stylesheet).unwrap();
        assert_eq!(
            stats,
            S2Stats {
                declarations_removed: 1,
                empty_rules_retired: 0,
            }
        );
        let first_block = stylesheet
            .declaration_blocks_in_source_order()
            .next()
            .unwrap()
            .0;
        let declarations = stylesheet
            .declarations_in_block(first_block)
            .unwrap()
            .collect::<std::vec::Vec<_>>();
        assert!(matches!(
            declarations[0].payload(),
            CssDeclaration::Property(Declaration::Tombstone)
        ));
        assert!(!matches!(
            declarations[1].payload(),
            CssDeclaration::Property(Declaration::Tombstone)
        ));
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_retires_an_empty_leaf_rule_without_rebuilding_topology() {
        let allocator = Allocator::new();
        let mut stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{color:red}b{width:1px}a{color:red}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

        let stats = state.run_s2_exact(&mut stylesheet).unwrap();
        assert_eq!(stats.declarations_removed, 1);
        assert_eq!(stats.empty_rules_retired, 1);
        assert_eq!(stylesheet.root_rules().count(), 2);
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_exposes_and_queues_only_the_new_local_s1_edge() {
        let allocator = Allocator::new();
        let mut stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{color:red}b{width:1px}a{height:2px}b{width:1px}a{padding:0}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

        assert_eq!(state.run_s1(&mut stylesheet).unwrap(), 0);
        let s2 = state.run_s2_exact(&mut stylesheet).unwrap();
        assert_eq!(s2.empty_rules_retired, 1);
        assert_eq!(state.run_s1(&mut stylesheet).unwrap(), 1);

        assert_eq!(stylesheet.root_rules().count(), 3);
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    #[test]
    fn s2_keeps_importance_context_mismatches_and_unparsed_values() {
        let allocator = Allocator::new();
        let mut stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{color:red!important;unknown:x}a{color:red;unknown:x}",
                ParserOptions {
                    error_recovery: true,
                    ..ParserOptions::default()
                },
            )
            .unwrap();
        let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

        assert_eq!(
            state.run_s2_exact(&mut stylesheet).unwrap(),
            S2Stats::default()
        );
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    #[test]
    fn s3_reuses_an_exhausted_left_block_with_order_independent_ir() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut stylesheet = Compiler::new(&allocator)
                .parse_test_stylesheet(
                    "a{color:red;width:1px}b{width:1px;color:red;height:2px}",
                    options,
                )
                .unwrap();
            let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

            assert_eq!(
                state.run_s3(&mut stylesheet, true).unwrap(),
                S3Stats {
                    reused_left_commits: 1,
                    allocated_shared_commits: 0,
                    rejected_no_common: 0,
                    rejected_unsafe_movement: 0,
                    rejected_capacity: 0,
                }
            );
            let actual = stylesheet
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
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s3_inserts_a_shared_rule_and_block_at_their_final_radix_ids() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut stylesheet = Compiler::new(&allocator)
                .parse_test_stylesheet("a{color:red;width:1px}b{color:red;height:2px}", options)
                .unwrap();
            let authored_declarations = stylesheet.declarations_in_source_order().count();
            let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

            assert_eq!(
                state.run_s3(&mut stylesheet, true).unwrap(),
                S3Stats {
                    reused_left_commits: 0,
                    allocated_shared_commits: 1,
                    rejected_no_common: 0,
                    rejected_unsafe_movement: 0,
                    rejected_capacity: 0,
                }
            );
            assert_eq!(stylesheet.root_rules().count(), 3);
            let shared_rule = stylesheet.root_rules().nth(1).unwrap().0;
            let shared_block = stylesheet
                .rule(shared_rule)
                .unwrap()
                .declaration_block()
                .unwrap();
            let synthesized = stylesheet
                .declaration_ids_in_block(shared_block)
                .unwrap()
                .next()
                .unwrap();
            assert!(!synthesized.is_primary());
            assert!(state.declaration_ir.occurrence(synthesized).is_some());
            assert_eq!(
                stylesheet.declarations_in_source_order().count(),
                authored_declarations + 1
            );
            assert!(
                state
                    .histories
                    .values()
                    .all(|history| { history.windows(2).all(|pair| pair[0] < pair[1]) })
            );
            let actual = stylesheet
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
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s3_insertion_retires_an_exhausted_right_endpoint_locally() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut stylesheet = Compiler::new(&allocator)
                .parse_test_stylesheet("a{width:1px;color:red}b{color:red}", options)
                .unwrap();
            let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

            assert_eq!(
                state.run_s3(&mut stylesheet, true).unwrap(),
                S3Stats {
                    reused_left_commits: 0,
                    allocated_shared_commits: 1,
                    rejected_no_common: 0,
                    rejected_unsafe_movement: 0,
                    rejected_capacity: 0,
                }
            );
            assert_eq!(stylesheet.root_rules().count(), 2);
            let actual = stylesheet
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
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn scheduler_returns_new_local_work_to_s1_and_s2_before_the_next_s3_edge() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut stylesheet = Compiler::new(&allocator)
                .parse_test_stylesheet("a{color:red}b{color:red}a,b{width:1px}", options)
                .unwrap();
            let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

            let stats = state.run(&mut stylesheet, true).unwrap();
            assert_eq!(stats.s3.reused_left_commits, 1);
            assert_eq!(stats.s1_commits, 1);
            let actual = stylesheet
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
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn scheduler_inserts_s3_history_in_semantic_order_for_s2() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut stylesheet = Compiler::new(&allocator)
                .parse_test_stylesheet(
                    "a,b{width:1px}c{display:block}a{width:1px}b{width:1px}",
                    options,
                )
                .unwrap();
            let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

            let stats = state.run(&mut stylesheet, true).unwrap();
            assert_eq!(stats.s3.reused_left_commits, 1);
            assert_eq!(stats.s2.declarations_removed, 1);
            let actual = stylesheet
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
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn scheduler_uses_the_s3_heap_for_overlapping_partial_edges() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut stylesheet = Compiler::new(&allocator)
                .parse_test_stylesheet("a{color:red}b{color:red;width:1px}c{width:1px}", options)
                .unwrap();
            let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

            let stats = state.run(&mut stylesheet, true).unwrap();
            assert_eq!(stats.s3.reused_left_commits, 2);
            let actual = stylesheet
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
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s1_after_s3_keeps_the_merged_declarations_in_one_radix_range() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut stylesheet = Compiler::new(&allocator)
                .parse_test_stylesheet("a{width:1px;color:red}b{color:red}a,b{padding:0}", options)
                .unwrap();
            let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

            let stats = state.run(&mut stylesheet, true).unwrap();
            assert_eq!(stats.s3.allocated_shared_commits, 1);
            assert_eq!(stats.s1_commits, 1);
            assert!(
                stylesheet
                    .declaration_blocks_in_source_order()
                    .any(|(_, block)| block.is_live() && block.declarations().len() == 2)
            );
            let actual = stylesheet
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
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s1_after_s3_keeps_a_large_merge_in_one_radix_range() {
        rocketcss_common::GhostToken::scope(|mut token| {
            let allocator = Allocator::new();
            let options = ParserOptions::default();
            let mut stylesheet = Compiler::new(&allocator)
                .parse_test_stylesheet(
                    "a{width:1px;color:red}b{color:red}a,b{padding:0;height:2px;opacity:.5}",
                    options,
                )
                .unwrap();
            let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

            let stats = state.run(&mut stylesheet, true).unwrap();
            assert_eq!(stats.s3.allocated_shared_commits, 1);
            assert_eq!(stats.s1_commits, 1);
            let live_range_lengths = stylesheet
                .declaration_blocks_in_source_order()
                .filter_map(|(_, block)| block.is_live().then_some(block.declarations().len()))
                .collect::<std::vec::Vec<_>>();
            assert!(
                live_range_lengths.contains(&5),
                "unexpected live declaration ranges: {live_range_lengths:?}"
            );
            let actual = stylesheet
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
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }

    #[test]
    fn s3_rejects_reordered_overlapping_effect_domains() {
        let allocator = Allocator::new();
        let mut stylesheet = Compiler::new(&allocator)
            .parse_test_stylesheet(
                "a{width:1px;min-width:2px}b{min-width:2px;width:1px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_stylesheet(&stylesheet).unwrap();

        assert_eq!(state.direct_style_edges.len(), 1);
        assert!(!state.direct_style_edges[0].same_effective_key);
        assert!(state.direct_style_edges[0].may_share_declaration);

        assert_eq!(
            state.run_s3(&mut stylesheet, true).unwrap(),
            S3Stats {
                reused_left_commits: 0,
                allocated_shared_commits: 0,
                rejected_no_common: 0,
                rejected_unsafe_movement: 1,
                rejected_capacity: 0,
            }
        );
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }
}
