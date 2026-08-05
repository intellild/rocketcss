mod declaration_ir;
mod partial_selector;
mod radix_state;

pub(crate) use radix_state::CrossRuleBuilder;

pub(crate) fn new_cross_rule_builder<'arena, 'ast>(
    compilation: &rocketcss_ast::radix_ast::Compilation<'ast>,
    allocator: &'arena rocketcss_common::Allocator,
) -> CrossRuleBuilder<'arena, 'ast> {
    radix_state::CrossRuleBuilder::new(compilation, allocator)
}

pub(crate) fn publish_cross_rule_block<'arena, 'ast>(
    builder: &mut CrossRuleBuilder<'arena, 'ast>,
    compilation: &rocketcss_ast::radix_ast::Compilation<'ast>,
    block: rocketcss_ast::radix_ast::ConcreteDeclarationBlockId<'ast>,
) -> Result<(), rocketcss_ast::radix_ast::ConcreteMutationError<'ast>> {
    builder.publish_block(compilation, block)
}

pub(crate) fn stabilize_cross_rule_builder<'arena, 'ast>(
    mut builder: CrossRuleBuilder<'arena, 'ast>,
    compilation: &mut rocketcss_ast::radix_ast::Compilation<'ast>,
    preserve_selector_compatibility: bool,
    key_remaps: &[rocketcss_ast::radix_ast::EffectiveKeyId],
) -> Result<(), rocketcss_ast::radix_ast::ConcreteMutationError<'ast>> {
    builder.finalize(key_remaps);
    radix_state::stabilize_with_builder(builder, compilation, preserve_selector_compatibility)
}
