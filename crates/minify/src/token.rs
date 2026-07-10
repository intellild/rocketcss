use rs_css_allocator::vec::Vec;
use rs_css_ast::{CssColor, DUMMY_SP, Function, Token, TokenOrValue, Url};

use crate::{MinifyContext, color, context::PropertyContext, length};

pub(crate) fn minify_token_or_value<'a>(
    value: &mut TokenOrValue<'a>,
    context: &mut MinifyContext<'a>,
) {
    if context.value_context.skip_value_transforms {
        return;
    }
    if minify_url_function(value, context) {
        context.record_value_normalized();
        return;
    }
    if minify_math_function(value, context) {
        context.record_value_normalized();
        return;
    }
    if minify_gradient_function(value, context) {
        context.record_value_normalized();
        return;
    }
    if context.value_context.allow_color
        && let Some(color) = color::parse_color_value(value)
    {
        *value = TokenOrValue::Color(context.allocator().boxed(CssColor::Rgba(color)));
        context.record_value_normalized();
        return;
    }

    if matches!(value, TokenOrValue::Function(_)) {
        let normalized = match context.value_context.property {
            PropertyContext::Timing => minify_timing_function(value, context),
            PropertyContext::Transform => minify_transform_function(value, context),
            _ => false,
        };
        if normalized {
            context.record_value_normalized();
            return;
        }
    }

    let TokenOrValue::Token(token) = value else {
        return;
    };
    match &mut **token {
        Token::Dimension { unit, value } => {
            if *value == 0.0
                && context.value_context.allow_unitless_zero
                && length::is_length_unit(unit)
            {
                **token = Token::Number(0.0);
                context.record_value_normalized();
            } else if let Some((number, normalized_unit)) = length::minify_dimension(*value, unit)
                && (number != *value || !unit.eq_ignore_ascii_case(normalized_unit))
            {
                *value = number;
                *unit = normalized_unit;
                context.record_value_normalized();
            }
        }
        Token::Percentage(value) if *value == 0.0 && context.value_context.allow_unitless_zero => {
            **token = Token::Number(0.0);
            context.record_value_normalized();
        }
        _ => {}
    }
}

fn minify_math_function<'a>(value: &mut TokenOrValue<'a>, context: &MinifyContext<'a>) -> bool {
    let TokenOrValue::Function(function) = value else {
        return false;
    };
    let Some(result) = evaluate_math_function(function) else {
        return false;
    };
    let token = match result {
        MathValue::Dimension(0.0, unit)
            if context.value_context.allow_unitless_zero && length::is_length_unit(unit) =>
        {
            Token::Number(0.0)
        }
        result => result.into_token(),
    };
    *value = TokenOrValue::Token(context.allocator().boxed(token));
    true
}

fn evaluate_math_function<'a>(function: &Function<'a>) -> Option<MathValue<'a>> {
    if function.name.eq_ignore_ascii_case("calc") {
        evaluate_calc(&function.arguments)
    } else if function.name.eq_ignore_ascii_case("min") || function.name.eq_ignore_ascii_case("max")
    {
        let groups = argument_groups(&function.arguments);
        let candidates = groups
            .iter()
            .map(|group| math_value(group))
            .collect::<Option<std::vec::Vec<_>>>()?;
        if candidates.is_empty() || !same_math_units(&candidates) {
            return None;
        }
        if function.name.eq_ignore_ascii_case("min") {
            candidates
                .into_iter()
                .min_by(|left, right| left.number().total_cmp(&right.number()))
        } else {
            candidates
                .into_iter()
                .max_by(|left, right| left.number().total_cmp(&right.number()))
        }
    } else if function.name.eq_ignore_ascii_case("clamp") {
        let groups = argument_groups(&function.arguments);
        let [minimum, preferred, maximum] = groups.as_slice() else {
            return None;
        };
        let values = [
            math_value(minimum)?,
            math_value(preferred)?,
            math_value(maximum)?,
        ];
        if !same_math_units(&values) {
            return None;
        }
        Some(
            values[1].with_number(
                values[1]
                    .number()
                    .clamp(values[0].number(), values[2].number()),
            ),
        )
    } else if function.name.eq_ignore_ascii_case("abs")
        || function.name.eq_ignore_ascii_case("sign")
    {
        let groups = argument_groups(&function.arguments);
        let [argument] = groups.as_slice() else {
            return None;
        };
        let argument = math_value(argument)?;
        if function.name.eq_ignore_ascii_case("abs") {
            Some(argument.with_number(argument.number().abs()))
        } else {
            Some(MathValue::Number(argument.number().signum()))
        }
    } else if function.name.eq_ignore_ascii_case("hypot") {
        let groups = argument_groups(&function.arguments);
        let candidates = groups
            .iter()
            .map(|group| math_value(group))
            .collect::<Option<std::vec::Vec<_>>>()?;
        if candidates.is_empty() || !same_math_units(&candidates) {
            return None;
        }
        Some(
            candidates[0].with_number(
                candidates
                    .iter()
                    .map(|value| value.number().powi(2))
                    .sum::<f32>()
                    .sqrt(),
            ),
        )
    } else if function.name.eq_ignore_ascii_case("rem") || function.name.eq_ignore_ascii_case("mod")
    {
        let groups = argument_groups(&function.arguments);
        let [left, right] = groups.as_slice() else {
            return None;
        };
        let values = [math_value(left)?, math_value(right)?];
        if !same_math_units(&values) || values[1].number() == 0.0 {
            return None;
        }
        let number = if function.name.eq_ignore_ascii_case("mod") {
            values[0].number().rem_euclid(values[1].number())
        } else {
            values[0].number() % values[1].number()
        };
        Some(values[0].with_number(number))
    } else {
        None
    }
}

