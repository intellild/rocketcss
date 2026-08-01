//! S3 selector partial factoring for the current nested AST.
//!
//! The design documents describe stable rule IDs plus a later S4/S5
//! reification step. The current AST has neither, so this pass commits only
//! the first source-ordered candidate it finds. Its caller then rebuilds S1
//! and S2 state before another S3 candidate may commit. This keeps physical
//! vector indices out of cached candidate state and gives higher-priority
//! declaration-history work a chance to stabilize after every structural
//! edit.

use rocketcss_ast::*;
use rocketcss_common::vec::Vec;
use rustc_hash::FxHashSet;

mod selector;

use self::selector::materialize_selector_union;

use crate::{MinifyContext, Options, OptionsOp};

#[derive(Clone, Copy, Debug)]
struct DeclarationSlot {
    block: DeclarationBlockId,
    index: usize,
}

#[derive(Clone, Copy, Debug)]
struct CommonDeclaration {
    left: DeclarationSlot,
    right: DeclarationSlot,
    right_order: usize,
}

struct PartialMergePlan<'ast> {
    selectors: SelectorList<'ast>,
    common: std::vec::Vec<CommonDeclaration>,
    span: Span,
    vendor_prefix: VendorPrefix,
    retain_left: bool,
    retain_right: bool,
}

pub(super) fn factor_first_partial_selector_candidate<'ast, 'scratch>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    declaration_blocks: &mut DeclarationBlockStore<'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> bool
where
    'ast: 'scratch,
{
    PartialSelectorFactoringPass {
        declaration_blocks,
        cx,
    }
    .factor_first(rules)
}

struct PartialSelectorFactoringPass<'pass, 'ast, 'scratch> {
    declaration_blocks: &'pass mut DeclarationBlockStore<'ast>,
    cx: &'pass mut MinifyContext<'scratch>,
}

