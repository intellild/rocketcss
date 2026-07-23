use std::hash::{Hash, Hasher};

use rocketcss_allocator::{
    Allocator, Ref,
    boxed::Box,
    prelude::{AdaptiveHashMap, AdaptiveHashSet},
    vec::Vec,
};
use rocketcss_ast::{
    CssRule, Declaration, DeclarationBlock, EqIgnoringTombstones, PropertyId, PseudoElement,
    Selector, SelectorComponent, SelectorList, Span, StyleRule, VendorPrefix,
};

use super::DeclarationBlockMinifier;
use crate::{MinifyContext, Options, OptionsOp};

#[derive(Clone, Copy, PartialEq, Eq)]
struct CascadeScope {
    layer: Option<LayerContextId>,
    origin: CascadeOrigin,
}

impl CascadeScope {
    const AUTHOR: Self = Self {
        layer: None,
        origin: CascadeOrigin::Author,
    };

    fn in_layer(self, layer: LayerContextId) -> Self {
        Self {
            layer: Some(layer),
            ..self
        }
    }

    fn declaration_context(self, important: bool) -> DeclarationHistoryContext {
        DeclarationHistoryContext {
            layer: self.layer,
            origin: self.origin,
            phase: if important {
                CascadePhase::Important
            } else {
                CascadePhase::Normal
            },
        }
    }
}

impl DeclarationHistoryContext {
    fn is_important(self) -> bool {
        matches!(self.phase, CascadePhase::Important)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct DeclarationHistoryContext {
    layer: Option<LayerContextId>,
    origin: CascadeOrigin,
    phase: CascadePhase,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct LayerContextId(u32);

#[derive(Clone, Copy, PartialEq, Eq)]
enum CascadeOrigin {
    Author,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CascadePhase {
    Normal,
    Important,
}

#[derive(Clone, Copy)]
struct SelectorHistoryKey<'list, 'ast> {
    selectors: &'list SelectorList<'ast>,
    vendor_prefix: VendorPrefix,
}

impl PartialEq for SelectorHistoryKey<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        self.vendor_prefix == other.vendor_prefix
            && equal_live_selectors(self.selectors, other.selectors)
    }
}

impl Eq for SelectorHistoryKey<'_, '_> {}

impl Hash for SelectorHistoryKey<'_, '_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vendor_prefix.bits().hash(state);
        let mut len = 0;
        for selector in self
            .selectors
            .iter()
            .filter(|selector| !selector.is_tombstone())
        {
            selector.hash(state);
            len += 1;
        }
        len.hash(state);
    }
}

#[derive(Default)]
struct HistoryTraversal {
    next_layer_context: u32,
}

impl HistoryTraversal {
    fn next_layer_context(&mut self) -> LayerContextId {
        let context = LayerContextId(self.next_layer_context);
        self.next_layer_context += 1;
        context
    }
}

pub(crate) fn merge_adjacent_style_rules<'ast, 'scratch>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    merge_rule_list(
        rules,
        minifier,
        cx,
        CascadeScope::AUTHOR,
        &mut HistoryTraversal::default(),
    );
}

fn merge_rule_list<'ast, 'scratch>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
    cascade_scope: CascadeScope,
    history_traversal: &mut HistoryTraversal,
) where
    'ast: 'scratch,
{
    if cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
        coalesce_adjacent_conditional_rules(rules);
        remove_empty_barriers(rules);
    }
    for rule in rules.iter_mut() {
        merge_children(rule, minifier, cx, cascade_scope, history_traversal);
    }
    if !cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
        return;
    }

    loop {
        let conditional_changed = coalesce_adjacent_conditional_rules(rules);
        if conditional_changed {
            for rule in rules.iter_mut() {
                merge_children(rule, minifier, cx, cascade_scope, history_traversal);
            }
        }
        let mut changed = conditional_changed;
        changed |= remove_empty_barriers(rules);
        changed |= prune_equal_selector_histories(rules, minifier, cx);
        changed |= merge_same_selector_edges(rules);
        changed |= factor_partial_edges(rules, cx, cascade_scope);
        if !changed {
            break;
        }
    }
}

