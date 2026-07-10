#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitMutNode};
use crate::AstType;
use rs_css_ast::*;
pub fn walk_media_condition<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MediaCondition<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaCondition);
    match node {
        MediaCondition::Feature(field_0) => {
            visitor.visit_media_feature((field_0).as_mut());
        }
        MediaCondition::Not(field_0) => {
            visitor.visit_media_condition((field_0).as_mut());
        }
        MediaCondition::Operation {
            conditions,
            operator,
        } => {
            for value_2 in (conditions).iter_mut() {
                visitor.visit_media_condition(value_2);
            }
            visitor.visit_operator(operator);
        }
        MediaCondition::Unknown(field_0) => {
            for value_3 in (field_0).iter_mut() {
                visitor.visit_token_or_value(value_3);
            }
        }
    }
    visitor.leave_node(AstType::MediaCondition);
}
pub fn walk_query_feature<'a, FeatureId, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut QueryFeature<'a, FeatureId>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
    FeatureId: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::QueryFeature);
    match node {
        QueryFeature::Plain { name, value } => {
            visitor.visit_media_feature_name((name).as_mut());
            visitor.visit_media_feature_value((value).as_mut());
        }
        QueryFeature::Boolean { name } => {
            visitor.visit_media_feature_name((name).as_mut());
        }
        QueryFeature::Range {
            name,
            operator,
            value,
        } => {
            visitor.visit_media_feature_name((name).as_mut());
            visitor.visit_media_feature_comparison(operator);
            visitor.visit_media_feature_value((value).as_mut());
        }
        QueryFeature::Interval {
            end,
            end_operator,
            name,
            start,
            start_operator,
        } => {
            visitor.visit_media_feature_value((end).as_mut());
            visitor.visit_media_feature_comparison(end_operator);
            visitor.visit_media_feature_name((name).as_mut());
            visitor.visit_media_feature_value((start).as_mut());
            visitor.visit_media_feature_comparison(start_operator);
        }
    }
    visitor.leave_node(AstType::QueryFeature);
}
pub fn walk_media_feature_name<'a, FeatureId, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut MediaFeatureName<'a, FeatureId>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
    FeatureId: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::MediaFeatureName);
    match node {
        MediaFeatureName::Standard(field_0) => {
            VisitMutNode::visit_node(field_0, visitor);
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
pub fn walk_media_feature_id<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MediaFeatureId)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaFeatureId);
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
pub fn walk_media_feature_value<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut MediaFeatureValue<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaFeatureValue);
    match node {
        MediaFeatureValue::Length(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
        MediaFeatureValue::Number(field_0) => {}
        MediaFeatureValue::Integer(field_0) => {}
        MediaFeatureValue::Boolean(field_0) => {}
        MediaFeatureValue::Resolution(field_0) => {
            visitor.visit_resolution((field_0).as_mut());
        }
        MediaFeatureValue::Ratio(field_0) => {
            visitor.visit_ratio((field_0).as_mut());
        }
        MediaFeatureValue::Ident(field_0) => {
            visitor.visit_str(field_0);
        }
        MediaFeatureValue::Env(field_0) => {
            visitor.visit_environment_variable((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::MediaFeatureValue);
}
pub fn walk_media_feature_comparison<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut MediaFeatureComparison,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaFeatureComparison);
    match node {
        MediaFeatureComparison::Equal => {}
        MediaFeatureComparison::GreaterThan => {}
        MediaFeatureComparison::GreaterThanEqual => {}
        MediaFeatureComparison::LessThan => {}
        MediaFeatureComparison::LessThanEqual => {}
    }
    visitor.leave_node(AstType::MediaFeatureComparison);
}
pub fn walk_operator<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Operator)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Operator);
    match node {
        Operator::And => {}
        Operator::Or => {}
    }
    visitor.leave_node(AstType::Operator);
}
pub fn walk_media_type<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MediaType<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaType);
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
pub fn walk_qualifier<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Qualifier)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Qualifier);
    match node {
        Qualifier::Only => {}
        Qualifier::Not => {}
    }
    visitor.leave_node(AstType::Qualifier);
}
pub fn walk_supports_condition<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut SupportsCondition<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SupportsCondition);
    match node {
        SupportsCondition::Not(field_0) => {
            visitor.visit_supports_condition((field_0).as_mut());
        }
        SupportsCondition::And(field_0) => {
            for value_1 in (field_0).iter_mut() {
                visitor.visit_supports_condition(value_1);
            }
        }
        SupportsCondition::Or(field_0) => {
            for value_2 in (field_0).iter_mut() {
                visitor.visit_supports_condition(value_2);
            }
        }
        SupportsCondition::Declaration { property_id, value } => {
            visitor.visit_property_id((property_id).as_mut());
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
pub fn walk_media_feature<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MediaFeature<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaFeature);
    visitor.visit_query_feature(node);
    visitor.leave_node(AstType::MediaFeature);
}
