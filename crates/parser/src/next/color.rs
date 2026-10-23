//! Color parsing into the flattened AST.

use super::{prelude::*, values::collect_tokens};

impl<'i> Parse<'i> for CssColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        let token = input.next()?.clone();
        let span = token_span(input);
        match token {
            OldToken::Ident(name) if name.eq_ignore_ascii_case("currentcolor") => Ok(input
                .ast_mut()
                .alloc_css_color(CssColorData::CurrentColor, span)),
            OldToken::Ident(name) => {
                let color = KnownColor::from_name(name)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                Ok(input
                    .ast_mut()
                    .alloc_css_color(CssColorData::Known(color), span))
            }
            OldToken::Hash(value) | OldToken::IdHash(value) => {
                let data = parse_hex_color(value)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                let rgba = input.ast_mut().alloc_rgba(data, span);
                Ok(input
                    .ast_mut()
                    .alloc_css_color(CssColorData::Rgba(rgba), span))
            }
            OldToken::Function(name) if KnownFunction::from_name(name).is_color() => {
                let arguments = input.parse_nested_block(|input| collect_tokens(input, 1))?;
                if crate::next::values::token_values_contain_opaque(input.ast(), arguments) {
                    return Err(location.new_custom_error(ParserError::InvalidValue));
                }
                let function = Function::new(name, arguments, input.ast_mut(), span);
                validate_rgb_function(input.ast_mut(), function);
                let valid = function
                    .flags(input.ast())
                    .contains(FunctionFlags::VALID_RGB);
                if matches!(
                    function.kind(input.ast()),
                    KnownFunction::Rgb | KnownFunction::Rgba
                ) && !valid
                {
                    return Err(location.new_custom_error(ParserError::InvalidValue));
                }
                Ok(input
                    .ast_mut()
                    .alloc_css_color(CssColorData::Function(function), span))
            }
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

pub(super) fn validate_rgb_function<'i>(ast: &mut Ast<'i>, function: Function<'i>) {
    if matches!(function.kind(ast), KnownFunction::Rgb | KnownFunction::Rgba) {
        let valid = is_supported_rgb_function(ast, function.arguments(ast));
        let mut flags = function.flags(ast);
        flags.set(FunctionFlags::VALID_RGB, valid);
        function.set_flags(ast, flags);
    }
}

fn is_supported_rgb_function<'i>(ast: &Ast<'i>, arguments: &'i [TokenOrValue<'i>]) -> bool {
    let mut components = arguments
        .iter()
        .filter(|value| !matches!(token_data(**value, ast), Some(TokenData::WhiteSpace(_))));
    let Some(first) = components.next() else {
        return false;
    };
    let Some(second) = components.next() else {
        return false;
    };

    let has_alpha = if is_comma(*second, ast) {
        validate_legacy_rgb(*first, &mut components, ast)
    } else {
        validate_modern_rgb(*first, *second, &mut components, ast)
    };
    has_alpha.is_some()
}

fn validate_legacy_rgb<'i>(
    first: TokenOrValue<'i>,
    components: &mut impl Iterator<Item = &'i TokenOrValue<'i>>,
    ast: &Ast<'i>,
) -> Option<bool> {
    let first_kind = rgb_component_kind(first, ast)?;
    let second = *components.next()?;
    let second_comma = *components.next()?;
    let third = *components.next()?;
    if rgb_component_kind(second, ast) != Some(first_kind)
        || !is_comma(second_comma, ast)
        || rgb_component_kind(third, ast) != Some(first_kind)
    {
        return None;
    }

    match components.next() {
        None => Some(false),
        Some(comma) if is_comma(*comma, ast) => {
            let alpha = *components.next()?;
            (is_rgb_alpha(alpha, ast) && components.next().is_none()).then_some(true)
        }
        Some(_) => None,
    }
}

fn validate_modern_rgb<'i>(
    first: TokenOrValue<'i>,
    second: TokenOrValue<'i>,
    components: &mut impl Iterator<Item = &'i TokenOrValue<'i>>,
    ast: &Ast<'i>,
) -> Option<bool> {
    if !is_modern_rgb_component(first, ast) || !is_modern_rgb_component(second, ast) {
        return None;
    }
    let third = *components.next()?;
    if !is_modern_rgb_component(third, ast) {
        return None;
    }

    match components.next() {
        None => Some(false),
        Some(slash) if is_slash(*slash, ast) => {
            let alpha = *components.next()?;
            (is_modern_rgb_alpha(alpha, ast) && components.next().is_none()).then_some(true)
        }
        Some(_) => None,
    }
}

fn token_data<'i>(value: TokenOrValue<'i>, ast: &Ast<'i>) -> Option<TokenData<'i>> {
    match value.data(ast) {
        TokenOrValueData::Token(token) => Some(token.data(ast)),
        _ => None,
    }
}

fn is_modern_rgb_component(value: TokenOrValue<'_>, ast: &Ast<'_>) -> bool {
    rgb_component_kind(value, ast).is_some() || is_none_keyword(value, ast)
}

fn is_modern_rgb_alpha(value: TokenOrValue<'_>, ast: &Ast<'_>) -> bool {
    is_rgb_alpha(value, ast) || is_none_keyword(value, ast)
}

fn is_none_keyword(value: TokenOrValue<'_>, ast: &Ast<'_>) -> bool {
    matches!(
        token_data(value, ast),
        Some(TokenData::Ident(value)) if value.eq_ignore_ascii_case("none")
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RgbComponentKind {
    Number,
    Percentage,
}

fn rgb_component_kind(value: TokenOrValue<'_>, ast: &Ast<'_>) -> Option<RgbComponentKind> {
    match token_data(value, ast)? {
        TokenData::Number(_) => Some(RgbComponentKind::Number),
        TokenData::Percentage(_) => Some(RgbComponentKind::Percentage),
        _ => None,
    }
}

fn is_rgb_alpha(value: TokenOrValue<'_>, ast: &Ast<'_>) -> bool {
    matches!(
        token_data(value, ast),
        Some(TokenData::Number(_) | TokenData::Percentage(_))
    )
}

fn is_comma(value: TokenOrValue<'_>, ast: &Ast<'_>) -> bool {
    matches!(token_data(value, ast), Some(TokenData::Comma))
}

fn is_slash(value: TokenOrValue<'_>, ast: &Ast<'_>) -> bool {
    matches!(token_data(value, ast), Some(TokenData::Delim("/")))
}

pub(super) fn parse_hex_color(value: &str) -> Option<RGBAData> {
    fn pair(value: &str) -> Option<u8> {
        u8::from_str_radix(value, 16).ok()
    }
    Some(match value.len() {
        3 | 4 => {
            let mut bytes = value.bytes().map(|byte| {
                let digit = (byte as char).to_digit(16)? as u8;
                Some(digit * 17)
            });
            RGBAData {
                red: bytes.next()??,
                green: bytes.next()??,
                blue: bytes.next()??,
                alpha: match bytes.next() {
                    Some(value) => value?,
                    None => 255,
                },
            }
        }
        6 | 8 => RGBAData {
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