#[derive(Clone, Copy)]
enum MathValue<'a> {
    Number(f32),
    Percentage(f32),
    Dimension(f32, &'a str),
}

impl<'a> MathValue<'a> {
    fn number(self) -> f32 {
        match self {
            Self::Number(value) | Self::Percentage(value) | Self::Dimension(value, _) => value,
        }
    }

    fn with_number(self, value: f32) -> Self {
        match self {
            Self::Number(_) => Self::Number(value),
            Self::Percentage(_) => Self::Percentage(value),
            Self::Dimension(_, unit) => Self::Dimension(value, unit),
        }
    }

    fn into_token(self) -> Token<'a> {
        match self {
            Self::Number(value) => Token::Number(value),
            Self::Percentage(value) => Token::Percentage(value),
            Self::Dimension(value, unit) => Token::Dimension { unit, value },
        }
    }
}

fn evaluate_calc<'a>(arguments: &[TokenOrValue<'a>]) -> Option<MathValue<'a>> {
    let values = arguments
        .iter()
        .filter(|value| !is_whitespace_or_comment(value))
        .collect::<std::vec::Vec<_>>();
    let [left, operator, right] = values.as_slice() else {
        return None;
    };
    let left = math_value_single(left)?;
    let right = math_value_single(right)?;
    let TokenOrValue::Token(operator) = operator else {
        return None;
    };
    let Token::Delim(operator) = **operator else {
        return None;
    };
    match operator {
        "+" | "-" if same_math_units(&[left, right]) => {
            Some(left.with_number(if operator == "+" {
                left.number() + right.number()
            } else {
                left.number() - right.number()
            }))
        }
        "*" => match (left, right) {
            (MathValue::Number(factor), value) | (value, MathValue::Number(factor)) => {
                Some(value.with_number(value.number() * factor))
            }
            _ => None,
        },
        "/" if matches!(right, MathValue::Number(_)) && right.number() != 0.0 => {
            Some(left.with_number(left.number() / right.number()))
        }
        _ => None,
    }
}

fn math_value<'a>(group: &[TokenOrValue<'a>]) -> Option<MathValue<'a>> {
    let [value] = group else {
        return None;
    };
    math_value_single(value)
}

fn math_value_single<'a>(value: &TokenOrValue<'a>) -> Option<MathValue<'a>> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Number(value) => Some(MathValue::Number(value)),
        Token::Percentage(value) => Some(MathValue::Percentage(value)),
        Token::Dimension { unit, value } => Some(MathValue::Dimension(value, unit)),
        _ => None,
    }
}

fn same_math_units(values: &[MathValue<'_>]) -> bool {
    let Some(first) = values.first() else {
        return false;
    };
    values.iter().skip(1).all(|value| match (*first, *value) {
        (MathValue::Number(_), MathValue::Number(_))
        | (MathValue::Percentage(_), MathValue::Percentage(_)) => true,
        (MathValue::Dimension(_, left), MathValue::Dimension(_, right)) => {
            left.eq_ignore_ascii_case(right)
        }
        _ => false,
    })
}

fn minify_url_function<'a>(value: &mut TokenOrValue<'a>, context: &MinifyContext<'a>) -> bool {
    let TokenOrValue::Function(function) = value else {
        return false;
    };
    if !function.name.eq_ignore_ascii_case("url") {
        return false;
    }
    let [TokenOrValue::Token(token)] = trim_group(&function.arguments) else {
        return false;
    };
    let Token::String(url) = **token else {
        return false;
    };
    *value = TokenOrValue::Url(context.allocator().boxed(Url {
        span: DUMMY_SP,
        url,
    }));
    true
}

