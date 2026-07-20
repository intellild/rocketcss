use super::*;

pub(super) fn rollback_gradient_color_replacements(arguments: &mut Vec<'_, TokenOrValue<'_>>) {
    for argument in arguments {
        let TokenOrValue::Function(function) = argument else {
            continue;
        };
        if matches!(
            function.replacement,
            Some(
                FunctionReplacement::Rgb { .. }
                    | FunctionReplacement::Rgba { .. }
                    | FunctionReplacement::GrayAlpha { .. }
            )
        ) {
            function.replacement = None;
        }
    }
}

pub(super) fn minify_gradient_direction(arguments: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let end = arguments
        .iter()
        .position(
            |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        )
        .unwrap_or(arguments.len());
    let mut items = arguments[..end]
        .iter()
        .enumerate()
        .filter(|(_, value)| !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_))));
    let Some((to_index, to)) = items.next() else {
        return false;
    };
    let Some((direction_index, direction)) = items.next() else {
        return false;
    };
    if items.next().is_some()
        || !matches!(to, TokenOrValue::Token(token) if matches!(&**token, Token::Ident(value) if match_ignore_ascii_case!(value, "to" => true, _ => false)))
    {
        return false;
    }
    let Some(degrees) = (match direction {
        TokenOrValue::Token(token) => match &**token {
            Token::Ident(value) => match_ignore_ascii_case!(
                value,
                "top" => Some(0.0),
                "right" => Some(90.0),
                "bottom" => Some(180.0),
                "left" => Some(270.0),
                _ => None,
            ),
            _ => None,
        },
        _ => None,
    }) else {
        return false;
    };
    let TokenOrValue::Token(token) = &mut arguments[to_index] else {
        return false;
    };
    **token = Token::Dimension {
        unit: Unit::Deg,
        value: degrees,
    };
    arguments.drain(to_index + 1..=direction_index);
    true
}

pub(super) fn minify_gradient_stops(arguments: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let mut changed = false;
    if let Some((color_index, position_index)) = first_gradient_stop(arguments)
        && is_zero_gradient_position(&arguments[position_index])
    {
        if let TokenOrValue::Function(function) = &mut arguments[color_index]
            && matches!(
                function.replacement,
                Some(FunctionReplacement::Rgba { alpha: 0.0, .. })
            )
        {
            function.set_name("transparent");
            function.arguments.clear();
            function.replacement = None;
            function.set_identifier(true);
        }
        arguments.drain(color_index + 1..=position_index);
        changed = true;
    }
    if let Some((color_index, position_index)) = last_gradient_stop(arguments)
        && is_full_gradient_position(&arguments[position_index])
    {
        arguments.drain(color_index + 1..=position_index);
        changed = true;
    }
    changed | clamp_gradient_stop_positions(arguments)
}

