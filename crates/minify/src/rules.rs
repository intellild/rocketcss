use std::ptr::NonNull;

use rocketcss_allocator::{hash_map::HashMap, vec::Vec};
use rocketcss_ast::{
    CssRule, CustomProperty, DeclarationBlock, EnvironmentVariable, Function, KeyframeSelector,
    StyleSheet, UnknownAtRule, UnparsedProperty, Variable, VendorPrefix,
};

use crate::{Minify, MinifyContext};

impl Minify for StyleSheet<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        crate::minify_style_sheet(self, context);
    }
}

impl Minify for KeyframeSelector<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if !context.options().normalize_values {
            return;
        }

        let changed = match self {
            KeyframeSelector::From => {
                *self = KeyframeSelector::Percentage(0.0);
                true
            }
            KeyframeSelector::Percentage(value) if *value == 1.0 => {
                *self = KeyframeSelector::To;
                true
            }
            _ => false,
        };
        if changed {
            context.record_value_normalized();
        }
    }
}

impl Minify for UnparsedProperty<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        self.value.minify(context);
    }
}

impl Minify for CustomProperty<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        self.value.minify(context);
    }
}

impl Minify for Function<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        self.arguments.minify(context);
    }
}

impl Minify for Variable<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(context);
        }
    }
}

impl Minify for EnvironmentVariable<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(context);
        }
    }
}

impl Minify for UnknownAtRule<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        self.prelude.minify(context);
        if let Some(block) = &mut self.block {
            block.minify(context);
        }
    }
}

