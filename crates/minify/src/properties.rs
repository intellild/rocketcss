use rocketcss_ast::{PropertyId, match_ignore_ascii_case};

use crate::{
    MinifyContext, Options, OptionsOp,
    context::{PropertyContext, ValueContext, ValueContextFlags},
};

pub(crate) fn value_context(
    property_id: &PropertyId<'_>,
    order_values: bool,
    convert_zero_percentages: bool,
) -> ValueContext {
    let property = if order_values {
        match property_id {
            PropertyId::Animation(_) => PropertyContext::Animation,
            PropertyId::Transition(_) => PropertyContext::Transition,
            PropertyId::Border
            | PropertyId::BorderTop
            | PropertyId::BorderRight
            | PropertyId::BorderBottom
            | PropertyId::BorderLeft
            | PropertyId::BorderBlockStart
            | PropertyId::BorderBlockEnd
            | PropertyId::BorderInlineStart
            | PropertyId::BorderInlineEnd
            | PropertyId::BorderInline
            | PropertyId::BorderBlock => PropertyContext::Border,
            PropertyId::Outline => PropertyContext::Outline,
            PropertyId::BoxShadow(_) => PropertyContext::BoxShadow,
            PropertyId::FlexFlow(_) => PropertyContext::FlexFlow,
            PropertyId::GridAutoFlow => PropertyContext::GridAutoFlow,
            PropertyId::GridColumn
            | PropertyId::GridRow
            | PropertyId::GridColumnStart
            | PropertyId::GridColumnEnd
            | PropertyId::GridRowStart
            | PropertyId::GridRowEnd => PropertyContext::GridLine,
            PropertyId::ListStyle => PropertyContext::ListStyle,
            PropertyId::ColumnRule => PropertyContext::Outline,
            PropertyId::Columns => PropertyContext::Columns,
            PropertyId::GridColumnGap | PropertyId::GridRowGap => PropertyContext::GridGap,
            _ => property_context(property_id),
        }
    } else {
        property_context(property_id)
    };
    let mut cx = ValueContext::new(property);
    cx.set_enabled(
        ValueContextFlags::ALLOW_UNITLESS_ZERO_LENGTH,
        !matches!(property_id, PropertyId::LineHeight),
    );
    cx.set_enabled(
        ValueContextFlags::ALLOW_UNITLESS_ZERO_PERCENTAGE,
        !matches!(property_id, PropertyId::Custom(_))
            && (convert_zero_percentages || !zero_percentage_requires_target_support(property_id)),
    );
    cx.set_enabled(
        ValueContextFlags::MINIFY_COLORS,
        should_minify_colors(property_id),
    );
    cx
}

fn property_context(property_id: &PropertyId<'_>) -> PropertyContext {
    match property_id {
        PropertyId::Animation(_)
        | PropertyId::Transition(_)
        | PropertyId::AnimationTimingFunction(_)
        | PropertyId::TransitionTimingFunction(_) => PropertyContext::TimingFunction,
        PropertyId::Display => PropertyContext::Display,
        PropertyId::Font | PropertyId::FontFamily => PropertyContext::Font,
        PropertyId::FontWeight => PropertyContext::FontWeight,
        PropertyId::Background
        | PropertyId::BackgroundPosition
        | PropertyId::PerspectiveOrigin(_) => PropertyContext::Position,
        PropertyId::BackgroundRepeat | PropertyId::MaskRepeat(_) => PropertyContext::Repeat,
        PropertyId::Transform(_) => PropertyContext::Transform,
        PropertyId::Margin
        | PropertyId::Padding
        | PropertyId::BorderSpacing
        | PropertyId::BorderColor
        | PropertyId::BorderStyle
        | PropertyId::BorderWidth
        | PropertyId::Inset
        | PropertyId::ScrollMargin
        | PropertyId::ScrollPadding => PropertyContext::Box,
        _ => PropertyContext::Generic,
    }
}

fn should_minify_colors(property_id: &PropertyId<'_>) -> bool {
    match property_id {
        PropertyId::FontWeight
        | PropertyId::FontSize
        | PropertyId::FontStretch
        | PropertyId::FontFamily
        | PropertyId::FontStyle
        | PropertyId::FontVariantCaps
        | PropertyId::Font
        | PropertyId::FontPalette
        | PropertyId::Filter(_)
        | PropertyId::Composes => false,
        PropertyId::Custom(name) => {
            !(match_ignore_ascii_case!(
                name,
                "src" | "-webkit-tap-highlight-color" => true,
                _ => false,
            ) || starts_with_ignore_ascii_case(name, "font")
                || starts_with_ignore_ascii_case(name, "filter")
                || starts_with_ignore_ascii_case(name, "composes"))
        }
        _ => true,
    }
}

fn starts_with_ignore_ascii_case(value: &str, prefix: &str) -> bool {
    value
        .get(..prefix.len())
        .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
}

pub(crate) fn custom_property_context(cx: &MinifyContext) -> ValueContext {
    let mut value_context = ValueContext::default();
    value_context.set_enabled(
        ValueContextFlags::SKIP_VALUE_TRANSFORMS,
        cx.is_enabled(Options::TRANSFORM_CUSTOM_PROPERTIES, OptionsOp::None),
    );
    value_context
}

fn zero_percentage_requires_target_support(property_id: &PropertyId<'_>) -> bool {
    matches!(
        property_id,
        PropertyId::BorderImageWidth
            | PropertyId::FlexBasis(_)
            | PropertyId::Height
            | PropertyId::LineHeight
            | PropertyId::MaxHeight
            | PropertyId::MaxWidth
            | PropertyId::MinHeight
            | PropertyId::MinWidth
            | PropertyId::StrokeDasharray
            | PropertyId::StrokeDashoffset
            | PropertyId::StrokeWidth
            | PropertyId::Width
    )
}
