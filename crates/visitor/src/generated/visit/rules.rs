#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use super::{Visit, VisitNode};
use crate::AstType;
use rocketcss_ast::*;
pub fn walk_keyframe_selector<'a, VisitorT>(visitor: &mut VisitorT, node: &KeyframeSelector<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::KeyframeSelector);
    match node {
        KeyframeSelector::Percentage(field_0) => {}
        KeyframeSelector::From => {}
        KeyframeSelector::To => {}
        KeyframeSelector::TimelineRangePercentage(field_0) => {
            visitor.visit_timeline_range_percentage((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::KeyframeSelector);
}
pub fn walk_keyframes_name<'a, VisitorT>(visitor: &mut VisitorT, node: &KeyframesName<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
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
pub fn walk_font_face_property<'a, VisitorT>(visitor: &mut VisitorT, node: &FontFaceProperty<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FontFaceProperty);
    match node {
        FontFaceProperty::Source(field_0) => {
            for value_0 in (field_0).iter() {
                visitor.visit_source(value_0);
            }
        }
        FontFaceProperty::FontFamily(field_0) => {
            visitor.visit_font_family((field_0).as_ref());
        }
        FontFaceProperty::FontStyle(field_0) => {
            visitor.visit_font_face_style((field_0).as_ref());
        }
        FontFaceProperty::FontWeight(field_0) => {
            visitor.visit_size_2_d((field_0).as_ref());
        }
        FontFaceProperty::FontStretch(field_0) => {
            visitor.visit_size_2_d((field_0).as_ref());
        }
        FontFaceProperty::UnicodeRange(field_0) => {
            for value_5 in (field_0).iter() {
                visitor.visit_unicode_range(value_5);
            }
        }
        FontFaceProperty::Custom(field_0) => {
            visitor.visit_custom_property((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::FontFaceProperty);
}
pub fn walk_source<'a, VisitorT>(visitor: &mut VisitorT, node: &Source<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Source);
    match node {
        Source::Url(field_0) => {
            visitor.visit_url_source((field_0).as_ref());
        }
        Source::Local(field_0) => {
            visitor.visit_font_family((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::Source);
}
pub fn walk_font_format<'a, VisitorT>(visitor: &mut VisitorT, node: &FontFormat<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
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
pub fn walk_font_technology<'a, VisitorT>(visitor: &mut VisitorT, node: &FontTechnology)
where
    VisitorT: ?Sized + Visit<'a>,
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
pub fn walk_font_face_style<'a, VisitorT>(visitor: &mut VisitorT, node: &FontFaceStyle<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FontFaceStyle);
    match node {
        FontFaceStyle::Normal => {}
        FontFaceStyle::Italic => {}
        FontFaceStyle::Oblique(field_0) => {
            visitor.visit_size_2_d((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::FontFaceStyle);
}
pub fn walk_font_palette_values_property<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &FontPaletteValuesProperty<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FontPaletteValuesProperty);
    match node {
        FontPaletteValuesProperty::FontFamily(field_0) => {
            visitor.visit_font_family((field_0).as_ref());
        }
        FontPaletteValuesProperty::BasePalette(field_0) => {
            visitor.visit_base_palette((field_0).as_ref());
        }
        FontPaletteValuesProperty::OverrideColors(field_0) => {
            for value_2 in (field_0).iter() {
                visitor.visit_override_colors(value_2);
            }
        }
        FontPaletteValuesProperty::Custom(field_0) => {
            visitor.visit_custom_property((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::FontPaletteValuesProperty);
}
pub fn walk_base_palette<'a, VisitorT>(visitor: &mut VisitorT, node: &BasePalette)
where
    VisitorT: ?Sized + Visit<'a>,
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
    node: &FontFeatureSubruleType,
) where
    VisitorT: ?Sized + Visit<'a>,
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
pub fn walk_page_margin_box<'a, VisitorT>(visitor: &mut VisitorT, node: &PageMarginBox)
where
    VisitorT: ?Sized + Visit<'a>,
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
pub fn walk_page_pseudo_class<'a, VisitorT>(visitor: &mut VisitorT, node: &PagePseudoClass)
where
    VisitorT: ?Sized + Visit<'a>,
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
pub fn walk_parsed_component<'a, VisitorT>(visitor: &mut VisitorT, node: &ParsedComponent<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ParsedComponent);
    match node {
        ParsedComponent::Length(field_0) => {
            visitor.visit_length((field_0).as_ref());
        }
        ParsedComponent::Number(field_0) => {}
        ParsedComponent::Percentage(field_0) => {}
        ParsedComponent::LengthPercentage(field_0) => {
            visitor.visit_length_percentage((field_0).as_ref());
        }
        ParsedComponent::String(field_0) => {
            visitor.visit_str(field_0);
        }
        ParsedComponent::Color(field_0) => {
            visitor.visit_css_color((field_0).as_ref());
        }
        ParsedComponent::Image(field_0) => {
            visitor.visit_image((field_0).as_ref());
        }
        ParsedComponent::Url(field_0) => {
            visitor.visit_url((field_0).as_ref());
        }
        ParsedComponent::Integer(field_0) => {}
        ParsedComponent::Angle(field_0) => {
            visitor.visit_angle((field_0).as_ref());
        }
        ParsedComponent::Time(field_0) => {
            visitor.visit_time((field_0).as_ref());
        }
        ParsedComponent::Resolution(field_0) => {
            visitor.visit_resolution((field_0).as_ref());
        }
        ParsedComponent::TransformFunction(field_0) => {
            visitor.visit_transform((field_0).as_ref());
        }
        ParsedComponent::TransformList(field_0) => {
            for value_9 in (field_0).iter() {
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
            for value_10 in (components).iter() {
                visitor.visit_parsed_component(value_10);
            }
            visitor.visit_multiplier(multiplier);
        }
        ParsedComponent::TokenList(field_0) => {
            for value_11 in (field_0).iter() {
                visitor.visit_token_or_value(value_11);
            }
        }
    }
    visitor.leave_node(AstType::ParsedComponent);
}
pub fn walk_multiplier<'a, VisitorT>(visitor: &mut VisitorT, node: &Multiplier)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Multiplier);
    match node {
        Multiplier::None => {}
        Multiplier::Space => {}
        Multiplier::Comma => {}
    }
    visitor.leave_node(AstType::Multiplier);
}
pub fn walk_syntax_string<'a, VisitorT>(visitor: &mut VisitorT, node: &SyntaxString<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::SyntaxString);
    match node {
        SyntaxString::Components(field_0) => {
            for value_0 in (field_0).iter() {
                visitor.visit_syntax_component(value_0);
            }
        }
        SyntaxString::Universal => {}
    }
    visitor.leave_node(AstType::SyntaxString);
}
pub fn walk_syntax_component_kind<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &SyntaxComponentKind<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
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
pub fn walk_container_condition<'a, VisitorT>(visitor: &mut VisitorT, node: &ContainerCondition<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ContainerCondition);
    match node {
        ContainerCondition::Feature(field_0) => {
            visitor.visit_container_size_feature((field_0).as_ref());
        }
        ContainerCondition::Not(field_0) => {
            visitor.visit_container_condition((field_0).as_ref());
        }
        ContainerCondition::Operation {
            conditions,
            operator,
        } => {
            for value_2 in (conditions).iter() {
                visitor.visit_container_condition(value_2);
            }
            visitor.visit_operator(operator);
        }
        ContainerCondition::Style(field_0) => {
            visitor.visit_style_query((field_0).as_ref());
        }
        ContainerCondition::ScrollState(field_0) => {
            visitor.visit_scroll_state_query((field_0).as_ref());
        }
        ContainerCondition::Unknown(field_0) => {
            for value_5 in (field_0).iter() {
                visitor.visit_token_or_value(value_5);
            }
        }
    }
    visitor.leave_node(AstType::ContainerCondition);
}
pub fn walk_container_size_feature_id<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ContainerSizeFeatureId,
) where
    VisitorT: ?Sized + Visit<'a>,
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
pub fn walk_style_query<'a, VisitorT>(visitor: &mut VisitorT, node: &StyleQuery<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::StyleQuery);
    match node {
        StyleQuery::Declaration(field_0) => {
            visitor.visit_declaration((field_0).as_ref());
        }
        StyleQuery::Property(field_0) => {
            visitor.visit_property_id((field_0).as_ref());
        }
        StyleQuery::Not(field_0) => {
            visitor.visit_style_query((field_0).as_ref());
        }
        StyleQuery::Operation {
            conditions,
            operator,
        } => {
            for value_3 in (conditions).iter() {
                visitor.visit_style_query(value_3);
            }
            visitor.visit_operator(operator);
        }
    }
    visitor.leave_node(AstType::StyleQuery);
}
pub fn walk_scroll_state_query<'a, VisitorT>(visitor: &mut VisitorT, node: &ScrollStateQuery<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollStateQuery);
    match node {
        ScrollStateQuery::Feature(field_0) => {
            visitor.visit_scroll_state_feature((field_0).as_ref());
        }
        ScrollStateQuery::Not(field_0) => {
            visitor.visit_scroll_state_query((field_0).as_ref());
        }
        ScrollStateQuery::Operation {
            conditions,
            operator,
        } => {
            for value_2 in (conditions).iter() {
                visitor.visit_scroll_state_query(value_2);
            }
            visitor.visit_operator(operator);
        }
    }
    visitor.leave_node(AstType::ScrollStateQuery);
}
pub fn walk_scroll_state_feature_id<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ScrollStateFeatureId,
) where
    VisitorT: ?Sized + Visit<'a>,
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
    node: &ViewTransitionProperty<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ViewTransitionProperty);
    match node {
        ViewTransitionProperty::Navigation(field_0) => {
            visitor.visit_navigation(field_0);
        }
        ViewTransitionProperty::Types(field_0) => {
            visitor.visit_none_or_custom_ident_list((field_0).as_ref());
        }
        ViewTransitionProperty::Custom(field_0) => {
            visitor.visit_custom_property((field_0).as_ref());
        }
    }
    visitor.leave_node(AstType::ViewTransitionProperty);
}
pub fn walk_navigation<'a, VisitorT>(visitor: &mut VisitorT, node: &Navigation)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Navigation);
    match node {
        Navigation::None => {}
        Navigation::Auto => {}
    }
    visitor.leave_node(AstType::Navigation);
}
pub fn walk_default_at_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &DefaultAtRule)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::DefaultAtRule);
    visitor.leave_node(AstType::DefaultAtRule);
}
pub fn walk_style_sheet<'a, VisitorT>(visitor: &mut VisitorT, node: &StyleSheet<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::StyleSheet);
    for value_0 in (&node.license_comments).iter() {
        visitor.visit_str(value_0);
    }
    for value_1 in (&node.rules).iter() {
        visitor.visit_css_rule(value_1);
    }
    for value_2 in (&node.source_map_urls).iter() {
        if let Some(value_3) = (value_2).as_ref() {
            visitor.visit_str(value_3);
        }
    }
    for value_4 in (&node.sources).iter() {
        visitor.visit_str(value_4);
    }
    visitor.leave_node(AstType::StyleSheet);
}
pub fn walk_media_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &MediaRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::MediaRule);
    visitor.visit_span(&node.span);
    visitor.visit_media_list(&node.query);
    for value_1 in (&node.rules).iter() {
        visitor.visit_css_rule(value_1);
    }
    visitor.leave_node(AstType::MediaRule);
}
pub fn walk_media_list<'a, VisitorT>(visitor: &mut VisitorT, node: &MediaList<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::MediaList);
    for value_0 in (&node.media_queries).iter() {
        visitor.visit_media_query(value_0);
    }
    visitor.leave_node(AstType::MediaList);
}
pub fn walk_media_query<'a, VisitorT>(visitor: &mut VisitorT, node: &MediaQuery<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::MediaQuery);
    if let Some(value_0) = (&node.condition).as_ref() {
        visitor.visit_media_condition((value_0).as_ref());
    }
    visitor.visit_media_type(&node.media_type);
    if let Some(value_3) = (&node.qualifier).as_ref() {
        visitor.visit_qualifier(value_3);
    }
    visitor.leave_node(AstType::MediaQuery);
}
pub fn walk_length_value<'a, VisitorT>(visitor: &mut VisitorT, node: &LengthValue)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::LengthValue);
    visitor.visit_length_unit(&node.unit);
    visitor.leave_node(AstType::LengthValue);
}
pub fn walk_environment_variable<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &EnvironmentVariable<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::EnvironmentVariable);
    if let Some(value_0) = (&node.fallback).as_ref() {
        for value_1 in (value_0).iter() {
            visitor.visit_token_or_value(value_1);
        }
    }
    for value_2 in (&node.indices).iter() {}
    visitor.visit_environment_variable_name((&node.name).as_ref());
    visitor.leave_node(AstType::EnvironmentVariable);
}
pub fn walk_url<'a, VisitorT>(visitor: &mut VisitorT, node: &Url<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Url);
    visitor.visit_span(&node.span);
    visitor.visit_str(&node.url);
    visitor.leave_node(AstType::Url);
}
pub fn walk_variable<'a, VisitorT>(visitor: &mut VisitorT, node: &Variable<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Variable);
    if let Some(value_0) = (&node.fallback).as_ref() {
        for value_1 in (value_0).iter() {
            visitor.visit_token_or_value(value_1);
        }
    }
    visitor.visit_dashed_ident_reference((&node.name).as_ref());
    visitor.leave_node(AstType::Variable);
}
pub fn walk_dashed_ident_reference<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &DashedIdentReference<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::DashedIdentReference);
    if let Some(value_0) = (&node.from).as_ref() {
        visitor.visit_specifier((value_0).as_ref());
    }
    visitor.visit_str(&node.ident);
    visitor.leave_node(AstType::DashedIdentReference);
}
pub fn walk_function<'a, VisitorT>(visitor: &mut VisitorT, node: &Function<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Function);
    for value_0 in (&node.arguments).iter() {
        visitor.visit_token_or_value(value_0);
    }
    visitor.visit_str(&node.name);
    visitor.leave_node(AstType::Function);
}
pub fn walk_import_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &ImportRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ImportRule);
    if let Some(value_0) = (&node.layer).as_ref() {
        for value_1 in (value_0).iter() {
            visitor.visit_str(value_1);
        }
    }
    visitor.visit_span(&node.span);
    if let Some(value_2) = (&node.media).as_ref() {
        visitor.visit_media_list((value_2).as_ref());
    }
    if let Some(value_4) = (&node.supports).as_ref() {
        visitor.visit_supports_condition((value_4).as_ref());
    }
    visitor.visit_str(&node.url);
    visitor.leave_node(AstType::ImportRule);
}
pub fn walk_style_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &StyleRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::StyleRule);
    visitor.visit_declaration_block((&node.declarations).as_ref());
    visitor.visit_span(&node.span);
    for value_1 in (&node.rules).iter() {
        visitor.visit_css_rule(value_1);
    }
    visitor.visit_selector_list((&node.selectors).as_ref());
    visitor.visit_vendor_prefix(&node.vendor_prefix);
    visitor.leave_node(AstType::StyleRule);
}
pub fn walk_declaration_block<'a, VisitorT>(visitor: &mut VisitorT, node: &DeclarationBlock<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::DeclarationBlock);
    for value_0 in (&node.declarations).iter() {
        visitor.visit_declaration(value_0);
    }
    visitor.leave_node(AstType::DeclarationBlock);
}
pub fn walk_position<'a, VisitorT>(visitor: &mut VisitorT, node: &Position<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Position);
    visitor.visit_position_component((&node.x).as_ref());
    visitor.visit_position_component((&node.y).as_ref());
    visitor.leave_node(AstType::Position);
}
pub fn walk_web_kit_gradient_point<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &WebKitGradientPoint<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::WebKitGradientPoint);
    visitor.visit_web_kit_gradient_point_component((&node.x).as_ref());
    visitor.visit_web_kit_gradient_point_component((&node.y).as_ref());
    visitor.leave_node(AstType::WebKitGradientPoint);
}
pub fn walk_web_kit_color_stop<'a, VisitorT>(visitor: &mut VisitorT, node: &WebKitColorStop<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::WebKitColorStop);
    visitor.visit_css_color((&node.color).as_ref());
    visitor.leave_node(AstType::WebKitColorStop);
}
pub fn walk_image_set<'a, VisitorT>(visitor: &mut VisitorT, node: &ImageSet<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ImageSet);
    for value_0 in (&node.options).iter() {
        visitor.visit_image_set_option(value_0);
    }
    visitor.visit_vendor_prefix(&node.vendor_prefix);
    visitor.leave_node(AstType::ImageSet);
}
pub fn walk_image_set_option<'a, VisitorT>(visitor: &mut VisitorT, node: &ImageSetOption<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ImageSetOption);
    if let Some(value_0) = (&node.file_type).as_ref() {
        visitor.visit_str(value_0);
    }
    visitor.visit_image((&node.image).as_ref());
    visitor.visit_resolution((&node.resolution).as_ref());
    visitor.leave_node(AstType::ImageSetOption);
}
pub fn walk_background_position<'a, VisitorT>(visitor: &mut VisitorT, node: &BackgroundPosition<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BackgroundPosition);
    visitor.visit_position_component((&node.x).as_ref());
    visitor.visit_position_component((&node.y).as_ref());
    visitor.leave_node(AstType::BackgroundPosition);
}
pub fn walk_background_repeat<'a, VisitorT>(visitor: &mut VisitorT, node: &BackgroundRepeat)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BackgroundRepeat);
    visitor.visit_background_repeat_keyword(&node.x);
    visitor.visit_background_repeat_keyword(&node.y);
    visitor.leave_node(AstType::BackgroundRepeat);
}
pub fn walk_background<'a, VisitorT>(visitor: &mut VisitorT, node: &Background<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Background);
    visitor.visit_background_attachment(&node.attachment);
    visitor.visit_background_clip(&node.clip);
    visitor.visit_css_color((&node.color).as_ref());
    visitor.visit_image((&node.image).as_ref());
    visitor.visit_background_origin(&node.origin);
    visitor.visit_background_position((&node.position).as_ref());
    visitor.visit_background_repeat((&node.repeat).as_ref());
    visitor.visit_background_size((&node.size).as_ref());
    visitor.leave_node(AstType::Background);
}
pub fn walk_box_shadow<'a, VisitorT>(visitor: &mut VisitorT, node: &BoxShadow<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BoxShadow);
    visitor.visit_length((&node.blur).as_ref());
    visitor.visit_css_color((&node.color).as_ref());
    visitor.visit_length((&node.spread).as_ref());
    visitor.visit_length((&node.x_offset).as_ref());
    visitor.visit_length((&node.y_offset).as_ref());
    visitor.leave_node(AstType::BoxShadow);
}
pub fn walk_aspect_ratio<'a, VisitorT>(visitor: &mut VisitorT, node: &AspectRatio<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::AspectRatio);
    if let Some(value_0) = (&node.ratio).as_ref() {
        visitor.visit_ratio((value_0).as_ref());
    }
    visitor.leave_node(AstType::AspectRatio);
}
pub fn walk_overflow<'a, VisitorT>(visitor: &mut VisitorT, node: &Overflow)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Overflow);
    visitor.visit_overflow_keyword(&node.x);
    visitor.visit_overflow_keyword(&node.y);
    visitor.leave_node(AstType::Overflow);
}
pub fn walk_inset_block<'a, VisitorT>(visitor: &mut VisitorT, node: &InsetBlock<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::InsetBlock);
    visitor.visit_length_percentage_or_auto((&node.block_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.block_start).as_ref());
    visitor.leave_node(AstType::InsetBlock);
}
pub fn walk_inset_inline<'a, VisitorT>(visitor: &mut VisitorT, node: &InsetInline<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::InsetInline);
    visitor.visit_length_percentage_or_auto((&node.inline_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.inline_start).as_ref());
    visitor.leave_node(AstType::InsetInline);
}
pub fn walk_inset<'a, VisitorT>(visitor: &mut VisitorT, node: &Inset<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Inset);
    visitor.visit_length_percentage_or_auto((&node.bottom).as_ref());
    visitor.visit_length_percentage_or_auto((&node.left).as_ref());
    visitor.visit_length_percentage_or_auto((&node.right).as_ref());
    visitor.visit_length_percentage_or_auto((&node.top).as_ref());
    visitor.leave_node(AstType::Inset);
}
pub fn walk_border_radius<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderRadius<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderRadius);
    visitor.visit_size_2_d((&node.bottom_left).as_ref());
    visitor.visit_size_2_d((&node.bottom_right).as_ref());
    visitor.visit_size_2_d((&node.top_left).as_ref());
    visitor.visit_size_2_d((&node.top_right).as_ref());
    visitor.leave_node(AstType::BorderRadius);
}
pub fn walk_border_image_repeat<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderImageRepeat)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderImageRepeat);
    visitor.visit_border_image_repeat_keyword(&node.horizontal);
    visitor.visit_border_image_repeat_keyword(&node.vertical);
    visitor.leave_node(AstType::BorderImageRepeat);
}
pub fn walk_border_image_slice<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderImageSlice<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderImageSlice);
    visitor.visit_rect((&node.offsets).as_ref());
    visitor.leave_node(AstType::BorderImageSlice);
}
pub fn walk_border_image<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderImage<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderImage);
    visitor.visit_rect((&node.outset).as_ref());
    visitor.visit_border_image_repeat((&node.repeat).as_ref());
    visitor.visit_border_image_slice((&node.slice).as_ref());
    visitor.visit_image((&node.source).as_ref());
    visitor.visit_rect((&node.width).as_ref());
    visitor.leave_node(AstType::BorderImage);
}
pub fn walk_border_color<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderColor<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderColor);
    visitor.visit_css_color((&node.bottom).as_ref());
    visitor.visit_css_color((&node.left).as_ref());
    visitor.visit_css_color((&node.right).as_ref());
    visitor.visit_css_color((&node.top).as_ref());
    visitor.leave_node(AstType::BorderColor);
}
pub fn walk_border_style<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderStyle)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderStyle);
    visitor.visit_line_style(&node.bottom);
    visitor.visit_line_style(&node.left);
    visitor.visit_line_style(&node.right);
    visitor.visit_line_style(&node.top);
    visitor.leave_node(AstType::BorderStyle);
}
pub fn walk_border_width<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderWidth<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderWidth);
    visitor.visit_border_side_width((&node.bottom).as_ref());
    visitor.visit_border_side_width((&node.left).as_ref());
    visitor.visit_border_side_width((&node.right).as_ref());
    visitor.visit_border_side_width((&node.top).as_ref());
    visitor.leave_node(AstType::BorderWidth);
}
pub fn walk_border_block_color<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderBlockColor<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderBlockColor);
    visitor.visit_css_color((&node.end).as_ref());
    visitor.visit_css_color((&node.start).as_ref());
    visitor.leave_node(AstType::BorderBlockColor);
}
pub fn walk_border_block_style<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderBlockStyle)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderBlockStyle);
    visitor.visit_line_style(&node.end);
    visitor.visit_line_style(&node.start);
    visitor.leave_node(AstType::BorderBlockStyle);
}
pub fn walk_border_block_width<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderBlockWidth<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderBlockWidth);
    visitor.visit_border_side_width((&node.end).as_ref());
    visitor.visit_border_side_width((&node.start).as_ref());
    visitor.leave_node(AstType::BorderBlockWidth);
}
pub fn walk_border_inline_color<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderInlineColor<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderInlineColor);
    visitor.visit_css_color((&node.end).as_ref());
    visitor.visit_css_color((&node.start).as_ref());
    visitor.leave_node(AstType::BorderInlineColor);
}
pub fn walk_border_inline_style<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderInlineStyle)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderInlineStyle);
    visitor.visit_line_style(&node.end);
    visitor.visit_line_style(&node.start);
    visitor.leave_node(AstType::BorderInlineStyle);
}
pub fn walk_border_inline_width<'a, VisitorT>(visitor: &mut VisitorT, node: &BorderInlineWidth<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::BorderInlineWidth);
    visitor.visit_border_side_width((&node.end).as_ref());
    visitor.visit_border_side_width((&node.start).as_ref());
    visitor.leave_node(AstType::BorderInlineWidth);
}
pub fn walk_generic_border<'a, S, VisitorT>(visitor: &mut VisitorT, node: &GenericBorder<'a, S>)
where
    VisitorT: ?Sized + Visit<'a>,
    S: VisitNode<'a, VisitorT>,
{
    visitor.enter_node(AstType::GenericBorder);
    visitor.visit_css_color((&node.color).as_ref());
    VisitNode::visit_node(&node.style, visitor);
    visitor.visit_border_side_width((&node.width).as_ref());
    visitor.leave_node(AstType::GenericBorder);
}
pub fn walk_flex_flow<'a, VisitorT>(visitor: &mut VisitorT, node: &FlexFlow)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FlexFlow);
    visitor.visit_flex_direction(&node.direction);
    visitor.visit_flex_wrap(&node.wrap);
    visitor.leave_node(AstType::FlexFlow);
}
pub fn walk_flex<'a, VisitorT>(visitor: &mut VisitorT, node: &Flex<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Flex);
    visitor.visit_length_percentage_or_auto((&node.basis).as_ref());
    visitor.leave_node(AstType::Flex);
}
pub fn walk_place_content<'a, VisitorT>(visitor: &mut VisitorT, node: &PlaceContent<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PlaceContent);
    visitor.visit_align_content((&node.align).as_ref());
    visitor.visit_justify_content((&node.justify).as_ref());
    visitor.leave_node(AstType::PlaceContent);
}
pub fn walk_place_self<'a, VisitorT>(visitor: &mut VisitorT, node: &PlaceSelf<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PlaceSelf);
    visitor.visit_align_self((&node.align).as_ref());
    visitor.visit_justify_self((&node.justify).as_ref());
    visitor.leave_node(AstType::PlaceSelf);
}
pub fn walk_place_items<'a, VisitorT>(visitor: &mut VisitorT, node: &PlaceItems<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PlaceItems);
    visitor.visit_align_items((&node.align).as_ref());
    visitor.visit_justify_items((&node.justify).as_ref());
    visitor.leave_node(AstType::PlaceItems);
}
pub fn walk_gap<'a, VisitorT>(visitor: &mut VisitorT, node: &Gap<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Gap);
    visitor.visit_gap_value((&node.column).as_ref());
    visitor.visit_gap_value((&node.row).as_ref());
    visitor.leave_node(AstType::Gap);
}
pub fn walk_track_repeat<'a, VisitorT>(visitor: &mut VisitorT, node: &TrackRepeat<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TrackRepeat);
    visitor.visit_repeat_count((&node.count).as_ref());
    for value_1 in (&node.line_names).iter() {
        for value_2 in (value_1).iter() {
            visitor.visit_str(value_2);
        }
    }
    for value_3 in (&node.track_sizes).iter() {
        visitor.visit_track_size(value_3);
    }
    visitor.leave_node(AstType::TrackRepeat);
}
pub fn walk_grid_auto_flow<'a, VisitorT>(visitor: &mut VisitorT, node: &GridAutoFlow)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::GridAutoFlow);
    visitor.visit_auto_flow_direction(&node.direction);
    visitor.leave_node(AstType::GridAutoFlow);
}
pub fn walk_grid_template<'a, VisitorT>(visitor: &mut VisitorT, node: &GridTemplate<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::GridTemplate);
    visitor.visit_grid_template_areas((&node.areas).as_ref());
    visitor.visit_track_sizing((&node.columns).as_ref());
    visitor.visit_track_sizing((&node.rows).as_ref());
    visitor.leave_node(AstType::GridTemplate);
}
pub fn walk_grid<'a, VisitorT>(visitor: &mut VisitorT, node: &Grid<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Grid);
    visitor.visit_grid_template_areas((&node.areas).as_ref());
    for value_1 in (&node.auto_columns).iter() {
        visitor.visit_track_size(value_1);
    }
    visitor.visit_grid_auto_flow((&node.auto_flow).as_ref());
    for value_3 in (&node.auto_rows).iter() {
        visitor.visit_track_size(value_3);
    }
    visitor.visit_track_sizing((&node.columns).as_ref());
    visitor.visit_track_sizing((&node.rows).as_ref());
    visitor.leave_node(AstType::Grid);
}
pub fn walk_grid_row<'a, VisitorT>(visitor: &mut VisitorT, node: &GridRow<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::GridRow);
    visitor.visit_grid_line((&node.end).as_ref());
    visitor.visit_grid_line((&node.start).as_ref());
    visitor.leave_node(AstType::GridRow);
}
pub fn walk_grid_column<'a, VisitorT>(visitor: &mut VisitorT, node: &GridColumn<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::GridColumn);
    visitor.visit_grid_line((&node.end).as_ref());
    visitor.visit_grid_line((&node.start).as_ref());
    visitor.leave_node(AstType::GridColumn);
}
pub fn walk_grid_area<'a, VisitorT>(visitor: &mut VisitorT, node: &GridArea<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::GridArea);
    visitor.visit_grid_line((&node.column_end).as_ref());
    visitor.visit_grid_line((&node.column_start).as_ref());
    visitor.visit_grid_line((&node.row_end).as_ref());
    visitor.visit_grid_line((&node.row_start).as_ref());
    visitor.leave_node(AstType::GridArea);
}
pub fn walk_margin_block<'a, VisitorT>(visitor: &mut VisitorT, node: &MarginBlock<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::MarginBlock);
    visitor.visit_length_percentage_or_auto((&node.block_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.block_start).as_ref());
    visitor.leave_node(AstType::MarginBlock);
}
pub fn walk_margin_inline<'a, VisitorT>(visitor: &mut VisitorT, node: &MarginInline<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::MarginInline);
    visitor.visit_length_percentage_or_auto((&node.inline_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.inline_start).as_ref());
    visitor.leave_node(AstType::MarginInline);
}
pub fn walk_margin<'a, VisitorT>(visitor: &mut VisitorT, node: &Margin<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Margin);
    visitor.visit_length_percentage_or_auto((&node.bottom).as_ref());
    visitor.visit_length_percentage_or_auto((&node.left).as_ref());
    visitor.visit_length_percentage_or_auto((&node.right).as_ref());
    visitor.visit_length_percentage_or_auto((&node.top).as_ref());
    visitor.leave_node(AstType::Margin);
}
pub fn walk_padding_block<'a, VisitorT>(visitor: &mut VisitorT, node: &PaddingBlock<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PaddingBlock);
    visitor.visit_length_percentage_or_auto((&node.block_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.block_start).as_ref());
    visitor.leave_node(AstType::PaddingBlock);
}
pub fn walk_padding_inline<'a, VisitorT>(visitor: &mut VisitorT, node: &PaddingInline<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PaddingInline);
    visitor.visit_length_percentage_or_auto((&node.inline_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.inline_start).as_ref());
    visitor.leave_node(AstType::PaddingInline);
}
pub fn walk_padding<'a, VisitorT>(visitor: &mut VisitorT, node: &Padding<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Padding);
    visitor.visit_length_percentage_or_auto((&node.bottom).as_ref());
    visitor.visit_length_percentage_or_auto((&node.left).as_ref());
    visitor.visit_length_percentage_or_auto((&node.right).as_ref());
    visitor.visit_length_percentage_or_auto((&node.top).as_ref());
    visitor.leave_node(AstType::Padding);
}
pub fn walk_scroll_margin_block<'a, VisitorT>(visitor: &mut VisitorT, node: &ScrollMarginBlock<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollMarginBlock);
    visitor.visit_length_percentage_or_auto((&node.block_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.block_start).as_ref());
    visitor.leave_node(AstType::ScrollMarginBlock);
}
pub fn walk_scroll_margin_inline<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ScrollMarginInline<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollMarginInline);
    visitor.visit_length_percentage_or_auto((&node.inline_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.inline_start).as_ref());
    visitor.leave_node(AstType::ScrollMarginInline);
}
pub fn walk_scroll_margin<'a, VisitorT>(visitor: &mut VisitorT, node: &ScrollMargin<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollMargin);
    visitor.visit_length_percentage_or_auto((&node.bottom).as_ref());
    visitor.visit_length_percentage_or_auto((&node.left).as_ref());
    visitor.visit_length_percentage_or_auto((&node.right).as_ref());
    visitor.visit_length_percentage_or_auto((&node.top).as_ref());
    visitor.leave_node(AstType::ScrollMargin);
}
pub fn walk_scroll_padding_block<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ScrollPaddingBlock<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollPaddingBlock);
    visitor.visit_length_percentage_or_auto((&node.block_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.block_start).as_ref());
    visitor.leave_node(AstType::ScrollPaddingBlock);
}
pub fn walk_scroll_padding_inline<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ScrollPaddingInline<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollPaddingInline);
    visitor.visit_length_percentage_or_auto((&node.inline_end).as_ref());
    visitor.visit_length_percentage_or_auto((&node.inline_start).as_ref());
    visitor.leave_node(AstType::ScrollPaddingInline);
}
pub fn walk_scroll_padding<'a, VisitorT>(visitor: &mut VisitorT, node: &ScrollPadding<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollPadding);
    visitor.visit_length_percentage_or_auto((&node.bottom).as_ref());
    visitor.visit_length_percentage_or_auto((&node.left).as_ref());
    visitor.visit_length_percentage_or_auto((&node.right).as_ref());
    visitor.visit_length_percentage_or_auto((&node.top).as_ref());
    visitor.leave_node(AstType::ScrollPadding);
}
pub fn walk_font<'a, VisitorT>(visitor: &mut VisitorT, node: &Font<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Font);
    for value_0 in (&node.family).iter() {
        visitor.visit_font_family(value_0);
    }
    visitor.visit_line_height((&node.line_height).as_ref());
    visitor.visit_font_size((&node.size).as_ref());
    visitor.visit_font_stretch((&node.stretch).as_ref());
    visitor.visit_font_style((&node.style).as_ref());
    visitor.visit_font_variant_caps(&node.variant_caps);
    visitor.visit_font_weight((&node.weight).as_ref());
    visitor.leave_node(AstType::Font);
}
pub fn walk_transition<'a, VisitorT>(visitor: &mut VisitorT, node: &Transition<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Transition);
    visitor.visit_time((&node.delay).as_ref());
    visitor.visit_time((&node.duration).as_ref());
    visitor.visit_property_id((&node.property).as_ref());
    visitor.visit_easing_function((&node.timing_function).as_ref());
    visitor.leave_node(AstType::Transition);
}
pub fn walk_scroll_timeline<'a, VisitorT>(visitor: &mut VisitorT, node: &ScrollTimeline)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollTimeline);
    visitor.visit_scroll_axis(&node.axis);
    visitor.visit_scroller(&node.scroller);
    visitor.leave_node(AstType::ScrollTimeline);
}
pub fn walk_view_timeline<'a, VisitorT>(visitor: &mut VisitorT, node: &ViewTimeline<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ViewTimeline);
    visitor.visit_scroll_axis(&node.axis);
    visitor.visit_size_2_d((&node.inset).as_ref());
    visitor.leave_node(AstType::ViewTimeline);
}
pub fn walk_animation_range<'a, VisitorT>(visitor: &mut VisitorT, node: &AnimationRange<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::AnimationRange);
    visitor.visit_animation_range_end((&node.end).as_ref());
    visitor.visit_animation_range_start((&node.start).as_ref());
    visitor.leave_node(AstType::AnimationRange);
}
pub fn walk_animation<'a, VisitorT>(visitor: &mut VisitorT, node: &Animation<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Animation);
    visitor.visit_time((&node.delay).as_ref());
    visitor.visit_animation_direction(&node.direction);
    visitor.visit_time((&node.duration).as_ref());
    visitor.visit_animation_fill_mode(&node.fill_mode);
    visitor.visit_animation_iteration_count((&node.iteration_count).as_ref());
    visitor.visit_animation_name((&node.name).as_ref());
    visitor.visit_animation_play_state(&node.play_state);
    visitor.visit_animation_timeline((&node.timeline).as_ref());
    visitor.visit_easing_function((&node.timing_function).as_ref());
    visitor.leave_node(AstType::Animation);
}
pub fn walk_matrix_for_float<'a, VisitorT>(visitor: &mut VisitorT, node: &MatrixForFloat)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::MatrixForFloat);
    visitor.leave_node(AstType::MatrixForFloat);
}
pub fn walk_matrix_3_d_for_float<'a, VisitorT>(visitor: &mut VisitorT, node: &Matrix3DForFloat)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Matrix3DForFloat);
    visitor.leave_node(AstType::Matrix3DForFloat);
}
pub fn walk_rotate<'a, VisitorT>(visitor: &mut VisitorT, node: &Rotate<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Rotate);
    visitor.visit_angle((&node.angle).as_ref());
    visitor.leave_node(AstType::Rotate);
}
pub fn walk_text_transform<'a, VisitorT>(visitor: &mut VisitorT, node: &TextTransform)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TextTransform);
    visitor.visit_text_transform_case(&node.case);
    visitor.leave_node(AstType::TextTransform);
}
pub fn walk_text_indent<'a, VisitorT>(visitor: &mut VisitorT, node: &TextIndent<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TextIndent);
    visitor.visit_length_percentage((&node.value).as_ref());
    visitor.leave_node(AstType::TextIndent);
}
pub fn walk_text_decoration<'a, VisitorT>(visitor: &mut VisitorT, node: &TextDecoration<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TextDecoration);
    visitor.visit_css_color((&node.color).as_ref());
    visitor.visit_text_decoration_line((&node.line).as_ref());
    visitor.visit_text_decoration_style(&node.style);
    visitor.visit_text_decoration_thickness((&node.thickness).as_ref());
    visitor.leave_node(AstType::TextDecoration);
}
pub fn walk_text_emphasis<'a, VisitorT>(visitor: &mut VisitorT, node: &TextEmphasis<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TextEmphasis);
    visitor.visit_css_color((&node.color).as_ref());
    visitor.visit_text_emphasis_style((&node.style).as_ref());
    visitor.leave_node(AstType::TextEmphasis);
}
pub fn walk_text_emphasis_position<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &TextEmphasisPosition,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TextEmphasisPosition);
    visitor.visit_text_emphasis_position_horizontal(&node.horizontal);
    visitor.visit_text_emphasis_position_vertical(&node.vertical);
    visitor.leave_node(AstType::TextEmphasisPosition);
}
pub fn walk_text_shadow<'a, VisitorT>(visitor: &mut VisitorT, node: &TextShadow<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TextShadow);
    visitor.visit_length((&node.blur).as_ref());
    visitor.visit_css_color((&node.color).as_ref());
    visitor.visit_length((&node.spread).as_ref());
    visitor.visit_length((&node.x_offset).as_ref());
    visitor.visit_length((&node.y_offset).as_ref());
    visitor.leave_node(AstType::TextShadow);
}
pub fn walk_cursor<'a, VisitorT>(visitor: &mut VisitorT, node: &Cursor<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Cursor);
    for value_0 in (&node.images).iter() {
        visitor.visit_cursor_image(value_0);
    }
    visitor.visit_cursor_keyword(&node.keyword);
    visitor.leave_node(AstType::Cursor);
}
pub fn walk_cursor_image<'a, VisitorT>(visitor: &mut VisitorT, node: &CursorImage<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::CursorImage);
    if let Some(value_0) = (&node.hotspot).as_ref() {}
    visitor.visit_url((&node.url).as_ref());
    visitor.leave_node(AstType::CursorImage);
}
pub fn walk_caret<'a, VisitorT>(visitor: &mut VisitorT, node: &Caret<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Caret);
    visitor.visit_color_or_auto((&node.color).as_ref());
    visitor.visit_caret_shape(&node.shape);
    visitor.leave_node(AstType::Caret);
}
pub fn walk_list_style<'a, VisitorT>(visitor: &mut VisitorT, node: &ListStyle<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ListStyle);
    visitor.visit_image((&node.image).as_ref());
    visitor.visit_list_style_type((&node.list_style_type).as_ref());
    visitor.visit_list_style_position(&node.position);
    visitor.leave_node(AstType::ListStyle);
}
pub fn walk_composes<'a, VisitorT>(visitor: &mut VisitorT, node: &Composes<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Composes);
    if let Some(value_0) = (&node.from).as_ref() {
        visitor.visit_specifier((value_0).as_ref());
    }
    visitor.visit_span(&node.span);
    for value_2 in (&node.names).iter() {
        visitor.visit_str(value_2);
    }
    visitor.leave_node(AstType::Composes);
}
pub fn walk_inset_rect<'a, VisitorT>(visitor: &mut VisitorT, node: &InsetRect<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::InsetRect);
    visitor.visit_border_radius((&node.radius).as_ref());
    visitor.visit_rect((&node.rect).as_ref());
    visitor.leave_node(AstType::InsetRect);
}
pub fn walk_circle_shape<'a, VisitorT>(visitor: &mut VisitorT, node: &CircleShape<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::CircleShape);
    visitor.visit_position((&node.position).as_ref());
    visitor.visit_shape_radius((&node.radius).as_ref());
    visitor.leave_node(AstType::CircleShape);
}
pub fn walk_ellipse_shape<'a, VisitorT>(visitor: &mut VisitorT, node: &EllipseShape<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::EllipseShape);
    visitor.visit_position((&node.position).as_ref());
    visitor.visit_shape_radius((&node.radius_x).as_ref());
    visitor.visit_shape_radius((&node.radius_y).as_ref());
    visitor.leave_node(AstType::EllipseShape);
}
pub fn walk_polygon<'a, VisitorT>(visitor: &mut VisitorT, node: &Polygon<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Polygon);
    visitor.visit_fill_rule(&node.fill_rule);
    for value_0 in (&node.points).iter() {
        visitor.visit_point(value_0);
    }
    visitor.leave_node(AstType::Polygon);
}
pub fn walk_point<'a, VisitorT>(visitor: &mut VisitorT, node: &Point<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Point);
    visitor.visit_length_percentage((&node.x).as_ref());
    visitor.visit_length_percentage((&node.y).as_ref());
    visitor.leave_node(AstType::Point);
}
pub fn walk_mask<'a, VisitorT>(visitor: &mut VisitorT, node: &Mask<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Mask);
    visitor.visit_mask_clip((&node.clip).as_ref());
    visitor.visit_mask_composite(&node.composite);
    visitor.visit_image((&node.image).as_ref());
    visitor.visit_mask_mode(&node.mode);
    visitor.visit_geometry_box(&node.origin);
    visitor.visit_position((&node.position).as_ref());
    visitor.visit_background_repeat((&node.repeat).as_ref());
    visitor.visit_background_size((&node.size).as_ref());
    visitor.leave_node(AstType::Mask);
}
pub fn walk_mask_border<'a, VisitorT>(visitor: &mut VisitorT, node: &MaskBorder<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::MaskBorder);
    visitor.visit_mask_border_mode(&node.mode);
    visitor.visit_rect((&node.outset).as_ref());
    visitor.visit_border_image_repeat((&node.repeat).as_ref());
    visitor.visit_border_image_slice((&node.slice).as_ref());
    visitor.visit_image((&node.source).as_ref());
    visitor.visit_rect((&node.width).as_ref());
    visitor.leave_node(AstType::MaskBorder);
}
pub fn walk_drop_shadow<'a, VisitorT>(visitor: &mut VisitorT, node: &DropShadow<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::DropShadow);
    visitor.visit_length((&node.blur).as_ref());
    visitor.visit_css_color((&node.color).as_ref());
    visitor.visit_length((&node.x_offset).as_ref());
    visitor.visit_length((&node.y_offset).as_ref());
    visitor.leave_node(AstType::DropShadow);
}
pub fn walk_container<'a, VisitorT>(visitor: &mut VisitorT, node: &Container<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Container);
    visitor.visit_container_type(&node.container_type);
    visitor.visit_container_name_list((&node.name).as_ref());
    visitor.leave_node(AstType::Container);
}
pub fn walk_color_scheme<'a, VisitorT>(visitor: &mut VisitorT, node: &ColorScheme)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ColorScheme);
    visitor.leave_node(AstType::ColorScheme);
}
pub fn walk_unparsed_property<'a, VisitorT>(visitor: &mut VisitorT, node: &UnparsedProperty<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::UnparsedProperty);
    visitor.visit_property_id((&node.property_id).as_ref());
    for value_1 in (&node.value).iter() {
        visitor.visit_token_or_value(value_1);
    }
    visitor.leave_node(AstType::UnparsedProperty);
}
pub fn walk_custom_property<'a, VisitorT>(visitor: &mut VisitorT, node: &CustomProperty<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::CustomProperty);
    visitor.visit_custom_property_name((&node.name).as_ref());
    for value_1 in (&node.value).iter() {
        visitor.visit_token_or_value(value_1);
    }
    visitor.leave_node(AstType::CustomProperty);
}
pub fn walk_view_transition_part_selector<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ViewTransitionPartSelector<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ViewTransitionPartSelector);
    for value_0 in (&node.classes).iter() {
        visitor.visit_str(value_0);
    }
    if let Some(value_1) = (&node.name).as_ref() {
        visitor.visit_view_transition_part_name((value_1).as_ref());
    }
    visitor.leave_node(AstType::ViewTransitionPartSelector);
}
pub fn walk_keyframes_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &KeyframesRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::KeyframesRule);
    for value_0 in (&node.keyframes).iter() {
        visitor.visit_keyframe(value_0);
    }
    visitor.visit_span(&node.span);
    visitor.visit_keyframes_name((&node.name).as_ref());
    visitor.visit_vendor_prefix(&node.vendor_prefix);
    visitor.leave_node(AstType::KeyframesRule);
}
pub fn walk_keyframe<'a, VisitorT>(visitor: &mut VisitorT, node: &Keyframe<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::Keyframe);
    visitor.visit_declaration_block((&node.declarations).as_ref());
    for value_1 in (&node.selectors).iter() {
        visitor.visit_keyframe_selector(value_1);
    }
    visitor.leave_node(AstType::Keyframe);
}
pub fn walk_timeline_range_percentage<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &TimelineRangePercentage,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::TimelineRangePercentage);
    visitor.visit_timeline_range_name(&node.name);
    visitor.leave_node(AstType::TimelineRangePercentage);
}
pub fn walk_font_face_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &FontFaceRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FontFaceRule);
    visitor.visit_span(&node.span);
    for value_0 in (&node.properties).iter() {
        visitor.visit_font_face_property(value_0);
    }
    visitor.leave_node(AstType::FontFaceRule);
}
pub fn walk_url_source<'a, VisitorT>(visitor: &mut VisitorT, node: &UrlSource<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::UrlSource);
    if let Some(value_0) = (&node.format).as_ref() {
        visitor.visit_font_format((value_0).as_ref());
    }
    for value_2 in (&node.tech).iter() {
        visitor.visit_font_technology(value_2);
    }
    visitor.visit_url((&node.url).as_ref());
    visitor.leave_node(AstType::UrlSource);
}
pub fn walk_unicode_range<'a, VisitorT>(visitor: &mut VisitorT, node: &UnicodeRange)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::UnicodeRange);
    visitor.leave_node(AstType::UnicodeRange);
}
pub fn walk_font_palette_values_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &FontPaletteValuesRule<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FontPaletteValuesRule);
    visitor.visit_span(&node.span);
    visitor.visit_str(&node.name);
    for value_0 in (&node.properties).iter() {
        visitor.visit_font_palette_values_property(value_0);
    }
    visitor.leave_node(AstType::FontPaletteValuesRule);
}
pub fn walk_override_colors<'a, VisitorT>(visitor: &mut VisitorT, node: &OverrideColors<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::OverrideColors);
    visitor.visit_css_color((&node.color).as_ref());
    visitor.leave_node(AstType::OverrideColors);
}
pub fn walk_font_feature_values_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &FontFeatureValuesRule<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FontFeatureValuesRule);
    visitor.visit_span(&node.span);
    for value_0 in (&node.name).iter() {
        visitor.visit_family_name(value_0);
    }
    for value_1 in (&node.rules).iter() {
        visitor.visit_font_feature_subrule(value_1);
    }
    visitor.leave_node(AstType::FontFeatureValuesRule);
}
pub fn walk_font_feature_subrule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &FontFeatureSubrule<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FontFeatureSubrule);
    for value_0 in (&node.declarations).iter() {
        visitor.visit_font_feature_declaration(value_0);
    }
    visitor.visit_span(&node.span);
    visitor.visit_font_feature_subrule_type(&node.name);
    visitor.leave_node(AstType::FontFeatureSubrule);
}
pub fn walk_font_feature_declaration<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &FontFeatureDeclaration<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FontFeatureDeclaration);
    visitor.visit_str(&node.name);
    for value_0 in (&node.values).iter() {}
    visitor.leave_node(AstType::FontFeatureDeclaration);
}
pub fn walk_family_name<'a, VisitorT>(visitor: &mut VisitorT, node: &FamilyName<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::FamilyName);
    visitor.visit_str(&node.0);
    visitor.leave_node(AstType::FamilyName);
}
pub fn walk_page_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &PageRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PageRule);
    visitor.visit_declaration_block((&node.declarations).as_ref());
    visitor.visit_span(&node.span);
    for value_1 in (&node.rules).iter() {
        visitor.visit_page_margin_rule(value_1);
    }
    for value_2 in (&node.selectors).iter() {
        visitor.visit_page_selector(value_2);
    }
    visitor.leave_node(AstType::PageRule);
}
pub fn walk_page_margin_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &PageMarginRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PageMarginRule);
    visitor.visit_declaration_block((&node.declarations).as_ref());
    visitor.visit_span(&node.span);
    visitor.visit_page_margin_box(&node.margin_box);
    visitor.leave_node(AstType::PageMarginRule);
}
pub fn walk_page_selector<'a, VisitorT>(visitor: &mut VisitorT, node: &PageSelector<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PageSelector);
    if let Some(value_0) = (&node.name).as_ref() {
        visitor.visit_str(value_0);
    }
    for value_1 in (&node.pseudo_classes).iter() {
        visitor.visit_page_pseudo_class(value_1);
    }
    visitor.leave_node(AstType::PageSelector);
}
pub fn walk_supports_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &SupportsRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::SupportsRule);
    visitor.visit_supports_condition((&node.condition).as_ref());
    visitor.visit_span(&node.span);
    for value_1 in (&node.rules).iter() {
        visitor.visit_css_rule(value_1);
    }
    visitor.leave_node(AstType::SupportsRule);
}
pub fn walk_counter_style_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &CounterStyleRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::CounterStyleRule);
    visitor.visit_declaration_block((&node.declarations).as_ref());
    visitor.visit_span(&node.span);
    visitor.visit_str(&node.name);
    visitor.leave_node(AstType::CounterStyleRule);
}
pub fn walk_namespace_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &NamespaceRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::NamespaceRule);
    visitor.visit_span(&node.span);
    if let Some(value_0) = (&node.prefix).as_ref() {
        visitor.visit_str(value_0);
    }
    visitor.visit_str(&node.url);
    visitor.leave_node(AstType::NamespaceRule);
}
pub fn walk_moz_document_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &MozDocumentRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::MozDocumentRule);
    visitor.visit_span(&node.span);
    for value_0 in (&node.rules).iter() {
        visitor.visit_css_rule(value_0);
    }
    visitor.leave_node(AstType::MozDocumentRule);
}
pub fn walk_nesting_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &NestingRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::NestingRule);
    visitor.visit_span(&node.span);
    visitor.visit_style_rule((&node.style).as_ref());
    visitor.leave_node(AstType::NestingRule);
}
pub fn walk_nested_declarations_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &NestedDeclarationsRule<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::NestedDeclarationsRule);
    visitor.visit_declaration_block((&node.declarations).as_ref());
    visitor.visit_span(&node.span);
    visitor.leave_node(AstType::NestedDeclarationsRule);
}
pub fn walk_viewport_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &ViewportRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ViewportRule);
    visitor.visit_declaration_block((&node.declarations).as_ref());
    visitor.visit_span(&node.span);
    visitor.visit_vendor_prefix(&node.vendor_prefix);
    visitor.leave_node(AstType::ViewportRule);
}
pub fn walk_custom_media_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &CustomMediaRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::CustomMediaRule);
    visitor.visit_span(&node.span);
    visitor.visit_str(&node.name);
    visitor.visit_media_list(&node.query);
    visitor.leave_node(AstType::CustomMediaRule);
}
pub fn walk_layer_statement_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &LayerStatementRule<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::LayerStatementRule);
    visitor.visit_span(&node.span);
    for value_0 in (&node.names).iter() {
        for value_1 in (value_0).iter() {
            visitor.visit_str(value_1);
        }
    }
    visitor.leave_node(AstType::LayerStatementRule);
}
pub fn walk_layer_block_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &LayerBlockRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::LayerBlockRule);
    visitor.visit_span(&node.span);
    if let Some(value_0) = (&node.name).as_ref() {
        for value_1 in (value_0).iter() {
            visitor.visit_str(value_1);
        }
    }
    for value_2 in (&node.rules).iter() {
        visitor.visit_css_rule(value_2);
    }
    visitor.leave_node(AstType::LayerBlockRule);
}
pub fn walk_property_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &PropertyRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PropertyRule);
    if let Some(value_0) = (&node.initial_value).as_ref() {
        visitor.visit_parsed_component((value_0).as_ref());
    }
    visitor.visit_span(&node.span);
    visitor.visit_str(&node.name);
    visitor.visit_syntax_string((&node.syntax).as_ref());
    visitor.leave_node(AstType::PropertyRule);
}
pub fn walk_syntax_component<'a, VisitorT>(visitor: &mut VisitorT, node: &SyntaxComponent<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::SyntaxComponent);
    visitor.visit_syntax_component_kind((&node.kind).as_ref());
    visitor.visit_multiplier(&node.multiplier);
    visitor.leave_node(AstType::SyntaxComponent);
}
pub fn walk_container_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &ContainerRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ContainerRule);
    if let Some(value_0) = (&node.condition).as_ref() {
        visitor.visit_container_condition((value_0).as_ref());
    }
    visitor.visit_span(&node.span);
    if let Some(value_2) = (&node.name).as_ref() {
        visitor.visit_str(value_2);
    }
    for value_3 in (&node.rules).iter() {
        visitor.visit_css_rule(value_3);
    }
    visitor.leave_node(AstType::ContainerRule);
}
pub fn walk_scope_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &ScopeRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScopeRule);
    visitor.visit_span(&node.span);
    for value_0 in (&node.rules).iter() {
        visitor.visit_css_rule(value_0);
    }
    if let Some(value_1) = (&node.scope_end).as_ref() {
        visitor.visit_selector_list((value_1).as_ref());
    }
    if let Some(value_3) = (&node.scope_start).as_ref() {
        visitor.visit_selector_list((value_3).as_ref());
    }
    visitor.leave_node(AstType::ScopeRule);
}
pub fn walk_starting_style_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &StartingStyleRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::StartingStyleRule);
    visitor.visit_span(&node.span);
    for value_0 in (&node.rules).iter() {
        visitor.visit_css_rule(value_0);
    }
    visitor.leave_node(AstType::StartingStyleRule);
}
pub fn walk_view_transition_rule<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ViewTransitionRule<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ViewTransitionRule);
    visitor.visit_span(&node.span);
    for value_0 in (&node.properties).iter() {
        visitor.visit_view_transition_property(value_0);
    }
    visitor.leave_node(AstType::ViewTransitionRule);
}
pub fn walk_position_try_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &PositionTryRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::PositionTryRule);
    visitor.visit_span(&node.span);
    visitor.visit_str(&node.name);
    visitor.visit_declaration_block((&node.declarations).as_ref());
    visitor.leave_node(AstType::PositionTryRule);
}
pub fn walk_unknown_at_rule<'a, VisitorT>(visitor: &mut VisitorT, node: &UnknownAtRule<'a>)
where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::UnknownAtRule);
    if let Some(value_0) = (&node.block).as_ref() {
        for value_1 in (value_0).iter() {
            visitor.visit_token_or_value(value_1);
        }
    }
    visitor.visit_span(&node.span);
    visitor.visit_str(&node.name);
    for value_2 in (&node.prelude).iter() {
        visitor.visit_token_or_value(value_2);
    }
    visitor.leave_node(AstType::UnknownAtRule);
}
pub fn walk_container_size_feature<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ContainerSizeFeature<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ContainerSizeFeature);
    visitor.visit_query_feature(node);
    visitor.leave_node(AstType::ContainerSizeFeature);
}
pub fn walk_scroll_state_feature<'a, VisitorT>(
    visitor: &mut VisitorT,
    node: &ScrollStateFeature<'a>,
) where
    VisitorT: ?Sized + Visit<'a>,
{
    visitor.enter_node(AstType::ScrollStateFeature);
    visitor.visit_query_feature(node);
    visitor.leave_node(AstType::ScrollStateFeature);
}
