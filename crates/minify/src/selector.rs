use rs_css_ast::{NthType, SelectorComponent, SelectorList};

use crate::MinifyContext;

pub(crate) fn minify_selector_list<'a>(
    selectors: &mut SelectorList<'a>,
    context: &mut MinifyContext,
) {
    if context.options().normalize_values {
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

    if context.options().deduplicate_lists {
        let before = selectors.len();
        let mut index = 0;
        while index < selectors.len() {
            if selectors[..index]
                .iter()
                .any(|selector| selector == &selectors[index])
            {
                selectors.remove(index);
            } else {
                index += 1;
            }
        }
        if before != selectors.len() {
            context.record_value_normalized();
        }
    }
}

fn remove_qualified_universal(selector: &mut rs_css_ast::Selector<'_>) {
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
