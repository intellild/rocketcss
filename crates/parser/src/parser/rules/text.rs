use crate::prelude::*;

impl<'i> Parse<'i> for TextTransform {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let case = TextTransformCase::parse(input)?;
        let mut full_width = false;
        let mut full_size_kana = false;
        while !input.is_exhausted() {
            let ident = input.expect_ident()?;
            match_ignore_ascii_case!(
                ident,
                "full-width" => full_width = true,
                "full-size-kana" => full_size_kana = true,
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }
        if matches!(case, TextTransformCase::None) && (full_width || full_size_kana) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self {
            case,
            full_size_kana,
            full_width,
        })
    }
}

impl<'i> Parse<'i> for TextIndent<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = store_node(LengthPercentage::parse(input)?, input);
        let mut each_line = false;
        let mut hanging = false;
        while !input.is_exhausted() {
            let ident = input.expect_ident()?;
            match_ignore_ascii_case!(
                ident,
                "each-line" => each_line = true,
                "hanging" => hanging = true,
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }
        Ok(Self {
            each_line,
            hanging,
            value,
        })
    }
}