impl<'ast, 'scratch> PartialSelectorFactoringPass<'_, 'ast, 'scratch>
where
    'ast: 'scratch,
{
    fn factor_first(&mut self, rules: &mut Vec<'ast, CssRule<'ast>>) -> bool {
        for index in 0..rules.len().saturating_sub(1) {
            let candidate = match (&rules[index], &rules[index + 1]) {
                (CssRule::Style(left), CssRule::Style(right)) => {
                    self.discover_candidate(left.as_ref().get_ref(), right.as_ref().get_ref())
                }
                _ => None,
            };
            if let Some(candidate) = candidate {
                self.commit_candidate(index, rules, candidate);
                return true;
            }
        }

        for rule in rules.iter_mut() {
            let nested = match rule {
                CssRule::Media(rule) => Some(&mut rule.rules),
                CssRule::Style(rule) => Some(rule.as_mut().rules_mut()),
                CssRule::Supports(rule) => Some(&mut rule.rules),
                CssRule::MozDocument(rule) => Some(&mut rule.rules),
                CssRule::Nesting(rule) => Some(rule.style.as_mut().rules_mut()),
                CssRule::LayerBlock(rule) => Some(&mut rule.rules),
                CssRule::Container(rule) => Some(&mut rule.rules),
                CssRule::Scope(rule) => Some(&mut rule.rules),
                CssRule::StartingStyle(rule) => Some(&mut rule.rules),
                _ => None,
            };
            if let Some(nested) = nested
                && self.factor_first(nested)
            {
                return true;
            }
        }
        false
    }

    fn discover_candidate(
        &self,
        left: &StyleRule<'ast>,
        right: &StyleRule<'ast>,
    ) -> Option<PartialMergePlan<'ast>> {
        if !left.rules.is_empty()
            || left.vendor_prefix != right.vendor_prefix
            || left.selectors == right.selectors
        {
            return None;
        }

        let left_declarations = live_declaration_slots(left.declarations, self.declaration_blocks);
        let right_declarations =
            live_declaration_slots(right.declarations, self.declaration_blocks);
        if left_declarations.is_empty() || right_declarations.is_empty() {
            return None;
        }

        let mut right_matched = vec![false; right_declarations.len()];
        let mut common = std::vec::Vec::new();
        for &left_slot in &left_declarations {
            let (left_declaration, left_important) =
                declaration_at(left_slot, self.declaration_blocks);
            let Some((right_order, &right_slot)) =
                right_declarations
                    .iter()
                    .enumerate()
                    .find(|(right_order, right_slot)| {
                        if right_matched[*right_order] {
                            return false;
                        }
                        let (right_declaration, right_important) =
                            declaration_at(**right_slot, self.declaration_blocks);
                        left_important == right_important
                            && left_declaration.eq_ignoring_tombstones(right_declaration)
                    })
            else {
                continue;
            };
            right_matched[right_order] = true;
            common.push(CommonDeclaration {
                left: left_slot,
                right: right_slot,
                right_order,
            });
        }
        if common.is_empty() {
            return None;
        }

        let mut left_common = FxHashSet::default();
        let mut right_common = FxHashSet::default();
        for common in &common {
            left_common.insert((common.left.block, common.left.index));
            right_common.insert((common.right.block, common.right.index));
        }
        let left_residual = left_declarations
            .iter()
            .copied()
            .filter(|slot| !left_common.contains(&(slot.block, slot.index)))
            .collect::<std::vec::Vec<_>>();
        let right_residual = right_declarations
            .iter()
            .copied()
            .filter(|slot| !right_common.contains(&(slot.block, slot.index)))
            .collect::<std::vec::Vec<_>>();
        if !partial_movement_is_safe(
            &common,
            &left_residual,
            &right_residual,
            self.declaration_blocks,
        ) {
            return None;
        }
        let selectors = materialize_selector_union(
            &left.selectors,
            &right.selectors,
            self.cx
                .is_enabled(Options::PRESERVE_SELECTOR_COMPATIBILITY, OptionsOp::Any),
        )?;

        Some(PartialMergePlan {
            selectors,
            common,
            span: Span::new(left.span.start, right.span.end),
            vendor_prefix: left.vendor_prefix,
            retain_left: !left_residual.is_empty(),
            retain_right: !right_residual.is_empty() || !right.rules.is_empty(),
        })
    }

    fn commit_candidate(
        &mut self,
        left_index: usize,
        rules: &mut Vec<'ast, CssRule<'ast>>,
        plan: PartialMergePlan<'ast>,
    ) {
        let allocator = plan.selectors.bump();
        let mut shared = DeclarationBlock::new(allocator);
        for common in &plan.common {
            let important = self
                .declaration_blocks
                .get(common.left.block)
                .is_important(common.left.index);
            let declaration = std::mem::replace(
                &mut self
                    .declaration_blocks
                    .get_mut(common.left.block)
                    .declarations[common.left.index],
                Declaration::Tombstone,
            );
            let removed_right = std::mem::replace(
                &mut self
                    .declaration_blocks
                    .get_mut(common.right.block)
                    .declarations[common.right.index],
                Declaration::Tombstone,
            );
            debug_assert!(declaration.eq_ignoring_tombstones(&removed_right));
            shared.push(declaration, important);
            self.cx.record_declaration_removed();
        }
        let declarations = self.declaration_blocks.push(shared);
        let shared_rule = CssRule::Style(allocator.pinned(StyleRule::new(
            declarations,
            plan.span,
            allocator.vec(),
            plan.selectors,
            plan.vendor_prefix,
        )));

        let right_index = left_index + 1;
        rules.insert(right_index, shared_rule);
        if !plan.retain_right {
            rules.remove(right_index + 1);
        }
        if !plan.retain_left {
            rules.remove(left_index);
        }
    }
}

fn live_declaration_slots(
    active: DeclarationBlockId,
    declaration_blocks: &DeclarationBlockStore<'_>,
) -> std::vec::Vec<DeclarationSlot> {
    let mut chain = std::vec::Vec::new();
    let mut seen = FxHashSet::default();
    let mut current = Some(active);
    while let Some(block) = current {
        if !seen.insert(block) {
            break;
        }
        chain.push(block);
        current = declaration_blocks.get(block).previous_merged();
    }
    chain.reverse();

    let mut declarations = std::vec::Vec::new();
    for block in chain {
        declarations.extend(
            declaration_blocks
                .get(block)
                .declarations
                .iter()
                .enumerate()
                .filter(|(_, declaration)| !declaration.is_tombstone())
                .map(|(index, _)| DeclarationSlot { block, index }),
        );
    }
    declarations
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
    left_residual: &[DeclarationSlot],
    right_residual: &[DeclarationSlot],
    declaration_blocks: &DeclarationBlockStore<'_>,
) -> bool {
    let common_domains = common
        .iter()
        .map(|common| movement_domain(declaration_at(common.left, declaration_blocks).0))
        .collect::<Option<std::vec::Vec<_>>>();
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
        .map(|&slot| movement_domain(declaration_at(slot, declaration_blocks).0))
        .collect::<Option<std::vec::Vec<_>>>();
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
    common_domains: &[MovementDomain<'_>],
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MovementDomain<'ast> {
    Custom(&'ast str),
    Background,
    Border,
    Color,
    Display,
    Font,
    Height,
    Margin(u8),
    Mask,
    Opacity,
    Padding(u8),
    Position,
    TextAlign,
    TextDecoration,
    TextTransform,
    Visibility,
    Width,
}

impl MovementDomain<'_> {
    fn overlaps(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Custom(left), Self::Custom(right)) => left == right,
            (Self::Margin(left), Self::Margin(right))
            | (Self::Padding(left), Self::Padding(right)) => left & right != 0,
            _ => self == other,
        }
    }
}

