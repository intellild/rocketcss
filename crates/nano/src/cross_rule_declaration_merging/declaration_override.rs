use rocketcss_allocator::{GhostToken, Ref};
use rocketcss_ast::{CssRule, DeclarationBlock, Selector, StyleRule, StyleSheet};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use super::CrossRuleDeclarationScanner;
use super::candidates::Candidate;
use crate::MinifyContext;
use crate::rules::DeclarationBlockMinifier;

#[derive(Clone, Copy, Debug)]
struct HistoryTail {
    representative: u32,
    tail: u32,
}

impl<'walk, 'ast, 'ghost> CrossRuleDeclarationScanner<'walk, 'ast, 'ghost> {
    pub(super) fn discover_declaration_override_candidates(&mut self) {
        let mut histories: FxHashMap<u64, SmallVec<[HistoryTail; 1]>> =
            FxHashMap::with_capacity_and_hasher(self.declaration_blocks.len(), Default::default());
        let mut candidates =
            std::vec::Vec::with_capacity(self.declaration_blocks.len().saturating_sub(1));

        for current in 0..self.declaration_blocks.len() {
            let current = u32::try_from(current).expect("declaration block index exceeds u32::MAX");
            let entry = &self.declaration_blocks
                [usize::try_from(current).expect("declaration block index fits usize")];
            let bucket = histories
                .entry(entry.effective_key.fingerprint())
                .or_default();
            let history = bucket.iter_mut().find(|history| {
                let representative =
                    &self.declaration_blocks[usize::try_from(history.representative)
                        .expect("declaration block index fits usize")];
                representative.effective_key == entry.effective_key
            });

            if let Some(history) = history {
                candidates.push(Candidate(history.tail, current));
                history.tail = current;
            } else {
                bucket.push(HistoryTail {
                    representative: current,
                    tail: current,
                });
            }
        }

        for candidate in candidates {
            self.declaration_override_candidates.push(candidate);
        }
    }

    pub(super) fn handle_declaration_override_candidate(&mut self, candidate: Candidate) {
        let Candidate(left, right) = candidate;
        debug_assert_eq!(
            self.declaration_blocks
                [usize::try_from(left).expect("declaration block index fits usize")]
            .effective_key,
            self.declaration_blocks
                [usize::try_from(right).expect("declaration block index fits usize")]
            .effective_key
        );

        let history = if let Some(history) = self.declaration_override_history_by_tail.remove(&left)
        {
            history
        } else {
            let history = self.declaration_override_commits.len();
            self.declaration_override_commits
                .push(std::vec::Vec::from([left]));
            history
        };
        self.declaration_override_commits[history].push(right);
        self.declaration_override_history_by_tail
            .insert(right, history);
    }

    pub(super) fn take_declaration_override_commit_pass(
        &mut self,
    ) -> Option<DeclarationOverrideCommitPass<'ast, 'ghost>> {
        if self.declaration_override_commits.is_empty() {
            return None;
        }

        let histories = std::mem::take(&mut self.declaration_override_commits)
            .into_iter()
            .map(|history| {
                history
                    .into_iter()
                    .map(|index| {
                        self.declaration_blocks
                            [usize::try_from(index).expect("declaration block index fits usize")]
                        .declaration_ref
                    })
                    .collect()
            })
            .collect();
        Some(DeclarationOverrideCommitPass { histories })
    }
}

