use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, VecDeque},
};

use rocketcss_ast::{
    CssRule, DeclarationBlockId, DeclarationBlockStore, SelectorList, Span, StyleRule, StyleSheet,
    VendorPrefix,
};
use rocketcss_common::{DenseId, vec::Vec};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use super::declaration_ir::{DeclarationSlot, FrozenDeclarationIrStore};
#[cfg(test)]
use super::partial_selector::PartialMergeRejection;
use super::partial_selector::{
    PartialMergePlacement, PartialRuleRef, commit_partial_merge_declarations,
    discover_partial_merge_plan, materialize_selector_union,
};
use crate::rules::DeclarationBlockMinifier;
use crate::utils::{
    DeclarationBlockDiscovery, DeclarationBlockEntryId, DeclarationBlockKind, EffectiveKeyId,
    RuleListId, RuleListSegmentId, WalkState, ends_rule_list_segment,
};
use crate::{MinifyContext, Options, OptionsOp};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct LiveBlockFlags: u8 {
        const HAS_CHILDREN = 1 << 0;
        const HAS_LIVE_SELECTORS = 1 << 1;
        const LIVE = 1 << 2;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SemanticOrderKey(SmallVec<[u32; 2]>);

impl SemanticOrderKey {
    fn initial(index: usize) -> Self {
        let value = u32::try_from(index)
            .ok()
            .and_then(|index| index.checked_add(1))
            .expect("declaration-block occurrence count exceeds u32::MAX");
        Self(SmallVec::from_slice(&[value]))
    }

    fn between(left: &Self, right: &Self) -> Self {
        debug_assert!(left < right);
        let mut digits = SmallVec::new();
        let mut index = 0;
        let mut upper_is_open = false;
        loop {
            let lower = left.0.get(index).copied().unwrap_or(0);
            let upper = if upper_is_open {
                u32::MAX
            } else {
                right.0.get(index).copied().unwrap_or(u32::MAX)
            };
            if lower < upper && upper - lower > 1 {
                digits.push(lower + (upper - lower) / 2);
                let result = Self(digits);
                debug_assert!(left < &result && &result < right);
                return result;
            }
            digits.push(lower);
            upper_is_open |= lower < upper;
            index += 1;
        }
    }
}

enum SelectorSource<'walk, 'ast> {
    Authored(&'walk SelectorList<'ast>),
    Synthesized(SelectorList<'ast>),
}

impl<'ast> SelectorSource<'_, 'ast> {
    fn get(&self) -> &SelectorList<'ast> {
        match self {
            Self::Authored(selectors) => selectors,
            Self::Synthesized(selectors) => selectors,
        }
    }
}

struct LiveBlock<'walk, 'ast> {
    effective_key: EffectiveKeyId,
    previous_live: Option<DeclarationBlockId>,
    next_live: Option<DeclarationBlockId>,
    rule_list: RuleListId,
    rule_list_segment: RuleListSegmentId,
    order: SemanticOrderKey,
    selectors: SelectorSource<'walk, 'ast>,
    span: Span,
    vendor_prefix: VendorPrefix,
    flags: LiveBlockFlags,
    revision: u32,
}

