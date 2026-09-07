use rocketcss_ast::{
    AstContext, NthType, Selector, SelectorComponent, SelectorList, VisitMutContext,
};
use rocketcss_common::prelude::{Allocator, Vec};

use crate::{MinifyContext, Options, OptionsOp};

pub(crate) fn minify_selector_list<'ast>(
    selectors: &mut SelectorList<'ast>,
    context: &mut MinifyContext,
    scratch: &Allocator,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    ast.rewrite_vec(selectors, |selectors, ast| {
        for selector in selectors.iter().copied() {
            ast.mutate_node(selector, |selector, _| {
                if matches!(selector, Selector::Unparsed(_)) {
                    *selector = Selector::Tombstone;
                    context.record_value_normalized();
                }
            });
        }

        if context.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::Any) {
            for selector in selectors.iter().copied() {
                ast.mutate_node(selector, |selector, ast| {
                    let Some(components) = selector.as_parsed_mut() else {
                        return;
                    };
                    ast.rewrite_vec(components, |components, ast| {
                        remove_qualified_universal(components, ast.ast_context());
                        for component in components.iter().copied() {
                            ast.mutate_node(component, |component, _| {
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
                            });
                        }
                    });
                });
            }
        }

        if context.is_enabled(Options::DEDUPLICATE_LISTS, OptionsOp::Any) {
            let before = selectors.len();
            deduplicate(selectors, scratch, ast.ast_context());
            if before != selectors.len() {
                context.record_value_normalized();
            }
        }
    });
}

fn deduplicate(
    selectors: &mut Vec<'_, rocketcss_ast::NodeId<'_, Selector<'_>>>,
    allocator: &Allocator,
    ast: &AstContext<'_>,
) {
    if selectors.len() < 2 {
        return;
    }

    let mut duplicate_indices = Vec::new_in(allocator);
    for (index, selector) in selectors.iter().copied().enumerate() {
        if matches!(ast.resolve_node(selector), Selector::Parsed(_))
            && selectors[..index].iter().any(|existing| {
                let existing = ast.resolve_node(*existing);
                matches!(existing, Selector::Parsed(_))
                    && crate::equality::css_values_are_equal(
                        ast,
                        &existing,
                        &ast.resolve_node(selector),
                    )
            })
        {
            duplicate_indices.push(index);
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

fn remove_qualified_universal(
    selector: &mut rocketcss_common::prelude::Vec<
        '_,
        rocketcss_ast::NodeId<'_, SelectorComponent<'_>>,
    >,
    ast: &AstContext<'_>,
) {
    let mut index = 0;
    while index < selector.len() {
        if !matches!(
            ast.resolve_node(selector[index]),
            SelectorComponent::ExplicitUniversalType
        ) {
            index += 1;
            continue;
        }
        let namespace_before = index > 0
            && matches!(
                ast.resolve_node(selector[index - 1]),
                SelectorComponent::ExplicitAnyNamespace
                    | SelectorComponent::ExplicitNoNamespace
                    | SelectorComponent::DefaultNamespace(_)
                    | SelectorComponent::Namespace { .. }
            );
        let qualified_after = selector.get(index + 1).is_some_and(|component| {
            !matches!(
                ast.resolve_node(*component),
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
