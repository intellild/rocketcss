use rocketcss_allocator::vec::Vec;
use rocketcss_ast::{KnownFunction, Token, TokenOrValue, Unit, match_ignore_ascii_case};

use crate::{
    Minify, MinifyContext, Options, OptionsOp,
    context::{PropertyContext, ValueContextFlags},
    length,
};

impl Minify for TokenOrValue<'_> {
    /// Normalizes one token node in place. The surrounding `TokenOrValue`
    /// variant and its arena allocation are preserved.
    fn minify(&mut self, cx: &mut MinifyContext) {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None)
            || cx
                .value_context
                .is_enabled(ValueContextFlags::SKIP_VALUE_TRANSFORMS)
        {
            return;
        }

        let TokenOrValue::Token(token) = self else {
            return;
        };
        match &mut **token {
            Token::String(value)
                if cx.value_context.property == PropertyContext::Font
                    && can_unquote_font(value) =>
            {
                **token = Token::UnquotedFont(value);
                cx.record_value_normalized();
            }
            Token::Hash(value) | Token::IdHash(value)
                if cx
                    .value_context
                    .is_enabled(ValueContextFlags::MINIFY_COLORS)
                    && is_hex_color(value) =>
            {
                **token = minify_hex_color(value);
                cx.record_value_normalized();
            }
            Token::Ident(value)
                if cx
                    .value_context
                    .is_enabled(ValueContextFlags::MINIFY_COLORS) =>
            {
                let Some(replacement) = minify_color_keyword(value) else {
                    return;
                };
                **token = replacement;
                cx.record_value_normalized();
            }
            Token::Dimension { unit, value } => {
                if *value == 0.0
                    && cx
                        .value_context
                        .is_enabled(ValueContextFlags::ALLOW_UNITLESS_ZERO_LENGTH)
                    && unit.is_length()
                {
                    **token = Token::Number(0.0);
                    cx.record_value_normalized();
                } else if let Some((number, normalized_unit)) =
                    length::minify_dimension(*value, *unit, cx)
                    && (number != *value || normalized_unit != *unit)
                {
                    *value = number;
                    *unit = normalized_unit;
                }
            }
            Token::Percentage(value)
                if *value == 0.0
                    && cx
                        .value_context
                        .is_enabled(ValueContextFlags::ALLOW_UNITLESS_ZERO_PERCENTAGE) =>
            {
                **token = Token::Number(0.0);
                cx.record_value_normalized();
            }
            Token::UnknownDimension { unit: ".", value } => {
                **token = Token::Number(*value);
                cx.record_value_normalized();
            }
            Token::UnknownDimension { unit, value }
                if unit.strip_prefix('.').is_some_and(
                    |unit| match_ignore_ascii_case!(unit, "px" => true, _ => false),
                ) =>
            {
                **token = Token::Dimension {
                    unit: Unit::Length(rocketcss_ast::LengthUnit::Px),
                    value: *value,
                };
                cx.record_value_normalized();
            }
            _ => {}
        }
    }
}

fn is_hex_color(value: &str) -> bool {
    matches!(value.len(), 3 | 4 | 6 | 8) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn minify_hex_color<'a>(value: &'a str) -> Token<'a> {
    match_ignore_ascii_case!(
        value,
        "ff0000" | "f00" => Token::Ident("red"),
        "f0ffff" => Token::Ident("azure"),
        "808080" => Token::Ident("gray"),
        _ => Token::MinifiedHash(value),
    )
}

fn minify_color_keyword(value: &str) -> Option<Token<'static>> {
    match_ignore_ascii_case!(
        value,
        "red" => Some(Token::Ident("red")),
        "blue" => Some(Token::Ident("blue")),
        "black" => Some(Token::MinifiedHash("000")),
        "white" => Some(Token::MinifiedHash("fff")),
        "yellow" => Some(Token::MinifiedHash("ff0")),
        "fuchsia" | "magenta" => Some(Token::MinifiedHash("f0f")),
        "lightgreen" => Some(Token::MinifiedHash("90ee90")),
        "grey" => Some(Token::Ident("grey")),
        _ => None,
    )
}

pub(crate) fn can_unquote_font(value: &str) -> bool {
    let Some(first) = value.chars().find(|character| !character.is_whitespace()) else {
        return false;
    };
    if first.is_ascii_digit()
        || value.chars().any(|character| {
            !character.is_ascii()
                || character.is_control()
                || matches!(character, '\\' | '"' | '\'')
        })
        || value.split_ascii_whitespace().any(is_generic_font_family)
    {
        return false;
    }
    let mut escaped_length = value.len();
    let mut characters = value.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        if character == ' ' {
            if characters.peek().is_some()
                && (index == 0
                    || characters
                        .peek()
                        .is_some_and(|(_, next)| next.is_ascii_digit()))
            {
                escaped_length += 1;
            }
        } else if !(character.is_ascii_alphanumeric() || matches!(character, '-' | '_')) {
            escaped_length += 1;
        }
    }
    escaped_length < value.len() + 2
}

