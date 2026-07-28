mod candidates;
mod same_selector;

use rocketcss_allocator::GhostToken;
use rocketcss_ast::{StyleRule, StyleSheet};

use self::candidates::{
    DeclarationOverrideCandidateList, PartialMergeCandidateList, SameSelectorCandidateList,
};
use crate::utils::walk_style_rules;
use crate::{MinifyContext, Options, OptionsOp};

#[derive(Debug)]
struct AdjacentStyleRuleScanner<'walk, 'ast, 'ghost> {
    style_rules: std::vec::Vec<&'walk StyleRule<'ast, 'ghost>>,
    selector_fingerprints: std::vec::Vec<Option<same_selector::SelectorFingerprint>>,
    same_selector_candidates: SameSelectorCandidateList,
    same_selector_commits: std::vec::Vec<candidates::Candidate>,
    declaration_override_candidates: DeclarationOverrideCandidateList,
    partial_merge_candidates: PartialMergeCandidateList,
}

impl<'walk, 'ast, 'ghost> AdjacentStyleRuleScanner<'walk, 'ast, 'ghost> {
    fn new(
        style_rules: std::vec::Vec<&'walk StyleRule<'ast, 'ghost>>,
        same_selector_candidates: SameSelectorCandidateList,
    ) -> Self {
        if let Some(last_index) = style_rules.len().checked_sub(1) {
            u32::try_from(last_index).expect("style rule index exceeds u32::MAX");
        }
        let selector_fingerprints = vec![None; style_rules.len()];

        Self {
            style_rules,
            selector_fingerprints,
            same_selector_candidates,
            same_selector_commits: std::vec::Vec::new(),
            declaration_override_candidates: DeclarationOverrideCandidateList::default(),
            partial_merge_candidates: PartialMergeCandidateList::default(),
        }
    }

    fn run(&mut self, token: &GhostToken<'ghost>) {
        loop {
            if let Some(candidate) = self.same_selector_candidates.pop() {
                self.handle_same_selector_candidate(candidate, token);
                continue;
            }

            if let Some(candidate) = self.declaration_override_candidates.pop() {
                self.handle_declaration_override_candidate(candidate);
                continue;
            }

            if let Some(candidate) = self.partial_merge_candidates.pop() {
                self.handle_partial_merge_candidate(candidate);
                continue;
            }

            break;
        }
    }

    fn handle_declaration_override_candidate(&mut self, _candidate: candidates::Candidate) {}

    fn handle_partial_merge_candidate(&mut self, _candidate: candidates::Candidate) {}
}

pub(crate) fn merge_cross_rule_declarations<'ast, 'ghost, 'scratch>(
    stylesheet: &mut StyleSheet<'ast, 'ghost>,
    token: &mut GhostToken<'ghost>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    if !cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
        return;
    }

    let commit_pass = {
        let style_rules = walk_style_rules(stylesheet);
        let candidate_indices = style_rules
            .iter()
            .enumerate()
            .map(|(index, rule)| {
                (
                    std::ptr::from_ref(*rule),
                    u32::try_from(index).expect("style rule index exceeds u32::MAX"),
                )
            })
            .collect();
        let same_selector_candidates =
            SameSelectorCandidateList::discover(stylesheet, &candidate_indices);
        let mut scanner = AdjacentStyleRuleScanner::new(style_rules, same_selector_candidates);
        scanner.run(token);
        scanner.into_same_selector_commit_pass(candidate_indices)
    };

    if let Some(commit_pass) = commit_pass {
        commit_pass.commit(stylesheet, token);
    }
}
