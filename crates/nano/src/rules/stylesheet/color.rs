use super::*;

pub(super) fn minify_hsl_function(
    function: &Function<'_>,
    arguments: &[TokenOrValue<'_>],
    cx: &MinifyContext,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<FunctionReplacement> {
    let is_hsl = match function.kind() {
        KnownFunction::Hsl => true,
        KnownFunction::Hsla => false,
        _ => return None,
    };
    let mut components = arguments.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), token if match token { Token::WhiteSpace(_) | Token::Comma => true, Token::Delim(value) => ast.ast_context().str(value) == "/", _ => false }))
    });
    let hue = color_number(components.next()?, ast)?;
    let saturation = color_percentage(components.next()?, ast)?;
    let lightness = color_percentage(components.next()?, ast)?;
    let alpha = match components.next() {
        Some(value) => color_alpha(value, ast)?,
        None if is_hsl => 1.0,
        None => return None,
    };
    if components.next().is_some() {
        return None;
    }
    let hue = hue.rem_euclid(360.0) / 60.0;
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = chroma * (1.0 - (hue.rem_euclid(2.0) - 1.0).abs());
    let (red, green, blue) = match hue as u8 {
        0 => (chroma, x, 0.0),
        1 => (x, chroma, 0.0),
        2 => (0.0, chroma, x),
        3 => (0.0, x, chroma),
        4 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };
    let match_value = lightness - chroma / 2.0;
    let red = ((red + match_value) * 255.0).round() as u8;
    let green = ((green + match_value) * 255.0).round() as u8;
    let blue = ((blue + match_value) * 255.0).round() as u8;
    Some(if alpha == 1.0 {
        FunctionReplacement::Rgb { red, green, blue }
    } else if red == green && green == blue && red > 0 && (lightness * 100.0).fract() == 0.0 {
        FunctionReplacement::GrayAlpha {
            alpha: (alpha * 1000.0).round() / 1000.0,
            lightness,
        }
    } else {
        FunctionReplacement::Rgba {
            alpha,
            red,
            green,
            blue,
            use_hex: cx.is_enabled(Options::USE_HEX_ALPHA_COLORS, OptionsOp::Any),
        }
    })
}

fn color_number(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    let Token::Number(value) = ast.ast_context().resolve_node(*token) else {
        return None;
    };
    Some(value)
}

fn color_percentage(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match ast.ast_context().resolve_node(*token) {
        Token::Percentage(value) => Some(value),
        Token::Number(0.0) => Some(0.0),
        _ => None,
    }
}

pub(super) fn minify_rgb_function(
    function: &Function<'_>,
    arguments: &[TokenOrValue<'_>],
    cx: &MinifyContext,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<FunctionReplacement> {
    if !matches!(function.kind(), KnownFunction::Rgb | KnownFunction::Rgba) {
        return None;
    }
    let mut components = arguments.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), token if match token { Token::WhiteSpace(_) | Token::Comma => true, Token::Delim(value) => ast.ast_context().str(value) == "/", _ => false }))
    });
    let (red, red_normalized) = color_component(components.next()?, ast)?;
    let (green, green_normalized) = color_component(components.next()?, ast)?;
    let (blue, blue_normalized) = color_component(components.next()?, ast)?;
    let alpha = match components.next() {
        Some(value) => color_alpha(value, ast)?,
        None => 1.0,
    };
    if components.next().is_some() {
        return None;
    }
    if alpha != 1.0 {
        let lightness = (red_normalized + green_normalized + blue_normalized) / 3.0;
        return Some(
            if red == green && green == blue && red > 0 && (lightness * 100.0).fract() == 0.0 {
                FunctionReplacement::GrayAlpha { alpha, lightness }
            } else {
                FunctionReplacement::Rgba {
                    alpha,
                    red,
                    green,
                    blue,
                    use_hex: cx.is_enabled(Options::USE_HEX_ALPHA_COLORS, OptionsOp::Any),
                }
            },
        );
    }
    Some(FunctionReplacement::Rgb { blue, green, red })
}

fn color_alpha(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match ast.ast_context().resolve_node(*token) {
        Token::Number(value) | Token::Percentage(value) => Some(value.clamp(0.0, 1.0)),
        _ => None,
    }
}

fn color_component(
    value: &TokenOrValue<'_>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<(u8, f32)> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    let (value, normalized) = match ast.ast_context().resolve_node(*token) {
        Token::Number(value) => {
            let value = value.clamp(0.0, 255.0);
            (value, value / 255.0)
        }
        Token::Percentage(value) => {
            let value = value.clamp(0.0, 1.0);
            (value * 255.0, value)
        }
        _ => return None,
    };
    Some((value.round() as u8, normalized))
}