fn merge_children<'ast, 'scratch>(
    rule: &mut CssRule<'ast>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
    cascade_scope: CascadeScope,
    history_traversal: &mut HistoryTraversal,
) where
    'ast: 'scratch,
{
    let (children, child_scope) = match rule {
        CssRule::Media(rule) => (Some(&mut rule.rules), cascade_scope),
        CssRule::Style(rule) => {
            if cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
                prune_parent_declaration_history(rule, minifier, cx);
            }
            (Some(&mut rule.rules), cascade_scope)
        }
        CssRule::Supports(rule) => (Some(&mut rule.rules), cascade_scope),
        CssRule::Container(rule) => (Some(&mut rule.rules), cascade_scope),
        // These contexts deliberately remain barriers to cross-rule identity,
        // but their own child lists may still perform local, context-preserving
        // same-selector coalescing.
        CssRule::MozDocument(rule) => (Some(&mut rule.rules), cascade_scope),
        CssRule::Nesting(rule) => (Some(&mut rule.style.rules), cascade_scope),
        CssRule::LayerBlock(rule) => (
            Some(&mut rule.rules),
            cascade_scope.in_layer(history_traversal.next_layer_context()),
        ),
        CssRule::Scope(rule) => (Some(&mut rule.rules), cascade_scope),
        CssRule::StartingStyle(rule) => (Some(&mut rule.rules), cascade_scope),
        _ => (None, cascade_scope),
    };
    if let Some(children) = children {
        merge_rule_list(children, minifier, cx, child_scope, history_traversal);
    }
}

fn prune_parent_declaration_history<'ast, 'scratch>(
    style: &mut StyleRule<'ast>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    let mut blocks = std::vec::Vec::new();
    collect_declaration_chain(&style.declarations, &mut blocks);
    for rule in &style.rules {
        match rule {
            CssRule::Style(_) => {}
            CssRule::NestedDeclarations(rule) => {
                collect_declaration_chain(&rule.declarations, &mut blocks);
            }
            _ => break,
        }
    }
    if blocks.len() > 1 {
        minifier.minify_sequence(&mut blocks, cx);
    }
}

fn coalesce_adjacent_conditional_rules(rules: &mut Vec<'_, CssRule<'_>>) -> bool {
    let mut changed = false;
    let mut index = 0;
    while index + 1 < rules.len() {
        let equal = match (&rules[index], &rules[index + 1]) {
            (CssRule::Media(left), CssRule::Media(right)) => {
                left.query == right.query
                    && rule_list_has_output(&left.rules)
                    && rule_list_has_output(&right.rules)
            }
            (CssRule::Supports(left), CssRule::Supports(right)) => {
                left.condition == right.condition
                    && rule_list_has_output(&left.rules)
                    && rule_list_has_output(&right.rules)
            }
            (CssRule::Container(left), CssRule::Container(right)) => {
                left.name == right.name
                    && left.condition == right.condition
                    && rule_list_has_output(&left.rules)
                    && rule_list_has_output(&right.rules)
            }
            _ => false,
        };
        if !equal {
            index += 1;
            continue;
        }

        let removed = rules.remove(index + 1);
        match (&mut rules[index], removed) {
            (CssRule::Media(left), CssRule::Media(mut right)) => {
                left.rules.append(&mut right.rules);
                left.span.end = left.span.end.max(right.span.end);
            }
            (CssRule::Supports(left), CssRule::Supports(mut right)) => {
                left.rules.append(&mut right.rules);
                left.span.end = left.span.end.max(right.span.end);
            }
            (CssRule::Container(left), CssRule::Container(mut right)) => {
                left.rules.append(&mut right.rules);
                left.span.end = left.span.end.max(right.span.end);
            }
            _ => unreachable!("equal conditional rules have matching variants"),
        }
        changed = true;
    }
    changed
}

fn remove_empty_barriers(rules: &mut Vec<'_, CssRule<'_>>) -> bool {
    let mut changed = false;
    let mut index = 0;
    while index < rules.len() {
        let remove = match &rules[index] {
            CssRule::NestedDeclarations(rule) => rule.declarations.is_output_empty(),
            CssRule::Media(rule) if !rule_list_has_output(&rule.rules) => {
                style_endpoints_surround(rules, index)
            }
            CssRule::Supports(rule) if !rule_list_has_output(&rule.rules) => {
                style_endpoints_surround(rules, index)
            }
            CssRule::Container(rule) if !rule_list_has_output(&rule.rules) => {
                style_endpoints_surround(rules, index)
            }
            _ => false,
        };
        if remove {
            rules.remove(index);
            changed = true;
        } else {
            index += 1;
        }
    }
    changed
}