struct PartialMergeTransition<'ast> {
    left: DeclarationBlockId,
    right: DeclarationBlockId,
    left_key: EffectiveKeyId,
    right_key: EffectiveKeyId,
    shared_key: EffectiveKeyId,
    selectors: SelectorList<'ast>,
    span: Span,
    vendor_prefix: VendorPrefix,
    retain_right: bool,
    rule_list: RuleListId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct EdgeCandidate {
    left: DeclarationBlockId,
    right: DeclarationBlockId,
    left_revision: u32,
    right_revision: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PartialCandidateKey {
    left_order: SemanticOrderKey,
    candidate: EdgeCandidate,
}

#[derive(Default)]
struct PartialCandidateQueue {
    pending: BinaryHeap<Reverse<PartialCandidateKey>>,
    queued: FxHashSet<EdgeCandidate>,
}

impl PartialCandidateQueue {
    fn push(&mut self, candidate: EdgeCandidate, left_order: SemanticOrderKey) -> bool {
        if self.queued.insert(candidate) {
            self.pending.push(Reverse(PartialCandidateKey {
                left_order,
                candidate,
            }));
            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> Option<EdgeCandidate> {
        let candidate = self.pending.pop()?.0.candidate;
        self.queued.remove(&candidate);
        Some(candidate)
    }
}

#[derive(Default)]
struct CandidateQueue {
    pending: VecDeque<EdgeCandidate>,
    queued: FxHashSet<EdgeCandidate>,
}

impl CandidateQueue {
    fn push(&mut self, candidate: EdgeCandidate) {
        if self.queued.insert(candidate) {
            self.pending.push_back(candidate);
        }
    }

    fn pop(&mut self) -> Option<EdgeCandidate> {
        let candidate = self.pending.pop_front()?;
        self.queued.remove(&candidate);
        Some(candidate)
    }
}

struct DirtyEffectiveKeys {
    pending: VecDeque<EffectiveKeyId>,
    queued: FxHashSet<EffectiveKeyId>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default)]
struct SchedulerStats {
    initial_collections: usize,
    s1_queue_pops: usize,
    s2_queue_pops: usize,
    s3_queue_pops: usize,
    s3_edges_considered: usize,
    s3_candidates_enqueued: usize,
    s3_commits: usize,
    s3_reused_left_commits: usize,
    s3_allocated_shared_commits: usize,
    s3_declaration_blocks_appended: usize,
    s3_occurrences_appended: usize,
    semantic_order_between_calls: usize,
    s3_bloom_rejections: usize,
    s3_ineligible_rejections: usize,
    s3_no_common_rejections: usize,
    s3_unsafe_movement_rejections: usize,
    s3_selector_rejections: usize,
    stale_candidates: usize,
    edge_classifications: usize,
    history_insertions: usize,
    reification_passes: usize,
}

impl DirtyEffectiveKeys {
    fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            queued: FxHashSet::default(),
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
}

pub(super) struct CrossRuleMergeState<'walk, 'ast> {
    discovery: DeclarationBlockDiscovery<'walk, 'ast>,
    declaration_ir: FrozenDeclarationIrStore<'ast>,
    nodes: std::vec::Vec<Option<LiveBlock<'walk, 'ast>>>,
    owner_by_block: std::vec::Vec<Option<DeclarationBlockId>>,
    histories: FxHashMap<EffectiveKeyId, BTreeMap<SemanticOrderKey, DeclarationBlockId>>,
    occurrence_order: std::vec::Vec<Option<SemanticOrderKey>>,
    style_segments: FxHashSet<(RuleListId, RuleListSegmentId)>,
    affected_rule_lists: FxHashSet<RuleListId>,
    same_selector_candidates: CandidateQueue,
    declaration_override_candidates: DirtyEffectiveKeys,
    partial_merge_candidates: PartialCandidateQueue,
    #[cfg(test)]
    stats: SchedulerStats,
}

impl<'walk, 'ast> CrossRuleMergeState<'walk, 'ast> {
    pub(super) fn new(
        discovery: DeclarationBlockDiscovery<'walk, 'ast>,
        declaration_blocks: &DeclarationBlockStore<'ast>,
        mut declaration_ir: FrozenDeclarationIrStore<'ast>,
    ) -> Self {
        let mut nodes = std::vec::Vec::with_capacity(declaration_blocks.len());
        nodes.resize_with(declaration_blocks.len(), || None);
        let mut owner_by_block = vec![None; declaration_blocks.len()];
        let mut occurrence_order = vec![None; declaration_blocks.len()];
        let mut histories: FxHashMap<_, BTreeMap<_, _>> = FxHashMap::default();
        let mut style_segments = FxHashSet::default();
        let mut last_style_by_segment: FxHashMap<
            (RuleListId, RuleListSegmentId),
            (DeclarationBlockId, DeclarationBlockEntryId),
        > = FxHashMap::default();

        for (entry_id, entry) in discovery.declaration_blocks.iter_enumerated() {
            declaration_ir.initialize_owner_chain(entry.declarations, declaration_blocks);
            let order = SemanticOrderKey::initial(entry_id.index());
            histories
                .entry(entry.effective_key)
                .or_default()
                .insert(order.clone(), entry.declarations);
            occurrence_order[entry.declarations.index()] = Some(order.clone());

            let DeclarationBlockKind::Style {
                selectors,
                span,
                vendor_prefix,
                has_children,
                has_live_selectors,
            } = entry.kind
            else {
                continue;
            };
            let block = entry.declarations;
            owner_by_block[block.index()] = Some(block);
            let segment = (entry.rule_list, entry.rule_list_segment);
            style_segments.insert(segment);
            let mut flags = LiveBlockFlags::LIVE;
            flags.set(LiveBlockFlags::HAS_CHILDREN, has_children);
            flags.set(LiveBlockFlags::HAS_LIVE_SELECTORS, has_live_selectors);
            nodes[block.index()] = Some(LiveBlock {
                effective_key: entry.effective_key,
                previous_live: None,
                next_live: None,
                rule_list: entry.rule_list,
                rule_list_segment: entry.rule_list_segment,
                order,
                selectors: SelectorSource::Authored(selectors),
                span,
                vendor_prefix,
                flags,
                revision: 0,
            });

            if let Some(&(previous, previous_entry)) = last_style_by_segment.get(&segment)
                && discovery.declaration_blocks[previous_entry].is_direct_sibling_of(entry)
            {
                nodes[previous.index()]
                    .as_mut()
                    .expect("the previous style block is live")
                    .next_live = Some(block);
                nodes[block.index()]
                    .as_mut()
                    .expect("the current style block is live")
                    .previous_live = Some(previous);
            }
            last_style_by_segment.insert(segment, (block, entry_id));
        }

        let mut state = Self {
            discovery,
            declaration_ir,
            nodes,
            owner_by_block,
            histories,
            occurrence_order,
            style_segments,
            affected_rule_lists: FxHashSet::default(),
            same_selector_candidates: CandidateQueue::default(),
            declaration_override_candidates: DirtyEffectiveKeys::new(),
            partial_merge_candidates: PartialCandidateQueue::default(),
            #[cfg(test)]
            stats: SchedulerStats {
                initial_collections: 1,
                ..SchedulerStats::default()
            },
        };
        let initial_edges = state
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(index, node)| {
                node.as_ref().and_then(|node| {
                    node.next_live.map(|right| {
                        (
                            DeclarationBlockId::from_index(index)
                                .expect("a live node index is a declaration block ID"),
                            right,
                        )
                    })
                })
            })
            .collect::<std::vec::Vec<_>>();
        for (left, right) in initial_edges {
            state.enqueue_edge(left, right);
        }
        let dirty_keys = state
            .histories
            .iter()
            .filter_map(|(&key, history)| (history.len() > 1).then_some(key))
            .collect::<std::vec::Vec<_>>();
        for key in dirty_keys {
            state.declaration_override_candidates.push(key);
        }
        state
    }

    #[cfg(test)]
    fn discover(
        stylesheet: &'walk StyleSheet<'ast>,
        declaration_blocks: &DeclarationBlockStore<'ast>,
    ) -> Self {
        let discovery = crate::utils::discover_declaration_blocks(stylesheet);
        let mut declaration_ir = FrozenDeclarationIrStore::default();
        for entry in discovery.declaration_blocks.iter() {
            declaration_ir.freeze_physical_block(
                entry.declarations,
                declaration_blocks.get(entry.declarations),
            );
        }
        Self::new(discovery, declaration_blocks, declaration_ir)
    }

    pub(super) fn run<'scratch>(
        &mut self,
        declaration_blocks: &mut DeclarationBlockStore<'ast>,
        declaration_block_minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
        cx: &mut MinifyContext<'scratch>,
    ) where
        'ast: 'scratch,
    {
        loop {
            if let Some(candidate) = self.same_selector_candidates.pop() {
                #[cfg(test)]
                {
                    self.stats.s1_queue_pops += 1;
                }
                self.commit_same_selector_candidate(candidate, declaration_blocks);
                continue;
            }
            if let Some(key) = self.declaration_override_candidates.pop() {
                #[cfg(test)]
                {
                    self.stats.s2_queue_pops += 1;
                }
                self.commit_declaration_override_candidate(
                    key,
                    declaration_blocks,
                    declaration_block_minifier,
                    cx,
                );
                continue;
            }
            if let Some(candidate) = self.partial_merge_candidates.pop() {
                #[cfg(test)]
                {
                    self.stats.s3_queue_pops += 1;
                }
                self.commit_partial_merge_candidate(candidate, declaration_blocks, cx);
                continue;
            }
            break;
        }
    }

    fn commit_same_selector_candidate(
        &mut self,
        candidate: EdgeCandidate,
        declaration_blocks: &mut DeclarationBlockStore<'ast>,
    ) {
        let Some((left, right)) = self.validate_edge(candidate, true) else {
            #[cfg(test)]
            {
                self.stats.stale_candidates += 1;
            }
            return;
        };
        let right_head = oldest_declaration_block(right, declaration_blocks);
        declaration_blocks
            .get_mut(right_head)
            .set_previous_merged(Some(left));
        for block in declaration_chain(left, declaration_blocks) {
            self.owner_by_block[block.index()] = Some(right);
        }
        self.declaration_ir.compose(left, right);
        self.bump_revision(right);
        let key = self.node(left).unwrap().effective_key;
        self.retire_node_without_relink(left);
        self.remove_history_occurrence(key, left);
        self.link_around_retired(left);
        self.declaration_override_candidates.push(key);
        self.enqueue_incident_edges(right);
    }

    fn commit_declaration_override_candidate<'scratch>(
        &mut self,
        key: EffectiveKeyId,
        declaration_blocks: &mut DeclarationBlockStore<'ast>,
        minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
        cx: &mut MinifyContext<'scratch>,
    ) where
        'ast: 'scratch,
    {
        let Some(history) = self.histories.get(&key) else {
            return;
        };
        let occurrences = history.values().copied().collect::<std::vec::Vec<_>>();
        let mut expanded = std::vec::Vec::new();
        let mut seen = FxHashSet::default();
        for &block in &occurrences {
            append_declaration_chain(block, declaration_blocks, &mut seen, &mut expanded);
        }
        if expanded.len() < 2 {
            return;
        }

        let mut removed = SmallVec::<[DeclarationSlot; 8]>::new();
        minifier.deduplicate_exact_sequence(&expanded, declaration_blocks, cx, |block, index| {
            removed.push(DeclarationSlot { block, index })
        });
        if removed.is_empty() {
            return;
        }

        let mut affected_owners = FxHashSet::default();
        for slot in removed {
            if let Some(owner) = self.owner_by_block[slot.block.index()]
                && let Some(occurrence) = self.declaration_ir.occurrence_for_slot(slot)
            {
                self.declaration_ir.mark_dead(owner, occurrence);
                affected_owners.insert(owner);
            }
        }
        for owner in affected_owners {
            if self.node(owner).is_some_and(|node| {
                node.flags.contains(LiveBlockFlags::LIVE)
                    && !node.flags.contains(LiveBlockFlags::HAS_CHILDREN)
                    && self.declaration_ir.live_count(owner) == 0
            }) {
                let owner_key = self.node(owner).unwrap().effective_key;
                self.retire_node_without_relink(owner);
                self.remove_history_occurrence(owner_key, owner);
                self.link_around_retired(owner);
                self.declaration_override_candidates.push(owner_key);
            } else {
                self.bump_revision(owner);
                self.enqueue_incident_edges(owner);
            }
        }
    }

    fn commit_partial_merge_candidate<'scratch>(
        &mut self,
        candidate: EdgeCandidate,
        declaration_blocks: &mut DeclarationBlockStore<'ast>,
        cx: &mut MinifyContext<'scratch>,
    ) where
        'ast: 'scratch,
    {
        let Some((left, right)) = self.validate_edge(candidate, false) else {
            #[cfg(test)]
            {
                self.stats.stale_candidates += 1;
            }
            return;
        };
        let (
            left_key,
            right_key,
            left_selectors,
            right_selectors,
            left_span,
            right_span,
            vendor_prefix,
            right_vendor_prefix,
            left_has_children,
            right_has_children,
            rule_list,
        ) = {
            let left_node = self.nodes[left.index()].as_ref().unwrap();
            let right_node = self.nodes[right.index()].as_ref().unwrap();
            (
                left_node.effective_key,
                right_node.effective_key,
                left_node.selectors.get(),
                right_node.selectors.get(),
                left_node.span,
                right_node.span,
                left_node.vendor_prefix,
                right_node.vendor_prefix,
                left_node.flags.contains(LiveBlockFlags::HAS_CHILDREN),
                right_node.flags.contains(LiveBlockFlags::HAS_CHILDREN),
                left_node.rule_list,
            )
        };
        let plan = discover_partial_merge_plan(
            PartialRuleRef {
                declarations: left,
                selectors: left_selectors,
                span: left_span,
                vendor_prefix,
                has_children: left_has_children,
            },
            PartialRuleRef {
                declarations: right,
                selectors: right_selectors,
                span: right_span,
                vendor_prefix: right_vendor_prefix,
                has_children: right_has_children,
            },
            declaration_blocks,
            &mut self.declaration_ir,
            cx,
        );
        let plan = match plan {
            Ok(plan) => plan,
            Err(rejection) => {
                #[cfg(test)]
                match rejection {
                    PartialMergeRejection::Ineligible => {
                        self.stats.s3_ineligible_rejections += 1;
                    }
                    PartialMergeRejection::NoCommonDeclaration => {
                        self.stats.s3_no_common_rejections += 1;
                    }
                    PartialMergeRejection::UnsafeMovement => {
                        self.stats.s3_unsafe_movement_rejections += 1;
                    }
                    PartialMergeRejection::IncompatibleSelectors => {
                        self.stats.s3_selector_rejections += 1;
                    }
                }
                #[cfg(not(test))]
                let _ = rejection;
                return;
            }
        };
        let key_selectors = materialize_selector_union(
            left_selectors,
            right_selectors,
            cx.is_enabled(Options::PRESERVE_SELECTOR_COMPATIBILITY, OptionsOp::Any),
        )
        .expect("selector compatibility was validated by S3 discovery");
        let key_selectors = key_selectors.bump().alloc(key_selectors);
        let Some(shared_key) = self.discovery.effective_keys.intern_selector_union(
            left_key,
            right_key,
            key_selectors,
            vendor_prefix,
        ) else {
            return;
        };

        let retain_right = plan.retain_right;
        let shared_span = plan.span;
        let shared_vendor_prefix = plan.vendor_prefix;
        let commit = commit_partial_merge_declarations(
            &plan,
            declaration_blocks,
            &mut self.declaration_ir,
            cx,
        );
        let transition = PartialMergeTransition {
            left,
            right,
            left_key,
            right_key,
            shared_key,
            selectors: plan.selectors,
            span: shared_span,
            vendor_prefix: shared_vendor_prefix,
            retain_right,
            rule_list,
        };
        #[cfg(test)]
        {
            self.stats.s3_commits += 1;
            self.stats.s3_occurrences_appended += match commit.placement {
                PartialMergePlacement::ReusedLeft => 0,
                PartialMergePlacement::AllocatedBetween => commit.declaration_count as usize,
            };
        }
        match commit.placement {
            PartialMergePlacement::ReusedLeft => {
                debug_assert_eq!(commit.shared, left);
                self.commit_partial_merge_reusing_left(transition);
            }
            PartialMergePlacement::AllocatedBetween => {
                self.commit_partial_merge_allocating(transition, commit.shared);
            }
        }
    }

    fn commit_partial_merge_reusing_left(&mut self, transition: PartialMergeTransition<'ast>) {
        let PartialMergeTransition {
            left,
            right,
            left_key,
            right_key,
            shared_key,
            selectors,
            span,
            vendor_prefix,
            retain_right,
            rule_list,
        } = transition;
        let previous = self.node(left).unwrap().previous_live;
        let next = self.node(right).unwrap().next_live;

        if !retain_right {
            self.retire_node_without_relink(right);
            self.remove_history_occurrence(right_key, right);
        }
        if shared_key != left_key {
            self.move_history_occurrence(left_key, shared_key, left);
            #[cfg(test)]
            {
                self.stats.history_insertions += 1;
            }
        }

        {
            let node = self.node_mut(left).unwrap();
            debug_assert!(!node.flags.contains(LiveBlockFlags::HAS_CHILDREN));
            node.effective_key = shared_key;
            node.selectors = SelectorSource::Synthesized(selectors);
            node.span = span;
            node.vendor_prefix = vendor_prefix;
            node.flags = LiveBlockFlags::LIVE | LiveBlockFlags::HAS_LIVE_SELECTORS;
            node.revision = node.revision.wrapping_add(1);
        }
        if retain_right {
            self.bump_revision(right);
        } else {
            self.set_next(left, next);
            if let Some(next) = next {
                self.set_previous(next, Some(left));
            }
            self.set_previous(right, None);
            self.set_next(right, None);
        }

        self.affected_rule_lists.insert(rule_list);
        #[cfg(test)]
        {
            self.stats.s3_reused_left_commits += 1;
        }
        self.declaration_override_candidates.push(left_key);
        self.declaration_override_candidates.push(right_key);
        self.declaration_override_candidates.push(shared_key);
        for node in [previous, Some(left), retain_right.then_some(right), next]
            .into_iter()
            .flatten()
        {
            if self
                .node(node)
                .is_some_and(|node| node.flags.contains(LiveBlockFlags::LIVE))
            {
                self.enqueue_incident_edges(node);
            }
        }
    }

    fn commit_partial_merge_allocating(
        &mut self,
        transition: PartialMergeTransition<'ast>,
        shared: DeclarationBlockId,
    ) {
        let PartialMergeTransition {
            left,
            right,
            left_key,
            right_key,
            shared_key,
            selectors,
            span,
            vendor_prefix,
            retain_right,
            rule_list,
        } = transition;
        let (left_order, right_order, rule_list_segment, previous, next) = {
            let left_node = self.node(left).unwrap();
            let right_node = self.node(right).unwrap();
            (
                left_node.order.clone(),
                right_node.order.clone(),
                left_node.rule_list_segment,
                left_node.previous_live,
                right_node.next_live,
            )
        };
        let shared_order = SemanticOrderKey::between(&left_order, &right_order);
        debug_assert_eq!(shared.index(), self.nodes.len());
        self.nodes.push(Some(LiveBlock {
            effective_key: shared_key,
            previous_live: Some(left),
            next_live: retain_right.then_some(right).or(next),
            rule_list,
            rule_list_segment,
            order: shared_order.clone(),
            selectors: SelectorSource::Synthesized(selectors),
            span,
            vendor_prefix,
            flags: LiveBlockFlags::LIVE | LiveBlockFlags::HAS_LIVE_SELECTORS,
            revision: 0,
        }));
        self.owner_by_block.push(Some(shared));
        self.occurrence_order.push(Some(shared_order.clone()));

        if !retain_right {
            self.retire_node_without_relink(right);
            self.remove_history_occurrence(right_key, right);
        }
        self.set_next(left, Some(shared));
        if retain_right {
            self.set_previous(right, Some(shared));
        } else if let Some(next) = next {
            self.set_previous(next, Some(shared));
        }

        self.histories
            .entry(shared_key)
            .or_default()
            .insert(shared_order, shared);
        self.affected_rule_lists.insert(rule_list);
        #[cfg(test)]
        {
            self.stats.s3_allocated_shared_commits += 1;
            self.stats.s3_declaration_blocks_appended += 1;
            self.stats.semantic_order_between_calls += 1;
            self.stats.history_insertions += 1;
        }
        self.declaration_override_candidates.push(left_key);
        self.declaration_override_candidates.push(right_key);
        self.declaration_override_candidates.push(shared_key);
        for node in [previous, Some(left), Some(shared), Some(right), next]
            .into_iter()
            .flatten()
        {
            if self
                .node(node)
                .is_some_and(|node| node.flags.contains(LiveBlockFlags::LIVE))
            {
                self.enqueue_incident_edges(node);
            }
        }
    }

    fn validate_edge(
        &self,
        candidate: EdgeCandidate,
        same_selector: bool,
    ) -> Option<(DeclarationBlockId, DeclarationBlockId)> {
        let left = self.node(candidate.left)?;
        let right = self.node(candidate.right)?;
        (left.flags.contains(LiveBlockFlags::LIVE)
            && right.flags.contains(LiveBlockFlags::LIVE)
            && left.revision == candidate.left_revision
            && right.revision == candidate.right_revision
            && left.next_live == Some(candidate.right)
            && right.previous_live == Some(candidate.left)
            && !left.flags.contains(LiveBlockFlags::HAS_CHILDREN)
            && left.flags.contains(LiveBlockFlags::HAS_LIVE_SELECTORS)
            && (left.effective_key == right.effective_key) == same_selector)
            .then_some((candidate.left, candidate.right))
    }

    fn enqueue_incident_edges(&mut self, node: DeclarationBlockId) {
        let Some(state) = self.node(node) else {
            return;
        };
        let previous = state.previous_live;
        let next = state.next_live;
        if let Some(previous) = previous {
            self.enqueue_edge(previous, node);
        }
        if let Some(next) = next {
            self.enqueue_edge(node, next);
        }
    }

    fn enqueue_edge(&mut self, left: DeclarationBlockId, right: DeclarationBlockId) {
        let Some(left_state) = self.node(left) else {
            return;
        };
        let Some(right_state) = self.node(right) else {
            return;
        };
        if !left_state.flags.contains(LiveBlockFlags::LIVE)
            || !right_state.flags.contains(LiveBlockFlags::LIVE)
            || left_state.next_live != Some(right)
            || right_state.previous_live != Some(left)
        {
            return;
        }
        let candidate = EdgeCandidate {
            left,
            right,
            left_revision: left_state.revision,
            right_revision: right_state.revision,
        };
        let same_selector = left_state.effective_key == right_state.effective_key;
        let left_order = left_state.order.clone();
        let left_property_bloom = self.declaration_ir.property_bloom(left);
        let right_property_bloom = self.declaration_ir.property_bloom(right);
        #[cfg(test)]
        {
            self.stats.edge_classifications += 1;
        }
        if same_selector {
            self.same_selector_candidates.push(candidate);
        } else {
            #[cfg(test)]
            {
                self.stats.s3_edges_considered += 1;
            }
            if !left_property_bloom.may_share_declaration(right_property_bloom) {
                #[cfg(test)]
                {
                    self.stats.s3_bloom_rejections += 1;
                }
            } else {
                let inserted = self.partial_merge_candidates.push(candidate, left_order);
                #[cfg(test)]
                if inserted {
                    self.stats.s3_candidates_enqueued += 1;
                }
                #[cfg(not(test))]
                let _ = inserted;
            }
        }
    }

    fn retire_node_without_relink(&mut self, block: DeclarationBlockId) {
        let Some(node) = self.node_mut(block) else {
            return;
        };
        let rule_list = node.rule_list;
        node.flags.remove(LiveBlockFlags::LIVE);
        node.revision = node.revision.wrapping_add(1);
        self.affected_rule_lists.insert(rule_list);
        if self.owner_by_block[block.index()] == Some(block) {
            self.owner_by_block[block.index()] = None;
        }
    }

    fn link_around_retired(&mut self, block: DeclarationBlockId) {
        let (previous, next) = {
            let node = self.node(block).unwrap();
            (node.previous_live, node.next_live)
        };
        if let Some(previous) = previous {
            self.set_next(previous, next);
        }
        if let Some(next) = next {
            self.set_previous(next, previous);
        }
        self.set_previous(block, None);
        self.set_next(block, None);
        if let (Some(previous), Some(next)) = (previous, next) {
            self.enqueue_edge(previous, next);
        }
    }

    fn remove_history_occurrence(&mut self, key: EffectiveKeyId, block: DeclarationBlockId) {
        let Some(order) = self.occurrence_order[block.index()].take() else {
            return;
        };
        if let Some(history) = self.histories.get_mut(&key) {
            history.remove(&order);
        }
    }

    fn move_history_occurrence(
        &mut self,
        from: EffectiveKeyId,
        to: EffectiveKeyId,
        block: DeclarationBlockId,
    ) {
        debug_assert_ne!(from, to);
        let order = self.occurrence_order[block.index()]
            .as_ref()
            .expect("a live block has a semantic occurrence order")
            .clone();
        let removed = self
            .histories
            .get_mut(&from)
            .and_then(|history| history.remove(&order));
        debug_assert_eq!(removed, Some(block));
        let replaced = self.histories.entry(to).or_default().insert(order, block);
        debug_assert!(replaced.is_none());
    }

    fn set_previous(&mut self, block: DeclarationBlockId, previous: Option<DeclarationBlockId>) {
        let node = self.node_mut(block).unwrap();
        if node.previous_live != previous {
            node.previous_live = previous;
            node.revision = node.revision.wrapping_add(1);
        }
    }

    fn set_next(&mut self, block: DeclarationBlockId, next: Option<DeclarationBlockId>) {
        let node = self.node_mut(block).unwrap();
        if node.next_live != next {
            node.next_live = next;
            node.revision = node.revision.wrapping_add(1);
        }
    }

    fn bump_revision(&mut self, block: DeclarationBlockId) {
        if let Some(node) = self.node_mut(block) {
            node.revision = node.revision.wrapping_add(1);
        }
    }

    fn node(&self, block: DeclarationBlockId) -> Option<&LiveBlock<'walk, 'ast>> {
        self.nodes.get(block.index())?.as_ref()
    }

    fn node_mut(&mut self, block: DeclarationBlockId) -> Option<&mut LiveBlock<'walk, 'ast>> {
        self.nodes.get_mut(block.index())?.as_mut()
    }

    pub(super) fn into_reification_plan(self) -> ReificationPlan<'ast> {
        let mut segments = FxHashMap::default();
        for segment in self.style_segments {
            if self.affected_rule_lists.contains(&segment.0) {
                segments.insert(segment, std::vec::Vec::new());
            }
        }
        for (block_index, node) in self.nodes.into_iter().enumerate() {
            let Some(node) = node else {
                continue;
            };
            if !node.flags.contains(LiveBlockFlags::LIVE) {
                continue;
            }
            if !self.affected_rule_lists.contains(&node.rule_list) {
                continue;
            }
            let block = DeclarationBlockId::from_index(block_index)
                .expect("a live node index is a valid declaration block ID");
            let output = match node.selectors {
                SelectorSource::Authored(_) => ReifiedStyleRule::Authored(block),
                SelectorSource::Synthesized(selectors) => ReifiedStyleRule::Synthesized {
                    declarations: block,
                    selectors,
                    span: node.span,
                    vendor_prefix: node.vendor_prefix,
                },
            };
            segments
                .entry((node.rule_list, node.rule_list_segment))
                .or_default()
                .push((node.order, output));
        }
        let segments = segments
            .into_iter()
            .map(|(segment, mut rules)| {
                rules.sort_unstable_by(|left, right| left.0.cmp(&right.0));
                (segment, rules.into_iter().map(|(_, rule)| rule).collect())
            })
            .collect();
        ReificationPlan {
            segments,
            affected_rule_lists: self.affected_rule_lists,
            #[cfg(test)]
            stats: self.stats,
        }
    }
}

