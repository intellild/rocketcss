//! S3 selector-union and declaration-movement proofs.
//!
//! This module only discovers and commits declaration movement. Logical
//! adjacency, EffectiveKey interning, queue propagation, and physical AST
//! reification are owned by the persistent scheduler.

use rocketcss_ast::*;
use smallvec::SmallVec;

mod selector;

pub(super) use self::selector::materialize_selector_union;

use super::declaration_ir::{
    DeclarationOccurrenceId, DeclarationSlot, FrozenDeclarationIrStore, MovementDomain,
    OccurrenceOrder,
};
use crate::{MinifyContext, Options, OptionsOp};

#[derive(Clone, Copy, Debug)]
struct CommonDeclaration {
    left: DeclarationOccurrenceId,
    right: DeclarationOccurrenceId,
    right_order: OccurrenceOrder,
}

pub(super) struct PartialRuleRef<'rule, 'ast> {
    pub(super) declarations: DeclarationBlockId,
    pub(super) selectors: &'rule SelectorList<'ast>,
    pub(super) span: Span,
    pub(super) vendor_prefix: VendorPrefix,
    pub(super) has_children: bool,
}

pub(super) struct PartialMergePlan<'ast> {
    pub(super) selectors: SelectorList<'ast>,
    common: SmallVec<[CommonDeclaration; 8]>,
    left: DeclarationBlockId,
    right: DeclarationBlockId,
    pub(super) span: Span,
    pub(super) vendor_prefix: VendorPrefix,
    pub(super) retain_left: bool,
    pub(super) retain_right: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PartialMergeRejection {
    Ineligible,
    NoCommonDeclaration,
    UnsafeMovement,
    IncompatibleSelectors,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PartialMergePlacement {
    ReusedLeft,
    AllocatedBetween,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct PartialMergeCommit {
    pub(super) shared: DeclarationBlockId,
    pub(super) placement: PartialMergePlacement,
    #[cfg(test)]
    pub(super) declaration_count: u32,
}

pub(super) fn discover_partial_merge_plan<'ast, 'scratch>(
    left: PartialRuleRef<'_, 'ast>,
    right: PartialRuleRef<'_, 'ast>,
    declaration_blocks: &DeclarationBlockStore<'ast>,
    declaration_ir: &mut FrozenDeclarationIrStore<'ast>,
    cx: &MinifyContext<'scratch>,
) -> Result<PartialMergePlan<'ast>, PartialMergeRejection>
where
    'ast: 'scratch,
{
    if left.has_children
        || left.vendor_prefix != right.vendor_prefix
        || left.selectors == right.selectors
    {
        return Err(PartialMergeRejection::Ineligible);
    }

    let left_declarations = declaration_ir
        .live_occurrences(left.declarations)
        .collect::<SmallVec<[_; 8]>>();
    let right_declarations = declaration_ir
        .live_occurrences(right.declarations)
        .collect::<SmallVec<[_; 8]>>();
    if left_declarations.is_empty() || right_declarations.is_empty() {
        return Err(PartialMergeRejection::NoCommonDeclaration);
    }

    let match_generation = declaration_ir.begin_matching();
    let mut common = SmallVec::<[CommonDeclaration; 8]>::new();
    for &left_occurrence in &left_declarations {
        if !declaration_ir.is_matchable(left_occurrence) {
            continue;
        }
        let left_slot = declaration_ir.occurrence(left_occurrence).slot;
        let (left_declaration, left_important) = declaration_at(left_slot, declaration_blocks);
        let Some((right_occurrence, right_order)) = declaration_ir
            .matching_occurrences(right.declarations, left_occurrence)
            .find(|&(right_occurrence, _)| {
                if declaration_ir.is_matched(right_occurrence, match_generation) {
                    return false;
                }
                let right_slot = declaration_ir.occurrence(right_occurrence).slot;
                let (right_declaration, right_important) =
                    declaration_at(right_slot, declaration_blocks);
                left_important == right_important
                    && left_declaration.eq_ignoring_tombstones(right_declaration)
            })
        else {
            continue;
        };
        declaration_ir.mark_matched(left_occurrence, match_generation);
        declaration_ir.mark_matched(right_occurrence, match_generation);
        common.push(CommonDeclaration {
            left: left_occurrence,
            right: right_occurrence,
            right_order,
        });
    }
    if common.is_empty() {
        return Err(PartialMergeRejection::NoCommonDeclaration);
    }

    let left_residual = left_declarations
        .iter()
        .copied()
        .filter(|&occurrence| !declaration_ir.is_matched(occurrence, match_generation))
        .collect::<SmallVec<[_; 8]>>();
    let right_residual = right_declarations
        .iter()
        .copied()
        .filter(|&occurrence| !declaration_ir.is_matched(occurrence, match_generation))
        .collect::<SmallVec<[_; 8]>>();
    if !partial_movement_is_safe(&common, &left_residual, &right_residual, declaration_ir) {
        return Err(PartialMergeRejection::UnsafeMovement);
    }
    let selectors = materialize_selector_union(
        left.selectors,
        right.selectors,
        cx.is_enabled(Options::PRESERVE_SELECTOR_COMPATIBILITY, OptionsOp::Any),
    )
    .ok_or(PartialMergeRejection::IncompatibleSelectors)?;

    Ok(PartialMergePlan {
        selectors,
        common,
        left: left.declarations,
        right: right.declarations,
        span: Span::new(left.span.start, right.span.end),
        vendor_prefix: left.vendor_prefix,
        retain_left: !left_residual.is_empty(),
        retain_right: !right_residual.is_empty() || right.has_children,
    })
}

pub(super) fn commit_partial_merge_declarations<'ast>(
    plan: &PartialMergePlan<'ast>,
    declaration_blocks: &mut DeclarationBlockStore<'ast>,
    declaration_ir: &mut FrozenDeclarationIrStore<'ast>,
    cx: &mut MinifyContext<'_>,
) -> PartialMergeCommit {
    if !plan.retain_left {
        return commit_partial_merge_declarations_reusing_left(
            plan,
            declaration_blocks,
            declaration_ir,
            cx,
        );
    }

    commit_partial_merge_declarations_allocating(plan, declaration_blocks, declaration_ir, cx)
}

fn commit_partial_merge_declarations_reusing_left<'ast>(
    plan: &PartialMergePlan<'ast>,
    declaration_blocks: &mut DeclarationBlockStore<'ast>,
    declaration_ir: &mut FrozenDeclarationIrStore<'ast>,
    cx: &mut MinifyContext<'_>,
) -> PartialMergeCommit {
    debug_assert!(!plan.retain_left);
    for common in &plan.common {
        let left = declaration_ir.occurrence(common.left).slot;
        let right = declaration_ir.occurrence(common.right).slot;
        debug_assert_eq!(
            declaration_blocks.get(left.block).is_important(left.index),
            declaration_blocks
                .get(right.block)
                .is_important(right.index)
        );
        let removed_right = std::mem::replace(
            &mut declaration_blocks.get_mut(right.block).declarations[right.index],
            Declaration::Tombstone,
        );
        debug_assert!(
            declaration_blocks.get(left.block).declarations[left.index]
                .eq_ignoring_tombstones(&removed_right)
        );
        cx.record_declaration_removed();
    }
    let pairs = plan
        .common
        .iter()
        .map(|common| (common.left, common.right))
        .collect::<SmallVec<[_; 8]>>();
    declaration_ir.reuse_left_as_shared(plan.left, plan.right, &pairs);
    PartialMergeCommit {
        shared: plan.left,
        placement: PartialMergePlacement::ReusedLeft,
        #[cfg(test)]
        declaration_count: u32::try_from(plan.common.len())
            .expect("common declaration count exceeds u32::MAX"),
    }
}

