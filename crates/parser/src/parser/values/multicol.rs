use crate::prelude::*;

impl<'i> Parse<'i> for LineStyle {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "none" => Ok(Self::None),
            "hidden" => Ok(Self::Hidden),
            "inset" => Ok(Self::Inset),
            "groove" => Ok(Self::Groove),
            "outset" => Ok(Self::Outset),
            "ridge" => Ok(Self::Ridge),
            "dotted" => Ok(Self::Dotted),
            "dashed" => Ok(Self::Dashed),
            "solid" => Ok(Self::Solid),
            "double" => Ok(Self::Double),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for BorderSideWidth<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        if let Ok(ident) = input.try_parse(Parser::expect_ident) {
            return match_ignore_ascii_case!(
                ident,
                "thin" => Ok(Self::Thin),
                "medium" => Ok(Self::Medium),
                "thick" => Ok(Self::Thick),
                _ => Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }
        let length = Length::parse(input)?;
        if !is_non_negative_length(&length) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::Length(allocator.boxed(length)))
    }
}

impl<'i> Parse<'i> for ColumnRule<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let mut width = None;
        let mut style = None;
        let mut color = None;

        while !input.is_exhausted() {
            if width.is_none()
                && let Ok(value) = input.try_parse(BorderSideWidth::parse)
            {
                width = Some(allocator.boxed(value));
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
                color = Some(allocator.boxed(value));
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        if width.is_none() && style.is_none() && color.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self {
            color,
            style,
            width,
        })
    }
}

impl<'i> Parse<'i> for ColumnWidth<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        let allocator = input.allocator();
        let length = Length::parse(input)?;
        if !is_non_negative_length(&length) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::Length(allocator.boxed(length)))
    }
}

impl<'i> Parse<'i> for ColumnCount {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
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
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let mut width = None;
        let mut count = None;
        let mut auto_count = 0u8;

        while !input.is_exhausted() {
            if width.is_none()
                && let Ok(value) = input.try_parse(Length::parse)
                && is_non_negative_length(&value)
            {
                width = Some(ColumnWidth::Length(allocator.boxed(value)));
                continue;
            }
            if count.is_none()
                && let Ok(value) = input.try_parse(Parser::expect_integer)
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
        Ok(Self {
            count: count.unwrap_or(ColumnCount::Auto),
            width: width.unwrap_or(ColumnWidth::Auto),
        })
    }
}

impl<'i> Parse<'i> for GapValue<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("normal"))
            .is_ok()
        {
            return Ok(Self::Normal);
        }
        let allocator = input.allocator();
        let value = LengthPercentage::parse(input)?;
        if !is_non_negative_length_percentage(&value) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::LengthPercentage(allocator.boxed(value)))
    }
}

fn is_non_negative_length(value: &Length<'_>) -> bool {
    match value {
        Length::Value(value) => value.value >= 0.0,
        Length::Calc(_) => true,
    }
}

fn is_non_negative_length_percentage(value: &LengthPercentage<'_>) -> bool {
    match value {
        LengthPercentage::Dimension(value) => value.value >= 0.0,
        LengthPercentage::Percentage(value) => *value >= 0.0,
        LengthPercentage::Zero | LengthPercentage::Calc(_) => true,
    }
}
