#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitorMut};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> VisitMut<'a> for CssRule<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_css_rule(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CssRule);
        let node = self;
        match node {
            CssRule::Media(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Import(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Style(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Keyframes(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::FontFace(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::FontPaletteValues(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::FontFeatureValues(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Page(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Supports(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::CounterStyle(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Namespace(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::MozDocument(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Nesting(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::NestedDeclarations(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Viewport(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::CustomMedia(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::LayerStatement(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::LayerBlock(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Property(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Container(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Scope(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::StartingStyle(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::ViewTransition(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::PositionTry(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Ignored => {}
            CssRule::Unknown(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            CssRule::Custom(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
        }
        visitor.leave_node(AstType::CssRule);
    }
}
