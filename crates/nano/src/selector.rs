use rocketcss_ast::{NthType, Selector, SelectorComponent, SelectorList};
use rustc_hash::FxHashSet;

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl Minify for SelectorList<'_> {
    fn minify<'cx>(&mut self, context: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        minify_selector_list(self, context);
    }
}

pub(crate) fn minify_selector_list(selectors: &mut [Selector<'_>], context: &mut MinifyContext) {
    for selector in selectors.iter_mut() {
        if matches!(selector, Selector::Unparsed(_)) {
            *selector = Selector::Tombstone;
            context.record_selector_removed();
        }
    }

    if context.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::Any) {
        for selector in selectors.iter_mut() {
            let Some(selector) = selector.as_parsed_mut() else {
                continue;
            };
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

    if context.is_enabled(Options::DEDUPLICATE_LISTS, OptionsOp::Any) && deduplicate(selectors) {
        context.record_selector_removed();
    }
}

fn deduplicate(selectors: &mut [Selector<'_>]) -> bool {
    if selectors.len() < 2 {
        return false;
    }

    let mut duplicate_indices = std::vec::Vec::new();
    {
        let mut seen = FxHashSet::default();
        for (index, selector) in selectors.iter().enumerate() {
            if matches!(selector, Selector::Parsed(_)) && !seen.insert(selector) {
                duplicate_indices.push(index);
            }
        }
    }
    if duplicate_indices.is_empty() {
        return false;
    }
    for index in duplicate_indices {
        selectors[index] = Selector::Tombstone;
    }
    true
}

fn remove_qualified_universal(selector: &mut std::vec::Vec<SelectorComponent<'_>>) {
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
