#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for CssColor<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_css_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::CssColor);
        let node = self;
        match node {
            CssColor::CurrentColor => {}
            CssColor::Rgba(field_0) => {
                Visit::visit(field_0, visitor);
            }
            CssColor::Lab(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssColor::Predefined(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssColor::Float(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssColor::LightDark(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            CssColor::System(field_0) => {
                Visit::visit(field_0, visitor);
            }
        }
        visitor.leave_node(AstType::CssColor);
    }
}
impl<'a> Visit<'a> for RGBA {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_rgba(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::RGBA);
        let node = self;
        visitor.leave_node(AstType::RGBA);
    }
}
impl<'a> Visit<'a> for LABColor {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_lab_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LABColor);
        let node = self;
        match node {
            LABColor::Lab { a, alpha, b, l } => {}
            LABColor::Lch { alpha, c, h, l } => {}
            LABColor::Oklab { a, alpha, b, l } => {}
            LABColor::Oklch { alpha, c, h, l } => {}
        }
        visitor.leave_node(AstType::LABColor);
    }
}
impl<'a> Visit<'a> for PredefinedColor {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_predefined_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::PredefinedColor);
        let node = self;
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
}
impl<'a> Visit<'a> for FloatColor {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_float_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::FloatColor);
        let node = self;
        match node {
            FloatColor::Rgb { alpha, b, g, r } => {}
            FloatColor::Hsl { alpha, h, l, s } => {}
            FloatColor::Hwb { alpha, b, h, w } => {}
        }
        visitor.leave_node(AstType::FloatColor);
    }
}
impl<'a> Visit<'a> for LightDark<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_light_dark(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LightDark);
        let node = self;
        Visit::visit((&node.dark).as_ref(), visitor);
        Visit::visit((&node.light).as_ref(), visitor);
        visitor.leave_node(AstType::LightDark);
    }
}
impl<'a> Visit<'a> for SystemColor {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_system_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SystemColor);
        let node = self;
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
}
impl<'a> Visit<'a> for UnresolvedColor<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_unresolved_color(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::UnresolvedColor);
        let node = self;
        match node {
            UnresolvedColor::Rgb { alpha, b, g, r } => {
                for value_0 in (alpha).iter() {
                    Visit::visit(value_0, visitor);
                }
            }
            UnresolvedColor::Hsl { alpha, h, l, s } => {
                for value_1 in (alpha).iter() {
                    Visit::visit(value_1, visitor);
                }
            }
            UnresolvedColor::LightDark { dark, light } => {
                for value_2 in (dark).iter() {
                    Visit::visit(value_2, visitor);
                }
                for value_3 in (light).iter() {
                    Visit::visit(value_3, visitor);
                }
            }
        }
        visitor.leave_node(AstType::UnresolvedColor);
    }
}
