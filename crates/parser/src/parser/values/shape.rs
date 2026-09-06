use crate::prelude::*;

impl<'i> Parse<'i> for GeometryBox {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "border-box" => Ok(Self::BorderBox),
            "padding-box" => Ok(Self::PaddingBox),
            "content-box" => Ok(Self::ContentBox),
            "margin-box" => Ok(Self::MarginBox),
            "fill-box" => Ok(Self::FillBox),
            "stroke-box" => Ok(Self::StrokeBox),
            "view-box" => Ok(Self::ViewBox),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}
