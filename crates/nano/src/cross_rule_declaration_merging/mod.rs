mod declaration_ir;
mod partial_selector;
mod state;

pub(crate) use state::CrossRuleBuilder;

pub(crate) fn new_cross_rule_builder<'scratch, 'ast>(
    compilation: &rocketcss_ast::AstContext<'ast>,
    allocator: &'scratch rocketcss_common::Allocator,
) -> CrossRuleBuilder<'scratch, 'ast> {
    state::CrossRuleBuilder::new(compilation, allocator)
}

pub(crate) fn publish_cross_rule_block<'scratch, 'ast>(
    builder: &mut CrossRuleBuilder<'scratch, 'ast>,
    compilation: &rocketcss_ast::AstContext<'ast>,
    block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
) -> Result<(), rocketcss_ast::ConcreteMutationError<'ast>> {
    builder.publish_block(compilation, block)
}

pub(crate) fn stabilize_cross_rule_builder<'scratch, 'ast>(
    mut builder: CrossRuleBuilder<'scratch, 'ast>,
    compilation: &mut rocketcss_ast::AstContext<'ast>,
    preserve_selector_compatibility: bool,
    key_remaps: &[rocketcss_ast::EffectiveKeyId<'ast>],
) -> Result<
    std::vec::Vec<rocketcss_ast::ConcreteDeclarationBlockId<'ast>>,
    rocketcss_ast::ConcreteMutationError<'ast>,
> {
    builder.finalize(key_remaps);
    state::stabilize_with_builder(builder, compilation, preserve_selector_compatibility)
}
