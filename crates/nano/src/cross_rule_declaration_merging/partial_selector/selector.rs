use rocketcss_ast::*;
use rocketcss_common::{Allocator, vec::Vec};
use rustc_hash::FxHashSet;
use std::mem::{Discriminant, discriminant};

pub(crate) fn materialize_selector_union<'ast>(
    left: &SelectorList<'ast>,
    right: &SelectorList<'ast>,
    preserve_compatibility: bool,
) -> Option<SelectorList<'ast>> {
    let left_compatibility = selector_compatibility(left)?;
    let right_compatibility = selector_compatibility(right)?;
    if left_compatibility.prefixes != right_compatibility.prefixes
        || preserve_compatibility && left_compatibility != right_compatibility
    {
        return None;
    }

    let allocator = left.bump();
    let mut left_prefixes = VendorPrefix::NONE;
    let mut right_prefixes = VendorPrefix::NONE;
    let mut selectors = Vec::with_capacity_in(left.len() + right.len(), allocator);
    append_materialized_selectors(left, allocator, &mut left_prefixes, &mut selectors)?;
    append_materialized_selectors(right, allocator, &mut right_prefixes, &mut selectors)?;
    debug_assert_eq!(left_prefixes, left_compatibility.prefixes);
    debug_assert_eq!(right_prefixes, right_compatibility.prefixes);
    (!selectors.is_empty()).then_some(selectors)
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct SelectorSyntaxFeatures: u32 {
        const NAMESPACE = 1 << 0;
        const ATTRIBUTE_EXISTS = 1 << 1;
        const ATTRIBUTE_CSS2_VALUE = 1 << 2;
        const ATTRIBUTE_CSS3_VALUE = 1 << 3;
        const ATTRIBUTE_CASE_MODIFIER = 1 << 4;
        const CHILD_COMBINATOR = 1 << 5;
        const NEXT_SIBLING_COMBINATOR = 1 << 6;
        const LATER_SIBLING_COMBINATOR = 1 << 7;
        const NON_BASELINE_COMBINATOR = 1 << 8;
        const NEGATION = 1 << 9;
        const ROOT = 1 << 10;
        const EMPTY = 1 << 11;
        const SCOPE = 1 << 12;
        const NTH = 1 << 13;
        const NTH_OF = 1 << 14;
        const SLOTTED = 1 << 15;
        const PART = 1 << 16;
        const HOST = 1 << 17;
        const WHERE = 1 << 18;
        const IS = 1 << 19;
        const ANY = 1 << 20;
        const HAS = 1 << 21;
        const NESTING = 1 << 22;
        const NEGATION_LIST = 1 << 23;
        const NEGATION_COMPLEX = 1 << 24;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SelectorCompatibility<'ast> {
    features: SelectorSyntaxFeatures,
    pseudo_classes: FxHashSet<Discriminant<PseudoClass<'ast>>>,
    pseudo_elements: FxHashSet<Discriminant<PseudoElement<'ast>>>,
    prefixes: VendorPrefix,
}

impl<'ast> Default for SelectorCompatibility<'ast> {
    fn default() -> Self {
        Self {
            features: SelectorSyntaxFeatures::default(),
            pseudo_classes: FxHashSet::default(),
            pseudo_elements: FxHashSet::default(),
            prefixes: VendorPrefix::NONE,
        }
    }
}

fn selector_compatibility<'ast>(
    selectors: &SelectorList<'ast>,
) -> Option<SelectorCompatibility<'ast>> {
    let mut compatibility = SelectorCompatibility::default();
    observe_selector_list_compatibility(selectors, &mut compatibility)?;
    Some(compatibility)
}

fn observe_selector_list_compatibility<'ast>(
    selectors: &SelectorList<'ast>,
    compatibility: &mut SelectorCompatibility<'ast>,
) -> Option<()> {
    for selector in selectors {
        match selector {
            Selector::Parsed(components) => {
                for component in components {
                    observe_selector_component_compatibility(component, compatibility)?;
                }
            }
            Selector::Tombstone => {}
            Selector::Unparsed(_) => return None,
        }
    }
    Some(())
}