fn minify_gradient_function<'a>(value: &mut TokenOrValue<'a>, context: &MinifyContext<'a>) -> bool {
    let TokenOrValue::Function(function) = value else {
        return false;
    };
    if !function.name.eq_ignore_ascii_case("linear-gradient")
        && !function
            .name
            .eq_ignore_ascii_case("repeating-linear-gradient")
    {
        return false;
    }
    let groups = argument_groups(&function.arguments);
    let Some(first) = groups.first() else {
        return false;
    };
    let identifiers = first
        .iter()
        .filter(|value| !is_whitespace_or_comment(value))
        .map(token_ident)
        .collect::<Option<std::vec::Vec<_>>>()
        .unwrap_or_default();
    let degrees = match identifiers.as_slice() {
        [to, direction] if to.eq_ignore_ascii_case("to") => {
            if direction.eq_ignore_ascii_case("top") {
                0.0
            } else if direction.eq_ignore_ascii_case("right") {
                90.0
            } else if direction.eq_ignore_ascii_case("bottom") {
                180.0
            } else if direction.eq_ignore_ascii_case("left") {
                270.0
            } else {
                return false;
            }
        }
        _ => return false,
    };
    replace_first_argument(
        &mut function.arguments,
        TokenOrValue::Token(context.allocator().boxed(Token::Dimension {
            unit: "deg",
            value: degrees,
        })),
        context,
    );
    true
}

fn replace_first_argument<'a>(
    arguments: &mut Vec<'a, TokenOrValue<'a>>,
    replacement: TokenOrValue<'a>,
    context: &MinifyContext<'a>,
) {
    let old = std::mem::replace(arguments, context.allocator().vec());
    let mut groups = split_owned(
        old,
        |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        context,
    );
    arguments.push(replacement);
    for group in groups.iter_mut().skip(1) {
        arguments.push(TokenOrValue::Token(context.allocator().boxed(Token::Comma)));
        arguments.extend(
            group
                .drain(..)
                .filter(|value| !is_whitespace_or_comment(value)),
        );
    }
}

pub(crate) fn minify_token_list<'a>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    context: &mut MinifyContext<'a>,
) {
    if !context.options().normalize_tokens || values.is_empty() {
        return;
    }

    let old = std::mem::replace(values, context.allocator().vec());
    let mut pending_space = false;
    for value in old {
        if is_whitespace_or_comment(&value) {
            pending_space = true;
            continue;
        }

        if let Some(previous) = values.last()
            && ((pending_space && whitespace_is_required(previous, &value))
                || tokens_would_merge(previous, &value))
        {
            values.push(TokenOrValue::Token(
                context.allocator().boxed(Token::WhiteSpace(" ")),
            ));
        }
        values.push(value);
        pending_space = false;
    }

    if context.options().normalize_values && !context.value_context.skip_value_transforms {
        match context.value_context.property {
            PropertyContext::Box => minify_box_sides(values, context),
            PropertyContext::Display => minify_display(values, context),
            PropertyContext::FontFamily => minify_font_family(values, context),
            PropertyContext::FontWeight => minify_font_weight(values, context),
            PropertyContext::Repeat => {
                minify_repeat_style(values, context);
                minify_position(values, context);
            }
            PropertyContext::Position => minify_position(values, context),
            _ => {}
        }
    }
}

fn minify_display<'a>(values: &mut Vec<'a, TokenOrValue<'a>>, context: &mut MinifyContext<'a>) {
    let groups = space_groups(values);
    let identifiers = groups
        .iter()
        .map(|group| group_ident(group))
        .collect::<Option<std::vec::Vec<_>>>();
    let Some(identifiers) = identifiers else {
        return;
    };
    let output: &[&'static str] = match identifiers.as_slice() {
        [block, flow]
            if block.eq_ignore_ascii_case("block") && flow.eq_ignore_ascii_case("flow") =>
        {
            &["block"]
        }
        [block, flow_root]
            if block.eq_ignore_ascii_case("block")
                && flow_root.eq_ignore_ascii_case("flow-root") =>
        {
            &["flow-root"]
        }
        [inline, flow]
            if inline.eq_ignore_ascii_case("inline") && flow.eq_ignore_ascii_case("flow") =>
        {
            &["inline"]
        }
        [inline, flow_root]
            if inline.eq_ignore_ascii_case("inline")
                && flow_root.eq_ignore_ascii_case("flow-root") =>
        {
            &["inline-block"]
        }
        [block, inside] if block.eq_ignore_ascii_case("block") => {
            if inside.eq_ignore_ascii_case("flex") {
                &["flex"]
            } else if inside.eq_ignore_ascii_case("grid") {
                &["grid"]
            } else if inside.eq_ignore_ascii_case("table") {
                &["table"]
            } else {
                return;
            }
        }
        [inline, inside] if inline.eq_ignore_ascii_case("inline") => {
            if inside.eq_ignore_ascii_case("flex") {
                &["inline-flex"]
            } else if inside.eq_ignore_ascii_case("grid") {
                &["inline-grid"]
            } else if inside.eq_ignore_ascii_case("table") {
                &["inline-table"]
            } else if inside.eq_ignore_ascii_case("ruby") {
                &["ruby"]
            } else {
                return;
            }
        }
        [list_item, block, flow]
            if list_item.eq_ignore_ascii_case("list-item")
                && block.eq_ignore_ascii_case("block")
                && flow.eq_ignore_ascii_case("flow") =>
        {
            &["list-item"]
        }
        [inline, flow, list_item]
            if inline.eq_ignore_ascii_case("inline")
                && flow.eq_ignore_ascii_case("flow")
                && list_item.eq_ignore_ascii_case("list-item") =>
        {
            &["inline", "list-item"]
        }
        [outer, flow]
            if flow.eq_ignore_ascii_case("flow")
                && matches_ignore_ascii_case(
                    outer,
                    &[
                        "run-in",
                        "table-cell",
                        "table-caption",
                        "ruby-base",
                        "ruby-text",
                    ],
                ) =>
        {
            let canonical = [
                "run-in",
                "table-cell",
                "table-caption",
                "ruby-base",
                "ruby-text",
            ]
            .into_iter()
            .find(|candidate| outer.eq_ignore_ascii_case(candidate))
            .expect("outer display value was matched above");
            set_ident_values(values, &[canonical], context);
            context.record_value_normalized();
            return;
        }
        _ => return,
    };
    set_ident_values(values, output, context);
    context.record_value_normalized();
}

