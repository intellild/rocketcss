#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for CssRule<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_css_rule(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CssRule);
        let node = self;
        match node {
            CssRule::Media(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Import(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Style(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Keyframes(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::FontFace(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::FontPaletteValues(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::FontFeatureValues(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Page(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Supports(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::CounterStyle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Namespace(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::MozDocument(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Nesting(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::NestedDeclarations(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Viewport(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::CustomMedia(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::LayerStatement(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::LayerBlock(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Property(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Container(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Scope(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::StartingStyle(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::ViewTransition(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::PositionTry(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Ignored => {}
            CssRule::Unknown(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssRule::Custom(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::CssRule);
    }
}
