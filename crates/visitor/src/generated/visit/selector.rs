#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, VisitNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_selector_component<'a, VisitorT>(visitor: &mut VisitorT, node: &SelectorComponent<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::SelectorComponent);
    match node {
        SelectorComponent::Combinator(field_0) => {
            visitor.visit_combinator(field_0);
        }
        SelectorComponent::ExplicitAnyNamespace => {}
        SelectorComponent::ExplicitNoNamespace => {}
        SelectorComponent::DefaultNamespace(field_0) => {
            visitor.visit_str(field_0);
        }
        SelectorComponent::Namespace { prefix, url } => {
            visitor.visit_str(prefix);
            visitor.visit_str(url);
        }
        SelectorComponent::ExplicitUniversalType => {}
        SelectorComponent::LocalName { name, lower_name } => {
            visitor.visit_str(name);
            visitor.visit_str(lower_name);
        }
        SelectorComponent::Id(field_0) => {
            visitor.visit_str(field_0);
        }
        SelectorComponent::Class(field_0) => {
            visitor.visit_str(field_0);
        }
        SelectorComponent::AttributeInNoNamespaceExists {
            local_name,
            local_name_lower,
        } => {
            visitor.visit_str(local_name);
            visitor.visit_str(local_name_lower);
        }
        SelectorComponent::AttributeInNoNamespace {
            local_name,
            operator,
            value,
            case_sensitivity,
            never_matches,
        } => {
            visitor.visit_str(local_name);
            visitor.visit_attr_selector_operator(operator);
            visitor.visit_str(value);
            visitor.visit_parsed_case_sensitivity(case_sensitivity);
        }
        SelectorComponent::AttributeOther(field_0) => {
            visitor.visit_attr_selector((field_0).as_ref());
        }
        SelectorComponent::Negation(field_0) => {
            for value_1 in (field_0).iter() {
                visitor.visit_selector(value_1);
            }
        }
        SelectorComponent::Root => {}
        SelectorComponent::Empty => {}
        SelectorComponent::Scope => {}
        SelectorComponent::Nth(field_0) => {
            visitor.visit_nth_selector_data(field_0);
        }
        SelectorComponent::NthOf { data, selectors } => {
            visitor.visit_nth_selector_data(data);
            for value_2 in (selectors).iter() {
                visitor.visit_selector(value_2);
            }
        }
        SelectorComponent::PseudoClass(field_0) => {
            visitor.visit_pseudo_class((field_0).as_ref());
        }
        SelectorComponent::Slotted(field_0) => {
            visitor.visit_selector((field_0).as_ref());
        }
        SelectorComponent::Part(field_0) => {
            for value_5 in (field_0).iter() {
                visitor.visit_str(value_5);
            }
        }
        SelectorComponent::Host(field_0) => {
            if let Some(value_6) = (field_0).as_ref() {
                visitor.visit_selector((value_6).as_ref());
            }
        }
        SelectorComponent::Where(field_0) => {
            for value_8 in (field_0).iter() {
                visitor.visit_selector(value_8);
            }
        }
        SelectorComponent::Is(field_0) => {
            for value_9 in (field_0).iter() {
                visitor.visit_selector(value_9);
            }
        }
        SelectorComponent::Any {
            vendor_prefix,
            selectors,
        } => {
            visitor.visit_vendor_prefix(vendor_prefix);
            for value_10 in (selectors).iter() {
                visitor.visit_selector(value_10);
            }
        }
        SelectorComponent::Has(field_0) => {
            for value_11 in (field_0).iter() {
                visitor.visit_selector(value_11);
            }
        }
        SelectorComponent::PseudoElement(field_0) => {
            visitor.visit_pseudo_element((field_0).as_ref());
        }
        SelectorComponent::Nesting => {}
    }
    visitor.leave_node(AstType::SelectorComponent);
}
pub fn walk_combinator<'a, VisitorT>(visitor: &mut VisitorT, node: &Combinator)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Combinator);
    match node {
        Combinator::Child => {}
        Combinator::Descendant => {}
        Combinator::NextSibling => {}
        Combinator::LaterSibling => {}
        Combinator::PseudoElement => {}
        Combinator::SlotAssignment => {}
        Combinator::Part => {}
        Combinator::DeepDescendant => {}
        Combinator::Deep => {}
    }
    visitor.leave_node(AstType::Combinator);
}
pub fn walk_attr_selector<'a, VisitorT>(visitor: &mut VisitorT, node: &AttrSelector<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::AttrSelector);
    if let Some(value_0) = (&node.namespace).as_ref() {
        visitor.visit_namespace_constraint(value_0);
    }
    visitor.visit_str(&node.local_name);
    visitor.visit_str(&node.local_name_lower);
    visitor.visit_attr_operation(&node.operation);
    visitor.leave_node(AstType::AttrSelector);
}
pub fn walk_namespace_constraint<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &NamespaceConstraint<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::NamespaceConstraint);
    match node {
        NamespaceConstraint::Any => {}
        NamespaceConstraint::Specific { prefix, url } => {
            visitor.visit_str(prefix);
            visitor.visit_str(url);
        }
    }
    visitor.leave_node(AstType::NamespaceConstraint);
}
pub fn walk_attr_operation<'a, VisitorT>(visitor: &mut VisitorT, node: &AttrOperation<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::AttrOperation);
    match node {
        AttrOperation::Exists => {}
        AttrOperation::WithValue {
            operator,
            case_sensitivity,
            expected_value,
        } => {
            visitor.visit_attr_selector_operator(operator);
            visitor.visit_parsed_case_sensitivity(case_sensitivity);
            visitor.visit_str(expected_value);
        }
    }
    visitor.leave_node(AstType::AttrOperation);
}
pub fn walk_parsed_case_sensitivity<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ParsedCaseSensitivity,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ParsedCaseSensitivity);
    match node {
        ParsedCaseSensitivity::ExplicitCaseSensitive => {}
        ParsedCaseSensitivity::AsciiCaseInsensitive => {}
        ParsedCaseSensitivity::CaseSensitive => {}
        ParsedCaseSensitivity::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument => {}
    }
    visitor.leave_node(AstType::ParsedCaseSensitivity);
}
pub fn walk_attr_selector_operator<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &AttrSelectorOperator,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::AttrSelectorOperator);
    match node {
        AttrSelectorOperator::Equal => {}
        AttrSelectorOperator::Includes => {}
        AttrSelectorOperator::DashMatch => {}
        AttrSelectorOperator::Prefix => {}
        AttrSelectorOperator::Substring => {}
        AttrSelectorOperator::Suffix => {}
    }
    visitor.leave_node(AstType::AttrSelectorOperator);
}
pub fn walk_nth_type<'a, VisitorT>(visitor: &mut VisitorT, node: &NthType)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::NthType);
    match node {
        NthType::Child => {}
        NthType::LastChild => {}
        NthType::OnlyChild => {}
        NthType::OfType => {}
        NthType::LastOfType => {}
        NthType::OnlyOfType => {}
        NthType::Col => {}
        NthType::LastCol => {}
    }
    visitor.leave_node(AstType::NthType);
}
pub fn walk_nth_selector_data<'a, VisitorT>(visitor: &mut VisitorT, node: &NthSelectorData)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::NthSelectorData);
    visitor.visit_nth_type(&node.kind);
    visitor.leave_node(AstType::NthSelectorData);
}
pub fn walk_direction<'a, VisitorT>(visitor: &mut VisitorT, node: &Direction)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Direction);
    match node {
        Direction::Ltr => {}
        Direction::Rtl => {}
    }
    visitor.leave_node(AstType::Direction);
}
pub fn walk_pseudo_class<'a, VisitorT>(visitor: &mut VisitorT, node: &PseudoClass<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PseudoClass);
    match node {
        PseudoClass::Lang { languages } => {
            for value_0 in (languages).iter() {
                visitor.visit_str(value_0);
            }
        }
        PseudoClass::Dir { direction } => {
            visitor.visit_direction(direction);
        }
        PseudoClass::Hover => {}
        PseudoClass::Active => {}
        PseudoClass::Focus => {}
        PseudoClass::FocusVisible => {}
        PseudoClass::FocusWithin => {}
        PseudoClass::Current => {}
        PseudoClass::Past => {}
        PseudoClass::Future => {}
        PseudoClass::Playing => {}
        PseudoClass::Paused => {}
        PseudoClass::Seeking => {}
        PseudoClass::Buffering => {}
        PseudoClass::Stalled => {}
        PseudoClass::Muted => {}
        PseudoClass::VolumeLocked => {}
        PseudoClass::Fullscreen(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoClass::Open => {}
        PseudoClass::Closed => {}
        PseudoClass::Modal => {}
        PseudoClass::PictureInPicture => {}
        PseudoClass::PopoverOpen => {}
        PseudoClass::Defined => {}
        PseudoClass::AnyLink(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoClass::Link => {}
        PseudoClass::LocalLink => {}
        PseudoClass::Target => {}
        PseudoClass::TargetCurrent => {}
        PseudoClass::TargetBefore => {}
        PseudoClass::TargetAfter => {}
        PseudoClass::TargetWithin => {}
        PseudoClass::Visited => {}
        PseudoClass::Enabled => {}
        PseudoClass::Disabled => {}
        PseudoClass::ReadOnly(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoClass::ReadWrite(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoClass::PlaceholderShown(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoClass::Default => {}
        PseudoClass::Checked => {}
        PseudoClass::Indeterminate => {}
        PseudoClass::Blank => {}
        PseudoClass::Valid => {}
        PseudoClass::Invalid => {}
        PseudoClass::InRange => {}
        PseudoClass::OutOfRange => {}
        PseudoClass::Required => {}
        PseudoClass::Optional => {}
        PseudoClass::UserValid => {}
        PseudoClass::UserInvalid => {}
        PseudoClass::Autofill(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoClass::ActiveViewTransition => {}
        PseudoClass::ActiveViewTransitionType { kinds } => {
            for value_1 in (kinds).iter() {
                visitor.visit_str(value_1);
            }
        }
        PseudoClass::State { state } => {
            visitor.visit_str(state);
        }
        PseudoClass::Local { selector } => {
            visitor.visit_selector((selector).as_ref());
        }
        PseudoClass::Global { selector } => {
            visitor.visit_selector((selector).as_ref());
        }
        PseudoClass::WebKitScrollbar(field_0) => {
            visitor.visit_web_kit_scrollbar_pseudo_class(field_0);
        }
        PseudoClass::Custom { name } => {
            visitor.visit_str(name);
        }
        PseudoClass::CustomFunction { name, arguments } => {
            visitor.visit_str(name);
            for value_4 in (arguments).iter() {
                visitor.visit_token_or_value(value_4);
            }
        }
    }
    visitor.leave_node(AstType::PseudoClass);
}
pub fn walk_web_kit_scrollbar_pseudo_class<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &WebKitScrollbarPseudoClass,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::WebKitScrollbarPseudoClass);
    match node {
        WebKitScrollbarPseudoClass::Horizontal => {}
        WebKitScrollbarPseudoClass::Vertical => {}
        WebKitScrollbarPseudoClass::Decrement => {}
        WebKitScrollbarPseudoClass::Increment => {}
        WebKitScrollbarPseudoClass::Start => {}
        WebKitScrollbarPseudoClass::End => {}
        WebKitScrollbarPseudoClass::DoubleButton => {}
        WebKitScrollbarPseudoClass::SingleButton => {}
        WebKitScrollbarPseudoClass::NoButton => {}
        WebKitScrollbarPseudoClass::CornerPresent => {}
        WebKitScrollbarPseudoClass::WindowInactive => {}
    }
    visitor.leave_node(AstType::WebKitScrollbarPseudoClass);
}
pub fn walk_pseudo_element<'a, VisitorT>(visitor: &mut VisitorT, node: &PseudoElement<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PseudoElement);
    match node {
        PseudoElement::After => {}
        PseudoElement::Before => {}
        PseudoElement::FirstLine => {}
        PseudoElement::FirstLetter => {}
        PseudoElement::DetailsContent => {}
        PseudoElement::TargetText => {}
        PseudoElement::SearchText => {}
        PseudoElement::Selection(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoElement::Placeholder(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoElement::HighlightFunction { name } => {
            visitor.visit_str(name);
        }
        PseudoElement::Marker => {}
        PseudoElement::Backdrop(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoElement::FileSelectorButton(field_0) => {
            visitor.visit_vendor_prefix(field_0);
        }
        PseudoElement::WebKitScrollbar(field_0) => {
            visitor.visit_web_kit_scrollbar_pseudo_element(field_0);
        }
        PseudoElement::Cue => {}
        PseudoElement::CueRegion => {}
        PseudoElement::CueFunction { selector } => {
            visitor.visit_selector((selector).as_ref());
        }
        PseudoElement::CueRegionFunction { selector } => {
            visitor.visit_selector((selector).as_ref());
        }
        PseudoElement::ViewTransition => {}
        PseudoElement::ViewTransitionGroup { part } => {
            visitor.visit_view_transition_part_selector((part).as_ref());
        }
        PseudoElement::ViewTransitionImagePair { part } => {
            visitor.visit_view_transition_part_selector((part).as_ref());
        }
        PseudoElement::ViewTransitionOld { part } => {
            visitor.visit_view_transition_part_selector((part).as_ref());
        }
        PseudoElement::ViewTransitionNew { part } => {
            visitor.visit_view_transition_part_selector((part).as_ref());
        }
        PseudoElement::PickerFunction { identifier } => {
            visitor.visit_str(identifier);
        }
        PseudoElement::PickerIcon => {}
        PseudoElement::Checkmark => {}
        PseudoElement::GrammarError => {}
        PseudoElement::SpellingError => {}
        PseudoElement::Custom { name } => {
            visitor.visit_str(name);
        }
        PseudoElement::CustomFunction { name, arguments } => {
            visitor.visit_str(name);
            for value_6 in (arguments).iter() {
                visitor.visit_token_or_value(value_6);
            }
        }
    }
    visitor.leave_node(AstType::PseudoElement);
}
pub fn walk_web_kit_scrollbar_pseudo_element<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &WebKitScrollbarPseudoElement,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::WebKitScrollbarPseudoElement);
    match node {
        WebKitScrollbarPseudoElement::Scrollbar => {}
        WebKitScrollbarPseudoElement::Button => {}
        WebKitScrollbarPseudoElement::Track => {}
        WebKitScrollbarPseudoElement::TrackPiece => {}
        WebKitScrollbarPseudoElement::Thumb => {}
        WebKitScrollbarPseudoElement::Corner => {}
        WebKitScrollbarPseudoElement::Resizer => {}
    }
    visitor.leave_node(AstType::WebKitScrollbarPseudoElement);
}
pub fn walk_view_transition_part_name<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ViewTransitionPartName<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ViewTransitionPartName);
    match node {
        ViewTransitionPartName::All => {}
        ViewTransitionPartName::Name(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::ViewTransitionPartName);
}
pub fn walk_selector_list<'a, VisitorT>(visitor: &mut VisitorT, node: &SelectorList<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::SelectorList);
    for value_0 in (node).iter() {
        visitor.visit_selector(value_0);
    }
    visitor.leave_node(AstType::SelectorList);
}
pub fn walk_selector<'a, VisitorT>(visitor: &mut VisitorT, node: &Selector<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Selector);
    for value_0 in (node).iter() {
        visitor.visit_selector_component(value_0);
    }
    visitor.leave_node(AstType::Selector);
}