enum ReifiedStyleRule<'ast> {
    Authored(DeclarationBlockId),
    Synthesized {
        declarations: DeclarationBlockId,
        selectors: SelectorList<'ast>,
        span: Span,
        vendor_prefix: VendorPrefix,
    },
}

pub(crate) struct ReificationPlan<'ast> {
    segments: FxHashMap<(RuleListId, RuleListSegmentId), std::vec::Vec<ReifiedStyleRule<'ast>>>,
    affected_rule_lists: FxHashSet<RuleListId>,
    #[cfg(test)]
    stats: SchedulerStats,
}

impl<'ast> ReificationPlan<'ast> {
    pub(crate) fn apply(mut self, stylesheet: &mut StyleSheet<'ast>) {
        if self.affected_rule_lists.is_empty() {
            debug_assert!(self.segments.is_empty());
            return;
        }
        let mut state = WalkState::default();
        self.reify_rule_list(&mut stylesheet.rules, &mut state);
        debug_assert!(self.segments.is_empty());
        debug_assert!(self.affected_rule_lists.is_empty());
    }

    fn reify_rule_list(&mut self, rules: &mut Vec<'ast, CssRule<'ast>>, state: &mut WalkState) {
        let rule_list = state.allocate_rule_list();
        let mut rule_list_segment = state.allocate_rule_list_segment();
        let mut locations = std::vec::Vec::with_capacity(rules.len());
        for rule in rules.iter_mut() {
            locations.push(rule_list_segment);
            match rule {
                CssRule::Media(rule) => self.reify_rule_list(&mut rule.rules, state),
                CssRule::Style(rule) => {
                    let rules = rule.as_mut().rules_mut();
                    if !rules.is_empty() {
                        self.reify_rule_list(rules, state);
                    }
                }
                CssRule::Supports(rule) => self.reify_rule_list(&mut rule.rules, state),
                CssRule::MozDocument(rule) => self.reify_rule_list(&mut rule.rules, state),
                CssRule::Nesting(rule) => {
                    let rules = rule.style.as_mut().rules_mut();
                    if !rules.is_empty() {
                        self.reify_rule_list(rules, state);
                    }
                }
                CssRule::LayerBlock(rule) => self.reify_rule_list(&mut rule.rules, state),
                CssRule::Container(rule) => self.reify_rule_list(&mut rule.rules, state),
                CssRule::Scope(rule) => self.reify_rule_list(&mut rule.rules, state),
                CssRule::StartingStyle(rule) => self.reify_rule_list(&mut rule.rules, state),
                _ => {}
            }
            if ends_rule_list_segment(rule) {
                rule_list_segment = state.allocate_rule_list_segment();
            }
        }

        if !self.affected_rule_lists.remove(&rule_list) {
            return;
        }
        #[cfg(test)]
        {
            self.stats.reification_passes += 1;
        }

        let allocator = rules.bump();
        let old_rules = std::mem::replace(rules, allocator.vec());
        let mut style_group = std::vec::Vec::new();
        let mut style_group_segment = None;
        for (rule, segment) in old_rules.into_iter().zip(locations) {
            let ends_segment = ends_rule_list_segment(&rule);
            if matches!(rule, CssRule::Style(_)) {
                style_group_segment.get_or_insert(segment);
                style_group.push(rule);
                if ends_segment {
                    self.flush_style_group(
                        rule_list,
                        style_group_segment.take().unwrap(),
                        &mut style_group,
                        rules,
                    );
                }
            } else {
                if let Some(group_segment) = style_group_segment.take() {
                    self.flush_style_group(rule_list, group_segment, &mut style_group, rules);
                }
                rules.push(rule);
            }
        }
        if let Some(group_segment) = style_group_segment {
            self.flush_style_group(rule_list, group_segment, &mut style_group, rules);
        }
    }

