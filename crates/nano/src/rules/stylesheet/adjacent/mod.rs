mod cleanup;
mod history;
mod partial;
mod scheduler;
mod state;

use std::hash::{Hash, Hasher};

use ahash::AHasher;
use rocketcss_allocator::{Ref, vec::Vec};
use rocketcss_ast::{
    Combinator, CssRule, DeclarationBlock, PseudoElement, Selector, SelectorComponent,
    SelectorList, StyleRule, VendorPrefix,
};

use super::DeclarationBlockMinifier;
use crate::{MinifyContext, Options, OptionsOp};
use scheduler::stabilize_rule_list;
use state::{CascadeScope, HistoryTraversal, SelectorSummary};

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
        remove_empty_barriers(rules);
        stabilize_rule_list(rules, minifier, cx, cascade_scope);

        if coalesce_adjacent_conditional_rules(rules) {
            for rule in rules.iter_mut() {
                merge_children(rule, minifier, cx, cascade_scope, history_traversal);
            }
            continue;
        }
        if remove_empty_barriers(rules) {
            continue;
        }
        break;
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

fn collect_declaration_chain<'ast>(
    tail: &std::pin::Pin<rocketcss_allocator::boxed::Box<'ast, DeclarationBlock<'ast>>>,
    blocks: &mut std::vec::Vec<Ref<'ast, DeclarationBlock<'ast>>>,
) {
    let chain_start = blocks.len();
    let mut current = Some(Ref::from_pinned_box(tail));
    while let Some(block) = current {
        blocks.push(block);
        current = block.get().get_ref().previous_merged();
    }
    blocks[chain_start..].reverse();
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

fn summarize_selectors(selectors: &SelectorList<'_>) -> SelectorSummary {
    let mut hasher = AHasher::default();
    let mut live_len = 0usize;
    let mut vendor_prefixes = 0;
    let mut materializable = true;
    for selector in selectors.iter().filter(|selector| !selector.is_tombstone()) {
        selector.hash(&mut hasher);
        live_len += 1;
        let Selector::Parsed(components) = selector else {
            materializable = false;
            continue;
        };
        for component in components {
            materializable &= selector_component_is_materializable(component);
            vendor_prefixes |= selector_component_vendor_prefixes(component);
        }
    }
    live_len.hash(&mut hasher);
    SelectorSummary {
        hash: hasher.finish(),
        live_len: u32::try_from(live_len).expect("selector list length exceeds u32"),
        vendor_prefixes,
        materializable,
    }
}

fn selector_component_is_materializable(component: &SelectorComponent<'_>) -> bool {
    match component {
        SelectorComponent::Combinator(Combinator::Descendant)
        | SelectorComponent::ExplicitAnyNamespace
        | SelectorComponent::ExplicitNoNamespace
        | SelectorComponent::DefaultNamespace(_)
        | SelectorComponent::Namespace { .. }
        | SelectorComponent::ExplicitUniversalType
        | SelectorComponent::LocalName { .. }
        | SelectorComponent::Id(_)
        | SelectorComponent::Class(_)
        | SelectorComponent::Root
        | SelectorComponent::Empty
        | SelectorComponent::Scope
        | SelectorComponent::Nth(_)
        | SelectorComponent::Part(_)
        | SelectorComponent::Nesting => true,
        SelectorComponent::PseudoElement(pseudo) => simple_pseudo_element_is_materializable(pseudo),
        _ => false,
    }
}

fn simple_pseudo_element_is_materializable(pseudo: &PseudoElement<'_>) -> bool {
    matches!(
        pseudo,
        PseudoElement::After
            | PseudoElement::Before
            | PseudoElement::FirstLine
            | PseudoElement::FirstLetter
            | PseudoElement::DetailsContent
            | PseudoElement::TargetText
            | PseudoElement::SearchText
            | PseudoElement::Selection(_)
            | PseudoElement::Placeholder(_)
            | PseudoElement::HighlightFunction { .. }
            | PseudoElement::Marker
            | PseudoElement::Backdrop(_)
            | PseudoElement::FileSelectorButton(_)
            | PseudoElement::WebKitScrollbar(_)
            | PseudoElement::Cue
            | PseudoElement::CueRegion
            | PseudoElement::ViewTransition
            | PseudoElement::PickerFunction { .. }
            | PseudoElement::PickerIcon
            | PseudoElement::Checkmark
            | PseudoElement::GrammarError
            | PseudoElement::SpellingError
    )
}

fn selector_component_vendor_prefixes(component: &SelectorComponent<'_>) -> u8 {
    let SelectorComponent::PseudoElement(pseudo) = component else {
        return 0;
    };
    match &**pseudo {
        PseudoElement::Selection(prefix)
        | PseudoElement::Placeholder(prefix)
        | PseudoElement::Backdrop(prefix)
        | PseudoElement::FileSelectorButton(prefix) => prefix.bits() & !VendorPrefix::NONE.bits(),
        _ => 0,
    }
}
