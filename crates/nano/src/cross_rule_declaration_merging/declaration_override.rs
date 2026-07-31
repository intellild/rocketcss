use std::num::NonZeroU32;

use rocketcss_ast::{DeclarationBlockId, DeclarationBlockStore};
use rustc_hash::FxHashSet;

use crate::MinifyContext;
use crate::rules::DeclarationBlockMinifier;
use crate::utils::DeclarationBlockEntry;

pub(super) struct DeclarationOverrideCommitPass {
    blocks: std::vec::Vec<DeclarationBlockId>,
    next_in_history: std::vec::Vec<Option<NonZeroU32>>,
    history_heads: std::vec::Vec<u32>,
}

#[derive(Debug, Default)]
pub(super) struct DeclarationOverrideCommitResult {
    pub(super) newly_empty: FxHashSet<DeclarationBlockId>,
}

impl DeclarationOverrideCommitPass {
    pub(super) fn discover(declaration_blocks: &[DeclarationBlockEntry]) -> Option<Self> {
        let history_count = declaration_blocks
            .iter()
            .filter(|entry| entry.starts_declaration_history())
            .count();
        if history_count == 0 {
            return None;
        }

        let mut blocks = std::vec::Vec::new();
        let mut next_in_history = std::vec::Vec::new();
        let mut history_heads = std::vec::Vec::with_capacity(history_count);
        for (source_head, entry) in declaration_blocks.iter().enumerate() {
            if !entry.starts_declaration_history() {
                continue;
            }

            history_heads.push(
                u32::try_from(blocks.len()).expect("declaration history count exceeds u32::MAX"),
            );
            let mut current = Some(source_head);
            while let Some(source_index) = current {
                let entry = &declaration_blocks[source_index];
                let output_index = blocks.len();
                blocks.push(entry.declarations);
                next_in_history.push(None);
                current = entry.next_declaration_history_entry();
                if current.is_some() {
                    let next_output = u32::try_from(blocks.len())
                        .expect("declaration history count exceeds u32::MAX");
                    next_in_history[output_index] = Some(encode_history_link(next_output));
                }
            }
        }

        Some(Self {
            blocks,
            next_in_history,
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
            while let Some(block) = current {
                let block = usize::try_from(block).expect("declaration block index fits usize");
                let declarations = self.blocks[block];
                append_declaration_chain(declarations, store, &mut seen, &mut expanded_history);
                current = self.next_in_history[block].map(|next| next.get() - 1);
            }

            minifier.deduplicate_exact_sequence(&expanded_history, store, cx, |declarations| {
                newly_empty.insert(declarations);
            });
        }
        DeclarationOverrideCommitResult { newly_empty }
    }
}

fn encode_history_link(index: u32) -> NonZeroU32 {
    NonZeroU32::new(
        index
            .checked_add(1)
            .expect("declaration history count exceeds u32::MAX"),
    )
    .expect("encoded declaration history index is non-zero")
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
    use rocketcss_allocator::Allocator;
    use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
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
            while let Some(block) = current {
                history_len += 1;
                current = pass.next_in_history
                    [usize::try_from(block).expect("declaration block index fits usize")]
                .map(|next| next.get() - 1);
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
