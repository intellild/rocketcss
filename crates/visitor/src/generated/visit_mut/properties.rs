#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitMutNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_blend_mode<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BlendMode)
where
    VisitorT: ?Sized + VisitMut<'a>,
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