    fn flush_style_group(
        &mut self,
        rule_list: RuleListId,
        rule_list_segment: RuleListSegmentId,
        rules: &mut std::vec::Vec<CssRule<'ast>>,
        output: &mut Vec<'ast, CssRule<'ast>>,
    ) {
        let Some(plan) = self.segments.remove(&(rule_list, rule_list_segment)) else {
            output.extend(rules.drain(..));
            return;
        };
        let mut authored = FxHashMap::default();
        for rule in rules.drain(..) {
            let CssRule::Style(style) = &rule else {
                unreachable!("a style group contains only style rules");
            };
            authored.insert(style.as_ref().get_ref().declarations, rule);
        }
        let allocator = output.bump();
        for rule in plan {
            match rule {
                ReifiedStyleRule::Authored(block) => output.push(
                    authored
                        .remove(&block)
                        .expect("a live authored block has a physical style rule"),
                ),
                ReifiedStyleRule::Synthesized {
                    declarations,
                    selectors,
                    span,
                    vendor_prefix,
                } => output.push(CssRule::Style(allocator.pinned(StyleRule::new(
                    declarations,
                    span,
                    allocator.vec(),
                    selectors,
                    vendor_prefix,
                )))),
            }
        }
    }
}

fn declaration_chain(
    active: DeclarationBlockId,
    store: &DeclarationBlockStore<'_>,
) -> std::vec::Vec<DeclarationBlockId> {
    let mut chain = std::vec::Vec::new();
    let mut seen = FxHashSet::default();
    let mut current = Some(active);
    while let Some(block) = current {
        if !seen.insert(block) {
            break;
        }
        chain.push(block);
        current = store.get(block).previous_merged();
    }
    chain
}

