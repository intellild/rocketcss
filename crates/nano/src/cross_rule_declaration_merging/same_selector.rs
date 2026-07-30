use rocketcss_allocator::{GhostToken, Ref};
use rocketcss_ast::{CssRule, DeclarationBlock, Selector, StyleRule, StyleSheet};
use rustc_hash::{FxHashMap, FxHashSet};

use super::CrossRuleDeclarationScanner;
use super::candidates::Candidate;
use crate::utils::DeclarationBlockEntry;

impl<'walk, 'ast, 'ghost> CrossRuleDeclarationScanner<'walk, 'ast, 'ghost> {
    pub(super) fn discover_same_selector_candidates(&mut self) {
        for left in 0..self.declaration_blocks.len().saturating_sub(1) {
            let left = u32::try_from(left).expect("declaration block index exceeds u32::MAX");
            let candidate = Candidate(left, left + 1);
            let (left, right) = self.candidate_blocks(candidate);
            if left.is_direct_sibling_of(right) {
                self.same_selector_candidates.push(candidate);
            }
        }
    }

    pub(super) fn handle_same_selector_candidate(&mut self, candidate: Candidate) {
        let (left, right) = self.candidate_blocks(candidate);
        if can_merge_same_selector_blocks(left, right) {
            self.same_selector_commits.push(candidate);
        }
    }

    fn candidate_blocks(
        &self,
        Candidate(left, right): Candidate,
    ) -> (
        &DeclarationBlockEntry<'walk, 'ast, 'ghost>,
        &DeclarationBlockEntry<'walk, 'ast, 'ghost>,
    ) {
        let left = &self.declaration_blocks
            [usize::try_from(left).expect("declaration block index fits usize")];
        let right = &self.declaration_blocks
            [usize::try_from(right).expect("declaration block index fits usize")];
        (left, right)
    }

    pub(super) fn take_same_selector_commit_pass(
        &mut self,
    ) -> Option<SameSelectorCommitPass<'ast, 'ghost>> {
        if self.same_selector_commits.is_empty() {
            return None;
        }

        let candidate_indices = self
            .declaration_blocks
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                (
                    std::ptr::from_ref(entry.declarations),
                    u32::try_from(index).expect("declaration block index exceeds u32::MAX"),
                )
            })
            .collect();
        Some(SameSelectorCommitPass {
            candidate_indices,
            candidates: std::mem::take(&mut self.same_selector_commits)
                .into_iter()
                .collect(),
        })
    }
}

pub(super) struct SameSelectorCommitPass<'ast, 'ghost> {
    candidate_indices: FxHashMap<*const DeclarationBlock<'ast, 'ghost>, u32>,
    candidates: FxHashSet<Candidate>,
}

impl<'ast, 'ghost> SameSelectorCommitPass<'ast, 'ghost> {
    pub(super) fn commit(
        &self,
        stylesheet: &mut StyleSheet<'ast, 'ghost>,
        token: &mut GhostToken<'ghost>,
    ) -> bool {
        self.commit_candidates(&mut stylesheet.rules, token)
    }

    fn commit_candidates(
        &self,
        rules: &mut [CssRule<'ast, 'ghost>],
        token: &mut GhostToken<'ghost>,
    ) -> bool {
        let mut changed = false;
        for index in 0..rules.len().saturating_sub(1) {
            let (left_rules, right_rules) = rules.split_at_mut(index + 1);
            let (CssRule::Style(left), CssRule::Style(right)) =
                (&mut left_rules[index], &mut right_rules[0])
            else {
                continue;
            };

            let Some(left_index) = self.declaration_block_index(left.as_ref().get_ref(), token)
            else {
                continue;
            };
            let Some(right_index) = self.declaration_block_index(right.as_ref().get_ref(), token)
            else {
                continue;
            };
            if self
                .candidates
                .contains(&Candidate(left_index, right_index))
            {
                changed |= self.commit_pair(left.as_mut(), right.as_mut(), token);
            }
        }

        for rule in rules {
            changed |= self.commit_children(rule, token);
        }
        changed
    }

    fn declaration_block_index(
        &self,
        rule: &StyleRule<'ast, 'ghost>,
        token: &GhostToken<'ghost>,
    ) -> Option<u32> {
        let declarations = rule.declarations.as_ref().borrow(token);
        self.candidate_indices
            .get(&std::ptr::from_ref(declarations.get_ref()))
            .copied()
    }

    fn commit_children(
        &self,
        rule: &mut CssRule<'ast, 'ghost>,
        token: &mut GhostToken<'ghost>,
    ) -> bool {
        match rule {
            CssRule::Media(rule) => self.commit_candidates(&mut rule.rules, token),
            CssRule::Style(rule) => self.commit_candidates(rule.as_mut().rules_mut(), token),
            CssRule::Supports(rule) => self.commit_candidates(&mut rule.rules, token),
            CssRule::MozDocument(rule) => self.commit_candidates(&mut rule.rules, token),
            CssRule::Nesting(rule) => {
                self.commit_candidates(rule.style.as_mut().rules_mut(), token)
            }
            CssRule::LayerBlock(rule) => self.commit_candidates(&mut rule.rules, token),
            CssRule::Container(rule) => self.commit_candidates(&mut rule.rules, token),
            CssRule::Scope(rule) => self.commit_candidates(&mut rule.rules, token),
            CssRule::StartingStyle(rule) => self.commit_candidates(&mut rule.rules, token),
            _ => false,
        }
    }

    fn commit_pair(
        &self,
        mut left: std::pin::Pin<&mut StyleRule<'ast, 'ghost>>,
        right: std::pin::Pin<&mut StyleRule<'ast, 'ghost>>,
        token: &mut GhostToken<'ghost>,
    ) -> bool {
        if !can_merge_same_selector(left.as_ref().get_ref(), right.as_ref().get_ref(), token) {
            return false;
        }

        let previous = Ref::from(&left.as_ref().get_ref().declarations);
        {
            let mut declarations = right
                .as_ref()
                .get_ref()
                .declarations
                .as_ref()
                .borrow_mut(token);
            declarations
                .as_mut()
                .get_mut()
                .set_previous_merged(Some(previous));
        }
        for selector in left.as_mut().selectors_mut() {
            *selector = Selector::Tombstone;
        }
        true
    }
}

fn can_merge_same_selector_blocks(
    left: &DeclarationBlockEntry<'_, '_, '_>,
    right: &DeclarationBlockEntry<'_, '_, '_>,
) -> bool {
    left.effective_key == right.effective_key && right.declarations.previous_merged().is_none()
}

fn can_merge_same_selector<'ast, 'ghost>(
    left: &StyleRule<'ast, 'ghost>,
    right: &StyleRule<'ast, 'ghost>,
    token: &GhostToken<'ghost>,
) -> bool {
    left.rules.is_empty()
        && left.vendor_prefix == right.vendor_prefix
        && equal_live_selectors(&left.selectors, &right.selectors)
        && right
            .declarations
            .as_ref()
            .borrow(token)
            .previous_merged()
            .is_none()
}

fn equal_live_selectors(
    left: &rocketcss_ast::SelectorList<'_>,
    right: &rocketcss_ast::SelectorList<'_>,
) -> bool {
    let mut left = left.iter().filter(|selector| !selector.is_tombstone());
    let mut right = right.iter().filter(|selector| !selector.is_tombstone());
    let Some(first) = left.next() else {
        return false;
    };
    let Some(other_first) = right.next() else {
        return false;
    };
    first == other_first && left.eq(right)
}
