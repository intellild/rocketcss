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

        let mut name = std::string::String::from(first);
        while !input.is_exhausted() {
            name.push(' ');
            name.push_str(input.expect_ident()?);
        }
        Ok(Self::Custom(allocator.alloc_str(&name)))
    }
}

pub(crate) fn parse_font_family_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Vec<'i, FontFamily<'i>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let parsed = input.parse_comma_separated(FontFamily::parse)?;
    let mut families = allocator.vec();
    families.extend(parsed);
    Ok(families)
}
