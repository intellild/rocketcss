//! Terminal application of cross-rule transformation plans.
//!
//! Semantic schedulers own logical IDs and decisions. This module owns the
//! only conversion of those decisions into Radix rules, blocks, declarations,
//! selector values, and effective keys.

use std::collections::BTreeMap;

use rocketcss_common::{Allocator, RadixIdKey, RadixIdRemap};
use rustc_hash::FxHashMap;

use crate::Span;

use super::*;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReificationRuleId(pub u32);

impl ReificationRuleId {
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReificationDeclarationId(pub u32);

impl ReificationDeclarationId {
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReificationSelectorId(pub u32);

impl ReificationSelectorId {
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReificationEffectiveKey {
    Existing(EffectiveKeyId),
    Synthesized(u32),
}

#[doc(hidden)]
#[derive(Debug)]
pub struct PendingReificationEffectiveKey {
    pub representative: EffectiveKeyId,
    pub selector: ReificationSelectorId,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FinalReificationSelector {
    Existing(SelectorValueId),
    Synthesized(ReificationSelectorId),
}

/// Final state of one rule participating in an affected direct rule list.
#[doc(hidden)]
#[derive(Debug)]
pub struct FinalReificationRule<'ast> {
    pub logical_id: ReificationRuleId,
    pub source_rule: Option<CssRuleId<'ast>>,
    pub source_block: Option<CssDeclarationBlockId<'ast>>,
    pub parent: Option<ReificationRuleId>,
    pub live: bool,
    pub effective_key: Option<ReificationEffectiveKey>,
    pub selector: Option<FinalReificationSelector>,
    pub span: Option<Span>,
    pub declarations: std::vec::Vec<ReificationDeclarationId>,
}

/// Final semantic order for one affected direct rule list.
#[doc(hidden)]
#[derive(Debug)]
pub struct FinalReificationRuleList {
    pub parent: Option<ReificationRuleId>,
    pub rules: std::vec::Vec<ReificationRuleId>,
}

#[doc(hidden)]
#[derive(Debug)]
pub enum ReificationStep {
    MergeAdjacent {
        left: ReificationRuleId,
        right: ReificationRuleId,
    },
    RemoveDeclaration {
        owner: ReificationRuleId,
        declaration: ReificationDeclarationId,
    },
    RetireRule {
        rule: ReificationRuleId,
    },
    ReuseLeftForPartialMerge {
        left: ReificationRuleId,
        right: ReificationRuleId,
        selector: ReificationSelectorId,
        effective_key: ReificationEffectiveKey,
        right_removed: std::vec::Vec<ReificationDeclarationId>,
        retire_right: bool,
        span: Span,
    },
    InsertPartialMerge {
        left: ReificationRuleId,
        right: ReificationRuleId,
        synthesized: ReificationRuleId,
        selector: ReificationSelectorId,
        effective_key: ReificationEffectiveKey,
        declarations: std::vec::Vec<(
            ReificationDeclarationId,
            ReificationDeclarationId,
            ReificationDeclarationId,
        )>,
        retire_right: bool,
    },
}

#[doc(hidden)]
pub struct ReificationPlan<'ast> {
    pub source_rules: std::vec::Vec<Option<CssRuleId<'ast>>>,
    pub source_blocks: std::vec::Vec<Option<CssDeclarationBlockId<'ast>>>,
    pub rule_orders: std::vec::Vec<std::vec::Vec<u32>>,
    pub declaration_sources: std::vec::Vec<DeclarationId>,
    pub selectors: std::vec::Vec<crate::SelectorList<'ast>>,
    pub pending_keys: std::vec::Vec<PendingReificationEffectiveKey>,
    pub final_rules: std::vec::Vec<FinalReificationRule<'ast>>,
    pub final_rule_lists: std::vec::Vec<FinalReificationRuleList>,
    pub steps: std::vec::Vec<ReificationStep>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReificationStats {
    pub reification_passes: u32,
    pub rule_tombstone_reuses: u32,
    pub block_tombstone_reuses: u32,
    pub declaration_tombstone_reuses: u32,
    pub residual_rule_inserts: u32,
    pub residual_declaration_inserts: u32,
    pub radix_relabel_groups: u32,
}

struct CompatibleRuleTombstone<'ast> {
    rule: CssRuleId<'ast>,
    block: CssDeclarationBlockId<'ast>,
    declarations: std::vec::Vec<DeclarationId>,
    append: DeclarationAppendContext<CssRule<'ast>>,
}

impl<'ast> StyleSheet<'ast> {
    /// Applies one complete transform-local plan after its scheduler has been
    /// dropped. No semantic choice is made beyond the plan boundary.
    pub fn apply_reification_plan(
        &mut self,
        mut plan: ReificationPlan<'ast>,
        allocator: &Allocator,
    ) -> Result<ReificationStats, StyleSheetMutationError<'ast>> {
        let mut stats = ReificationStats::default();
        if plan.steps.is_empty() {
            return Ok(stats);
        }
        stats.reification_passes = 1;
        let mut rules = std::mem::take(&mut plan.source_rules);
        let mut blocks = std::mem::take(&mut plan.source_blocks);
        let mut declarations = plan
            .declaration_sources
            .iter()
            .copied()
            .map(Some)
            .collect::<std::vec::Vec<_>>();
        let mut selector_ids = std::vec::Vec::with_capacity(plan.selectors.len());
        let mut key_ids = std::vec::Vec::with_capacity(plan.pending_keys.len());
        let all_synthesized = plan
            .steps
            .iter()
            .filter_map(|step| match step {
                ReificationStep::InsertPartialMerge { synthesized, .. } => Some(*synthesized),
                _ => None,
            })
            .collect::<std::vec::Vec<_>>();
        let mut reserved_rules = std::vec![None; plan.rule_orders.len()];
        let mut reserved_blocks = std::vec![None; plan.rule_orders.len()];

