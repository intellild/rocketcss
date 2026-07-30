use rocketcss_allocator::{GhostToken, Ref};
use rocketcss_ast::{CssRule, DeclarationBlock, Selector, StyleSheet};
use rustc_hash::{FxHashMap, FxHashSet};

use super::candidates::{Candidate, SameSelectorCandidateList};
use crate::utils::{
    DeclarationBlockEntry, DeclarationBlockKind, EffectiveKeyId, RuleListId, RuleListSegmentId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RuleId(u32);

impl RuleId {
    fn index(self) -> usize {
        usize::try_from(self.0).expect("rule ID fits usize")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SequenceId(u32);

impl SequenceId {
    fn index(self) -> usize {
        usize::try_from(self.0).expect("declaration sequence ID fits usize")
    }
}

#[derive(Debug)]
struct LiveRule<'ast, 'ghost> {
    declarations: Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
    effective_key: EffectiveKeyId,
    previous_live: Option<RuleId>,
    next_live: Option<RuleId>,
    sequence: SequenceId,
    has_children: bool,
    has_live_selectors: bool,
    live: bool,
}

#[derive(Debug)]
struct DeclarationSequence<'ast, 'ghost> {
    parent: SequenceId,
    head: Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
    tail: Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
    non_empty_block_count: u32,
    active_owner: RuleId,
}

#[derive(Debug)]
pub(super) struct LiveSiblingGraph<'ast, 'ghost> {
    rules: std::vec::Vec<LiveRule<'ast, 'ghost>>,
    sequences: std::vec::Vec<DeclarationSequence<'ast, 'ghost>>,
    sequence_by_block: FxHashMap<*const DeclarationBlock<'ast, 'ghost>, SequenceId>,
    non_empty_blocks: FxHashSet<*const DeclarationBlock<'ast, 'ghost>>,
    same_selector_candidates: SameSelectorCandidateList,
    declaration_links: std::vec::Vec<(
        Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
        Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
    )>,
}

impl<'ast, 'ghost> LiveSiblingGraph<'ast, 'ghost> {
    pub(super) fn new(
        declaration_blocks: &[DeclarationBlockEntry<'_, 'ast, 'ghost>],
        token: &GhostToken<'ghost>,
    ) -> Self {
        let style_rule_count = declaration_blocks
            .iter()
            .filter(|entry| matches!(entry.kind, DeclarationBlockKind::Style { .. }))
            .count();
        let mut graph = Self {
            rules: std::vec::Vec::with_capacity(style_rule_count),
            sequences: std::vec::Vec::with_capacity(style_rule_count),
            sequence_by_block: FxHashMap::with_capacity_and_hasher(
                declaration_blocks.len(),
                Default::default(),
            ),
            non_empty_blocks: FxHashSet::with_capacity_and_hasher(
                declaration_blocks.len(),
                Default::default(),
            ),
            same_selector_candidates: SameSelectorCandidateList::default(),
            declaration_links: std::vec::Vec::with_capacity(style_rule_count.saturating_sub(1)),
        };
        let mut last_style_by_segment: FxHashMap<(RuleListId, RuleListSegmentId), (RuleId, usize)> =
            FxHashMap::default();
        let mut declaration_chain = std::vec::Vec::new();
        let mut declaration_chain_seen = FxHashSet::default();

        for (entry_index, entry) in declaration_blocks.iter().enumerate() {
            let DeclarationBlockKind::Style { .. } = entry.kind else {
                continue;
            };
            let rule = graph.push_rule(
                entry,
                token,
                &mut declaration_chain,
                &mut declaration_chain_seen,
            );
            let segment = (entry.rule_list, entry.rule_list_segment);
            if let Some(&(previous, previous_entry)) = last_style_by_segment.get(&segment)
                && declaration_blocks[previous_entry].is_direct_sibling_of(entry)
            {
                graph.connect(previous, rule);
            }
            last_style_by_segment.insert(segment, (rule, entry_index));
        }

        for rule in 0..graph.rules.len() {
            let rule = RuleId(u32::try_from(rule).expect("style rule count exceeds u32::MAX"));
            if let Some(next) = graph.rules[rule.index()].next_live {
                graph.enqueue_same_selector_edge(rule, next);
            }
        }
        graph
    }

    fn push_rule(
        &mut self,
        entry: &DeclarationBlockEntry<'_, 'ast, 'ghost>,
        token: &GhostToken<'ghost>,
        declaration_chain: &mut std::vec::Vec<Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>>,
        declaration_chain_seen: &mut FxHashSet<*const DeclarationBlock<'ast, 'ghost>>,
    ) -> RuleId {
        let DeclarationBlockKind::Style {
            has_children,
            has_live_selectors,
        } = entry.kind
        else {
            unreachable!("only style rules are inserted into the live sibling graph");
        };
        let rule =
            RuleId(u32::try_from(self.rules.len()).expect("style rule count exceeds u32::MAX"));
        let sequence = SequenceId(
            u32::try_from(self.sequences.len()).expect("sequence count exceeds u32::MAX"),
        );
        collect_declaration_chain(
            entry.declaration_ref,
            entry.declarations,
            token,
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
            let declarations_ptr = std::ptr::from_ref(declarations.get(token).get_ref());
            let old_sequence = self.sequence_by_block.insert(declarations_ptr, sequence);
            debug_assert!(
                old_sequence.is_none(),
                "a declaration block belongs to only one sequence"
            );
            if !declarations.get(token).is_output_empty() {
                self.non_empty_blocks.insert(declarations_ptr);
                non_empty_block_count = non_empty_block_count
                    .checked_add(1)
                    .expect("declaration block count exceeds u32::MAX");
            }
        }
        self.sequences.push(DeclarationSequence {
            parent: sequence,
            head,
            tail,
            non_empty_block_count,
            active_owner: rule,
        });
        self.rules.push(LiveRule {
            declarations: entry.declaration_ref,
            effective_key: entry.effective_key,
            previous_live: None,
            next_live: None,
            sequence,
            has_children,
            has_live_selectors,
            live: true,
        });
        rule
    }

    fn connect(&mut self, left: RuleId, right: RuleId) {
        debug_assert!(self.rules[left.index()].next_live.is_none());
        debug_assert!(self.rules[right.index()].previous_live.is_none());
        self.rules[left.index()].next_live = Some(right);
        self.rules[right.index()].previous_live = Some(left);
    }

    pub(super) fn declaration_block_became_empty(
        &mut self,
        declarations: *const DeclarationBlock<'ast, 'ghost>,
    ) {
        if !self.non_empty_blocks.remove(&declarations) {
            return;
        }
        let Some(&sequence) = self.sequence_by_block.get(&declarations) else {
            return;
        };
        let sequence = self.find_sequence(sequence);
        let state = &mut self.sequences[sequence.index()];
        state.non_empty_block_count = state
            .non_empty_block_count
            .checked_sub(1)
            .expect("an empty declaration block was counted as non-empty");
        if state.non_empty_block_count != 0 {
            return;
        }
        let owner = state.active_owner;
        if self.rules[owner.index()].live && !self.rules[owner.index()].has_children {
            self.unlink_rule(owner);
        }
    }

    pub(super) fn commit(
        &self,
        stylesheet: &mut StyleSheet<'ast, 'ghost>,
        token: &mut GhostToken<'ghost>,
    ) -> bool {
        for &(current, previous) in &self.declaration_links {
            current
                .get_mut(token)
                .get_mut()
                .set_previous_merged(Some(previous));
        }

        let retired: FxHashSet<_> = self
            .rules
            .iter()
            .filter(|rule| !rule.live)
            .map(|rule| {
                let declarations = rule.declarations.get(token);
                std::ptr::from_ref(declarations.get_ref())
            })
            .collect();
        if retired.is_empty() {
            return false;
        }
        retire_style_rules(&mut stylesheet.rules, &retired, token);
        true
    }

    pub(super) fn candidate_is_live_edge(
        &self,
        Candidate(left, right): Candidate,
    ) -> Option<(RuleId, RuleId)> {
        let left = RuleId(left);
        let right = RuleId(right);
        let left_state = self.rules.get(left.index())?;
        let right_state = self.rules.get(right.index())?;
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
        let left_sequence = self.find_sequence(self.rules[left.index()].sequence);
        let right_sequence = self.find_sequence(self.rules[right.index()].sequence);
        debug_assert_ne!(left_sequence, right_sequence);

        let left_head = self.sequences[left_sequence.index()].head;
        let left_tail = self.sequences[left_sequence.index()].tail;
        let left_non_empty = self.sequences[left_sequence.index()].non_empty_block_count;
        let right_head = self.sequences[right_sequence.index()].head;
        let right_non_empty = self.sequences[right_sequence.index()].non_empty_block_count;
        self.declaration_links.push((right_head, left_tail));

        self.sequences[left_sequence.index()].parent = right_sequence;
        let right_state = &mut self.sequences[right_sequence.index()];
        right_state.head = left_head;
        right_state.non_empty_block_count = left_non_empty
            .checked_add(right_non_empty)
            .expect("declaration block count exceeds u32::MAX");
        right_state.active_owner = right;
        self.rules[right.index()].sequence = right_sequence;
        self.unlink_rule(left);
    }

    fn find_sequence(&mut self, sequence: SequenceId) -> SequenceId {
        let mut root = sequence;
        while self.sequences[root.index()].parent != root {
            root = self.sequences[root.index()].parent;
        }
        let mut current = sequence;
        while current != root {
            let parent = self.sequences[current.index()].parent;
            self.sequences[current.index()].parent = root;
            current = parent;
        }
        root
    }

    fn unlink_rule(&mut self, rule: RuleId) {
        let previous = self.rules[rule.index()].previous_live;
        let next = self.rules[rule.index()].next_live;
        if let Some(previous) = previous {
            self.rules[previous.index()].next_live = next;
        }
        if let Some(next) = next {
            self.rules[next.index()].previous_live = previous;
        }
        let state = &mut self.rules[rule.index()];
        state.previous_live = None;
        state.next_live = None;
        state.live = false;

        if let (Some(previous), Some(next)) = (previous, next) {
            self.enqueue_same_selector_edge(previous, next);
        }
    }

    fn enqueue_same_selector_edge(&mut self, left: RuleId, right: RuleId) {
        if self.rules[left.index()].effective_key == self.rules[right.index()].effective_key {
            self.same_selector_candidates
                .push(Candidate(left.0, right.0));
        }
    }
}

fn collect_declaration_chain<'ast, 'ghost>(
    declaration_ref: Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
    declarations: &DeclarationBlock<'ast, 'ghost>,
    token: &GhostToken<'ghost>,
    chain: &mut std::vec::Vec<Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>>,
    seen: &mut FxHashSet<*const DeclarationBlock<'ast, 'ghost>>,
) {
    chain.clear();
    seen.clear();
    chain.push(declaration_ref);
    seen.insert(std::ptr::from_ref(declarations));
    let mut current = declarations.previous_merged();
    while let Some(declarations) = current {
        let block = declarations.get(token);
        if !seen.insert(std::ptr::from_ref(block.get_ref())) {
            break;
        }
        current = block.previous_merged();
        chain.push(declarations);
    }
}

fn retire_style_rules<'ast, 'ghost>(
    rules: &mut [CssRule<'ast, 'ghost>],
    retired: &FxHashSet<*const DeclarationBlock<'ast, 'ghost>>,
    token: &mut GhostToken<'ghost>,
) {
    for rule in rules {
        match rule {
            CssRule::Media(rule) => retire_style_rules(&mut rule.rules, retired, token),
            CssRule::Style(rule) => {
                retire_style_rules(rule.as_mut().rules_mut(), retired, token);
                let should_retire = {
                    let declarations = rule.as_ref().get_ref().declarations.as_ref().borrow(token);
                    retired.contains(&std::ptr::from_ref(declarations.get_ref()))
                };
                if should_retire {
                    for selector in rule.as_mut().selectors_mut() {
                        *selector = Selector::Tombstone;
                    }
                }
            }
            CssRule::Supports(rule) => retire_style_rules(&mut rule.rules, retired, token),
            CssRule::MozDocument(rule) => retire_style_rules(&mut rule.rules, retired, token),
            CssRule::Nesting(rule) => {
                retire_style_rules(rule.style.as_mut().rules_mut(), retired, token)
            }
            CssRule::LayerBlock(rule) => retire_style_rules(&mut rule.rules, retired, token),
            CssRule::Container(rule) => retire_style_rules(&mut rule.rules, retired, token),
            CssRule::Scope(rule) => retire_style_rules(&mut rule.rules, retired, token),
            CssRule::StartingStyle(rule) => retire_style_rules(&mut rule.rules, retired, token),
            _ => {}
        }
    }
}
