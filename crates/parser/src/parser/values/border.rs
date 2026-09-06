use crate::parser::length::is_non_negative_length;
use crate::prelude::*;

impl<'i> Parse<'i> for OutlineStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        LineStyle::parse(input).map(Self::LineStyle)
    }
}

impl<'i> Parse<'i> for LineStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
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
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
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
        Ok(Self::Length(store_node(length, input)))
    }
}