        for step in std::mem::take(&mut plan.steps) {
            match step {
                ReificationStep::MergeAdjacent { left, right } => {
                    let edge = find_edge(self, &rules, left, right)?;
                    let merged = self.merge_adjacent_rule_declaration_blocks(edge)?;
                    blocks[right.index()] = Some(merged.retained_block);
                }
                ReificationStep::RemoveDeclaration { owner, declaration } => {
                    let block = blocks[owner.index()].ok_or_else(|| missing_rule(self, &rules))?;
                    let actual = declarations[declaration.index()].ok_or(
                        StyleSheetMutationError::UnknownDeclaration(
                            plan.declaration_sources[declaration.index()],
                        ),
                    )?;
                    self.replace_declaration(
                        block,
                        actual,
                        CssDeclaration::Property(crate::Declaration::Tombstone),
                    )?;
                }
                ReificationStep::RetireRule { rule } => {
                    let context = find_context(self, &rules, rule)?;
                    self.retire_rule(context)?;
                }
                ReificationStep::ReuseLeftForPartialMerge {
                    left,
                    right,
                    selector,
                    effective_key,
                    right_removed,
                    retire_right,
                    span,
                } => {
                    let kind = style_kind(self, &rules, left)?;
                    let prefix = style_prefix(self, &rules, left)?;
                    let selector = intern_selector(
                        self,
                        &mut plan.selectors,
                        &mut selector_ids,
                        selector,
                        kind,
                        prefix,
                    )?;
                    let _ = resolve_key(
                        self,
                        &plan.pending_keys,
                        &mut key_ids,
                        &selector_ids,
                        effective_key,
                    )?;
                    let edge = find_edge(self, &rules, left, right)?;
                    self.replace_rule_selector_value_in_edge(edge, selector, allocator)?;
                    let left_rule =
                        rules[left.index()].ok_or_else(|| missing_rule(self, &rules))?;
                    set_rule_span(self, left_rule, span)?;
                    let right_block =
                        blocks[right.index()].ok_or_else(|| missing_rule(self, &rules))?;
                    for declaration in right_removed {
                        let actual = declarations[declaration.index()].ok_or(
                            StyleSheetMutationError::UnknownDeclaration(
                                plan.declaration_sources[declaration.index()],
                            ),
                        )?;
                        self.replace_declaration(
                            right_block,
                            actual,
                            CssDeclaration::Property(crate::Declaration::Tombstone),
                        )?;
                    }
                    if retire_right {
                        let context = find_context(self, &rules, right)?;
                        self.retire_rule(context)?;
                    }
                }
                ReificationStep::InsertPartialMerge {
                    left,
                    right,
                    synthesized,
                    selector,
                    effective_key,
                    declarations: moved,
                    retire_right,
                } => {
                    let kind = style_kind(self, &rules, left)?;
                    let prefix = style_prefix(self, &rules, left)?;
                    let selector_id = intern_selector(
                        self,
                        &mut plan.selectors,
                        &mut selector_ids,
                        selector,
                        kind,
                        prefix,
                    )?;
                    let key = resolve_key(
                        self,
                        &plan.pending_keys,
                        &mut key_ids,
                        &selector_ids,
                        effective_key,
                    )?;
                    let edge = find_edge(self, &rules, left, right)?;
                    let left_block =
                        blocks[left.index()].ok_or_else(|| missing_rule(self, &rules))?;
                    let append = self.declaration_append_context(left_block)?;
                    let span = Span::new(
                        rule_span(self, rules[left.index()].unwrap())?.start,
                        rule_span(self, rules[right.index()].unwrap())?.end,
                    );
                    let payload = match kind {
                        SelectorFrameKind::Style => CssRule::Style(crate::StyleRule {
                            span,
                            selector_value: selector_id,
                            vendor_prefix: prefix,
                        }),
                        SelectorFrameKind::Nesting => CssRule::Nesting(crate::NestingRule {
                            span,
                            selector_value: selector_id,
                        }),
                    };
                    if let Some(CompatibleRuleTombstone {
                        rule: reused_rule,
                        block: reused_block,
                        declarations: reused_declarations,
                        append,
                    }) = self.compatible_rule_tombstone(edge, moved.len())?
                    {
                        let left_block = blocks[left.index()].unwrap();
                        let right_block = blocks[right.index()].unwrap();
                        let mut payloads = std::vec::Vec::with_capacity(moved.len());
                        for &(_, left_declaration, right_declaration) in &moved {
                            let left_actual = declarations[left_declaration.index()].ok_or(
                                StyleSheetMutationError::UnknownDeclaration(
                                    plan.declaration_sources[left_declaration.index()],
                                ),
                            )?;
                            let right_actual = declarations[right_declaration.index()].ok_or(
                                StyleSheetMutationError::UnknownDeclaration(
                                    plan.declaration_sources[right_declaration.index()],
                                ),
                            )?;
                            let important = self
                                .declaration(left_actual)
                                .ok_or(StyleSheetMutationError::UnknownDeclaration(left_actual))?
                                .is_important();
                            let payload = self.replace_declaration(
                                left_block,
                                left_actual,
                                CssDeclaration::Property(crate::Declaration::Tombstone),
                            )?;
                            self.replace_declaration(
                                right_block,
                                right_actual,
                                CssDeclaration::Property(crate::Declaration::Tombstone),
                            )?;
                            payloads.push((payload, important));
                        }
                        let residual_payloads = self.reactivate_rule_tombstone(
                            reused_rule,
                            reused_block,
                            &reused_declarations,
                            payload,
                            key,
                            payloads,
                        )?;
                        let residual_count = residual_payloads.len();
                        if !residual_payloads.is_empty() {
                            self.insert_transformed_declarations_with_context(
                                append,
                                residual_payloads,
                            )?;
                        }
                        let final_declarations = self
                            .declaration_ids_in_block(reused_block)?
                            .take(moved.len())
                            .collect::<std::vec::Vec<_>>();
                        debug_assert_eq!(final_declarations.len(), moved.len());
                        rules.resize(rules.len().max(synthesized.index() + 1), None);
                        blocks.resize(blocks.len().max(synthesized.index() + 1), None);
                        rules[synthesized.index()] = Some(reused_rule);
                        blocks[synthesized.index()] = Some(reused_block);
                        let reused_declaration_count = reused_declarations.len();
                        declarations.resize(
                            declarations.len().max(
                                moved
                                    .iter()
                                    .map(|(id, _, _)| id.index() + 1)
                                    .max()
                                    .unwrap_or(0),
                            ),
                            None,
                        );
                        for ((logical, _, _), actual) in moved.into_iter().zip(final_declarations) {
                            declarations[logical.index()] = Some(actual);
                        }
                        stats.rule_tombstone_reuses += 1;
                        stats.block_tombstone_reuses += 1;
                        stats.declaration_tombstone_reuses +=
                            u32::try_from(reused_declaration_count).unwrap_or(u32::MAX);
                        stats.residual_declaration_inserts +=
                            u32::try_from(residual_count).unwrap_or(u32::MAX);
                        if retire_right {
                            let context = find_context(self, &rules, right)?;
                            self.retire_rule(context)?;
                        }
                        continue;
                    }
                    if reserved_rules[synthesized.index()].is_none() {
                        let pending = all_synthesized
                            .iter()
                            .copied()
                            .filter(|logical| rules[logical.index()].is_none())
                            .collect::<std::vec::Vec<_>>();
                        reserved_rules = self.reserve_reification_rule_slots(
                            &plan, &pending, &mut rules, &mut stats,
                        )?;
                        reserved_blocks = self.reserve_reification_block_slots(
                            &plan,
                            &pending,
                            &mut blocks,
                            &mut stats,
                        )?;
                    }
                    let reserved_rule = reserved_rules[synthesized.index()].ok_or(
                        StyleSheetMutationError::LocalRuleCapacityExhausted(edge.left()),
                    )?;
                    let reserved_block = reserved_blocks[synthesized.index()].ok_or(
                        StyleSheetMutationError::LocalDeclarationBlockCapacityExhausted(
                            append.block(),
                        ),
                    )?;
                    self.activate_reserved_rule_with_block(
                        edge,
                        append,
                        reserved_rule,
                        reserved_block,
                        payload,
                        key,
                    )?;
                    rules.resize(rules.len().max(synthesized.index() + 1), None);
                    blocks.resize(blocks.len().max(synthesized.index() + 1), None);
                    rules[synthesized.index()] = Some(reserved_rule);
                    blocks[synthesized.index()] = Some(reserved_block);

                    let left_block = blocks[left.index()].unwrap();
                    let right_block = blocks[right.index()].unwrap();
                    let mut payloads = std::vec::Vec::with_capacity(moved.len());
                    for &(_, left_declaration, right_declaration) in &moved {
                        let left_actual = declarations[left_declaration.index()].ok_or(
                            StyleSheetMutationError::UnknownDeclaration(
                                plan.declaration_sources[left_declaration.index()],
                            ),
                        )?;
                        let right_actual = declarations[right_declaration.index()].ok_or(
                            StyleSheetMutationError::UnknownDeclaration(
                                plan.declaration_sources[right_declaration.index()],
                            ),
                        )?;
                        let important = self
                            .declaration(left_actual)
                            .ok_or(StyleSheetMutationError::UnknownDeclaration(left_actual))?
                            .is_important();
                        let payload = self.replace_declaration(
                            left_block,
                            left_actual,
                            CssDeclaration::Property(crate::Declaration::Tombstone),
                        )?;
                        self.replace_declaration(
                            right_block,
                            right_actual,
                            CssDeclaration::Property(crate::Declaration::Tombstone),
                        )?;
                        payloads.push((payload, important));
                    }
                    self.insert_transformed_declarations_at_block_end(reserved_block, payloads)?;
                    stats.residual_rule_inserts += 1;
                    stats.residual_declaration_inserts +=
                        u32::try_from(moved.len()).unwrap_or(u32::MAX);
                    let actual = self
                        .declaration_ids_in_block(reserved_block)?
                        .collect::<std::vec::Vec<_>>();
                    debug_assert_eq!(actual.len(), moved.len());
                    declarations.resize(
                        declarations.len().max(
                            moved
                                .iter()
                                .map(|(id, _, _)| id.index() + 1)
                                .max()
                                .unwrap_or(0),
                        ),
                        None,
                    );
                    for ((logical, _, _), actual) in moved.into_iter().zip(actual) {
                        declarations[logical.index()] = Some(actual);
                    }
                    if retire_right {
                        let context = find_context(self, &rules, right)?;
                        self.retire_rule(context)?;
                    }
                }
            }
        }
        self.validate_final_reification(
            &plan,
            &rules,
            &blocks,
            &declarations,
            &selector_ids,
            &key_ids,
        )?;
        debug_assert_eq!(self.validate_ast(), Ok(()));
        Ok(stats)
    }