fn movement_domain<'ast>(declaration: &Declaration<'ast>) -> Option<MovementDomain<'ast>> {
    use MovementDomain as Domain;
    use PropertyId as Id;

    Some(match declaration.property_id()? {
        Id::Custom(name) => Domain::Custom(name),
        Id::BackgroundColor
        | Id::BackgroundImage
        | Id::BackgroundPositionX
        | Id::BackgroundPositionY
        | Id::BackgroundPosition
        | Id::BackgroundSize
        | Id::BackgroundRepeat
        | Id::BackgroundAttachment
        | Id::BackgroundClip(_)
        | Id::BackgroundOrigin
        | Id::Background => Domain::Background,
        Id::BorderTopColor
        | Id::BorderBottomColor
        | Id::BorderLeftColor
        | Id::BorderRightColor
        | Id::BorderTopStyle
        | Id::BorderBottomStyle
        | Id::BorderLeftStyle
        | Id::BorderRightStyle
        | Id::BorderTopWidth
        | Id::BorderBottomWidth
        | Id::BorderLeftWidth
        | Id::BorderRightWidth
        | Id::BorderColor
        | Id::BorderStyle
        | Id::BorderWidth
        | Id::Border
        | Id::BorderTop
        | Id::BorderBottom
        | Id::BorderLeft
        | Id::BorderRight => Domain::Border,
        Id::Color => Domain::Color,
        Id::Display => Domain::Display,
        Id::FontWeight
        | Id::FontSize
        | Id::FontStretch
        | Id::FontFamily
        | Id::FontStyle
        | Id::FontVariantCaps
        | Id::LineHeight
        | Id::Font => Domain::Font,
        Id::Height | Id::MinHeight | Id::MaxHeight => Domain::Height,
        Id::MarginTop => Domain::Margin(0b0001),
        Id::MarginRight => Domain::Margin(0b0010),
        Id::MarginBottom => Domain::Margin(0b0100),
        Id::MarginLeft => Domain::Margin(0b1000),
        Id::Margin => Domain::Margin(0b1111),
        Id::MaskImage(_)
        | Id::MaskMode
        | Id::MaskRepeat(_)
        | Id::MaskPositionX
        | Id::MaskPositionY
        | Id::MaskPosition(_)
        | Id::MaskClip(_)
        | Id::MaskOrigin(_)
        | Id::MaskSize(_)
        | Id::MaskComposite
        | Id::MaskType
        | Id::Mask(_) => Domain::Mask,
        Id::Opacity => Domain::Opacity,
        Id::PaddingTop => Domain::Padding(0b0001),
        Id::PaddingRight => Domain::Padding(0b0010),
        Id::PaddingBottom => Domain::Padding(0b0100),
        Id::PaddingLeft => Domain::Padding(0b1000),
        Id::Padding => Domain::Padding(0b1111),
        Id::Position | Id::Top | Id::Right | Id::Bottom | Id::Left => Domain::Position,
        Id::TextAlign => Domain::TextAlign,
        Id::TextDecorationLine(_)
        | Id::TextDecorationStyle(_)
        | Id::TextDecorationColor(_)
        | Id::TextDecorationThickness
        | Id::TextDecoration(_) => Domain::TextDecoration,
        Id::TextTransform => Domain::TextTransform,
        Id::Visibility => Domain::Visibility,
        Id::Width | Id::MinWidth | Id::MaxWidth => Domain::Width,
        _ => return None,
    })
}
