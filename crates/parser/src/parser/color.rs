use crate::prelude::*;

impl<'i> Parse<'i> for CssColor<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        let token = input.next()?.clone();
        match token {
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("currentcolor") => {
                Ok(CssColor::CurrentColor)
            }
            ValueToken::Ident(name) => KnownColor::from_name(name)
                .map(CssColor::Known)
                .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue)),
            ValueToken::Hash(value) | ValueToken::IdHash(value) => parse_hex_color(value)
                .map(CssColor::Rgba)
                .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue)),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

pub(super) fn parse_hex_color(value: &str) -> Option<RGBA> {
    fn pair(value: &str) -> Option<u8> {
        u8::from_str_radix(value, 16).ok()
    }
    Some(match value.len() {
        3 | 4 => {
            let mut bytes = value.bytes().map(|byte| {
                let digit = (byte as char).to_digit(16)? as u8;
                Some(digit * 17)
            });
            RGBA {
                red: bytes.next()??,
                green: bytes.next()??,
                blue: bytes.next()??,
                alpha: match bytes.next() {
                    Some(value) => value?,
                    None => 255,
                },
            }
        }
        6 | 8 => RGBA {
            red: pair(&value[0..2])?,
            green: pair(&value[2..4])?,
            blue: pair(&value[4..6])?,
            alpha: if value.len() == 8 {
                pair(&value[6..8])?
            } else {
                255
            },
        },
        _ => return None,
    })
}
