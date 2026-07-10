use rs_css_allocator::vec::Vec;
use rs_css_ast::{CssRule, DeclarationBlock, KeyframeSelector, KeyframesRule, StyleRule};

use crate::{MinifyContext, css_rule, properties};

pub(crate) fn minify_keyframe_selector(selector: &mut KeyframeSelector<'_>) -> bool {
    match selector {
        KeyframeSelector::From => {
            *selector = KeyframeSelector::Percentage(0.0);
            true
        }
        KeyframeSelector::Percentage(value) if *value == 1.0 => {
            *selector = KeyframeSelector::To;
            true
        }
        _ => false,
    }
}

pub(crate) fn minify_keyframes<'a>(
    keyframes: &mut KeyframesRule<'a>,
    context: &mut MinifyContext<'a>,
) {
    if context.options().discard_duplicates {
        for keyframe in keyframes.keyframes.iter_mut() {
            keyframe.selectors.dedup();
        }
    }
    if !context.options().merge_rules {
        return;
    }
    let mut index = 0;
    while index < keyframes.keyframes.len() {
        let mut previous_index = 0;
        let mut did_merge = false;
        while previous_index < index {
            let (previous, current) = keyframes.keyframes.split_at_mut(index);
            let previous = &mut previous[previous_index];
            let current = &mut current[0];
            if css_rule::declaration_blocks_equal(&previous.declarations, &current.declarations) {
                previous.selectors.extend(current.selectors.drain(..));
                previous.selectors.dedup();
                keyframes.keyframes.remove(index);
                context.record_rule_merged();
                did_merge = true;
                break;
            }
            previous_index += 1;
        }
        if !did_merge {
            index += 1;
        }
    }
}

pub(crate) fn minify_rule_list<'a>(
    rules: &mut Vec<'a, CssRule<'a>>,
    context: &mut MinifyContext<'a>,
) {
    let old = std::mem::replace(rules, context.allocator().vec());
    for mut rule in old {
        minify_nested_rules(&mut rule, context);
        if css_rule::is_empty(&rule) {
            context.record_rule_removed();
            continue;
        }
        if context.options().merge_rules
            && rules
                .last_mut()
                .is_some_and(|previous| try_merge(previous, &mut rule, context))
        {
            context.record_rule_merged();
            continue;
        }
        rules.push(rule);
    }

    if context.options().merge_rules {
        merge_named_layers(rules, context);
    }
    if context.options().discard_duplicates {
        discard_repeated_style_declarations(rules, context);
        discard_duplicate_rules(rules, context);
    }
}

fn minify_nested_rules<'a>(rule: &mut CssRule<'a>, context: &mut MinifyContext<'a>) {
    match rule {
        CssRule::Media(rule) => minify_rule_list(&mut rule.rules, context),
        CssRule::Style(rule) => minify_rule_list(&mut rule.rules, context),
        CssRule::Supports(rule) => minify_rule_list(&mut rule.rules, context),
        CssRule::MozDocument(rule) => minify_rule_list(&mut rule.rules, context),
        CssRule::Nesting(rule) => minify_rule_list(&mut rule.style.rules, context),
        CssRule::LayerBlock(rule) => minify_rule_list(&mut rule.rules, context),
        CssRule::Container(rule) => minify_rule_list(&mut rule.rules, context),
        CssRule::Scope(rule) => minify_rule_list(&mut rule.rules, context),
        CssRule::StartingStyle(rule) => minify_rule_list(&mut rule.rules, context),
        _ => {}
    }
}

fn try_merge<'a>(
    previous: &mut CssRule<'a>,
    current: &mut CssRule<'a>,
    context: &mut MinifyContext<'a>,
) -> bool {
    match (previous, current) {
        (CssRule::Style(previous), CssRule::Style(current))
            if previous.vendor_prefix == current.vendor_prefix
                && previous.selectors == current.selectors =>
        {
            append_declarations(
                &mut previous.declarations,
                &mut current.declarations,
                context,
            );
            previous.rules.extend(current.rules.drain(..));
            minify_rule_list(&mut previous.rules, context);
            true
        }
        (CssRule::Style(previous), CssRule::Style(current))
            if previous.vendor_prefix == current.vendor_prefix
                && previous.rules.is_empty()
                && current.rules.is_empty()
                && css_rule::declaration_blocks_equal(
                    &previous.declarations,
                    &current.declarations,
                ) =>
        {
            previous.selectors.extend(current.selectors.drain(..));
            previous.selectors.dedup();
            true
        }
        (CssRule::Media(previous), CssRule::Media(current)) if previous.query == current.query => {
            previous.rules.extend(current.rules.drain(..));
            minify_rule_list(&mut previous.rules, context);
            true
        }
        (CssRule::Supports(previous), CssRule::Supports(current))
            if previous.condition == current.condition =>
        {
            previous.rules.extend(current.rules.drain(..));
            minify_rule_list(&mut previous.rules, context);
            true
        }
        (CssRule::Container(previous), CssRule::Container(current))
            if previous.name == current.name && previous.condition == current.condition =>
        {
            previous.rules.extend(current.rules.drain(..));
            minify_rule_list(&mut previous.rules, context);
            true
        }
        (CssRule::LayerBlock(previous), CssRule::LayerBlock(current))
            if previous.name == current.name =>
        {
            previous.rules.extend(current.rules.drain(..));
            minify_rule_list(&mut previous.rules, context);
            true
        }
        (CssRule::Scope(previous), CssRule::Scope(current))
            if previous.scope_start == current.scope_start
                && previous.scope_end == current.scope_end =>
        {
            previous.rules.extend(current.rules.drain(..));
            minify_rule_list(&mut previous.rules, context);
            true
        }
        (CssRule::StartingStyle(previous), CssRule::StartingStyle(current)) => {
            previous.rules.extend(current.rules.drain(..));
            minify_rule_list(&mut previous.rules, context);
            true
        }
        (CssRule::LayerStatement(previous), CssRule::LayerStatement(current)) => {
            previous.names.extend(current.names.drain(..));
            previous.names.dedup();
            true
        }
        _ => false,
    }
}

