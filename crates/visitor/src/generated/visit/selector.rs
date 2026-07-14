#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for SelectorComponent<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_selector_component(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SelectorComponent);
        let node = self;
        match node {
            SelectorComponent::Combinator(field_0) => {
                Visit::visit(field_0, visitor);
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
                Visit::visit(operator, visitor);
                visitor.visit_str(value);
                Visit::visit(case_sensitivity, visitor);
            }
            SelectorComponent::AttributeOther(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
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
                Visit::visit(field_0, visitor);
            }
            SelectorComponent::NthOf { data, selectors } => {
                Visit::visit(data, visitor);
                for value_2 in (selectors).iter() {
                    visitor.visit_selector(value_2);
                }
            }
            SelectorComponent::PseudoClass(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
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
                Visit::visit(vendor_prefix, visitor);
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
                Visit::visit((field_0).as_ref(), visitor);
            }
            SelectorComponent::Nesting => {}
        }
        visitor.leave_node(AstType::SelectorComponent);
    }
}
impl<'a> Visit<'a> for Combinator {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_combinator(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Combinator);
        let node = self;
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
}
impl<'a> Visit<'a> for AttrSelector<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_attr_selector(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AttrSelector);
        let node = self;
        if let Some(value_0) = (&node.namespace).as_ref() {
            Visit::visit(value_0, visitor);
        }
        visitor.visit_str(&node.local_name);
        visitor.visit_str(&node.local_name_lower);
        Visit::visit(&node.operation, visitor);
        visitor.leave_node(AstType::AttrSelector);
    }
}
impl<'a> Visit<'a> for NamespaceConstraint<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_namespace_constraint(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NamespaceConstraint);
        let node = self;
        match node {
            NamespaceConstraint::Any => {}
            NamespaceConstraint::Specific { prefix, url } => {
                visitor.visit_str(prefix);
                visitor.visit_str(url);
            }
        }
        visitor.leave_node(AstType::NamespaceConstraint);
    }
}
impl<'a> Visit<'a> for AttrOperation<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_attr_operation(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AttrOperation);
        let node = self;
        match node {
            AttrOperation::Exists => {}
            AttrOperation::WithValue {
                operator,
                case_sensitivity,
                expected_value,
            } => {
                Visit::visit(operator, visitor);
                Visit::visit(case_sensitivity, visitor);
                visitor.visit_str(expected_value);
            }
        }
        visitor.leave_node(AstType::AttrOperation);
    }
}
impl<'a> Visit<'a> for ParsedCaseSensitivity {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_parsed_case_sensitivity(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ParsedCaseSensitivity);
        let node = self;
        match node {
            ParsedCaseSensitivity::ExplicitCaseSensitive => {}
            ParsedCaseSensitivity::AsciiCaseInsensitive => {}
            ParsedCaseSensitivity::CaseSensitive => {}
            ParsedCaseSensitivity::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument => {}
        }
        visitor.leave_node(AstType::ParsedCaseSensitivity);
    }
}
impl<'a> Visit<'a> for AttrSelectorOperator {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_attr_selector_operator(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::AttrSelectorOperator);
        let node = self;
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
}
impl<'a> Visit<'a> for NthType {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_nth_type(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NthType);
        let node = self;
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
}
impl<'a> Visit<'a> for NthSelectorData {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_nth_selector_data(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::NthSelectorData);
        let node = self;
        Visit::visit(&node.kind, visitor);
        visitor.leave_node(AstType::NthSelectorData);
    }
}
impl<'a> Visit<'a> for Direction {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_direction(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Direction);
        let node = self;
        match node {
            Direction::Ltr => {}
            Direction::Rtl => {}
        }
        visitor.leave_node(AstType::Direction);
    }
}
impl<'a> Visit<'a> for PseudoClass<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_pseudo_class(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PseudoClass);
        let node = self;
        match node {
            PseudoClass::Lang { languages } => {
                for value_0 in (languages).iter() {
                    visitor.visit_str(value_0);
                }
            }
            PseudoClass::Dir { direction } => {
                Visit::visit(direction, visitor);
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
                Visit::visit(field_0, visitor);
            }
            PseudoClass::Open => {}
            PseudoClass::Closed => {}
            PseudoClass::Modal => {}
            PseudoClass::PictureInPicture => {}
            PseudoClass::PopoverOpen => {}
            PseudoClass::Defined => {}
            PseudoClass::AnyLink(field_0) => {
                Visit::visit(field_0, visitor);
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
                Visit::visit(field_0, visitor);
            }
            PseudoClass::ReadWrite(field_0) => {
                Visit::visit(field_0, visitor);
            }
            PseudoClass::PlaceholderShown(field_0) => {
                Visit::visit(field_0, visitor);
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
                Visit::visit(field_0, visitor);
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
                Visit::visit(field_0, visitor);
            }
            PseudoClass::Custom { name } => {
                visitor.visit_str(name);
            }
            PseudoClass::CustomFunction { name, arguments } => {
                visitor.visit_str(name);
                for value_4 in (arguments).iter() {
                    Visit::visit(value_4, visitor);
                }
            }
        }
        visitor.leave_node(AstType::PseudoClass);
    }
}
impl<'a> Visit<'a> for WebKitScrollbarPseudoClass {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_scrollbar_pseudo_class(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitScrollbarPseudoClass);
        let node = self;
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
}
impl<'a> Visit<'a> for PseudoElement<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_pseudo_element(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PseudoElement);
        let node = self;
        match node {
            PseudoElement::After => {}
            PseudoElement::Before => {}
            PseudoElement::FirstLine => {}
            PseudoElement::FirstLetter => {}
            PseudoElement::DetailsContent => {}
            PseudoElement::TargetText => {}
            PseudoElement::SearchText => {}
            PseudoElement::Selection(field_0) => {
                Visit::visit(field_0, visitor);
            }
            PseudoElement::Placeholder(field_0) => {
                Visit::visit(field_0, visitor);
            }
            PseudoElement::HighlightFunction { name } => {
                visitor.visit_str(name);
            }
            PseudoElement::Marker => {}
            PseudoElement::Backdrop(field_0) => {
                Visit::visit(field_0, visitor);
            }
            PseudoElement::FileSelectorButton(field_0) => {
                Visit::visit(field_0, visitor);
            }
            PseudoElement::WebKitScrollbar(field_0) => {
                Visit::visit(field_0, visitor);
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
                Visit::visit((part).as_ref(), visitor);
            }
            PseudoElement::ViewTransitionImagePair { part } => {
                Visit::visit((part).as_ref(), visitor);
            }
            PseudoElement::ViewTransitionOld { part } => {
                Visit::visit((part).as_ref(), visitor);
            }
            PseudoElement::ViewTransitionNew { part } => {
                Visit::visit((part).as_ref(), visitor);
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
                    Visit::visit(value_6, visitor);
                }
            }
        }
        visitor.leave_node(AstType::PseudoElement);
    }
}
impl<'a> Visit<'a> for WebKitScrollbarPseudoElement {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_scrollbar_pseudo_element(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::WebKitScrollbarPseudoElement);
        let node = self;
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
}
impl<'a> Visit<'a> for ViewTransitionPartName<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_part_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::ViewTransitionPartName);
        let node = self;
        match node {
            ViewTransitionPartName::All => {}
            ViewTransitionPartName::Name(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::ViewTransitionPartName);
    }
}
