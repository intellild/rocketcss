mod declaration_ir;
mod partial_selector;
mod radix_state;

pub(crate) fn stabilize_cross_rule_declarations<'ast>(
    compilation: &mut rocketcss_ast::radix_ast::Compilation<'ast>,
    preserve_selector_compatibility: bool,
) -> Result<(), rocketcss_ast::radix_ast::MutationError> {
    radix_state::stabilize(compilation, preserve_selector_compatibility)
}