    fn reserve_reification_rule_slots(
        &mut self,
        plan: &ReificationPlan<'ast>,
        synthesized: &[ReificationRuleId],
        rules: &mut [Option<CssRuleId<'ast>>],
        stats: &mut ReificationStats,
    ) -> Result<std::vec::Vec<Option<CssRuleId<'ast>>>, StyleSheetMutationError<'ast>> {
        let physical = self
            .rules
            .iter_enumerated()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>();
        let groups = reservation_groups(&physical, rules, &plan.rule_orders, synthesized)
            .ok_or_else(|| missing_rule(self, rules))?;
        let mut reserved = std::vec![None; plan.rule_orders.len()];
        for (primary, placements) in groups {
            let positions = placements
                .iter()
                .map(|(_, position)| *position)
                .collect::<std::vec::Vec<_>>();
            let reservation = self
                .rules
                .reserve_sibling_positions(primary, &positions)
                .ok_or(StyleSheetMutationError::LocalRuleCapacityExhausted(primary))?;
            if !reservation.remaps.is_empty() {
                stats.radix_relabel_groups += 1;
                self.repair_rule_id_remaps(&reservation.remaps);
                remap_rule_slots(rules, &reservation.remaps);
            }
            for ((logical, _), actual) in placements.into_iter().zip(reservation.reserved) {
                reserved[logical.index()] = Some(actual);
            }
        }
        Ok(reserved)
    }

    fn reserve_reification_block_slots(
        &mut self,
        plan: &ReificationPlan<'ast>,
        synthesized: &[ReificationRuleId],
        blocks: &mut [Option<CssDeclarationBlockId<'ast>>],
        stats: &mut ReificationStats,
    ) -> Result<std::vec::Vec<Option<CssDeclarationBlockId<'ast>>>, StyleSheetMutationError<'ast>>
    {
        let physical = self
            .declaration_blocks
            .iter_enumerated()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>();
        let groups = reservation_groups(&physical, blocks, &plan.rule_orders, synthesized)
            .ok_or_else(|| {
                self.root_rules()
                    .next()
                    .map(|(rule, _)| StyleSheetMutationError::InvalidRuleTopology(rule))
                    .expect("a reification plan requires at least one source rule")
            })?;
        let mut reserved = std::vec![None; plan.rule_orders.len()];
        for (primary, placements) in groups {
            let positions = placements
                .iter()
                .map(|(_, position)| *position)
                .collect::<std::vec::Vec<_>>();
            let reservation = self
                .declaration_blocks
                .reserve_sibling_positions(primary, &positions)
                .ok_or(StyleSheetMutationError::LocalDeclarationBlockCapacityExhausted(primary))?;
            if !reservation.remaps.is_empty() {
                stats.radix_relabel_groups += 1;
                self.authored_declaration_append = None;
                self.rules.for_each_enumerated_mut(|_, rule| {
                    rule.declaration_block = rule
                        .declaration_block
                        .map(|id| mutation::remap_declaration_block_id(id, &reservation.remaps));
                });
                remap_block_slots(blocks, &reservation.remaps);
            }
            for ((logical, _), actual) in placements.into_iter().zip(reservation.reserved) {
                reserved[logical.index()] = Some(actual);
            }
        }
        Ok(reserved)
    }

    fn activate_reserved_rule_with_block(
        &mut self,
        edge: DirectRuleEdge<CssRule<'ast>>,
        append: DeclarationAppendContext<CssRule<'ast>>,
        reserved_rule: CssRuleId<'ast>,
        reserved_block: CssDeclarationBlockId<'ast>,
        payload: CssRule<'ast>,
        effective_key: EffectiveKeyId,
    ) -> Result<(), StyleSheetMutationError<'ast>> {
        let context = self.resolve_direct_rule_window(edge.left_context())?;
        let before_block = self.first_declaration_block_after_rule(edge)?;
        if append.position.next != Some(before_block)
            || !reserved_id_is_between(
                reserved_rule,
                context.insertion_anchor,
                context.storage_before,
            )
            || !reserved_id_is_between(reserved_block, append.block(), Some(before_block))
            || self.effective_keys.try_get(effective_key).is_none()
        {
            return Err(StyleSheetMutationError::InvalidRuleTopology(edge.left()));
        }

        let parent = context.parent;
        let inserted = self
            .rules
            .activate_reserved_sibling(
                reserved_rule,
                RuleRecord {
                    payload,
                    parent,
                    descendant_count: 0,
                    nested_rule_count: 0,
                    subtree_last: None,
                    declaration_block: None,
                    live: true,
                },
            )
            .ok_or(StyleSheetMutationError::LocalRuleCapacityExhausted(
                context.insertion_anchor,
            ))?;
        debug_assert_eq!(inserted, reserved_rule);
        self.authored_declaration_append = None;

        let direct_parent = parent;
        let mut ancestor = parent;
        while let Some(id) = ancestor {
            let rule = self
                .rules
                .get_mut(id)
                .expect("a reserved insertion ancestor remains resolvable");
            rule.descendant_count += 1;
            if Some(id) == direct_parent {
                rule.nested_rule_count += 1;
            }
            if rule.subtree_last.unwrap_or(id) == context.insertion_anchor {
                rule.subtree_last = Some(reserved_rule);
            }
            ancestor = rule.parent;
        }

        let inserted = self
            .declaration_blocks
            .activate_reserved_sibling(
                reserved_block,
                DeclarationBlock {
                    declarations: DeclarationList::empty(),
                    owner: DeclarationBlockOwner::Rule(reserved_rule),
                    effective_key,
                    live: true,
                },
            )
            .ok_or(
                StyleSheetMutationError::LocalDeclarationBlockCapacityExhausted(append.block()),
            )?;
        debug_assert_eq!(inserted, reserved_block);
        self.rules
            .get_mut(reserved_rule)
            .expect("the reserved block owner was activated")
            .declaration_block = Some(reserved_block);
        Ok(())
    }

    fn validate_final_reification(
        &self,
        plan: &ReificationPlan<'ast>,
        rules: &[Option<CssRuleId<'ast>>],
        blocks: &[Option<CssDeclarationBlockId<'ast>>],
        declarations: &[Option<DeclarationId>],
        selectors: &[SelectorValueId],
        effective_keys: &[EffectiveKeyId],
    ) -> Result<(), StyleSheetMutationError<'ast>> {
        for list in &plan.final_rule_lists {
            let parent = list
                .parent
                .map(|parent| rules[parent.index()].ok_or_else(|| missing_rule(self, rules)))
                .transpose()?;
            let actual = if let Some(parent) = parent {
                self.nested_rule_ids(parent)?.collect::<std::vec::Vec<_>>()
            } else {
                self.root_rule_ids().collect::<std::vec::Vec<_>>()
            };
            let expected = list
                .rules
                .iter()
                .map(|rule| rules[rule.index()].ok_or_else(|| missing_rule(self, rules)))
                .collect::<Result<std::vec::Vec<_>, _>>()?;
            if actual != expected {
                return Err(StyleSheetMutationError::InvalidRuleTopology(
                    expected
                        .first()
                        .copied()
                        .or(parent)
                        .ok_or_else(|| missing_rule(self, rules))?,
                ));
            }
        }

        for final_rule in plan.final_rules.iter().filter(|rule| rule.live) {
            let rule =
                rules[final_rule.logical_id.index()].ok_or_else(|| missing_rule(self, rules))?;
            let record = self
                .rule(rule)
                .ok_or(StyleSheetMutationError::UnknownRule(rule))?;
            let expected_parent = final_rule
                .parent
                .map(|parent| rules[parent.index()].ok_or_else(|| missing_rule(self, rules)))
                .transpose()?;
            if !record.live || record.parent() != expected_parent {
                return Err(StyleSheetMutationError::InvalidRuleTopology(rule));
            }
            if let Some(expected_selector) = final_rule.selector {
                let expected_selector = match expected_selector {
                    FinalReificationSelector::Existing(selector) => selector,
                    FinalReificationSelector::Synthesized(selector) => selectors
                        .get(selector.index())
                        .copied()
                        .ok_or(StyleSheetMutationError::InvalidRuleTopology(rule))?,
                };
                let (actual_selector, actual_span) = match record.payload() {
                    CssRule::Style(payload) => (payload.selector_value, payload.span),
                    CssRule::Nesting(payload) => (payload.selector_value, payload.span),
                    _ => return Err(StyleSheetMutationError::InvalidRuleTopology(rule)),
                };
                if actual_selector != expected_selector
                    || final_rule.span.is_some_and(|span| span != actual_span)
                {
                    return Err(StyleSheetMutationError::InvalidRuleTopology(rule));
                }
            }
            let Some(block) = blocks[final_rule.logical_id.index()] else {
                if final_rule.source_block.is_some() {
                    return Err(StyleSheetMutationError::InvalidRuleTopology(rule));
                }
                continue;
            };
            let block_record = self
                .declaration_block(block)
                .ok_or(StyleSheetMutationError::UnknownDeclarationBlock(block))?;
            let expected_key = match final_rule.effective_key {
                Some(ReificationEffectiveKey::Existing(key)) => Some(key),
                Some(ReificationEffectiveKey::Synthesized(key)) => {
                    effective_keys.get(key as usize).copied()
                }
                None => None,
            };
            if !block_record.live
                || block_record.owner() != DeclarationBlockOwner::Rule(rule)
                || expected_key.is_some_and(|key| key != block_record.effective_key())
            {
                return Err(StyleSheetMutationError::InvalidRuleTopology(rule));
            }
            let actual_declarations = self
                .declaration_occurrences_in_block(block)?
                .filter_map(|(id, record)| {
                    (!matches!(
                        record.payload(),
                        CssDeclaration::Property(crate::Declaration::Tombstone)
                    ))
                    .then_some(id)
                })
                .collect::<std::vec::Vec<_>>();
            let expected_declarations = final_rule
                .declarations
                .iter()
                .map(|declaration| {
                    declarations
                        .get(declaration.index())
                        .copied()
                        .flatten()
                        .ok_or(StyleSheetMutationError::UnknownDeclaration(
                            plan.declaration_sources[declaration.index()],
                        ))
                })
                .collect::<Result<std::vec::Vec<_>, _>>()?;
            if actual_declarations != expected_declarations {
                return Err(StyleSheetMutationError::NonContiguousDeclarationRange(
                    block,
                ));
            }
        }
        Ok(())
    }

    fn compatible_rule_tombstone(
        &self,
        edge: DirectRuleEdge<CssRule<'ast>>,
        declaration_count: usize,
    ) -> Result<Option<CompatibleRuleTombstone<'ast>>, StyleSheetMutationError<'ast>> {
        let context = self.resolve_direct_rule_window(edge.left_context())?;
        let bridge = self
            .rules
            .ids_in_range(context.bridge)
            .ok_or(StyleSheetMutationError::InvalidRuleTopology(edge.left()))?;
        for rule in bridge {
            let record = self
                .rules
                .get(rule)
                .ok_or(StyleSheetMutationError::InvalidRuleTopology(rule))?;
            if record.live
                || record.parent != context.parent
                || record.descendant_count != 0
                || record.nested_rule_count != 0
                || record.subtree_last.is_some()
            {
                continue;
            }
            let Some(block) = record.declaration_block else {
                continue;
            };
            let block_record = self
                .declaration_blocks
                .get(block)
                .ok_or(StyleSheetMutationError::UnknownDeclarationBlock(block))?;
            if block_record.live || block_record.owner != DeclarationBlockOwner::Rule(rule) {
                continue;
            }
            let all_declarations = self
                .declarations
                .ids_in_range(block_record.declarations)
                .ok_or(StyleSheetMutationError::NonContiguousDeclarationRange(
                    block,
                ))?
                .collect::<std::vec::Vec<_>>();
            if all_declarations.iter().any(|&declaration| {
                !matches!(
                    self.declarations
                        .get(declaration)
                        .map(DeclarationRecord::payload),
                    Some(CssDeclaration::Property(crate::Declaration::Tombstone))
                )
            }) {
                continue;
            }
            let position = self
                .declaration_block_positions()
                .find(|position| position.block() == block)
                .ok_or(StyleSheetMutationError::UnknownDeclarationBlock(block))?;
            let append = position.append_context();
            let reused = all_declarations
                .into_iter()
                .take(declaration_count)
                .collect::<std::vec::Vec<_>>();
            let residual = declaration_count - reused.len();
            if !self.can_insert_declaration_range_between(
                append.after,
                append.before,
                u32::try_from(residual)
                    .map_err(|_| StyleSheetMutationError::DeclarationCapacityExhausted)?,
            ) {
                continue;
            }
            return Ok(Some(CompatibleRuleTombstone {
                rule,
                block,
                declarations: reused,
                append,
            }));
        }
        Ok(None)
    }

    fn reactivate_rule_tombstone(
        &mut self,
        rule: CssRuleId<'ast>,
        block: CssDeclarationBlockId<'ast>,
        declarations: &[DeclarationId],
        payload: CssRule<'ast>,
        effective_key: EffectiveKeyId,
        values: std::vec::Vec<(CssDeclaration<'ast>, bool)>,
    ) -> Result<std::vec::Vec<(CssDeclaration<'ast>, bool)>, StyleSheetMutationError<'ast>> {
        let parent = self
            .rules
            .get(rule)
            .ok_or(StyleSheetMutationError::UnknownRule(rule))?
            .parent;
        let block_record = self
            .declaration_blocks
            .get(block)
            .ok_or(StyleSheetMutationError::UnknownDeclarationBlock(block))?;
        if block_record.live
            || block_record.owner != DeclarationBlockOwner::Rule(rule)
            || declarations.len() > values.len()
        {
            return Err(StyleSheetMutationError::InvalidRuleTopology(rule));
        }
        let mut values = values.into_iter();
        for &declaration in declarations {
            let (payload, important) = values
                .next()
                .expect("a reusable declaration slot has a planned payload");
            let record = self
                .declarations
                .get_mut(declaration)
                .ok_or(StyleSheetMutationError::UnknownDeclaration(declaration))?;
            record.payload = payload;
            record.important = important;
        }
        let block_record = self
            .declaration_blocks
            .get_mut(block)
            .expect("the reusable block was validated");
        block_record.live = true;
        block_record.owner = DeclarationBlockOwner::Rule(rule);
        block_record.effective_key = effective_key;
        let rule_record = self
            .rules
            .get_mut(rule)
            .expect("the reusable rule was validated");
        rule_record.payload = payload;
        rule_record.live = true;
        rule_record.parent = parent;
        rule_record.descendant_count = 0;
        rule_record.nested_rule_count = 0;
        rule_record.subtree_last = None;
        rule_record.declaration_block = Some(block);
        if let Some(parent) = parent {
            let parent = self
                .rules
                .get_mut(parent)
                .ok_or(StyleSheetMutationError::UnknownRule(parent))?;
            parent.nested_rule_count = parent
                .nested_rule_count
                .checked_add(1)
                .ok_or(StyleSheetMutationError::InvalidRuleTopology(rule))?;
        }
        Ok(values.collect())
    }
}

fn intern_selector<'ast>(
    stylesheet: &mut StyleSheet<'ast>,
    selectors: &mut [crate::SelectorList<'ast>],
    selector_ids: &mut std::vec::Vec<SelectorValueId>,
    selector: ReificationSelectorId,
    kind: SelectorFrameKind,
    prefix: crate::VendorPrefix,
) -> Result<SelectorValueId, StyleSheetMutationError<'ast>> {
    if let Some(&id) = selector_ids.get(selector.index()) {
        return Ok(id);
    }
    debug_assert_eq!(selector.index(), selector_ids.len());
    let selectors = std::mem::replace(
        &mut selectors[selector.index()],
        rocketcss_common::vec::Vec::new_in(stylesheet.allocator()),
    );
    let id = stylesheet.intern_selector_value(selectors, kind, prefix)?;
    selector_ids.push(id);
    Ok(id)
}

fn resolve_key<'ast>(
    stylesheet: &mut StyleSheet<'ast>,
    pending: &[PendingReificationEffectiveKey],
    resolved: &mut std::vec::Vec<EffectiveKeyId>,
    selectors: &[SelectorValueId],
    key: ReificationEffectiveKey,
) -> Result<EffectiveKeyId, StyleSheetMutationError<'ast>> {
    match key {
        ReificationEffectiveKey::Existing(key) => Ok(key),
        ReificationEffectiveKey::Synthesized(index) => {
            if let Some(&key) = resolved.get(index as usize) {
                return Ok(key);
            }
            debug_assert_eq!(index as usize, resolved.len());
            let pending = &pending[index as usize];
            let selector = selectors[pending.selector.index()];
            let key = stylesheet
                .intern_selector_union_effective_key(
                    pending.representative,
                    pending.representative,
                    selector,
                )?
                .ok_or(StyleSheetMutationError::UnknownEffectiveKey(
                    pending.representative,
                ))?;
            resolved.push(key);
            Ok(key)
        }
    }
}

fn find_edge<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rules: &[Option<CssRuleId<'ast>>],
    left: ReificationRuleId,
    right: ReificationRuleId,
) -> Result<DirectRuleEdge<CssRule<'ast>>, StyleSheetMutationError<'ast>> {
    let left_rule = rules[left.index()].ok_or_else(|| missing_rule(stylesheet, rules))?;
    let right_rule = rules[right.index()].ok_or_else(|| missing_rule(stylesheet, rules))?;
    let parent = stylesheet
        .rule(left_rule)
        .ok_or(StyleSheetMutationError::UnknownRule(left_rule))?
        .parent();
    if let Some(parent) = parent {
        stylesheet
            .nested_rule_edges(parent)?
            .find(|edge| edge.left() == left_rule && edge.right() == right_rule)
            .ok_or(StyleSheetMutationError::InvalidRuleTopology(left_rule))
    } else {
        stylesheet
            .root_rule_edges()
            .find(|edge| edge.left() == left_rule && edge.right() == right_rule)
            .ok_or(StyleSheetMutationError::InvalidRuleTopology(left_rule))
    }
}

fn find_context<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rules: &[Option<CssRuleId<'ast>>],
    node: ReificationRuleId,
) -> Result<DirectRuleContext<CssRule<'ast>>, StyleSheetMutationError<'ast>> {
    let rule = rules[node.index()].ok_or_else(|| missing_rule(stylesheet, rules))?;
    let parent = stylesheet
        .rule(rule)
        .ok_or(StyleSheetMutationError::UnknownRule(rule))?
        .parent();
    if let Some(parent) = parent {
        stylesheet
            .nested_rule_contexts(parent)?
            .find(|context| context.rule() == rule)
            .ok_or(StyleSheetMutationError::InvalidRuleTopology(rule))
    } else {
        stylesheet
            .root_rule_contexts()
            .find(|context| context.rule() == rule)
            .ok_or(StyleSheetMutationError::InvalidRuleTopology(rule))
    }
}

fn missing_rule<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rules: &[Option<CssRuleId<'ast>>],
) -> StyleSheetMutationError<'ast> {
    rules
        .iter()
        .flatten()
        .next()
        .copied()
        .map(StyleSheetMutationError::InvalidRuleTopology)
        .or_else(|| {
            stylesheet
                .root_rules()
                .next()
                .map(|(rule, _)| StyleSheetMutationError::InvalidRuleTopology(rule))
        })
        .expect("a reification plan requires at least one source rule")
}

fn style_kind<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rules: &[Option<CssRuleId<'ast>>],
    node: ReificationRuleId,
) -> Result<SelectorFrameKind, StyleSheetMutationError<'ast>> {
    let rule = rules[node.index()].ok_or_else(|| missing_rule(stylesheet, rules))?;
    match stylesheet.rule(rule).map(|rule| rule.payload()) {
        Some(CssRule::Style(_)) => Ok(SelectorFrameKind::Style),
        Some(CssRule::Nesting(_)) => Ok(SelectorFrameKind::Nesting),
        _ => Err(StyleSheetMutationError::InvalidRuleTopology(rule)),
    }
}