fn minify_font_family<'a>(values: &mut Vec<'a, TokenOrValue<'a>>, context: &mut MinifyContext<'a>) {
    let old = std::mem::replace(values, context.allocator().vec());
    let groups = split_owned(
        old,
        |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        context,
    );
    let mut normalized: std::vec::Vec<Vec<'a, TokenOrValue<'a>>> = std::vec::Vec::new();
    for mut group in groups {
        if let [TokenOrValue::Token(token)] = group.as_slice()
            && let Token::String(family) = **token
            && can_unquote_font_family(family)
        {
            group.clear();
            for (index, word) in family.split_ascii_whitespace().enumerate() {
                if index > 0 {
                    group.push(TokenOrValue::Token(
                        context.allocator().boxed(Token::WhiteSpace(" ")),
                    ));
                }
                group.push(TokenOrValue::Token(
                    context.allocator().boxed(Token::Ident(word)),
                ));
            }
            context.record_value_normalized();
        }
        if !normalized.iter().any(|candidate| candidate == &group) {
            normalized.push(group);
        } else {
            context.record_value_normalized();
        }
    }
    for (index, mut group) in normalized.into_iter().enumerate() {
        if index > 0 {
            values.push(TokenOrValue::Token(context.allocator().boxed(Token::Comma)));
        }
        values.extend(group.drain(..));
    }
}

fn can_unquote_font_family(value: &str) -> bool {
    if value.is_empty() || value.trim() != value {
        return false;
    }
    let generic = [
        "serif",
        "sans-serif",
        "monospace",
        "cursive",
        "fantasy",
        "system-ui",
        "ui-serif",
        "ui-sans-serif",
        "ui-monospace",
        "ui-rounded",
        "math",
        "emoji",
        "fangsong",
    ];
    value.split_ascii_whitespace().all(|word| {
        !generic
            .iter()
            .any(|generic| word.eq_ignore_ascii_case(generic))
            && is_simple_identifier(word)
    })
}

fn is_simple_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    if first.is_ascii_digit()
        || !(first == '_' || first == '-' || first.is_alphabetic() || !first.is_ascii())
    {
        return false;
    }
    characters.all(|character| {
        character == '_' || character == '-' || character.is_alphanumeric() || !character.is_ascii()
    })
}

fn minify_font_weight<'a>(values: &mut Vec<'a, TokenOrValue<'a>>, context: &mut MinifyContext<'a>) {
    let [TokenOrValue::Token(token)] = values.as_slice() else {
        return;
    };
    let Token::Ident(value) = **token else {
        return;
    };
    let weight = if value.eq_ignore_ascii_case("normal") {
        400.0
    } else if value.eq_ignore_ascii_case("bold") {
        700.0
    } else {
        return;
    };
    values[0] = TokenOrValue::Token(context.allocator().boxed(Token::Number(weight)));
    context.record_value_normalized();
}

fn set_ident_values<'a>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    output: &[&'static str],
    context: &MinifyContext<'a>,
) {
    values.clear();
    for (index, value) in output.iter().copied().enumerate() {
        if index > 0 {
            values.push(TokenOrValue::Token(
                context.allocator().boxed(Token::WhiteSpace(" ")),
            ));
        }
        values.push(TokenOrValue::Token(
            context.allocator().boxed(Token::Ident(value)),
        ));
    }
}

