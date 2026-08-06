mod declaration_ir;
mod partial_selector;
mod scheduler;

pub(crate) use scheduler::CrossRuleStats;

pub(crate) fn stabilize<'ast>(
    stylesheet: &mut rocketcss_ast::StyleSheet<'ast>,
    allocator: &rocketcss_common::Allocator,
    preserve_selector_compatibility: bool,
) -> Result<scheduler::CrossRuleStats, rocketcss_ast::StyleSheetMutationError<'ast>> {
    let (plan, mut stats) = {
        let scheduler = scheduler::CrossRuleScheduler::from_stylesheet(&*stylesheet, allocator)?;
        scheduler.stabilize(&*stylesheet, preserve_selector_compatibility)
    };
    let reification = stylesheet.apply_reification_plan(plan, allocator)?;
    stats.reification_passes = reification.reification_passes;
    stats.rule_tombstone_reuses = reification.rule_tombstone_reuses;
    stats.block_tombstone_reuses = reification.block_tombstone_reuses;
    stats.declaration_tombstone_reuses = reification.declaration_tombstone_reuses;
    stats.residual_rule_inserts = reification.residual_rule_inserts;
    stats.residual_declaration_inserts = reification.residual_declaration_inserts;
    stats.radix_relabel_groups = reification.radix_relabel_groups;
    Ok(stats)
}
