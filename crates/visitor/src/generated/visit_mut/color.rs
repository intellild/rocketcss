#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitMutNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_css_color<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CssColor<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CssColor);
    match node {
        CssColor::CurrentColor => {}
        CssColor::Rgba(field_0) => {
            visitor.visit_rgba(field_0);
        }
        CssColor::Lab(field_0) => {
            visitor.visit_lab_color((field_0).as_mut());
        }
        CssColor::Predefined(field_0) => {
            visitor.visit_predefined_color((field_0).as_mut());
        }
        CssColor::Float(field_0) => {
            visitor.visit_float_color((field_0).as_mut());
        }
        CssColor::LightDark(field_0) => {
            visitor.visit_light_dark((field_0).as_mut());
        }
        CssColor::System(field_0) => {
            visitor.visit_system_color(field_0);
        }
    }
    visitor.leave_node(AstType::CssColor);
}
pub fn walk_rgba<'a, VisitorT>(visitor: &mut VisitorT, node: &mut RGBA)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::RGBA);
    visitor.leave_node(AstType::RGBA);
}
pub fn walk_lab_color<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LABColor)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LABColor);
    match node {
        LABColor::Lab { a, alpha, b, l } => {}
        LABColor::Lch { alpha, c, h, l } => {}
        LABColor::Oklab { a, alpha, b, l } => {}
        LABColor::Oklch { alpha, c, h, l } => {}
    }
    visitor.leave_node(AstType::LABColor);
}
pub fn walk_predefined_color<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PredefinedColor)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PredefinedColor);
    match node {
        PredefinedColor::Srgb { alpha, b, g, r } => {}
        PredefinedColor::SrgbLinear { alpha, b, g, r } => {}
        PredefinedColor::DisplayP3 { alpha, b, g, r } => {}
        PredefinedColor::A98Rgb { alpha, b, g, r } => {}
        PredefinedColor::ProphotoRgb { alpha, b, g, r } => {}
        PredefinedColor::Rec2020 { alpha, b, g, r } => {}
        PredefinedColor::XyzD50 { alpha, x, y, z } => {}
        PredefinedColor::XyzD65 { alpha, x, y, z } => {}
    }
    visitor.leave_node(AstType::PredefinedColor);
}
pub fn walk_float_color<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FloatColor)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FloatColor);
    match node {
        FloatColor::Rgb { alpha, b, g, r } => {}
        FloatColor::Hsl { alpha, h, l, s } => {}
        FloatColor::Hwb { alpha, b, h, w } => {}
    }
    visitor.leave_node(AstType::FloatColor);
}
pub fn walk_light_dark<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LightDark<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LightDark);
    visitor.visit_css_color((&mut node.dark).as_mut());
    visitor.visit_css_color((&mut node.light).as_mut());
    visitor.leave_node(AstType::LightDark);
}
pub fn walk_system_color<'a, VisitorT>(visitor: &mut VisitorT, node: &mut SystemColor)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SystemColor);
    match node {
        SystemColor::Accentcolor => {}
        SystemColor::Accentcolortext => {}
        SystemColor::Activetext => {}
        SystemColor::Buttonborder => {}
        SystemColor::Buttonface => {}
        SystemColor::Buttontext => {}
        SystemColor::Canvas => {}
        SystemColor::Canvastext => {}
        SystemColor::Field => {}
        SystemColor::Fieldtext => {}
        SystemColor::Graytext => {}
        SystemColor::Highlight => {}
        SystemColor::Highlighttext => {}
        SystemColor::Linktext => {}
        SystemColor::Mark => {}
        SystemColor::Marktext => {}
        SystemColor::Selecteditem => {}
        SystemColor::Selecteditemtext => {}
        SystemColor::Visitedtext => {}
        SystemColor::Activeborder => {}
        SystemColor::Activecaption => {}
        SystemColor::Appworkspace => {}
        SystemColor::Background => {}
        SystemColor::Buttonhighlight => {}
        SystemColor::Buttonshadow => {}
        SystemColor::Captiontext => {}
        SystemColor::Inactiveborder => {}
        SystemColor::Inactivecaption => {}
        SystemColor::Inactivecaptiontext => {}
        SystemColor::Infobackground => {}
        SystemColor::Infotext => {}
        SystemColor::Menu => {}
        SystemColor::Menutext => {}
        SystemColor::Scrollbar => {}
        SystemColor::Threeddarkshadow => {}
        SystemColor::Threedface => {}
        SystemColor::Threedhighlight => {}
        SystemColor::Threedlightshadow => {}
        SystemColor::Threedshadow => {}
        SystemColor::Window => {}
        SystemColor::Windowframe => {}
        SystemColor::Windowtext => {}
    }
    visitor.leave_node(AstType::SystemColor);
}
pub fn walk_unresolved_color<'a, VisitorT>(visitor: &mut VisitorT, node: &mut UnresolvedColor<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::UnresolvedColor);
    match node {
        UnresolvedColor::Rgb { alpha, b, g, r } => {
            for value_0 in (alpha).iter_mut() {
                visitor.visit_token_or_value(value_0);
            }
        }
        UnresolvedColor::Hsl { alpha, h, l, s } => {
            for value_1 in (alpha).iter_mut() {
                visitor.visit_token_or_value(value_1);
            }
        }
        UnresolvedColor::LightDark { dark, light } => {
            for value_2 in (dark).iter_mut() {
                visitor.visit_token_or_value(value_2);
            }
            for value_3 in (light).iter_mut() {
                visitor.visit_token_or_value(value_3);
            }
        }
    }
    visitor.leave_node(AstType::UnresolvedColor);
}
