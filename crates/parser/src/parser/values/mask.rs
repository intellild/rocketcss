use crate::prelude::*;

impl<'i> Parse<'i> for MaskMode {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "luminance" => Ok(Self::Luminance),
            "alpha" => Ok(Self::Alpha),
            "match-source" => Ok(Self::MatchSource),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for MaskComposite {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "add" => Ok(Self::Add),
            "subtract" => Ok(Self::Subtract),
            "intersect" => Ok(Self::Intersect),
            "exclude" => Ok(Self::Exclude),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for WebKitMaskComposite {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "clear" => Ok(Self::Clear),
            "copy" => Ok(Self::Copy),
            "source-over" => Ok(Self::SourceOver),
            "source-in" => Ok(Self::SourceIn),
            "source-out" => Ok(Self::SourceOut),
            "source-atop" => Ok(Self::SourceAtop),
            "destination-over" => Ok(Self::DestinationOver),
            "destination-in" => Ok(Self::DestinationIn),
            "destination-out" => Ok(Self::DestinationOut),
            "destination-atop" => Ok(Self::DestinationAtop),
            "xor" => Ok(Self::Xor),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for MaskType {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "luminance" => Ok(Self::Luminance),
            "alpha" => Ok(Self::Alpha),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for MaskBorderMode {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        MaskType::parse(input).map(|value| match value {
            MaskType::Luminance => Self::Luminance,
            MaskType::Alpha => Self::Alpha,
        })
    }
}

impl<'i> Parse<'i> for WebKitMaskSourceType {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "auto" => Ok(Self::Auto),
            "luminance" => Ok(Self::Luminance),
            "alpha" => Ok(Self::Alpha),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for MaskClip {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("no-clip"))
            .is_ok()
        {
            return Ok(Self::NoClip);
        }
        GeometryBox::parse(input).map(Self::GeometryBox)
    }
}
