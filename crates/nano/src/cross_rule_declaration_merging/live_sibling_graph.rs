use rocketcss_ast::{CssRule, DeclarationBlockId, DeclarationBlockStore, Selector, StyleSheet};
use rocketcss_common::{DenseMap, DenseStore, define_dense_id};
use rustc_hash::FxHashMap;

use super::RuleId;
use super::candidates::{Candidate, SameSelectorCandidateList};
use crate::utils::{
    DeclarationBlockEntries, DeclarationBlockEntry, DeclarationBlockEntryId, DeclarationBlockKind,
    EffectiveKeyId, RuleListId, RuleListSegmentId,
};

define_dense_id!(struct SequenceId);

const _: () = {
    assert!(std::mem::size_of::<Option<RuleId>>() == std::mem::size_of::<u32>());
    assert!(std::mem::size_of::<Option<SequenceId>>() == std::mem::size_of::<u32>());
};

#[derive(Debug)]
struct LiveRule {
    declarations: DeclarationBlockId,
    effective_key: EffectiveKeyId,
    previous_live: Option<RuleId>,
    next_live: Option<RuleId>,
    sequence: SequenceId,
    has_children: bool,
    has_live_selectors: bool,
    live: bool,
}

#[derive(Debug)]
struct DeclarationSequence {
    parent: SequenceId,
    head: DeclarationBlockId,
    tail: DeclarationBlockId,
    non_empty_block_count: u32,
    active_owner: RuleId,
}

#[derive(Debug)]
pub(super) struct LiveSiblingGraph {
    rules: DenseStore<RuleId, LiveRule>,
    sequences: DenseStore<SequenceId, DeclarationSequence>,
    sequence_by_block: DenseMap<DeclarationBlockId, Option<SequenceId>>,
    non_empty_blocks: DenseMap<DeclarationBlockId, bool>,
    same_selector_candidates: SameSelectorCandidateList,
    declaration_links: Vec<(DeclarationBlockId, DeclarationBlockId)>,
}

impl LiveSiblingGraph {
    pub(super) fn new(
        declaration_blocks: &DeclarationBlockEntries,
        store: &DeclarationBlockStore<'_>,
    ) -> Self {
        let style_rule_count = declaration_blocks
            .iter()
            .filter(|entry| matches!(entry.kind, DeclarationBlockKind::Style { .. }))
            .count();
        let mut graph = Self {
            rules: DenseStore::with_capacity(style_rule_count),
            sequences: DenseStore::with_capacity(style_rule_count),
            sequence_by_block: DenseMap::from_store(store, |_| None),
            non_empty_blocks: DenseMap::from_store(store, |_| false),
            same_selector_candidates: SameSelectorCandidateList::default(),
            declaration_links: Vec::with_capacity(style_rule_count.saturating_sub(1)),
        };
        let mut last_style_by_segment: FxHashMap<
            (RuleListId, RuleListSegmentId),
            (RuleId, DeclarationBlockEntryId),
        > = FxHashMap::default();
        let mut declaration_chain = Vec::new();
        let mut declaration_chain_seen = DenseMap::from_store(store, |_| false);

        for (entry_id, entry) in declaration_blocks.iter_enumerated() {
            let DeclarationBlockKind::Style { .. } = entry.kind else {
                continue;
            };
            let rule = graph.push_rule(
                entry,
                store,
                &mut declaration_chain,
                &mut declaration_chain_seen,
            );
            let segment = (entry.rule_list, entry.rule_list_segment);
            if let Some(&(previous, previous_entry)) = last_style_by_segment.get(&segment)
                && declaration_blocks[previous_entry].is_direct_sibling_of(entry)
            {
                graph.connect(previous, rule);
                graph.enqueue_same_selector_edge(previous, rule);
            }
            last_style_by_segment.insert(segment, (rule, entry_id));
        }
        graph
    }