fn oldest_declaration_block(
    active: DeclarationBlockId,
    store: &DeclarationBlockStore<'_>,
) -> DeclarationBlockId {
    declaration_chain(active, store)
        .last()
        .copied()
        .unwrap_or(active)
}

fn append_declaration_chain(
    active: DeclarationBlockId,
    store: &DeclarationBlockStore<'_>,
    seen: &mut FxHashSet<DeclarationBlockId>,
    output: &mut std::vec::Vec<DeclarationBlockId>,
) {
    let mut chain = declaration_chain(active, store);
    chain.reverse();
    output.extend(chain.into_iter().filter(|block| seen.insert(*block)));
}

#[cfg(test)]
mod tests {
    use rocketcss_common::Allocator;
    use rocketcss_parser::{ParserOptions, parse};

    use super::*;
    use crate::MinifyOptions;

    #[test]
    fn semantic_order_supports_repeated_insertions_in_one_interval() {
        let left = SemanticOrderKey::initial(0);
        let right = SemanticOrderKey::initial(1);
        let mut previous = left.clone();
        for _ in 0..256 {
            let current = SemanticOrderKey::between(&previous, &right);
            assert!(previous < current && current < right);
            previous = current;
        }
    }

    #[test]
    fn partial_candidate_heap_pops_the_earliest_semantic_edge() {
        let candidate = |index| EdgeCandidate {
            left: DeclarationBlockId::from_index(index).unwrap(),
            right: DeclarationBlockId::from_index(index + 1).unwrap(),
            left_revision: 0,
            right_revision: 0,
        };
        let mut queue = PartialCandidateQueue::default();
        for index in [2, 0, 1] {
            assert!(queue.push(candidate(index), SemanticOrderKey::initial(index),));
        }
        assert_eq!(queue.pop().unwrap().left.index(), 0);
        assert_eq!(queue.pop().unwrap().left.index(), 1);
        assert_eq!(queue.pop().unwrap().left.index(), 2);
        assert!(queue.pop().is_none());
    }