fn minify_timing_function<'a>(value: &mut TokenOrValue<'a>, context: &MinifyContext<'a>) -> bool {
    let TokenOrValue::Function(function) = value else {
        return false;
    };
    if function.name.eq_ignore_ascii_case("cubic-bezier") {
        let Some(numbers) = argument_numbers(&function.arguments) else {
            return false;
        };
        let keyword = match numbers.as_slice() {
            [a, b, c, d] if near(*a, 0.25) && near(*b, 0.1) && near(*c, 0.25) && near(*d, 1.0) => {
                "ease"
            }
            [a, b, c, d] if near(*a, 0.0) && near(*b, 0.0) && near(*c, 1.0) && near(*d, 1.0) => {
                "linear"
            }
            [a, b, c, d] if near(*a, 0.42) && near(*b, 0.0) && near(*c, 1.0) && near(*d, 1.0) => {
                "ease-in"
            }
            [a, b, c, d] if near(*a, 0.0) && near(*b, 0.0) && near(*c, 0.58) && near(*d, 1.0) => {
                "ease-out"
            }
            [a, b, c, d] if near(*a, 0.42) && near(*b, 0.0) && near(*c, 0.58) && near(*d, 1.0) => {
                "ease-in-out"
            }
            _ => return false,
        };
        *value = TokenOrValue::Token(context.allocator().boxed(Token::Ident(keyword)));
        return true;
    }

    if function.name.eq_ignore_ascii_case("steps") {
        let groups = argument_groups(&function.arguments);
        if groups.is_empty() || groups.len() > 2 {
            return false;
        }
        let Some(count) = group_number(groups[0]) else {
            return false;
        };
        let position = groups.get(1).and_then(|group| group_ident(group));
        if near(count, 1.0)
            && position.is_some_and(|position| {
                position.eq_ignore_ascii_case("start")
                    || position.eq_ignore_ascii_case("jump-start")
            })
        {
            *value = TokenOrValue::Token(context.allocator().boxed(Token::Ident("step-start")));
            return true;
        }
        if near(count, 1.0)
            && position.is_some_and(|position| {
                position.eq_ignore_ascii_case("end") || position.eq_ignore_ascii_case("jump-end")
            })
        {
            *value = TokenOrValue::Token(context.allocator().boxed(Token::Ident("step-end")));
            return true;
        }
        if groups.len() == 2
            && position.is_some_and(|position| {
                position.eq_ignore_ascii_case("end") || position.eq_ignore_ascii_case("jump-end")
            })
        {
            retain_argument_groups(&mut function.arguments, &[0], context);
            function.name = "steps";
            return true;
        }
    }
    false
}

fn minify_transform_function<'a>(
    value: &mut TokenOrValue<'a>,
    context: &MinifyContext<'a>,
) -> bool {
    let TokenOrValue::Function(function) = value else {
        return false;
    };
    if function.name.eq_ignore_ascii_case("rotatez") {
        function.name = "rotate";
        return true;
    }

    let groups = argument_groups(&function.arguments);
    let numbers = groups
        .iter()
        .map(|group| group_number(group))
        .collect::<Option<std::vec::Vec<_>>>()
        .unwrap_or_default();

    if function.name.eq_ignore_ascii_case("matrix3d")
        && numbers.len() == 16
        && [2, 3, 6, 7, 8, 9, 11, 14]
            .into_iter()
            .all(|index| near(numbers[index], 0.0))
        && near(numbers[10], 1.0)
        && near(numbers[15], 1.0)
    {
        retain_argument_groups(&mut function.arguments, &[0, 1, 4, 5, 12, 13], context);
        function.name = "matrix";
        return true;
    }
    if function.name.eq_ignore_ascii_case("rotate3d") && groups.len() == 4 {
        let axes = groups[..3]
            .iter()
            .map(|group| group_number(group))
            .collect::<Option<std::vec::Vec<_>>>();
        let name = match axes.as_deref() {
            Some([x, y, z]) if near(*x, 1.0) && near(*y, 0.0) && near(*z, 0.0) => "rotateX",
            Some([x, y, z]) if near(*x, 0.0) && near(*y, 1.0) && near(*z, 0.0) => "rotateY",
            Some([x, y, z]) if near(*x, 0.0) && near(*y, 0.0) && near(*z, 1.0) => "rotate",
            _ => return false,
        };
        retain_argument_groups(&mut function.arguments, &[3], context);
        function.name = name;
        return true;
    }
    if function.name.eq_ignore_ascii_case("scale") && numbers.len() == 2 {
        let (name, selected): (&str, &[usize]) = if near(numbers[0], numbers[1]) {
            ("scale", &[0])
        } else if near(numbers[1], 1.0) {
            ("scaleX", &[0])
        } else if near(numbers[0], 1.0) {
            ("scaleY", &[1])
        } else {
            return false;
        };
        retain_argument_groups(&mut function.arguments, selected, context);
        function.name = name;
        return true;
    }
    if function.name.eq_ignore_ascii_case("scale3d") && numbers.len() == 3 {
        let (name, selected): (&str, &[usize]) = if near(numbers[1], 1.0) && near(numbers[2], 1.0) {
            ("scaleX", &[0])
        } else if near(numbers[0], 1.0) && near(numbers[2], 1.0) {
            ("scaleY", &[1])
        } else if near(numbers[0], 1.0) && near(numbers[1], 1.0) {
            ("scaleZ", &[2])
        } else {
            return false;
        };
        retain_argument_groups(&mut function.arguments, selected, context);
        function.name = name;
        return true;
    }
    if function.name.eq_ignore_ascii_case("translate") && groups.len() == 2 {
        let second = group_number(groups[1]);
        let first = group_number(groups[0]);
        if second.is_some_and(|number| near(number, 0.0)) {
            retain_argument_groups(&mut function.arguments, &[0], context);
            return true;
        }
        if first.is_some_and(|number| near(number, 0.0)) {
            retain_argument_groups(&mut function.arguments, &[1], context);
            function.name = "translateY";
            return true;
        }
    }
    if function.name.eq_ignore_ascii_case("translate3d") && groups.len() == 3 {
        let first = group_number(groups[0]);
        let second = group_number(groups[1]);
        if first.is_some_and(|number| near(number, 0.0))
            && second.is_some_and(|number| near(number, 0.0))
        {
            retain_argument_groups(&mut function.arguments, &[2], context);
            function.name = "translateZ";
            return true;
        }
    }
    false
}

