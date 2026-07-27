use std::collections::{HashMap, HashSet};

use rocketcss_allocator::{GhostToken, Ref};
use rocketcss_ast::{CssRule, Selector, StyleRule, StyleSheet};

use super::AdjacentStyleRuleScanner;
use super::candidates::Candidate;

impl<'walk, 'ast, 'ghost> AdjacentStyleRuleScanner<'walk, 'ast, 'ghost> {
    pub(super) fn discover_same_selector_candidates(&mut self) {
        for left in 0..self.style_rules.len().saturating_sub(1) {
            let left = u32::try_from(left).expect("style rule index exceeds u32::MAX");
            self.same_selector_candidates
                .push(Candidate(left, left + 1));
        }
    }

    pub(super) fn handle_same_selector_candidate(
        &mut self,
        candidate: Candidate,
        token: &GhostToken<'ghost>,
    ) {
        let (left, right) = self.candidate_rules(candidate);
        if can_merge_same_selector(left, right, token) {
            self.same_selector_commits.push(candidate);
        }
    }

    fn candidate_rules(
        &self,
        Candidate(left, right): Candidate,
    ) -> (&StyleRule<'ast, 'ghost>, &StyleRule<'ast, 'ghost>) {
        let left = self.style_rules[usize::try_from(left).expect("style rule index fits usize")];
        let right = self.style_rules[usize::try_from(right).expect("style rule index fits usize")];
        (left, right)
    }

    pub(super) fn into_same_selector_commit_pass(
        self,
        candidate_indices: HashMap<*const StyleRule<'ast, 'ghost>, u32>,
    ) -> Option<SameSelectorCommitPass<'ast, 'ghost>> {
        if self.same_selector_commits.is_empty() {
            return None;
        }

        Some(SameSelectorCommitPass {
            candidate_indices,
            candidates: self.same_selector_commits.into_iter().collect(),
        })
    }
}

pub(super) struct SameSelectorCommitPass<'ast, 'ghost> {
    candidate_indices: HashMap<*const StyleRule<'ast, 'ghost>, u32>,
    candidates: HashSet<Candidate>,
}

impl<'ast, 'ghost> SameSelectorCommitPass<'ast, 'ghost> {
    pub(super) fn commit(
        &self,
        stylesheet: &mut StyleSheet<'ast, 'ghost>,
        token: &mut GhostToken<'ghost>,
    ) {
        self.commit_candidates(&mut stylesheet.rules, token);
    }

    fn commit_candidates(
        &self,
        rules: &mut [CssRule<'ast, 'ghost>],
        token: &mut GhostToken<'ghost>,
    ) {
        for index in 0..rules.len().saturating_sub(1) {
            let (left_rules, right_rules) = rules.split_at_mut(index + 1);
            let (CssRule::Style(left), CssRule::Style(right)) =
                (&mut left_rules[index], &mut right_rules[0])
            else {
                continue;
            };

            let left_index = self.candidate_indices[&std::ptr::from_ref(left.as_ref().get_ref())];
            let right_index = self.candidate_indices[&std::ptr::from_ref(right.as_ref().get_ref())];
            if self
                .candidates
                .contains(&Candidate(left_index, right_index))
            {
                self.commit_pair(left.as_mut(), right.as_mut(), token);
            }
        }

        for rule in rules {
            self.commit_children(rule, token);
        }
    }

    fn commit_children(&self, rule: &mut CssRule<'ast, 'ghost>, token: &mut GhostToken<'ghost>) {
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
            _ => {}
        }
    }

    fn commit_pair(
        &self,
        mut left: std::pin::Pin<&mut StyleRule<'ast, 'ghost>>,
        right: std::pin::Pin<&mut StyleRule<'ast, 'ghost>>,
        token: &mut GhostToken<'ghost>,
    ) {
        if !can_merge_same_selector(left.as_ref().get_ref(), right.as_ref().get_ref(), token) {
            return;
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
    }
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