fn style_endpoints_surround(rules: &[CssRule<'_>], index: usize) -> bool {
    let left = rules[..index].iter().rev().find_map(|rule| match rule {
        CssRule::Style(rule) if has_live_selector(rule) => Some(true),
        CssRule::Style(rule) if rule.rules.is_empty() => None,
        _ => Some(false),
    });
    let right = rules[index + 1..].iter().find_map(|rule| match rule {
        CssRule::Style(rule) if has_live_selector(rule) => Some(true),
        CssRule::Style(rule) if rule.rules.is_empty() => None,
        _ => Some(false),
    });
    left == Some(true) && right == Some(true)
}

fn rule_list_has_output(rules: &[CssRule<'_>]) -> bool {
    rules.iter().any(|rule| match rule {
        CssRule::Style(rule) => has_live_selector(rule),
        CssRule::NestedDeclarations(rule) => !rule.declarations.is_output_empty(),
        CssRule::Media(rule) => rule_list_has_output(&rule.rules),
        CssRule::Supports(rule) => rule_list_has_output(&rule.rules),
        CssRule::Container(rule) => rule_list_has_output(&rule.rules),
        _ => true,
    })
}

fn prune_equal_selector_histories<'ast, 'scratch>(
    rules: &Vec<'ast, CssRule<'ast>>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> bool
where
    'ast: 'scratch,
{
    let mut changed = false;
    let mut segment_start = 0;
    while segment_start < rules.len() {
        let segment_end = next_segment_end(rules, segment_start);
        let mut history_indices =
            AdaptiveHashMap::<SelectorHistoryKey<'_, '_>, usize>::new_in(cx.allocator());
        let mut histories =
            std::vec::Vec::<std::vec::Vec<Ref<'ast, DeclarationBlock<'ast>>>>::new();

        for index in segment_start..segment_end {
            let CssRule::Style(rule) = &rules[index] else {
                continue;
            };
            if !has_live_selector(rule) {
                continue;
            }

            let key = SelectorHistoryKey {
                selectors: &rule.selectors,
                vendor_prefix: rule.vendor_prefix,
            };
            let history = if let Some(history) = history_indices.get(&key) {
                *history
            } else {
                let history = histories.len();
                history_indices.insert(key, history);
                histories.push(std::vec::Vec::new());
                history
            };
            collect_declaration_chain(&rule.declarations, &mut histories[history]);
        }

        for mut blocks in histories {
            if blocks.len() > 1 {
                let removed_before = cx.stats().declarations_removed;
                minifier.minify_sequence(&mut blocks, cx);
                changed |= cx.stats().declarations_removed != removed_before;
            }
        }
        segment_start = if segment_end == segment_start {
            segment_start + 1
        } else {
            segment_end + usize::from(segment_end < rules.len())
        };
    }
    changed
}

fn next_segment_end(rules: &[CssRule<'_>], start: usize) -> usize {
    let mut end = start;
    while end < rules.len() {
        match &rules[end] {
            CssRule::Style(rule) if has_live_selector(rule) || rule.rules.is_empty() => end += 1,
            _ => break,
        }
    }
    end
}

fn collect_declaration_chain<'ast>(
    tail: &std::pin::Pin<Box<'ast, DeclarationBlock<'ast>>>,
    blocks: &mut std::vec::Vec<Ref<'ast, DeclarationBlock<'ast>>>,
) {
    collect_declaration_ref_chain(Ref::from_pinned_box(tail), blocks);
}

fn collect_declaration_ref_chain<'ast>(
    tail: Ref<'ast, DeclarationBlock<'ast>>,
    blocks: &mut std::vec::Vec<Ref<'ast, DeclarationBlock<'ast>>>,
) {
    let chain_start = blocks.len();
    let mut current = Some(tail);
    while let Some(block) = current {
        blocks.push(block);
        current = block.get().get_ref().previous_merged();
    }
    blocks[chain_start..].reverse();
}

fn merge_same_selector_edges(rules: &mut Vec<'_, CssRule<'_>>) -> bool {
    let mut changed = false;
    let mut segment_start = 0;
    while segment_start < rules.len() {
        let segment_end = next_segment_end(rules, segment_start);
        let endpoints = (segment_start..segment_end)
            .filter(
                |&index| matches!(&rules[index], CssRule::Style(rule) if has_live_selector(rule)),
            )
            .collect::<std::vec::Vec<_>>();
        for edge in endpoints.windows(2) {
            let [left, right] = *edge else { unreachable!() };
            if !can_merge_same_selector(&rules[left], &rules[right]) {
                continue;
            }
            let (before, after) = rules.split_at_mut(right);
            let CssRule::Style(left_rule) = &mut before[left] else {
                unreachable!()
            };
            let CssRule::Style(right_rule) = &mut after[0] else {
                unreachable!()
            };
            right_rule
                .declarations
                .as_mut()
                .set_previous_merged(Some(Ref::from_pinned_box(&left_rule.declarations)));
            tombstone_selectors(left_rule);
            changed = true;
        }
        segment_start = if segment_end == segment_start {
            segment_start + 1
        } else {
            segment_end + usize::from(segment_end < rules.len())
        };
    }
    changed
}

fn can_merge_same_selector(previous: &CssRule<'_>, current: &CssRule<'_>) -> bool {
    let (CssRule::Style(previous), CssRule::Style(current)) = (previous, current) else {
        return false;
    };
    previous.rules.is_empty()
        && previous.vendor_prefix == current.vendor_prefix
        && current.declarations.previous_merged().is_none()
        && equal_live_selectors(&previous.selectors, &current.selectors)
}

fn factor_partial_edges<'ast>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    cx: &mut MinifyContext<'_>,
    cascade_scope: CascadeScope,
) -> bool {
    let mut plans = std::vec::Vec::new();
    let mut segment_start = 0;
    while segment_start < rules.len() {
        let segment_end = next_segment_end(rules, segment_start);
        let endpoints = (segment_start..segment_end)
            .filter(
                |&index| matches!(&rules[index], CssRule::Style(rule) if has_live_selector(rule)),
            )
            .collect::<std::vec::Vec<_>>();
        let mut last_selected_right = None;
        for edge in endpoints.windows(2) {
            let [left, right] = *edge else { unreachable!() };
            if last_selected_right.is_some_and(|selected| left <= selected) {
                continue;
            }
            if let Some(plan) = prepare_partial_plan(rules, left, right, cascade_scope) {
                last_selected_right = Some(right);
                plans.push(plan);
            }
        }
        segment_start = if segment_end == segment_start {
            segment_start + 1
        } else {
            segment_end + usize::from(segment_end < rules.len())
        };
    }
    let changed = !plans.is_empty();
    let mut insertions = std::vec::Vec::with_capacity(plans.len());
    for plan in plans {
        let right = plan.right;
        insertions.push((right, commit_partial_plan(rules, plan, cx)));
    }
    if changed {
        rebuild_after_partial_plans(rules, insertions);
    }
    changed
}

struct PartialPlan<'ast> {
    left: usize,
    right: usize,
    left_occurrences: std::vec::Vec<DeclarationOccurrence<'ast>>,
    right_occurrences: std::vec::Vec<DeclarationOccurrence<'ast>>,
    left_common_start: usize,
    right_common_start: usize,
    common_len: usize,
    selectors: Box<'ast, SelectorList<'ast>>,
    span: Span,
    vendor_prefix: VendorPrefix,
}