    fn push_rule(
        &mut self,
        entry: &DeclarationBlockEntry,
        store: &DeclarationBlockStore<'_>,
        declaration_chain: &mut Vec<DeclarationBlockId>,
        declaration_chain_seen: &mut DenseMap<DeclarationBlockId, bool>,
    ) -> RuleId {
        let DeclarationBlockKind::Style {
            has_children,
            has_live_selectors,
        } = entry.kind
        else {
            unreachable!("only style rules are inserted into the live sibling graph");
        };
        let rule = self.rules.next_id();
        let sequence = self.sequences.next_id();
        collect_declaration_chain(
            entry.declarations,
            store,
            declaration_chain,
            declaration_chain_seen,
        );
        declaration_chain.reverse();
        let head = *declaration_chain
            .first()
            .expect("a declaration chain contains its active block");
        let tail = *declaration_chain
            .last()
            .expect("a declaration chain contains its active block");
        let mut non_empty_block_count = 0_u32;
        for &declarations in declaration_chain.iter() {
            let old_sequence = self.sequence_by_block[declarations].replace(sequence);
            debug_assert!(
                old_sequence.is_none(),
                "a declaration block belongs to only one sequence"
            );
            if !store.get(declarations).is_output_empty() {
                self.non_empty_blocks[declarations] = true;
                non_empty_block_count = non_empty_block_count
                    .checked_add(1)
                    .expect("declaration block count exceeds u32::MAX");
            }
        }
        let inserted_sequence = self.sequences.push(DeclarationSequence {
            parent: sequence,
            head,
            tail,
            non_empty_block_count,
            active_owner: rule,
        });
        debug_assert_eq!(inserted_sequence, sequence);
        let inserted_rule = self.rules.push(LiveRule {
            declarations: entry.declarations,
            effective_key: entry.effective_key,
            previous_live: None,
            next_live: None,
            sequence,
            has_children,
            has_live_selectors,
            live: true,
        });
        debug_assert_eq!(inserted_rule, rule);
        rule
    }

    fn connect(&mut self, left: RuleId, right: RuleId) {
        debug_assert!(self.rules[left].next_live.is_none());
        debug_assert!(self.rules[right].previous_live.is_none());
        self.rules[left].next_live = Some(right);
        self.rules[right].previous_live = Some(left);
    }

    pub(super) fn declaration_block_became_empty(&mut self, declarations: DeclarationBlockId) {
        if !std::mem::take(&mut self.non_empty_blocks[declarations]) {
            return;
        }
        let Some(sequence) = self.sequence_by_block[declarations] else {
            return;
        };
        let sequence = self.find_sequence(sequence);
        let state = &mut self.sequences[sequence];
        state.non_empty_block_count = state
            .non_empty_block_count
            .checked_sub(1)
            .expect("an empty declaration block was counted as non-empty");
        if state.non_empty_block_count != 0 {
            return;
        }
        let owner = state.active_owner;
        if self.rules[owner].live && !self.rules[owner].has_children {
            self.unlink_rule(owner);
        }
    }

    pub(super) fn commit(
        &self,
        stylesheet: &mut StyleSheet<'_>,
        store: &mut DeclarationBlockStore<'_>,
    ) -> bool {
        for &(current, previous) in &self.declaration_links {
            store.get_mut(current).set_previous_merged(Some(previous));
        }

        let mut retired = DenseMap::from_store(store, |_| false);
        let mut has_retired = false;
        for rule in self.rules.iter().filter(|rule| !rule.live) {
            retired[rule.declarations] = true;
            has_retired = true;
        }
        if !has_retired {
            return false;
        }
        retire_style_rules(&mut stylesheet.rules, &retired);
        true
    }

    pub(super) fn candidate_is_live_edge(
        &self,
        Candidate(left, right): Candidate,
    ) -> Option<(RuleId, RuleId)> {
        let left_state = self.rules.try_get(left)?;
        let right_state = self.rules.try_get(right)?;
        (left_state.live
            && right_state.live
            && left_state.next_live == Some(right)
            && right_state.previous_live == Some(left)
            && !left_state.has_children
            && left_state.has_live_selectors
            && left_state.effective_key == right_state.effective_key)
            .then_some((left, right))
    }

    pub(super) fn pop_same_selector_candidate(&mut self) -> Option<Candidate> {
        self.same_selector_candidates.pop()
    }