/// Coalesces adjacent equivalent conditional rules before their contents are
/// minified. This is the fast path used by the local Lightning CSS branch: a
/// run of conditional blocks is appended once and recursively processed once,
/// rather than repeatedly minifying a growing accumulated rule list.
pub(crate) fn coalesce_conditional_rules<'a>(
    rules: &mut Vec<'a, CssRule<'a>>,
    context: &mut MinifyContext,
) {
    let mut previous_live = None;
    for index in 0..rules.len() {
        if matches!(rules[index], CssRule::Ignored) {
            continue;
        }

        let merge = previous_live.is_some_and(|previous_index| {
            match (&rules[previous_index], &rules[index]) {
                (CssRule::Media(left), CssRule::Media(right)) => left.query == right.query,
                (CssRule::Supports(left), CssRule::Supports(right)) => {
                    left.condition == right.condition
                }
                (CssRule::Container(left), CssRule::Container(right)) => {
                    left.name == right.name && left.condition == right.condition
                }
                _ => false,
            }
        });

        if !merge {
            previous_live = Some(index);
            continue;
        }

        let previous_index = previous_live.expect("checked above");
        let (before, current_and_after) = rules.split_at_mut(index);
        let current = std::mem::replace(&mut current_and_after[0], CssRule::Ignored);
        match (&mut before[previous_index], current) {
            (CssRule::Media(left), CssRule::Media(mut right)) => {
                left.rules.append(&mut right.rules);
            }
            (CssRule::Supports(left), CssRule::Supports(mut right)) => {
                left.rules.append(&mut right.rules);
            }
            (CssRule::Container(left), CssRule::Container(mut right)) => {
                left.rules.append(&mut right.rules);
            }
            _ => unreachable!("conditional rule kind changed after comparison"),
        }
        context.record_conditional_rule_merged();
    }

    for rule in rules.iter_mut() {
        match rule {
            CssRule::Style(rule) => coalesce_conditional_rules(&mut rule.rules, context),
            CssRule::Media(rule) => coalesce_conditional_rules(&mut rule.rules, context),
            CssRule::Supports(rule) => coalesce_conditional_rules(&mut rule.rules, context),
            CssRule::MozDocument(rule) => coalesce_conditional_rules(&mut rule.rules, context),
            CssRule::Nesting(rule) => {
                coalesce_conditional_rules(&mut rule.style.rules, context);
            }
            CssRule::LayerBlock(rule) => coalesce_conditional_rules(&mut rule.rules, context),
            CssRule::Container(rule) => coalesce_conditional_rules(&mut rule.rules, context),
            CssRule::Scope(rule) => coalesce_conditional_rules(&mut rule.rules, context),
            CssRule::StartingStyle(rule) => coalesce_conditional_rules(&mut rule.rules, context),
            _ => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DeclarationKey<'a> {
    name: &'a str,
    vendor_prefix: VendorPrefix,
    important: bool,
}

#[derive(Clone, Copy)]
struct DeclarationLocation<'a> {
    block: NonNull<DeclarationBlock<'a>>,
    index: usize,
}

/// Merges adjacent compatible style rules within one rule list. Every at-rule
/// ends the current IR segment, so block links never cross a conditional or
/// other at-rule boundary. Nested rule lists are minified in their own scope.
pub(crate) fn minify_rule_list<'a>(rules: &mut Vec<'a, CssRule<'a>>, context: &mut MinifyContext) {
    for rule in rules.iter_mut() {
        match rule {
            CssRule::Style(rule) => minify_rule_list(&mut rule.rules, context),
            CssRule::Media(rule) => minify_rule_list(&mut rule.rules, context),
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

    let allocator = rules.bump();
    let mut declarations = HashMap::new_in(allocator);
    let mut previous_style = None;

    for index in 0..rules.len() {
        if matches!(rules[index], CssRule::Ignored) {
            continue;
        }

        if !matches!(rules[index], CssRule::Style(_)) {
            // All at-rules are barriers in the first IR design. Dedicated
            // optimizations such as the conditional fast path run separately.
            previous_style = None;
            declarations.clear();
            continue;
        }

        let can_merge = previous_style.is_some_and(|previous_index| {
            let CssRule::Style(previous) = &rules[previous_index] else {
                return false;
            };
            let CssRule::Style(current) = &rules[index] else {
                return false;
            };
            previous.rules.is_empty()
                && current.rules.is_empty()
                && previous.vendor_prefix == current.vendor_prefix
                && previous.selectors == current.selectors
        });

        if can_merge {
            let previous_index = previous_style.expect("checked above");
            let (before, current_and_after) = rules.split_at_mut(index);
            let previous_rule = std::mem::replace(&mut before[previous_index], CssRule::Ignored);
            let CssRule::Style(mut previous) = previous_rule else {
                unreachable!("style merge candidate changed kind")
            };
            let CssRule::Style(current) = &mut current_and_after[0] else {
                unreachable!("current style rule changed kind")
            };

            let previous_block = NonNull::from(previous.declarations.as_mut());
            // SAFETY: declaration blocks are arena boxed and never move. The
            // previous rule is the live tail of this adjacent merge chain.
            unsafe { current.declarations.link_previous(previous_block) };
            process_declarations(&mut current.declarations, &mut declarations, context);
            context.record_style_rule_merged();
            previous_style = Some(index);
            continue;
        }

        declarations.clear();
        let CssRule::Style(current) = &mut rules[index] else {
            unreachable!()
        };
        process_declarations(&mut current.declarations, &mut declarations, context);
        previous_style = current.rules.is_empty().then_some(index);
    }
}

fn process_declarations<'a>(
    block: &mut DeclarationBlock<'a>,
    declarations: &mut HashMap<'a, DeclarationKey<'a>, DeclarationLocation<'a>>,
    context: &mut MinifyContext,
) {
    let block_pointer = NonNull::from(&mut *block);
    for index in 0..block.declarations.len() {
        let declaration = &block.declarations[index];
        let key = DeclarationKey {
            name: declaration.name(),
            vendor_prefix: declaration.vendor_prefix(),
            important: block.is_important(index),
        };

        if let Some(previous) = declarations.get(&key).copied() {
            // SAFETY: all locations refer to arena-boxed declaration blocks.
            // No declaration vectors are resized or reordered by this pass,
            // so the stored logical index remains valid.
            let duplicate = unsafe {
                previous.block.as_ref().declarations[previous.index]
                    == block_pointer.as_ref().declarations[index]
            };
            if duplicate {
                unsafe { previous.block.as_ptr().as_mut() }
                    .expect("NonNull pointer")
                    .mark_invalid(previous.index);
                context.record_declaration_removed();
            }
        }

        declarations.insert(
            key,
            DeclarationLocation {
                block: block_pointer,
                index,
            },
        );
    }
}