fn is_generic_font_family(value: &str) -> bool {
    match_ignore_ascii_case!(
        value,
        "serif" | "sans-serif" | "monospace" | "cursive" | "fantasy" | "system-ui" | "ui-serif" | "ui-sans-serif" | "ui-monospace" | "ui-rounded" | "emoji" | "math" | "fangsong" => true,
        _ => false,
    )
}

impl<'a> Minify for Vec<'a, TokenOrValue<'a>> {
    /// Removes comments and redundant whitespace by compacting the existing
    /// arena vector. Separator tokens are reused rather than allocated again.
    fn minify(&mut self, cx: &mut MinifyContext) {
        protect_adjacent_function_replacements(self);
        if cx.is_enabled(Options::DISCARD_COMMENTS, OptionsOp::Any) {
            discard_comments(self, cx);
        }
        if cx.is_enabled(Options::NORMALIZE_WHITESPACE, OptionsOp::Any) {
            normalize_whitespace(self, cx);
        }
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None)
            || cx
                .value_context
                .is_enabled(ValueContextFlags::SKIP_VALUE_TRANSFORMS)
        {
            return;
        }

        if minify_broken_decimal_tokens(self) {
            cx.record_value_normalized();
        }
        if cx.is_enabled(Options::NORMALIZE_URLS, OptionsOp::Any) && normalize_url_values(self) {
            cx.record_value_normalized();
        }

        match cx.value_context.property {
            PropertyContext::Animation => minify_animation(self, cx),
            PropertyContext::Border | PropertyContext::Outline => minify_ordered_border(self, cx),
            PropertyContext::Box => minify_box_sides(self, cx),
            PropertyContext::BoxShadow => minify_box_shadow(self, cx),
            PropertyContext::Columns => minify_ordered_columns(self, cx),
            PropertyContext::Display => minify_display(self, cx),
            PropertyContext::FlexFlow => minify_flex_flow(self, cx),
            PropertyContext::Font => minify_font(self, cx),
            PropertyContext::FontWeight => minify_font_weight(self, cx),
            PropertyContext::GridAutoFlow => minify_grid_auto_flow(self, cx),
            PropertyContext::GridGap => minify_grid_gap(self, cx),
            PropertyContext::GridLine => minify_grid_line(self, cx),
            PropertyContext::ListStyle => minify_list_style(self, cx),
            PropertyContext::Position => {
                minify_positions(self, cx);
                minify_repeat_style(self, cx);
            }
            PropertyContext::Repeat => minify_repeat_style(self, cx),
            PropertyContext::TimingFunction => {}
            PropertyContext::Transform => {}
            PropertyContext::Transition => minify_transition(self, cx),
            PropertyContext::Generic => {}
        }
    }
}

fn normalize_url_values<'a>(values: &mut Vec<'a, TokenOrValue<'a>>) -> bool {
    let allocator = values.bump();
    let mut changed = false;
    for value in values {
        let TokenOrValue::Url(url) = value else {
            continue;
        };
        if let Some(normalized) = crate::rules::normalize_url_text(url.url) {
            url.url = allocator.alloc_str(&normalized);
            changed = true;
        }
    }
    changed
}

fn minify_broken_decimal_tokens(values: &mut Vec<'_, TokenOrValue<'_>>) -> bool {
    let mut changed = false;
    let mut index = 0;
    while index + 1 < values.len() {
        let is_number = matches!(values[index], TokenOrValue::Token(ref token) if matches!(**token, Token::Number(_)));
        let is_dot = matches!(values[index + 1], TokenOrValue::Token(ref token) if matches!(&**token, Token::Delim(value) if *value == "."));
        if !is_number || !is_dot {
            index += 1;
            continue;
        }
        if values.get(index + 2).is_some_and(|value| {
            matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::Ident(unit) if match_ignore_ascii_case!(unit, "px" => true, _ => false)))
        }) {
            let TokenOrValue::Token(token) = &mut values[index] else {
                unreachable!()
            };
            let Token::Number(value) = **token else {
                unreachable!()
            };
            **token = Token::Dimension {
                unit: Unit::Length(rocketcss_ast::LengthUnit::Px),
                value,
            };
            values.drain(index + 1..=index + 2);
            changed = true;
            continue;
        }
        let next_is_boundary = values.get(index + 2).is_none_or(|value| {
            matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_) | Token::Comma | Token::Semicolon | Token::CloseParenthesis))
        });
        if next_is_boundary {
            values.remove(index + 1);
            changed = true;
            continue;
        }
        index += 1;
    }
    changed
}

