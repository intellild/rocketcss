use std::collections::{HashMap, HashSet};

use rocketcss_allocator::{Ref, vec::Vec};
use rocketcss_ast::{CssRule, DeclarationBlock, StyleRule};

use super::{
    DeclarationBlockMinifier, equal_live_selectors, has_live_selector,
    partial::{FactorCandidate, FactorCandidateId},
    state::{
        CascadeScope, DeclarationEntryId, DeclarationEntryState, EdgeId, EdgeState, EdgeStatus,
        HistoryId, RuleId, RuleState, SegmentId, SequenceId, SequenceState, WorkQueues,
    },
};
use crate::MinifyContext;

const ORDER_STRIDE: u64 = 1 << 32;

pub(super) fn stabilize_rule_list<'ast, 'scratch>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
    cascade_scope: CascadeScope,
) -> bool
where
    'ast: 'scratch,
{
    Stabilizer::new(rules, minifier, cx, cascade_scope).run()
}

pub(super) struct Stabilizer<'list, 'scratch, 'ast> {
    pub(super) rules: &'list mut Vec<'ast, CssRule<'ast>>,
    pub(super) minifier: &'list mut DeclarationBlockMinifier<'scratch, 'ast>,
    pub(super) cx: &'list mut MinifyContext<'scratch>,
    pub(super) cascade_scope: CascadeScope,
    pub(super) storage: std::vec::Vec<Option<CssRule<'ast>>>,
    pub(super) slot_order_labels: std::vec::Vec<u64>,
    pub(super) slot_to_rule: std::vec::Vec<Option<RuleId>>,
    pub(super) rule_states: std::vec::Vec<RuleState>,
    pub(super) sequences: std::vec::Vec<SequenceState<'ast>>,
    pub(super) declaration_entries: std::vec::Vec<DeclarationEntryState<'ast>>,
    pub(super) histories: std::vec::Vec<super::state::HistoryState>,
    pub(super) history_buckets: HashMap<u64, std::vec::Vec<HistoryId>>,
    pub(super) edges: std::vec::Vec<EdgeState>,
    pub(super) queues: WorkQueues,
    pub(super) candidates: std::vec::Vec<Option<FactorCandidate>>,
    pub(super) candidate_order:
        std::collections::BinaryHeap<std::cmp::Reverse<(u64, FactorCandidateId)>>,
    pub(super) changed: bool,
}

