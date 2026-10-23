//! Length, percentage, and size parsing into the flattened AST.

use super::{
    prelude::*,
    values::{collect_tokens, token_values_contain_opaque},
};

impl<'i> Parse<'i> for Length<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        let span = token_span(input);
        match input.next()?.clone() {
            OldToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                let length = input
                    .ast_mut()
                    .alloc_length_value(LengthValueData { unit, value }, span);
                Ok(input
                    .ast_mut()
                    .alloc_length(LengthData::Value(length), span))
            }
            OldToken::Number(0.0) => {
                let length = input.ast_mut().alloc_length_value(
                    LengthValueData {
                        unit: LengthUnit::Px,
                        value: 0.0,
                    },
                    span,
                );
                Ok(input
                    .ast_mut()
                    .alloc_length(LengthData::Value(length), span))
            }
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for LengthPercentage<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        let span = token_span(input);
        match input.next()?.clone() {
            OldToken::Percentage(value) => Ok(input
                .ast_mut()
                .alloc_dimension_percentage(DimensionPercentageData::Percentage(value), span)),
            OldToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                let length = input
                    .ast_mut()
                    .alloc_length_value(LengthValueData { unit, value }, span);
                Ok(input
                    .ast_mut()
                    .alloc_dimension_percentage(DimensionPercentageData::Dimension(length), span))
            }
            OldToken::Number(0.0) => Ok(input
                .ast_mut()
                .alloc_dimension_percentage(DimensionPercentageData::Zero, span)),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for Size<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        let span = token_span(input);
        let token = input.next()?.clone();
        match token {
            OldToken::Ident(name) if name.eq_ignore_ascii_case("auto") => {
                Ok(input.ast_mut().alloc_size(SizeData::Auto, span))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("min-content") => {
                Ok(input.ast_mut().alloc_size(
                    SizeData::MinContent {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                ))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("max-content") => {
                Ok(input.ast_mut().alloc_size(
                    SizeData::MaxContent {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                ))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("fit-content") => {
                Ok(input.ast_mut().alloc_size(
                    SizeData::FitContent {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                ))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("stretch") => {
                Ok(input.ast_mut().alloc_size(
                    SizeData::Stretch {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                ))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("contain") => {
                Ok(input.ast_mut().alloc_size(SizeData::Contain, span))
            }
            OldToken::Function(name) if name.eq_ignore_ascii_case("fit-content") => {
                let value = input.parse_nested_block(|input| {
                    let value = <LengthPercentage>::parse(input)?;
                    input.expect_exhausted()?;
                    Ok(value)
                })?;
                Ok(input
                    .ast_mut()
                    .alloc_size(SizeData::FitContentFunction(value), span))
            }
            OldToken::Function(name) if KnownFunction::from_name(name).is_math() => {
                let arguments = input.parse_nested_block(|input| collect_tokens(input, 1))?;
                if token_values_contain_opaque(input.ast(), arguments) {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                let function = Function::new(name, arguments, input.ast_mut(), span);
                Ok(input
                    .ast_mut()
                    .alloc_size(SizeData::MathFunction(function), span))
            }
            OldToken::Percentage(value) => {
                let value = input
                    .ast_mut()
                    .alloc_dimension_percentage(DimensionPercentageData::Percentage(value), span);
                Ok(input
                    .ast_mut()
                    .alloc_size(SizeData::LengthPercentage(value), span))
            }
            OldToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                let length = input
                    .ast_mut()
                    .alloc_length_value(LengthValueData { unit, value }, span);
                let value = input
                    .ast_mut()
                    .alloc_dimension_percentage(DimensionPercentageData::Dimension(length), span);
                Ok(input
                    .ast_mut()
                    .alloc_size(SizeData::LengthPercentage(value), span))
            }
            OldToken::Number(0.0) => {
                let length = input.ast_mut().alloc_length_value(
                    LengthValueData {
                        unit: LengthUnit::Px,
                        value: 0.0,
                    },
                    span,
                );
                let value = input
                    .ast_mut()
                    .alloc_dimension_percentage(DimensionPercentageData::Dimension(length), span);
                Ok(input
                    .ast_mut()
                    .alloc_size(SizeData::LengthPercentage(value), span))
            }
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for MaxSize<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        let span = token_span(input);
        let token = input.next()?.clone();
        match token {
            OldToken::Ident(name) if name.eq_ignore_ascii_case("none") => {
                Ok(input.ast_mut().alloc_max_size(MaxSizeData::None, span))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("min-content") => {
                Ok(input.ast_mut().alloc_max_size(
                    MaxSizeData::MinContent {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                ))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("max-content") => {
                Ok(input.ast_mut().alloc_max_size(
                    MaxSizeData::MaxContent {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                ))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("fit-content") => {
                Ok(input.ast_mut().alloc_max_size(
                    MaxSizeData::FitContent {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                ))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("stretch") => {
                Ok(input.ast_mut().alloc_max_size(
                    MaxSizeData::Stretch {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                ))
            }
            OldToken::Ident(name) if name.eq_ignore_ascii_case("contain") => {
                Ok(input.ast_mut().alloc_max_size(MaxSizeData::Contain, span))
            }
            OldToken::Function(name) if name.eq_ignore_ascii_case("fit-content") => {
                let value = input.parse_nested_block(|input| {
                    let value = <LengthPercentage>::parse(input)?;
                    input.expect_exhausted()?;
                    Ok(value)
                })?;
                Ok(input
                    .ast_mut()
                    .alloc_max_size(MaxSizeData::FitContentFunction(value), span))
            }
            OldToken::Function(name) if KnownFunction::from_name(name).is_math() => {
                let arguments = input.parse_nested_block(|input| collect_tokens(input, 1))?;
                if token_values_contain_opaque(input.ast(), arguments) {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                let function = Function::new(name, arguments, input.ast_mut(), span);
                Ok(input
                    .ast_mut()
                    .alloc_max_size(MaxSizeData::MathFunction(function), span))
            }
            OldToken::Percentage(value) => {
                let value = input
                    .ast_mut()
                    .alloc_dimension_percentage(DimensionPercentageData::Percentage(value), span);
                Ok(input
                    .ast_mut()
                    .alloc_max_size(MaxSizeData::LengthPercentage(value), span))
            }
            OldToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                let length = input
                    .ast_mut()
                    .alloc_length_value(LengthValueData { unit, value }, span);
                let value = input
                    .ast_mut()
                    .alloc_dimension_percentage(DimensionPercentageData::Dimension(length), span);
                Ok(input
                    .ast_mut()
                    .alloc_max_size(MaxSizeData::LengthPercentage(value), span))
            }
            OldToken::Number(0.0) => {
                let length = input.ast_mut().alloc_length_value(
                    LengthValueData {
                        unit: LengthUnit::Px,
                        value: 0.0,
                    },
                    span,
                );
                let value = input
                    .ast_mut()
                    .alloc_dimension_percentage(DimensionPercentageData::Dimension(length), span);
                Ok(input
                    .ast_mut()
                    .alloc_max_size(MaxSizeData::LengthPercentage(value), span))
            }
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

pub(super) fn parse_length_unit(unit: &OldUnit) -> Option<LengthUnit> {
    unit.length().map(super::token::convert_length_unit)
}

impl<'i> Parse<'i> for Angle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        match input.next()?.clone() {
            OldToken::Number(0.0) => Ok(Self::Deg(0.0)),
            OldToken::Dimension { unit, value } => match unit {
                OldUnit::Deg => Ok(Self::Deg(value)),
                OldUnit::Rad => Ok(Self::Rad(value)),
                OldUnit::Grad => Ok(Self::Grad(value)),
                OldUnit::Turn => Ok(Self::Turn(value)),
                _ => Err(input.new_custom_error(ParserError::InvalidValue)),
            },
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        }
    }
}