fn protect_adjacent_function_replacements(values: &mut Vec<'_, TokenOrValue<'_>>) {
    for index in 0..values.len() {
        let has_unsafe_neighbor = values.get(index.wrapping_sub(1)).is_some_and(|value| {
            !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_) | Token::Comma))
        });
        if !has_unsafe_neighbor {
            continue;
        }
        let TokenOrValue::Function(function) = &mut values[index] else {
            continue;
        };
        if matches!(
            function.replacement,
            Some(rocketcss_ast::FunctionReplacement::Rgb { .. })
        ) {
            function.replacement = None;
        }
    }
}

fn minify_display(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let replacement = match values.as_slice() {
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "block", "flow") =>
        {
            Some("block")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "block", "flow-root") =>
        {
            Some("flow-root")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "inline", "flow") =>
        {
            Some("inline")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "inline", "flow-root") =>
        {
            Some("inline-block")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "run-in", "flow") =>
        {
            Some("run-in")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "block", "flex") =>
        {
            Some("flex")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "inline", "flex") =>
        {
            Some("inline-flex")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "block", "grid") =>
        {
            Some("grid")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "inline", "grid") =>
        {
            Some("inline-grid")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "inline", "ruby") =>
        {
            Some("ruby")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "block", "table") =>
        {
            Some("table")
        }
        [first, space, second]
            if is_whitespace(space) && ident_pair(first, second, "inline", "table") =>
        {
            Some("inline-table")
        }
        [first, space, second]
            if is_whitespace(space)
                && (["table-cell", "table-caption", "ruby-base", "ruby-text"]
                    .iter()
                    .any(|keyword| ident_pair(first, second, keyword, "flow"))) =>
        {
            ["table-cell", "table-caption", "ruby-base", "ruby-text"]
                .into_iter()
                .find(|keyword| ident_pair(first, second, keyword, "flow"))
        }
        [first, space_1, second, space_2, third]
            if is_whitespace(space_1)
                && is_whitespace(space_2)
                && token_ident(first).is_some_and(
                    |value| match_ignore_ascii_case!(value, "list-item" => true, _ => false),
                )
                && token_ident(second).is_some_and(
                    |value| match_ignore_ascii_case!(value, "block" => true, _ => false),
                )
                && token_ident(third).is_some_and(
                    |value| match_ignore_ascii_case!(value, "flow" => true, _ => false),
                ) =>
        {
            Some("list-item")
        }
        [first, space_1, second, space_2, third]
            if is_whitespace(space_1)
                && is_whitespace(space_2)
                && token_ident(first).is_some_and(
                    |value| match_ignore_ascii_case!(value, "inline" => true, _ => false),
                )
                && token_ident(second).is_some_and(
                    |value| match_ignore_ascii_case!(value, "flow" => true, _ => false),
                )
                && token_ident(third).is_some_and(
                    |value| match_ignore_ascii_case!(value, "list-item" => true, _ => false),
                ) =>
        {
            let TokenOrValue::Token(token) = &mut values[2] else {
                unreachable!()
            };
            **token = Token::Ident("list-item");
            values.truncate(3);
            cx.record_value_normalized();
            return;
        }
        _ => None,
    };
    let Some(replacement) = replacement else {
        return;
    };
    let TokenOrValue::Token(token) = &mut values[0] else {
        return;
    };
    **token = Token::Ident(replacement);
    values.truncate(1);
    cx.record_value_normalized();
}

fn ident_pair(
    first: &TokenOrValue<'_>,
    second: &TokenOrValue<'_>,
    expected_first: &str,
    expected_second: &str,
) -> bool {
    token_ident(first).is_some_and(|value| value.eq_ignore_ascii_case(expected_first))
        && token_ident(second).is_some_and(|value| value.eq_ignore_ascii_case(expected_second))
}

fn minify_positions(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut layer_start = 0;
    while layer_start < values.len() {
        let layer_end = values[layer_start..]
            .iter()
            .position(is_comma)
            .map_or(values.len(), |index| layer_start + index);
        if minify_position_layer(values, layer_start, layer_end) {
            cx.record_value_normalized();
        }
        let Some(comma) = values[layer_start..].iter().position(is_comma) else {
            break;
        };
        layer_start += comma + 1;
    }
}

