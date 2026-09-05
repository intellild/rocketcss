//! Incremental cross-rule scheduler over the compiler-owned persistent AST.
//!
//! Declaration summaries are published at the end of each local block pass.
//! Finalization consumes that publication tape plus the final context remap;
//! ownership, effective context, source identity, and adjacency stay
//! authoritative in the AST rather than being reconstructed into Nano records.

use rocketcss_ast::{
    AstContext, ConcreteDeclarationBlockId as DeclarationBlockId,
    ConcreteMutationError as MutationError, ConcreteRuleId as RuleId, CssRulePayload, Declaration,
    DeclarationBlockOwner, DeclarationPayload, EffectiveKeyId, EqIgnoringTombstones,
    NestingRulePayload, Span, StyleRulePayload,
};
use rocketcss_common::{
    Allocator,
    prelude::{HashMap, HashSet, Vec},
};
use std::{cmp::Reverse, collections::VecDeque};

use crate::rules::layout::{ALL_BOX_SIDES, BoxFamily, materialize_box_longhands};

use super::declaration_ir::{
    CompactPropertyKey, DeclarationIrStore, EffectExpansion, MovementDomain,
};
use super::partial_selector::materialize_selector_union;

pub(crate) struct CrossRuleBuilder<'scratch, 'ast> {
    state: CrossRuleState<'scratch, 'ast>,
}

impl<'scratch, 'ast> CrossRuleBuilder<'scratch, 'ast> {
    pub(super) fn new(compilation: &AstContext<'ast>, allocator: &'scratch Allocator) -> Self {
        Self {
            state: CrossRuleState::new_in(compilation, allocator),
        }
    }

    pub(super) fn publish_block(
        &mut self,
        compilation: &AstContext<'ast>,
        block: DeclarationBlockId<'ast>,
    ) -> Result<(), MutationError<'ast>> {
        self.state.publish_block(compilation, block)
    }

    pub(super) fn finalize(&mut self, key_remaps: &[EffectiveKeyId<'ast>]) {
        self.state.finalize_published_blocks(key_remaps);
    }
}

pub(super) fn stabilize_with_builder<'scratch, 'ast>(
    mut builder: CrossRuleBuilder<'scratch, 'ast>,
    compilation: &mut AstContext<'ast>,
    preserve_selector_compatibility: bool,
) -> Result<std::vec::Vec<DeclarationBlockId<'ast>>, MutationError<'ast>> {
    builder
        .state
        .run(compilation, preserve_selector_compatibility)?;
    builder.state.commit_s5(compilation)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Candidate<'ast> {
    left: DeclarationBlockId<'ast>,
    right: DeclarationBlockId<'ast>,
    left_revision: u32,
    right_revision: u32,
    same_effective_key: bool,
    may_share_declaration: bool,
}

#[derive(Debug)]
struct SameSelectorCandidateList<'scratch, 'ast> {
    pending: VecDeque<Candidate<'ast>>,
    queued: HashSet<'scratch, Candidate<'ast>>,
}