    pub(super) fn concatenate_and_retire_left(&mut self, left: RuleId, right: RuleId) {
        let left_sequence = self.find_sequence(self.rules[left].sequence);
        let right_sequence = self.find_sequence(self.rules[right].sequence);
        debug_assert_ne!(left_sequence, right_sequence);

        let left_head = self.sequences[left_sequence].head;
        let left_tail = self.sequences[left_sequence].tail;
        let left_non_empty = self.sequences[left_sequence].non_empty_block_count;
        let right_head = self.sequences[right_sequence].head;
        let right_non_empty = self.sequences[right_sequence].non_empty_block_count;
        self.declaration_links.push((right_head, left_tail));

        self.sequences[left_sequence].parent = right_sequence;
        let right_state = &mut self.sequences[right_sequence];
        right_state.head = left_head;
        right_state.non_empty_block_count = left_non_empty
            .checked_add(right_non_empty)
            .expect("declaration block count exceeds u32::MAX");
        right_state.active_owner = right;
        self.rules[right].sequence = right_sequence;
        self.unlink_rule(left);
    }

    fn find_sequence(&mut self, sequence: SequenceId) -> SequenceId {
        let mut root = sequence;
        while self.sequences[root].parent != root {
            root = self.sequences[root].parent;
        }
        let mut current = sequence;
        while current != root {
            let parent = self.sequences[current].parent;
            self.sequences[current].parent = root;
            current = parent;
        }
        root
    }

    fn unlink_rule(&mut self, rule: RuleId) {
        let previous = self.rules[rule].previous_live;
        let next = self.rules[rule].next_live;
        if let Some(previous) = previous {
            self.rules[previous].next_live = next;
        }
        if let Some(next) = next {
            self.rules[next].previous_live = previous;
        }
        let state = &mut self.rules[rule];
        state.previous_live = None;
        state.next_live = None;
        state.live = false;

        if let (Some(previous), Some(next)) = (previous, next) {
            self.enqueue_same_selector_edge(previous, next);
        }
    }

    fn enqueue_same_selector_edge(&mut self, left: RuleId, right: RuleId) {
        if self.rules[left].effective_key == self.rules[right].effective_key {
            self.same_selector_candidates.push(Candidate(left, right));
        }
    }
}

fn collect_declaration_chain(
    declarations: DeclarationBlockId,
    store: &DeclarationBlockStore<'_>,
    chain: &mut Vec<DeclarationBlockId>,
    seen: &mut DenseMap<DeclarationBlockId, bool>,
) {
    for declarations in chain.drain(..) {
        seen[declarations] = false;
    }
    chain.push(declarations);
    seen[declarations] = true;
    let mut current = store.get(declarations).previous_merged();
    while let Some(declarations) = current {
        if std::mem::replace(&mut seen[declarations], true) {
            break;
        }
        current = store.get(declarations).previous_merged();
        chain.push(declarations);
    }
}

fn retire_style_rules<'ast>(
    rules: &mut [CssRule<'ast>],
    retired: &DenseMap<DeclarationBlockId, bool>,
) {
    for rule in rules {
        match rule {
            CssRule::Media(rule) => retire_style_rules(&mut rule.rules, retired),
            CssRule::Style(rule) => {
                retire_style_rules(rule.as_mut().rules_mut(), retired);
                let should_retire = retired[rule.as_ref().get_ref().declarations];
                if should_retire {
                    for selector in rule.as_mut().selectors_mut() {
                        *selector = Selector::Tombstone;
                    }
                }
            }
            CssRule::Supports(rule) => retire_style_rules(&mut rule.rules, retired),
            CssRule::MozDocument(rule) => retire_style_rules(&mut rule.rules, retired),
            CssRule::Nesting(rule) => retire_style_rules(rule.style.as_mut().rules_mut(), retired),
            CssRule::LayerBlock(rule) => retire_style_rules(&mut rule.rules, retired),
            CssRule::Container(rule) => retire_style_rules(&mut rule.rules, retired),
            CssRule::Scope(rule) => retire_style_rules(&mut rule.rules, retired),
            CssRule::StartingStyle(rule) => retire_style_rules(&mut rule.rules, retired),
            _ => {}
        }
    }
}