#[derive(Clone, Copy)]
struct DeclarationOccurrence<'ast> {
    block: Ref<'ast, DeclarationBlock<'ast>>,
    index: usize,
    history_context: DeclarationHistoryContext,
}

fn prepare_partial_plan<'ast>(
    rules: &Vec<'ast, CssRule<'ast>>,
    left: usize,
    right: usize,
    cascade_scope: CascadeScope,
) -> Option<PartialPlan<'ast>> {
    let (CssRule::Style(left_rule), CssRule::Style(right_rule)) = (&rules[left], &rules[right])
    else {
        return None;
    };
    if !left_rule.rules.is_empty()
        || left_rule.vendor_prefix != right_rule.vendor_prefix
        || equal_live_selectors(&left_rule.selectors, &right_rule.selectors)
        || selector_vendor_prefixes(&left_rule.selectors)
            != selector_vendor_prefixes(&right_rule.selectors)
    {
        return None;
    }

    let left_occurrences = declaration_occurrences(&left_rule.declarations, cascade_scope);
    let right_occurrences = declaration_occurrences(&right_rule.declarations, cascade_scope);

    let common = best_common_range(&left_occurrences, &right_occurrences);
    let (left_common_start, right_common_start, common_len) = match common {
        Some(common) => common,
        None if left_occurrences.is_empty()
            && right_occurrences.is_empty()
            && right_rule.rules.is_empty() =>
        {
            (0, 0, 0)
        }
        None => return None,
    };
    let allocator = rules.bump();
    let selectors = clone_selector_union(&left_rule.selectors, &right_rule.selectors, allocator)?;

    Some(PartialPlan {
        left,
        right,
        left_occurrences,
        right_occurrences,
        left_common_start,
        right_common_start,
        common_len,
        selectors,
        span: Span {
            start: left_rule.span.start.min(right_rule.span.start),
            end: left_rule.span.end.max(right_rule.span.end),
        },
        vendor_prefix: left_rule.vendor_prefix,
    })
}