fn observe_selector_component_compatibility<'ast>(
    component: &SelectorComponent<'ast>,
    compatibility: &mut SelectorCompatibility<'ast>,
) -> Option<()> {
    use SelectorComponent as Component;
    use SelectorSyntaxFeatures as Feature;

    match component {
        Component::Combinator(combinator) => match combinator {
            Combinator::Child => compatibility.features |= Feature::CHILD_COMBINATOR,
            Combinator::NextSibling => {
                compatibility.features |= Feature::NEXT_SIBLING_COMBINATOR;
            }
            Combinator::LaterSibling => {
                compatibility.features |= Feature::LATER_SIBLING_COMBINATOR;
            }
            Combinator::Descendant => {}
            Combinator::PseudoElement
            | Combinator::SlotAssignment
            | Combinator::Part
            | Combinator::DeepDescendant
            | Combinator::Deep => {
                compatibility.features |= Feature::NON_BASELINE_COMBINATOR;
            }
        },
        Component::ExplicitAnyNamespace
        | Component::ExplicitNoNamespace
        | Component::DefaultNamespace(_)
        | Component::Namespace { .. } => compatibility.features |= Feature::NAMESPACE,
        Component::AttributeInNoNamespaceExists { .. } => {
            compatibility.features |= Feature::ATTRIBUTE_EXISTS;
        }
        Component::AttributeInNoNamespace {
            operator,
            case_sensitivity,
            ..
        } => {
            observe_attribute_compatibility(*operator, *case_sensitivity, compatibility);
        }
        Component::AttributeOther(attribute) => {
            if attribute.namespace.is_some() {
                compatibility.features |= Feature::NAMESPACE;
            }
            match attribute.operation {
                AttrOperation::Exists => compatibility.features |= Feature::ATTRIBUTE_EXISTS,
                AttrOperation::WithValue {
                    operator,
                    case_sensitivity,
                    ..
                } => observe_attribute_compatibility(operator, case_sensitivity, compatibility),
            }
        }
        Component::Negation(selectors) => {
            compatibility.features |= Feature::NEGATION;
            let mut live_selectors = selectors
                .iter()
                .filter(|selector| !matches!(selector, Selector::Tombstone));
            if live_selectors.clone().count() != 1 {
                compatibility.features |= Feature::NEGATION_LIST;
            }
            if live_selectors.any(|selector| {
                !matches!(selector, Selector::Parsed(components) if components.len() == 1)
            }) {
                compatibility.features |= Feature::NEGATION_COMPLEX;
            }
            observe_selector_list_compatibility(selectors, compatibility)?;
        }
        Component::Root => compatibility.features |= Feature::ROOT,
        Component::Empty => compatibility.features |= Feature::EMPTY,
        Component::Scope => compatibility.features |= Feature::SCOPE,
        Component::Nth(_) => compatibility.features |= Feature::NTH,
        Component::NthOf { selectors, .. } => {
            compatibility.features |= Feature::NTH_OF;
            observe_selector_list_compatibility(selectors, compatibility)?;
        }
        Component::PseudoClass(value) => {
            observe_pseudo_class_compatibility(value, compatibility)?;
        }
        Component::Slotted(selector) => {
            compatibility.features |= Feature::SLOTTED;
            observe_selector_compatibility(selector, compatibility)?;
        }
        Component::Part(_) => compatibility.features |= Feature::PART,
        Component::Host(selector) => {
            compatibility.features |= Feature::HOST;
            if let Some(selector) = selector {
                observe_selector_compatibility(selector, compatibility)?;
            }
        }
        Component::Where(selectors) => {
            compatibility.features |= Feature::WHERE;
            observe_selector_list_compatibility(selectors, compatibility)?;
        }
        Component::Is(selectors) => {
            compatibility.features |= Feature::IS;
            observe_selector_list_compatibility(selectors, compatibility)?;
        }
        Component::Any {
            vendor_prefix,
            selectors,
        } => {
            compatibility.features |= Feature::ANY;
            compatibility.prefixes |= *vendor_prefix;
            observe_selector_list_compatibility(selectors, compatibility)?;
        }
        Component::Has(selectors) => {
            compatibility.features |= Feature::HAS;
            observe_selector_list_compatibility(selectors, compatibility)?;
        }
        Component::PseudoElement(value) => {
            observe_pseudo_element_compatibility(value, compatibility)?;
        }
        Component::Nesting => compatibility.features |= Feature::NESTING,
        Component::ExplicitUniversalType
        | Component::LocalName { .. }
        | Component::Id(_)
        | Component::Class(_) => {}
    }
    Some(())
}

