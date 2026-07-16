use crate::prelude::*;

impl<'i> Parse<'i> for FontFamily<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(name) = input.try_parse(Parser::expect_string) {
            input.expect_exhausted()?;
            return Ok(Self::Custom(name));
        }

        let allocator = input.allocator();
        let first = input.expect_ident()?;
        if input.is_exhausted() {
            return Ok(Self::from_name(first));
        }
        if !matches!(Self::from_name(first), Self::Custom(_)) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        let mut name = std::string::String::from(first);
        while !input.is_exhausted() {
            let part = input.expect_ident()?;
            if !matches!(Self::from_name(part), Self::Custom(_)) {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            name.push(' ');
            name.push_str(part);
        }
        Ok(Self::Custom(allocator.alloc_str(&name)))
    }
}

pub(crate) fn parse_font_family_list<'i, 't>(
    input: &mut Parser<'i, 't>,
    depth: usize,
) -> Result<Vec<'i, FontFamily<'i>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let mut families = allocator.vec();
    loop {
        let family = input.parse_until_before(Delimiter::Comma, |input| {
            if let Ok(family) = input.try_parse(FontFamily::parse) {
                return Ok(family);
            }
            super::collect_tokens(input, allocator, depth + 1).map(FontFamily::Unparsed)
        })?;
        families.push(family);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    Ok(families)
}