fn declaration_occurrences<'ast>(
    tail: &std::pin::Pin<Box<'ast, DeclarationBlock<'ast>>>,
    cascade_scope: CascadeScope,
) -> std::vec::Vec<DeclarationOccurrence<'ast>> {
    let mut blocks = std::vec::Vec::new();
    collect_declaration_chain(tail, &mut blocks);
    let mut occurrences = std::vec::Vec::new();
    for block in blocks {
        for (index, (declaration, important)) in block.get().get_ref().iter().enumerate() {
            if !declaration.is_tombstone() {
                occurrences.push(DeclarationOccurrence {
                    block,
                    index,
                    history_context: cascade_scope.declaration_context(important),
                });
            }
        }
    }
    occurrences
}

fn best_common_range(
    left: &[DeclarationOccurrence<'_>],
    right: &[DeclarationOccurrence<'_>],
) -> Option<(usize, usize, usize)> {
    let mut best = None;
    for left_start in 0..left.len() {
        for right_start in 0..right.len() {
            let mut len = 0;
            while left_start + len < left.len()
                && right_start + len < right.len()
                && occurrences_equal(left[left_start + len], right[right_start + len])
            {
                len += 1;
            }
            if len == 0
                || best.is_some_and(|(_, _, best_len)| best_len >= len)
                || !movement_is_safe(left, right, left_start, right_start, len)
            {
                continue;
            }
            best = Some((left_start, right_start, len));
        }
    }
    best
}

fn occurrences_equal(left: DeclarationOccurrence<'_>, right: DeclarationOccurrence<'_>) -> bool {
    left.history_context == right.history_context
        && occurrence_declaration(left).eq_ignoring_tombstones(occurrence_declaration(right))
}

fn movement_is_safe(
    left: &[DeclarationOccurrence<'_>],
    right: &[DeclarationOccurrence<'_>],
    left_start: usize,
    right_start: usize,
    len: usize,
) -> bool {
    let common = &left[left_start..left_start + len];
    let crossed_left = &left[left_start + len..];
    let crossed_right = &right[..right_start];
    !common.iter().any(|common| {
        crossed_left
            .iter()
            .chain(crossed_right)
            .any(|crossed| declarations_conflict(*common, *crossed))
    })
}

fn declarations_conflict(
    left: DeclarationOccurrence<'_>,
    right: DeclarationOccurrence<'_>,
) -> bool {
    if left.history_context != right.history_context {
        return true;
    }
    let left = occurrence_declaration(left);
    let right = occurrence_declaration(right);
    let (Some(left_id), Some(right_id)) = (left.property_id(), right.property_id()) else {
        return true;
    };
    if left_id == right_id
        || matches!(left_id, PropertyId::All)
        || matches!(right_id, PropertyId::All)
    {
        return true;
    }
    if matches!(left, Declaration::Custom(_)) || matches!(right, Declaration::Custom(_)) {
        return false;
    }
    if matches!(left_id, PropertyId::Custom(_) | PropertyId::Unparsed)
        || matches!(right_id, PropertyId::Custom(_) | PropertyId::Unparsed)
    {
        return true;
    }

    property_relation(left_id)
        .zip(property_relation(right_id))
        .is_some_and(|(left, right)| {
            left.family == right.family && (left.shorthand || right.shorthand)
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PropertyFamily {
    Background,
    Border,
    Flex,
    Inset,
    Margin,
    Overflow,
    Padding,
    PlaceContent,
}

#[derive(Clone, Copy)]
struct PropertyRelation {
    family: PropertyFamily,
    shorthand: bool,
}

fn property_relation(property: PropertyId<'_>) -> Option<PropertyRelation> {
    let relation = match property {
        PropertyId::Background => (PropertyFamily::Background, true),
        PropertyId::BackgroundAttachment
        | PropertyId::BackgroundClip(_)
        | PropertyId::BackgroundColor
        | PropertyId::BackgroundImage
        | PropertyId::BackgroundOrigin
        | PropertyId::BackgroundPosition
        | PropertyId::BackgroundPositionX
        | PropertyId::BackgroundPositionY
        | PropertyId::BackgroundRepeat
        | PropertyId::BackgroundSize => (PropertyFamily::Background, false),
        PropertyId::Border
        | PropertyId::BorderBlock
        | PropertyId::BorderBlockColor
        | PropertyId::BorderBlockEnd
        | PropertyId::BorderBlockEndColor
        | PropertyId::BorderBlockEndStyle
        | PropertyId::BorderBlockEndWidth
        | PropertyId::BorderBlockStart
        | PropertyId::BorderBlockStartColor
        | PropertyId::BorderBlockStartStyle
        | PropertyId::BorderBlockStartWidth
        | PropertyId::BorderBlockStyle
        | PropertyId::BorderBlockWidth
        | PropertyId::BorderBottom
        | PropertyId::BorderBottomColor
        | PropertyId::BorderBottomStyle
        | PropertyId::BorderBottomWidth
        | PropertyId::BorderColor
        | PropertyId::BorderInline
        | PropertyId::BorderInlineColor
        | PropertyId::BorderInlineEnd
        | PropertyId::BorderInlineEndColor
        | PropertyId::BorderInlineEndStyle
        | PropertyId::BorderInlineEndWidth
        | PropertyId::BorderInlineStart
        | PropertyId::BorderInlineStartColor
        | PropertyId::BorderInlineStartStyle
        | PropertyId::BorderInlineStartWidth
        | PropertyId::BorderInlineStyle
        | PropertyId::BorderInlineWidth
        | PropertyId::BorderLeft
        | PropertyId::BorderLeftColor
        | PropertyId::BorderLeftStyle
        | PropertyId::BorderLeftWidth
        | PropertyId::BorderRight
        | PropertyId::BorderRightColor
        | PropertyId::BorderRightStyle
        | PropertyId::BorderRightWidth
        | PropertyId::BorderStyle
        | PropertyId::BorderTop
        | PropertyId::BorderTopColor
        | PropertyId::BorderTopStyle
        | PropertyId::BorderTopWidth
        | PropertyId::BorderWidth => (PropertyFamily::Border, true),
        PropertyId::Flex(_) => (PropertyFamily::Flex, true),
        PropertyId::FlexBasis(_) | PropertyId::FlexGrow(_) | PropertyId::FlexShrink(_) => {
            (PropertyFamily::Flex, false)
        }
        PropertyId::Inset | PropertyId::InsetBlock | PropertyId::InsetInline => {
            (PropertyFamily::Inset, true)
        }
        PropertyId::InsetBlockEnd
        | PropertyId::InsetBlockStart
        | PropertyId::InsetInlineEnd
        | PropertyId::InsetInlineStart
        | PropertyId::Bottom
        | PropertyId::Left
        | PropertyId::Right
        | PropertyId::Top => (PropertyFamily::Inset, false),
        PropertyId::Margin | PropertyId::MarginBlock | PropertyId::MarginInline => {
            (PropertyFamily::Margin, true)
        }
        PropertyId::MarginBlockEnd
        | PropertyId::MarginBlockStart
        | PropertyId::MarginBottom
        | PropertyId::MarginInlineEnd
        | PropertyId::MarginInlineStart
        | PropertyId::MarginLeft
        | PropertyId::MarginRight
        | PropertyId::MarginTop => (PropertyFamily::Margin, false),
        PropertyId::Overflow => (PropertyFamily::Overflow, true),
        PropertyId::OverflowX | PropertyId::OverflowY => (PropertyFamily::Overflow, false),
        PropertyId::Padding | PropertyId::PaddingBlock | PropertyId::PaddingInline => {
            (PropertyFamily::Padding, true)
        }
        PropertyId::PaddingBlockEnd
        | PropertyId::PaddingBlockStart
        | PropertyId::PaddingBottom
        | PropertyId::PaddingInlineEnd
        | PropertyId::PaddingInlineStart
        | PropertyId::PaddingLeft
        | PropertyId::PaddingRight
        | PropertyId::PaddingTop => (PropertyFamily::Padding, false),
        PropertyId::PlaceContent => (PropertyFamily::PlaceContent, true),
        PropertyId::AlignContent(_) | PropertyId::JustifyContent(_) => {
            (PropertyFamily::PlaceContent, false)
        }
        _ => return None,
    };
    Some(PropertyRelation {
        family: relation.0,
        shorthand: relation.1,
    })
}

fn commit_partial_plan<'ast>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    plan: PartialPlan<'ast>,
    cx: &mut MinifyContext<'_>,
) -> CssRule<'ast> {
    let allocator = rules.bump();
    let mut shared_declarations = DeclarationBlock::new(allocator);

    for offset in 0..plan.common_len {
        let left = plan.left_occurrences[plan.left_common_start + offset];
        let right = plan.right_occurrences[plan.right_common_start + offset];
        let declaration = replace_occurrence(left, Declaration::Tombstone);
        replace_occurrence(right, Declaration::Tombstone);
        shared_declarations.push(declaration, left.history_context.is_important());
        cx.record_declaration_removed();
    }

    let shared = CssRule::Style(allocator.boxed(StyleRule {
        declarations: allocator.pinned(shared_declarations),
        span: plan.span,
        rules: allocator.vec(),
        selectors: plan.selectors,
        vendor_prefix: plan.vendor_prefix,
    }));

    let left_empty = declaration_sequence_is_empty(&plan.left_occurrences);
    let right_empty = declaration_sequence_is_empty(&plan.right_occurrences);
    if left_empty {
        let CssRule::Style(left) = &mut rules[plan.left] else {
            unreachable!()
        };
        if left.rules.is_empty() {
            tombstone_selectors(left);
        }
    }
    if right_empty {
        let CssRule::Style(right) = &mut rules[plan.right] else {
            unreachable!()
        };
        if right.rules.is_empty() {
            tombstone_selectors(right);
        }
    }
    shared
}

fn rebuild_after_partial_plans<'ast>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    insertions: std::vec::Vec<(usize, CssRule<'ast>)>,
) {
    let allocator = rules.bump();
    let capacity = rules.len() + insertions.len();
    let old_rules = std::mem::replace(rules, Vec::with_capacity_in(capacity, allocator));
    let mut insertions = insertions.into_iter().peekable();

    for (index, rule) in old_rules.into_iter().enumerate() {
        while insertions
            .peek()
            .is_some_and(|(insertion_index, _)| *insertion_index == index)
        {
            let (_, shared) = insertions.next().expect("peeked insertion exists");
            rules.push(shared);
        }
        if !matches!(&rule, CssRule::Style(style)
            if style.rules.is_empty() && !has_live_selector(style))
        {
            rules.push(rule);
        }
    }
    debug_assert!(insertions.next().is_none());
}

fn occurrence_declaration(occurrence: DeclarationOccurrence<'_>) -> &Declaration<'_> {
    &occurrence.block.get().get_ref().declarations[occurrence.index]
}

fn replace_occurrence<'ast>(
    mut occurrence: DeclarationOccurrence<'ast>,
    replacement: Declaration<'ast>,
) -> Declaration<'ast> {
    // SAFETY: a partial-merge commit owns the only mutation phase. Each
    // occurrence is present in exactly one endpoint sequence and is replaced
    // once before any later graph work is discovered.
    let block = unsafe { occurrence.block.get_mut_unchecked().get_unchecked_mut() };
    std::mem::replace(&mut block.declarations[occurrence.index], replacement)
}

fn declaration_sequence_is_empty(occurrences: &[DeclarationOccurrence<'_>]) -> bool {
    occurrences
        .iter()
        .all(|occurrence| occurrence_declaration(*occurrence).is_tombstone())
}

fn has_live_selector(rule: &StyleRule<'_>) -> bool {
    rule.selectors
        .iter()
        .any(|selector| !selector.is_tombstone())
}

fn equal_live_selectors(left: &SelectorList<'_>, right: &SelectorList<'_>) -> bool {
    left.iter()
        .filter(|selector| !selector.is_tombstone())
        .eq(right.iter().filter(|selector| !selector.is_tombstone()))
}

fn tombstone_selectors(rule: &mut StyleRule<'_>) {
    for selector in rule.selectors.iter_mut() {
        *selector = Selector::Tombstone;
    }
}

fn clone_selector_union<'ast>(
    left: &SelectorList<'ast>,
    right: &SelectorList<'ast>,
    allocator: &'ast Allocator,
) -> Option<Box<'ast, SelectorList<'ast>>> {
    let mut union = allocator.vec();
    let mut seen = AdaptiveHashSet::<_, 4>::new_in(allocator);
    for selector in left
        .iter()
        .chain(right)
        .filter(|selector| !selector.is_tombstone())
    {
        if !seen.insert(selector) {
            continue;
        }
        union.push(clone_selector(selector, allocator)?);
    }
    Some(allocator.boxed(union))
}

fn clone_selector<'ast>(
    selector: &Selector<'ast>,
    allocator: &'ast Allocator,
) -> Option<Selector<'ast>> {
    let Selector::Parsed(components) = selector else {
        return None;
    };
    let mut cloned = allocator.vec();
    for component in components {
        cloned.push(clone_selector_component(component, allocator)?);
    }
    Some(Selector::Parsed(cloned))
}