fn observe_selector_compatibility<'ast>(
    selector: &Selector<'ast>,
    compatibility: &mut SelectorCompatibility<'ast>,
) -> Option<()> {
    let Selector::Parsed(components) = selector else {
        return matches!(selector, Selector::Tombstone).then_some(());
    };
    for component in components {
        observe_selector_component_compatibility(component, compatibility)?;
    }
    Some(())
}

fn observe_attribute_compatibility(
    operator: AttrSelectorOperator,
    case_sensitivity: ParsedCaseSensitivity,
    compatibility: &mut SelectorCompatibility<'_>,
) {
    use SelectorSyntaxFeatures as Feature;

    compatibility.features |= match operator {
        AttrSelectorOperator::Equal
        | AttrSelectorOperator::Includes
        | AttrSelectorOperator::DashMatch => Feature::ATTRIBUTE_CSS2_VALUE,
        AttrSelectorOperator::Prefix
        | AttrSelectorOperator::Substring
        | AttrSelectorOperator::Suffix => Feature::ATTRIBUTE_CSS3_VALUE,
    };
    if !matches!(case_sensitivity, ParsedCaseSensitivity::CaseSensitive) {
        compatibility.features |= Feature::ATTRIBUTE_CASE_MODIFIER;
    }
}

fn observe_pseudo_class_compatibility<'ast>(
    value: &PseudoClass<'ast>,
    compatibility: &mut SelectorCompatibility<'ast>,
) -> Option<()> {
    use PseudoClass as Pseudo;

    compatibility.pseudo_classes.insert(discriminant(value));
    match value {
        Pseudo::Fullscreen(prefix)
        | Pseudo::AnyLink(prefix)
        | Pseudo::ReadOnly(prefix)
        | Pseudo::ReadWrite(prefix)
        | Pseudo::PlaceholderShown(prefix)
        | Pseudo::Autofill(prefix) => compatibility.prefixes |= *prefix,
        Pseudo::WebKitScrollbar(_) => compatibility.prefixes |= VendorPrefix::WEBKIT,
        Pseudo::Local { .. }
        | Pseudo::Global { .. }
        | Pseudo::Custom { .. }
        | Pseudo::CustomFunction { .. } => return None,
        _ => {}
    }
    Some(())
}

fn observe_pseudo_element_compatibility<'ast>(
    value: &PseudoElement<'ast>,
    compatibility: &mut SelectorCompatibility<'ast>,
) -> Option<()> {
    use PseudoElement as Pseudo;

    compatibility.pseudo_elements.insert(discriminant(value));
    match value {
        Pseudo::Selection(prefix)
        | Pseudo::Placeholder(prefix)
        | Pseudo::Backdrop(prefix)
        | Pseudo::FileSelectorButton(prefix) => compatibility.prefixes |= *prefix,
        Pseudo::WebKitScrollbar(_) => compatibility.prefixes |= VendorPrefix::WEBKIT,
        Pseudo::CueFunction { selector } | Pseudo::CueRegionFunction { selector } => {
            observe_selector_compatibility(selector, compatibility)?;
        }
        Pseudo::Custom { .. } | Pseudo::CustomFunction { .. } => return None,
        _ => {}
    }
    Some(())
}

fn append_materialized_selectors<'ast>(
    source: &SelectorList<'ast>,
    allocator: &'ast Allocator,
    prefixes: &mut VendorPrefix,
    output: &mut SelectorList<'ast>,
) -> Option<()> {
    for selector in source {
        if selector.is_tombstone() {
            continue;
        }
        let selector = clone_selector(selector, allocator, prefixes)?;
        if !output.iter().any(|existing| existing == &selector) {
            output.push(selector);
        }
    }
    Some(())
}

fn clone_selector<'ast>(
    selector: &Selector<'ast>,
    allocator: &'ast Allocator,
    prefixes: &mut VendorPrefix,
) -> Option<Selector<'ast>> {
    let Selector::Parsed(components) = selector else {
        return None;
    };
    let mut cloned = Vec::with_capacity_in(components.len(), allocator);
    for component in components {
        cloned.push(clone_selector_component(component, allocator, prefixes)?);
    }
    Some(Selector::Parsed(cloned))
}

fn clone_selector_list<'ast>(
    selectors: &SelectorList<'ast>,
    allocator: &'ast Allocator,
    prefixes: &mut VendorPrefix,
) -> Option<SelectorList<'ast>> {
    let mut cloned = Vec::with_capacity_in(selectors.len(), allocator);
    for selector in selectors {
        cloned.push(clone_selector(selector, allocator, prefixes)?);
    }
    Some(cloned)
}

