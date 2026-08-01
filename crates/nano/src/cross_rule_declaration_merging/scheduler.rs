use std::collections::{BTreeMap, VecDeque};

use rocketcss_ast::{
    CssRule, DeclarationBlockId, DeclarationBlockStore, SelectorList, Span, StyleRule, StyleSheet,
    VendorPrefix,
};
use rocketcss_common::{DenseId, vec::Vec};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use super::partial_selector::{
    PartialRuleRef, commit_partial_merge_declarations, discover_partial_merge_plan,
    materialize_selector_union,
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
    right_order: SemanticOrderKey,
    candidate: EdgeCandidate,
}

#[derive(Default)]
struct PartialCandidateQueue {
    pending: BTreeMap<PartialCandidateKey, EdgeCandidate>,
    queued: FxHashSet<EdgeCandidate>,
}

impl PartialCandidateQueue {
    fn push(
        &mut self,
        candidate: EdgeCandidate,
        left_order: SemanticOrderKey,
        right_order: SemanticOrderKey,
    ) {
        if self.queued.insert(candidate) {
            self.pending.insert(
                PartialCandidateKey {
                    left_order,
                    right_order,
                    candidate,
                },
                candidate,
            );
        }
    }

    fn pop(&mut self) -> Option<EdgeCandidate> {
        let (_, candidate) = self.pending.pop_first()?;
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
    s3_commits: usize,
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
        Self::new(
            crate::utils::discover_declaration_blocks(stylesheet),
            declaration_blocks,
        )
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

        let removed_before = cx.stats().declarations_removed;
        minifier.deduplicate_exact_sequence(&expanded, declaration_blocks, cx, |_| {});
        if cx.stats().declarations_removed == removed_before {
            return;
        }

        let mut affected_owners = FxHashSet::default();
        for block in expanded {
            if let Some(owner) = self.owner_by_block[block.index()] {
                affected_owners.insert(owner);
            }
        }
        for owner in affected_owners {
            if self.node(owner).is_some_and(|node| {
                node.flags.contains(LiveBlockFlags::LIVE)
                    && !node.flags.contains(LiveBlockFlags::HAS_CHILDREN)
                    && declaration_chain_is_empty(owner, declaration_blocks)
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
            left_order,
            right_order,
            rule_list,
            rule_list_segment,
        ) = {
            let left_node = self.node(left).unwrap();
            let right_node = self.node(right).unwrap();
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
                left_node.order.clone(),
                right_node.order.clone(),
                left_node.rule_list,
                left_node.rule_list_segment,
            )
        };
        let Some(plan) = discover_partial_merge_plan(
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
            cx,
        ) else {
            return;
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

        let retain_left = plan.retain_left;
        let retain_right = plan.retain_right;
        let shared_order = SemanticOrderKey::between(&left_order, &right_order);
        let shared_span = plan.span;
        let shared_vendor_prefix = plan.vendor_prefix;
        let shared = commit_partial_merge_declarations(&plan, declaration_blocks, cx);
        self.affected_rule_lists.insert(rule_list);
        let shared_selectors = plan.selectors;
        debug_assert_eq!(shared.index(), self.nodes.len());
        self.nodes.push(Some(LiveBlock {
            effective_key: shared_key,
            previous_live: None,
            next_live: None,
            rule_list,
            rule_list_segment,
            order: shared_order.clone(),
            selectors: SelectorSource::Synthesized(shared_selectors),
            span: shared_span,
            vendor_prefix: shared_vendor_prefix,
            flags: LiveBlockFlags::LIVE | LiveBlockFlags::HAS_LIVE_SELECTORS,
            revision: 0,
        }));
        self.owner_by_block.push(Some(shared));
        self.occurrence_order.push(Some(shared_order.clone()));

        let previous = self.node(left).unwrap().previous_live;
        let next = self.node(right).unwrap().next_live;
        if !retain_left {
            self.retire_node_without_relink(left);
            self.remove_history_occurrence(left_key, left);
        }
        if !retain_right {
            self.retire_node_without_relink(right);
            self.remove_history_occurrence(right_key, right);
        }

        let first = if retain_left { left } else { shared };
        let last = if retain_right { right } else { shared };
        self.set_previous(first, previous);
        self.set_next(last, next);
        if let Some(previous) = previous {
            self.set_next(previous, Some(first));
        }
        if let Some(next) = next {
            self.set_previous(next, Some(last));
        }
        if retain_left {
            self.set_next(left, Some(shared));
            self.set_previous(shared, Some(left));
        } else {
            self.set_previous(shared, previous);
        }
        if retain_right {
            self.set_next(shared, Some(right));
            self.set_previous(right, Some(shared));
        } else {
            self.set_next(shared, next);
        }

        self.histories
            .entry(shared_key)
            .or_default()
            .insert(shared_order, shared);
        #[cfg(test)]
        {
            self.stats.s3_commits += 1;
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
        let right_order = right_state.order.clone();
        #[cfg(test)]
        {
            self.stats.edge_classifications += 1;
        }
        if same_selector {
            self.same_selector_candidates.push(candidate);
        } else {
            self.partial_merge_candidates
                .push(candidate, left_order, right_order);
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

fn declaration_chain_is_empty(
    active: DeclarationBlockId,
    store: &DeclarationBlockStore<'_>,
) -> bool {
    declaration_chain(active, store)
        .into_iter()
        .all(|block| store.get(block).is_output_empty())
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
    fn overlapping_chain_uses_one_collection_and_linear_local_work() {
        const RULE_COUNT: usize = 64;
        let source = (0..RULE_COUNT)
            .map(|index| format!(".r{index}{{x:1}}"))
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
            state.run(declaration_blocks, &mut minifier, &mut cx);

            assert_eq!(state.stats.initial_collections, 1);
            assert_eq!(state.stats.s3_commits, RULE_COUNT - 1);
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
                ".root{z:1}@media print{.a{x:1}.b{x:1}}",
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
                ".a{x:1}.b{y:1}",
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
