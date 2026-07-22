use crate::prelude::*;

impl<'i> Parse<'i> for Length<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        match input.next()?.clone() {
            ValueToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                Ok(Self::Value(LengthValue { unit, value }))
            }
            ValueToken::Number(0.0) => Ok(Self::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 0.0,
            })),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for LengthPercentage<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        match input.next()?.clone() {
            ValueToken::Percentage(value) => Ok(Self::Percentage(value)),
            ValueToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                Ok(Self::Dimension(LengthValue { unit, value }))
            }
            ValueToken::Number(0.0) => Ok(Self::Zero),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for Size<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let location = input.current_source_location();
        let token = input.next()?.clone();
        match token {
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("auto") => Ok(Size::Auto),
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("min-content") => {
                Ok(Size::MinContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("max-content") => {
                Ok(Size::MaxContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("fit-content") => {
                Ok(Size::FitContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("stretch") => Ok(Size::Stretch {
                vendor_prefix: VendorPrefix::NONE,
            }),
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("contain") => Ok(Size::Contain),
            ValueToken::Percentage(value) => Ok(Size::LengthPercentage(
                allocator.boxed(DimensionPercentage::Percentage(value)),
            )),
            ValueToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                Ok(Size::LengthPercentage(allocator.boxed(
                    DimensionPercentage::Dimension(LengthValue { unit, value }),
                )))
            }
            ValueToken::Number(0.0) => Ok(Size::LengthPercentage(allocator.boxed(
                DimensionPercentage::Dimension(LengthValue {
                    unit: LengthUnit::Px,
                    value: 0.0,
                }),
            ))),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
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
