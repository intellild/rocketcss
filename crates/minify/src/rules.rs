use rocketcss_allocator::vec::Vec;
use rocketcss_ast::{
    CustomProperty, Declaration, DeclarationBlock, EnvironmentVariable, Function,
    FunctionReplacement, KeyframeSelector, KnownFunction, LengthUnit, StyleSheet, Token,
    TokenOrValue, Unit, UnknownAtRule, UnparsedProperty, Variable, match_ignore_ascii_case,
};

use crate::{Minify, MinifyContext, Options, OptionsOp, context::ValueContextFlags};

impl Minify for StyleSheet<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        crate::minify_style_sheet(self, cx);
    }
}

impl Minify for DeclarationBlock<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        deduplicate_declarations(self, cx);
    }
}

fn deduplicate_declarations(block: &mut DeclarationBlock<'_>, cx: &mut MinifyContext) {
    for current in 1..block.len() {
        if block.declarations[current].is_tombstone() {
            continue;
        }
        let previous = (0..current).rev().find(|&previous| {
            !block.declarations[previous].is_tombstone()
                && block.is_important(previous) == block.is_important(current)
                && block.declarations[previous].name() == block.declarations[current].name()
                && block.declarations[previous].vendor_prefix()
                    == block.declarations[current].vendor_prefix()
        });
        if let Some(previous) = previous
            && block.declarations[previous] == block.declarations[current]
        {
            block.declarations[previous] = Declaration::Tombstone;
            cx.record_declaration_removed();
        }
    }
}

impl Minify for KeyframeSelector<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
            return;
        }
        let changed = match self {
            KeyframeSelector::From => {
                *self = KeyframeSelector::Percentage(0.0);
                true
            }
            KeyframeSelector::Percentage(value) if *value == 1.0 => {
                *self = KeyframeSelector::To;
                true
            }
            _ => false,
        };
        if changed {
            cx.record_value_normalized();
        }
    }
}

impl Minify for UnparsedProperty<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.value.minify(cx);
    }
}

impl Minify for CustomProperty<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.value.minify(cx);
    }
}

fn token_or_value_contains_variable(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Var(_) => true,
        TokenOrValue::Function(function) => {
            function.kind() == KnownFunction::Var
                || function
                    .arguments
                    .iter()
                    .any(token_or_value_contains_variable)
        }
        _ => false,
    }
}