pub(super) struct DeclarationOverrideCommitPass<'ast, 'ghost> {
    histories: std::vec::Vec<std::vec::Vec<Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct DeclarationOverrideCommitResult {
    pub(super) declarations_removed: bool,
    pub(super) rules_retired: bool,
}

impl<'ast, 'ghost> DeclarationOverrideCommitPass<'ast, 'ghost> {
    pub(super) fn commit<'scratch>(
        &self,
        stylesheet: &mut StyleSheet<'ast, 'ghost>,
        minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
        token: &mut GhostToken<'ghost>,
        cx: &mut MinifyContext<'scratch>,
    ) -> DeclarationOverrideCommitResult
    where
        'ast: 'scratch,
    {
        let declarations_removed = cx.stats().declarations_removed;
        let mut newly_empty = FxHashSet::default();
        for history in &self.histories {
            let mut expanded_history = std::vec::Vec::new();
            let mut seen = FxHashSet::default();
            for &declarations in history {
                append_declaration_chain(declarations, token, &mut seen, &mut expanded_history);
            }

            let was_output_empty: std::vec::Vec<_> = expanded_history
                .iter()
                .map(|declarations| declarations.get(token).is_output_empty())
                .collect();
            minifier.deduplicate_exact_sequence(&expanded_history, token, cx);
            for (&declarations, was_output_empty) in expanded_history.iter().zip(was_output_empty) {
                if !was_output_empty && declarations.get(token).is_output_empty() {
                    newly_empty.insert(std::ptr::from_ref(declarations.get(token).get_ref()));
                }
            }
        }
        DeclarationOverrideCommitResult {
            declarations_removed: cx.stats().declarations_removed != declarations_removed,
            rules_retired: retire_empty_style_rules(&mut stylesheet.rules, &newly_empty, token),
        }
    }
}

fn append_declaration_chain<'ast, 'ghost>(
    declarations: Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
    token: &GhostToken<'ghost>,
    seen: &mut FxHashSet<*const DeclarationBlock<'ast, 'ghost>>,
    output: &mut std::vec::Vec<Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>>,
) {
    if !seen.insert(std::ptr::from_ref(declarations.get(token).get_ref())) {
        return;
    }
    let previous = declarations.get(token).previous_merged();
    if let Some(previous) = previous {
        append_declaration_chain(previous, token, seen, output);
    }
    output.push(declarations);
}

fn retire_empty_style_rules<'ast, 'ghost>(
    rules: &mut [CssRule<'ast, 'ghost>],
    newly_empty: &FxHashSet<*const DeclarationBlock<'ast, 'ghost>>,
    token: &mut GhostToken<'ghost>,
) -> bool {
    let mut changed = false;
    for rule in rules {
        match rule {
            CssRule::Media(rule) => {
                changed |= retire_empty_style_rules(&mut rule.rules, newly_empty, token)
            }
            CssRule::Style(rule) => {
                changed |= retire_empty_style_rules(rule.as_mut().rules_mut(), newly_empty, token);
                let style = rule.as_ref().get_ref();
                let declarations = style.declarations.as_ref().borrow(token);
                if newly_empty.contains(&std::ptr::from_ref(declarations.get_ref()))
                    && style.rules.is_empty()
                    && style_rule_declaration_chain_is_output_empty(style, token)
                {
                    for selector in rule.as_mut().selectors_mut() {
                        changed |= !selector.is_tombstone();
                        *selector = Selector::Tombstone;
                    }
                }
            }
            CssRule::Supports(rule) => {
                changed |= retire_empty_style_rules(&mut rule.rules, newly_empty, token)
            }
            CssRule::MozDocument(rule) => {
                changed |= retire_empty_style_rules(&mut rule.rules, newly_empty, token)
            }
            CssRule::Nesting(rule) => {
                changed |=
                    retire_empty_style_rules(rule.style.as_mut().rules_mut(), newly_empty, token)
            }
            CssRule::LayerBlock(rule) => {
                changed |= retire_empty_style_rules(&mut rule.rules, newly_empty, token)
            }
            CssRule::Container(rule) => {
                changed |= retire_empty_style_rules(&mut rule.rules, newly_empty, token)
            }
            CssRule::Scope(rule) => {
                changed |= retire_empty_style_rules(&mut rule.rules, newly_empty, token)
            }
            CssRule::StartingStyle(rule) => {
                changed |= retire_empty_style_rules(&mut rule.rules, newly_empty, token)
            }
            _ => {}
        }
    }
    changed
}