fn clone_selector_component<'ast>(
    component: &SelectorComponent<'ast>,
    allocator: &'ast Allocator,
    prefixes: &mut VendorPrefix,
) -> Option<SelectorComponent<'ast>> {
    use SelectorComponent as Component;
    Some(match component {
        Component::Combinator(value) => Component::Combinator(*value),
        Component::ExplicitAnyNamespace => Component::ExplicitAnyNamespace,
        Component::ExplicitNoNamespace => Component::ExplicitNoNamespace,
        Component::DefaultNamespace(value) => Component::DefaultNamespace(*value),
        Component::Namespace { prefix, url } => Component::Namespace {
            prefix: *prefix,
            url: *url,
        },
        Component::ExplicitUniversalType => Component::ExplicitUniversalType,
        Component::LocalName { name, lower_name } => Component::LocalName {
            name: *name,
            lower_name: *lower_name,
        },
        Component::Id(value) => Component::Id(*value),
        Component::Class(value) => Component::Class(*value),
        Component::AttributeInNoNamespaceExists {
            local_name,
            local_name_lower,
        } => Component::AttributeInNoNamespaceExists {
            local_name: *local_name,
            local_name_lower: *local_name_lower,
        },
        Component::AttributeInNoNamespace {
            local_name,
            operator,
            value,
            case_sensitivity,
            never_matches,
        } => Component::AttributeInNoNamespace {
            local_name: *local_name,
            operator: *operator,
            value: *value,
            case_sensitivity: *case_sensitivity,
            never_matches: *never_matches,
        },
        Component::AttributeOther(attribute) => {
            Component::AttributeOther(allocator.boxed(clone_attribute_selector(attribute)))
        }
        Component::Negation(selectors) => {
            Component::Negation(clone_selector_list(selectors, allocator, prefixes)?)
        }
        Component::Root => Component::Root,
        Component::Empty => Component::Empty,
        Component::Scope => Component::Scope,
        Component::Nth(value) => Component::Nth(*value),
        Component::NthOf { data, selectors } => Component::NthOf {
            data: *data,
            selectors: clone_selector_list(selectors, allocator, prefixes)?,
        },
        Component::PseudoClass(value) => {
            Component::PseudoClass(allocator.boxed(clone_pseudo_class(value, allocator, prefixes)?))
        }
        Component::Slotted(selector) => {
            Component::Slotted(allocator.boxed(clone_selector(selector, allocator, prefixes)?))
        }
        Component::Part(names) => Component::Part(names.clone()),
        Component::Host(selector) => Component::Host(match selector {
            Some(selector) => Some(allocator.boxed(clone_selector(selector, allocator, prefixes)?)),
            None => None,
        }),
        Component::Where(selectors) => {
            Component::Where(clone_selector_list(selectors, allocator, prefixes)?)
        }
        Component::Is(selectors) => {
            Component::Is(clone_selector_list(selectors, allocator, prefixes)?)
        }
        Component::Any {
            vendor_prefix,
            selectors,
        } => {
            *prefixes |= *vendor_prefix;
            Component::Any {
                vendor_prefix: *vendor_prefix,
                selectors: clone_selector_list(selectors, allocator, prefixes)?,
            }
        }
        Component::Has(selectors) => {
            Component::Has(clone_selector_list(selectors, allocator, prefixes)?)
        }
        Component::PseudoElement(value) => Component::PseudoElement(
            allocator.boxed(clone_pseudo_element(value, allocator, prefixes)?),
        ),
        Component::Nesting => Component::Nesting,
    })
}

fn clone_attribute_selector<'ast>(attribute: &AttrSelector<'ast>) -> AttrSelector<'ast> {
    AttrSelector {
        namespace: attribute
            .namespace
            .as_ref()
            .map(|namespace| match namespace {
                NamespaceConstraint::Any => NamespaceConstraint::Any,
                NamespaceConstraint::Specific { prefix, url } => NamespaceConstraint::Specific {
                    prefix: *prefix,
                    url: *url,
                },
            }),
        local_name: attribute.local_name,
        local_name_lower: attribute.local_name_lower,
        operation: match &attribute.operation {
            AttrOperation::Exists => AttrOperation::Exists,
            AttrOperation::WithValue {
                operator,
                case_sensitivity,
                expected_value,
            } => AttrOperation::WithValue {
                operator: *operator,
                case_sensitivity: *case_sensitivity,
                expected_value: *expected_value,
            },
        },
        never_matches: attribute.never_matches,
    }
}

