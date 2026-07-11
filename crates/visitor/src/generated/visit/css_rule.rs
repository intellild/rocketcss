#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, VisitNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_css_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &CssRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::CssRule);
    match node {
        CssRule::Media(field_0) => {
            visitor.visit_media_rule((field_0).as_ref());
        }
        CssRule::Import(field_0) => {
            visitor.visit_import_rule((field_0).as_ref());
        }
        CssRule::Style(field_0) => {
            visitor.visit_style_rule((field_0).as_ref());
        }
        CssRule::Keyframes(field_0) => {
            visitor.visit_keyframes_rule((field_0).as_ref());
        }
        CssRule::FontFace(field_0) => {
            visitor.visit_font_face_rule((field_0).as_ref());
        }
        CssRule::FontPaletteValues(field_0) => {
            visitor.visit_font_palette_values_rule((field_0).as_ref());
        }
        CssRule::FontFeatureValues(field_0) => {
            visitor.visit_font_feature_values_rule((field_0).as_ref());
        }
        CssRule::Page(field_0) => {
            visitor.visit_page_rule((field_0).as_ref());
        }
        CssRule::Supports(field_0) => {
            visitor.visit_supports_rule((field_0).as_ref());
        }
        CssRule::CounterStyle(field_0) => {
            visitor.visit_counter_style_rule((field_0).as_ref());
        }
        CssRule::Namespace(field_0) => {
            visitor.visit_namespace_rule((field_0).as_ref());
        }
        CssRule::MozDocument(field_0) => {
            visitor.visit_moz_document_rule((field_0).as_ref());
        }
        CssRule::Nesting(field_0) => {
            visitor.visit_nesting_rule((field_0).as_ref());
        }
        CssRule::NestedDeclarations(field_0) => {
            visitor.visit_nested_declarations_rule((field_0).as_ref());
        }
        CssRule::Viewport(field_0) => {
            visitor.visit_viewport_rule((field_0).as_ref());
        }
        CssRule::CustomMedia(field_0) => {
            visitor.visit_custom_media_rule((field_0).as_ref());
        }
        CssRule::LayerStatement(field_0) => {
            visitor.visit_layer_statement_rule((field_0).as_ref());
        }
        CssRule::LayerBlock(field_0) => {
            visitor.visit_layer_block_rule((field_0).as_ref());
        }
        CssRule::Property(field_0) => {
            visitor.visit_property_rule((field_0).as_ref());
        }
        CssRule::Container(field_0) => {
            visitor.visit_container_rule((field_0).as_ref());
        }
        CssRule::Scope(field_0) => {
            visitor.visit_scope_rule((field_0).as_ref());
        }
        CssRule::StartingStyle(field_0) => {
            visitor.visit_starting_style_rule((field_0).as_ref());
        }
        CssRule::ViewTransition(field_0) => {
            visitor.visit_view_transition_rule((field_0).as_ref());
        }
        CssRule::PositionTry(field_0) => {
            visitor.visit_position_try_rule((field_0).as_ref());
        }
        CssRule::Ignored => {}
        CssRule::Unknown(field_0) => {
            visitor.visit_unknown_at_rule((field_0).as_ref());
        }
        CssRule::Custom(field_0) => {
            visitor.visit_default_at_rule((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::CssRule);
}