impl<'list, 'scratch, 'ast> Stabilizer<'list, 'scratch, 'ast>
where
    'ast: 'scratch,
{
    fn new(
        rules: &'list mut Vec<'ast, CssRule<'ast>>,
        minifier: &'list mut DeclarationBlockMinifier<'scratch, 'ast>,
        cx: &'list mut MinifyContext<'scratch>,
        cascade_scope: CascadeScope,
    ) -> Self {
        let allocator = rules.bump();
        let old_rules = std::mem::replace(rules, allocator.vec());
        let storage = old_rules
            .into_iter()
            .map(Some)
            .collect::<std::vec::Vec<_>>();
        let slot_order_labels = (0..storage.len())
            .map(|index| (index as u64 + 1) * ORDER_STRIDE)
            .collect();
        let slot_to_rule = std::vec![None; storage.len()];
        let mut stabilizer = Self {
            rules,
            minifier,
            cx,
            cascade_scope,
            storage,
            slot_order_labels,
            slot_to_rule,
            rule_states: std::vec::Vec::new(),
            sequences: std::vec::Vec::new(),
            declaration_entries: std::vec::Vec::new(),
            histories: std::vec::Vec::new(),
            history_buckets: HashMap::new(),
            edges: std::vec::Vec::new(),
            queues: WorkQueues::default(),
            candidates: std::vec::Vec::new(),
            candidate_order: std::collections::BinaryHeap::new(),
            changed: false,
        };
        stabilizer.initialize_graph();
        stabilizer
    }

    fn initialize_graph(&mut self) {
        let mut segment = SegmentId(0);
        let mut previous_live = None;
        let mut seen_blocks = HashSet::new();

        for slot in 0..self.storage.len() {
            let is_segment_member = matches!(
                self.storage[slot].as_ref(),
                Some(CssRule::Style(rule)) if has_live_selector(rule) || rule.rules.is_empty()
            );
            let is_live_style = matches!(
                self.storage[slot].as_ref(),
                Some(CssRule::Style(rule)) if has_live_selector(rule)
            );
            if !is_segment_member {
                segment = SegmentId(segment.0 + 1);
                previous_live = None;
                continue;
            }
            if !is_live_style {
                continue;
            }

            let rule_id = RuleId(self.rule_states.len() as u32);
            let blocks = {
                let CssRule::Style(rule) = self.storage[slot].as_ref().expect("stored rule") else {
                    unreachable!()
                };
                declaration_chain(&rule.declarations)
            };
            let sequence_id = SequenceId(self.sequences.len() as u32);
            let mut entry_ids = std::vec::Vec::with_capacity(blocks.len());
            for block in blocks {
                debug_assert!(
                    seen_blocks.insert(block),
                    "a declaration block must belong to one live sequence"
                );
                let entry = DeclarationEntryId(self.declaration_entries.len() as u32);
                self.declaration_entries.push(DeclarationEntryState {
                    block,
                    sequence: sequence_id,
                });
                entry_ids.push(entry);
            }
            self.sequences.push(SequenceState {
                blocks: entry_ids,
                revision: 0,
                fingerprint: None,
                owner: rule_id,
            });

            let retained_child_count = match self.storage[slot].as_ref().expect("stored rule") {
                CssRule::Style(rule) => rule.rules.len() as u32,
                _ => unreachable!(),
            };
            self.rule_states.push(RuleState {
                ast_slot: slot,
                live: true,
                previous_live,
                next_live: None,
                previous_edge: None,
                next_edge: None,
                segment,
                sequence: sequence_id,
                history: HistoryId(u32::MAX),
                retained_child_count,
                order_label: self.slot_order_labels[slot],
            });
            self.slot_to_rule[slot] = Some(rule_id);
            if let Some(previous) = previous_live {
                self.rule_states[previous.index()].next_live = Some(rule_id);
            }
            previous_live = Some(rule_id);
        }

        for raw in 0..self.rule_states.len() {
            let rule = RuleId(raw as u32);
            let history = self.register_rule_history(rule);
            self.rule_states[raw].history = history;
        }
        for raw in 0..self.histories.len() {
            self.queue_history(HistoryId(raw as u32));
        }

        for raw in 0..self.rule_states.len() {
            let left = RuleId(raw as u32);
            if let Some(right) = self.rule_states[raw].next_live {
                self.connect_edge(left, right, false);
            }
        }
        self.debug_assert_invariants();
    }

    fn run(mut self) -> bool {
        loop {
            if let Some(edge) = self.queues.same_selector.pop_front() {
                self.process_same_selector_edge(edge);
                continue;
            }
            if let Some(history) = self.queues.histories.pop_front() {
                self.process_history(history);
                continue;
            }
            if let Some(edge) = self.queues.partial.pop_front() {
                self.process_partial_edge(edge);
                continue;
            }
            if self.commit_leftmost_candidate() {
                continue;
            }
            break;
        }

        self.debug_assert_invariants();
        let changed = self.changed;
        self.rebuild_rule_list();
        changed
    }

    pub(super) fn rule(&self, rule: RuleId) -> &StyleRule<'ast> {
        let slot = self.rule_states[rule.index()].ast_slot;
        let CssRule::Style(rule) = self.storage[slot].as_ref().expect("live rule has storage")
        else {
            unreachable!("rule state points at a style rule")
        };
        rule
    }

    pub(super) fn rule_mut(&mut self, rule: RuleId) -> &mut StyleRule<'ast> {
        let slot = self.rule_states[rule.index()].ast_slot;
        let CssRule::Style(rule) = self.storage[slot].as_mut().expect("live rule has storage")
        else {
            unreachable!("rule state points at a style rule")
        };
        rule
    }

    pub(super) fn entry_block(
        &self,
        entry: DeclarationEntryId,
    ) -> Ref<'ast, DeclarationBlock<'ast>> {
        self.declaration_entries[entry.index()].block
    }

    fn process_same_selector_edge(&mut self, edge: EdgeId) {
        if self.edges[edge.index()].status != EdgeStatus::DirtySameSelector {
            return;
        }
        let (left, right) = {
            let edge = &self.edges[edge.index()];
            (edge.left, edge.right)
        };
        if !self.edge_is_current(edge) {
            self.edges[edge.index()].status = EdgeStatus::Stale;
            return;
        }
        if !self.can_merge_same_selector(left, right) {
            self.classify_existing_edge(edge);
            return;
        }

        let left_tail = Ref::from_pinned_box(&self.rule(left).declarations);
        self.rule_mut(right)
            .declarations
            .as_mut()
            .set_previous_merged(Some(left_tail));

        let left_sequence = self.rule_states[left.index()].sequence;
        let right_sequence = self.rule_states[right.index()].sequence;
        let mut combined = std::mem::take(&mut self.sequences[left_sequence.index()].blocks);
        combined.extend_from_slice(&self.sequences[right_sequence.index()].blocks);
        self.sequences[right_sequence.index()].blocks = combined;
        self.sequences[right_sequence.index()].revision = self.sequences[right_sequence.index()]
            .revision
            .checked_add(1)
            .expect("sequence revision overflow");
        self.sequences[right_sequence.index()].fingerprint = None;
        self.sequences[right_sequence.index()].owner = right;
        for &entry in &self.sequences[right_sequence.index()].blocks {
            self.declaration_entries[entry.index()].sequence = right_sequence;
        }
        self.rule_states[right.index()].sequence = right_sequence;

        debug_assert_eq!(
            self.rule_states[left.index()].history,
            self.rule_states[right.index()].history
        );
        self.rule_states[left.index()].live = false;
        self.atomic_reconnect(&[left, right], &[right]);
        self.changed = true;
    }

    fn can_merge_same_selector(&self, left: RuleId, right: RuleId) -> bool {
        let left_rule = self.rule(left);
        let right_rule = self.rule(right);
        left_rule.rules.is_empty()
            && left_rule.vendor_prefix == right_rule.vendor_prefix
            && right_rule.declarations.previous_merged().is_none()
            && equal_live_selectors(&left_rule.selectors, &right_rule.selectors)
    }

    pub(super) fn edge_is_current(&self, edge: EdgeId) -> bool {
        let edge_state = &self.edges[edge.index()];
        let left = &self.rule_states[edge_state.left.index()];
        let right = &self.rule_states[edge_state.right.index()];
        left.live
            && right.live
            && left.next_live == Some(edge_state.right)
            && right.previous_live == Some(edge_state.left)
            && left.next_edge == Some(edge)
            && right.previous_edge == Some(edge)
            && left.segment == right.segment
    }

    pub(super) fn mark_sequence_changed(&mut self, sequence: SequenceId) {
        let state = &mut self.sequences[sequence.index()];
        state.revision = state
            .revision
            .checked_add(1)
            .expect("sequence revision overflow");
        state.fingerprint = None;
        let owner = state.owner;
        if !self.rule_states[owner.index()].live {
            return;
        }
        let previous = self.rule_states[owner.index()].previous_edge;
        let next = self.rule_states[owner.index()].next_edge;
        if let Some(edge) = previous {
            self.mark_edge_dirty(edge);
        }
        if let Some(edge) = next {
            self.mark_edge_dirty(edge);
        }
    }

    pub(super) fn mark_edge_dirty(&mut self, edge: EdgeId) {
        if !self.edge_is_current(edge) {
            self.edges[edge.index()].status = EdgeStatus::Stale;
            return;
        }
        self.classify_existing_edge(edge);
    }

    fn classify_existing_edge(&mut self, edge: EdgeId) {
        let (left, right) = {
            let state = &self.edges[edge.index()];
            (state.left, state.right)
        };
        if self.can_merge_same_selector(left, right) {
            if self.edges[edge.index()].status != EdgeStatus::DirtySameSelector {
                self.edges[edge.index()].status = EdgeStatus::DirtySameSelector;
                self.queues.same_selector.push_back(edge);
            }
        } else if equal_live_selectors(&self.rule(left).selectors, &self.rule(right).selectors) {
            self.edges[edge.index()].status = EdgeStatus::Stable;
        } else if self.edges[edge.index()].status != EdgeStatus::DirtyPartial {
            self.edges[edge.index()].status = EdgeStatus::DirtyPartial;
            self.queues.partial.push_back(edge);
        }
    }

    pub(super) fn connect_edge(
        &mut self,
        left: RuleId,
        right: RuleId,
        prioritize_same_selector: bool,
    ) -> EdgeId {
        debug_assert!(self.rule_states[left.index()].live);
        debug_assert!(self.rule_states[right.index()].live);
        debug_assert_eq!(
            self.rule_states[left.index()].segment,
            self.rule_states[right.index()].segment
        );
        let edge = EdgeId(self.edges.len() as u32);
        self.edges.push(EdgeState {
            left,
            right,
            status: EdgeStatus::Stable,
        });
        self.rule_states[left.index()].next_live = Some(right);
        self.rule_states[left.index()].next_edge = Some(edge);
        self.rule_states[right.index()].previous_live = Some(left);
        self.rule_states[right.index()].previous_edge = Some(edge);
        if prioritize_same_selector && self.can_merge_same_selector(left, right) {
            self.edges[edge.index()].status = EdgeStatus::DirtySameSelector;
            self.queues.same_selector.push_front(edge);
        } else {
            self.classify_existing_edge(edge);
        }
        edge
    }

    fn stale_edge(&mut self, edge: EdgeId) {
        self.edges[edge.index()].status = EdgeStatus::Stale;
    }

    pub(super) fn atomic_reconnect(&mut self, members: &[RuleId], replacement: &[RuleId]) {
        let left = *members.first().expect("a transition has a left endpoint");
        let right = *members.last().expect("a transition has a right endpoint");
        let previous = self.rule_states[left.index()].previous_live;
        let next = self.rule_states[right.index()].next_live;
        let mut incident = std::vec::Vec::with_capacity(members.len() + 1);
        for &member in members {
            if let Some(edge) = self.rule_states[member.index()].previous_edge
                && !incident.contains(&edge)
            {
                incident.push(edge);
            }
            if let Some(edge) = self.rule_states[member.index()].next_edge
                && !incident.contains(&edge)
            {
                incident.push(edge);
            }
        }
        for edge in incident {
            self.stale_edge(edge);
        }

        if let Some(previous) = previous {
            self.rule_states[previous.index()].next_live = None;
            self.rule_states[previous.index()].next_edge = None;
        }
        if let Some(next) = next {
            self.rule_states[next.index()].previous_live = None;
            self.rule_states[next.index()].previous_edge = None;
        }
        for &member in members {
            self.rule_states[member.index()].previous_live = None;
            self.rule_states[member.index()].next_live = None;
            self.rule_states[member.index()].previous_edge = None;
            self.rule_states[member.index()].next_edge = None;
        }

        let mut previous_node = previous;
        for &node in replacement {
            self.rule_states[node.index()].previous_live = previous_node;
            self.rule_states[node.index()].next_live = None;
            self.rule_states[node.index()].previous_edge = None;
            self.rule_states[node.index()].next_edge = None;
            if let Some(previous_node) = previous_node {
                self.connect_edge(previous_node, node, true);
            }
            previous_node = Some(node);
        }
        if let Some(next) = next {
            if let Some(previous_node) = previous_node {
                self.connect_edge(previous_node, next, true);
            } else {
                self.rule_states[next.index()].previous_live = None;
            }
        }
    }

    pub(super) fn allocate_order_between(&mut self, left: RuleId, right: RuleId) -> u64 {
        let mut left_label = self.rule_states[left.index()].order_label;
        let mut right_label = self.rule_states[right.index()].order_label;
        if right_label - left_label <= 1 {
            self.renumber_order_labels();
            left_label = self.rule_states[left.index()].order_label;
            right_label = self.rule_states[right.index()].order_label;
        }
        left_label + (right_label - left_label) / 2
    }

    fn renumber_order_labels(&mut self) {
        let mut slots = (0..self.storage.len()).collect::<std::vec::Vec<_>>();
        slots.sort_unstable_by_key(|&slot| self.slot_order_labels[slot]);
        for (index, slot) in slots.into_iter().enumerate() {
            let label = (index as u64 + 1) * ORDER_STRIDE;
            self.slot_order_labels[slot] = label;
            if let Some(rule) = self.slot_to_rule[slot] {
                self.rule_states[rule.index()].order_label = label;
            }
        }
    }

    pub(super) fn debug_assert_invariants(&self) {
        #[cfg(debug_assertions)]
        {
            for (raw, rule) in self.rule_states.iter().enumerate() {
                let id = RuleId(raw as u32);
                if !rule.live {
                    debug_assert!(rule.previous_edge.is_none());
                    debug_assert!(rule.next_edge.is_none());
                    continue;
                }
                if let Some(previous) = rule.previous_live {
                    debug_assert_eq!(self.rule_states[previous.index()].next_live, Some(id));
                    debug_assert_eq!(self.rule_states[previous.index()].segment, rule.segment);
                }
                if let Some(next) = rule.next_live {
                    debug_assert_eq!(self.rule_states[next.index()].previous_live, Some(id));
                    debug_assert_eq!(self.rule_states[next.index()].segment, rule.segment);
                }
            }
            for (raw, edge) in self.edges.iter().enumerate() {
                if edge.status == EdgeStatus::Stale {
                    continue;
                }
                debug_assert!(self.edge_is_current(EdgeId(raw as u32)));
            }
            for history in &self.histories {
                if !history.queued {
                    debug_assert_eq!(history.generation, history.consumed_generation);
                }
            }
        }
    }
}

fn declaration_chain<'ast>(
    tail: &std::pin::Pin<rocketcss_allocator::boxed::Box<'ast, DeclarationBlock<'ast>>>,
) -> std::vec::Vec<Ref<'ast, DeclarationBlock<'ast>>> {
    let mut blocks = std::vec::Vec::new();
    let mut current = Some(Ref::from_pinned_box(tail));
    while let Some(block) = current {
        blocks.push(block);
        current = block.get().get_ref().previous_merged();
    }
    blocks.reverse();
    blocks
}
