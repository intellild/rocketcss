use super::values::single_token;
use crate::prelude::*;

pub(super) fn parse_size<'i>(
    value: &[TokenOrValue<'i>],
    allocator: &'i Allocator,
) -> Option<Size<'i>> {
    let token = single_token(value)?;
    match token {
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("auto") => Some(Size::Auto),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("min-content") => {
            Some(Size::MinContent {
                vendor_prefix: VendorPrefix::NONE,
            })
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("max-content") => {
            Some(Size::MaxContent {
                vendor_prefix: VendorPrefix::NONE,
            })
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("fit-content") => {
            Some(Size::FitContent {
                vendor_prefix: VendorPrefix::NONE,
            })
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("stretch") => Some(Size::Stretch {
            vendor_prefix: VendorPrefix::NONE,
        }),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("contain") => Some(Size::Contain),
        ValueToken::Percentage(value) => Some(Size::LengthPercentage(
            allocator.boxed(DimensionPercentage::Percentage(*value)),
        )),
        ValueToken::Dimension { unit, value } => {
            let unit = parse_length_unit(unit)?;
            Some(Size::LengthPercentage(allocator.boxed(
                DimensionPercentage::Dimension(allocator.boxed(LengthValue {
                    unit,
                    value: *value,
                })),
            )))
        }
        ValueToken::Number(value) if *value == 0.0 => {
            Some(Size::LengthPercentage(allocator.boxed(
                DimensionPercentage::Dimension(allocator.boxed(LengthValue {
                    unit: LengthUnit::Px,
                    value: 0.0,
                })),
            )))
        }
        _ => None,
    }
}

pub(super) fn parse_length_unit(unit: &str) -> Option<LengthUnit> {
    Some(if unit.eq_ignore_ascii_case("px") {
        LengthUnit::Px
    } else if unit.eq_ignore_ascii_case("in") {
        LengthUnit::In
    } else if unit.eq_ignore_ascii_case("cm") {
        LengthUnit::Cm
    } else if unit.eq_ignore_ascii_case("mm") {
        LengthUnit::Mm
    } else if unit.eq_ignore_ascii_case("q") {
        LengthUnit::Q
    } else if unit.eq_ignore_ascii_case("pt") {
        LengthUnit::Pt
    } else if unit.eq_ignore_ascii_case("pc") {
        LengthUnit::Pc
    } else if unit.eq_ignore_ascii_case("em") {
        LengthUnit::Em
    } else if unit.eq_ignore_ascii_case("rem") {
        LengthUnit::Rem
    } else if unit.eq_ignore_ascii_case("ex") {
        LengthUnit::Ex
    } else if unit.eq_ignore_ascii_case("ch") {
        LengthUnit::Ch
    } else if unit.eq_ignore_ascii_case("lh") {
        LengthUnit::Lh
    } else if unit.eq_ignore_ascii_case("rlh") {
        LengthUnit::Rlh
    } else if unit.eq_ignore_ascii_case("vw") {
        LengthUnit::Vw
    } else if unit.eq_ignore_ascii_case("vh") {
        LengthUnit::Vh
    } else if unit.eq_ignore_ascii_case("vmin") {
        LengthUnit::Vmin
    } else if unit.eq_ignore_ascii_case("vmax") {
        LengthUnit::Vmax
    } else {
        return None;
    })
}
