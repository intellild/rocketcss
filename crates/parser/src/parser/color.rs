use super::values::single_token;
use crate::prelude::*;

pub(super) fn parse_color<'i>(
    value: &[TokenOrValue<'i>],
    _allocator: &'i Allocator,
) -> Option<CssColor<'i>> {
    let token = single_token(value)?;
    match token {
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("currentcolor") => {
            Some(CssColor::CurrentColor)
        }
        ValueToken::Ident(name) => named_color(name).map(CssColor::Rgba),
        ValueToken::Hash(value) | ValueToken::IdHash(value) => {
            parse_hex_color(value).map(CssColor::Rgba)
        }
        _ => None,
    }
}

pub(super) fn named_color(name: &str) -> Option<RGBA> {
    match_ignore_ascii_case!(
        name,
        "transparent" => Some(RGBA {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }),
        "black" => Some(rgba(0, 0, 0)),
        "silver" => Some(rgba(192, 192, 192)),
        "gray" => Some(rgba(128, 128, 128)),
        "white" => Some(rgba(255, 255, 255)),
        "maroon" => Some(rgba(128, 0, 0)),
        "red" => Some(rgba(255, 0, 0)),
        "purple" => Some(rgba(128, 0, 128)),
        "fuchsia" => Some(rgba(255, 0, 255)),
        "green" => Some(rgba(0, 128, 0)),
        "lime" => Some(rgba(0, 255, 0)),
        "olive" => Some(rgba(128, 128, 0)),
        "yellow" => Some(rgba(255, 255, 0)),
        "navy" => Some(rgba(0, 0, 128)),
        "blue" => Some(rgba(0, 0, 255)),
        "teal" => Some(rgba(0, 128, 128)),
        "aqua" => Some(rgba(0, 255, 255)),
        _ => None,
    )
}

const fn rgba(red: u8, green: u8, blue: u8) -> RGBA {
    RGBA {
        red,
        green,
        blue,
        alpha: 255,
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
