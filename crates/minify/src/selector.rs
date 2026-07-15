use rocketcss_allocator::prelude::{AdaptiveHashSet, Allocator, Vec};
use rocketcss_ast::{NthType, SelectorComponent, SelectorList};

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl Minify for SelectorList<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        let scratch = Allocator::new();
        minify_selector_list(self, context, &scratch);
    }
}

pub(crate) fn minify_selector_list(
    selectors: &mut SelectorList<'_>,
    context: &mut MinifyContext,
    scratch: &Allocator,
) {
    if context.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::Any) {
        for selector in selectors.iter_mut() {
            remove_qualified_universal(selector);
            for component in selector.iter_mut() {
                if let SelectorComponent::Nth(data) = component
                    && data.a == 0
                    && data.b == 1
                    && matches!(
                        data.kind,
                        NthType::Child | NthType::LastChild | NthType::OfType | NthType::LastOfType
                    )
                {
                    data.is_function = false;
                }
            }
        }
    }

    if context.is_enabled(Options::DEDUPLICATE_LISTS, OptionsOp::Any) {
        let before = selectors.len();
        deduplicate(selectors, scratch);
        if before != selectors.len() {
            context.record_value_normalized();
        }
    }
}

fn deduplicate(selectors: &mut SelectorList<'_>, allocator: &Allocator) {
    if selectors.len() < 2 {
        return;
    }

    let mut duplicate_indices = Vec::new_in(allocator);
    {
        let mut seen = AdaptiveHashSet::<_, 4>::new_in(allocator);
        for (index, selector) in selectors.iter().enumerate() {
            if !seen.insert(selector) {
                duplicate_indices.push(index);
            }
        }
    }
    if duplicate_indices.is_empty() {
        return;
    }

    let original_len = selectors.len();
    let mut duplicate_indices = duplicate_indices.into_iter();
    let mut next_duplicate = duplicate_indices.next();
    let mut index = 0;
    selectors.retain(|_| {
        let keep = next_duplicate != Some(index);
        if !keep {
            next_duplicate = duplicate_indices.next();
        }
        index += 1;
        keep
    });
    debug_assert!(next_duplicate.is_none());
    debug_assert_eq!(index, original_len);
}

fn remove_qualified_universal(selector: &mut rocketcss_ast::Selector<'_>) {
    let mut index = 0;
    while index < selector.len() {
        if !matches!(selector[index], SelectorComponent::ExplicitUniversalType) {
            index += 1;
            continue;
        }
        let namespace_before = index > 0
            && matches!(
                selector[index - 1],
                SelectorComponent::ExplicitAnyNamespace
                    | SelectorComponent::ExplicitNoNamespace
                    | SelectorComponent::DefaultNamespace(_)
                    | SelectorComponent::Namespace { .. }
            );
        let qualified_after = selector.get(index + 1).is_some_and(|component| {
            !matches!(
                component,
                SelectorComponent::Combinator(_)
                    | SelectorComponent::PseudoElement(_)
                    | SelectorComponent::ExplicitAnyNamespace
                    | SelectorComponent::ExplicitNoNamespace
                    | SelectorComponent::DefaultNamespace(_)
                    | SelectorComponent::Namespace { .. }
            )
        });
        if !namespace_before && qualified_after {
            selector.remove(index);
        } else {
            index += 1;
        }
    }
}
