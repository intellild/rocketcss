use rocketcss_ast::PropertyId;

use crate::{
    MinifyContext,
    context::{PropertyContext, ValueContext},
};

pub(crate) fn value_context(
    property_id: &PropertyId<'_>,
    order_values: bool,
    convert_zero_percentages: bool,
) -> ValueContext {
    let name = property_id.name();
    ValueContext {
        allow_unitless_zero_length: !name.eq_ignore_ascii_case("line-height"),
        allow_unitless_zero_percentage: !matches!(property_id, PropertyId::Custom(_))
            && (convert_zero_percentages || !zero_percentage_requires_target_support(name)),
        minify_colors: should_minify_colors(name),
        skip_value_transforms: false,
        property: if order_values && is_animation_shorthand(name) {
            PropertyContext::Animation
        } else if order_values && is_transition_shorthand(name) {
            PropertyContext::Transition
        } else if order_values && is_ordered_border(name) {
            PropertyContext::Border
        } else if order_values
            && (name.eq_ignore_ascii_case("outline") || name.eq_ignore_ascii_case("column-rule"))
        {
            PropertyContext::Outline
        } else if order_values
            && (name.eq_ignore_ascii_case("box-shadow") || name.ends_with("-box-shadow"))
        {
            PropertyContext::BoxShadow
        } else if order_values && name.eq_ignore_ascii_case("flex-flow") {
            PropertyContext::FlexFlow
        } else if order_values && name.eq_ignore_ascii_case("columns") {
            PropertyContext::Columns
        } else if order_values && name.eq_ignore_ascii_case("grid-auto-flow") {
            PropertyContext::GridAutoFlow
        } else if order_values
            && (name.eq_ignore_ascii_case("grid-column-gap")
                || name.eq_ignore_ascii_case("grid-row-gap"))
        {
            PropertyContext::GridGap
        } else if order_values && is_grid_line(name) {
            PropertyContext::GridLine
        } else if order_values && name.eq_ignore_ascii_case("list-style") {
            PropertyContext::ListStyle
        } else if name.eq_ignore_ascii_case("display") {
            PropertyContext::Display
        } else if name.eq_ignore_ascii_case("font") || name.eq_ignore_ascii_case("font-family") {
            PropertyContext::Font
        } else if name.eq_ignore_ascii_case("font-weight") {
            PropertyContext::FontWeight
        } else if is_position_value(name) {
            PropertyContext::Position
        } else if name.eq_ignore_ascii_case("background-repeat")
            || name.eq_ignore_ascii_case("mask-repeat")
        {
            PropertyContext::Repeat
        } else if is_timing_function_value(name) {
            PropertyContext::TimingFunction
        } else if name
            .get(name.len().saturating_sub("transform".len())..)
            .is_some_and(|suffix| suffix.eq_ignore_ascii_case("transform"))
        {
            PropertyContext::Transform
        } else if is_box_value(name) {
            PropertyContext::Box
        } else {
            PropertyContext::Generic
        },
    }
}

fn is_grid_line(name: &str) -> bool {
    [
        "grid-column",
        "grid-row",
        "grid-column-start",
        "grid-column-end",
        "grid-row-start",
        "grid-row-end",
    ]
    .iter()
    .any(|property| name.eq_ignore_ascii_case(property))
}

fn is_animation_shorthand(name: &str) -> bool {
    name.eq_ignore_ascii_case("animation")
        || name
            .strip_prefix('-')
            .and_then(|name| name.split_once('-'))
            .is_some_and(|(_, name)| name.eq_ignore_ascii_case("animation"))
}

fn is_transition_shorthand(name: &str) -> bool {
    name.eq_ignore_ascii_case("transition")
        || name
            .strip_prefix('-')
            .and_then(|name| name.split_once('-'))
            .is_some_and(|(_, name)| name.eq_ignore_ascii_case("transition"))
}

fn is_ordered_border(name: &str) -> bool {
    [
        "border",
        "border-top",
        "border-right",
        "border-bottom",
        "border-left",
        "border-block-start",
        "border-block-end",
        "border-inline-start",
        "border-inline-end",
        "border-inline",
        "border-block",
    ]
    .iter()
    .any(|property| name.eq_ignore_ascii_case(property))
}

fn should_minify_colors(name: &str) -> bool {
    !(name.eq_ignore_ascii_case("src")
        || name.eq_ignore_ascii_case("-webkit-tap-highlight-color")
        || starts_with_ignore_ascii_case(name, "font")
        || starts_with_ignore_ascii_case(name, "filter")
        || starts_with_ignore_ascii_case(name, "composes"))
}

