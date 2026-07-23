use rocketcss_allocator::{GhostToken, Ref, vec::Vec};
use rocketcss_ast::{CssRule, Selector, SelectorList, StyleRule};

use super::DeclarationBlockMinifier;
use crate::{MinifyContext, Options, OptionsOp};

pub(crate) fn merge_adjacent_style_rules<'ast, 'ghost, 'scratch>(
    rules: &mut Vec<'ast, CssRule<'ast, 'ghost>>,
    token: &mut GhostToken<'ghost>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    for rule in rules.iter_mut() {
        merge_children(rule, token, minifier, cx);
    }
    if !cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
        return;
    }
    merge_current_list(rules, token, minifier, cx);
}

fn merge_children<'ast, 'ghost, 'scratch>(
    rule: &mut CssRule<'ast, 'ghost>,
    token: &mut GhostToken<'ghost>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    match rule {
        CssRule::Media(rule) => merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx),
        CssRule::Style(rule) => merge_style_children(*rule, token, minifier, cx),
        CssRule::Supports(rule) => merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx),
        CssRule::MozDocument(rule) => {
            merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx)
        }
        CssRule::Nesting(rule) => merge_style_children(rule.style, token, minifier, cx),
        CssRule::LayerBlock(rule) => {
            merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx)
        }
        CssRule::Container(rule) => {
            merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx)
        }
        CssRule::Scope(rule) => merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx),
        CssRule::StartingStyle(rule) => {
            merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx)
        }
        _ => {}
    }
}

fn merge_style_children<'ast, 'ghost, 'scratch>(
    reference: Ref<'ast, 'ghost, StyleRule<'ast, 'ghost>>,
    token: &mut GhostToken<'ghost>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    let mut children = {
        let mut style = reference.get_mut(token);
        // SAFETY: replacing the rules vector does not move the pinned style.
        let style = unsafe { style.as_mut().get_unchecked_mut() };
        let empty = Vec::new_in(style.rules.bump());
        std::mem::replace(&mut style.rules, empty)
    };
    merge_adjacent_style_rules(&mut children, token, minifier, cx);
    let mut style = reference.get_mut(token);
    // SAFETY: restoring the rules vector does not move the pinned style.
    let style = unsafe { style.as_mut().get_unchecked_mut() };
    debug_assert!(style.rules.is_empty());
    style.rules = children;
}

fn merge_current_list<'ast, 'ghost, 'scratch>(
    rules: &mut Vec<'ast, CssRule<'ast, 'ghost>>,
    token: &mut GhostToken<'ghost>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    let mut start = 0;
    while start < rules.len() {
        let mut end = start;
        while end + 1 < rules.len() && can_merge(&rules[end], &rules[end + 1], token) {
            end += 1;
        }
        if end > start {
            merge_run(&mut rules[start..=end], token, minifier, cx);
        }
        start = end + 1;
    }
}

fn can_merge<'ghost>(
    previous: &CssRule<'_, 'ghost>,
    current: &CssRule<'_, 'ghost>,
    token: &GhostToken<'ghost>,
) -> bool {
    let (CssRule::Style(previous), CssRule::Style(current)) = (previous, current) else {
        return false;
    };
    let previous = previous.get(token);
    let current = current.get(token);
    previous.rules.is_empty()
        && previous.vendor_prefix == current.vendor_prefix
        && has_live_selector(previous.get_ref())
        && has_live_selector(current.get_ref())
        && current.previous_merged().is_none()
        && equal_live_selectors(&previous.selectors, &current.selectors)
}

fn has_live_selector(rule: &StyleRule<'_, '_>) -> bool {
    rule.selectors
        .iter()
        .any(|selector| !selector.is_tombstone())
}

fn equal_live_selectors(left: &SelectorList<'_>, right: &SelectorList<'_>) -> bool {
    left.iter()
        .filter(|selector| !selector.is_tombstone())
        .eq(right.iter().filter(|selector| !selector.is_tombstone()))
}

fn merge_run<'ast, 'ghost, 'scratch>(
    run: &mut [CssRule<'ast, 'ghost>],
    token: &mut GhostToken<'ghost>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    let mut blocks = minifier.allocator().vec();
    let mut styles = minifier.allocator().vec();
    for rule in run.iter() {
        let CssRule::Style(style) = rule else {
            unreachable!("eligible runs contain only style rules")
        };
        styles.push(*style);
        blocks.push(style.get(token).declarations);
    }
    minifier.minify_sequence(&blocks, token, cx);

    let run_len = run.len();
    for (index, rule) in run.iter_mut().enumerate() {
        let CssRule::Style(style) = rule else {
            unreachable!("eligible runs contain only style rules")
        };
        if index > 0 {
            style
                .get_mut(token)
                .set_previous_merged(Some(styles[index - 1]));
        }
        if index + 1 < run_len {
            for selector in style.get_mut(token).selectors_mut().iter_mut() {
                *selector = Selector::Tombstone;
            }
        }
    }
}
