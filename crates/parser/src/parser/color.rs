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
    Some(if name.eq_ignore_ascii_case("transparent") {
        RGBA {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    } else if name.eq_ignore_ascii_case("black") {
        rgba(0, 0, 0)
    } else if name.eq_ignore_ascii_case("silver") {
        rgba(192, 192, 192)
    } else if name.eq_ignore_ascii_case("gray") {
        rgba(128, 128, 128)
    } else if name.eq_ignore_ascii_case("white") {
        rgba(255, 255, 255)
    } else if name.eq_ignore_ascii_case("maroon") {
        rgba(128, 0, 0)
    } else if name.eq_ignore_ascii_case("red") {
        rgba(255, 0, 0)
    } else if name.eq_ignore_ascii_case("purple") {
        rgba(128, 0, 128)
    } else if name.eq_ignore_ascii_case("fuchsia") {
        rgba(255, 0, 255)
    } else if name.eq_ignore_ascii_case("green") {
        rgba(0, 128, 0)
    } else if name.eq_ignore_ascii_case("lime") {
        rgba(0, 255, 0)
    } else if name.eq_ignore_ascii_case("olive") {
        rgba(128, 128, 0)
    } else if name.eq_ignore_ascii_case("yellow") {
        rgba(255, 255, 0)
    } else if name.eq_ignore_ascii_case("navy") {
        rgba(0, 0, 128)
    } else if name.eq_ignore_ascii_case("blue") {
        rgba(0, 0, 255)
    } else if name.eq_ignore_ascii_case("teal") {
        rgba(0, 128, 128)
    } else if name.eq_ignore_ascii_case("aqua") {
        rgba(0, 255, 255)
    } else {
        return None;
    })
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