    #[test]
    fn property_bloom_rejects_only_impossible_s3_edges() {
        let stats = scheduler_stats(".a{x:1}.b{y:1}.c{x:2!important}.d{x:3}");
        assert_eq!(stats.s3_edges_considered, 3);
        assert_eq!(stats.s3_bloom_rejections, 3);
        assert_eq!(stats.s3_candidates_enqueued, 0);
        assert_eq!(stats.s3_queue_pops, 0);

        let stats = scheduler_stats(".a{color:red}.b{color:blue}");
        assert_eq!(stats.s3_bloom_rejections, 0);
        assert_eq!(stats.s3_candidates_enqueued, 1);
        assert_eq!(stats.s3_queue_pops, 1);
        assert_eq!(stats.s3_no_common_rejections, 1);
        assert_eq!(stats.s3_commits, 0);

        let stats = scheduler_stats(".a{opacity:.5}.b{opacity:.5}");
        assert_eq!(stats.s3_bloom_rejections, 0);
        assert_eq!(stats.s3_candidates_enqueued, 1);
        assert_eq!(stats.s3_commits, 1);

        let stats = scheduler_stats(".a{display:table-cell flow}.b{display:table-cell flow}");
        assert_eq!(stats.s3_bloom_rejections, 1);
        assert_eq!(stats.s3_candidates_enqueued, 0);
        assert_eq!(stats.s3_commits, 0);
    }