impl Minify for Function<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if cx
            .value_context
            .is_enabled(ValueContextFlags::SKIP_VALUE_TRANSFORMS)
        {
            return;
        }
        if let Some(canonical) = match self.kind() {
            KnownFunction::Rgb => Some("rgb"),
            KnownFunction::Rgba => Some("rgba"),
            KnownFunction::Hsl => Some("hsl"),
            KnownFunction::Hsla => Some("hsla"),
            KnownFunction::Hwb => Some("hwb"),
            _ => None,
        } {
            self.set_name(canonical);
            canonicalize_nested_variable_functions(&mut self.arguments);
        }
        let is_gradient = self.kind().is_gradient();
        let gradient_contains_variable =
            is_gradient && self.arguments.iter().any(token_or_value_contains_variable);
        if gradient_contains_variable {
            rollback_gradient_color_replacements(&mut self.arguments);
        }
        let preserve_space_after_comma = cx
            .value_context
            .is_enabled(ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA);
        cx.value_context.set_enabled(
            ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA,
            cx.is_enabled(Options::PRESERVE_VARIABLE_FALLBACK_SPACE, OptionsOp::Any)
                && self.kind().is_variable(),
        );
        self.arguments.minify(cx);
        cx.value_context.set_enabled(
            ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA,
            preserve_space_after_comma,
        );
        if is_gradient
            && !gradient_contains_variable
            && (minify_gradient_direction(&mut self.arguments)
                | minify_gradient_stops(&mut self.arguments))
        {
            cx.record_value_normalized();
        }
        if cx
            .value_context
            .is_enabled(ValueContextFlags::MINIFY_COLORS)
            && let Some(color) =
                minify_rgb_function(self, cx).or_else(|| minify_hsl_function(self, cx))
        {
            self.replacement = Some(color);
            cx.record_value_normalized();
            return;
        }
        if self.kind() == KnownFunction::Calc && !self.is_vendor_prefixed() {
            if let Some(linear) = calc_linear_expression(&self.arguments)
                .map(|linear| linear.round(cx.options().calc_precision))
                && linear.write_to(self)
            {
                cx.record_value_normalized();
                if self.replacement.is_some() {
                    return;
                }
            }
            if remove_redundant_calc_parentheses(&mut self.arguments) {
                cx.record_value_normalized();
            }
            if minify_flat_calc_operations(&mut self.arguments) {
                cx.record_value_normalized();
            }
            if let Some(value) = simple_calc_value(&self.arguments) {
                self.replacement = Some(value);
                self.arguments.clear();
                cx.record_value_normalized();
                return;
            }
        }
        if self.kind() == KnownFunction::Url {
            if cx.is_enabled(Options::NORMALIZE_URLS, OptionsOp::Any) {
                self.set_name("url");
                let allocator = self.arguments.bump();
                if let [TokenOrValue::Token(token)] = self.arguments.as_mut_slice()
                    && let Token::String(value) = &mut **token
                {
                    if let Some(normalized) = normalize_url_text(value) {
                        *value = allocator.alloc_str(&normalized);
                        cx.record_value_normalized();
                    }
                    let unquoted_url = !value.get(..5).is_some_and(
                        |prefix| match_ignore_ascii_case!(prefix, "data:" => true, _ => false),
                    ) && can_unquote_url(value);
                    self.set_unquoted_url(unquoted_url);
                }
            } else if matches!(self.arguments.as_slice(), [TokenOrValue::Token(token)]
                if matches!(&**token, Token::String(value)
                    if !value.get(..5).is_some_and(|prefix| {
                        match_ignore_ascii_case!(prefix, "data:" => true, _ => false)
                    })
                        && can_unquote_url(value)))
            {
                self.set_unquoted_url(true);
                cx.record_value_normalized();
            }
        }
        if cx.value_context.property == crate::context::PropertyContext::Transform
            && minify_transform_function(self)
        {
            cx.record_value_normalized();
        }
        if !matches!(
            cx.value_context.property,
            crate::context::PropertyContext::TimingFunction
                | crate::context::PropertyContext::Animation
                | crate::context::PropertyContext::Transition
        ) {
            return;
        }

        let replacement = match self.kind() {
            KnownFunction::CubicBezier => minify_cubic_bezier(&self.arguments),
            KnownFunction::Steps => minify_steps(&mut self.arguments),
            _ => None,
        };
        if let Some(replacement) = replacement {
            self.set_name(replacement);
            self.arguments.clear();
            self.set_identifier(true);
            cx.record_value_normalized();
        }
    }
}

fn canonicalize_nested_variable_functions(arguments: &mut Vec<'_, TokenOrValue<'_>>) {
    for argument in arguments {
        let TokenOrValue::Function(function) = argument else {
            continue;
        };
        if let Some(name) = match function.kind() {
            KnownFunction::Var => Some("var"),
            KnownFunction::Env => Some("env"),
            _ => None,
        } {
            function.set_name(name);
        }
        canonicalize_nested_variable_functions(&mut function.arguments);
    }
}

