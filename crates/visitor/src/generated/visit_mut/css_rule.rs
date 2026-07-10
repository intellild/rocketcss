#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitMutNode};
use crate::AstType;
use rs_css_ast::*;
pub fn walk_css_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CssRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CssRule);
    match node {
        CssRule::Media(field_0) => {
            visitor.visit_media_rule((field_0).as_mut());
        }
        CssRule::Import(field_0) => {
            visitor.visit_import_rule((field_0).as_mut());
        }
        CssRule::Style(field_0) => {
            visitor.visit_style_rule((field_0).as_mut());
        }
        CssRule::Keyframes(field_0) => {
            visitor.visit_keyframes_rule((field_0).as_mut());
        }
        CssRule::FontFace(field_0) => {
            visitor.visit_font_face_rule((field_0).as_mut());
        }
        CssRule::FontPaletteValues(field_0) => {
            visitor.visit_font_palette_values_rule((field_0).as_mut());
        }
        CssRule::FontFeatureValues(field_0) => {
            visitor.visit_font_feature_values_rule((field_0).as_mut());
        }
        CssRule::Page(field_0) => {
            visitor.visit_page_rule((field_0).as_mut());
        }
        CssRule::Supports(field_0) => {
            visitor.visit_supports_rule((field_0).as_mut());
        }
        CssRule::CounterStyle(field_0) => {
            visitor.visit_counter_style_rule((field_0).as_mut());
        }
        CssRule::Namespace(field_0) => {
            visitor.visit_namespace_rule((field_0).as_mut());
        }
        CssRule::MozDocument(field_0) => {
            visitor.visit_moz_document_rule((field_0).as_mut());
        }
        CssRule::Nesting(field_0) => {
            visitor.visit_nesting_rule((field_0).as_mut());
        }
        CssRule::NestedDeclarations(field_0) => {
            visitor.visit_nested_declarations_rule((field_0).as_mut());
        }
        CssRule::Viewport(field_0) => {
            visitor.visit_viewport_rule((field_0).as_mut());
        }
        CssRule::CustomMedia(field_0) => {
            visitor.visit_custom_media_rule((field_0).as_mut());
        }
        CssRule::LayerStatement(field_0) => {
            visitor.visit_layer_statement_rule((field_0).as_mut());
        }
        CssRule::LayerBlock(field_0) => {
            visitor.visit_layer_block_rule((field_0).as_mut());
        }
        CssRule::Property(field_0) => {
            visitor.visit_property_rule((field_0).as_mut());
        }
        CssRule::Container(field_0) => {
            visitor.visit_container_rule((field_0).as_mut());
        }
        CssRule::Scope(field_0) => {
            visitor.visit_scope_rule((field_0).as_mut());
        }
        CssRule::StartingStyle(field_0) => {
            visitor.visit_starting_style_rule((field_0).as_mut());
        }
        CssRule::ViewTransition(field_0) => {
            visitor.visit_view_transition_rule((field_0).as_mut());
        }
        CssRule::PositionTry(field_0) => {
            visitor.visit_position_try_rule((field_0).as_mut());
        }
        CssRule::Ignored => {}
        CssRule::Unknown(field_0) => {
            visitor.visit_unknown_at_rule((field_0).as_mut());
        }
        CssRule::Custom(field_0) => {
            visitor.visit_default_at_rule((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::CssRule);
}
