//! Multi-column parsing into the flattened AST.

use super::{
    border::{is_non_negative_length, is_non_negative_length_percentage},
    prelude::*,
};

impl<'i> Parse<'i> for ColumnRule<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
        let mut width = None;
        let mut style = None;
        let mut color = None;

        while !input.is_exhausted() {
            if width.is_none()
                && let Ok(value) = input.try_parse(BorderSideWidth::parse)
            {
                width = Some(value);
                continue;
            }
            if style.is_none()
                && let Ok(value) = input.try_parse(LineStyle::parse)
            {
                style = Some(value);
                continue;
            }
            if color.is_none()
                && let Ok(value) = input.try_parse(CssColor::parse)
            {
                color = Some(value);
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        if width.is_none() && style.is_none() && color.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(input.ast_mut().alloc_column_rule(
            ColumnRuleData {
                color,
                style,
                width,
            },
            span,
        ))
    }
}

impl<'i> Parse<'i> for ColumnWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(input
                .ast_mut()
                .alloc_column_width(ColumnWidthData::Auto, span));
        }
        let length = Length::parse(input)?;
        if !is_non_negative_length(&length, input.ast()) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(input
            .ast_mut()
            .alloc_column_width(ColumnWidthData::Length(length), span))
    }
}

impl<'i> Parse<'i> for ColumnCount {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        let value = input.expect_integer()?;
        if value < 1 {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::Integer(value))
    }
}

impl<'i> Parse<'i> for Columns<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
        let mut width = None;
        let mut count = None;
        let mut auto_count = 0u8;

        while !input.is_exhausted() {
            if width.is_none()
                && let Ok(value) = input.try_parse(Length::parse)
                && is_non_negative_length(&value, input.ast())
            {
                width = Some(
                    input
                        .ast_mut()
                        .alloc_column_width(ColumnWidthData::Length(value), span),
                );
                continue;
            }
            if count.is_none()
                && let Ok(value) = input.try_parse(Compiler::expect_integer)
                && value >= 1
            {
                count = Some(ColumnCount::Integer(value));
                continue;
            }
            if auto_count < 2
                && input
                    .try_parse(|input| input.expect_ident_matching("auto"))
                    .is_ok()
            {
                auto_count += 1;
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        let missing_components = u8::from(width.is_none()) + u8::from(count.is_none());
        if missing_components == 2 && auto_count == 0 || auto_count > missing_components {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let width = match width {
            Some(width) => width,
            None => input
                .ast_mut()
                .alloc_column_width(ColumnWidthData::Auto, span),
        };
        Ok(input.ast_mut().alloc_columns(
            ColumnsData {
                count: count.unwrap_or(ColumnCount::Auto),
                width,
            },
            span,
        ))
    }
}

impl<'i> Parse<'i> for GapValue<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
        if input
            .try_parse(|input| input.expect_ident_matching("normal"))
            .is_ok()
        {
            return Ok(input.ast_mut().alloc_gap_value(GapValueData::Normal, span));
        }
        let value = LengthPercentage::parse(input)?;
        if !is_non_negative_length_percentage(&value, input.ast()) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(input
            .ast_mut()
            .alloc_gap_value(GapValueData::LengthPercentage(value), span))
    }
}

impl<'i> Parse<'i> for Gap<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
        let first_state = input.state();
        let row = GapValue::parse(input)?;
        let column = if input.is_exhausted() {
            input.reset(&first_state);
            GapValue::parse(input)?
        } else {
            GapValue::parse(input)?
        };
        Ok(input.ast_mut().alloc_gap(GapData { column, row }, span))
    }
}
