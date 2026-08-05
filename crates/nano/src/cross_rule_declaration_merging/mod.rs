mod declaration_ir;
mod partial_selector;
mod radix_state;

pub(crate) use radix_state::CrossRuleBuilder;

pub(crate) fn new_cross_rule_builder<'arena, 'ast>(
    stylesheet: &rocketcss_ast::StyleSheet<'ast>,
    allocator: &'arena rocketcss_common::Allocator,
) -> CrossRuleBuilder<'arena, 'ast> {
    radix_state::CrossRuleBuilder::new(stylesheet, allocator)
}

pub(crate) fn publish_cross_rule_block<'arena, 'ast>(
    builder: &mut CrossRuleBuilder<'arena, 'ast>,
    stylesheet: &rocketcss_ast::StyleSheet<'ast>,
    block: rocketcss_ast::CssDeclarationBlockId<'ast>,
) -> Result<(), rocketcss_ast::StyleSheetMutationError<'ast>> {
    builder.publish_block(stylesheet, block)
}

pub(crate) fn stabilize_cross_rule_builder<'arena, 'ast>(
    mut builder: CrossRuleBuilder<'arena, 'ast>,
    stylesheet: &mut rocketcss_ast::StyleSheet<'ast>,
    preserve_selector_compatibility: bool,
    key_remaps: &[rocketcss_ast::EffectiveKeyId],
) -> Result<(), rocketcss_ast::StyleSheetMutationError<'ast>> {
    builder.finalize(stylesheet, key_remaps)?;
    radix_state::stabilize_with_builder(builder, stylesheet, preserve_selector_compatibility)
}
