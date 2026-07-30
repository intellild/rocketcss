mod candidates;
mod declaration_override;
mod live_sibling_graph;
mod same_selector;

use rocketcss_allocator::{GhostToken, vec::Vec};
use rocketcss_ast::{CssRule, StyleSheet};

use self::declaration_override::DeclarationOverrideCommitPass;
use self::live_sibling_graph::LiveSiblingGraph;
use crate::rules::DeclarationBlockMinifier;
use crate::utils::walk_declaration_blocks;
use crate::{MinifyContext, Options, OptionsOp};

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

    let (mut live_sibling_graph, declaration_override_commit_pass) = {
        let declaration_blocks = walk_declaration_blocks(stylesheet, token);
        let mut live_sibling_graph = LiveSiblingGraph::new(&declaration_blocks, token);
        live_sibling_graph.stabilize_same_selector_candidates();
        let declaration_override_commit_pass =
            DeclarationOverrideCommitPass::discover(&declaration_blocks);
        (live_sibling_graph, declaration_override_commit_pass)
    };

    if let Some(commit_pass) = declaration_override_commit_pass {
        let result = commit_pass.commit(declaration_block_minifier, token, cx);
        for declarations in result.newly_empty {
            live_sibling_graph.declaration_block_became_empty(declarations);
        }
        live_sibling_graph.stabilize_same_selector_candidates();
    }

    if live_sibling_graph.commit(stylesheet, token) {
        compact_retired_style_rules(&mut stylesheet.rules);
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