fn rollback_gradient_color_replacements(arguments: &mut Vec<'_, TokenOrValue<'_>>) {
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

fn minify_gradient_direction(arguments: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
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

fn minify_gradient_stops(arguments: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
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

fn minify_hsl_function(function: &Function<'_>, cx: &MinifyContext) -> Option<FunctionReplacement> {
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

fn minify_rgb_function(function: &Function<'_>, cx: &MinifyContext) -> Option<FunctionReplacement> {
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

fn minify_transform_function(function: &mut Function<'_>) -> bool {
    if function.kind() == KnownFunction::RotateZ && function.arguments.len() == 1 {
        function.set_name("rotate");
        return true;
    }
    if function.kind() == KnownFunction::Matrix3d {
        let values = &function.arguments;
        if values.len() == 31
            && number_at(values, 4) == Some(0.0)
            && number_at(values, 6) == Some(0.0)
            && number_at(values, 12) == Some(0.0)
            && number_at(values, 14) == Some(0.0)
            && number_at(values, 16) == Some(0.0)
            && number_at(values, 18) == Some(0.0)
            && number_at(values, 20) == Some(1.0)
            && number_at(values, 22) == Some(0.0)
            && number_at(values, 28) == Some(0.0)
            && number_at(values, 30) == Some(1.0)
        {
            function.set_name("matrix");
            compact_arguments(
                &mut function.arguments,
                &[0, 1, 2, 3, 8, 9, 10, 11, 24, 25, 26],
            );
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Rotate3d && function.arguments.len() == 7 {
        let name = match (
            number_at(&function.arguments, 0),
            number_at(&function.arguments, 2),
            number_at(&function.arguments, 4),
        ) {
            (Some(1.0), Some(0.0), Some(0.0)) => "rotateX",
            (Some(0.0), Some(1.0), Some(0.0)) => "rotateY",
            (Some(0.0), Some(0.0), Some(1.0)) => "rotate",
            _ => return false,
        };
        function.set_name(name);
        compact_arguments(&mut function.arguments, &[6]);
        return true;
    }
    if function.kind() == KnownFunction::Scale && function.arguments.len() == 3 {
        if function.arguments[0] == function.arguments[2]
            && !is_empty_variable_function(&function.arguments[0])
        {
            function.arguments.truncate(1);
            return true;
        }
        let first = number_at(&function.arguments, 0);
        let second = number_at(&function.arguments, 2);
        if first == second && first.is_some() {
            function.arguments.truncate(1);
            return true;
        }
        if second == Some(1.0) {
            function.set_name("scaleX");
            function.arguments.truncate(1);
            return true;
        }
        if first == Some(1.0) {
            function.set_name("scaleY");
            compact_arguments(&mut function.arguments, &[2]);
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Scale3d && function.arguments.len() == 5 {
        let values = [
            number_at(&function.arguments, 0),
            number_at(&function.arguments, 2),
            number_at(&function.arguments, 4),
        ];
        let (name, index) = if values[1] == Some(1.0) && values[2] == Some(1.0) {
            ("scaleX", 0)
        } else if values[0] == Some(1.0) && values[2] == Some(1.0) {
            ("scaleY", 2)
        } else if values[0] == Some(1.0) && values[1] == Some(1.0) {
            ("scaleZ", 4)
        } else {
            return false;
        };
        function.set_name(name);
        compact_arguments(&mut function.arguments, &[index]);
        return true;
    }
    if function.kind() == KnownFunction::Translate && function.arguments.len() == 3 {
        if number_at(&function.arguments, 2) == Some(0.0) {
            function.arguments.truncate(1);
            return true;
        }
        if number_at(&function.arguments, 0) == Some(0.0) {
            function.set_name("translateY");
            compact_arguments(&mut function.arguments, &[2]);
            return true;
        }
        return false;
    }
    if function.kind() == KnownFunction::Translate3d
        && function.arguments.len() == 5
        && number_at(&function.arguments, 0) == Some(0.0)
        && number_at(&function.arguments, 2) == Some(0.0)
    {
        function.set_name("translateZ");
        compact_arguments(&mut function.arguments, &[4]);
        return true;
    }
    false
}

fn is_empty_variable_function(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Function(function)
        if function.arguments.is_empty() && function.kind().is_variable())
}

fn simple_calc_value(values: &[TokenOrValue<'_>]) -> Option<FunctionReplacement> {
    let mut values = values.iter().filter(|value| {
        !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
    });
    let left = calc_value(values.next()?)?;
    let Some(operator) = values.next() else {
        return Some(unitless_calc_zero(left));
    };
    let right = calc_value(values.next()?)?;
    if values.next().is_some() {
        return None;
    }
    let TokenOrValue::Token(operator) = operator else {
        return None;
    };
    let Token::Delim(operator) = &**operator else {
        return None;
    };
    calculate_values(left, operator, right).map(unitless_calc_zero)
}

const MAX_CALC_TERMS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CalcTermKind {
    Number,
    Percentage,
    Dimension(Unit),
}

#[derive(Clone, Copy, Debug)]
struct CalcTerm {
    explicit_zero: bool,
    kind: CalcTermKind,
    value: f32,
}

impl CalcTerm {
    const EMPTY: Self = Self {
        explicit_zero: false,
        kind: CalcTermKind::Number,
        value: 0.0,
    };
}

#[derive(Clone, Copy, Debug)]
struct CalcLinear {
    terms: [CalcTerm; MAX_CALC_TERMS],
    len: usize,
}

impl CalcLinear {
    const fn empty() -> Self {
        Self {
            terms: [CalcTerm::EMPTY; MAX_CALC_TERMS],
            len: 0,
        }
    }

    fn from_value(value: FunctionReplacement) -> Option<Self> {
        let (kind, value) = match value {
            FunctionReplacement::Number(value) => (CalcTermKind::Number, value),
            FunctionReplacement::Percentage(value) => (CalcTermKind::Percentage, value),
            FunctionReplacement::Dimension { unit, value } => {
                (CalcTermKind::Dimension(unit), value)
            }
            _ => return None,
        };
        let mut result = Self::empty();
        result.terms[0] = CalcTerm {
            explicit_zero: value == 0.0,
            kind,
            value,
        };
        result.len = 1;
        Some(result)
    }

    fn add(mut self, right: Self, sign: f32) -> Option<Self> {
        for right in right.terms[..right.len].iter().copied() {
            if let Some(left) = self.terms[..self.len]
                .iter_mut()
                .find(|left| left.kind == right.kind)
            {
                left.value += right.value * sign;
                left.explicit_zero &= right.explicit_zero;
                continue;
            }
            if self.len == MAX_CALC_TERMS {
                return None;
            }
            self.terms[self.len] = CalcTerm {
                explicit_zero: right.explicit_zero,
                kind: right.kind,
                value: right.value * sign,
            };
            self.len += 1;
        }
        Some(self)
    }

    fn scale(mut self, factor: f32) -> Self {
        for term in &mut self.terms[..self.len] {
            term.value *= factor;
        }
        self
    }

    fn round(mut self, precision: Option<u8>) -> Self {
        let Some(precision) = precision else {
            return self;
        };
        let factor = 10_f64.powi(i32::from(precision));
        for term in &mut self.terms[..self.len] {
            term.value = ((f64::from(term.value) * factor).round() / factor) as f32;
        }
        self
    }

    fn scalar(self) -> Option<f32> {
        (self.len == 1 && self.terms[0].kind == CalcTermKind::Number).then_some(self.terms[0].value)
    }

    fn compact_cancelled_terms(&mut self) {
        let mut target = 0;
        for source in 0..self.len {
            let term = self.terms[source];
            if term.value == 0.0 && !term.explicit_zero {
                continue;
            }
            self.terms[target] = term;
            target += 1;
        }
        self.len = target;
    }

    fn replacement(self) -> Option<FunctionReplacement> {
        if self.len == 0 {
            return Some(FunctionReplacement::Number(0.0));
        }
        if self.len != 1 {
            return None;
        }
        Some(match self.terms[0] {
            CalcTerm {
                kind: CalcTermKind::Number,
                value,
                ..
            } => FunctionReplacement::Number(value),
            CalcTerm {
                kind: CalcTermKind::Percentage,
                value,
                ..
            } => FunctionReplacement::Percentage(value),
            CalcTerm {
                kind: CalcTermKind::Dimension(unit),
                value,
                ..
            } => FunctionReplacement::Dimension { unit, value },
        })
    }

    fn write_to(mut self, function: &mut Function<'_>) -> bool {
        self.compact_cancelled_terms();
        if let Some(replacement) = self.replacement() {
            function.replacement = Some(unitless_calc_zero(replacement));
            function.arguments.clear();
            return true;
        }

        let required = 1 + (self.len - 1) * 4;
        if function
            .arguments
            .iter()
            .filter(|value| matches!(value, TokenOrValue::Token(_)))
            .count()
            < required
        {
            return false;
        }
        for target in 0..required {
            if matches!(function.arguments[target], TokenOrValue::Token(_)) {
                continue;
            }
            let Some(source) = function.arguments[target + 1..]
                .iter()
                .position(|value| matches!(value, TokenOrValue::Token(_)))
                .map(|source| target + 1 + source)
            else {
                return false;
            };
            function.arguments.swap(target, source);
        }

        let mut output = 0;
        for (index, term) in self.terms[..self.len].iter().copied().enumerate() {
            if index != 0 {
                set_calc_token(&mut function.arguments[output], Token::WhiteSpace(" "));
                output += 1;
                set_calc_token(
                    &mut function.arguments[output],
                    Token::Delim(if term.value < 0.0 { "-" } else { "+" }),
                );
                output += 1;
                set_calc_token(&mut function.arguments[output], Token::WhiteSpace(" "));
                output += 1;
            }
            let value = if index == 0 {
                term.value
            } else {
                term.value.abs()
            };
            let token = match term.kind {
                CalcTermKind::Number => Token::Number(value),
                CalcTermKind::Percentage => Token::Percentage(value),
                CalcTermKind::Dimension(unit) => Token::Dimension { unit, value },
            };
            set_calc_token(&mut function.arguments[output], token);
            output += 1;
        }
        function.arguments.truncate(required);
        true
    }
}

fn set_calc_token<'a>(value: &mut TokenOrValue<'a>, token_value: Token<'a>) {
    let TokenOrValue::Token(token) = value else {
        unreachable!("calc output slots were normalized to tokens")
    };
    **token = token_value;
}

fn calc_linear_expression(values: &[TokenOrValue<'_>]) -> Option<CalcLinear> {
    let mut parser = CalcLinearParser { index: 0, values };
    let mut result = parser.expression(false)?;
    parser.skip_whitespace();
    if parser.index != values.len() {
        return None;
    }
    result.compact_cancelled_terms();
    Some(result)
}

struct CalcLinearParser<'values, 'arena> {
    index: usize,
    values: &'values [TokenOrValue<'arena>],
}

impl CalcLinearParser<'_, '_> {
    fn expression(&mut self, nested: bool) -> Option<CalcLinear> {
        let mut result = self.term()?;
        loop {
            self.skip_whitespace();
            if nested && self.is_close_parenthesis() {
                break;
            }
            let Some(operator) = self.operator(&["+", "-"]) else {
                break;
            };
            let right = self.term()?;
            result = result.add(right, if operator == "+" { 1.0 } else { -1.0 })?;
        }
        Some(result)
    }

    fn term(&mut self) -> Option<CalcLinear> {
        let mut result = self.factor()?;
        loop {
            self.skip_whitespace();
            let Some(operator) = self.operator(&["*", "/"]) else {
                break;
            };
            let right = self.factor()?;
            result = match operator {
                "*" => {
                    if let Some(scalar) = result.scalar() {
                        right.scale(scalar)
                    } else {
                        result.scale(right.scalar()?)
                    }
                }
                "/" => {
                    let divisor = right.scalar()?;
                    if divisor == 0.0 {
                        return None;
                    }
                    result.scale(1.0 / divisor)
                }
                _ => unreachable!(),
            };
        }
        Some(result)
    }

    fn factor(&mut self) -> Option<CalcLinear> {
        self.skip_whitespace();
        let mut sign = 1.0;
        while let Some(operator) = self.operator(&["+", "-"]) {
            if operator == "-" {
                sign = -sign;
            }
            self.skip_whitespace();
        }
        let value = self.values.get(self.index)?;
        let mut result = match value {
            TokenOrValue::Token(token) if matches!(**token, Token::ParenthesisBlock) => {
                self.index += 1;
                let result = self.expression(true)?;
                self.skip_whitespace();
                if !self.is_close_parenthesis() {
                    return None;
                }
                self.index += 1;
                result
            }
            TokenOrValue::Function(function)
                if function.kind() == KnownFunction::Calc && !function.is_vendor_prefixed() =>
            {
                self.index += 1;
                if let Some(replacement) = function.replacement {
                    CalcLinear::from_value(replacement)?
                } else {
                    calc_linear_expression(&function.arguments)?
                }
            }
            value => {
                self.index += 1;
                CalcLinear::from_value(calc_value(value)?)?
            }
        };
        if sign < 0.0 {
            result = result.scale(-1.0);
        }
        Some(result)
    }

    fn operator<'operator>(
        &mut self,
        allowed: &'operator [&'operator str],
    ) -> Option<&'operator str> {
        let TokenOrValue::Token(token) = self.values.get(self.index)? else {
            return None;
        };
        let Token::Delim(operator) = &**token else {
            return None;
        };
        let operator = allowed
            .iter()
            .copied()
            .find(|allowed| operator == allowed)?;
        self.index += 1;
        Some(operator)
    }

    fn is_close_parenthesis(&self) -> bool {
        matches!(self.values.get(self.index), Some(TokenOrValue::Token(token)) if matches!(**token, Token::CloseParenthesis))
    }

    fn skip_whitespace(&mut self) {
        while self.values.get(self.index).is_some_and(is_calc_whitespace) {
            self.index += 1;
        }
    }
}

fn unitless_calc_zero(value: FunctionReplacement) -> FunctionReplacement {
    match value {
        FunctionReplacement::Dimension { value: 0.0, .. }
        | FunctionReplacement::Percentage(0.0) => FunctionReplacement::Number(0.0),
        value => value,
    }
}

fn minify_flat_calc_operations(values: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let mut changed = false;
    loop {
        let mut reduced = false;
        for operator_index in 0..values.len() {
            let TokenOrValue::Token(operator) = &values[operator_index] else {
                continue;
            };
            let Token::Delim(operator) = &**operator else {
                continue;
            };
            if !matches!(*operator, "*" | "/") {
                continue;
            }
            let Some(left_index) = values[..operator_index]
                .iter()
                .rposition(|value| !is_calc_whitespace(value))
            else {
                continue;
            };
            let Some(right_index) = values[operator_index + 1..]
                .iter()
                .position(|value| !is_calc_whitespace(value))
                .map(|index| operator_index + 1 + index)
            else {
                continue;
            };
            let Some(result) = calc_value(&values[left_index])
                .zip(calc_value(&values[right_index]))
                .and_then(|(left, right)| calculate_values(left, operator, right))
            else {
                continue;
            };
            if !set_calc_value(&mut values[left_index], result) {
                continue;
            }
            values.drain(left_index + 1..=right_index);
            reduced = true;
            changed = true;
            break;
        }
        if !reduced {
            break;
        }
    }

    loop {
        let mut reduced = false;
        for operator_index in 0..values.len() {
            let TokenOrValue::Token(operator) = &values[operator_index] else {
                continue;
            };
            let Token::Delim(operator) = &**operator else {
                continue;
            };
            if !matches!(*operator, "+" | "-") {
                continue;
            }
            let Some(left_index) = values[..operator_index]
                .iter()
                .rposition(|value| !is_calc_whitespace(value))
            else {
                continue;
            };
            let Some(right_index) = values[operator_index + 1..]
                .iter()
                .position(|value| !is_calc_whitespace(value))
                .map(|index| operator_index + 1 + index)
            else {
                continue;
            };
            if !calc_value(&values[right_index]).is_some_and(calc_value_is_zero) {
                continue;
            }
            values.drain(left_index + 1..=right_index);
            reduced = true;
            changed = true;
            break;
        }
        if !reduced {
            return changed;
        }
    }
}

fn calc_value_is_zero(value: FunctionReplacement) -> bool {
    matches!(
        value,
        FunctionReplacement::Number(0.0)
            | FunctionReplacement::Dimension { value: 0.0, .. }
            | FunctionReplacement::Percentage(0.0)
    )
}

fn is_calc_whitespace(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_) | Token::Comment(_)))
}

fn set_calc_value(value: &mut TokenOrValue<'_>, result: FunctionReplacement) -> bool {
    match value {
        TokenOrValue::Token(token) => {
            **token = match result {
                FunctionReplacement::Number(value) => Token::Number(value),
                FunctionReplacement::Dimension { unit, value } => Token::Dimension { unit, value },
                FunctionReplacement::Percentage(value) => Token::Percentage(value),
                _ => return false,
            };
            true
        }
        TokenOrValue::Function(function) => {
            function.arguments.clear();
            function.replacement = Some(result);
            true
        }
        TokenOrValue::Length(value) => match result {
            FunctionReplacement::Dimension {
                unit: Unit::Length(unit),
                value: result,
            } if unit == value.unit => {
                value.value = result;
                true
            }
            FunctionReplacement::Number(0.0) => {
                value.value = 0.0;
                true
            }
            _ => false,
        },
        _ => false,
    }
}

fn remove_redundant_calc_parentheses(values: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let Some(open) = values.iter().position(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::ParenthesisBlock))
    }) else {
        return false;
    };
    let Some(close) = values[open + 1..]
        .iter()
        .position(|value| {
            matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::CloseParenthesis))
        })
        .map(|index| open + 1 + index)
    else {
        return false;
    };
    let previous = values[..open]
        .iter()
        .rev()
        .find(|value| !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_))));
    let preceded_by_addition = previous.is_none_or(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::Delim("+") | Token::Delim("-")))
    });
    let contains_addition = values[open + 1..close].iter().any(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::Delim("+") | Token::Delim("-")))
    });
    if !preceded_by_addition || contains_addition {
        return false;
    }
    let _ = values.remove(close);
    let _ = values.remove(open);
    true
}