fn minify_repeat_style<'a>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    context: &mut MinifyContext<'a>,
) {
    let mut index = 0;
    while index + 2 < values.len() {
        let Some(left) = token_ident(&values[index]) else {
            index += 1;
            continue;
        };
        if !is_whitespace_or_comment(&values[index + 1]) {
            index += 1;
            continue;
        }
        let Some(right) = token_ident(&values[index + 2]) else {
            index += 1;
            continue;
        };
        let replacement = if left.eq_ignore_ascii_case("repeat")
            && right.eq_ignore_ascii_case("no-repeat")
        {
            Some("repeat-x")
        } else if left.eq_ignore_ascii_case("no-repeat") && right.eq_ignore_ascii_case("repeat") {
            Some("repeat-y")
        } else if left.eq_ignore_ascii_case(right)
            && matches_ignore_ascii_case(left, &["repeat", "space", "round", "no-repeat"])
        {
            Some(match left.to_ascii_lowercase().as_str() {
                "repeat" => "repeat",
                "space" => "space",
                "round" => "round",
                _ => "no-repeat",
            })
        } else {
            None
        };
        if let Some(replacement) = replacement {
            values[index] =
                TokenOrValue::Token(context.allocator().boxed(Token::Ident(replacement)));
            values.remove(index + 1);
            values.remove(index + 1);
            context.record_value_normalized();
        } else {
            index += 1;
        }
    }
}

fn minify_box_sides<'a>(values: &mut Vec<'a, TokenOrValue<'a>>, context: &mut MinifyContext<'a>) {
    let groups = space_groups(values);
    if !(2..=4).contains(&groups.len()) || groups.iter().any(|group| group.is_empty()) {
        return;
    }
    let selected: &[usize] = match groups.len() {
        2 if groups[0] == groups[1] => &[0],
        3 if groups[0] == groups[1] && groups[1] == groups[2] => &[0],
        3 if groups[0] == groups[2] => &[0, 1],
        4 if groups[0] == groups[1] && groups[1] == groups[2] && groups[2] == groups[3] => &[0],
        4 if groups[0] == groups[2] && groups[1] == groups[3] => &[0, 1],
        4 if groups[1] == groups[3] => &[0, 1, 2],
        _ => return,
    };
    retain_space_groups(values, selected, context);
    context.record_value_normalized();
}

fn minify_position<'a>(values: &mut Vec<'a, TokenOrValue<'a>>, context: &mut MinifyContext<'a>) {
    let action = {
        let groups = space_groups(values);
        match groups.as_slice() {
            [single] => match group_ident(single).and_then(canonical_position_keyword) {
                Some("left") => PositionAction::SetOne(PositionOutput::Number(0.0)),
                Some("right") => PositionAction::SetOne(PositionOutput::Percentage(1.0)),
                Some("center") => PositionAction::SetOne(PositionOutput::Percentage(0.5)),
                _ => PositionAction::None,
            },
            [first, second] => {
                let first_keyword = group_ident(first).and_then(canonical_position_keyword);
                let second_keyword = group_ident(second).and_then(canonical_position_keyword);
                if second_keyword == Some("center") && first_keyword.is_none() {
                    PositionAction::RetainFirst
                } else if let (Some(first), Some(second)) = (first_keyword, second_keyword) {
                    let Some((horizontal, vertical)) = position_pair(first, second) else {
                        return;
                    };
                    if horizontal == "center" && matches!(vertical, "top" | "bottom") {
                        PositionAction::SetOne(PositionOutput::Ident(vertical))
                    } else {
                        let Some(x) = horizontal_position(horizontal) else {
                            return;
                        };
                        if vertical == "center" {
                            PositionAction::SetOne(x)
                        } else {
                            let Some(y) = vertical_position(vertical) else {
                                return;
                            };
                            PositionAction::SetTwo(x, y)
                        }
                    }
                } else {
                    PositionAction::None
                }
            }
            _ => PositionAction::None,
        }
    };
    match action {
        PositionAction::None => {}
        PositionAction::RetainFirst => {
            retain_space_groups(values, &[0], context);
            context.record_value_normalized();
        }
        PositionAction::SetOne(value) => {
            set_position(values, &[value], context);
            context.record_value_normalized();
        }
        PositionAction::SetTwo(first, second) => {
            set_position(values, &[first, second], context);
            context.record_value_normalized();
        }
    }
}

