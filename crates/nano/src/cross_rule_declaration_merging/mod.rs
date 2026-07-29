mod candidates;
mod same_selector;

use rocketcss_allocator::GhostToken;
use rocketcss_ast::StyleSheet;

use self::candidates::{
    DeclarationOverrideCandidateList, PartialMergeCandidateList, SameSelectorCandidateList,
};
use crate::utils::{DeclarationBlockEntry, walk_declaration_blocks};
use crate::{MinifyContext, Options, OptionsOp};

#[derive(Debug)]
struct AdjacentDeclarationBlockScanner<'walk, 'ast, 'ghost> {
    declaration_blocks: std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
    same_selector_candidates: SameSelectorCandidateList,
    same_selector_commits: std::vec::Vec<candidates::Candidate>,
    declaration_override_candidates: DeclarationOverrideCandidateList,
    partial_merge_candidates: PartialMergeCandidateList,
}

impl<'walk, 'ast, 'ghost> AdjacentDeclarationBlockScanner<'walk, 'ast, 'ghost> {
    fn new(declaration_blocks: std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>) -> Self {
        if let Some(last_index) = declaration_blocks.len().checked_sub(1) {
            u32::try_from(last_index).expect("declaration block index exceeds u32::MAX");
        }
        let adjacent_block_count = declaration_blocks.len().saturating_sub(1);

        Self {
            declaration_blocks,
            same_selector_candidates: SameSelectorCandidateList::with_capacity(
                adjacent_block_count,
            ),
            same_selector_commits: std::vec::Vec::new(),
            declaration_override_candidates: DeclarationOverrideCandidateList::default(),
            partial_merge_candidates: PartialMergeCandidateList::default(),
        }
    }

    fn run(&mut self) {
        self.discover_same_selector_candidates();

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
        let declaration_blocks = walk_declaration_blocks(stylesheet, token);
        let candidate_indices = declaration_blocks
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                (
                    std::ptr::from_ref(entry.declarations),
                    u32::try_from(index).expect("declaration block index exceeds u32::MAX"),
                )
            })
            .collect();
        let mut scanner = AdjacentDeclarationBlockScanner::new(declaration_blocks);
        scanner.run();
        scanner.into_same_selector_commit_pass(candidate_indices)
    };

    if let Some(commit_pass) = commit_pass {
        commit_pass.commit(stylesheet, token);
    }
}
