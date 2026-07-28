use rocketcss_allocator::{GhostToken, Ref};
use rocketcss_ast::{CssRule, Selector, StyleRule, StyleSheet};
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};

use super::AdjacentStyleRuleScanner;
use super::candidates::{Candidate, SameSelectorCandidateList};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SelectorFingerprint {
    live_selector_count: u32,
    hash: u64,
}

impl SameSelectorCandidateList {
    pub(super) fn discover<'ast, 'ghost>(
        stylesheet: &StyleSheet<'ast, 'ghost>,
        candidate_indices: &FxHashMap<*const StyleRule<'ast, 'ghost>, u32>,
    ) -> Self {
        let mut candidates = Self::with_capacity(candidate_indices.len().saturating_sub(1));
        candidates.discover_rule_list(&stylesheet.rules, candidate_indices);
        candidates
    }

    fn discover_rule_list<'ast, 'ghost>(
        &mut self,
        rules: &[CssRule<'ast, 'ghost>],
        candidate_indices: &FxHashMap<*const StyleRule<'ast, 'ghost>, u32>,
    ) {
        for pair in rules.windows(2) {
            let [CssRule::Style(left), CssRule::Style(right)] = pair else {
                continue;
            };
            let left = candidate_indices[&std::ptr::from_ref(left.as_ref().get_ref())];
            let right = candidate_indices[&std::ptr::from_ref(right.as_ref().get_ref())];
            self.push(Candidate(left, right));
        }

        for rule in rules {
            match rule {
                CssRule::Media(rule) => self.discover_rule_list(&rule.rules, candidate_indices),
                CssRule::Style(rule) => self.discover_rule_list(&rule.rules, candidate_indices),
                CssRule::Supports(rule) => self.discover_rule_list(&rule.rules, candidate_indices),
                CssRule::MozDocument(rule) => {
                    self.discover_rule_list(&rule.rules, candidate_indices)
                }
                CssRule::Nesting(rule) => {
                    self.discover_rule_list(&rule.style.rules, candidate_indices)
                }
                CssRule::LayerBlock(rule) => {
                    self.discover_rule_list(&rule.rules, candidate_indices)
                }
                CssRule::Container(rule) => self.discover_rule_list(&rule.rules, candidate_indices),
                CssRule::Scope(rule) => self.discover_rule_list(&rule.rules, candidate_indices),
                CssRule::StartingStyle(rule) => {
                    self.discover_rule_list(&rule.rules, candidate_indices)
                }
                _ => {}
            }
        }
    }
}

impl<'walk, 'ast, 'ghost> AdjacentStyleRuleScanner<'walk, 'ast, 'ghost> {
    pub(super) fn handle_same_selector_candidate(
        &mut self,
        candidate: Candidate,
        token: &GhostToken<'ghost>,
    ) {
        let can_compare_selectors = {
            let (left, right) = self.candidate_rules(candidate);
            can_compare_same_selector(left, right, token)
        };
        if !can_compare_selectors {
            return;
        }

        let Candidate(left, right) = candidate;
        let left_fingerprint = self.selector_fingerprint(left);
        let right_fingerprint = self.selector_fingerprint(right);
        if left_fingerprint.live_selector_count != 0 && left_fingerprint == right_fingerprint && {
            let (left, right) = self.candidate_rules(candidate);
            equal_live_selectors(&left.selectors, &right.selectors)
        } {
            self.same_selector_commits.push(candidate);
        }
    }

    fn selector_fingerprint(&mut self, index: u32) -> SelectorFingerprint {
        let index = usize::try_from(index).expect("style rule index fits usize");
        if let Some(fingerprint) = self.selector_fingerprints[index] {
            return fingerprint;
        }
        let fingerprint = fingerprint_live_selectors(&self.style_rules[index].selectors);
        self.selector_fingerprints[index] = Some(fingerprint);
        fingerprint
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
        candidate_indices: FxHashMap<*const StyleRule<'ast, 'ghost>, u32>,
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
    candidate_indices: FxHashMap<*const StyleRule<'ast, 'ghost>, u32>,
    candidates: FxHashSet<Candidate>,
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
        debug_assert!(left.as_ref().get_ref().rules.is_empty());
        debug_assert_eq!(
            left.as_ref().get_ref().vendor_prefix,
            right.as_ref().get_ref().vendor_prefix
        );
        debug_assert!(
            right
                .as_ref()
                .get_ref()
                .declarations
                .as_ref()
                .borrow(token)
                .previous_merged()
                .is_none()
        );

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

fn can_compare_same_selector<'ast, 'ghost>(
    left: &StyleRule<'ast, 'ghost>,
    right: &StyleRule<'ast, 'ghost>,
    token: &GhostToken<'ghost>,
) -> bool {
    left.rules.is_empty()
        && left.vendor_prefix == right.vendor_prefix
        && right
            .declarations
            .as_ref()
            .borrow(token)
            .previous_merged()
            .is_none()
}

fn fingerprint_live_selectors(selectors: &rocketcss_ast::SelectorList<'_>) -> SelectorFingerprint {
    let mut hasher = FxHasher::default();
    let mut live_selector_count = 0usize;
    for selector in selectors.iter().filter(|selector| !selector.is_tombstone()) {
        selector.hash(&mut hasher);
        live_selector_count += 1;
    }
    live_selector_count.hash(&mut hasher);
    SelectorFingerprint {
        live_selector_count: u32::try_from(live_selector_count)
            .expect("selector count exceeds u32::MAX"),
        hash: hasher.finish(),
    }
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
