#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitorMut};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> VisitMut<'a> for MediaCondition<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_condition(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaCondition);
        let node = self;
        match node {
            MediaCondition::Feature(field_0) => {
                visitor.visit_media_feature((field_0).as_mut());
            }
            MediaCondition::Not(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            MediaCondition::Operation {
                conditions,
                operator,
            } => {
                for value_2 in (conditions).iter_mut() {
                    VisitMut::visit_mut(value_2, visitor);
                }
                VisitMut::visit_mut(operator, visitor);
            }
            MediaCondition::Unknown(field_0) => {
                for value_3 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_3, visitor);
                }
            }
        }
        visitor.leave_node(AstType::MediaCondition);
    }
}
impl<'a, FeatureId> VisitMut<'a> for QueryFeature<'a, FeatureId>
where
    FeatureId: VisitMut<'a>,
{
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_query_feature(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::QueryFeature);
        let node = self;
        match node {
            QueryFeature::Plain { name, value } => {
                VisitMut::visit_mut(name, visitor);
                VisitMut::visit_mut(value, visitor);
            }
            QueryFeature::Boolean { name } => {
                VisitMut::visit_mut(name, visitor);
            }
            QueryFeature::Range {
                name,
                operator,
                value,
            } => {
                VisitMut::visit_mut(name, visitor);
                VisitMut::visit_mut(operator, visitor);
                VisitMut::visit_mut(value, visitor);
            }
            QueryFeature::Interval {
                end,
                end_operator,
                name,
                start,
                start_operator,
            } => {
                VisitMut::visit_mut((end).as_mut(), visitor);
                VisitMut::visit_mut(end_operator, visitor);
                VisitMut::visit_mut(name, visitor);
                VisitMut::visit_mut((start).as_mut(), visitor);
                VisitMut::visit_mut(start_operator, visitor);
            }
        }
        visitor.leave_node(AstType::QueryFeature);
    }
}
impl<'a, FeatureId> VisitMut<'a> for MediaFeatureName<'a, FeatureId>
where
    FeatureId: VisitMut<'a>,
{
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_name(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaFeatureName);
        let node = self;
        match node {
            MediaFeatureName::Standard(field_0) => {
                VisitMut::visit_mut(field_0, visitor);
            }
            MediaFeatureName::Custom(field_0) => {
                visitor.visit_str(field_0);
            }
            MediaFeatureName::Unknown(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::MediaFeatureName);
    }
}
impl<'a> VisitMut<'a> for MediaFeatureId {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_id(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaFeatureId);
        let node = self;
        match node {
            MediaFeatureId::Width => {}
            MediaFeatureId::Height => {}
            MediaFeatureId::AspectRatio => {}
            MediaFeatureId::Orientation => {}
            MediaFeatureId::OverflowBlock => {}
            MediaFeatureId::OverflowInline => {}
            MediaFeatureId::HorizontalViewportSegments => {}
            MediaFeatureId::VerticalViewportSegments => {}
            MediaFeatureId::DisplayMode => {}
            MediaFeatureId::Resolution => {}
            MediaFeatureId::Scan => {}
            MediaFeatureId::Grid => {}
            MediaFeatureId::Update => {}
            MediaFeatureId::EnvironmentBlending => {}
            MediaFeatureId::Color => {}
            MediaFeatureId::ColorIndex => {}
            MediaFeatureId::Monochrome => {}
            MediaFeatureId::ColorGamut => {}
            MediaFeatureId::DynamicRange => {}
            MediaFeatureId::InvertedColors => {}
            MediaFeatureId::Pointer => {}
            MediaFeatureId::Hover => {}
            MediaFeatureId::AnyPointer => {}
            MediaFeatureId::AnyHover => {}
            MediaFeatureId::NavControls => {}
            MediaFeatureId::VideoColorGamut => {}
            MediaFeatureId::VideoDynamicRange => {}
            MediaFeatureId::Scripting => {}
            MediaFeatureId::PrefersReducedMotion => {}
            MediaFeatureId::PrefersReducedTransparency => {}
            MediaFeatureId::PrefersContrast => {}
            MediaFeatureId::ForcedColors => {}
            MediaFeatureId::PrefersColorScheme => {}
            MediaFeatureId::PrefersReducedData => {}
            MediaFeatureId::DeviceWidth => {}
            MediaFeatureId::DeviceHeight => {}
            MediaFeatureId::DeviceAspectRatio => {}
            MediaFeatureId::WebkitDevicePixelRatio => {}
            MediaFeatureId::MozDevicePixelRatio => {}
        }
        visitor.leave_node(AstType::MediaFeatureId);
    }
}
impl<'a> VisitMut<'a> for MediaFeatureValue<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_value(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaFeatureValue);
        let node = self;
        match node {
            MediaFeatureValue::Length(field_0) => {
                VisitMut::visit_mut(field_0, visitor);
            }
            MediaFeatureValue::Number(field_0) => {}
            MediaFeatureValue::Integer(field_0) => {}
            MediaFeatureValue::Boolean(field_0) => {}
            MediaFeatureValue::Resolution(field_0) => {
                VisitMut::visit_mut(field_0, visitor);
            }
            MediaFeatureValue::Ratio(field_0) => {
                VisitMut::visit_mut(field_0, visitor);
            }
            MediaFeatureValue::Ident(field_0) => {
                visitor.visit_str(field_0);
            }
            MediaFeatureValue::Env(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
        }
        visitor.leave_node(AstType::MediaFeatureValue);
    }
}
impl<'a> VisitMut<'a> for MediaFeatureComparison {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_comparison(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaFeatureComparison);
        let node = self;
        match node {
            MediaFeatureComparison::Equal => {}
            MediaFeatureComparison::GreaterThan => {}
            MediaFeatureComparison::GreaterThanEqual => {}
            MediaFeatureComparison::LessThan => {}
            MediaFeatureComparison::LessThanEqual => {}
        }
        visitor.leave_node(AstType::MediaFeatureComparison);
    }
}
impl<'a> VisitMut<'a> for Operator {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_operator(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Operator);
        let node = self;
        match node {
            Operator::And => {}
            Operator::Or => {}
        }
        visitor.leave_node(AstType::Operator);
    }
}
impl<'a> VisitMut<'a> for MediaType<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_type(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaType);
        let node = self;
        match node {
            MediaType::All => {}
            MediaType::Print => {}
            MediaType::Screen => {}
            MediaType::Custom(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::MediaType);
    }
}
impl<'a> VisitMut<'a> for Qualifier {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_qualifier(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Qualifier);
        let node = self;
        match node {
            Qualifier::Only => {}
            Qualifier::Not => {}
        }
        visitor.leave_node(AstType::Qualifier);
    }
}
impl<'a> VisitMut<'a> for SupportsCondition<'a> {
    #[inline]
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.visit_supports_condition(self);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SupportsCondition);
        let node = self;
        match node {
            SupportsCondition::Not(field_0) => {
                VisitMut::visit_mut((field_0).as_mut(), visitor);
            }
            SupportsCondition::And(field_0) => {
                for value_1 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_1, visitor);
                }
            }
            SupportsCondition::Or(field_0) => {
                for value_2 in (field_0).iter_mut() {
                    VisitMut::visit_mut(value_2, visitor);
                }
            }
            SupportsCondition::Declaration { property_id, value } => {
                VisitMut::visit_mut((property_id).as_mut(), visitor);
                visitor.visit_str(value);
            }
            SupportsCondition::Selector(field_0) => {
                visitor.visit_str(field_0);
            }
            SupportsCondition::Unknown(field_0) => {
                visitor.visit_str(field_0);
            }
        }
        visitor.leave_node(AstType::SupportsCondition);
    }
}