fn style_rule_declaration_chain_is_output_empty<'ghost>(
    rule: &StyleRule<'_, 'ghost>,
    token: &GhostToken<'ghost>,
) -> bool {
    let mut declarations = rule.declarations.as_ref().borrow(token);
    loop {
        if !declarations.is_output_empty() {
            return false;
        }
        let Some(previous) = declarations.previous_merged() else {
            return true;
        };
        declarations = previous.get(token);
    }
}

#[cfg(test)]
mod tests {
    use rocketcss_allocator::Allocator;
    use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
    use rocketcss_parser::{ParserOptions, parse};

    use super::*;
    use crate::MinifyOptions;
    use crate::utils::walk_declaration_blocks;

    #[test]
    fn discovers_s2_history_in_fifo_order_without_an_s1_commit() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                "a{x:1}.bar-1{y:1}a{x:1}.bar-2{y:1}a{x:1}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let declaration_blocks = walk_declaration_blocks(&stylesheet, &token);
            let mut scanner = CrossRuleDeclarationScanner::new(declaration_blocks);

            scanner.discover_same_selector_candidates();
            while let Some(candidate) = scanner.same_selector_candidates.pop() {
                scanner.handle_same_selector_candidate(candidate);
            }
            assert!(scanner.same_selector_commits.is_empty());

            scanner.discover_declaration_override_candidates();
            assert_eq!(
                scanner.declaration_override_candidates.pop(),
                Some(Candidate(0, 2))
            );
            assert_eq!(
                scanner.declaration_override_candidates.pop(),
                Some(Candidate(2, 4))
            );
            assert_eq!(scanner.declaration_override_candidates.pop(), None);
        });
    }

    #[test]
    fn commits_s2_without_running_s1() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut stylesheet = parse(
                "a{width:1px}.bar{x:1}a{width:1px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let commit_pass = {
                let declaration_blocks = walk_declaration_blocks(&stylesheet, &token);
                let mut scanner = CrossRuleDeclarationScanner::new(declaration_blocks);
                scanner.discover_same_selector_candidates();
                while let Some(candidate) = scanner.same_selector_candidates.pop() {
                    scanner.handle_same_selector_candidate(candidate);
                }
                assert!(scanner.same_selector_commits.is_empty());
                scanner.discover_declaration_override_candidates();
                while let Some(candidate) = scanner.declaration_override_candidates.pop() {
                    scanner.handle_declaration_override_candidate(candidate);
                }
                scanner
                    .take_declaration_override_commit_pass()
                    .expect("the two a blocks share one exact-only S2 history")
            };

            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            assert_eq!(
                commit_pass.commit(&mut stylesheet, &mut minifier, &mut token, &mut cx),
                DeclarationOverrideCommitResult {
                    declarations_removed: true,
                    rules_retired: true,
                }
            );
            assert_eq!(
                stylesheet
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::new(&token)
                    )
                    .unwrap(),
                ".bar{x:1}a{width:1px}"
            );
        });
    }

    #[test]
    fn reports_declaration_only_changes_without_structural_retirement() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut stylesheet = parse(
                "a{width:1px;height:1px}.bar{x:1}a{width:1px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let commit_pass = {
                let declaration_blocks = walk_declaration_blocks(&stylesheet, &token);
                let mut scanner = CrossRuleDeclarationScanner::new(declaration_blocks);
                scanner.discover_declaration_override_candidates();
                while let Some(candidate) = scanner.declaration_override_candidates.pop() {
                    scanner.handle_declaration_override_candidate(candidate);
                }
                scanner
                    .take_declaration_override_commit_pass()
                    .expect("the two a blocks share one exact-only S2 history")
            };

            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            assert_eq!(
                commit_pass.commit(&mut stylesheet, &mut minifier, &mut token, &mut cx),
                DeclarationOverrideCommitResult {
                    declarations_removed: true,
                    rules_retired: false,
                }
            );
            assert_eq!(
                stylesheet
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::new(&token)
                    )
                    .unwrap(),
                "a{height:1px}.bar{x:1}a{width:1px}"
            );
        });
    }
}