fn style_prefix<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rules: &[Option<CssRuleId<'ast>>],
    node: ReificationRuleId,
) -> Result<crate::VendorPrefix, StyleSheetMutationError<'ast>> {
    let rule = rules[node.index()].ok_or_else(|| missing_rule(stylesheet, rules))?;
    match stylesheet.rule(rule).map(|rule| rule.payload()) {
        Some(CssRule::Style(payload)) => Ok(payload.vendor_prefix),
        Some(CssRule::Nesting(_)) => Ok(crate::VendorPrefix::NONE),
        _ => Err(StyleSheetMutationError::InvalidRuleTopology(rule)),
    }
}

fn set_rule_span<'ast>(
    stylesheet: &mut StyleSheet<'ast>,
    rule: CssRuleId<'ast>,
    span: Span,
) -> Result<(), StyleSheetMutationError<'ast>> {
    stylesheet.transform_rule_payload(rule, |payload| match payload {
        CssRule::Style(payload) => payload.span = span,
        CssRule::Nesting(payload) => payload.span = span,
        _ => unreachable!("a validated S3 endpoint is selector-bearing"),
    })
}

fn rule_span<'ast>(
    stylesheet: &StyleSheet<'ast>,
    rule: CssRuleId<'ast>,
) -> Result<Span, StyleSheetMutationError<'ast>> {
    match stylesheet.rule(rule).map(|rule| rule.payload()) {
        Some(CssRule::Style(payload)) => Ok(payload.span),
        Some(CssRule::Nesting(payload)) => Ok(payload.span),
        _ => Err(StyleSheetMutationError::InvalidRuleTopology(rule)),
    }
}

