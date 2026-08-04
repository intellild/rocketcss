mod declaration_ir;
mod partial_selector;
mod scheduler;

use rocketcss_ast::DeclarationBlockStore;

pub(crate) use self::declaration_ir::FrozenDeclarationIrStore;
use self::scheduler::CrossRuleMergeState;
use crate::rules::DeclarationBlockMinifier;
use crate::utils::DeclarationBlockDiscovery;
use crate::{MinifyContext, Options, OptionsOp};

pub(crate) fn stabilize_cross_rule_declarations<'walk, 'ast, 'scratch>(
    discovery: DeclarationBlockDiscovery<'walk, 'ast>,
    declaration_blocks: &mut DeclarationBlockStore<'ast>,
    declaration_ir: FrozenDeclarationIrStore<'ast>,
    declaration_block_minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> scheduler::ReificationPlan<'ast>
where
    'ast: 'scratch,
{
    debug_assert!(cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any));

    let mut state = CrossRuleMergeState::new(discovery, declaration_blocks, declaration_ir);
    state.run(declaration_blocks, declaration_block_minifier, cx);
    state.into_reification_plan()
}
