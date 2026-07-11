#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{VisitMut, VisitMutNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_keyframe_selector<'a, VisitorT>(visitor: &mut VisitorT, node: &mut KeyframeSelector<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::KeyframeSelector);
    match node {
        KeyframeSelector::Percentage(field_0) => {}
        KeyframeSelector::From => {}
        KeyframeSelector::To => {}
        KeyframeSelector::TimelineRangePercentage(field_0) => {
            visitor.visit_timeline_range_percentage((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::KeyframeSelector);
}
pub fn walk_keyframes_name<'a, VisitorT>(visitor: &mut VisitorT, node: &mut KeyframesName<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::KeyframesName);
    match node {
        KeyframesName::Ident(field_0) => {
            visitor.visit_str(field_0);
        }
        KeyframesName::Custom(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::KeyframesName);
}
pub fn walk_font_face_property<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut FontFaceProperty<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFaceProperty);
    match node {
        FontFaceProperty::Source(field_0) => {
            for value_0 in (field_0).iter_mut() {
                visitor.visit_source(value_0);
            }
        }
        FontFaceProperty::FontFamily(field_0) => {
            visitor.visit_font_family((field_0).as_mut());
        }
        FontFaceProperty::FontStyle(field_0) => {
            visitor.visit_font_face_style((field_0).as_mut());
        }
        FontFaceProperty::FontWeight(field_0) => {
            visitor.visit_size_2_d((field_0).as_mut());
        }
        FontFaceProperty::FontStretch(field_0) => {
            visitor.visit_size_2_d((field_0).as_mut());
        }
        FontFaceProperty::UnicodeRange(field_0) => {
            for value_5 in (field_0).iter_mut() {
                visitor.visit_unicode_range(value_5);
            }
        }
        FontFaceProperty::Custom(field_0) => {
            visitor.visit_custom_property((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::FontFaceProperty);
}
pub fn walk_source<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Source<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Source);
    match node {
        Source::Url(field_0) => {
            visitor.visit_url_source((field_0).as_mut());
        }
        Source::Local(field_0) => {
            visitor.visit_font_family((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::Source);
}
pub fn walk_font_format<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontFormat<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFormat);
    match node {
        FontFormat::Woff => {}
        FontFormat::Woff2 => {}
        FontFormat::Truetype => {}
        FontFormat::Opentype => {}
        FontFormat::EmbeddedOpentype => {}
        FontFormat::Collection => {}
        FontFormat::Svg => {}
        FontFormat::String(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::FontFormat);
}
pub fn walk_font_technology<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontTechnology)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontTechnology);
    match node {
        FontTechnology::FeaturesOpentype => {}
        FontTechnology::FeaturesAat => {}
        FontTechnology::FeaturesGraphite => {}
        FontTechnology::ColorColrv0 => {}
        FontTechnology::ColorColrv1 => {}
        FontTechnology::ColorSvg => {}
        FontTechnology::ColorSbix => {}
        FontTechnology::ColorCbdt => {}
        FontTechnology::Variations => {}
        FontTechnology::Palettes => {}
        FontTechnology::Incremental => {}
    }
    visitor.leave_node(AstType::FontTechnology);
}
pub fn walk_font_face_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontFaceStyle<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFaceStyle);
    match node {
        FontFaceStyle::Normal => {}
        FontFaceStyle::Italic => {}
        FontFaceStyle::Oblique(field_0) => {
            visitor.visit_size_2_d((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::FontFaceStyle);
}
pub fn walk_font_palette_values_property<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut FontPaletteValuesProperty<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontPaletteValuesProperty);
    match node {
        FontPaletteValuesProperty::FontFamily(field_0) => {
            visitor.visit_font_family((field_0).as_mut());
        }
        FontPaletteValuesProperty::BasePalette(field_0) => {
            visitor.visit_base_palette((field_0).as_mut());
        }
        FontPaletteValuesProperty::OverrideColors(field_0) => {
            for value_2 in (field_0).iter_mut() {
                visitor.visit_override_colors(value_2);
            }
        }
        FontPaletteValuesProperty::Custom(field_0) => {
            visitor.visit_custom_property((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::FontPaletteValuesProperty);
}
pub fn walk_base_palette<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BasePalette)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BasePalette);
    match node {
        BasePalette::Light => {}
        BasePalette::Dark => {}
        BasePalette::Integer(field_0) => {}
    }
    visitor.leave_node(AstType::BasePalette);
}
pub fn walk_font_feature_subrule_type<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut FontFeatureSubruleType,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFeatureSubruleType);
    match node {
        FontFeatureSubruleType::Stylistic => {}
        FontFeatureSubruleType::HistoricalForms => {}
        FontFeatureSubruleType::Styleset => {}
        FontFeatureSubruleType::CharacterVariant => {}
        FontFeatureSubruleType::Swash => {}
        FontFeatureSubruleType::Ornaments => {}
        FontFeatureSubruleType::Annotation => {}
    }
    visitor.leave_node(AstType::FontFeatureSubruleType);
}
pub fn walk_page_margin_box<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PageMarginBox)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PageMarginBox);
    match node {
        PageMarginBox::TopLeftCorner => {}
        PageMarginBox::TopLeft => {}
        PageMarginBox::TopCenter => {}
        PageMarginBox::TopRight => {}
        PageMarginBox::TopRightCorner => {}
        PageMarginBox::LeftTop => {}
        PageMarginBox::LeftMiddle => {}
        PageMarginBox::LeftBottom => {}
        PageMarginBox::RightTop => {}
        PageMarginBox::RightMiddle => {}
        PageMarginBox::RightBottom => {}
        PageMarginBox::BottomLeftCorner => {}
        PageMarginBox::BottomLeft => {}
        PageMarginBox::BottomCenter => {}
        PageMarginBox::BottomRight => {}
        PageMarginBox::BottomRightCorner => {}
    }
    visitor.leave_node(AstType::PageMarginBox);
}
pub fn walk_page_pseudo_class<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PagePseudoClass)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PagePseudoClass);
    match node {
        PagePseudoClass::Left => {}
        PagePseudoClass::Right => {}
        PagePseudoClass::First => {}
        PagePseudoClass::Last => {}
        PagePseudoClass::Blank => {}
    }
    visitor.leave_node(AstType::PagePseudoClass);
}
pub fn walk_parsed_component<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ParsedComponent<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ParsedComponent);
    match node {
        ParsedComponent::Length(field_0) => {
            visitor.visit_length((field_0).as_mut());
        }
        ParsedComponent::Number(field_0) => {}
        ParsedComponent::Percentage(field_0) => {}
        ParsedComponent::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_mut());
        }
        ParsedComponent::String(field_0) => {
            visitor.visit_str(field_0);
        }
        ParsedComponent::Color(field_0) => {
            visitor.visit_css_color((field_0).as_mut());
        }
        ParsedComponent::Image(field_0) => {
            visitor.visit_image((field_0).as_mut());
        }
        ParsedComponent::Url(field_0) => {
            visitor.visit_url((field_0).as_mut());
        }
        ParsedComponent::Integer(field_0) => {}
        ParsedComponent::Angle(field_0) => {
            visitor.visit_angle((field_0).as_mut());
        }
        ParsedComponent::Time(field_0) => {
            visitor.visit_time((field_0).as_mut());
        }
        ParsedComponent::Resolution(field_0) => {
            visitor.visit_resolution((field_0).as_mut());
        }
        ParsedComponent::TransformFunction(field_0) => {
            visitor.visit_transform((field_0).as_mut());
        }
        ParsedComponent::TransformList(field_0) => {
            for value_9 in (field_0).iter_mut() {
                visitor.visit_transform(value_9);
            }
        }
        ParsedComponent::CustomIdent(field_0) => {
            visitor.visit_str(field_0);
        }
        ParsedComponent::Literal(field_0) => {
            visitor.visit_str(field_0);
        }
        ParsedComponent::Repeated {
            components,
            multiplier,
        } => {
            for value_10 in (components).iter_mut() {
                visitor.visit_parsed_component(value_10);
            }
            visitor.visit_multiplier(multiplier);
        }
        ParsedComponent::TokenList(field_0) => {
            for value_11 in (field_0).iter_mut() {
                visitor.visit_token_or_value(value_11);
            }
        }
    }
    visitor.leave_node(AstType::ParsedComponent);
}
pub fn walk_multiplier<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Multiplier)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Multiplier);
    match node {
        Multiplier::None => {}
        Multiplier::Space => {}
        Multiplier::Comma => {}
    }
    visitor.leave_node(AstType::Multiplier);
}
pub fn walk_syntax_string<'a, VisitorT>(visitor: &mut VisitorT, node: &mut SyntaxString<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SyntaxString);
    match node {
        SyntaxString::Components(field_0) => {
            for value_0 in (field_0).iter_mut() {
                visitor.visit_syntax_component(value_0);
            }
        }
        SyntaxString::Universal => {}
    }
    visitor.leave_node(AstType::SyntaxString);
}
pub fn walk_syntax_component_kind<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut SyntaxComponentKind<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SyntaxComponentKind);
    match node {
        SyntaxComponentKind::Length => {}
        SyntaxComponentKind::Number => {}
        SyntaxComponentKind::Percentage => {}
        SyntaxComponentKind::LengthPercentage => {}
        SyntaxComponentKind::String => {}
        SyntaxComponentKind::Color => {}
        SyntaxComponentKind::Image => {}
        SyntaxComponentKind::Url => {}
        SyntaxComponentKind::Integer => {}
        SyntaxComponentKind::Angle => {}
        SyntaxComponentKind::Time => {}
        SyntaxComponentKind::Resolution => {}
        SyntaxComponentKind::TransformFunction => {}
        SyntaxComponentKind::TransformList => {}
        SyntaxComponentKind::CustomIdent => {}
        SyntaxComponentKind::Literal(field_0) => {
            visitor.visit_str(field_0);
        }
    }
    visitor.leave_node(AstType::SyntaxComponentKind);
}
pub fn walk_container_condition<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ContainerCondition<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ContainerCondition);
    match node {
        ContainerCondition::Feature(field_0) => {
            visitor.visit_container_size_feature((field_0).as_mut());
        }
        ContainerCondition::Not(field_0) => {
            visitor.visit_container_condition((field_0).as_mut());
        }
        ContainerCondition::Operation {
            conditions,
            operator,
        } => {
            for value_2 in (conditions).iter_mut() {
                visitor.visit_container_condition(value_2);
            }
            visitor.visit_operator(operator);
        }
        ContainerCondition::Style(field_0) => {
            visitor.visit_style_query((field_0).as_mut());
        }
        ContainerCondition::ScrollState(field_0) => {
            visitor.visit_scroll_state_query((field_0).as_mut());
        }
        ContainerCondition::Unknown(field_0) => {
            for value_5 in (field_0).iter_mut() {
                visitor.visit_token_or_value(value_5);
            }
        }
    }
    visitor.leave_node(AstType::ContainerCondition);
}
pub fn walk_container_size_feature_id<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ContainerSizeFeatureId,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ContainerSizeFeatureId);
    match node {
        ContainerSizeFeatureId::Width => {}
        ContainerSizeFeatureId::Height => {}
        ContainerSizeFeatureId::InlineSize => {}
        ContainerSizeFeatureId::BlockSize => {}
        ContainerSizeFeatureId::AspectRatio => {}
        ContainerSizeFeatureId::Orientation => {}
    }
    visitor.leave_node(AstType::ContainerSizeFeatureId);
}
pub fn walk_style_query<'a, VisitorT>(visitor: &mut VisitorT, node: &mut StyleQuery<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::StyleQuery);
    match node {
        StyleQuery::Declaration(field_0) => {
            visitor.visit_declaration((field_0).as_mut());
        }
        StyleQuery::Property(field_0) => {
            visitor.visit_property_id((field_0).as_mut());
        }
        StyleQuery::Not(field_0) => {
            visitor.visit_style_query((field_0).as_mut());
        }
        StyleQuery::Operation {
            conditions,
            operator,
        } => {
            for value_3 in (conditions).iter_mut() {
                visitor.visit_style_query(value_3);
            }
            visitor.visit_operator(operator);
        }
    }
    visitor.leave_node(AstType::StyleQuery);
}
pub fn walk_scroll_state_query<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ScrollStateQuery<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollStateQuery);
    match node {
        ScrollStateQuery::Feature(field_0) => {
            visitor.visit_scroll_state_feature((field_0).as_mut());
        }
        ScrollStateQuery::Not(field_0) => {
            visitor.visit_scroll_state_query((field_0).as_mut());
        }
        ScrollStateQuery::Operation {
            conditions,
            operator,
        } => {
            for value_2 in (conditions).iter_mut() {
                visitor.visit_scroll_state_query(value_2);
            }
            visitor.visit_operator(operator);
        }
    }
    visitor.leave_node(AstType::ScrollStateQuery);
}
pub fn walk_scroll_state_feature_id<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ScrollStateFeatureId,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollStateFeatureId);
    match node {
        ScrollStateFeatureId::Stuck => {}
        ScrollStateFeatureId::Snapped => {}
        ScrollStateFeatureId::Scrollable => {}
        ScrollStateFeatureId::Scrolled => {}
    }
    visitor.leave_node(AstType::ScrollStateFeatureId);
}
pub fn walk_view_transition_property<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ViewTransitionProperty<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ViewTransitionProperty);
    match node {
        ViewTransitionProperty::Navigation(field_0) => {
            visitor.visit_navigation(field_0);
        }
        ViewTransitionProperty::Types(field_0) => {
            visitor.visit_none_or_custom_ident_list((field_0).as_mut());
        }
        ViewTransitionProperty::Custom(field_0) => {
            visitor.visit_custom_property((field_0).as_mut());
        }
    }
    visitor.leave_node(AstType::ViewTransitionProperty);
}
pub fn walk_navigation<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Navigation)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Navigation);
    match node {
        Navigation::None => {}
        Navigation::Auto => {}
    }
    visitor.leave_node(AstType::Navigation);
}
pub fn walk_default_at_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut DefaultAtRule)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::DefaultAtRule);
    visitor.leave_node(AstType::DefaultAtRule);
}
pub fn walk_style_sheet<'a, VisitorT>(visitor: &mut VisitorT, node: &mut StyleSheet<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::StyleSheet);
    for value_0 in (&mut node.license_comments).iter_mut() {
        visitor.visit_str(value_0);
    }
    for value_1 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_1);
    }
    for value_2 in (&mut node.source_map_urls).iter_mut() {
        if let Some(value_3) = (value_2).as_mut() {
            visitor.visit_str(value_3);
        }
    }
    for value_4 in (&mut node.sources).iter_mut() {
        visitor.visit_str(value_4);
    }
    visitor.leave_node(AstType::StyleSheet);
}
pub fn walk_media_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MediaRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaRule);
    visitor.visit_span(&mut node.span);
    visitor.visit_media_list((&mut node.query).as_mut());
    for value_1 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_1);
    }
    visitor.leave_node(AstType::MediaRule);
}
pub fn walk_media_list<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MediaList<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaList);
    for value_0 in (&mut node.media_queries).iter_mut() {
        visitor.visit_media_query(value_0);
    }
    visitor.leave_node(AstType::MediaList);
}
pub fn walk_media_query<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MediaQuery<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MediaQuery);
    if let Some(value_0) = (&mut node.condition).as_mut() {
        visitor.visit_media_condition((value_0).as_mut());
    }
    visitor.visit_media_type((&mut node.media_type).as_mut());
    if let Some(value_3) = (&mut node.qualifier).as_mut() {
        visitor.visit_qualifier(value_3);
    }
    visitor.leave_node(AstType::MediaQuery);
}
pub fn walk_length_value<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LengthValue)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LengthValue);
    visitor.visit_length_unit(&mut node.unit);
    visitor.leave_node(AstType::LengthValue);
}
pub fn walk_environment_variable<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut EnvironmentVariable<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::EnvironmentVariable);
    if let Some(value_0) = (&mut node.fallback).as_mut() {
        for value_1 in (value_0).iter_mut() {
            visitor.visit_token_or_value(value_1);
        }
    }
    for value_2 in (&mut node.indices).iter_mut() {}
    visitor.visit_environment_variable_name((&mut node.name).as_mut());
    visitor.leave_node(AstType::EnvironmentVariable);
}
pub fn walk_url<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Url<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Url);
    visitor.visit_span(&mut node.span);
    visitor.visit_str(&mut node.url);
    visitor.leave_node(AstType::Url);
}
pub fn walk_variable<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Variable<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Variable);
    if let Some(value_0) = (&mut node.fallback).as_mut() {
        for value_1 in (value_0).iter_mut() {
            visitor.visit_token_or_value(value_1);
        }
    }
    visitor.visit_dashed_ident_reference((&mut node.name).as_mut());
    visitor.leave_node(AstType::Variable);
}
pub fn walk_dashed_ident_reference<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut DashedIdentReference<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::DashedIdentReference);
    if let Some(value_0) = (&mut node.from).as_mut() {
        visitor.visit_specifier((value_0).as_mut());
    }
    visitor.visit_str(&mut node.ident);
    visitor.leave_node(AstType::DashedIdentReference);
}
pub fn walk_function<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Function<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Function);
    for value_0 in (&mut node.arguments).iter_mut() {
        visitor.visit_token_or_value(value_0);
    }
    visitor.visit_str(&mut node.name);
    visitor.leave_node(AstType::Function);
}
pub fn walk_import_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ImportRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ImportRule);
    if let Some(value_0) = (&mut node.layer).as_mut() {
        for value_1 in (value_0).iter_mut() {
            visitor.visit_str(value_1);
        }
    }
    visitor.visit_span(&mut node.span);
    if let Some(value_2) = (&mut node.media).as_mut() {
        visitor.visit_media_list((value_2).as_mut());
    }
    if let Some(value_4) = (&mut node.supports).as_mut() {
        visitor.visit_supports_condition((value_4).as_mut());
    }
    visitor.visit_str(&mut node.url);
    visitor.leave_node(AstType::ImportRule);
}
pub fn walk_style_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut StyleRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::StyleRule);
    visitor.visit_declaration_block((&mut node.declarations).as_mut());
    visitor.visit_span(&mut node.span);
    for value_1 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_1);
    }
    visitor.visit_selector_list((&mut node.selectors).as_mut());
    visitor.visit_vendor_prefix(&mut node.vendor_prefix);
    visitor.leave_node(AstType::StyleRule);
}
pub fn walk_declaration_block<'a, VisitorT>(visitor: &mut VisitorT, node: &mut DeclarationBlock<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::DeclarationBlock);
    for value_0 in (&mut node.declarations).iter_mut() {
        visitor.visit_declaration(value_0);
    }
    visitor.leave_node(AstType::DeclarationBlock);
}
pub fn walk_position<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Position<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Position);
    visitor.visit_position_component((&mut node.x).as_mut());
    visitor.visit_position_component((&mut node.y).as_mut());
    visitor.leave_node(AstType::Position);
}
pub fn walk_web_kit_gradient_point<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut WebKitGradientPoint<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::WebKitGradientPoint);
    visitor.visit_web_kit_gradient_point_component((&mut node.x).as_mut());
    visitor.visit_web_kit_gradient_point_component((&mut node.y).as_mut());
    visitor.leave_node(AstType::WebKitGradientPoint);
}
pub fn walk_web_kit_color_stop<'a, VisitorT>(visitor: &mut VisitorT, node: &mut WebKitColorStop<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::WebKitColorStop);
    visitor.visit_css_color((&mut node.color).as_mut());
    visitor.leave_node(AstType::WebKitColorStop);
}
pub fn walk_image_set<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ImageSet<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ImageSet);
    for value_0 in (&mut node.options).iter_mut() {
        visitor.visit_image_set_option(value_0);
    }
    visitor.visit_vendor_prefix(&mut node.vendor_prefix);
    visitor.leave_node(AstType::ImageSet);
}
pub fn walk_image_set_option<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ImageSetOption<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ImageSetOption);
    if let Some(value_0) = (&mut node.file_type).as_mut() {
        visitor.visit_str(value_0);
    }
    visitor.visit_image((&mut node.image).as_mut());
    visitor.visit_resolution((&mut node.resolution).as_mut());
    visitor.leave_node(AstType::ImageSetOption);
}
pub fn walk_background_position<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BackgroundPosition<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BackgroundPosition);
    visitor.visit_position_component((&mut node.x).as_mut());
    visitor.visit_position_component((&mut node.y).as_mut());
    visitor.leave_node(AstType::BackgroundPosition);
}
pub fn walk_background_repeat<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BackgroundRepeat)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BackgroundRepeat);
    visitor.visit_background_repeat_keyword(&mut node.x);
    visitor.visit_background_repeat_keyword(&mut node.y);
    visitor.leave_node(AstType::BackgroundRepeat);
}
pub fn walk_background<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Background<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Background);
    visitor.visit_background_attachment(&mut node.attachment);
    visitor.visit_background_clip(&mut node.clip);
    visitor.visit_css_color((&mut node.color).as_mut());
    visitor.visit_image((&mut node.image).as_mut());
    visitor.visit_background_origin(&mut node.origin);
    visitor.visit_background_position((&mut node.position).as_mut());
    visitor.visit_background_repeat((&mut node.repeat).as_mut());
    visitor.visit_background_size((&mut node.size).as_mut());
    visitor.leave_node(AstType::Background);
}
pub fn walk_box_shadow<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BoxShadow<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BoxShadow);
    visitor.visit_length((&mut node.blur).as_mut());
    visitor.visit_css_color((&mut node.color).as_mut());
    visitor.visit_length((&mut node.spread).as_mut());
    visitor.visit_length((&mut node.x_offset).as_mut());
    visitor.visit_length((&mut node.y_offset).as_mut());
    visitor.leave_node(AstType::BoxShadow);
}
pub fn walk_aspect_ratio<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AspectRatio<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AspectRatio);
    if let Some(value_0) = (&mut node.ratio).as_mut() {
        visitor.visit_ratio((value_0).as_mut());
    }
    visitor.leave_node(AstType::AspectRatio);
}
pub fn walk_overflow<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Overflow)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Overflow);
    visitor.visit_overflow_keyword(&mut node.x);
    visitor.visit_overflow_keyword(&mut node.y);
    visitor.leave_node(AstType::Overflow);
}
pub fn walk_inset_block<'a, VisitorT>(visitor: &mut VisitorT, node: &mut InsetBlock<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::InsetBlock);
    visitor.visit_length_percentage_or_auto((&mut node.block_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.block_start).as_mut());
    visitor.leave_node(AstType::InsetBlock);
}
pub fn walk_inset_inline<'a, VisitorT>(visitor: &mut VisitorT, node: &mut InsetInline<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::InsetInline);
    visitor.visit_length_percentage_or_auto((&mut node.inline_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.inline_start).as_mut());
    visitor.leave_node(AstType::InsetInline);
}
pub fn walk_inset<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Inset<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Inset);
    visitor.visit_length_percentage_or_auto((&mut node.bottom).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.left).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.right).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.top).as_mut());
    visitor.leave_node(AstType::Inset);
}
pub fn walk_border_radius<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderRadius<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderRadius);
    visitor.visit_size_2_d((&mut node.bottom_left).as_mut());
    visitor.visit_size_2_d((&mut node.bottom_right).as_mut());
    visitor.visit_size_2_d((&mut node.top_left).as_mut());
    visitor.visit_size_2_d((&mut node.top_right).as_mut());
    visitor.leave_node(AstType::BorderRadius);
}
pub fn walk_border_image_repeat<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderImageRepeat)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderImageRepeat);
    visitor.visit_border_image_repeat_keyword(&mut node.horizontal);
    visitor.visit_border_image_repeat_keyword(&mut node.vertical);
    visitor.leave_node(AstType::BorderImageRepeat);
}
pub fn walk_border_image_slice<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BorderImageSlice<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderImageSlice);
    visitor.visit_rect((&mut node.offsets).as_mut());
    visitor.leave_node(AstType::BorderImageSlice);
}
pub fn walk_border_image<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderImage<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderImage);
    visitor.visit_rect((&mut node.outset).as_mut());
    visitor.visit_border_image_repeat((&mut node.repeat).as_mut());
    visitor.visit_border_image_slice((&mut node.slice).as_mut());
    visitor.visit_image((&mut node.source).as_mut());
    visitor.visit_rect((&mut node.width).as_mut());
    visitor.leave_node(AstType::BorderImage);
}
pub fn walk_border_color<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderColor<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderColor);
    visitor.visit_css_color((&mut node.bottom).as_mut());
    visitor.visit_css_color((&mut node.left).as_mut());
    visitor.visit_css_color((&mut node.right).as_mut());
    visitor.visit_css_color((&mut node.top).as_mut());
    visitor.leave_node(AstType::BorderColor);
}
pub fn walk_border_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderStyle)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderStyle);
    visitor.visit_line_style(&mut node.bottom);
    visitor.visit_line_style(&mut node.left);
    visitor.visit_line_style(&mut node.right);
    visitor.visit_line_style(&mut node.top);
    visitor.leave_node(AstType::BorderStyle);
}
pub fn walk_border_width<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderWidth<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderWidth);
    visitor.visit_border_side_width((&mut node.bottom).as_mut());
    visitor.visit_border_side_width((&mut node.left).as_mut());
    visitor.visit_border_side_width((&mut node.right).as_mut());
    visitor.visit_border_side_width((&mut node.top).as_mut());
    visitor.leave_node(AstType::BorderWidth);
}
pub fn walk_border_block_color<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BorderBlockColor<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderBlockColor);
    visitor.visit_css_color((&mut node.end).as_mut());
    visitor.visit_css_color((&mut node.start).as_mut());
    visitor.leave_node(AstType::BorderBlockColor);
}
pub fn walk_border_block_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderBlockStyle)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderBlockStyle);
    visitor.visit_line_style(&mut node.end);
    visitor.visit_line_style(&mut node.start);
    visitor.leave_node(AstType::BorderBlockStyle);
}
pub fn walk_border_block_width<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BorderBlockWidth<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderBlockWidth);
    visitor.visit_border_side_width((&mut node.end).as_mut());
    visitor.visit_border_side_width((&mut node.start).as_mut());
    visitor.leave_node(AstType::BorderBlockWidth);
}
pub fn walk_border_inline_color<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BorderInlineColor<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderInlineColor);
    visitor.visit_css_color((&mut node.end).as_mut());
    visitor.visit_css_color((&mut node.start).as_mut());
    visitor.leave_node(AstType::BorderInlineColor);
}
pub fn walk_border_inline_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut BorderInlineStyle)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderInlineStyle);
    visitor.visit_line_style(&mut node.end);
    visitor.visit_line_style(&mut node.start);
    visitor.leave_node(AstType::BorderInlineStyle);
}
pub fn walk_border_inline_width<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut BorderInlineWidth<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::BorderInlineWidth);
    visitor.visit_border_side_width((&mut node.end).as_mut());
    visitor.visit_border_side_width((&mut node.start).as_mut());
    visitor.leave_node(AstType::BorderInlineWidth);
}
pub fn walk_generic_border<'a, S, VisitorT>(visitor: &mut VisitorT, node: &mut GenericBorder<'a, S>)
where
    VisitorT: ?Sized + VisitMut<'a>,
    S: VisitMutNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::GenericBorder);
    visitor.visit_css_color((&mut node.color).as_mut());
    VisitMutNode::visit_node(&mut node.style, visitor);
    visitor.visit_border_side_width((&mut node.width).as_mut());
    visitor.leave_node(AstType::GenericBorder);
}
pub fn walk_flex_flow<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FlexFlow)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FlexFlow);
    visitor.visit_flex_direction(&mut node.direction);
    visitor.visit_flex_wrap(&mut node.wrap);
    visitor.leave_node(AstType::FlexFlow);
}
pub fn walk_flex<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Flex<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Flex);
    visitor.visit_length_percentage_or_auto((&mut node.basis).as_mut());
    visitor.leave_node(AstType::Flex);
}
pub fn walk_place_content<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PlaceContent<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PlaceContent);
    visitor.visit_align_content((&mut node.align).as_mut());
    visitor.visit_justify_content((&mut node.justify).as_mut());
    visitor.leave_node(AstType::PlaceContent);
}
pub fn walk_place_self<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PlaceSelf<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PlaceSelf);
    visitor.visit_align_self((&mut node.align).as_mut());
    visitor.visit_justify_self((&mut node.justify).as_mut());
    visitor.leave_node(AstType::PlaceSelf);
}
pub fn walk_place_items<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PlaceItems<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PlaceItems);
    visitor.visit_align_items((&mut node.align).as_mut());
    visitor.visit_justify_items((&mut node.justify).as_mut());
    visitor.leave_node(AstType::PlaceItems);
}
pub fn walk_gap<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Gap<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Gap);
    visitor.visit_gap_value((&mut node.column).as_mut());
    visitor.visit_gap_value((&mut node.row).as_mut());
    visitor.leave_node(AstType::Gap);
}
pub fn walk_track_repeat<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TrackRepeat<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TrackRepeat);
    visitor.visit_repeat_count((&mut node.count).as_mut());
    for value_1 in (&mut node.line_names).iter_mut() {
        for value_2 in (value_1).iter_mut() {
            visitor.visit_str(value_2);
        }
    }
    for value_3 in (&mut node.track_sizes).iter_mut() {
        visitor.visit_track_size(value_3);
    }
    visitor.leave_node(AstType::TrackRepeat);
}
pub fn walk_grid_auto_flow<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GridAutoFlow)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GridAutoFlow);
    visitor.visit_auto_flow_direction(&mut node.direction);
    visitor.leave_node(AstType::GridAutoFlow);
}
pub fn walk_grid_template<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GridTemplate<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GridTemplate);
    visitor.visit_grid_template_areas((&mut node.areas).as_mut());
    visitor.visit_track_sizing((&mut node.columns).as_mut());
    visitor.visit_track_sizing((&mut node.rows).as_mut());
    visitor.leave_node(AstType::GridTemplate);
}
pub fn walk_grid<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Grid<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Grid);
    visitor.visit_grid_template_areas((&mut node.areas).as_mut());
    for value_1 in (&mut node.auto_columns).iter_mut() {
        visitor.visit_track_size(value_1);
    }
    visitor.visit_grid_auto_flow((&mut node.auto_flow).as_mut());
    for value_3 in (&mut node.auto_rows).iter_mut() {
        visitor.visit_track_size(value_3);
    }
    visitor.visit_track_sizing((&mut node.columns).as_mut());
    visitor.visit_track_sizing((&mut node.rows).as_mut());
    visitor.leave_node(AstType::Grid);
}
pub fn walk_grid_row<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GridRow<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GridRow);
    visitor.visit_grid_line((&mut node.end).as_mut());
    visitor.visit_grid_line((&mut node.start).as_mut());
    visitor.leave_node(AstType::GridRow);
}
pub fn walk_grid_column<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GridColumn<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GridColumn);
    visitor.visit_grid_line((&mut node.end).as_mut());
    visitor.visit_grid_line((&mut node.start).as_mut());
    visitor.leave_node(AstType::GridColumn);
}
pub fn walk_grid_area<'a, VisitorT>(visitor: &mut VisitorT, node: &mut GridArea<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::GridArea);
    visitor.visit_grid_line((&mut node.column_end).as_mut());
    visitor.visit_grid_line((&mut node.column_start).as_mut());
    visitor.visit_grid_line((&mut node.row_end).as_mut());
    visitor.visit_grid_line((&mut node.row_start).as_mut());
    visitor.leave_node(AstType::GridArea);
}
pub fn walk_margin_block<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MarginBlock<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MarginBlock);
    visitor.visit_length_percentage_or_auto((&mut node.block_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.block_start).as_mut());
    visitor.leave_node(AstType::MarginBlock);
}
pub fn walk_margin_inline<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MarginInline<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MarginInline);
    visitor.visit_length_percentage_or_auto((&mut node.inline_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.inline_start).as_mut());
    visitor.leave_node(AstType::MarginInline);
}
pub fn walk_margin<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Margin<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Margin);
    visitor.visit_length_percentage_or_auto((&mut node.bottom).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.left).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.right).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.top).as_mut());
    visitor.leave_node(AstType::Margin);
}
pub fn walk_padding_block<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PaddingBlock<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PaddingBlock);
    visitor.visit_length_percentage_or_auto((&mut node.block_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.block_start).as_mut());
    visitor.leave_node(AstType::PaddingBlock);
}
pub fn walk_padding_inline<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PaddingInline<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PaddingInline);
    visitor.visit_length_percentage_or_auto((&mut node.inline_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.inline_start).as_mut());
    visitor.leave_node(AstType::PaddingInline);
}
pub fn walk_padding<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Padding<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Padding);
    visitor.visit_length_percentage_or_auto((&mut node.bottom).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.left).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.right).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.top).as_mut());
    visitor.leave_node(AstType::Padding);
}
pub fn walk_scroll_margin_block<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ScrollMarginBlock<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollMarginBlock);
    visitor.visit_length_percentage_or_auto((&mut node.block_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.block_start).as_mut());
    visitor.leave_node(AstType::ScrollMarginBlock);
}
pub fn walk_scroll_margin_inline<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ScrollMarginInline<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollMarginInline);
    visitor.visit_length_percentage_or_auto((&mut node.inline_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.inline_start).as_mut());
    visitor.leave_node(AstType::ScrollMarginInline);
}
pub fn walk_scroll_margin<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ScrollMargin<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollMargin);
    visitor.visit_length_percentage_or_auto((&mut node.bottom).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.left).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.right).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.top).as_mut());
    visitor.leave_node(AstType::ScrollMargin);
}
pub fn walk_scroll_padding_block<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ScrollPaddingBlock<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollPaddingBlock);
    visitor.visit_length_percentage_or_auto((&mut node.block_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.block_start).as_mut());
    visitor.leave_node(AstType::ScrollPaddingBlock);
}
pub fn walk_scroll_padding_inline<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ScrollPaddingInline<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollPaddingInline);
    visitor.visit_length_percentage_or_auto((&mut node.inline_end).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.inline_start).as_mut());
    visitor.leave_node(AstType::ScrollPaddingInline);
}
pub fn walk_scroll_padding<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ScrollPadding<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollPadding);
    visitor.visit_length_percentage_or_auto((&mut node.bottom).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.left).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.right).as_mut());
    visitor.visit_length_percentage_or_auto((&mut node.top).as_mut());
    visitor.leave_node(AstType::ScrollPadding);
}
pub fn walk_font<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Font<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Font);
    for value_0 in (&mut node.family).iter_mut() {
        visitor.visit_font_family(value_0);
    }
    visitor.visit_line_height((&mut node.line_height).as_mut());
    visitor.visit_font_size((&mut node.size).as_mut());
    visitor.visit_font_stretch((&mut node.stretch).as_mut());
    visitor.visit_font_style((&mut node.style).as_mut());
    visitor.visit_font_variant_caps(&mut node.variant_caps);
    visitor.visit_font_weight((&mut node.weight).as_mut());
    visitor.leave_node(AstType::Font);
}
pub fn walk_transition<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Transition<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Transition);
    visitor.visit_time((&mut node.delay).as_mut());
    visitor.visit_time((&mut node.duration).as_mut());
    visitor.visit_property_id((&mut node.property).as_mut());
    visitor.visit_easing_function((&mut node.timing_function).as_mut());
    visitor.leave_node(AstType::Transition);
}
pub fn walk_scroll_timeline<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ScrollTimeline)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollTimeline);
    visitor.visit_scroll_axis(&mut node.axis);
    visitor.visit_scroller(&mut node.scroller);
    visitor.leave_node(AstType::ScrollTimeline);
}
pub fn walk_view_timeline<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ViewTimeline<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ViewTimeline);
    visitor.visit_scroll_axis(&mut node.axis);
    visitor.visit_size_2_d((&mut node.inset).as_mut());
    visitor.leave_node(AstType::ViewTimeline);
}
pub fn walk_animation_range<'a, VisitorT>(visitor: &mut VisitorT, node: &mut AnimationRange<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::AnimationRange);
    visitor.visit_animation_range_end((&mut node.end).as_mut());
    visitor.visit_animation_range_start((&mut node.start).as_mut());
    visitor.leave_node(AstType::AnimationRange);
}
pub fn walk_animation<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Animation<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Animation);
    visitor.visit_time((&mut node.delay).as_mut());
    visitor.visit_animation_direction(&mut node.direction);
    visitor.visit_time((&mut node.duration).as_mut());
    visitor.visit_animation_fill_mode(&mut node.fill_mode);
    visitor.visit_animation_iteration_count((&mut node.iteration_count).as_mut());
    visitor.visit_animation_name((&mut node.name).as_mut());
    visitor.visit_animation_play_state(&mut node.play_state);
    visitor.visit_animation_timeline((&mut node.timeline).as_mut());
    visitor.visit_easing_function((&mut node.timing_function).as_mut());
    visitor.leave_node(AstType::Animation);
}
pub fn walk_matrix_for_float<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MatrixForFloat)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MatrixForFloat);
    visitor.leave_node(AstType::MatrixForFloat);
}
pub fn walk_matrix_3_d_for_float<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Matrix3DForFloat)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Matrix3DForFloat);
    visitor.leave_node(AstType::Matrix3DForFloat);
}
pub fn walk_rotate<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Rotate<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Rotate);
    visitor.visit_angle((&mut node.angle).as_mut());
    visitor.leave_node(AstType::Rotate);
}
pub fn walk_text_transform<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextTransform)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextTransform);
    visitor.visit_text_transform_case(&mut node.case);
    visitor.leave_node(AstType::TextTransform);
}
pub fn walk_text_indent<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextIndent<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextIndent);
    visitor.visit_length_percentage((&mut node.value).as_mut());
    visitor.leave_node(AstType::TextIndent);
}
pub fn walk_text_decoration<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextDecoration<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextDecoration);
    visitor.visit_css_color((&mut node.color).as_mut());
    visitor.visit_text_decoration_line((&mut node.line).as_mut());
    visitor.visit_text_decoration_style(&mut node.style);
    visitor.visit_text_decoration_thickness((&mut node.thickness).as_mut());
    visitor.leave_node(AstType::TextDecoration);
}
pub fn walk_text_emphasis<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextEmphasis<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextEmphasis);
    visitor.visit_css_color((&mut node.color).as_mut());
    visitor.visit_text_emphasis_style((&mut node.style).as_mut());
    visitor.leave_node(AstType::TextEmphasis);
}
pub fn walk_text_emphasis_position<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TextEmphasisPosition,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextEmphasisPosition);
    visitor.visit_text_emphasis_position_horizontal(&mut node.horizontal);
    visitor.visit_text_emphasis_position_vertical(&mut node.vertical);
    visitor.leave_node(AstType::TextEmphasisPosition);
}
pub fn walk_text_shadow<'a, VisitorT>(visitor: &mut VisitorT, node: &mut TextShadow<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TextShadow);
    visitor.visit_length((&mut node.blur).as_mut());
    visitor.visit_css_color((&mut node.color).as_mut());
    visitor.visit_length((&mut node.spread).as_mut());
    visitor.visit_length((&mut node.x_offset).as_mut());
    visitor.visit_length((&mut node.y_offset).as_mut());
    visitor.leave_node(AstType::TextShadow);
}
pub fn walk_cursor<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Cursor<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Cursor);
    for value_0 in (&mut node.images).iter_mut() {
        visitor.visit_cursor_image(value_0);
    }
    visitor.visit_cursor_keyword(&mut node.keyword);
    visitor.leave_node(AstType::Cursor);
}
pub fn walk_cursor_image<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CursorImage<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CursorImage);
    if let Some(value_0) = (&mut node.hotspot).as_mut() {}
    visitor.visit_url((&mut node.url).as_mut());
    visitor.leave_node(AstType::CursorImage);
}
pub fn walk_caret<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Caret<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Caret);
    visitor.visit_color_or_auto((&mut node.color).as_mut());
    visitor.visit_caret_shape(&mut node.shape);
    visitor.leave_node(AstType::Caret);
}
pub fn walk_list_style<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ListStyle<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ListStyle);
    visitor.visit_image((&mut node.image).as_mut());
    visitor.visit_list_style_type((&mut node.list_style_type).as_mut());
    visitor.visit_list_style_position(&mut node.position);
    visitor.leave_node(AstType::ListStyle);
}
pub fn walk_composes<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Composes<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Composes);
    if let Some(value_0) = (&mut node.from).as_mut() {
        visitor.visit_specifier((value_0).as_mut());
    }
    visitor.visit_span(&mut node.span);
    for value_2 in (&mut node.names).iter_mut() {
        visitor.visit_str(value_2);
    }
    visitor.leave_node(AstType::Composes);
}
pub fn walk_inset_rect<'a, VisitorT>(visitor: &mut VisitorT, node: &mut InsetRect<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::InsetRect);
    visitor.visit_border_radius((&mut node.radius).as_mut());
    visitor.visit_rect((&mut node.rect).as_mut());
    visitor.leave_node(AstType::InsetRect);
}
pub fn walk_circle_shape<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CircleShape<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CircleShape);
    visitor.visit_position((&mut node.position).as_mut());
    visitor.visit_shape_radius((&mut node.radius).as_mut());
    visitor.leave_node(AstType::CircleShape);
}
pub fn walk_ellipse_shape<'a, VisitorT>(visitor: &mut VisitorT, node: &mut EllipseShape<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::EllipseShape);
    visitor.visit_position((&mut node.position).as_mut());
    visitor.visit_shape_radius((&mut node.radius_x).as_mut());
    visitor.visit_shape_radius((&mut node.radius_y).as_mut());
    visitor.leave_node(AstType::EllipseShape);
}
pub fn walk_polygon<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Polygon<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Polygon);
    visitor.visit_fill_rule(&mut node.fill_rule);
    for value_0 in (&mut node.points).iter_mut() {
        visitor.visit_point(value_0);
    }
    visitor.leave_node(AstType::Polygon);
}
pub fn walk_point<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Point<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Point);
    visitor.visit_length_percentage((&mut node.x).as_mut());
    visitor.visit_length_percentage((&mut node.y).as_mut());
    visitor.leave_node(AstType::Point);
}
pub fn walk_mask<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Mask<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Mask);
    visitor.visit_mask_clip((&mut node.clip).as_mut());
    visitor.visit_mask_composite(&mut node.composite);
    visitor.visit_image((&mut node.image).as_mut());
    visitor.visit_mask_mode(&mut node.mode);
    visitor.visit_geometry_box(&mut node.origin);
    visitor.visit_position((&mut node.position).as_mut());
    visitor.visit_background_repeat((&mut node.repeat).as_mut());
    visitor.visit_background_size((&mut node.size).as_mut());
    visitor.leave_node(AstType::Mask);
}
pub fn walk_mask_border<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MaskBorder<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MaskBorder);
    visitor.visit_mask_border_mode(&mut node.mode);
    visitor.visit_rect((&mut node.outset).as_mut());
    visitor.visit_border_image_repeat((&mut node.repeat).as_mut());
    visitor.visit_border_image_slice((&mut node.slice).as_mut());
    visitor.visit_image((&mut node.source).as_mut());
    visitor.visit_rect((&mut node.width).as_mut());
    visitor.leave_node(AstType::MaskBorder);
}
pub fn walk_drop_shadow<'a, VisitorT>(visitor: &mut VisitorT, node: &mut DropShadow<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::DropShadow);
    visitor.visit_length((&mut node.blur).as_mut());
    visitor.visit_css_color((&mut node.color).as_mut());
    visitor.visit_length((&mut node.x_offset).as_mut());
    visitor.visit_length((&mut node.y_offset).as_mut());
    visitor.leave_node(AstType::DropShadow);
}
pub fn walk_container<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Container<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Container);
    visitor.visit_container_type(&mut node.container_type);
    visitor.visit_container_name_list((&mut node.name).as_mut());
    visitor.leave_node(AstType::Container);
}
pub fn walk_color_scheme<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ColorScheme)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ColorScheme);
    visitor.leave_node(AstType::ColorScheme);
}
pub fn walk_unparsed_property<'a, VisitorT>(visitor: &mut VisitorT, node: &mut UnparsedProperty<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::UnparsedProperty);
    visitor.visit_property_id((&mut node.property_id).as_mut());
    for value_1 in (&mut node.value).iter_mut() {
        visitor.visit_token_or_value(value_1);
    }
    visitor.leave_node(AstType::UnparsedProperty);
}
pub fn walk_custom_property<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CustomProperty<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CustomProperty);
    visitor.visit_custom_property_name((&mut node.name).as_mut());
    for value_1 in (&mut node.value).iter_mut() {
        visitor.visit_token_or_value(value_1);
    }
    visitor.leave_node(AstType::CustomProperty);
}
pub fn walk_view_transition_part_selector<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ViewTransitionPartSelector<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ViewTransitionPartSelector);
    for value_0 in (&mut node.classes).iter_mut() {
        visitor.visit_str(value_0);
    }
    if let Some(value_1) = (&mut node.name).as_mut() {
        visitor.visit_view_transition_part_name((value_1).as_mut());
    }
    visitor.leave_node(AstType::ViewTransitionPartSelector);
}
pub fn walk_keyframes_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut KeyframesRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::KeyframesRule);
    for value_0 in (&mut node.keyframes).iter_mut() {
        visitor.visit_keyframe(value_0);
    }
    visitor.visit_span(&mut node.span);
    visitor.visit_keyframes_name((&mut node.name).as_mut());
    visitor.visit_vendor_prefix(&mut node.vendor_prefix);
    visitor.leave_node(AstType::KeyframesRule);
}
pub fn walk_keyframe<'a, VisitorT>(visitor: &mut VisitorT, node: &mut Keyframe<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::Keyframe);
    visitor.visit_declaration_block((&mut node.declarations).as_mut());
    for value_1 in (&mut node.selectors).iter_mut() {
        visitor.visit_keyframe_selector(value_1);
    }
    visitor.leave_node(AstType::Keyframe);
}
pub fn walk_timeline_range_percentage<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut TimelineRangePercentage,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::TimelineRangePercentage);
    visitor.visit_timeline_range_name(&mut node.name);
    visitor.leave_node(AstType::TimelineRangePercentage);
}
pub fn walk_font_face_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FontFaceRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFaceRule);
    visitor.visit_span(&mut node.span);
    for value_0 in (&mut node.properties).iter_mut() {
        visitor.visit_font_face_property(value_0);
    }
    visitor.leave_node(AstType::FontFaceRule);
}
pub fn walk_url_source<'a, VisitorT>(visitor: &mut VisitorT, node: &mut UrlSource<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::UrlSource);
    if let Some(value_0) = (&mut node.format).as_mut() {
        visitor.visit_font_format((value_0).as_mut());
    }
    for value_2 in (&mut node.tech).iter_mut() {
        visitor.visit_font_technology(value_2);
    }
    visitor.visit_url((&mut node.url).as_mut());
    visitor.leave_node(AstType::UrlSource);
}
pub fn walk_unicode_range<'a, VisitorT>(visitor: &mut VisitorT, node: &mut UnicodeRange)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::UnicodeRange);
    visitor.leave_node(AstType::UnicodeRange);
}
pub fn walk_font_palette_values_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut FontPaletteValuesRule<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontPaletteValuesRule);
    visitor.visit_span(&mut node.span);
    visitor.visit_str(&mut node.name);
    for value_0 in (&mut node.properties).iter_mut() {
        visitor.visit_font_palette_values_property(value_0);
    }
    visitor.leave_node(AstType::FontPaletteValuesRule);
}
pub fn walk_override_colors<'a, VisitorT>(visitor: &mut VisitorT, node: &mut OverrideColors<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::OverrideColors);
    visitor.visit_css_color((&mut node.color).as_mut());
    visitor.leave_node(AstType::OverrideColors);
}
pub fn walk_font_feature_values_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut FontFeatureValuesRule<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFeatureValuesRule);
    visitor.visit_span(&mut node.span);
    for value_0 in (&mut node.name).iter_mut() {
        visitor.visit_family_name(value_0);
    }
    for value_1 in (&mut node.rules).iter_mut() {
        visitor.visit_font_feature_subrule(value_1);
    }
    visitor.leave_node(AstType::FontFeatureValuesRule);
}
pub fn walk_font_feature_subrule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut FontFeatureSubrule<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFeatureSubrule);
    for value_0 in (&mut node.declarations).iter_mut() {
        visitor.visit_font_feature_declaration(value_0);
    }
    visitor.visit_span(&mut node.span);
    visitor.visit_font_feature_subrule_type(&mut node.name);
    visitor.leave_node(AstType::FontFeatureSubrule);
}
pub fn walk_font_feature_declaration<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut FontFeatureDeclaration<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FontFeatureDeclaration);
    visitor.visit_str(&mut node.name);
    for value_0 in (&mut node.values).iter_mut() {}
    visitor.leave_node(AstType::FontFeatureDeclaration);
}
pub fn walk_family_name<'a, VisitorT>(visitor: &mut VisitorT, node: &mut FamilyName<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::FamilyName);
    visitor.visit_str(&mut node.0);
    visitor.leave_node(AstType::FamilyName);
}
pub fn walk_page_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PageRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PageRule);
    visitor.visit_declaration_block((&mut node.declarations).as_mut());
    visitor.visit_span(&mut node.span);
    for value_1 in (&mut node.rules).iter_mut() {
        visitor.visit_page_margin_rule(value_1);
    }
    for value_2 in (&mut node.selectors).iter_mut() {
        visitor.visit_page_selector(value_2);
    }
    visitor.leave_node(AstType::PageRule);
}
pub fn walk_page_margin_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PageMarginRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PageMarginRule);
    visitor.visit_declaration_block((&mut node.declarations).as_mut());
    visitor.visit_span(&mut node.span);
    visitor.visit_page_margin_box(&mut node.margin_box);
    visitor.leave_node(AstType::PageMarginRule);
}
pub fn walk_page_selector<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PageSelector<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PageSelector);
    if let Some(value_0) = (&mut node.name).as_mut() {
        visitor.visit_str(value_0);
    }
    for value_1 in (&mut node.pseudo_classes).iter_mut() {
        visitor.visit_page_pseudo_class(value_1);
    }
    visitor.leave_node(AstType::PageSelector);
}
pub fn walk_supports_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut SupportsRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SupportsRule);
    visitor.visit_supports_condition((&mut node.condition).as_mut());
    visitor.visit_span(&mut node.span);
    for value_1 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_1);
    }
    visitor.leave_node(AstType::SupportsRule);
}
pub fn walk_counter_style_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut CounterStyleRule<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CounterStyleRule);
    visitor.visit_declaration_block((&mut node.declarations).as_mut());
    visitor.visit_span(&mut node.span);
    visitor.visit_str(&mut node.name);
    visitor.leave_node(AstType::CounterStyleRule);
}
pub fn walk_namespace_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut NamespaceRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::NamespaceRule);
    visitor.visit_span(&mut node.span);
    if let Some(value_0) = (&mut node.prefix).as_mut() {
        visitor.visit_str(value_0);
    }
    visitor.visit_str(&mut node.url);
    visitor.leave_node(AstType::NamespaceRule);
}
pub fn walk_moz_document_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut MozDocumentRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::MozDocumentRule);
    visitor.visit_span(&mut node.span);
    for value_0 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_0);
    }
    visitor.leave_node(AstType::MozDocumentRule);
}
pub fn walk_nesting_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut NestingRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::NestingRule);
    visitor.visit_span(&mut node.span);
    visitor.visit_style_rule((&mut node.style).as_mut());
    visitor.leave_node(AstType::NestingRule);
}
pub fn walk_nested_declarations_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut NestedDeclarationsRule<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::NestedDeclarationsRule);
    visitor.visit_declaration_block((&mut node.declarations).as_mut());
    visitor.visit_span(&mut node.span);
    visitor.leave_node(AstType::NestedDeclarationsRule);
}
pub fn walk_viewport_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ViewportRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ViewportRule);
    visitor.visit_declaration_block((&mut node.declarations).as_mut());
    visitor.visit_span(&mut node.span);
    visitor.visit_vendor_prefix(&mut node.vendor_prefix);
    visitor.leave_node(AstType::ViewportRule);
}
pub fn walk_custom_media_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut CustomMediaRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::CustomMediaRule);
    visitor.visit_span(&mut node.span);
    visitor.visit_str(&mut node.name);
    visitor.visit_media_list((&mut node.query).as_mut());
    visitor.leave_node(AstType::CustomMediaRule);
}
pub fn walk_layer_statement_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut LayerStatementRule<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LayerStatementRule);
    visitor.visit_span(&mut node.span);
    for value_0 in (&mut node.names).iter_mut() {
        for value_1 in (value_0).iter_mut() {
            visitor.visit_str(value_1);
        }
    }
    visitor.leave_node(AstType::LayerStatementRule);
}
pub fn walk_layer_block_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut LayerBlockRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::LayerBlockRule);
    visitor.visit_span(&mut node.span);
    if let Some(value_0) = (&mut node.name).as_mut() {
        for value_1 in (value_0).iter_mut() {
            visitor.visit_str(value_1);
        }
    }
    for value_2 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_2);
    }
    visitor.leave_node(AstType::LayerBlockRule);
}
pub fn walk_property_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PropertyRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PropertyRule);
    if let Some(value_0) = (&mut node.initial_value).as_mut() {
        visitor.visit_parsed_component((value_0).as_mut());
    }
    visitor.visit_span(&mut node.span);
    visitor.visit_str(&mut node.name);
    visitor.visit_syntax_string((&mut node.syntax).as_mut());
    visitor.leave_node(AstType::PropertyRule);
}
pub fn walk_syntax_component<'a, VisitorT>(visitor: &mut VisitorT, node: &mut SyntaxComponent<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::SyntaxComponent);
    visitor.visit_syntax_component_kind((&mut node.kind).as_mut());
    visitor.visit_multiplier(&mut node.multiplier);
    visitor.leave_node(AstType::SyntaxComponent);
}
pub fn walk_container_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ContainerRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ContainerRule);
    if let Some(value_0) = (&mut node.condition).as_mut() {
        visitor.visit_container_condition((value_0).as_mut());
    }
    visitor.visit_span(&mut node.span);
    if let Some(value_2) = (&mut node.name).as_mut() {
        visitor.visit_str(value_2);
    }
    for value_3 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_3);
    }
    visitor.leave_node(AstType::ContainerRule);
}
pub fn walk_scope_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut ScopeRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScopeRule);
    visitor.visit_span(&mut node.span);
    for value_0 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_0);
    }
    if let Some(value_1) = (&mut node.scope_end).as_mut() {
        visitor.visit_selector_list((value_1).as_mut());
    }
    if let Some(value_3) = (&mut node.scope_start).as_mut() {
        visitor.visit_selector_list((value_3).as_mut());
    }
    visitor.leave_node(AstType::ScopeRule);
}
pub fn walk_starting_style_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut StartingStyleRule<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::StartingStyleRule);
    visitor.visit_span(&mut node.span);
    for value_0 in (&mut node.rules).iter_mut() {
        visitor.visit_css_rule(value_0);
    }
    visitor.leave_node(AstType::StartingStyleRule);
}
pub fn walk_view_transition_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ViewTransitionRule<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ViewTransitionRule);
    visitor.visit_span(&mut node.span);
    for value_0 in (&mut node.properties).iter_mut() {
        visitor.visit_view_transition_property(value_0);
    }
    visitor.leave_node(AstType::ViewTransitionRule);
}
pub fn walk_position_try_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut PositionTryRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::PositionTryRule);
    visitor.visit_span(&mut node.span);
    visitor.visit_str(&mut node.name);
    visitor.visit_declaration_block((&mut node.declarations).as_mut());
    visitor.leave_node(AstType::PositionTryRule);
}
pub fn walk_unknown_at_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &mut UnknownAtRule<'a>)
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::UnknownAtRule);
    if let Some(value_0) = (&mut node.block).as_mut() {
        for value_1 in (value_0).iter_mut() {
            visitor.visit_token_or_value(value_1);
        }
    }
    visitor.visit_span(&mut node.span);
    visitor.visit_str(&mut node.name);
    for value_2 in (&mut node.prelude).iter_mut() {
        visitor.visit_token_or_value(value_2);
    }
    visitor.leave_node(AstType::UnknownAtRule);
}
pub fn walk_container_size_feature<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ContainerSizeFeature<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ContainerSizeFeature);
    visitor.visit_query_feature(node);
    visitor.leave_node(AstType::ContainerSizeFeature);
}
pub fn walk_scroll_state_feature<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &mut ScrollStateFeature<'a>,
) where
    VisitorT: ?Sized + VisitMut<'a>,
{
    visitor.enter_node(AstType::ScrollStateFeature);
    visitor.visit_query_feature(node);
    visitor.leave_node(AstType::ScrollStateFeature);
}
