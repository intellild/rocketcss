#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, VisitNode};
use crate::AstType;
use rs_css_ast::*;
pub fn walk_length<'a, VisitorT>(visitor: &mut VisitorT, node: &Length<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Length);
    match node {
        Length::Value(field_0) => {
            visitor.visit_length_value((field_0).as_ref());
        }
        Length::Calc(field_0) => {
            visitor.visit_calc((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::Length);
}
pub fn walk_length_unit<'a, VisitorT>(visitor: &mut VisitorT, node: &LengthUnit)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::LengthUnit);
    match node {
        LengthUnit::Px => {}
        LengthUnit::In => {}
        LengthUnit::Cm => {}
        LengthUnit::Mm => {}
        LengthUnit::Q => {}
        LengthUnit::Pt => {}
        LengthUnit::Pc => {}
        LengthUnit::Em => {}
        LengthUnit::Rem => {}
        LengthUnit::Ex => {}
        LengthUnit::Rex => {}
        LengthUnit::Ch => {}
        LengthUnit::Rch => {}
        LengthUnit::Cap => {}
        LengthUnit::Rcap => {}
        LengthUnit::Ic => {}
        LengthUnit::Ric => {}
        LengthUnit::Lh => {}
        LengthUnit::Rlh => {}
        LengthUnit::Vw => {}
        LengthUnit::Lvw => {}
        LengthUnit::Svw => {}
        LengthUnit::Dvw => {}
        LengthUnit::Cqw => {}
        LengthUnit::Vh => {}
        LengthUnit::Lvh => {}
        LengthUnit::Svh => {}
        LengthUnit::Dvh => {}
        LengthUnit::Cqh => {}
        LengthUnit::Vi => {}
        LengthUnit::Svi => {}
        LengthUnit::Lvi => {}
        LengthUnit::Dvi => {}
        LengthUnit::Cqi => {}
        LengthUnit::Vb => {}
        LengthUnit::Svb => {}
        LengthUnit::Lvb => {}
        LengthUnit::Dvb => {}
        LengthUnit::Cqb => {}
        LengthUnit::Vmin => {}
        LengthUnit::Svmin => {}
        LengthUnit::Lvmin => {}
        LengthUnit::Dvmin => {}
        LengthUnit::Cqmin => {}
        LengthUnit::Vmax => {}
        LengthUnit::Svmax => {}
        LengthUnit::Lvmax => {}
        LengthUnit::Dvmax => {}
        LengthUnit::Cqmax => {}
    }
    visitor.leave_node(AstType::LengthUnit);
}
pub fn walk_calc<'a, V, VisitorT>(visitor: &mut VisitorT, node: &Calc<'a, V>)
where
    VisitorT: ?Sized + Visit<'a>,
    V: VisitNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::Calc);
    match node {
        Calc::Value(field_0) => {
            VisitNode::visit_node((field_0).as_ref(), visitor);
        }
        Calc::Number(field_0) => {}
        Calc::Sum(field_0) => {
            visitor.visit_calc((&(field_0).0).as_ref());
            visitor.visit_calc((&(field_0).1).as_ref());
        }
        Calc::Product(field_0) => {
            visitor.visit_calc((&(field_0).1).as_ref());
        }
        Calc::Function(field_0) => {
            visitor.visit_math_function((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::Calc);
}
pub fn walk_math_function<'a, V, VisitorT>(visitor: &mut VisitorT, node: &MathFunction<'a, V>)
where
    VisitorT: ?Sized + Visit<'a>,
    V: VisitNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::MathFunction);
    match node {
        MathFunction::Calc(field_0) => {
            visitor.visit_calc((field_0).as_ref());
        }
        MathFunction::Min(field_0) => {
            for value_1 in (field_0).iter() {
                visitor.visit_calc(value_1);
            }
        }
        MathFunction::Max(field_0) => {
            for value_2 in (field_0).iter() {
                visitor.visit_calc(value_2);
            }
        }
        MathFunction::Clamp(field_0) => {
            visitor.visit_calc((&(field_0).0).as_ref());
            visitor.visit_calc((&(field_0).1).as_ref());
            visitor.visit_calc((&(field_0).2).as_ref());
        }
        MathFunction::Round(field_0) => {
            visitor.visit_rounding_strategy(&(field_0).0);
            visitor.visit_calc((&(field_0).1).as_ref());
            visitor.visit_calc((&(field_0).2).as_ref());
        }
        MathFunction::Rem(field_0) => {
            visitor.visit_calc((&(field_0).0).as_ref());
            visitor.visit_calc((&(field_0).1).as_ref());
        }
        MathFunction::Mod(field_0) => {
            visitor.visit_calc((&(field_0).0).as_ref());
            visitor.visit_calc((&(field_0).1).as_ref());
        }
        MathFunction::Abs(field_0) => {
            visitor.visit_calc((field_0).as_ref());
        }
        MathFunction::Sign(field_0) => {
            visitor.visit_calc((field_0).as_ref());
        }
        MathFunction::Hypot(field_0) => {
            for value_14 in (field_0).iter() {
                visitor.visit_calc(value_14);
            }
        }
    }
    visitor.leave_node(AstType::MathFunction);
}
pub fn walk_rounding_strategy<'a, VisitorT>(visitor: &mut VisitorT, node: &RoundingStrategy)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::RoundingStrategy);
    match node {
        RoundingStrategy::Nearest => {}
        RoundingStrategy::Up => {}
        RoundingStrategy::Down => {}
        RoundingStrategy::ToZero => {}
    }
    visitor.leave_node(AstType::RoundingStrategy);
}
pub fn walk_resolution<'a, VisitorT>(visitor: &mut VisitorT, node: &Resolution)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Resolution);
    match node {
        Resolution::Dpi(field_0) => {}
        Resolution::Dpcm(field_0) => {}
        Resolution::Dppx(field_0) => {}
    }
    visitor.leave_node(AstType::Resolution);
}
pub fn walk_ratio<'a, VisitorT>(visitor: &mut VisitorT, node: &Ratio)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Ratio);
    visitor.leave_node(AstType::Ratio);
}
pub fn walk_angle<'a, VisitorT>(visitor: &mut VisitorT, node: &Angle)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Angle);
    match node {
        Angle::Deg(field_0) => {}
        Angle::Rad(field_0) => {}
        Angle::Grad(field_0) => {}
        Angle::Turn(field_0) => {}
    }
    visitor.leave_node(AstType::Angle);
}
pub fn walk_time<'a, VisitorT>(visitor: &mut VisitorT, node: &Time)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Time);
    match node {
        Time::Seconds(field_0) => {}
        Time::Milliseconds(field_0) => {}
    }
    visitor.leave_node(AstType::Time);
}