fn starts_with_ignore_ascii_case(value: &str, prefix: &str) -> bool {
    value
        .get(..prefix.len())
        .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
}

fn is_position_value(name: &str) -> bool {
    name.eq_ignore_ascii_case("background")
        || name.eq_ignore_ascii_case("background-position")
        || name.eq_ignore_ascii_case("perspective-origin")
        || name.strip_prefix('-').is_some_and(|prefixed| {
            prefixed.find('-').is_some_and(|separator| {
                prefixed[separator + 1..].eq_ignore_ascii_case("perspective-origin")
            })
        })
}

fn is_timing_function_value(name: &str) -> bool {
    const PROPERTIES: [&str; 4] = [
        "animation",
        "animation-timing-function",
        "transition",
        "transition-timing-function",
    ];

    PROPERTIES.into_iter().any(|property| {
        name.eq_ignore_ascii_case(property)
            || name.strip_prefix('-').is_some_and(|prefixed| {
                prefixed.find('-').is_some_and(|separator| {
                    prefixed[separator + 1..].eq_ignore_ascii_case(property)
                })
            })
    })
}

pub(crate) fn custom_property_context(context: &MinifyContext) -> ValueContext {
    ValueContext {
        allow_unitless_zero_length: false,
        allow_unitless_zero_percentage: false,
        minify_colors: true,
        property: PropertyContext::Generic,
        skip_value_transforms: !context.options().transform_custom_properties,
    }
}

fn zero_percentage_requires_target_support(name: &str) -> bool {
    [
        "border-image-width",
        "flex-basis",
        "height",
        "line-height",
        "max-height",
        "max-width",
        "min-height",
        "min-width",
        "stroke-dasharray",
        "stroke-dashoffset",
        "stroke-width",
        "width",
    ]
    .iter()
    .any(|property| name.eq_ignore_ascii_case(property))
}

fn is_box_value(name: &str) -> bool {
    matches!(
        name,
        "margin"
            | "padding"
            | "border-spacing"
            | "border-color"
            | "border-style"
            | "border-width"
            | "inset"
            | "scroll-margin"
            | "scroll-padding"
    )
}

