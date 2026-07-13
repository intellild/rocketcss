use std::cmp::Ordering;

use rocketcss_ast::{Combinator, NthType, PseudoClass, Selector, SelectorComponent, SelectorList};

use crate::{Minify, MinifyContext, Options};

impl Minify for SelectorList<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if context.options().is_enabled(Options::NORMALIZE_VALUES) {
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

        let mut deduplicated = false;
        if context.options().is_enabled(Options::DEDUPLICATE_LISTS) {
            deduplicated = deduplicate_selectors(self);
            if deduplicated {
                context.record_value_normalized();
            }
        }
        if context.options().is_enabled(Options::MERGE_SELECTORS)
            && merge_common_selector_parts(
                self,
                context.options().is_enabled(Options::SORT_SELECTOR_MERGES),
                deduplicated,
            )
        {
            context.record_value_normalized();
        }
        if context.options().is_enabled(Options::SORT_SELECTORS)
            && self.iter().all(selector_is_sortable)
            && self
                .windows(2)
                .any(|pair| compare_selectors(&pair[0], &pair[1]).is_gt())
        {
            self.sort_unstable_by(compare_selectors);
            context.record_value_normalized();
        }
    }
}

fn merge_common_selector_parts(
    selectors: &mut SelectorList<'_>,
    sort: bool,
    allow_pair: bool,
) -> bool {
    if selectors.len() < 2
        || selectors.len() < 3 && !allow_pair
        || selectors.iter().any(|selector| {
            selector.iter().any(|component| {
                matches!(component, SelectorComponent::PseudoElement(_))
                    || matches!(component, SelectorComponent::PseudoClass(pseudo)
                            if matches!(&**pseudo, PseudoClass::Custom { name }
                                if ["before", "after", "first-line", "first-letter"]
                                    .iter()
                                    .any(|legacy| name.eq_ignore_ascii_case(legacy))))
            })
        })
    {
        return false;
    }
    let min_len = selectors.iter().map(Selector::len).min().unwrap_or(0);
    let mut prefix = 0;
    while prefix < min_len
        && selectors[1..]
            .iter()
            .all(|selector| selector[prefix] == selectors[0][prefix])
    {
        prefix += 1;
    }
    prefix = selectors[0][..prefix]
        .iter()
        .rposition(|component| matches!(component, SelectorComponent::Combinator(_)))
        .map_or(0, |index| index + 1);
    let mut suffix = 0;
    while prefix + suffix < min_len
        && selectors[1..].iter().all(|selector| {
            selector[selector.len() - 1 - suffix] == selectors[0][selectors[0].len() - 1 - suffix]
        })
    {
        suffix += 1;
    }
    if prefix == 0 && suffix == 0
        || selectors
            .iter()
            .any(|selector| selector.len() == prefix + suffix)
        || selectors.iter().any(|selector| {
            selector[prefix..selector.len() - suffix]
                .iter()
                .any(|component| {
                    matches!(
                        component,
                        SelectorComponent::Combinator(_)
                            | SelectorComponent::Negation(_)
                            | SelectorComponent::PseudoElement(_)
                    )
                })
        })
    {
        return false;
    }
    let specificity =
        selector_slice_specificity(&selectors[0][prefix..selectors[0].len() - suffix]);
    if specificity.is_none()
        || selectors[1..].iter().any(|selector| {
            selector_slice_specificity(&selector[prefix..selector.len() - suffix]) != specificity
        })
    {
        return false;
    }

    let allocator = selectors.bump();
    let first_middle_len = selectors[0].len() - prefix - suffix;
    let mut first_alternative = allocator.vec();
    for _ in 0..first_middle_len {
        first_alternative.push(selectors[0].remove(prefix));
    }
    let mut alternatives = allocator.vec();
    alternatives.push(first_alternative);
    while selectors.len() > 1 {
        let mut selector = selectors
            .pop()
            .expect("selector list has remaining entries");
        selector.truncate(selector.len() - suffix);
        drop(selector.drain(..prefix));
        alternatives.push(selector);
    }
    alternatives[1..].reverse();
    if sort
        && alternatives.iter().all(selector_is_sortable)
        && alternatives
            .windows(2)
            .any(|pair| compare_selectors(&pair[0], &pair[1]).is_gt())
    {
        alternatives.sort_unstable_by(compare_selectors);
    }
    selectors[0].insert(prefix, SelectorComponent::Is(alternatives));
    true
}