fn calc_value(value: &TokenOrValue<'_>) -> Option<FunctionReplacement> {
    match value {
        TokenOrValue::Token(token) => match **token {
            Token::Number(value) => Some(FunctionReplacement::Number(value)),
            Token::Dimension { unit, value } => {
                Some(FunctionReplacement::Dimension { unit, value })
            }
            Token::Percentage(value) => Some(FunctionReplacement::Percentage(value)),
            _ => None,
        },
        TokenOrValue::Length(value) => Some(FunctionReplacement::Dimension {
            unit: Unit::Length(value.unit),
            value: value.value,
        }),
        TokenOrValue::Function(function) => function.replacement,
        _ => None,
    }
}

fn calculate_values(
    left: FunctionReplacement,
    operator: &str,
    right: FunctionReplacement,
) -> Option<FunctionReplacement> {
    use FunctionReplacement::{Dimension, Number, Percentage};
    match (left, operator, right) {
        (Number(left), "+", Number(right)) => Some(Number(left + right)),
        (Number(left), "-", Number(right)) => Some(Number(left - right)),
        (Number(left), "*", Number(right)) => Some(Number(left * right)),
        (Number(left), "/", Number(right)) if right != 0.0 => Some(Number(left / right)),
        (
            Dimension {
                unit: left_unit,
                value: left,
            },
            "+",
            Dimension {
                unit: right_unit,
                value: right,
            },
        ) if left_unit == right_unit => Some(Dimension {
            unit: left_unit,
            value: left + right,
        }),
        (
            Dimension {
                unit: left_unit,
                value: left,
            },
            "-",
            Dimension {
                unit: right_unit,
                value: right,
            },
        ) if left_unit == right_unit => Some(Dimension {
            unit: left_unit,
            value: left - right,
        }),
        (Dimension { unit, value }, "*", Number(number))
        | (Number(number), "*", Dimension { unit, value }) => Some(Dimension {
            unit,
            value: value * number,
        }),
        (Dimension { unit, value }, "/", Number(number)) if number != 0.0 => Some(Dimension {
            unit,
            value: value / number,
        }),
        (Percentage(left), "+", Percentage(right)) => Some(Percentage(left + right)),
        (Percentage(left), "-", Percentage(right)) => Some(Percentage(left - right)),
        (Percentage(value), "*", Number(number)) | (Number(number), "*", Percentage(value)) => {
            Some(Percentage(value * number))
        }
        (Percentage(value), "/", Number(number)) if number != 0.0 => {
            Some(Percentage(value / number))
        }
        _ => None,
    }
}