fn clone_selector_component<'ast>(
    component: &SelectorComponent<'ast>,
    allocator: &'ast Allocator,
) -> Option<SelectorComponent<'ast>> {
    Some(match component {
        SelectorComponent::Combinator(rocketcss_ast::Combinator::Descendant) => {
            SelectorComponent::Combinator(rocketcss_ast::Combinator::Descendant)
        }
        SelectorComponent::ExplicitAnyNamespace => SelectorComponent::ExplicitAnyNamespace,
        SelectorComponent::ExplicitNoNamespace => SelectorComponent::ExplicitNoNamespace,
        SelectorComponent::DefaultNamespace(value) => SelectorComponent::DefaultNamespace(value),
        SelectorComponent::Namespace { prefix, url } => {
            SelectorComponent::Namespace { prefix, url }
        }
        SelectorComponent::ExplicitUniversalType => SelectorComponent::ExplicitUniversalType,
        SelectorComponent::LocalName { name, lower_name } => {
            SelectorComponent::LocalName { name, lower_name }
        }
        SelectorComponent::Id(value) => SelectorComponent::Id(value),
        SelectorComponent::Class(value) => SelectorComponent::Class(value),
        SelectorComponent::Root => SelectorComponent::Root,
        SelectorComponent::Empty => SelectorComponent::Empty,
        SelectorComponent::Scope => SelectorComponent::Scope,
        SelectorComponent::Nth(value) => SelectorComponent::Nth(*value),
        SelectorComponent::PseudoElement(value) => {
            SelectorComponent::PseudoElement(allocator.boxed(clone_simple_pseudo_element(value)?))
        }
        SelectorComponent::Part(values) => {
            let mut cloned = allocator.vec();
            cloned.extend_from_slice_copy(values);
            SelectorComponent::Part(cloned)
        }
        SelectorComponent::Nesting => SelectorComponent::Nesting,
        // Selector functions and CSS Modules contexts require a complete
        // recursive origin/serialization-context clone. Until that clone is
        // available they are conservative opaque endpoints for S3.
        _ => return None,
    })
}

