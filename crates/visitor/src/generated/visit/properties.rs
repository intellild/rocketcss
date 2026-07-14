#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for BlendMode {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_blend_mode(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::BlendMode);
        let node = self;
        match node {
            BlendMode::Normal => {}
            BlendMode::Multiply => {}
            BlendMode::Screen => {}
            BlendMode::Overlay => {}
            BlendMode::Darken => {}
            BlendMode::Lighten => {}
            BlendMode::ColorDodge => {}
            BlendMode::ColorBurn => {}
            BlendMode::HardLight => {}
            BlendMode::SoftLight => {}
            BlendMode::Difference => {}
            BlendMode::Exclusion => {}
            BlendMode::Hue => {}
            BlendMode::Saturation => {}
            BlendMode::Color => {}
            BlendMode::Luminosity => {}
            BlendMode::PlusDarker => {}
            BlendMode::PlusLighter => {}
        }
        visitor.leave_node(AstType::BlendMode);
    }
}