fn selector_slice_specificity(components: &[SelectorComponent<'_>]) -> Option<(u16, u16, u16)> {
    let mut specificity = (0, 0, 0);
    for component in components {
        match component {
            SelectorComponent::Id(_) => specificity.0 += 1,
            SelectorComponent::Class(_)
            | SelectorComponent::AttributeInNoNamespaceExists { .. }
            | SelectorComponent::AttributeInNoNamespace { .. }
            | SelectorComponent::AttributeOther(_)
            | SelectorComponent::PseudoClass(_)
            | SelectorComponent::Root
            | SelectorComponent::Empty
            | SelectorComponent::Scope
            | SelectorComponent::Nth(_)
            | SelectorComponent::NthOf { .. } => specificity.1 += 1,
            SelectorComponent::LocalName { .. } => specificity.2 += 1,
            SelectorComponent::ExplicitUniversalType => {}
            _ => return None,
        }
    }
    Some(specificity)
}

pub(crate) fn minify_selector(selector: &mut Selector<'_>) -> bool {
    let before = selector.len();
    remove_qualified_universal(selector);
    let mut changed = before != selector.len();
    for component in selector {
        let selectors = match component {
            SelectorComponent::Negation(selectors)
            | SelectorComponent::Where(selectors)
            | SelectorComponent::Is(selectors)
            | SelectorComponent::Has(selectors) => Some(selectors),
            SelectorComponent::Any { selectors, .. }
            | SelectorComponent::NthOf { selectors, .. } => Some(selectors),
            _ => None,
        };
        if selectors.is_some_and(deduplicate_selectors) {
            changed = true;
        }
    }
    changed
}

fn deduplicate_selectors(selectors: &mut SelectorList<'_>) -> bool {
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
    before != selectors.len()
}

fn selector_is_sortable(selector: &Selector<'_>) -> bool {
    selector.iter().all(component_is_sortable)
}

fn component_is_sortable(component: &SelectorComponent<'_>) -> bool {
    match component {
        SelectorComponent::Combinator(_)
        | SelectorComponent::ExplicitUniversalType
        | SelectorComponent::LocalName { .. }
        | SelectorComponent::Id(_)
        | SelectorComponent::Class(_)
        | SelectorComponent::AttributeInNoNamespaceExists { .. }
        | SelectorComponent::AttributeInNoNamespace { .. }
        | SelectorComponent::Root
        | SelectorComponent::Empty
        | SelectorComponent::Scope
        | SelectorComponent::Nth(_)
        | SelectorComponent::PseudoElement(_)
        | SelectorComponent::Nesting => true,
        SelectorComponent::Negation(selectors)
        | SelectorComponent::Where(selectors)
        | SelectorComponent::Is(selectors)
        | SelectorComponent::Has(selectors) => selectors.iter().all(selector_is_sortable),
        SelectorComponent::PseudoClass(pseudo) => pseudo_class_name(pseudo).is_some(),
        _ => false,
    }
}

pub(crate) fn compare_selectors(left: &Selector<'_>, right: &Selector<'_>) -> Ordering {
    for (left, right) in left.iter().zip(right) {
        let ordering = compare_components(left, right);
        if !ordering.is_eq() {
            return ordering;
        }
    }
    left.len().cmp(&right.len())
}

fn compare_components(left: &SelectorComponent<'_>, right: &SelectorComponent<'_>) -> Ordering {
    let ordering = component_leading_byte(left).cmp(&component_leading_byte(right));
    if !ordering.is_eq() {
        return ordering;
    }
    match (left, right) {
        (SelectorComponent::Combinator(left), SelectorComponent::Combinator(right)) => {
            combinator_byte(*left).cmp(&combinator_byte(*right))
        }
        (
            SelectorComponent::LocalName { name: left, .. },
            SelectorComponent::LocalName { name: right, .. },
        )
        | (SelectorComponent::Id(left), SelectorComponent::Id(right))
        | (SelectorComponent::Class(left), SelectorComponent::Class(right)) => left.cmp(right),
        (
            SelectorComponent::AttributeInNoNamespaceExists {
                local_name: left, ..
            },
            SelectorComponent::AttributeInNoNamespaceExists {
                local_name: right, ..
            },
        ) => left.cmp(right),
        (SelectorComponent::Negation(left), SelectorComponent::Negation(right))
        | (SelectorComponent::Where(left), SelectorComponent::Where(right))
        | (SelectorComponent::Is(left), SelectorComponent::Is(right))
        | (SelectorComponent::Has(left), SelectorComponent::Has(right)) => {
            compare_selector_lists(left, right)
        }
        (SelectorComponent::PseudoClass(left), SelectorComponent::PseudoClass(right)) => {
            pseudo_class_name(left).cmp(&pseudo_class_name(right))
        }
        (SelectorComponent::Negation(_), SelectorComponent::PseudoClass(right)) => {
            "not".cmp(pseudo_class_name(right).expect("sortable pseudo class"))
        }
        (SelectorComponent::PseudoClass(left), SelectorComponent::Negation(_)) => {
            pseudo_class_name(left)
                .expect("sortable pseudo class")
                .cmp("not")
        }
        _ => component_sort_name(left)
            .zip(component_sort_name(right))
            .map_or(Ordering::Equal, |(left, right)| left.cmp(right)),
    }
}

fn compare_selector_lists(left: &SelectorList<'_>, right: &SelectorList<'_>) -> Ordering {
    for (left, right) in left.iter().zip(right) {
        let ordering = compare_selectors(left, right);
        if !ordering.is_eq() {
            return ordering;
        }
    }
    left.len().cmp(&right.len())
}

fn component_leading_byte(component: &SelectorComponent<'_>) -> u8 {
    match component {
        SelectorComponent::Combinator(combinator) => combinator_byte(*combinator),
        SelectorComponent::Id(_) => b'#',
        SelectorComponent::Nesting => b'&',
        SelectorComponent::ExplicitUniversalType => b'*',
        SelectorComponent::Class(_) => b'.',
        SelectorComponent::Negation(_)
        | SelectorComponent::Where(_)
        | SelectorComponent::Is(_)
        | SelectorComponent::Has(_)
        | SelectorComponent::PseudoClass(_)
        | SelectorComponent::PseudoElement(_)
        | SelectorComponent::Root
        | SelectorComponent::Empty
        | SelectorComponent::Scope
        | SelectorComponent::Nth(_)
        | SelectorComponent::NthOf { .. } => b':',
        SelectorComponent::AttributeInNoNamespaceExists { .. }
        | SelectorComponent::AttributeInNoNamespace { .. } => b'[',
        SelectorComponent::LocalName { name, .. } => name.as_bytes().first().copied().unwrap_or(0),
        _ => u8::MAX,
    }
}

fn combinator_byte(combinator: Combinator) -> u8 {
    match combinator {
        Combinator::Descendant => b' ',
        Combinator::NextSibling => b'+',
        Combinator::Child => b'>',
        Combinator::LaterSibling => b'~',
        _ => u8::MAX,
    }
}

fn pseudo_class_name<'a>(pseudo: &'a PseudoClass<'_>) -> Option<&'a str> {
    match pseudo {
        PseudoClass::Active => Some("active"),
        PseudoClass::Checked => Some("checked"),
        PseudoClass::Disabled => Some("disabled"),
        PseudoClass::Enabled => Some("enabled"),
        PseudoClass::Focus => Some("focus"),
        PseudoClass::FocusVisible => Some("focus-visible"),
        PseudoClass::FocusWithin => Some("focus-within"),
        PseudoClass::Hover => Some("hover"),
        PseudoClass::Link => Some("link"),
        PseudoClass::Visited => Some("visited"),
        PseudoClass::Custom { name } => Some(name),
        _ => None,
    }
}

fn component_sort_name<'a>(component: &'a SelectorComponent<'_>) -> Option<&'a str> {
    match component {
        SelectorComponent::Root => Some("root"),
        SelectorComponent::Empty => Some("empty"),
        SelectorComponent::Scope => Some("scope"),
        SelectorComponent::PseudoClass(pseudo) => pseudo_class_name(pseudo),
        SelectorComponent::PseudoElement(pseudo) => match &**pseudo {
            rocketcss_ast::PseudoElement::After => Some("after"),
            rocketcss_ast::PseudoElement::Before => Some("before"),
            rocketcss_ast::PseudoElement::FirstLetter => Some("first-letter"),
            rocketcss_ast::PseudoElement::FirstLine => Some("first-line"),
            rocketcss_ast::PseudoElement::Marker => Some("marker"),
            rocketcss_ast::PseudoElement::Placeholder(_) => Some("placeholder"),
            rocketcss_ast::PseudoElement::Selection(_) => Some("selection"),
            rocketcss_ast::PseudoElement::Custom { name } => Some(name),
            _ => None,
        },
        _ => None,
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
