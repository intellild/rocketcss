use rs_css_ast::{Function, RGBA, Token, TokenOrValue};

pub(crate) fn parse_color_value(value: &TokenOrValue<'_>) -> Option<RGBA> {
    match value {
        TokenOrValue::Token(token) => match &**token {
            Token::Ident(name) => named_color(name),
            Token::Hash(value) | Token::IdHash(value) => parse_hex_color(value),
            _ => None,
        },
        TokenOrValue::Function(function) => parse_color_function(function),
        _ => None,
    }
}

fn parse_color_function(function: &Function<'_>) -> Option<RGBA> {
    if function.name.eq_ignore_ascii_case("rgb") || function.name.eq_ignore_ascii_case("rgba") {
        parse_rgb(&function.arguments)
    } else if function.name.eq_ignore_ascii_case("hsl")
        || function.name.eq_ignore_ascii_case("hsla")
    {
        parse_hsl(&function.arguments)
    } else {
        None
    }
}

fn parse_rgb(arguments: &[TokenOrValue<'_>]) -> Option<RGBA> {
    let components = numeric_components(arguments)?;
    if !(3..=4).contains(&components.len()) {
        return None;
    }
    let red = color_channel(components[0])?;
    let green = color_channel(components[1])?;
    let blue = color_channel(components[2])?;
    let alpha = components
        .get(3)
        .map_or(Some(255), |value| alpha_channel(*value))?;
    Some(RGBA {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_hsl(arguments: &[TokenOrValue<'_>]) -> Option<RGBA> {
    let components = numeric_components(arguments)?;
    if !(3..=4).contains(&components.len()) {
        return None;
    }
    let hue = hue_degrees(components[0])?;
    let Numeric::Percentage(saturation) = components[1] else {
        return None;
    };
    let Numeric::Percentage(lightness) = components[2] else {
        return None;
    };
    let alpha = components
        .get(3)
        .map_or(Some(255), |value| alpha_channel(*value))?;
    let (red, green, blue) = hsl_to_rgb(hue, saturation, lightness);
    Some(RGBA {
        red,
        green,
        blue,
        alpha,
    })
}

#[derive(Clone, Copy)]
enum Numeric<'a> {
    Number(f32),
    Percentage(f32),
    Dimension(f32, &'a str),
}

fn numeric_components<'a>(arguments: &[TokenOrValue<'a>]) -> Option<std::vec::Vec<Numeric<'a>>> {
    let mut result = std::vec::Vec::new();
    for argument in arguments {
        let TokenOrValue::Token(token) = argument else {
            return None;
        };
        match &**token {
            Token::Number(value) => result.push(Numeric::Number(*value)),
            Token::Percentage(value) => result.push(Numeric::Percentage(*value)),
            Token::Dimension { unit, value } => result.push(Numeric::Dimension(*value, unit)),
            Token::WhiteSpace(_) | Token::Comment(_) | Token::Comma => {}
            Token::Delim(value) if *value == "/" => {}
            _ => return None,
        }
    }
    Some(result)
}

fn hue_degrees(value: Numeric<'_>) -> Option<f32> {
    match value {
        Numeric::Number(value) => Some(value),
        Numeric::Dimension(value, unit) if unit.eq_ignore_ascii_case("deg") => Some(value),
        Numeric::Dimension(value, unit) if unit.eq_ignore_ascii_case("grad") => Some(value * 0.9),
        Numeric::Dimension(value, unit) if unit.eq_ignore_ascii_case("rad") => {
            Some(value.to_degrees())
        }
        Numeric::Dimension(value, unit) if unit.eq_ignore_ascii_case("turn") => Some(value * 360.0),
        _ => None,
    }
}

fn color_channel(value: Numeric<'_>) -> Option<u8> {
    let value = match value {
        Numeric::Number(value) => value,
        Numeric::Percentage(value) => value * 255.0,
        Numeric::Dimension(..) => return None,
    };
    (value.is_finite() && (0.0..=255.0).contains(&value)).then(|| value.round() as u8)
}

fn alpha_channel(value: Numeric<'_>) -> Option<u8> {
    let value = match value {
        Numeric::Number(value) | Numeric::Percentage(value) => value,
        Numeric::Dimension(..) => return None,
    };
    (value.is_finite() && (0.0..=1.0).contains(&value)).then(|| (value * 255.0).round() as u8)
}

fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (u8, u8, u8) {
    let hue = f64::from(hue).rem_euclid(360.0) / 360.0;
    let saturation = f64::from(saturation);
    let lightness = f64::from(lightness);
    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - lightness * saturation
    };
    let p = 2.0 * lightness - q;
    let channel = |offset: f64| {
        let value = (hue + offset).rem_euclid(1.0);
        let value = if saturation == 0.0 {
            lightness
        } else if value < 1.0 / 6.0 {
            p + (q - p) * 6.0 * value
        } else if value < 0.5 {
            q
        } else if value < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - value) * 6.0
        } else {
            p
        };
        (value * 255.0).round() as u8
    };
    (channel(1.0 / 3.0), channel(0.0), channel(-1.0 / 3.0))
}

fn named_color(name: &str) -> Option<RGBA> {
    let (red, green, blue, alpha) = if name.eq_ignore_ascii_case("transparent") {
        (0, 0, 0, 0)
    } else if name.eq_ignore_ascii_case("black") {
        (0, 0, 0, 255)
    } else if name.eq_ignore_ascii_case("silver") {
        (192, 192, 192, 255)
    } else if name.eq_ignore_ascii_case("gray") || name.eq_ignore_ascii_case("grey") {
        (128, 128, 128, 255)
    } else if name.eq_ignore_ascii_case("white") {
        (255, 255, 255, 255)
    } else if name.eq_ignore_ascii_case("maroon") {
        (128, 0, 0, 255)
    } else if name.eq_ignore_ascii_case("red") {
        (255, 0, 0, 255)
    } else if name.eq_ignore_ascii_case("purple") {
        (128, 0, 128, 255)
    } else if name.eq_ignore_ascii_case("fuchsia") {
        (255, 0, 255, 255)
    } else if name.eq_ignore_ascii_case("green") {
        (0, 128, 0, 255)
    } else if name.eq_ignore_ascii_case("lime") {
        (0, 255, 0, 255)
    } else if name.eq_ignore_ascii_case("olive") {
        (128, 128, 0, 255)
    } else if name.eq_ignore_ascii_case("yellow") {
        (255, 255, 0, 255)
    } else if name.eq_ignore_ascii_case("navy") {
        (0, 0, 128, 255)
    } else if name.eq_ignore_ascii_case("blue") {
        (0, 0, 255, 255)
    } else if name.eq_ignore_ascii_case("teal") {
        (0, 128, 128, 255)
    } else if name.eq_ignore_ascii_case("aqua") {
        (0, 255, 255, 255)
    } else if name.eq_ignore_ascii_case("orange") {
        (255, 165, 0, 255)
    } else if name.eq_ignore_ascii_case("azure") {
        (240, 255, 255, 255)
    } else if name.eq_ignore_ascii_case("rebeccapurple") {
        (102, 51, 153, 255)
    } else {
        return None;
    };
    Some(RGBA {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_hex_color(value: &str) -> Option<RGBA> {
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