fn remap_rule_slots<'ast>(
    rules: &mut [Option<CssRuleId<'ast>>],
    remaps: &[RadixIdRemap<CssRuleId<'ast>>],
) {
    for rule in rules.iter_mut().flatten() {
        if let Some(remap) = remaps.iter().find(|remap| remap.old == *rule) {
            *rule = remap.new;
        }
    }
}

fn remap_block_slots<'ast>(
    blocks: &mut [Option<CssDeclarationBlockId<'ast>>],
    remaps: &[RadixIdRemap<CssDeclarationBlockId<'ast>>],
) {
    for block in blocks.iter_mut().flatten() {
        if let Some(remap) = remaps.iter().find(|remap| remap.old == *block) {
            *block = remap.new;
        }
    }
}

type ReservationPlacement = (ReificationRuleId, usize);
type ReservationGroups<I> = std::vec::Vec<(I, std::vec::Vec<ReservationPlacement>)>;
type ReservationGroupMap<I> = BTreeMap<usize, (I, std::vec::Vec<ReservationPlacement>)>;

fn reservation_groups<I: RadixIdKey + std::hash::Hash>(
    physical: &[I],
    sources: &[Option<I>],
    orders: &[std::vec::Vec<u32>],
    synthesized: &[ReificationRuleId],
) -> Option<ReservationGroups<I>> {
    let source_orders = sources
        .iter()
        .enumerate()
        .filter_map(|(index, source)| source.map(|source| (source, index)))
        .collect::<FxHashMap<_, _>>();
    let mut synthesized = synthesized.to_vec();
    synthesized.sort_unstable_by(|left, right| {
        orders[left.index()]
            .cmp(&orders[right.index()])
            .then_with(|| left.cmp(right))
    });
    synthesized.dedup();

    let mut groups = ReservationGroupMap::new();
    let mut current_primary = None;
    let mut sibling_position = 0;
    let mut next_synthesized = 0;
    let place = |logical: ReificationRuleId,
                 current_primary: Option<I>,
                 sibling_position: &mut usize,
                 groups: &mut ReservationGroupMap<I>| {
        let primary = current_primary?;
        groups
            .entry(primary.primary_index())
            .or_insert_with(|| (primary, std::vec::Vec::new()))
            .1
            .push((logical, *sibling_position));
        *sibling_position += 1;
        Some(())
    };

    for &id in physical {
        if let Some(&source_index) = source_orders.get(&id) {
            while let Some(&logical) = synthesized.get(next_synthesized)
                && orders[logical.index()] < orders[source_index]
            {
                place(logical, current_primary, &mut sibling_position, &mut groups)?;
                next_synthesized += 1;
            }
        }
        if id.is_primary() {
            current_primary = (!id.is_overflow()).then_some(id);
            sibling_position = 0;
        } else {
            let primary = current_primary?;
            if primary.primary_index() != id.primary_index() {
                return None;
            }
            sibling_position += 1;
        }
    }
    while let Some(&logical) = synthesized.get(next_synthesized) {
        place(logical, current_primary, &mut sibling_position, &mut groups)?;
        next_synthesized += 1;
    }
    Some(groups.into_values().collect())
}

fn reserved_id_is_between<I: RadixIdKey>(reserved: I, after: I, before: Option<I>) -> bool {
    if reserved.is_primary()
        || reserved.is_overflow()
        || after.is_overflow()
        || reserved.primary_index() != after.primary_index()
        || reserved.sibling_key() <= after.sibling_key()
    {
        return false;
    }
    before.is_none_or(|before| {
        before.primary_index() != reserved.primary_index()
            || reserved.sibling_key() < before.sibling_key()
    })
}