fn commit_partial_merge_declarations_allocating<'ast>(
    plan: &PartialMergePlan<'ast>,
    declaration_blocks: &mut DeclarationBlockStore<'ast>,
    declaration_ir: &mut FrozenDeclarationIrStore<'ast>,
    cx: &mut MinifyContext<'_>,
) -> PartialMergeCommit {
    let allocator = plan.selectors.bump();
    let mut shared = DeclarationBlock::new(allocator);
    for common in &plan.common {
        let left = declaration_ir.occurrence(common.left).slot;
        let right = declaration_ir.occurrence(common.right).slot;
        let important = declaration_blocks.get(left.block).is_important(left.index);
        let declaration = std::mem::replace(
            &mut declaration_blocks.get_mut(left.block).declarations[left.index],
            Declaration::Tombstone,
        );
        let removed_right = std::mem::replace(
            &mut declaration_blocks.get_mut(right.block).declarations[right.index],
            Declaration::Tombstone,
        );
        debug_assert!(declaration.eq_ignoring_tombstones(&removed_right));
        shared.push(declaration, important);
        cx.record_declaration_removed();
    }
    let shared = declaration_blocks.push(shared);
    let pairs = plan
        .common
        .iter()
        .map(|common| (common.left, common.right))
        .collect::<SmallVec<[_; 8]>>();
    declaration_ir.transfer_common(shared, plan.left, plan.right, &pairs);
    PartialMergeCommit {
        shared,
        placement: PartialMergePlacement::AllocatedBetween,
        #[cfg(test)]
        declaration_count: u32::try_from(plan.common.len())
            .expect("common declaration count exceeds u32::MAX"),
    }
}

fn declaration_at<'a, 'ast>(
    slot: DeclarationSlot,
    declaration_blocks: &'a DeclarationBlockStore<'ast>,
) -> (&'a Declaration<'ast>, bool) {
    let block = declaration_blocks.get(slot.block);
    (
        &block.declarations[slot.index],
        block.is_important(slot.index),
    )
}

fn partial_movement_is_safe(
    common: &[CommonDeclaration],
    left_residual: &[DeclarationOccurrenceId],
    right_residual: &[DeclarationOccurrenceId],
    declaration_ir: &FrozenDeclarationIrStore<'_>,
) -> bool {
    let common_domains = common
        .iter()
        .map(|common| declaration_ir.occurrence(common.left).movement_domain)
        .collect::<Option<SmallVec<[_; 8]>>>();
    if left_residual.is_empty() && right_residual.is_empty() {
        let Some(common_domains) = common_domains else {
            return common
                .windows(2)
                .all(|pair| pair[0].right_order < pair[1].right_order);
        };
        return common_effect_order_is_safe(common, &common_domains);
    }
    let Some(common_domains) = common_domains else {
        return false;
    };
    let residual_domains = left_residual
        .iter()
        .chain(right_residual)
        .map(|&occurrence| declaration_ir.occurrence(occurrence).movement_domain)
        .collect::<Option<SmallVec<[_; 8]>>>();
    let Some(residual_domains) = residual_domains else {
        return false;
    };
    if common_domains.iter().any(|common| {
        residual_domains
            .iter()
            .any(|residual| common.overlaps(residual))
    }) {
        return false;
    }

    common_effect_order_is_safe(common, &common_domains)
}

fn common_effect_order_is_safe(
    common: &[CommonDeclaration],
    common_domains: &[MovementDomain],
) -> bool {
    for left in 0..common.len() {
        for right in left + 1..common.len() {
            if common[left].right_order > common[right].right_order
                && common_domains[left].overlaps(&common_domains[right])
            {
                return false;
            }
        }
    }
    true
}
