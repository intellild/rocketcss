use super::values::collect_tokens;
use crate::prelude::*;

impl<'i> Parse<'i> for CssColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        let token = input.next()?.clone();
        match token {
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("currentcolor") => {
                Ok(CssColor::CurrentColor)
            }
            ValueToken::Ident(name) => KnownColor::from_name(&name)
                .map(CssColor::Known)
                .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue)),
            ValueToken::Hash(value) | ValueToken::IdHash(value) => parse_hex_color(&value)
                .map(CssColor::Rgba)
                .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue)),
            ValueToken::Function(name) if KnownFunction::from_name(&name).is_color() => {
                let arguments = input.parse_nested_block(|input| collect_tokens(input, 1))?;
                let mut function = Function::new(name, arguments);
                if matches!(function.kind(), KnownFunction::Rgb | KnownFunction::Rgba) {
                    if !is_supported_rgb_function(&function) {
                        return Err(location.new_custom_error(ParserError::InvalidValue));
                    }
                    function.set_valid_rgb(true);
                }
                Ok(CssColor::Function(std::boxed::Box::new(function)))
            }
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

pub(super) fn validate_rgb_function(function: &mut Function<'_>) {
    if matches!(function.kind(), KnownFunction::Rgb | KnownFunction::Rgba) {
        function.set_valid_rgb(is_supported_rgb_function(function));
    }
}

fn is_supported_rgb_function(function: &Function<'_>) -> bool {
    let mut components = function.arguments.iter().filter(|value| {
        !matches!(
            value,
            TokenOrValue::Token(token) if matches!(**token, ValueToken::WhiteSpace(_))
        )
    });
    let Some(first) = components.next() else {
        return false;
    };
    let Some(second) = components.next() else {
        return false;
    };

    let has_alpha = if is_comma(second) {
        validate_legacy_rgb(first, &mut components)
    } else {
        validate_modern_rgb(first, second, &mut components)
    };
    has_alpha.is_some()
}

fn validate_legacy_rgb<'a>(
    first: &'a TokenOrValue<'a>,
    components: &mut impl Iterator<Item = &'a TokenOrValue<'a>>,
) -> Option<bool> {
    let first_kind = rgb_component_kind(first)?;
    let second = components.next()?;
    let second_comma = components.next()?;
    let third = components.next()?;
    if rgb_component_kind(second) != Some(first_kind)
        || !is_comma(second_comma)
        || rgb_component_kind(third) != Some(first_kind)
    {
        return None;
    }

    match components.next() {
        None => Some(false),
        Some(comma) if is_comma(comma) => {
            let alpha = components.next()?;
            (is_rgb_alpha(alpha) && components.next().is_none()).then_some(true)
        }
        Some(_) => None,
    }
}

fn validate_modern_rgb<'a>(
    first: &'a TokenOrValue<'a>,
    second: &'a TokenOrValue<'a>,
    components: &mut impl Iterator<Item = &'a TokenOrValue<'a>>,
) -> Option<bool> {
    if !is_modern_rgb_component(first) || !is_modern_rgb_component(second) {
        return None;
    }
    let third = components.next()?;
    if !is_modern_rgb_component(third) {
        return None;
    }

    match components.next() {
        None => Some(false),
        Some(slash) if is_slash(slash) => {
            let alpha = components.next()?;
            (is_modern_rgb_alpha(alpha) && components.next().is_none()).then_some(true)
        }
        Some(_) => None,
    }
}

fn is_modern_rgb_component(value: &TokenOrValue<'_>) -> bool {
    rgb_component_kind(value).is_some() || is_none_keyword(value)
}

fn is_modern_rgb_alpha(value: &TokenOrValue<'_>) -> bool {
    is_rgb_alpha(value) || is_none_keyword(value)
}

fn is_none_keyword(value: &TokenOrValue<'_>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token)
            if matches!(
                &**token,
                ValueToken::Ident(value) if value.eq_ignore_ascii_case("none")
            )
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RgbComponentKind {
    Number,
    Percentage,
}

fn rgb_component_kind(value: &TokenOrValue<'_>) -> Option<RgbComponentKind> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        ValueToken::Number(_) => Some(RgbComponentKind::Number),
        ValueToken::Percentage(_) => Some(RgbComponentKind::Percentage),
        _ => None,
    }
}

fn is_rgb_alpha(value: &TokenOrValue<'_>) -> bool {
    let TokenOrValue::Token(token) = value else {
        return false;
    };
    matches!(**token, ValueToken::Number(_) | ValueToken::Percentage(_))
}

fn is_comma(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, ValueToken::Comma))
}

fn is_slash(value: &TokenOrValue<'_>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token) if matches!(**token, ValueToken::Delim("/"))
    )
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