fn number_at(values: &[TokenOrValue<'_>], index: usize) -> Option<f32> {
    values.get(index).and_then(token_number)
}

fn compact_arguments(
    arguments: &mut rocketcss_allocator::vec::Vec<'_, TokenOrValue<'_>>,
    indices: &[usize],
) {
    for (target, &source) in indices.iter().enumerate() {
        if target != source {
            arguments.swap(target, source);
        }
    }
    arguments.truncate(indices.len());
}

fn can_unquote_url(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(|character| {
            character.is_whitespace()
                || character.is_control()
                || matches!(character, '(' | ')' | '\\')
        })
}

pub(crate) fn normalize_url_text(value: &str) -> Option<std::string::String> {
    let trimmed = value.trim();
    if trimmed
        .get(..5)
        .is_some_and(|prefix| match_ignore_ascii_case!(prefix, "data:" => true, _ => false))
    {
        return (trimmed != value).then(|| trimmed.to_owned());
    }

    let suffix_start = trimmed.find(['?', '#']).unwrap_or(trimmed.len());
    let (base, suffix) = trimmed.split_at(suffix_start);
    let (authority, path) = split_url_authority(base);
    let authority = normalize_url_authority(authority);
    let path = normalize_url_path(path);
    let mut normalized = std::string::String::with_capacity(trimmed.len());
    normalized.push_str(&authority);
    normalized.push_str(&path);
    normalized.push_str(suffix);
    (normalized != value).then_some(normalized)
}

fn split_url_authority(value: &str) -> (&str, &str) {
    let authority_start = if let Some(scheme) = value.find("://") {
        scheme + 3
    } else if value.starts_with("//") {
        2
    } else {
        return ("", value);
    };
    let Some(path_start) = value[authority_start..].find('/') else {
        return (value, "");
    };
    let path_start = authority_start + path_start + 1;
    (&value[..path_start], &value[path_start..])
}

fn normalize_url_authority(value: &str) -> std::string::String {
    if value.is_empty() {
        return std::string::String::new();
    }
    let without_slash = value.strip_suffix('/').unwrap_or(value);
    let default_port = if without_slash
        .get(..7)
        .is_some_and(|prefix| match_ignore_ascii_case!(prefix, "http://" => true, _ => false))
        || without_slash.starts_with("//")
    {
        Some(":80")
    } else if without_slash
        .get(..8)
        .is_some_and(|prefix| match_ignore_ascii_case!(prefix, "https://" => true, _ => false))
    {
        Some(":443")
    } else {
        None
    };
    let Some(port) = default_port.filter(|port| without_slash.ends_with(port)) else {
        return value.to_owned();
    };
    let mut normalized = std::string::String::with_capacity(value.len() - port.len());
    normalized.push_str(&without_slash[..without_slash.len() - port.len()]);
    if value.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

fn normalize_url_path(value: &str) -> std::string::String {
    if value.is_empty() {
        return std::string::String::new();
    }
    let leading_slash = value.starts_with('/');
    let trailing_slash = value.ends_with('/');
    let mut segments = std::vec::Vec::new();
    for segment in value.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if segments.last().is_some_and(|segment| *segment != "..") {
                    segments.pop();
                } else if !leading_slash {
                    segments.push(segment);
                }
            }
            _ => segments.push(segment),
        }
    }
    let mut normalized = std::string::String::with_capacity(value.len());
    if leading_slash {
        normalized.push('/');
    }
    for (index, segment) in segments.iter().enumerate() {
        if index != 0 {
            normalized.push('/');
        }
        normalized.push_str(segment);
    }
    if trailing_slash && !normalized.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