#[derive(Clone, Copy)]
enum PositionOutput {
    Ident(&'static str),
    Number(f32),
    Percentage(f32),
}

#[derive(Clone, Copy)]
enum PositionAction {
    None,
    RetainFirst,
    SetOne(PositionOutput),
    SetTwo(PositionOutput, PositionOutput),
}

fn canonical_position_keyword(value: &str) -> Option<&'static str> {
    ["left", "right", "top", "bottom", "center"]
        .into_iter()
        .find(|keyword| value.eq_ignore_ascii_case(keyword))
}

fn position_pair(
    first: &'static str,
    second: &'static str,
) -> Option<(&'static str, &'static str)> {
    let first_horizontal = matches!(first, "left" | "right");
    let first_vertical = matches!(first, "top" | "bottom");
    let second_horizontal = matches!(second, "left" | "right");
    let second_vertical = matches!(second, "top" | "bottom");
    if first_horizontal && (second_vertical || second == "center") {
        Some((first, second))
    } else if first_vertical && (second_horizontal || second == "center") {
        Some((second, first))
    } else if first == "center" && second_vertical {
        Some((first, second))
    } else if first == "center" && second_horizontal {
        Some((second, first))
    } else if first == "center" && second == "center" {
        Some((first, second))
    } else {
        None
    }
}

fn horizontal_position(value: &str) -> Option<PositionOutput> {
    if value == "left" {
        Some(PositionOutput::Number(0.0))
    } else if value == "right" {
        Some(PositionOutput::Percentage(1.0))
    } else if value == "center" {
        Some(PositionOutput::Percentage(0.5))
    } else {
        None
    }
}

fn vertical_position(value: &str) -> Option<PositionOutput> {
    if value == "top" {
        Some(PositionOutput::Number(0.0))
    } else if value == "bottom" {
        Some(PositionOutput::Percentage(1.0))
    } else if value == "center" {
        Some(PositionOutput::Percentage(0.5))
    } else {
        None
    }
}

fn set_position<'a>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    output: &[PositionOutput],
    context: &MinifyContext<'a>,
) {
    values.clear();
    for (index, value) in output.iter().copied().enumerate() {
        if index > 0 {
            values.push(TokenOrValue::Token(
                context.allocator().boxed(Token::WhiteSpace(" ")),
            ));
        }
        let token = match value {
            PositionOutput::Ident(value) => Token::Ident(value),
            PositionOutput::Number(value) => Token::Number(value),
            PositionOutput::Percentage(value) => Token::Percentage(value),
        };
        values.push(TokenOrValue::Token(context.allocator().boxed(token)));
    }
}

fn argument_numbers(arguments: &[TokenOrValue<'_>]) -> Option<std::vec::Vec<f32>> {
    argument_groups(arguments)
        .into_iter()
        .map(group_number)
        .collect()
}

fn argument_groups<'slice, 'a>(
    arguments: &'slice [TokenOrValue<'a>],
) -> std::vec::Vec<&'slice [TokenOrValue<'a>]> {
    let mut groups = std::vec::Vec::new();
    let mut start = 0;
    for (index, value) in arguments.iter().enumerate() {
        if matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)) {
            groups.push(trim_group(&arguments[start..index]));
            start = index + 1;
        }
    }
    groups.push(trim_group(&arguments[start..]));
    groups
}

fn group_number(group: &[TokenOrValue<'_>]) -> Option<f32> {
    match trim_group(group) {
        [TokenOrValue::Token(token)] => match **token {
            Token::Number(number) => Some(number),
            Token::Dimension { value, .. } => Some(value),
            _ => None,
        },
        _ => None,
    }
}

fn group_ident<'a>(group: &'a [TokenOrValue<'a>]) -> Option<&'a str> {
    let [TokenOrValue::Token(token)] = trim_group(group) else {
        return None;
    };
    match **token {
        Token::Ident(value) => Some(value),
        _ => None,
    }
}

fn trim_group<'slice, 'a>(mut group: &'slice [TokenOrValue<'a>]) -> &'slice [TokenOrValue<'a>] {
    while group.first().is_some_and(is_whitespace_or_comment) {
        group = &group[1..];
    }
    while group.last().is_some_and(is_whitespace_or_comment) {
        group = &group[..group.len() - 1];
    }
    group
}

fn retain_argument_groups<'a>(
    arguments: &mut Vec<'a, TokenOrValue<'a>>,
    selected: &[usize],
    context: &MinifyContext<'a>,
) {
    let old = std::mem::replace(arguments, context.allocator().vec());
    let mut groups = split_owned(
        old,
        |value| matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma)),
        context,
    );
    for (output_index, group_index) in selected.iter().copied().enumerate() {
        if output_index > 0 {
            arguments.push(TokenOrValue::Token(context.allocator().boxed(Token::Comma)));
        }
        arguments.extend(
            groups[group_index]
                .drain(..)
                .filter(|value| !is_whitespace_or_comment(value)),
        );
    }
}