fn minify_position_layer(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    layer_start: usize,
    layer_end: usize,
) -> bool {
    let mut start = None;
    let mut end = None;
    for index in layer_start..layer_end {
        if is_slash(&values[index]) {
            break;
        }
        if is_variable(&values[index]) {
            return false;
        }
        if is_position_component(&values[index]) {
            start.get_or_insert(index);
            end = Some(index);
        }
    }

    let (Some(start), Some(end)) = (start, end) else {
        return false;
    };
    if end - start > 2 {
        return false;
    }
    if start == end {
        return normalize_horizontal_keyword(&mut values[start]);
    }
    if end != start + 2 || !is_whitespace(&values[start + 1]) {
        return false;
    }

    let Some(second) = token_ident(&values[end]) else {
        return false;
    };
    let second = position_keyword(second);

    if second == Some(PositionKeyword::Center) {
        normalize_horizontal_keyword(&mut values[start]);
        drop(values.drain(start + 1..=end));
        return true;
    }
    let Some(first) = token_ident(&values[start]) else {
        return false;
    };
    let first = position_keyword(first);
    if first == Some(PositionKeyword::Center) {
        if matches!(second, Some(PositionKeyword::Left | PositionKeyword::Right)) {
            set_position_keyword(&mut values[end], second.expect("matched above"));
        }
        drop(values.drain(start..end));
        return second.is_some();
    }

    match (first, second) {
        (
            Some(horizontal @ (PositionKeyword::Left | PositionKeyword::Right)),
            Some(vertical @ (PositionKeyword::Top | PositionKeyword::Bottom)),
        ) => {
            set_position_keyword(&mut values[start], horizontal);
            set_position_keyword(&mut values[end], vertical);
            true
        }
        (
            Some(vertical @ (PositionKeyword::Top | PositionKeyword::Bottom)),
            Some(horizontal @ (PositionKeyword::Left | PositionKeyword::Right)),
        ) => {
            set_position_keyword(&mut values[start], horizontal);
            set_position_keyword(&mut values[end], vertical);
            true
        }
        _ => false,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PositionKeyword {
    Top,
    Right,
    Bottom,
    Left,
    Center,
}

fn position_keyword(value: &str) -> Option<PositionKeyword> {
    match_ignore_ascii_case!(
        value,
        "top" => Some(PositionKeyword::Top),
        "right" => Some(PositionKeyword::Right),
        "bottom" => Some(PositionKeyword::Bottom),
        "left" => Some(PositionKeyword::Left),
        "center" => Some(PositionKeyword::Center),
        _ => None,
    )
}

fn normalize_horizontal_keyword(value: &mut TokenOrValue<'_>) -> bool {
    let Some(keyword) = token_ident(value).and_then(position_keyword) else {
        return false;
    };
    if matches!(
        keyword,
        PositionKeyword::Left | PositionKeyword::Right | PositionKeyword::Center
    ) {
        set_position_keyword(value, keyword);
        true
    } else {
        false
    }
}

fn set_position_keyword(value: &mut TokenOrValue<'_>, keyword: PositionKeyword) {
    let TokenOrValue::Token(token) = value else {
        unreachable!("position keyword was classified as a token")
    };
    **token = match keyword {
        PositionKeyword::Top | PositionKeyword::Left => Token::Number(0.0),
        PositionKeyword::Right | PositionKeyword::Bottom => Token::Percentage(1.0),
        PositionKeyword::Center => Token::Percentage(0.5),
    };
}

fn is_position_component(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Token(token) => {
            matches!(
                &**token,
                Token::Ident(value) if position_keyword(value).is_some()
            ) || matches!(
                **token,
                Token::Number(_)
                    | Token::Percentage(_)
                    | Token::Dimension { .. }
                    | Token::UnknownDimension { .. }
            )
        }
        TokenOrValue::Function(function) => {
            function.kind().is_math_value() && !function.is_vendor_prefixed()
        }
        TokenOrValue::Length(_) => true,
        _ => false,
    }
}

fn is_variable(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Var(_) | TokenOrValue::Env(_) => true,
        TokenOrValue::Function(function) => function.kind().is_variable(),
        _ => false,
    }
}

fn is_comma(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma))
}

fn is_slash(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Delim("/")))
}

fn discard_comments(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut index = 0;
    while index < values.len() {
        if !matches!(&values[index], TokenOrValue::Token(token) if matches!(**token, Token::Comment(_)))
        {
            index += 1;
            continue;
        }

        let needs_separator = index > 0
            && index + 1 < values.len()
            && !is_whitespace_or_comment(&values[index - 1])
            && !is_whitespace_or_comment(&values[index + 1])
            && whitespace_is_required(&values[index - 1], &values[index + 1]);
        if needs_separator {
            let TokenOrValue::Token(token) = &mut values[index] else {
                unreachable!("comments are tokens")
            };
            **token = Token::WhiteSpace(" ");
            index += 1;
        } else {
            values.remove(index);
        }
        cx.record_value_normalized();
    }
}

fn normalize_whitespace(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut index = 0;
    while index < values.len() {
        if !is_whitespace(&values[index]) {
            index += 1;
            continue;
        }

        let start = index;
        let mut end = start + 1;
        while end < values.len() && is_whitespace(&values[end]) {
            end += 1;
        }

        let keep_space = start > 0
            && end < values.len()
            && (whitespace_is_required(&values[start - 1], &values[end])
                || multiplication_before_parentheses(values, start, end)
                || (cx
                    .value_context
                    .is_enabled(ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA)
                    && is_comma(&values[start - 1])));
        if keep_space {
            let TokenOrValue::Token(token) = &mut values[start] else {
                unreachable!("separator nodes are tokens")
            };
            let was_normalized_space = matches!(**token, Token::WhiteSpace(" "));
            **token = Token::WhiteSpace(" ");
            if end > start + 1 {
                drop(values.drain(start + 1..end));
                cx.record_value_normalized();
            } else if !was_normalized_space {
                cx.record_value_normalized();
            }
            index = start + 1;
        } else {
            drop(values.drain(start..end));
            cx.record_value_normalized();
            index = start;
        }
    }
}