pub(crate) fn initial_value(property: &str) -> Option<&'static str> {
    match property {
        "accent-color" => Some("auto"),
        "align-content" => Some("normal"),
        "align-items" => Some("normal"),
        "align-self" => Some("auto"),
        "align-tracks" => Some("normal"),
        "animation-delay" => Some("0s"),
        "animation-direction" => Some("normal"),
        "animation-duration" => Some("0s"),
        "animation-fill-mode" => Some("none"),
        "animation-iteration-count" => Some("1"),
        "animation-name" => Some("none"),
        "animation-range-end" => Some("normal"),
        "animation-range-start" => Some("normal"),
        "animation-timing-function" => Some("ease"),
        "animation-timeline" => Some("auto"),
        "appearance" => Some("none"),
        "aspect-ratio" => Some("auto"),
        "azimuth" => Some("center"),
        "backdrop-filter" => Some("none"),
        "background-attachment" => Some("scroll"),
        "background-blend-mode" => Some("normal"),
        "background-image" => Some("none"),
        "background-position-x" => Some("0%"),
        "background-position-y" => Some("0%"),
        "background-repeat" => Some("repeat"),
        "block-size" => Some("auto"),
        "border-block-style" => Some("none"),
        "border-block-width" => Some("medium"),
        "border-block-end-style" => Some("none"),
        "border-block-end-width" => Some("medium"),
        "border-block-start-style" => Some("none"),
        "border-block-start-width" => Some("medium"),
        "border-bottom-left-radius" => Some("0"),
        "border-bottom-right-radius" => Some("0"),
        "border-bottom-style" => Some("none"),
        "border-bottom-width" => Some("medium"),
        "border-end-end-radius" => Some("0"),
        "border-end-start-radius" => Some("0"),
        "border-image-outset" => Some("0"),
        "border-image-slice" => Some("100%"),
        "border-image-source" => Some("none"),
        "border-image-width" => Some("1"),
        "border-inline-style" => Some("none"),
        "border-inline-width" => Some("medium"),
        "border-inline-end-style" => Some("none"),
        "border-inline-end-width" => Some("medium"),
        "border-inline-start-style" => Some("none"),
        "border-inline-start-width" => Some("medium"),
        "border-left-style" => Some("none"),
        "border-left-width" => Some("medium"),
        "border-right-style" => Some("none"),
        "border-right-width" => Some("medium"),
        "border-spacing" => Some("0"),
        "border-start-end-radius" => Some("0"),
        "border-start-start-radius" => Some("0"),
        "border-top-left-radius" => Some("0"),
        "border-top-right-radius" => Some("0"),
        "border-top-style" => Some("none"),
        "border-top-width" => Some("medium"),
        "bottom" => Some("auto"),
        "box-decoration-break" => Some("slice"),
        "box-shadow" => Some("none"),
        "break-after" => Some("auto"),
        "break-before" => Some("auto"),
        "break-inside" => Some("auto"),
        "caption-side" => Some("top"),
        "caret-color" => Some("auto"),
        "caret-shape" => Some("auto"),
        "clear" => Some("none"),
        "clip" => Some("auto"),
        "clip-path" => Some("none"),
        "color-scheme" => Some("normal"),
        "column-count" => Some("auto"),
        "column-gap" => Some("normal"),
        "column-rule-style" => Some("none"),
        "column-rule-width" => Some("medium"),
        "column-span" => Some("none"),
        "column-width" => Some("auto"),
        "contain" => Some("none"),
        "contain-intrinsic-block-size" => Some("none"),
        "contain-intrinsic-height" => Some("none"),
        "contain-intrinsic-inline-size" => Some("none"),
        "contain-intrinsic-width" => Some("none"),
        "container-name" => Some("none"),
        "container-type" => Some("normal"),
        "content" => Some("normal"),
        "counter-increment" => Some("none"),
        "counter-reset" => Some("none"),
        "counter-set" => Some("none"),
        "cursor" => Some("auto"),
        "direction" => Some("ltr"),
        "empty-cells" => Some("show"),
        "filter" => Some("none"),
        "flex-basis" => Some("auto"),
        "flex-direction" => Some("row"),
        "flex-grow" => Some("0"),
        "flex-shrink" => Some("1"),
        "flex-wrap" => Some("nowrap"),
        "float" => Some("none"),
        "font-feature-settings" => Some("normal"),
        "font-kerning" => Some("auto"),
        "font-language-override" => Some("normal"),
        "font-optical-sizing" => Some("auto"),
        "font-palette" => Some("normal"),
        "font-variation-settings" => Some("normal"),
        "font-size" => Some("medium"),
        "font-size-adjust" => Some("none"),
        "font-stretch" => Some("normal"),
        "font-style" => Some("normal"),
        "font-synthesis-position" => Some("none"),
        "font-synthesis-small-caps" => Some("auto"),
        "font-synthesis-style" => Some("auto"),
        "font-synthesis-weight" => Some("auto"),
        "font-variant" => Some("normal"),
        "font-variant-alternates" => Some("normal"),
        "font-variant-caps" => Some("normal"),
        "font-variant-east-asian" => Some("normal"),
        "font-variant-emoji" => Some("normal"),
        "font-variant-ligatures" => Some("normal"),
        "font-variant-numeric" => Some("normal"),
        "font-variant-position" => Some("normal"),
        "font-weight" => Some("normal"),
        "forced-color-adjust" => Some("auto"),
        "grid-auto-columns" => Some("auto"),
        "grid-auto-flow" => Some("row"),
        "grid-auto-rows" => Some("auto"),
        "grid-column-end" => Some("auto"),
        "grid-column-gap" => Some("0"),
        "grid-column-start" => Some("auto"),
        "grid-row-end" => Some("auto"),
        "grid-row-gap" => Some("0"),
        "grid-row-start" => Some("auto"),
        "grid-template-areas" => Some("none"),
        "grid-template-columns" => Some("none"),
        "grid-template-rows" => Some("none"),
        "hanging-punctuation" => Some("none"),
        "height" => Some("auto"),
        "hyphenate-character" => Some("auto"),
        "hyphenate-limit-chars" => Some("auto"),
        "hyphens" => Some("manual"),
        "image-rendering" => Some("auto"),
        "image-resolution" => Some("1dppx"),
        "ime-mode" => Some("auto"),
        "initial-letter" => Some("normal"),
        "initial-letter-align" => Some("auto"),
        "inline-size" => Some("auto"),
        "input-security" => Some("auto"),
        "inset-block-end" => Some("auto"),
        "inset-block-start" => Some("auto"),
        "inset-inline-end" => Some("auto"),
        "inset-inline-start" => Some("auto"),
        "isolation" => Some("auto"),
        "justify-content" => Some("normal"),
        "justify-items" => Some("legacy"),
        "justify-self" => Some("auto"),
        "justify-tracks" => Some("normal"),
        "left" => Some("auto"),
        "letter-spacing" => Some("normal"),
        "line-break" => Some("auto"),
        "line-clamp" => Some("none"),
        "line-height" => Some("normal"),
        "line-height-step" => Some("0"),
        "list-style-image" => Some("none"),
        "list-style-type" => Some("disc"),
        "margin-block-end" => Some("0"),
        "margin-block-start" => Some("0"),
        "margin-bottom" => Some("0"),
        "margin-inline-end" => Some("0"),
        "margin-inline-start" => Some("0"),
        "margin-left" => Some("0"),
        "margin-right" => Some("0"),
        "margin-top" => Some("0"),
        "margin-trim" => Some("none"),
        "mask-border-mode" => Some("alpha"),
        "mask-border-outset" => Some("0"),
        "mask-border-slice" => Some("0"),
        "mask-border-source" => Some("none"),
        "mask-border-width" => Some("auto"),
        "mask-composite" => Some("add"),
        "mask-image" => Some("none"),
        "mask-repeat" => Some("repeat"),
        "mask-size" => Some("auto"),
        "masonry-auto-flow" => Some("pack"),
        "math-depth" => Some("0"),
        "math-shift" => Some("normal"),
        "math-style" => Some("normal"),
        "max-block-size" => Some("none"),
        "max-height" => Some("none"),
        "max-inline-size" => Some("none"),
        "max-lines" => Some("none"),
        "max-width" => Some("none"),
        "min-block-size" => Some("0"),
        "min-height" => Some("auto"),
        "min-inline-size" => Some("0"),
        "min-width" => Some("auto"),
        "mix-blend-mode" => Some("normal"),
        "object-fit" => Some("fill"),
        "offset-anchor" => Some("auto"),
        "offset-distance" => Some("0"),
        "offset-path" => Some("none"),
        "offset-position" => Some("normal"),
        "offset-rotate" => Some("auto"),
        "opacity" => Some("1"),
        "order" => Some("0"),
        "orphans" => Some("2"),
        "outline-offset" => Some("0"),
        "outline-style" => Some("none"),
        "outline-width" => Some("medium"),
        "overflow-anchor" => Some("auto"),
        "overflow-block" => Some("auto"),
        "overflow-clip-margin" => Some("0px"),
        "overflow-inline" => Some("auto"),
        "overflow-wrap" => Some("normal"),
        "overlay" => Some("none"),
        "overscroll-behavior" => Some("auto"),
        "overscroll-behavior-block" => Some("auto"),
        "overscroll-behavior-inline" => Some("auto"),
        "overscroll-behavior-x" => Some("auto"),
        "overscroll-behavior-y" => Some("auto"),
        "padding-block-end" => Some("0"),
        "padding-block-start" => Some("0"),
        "padding-bottom" => Some("0"),
        "padding-inline-end" => Some("0"),
        "padding-inline-start" => Some("0"),
        "padding-left" => Some("0"),
        "padding-right" => Some("0"),
        "padding-top" => Some("0"),
        "page" => Some("auto"),
        "page-break-after" => Some("auto"),
        "page-break-before" => Some("auto"),
        "page-break-inside" => Some("auto"),
        "paint-order" => Some("normal"),
        "perspective" => Some("none"),
        "pointer-events" => Some("auto"),
        "position" => Some("static"),
        "resize" => Some("none"),
        "right" => Some("auto"),
        "rotate" => Some("none"),
        "row-gap" => Some("normal"),
        "scale" => Some("none"),
        "scrollbar-color" => Some("auto"),
        "scrollbar-gutter" => Some("auto"),
        "scrollbar-width" => Some("auto"),
        "scroll-behavior" => Some("auto"),
        "scroll-margin-block-start" => Some("0"),
        "scroll-margin-block-end" => Some("0"),
        "scroll-margin-bottom" => Some("0"),
        "scroll-margin-inline-start" => Some("0"),
        "scroll-margin-inline-end" => Some("0"),
        "scroll-margin-left" => Some("0"),
        "scroll-margin-right" => Some("0"),
        "scroll-margin-top" => Some("0"),
        "scroll-padding-block-start" => Some("auto"),
        "scroll-padding-block-end" => Some("auto"),
        "scroll-padding-bottom" => Some("auto"),
        "scroll-padding-inline-start" => Some("auto"),
        "scroll-padding-inline-end" => Some("auto"),
        "scroll-padding-left" => Some("auto"),
        "scroll-padding-right" => Some("auto"),
        "scroll-padding-top" => Some("auto"),
        "scroll-snap-align" => Some("none"),
        "scroll-snap-coordinate" => Some("none"),
        "scroll-snap-points-x" => Some("none"),
        "scroll-snap-points-y" => Some("none"),
        "scroll-snap-stop" => Some("normal"),
        "scroll-snap-type" => Some("none"),
        "scroll-snap-type-x" => Some("none"),
        "scroll-snap-type-y" => Some("none"),
        "scroll-timeline-axis" => Some("block"),
        "scroll-timeline-name" => Some("none"),
        "shape-image-threshold" => Some("0.0"),
        "shape-margin" => Some("0"),
        "shape-outside" => Some("none"),
        "tab-size" => Some("8"),
        "table-layout" => Some("auto"),
        "text-align-last" => Some("auto"),
        "text-combine-upright" => Some("none"),
        "text-decoration-line" => Some("none"),
        "text-decoration-skip-ink" => Some("auto"),
        "text-decoration-style" => Some("solid"),
        "text-decoration-thickness" => Some("auto"),
        "text-emphasis-style" => Some("none"),
        "text-indent" => Some("0"),
        "text-justify" => Some("auto"),
        "text-orientation" => Some("mixed"),
        "text-overflow" => Some("clip"),
        "text-rendering" => Some("auto"),
        "text-shadow" => Some("none"),
        "text-transform" => Some("none"),
        "text-underline-offset" => Some("auto"),
        "text-underline-position" => Some("auto"),
        "text-wrap" => Some("wrap"),
        "timeline-scope" => Some("none"),
        "top" => Some("auto"),
        "touch-action" => Some("auto"),
        "transform" => Some("none"),
        "transform-style" => Some("flat"),
        "transition-behavior" => Some("normal"),
        "transition-delay" => Some("0s"),
        "transition-duration" => Some("0s"),
        "transition-property" => Some("all"),
        "transition-timing-function" => Some("ease"),
        "translate" => Some("none"),
        "unicode-bidi" => Some("normal"),
        "user-select" => Some("auto"),
        "view-timeline-axis" => Some("block"),
        "view-timeline-inset" => Some("auto"),
        "view-timeline-name" => Some("none"),
        "view-transition-name" => Some("none"),
        "white-space" => Some("normal"),
        "widows" => Some("2"),
        "width" => Some("auto"),
        "will-change" => Some("auto"),
        "word-break" => Some("normal"),
        "word-spacing" => Some("normal"),
        "word-wrap" => Some("normal"),
        "z-index" => Some("auto"),
        _ => None,
    }
}