fn clone_pseudo_class<'ast>(
    value: &PseudoClass<'ast>,
    _allocator: &'ast Allocator,
    prefixes: &mut VendorPrefix,
) -> Option<PseudoClass<'ast>> {
    use PseudoClass as Pseudo;
    Some(match value {
        Pseudo::Lang { languages } => Pseudo::Lang {
            languages: languages.clone(),
        },
        Pseudo::Dir { direction } => Pseudo::Dir {
            direction: *direction,
        },
        Pseudo::Hover => Pseudo::Hover,
        Pseudo::Active => Pseudo::Active,
        Pseudo::Focus => Pseudo::Focus,
        Pseudo::FocusVisible => Pseudo::FocusVisible,
        Pseudo::FocusWithin => Pseudo::FocusWithin,
        Pseudo::Current => Pseudo::Current,
        Pseudo::Past => Pseudo::Past,
        Pseudo::Future => Pseudo::Future,
        Pseudo::Playing => Pseudo::Playing,
        Pseudo::Paused => Pseudo::Paused,
        Pseudo::Seeking => Pseudo::Seeking,
        Pseudo::Buffering => Pseudo::Buffering,
        Pseudo::Stalled => Pseudo::Stalled,
        Pseudo::Muted => Pseudo::Muted,
        Pseudo::VolumeLocked => Pseudo::VolumeLocked,
        Pseudo::Fullscreen(prefix) => {
            *prefixes |= *prefix;
            Pseudo::Fullscreen(*prefix)
        }
        Pseudo::Open => Pseudo::Open,
        Pseudo::Closed => Pseudo::Closed,
        Pseudo::Modal => Pseudo::Modal,
        Pseudo::PictureInPicture => Pseudo::PictureInPicture,
        Pseudo::PopoverOpen => Pseudo::PopoverOpen,
        Pseudo::Defined => Pseudo::Defined,
        Pseudo::AnyLink(prefix) => {
            *prefixes |= *prefix;
            Pseudo::AnyLink(*prefix)
        }
        Pseudo::Link => Pseudo::Link,
        Pseudo::LocalLink => Pseudo::LocalLink,
        Pseudo::Target => Pseudo::Target,
        Pseudo::TargetCurrent => Pseudo::TargetCurrent,
        Pseudo::TargetBefore => Pseudo::TargetBefore,
        Pseudo::TargetAfter => Pseudo::TargetAfter,
        Pseudo::TargetWithin => Pseudo::TargetWithin,
        Pseudo::Visited => Pseudo::Visited,
        Pseudo::Enabled => Pseudo::Enabled,
        Pseudo::Disabled => Pseudo::Disabled,
        Pseudo::ReadOnly(prefix) => {
            *prefixes |= *prefix;
            Pseudo::ReadOnly(*prefix)
        }
        Pseudo::ReadWrite(prefix) => {
            *prefixes |= *prefix;
            Pseudo::ReadWrite(*prefix)
        }
        Pseudo::PlaceholderShown(prefix) => {
            *prefixes |= *prefix;
            Pseudo::PlaceholderShown(*prefix)
        }
        Pseudo::Default => Pseudo::Default,
        Pseudo::Checked => Pseudo::Checked,
        Pseudo::Indeterminate => Pseudo::Indeterminate,
        Pseudo::Blank => Pseudo::Blank,
        Pseudo::Valid => Pseudo::Valid,
        Pseudo::Invalid => Pseudo::Invalid,
        Pseudo::InRange => Pseudo::InRange,
        Pseudo::OutOfRange => Pseudo::OutOfRange,
        Pseudo::Required => Pseudo::Required,
        Pseudo::Optional => Pseudo::Optional,
        Pseudo::UserValid => Pseudo::UserValid,
        Pseudo::UserInvalid => Pseudo::UserInvalid,
        Pseudo::Autofill(prefix) => {
            *prefixes |= *prefix;
            Pseudo::Autofill(*prefix)
        }
        Pseudo::ActiveViewTransition => Pseudo::ActiveViewTransition,
        Pseudo::ActiveViewTransitionType { kinds } => Pseudo::ActiveViewTransitionType {
            kinds: kinds.clone(),
        },
        Pseudo::State { state } => Pseudo::State { state: *state },
        Pseudo::Local { .. }
        | Pseudo::Global { .. }
        | Pseudo::Custom { .. }
        | Pseudo::CustomFunction { .. } => return None,
        Pseudo::WebKitScrollbar(value) => {
            *prefixes |= VendorPrefix::WEBKIT;
            Pseudo::WebKitScrollbar(*value)
        }
    })
}