impl<'scratch, 'ast> SameSelectorCandidateList<'scratch, 'ast> {
    fn new_in(allocator: &'scratch Allocator) -> Self {
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
}

#[derive(Debug)]
struct PartialMergeCandidateList<'scratch, 'ast> {
    pending: Vec<'scratch, Reverse<Candidate<'ast>>>,
    queued: HashSet<'scratch, Candidate<'ast>>,
}

impl<'scratch, 'ast> PartialMergeCandidateList<'scratch, 'ast> {
    fn new_in(allocator: &'scratch Allocator) -> Self {
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
struct DeclarationOverrideCandidateList<'scratch, 'ast> {
    pending: VecDeque<EffectiveKeyId<'ast>>,
    queued: HashSet<'scratch, EffectiveKeyId<'ast>>,
}

#[derive(Debug)]
struct S4PlanItemList<'scratch, 'ast> {
    pending: VecDeque<rocketcss_ast::DeclarationId<'ast>>,
    queued: HashSet<'scratch, rocketcss_ast::DeclarationId<'ast>>,
}

impl<'scratch, 'ast> S4PlanItemList<'scratch, 'ast> {
    fn new_in(allocator: &'scratch Allocator) -> Self {
        Self {
            pending: VecDeque::new(),
            queued: HashSet::new_in(allocator),
        }
    }

    fn push(&mut self, declaration: rocketcss_ast::DeclarationId<'ast>) {
        if self.queued.insert(declaration) {
            self.pending.push_back(declaration);
        }
    }

    fn pop(&mut self) -> Option<rocketcss_ast::DeclarationId<'ast>> {
        let declaration = self.pending.pop_front()?;
        self.queued.remove(&declaration);
        Some(declaration)
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

#[derive(Clone, Copy, Debug)]
enum AstDeclarationPlanKind {
    MaterializeBoxLonghands { family: BoxFamily, live_effects: u8 },
}

#[derive(Clone, Copy, Debug)]
struct AstDeclarationPlan<'ast> {
    origin: rocketcss_ast::DeclarationId<'ast>,
    owner: DeclarationBlockId<'ast>,
    block_revision: u32,
    effect_revision: u32,
    important: bool,
    kind: AstDeclarationPlanKind,
}

impl<'scratch, 'ast> DeclarationOverrideCandidateList<'scratch, 'ast> {
    fn new_in(allocator: &'scratch Allocator) -> Self {
        Self {
            pending: VecDeque::new(),
            queued: HashSet::new_in(allocator),
        }
    }

    fn push(&mut self, key: EffectiveKeyId<'ast>) {
        if self.queued.insert(key) {
            self.pending.push_back(key);
        }
    }

    fn pop(&mut self) -> Option<EffectiveKeyId<'ast>> {
        let key = self.pending.pop_front()?;
        self.queued.remove(&key);
        Some(key)
    }

    fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

#[derive(Debug)]
struct MinifyScratch<'scratch, 'ast> {
    history: Vec<'scratch, DeclarationBlockId<'ast>>,
    left_declarations: Vec<'scratch, rocketcss_ast::DeclarationId<'ast>>,
    right_declarations: Vec<'scratch, rocketcss_ast::DeclarationId<'ast>>,
    left_residual: Vec<'scratch, rocketcss_ast::DeclarationId<'ast>>,
    right_residual: Vec<'scratch, rocketcss_ast::DeclarationId<'ast>>,
    common: Vec<'scratch, CommonDeclaration<'ast>>,
    block_declarations: Vec<'scratch, rocketcss_ast::DeclarationId<'ast>>,
    matched_right: HashSet<'scratch, rocketcss_ast::DeclarationId<'ast>>,
    matched_left: HashSet<'scratch, rocketcss_ast::DeclarationId<'ast>>,
    affected_blocks: HashSet<'scratch, DeclarationBlockId<'ast>>,
    previous_by_property: HashMap<
        'scratch,
        CompactPropertyKey,
        (DeclarationBlockId<'ast>, rocketcss_ast::DeclarationId<'ast>),
    >,
    previous_box_effects:
        [Option<(DeclarationBlockId<'ast>, rocketcss_ast::DeclarationId<'ast>)>; 16],
}

impl<'scratch, 'ast> MinifyScratch<'scratch, 'ast> {
    fn new_in(allocator: &'scratch Allocator) -> Self {
        Self {
            history: allocator.vec(),
            left_declarations: allocator.vec(),
            right_declarations: allocator.vec(),
            left_residual: allocator.vec(),
            right_residual: allocator.vec(),
            common: allocator.vec(),
            block_declarations: allocator.vec(),
            matched_right: HashSet::new_in(allocator),
            matched_left: HashSet::new_in(allocator),
            affected_blocks: HashSet::new_in(allocator),
            previous_by_property: HashMap::new_in(allocator),
            previous_box_effects: [None; 16],
        }
    }
}

#[inline]
fn box_effect_slot(family: BoxFamily, important: bool, side: usize) -> usize {
    usize::from(important) * BoxFamily::COUNT * 4 + family.index() * 4 + side
}

fn clear_box_effect_history<'ast>(
    history: &mut [Option<(DeclarationBlockId<'ast>, rocketcss_ast::DeclarationId<'ast>)>; 16],
    family: Option<BoxFamily>,
) {
    if let Some(family) = family {
        for important in [false, true] {
            for side in 0..4 {
                history[box_effect_slot(family, important, side)] = None;
            }
        }
    } else {
        history.fill(None);
    }
}

#[derive(Clone, Copy, Debug)]
struct PublishedBlock<'ast> {
    block: DeclarationBlockId<'ast>,
    effective_key: EffectiveKeyId<'ast>,
    revision: u32,
    previous_style_block: Option<(DeclarationBlockId<'ast>, EffectiveKeyId<'ast>, u32)>,
}

struct CrossRuleState<'scratch, 'ast> {
    allocator: &'scratch Allocator,
    declaration_ir: DeclarationIrStore<'scratch, 'ast>,
    // The authored common case is one occurrence per key. Keep that one ID
    // inline in the arena vector and grow it only when S2 needs a history.
    histories: HashMap<'scratch, EffectiveKeyId<'ast>, Vec<'scratch, DeclarationBlockId<'ast>>>,
    published_blocks: Vec<'scratch, PublishedBlock<'ast>>,
    direct_style_edges: Vec<'scratch, Candidate<'ast>>,
    same_selector_candidates: SameSelectorCandidateList<'scratch, 'ast>,
    declaration_override_candidates: DeclarationOverrideCandidateList<'scratch, 'ast>,
    partial_merge_candidates: PartialMergeCandidateList<'scratch, 'ast>,
    dirty_s4_plan_items: S4PlanItemList<'scratch, 'ast>,
    declaration_plans: Vec<'scratch, AstDeclarationPlan<'ast>>,
    representation_dirty_blocks: Vec<'scratch, DeclarationBlockId<'ast>>,
    representation_dirty_set: HashSet<'scratch, DeclarationBlockId<'ast>>,
    scratch: MinifyScratch<'scratch, 'ast>,
    published_block_count: usize,
}

impl<'scratch, 'ast> CrossRuleState<'scratch, 'ast> {
    fn new_in(compilation: &AstContext<'ast>, allocator: &'scratch Allocator) -> Self {
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
            dirty_s4_plan_items: S4PlanItemList::new_in(allocator),
            declaration_plans: allocator.vec(),
            representation_dirty_blocks: allocator.vec(),
            representation_dirty_set: HashSet::new_in(allocator),
            scratch: MinifyScratch::new_in(allocator),
            published_block_count: 0,
        }
    }

