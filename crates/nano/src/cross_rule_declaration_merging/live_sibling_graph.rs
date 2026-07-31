use rocketcss_ast::{
    CssRule, DeclarationBlockId, DeclarationBlockStore, EffectiveKeyId, RuleListId, RuleStore,
    Selector,
};
use rocketcss_common::{DenseMap, DenseStore, define_dense_id};
use rustc_hash::FxHashMap;

use super::RuleId;
use super::candidates::{Candidate, SameSelectorCandidateList};
use super::discovery::{
    DeclarationBlockEntries, DeclarationBlockEntry, DeclarationBlockEntryId, DeclarationBlockKind,
    RuleListSegmentId,
};

define_dense_id!(struct SequenceId);

const _: () = {
    assert!(std::mem::size_of::<Option<RuleId>>() == std::mem::size_of::<u32>());
    assert!(std::mem::size_of::<Option<SequenceId>>() == std::mem::size_of::<u32>());
};

#[derive(Debug)]
struct LiveRule {
    ast_rule: rocketcss_ast::RuleId,
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
            sequence_by_block: store.map(|_| None),
            non_empty_blocks: store.map(|_| false),
            same_selector_candidates: SameSelectorCandidateList::default(),
            declaration_links: Vec::with_capacity(style_rule_count.saturating_sub(1)),
        };
        let mut last_style_by_segment: FxHashMap<
            (RuleListId, RuleListSegmentId),
            (RuleId, DeclarationBlockEntryId),
        > = FxHashMap::default();
        for (entry_id, entry) in declaration_blocks.iter_enumerated() {
            let DeclarationBlockKind::Style { .. } = entry.kind else {
                continue;
            };
            let rule = graph.push_rule(entry, store);
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
        let declarations = entry.declarations;
        let old_sequence = self.sequence_by_block[declarations].replace(sequence);
        debug_assert!(
            old_sequence.is_none(),
            "a declaration block belongs to only one sequence"
        );
        let non_empty_block_count = u32::from(!store.view(declarations).is_output_empty());
        self.non_empty_blocks[declarations] = non_empty_block_count != 0;
        let inserted_sequence = self.sequences.push(DeclarationSequence {
            parent: sequence,
            head: declarations,
            tail: declarations,
            non_empty_block_count,
            active_owner: rule,
        });
        debug_assert_eq!(inserted_sequence, sequence);
        let inserted_rule = self.rules.push(LiveRule {
            ast_rule: entry.rule,
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
        rules: &mut RuleStore<'_>,
        store: &mut DeclarationBlockStore<'_>,
    ) -> bool {
        materialize_declaration_links(&self.declaration_links, store);

        let mut has_retired = false;
        for rule in self.rules.iter().filter(|rule| !rule.live) {
            match rules.get_mut(rule.ast_rule) {
                CssRule::Style(style) => {
                    let selectors = style.selectors;
                    for selector in rules.selectors_mut(selectors) {
                        *selector = Selector::Tombstone;
                    }
                }
                CssRule::Nesting(nesting) => {
                    let selectors = nesting.style.selectors;
                    for selector in rules.selectors_mut(selectors) {
                        *selector = Selector::Tombstone;
                    }
                }
                _ => unreachable!("the live graph contains only style rules"),
            }
            has_retired = true;
        }
        has_retired
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

fn materialize_declaration_links(
    links: &[(DeclarationBlockId, DeclarationBlockId)],
    store: &mut DeclarationBlockStore<'_>,
) {
    let mut previous_by_current = FxHashMap::default();
    for &(current, previous) in links {
        let replaced = previous_by_current.insert(current, previous);
        debug_assert!(replaced.is_none(), "a block has one logical predecessor");
    }

    let mut applied = rustc_hash::FxHashSet::default();
    let mut chain = Vec::new();
    for &(target, _) in links {
        chain.clear();
        let mut current = target;
        while !applied.contains(&current) {
            let Some(&previous) = previous_by_current.get(&current) else {
                break;
            };
            chain.push((current, previous));
            current = previous;
        }
        for &(current, previous) in chain.iter().rev() {
            if applied.insert(current) {
                store.prepend_block(current, previous);
            }
        }
    }
}
