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
        CssRule::Style(rule) => {
            merge_adjacent_style_rules(rule.as_mut().rules_mut(), token, minifier, cx)
        }
        CssRule::Supports(rule) => merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx),
        CssRule::MozDocument(rule) => {
            merge_adjacent_style_rules(&mut rule.rules, token, minifier, cx)
        }
        CssRule::Nesting(rule) => {
            merge_adjacent_style_rules(rule.style.as_mut().rules_mut(), token, minifier, cx)
        }
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
    let previous = previous.as_ref().get_ref();
    let current = current.as_ref().get_ref();
    previous.rules.is_empty()
        && previous.vendor_prefix == current.vendor_prefix
        && has_live_selector(previous)
        && has_live_selector(current)
        && current
            .declarations
            .as_ref()
            .borrow(token)
            .previous_merged()
            .is_none()
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
    for rule in run.iter() {
        let CssRule::Style(style) = rule else {
            unreachable!("eligible runs contain only style rules")
        };
        let style = style.as_ref().get_ref();
        blocks.push(Ref::from(&style.declarations));
    }
    minifier.minify_sequence(&blocks, token, cx);

    let run_len = run.len();
    for (index, rule) in run.iter_mut().enumerate() {
        let CssRule::Style(style) = rule else {
            unreachable!("eligible runs contain only style rules")
        };
        if index > 0 {
            let previous = blocks[index - 1];
            style
                .as_ref()
                .get_ref()
                .declarations
                .as_ref()
                .borrow_mut(token)
                .get_mut()
                .set_previous_merged(Some(previous));
        }
        if index + 1 < run_len {
            for selector in style.as_mut().selectors_mut().iter_mut() {
                *selector = Selector::Tombstone;
            }
        }
    }
}
