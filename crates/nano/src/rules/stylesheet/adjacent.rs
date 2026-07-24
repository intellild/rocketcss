use rocketcss_allocator::{Ref, vec::Vec};
use rocketcss_ast::{CssRule, Selector, SelectorList, StyleRule};

use super::DeclarationBlockMinifier;
use crate::{MinifyContext, Options, OptionsOp};

pub(crate) fn merge_adjacent_style_rules<'ast, 'scratch>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    for rule in rules.iter_mut() {
        merge_children(rule, minifier, cx);
    }
    if !cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
        return;
    }
    merge_current_list(rules, minifier, cx);
}

fn merge_children<'ast, 'scratch>(
    rule: &mut CssRule<'ast>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    let children = match rule {
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
    if let Some(children) = children {
        merge_adjacent_style_rules(children, minifier, cx);
    }
}

fn merge_current_list<'ast, 'scratch>(
    rules: &mut Vec<'ast, CssRule<'ast>>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    let mut start = 0;
    while start < rules.len() {
        let mut end = start;
        while end + 1 < rules.len() && can_merge(&rules[end], &rules[end + 1]) {
            end += 1;
        }
        if end > start {
            merge_run(&mut rules[start..=end], minifier, cx);
        }
        start = end + 1;
    }
}

fn can_merge(previous: &CssRule<'_>, current: &CssRule<'_>) -> bool {
    let (CssRule::Style(previous), CssRule::Style(current)) = (previous, current) else {
        return false;
    };
    previous.rules.is_empty()
        && previous.vendor_prefix == current.vendor_prefix
        && has_live_selector(previous)
        && has_live_selector(current)
        && current.previous_merged().is_none()
        && equal_live_selectors(&previous.selectors, &current.selectors)
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

fn merge_run<'ast, 'scratch>(
    run: &mut [CssRule<'ast>],
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    let mut blocks = minifier.allocator().vec();
    let mut styles = minifier.allocator().vec();
    for rule in run.iter_mut() {
        let CssRule::Style(style) = rule else {
            unreachable!("eligible runs contain only style rules")
        };
        styles.push(Ref::from_pinned_box(style));
        blocks.push(Ref::from_pin(std::pin::Pin::new(
            style.as_mut().declarations_mut(),
        )));
    }
    minifier.minify_sequence(&mut blocks, cx);

    let run_len = run.len();
    for (index, rule) in run.iter_mut().enumerate() {
        let CssRule::Style(style) = rule else {
            unreachable!("eligible runs contain only style rules")
        };
        if index > 0 {
            style.as_mut().set_previous_merged(Some(styles[index - 1]));
        }
        if index + 1 < run_len {
            for selector in style.as_mut().selectors_mut().iter_mut() {
                *selector = Selector::Tombstone;
            }
        }
    }
}