fn space_groups<'a>(values: &'a [TokenOrValue<'a>]) -> std::vec::Vec<&'a [TokenOrValue<'a>]> {
    if values.iter().any(|value| {
        matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma | Token::Delim("/")))
    }) {
        return std::vec::Vec::new();
    }
    let mut groups = std::vec::Vec::new();
    let mut start = 0;
    for (index, value) in values.iter().enumerate() {
        if is_whitespace_or_comment(value) {
            if start < index {
                groups.push(&values[start..index]);
            }
            start = index + 1;
        }
    }
    if start < values.len() {
        groups.push(&values[start..]);
    }
    groups
}

fn retain_space_groups<'a>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    selected: &[usize],
    context: &MinifyContext<'a>,
) {
    let old = std::mem::replace(values, context.allocator().vec());
    let mut groups = split_owned(old, is_whitespace_or_comment, context);
    for (output_index, group_index) in selected.iter().copied().enumerate() {
        if output_index > 0 {
            values.push(TokenOrValue::Token(
                context.allocator().boxed(Token::WhiteSpace(" ")),
            ));
        }
        values.extend(groups[group_index].drain(..));
    }
}

fn split_owned<'a>(
    values: Vec<'a, TokenOrValue<'a>>,
    separator: impl Fn(&TokenOrValue<'a>) -> bool,
    context: &MinifyContext<'a>,
) -> std::vec::Vec<Vec<'a, TokenOrValue<'a>>> {
    let mut groups = std::vec::Vec::new();
    let mut current = context.allocator().vec();
    for value in values {
        if separator(&value) {
            if !current.is_empty() {
                groups.push(current);
                current = context.allocator().vec();
            }
        } else {
            current.push(value);
        }
    }
    if !current.is_empty() {
        groups.push(current);
    }
    groups
}

fn token_ident<'a>(value: &TokenOrValue<'a>) -> Option<&'a str> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Ident(value) => Some(value),
        _ => None,
    }
}

fn matches_ignore_ascii_case(value: &str, expected: &[&str]) -> bool {
    expected
        .iter()
        .any(|expected| value.eq_ignore_ascii_case(expected))
}

fn near(left: f32, right: f32) -> bool {
    (left - right).abs() < 0.000_001
}

fn is_whitespace_or_comment(value: &TokenOrValue<'_>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token)
            if matches!(**token, Token::WhiteSpace(_) | Token::Comment(_))
    )
}

fn whitespace_is_required(left: &TokenOrValue<'_>, right: &TokenOrValue<'_>) -> bool {
    !ends_with_open_punctuation(left) && !starts_with_close_punctuation(right)
}

fn ends_with_open_punctuation(value: &TokenOrValue<'_>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token)
            if matches!(
                **token,
                Token::Comma
                    | Token::Colon
                    | Token::Semicolon
                    | Token::ParenthesisBlock
                    | Token::SquareBracketBlock
                    | Token::CurlyBracketBlock
            ) || matches!(**token, Token::Delim("/"))
    )
}

fn starts_with_close_punctuation(value: &TokenOrValue<'_>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token)
            if matches!(
                **token,
                Token::Comma
                    | Token::Colon
                    | Token::Semicolon
                    | Token::CloseParenthesis
                    | Token::CloseSquareBracket
                    | Token::CloseCurlyBracket
            ) || matches!(**token, Token::Delim("/"))
    )
}

fn tokens_would_merge(left: &TokenOrValue<'_>, right: &TokenOrValue<'_>) -> bool {
    output_ends_with_name(left) && output_starts_with_name(right)
}

fn output_ends_with_name(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Color(_) | TokenOrValue::Length(_) | TokenOrValue::Angle(_) => true,
        TokenOrValue::Time(_)
        | TokenOrValue::Resolution(_)
        | TokenOrValue::DashedIdent(_)
        | TokenOrValue::AnimationName(_) => true,
        TokenOrValue::Token(token) => matches!(
            **token,
            Token::Ident(_)
                | Token::AtKeyword(_)
                | Token::Hash(_)
                | Token::IdHash(_)
                | Token::Number(_)
                | Token::Percentage(_)
                | Token::Dimension { .. }
        ),
        TokenOrValue::Url(_)
        | TokenOrValue::Var(_)
        | TokenOrValue::Env(_)
        | TokenOrValue::Function(_)
        | TokenOrValue::UnresolvedColor(_) => false,
    }
}

fn output_starts_with_name(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Token(token) => matches!(
            **token,
            Token::Ident(_)
                | Token::AtKeyword(_)
                | Token::Hash(_)
                | Token::IdHash(_)
                | Token::Number(_)
                | Token::Percentage(_)
                | Token::Dimension { .. }
                | Token::Function(_)
        ),
        TokenOrValue::Color(_)
        | TokenOrValue::UnresolvedColor(_)
        | TokenOrValue::Url(_)
        | TokenOrValue::Var(_)
        | TokenOrValue::Env(_)
        | TokenOrValue::Function(_)
        | TokenOrValue::Length(_)
        | TokenOrValue::Angle(_)
        | TokenOrValue::Time(_)
        | TokenOrValue::Resolution(_)
        | TokenOrValue::DashedIdent(_)
        | TokenOrValue::AnimationName(_) => true,
    }
}