fn minify_cubic_bezier(arguments: &[TokenOrValue<'_>]) -> Option<&'static str> {
    let [a, comma_1, b, comma_2, c, comma_3, d] = arguments else {
        return None;
    };
    if !is_comma(comma_1) || !is_comma(comma_2) || !is_comma(comma_3) {
        return None;
    }
    match (
        token_number(a)?,
        token_number(b)?,
        token_number(c)?,
        token_number(d)?,
    ) {
        (0.25, 0.1, 0.25, 1.0) => Some("ease"),
        (0.0, 0.0, 1.0, 1.0) => Some("linear"),
        (0.42, 0.0, 1.0, 1.0) => Some("ease-in"),
        (0.0, 0.0, 0.58, 1.0) => Some("ease-out"),
        (0.42, 0.0, 0.58, 1.0) => Some("ease-in-out"),
        _ => None,
    }
}

fn minify_steps(
    arguments: &mut rocketcss_allocator::vec::Vec<'_, TokenOrValue<'_>>,
) -> Option<&'static str> {
    let [count, comma, position] = arguments.as_slice() else {
        return None;
    };
    if !is_comma(comma) {
        return None;
    }
    let position = token_ident(position)?;
    let is_start = match_ignore_ascii_case!(position, "start" | "jump-start" => true, _ => false);
    let is_end = match_ignore_ascii_case!(position, "end" | "jump-end" => true, _ => false);
    if token_number(count) == Some(1.0) {
        if is_start {
            return Some("step-start");
        }
        if is_end {
            return Some("step-end");
        }
    }
    if is_end {
        arguments.truncate(1);
    }
    None
}

fn token_number(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Number(value) => Some(value),
        Token::Dimension { value, .. } | Token::UnknownDimension { value, .. } => Some(value),
        _ => None,
    }
}

fn token_ident<'a>(value: &'a TokenOrValue<'a>) -> Option<&'a str> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Ident(value) => Some(value),
        _ => None,
    }
}

fn is_comma(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma))
}

impl Minify for Variable<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(cx);
        }
    }
}

impl Minify for EnvironmentVariable<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(cx);
        }
    }
}

impl Minify for UnknownAtRule<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.prelude.minify(cx);
        if let Some(block) = &mut self.block {
            block.minify(cx);
        }
    }
}