fn clone_pseudo_element<'ast>(
    value: &PseudoElement<'ast>,
    allocator: &'ast Allocator,
    prefixes: &mut VendorPrefix,
) -> Option<PseudoElement<'ast>> {
    use PseudoElement as Pseudo;
    Some(match value {
        Pseudo::After => Pseudo::After,
        Pseudo::Before => Pseudo::Before,
        Pseudo::FirstLine => Pseudo::FirstLine,
        Pseudo::FirstLetter => Pseudo::FirstLetter,
        Pseudo::DetailsContent => Pseudo::DetailsContent,
        Pseudo::TargetText => Pseudo::TargetText,
        Pseudo::SearchText => Pseudo::SearchText,
        Pseudo::Selection(prefix) => {
            *prefixes |= *prefix;
            Pseudo::Selection(*prefix)
        }
        Pseudo::Placeholder(prefix) => {
            *prefixes |= *prefix;
            Pseudo::Placeholder(*prefix)
        }
        Pseudo::HighlightFunction { name } => Pseudo::HighlightFunction { name: *name },
        Pseudo::Marker => Pseudo::Marker,
        Pseudo::Backdrop(prefix) => {
            *prefixes |= *prefix;
            Pseudo::Backdrop(*prefix)
        }
        Pseudo::FileSelectorButton(prefix) => {
            *prefixes |= *prefix;
            Pseudo::FileSelectorButton(*prefix)
        }
        Pseudo::WebKitScrollbar(value) => {
            *prefixes |= VendorPrefix::WEBKIT;
            Pseudo::WebKitScrollbar(*value)
        }
        Pseudo::Cue => Pseudo::Cue,
        Pseudo::CueRegion => Pseudo::CueRegion,
        Pseudo::CueFunction { selector } => Pseudo::CueFunction {
            selector: allocator.boxed(clone_selector(selector, allocator, prefixes)?),
        },
        Pseudo::CueRegionFunction { selector } => Pseudo::CueRegionFunction {
            selector: allocator.boxed(clone_selector(selector, allocator, prefixes)?),
        },
        Pseudo::ViewTransition => Pseudo::ViewTransition,
        Pseudo::ViewTransitionGroup { part } => Pseudo::ViewTransitionGroup {
            part: allocator.boxed(clone_view_transition_part(part, allocator)),
        },
        Pseudo::ViewTransitionImagePair { part } => Pseudo::ViewTransitionImagePair {
            part: allocator.boxed(clone_view_transition_part(part, allocator)),
        },
        Pseudo::ViewTransitionOld { part } => Pseudo::ViewTransitionOld {
            part: allocator.boxed(clone_view_transition_part(part, allocator)),
        },
        Pseudo::ViewTransitionNew { part } => Pseudo::ViewTransitionNew {
            part: allocator.boxed(clone_view_transition_part(part, allocator)),
        },
        Pseudo::PickerFunction { identifier } => Pseudo::PickerFunction {
            identifier: *identifier,
        },
        Pseudo::PickerIcon => Pseudo::PickerIcon,
        Pseudo::Checkmark => Pseudo::Checkmark,
        Pseudo::GrammarError => Pseudo::GrammarError,
        Pseudo::SpellingError => Pseudo::SpellingError,
        Pseudo::Custom { .. } | Pseudo::CustomFunction { .. } => return None,
    })
}

fn clone_view_transition_part<'ast>(
    part: &ViewTransitionPartSelector<'ast>,
    allocator: &'ast Allocator,
) -> ViewTransitionPartSelector<'ast> {
    ViewTransitionPartSelector {
        classes: part.classes.clone(),
        name: part.name.as_ref().map(|name| {
            allocator.boxed(match &**name {
                ViewTransitionPartName::All => ViewTransitionPartName::All,
                ViewTransitionPartName::Name(name) => ViewTransitionPartName::Name(*name),
            })
        }),
    }
}