    #[cfg(test)]
    fn publish_all_blocks(
        &mut self,
        compilation: &AstContext<'ast>,
    ) -> Result<(), MutationError<'ast>> {
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
        compilation: &AstContext<'ast>,
        block: DeclarationBlockId<'ast>,
    ) -> Result<(), MutationError<'ast>> {
        self.declaration_ir.freeze_block(compilation, block)?;
        let block_record = compilation
            .declaration_block(block)
            .ok_or(MutationError::<'ast>::UnknownDeclarationBlock(block))?;
        let DeclarationBlockOwner::Rule(owner) = block_record.owner();
        let owner_record = compilation
            .rule(owner)
            .ok_or(MutationError::<'ast>::InvalidRuleTopology(owner))?;
        let previous_style_block = if is_style_owner(owner_record.payload())
            && let Some(previous) = owner_record.previous_sibling()
        {
            let previous_record = compilation
                .rule(previous)
                .ok_or(MutationError::<'ast>::InvalidRuleTopology(previous))?;
            if is_style_owner(previous_record.payload()) {
                let previous_block = previous_record
                    .declaration_block()
                    .ok_or(MutationError::<'ast>::InvalidRuleTopology(previous))?;
                let previous_block_record = compilation.declaration_block(previous_block).ok_or(
                    MutationError::<'ast>::UnknownDeclarationBlock(previous_block),
                )?;
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

    fn finalize_published_blocks(&mut self, key_remaps: &[EffectiveKeyId<'ast>]) {
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
            if self.append_history_occurrence(published.effective_key, published.block) {
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
        compilation: &AstContext<'ast>,
    ) -> Result<CrossRuleState<'minify, 'ast>, MutationError<'ast>>
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
struct CommonDeclaration<'ast> {
    left: rocketcss_ast::DeclarationId<'ast>,
    right: rocketcss_ast::DeclarationId<'ast>,
    right_order: usize,
}

impl<'scratch, 'ast> CrossRuleState<'scratch, 'ast> {
    fn run(
        &mut self,
        compilation: &mut AstContext<'ast>,
        preserve_selector_compatibility: bool,
    ) -> Result<SchedulerStats, MutationError<'ast>> {
        let mut stats = SchedulerStats::default();
        loop {
            stats.s1_commits += self.run_s1(compilation)?;
            let s2 = self.run_s2(compilation)?;
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
                self.run_s4(compilation)?;
                self.assert_semantic_fixed_point();
                return Ok(stats);
            }
        }
    }

    #[inline]
    fn is_semantic_fixed_point(&self) -> bool {
        self.same_selector_candidates.is_empty()
            && self.declaration_override_candidates.is_empty()
            && self.partial_merge_candidates.is_empty()
            && self.dirty_s4_plan_items.is_empty()
    }

    #[inline]
    fn assert_semantic_fixed_point(&self) {
        assert!(
            self.is_semantic_fixed_point(),
            "S5 requires every S1-S4 work queue to be drained"
        );
    }

    fn run_s4(&mut self, compilation: &AstContext<'ast>) -> Result<(), MutationError<'ast>> {
        while let Some(origin) = self.dirty_s4_plan_items.pop() {
            let Some(occurrence) = self.declaration_ir.occurrence(origin).copied() else {
                continue;
            };
            let EffectExpansion::BoxShorthand(family) = occurrence.expansion else {
                continue;
            };
            let live_effects = occurrence.live_effects();
            if live_effects == 0 || live_effects == ALL_BOX_SIDES {
                continue;
            }
            let block = compilation.declaration_block(occurrence.owner).ok_or(
                MutationError::<'ast>::UnknownDeclarationBlock(occurrence.owner),
            )?;
            let record = compilation
                .declaration(origin)
                .ok_or(MutationError::<'ast>::UnknownDeclaration(origin))?;
            self.declaration_plans.push(AstDeclarationPlan {
                origin,
                owner: occurrence.owner,
                block_revision: block.revision(),
                effect_revision: occurrence.effect_revision(),
                important: record.is_important(),
                kind: AstDeclarationPlanKind::MaterializeBoxLonghands {
                    family,
                    live_effects,
                },
            });
            self.mark_representation_dirty(occurrence.owner);
        }
        Ok(())
    }

    /// Terminal S5 boundary. All representation choices have already been
    /// made by S4; this method only validates, preflights, and commits them.
    fn commit_s5(
        self,
        compilation: &mut AstContext<'ast>,
    ) -> Result<std::vec::Vec<DeclarationBlockId<'ast>>, MutationError<'ast>> {
        self.assert_semantic_fixed_point();

        let mut additional = 0_usize;
        let mut origins: HashSet<'scratch, rocketcss_ast::DeclarationId<'ast>> =
            HashSet::new_in(self.allocator);
        let mut additional_by_block: HashMap<'scratch, DeclarationBlockId<'ast>, usize> =
            HashMap::new_in(self.allocator);
        for plan in &self.declaration_plans {
            if !origins.insert(plan.origin) {
                return Err(MutationError::<'ast>::UnknownDeclaration(plan.origin));
            }
            let block = compilation
                .declaration_block(plan.owner)
                .ok_or(MutationError::<'ast>::UnknownDeclarationBlock(plan.owner))?;
            if !block.is_live() || block.revision() != plan.block_revision {
                return Err(MutationError::<'ast>::UnknownDeclarationBlock(plan.owner));
            }
            let occurrence = self
                .declaration_ir
                .occurrence(plan.origin)
                .copied()
                .ok_or(MutationError::<'ast>::UnknownDeclaration(plan.origin))?;
            if occurrence.owner != plan.owner
                || occurrence.effect_revision() != plan.effect_revision
            {
                return Err(MutationError::<'ast>::UnknownDeclaration(plan.origin));
            }
            let AstDeclarationPlanKind::MaterializeBoxLonghands {
                family,
                live_effects,
            } = plan.kind;
            if occurrence.expansion != EffectExpansion::BoxShorthand(family)
                || occurrence.live_effects() != live_effects
                || live_effects == 0
                || live_effects == ALL_BOX_SIDES
                || live_effects & !ALL_BOX_SIDES != 0
            {
                return Err(MutationError::<'ast>::UnknownDeclaration(plan.origin));
            }
            let record = compilation
                .declaration(plan.origin)
                .ok_or(MutationError::<'ast>::UnknownDeclaration(plan.origin))?;
            if record.is_important() != plan.important {
                return Err(MutationError::<'ast>::UnknownDeclaration(plan.origin));
            }
            let matching_payload = matches!(
                (family, record.payload()),
                (
                    BoxFamily::Margin,
                    DeclarationPayload::Property(Declaration::Margin(_))
                ) | (
                    BoxFamily::Padding,
                    DeclarationPayload::Property(Declaration::Padding(_))
                )
            );
            if !matching_payload {
                return Err(MutationError::<'ast>::UnknownDeclaration(plan.origin));
            }
            let plan_additional = (live_effects.count_ones() as usize)
                .checked_sub(1)
                .ok_or(MutationError::<'ast>::DeclarationCapacityExhausted)?;
            additional = additional
                .checked_add(plan_additional)
                .ok_or(MutationError::<'ast>::DeclarationCapacityExhausted)?;
            let block_additional = additional_by_block
                .get(&plan.owner)
                .copied()
                .unwrap_or(0_usize)
                .checked_add(plan_additional)
                .ok_or(MutationError::<'ast>::DeclarationCapacityExhausted)?;
            compilation.validate_declaration_rewrite(plan.owner, plan.origin, block_additional)?;
            additional_by_block.insert(plan.owner, block_additional);
        }
        if !compilation.can_insert_transformed_declarations(additional) {
            return Err(MutationError::<'ast>::DeclarationCapacityExhausted);
        }

        for plan in &self.declaration_plans {
            let AstDeclarationPlanKind::MaterializeBoxLonghands {
                family,
                live_effects,
            } = plan.kind;
            let additional = live_effects.count_ones() as usize - 1;
            let replacements = {
                let record = compilation
                    .declaration(plan.origin)
                    .expect("S4 validated the declaration before terminal commit");
                let DeclarationPayload::Property(original) = record.payload() else {
                    unreachable!("an S4 box plan owns a property declaration")
                };
                materialize_box_longhands(original, family, live_effects, compilation)
                    .expect("S4 validated a typed box shorthand before terminal commit")
            };
            let result = compilation.rewrite_declaration_with_sequence(
                plan.owner,
                plan.origin,
                additional,
                DeclarationPayload::Property(Declaration::Tombstone),
                move |_original, important| {
                    replacements
                        .into_iter()
                        .map(|declaration| (DeclarationPayload::Property(declaration), important))
                        .collect()
                },
            );
            if result.is_err() {
                unreachable!("a fully preflighted S5 declaration plan cannot fail during commit");
            }
        }
        Ok(self.representation_dirty_blocks.iter().copied().collect())
    }

    fn mark_representation_dirty(&mut self, block: DeclarationBlockId<'ast>) {
        if self.representation_dirty_set.insert(block) {
            self.representation_dirty_blocks.push(block);
        }
    }

    fn run_s1(&mut self, compilation: &mut AstContext<'ast>) -> Result<usize, MutationError<'ast>> {
        let mut commits = 0;
        while let Some(candidate) = self.same_selector_candidates.pop() {
            let Some((left_rule, right_rule, key)) = validate_s1(compilation, candidate) else {
                continue;
            };
            let merged =
                compilation.merge_adjacent_rule_declaration_blocks(left_rule, right_rule)?;
            debug_assert_eq!(merged.effective_key, key);
            self.declaration_ir.compose(
                compilation,
                merged.retired_block,
                merged.retained_block,
            )?;
            if self
                .declaration_ir
                .block_has_box_effects(merged.retained_block)
            {
                self.mark_representation_dirty(merged.retained_block);
            }
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

    fn run_s2(
        &mut self,
        compilation: &mut AstContext<'ast>,
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
            self.scratch.previous_box_effects.fill(None);
            for &block in &self.scratch.history {
                let block_record = compilation
                    .declaration_block(block)
                    .ok_or(MutationError::<'ast>::UnknownDeclarationBlock(block))?;
                let DeclarationBlockOwner::Rule(owner) = block_record.owner();
                if compilation
                    .rule(owner)
                    .is_none_or(|rule| !is_style_owner(rule.payload()))
                {
                    self.scratch.previous_box_effects.fill(None);
                }
                self.scratch.block_declarations.clear();
                self.scratch
                    .block_declarations
                    .extend(compilation.declaration_ids_in_block(block)?);
                for index in 0..self.scratch.block_declarations.len() {
                    let declaration = self.scratch.block_declarations[index];
                    let Some(occurrence) = self.declaration_ir.occurrence(declaration).copied()
                    else {
                        continue;
                    };
                    if !occurrence.is_live() {
                        continue;
                    }
                    debug_assert_eq!(occurrence.owner, block);
                    let important = compilation
                        .declaration(declaration)
                        .ok_or(MutationError::<'ast>::UnknownDeclaration(declaration))?
                        .is_important();
                    match occurrence.expansion {
                        EffectExpansion::Barrier(family) => {
                            clear_box_effect_history(
                                &mut self.scratch.previous_box_effects,
                                family,
                            );
                        }
                        EffectExpansion::BoxShorthand(family)
                        | EffectExpansion::BoxLonghand(family, _) => {
                            let live_effects = occurrence.live_effects();
                            for side in 0..4 {
                                let effect = 1 << side;
                                if live_effects & effect == 0 {
                                    continue;
                                }
                                let slot = box_effect_slot(family, important, side);
                                if let Some((previous_block, previous)) =
                                    self.scratch.previous_box_effects[slot]
                                    && self.declaration_ir.mark_effects_dead(
                                        previous_block,
                                        previous,
                                        effect,
                                    )
                                {
                                    let updated = self
                                        .declaration_ir
                                        .occurrence(previous)
                                        .copied()
                                        .expect("an S2 effect remains published");
                                    if !updated.is_live() {
                                        compilation.replace_declaration(
                                            previous_block,
                                            previous,
                                            DeclarationPayload::Property(Declaration::Tombstone),
                                        )?;
                                        self.scratch.affected_blocks.insert(previous_block);
                                        stats.declarations_removed += 1;
                                    } else if matches!(
                                        updated.expansion,
                                        EffectExpansion::BoxShorthand(_)
                                    ) && !updated.is_fully_live()
                                    {
                                        self.dirty_s4_plan_items.push(previous);
                                    }
                                }
                                self.scratch.previous_box_effects[slot] =
                                    Some((block, declaration));
                            }
                        }
                        EffectExpansion::Exact | EffectExpansion::Opaque => {
                            let Some(property_key) = occurrence.property_key else {
                                continue;
                            };
                            if let Some(&(previous_block, previous)) =
                                self.scratch.previous_by_property.get(&property_key)
                                && declarations_are_exactly_equal(
                                    compilation,
                                    &mut self.declaration_ir,
                                    previous,
                                    declaration,
                                )
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
        compilation: &mut AstContext<'ast>,
        preserve_selector_compatibility: bool,
    ) -> Result<S3Stats, MutationError<'ast>> {
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
                let Some(left_ir) = self.declaration_ir.occurrence(left).copied() else {
                    continue;
                };
                if !left_ir.is_exact_match_candidate() {
                    continue;
                }
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
                if let Some(indexed_count) = self
                    .declaration_ir
                    .property_candidates(candidate.right, property_key)
                    .map(<[_]>::len)
                {
                    for index in 0..indexed_count {
                        let indexed = self
                            .declaration_ir
                            .property_candidates(candidate.right, property_key)
                            .and_then(|candidates| candidates.get(index))
                            .copied()
                            .expect("the immutable property index remains stable during matching");
                        let right = indexed.declaration;
                        let right_order = indexed.order;
                        if self.scratch.matched_right.contains(&right)
                            || !self.scratch.right_declarations.contains(&right)
                            || self
                                .declaration_ir
                                .occurrence(right)
                                .is_none_or(|right| !right.is_exact_match_candidate())
                            || !declarations_have_equal_effect(
                                compilation,
                                &mut self.declaration_ir,
                                left,
                                right,
                            )
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
                            || self.declaration_ir.occurrence(right).is_none_or(|right| {
                                !right.is_exact_match_candidate()
                                    || right.property_key != Some(property_key)
                            })
                            || !declarations_have_equal_effect(
                                compilation,
                                &mut self.declaration_ir,
                                left,
                                right,
                            )
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
            if !ast_partial_movement_is_safe(
                &self.scratch.common,
                &self.scratch.left_residual,
                &self.scratch.right_residual,
                &self.declaration_ir,
            ) {
                stats.rejected_unsafe_movement += 1;
                continue;
            }

            let left_selectors = *compilation
                .selector_value(endpoints.left_selector)
                .expect("a validated selector value remains resolvable")
                .selectors();
            let right_selectors = *compilation
                .selector_value(endpoints.right_selector)
                .expect("a validated selector value remains resolvable")
                .selectors();
            let Some(selectors) = materialize_selector_union(
                &left_selectors,
                &right_selectors,
                preserve_selector_compatibility,
                self.allocator,
                compilation,
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
                compilation.set_rule_span(endpoints.left_rule, endpoints.span)?;

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
                self.insert_history_occurrence(compilation, shared_key, candidate.left);

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

            if !compilation.can_insert_declaration_block(self.scratch.common.len()) {
                stats.rejected_capacity += 1;
                continue;
            }
            let payload = match endpoints.selector_kind {
                rocketcss_ast::SelectorFrameKind::Style => {
                    CssRulePayload::Style(StyleRulePayload {
                        selector_value,
                        vendor_prefix: endpoints.vendor_prefix,
                    })
                }
                rocketcss_ast::SelectorFrameKind::Nesting => {
                    CssRulePayload::Nesting(NestingRulePayload { selector_value })
                }
            };
            let rule_result = match compilation.insert_rule_after_with_span(
                endpoints.left_rule,
                payload,
                endpoints.span,
            ) {
                Ok(result) => result,
                Err(MutationError::<'ast>::RuleCapacityExhausted) => {
                    stats.rejected_capacity += 1;
                    continue;
                }
                Err(error) => return Err(error),
            };
            let left_rule = endpoints.left_rule;
            let right_rule = endpoints.right_rule;
            let shared_rule = rule_result;
            let shared_block = compilation.insert_declaration_block(shared_rule, shared_key)?;
            let left_block = candidate.left;
            let right_block = candidate.right;

            for declaration in &self.scratch.common {
                let important = compilation
                    .declaration(declaration.left)
                    .ok_or(MutationError::<'ast>::UnknownDeclaration(declaration.left))?
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
            self.insert_history_occurrence(compilation, shared_key, shared_block);

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

    fn remove_history_occurrence(
        &mut self,
        key: EffectiveKeyId<'ast>,
        block: DeclarationBlockId<'ast>,
    ) {
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

    fn append_history_occurrence(
        &mut self,
        key: EffectiveKeyId<'ast>,
        block: DeclarationBlockId<'ast>,
    ) -> bool {
        if !self.histories.contains_key(&key) {
            self.histories.insert(key, self.allocator.vec());
        }
        let history = self
            .histories
            .get_mut(&key)
            .expect("the history was inserted above");
        if history.last() != Some(&block) {
            history.push(block);
        }
        history.len() == 2
    }

    fn insert_history_occurrence(
        &mut self,
        compilation: &AstContext<'ast>,
        key: EffectiveKeyId<'ast>,
        block: DeclarationBlockId<'ast>,
    ) -> bool {
        if !self.histories.contains_key(&key) {
            self.histories.insert(key, self.allocator.vec());
        }
        let history = self
            .histories
            .get_mut(&key)
            .expect("the history was inserted above");
        if history.contains(&block) {
            return history.len() == 2;
        }
        let source_order_id = compilation
            .declaration_block_source_order_id(block)
            .expect("a history occurrence has a source-order label");
        let insertion_index = history
            .binary_search_by_key(&source_order_id, |candidate| {
                compilation
                    .declaration_block_source_order_id(*candidate)
                    .expect("an existing history occurrence has a source-order label")
            })
            .unwrap_or_else(|index| index);
        history.insert(insertion_index, block);
        history.len() == 2
    }

    fn publish_incident_edges(
        &mut self,
        compilation: &AstContext<'ast>,
        left: RuleId<'ast>,
        right: Option<RuleId<'ast>>,
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

    fn enqueue_rule_incident_edges(&mut self, compilation: &AstContext<'ast>, rule: RuleId<'ast>) {
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

    fn enqueue_edge_candidate(&mut self, edge: Candidate<'ast>) {
        self.direct_style_edges.push(edge);
        if edge.same_effective_key {
            self.same_selector_candidates.push(edge);
        } else if edge.may_share_declaration {
            self.partial_merge_candidates.push(edge);
        }
    }

    fn edge_candidate(
        &self,
        compilation: &AstContext<'ast>,
        left: RuleId<'ast>,
        right: RuleId<'ast>,
    ) -> Option<Candidate<'ast>> {
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

fn is_style_owner(payload: &CssRulePayload<'_>) -> bool {
    matches!(
        payload,
        CssRulePayload::Style(_) | CssRulePayload::Nesting(_)
    )
}

fn validate_s1<'ast>(
    compilation: &AstContext<'ast>,
    candidate: Candidate<'ast>,
) -> Option<(RuleId<'ast>, RuleId<'ast>, EffectiveKeyId<'ast>)> {
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

fn declarations_are_exactly_equal<'ast>(
    compilation: &AstContext<'ast>,
    declaration_ir: &mut DeclarationIrStore<'_, 'ast>,
    left: rocketcss_ast::DeclarationId<'ast>,
    right: rocketcss_ast::DeclarationId<'ast>,
) -> bool {
    let left_id = left;
    let right_id = right;
    let Some(left) = compilation.declaration(left) else {
        return false;
    };
    let Some(right) = compilation.declaration(right) else {
        return false;
    };
    if left.is_important() != right.is_important() {
        return false;
    }
    let (DeclarationPayload::Property(left_value), DeclarationPayload::Property(right_value)) =
        (left.payload(), right.payload())
    else {
        return false;
    };
    if left_value == right_value {
        return true;
    }
    if let Some(equal) =
        crate::equality::known_declaration_structural_equality(compilation, left_value, right_value)
    {
        return equal;
    }
    if !declaration_ir.declarations_have_equal_css(compilation, left_id, right_id) {
        return false;
    }
    crate::equality::declarations_with_equal_css_are_equal(compilation, left_value, right_value)
}

#[derive(Clone, Copy)]
struct S3Endpoints<'ast> {
    left_rule: RuleId<'ast>,
    right_rule: RuleId<'ast>,
    left_key: EffectiveKeyId<'ast>,
    right_key: EffectiveKeyId<'ast>,
    left_selector: rocketcss_ast::SelectorValueId<'ast>,
    right_selector: rocketcss_ast::SelectorValueId<'ast>,
    selector_kind: rocketcss_ast::SelectorFrameKind,
    vendor_prefix: rocketcss_ast::VendorPrefix,
    span: Span,
}

fn validate_s3<'ast>(
    compilation: &AstContext<'ast>,
    candidate: Candidate<'ast>,
) -> Option<S3Endpoints<'ast>> {
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
    let left_selector = match left_rule.payload() {
        CssRulePayload::Style(payload) => payload.selector_value,
        CssRulePayload::Nesting(payload) => payload.selector_value,
        _ => return None,
    };
    let right_selector = match right_rule.payload() {
        CssRulePayload::Style(payload) => payload.selector_value,
        CssRulePayload::Nesting(payload) => payload.selector_value,
        _ => return None,
    };
    let left_span = compilation.rule_span(left_rule_id)?;
    let right_span = compilation.rule_span(right_rule_id)?;
    let left_value = compilation.selector_value(left_selector)?;
    let right_value = compilation.selector_value(right_selector)?;
    if left_selector == right_selector
        || left_value.kind() != right_value.kind()
        || left_value.vendor_prefix() != right_value.vendor_prefix()
    {
        return None;
    }
    Some(S3Endpoints {
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

fn declarations_have_equal_effect<'ast>(
    compilation: &AstContext<'ast>,
    declaration_ir: &mut DeclarationIrStore<'_, 'ast>,
    left: rocketcss_ast::DeclarationId<'ast>,
    right: rocketcss_ast::DeclarationId<'ast>,
) -> bool {
    let left_id = left;
    let right_id = right;
    let Some(left) = compilation.declaration(left) else {
        return false;
    };
    let Some(right) = compilation.declaration(right) else {
        return false;
    };
    if left.is_important() != right.is_important() {
        return false;
    }
    let (DeclarationPayload::Property(left_value), DeclarationPayload::Property(right_value)) =
        (left.payload(), right.payload())
    else {
        return false;
    };
    if left_value.eq_ignoring_tombstones(right_value, compilation) {
        return true;
    }
    if let Some(equal) =
        crate::equality::known_declaration_structural_equality(compilation, left_value, right_value)
    {
        return equal;
    }
    if !declaration_ir.declarations_have_equal_css(compilation, left_id, right_id) {
        return false;
    }
    crate::equality::declarations_with_equal_css_are_equal(compilation, left_value, right_value)
}

fn has_opaque_domain_conflict<'ast>(
    declaration_ir: &DeclarationIrStore<'_, 'ast>,
    domain: MovementDomain,
    declarations: &[rocketcss_ast::DeclarationId<'ast>],
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

fn ast_partial_movement_is_safe<'ast>(
    common: &[CommonDeclaration<'ast>],
    left_residual: &[rocketcss_ast::DeclarationId<'ast>],
    right_residual: &[rocketcss_ast::DeclarationId<'ast>],
    declaration_ir: &DeclarationIrStore<'_, 'ast>,
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
        return ast_common_effect_order_is_safe(common, declaration_ir);
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
    ast_common_effect_order_is_safe(common, declaration_ir)
}

fn ast_common_effect_order_is_safe<'ast>(
    common: &[CommonDeclaration<'ast>],
    declaration_ir: &DeclarationIrStore<'_, 'ast>,
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
        ) -> Result<rocketcss_ast::AstContext<'ast>, rocketcss_parser::Error<'ast>>;
    }

    impl<'ast> ParseTestCompilation<'ast> for Compiler<'ast> {
        fn parse_test_compilation(
            &mut self,
            source: &'ast str,
            options: ParserOptions<'ast>,
        ) -> Result<rocketcss_ast::AstContext<'ast>, rocketcss_parser::Error<'ast>> {
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
    fn nested_declaration_blocks_join_history_without_becoming_style_edges() {
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

        let stats = state.run_s2(&mut compilation).unwrap();
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

        let stats = state.run_s2(&mut compilation).unwrap();
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
        let s2 = state.run_s2(&mut compilation).unwrap();
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

        assert_eq!(state.run_s2(&mut compilation).unwrap(), S2Stats::default());
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
    fn s3_inserts_a_shared_rule_and_block_at_their_final_ast_ids() {
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
            assert!(state.histories.values().all(|history| {
                history.windows(2).all(|pair| {
                    compilation
                        .declaration_block_source_order_id(pair[0])
                        .unwrap()
                        < compilation
                            .declaration_block_source_order_id(pair[1])
                            .unwrap()
                })
            }));
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
    fn s1_after_s3_commits_a_noncontiguous_declaration_chain() {
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
                    .any(|(block_id, block)| {
                        if !block.is_live() {
                            return false;
                        }
                        let declarations = compilation
                            .declaration_ids_in_block(block_id)
                            .unwrap()
                            .map(|id| id.index())
                            .collect::<std::vec::Vec<_>>();
                        declarations.windows(2).any(|pair| pair[0] + 1 != pair[1])
                    })
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
    fn s1_after_s3_preserves_a_large_noncontiguous_declaration_chain() {
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
                    .any(|(block_id, block)| {
                        if !block.is_live() {
                            return false;
                        }
                        let declarations = compilation
                            .declaration_ids_in_block(block_id)
                            .unwrap()
                            .map(|id| id.index())
                            .collect::<std::vec::Vec<_>>();
                        declarations.len() > 4
                            && declarations.windows(2).any(|pair| pair[0] + 1 != pair[1])
                    })
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

    #[test]
    fn s5_accepts_a_no_plan_fixed_point() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation("a{color:red}", ParserOptions::default())
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        state.run(&mut compilation, true).unwrap();
        assert!(state.is_semantic_fixed_point());
        assert!(state.declaration_plans.is_empty());
        assert!(state.commit_s5(&mut compilation).unwrap().is_empty());
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s5_commits_two_typed_plans_in_one_block_and_deduplicates_dirty_output() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                ".a{margin:1px;padding:2px}.x{display:block}.a{margin-left:3px;padding-top:4px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        state.run(&mut compilation, true).unwrap();
        assert_eq!(state.declaration_plans.len(), 2);
        let owner = state.declaration_plans[0].owner;
        assert!(
            state
                .declaration_plans
                .iter()
                .all(|plan| plan.owner == owner)
        );
        let dirty = state.commit_s5(&mut compilation).unwrap();

        assert_eq!(dirty, [owner]);
        let declarations = compilation
            .declarations_in_block(owner)
            .unwrap()
            .map(|record| record.payload())
            .collect::<std::vec::Vec<_>>();
        assert_eq!(declarations.len(), 6);
        assert!(matches!(
            declarations.as_slice(),
            [
                DeclarationPayload::Property(Declaration::MarginTop(_)),
                DeclarationPayload::Property(Declaration::MarginRight(_)),
                DeclarationPayload::Property(Declaration::MarginBottom(_)),
                DeclarationPayload::Property(Declaration::PaddingRight(_)),
                DeclarationPayload::Property(Declaration::PaddingBottom(_)),
                DeclarationPayload::Property(Declaration::PaddingLeft(_)),
            ]
        ));
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s5_rejects_duplicate_origins_before_committing() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                ".a{margin:1px}.x{display:block}.a{margin-left:2px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        state.run(&mut compilation, true).unwrap();
        let plan = state.declaration_plans[0];
        state.declaration_plans.push(plan);
        let revision = compilation
            .declaration_block(plan.owner)
            .unwrap()
            .revision();

        assert_eq!(
            state.commit_s5(&mut compilation),
            Err(MutationError::UnknownDeclaration(plan.origin))
        );
        assert!(matches!(
            compilation.declaration(plan.origin).unwrap().payload(),
            DeclarationPayload::Property(Declaration::Margin(_))
        ));
        assert_eq!(
            compilation
                .declaration_block(plan.owner)
                .unwrap()
                .revision(),
            revision
        );
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s5_batch_preflight_rejects_the_last_invalid_plan_atomically() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                ".a{margin:1px;padding:2px}.x{display:block}.a{margin-left:3px;padding-top:4px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        state.run(&mut compilation, true).unwrap();
        assert_eq!(state.declaration_plans.len(), 2);
        let first = state.declaration_plans[0];
        let last = state.declaration_plans.last_mut().unwrap();
        let AstDeclarationPlanKind::MaterializeBoxLonghands { live_effects, .. } = &mut last.kind;
        *live_effects = 0;
        let revision = compilation
            .declaration_block(first.owner)
            .unwrap()
            .revision();

        assert!(state.commit_s5(&mut compilation).is_err());
        assert!(matches!(
            compilation.declaration(first.origin).unwrap().payload(),
            DeclarationPayload::Property(Declaration::Margin(_))
        ));
        assert_eq!(
            compilation
                .declaration_block(first.owner)
                .unwrap()
                .revision(),
            revision
        );
        assert_eq!(compilation.validate_ast(), Ok(()));
    }

    #[test]
    fn s5_rejects_every_stale_or_mismatched_plan_snapshot() {
        for mismatch in 0..4 {
            let allocator = Allocator::new();
            let mut compilation = Compiler::new(&allocator)
                .parse_test_compilation(
                    ".a{margin:1px}.x{display:block}.a{margin-left:2px}",
                    ParserOptions::default(),
                )
                .unwrap();
            let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

            state.run(&mut compilation, true).unwrap();
            let plan = &mut state.declaration_plans[0];
            let origin = plan.origin;
            let owner = plan.owner;
            let revision = compilation.declaration_block(owner).unwrap().revision();
            match mismatch {
                0 => plan.block_revision = plan.block_revision.wrapping_add(1),
                1 => plan.effect_revision = plan.effect_revision.wrapping_add(1),
                2 => plan.important = !plan.important,
                3 => {
                    let AstDeclarationPlanKind::MaterializeBoxLonghands { family, .. } =
                        &mut plan.kind;
                    *family = BoxFamily::Padding;
                }
                _ => unreachable!(),
            }

            assert!(state.commit_s5(&mut compilation).is_err());
            assert!(matches!(
                compilation.declaration(origin).unwrap().payload(),
                DeclarationPayload::Property(Declaration::Margin(_))
            ));
            assert_eq!(
                compilation.declaration_block(owner).unwrap().revision(),
                revision
            );
            assert_eq!(compilation.validate_ast(), Ok(()));
        }
    }

    #[test]
    fn s5_commits_plans_from_separate_blocks_in_dirty_order() {
        let allocator = Allocator::new();
        let mut compilation = Compiler::new(&allocator)
            .parse_test_compilation(
                ".a{margin:1px}.b{padding:2px}.x{display:block}.a{margin-left:3px}.b{padding-top:4px}",
                ParserOptions::default(),
            )
            .unwrap();
        let mut state = CrossRuleState::from_compilation(&compilation).unwrap();

        state.run(&mut compilation, true).unwrap();
        assert_eq!(state.declaration_plans.len(), 2);
        let planned_owners = state
            .declaration_plans
            .iter()
            .map(|plan| plan.owner)
            .collect::<std::vec::Vec<_>>();
        assert_ne!(planned_owners[0], planned_owners[1]);

        let dirty = state.commit_s5(&mut compilation).unwrap();
        assert_eq!(dirty, planned_owners);
        assert_eq!(compilation.validate_ast(), Ok(()));
    }
}
