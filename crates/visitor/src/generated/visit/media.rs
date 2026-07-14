#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, Visitor};
use crate::AstType;
use rocketcss_ast::*;
impl<'a> Visit<'a> for MediaCondition<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_condition(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaCondition);
        let node = self;
        match node {
            MediaCondition::Feature(field_0) => {
                visitor.visit_media_feature((field_0).as_ref());
            }
            MediaCondition::Not(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            MediaCondition::Operation {
                conditions,
                operator,
            } => {
                for value_2 in (conditions).iter() {
                    Visit::visit(value_2, visitor);
                }
                Visit::visit(operator, visitor);
            }
            MediaCondition::Unknown(field_0) => {
                for value_3 in (field_0).iter() {
                    Visit::visit(value_3, visitor);
                }
            }
        }
        visitor.leave_node(AstType::MediaCondition);
    }
}
impl<'a, FeatureId> Visit<'a> for QueryFeature<'a, FeatureId>
where
    FeatureId: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_query_feature(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::QueryFeature);
        let node = self;
        match node {
            QueryFeature::Plain { name, value } => {
                Visit::visit(name, visitor);
                Visit::visit(value, visitor);
            }
            QueryFeature::Boolean { name } => {
                Visit::visit(name, visitor);
            }
            QueryFeature::Range {
                name,
                operator,
                value,
            } => {
                Visit::visit(name, visitor);
                Visit::visit(operator, visitor);
                Visit::visit(value, visitor);
            }
            QueryFeature::Interval {
                end,
                end_operator,
                name,
                start,
                start_operator,
            } => {
                Visit::visit((end).as_ref(), visitor);
                Visit::visit(end_operator, visitor);
                Visit::visit(name, visitor);
                Visit::visit((start).as_ref(), visitor);
                Visit::visit(start_operator, visitor);
            }
        }
        visitor.leave_node(AstType::QueryFeature);
    }
}
impl<'a, FeatureId> Visit<'a> for MediaFeatureName<'a, FeatureId>
where
    FeatureId: Visit<'a>,
{
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_name(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaFeatureName);
        let node = self;
        match node {
            MediaFeatureName::Standard(field_0) => {
                Visit::visit(field_0, visitor);
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
impl<'a> Visit<'a> for MediaFeatureId {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_id(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
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
impl<'a> Visit<'a> for MediaFeatureValue<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_value(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::MediaFeatureValue);
        let node = self;
        match node {
            MediaFeatureValue::Length(field_0) => {
                Visit::visit(field_0, visitor);
            }
            MediaFeatureValue::Number(field_0) => {}
            MediaFeatureValue::Integer(field_0) => {}
            MediaFeatureValue::Boolean(field_0) => {}
            MediaFeatureValue::Resolution(field_0) => {
                Visit::visit(field_0, visitor);
            }
            MediaFeatureValue::Ratio(field_0) => {
                Visit::visit(field_0, visitor);
            }
            MediaFeatureValue::Ident(field_0) => {
                visitor.visit_str(field_0);
            }
            MediaFeatureValue::Env(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
        }
        visitor.leave_node(AstType::MediaFeatureValue);
    }
}
impl<'a> Visit<'a> for MediaFeatureComparison {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_comparison(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
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
impl<'a> Visit<'a> for Operator {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_operator(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Operator);
        let node = self;
        match node {
            Operator::And => {}
            Operator::Or => {}
        }
        visitor.leave_node(AstType::Operator);
    }
}
impl<'a> Visit<'a> for MediaType<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_media_type(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
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
impl<'a> Visit<'a> for Qualifier {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_qualifier(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::Qualifier);
        let node = self;
        match node {
            Qualifier::Only => {}
            Qualifier::Not => {}
        }
        visitor.leave_node(AstType::Qualifier);
    }
}
impl<'a> Visit<'a> for SupportsCondition<'a> {
    #[inline]
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.visit_supports_condition(self);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        visitor.enter_node(AstType::SupportsCondition);
        let node = self;
        match node {
            SupportsCondition::Not(field_0) => {
                Visit::visit((field_0).as_ref(), visitor);
            }
            SupportsCondition::And(field_0) => {
                for value_1 in (field_0).iter() {
                    Visit::visit(value_1, visitor);
                }
            }
            SupportsCondition::Or(field_0) => {
                for value_2 in (field_0).iter() {
                    Visit::visit(value_2, visitor);
                }
            }
            SupportsCondition::Declaration { property_id, value } => {
                Visit::visit((property_id).as_ref(), visitor);
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