fn multiplication_before_parentheses(
    values: &[TokenOrValue<'_>],
    whitespace_start: usize,
    whitespace_end: usize,
) -> bool {
    let before = &values[whitespace_start - 1];
    let after = &values[whitespace_end];
    if is_delim(before, "*") && is_open_parenthesis(after) {
        return true;
    }
    is_delim(after, "*")
        && values
            .get(whitespace_end + 1..)
            .and_then(|values| values.iter().find(|value| !is_whitespace_or_comment(value)))
            .is_some_and(is_open_parenthesis)
}

fn is_delim(value: &TokenOrValue<'_>, expected: &str) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(&**token, Token::Delim(value) if *value == expected))
}

fn is_open_parenthesis(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::ParenthesisBlock))
}

fn minify_transition(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut start = 0;
    let mut changed = false;
    loop {
        let end = values[start..]
            .iter()
            .position(is_comma)
            .map_or(values.len(), |index| start + index);
        changed |= sort_transition_layer(values, start, end);
        if end == values.len() {
            break;
        }
        start = end + 1;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn sort_transition_layer(values: &mut Vec<'_, TokenOrValue<'_>>, start: usize, end: usize) -> bool {
    let Some((items, count)) = collect_layer_items(values, start, end) else {
        return false;
    };
    let mut ranks = [0u8; 16];
    let mut time_count = 0;
    for position in 0..count {
        let value = &values[items[position]];
        ranks[position] = if is_time_value(value) {
            time_count += 1;
            if time_count == 1 { 1 } else { 3 }
        } else if is_timing_value(value) {
            2
        } else if token_ident(value).is_some_and(|value| {
            match_ignore_ascii_case!(value, "normal" | "allow-discrete" => true, _ => false)
        }) {
            4
        } else if is_variable(value) {
            return false;
        } else {
            0
        };
    }
    sort_items_with_ranks(values, items, ranks, count)
}

fn minify_animation(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut start = 0;
    let mut changed = false;
    loop {
        let end = values[start..]
            .iter()
            .position(is_comma)
            .map_or(values.len(), |index| start + index);
        changed |= sort_animation_layer(values, start, end);
        if end == values.len() {
            break;
        }
        start = end + 1;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn sort_animation_layer(values: &mut Vec<'_, TokenOrValue<'_>>, start: usize, end: usize) -> bool {
    let Some((items, count)) = collect_layer_items(values, start, end) else {
        return false;
    };
    let mut ranks = [0u8; 16];
    let mut time_count = 0;
    let last_timing = (0..count)
        .rev()
        .find(|&position| is_timing_value(&values[items[position]]));
    let last_direction = (0..count).rev().find(|&position| {
        token_ident(&values[items[position]]).is_some_and(is_animation_direction)
    });
    let last_fill = (0..count).rev().find(|&position| {
        token_ident(&values[items[position]]).is_some_and(is_animation_fill_mode)
    });
    let last_play = (0..count).rev().find(|&position| {
        token_ident(&values[items[position]]).is_some_and(is_animation_play_state)
    });
    for position in 0..count {
        let value = &values[items[position]];
        ranks[position] = if is_time_value(value) {
            time_count += 1;
            if time_count == 1 { 1 } else { 3 }
        } else if is_timing_value(value) && last_timing == Some(position) {
            2
        } else if matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Number(_)))
            || token_ident(value).is_some_and(
                |value| match_ignore_ascii_case!(value, "infinite" => true, _ => false),
            )
        {
            4
        } else if token_ident(value).is_some_and(is_animation_direction)
            && last_direction == Some(position)
        {
            5
        } else if token_ident(value).is_some_and(is_animation_fill_mode)
            && last_fill == Some(position)
        {
            6
        } else if token_ident(value).is_some_and(is_animation_play_state)
            && last_play == Some(position)
        {
            7
        } else if is_variable(value) {
            return false;
        } else {
            0
        };
    }
    sort_items_with_ranks(values, items, ranks, count)
}

fn is_animation_direction(value: &str) -> bool {
    match_ignore_ascii_case!(
        value,
        "normal" | "reverse" | "alternate" | "alternate-reverse" => true,
        _ => false,
    )
}

fn is_animation_fill_mode(value: &str) -> bool {
    match_ignore_ascii_case!(
        value,
        "none" | "forwards" | "backwards" | "both" => true,
        _ => false,
    )
}

fn is_animation_play_state(value: &str) -> bool {
    match_ignore_ascii_case!(value, "running" | "paused" => true, _ => false)
}

fn is_time_value(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Time(_))
        || matches!(value, TokenOrValue::Token(token)
            if matches!(**token, Token::Dimension { unit: Unit::Seconds | Unit::Milliseconds, .. }))
}

fn is_timing_value(value: &TokenOrValue<'_>) -> bool {
    token_ident(value).is_some_and(|value| {
        match_ignore_ascii_case!(
            value,
            "linear" | "ease" | "ease-in" | "ease-out" | "ease-in-out" | "step-start" | "step-end" => true,
            _ => false,
        )
    }) || matches!(value, TokenOrValue::Function(function)
        if matches!(
            function.kind(),
            KnownFunction::Steps
                | KnownFunction::CubicBezier
                | KnownFunction::Linear
                | KnownFunction::Frames
        ))
}

fn collect_layer_items(
    values: &[TokenOrValue<'_>],
    start: usize,
    end: usize,
) -> Option<([usize; 16], usize)> {
    let mut items = [0usize; 16];
    let mut count = 0;
    for (index, value) in values.iter().enumerate().take(end).skip(start) {
        if is_whitespace(value) {
            continue;
        }
        if count == items.len() {
            return None;
        }
        items[count] = index;
        count += 1;
    }
    Some((items, count))
}

fn sort_items_with_ranks(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    items: [usize; 16],
    mut ranks: [u8; 16],
    count: usize,
) -> bool {
    let mut changed = false;
    for right in 1..count {
        let mut current = right;
        while current > 0 && ranks[current - 1] > ranks[current] {
            values.swap(items[current - 1], items[current]);
            ranks.swap(current - 1, current);
            current -= 1;
            changed = true;
        }
    }
    changed
}

fn minify_grid_auto_flow(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let [first, space, second] = values.as_slice() else {
        return;
    };
    if is_whitespace(space)
        && token_ident(first)
            .is_some_and(|value| match_ignore_ascii_case!(value, "dense" => true, _ => false))
        && token_ident(second).is_some_and(
            |value| match_ignore_ascii_case!(value, "row" | "column" => true, _ => false),
        )
    {
        values.swap(0, 2);
        cx.record_value_normalized();
    }
}

fn minify_grid_gap(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let [first, space, second] = values.as_slice() else {
        return;
    };
    if is_whitespace(space)
        && !token_ident(first)
            .is_some_and(|value| match_ignore_ascii_case!(value, "normal" => true, _ => false))
        && token_ident(second)
            .is_some_and(|value| match_ignore_ascii_case!(value, "normal" => true, _ => false))
    {
        values.swap(0, 2);
        cx.record_value_normalized();
    }
}

fn minify_grid_line(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut changed = false;
    let mut index = 0;
    while index + 2 < values.len() {
        if matches!(&values[index], TokenOrValue::Token(token) if matches!(**token, Token::Number(_)))
            && is_whitespace(&values[index + 1])
            && token_ident(&values[index + 2])
                .is_some_and(|value| match_ignore_ascii_case!(value, "span" => true, _ => false))
        {
            values.swap(index, index + 2);
            changed = true;
        }
        index += 1;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn minify_list_style(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    if sort_layer_by_rank(values, 0, values.len(), list_style_rank) {
        cx.record_value_normalized();
    }
}

fn list_style_rank(value: &TokenOrValue<'_>) -> Option<u8> {
    if token_ident(value).is_some_and(
        |value| match_ignore_ascii_case!(value, "inside" | "outside" => true, _ => false),
    ) {
        Some(1)
    } else if matches!(value, TokenOrValue::Url(_))
        || matches!(value, TokenOrValue::Function(function) if function.kind() == KnownFunction::Url)
    {
        Some(2)
    } else if is_variable(value) {
        None
    } else {
        Some(0)
    }
}

fn minify_ordered_columns(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let Some((items, count)) = collect_layer_items(values, 0, values.len()) else {
        return;
    };
    if count == 2
        && token_ident(&values[items[0]])
            .is_some_and(|value| match_ignore_ascii_case!(value, "auto" => true, _ => false))
        && token_ident(&values[items[1]])
            .is_some_and(|value| match_ignore_ascii_case!(value, "auto" => true, _ => false))
    {
        values.truncate(items[0] + 1);
        cx.record_value_normalized();
        return;
    }
    let mut changed = false;
    if count > 2 {
        values.truncate(items[1] + 1);
        changed = true;
    }
    if count >= 2
        && columns_rank(&values[items[0]]).zip(columns_rank(&values[items[1]])) == Some((1, 0))
    {
        values.swap(items[0], items[1]);
        changed = true;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn columns_rank(value: &TokenOrValue<'_>) -> Option<u8> {
    match value {
        TokenOrValue::Length(_) => Some(0),
        TokenOrValue::Token(token) => match **token {
            Token::Dimension { .. } => Some(0),
            Token::Number(_) => Some(1),
            Token::Ident(value) if match_ignore_ascii_case!(value, "auto" => true, _ => false) => {
                Some(1)
            }
            _ => None,
        },
        _ => None,
    }
}

fn minify_ordered_border(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut changed = sort_layer_by_rank(values, 0, values.len(), border_value_rank);
    loop {
        let mut items = values
            .iter()
            .enumerate()
            .filter(|(_, value)| !is_whitespace(value));
        let Some((first_index, first)) = items.next() else {
            break;
        };
        let Some((second_index, second)) = items.next() else {
            break;
        };
        if border_value_rank(first) != Some(0) || border_value_rank(second) != Some(0) {
            break;
        }
        drop(values.drain(first_index..second_index));
        changed = true;
    }
    let mut item_count = 0;
    let mut last_item = None;
    for (index, value) in values.iter().enumerate() {
        if !is_whitespace(value) {
            item_count += 1;
            last_item = Some(index);
        }
    }
    if item_count > 1
        && last_item.is_some_and(|index| {
            token_ident(&values[index]).is_some_and(
                |value| match_ignore_ascii_case!(value, "currentcolor" => true, _ => false),
            )
        })
    {
        let last_item = last_item.expect("checked above");
        let start = last_item.saturating_sub(1);
        drop(values.drain(start..=last_item));
        changed = true;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn border_value_rank(value: &TokenOrValue<'_>) -> Option<u8> {
    match value {
        TokenOrValue::Length(_) => Some(0),
        TokenOrValue::Function(function)
            if function.kind().is_math_value() && !function.is_vendor_prefixed() =>
        {
            Some(0)
        }
        TokenOrValue::Function(function) if function.kind().is_color() => Some(2),
        TokenOrValue::Color(_) | TokenOrValue::UnresolvedColor(_) => Some(2),
        TokenOrValue::Token(token) => match &**token {
            Token::Number(_) | Token::Dimension { .. } => Some(0),
            Token::Ident(value)
                if match_ignore_ascii_case!(
                    value,
                    "thin" | "medium" | "thick" => true,
                    _ => false,
                ) =>
            {
                Some(0)
            }
            Token::Ident(value)
                if match_ignore_ascii_case!(
                    value,
                    "none" | "hidden" | "dotted" | "dashed" | "solid" | "double" | "groove" | "ridge" | "inset" | "outset" | "auto" => true,
                    _ => false,
                ) =>
            {
                Some(1)
            }
            Token::Ident(value) if value.starts_with('_') || value.ends_with('_') => None,
            Token::Ident(_) | Token::Hash(_) | Token::IdHash(_) | Token::MinifiedHash(_) => Some(2),
            _ => None,
        },
        _ => None,
    }
}

fn minify_flex_flow(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let [first, space, second] = values.as_slice() else {
        return;
    };
    if !is_whitespace(space)
        || !token_ident(first).is_some_and(is_flex_wrap)
        || !token_ident(second).is_some_and(is_flex_direction)
    {
        return;
    }
    values.swap(0, 2);
    cx.record_value_normalized();
}

fn is_flex_wrap(value: &str) -> bool {
    match_ignore_ascii_case!(value, "nowrap" | "wrap" | "wrap-reverse" => true, _ => false)
}

fn is_flex_direction(value: &str) -> bool {
    match_ignore_ascii_case!(
        value,
        "row" | "row-reverse" | "column" | "column-reverse" => true,
        _ => false,
    )
}

fn minify_box_shadow(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut start = 0;
    let mut changed = false;
    loop {
        let end = values[start..]
            .iter()
            .position(is_comma)
            .map_or(values.len(), |index| start + index);
        changed |= sort_layer_by_rank(values, start, end, box_shadow_value_rank);
        if end == values.len() {
            break;
        }
        start = end + 1;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn box_shadow_value_rank(value: &TokenOrValue<'_>) -> Option<u8> {
    if token_ident(value)
        .is_some_and(|value| match_ignore_ascii_case!(value, "inset" => true, _ => false))
    {
        return Some(0);
    }
    match value {
        TokenOrValue::Length(_) => Some(1),
        TokenOrValue::Function(function) if function.kind().is_math_value() => Some(1),
        TokenOrValue::Token(token)
            if matches!(**token, Token::Number(_) | Token::Dimension { .. }) =>
        {
            Some(1)
        }
        TokenOrValue::Color(_)
        | TokenOrValue::UnresolvedColor(_)
        | TokenOrValue::Function(_)
        | TokenOrValue::Token(_) => Some(2),
        _ => None,
    }
}

fn sort_layer_by_rank(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    start: usize,
    end: usize,
    rank: fn(&TokenOrValue<'_>) -> Option<u8>,
) -> bool {
    let mut items = [0usize; 16];
    let mut count = 0;
    for index in start..end {
        if is_whitespace(&values[index]) {
            continue;
        }
        if count == items.len() || rank(&values[index]).is_none() {
            return false;
        }
        items[count] = index;
        count += 1;
    }
    let mut changed = false;
    for right in 1..count {
        let mut current = right;
        while current > 0
            && rank(&values[items[current - 1]]).expect("validated rank")
                > rank(&values[items[current]]).expect("validated rank")
        {
            values.swap(items[current - 1], items[current]);
            current -= 1;
            changed = true;
        }
    }
    changed
}

fn minify_box_sides(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let count = match values.len() {
        1 => 1,
        3 if is_whitespace(&values[1]) => 2,
        5 if is_whitespace(&values[1]) && is_whitespace(&values[3]) => 3,
        7 if is_whitespace(&values[1])
            && is_whitespace(&values[3])
            && is_whitespace(&values[5]) =>
        {
            4
        }
        _ => return,
    };
    if count < 2 {
        return;
    }

    let item = |index: usize| &values[index * 2];
    let keep = match count {
        2 if item(0) == item(1) => 1,
        3 if item(0) == item(1) && item(1) == item(2) => 1,
        3 if item(0) == item(2) => 2,
        4 if item(0) == item(1) && item(1) == item(2) && item(2) == item(3) => 1,
        4 if item(0) == item(2) && item(1) == item(3) => 2,
        4 if item(1) == item(3) => 3,
        _ => count,
    };
    if keep < count {
        values.truncate(keep * 2 - 1);
        cx.record_value_normalized();
    }
}

fn minify_font_weight(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let [TokenOrValue::Token(token)] = values.as_mut_slice() else {
        return;
    };
    let Token::Ident(value) = **token else {
        return;
    };
    let weight = match_ignore_ascii_case!(
        value,
        "normal" => 400.0,
        "bold" => 700.0,
        _ => return,
    );
    **token = Token::Number(weight);
    cx.record_value_normalized();
}

fn minify_font(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    for value in values.iter_mut() {
        let TokenOrValue::Token(token) = value else {
            continue;
        };
        if matches!(&**token, Token::Ident(value) if match_ignore_ascii_case!(value, "bold" => true, _ => false))
        {
            **token = Token::Number(700.0);
            cx.record_value_normalized();
        }
    }

    if let Some(generic) = values
        .iter()
        .position(|value| token_ident(value).is_some_and(is_generic_font_family))
        && values.get(generic + 1).is_some_and(is_comma)
        && values[..generic].iter().any(is_comma)
    {
        values.truncate(generic + 1);
        cx.record_value_normalized();
        return;
    }

    let is_simple_family_list = values.iter().enumerate().all(|(index, value)| {
        if index % 2 == 0 {
            token_ident(value).is_some()
        } else {
            is_comma(value)
        }
    });
    if !is_simple_family_list {
        return;
    }
    let mut current = 2;
    while current < values.len() {
        let Some(name) = token_ident(&values[current]) else {
            unreachable!("simple font family entries are identifiers")
        };
        let duplicate = !is_generic_font_family(name)
            && (0..current).step_by(2).any(|previous| {
                token_ident(&values[previous]).is_some_and(|value| value.eq_ignore_ascii_case(name))
            });
        if duplicate {
            drop(values.drain(current - 1..=current));
            cx.record_value_normalized();
        } else {
            current += 2;
        }
    }
}

fn minify_repeat_style(values: &mut Vec<'_, TokenOrValue<'_>>, cx: &mut MinifyContext) {
    let mut index = 0;
    while index + 2 < values.len() {
        let Some(left) = token_ident(&values[index]) else {
            index += 1;
            continue;
        };
        if !is_whitespace(&values[index + 1]) {
            index += 1;
            continue;
        }
        let Some(right) = token_ident(&values[index + 2]) else {
            index += 1;
            continue;
        };

        let replacement = if match_ignore_ascii_case!(left, "repeat" => true, _ => false)
            && match_ignore_ascii_case!(right, "no-repeat" => true, _ => false)
        {
            Some("repeat-x")
        } else if match_ignore_ascii_case!(left, "no-repeat" => true, _ => false)
            && match_ignore_ascii_case!(right, "repeat" => true, _ => false)
        {
            Some("repeat-y")
        } else if left.eq_ignore_ascii_case(right) {
            canonical_repeat(left)
        } else {
            None
        };
        let Some(replacement) = replacement else {
            index += 1;
            continue;
        };

        let TokenOrValue::Token(token) = &mut values[index] else {
            unreachable!("repeat value was classified as a token")
        };
        **token = Token::Ident(replacement);
        drop(values.drain(index + 1..=index + 2));
        cx.record_value_normalized();
    }
}

fn canonical_repeat(value: &str) -> Option<&'static str> {
    match_ignore_ascii_case!(
        value,
        "repeat" => Some("repeat"),
        "space" => Some("space"),
        "round" => Some("round"),
        "no-repeat" => Some("no-repeat"),
        _ => None,
    )
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

fn is_whitespace(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
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
            ) || matches!(**token, Token::Delim("/") | Token::Delim("*"))
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
            ) || matches!(**token, Token::Delim("/") | Token::Delim("*"))
    )
}
