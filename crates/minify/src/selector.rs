use rocketcss_ast::{NthType, SelectorComponent, SelectorList};

use crate::{Minify, MinifyContext};

impl Minify for SelectorList<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if context.options().normalize_values {
            for selector in self.iter_mut() {
                remove_qualified_universal(selector);
                for component in selector.iter_mut() {
                    if let SelectorComponent::Nth(data) = component
                        && data.a == 0
                        && data.b == 1
                        && matches!(
                            data.kind,
                            NthType::Child
                                | NthType::LastChild
                                | NthType::OfType
                                | NthType::LastOfType
                        )
                    {
                        data.is_function = false;
                    }
                }
            }
        }

        if context.options().deduplicate_lists {
            let before = self.len();
            let mut index = 0;
            while index < self.len() {
                if self[..index]
                    .iter()
                    .any(|selector| selector == &self[index])
                {
                    self.remove(index);
                } else {
                    index += 1;
                }
            }
            if before != self.len() {
                context.record_value_normalized();
            }
        }
    }
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
