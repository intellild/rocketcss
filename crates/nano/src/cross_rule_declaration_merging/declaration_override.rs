use std::num::NonZeroU32;

use rocketcss_allocator::{GhostToken, Ref};
use rocketcss_ast::DeclarationBlock;
use rustc_hash::FxHashSet;

use super::effective_key::EffectiveKeyId;
use crate::MinifyContext;
use crate::rules::DeclarationBlockMinifier;
use crate::utils::DeclarationBlockEntry;

pub(super) struct DeclarationOverrideCommitPass<'ast, 'ghost> {
    blocks: std::vec::Vec<Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>>,
    next_in_history: std::vec::Vec<Option<NonZeroU32>>,
    history_heads: std::vec::Vec<u32>,
}

#[derive(Debug, Default)]
pub(super) struct DeclarationOverrideCommitResult<'ast, 'ghost> {
    pub(super) newly_empty: FxHashSet<*const DeclarationBlock<'ast, 'ghost>>,
}

impl<'ast, 'ghost> DeclarationOverrideCommitPass<'ast, 'ghost> {
    pub(super) fn discover(
        declaration_blocks: &[DeclarationBlockEntry<'_, 'ast, 'ghost>],
        effective_keys: &[EffectiveKeyId],
        effective_key_count: usize,
    ) -> Option<Self> {
        debug_assert_eq!(declaration_blocks.len(), effective_keys.len());
        let mut counts = std::vec![0_u32; effective_key_count];
        for &effective_key in effective_keys {
            let count = &mut counts[effective_key.index()];
            *count = count
                .checked_add(1)
                .expect("declaration history length exceeds u32::MAX");
        }
        let repeated_block_count = effective_keys
            .iter()
            .filter(|effective_key| counts[effective_key.index()] > 1)
            .count();
        if repeated_block_count == 0 {
            return None;
        }

        let mut blocks = std::vec::Vec::with_capacity(repeated_block_count);
        let mut next_in_history = std::vec::Vec::with_capacity(repeated_block_count);
        let mut history_heads_by_key = std::vec![None; effective_key_count];
        let mut history_tails_by_key = std::vec![None; effective_key_count];
        for (entry, &effective_key) in declaration_blocks.iter().zip(effective_keys) {
            let key = effective_key.index();
            if counts[key] == 1 {
                continue;
            }
            let current =
                u32::try_from(blocks.len()).expect("declaration history count exceeds u32::MAX");
            let current_link = encode_history_link(current);
            blocks.push(entry.declaration_ref);
            next_in_history.push(None);
            if let Some(previous) = history_tails_by_key[key] {
                next_in_history[decode_history_link(previous)] = Some(current_link);
            } else {
                history_heads_by_key[key] = Some(current_link);
            }
            history_tails_by_key[key] = Some(current_link);
        }

        Some(Self {
            blocks,
            next_in_history,
            history_heads: history_heads_by_key
                .into_iter()
                .flatten()
                .map(|head| head.get() - 1)
                .collect(),
        })
    }

    pub(super) fn commit<'scratch>(
        &self,
        minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
        token: &mut GhostToken<'ghost>,
        cx: &mut MinifyContext<'scratch>,
    ) -> DeclarationOverrideCommitResult<'ast, 'ghost>
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
                append_declaration_chain(declarations, token, &mut seen, &mut expanded_history);
                current = self.next_in_history[block].map(|next| next.get() - 1);
            }

            minifier.deduplicate_exact_sequence(&expanded_history, token, cx, |declarations| {
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

fn decode_history_link(link: NonZeroU32) -> usize {
    usize::try_from(link.get() - 1).expect("declaration history index fits usize")
}

fn append_declaration_chain<'ast, 'ghost>(
    declarations: Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
    token: &GhostToken<'ghost>,
    seen: &mut FxHashSet<*const DeclarationBlock<'ast, 'ghost>>,
    output: &mut std::vec::Vec<Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>>,
) {
    let declarations_ptr = std::ptr::from_ref(declarations.get(token).get_ref());
    if !seen.insert(declarations_ptr) {
        return;
    }
    if let Some(previous) = declarations.get(token).previous_merged() {
        append_declaration_chain(previous, token, seen, output);
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
    use crate::cross_rule_declaration_merging::effective_key::intern_effective_keys;
    use crate::utils::walk_declaration_blocks;

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
            let declaration_blocks = walk_declaration_blocks(&stylesheet, &token);
            let (effective_keys, effective_key_count) = intern_effective_keys(&declaration_blocks);

            let pass = DeclarationOverrideCommitPass::discover(
                &declaration_blocks,
                &effective_keys,
                effective_key_count,
            )
            .expect("both selectors have declaration histories");

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
            let stylesheet = parse(
                "a{width:1px}.bar{x:1}a{width:1px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let pass = {
                let declaration_blocks = walk_declaration_blocks(&stylesheet, &token);
                let (effective_keys, effective_key_count) =
                    intern_effective_keys(&declaration_blocks);
                DeclarationOverrideCommitPass::discover(
                    &declaration_blocks,
                    &effective_keys,
                    effective_key_count,
                )
                .expect("the two a blocks share one exact-only S2 history")
            };

            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let result = pass.commit(&mut minifier, &mut token, &mut cx);
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
            let stylesheet = parse(
                "a{width:1px;height:1px}.bar{x:1}a{width:1px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let pass = {
                let declaration_blocks = walk_declaration_blocks(&stylesheet, &token);
                let (effective_keys, effective_key_count) =
                    intern_effective_keys(&declaration_blocks);
                DeclarationOverrideCommitPass::discover(
                    &declaration_blocks,
                    &effective_keys,
                    effective_key_count,
                )
                .expect("the two a blocks share one exact-only S2 history")
            };

            let scratch = Allocator::new();
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let result = pass.commit(&mut minifier, &mut token, &mut cx);
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