    #[test]
    fn s3_reuses_only_an_exhausted_left_endpoint() {
        let stats = scheduler_stats(".a{color:red}.b{color:red}");
        assert_eq!(stats.s3_commits, 1);
        assert_eq!(stats.s3_reused_left_commits, 1);
        assert_eq!(stats.s3_allocated_shared_commits, 0);

        let stats = scheduler_stats(".a{color:red}.b{color:red;width:1px}");
        assert_eq!(stats.s3_commits, 1);
        assert_eq!(stats.s3_reused_left_commits, 1);
        assert_eq!(stats.s3_allocated_shared_commits, 0);
        assert_eq!(stats.s3_declaration_blocks_appended, 0);
        assert_eq!(stats.s3_occurrences_appended, 0);
        assert_eq!(stats.semantic_order_between_calls, 0);

        let stats = scheduler_stats(".a{color:red}.b{color:red;&:hover{x:1}}");
        assert_eq!(stats.s3_commits, 1);
        assert_eq!(stats.s3_reused_left_commits, 1);
        assert_eq!(stats.s3_allocated_shared_commits, 0);

        let stats = scheduler_stats(".a{color:red;width:1px}.b{color:red}");
        assert_eq!(stats.s3_commits, 1);
        assert_eq!(stats.s3_reused_left_commits, 0);
        assert_eq!(stats.s3_allocated_shared_commits, 1);
        assert_eq!(stats.s3_declaration_blocks_appended, 1);
        assert_eq!(stats.s3_occurrences_appended, 1);
        assert_eq!(stats.semantic_order_between_calls, 1);

        let stats = scheduler_stats(".a{color:red;width:1px}.b{color:red;height:2px}");
        assert_eq!(stats.s3_commits, 1);
        assert_eq!(stats.s3_reused_left_commits, 0);
        assert_eq!(stats.s3_allocated_shared_commits, 1);
        assert_eq!(stats.s3_declaration_blocks_appended, 1);
        assert_eq!(stats.s3_occurrences_appended, 1);
        assert_eq!(stats.semantic_order_between_calls, 1);
    }

