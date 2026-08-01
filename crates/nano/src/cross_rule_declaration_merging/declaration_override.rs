use rocketcss_ast::{DeclarationBlockId, DeclarationBlockStore};
use rocketcss_common::{DenseStore, define_dense_id};
use rustc_hash::FxHashSet;

use crate::MinifyContext;
use crate::rules::DeclarationBlockMinifier;
use crate::utils::DeclarationBlockEntries;

define_dense_id!(struct OverrideEntryId);

struct OverrideEntry {
    block: DeclarationBlockId,
    next: Option<OverrideEntryId>,
}

pub(super) struct DeclarationOverrideCommitPass {
    entries: DenseStore<OverrideEntryId, OverrideEntry>,
    history_heads: std::vec::Vec<OverrideEntryId>,
}

#[derive(Debug, Default)]
pub(super) struct DeclarationOverrideCommitResult {
    pub(super) newly_empty: FxHashSet<DeclarationBlockId>,
}

impl DeclarationOverrideCommitPass {
    pub(super) fn discover(declaration_blocks: &DeclarationBlockEntries) -> Option<Self> {
        let history_count = declaration_blocks
            .iter()
            .filter(|entry| entry.starts_declaration_history())
            .count();
        if history_count == 0 {
            return None;
        }

        let mut entries = DenseStore::new();
        let mut history_heads = std::vec::Vec::with_capacity(history_count);
        for (source_head, entry) in declaration_blocks.iter_enumerated() {
            if !entry.starts_declaration_history() {
                continue;
            }

            history_heads.push(entries.next_id());
            let mut current = Some(source_head);
            while let Some(source_id) = current {
                let entry = &declaration_blocks[source_id];
                let output_id = entries.push(OverrideEntry {
                    block: entry.declarations,
                    next: None,
                });
                current = entry.next_declaration_history_entry();
                if current.is_some() {
                    entries[output_id].next = Some(entries.next_id());
                }
            }
        }

        Some(Self {
            entries,
            history_heads,
        })
    }

    pub(super) fn commit<'ast, 'scratch>(
        &self,
        minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
        store: &mut DeclarationBlockStore<'ast>,
        cx: &mut MinifyContext<'scratch>,
    ) -> DeclarationOverrideCommitResult
    where
        'ast: 'scratch,
    {
        let mut newly_empty = FxHashSet::default();
        let mut expanded_history = std::vec::Vec::new();
        let mut seen = FxHashSet::default();
        for &history_head in &self.history_heads {
            expanded_history.clear();
            seen.clear();
            let mut current = Some(history_head);
            while let Some(entry_id) = current {
                let entry = &self.entries[entry_id];
                let declarations = entry.block;
                append_declaration_chain(declarations, store, &mut seen, &mut expanded_history);
                current = entry.next;
            }

            minifier.deduplicate_exact_sequence(&expanded_history, store, cx, |declarations| {
                newly_empty.insert(declarations);
            });
        }
        DeclarationOverrideCommitResult { newly_empty }
    }
}

fn append_declaration_chain(
    declarations: DeclarationBlockId,
    store: &DeclarationBlockStore<'_>,
    seen: &mut FxHashSet<DeclarationBlockId>,
    output: &mut std::vec::Vec<DeclarationBlockId>,
) {
    if !seen.insert(declarations) {
        return;
    }
    if let Some(previous) = store.get(declarations).previous_merged() {
        append_declaration_chain(previous, store, seen, output);
    }
    output.push(declarations);
}

#[cfg(test)]
mod tests {
    use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
    use rocketcss_common::Allocator;
    use rocketcss_parser::{ParserOptions, parse};

    use super::*;
    use crate::MinifyOptions;
    use crate::utils::walk_declaration_blocks;

    #[test]
    fn does_not_materialize_unique_histories() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                "a{x:1}b{x:2}c{x:3}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let declaration_blocks = walk_declaration_blocks(&stylesheet);

            assert!(DeclarationOverrideCommitPass::discover(&declaration_blocks).is_none());
        });
    }

    #[test]
    fn discovers_s2_history_in_source_order() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                "a{x:1}.bar-1{y:1}a{x:1}.bar-2{y:1}a{x:1}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let declaration_blocks = walk_declaration_blocks(&stylesheet);

            let pass = DeclarationOverrideCommitPass::discover(&declaration_blocks)
                .expect("the a selector has a declaration history");

            assert_eq!(pass.history_heads.len(), 1);
            let mut history_len = 0;
            let mut current = Some(pass.history_heads[0]);
            while let Some(entry_id) = current {
                history_len += 1;
                current = pass.entries[entry_id].next;
            }
            assert_eq!(history_len, 3);
        });
    }

    #[test]
    fn reports_newly_empty_blocks_to_the_live_graph() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut stylesheet = parse(
                "a{width:1px}.bar{x:1}a{width:1px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let pass = {
                let declaration_blocks = walk_declaration_blocks(&stylesheet);
                DeclarationOverrideCommitPass::discover(&declaration_blocks)
                    .expect("the two a blocks share one exact-only S2 history")
            };

            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let result = pass.commit(&mut minifier, stylesheet.parts_mut().1, &mut cx);
            assert_eq!(cx.stats().declarations_removed, 1);
            assert_eq!(result.newly_empty.len(), 1);
            assert_eq!(
                stylesheet
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::new(&token)
                    )
                    .unwrap(),
                "a{}.bar{x:1}a{width:1px}"
            );
        });
    }

    #[test]
    fn reports_declaration_only_changes_without_an_empty_block() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut stylesheet = parse(
                "a{width:1px;height:1px}.bar{x:1}a{width:1px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let pass = {
                let declaration_blocks = walk_declaration_blocks(&stylesheet);
                DeclarationOverrideCommitPass::discover(&declaration_blocks)
                    .expect("the two a blocks share one exact-only S2 history")
            };

            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let result = pass.commit(&mut minifier, stylesheet.parts_mut().1, &mut cx);
            assert_eq!(cx.stats().declarations_removed, 1);
            assert!(result.newly_empty.is_empty());
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
