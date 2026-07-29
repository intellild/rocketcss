mod candidates;
mod declaration_override;
mod same_selector;

use rocketcss_allocator::{GhostToken, vec::Vec};
use rocketcss_ast::{CssRule, StyleSheet};
use rustc_hash::FxHashMap;

use self::candidates::{
    DeclarationOverrideCandidateList, PartialMergeCandidateList, SameSelectorCandidateList,
};
use crate::rules::DeclarationBlockMinifier;
use crate::utils::{DeclarationBlockEntry, walk_declaration_blocks};
use crate::{MinifyContext, Options, OptionsOp};

#[derive(Debug)]
struct CrossRuleDeclarationScanner<'walk, 'ast, 'ghost> {
    declaration_blocks: std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
    same_selector_candidates: SameSelectorCandidateList,
    same_selector_commits: std::vec::Vec<candidates::Candidate>,
    declaration_override_candidates: DeclarationOverrideCandidateList,
    declaration_override_commits: std::vec::Vec<std::vec::Vec<u32>>,
    declaration_override_history_by_tail: FxHashMap<u32, usize>,
    partial_merge_candidates: PartialMergeCandidateList,
}

impl<'walk, 'ast, 'ghost> CrossRuleDeclarationScanner<'walk, 'ast, 'ghost> {
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
            declaration_override_candidates: DeclarationOverrideCandidateList::with_capacity(
                adjacent_block_count,
            ),
            declaration_override_commits: std::vec::Vec::new(),
            declaration_override_history_by_tail: FxHashMap::default(),
            partial_merge_candidates: PartialMergeCandidateList::default(),
        }
    }

    fn run(&mut self) {
        self.discover_same_selector_candidates();
        self.discover_declaration_override_candidates();

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

    fn handle_partial_merge_candidate(&mut self, _candidate: candidates::Candidate) {}
}

pub(crate) fn merge_cross_rule_declarations<'ast, 'ghost, 'scratch>(
    stylesheet: &mut StyleSheet<'ast, 'ghost>,
    token: &mut GhostToken<'ghost>,
    declaration_block_minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    if !cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
        return;
    }

    loop {
        let (same_selector_commit_pass, declaration_override_commit_pass) = {
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
            let mut scanner = CrossRuleDeclarationScanner::new(declaration_blocks);
            scanner.run();
            let same_selector_commit_pass =
                scanner.take_same_selector_commit_pass(candidate_indices);
            let declaration_override_commit_pass = scanner.take_declaration_override_commit_pass();
            (same_selector_commit_pass, declaration_override_commit_pass)
        };

        let mut changed = false;
        if let Some(commit_pass) = same_selector_commit_pass {
            changed |= commit_pass.commit(stylesheet, token);
        }
        if let Some(commit_pass) = declaration_override_commit_pass {
            changed |= commit_pass.commit(stylesheet, declaration_block_minifier, token, cx);
        }
        changed |= compact_retired_style_rules(&mut stylesheet.rules);
        if !changed {
            break;
        }
    }
}

fn compact_retired_style_rules(rules: &mut Vec<'_, CssRule<'_, '_>>) -> bool {
    let mut changed = false;
    for rule in rules.iter_mut() {
        match rule {
            CssRule::Media(rule) => changed |= compact_retired_style_rules(&mut rule.rules),
            CssRule::Style(rule) => {
                changed |= compact_retired_style_rules(rule.as_mut().rules_mut())
            }
            CssRule::Supports(rule) => changed |= compact_retired_style_rules(&mut rule.rules),
            CssRule::MozDocument(rule) => changed |= compact_retired_style_rules(&mut rule.rules),
            CssRule::Nesting(rule) => {
                changed |= compact_retired_style_rules(rule.style.as_mut().rules_mut())
            }
            CssRule::LayerBlock(rule) => changed |= compact_retired_style_rules(&mut rule.rules),
            CssRule::Container(rule) => changed |= compact_retired_style_rules(&mut rule.rules),
            CssRule::Scope(rule) => changed |= compact_retired_style_rules(&mut rule.rules),
            CssRule::StartingStyle(rule) => changed |= compact_retired_style_rules(&mut rule.rules),
            _ => {}
        }
    }

    let previous_len = rules.len();
    rules.retain(|rule| {
        !matches!(
            rule,
            CssRule::Style(rule)
                if rule.as_ref().get_ref().rules.is_empty()
                    && rule
                        .as_ref()
                        .get_ref()
                        .selectors
                        .iter()
                        .all(|selector| selector.is_tombstone())
        )
    });
    changed | (rules.len() != previous_len)
}
