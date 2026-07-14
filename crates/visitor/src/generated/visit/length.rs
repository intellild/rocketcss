#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for Length<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_length(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Length);
        let node = self;
        match node {
            Length::Value(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Length::Calc(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Length);
    }
}
impl<'a> Visit<'a> for LengthUnit {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_length_unit(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::LengthUnit);
        let node = self;
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
}
impl<'a, V> Visit<'a> for Calc<'a, V>
where
    V: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_calc(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Calc);
        let node = self;
        match node {
            Calc::Value(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            Calc::Number(field_0) => {}
            Calc::Sum(field_0) => {
                Visit::visit((&(field_0).0).as_ref(), visitor);
                Visit::visit((&(field_0).1).as_ref(), visitor);
            }
            Calc::Product(field_0) => {
                Visit::visit((&(field_0).1).as_ref(), visitor);
            }
            Calc::Function(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::Calc);
    }
}
impl<'a, V> Visit<'a> for MathFunction<'a, V>
where
    V: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_math_function(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MathFunction);
        let node = self;
        match node {
            MathFunction::Calc(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            MathFunction::Min(field_0) => {
                for value_1 in (field_0).iter() {
                    Visit::visit(value_1, visitor);
                }
            }
            MathFunction::Max(field_0) => {
                for value_2 in (field_0).iter() {
                    Visit::visit(value_2, visitor);
                }
            }
            MathFunction::Clamp(field_0) => {
                Visit::visit((&(field_0).0).as_ref(), visitor);
                Visit::visit((&(field_0).1).as_ref(), visitor);
                Visit::visit((&(field_0).2).as_ref(), visitor);
            }
            MathFunction::Round(field_0) => {
                Visit::visit(&(field_0).0, visitor);
                Visit::visit((&(field_0).1).as_ref(), visitor);
                Visit::visit((&(field_0).2).as_ref(), visitor);
            }
            MathFunction::Rem(field_0) => {
                Visit::visit((&(field_0).0).as_ref(), visitor);
                Visit::visit((&(field_0).1).as_ref(), visitor);
            }
            MathFunction::Mod(field_0) => {
                Visit::visit((&(field_0).0).as_ref(), visitor);
                Visit::visit((&(field_0).1).as_ref(), visitor);
            }
            MathFunction::Abs(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            MathFunction::Sign(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            MathFunction::Hypot(field_0) => {
                for value_14 in (field_0).iter() {
                    Visit::visit(value_14, visitor);
                }
            }
        }
        visitor.leave_node(AstType::MathFunction);
    }
}
impl<'a> Visit<'a> for RoundingStrategy {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_rounding_strategy(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::RoundingStrategy);
        let node = self;
        match node {
            RoundingStrategy::Nearest => {}
            RoundingStrategy::Up => {}
            RoundingStrategy::Down => {}
            RoundingStrategy::ToZero => {}
        }
        visitor.leave_node(AstType::RoundingStrategy);
    }
}
impl<'a> Visit<'a> for Resolution {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_resolution(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Resolution);
        let node = self;
        match node {
            Resolution::Dpi(field_0) => {}
            Resolution::Dpcm(field_0) => {}
            Resolution::Dppx(field_0) => {}
        }
        visitor.leave_node(AstType::Resolution);
    }
}
impl<'a> Visit<'a> for Ratio {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_ratio(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Ratio);
        let node = self;
        visitor.leave_node(AstType::Ratio);
    }
}
impl<'a> Visit<'a> for Angle {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_angle(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Angle);
        let node = self;
        match node {
            Angle::Deg(field_0) => {}
            Angle::Rad(field_0) => {}
            Angle::Grad(field_0) => {}
            Angle::Turn(field_0) => {}
        }
        visitor.leave_node(AstType::Angle);
    }
}
impl<'a> Visit<'a> for Time {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_time(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Time);
        let node = self;
        match node {
            Time::Seconds(field_0) => {}
            Time::Milliseconds(field_0) => {}
        }
        visitor.leave_node(AstType::Time);
    }
}
