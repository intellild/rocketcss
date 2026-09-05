use super::stylesheet::span_from;
use super::values::{collect_tokens, token_values_contain_opaque};
use crate::prelude::*;

pub(crate) fn parse_css_color<'i>(
    input: &mut Compiler<'i>,
) -> Result<NodeId<'i, CssColor<'i>>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let color = CssColor::parse(input)?;
    let span = span_from(&start, input.position());
    Ok(input.ast_context_mut().alloc_node(color, span))
}

impl<'i> Parse<'i> for CssColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
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
            ValueToken::Function(name) if KnownFunction::from_name(name).is_color() => {
                let allocator = input.allocator();
                let arguments =
                    input.parse_nested_block(|input| collect_tokens(input, allocator, 1))?;
                if token_values_contain_opaque(input.ast_context(), &arguments) {
                    return Err(location.new_custom_error(ParserError::InvalidValue));
                }
                let arguments = store_vec(arguments, input);
                let mut function = Function::new(name, arguments);
                if matches!(function.kind(), KnownFunction::Rgb | KnownFunction::Rgba) {
                    if !is_supported_rgb_function(input.ast_context(), &function) {
                        return Err(location.new_custom_error(ParserError::InvalidValue));
                    }
                    function.set_valid_rgb(true);
                }
                Ok(CssColor::Function(store_node(function, input)))
            }
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

pub(super) fn validate_rgb_function<'i>(ast: &Compilation<'i>, function: &mut Function<'i>) {
    if matches!(function.kind(), KnownFunction::Rgb | KnownFunction::Rgba) {
        function.set_valid_rgb(is_supported_rgb_function(ast, function));
    }
}

fn is_supported_rgb_function<'i>(ast: &Compilation<'i>, function: &Function<'i>) -> bool {
    let mut components = ast.vec(function.arguments).iter().filter(|value| {
        !matches!(
            value,
            TokenOrValue::Token(token) if matches!(ast.node(*token), ValueToken::WhiteSpace(_))
        )
    });
    let Some(first) = components.next() else {
        return false;
    };
    let Some(second) = components.next() else {
        return false;
    };

    let has_alpha = if is_comma(ast, second) {
        validate_legacy_rgb(ast, first, &mut components)
    } else {
        validate_modern_rgb(ast, first, second, &mut components)
    };
    has_alpha.is_some()
}

fn validate_legacy_rgb<'a, 'i>(
    ast: &Compilation<'i>,
    first: &'a TokenOrValue<'i>,
    components: &mut impl Iterator<Item = &'a TokenOrValue<'i>>,
) -> Option<bool> {
    let first_kind = rgb_component_kind(ast, first)?;
    let second = components.next()?;
    let second_comma = components.next()?;
    let third = components.next()?;
    if rgb_component_kind(ast, second) != Some(first_kind)
        || !is_comma(ast, second_comma)
        || rgb_component_kind(ast, third) != Some(first_kind)
    {
        return None;
    }

    match components.next() {
        None => Some(false),
        Some(comma) if is_comma(ast, comma) => {
            let alpha = components.next()?;
            (is_rgb_alpha(ast, alpha) && components.next().is_none()).then_some(true)
        }
        Some(_) => None,
    }
}

fn validate_modern_rgb<'a, 'i>(
    ast: &Compilation<'i>,
    first: &'a TokenOrValue<'i>,
    second: &'a TokenOrValue<'i>,
    components: &mut impl Iterator<Item = &'a TokenOrValue<'i>>,
) -> Option<bool> {
    if !is_modern_rgb_component(ast, first) || !is_modern_rgb_component(ast, second) {
        return None;
    }
    let third = components.next()?;
    if !is_modern_rgb_component(ast, third) {
        return None;
    }

    match components.next() {
        None => Some(false),
        Some(slash) if is_slash(ast, slash) => {
            let alpha = components.next()?;
            (is_modern_rgb_alpha(ast, alpha) && components.next().is_none()).then_some(true)
        }
        Some(_) => None,
    }
}

fn is_modern_rgb_component<'i>(ast: &Compilation<'i>, value: &TokenOrValue<'i>) -> bool {
    rgb_component_kind(ast, value).is_some() || is_none_keyword(ast, value)
}

fn is_modern_rgb_alpha<'i>(ast: &Compilation<'i>, value: &TokenOrValue<'i>) -> bool {
    is_rgb_alpha(ast, value) || is_none_keyword(ast, value)
}

fn is_none_keyword<'i>(ast: &Compilation<'i>, value: &TokenOrValue<'i>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token)
            if matches!(
                ast.node(*token),
                ValueToken::Ident(value) if value.eq_ignore_ascii_case("none")
            )
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RgbComponentKind {
    Number,
    Percentage,
}

fn rgb_component_kind<'i>(
    ast: &Compilation<'i>,
    value: &TokenOrValue<'i>,
) -> Option<RgbComponentKind> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match ast.node(*token) {
        ValueToken::Number(_) => Some(RgbComponentKind::Number),
        ValueToken::Percentage(_) => Some(RgbComponentKind::Percentage),
        _ => None,
    }
}

fn is_rgb_alpha<'i>(ast: &Compilation<'i>, value: &TokenOrValue<'i>) -> bool {
    let TokenOrValue::Token(token) = value else {
        return false;
    };
    matches!(
        ast.node(*token),
        ValueToken::Number(_) | ValueToken::Percentage(_)
    )
}

fn is_comma<'i>(ast: &Compilation<'i>, value: &TokenOrValue<'i>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(ast.node(*token), ValueToken::Comma))
}

fn is_slash<'i>(ast: &Compilation<'i>, value: &TokenOrValue<'i>) -> bool {
    matches!(
        value, TokenOrValue::Token(token) if matches!(ast.node(*token), ValueToken::Delim("/"))
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
