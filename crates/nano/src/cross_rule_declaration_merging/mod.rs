mod candidates;
mod declaration_override;
pub(crate) mod discovery;
mod live_sibling_graph;
mod same_selector;

use rocketcss_ast::{DeclarationBlockStore, RuleStore};
use rocketcss_common::define_dense_id;

use self::declaration_override::DeclarationOverrideCommitPass;
pub(crate) use self::discovery::DeclarationBlockDiscovery;
use self::discovery::DeclarationBlockEntries;
use self::live_sibling_graph::LiveSiblingGraph;
use crate::MinifyContext;
use crate::rules::DeclarationBlockMinifier;

define_dense_id!(pub(super) struct RuleId);

pub(crate) fn merge_cross_rule_declarations<'ast, 'scratch>(
    rules: &mut RuleStore<'ast>,
    declaration_blocks: &mut DeclarationBlockStore<'ast>,
    entries: DeclarationBlockEntries,
    declaration_block_minifier: &mut DeclarationBlockMinifier<'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> bool
where
    'ast: 'scratch,
{
    discovery::attach_effective_keys(&entries, declaration_blocks);
    let (mut live_sibling_graph, declaration_override_commit_pass) = {
        let mut live_sibling_graph = LiveSiblingGraph::new(&entries, declaration_blocks);
        live_sibling_graph.stabilize_same_selector_candidates();
        let declaration_override_commit_pass = DeclarationOverrideCommitPass::discover(&entries);
        (live_sibling_graph, declaration_override_commit_pass)
    };

    if let Some(commit_pass) = declaration_override_commit_pass {
        let result = commit_pass.commit(declaration_block_minifier, declaration_blocks, cx);
        for declarations in result.newly_empty {
            live_sibling_graph.declaration_block_became_empty(declarations);
        }
        live_sibling_graph.stabilize_same_selector_candidates();
    }

    live_sibling_graph.commit(rules, declaration_blocks)
}