fn first_gradient_stop(arguments: &[TokenOrValue<'_>]) -> Option<(usize, usize)> {
    let mut start = 0;
    loop {
        let end = next_comma(arguments, start);
        if !is_gradient_prelude(arguments, start, end) {
            return gradient_stop(arguments, start, end);
        }
        if end == arguments.len() {
            return None;
        }
        start = end + 1;
    }
}

fn last_gradient_stop(arguments: &[TokenOrValue<'_>]) -> Option<(usize, usize)> {
    let start = arguments
        .iter()
        .rposition(
            |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        )
        .map_or(0, |index| index + 1);
    gradient_stop(arguments, start, arguments.len())
}

fn is_gradient_prelude(arguments: &[TokenOrValue<'_>], start: usize, end: usize) -> bool {
    let Some(first) = arguments[start..end].iter().find(|value| {
        !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
    }) else {
        return true;
    };
    match first {
        TokenOrValue::Angle(_) => true,
        TokenOrValue::Token(token) => match &**token {
            Token::Number(_) | Token::Percentage(_) => true,
            Token::Dimension { unit, .. } => !unit.is_length(),
            Token::Ident(value) => match_ignore_ascii_case!(
                value,
                "at" | "to" | "center" | "circle" | "ellipse" | "closest-side" | "closest-corner" | "farthest-side" | "farthest-corner" | "contain" | "cover" => true,
                _ => false,
            ),
            _ => false,
        },
        _ => false,
    }
}

fn next_comma(arguments: &[TokenOrValue<'_>], start: usize) -> usize {
    arguments[start..]
        .iter()
        .position(
            |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        )
        .map_or(arguments.len(), |index| start + index)
}

fn gradient_stop(
    arguments: &[TokenOrValue<'_>],
    start: usize,
    end: usize,
) -> Option<(usize, usize)> {
    let mut items = arguments[start..end]
        .iter()
        .enumerate()
        .filter(|(_, value)| !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_))))
        .map(|(index, _)| start + index);
    let color = items.next()?;
    let position = items.next()?;
    if items.next().is_some()
        || !is_color_value(&arguments[color])
        || gradient_position(&arguments[position]).is_none()
    {
        return None;
    }
    Some((color, position))
}

#[derive(Clone, Copy)]
enum GradientPosition {
    Number(f32),
    Percentage(f32),
    Length(LengthUnit, f32),
}

fn gradient_position(value: &TokenOrValue<'_>) -> Option<GradientPosition> {
    match value {
        TokenOrValue::Length(value) => Some(GradientPosition::Length(value.unit, value.value)),
        TokenOrValue::Function(function) => match function.replacement {
            Some(FunctionReplacement::Number(value)) => Some(GradientPosition::Number(value)),
            Some(FunctionReplacement::Percentage(value)) => {
                Some(GradientPosition::Percentage(value))
            }
            Some(FunctionReplacement::Dimension {
                unit: Unit::Length(unit),
                value,
            }) => Some(GradientPosition::Length(unit, value)),
            _ => None,
        },
        TokenOrValue::Token(token) => match **token {
            Token::Number(value) => Some(GradientPosition::Number(value)),
            Token::Percentage(value) => Some(GradientPosition::Percentage(value)),
            Token::Dimension {
                unit: Unit::Length(unit),
                value,
            } => Some(GradientPosition::Length(unit, value)),
            _ => None,
        },
        _ => None,
    }
}

fn clamp_gradient_stop_positions(arguments: &mut [TokenOrValue<'_>]) -> bool {
    let mut start = 0;
    let mut previous = None;
    let mut changed = false;
    loop {
        let end = next_comma(arguments, start);
        if let Some((_, position_index)) = gradient_stop(arguments, start, end) {
            let current = gradient_position(&arguments[position_index])
                .expect("gradient_stop validates its position");
            if previous.is_some_and(|previous| gradient_position_lte(current, previous)) {
                set_gradient_position_zero(&mut arguments[position_index]);
                changed = true;
            } else {
                previous = Some(current);
            }
        }
        if end == arguments.len() {
            return changed;
        }
        start = end + 1;
    }
}

fn gradient_position_lte(left: GradientPosition, right: GradientPosition) -> bool {
    match (left, right) {
        (GradientPosition::Number(left), GradientPosition::Number(right))
        | (GradientPosition::Percentage(left), GradientPosition::Percentage(right)) => {
            left <= right
        }
        (
            GradientPosition::Length(left_unit, left),
            GradientPosition::Length(right_unit, right),
        ) if left_unit == right_unit => left <= right,
        (GradientPosition::Number(0.0), _) => true,
        _ => false,
    }
}

fn set_gradient_position_zero(value: &mut TokenOrValue<'_>) {
    match value {
        TokenOrValue::Length(value) => value.value = 0.0,
        TokenOrValue::Function(function) => {
            function.arguments.clear();
            function.replacement = Some(FunctionReplacement::Number(0.0));
        }
        TokenOrValue::Token(token) => **token = Token::Number(0.0),
        _ => {}
    }
}

fn is_zero_gradient_position(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token)
        if matches!(**token, Token::Number(0.0) | Token::Percentage(0.0)))
}

fn is_full_gradient_position(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Percentage(1.0)))
}

fn is_color_value(value: &TokenOrValue<'_>) -> bool {
    matches!(
        value,
        TokenOrValue::Color(_) | TokenOrValue::UnresolvedColor(_)
    ) || matches!(value, TokenOrValue::Function(function) if function.kind().is_color())
        || matches!(value, TokenOrValue::Token(token)
            if matches!(**token, Token::Ident(_) | Token::Hash(_) | Token::IdHash(_) | Token::MinifiedHash(_)))
}
