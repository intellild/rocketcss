use std::collections::{BTreeSet, HashSet, VecDeque};

use rocketcss_allocator::GhostToken;
use rocketcss_ast::{StyleRule, StyleSheet};

use super::DeclarationBlockMinifier;
use crate::utils::walk_style_rules;
use crate::{MinifyContext, Options, OptionsOp};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Candidate(u32, u32);

#[derive(Debug, Default)]
struct CandidateQueue {
    candidates: VecDeque<Candidate>,
    queued: HashSet<Candidate>,
}

impl CandidateQueue {
    #[allow(dead_code)]
    fn push(&mut self, candidate: Candidate) {
        if self.queued.insert(candidate) {
            self.candidates.push_back(candidate);
        }
    }

    fn pop(&mut self) -> Option<Candidate> {
        let candidate = self.candidates.pop_front()?;
        self.queued.remove(&candidate);
        Some(candidate)
    }
}

#[derive(Debug, Default)]
struct SameSelectorCandidateList(CandidateQueue);

impl SameSelectorCandidateList {
    #[allow(dead_code)]
    fn push(&mut self, candidate: Candidate) {
        self.0.push(candidate);
    }

    fn pop(&mut self) -> Option<Candidate> {
        self.0.pop()
    }
}

#[derive(Debug, Default)]
struct DeclarationOverrideCandidateList(CandidateQueue);

impl DeclarationOverrideCandidateList {
    #[allow(dead_code)]
    fn push(&mut self, candidate: Candidate) {
        self.0.push(candidate);
    }

    fn pop(&mut self) -> Option<Candidate> {
        self.0.pop()
    }
}

#[derive(Debug, Default)]
struct PartialMergeCandidateList {
    candidates: BTreeSet<Candidate>,
}

impl PartialMergeCandidateList {
    #[allow(dead_code)]
    fn push(&mut self, candidate: Candidate) {
        self.candidates.insert(candidate);
    }

    fn pop(&mut self) -> Option<Candidate> {
        self.candidates.pop_first()
    }
}

#[derive(Debug)]
struct AdjacentStyleRuleScanner<'walk, 'ast, 'ghost> {
    #[allow(dead_code)]
    style_rules: std::vec::Vec<&'walk StyleRule<'ast, 'ghost>>,
    same_selector_candidates: SameSelectorCandidateList,
    declaration_override_candidates: DeclarationOverrideCandidateList,
    partial_merge_candidates: PartialMergeCandidateList,
}

impl<'walk, 'ast, 'ghost> AdjacentStyleRuleScanner<'walk, 'ast, 'ghost> {
    fn new(style_rules: std::vec::Vec<&'walk StyleRule<'ast, 'ghost>>) -> Self {
        if let Some(last_index) = style_rules.len().checked_sub(1) {
            u32::try_from(last_index).expect("style rule index exceeds u32::MAX");
        }

        Self {
            style_rules,
            same_selector_candidates: SameSelectorCandidateList::default(),
            declaration_override_candidates: DeclarationOverrideCandidateList::default(),
            partial_merge_candidates: PartialMergeCandidateList::default(),
        }
    }

    fn run(&mut self) {
        loop {
            if let Some(candidate) = self.same_selector_candidates.pop() {
                self.handle_same_selector_candidate(candidate);
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

    fn handle_same_selector_candidate(&mut self, _candidate: Candidate) {}

    fn handle_declaration_override_candidate(&mut self, _candidate: Candidate) {}

    fn handle_partial_merge_candidate(&mut self, _candidate: Candidate) {}
}

pub(crate) fn merge_adjacent_style_rules<'ast, 'ghost, 'scratch>(
    stylesheet: &mut StyleSheet<'ast, 'ghost>,
    _token: &mut GhostToken<'ghost>,
    _minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    if !cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
        return;
    }

    let style_rules = walk_style_rules(stylesheet);
    AdjacentStyleRuleScanner::new(style_rules).run();
}