pub(crate) fn is_initial_value(property: &str, value: &str) -> bool {
    match property {
        "background-clip" => value.eq_ignore_ascii_case("border-box"),
        "background-color" => value.eq_ignore_ascii_case("transparent"),
        "background-origin" => value.eq_ignore_ascii_case("padding-box"),
        "border-block-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-block-end-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-block-start-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-bottom-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-collapse" => value.eq_ignore_ascii_case("separate"),
        "border-inline-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-inline-end-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-inline-start-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-left-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-right-color" => value.eq_ignore_ascii_case("currentcolor"),
        "border-top-color" => value.eq_ignore_ascii_case("currentcolor"),
        "box-sizing" => value.eq_ignore_ascii_case("content-box"),
        "color" => value.eq_ignore_ascii_case("canvastext"),
        "column-rule-color" => value.eq_ignore_ascii_case("currentcolor"),
        "image-orientation" => value.eq_ignore_ascii_case("from-image"),
        "mask-clip" => value.eq_ignore_ascii_case("border-box"),
        "mask-mode" => value.eq_ignore_ascii_case("match-source"),
        "mask-origin" => value.eq_ignore_ascii_case("border-box"),
        "mask-type" => value.eq_ignore_ascii_case("luminance"),
        "ruby-align" => value.eq_ignore_ascii_case("space-around"),
        "ruby-merge" => value.eq_ignore_ascii_case("separate"),
        "ruby-position" => value.eq_ignore_ascii_case("alternate"),
        "text-decoration-color" => value.eq_ignore_ascii_case("currentcolor"),
        "text-emphasis-color" => value.eq_ignore_ascii_case("currentcolor"),
        "vertical-align" => value.eq_ignore_ascii_case("baseline"),
        "white-space-collapse" => value.eq_ignore_ascii_case("collapse"),
        _ => false,
    }
}
