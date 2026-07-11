use rocketcss_ast::PropertyId;

use crate::{
    MinifyContext,
    context::{PropertyContext, ValueContext},
};

pub(crate) fn value_context(property_id: &PropertyId<'_>) -> ValueContext {
    let name = property_id.name();
    ValueContext {
        allow_unitless_zero: true,
        skip_value_transforms: false,
        property: if name.eq_ignore_ascii_case("font-weight") {
            PropertyContext::FontWeight
        } else if name.eq_ignore_ascii_case("background-repeat")
            || name.eq_ignore_ascii_case("mask-repeat")
        {
            PropertyContext::Repeat
        } else if is_box_value(name) {
            PropertyContext::Box
        } else {
            PropertyContext::Generic
        },
    }
}

pub(crate) fn custom_property_context(context: &MinifyContext) -> ValueContext {
    ValueContext {
        allow_unitless_zero: false,
        property: PropertyContext::Generic,
        skip_value_transforms: !context.options().transform_custom_properties,
    }
}

fn is_box_value(name: &str) -> bool {
    matches!(
        name,
        "margin"
            | "padding"
            | "border-color"
            | "border-style"
            | "border-width"
            | "inset"
            | "scroll-margin"
            | "scroll-padding"
    )
}
