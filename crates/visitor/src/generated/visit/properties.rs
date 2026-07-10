#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, VisitNode};
use crate::AstType;
use rs_css_ast::*;
pub fn walk_blend_mode<'a, VisitorT>(visitor: &mut VisitorT, node: &BlendMode)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BlendMode);
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
