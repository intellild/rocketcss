use super::*;

pub(super) fn minify_hsl_function(
    function: &Function<'_>,
    cx: &MinifyContext,
) -> Option<FunctionReplacement> {
    let is_hsl = match function.kind() {
        KnownFunction::Hsl => true,
        KnownFunction::Hsla => false,
        _ => return None,
    };
    let mut components = function.arguments.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token)
            if matches!(**token, Token::WhiteSpace(_) | Token::Comma | Token::Delim("/")))
    });
    let hue = color_number(components.next()?)?;
    let saturation = color_percentage(components.next()?)?;
    let lightness = color_percentage(components.next()?)?;
    let alpha = match components.next() {
        Some(value) => color_alpha(value)?,
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

fn color_number(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    let Token::Number(value) = **token else {
        return None;
    };
    Some(value)
}

fn color_percentage(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Percentage(value) => Some(value),
        Token::Number(0.0) => Some(0.0),
        _ => None,
    }
}

pub(super) fn minify_rgb_function(
    function: &Function<'_>,
    cx: &MinifyContext,
) -> Option<FunctionReplacement> {
    let is_rgb = match function.kind() {
        KnownFunction::Rgb => true,
        KnownFunction::Rgba => false,
        _ => return None,
    };
    let mut components = function.arguments.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token)
            if matches!(**token, Token::WhiteSpace(_) | Token::Comma | Token::Delim("/")))
    });
    let (red, red_percentage, red_normalized) = color_component(components.next()?)?;
    let (green, green_percentage, green_normalized) = color_component(components.next()?)?;
    let (blue, blue_percentage, blue_normalized) = color_component(components.next()?)?;
    let uses_percentage = red_percentage.or(green_percentage).or(blue_percentage);
    if [red_percentage, green_percentage, blue_percentage]
        .into_iter()
        .flatten()
        .any(|component| Some(component) != uses_percentage)
    {
        return None;
    }
    let alpha = match components.next() {
        Some(value) => color_alpha(value)?,
        None if is_rgb => 1.0,
        None => return None,
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

fn color_alpha(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Number(value) => Some(value),
        Token::Percentage(value) => Some(value),
        _ => None,
    }
}

fn color_component(value: &TokenOrValue<'_>) -> Option<(u8, Option<bool>, f32)> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    let (value, percentage, normalized) = match **token {
        Token::Number(value) if (0.0..=255.0).contains(&value) => {
            (value, (value != 0.0).then_some(false), value / 255.0)
        }
        Token::Percentage(value) if (0.0..=1.0).contains(&value) => {
            (value * 255.0, (value != 0.0).then_some(true), value)
        }
        _ => return None,
    };
    Some((value.round() as u8, percentage, normalized))
}