fn append_declarations<'a>(
    destination: &mut DeclarationBlock<'a>,
    source: &mut DeclarationBlock<'a>,
    context: &mut MinifyContext<'a>,
) {
    let source = std::mem::replace(source, DeclarationBlock::new(context.allocator()));
    for (declaration, important) in source
        .declarations
        .into_iter()
        .zip(source.declarations_importance.iter())
    {
        destination.push(declaration, important);
    }
    properties::minify_declaration_block(destination, context);
}

fn discard_repeated_style_declarations<'a>(
    rules: &mut Vec<'a, CssRule<'a>>,
    context: &mut MinifyContext<'a>,
) {
    let mut current_index = 0;
    while current_index < rules.len() {
        let mut previous_index = 0;
        while previous_index < current_index {
            let (before, current) = rules.split_at_mut(current_index);
            let Some((previous, current)) =
                same_style_pair(&mut before[previous_index], &current[0])
            else {
                previous_index += 1;
                continue;
            };
            let removed = remove_declarations_present_in(previous, current, context);
            if removed > 0 {
                context.record_declarations_removed(removed);
            }
            if previous.declarations.is_empty() && previous.rules.is_empty() {
                rules.remove(previous_index);
                context.record_rule_removed();
                current_index -= 1;
            } else {
                previous_index += 1;
            }
        }
        current_index += 1;
    }
}

fn same_style_pair<'a, 'b>(
    previous: &'b mut CssRule<'a>,
    current: &'b CssRule<'a>,
) -> Option<(&'b mut StyleRule<'a>, &'b StyleRule<'a>)> {
    let (CssRule::Style(previous), CssRule::Style(current)) = (previous, current) else {
        return None;
    };
    (previous.vendor_prefix == current.vendor_prefix
        && previous.selectors == current.selectors
        && previous.rules.is_empty()
        && current.rules.is_empty())
    .then_some((previous, current))
}

fn remove_declarations_present_in<'a>(
    previous: &mut StyleRule<'a>,
    current: &StyleRule<'a>,
    context: &MinifyContext<'a>,
) -> usize {
    let old = std::mem::replace(
        &mut *previous.declarations,
        DeclarationBlock::new(context.allocator()),
    );
    let before = old.len();
    for (declaration, important) in old
        .declarations
        .into_iter()
        .zip(old.declarations_importance.iter())
    {
        if !current
            .declarations
            .iter()
            .any(|(candidate, candidate_important)| {
                candidate_important == important && candidate == &declaration
            })
        {
            previous.declarations.push(declaration, important);
        }
    }
    before - previous.declarations.len()
}

fn discard_duplicate_rules<'a>(rules: &mut Vec<'a, CssRule<'a>>, context: &mut MinifyContext<'a>) {
    let mut current_index = 0;
    while current_index < rules.len() {
        let mut previous_index = 0;
        while previous_index < current_index {
            if css_rule::same_rule(&rules[previous_index], &rules[current_index])
                || css_rule::is_overridden_by(&rules[previous_index], &rules[current_index])
            {
                rules.remove(previous_index);
                context.record_rule_removed();
                current_index -= 1;
            } else {
                previous_index += 1;
            }
        }
        current_index += 1;
    }
}

fn merge_named_layers<'a>(rules: &mut Vec<'a, CssRule<'a>>, context: &mut MinifyContext<'a>) {
    let mut index = 0;
    while index < rules.len() {
        let mut previous_index = 0;
        let mut merged = false;
        while previous_index < index {
            let (previous, current) = rules.split_at_mut(index);
            let (CssRule::LayerBlock(previous), CssRule::LayerBlock(current)) =
                (&mut previous[previous_index], &mut current[0])
            else {
                previous_index += 1;
                continue;
            };
            if previous.name.is_some() && previous.name == current.name {
                previous.rules.extend(current.rules.drain(..));
                minify_rule_list(&mut previous.rules, context);
                rules.remove(index);
                context.record_rule_merged();
                merged = true;
                break;
            }
            previous_index += 1;
        }
        if !merged {
            index += 1;
        }
    }
}
