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

pub(super) const fn parse_length_unit(unit: &Unit) -> Option<LengthUnit> {
    unit.length()
}

pub(super) fn parse_length_unit_name(unit: &str) -> Option<LengthUnit> {
    match_ignore_ascii_case!(
        unit,
        "px" => Some(LengthUnit::Px),
        "in" => Some(LengthUnit::In),
        "cm" => Some(LengthUnit::Cm),
        "mm" => Some(LengthUnit::Mm),
        "q" => Some(LengthUnit::Q),
        "pt" => Some(LengthUnit::Pt),
        "pc" => Some(LengthUnit::Pc),
        "em" => Some(LengthUnit::Em),
        "rem" => Some(LengthUnit::Rem),
        "ex" => Some(LengthUnit::Ex),
        "rex" => Some(LengthUnit::Rex),
        "ch" => Some(LengthUnit::Ch),
        "rch" => Some(LengthUnit::Rch),
        "cap" => Some(LengthUnit::Cap),
        "rcap" => Some(LengthUnit::Rcap),
        "ic" => Some(LengthUnit::Ic),
        "ric" => Some(LengthUnit::Ric),
        "lh" => Some(LengthUnit::Lh),
        "rlh" => Some(LengthUnit::Rlh),
        "vw" => Some(LengthUnit::Vw),
        "lvw" => Some(LengthUnit::Lvw),
        "svw" => Some(LengthUnit::Svw),
        "dvw" => Some(LengthUnit::Dvw),
        "cqw" => Some(LengthUnit::Cqw),
        "vh" => Some(LengthUnit::Vh),
        "lvh" => Some(LengthUnit::Lvh),
        "svh" => Some(LengthUnit::Svh),
        "dvh" => Some(LengthUnit::Dvh),
        "cqh" => Some(LengthUnit::Cqh),
        "vi" => Some(LengthUnit::Vi),
        "svi" => Some(LengthUnit::Svi),
        "lvi" => Some(LengthUnit::Lvi),
        "dvi" => Some(LengthUnit::Dvi),
        "cqi" => Some(LengthUnit::Cqi),
        "vb" => Some(LengthUnit::Vb),
        "svb" => Some(LengthUnit::Svb),
        "lvb" => Some(LengthUnit::Lvb),
        "dvb" => Some(LengthUnit::Dvb),
        "cqb" => Some(LengthUnit::Cqb),
        "vmin" => Some(LengthUnit::Vmin),
        "svmin" => Some(LengthUnit::Svmin),
        "lvmin" => Some(LengthUnit::Lvmin),
        "dvmin" => Some(LengthUnit::Dvmin),
        "cqmin" => Some(LengthUnit::Cqmin),
        "vmax" => Some(LengthUnit::Vmax),
        "svmax" => Some(LengthUnit::Svmax),
        "lvmax" => Some(LengthUnit::Lvmax),
        "dvmax" => Some(LengthUnit::Dvmax),
        "cqmax" => Some(LengthUnit::Cqmax),
        _ => None,
    )
}