fn clone_simple_pseudo_element<'ast>(pseudo: &PseudoElement<'ast>) -> Option<PseudoElement<'ast>> {
    Some(match pseudo {
        PseudoElement::After => PseudoElement::After,
        PseudoElement::Before => PseudoElement::Before,
        PseudoElement::FirstLine => PseudoElement::FirstLine,
        PseudoElement::FirstLetter => PseudoElement::FirstLetter,
        PseudoElement::DetailsContent => PseudoElement::DetailsContent,
        PseudoElement::TargetText => PseudoElement::TargetText,
        PseudoElement::SearchText => PseudoElement::SearchText,
        PseudoElement::Selection(prefix) => PseudoElement::Selection(*prefix),
        PseudoElement::Placeholder(prefix) => PseudoElement::Placeholder(*prefix),
        PseudoElement::HighlightFunction { name } => PseudoElement::HighlightFunction { name },
        PseudoElement::Marker => PseudoElement::Marker,
        PseudoElement::Backdrop(prefix) => PseudoElement::Backdrop(*prefix),
        PseudoElement::FileSelectorButton(prefix) => PseudoElement::FileSelectorButton(*prefix),
        PseudoElement::WebKitScrollbar(value) => PseudoElement::WebKitScrollbar(*value),
        PseudoElement::Cue => PseudoElement::Cue,
        PseudoElement::CueRegion => PseudoElement::CueRegion,
        PseudoElement::ViewTransition => PseudoElement::ViewTransition,
        PseudoElement::PickerFunction { identifier } => {
            PseudoElement::PickerFunction { identifier }
        }
        PseudoElement::PickerIcon => PseudoElement::PickerIcon,
        PseudoElement::Checkmark => PseudoElement::Checkmark,
        PseudoElement::GrammarError => PseudoElement::GrammarError,
        PseudoElement::SpellingError => PseudoElement::SpellingError,
        PseudoElement::Custom { .. } => return None,
        _ => return None,
    })
}

fn selector_vendor_prefixes(selectors: &SelectorList<'_>) -> u8 {
    selectors
        .iter()
        .filter_map(Selector::as_parsed)
        .flatten()
        .fold(0, |prefixes, component| {
            prefixes
                | match component {
                    SelectorComponent::PseudoElement(pseudo) => match &**pseudo {
                        PseudoElement::Selection(prefix)
                        | PseudoElement::Placeholder(prefix)
                        | PseudoElement::Backdrop(prefix)
                        | PseudoElement::FileSelectorButton(prefix) => {
                            prefix.bits() & !VendorPrefix::NONE.bits()
                        }
                        _ => 0,
                    },
                    _ => 0,
                }
        })
}