    #[test]
    fn s3_reuses_an_exhausted_left_endpoint_composed_by_s1() {
        let stats = scheduler_stats(".a{color:red}.a{width:1px}.b{color:red;width:1px}");
        assert_eq!(stats.s3_commits, 1);
        assert_eq!(stats.s3_reused_left_commits, 1);
        assert_eq!(stats.s3_allocated_shared_commits, 0);
        assert_eq!(stats.s3_declaration_blocks_appended, 0);
        assert_eq!(stats.s3_occurrences_appended, 0);
        assert_eq!(stats.semantic_order_between_calls, 0);
    }

    fn scheduler_stats(source: &str) -> SchedulerStats {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut compilation =
                parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let (stylesheet, declaration_blocks) = compilation.parts_mut();
            let mut state = CrossRuleMergeState::discover(stylesheet, declaration_blocks);
            state.run(declaration_blocks, &mut minifier, &mut cx);
            state.stats
        })
    }

    #[test]
    fn overlapping_chain_uses_one_collection_and_linear_local_work() {
        const RULE_COUNT: usize = 64;
        let source = (0..RULE_COUNT)
            .map(|index| format!(".r{index}{{opacity:.5}}"))
            .collect::<String>();
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut compilation =
                parse(&source, &allocator, &mut token, ParserOptions::default()).unwrap();
            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let (stylesheet, declaration_blocks) = compilation.parts_mut();
            let mut state = CrossRuleMergeState::discover(stylesheet, declaration_blocks);
            let declaration_block_count = declaration_blocks.len();
            let occurrence_count = state.declaration_ir.occurrence_count();
            let property_index_count = state.declaration_ir.property_index_count();
            state.run(declaration_blocks, &mut minifier, &mut cx);

            assert_eq!(state.stats.initial_collections, 1);
            assert_eq!(state.stats.s3_commits, RULE_COUNT - 1);
            assert_eq!(state.stats.s3_reused_left_commits, RULE_COUNT - 1);
            assert_eq!(state.stats.s3_allocated_shared_commits, 0);
            assert_eq!(state.stats.s3_declaration_blocks_appended, 0);
            assert_eq!(state.stats.s3_occurrences_appended, 0);
            assert_eq!(state.stats.semantic_order_between_calls, 0);
            assert_eq!(declaration_blocks.len(), declaration_block_count);
            assert_eq!(state.declaration_ir.occurrence_count(), occurrence_count);
            assert_eq!(
                state.declaration_ir.property_index_count(),
                property_index_count
            );
            let first = DeclarationBlockId::from_index(0).unwrap();
            assert!(
                state
                    .node(first)
                    .is_some_and(|node| node.flags.contains(LiveBlockFlags::LIVE))
            );
            assert_eq!(state.owner_by_block[first.index()], Some(first));
            assert_eq!(
                state
                    .nodes
                    .iter()
                    .flatten()
                    .filter(|node| node.flags.contains(LiveBlockFlags::LIVE))
                    .count(),
                1
            );
            assert_eq!(state.stats.history_insertions, RULE_COUNT - 1);
            assert!(state.stats.stale_candidates > 0);
            assert!(state.stats.s2_queue_pops <= RULE_COUNT * 4);
            assert!(
                state.stats.s3_queue_pops <= RULE_COUNT * 6,
                "scheduler stats: {:?}",
                state.stats
            );
            assert!(
                state.stats.edge_classifications <= RULE_COUNT * 8,
                "scheduler stats: {:?}",
                state.stats
            );
            assert_eq!(cx.stats().declarations_removed as usize, RULE_COUNT - 1);

            let mut plan = state.into_reification_plan();
            let mut walk_state = WalkState::default();
            plan.reify_rule_list(&mut stylesheet.rules, &mut walk_state);
            assert_eq!(plan.stats.reification_passes, 1);
            assert!(plan.segments.is_empty());
        });
    }

    #[test]
    fn reification_skips_unchanged_lists_and_rebuilds_only_affected_lists() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut compilation = parse(
                ".root{display:block}@media print{.a{opacity:.5}.b{opacity:.5}}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let (stylesheet, declaration_blocks) = compilation.parts_mut();
            let mut state = CrossRuleMergeState::discover(stylesheet, declaration_blocks);
            state.run(declaration_blocks, &mut minifier, &mut cx);
            let mut plan = state.into_reification_plan();

            let mut walk_state = WalkState::default();
            plan.reify_rule_list(&mut stylesheet.rules, &mut walk_state);
            assert_eq!(plan.stats.reification_passes, 1);
            assert!(plan.segments.is_empty());
            assert!(plan.affected_rule_lists.is_empty());
        });

        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut compilation = parse(
                ".a{color:red}.b{display:block}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let (stylesheet, declaration_blocks) = compilation.parts_mut();
            let mut state = CrossRuleMergeState::discover(stylesheet, declaration_blocks);
            state.run(declaration_blocks, &mut minifier, &mut cx);
            let plan = state.into_reification_plan();
            assert!(plan.affected_rule_lists.is_empty());
            assert!(plan.segments.is_empty());
        });
    }
}
