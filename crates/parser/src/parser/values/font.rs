use crate::prelude::*;

impl<'i> Parse<'i> for FontFamily<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(name) = input.try_parse(Compiler::expect_string) {
            input.expect_exhausted()?;
            return Ok(Self::Custom(name));
        }

        let first = input.expect_ident()?;
        let first_atom = first.clone();
        if input.is_exhausted() {
            return Ok(Self::from_name(first_atom));
        }
        if !matches!(Self::from_name(first_atom.clone()), Self::Custom(_)) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        let mut name = std::string::String::from(first.as_str());
        while !input.is_exhausted() {
            let part = input.expect_ident()?;
            if !matches!(Self::from_name(part.clone()), Self::Custom(_)) {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            name.push(' ');
            name.push_str(&part);
        }
        Ok(Self::Custom(input.intern(&name)))
    }
}

pub(crate) fn parse_font_family_list<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
) -> Result<std::vec::Vec<FontFamily<'i>>, ParseError<'i, ParserError<'i>>> {
    let mut families = std::vec::Vec::new();
    loop {
        let family = input.parse_until_before(Delimiter::Comma, |input| {
            if let Ok(family) = input.try_parse(FontFamily::parse) {
                return Ok(family);
            }
            super::collect_tokens(input, depth + 1).map(FontFamily::Unparsed)
        })?;
        families.push(family);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    Ok(families)
}
