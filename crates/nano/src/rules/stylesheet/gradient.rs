use super::*;

pub(super) fn rollback_gradient_color_replacements<'ast>(
    arguments: &mut Vec<'ast, TokenOrValue<'ast>>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    for argument in arguments {
        let TokenOrValue::Function(function) = argument else {
            continue;
        };
        let has_replacement = matches!(
            ast.ast_context().resolve_node(*function).replacement,
            Some(
                FunctionReplacement::Rgb { .. }
                    | FunctionReplacement::Rgba { .. }
                    | FunctionReplacement::GrayAlpha { .. }
            )
        );
        if has_replacement {
            ast.mutate_node(*function, |function, _| function.replacement = None);
        }
    }
}

pub(super) fn minify_gradient_direction<'ast>(
    arguments: &mut Vec<'ast, TokenOrValue<'ast>>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) -> bool {
    let end = arguments
        .iter()
        .position(|value| {
            matches!(value, TokenOrValue::Token(token)
                if matches!(ast.ast_context().resolve_node(*token), Token::Comma))
        })
        .unwrap_or(arguments.len());
    let mut items = arguments[..end].iter().enumerate().filter(|(_, value)| {
        !matches!(value, TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), Token::WhiteSpace(_)))
    });
    let Some((to_index, to)) = items.next() else {
        return false;
    };
    let Some((direction_index, direction)) = items.next() else {
        return false;
    };
    if items.next().is_some()
        || !matches!(to, TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), Token::Ident(value)
                if match_ignore_ascii_case!(ast.ast_context().str(value), "to" => true, _ => false)))
    {
        return false;
    }
    let Some(degrees) = (match direction {
        TokenOrValue::Token(token) => match ast.ast_context().resolve_node(*token) {
            Token::Ident(value) => match_ignore_ascii_case!(
                ast.ast_context().str(value),
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
    ast.mutate_node(*token, |token, _| {
        *token = Token::Dimension {
            unit: Unit::Deg,
            value: degrees,
        };
    });
    arguments.drain(to_index + 1..=direction_index);
    true
}

pub(super) fn minify_gradient_stops<'ast>(
    arguments: &mut Vec<'ast, TokenOrValue<'ast>>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) -> bool {
    let mut changed = false;
    if let Some((color_index, position_index)) = first_gradient_stop(arguments, ast)
        && is_zero_gradient_position(&arguments[position_index], ast)
    {
        if let TokenOrValue::Function(function) = &mut arguments[color_index]
            && matches!(
                ast.ast_context().resolve_node(*function).replacement,
                Some(FunctionReplacement::Rgba { alpha: 0.0, .. })
            )
        {
            ast.mutate_node(*function, |function, ast| {
                function.set_name("transparent", ast.ast_context_mut());
                let mut arguments = function.arguments;
                ast.rewrite_vec(&mut arguments, |arguments, _| arguments.clear());
                function.arguments = arguments;
                function.replacement = None;
                function.set_identifier(true);
            });
        }
        arguments.drain(color_index + 1..=position_index);
        changed = true;
    }
    if let Some((color_index, position_index)) = last_gradient_stop(arguments, ast)
        && is_full_gradient_position(&arguments[position_index], ast)
    {
        arguments.drain(color_index + 1..=position_index);
        changed = true;
    }
    changed | clamp_gradient_stop_positions(arguments, ast)
}

fn first_gradient_stop(
    arguments: &[TokenOrValue<'_>],
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<(usize, usize)> {
    let mut start = 0;
    loop {
        let end = next_comma(arguments, start, ast);
        if !is_gradient_prelude(arguments, start, end, ast) {
            return gradient_stop(arguments, start, end, ast);
        }
        if end == arguments.len() {
            return None;
        }
        start = end + 1;
    }
}

fn last_gradient_stop(
    arguments: &[TokenOrValue<'_>],
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<(usize, usize)> {
    let start = arguments
        .iter()
        .rposition(|value| {
            matches!(value, TokenOrValue::Token(token)
                if matches!(ast.ast_context().resolve_node(*token), Token::Comma))
        })
        .map_or(0, |index| index + 1);
    gradient_stop(arguments, start, arguments.len(), ast)
}

fn is_gradient_prelude(
    arguments: &[TokenOrValue<'_>],
    start: usize,
    end: usize,
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    let Some(first) = arguments[start..end].iter().find(|value| {
        !matches!(value, TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), Token::WhiteSpace(_)))
    }) else {
        return true;
    };
    match first {
        TokenOrValue::Angle(_) => true,
        TokenOrValue::Token(token) => match ast.ast_context().resolve_node(*token) {
            Token::Number(_) | Token::Percentage(_) => true,
            Token::Dimension { unit, .. } => !unit.is_length(),
            Token::Ident(value) => match_ignore_ascii_case!(
                ast.ast_context().str(value),
                "at" | "to" | "center" | "circle" | "ellipse" | "closest-side" | "closest-corner" | "farthest-side" | "farthest-corner" | "contain" | "cover" => true,
                _ => false,
            ),
            _ => false,
        },
        _ => false,
    }
}

fn next_comma(
    arguments: &[TokenOrValue<'_>],
    start: usize,
    ast: &VisitMutContext<'_, '_, '_>,
) -> usize {
    arguments[start..]
        .iter()
        .position(|value| {
            matches!(value, TokenOrValue::Token(token)
                if matches!(ast.ast_context().resolve_node(*token), Token::Comma))
        })
        .map_or(arguments.len(), |index| start + index)
}

fn gradient_stop(
    arguments: &[TokenOrValue<'_>],
    start: usize,
    end: usize,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<(usize, usize)> {
    let mut items = arguments[start..end]
        .iter()
        .enumerate()
        .filter(|(_, value)| {
            !matches!(value, TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), Token::WhiteSpace(_)))
        })
        .map(|(index, _)| start + index);
    let color = items.next()?;
    let position = items.next()?;
    if items.next().is_some()
        || !is_color_value(&arguments[color], ast)
        || gradient_position(&arguments[position], ast).is_none()
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

fn gradient_position(
    value: &TokenOrValue<'_>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<GradientPosition> {
    match value {
        TokenOrValue::Length(value) => Some(GradientPosition::Length(value.unit, value.value)),
        TokenOrValue::Function(function) => {
            match ast.ast_context().resolve_node(*function).replacement {
                Some(FunctionReplacement::Number(value)) => Some(GradientPosition::Number(value)),
                Some(FunctionReplacement::Percentage(value)) => {
                    Some(GradientPosition::Percentage(value))
                }
                Some(FunctionReplacement::Dimension {
                    unit: Unit::Length(unit),
                    value,
                }) => Some(GradientPosition::Length(unit, value)),
                _ => None,
            }
        }
        TokenOrValue::Token(token) => match ast.ast_context().resolve_node(*token) {
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

fn clamp_gradient_stop_positions<'ast>(
    arguments: &mut [TokenOrValue<'ast>],
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) -> bool {
    let mut start = 0;
    let mut previous = None;
    let mut changed = false;
    loop {
        let end = next_comma(arguments, start, ast);
        if let Some((_, position_index)) = gradient_stop(arguments, start, end, ast) {
            let current = gradient_position(&arguments[position_index], ast)
                .expect("gradient_stop validates its position");
            if previous.is_some_and(|previous| gradient_position_lte(current, previous)) {
                set_gradient_position_zero(&mut arguments[position_index], ast);
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

fn set_gradient_position_zero<'ast>(
    value: &mut TokenOrValue<'ast>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    match value {
        TokenOrValue::Length(value) => value.value = 0.0,
        TokenOrValue::Function(function) => ast.mutate_node(*function, |function, ast| {
            let mut arguments = function.arguments;
            ast.rewrite_vec(&mut arguments, |arguments, _| arguments.clear());
            function.arguments = arguments;
            function.replacement = Some(FunctionReplacement::Number(0.0));
        }),
        TokenOrValue::Token(token) => {
            ast.mutate_node(*token, |token, _| *token = Token::Number(0.0));
        }
        _ => {}
    }
}

fn is_zero_gradient_position(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token)
        if matches!(ast.ast_context().resolve_node(*token), Token::Number(0.0) | Token::Percentage(0.0)))
}

fn is_full_gradient_position(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token)
        if matches!(ast.ast_context().resolve_node(*token), Token::Percentage(1.0)))
}

fn is_color_value(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(
        value,
        TokenOrValue::Color(_) | TokenOrValue::UnresolvedColor(_)
    ) || matches!(value, TokenOrValue::Function(function)
        if ast.ast_context().resolve_node(*function).kind().is_color())
        || matches!(value, TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), Token::Ident(_) | Token::Hash(_) | Token::IdHash(_) | Token::MinifiedHash(_)))
}
