use rocketcss_ast::{
    FontFamily, KnownFunction, Token, TokenOrValue, Unit, VisitMutContext, match_ignore_ascii_case,
};
use rocketcss_common::vec::Vec;

use crate::{
    Minify, MinifyContext, Options, OptionsOp,
    context::{PropertyContext, ValueContextFlags},
    length,
};

impl Minify for Token<'_> {
    /// Normalizes one stored token node in place.
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None)
            || cx
                .value_context
                .is_enabled(ValueContextFlags::SKIP_VALUE_TRANSFORMS)
            || (cx
                .value_context
                .is_enabled(ValueContextFlags::SKIP_RAW_TOKEN_TRANSFORMS))
        {
            return;
        }

        match self {
            Token::Number(value) if *value == 0.0 && value.is_sign_negative() => {
                *value = 0.0;
                cx.record_value_normalized();
            }
            Token::String(value)
                if cx.value_context.property == PropertyContext::Font
                    && can_unquote_font(value) =>
            {
                *self = Token::UnquotedFont(value);
                cx.record_value_normalized();
            }
            Token::Hash(value) | Token::IdHash(value)
                if cx
                    .value_context
                    .is_enabled(ValueContextFlags::MINIFY_COLORS)
                    && is_hex_color(value) =>
            {
                *self = minify_hex_color(value);
                cx.record_value_normalized();
            }
            Token::Dimension { unit, value } => {
                if *value == 0.0
                    && cx
                        .value_context
                        .is_enabled(ValueContextFlags::ALLOW_UNITLESS_ZERO_LENGTH)
                    && unit.is_length()
                {
                    *self = Token::Number(0.0);
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
                *self = Token::Number(0.0);
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
    FontFamily::from_name(value).is_generic()
}

pub(crate) fn minify_token_values<'a, 'cx, 'ghost>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    cx: &mut MinifyContext<'cx>,
    ast: &mut VisitMutContext<'_, 'a, 'ghost>,
) where
    'a: 'cx,
{
    // Removes comments and redundant whitespace by compacting the existing
    // arena vector. Separator tokens are reused rather than allocated again.
    if cx
        .value_context
        .is_enabled(ValueContextFlags::SKIP_VALUE_TRANSFORMS)
    {
        return;
    }
    match (
        cx.is_enabled(Options::DISCARD_COMMENTS, OptionsOp::Any),
        cx.is_enabled(Options::NORMALIZE_WHITESPACE, OptionsOp::Any),
    ) {
        (true, true) => {
            let preserve_space_after_comma = cx
                .value_context
                .is_enabled(ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA);
            protect_adjacent_function_replacements(values, ast);
            let normalized =
                compact_comments_and_whitespace(values, preserve_space_after_comma, ast);
            record_value_normalized(cx, normalized);
        }
        (true, false) => {
            protect_adjacent_function_replacements(values, ast);
            compact_comments(values, cx, ast);
        }
        (false, true) => {
            protect_adjacent_function_replacements(values, ast);
            compact_whitespace(values, cx, ast);
        }
        (false, false) => protect_adjacent_function_replacements(values, ast),
    }
    minify_compacted_token_values(values, cx, ast);
}

pub(crate) fn compact_comments_and_whitespace<'a>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    preserve_space_after_comma: bool,
    ast: &mut VisitMutContext<'_, 'a, '_>,
) -> usize {
    compact_comments_and_whitespace_with(values, preserve_space_after_comma, ast)
}

pub(crate) fn minify_compacted_token_values<'a, 'cx>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    cx: &mut MinifyContext<'cx>,
    ast: &mut VisitMutContext<'_, 'a, '_>,
) where
    'a: 'cx,
{
    if cx
        .value_context
        .is_enabled(ValueContextFlags::SKIP_RAW_TOKEN_TRANSFORMS)
    {
        return;
    }
    if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
        return;
    }

    match cx.value_context.property {
        PropertyContext::Animation => minify_animation(values, cx, ast),
        PropertyContext::Border | PropertyContext::Outline => {
            minify_ordered_border(values, cx, ast)
        }
        PropertyContext::Box => minify_box_sides(values, cx, ast),
        PropertyContext::BoxShadow => minify_box_shadow(values, cx, ast),
        PropertyContext::Columns => minify_ordered_columns(values, cx, ast),
        PropertyContext::Display => minify_display(values, cx, ast),
        PropertyContext::FlexFlow => minify_flex_flow(values, cx, ast),
        PropertyContext::Font => minify_font(values, cx, ast),
        PropertyContext::FontWeight => minify_font_weight(values, cx, ast),
        PropertyContext::GridAutoFlow => minify_grid_auto_flow(values, cx, ast),
        PropertyContext::GridGap => minify_grid_gap(values, cx, ast),
        PropertyContext::GridLine => minify_grid_line(values, cx, ast),
        PropertyContext::ListStyle => minify_list_style(values, cx, ast),
        PropertyContext::Position => {
            minify_positions(values, cx, ast);
            minify_repeat_style(values, cx, ast);
        }
        PropertyContext::Repeat => minify_repeat_style(values, cx, ast),
        PropertyContext::TimingFunction => {}
        PropertyContext::Transform => {}
        PropertyContext::Transition => minify_transition(values, cx, ast),
        PropertyContext::Generic => {}
    }
}

fn protect_adjacent_function_replacements<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    for index in 0..values.len() {
        let has_unsafe_neighbor = values.get(index.wrapping_sub(1)).is_some_and(|value| {
            !matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::WhiteSpace(_) | Token::Comma))
        });
        if !has_unsafe_neighbor {
            continue;
        }
        let TokenOrValue::Function(function) = &mut values[index] else {
            continue;
        };
        ast.mutate_node(*function, |function, _| {
            if matches!(
                function.replacement,
                Some(rocketcss_ast::FunctionReplacement::Rgb { .. })
            ) {
                function.replacement = None;
            }
        });
    }
}

fn minify_display<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    let replacement = match values.as_slice() {
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "block", "flow", ast) =>
        {
            Some("block")
        }
        [first, space, second]
            if is_whitespace(space, ast)
                && ident_pair(first, second, "block", "flow-root", ast) =>
        {
            Some("flow-root")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "inline", "flow", ast) =>
        {
            Some("inline")
        }
        [first, space, second]
            if is_whitespace(space, ast)
                && ident_pair(first, second, "inline", "flow-root", ast) =>
        {
            Some("inline-block")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "run-in", "flow", ast) =>
        {
            Some("run-in")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "block", "flex", ast) =>
        {
            Some("flex")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "inline", "flex", ast) =>
        {
            Some("inline-flex")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "block", "grid", ast) =>
        {
            Some("grid")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "inline", "grid", ast) =>
        {
            Some("inline-grid")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "inline", "ruby", ast) =>
        {
            Some("ruby")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "block", "table", ast) =>
        {
            Some("table")
        }
        [first, space, second]
            if is_whitespace(space, ast) && ident_pair(first, second, "inline", "table", ast) =>
        {
            Some("inline-table")
        }
        [first, space, second]
            if is_whitespace(space, ast)
                && (["table-cell", "table-caption", "ruby-base", "ruby-text"]
                    .iter()
                    .any(|keyword| ident_pair(first, second, keyword, "flow", ast))) =>
        {
            ["table-cell", "table-caption", "ruby-base", "ruby-text"]
                .into_iter()
                .find(|keyword| ident_pair(first, second, keyword, "flow", ast))
        }
        [first, space_1, second, space_2, third]
            if is_whitespace(space_1, ast)
                && is_whitespace(space_2, ast)
                && token_ident(first, ast).is_some_and(
                    |value| match_ignore_ascii_case!(value, "list-item" => true, _ => false),
                )
                && token_ident(second, ast).is_some_and(
                    |value| match_ignore_ascii_case!(value, "block" => true, _ => false),
                )
                && token_ident(third, ast).is_some_and(
                    |value| match_ignore_ascii_case!(value, "flow" => true, _ => false),
                ) =>
        {
            Some("list-item")
        }
        [first, space_1, second, space_2, third]
            if is_whitespace(space_1, ast)
                && is_whitespace(space_2, ast)
                && token_ident(first, ast).is_some_and(
                    |value| match_ignore_ascii_case!(value, "inline" => true, _ => false),
                )
                && token_ident(second, ast).is_some_and(
                    |value| match_ignore_ascii_case!(value, "flow" => true, _ => false),
                )
                && token_ident(third, ast).is_some_and(
                    |value| match_ignore_ascii_case!(value, "list-item" => true, _ => false),
                ) =>
        {
            let TokenOrValue::Token(token) = &mut values[2] else {
                unreachable!()
            };
            ast.mutate_node(*token, |token, _| *token = Token::Ident("list-item"));
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
    ast.mutate_node(*token, |token, _| *token = Token::Ident(replacement));
    values.truncate(1);
    cx.record_value_normalized();
}

fn ident_pair(
    first: &TokenOrValue<'_>,
    second: &TokenOrValue<'_>,
    expected_first: &str,
    expected_second: &str,
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    token_ident(first, ast).is_some_and(|value| value.eq_ignore_ascii_case(expected_first))
        && token_ident(second, ast).is_some_and(|value| value.eq_ignore_ascii_case(expected_second))
}

fn minify_positions<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    let mut layer_start = 0;
    while layer_start < values.len() {
        let layer_end = values[layer_start..]
            .iter()
            .position(|value| is_comma(value, ast))
            .map_or(values.len(), |index| layer_start + index);
        if minify_position_layer(values, layer_start, layer_end, ast) {
            cx.record_value_normalized();
        }
        let Some(comma) = values[layer_start..]
            .iter()
            .position(|value| is_comma(value, ast))
        else {
            break;
        };
        layer_start += comma + 1;
    }
}

fn minify_position_layer<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    layer_start: usize,
    layer_end: usize,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) -> bool {
    let mut start = None;
    let mut end = None;
    for index in layer_start..layer_end {
        if is_slash(&values[index], ast) {
            break;
        }
        if is_variable(&values[index], ast) {
            return false;
        }
        if is_position_component(&values[index], ast) {
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
        return normalize_horizontal_keyword(&mut values[start], ast);
    }
    if end != start + 2 || !is_whitespace(&values[start + 1], ast) {
        return false;
    }

    let Some(second) = token_ident(&values[end], ast) else {
        return false;
    };
    let second = position_keyword(second);

    if second == Some(PositionKeyword::Center) {
        normalize_horizontal_keyword(&mut values[start], ast);
        drop(values.drain(start + 1..=end));
        return true;
    }
    let Some(first) = token_ident(&values[start], ast) else {
        return false;
    };
    let first = position_keyword(first);
    if first == Some(PositionKeyword::Center) {
        if matches!(second, Some(PositionKeyword::Left | PositionKeyword::Right)) {
            set_position_keyword(&mut values[end], second.expect("matched above"), ast);
        }
        drop(values.drain(start..end));
        return second.is_some();
    }

    match (first, second) {
        (
            Some(horizontal @ (PositionKeyword::Left | PositionKeyword::Right)),
            Some(vertical @ (PositionKeyword::Top | PositionKeyword::Bottom)),
        ) => {
            set_position_keyword(&mut values[start], horizontal, ast);
            set_position_keyword(&mut values[end], vertical, ast);
            true
        }
        (
            Some(vertical @ (PositionKeyword::Top | PositionKeyword::Bottom)),
            Some(horizontal @ (PositionKeyword::Left | PositionKeyword::Right)),
        ) => {
            set_position_keyword(&mut values[start], horizontal, ast);
            set_position_keyword(&mut values[end], vertical, ast);
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

fn normalize_horizontal_keyword<'ast>(
    value: &mut TokenOrValue<'ast>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) -> bool {
    let Some(keyword) = token_ident(value, ast).and_then(position_keyword) else {
        return false;
    };
    if matches!(
        keyword,
        PositionKeyword::Left | PositionKeyword::Right | PositionKeyword::Center
    ) {
        set_position_keyword(value, keyword, ast);
        true
    } else {
        false
    }
}

fn set_position_keyword<'ast>(
    value: &mut TokenOrValue<'ast>,
    keyword: PositionKeyword,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    let TokenOrValue::Token(token) = value else {
        unreachable!("position keyword was classified as a token")
    };
    let value = match keyword {
        PositionKeyword::Top | PositionKeyword::Left => Token::Number(0.0),
        PositionKeyword::Right | PositionKeyword::Bottom => Token::Percentage(1.0),
        PositionKeyword::Center => Token::Percentage(0.5),
    };
    ast.mutate_node(*token, |token, _| *token = value);
}

fn is_position_component(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    match value {
        TokenOrValue::Token(token) => {
            matches!(
                ast.ast_context().resolve_node(*token),
                Token::Ident(value) if position_keyword(value).is_some()
            ) || matches!(
                ast.ast_context().resolve_node(*token),
                Token::Number(_)
                    | Token::Percentage(_)
                    | Token::Dimension { .. }
                    | Token::UnknownDimension { .. }
            )
        }
        TokenOrValue::Function(function) => {
            let function = ast.ast_context().resolve_node(*function);
            function.kind().is_math_value() && !function.is_vendor_prefixed()
        }
        TokenOrValue::Length(_) => true,
        _ => false,
    }
}

fn is_variable(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    match value {
        TokenOrValue::Var(_) | TokenOrValue::Env(_) => true,
        TokenOrValue::Function(function) => ast
            .ast_context()
            .resolve_node(*function)
            .kind()
            .is_variable(),
        _ => false,
    }
}

fn is_comma(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::Comma))
}

fn is_slash(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::Delim("/")))
}

fn compact_comments_and_whitespace_with<'a>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    preserve_space_after_comma: bool,
    ast: &mut VisitMutContext<'_, 'a, '_>,
) -> usize {
    let len = values.len();
    let mut read = 0;
    let mut write = 0;
    let mut normalized = 0;
    while read < len {
        if !is_whitespace_or_comment(&values[read], ast) {
            retain_compacted_value(values, read, &mut write);
            read += 1;
            continue;
        }

        let mut first_whitespace = None;
        let mut whitespace_count = 0;
        let mut comment_count = 0;
        while read < len && is_whitespace_or_comment(&values[read], ast) {
            if is_whitespace(&values[read], ast) {
                first_whitespace.get_or_insert(read);
                whitespace_count += 1;
            } else {
                comment_count += 1;
            }
            read += 1;
        }

        let has_neighbors = write > 0 && read < len;
        let whitespace_required =
            has_neighbors && whitespace_is_required(&values[write - 1], &values[read], ast);
        let comment_became_whitespace = whitespace_count == 0 && whitespace_required;
        let has_whitespace = whitespace_count > 0 || comment_became_whitespace;
        let keep_space = has_whitespace
            && has_neighbors
            && (whitespace_required
                || multiplication_requires_whitespace(
                    &values[write - 1],
                    &values[read],
                    &values[read + 1..],
                    ast,
                )
                || (preserve_space_after_comma && is_comma(&values[write - 1], ast)));

        normalized += comment_count;

        let whitespace_changed = if whitespace_count == 0 {
            false
        } else if keep_space {
            whitespace_count > 1
                || first_whitespace
                    .is_some_and(|index| !is_normalized_whitespace(&values[index], ast))
        } else {
            true
        };
        if whitespace_changed {
            normalized += 1;
        }

        if keep_space {
            let separator = first_whitespace.unwrap_or(read - 1);
            set_normalized_whitespace(&mut values[separator], ast);
            retain_compacted_value(values, separator, &mut write);
        }
    }
    values.truncate(write);
    normalized
}

fn compact_comments<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    let len = values.len();
    let mut read = 0;
    let mut write = 0;
    while read < len {
        if !matches!(&values[read], TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::Comment(_)))
        {
            retain_compacted_value(values, read, &mut write);
            read += 1;
            continue;
        }

        let start = read;
        while read < len
            && matches!(&values[read], TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::Comment(_)))
        {
            read += 1;
        }

        let keep_space = write > 0
            && read < len
            && !is_whitespace_or_comment(&values[write - 1], ast)
            && !is_whitespace_or_comment(&values[read], ast)
            && whitespace_is_required(&values[write - 1], &values[read], ast);
        record_value_normalized(cx, read - start);
        if keep_space {
            let separator = read - 1;
            set_normalized_whitespace(&mut values[separator], ast);
            retain_compacted_value(values, separator, &mut write);
        }
    }
    values.truncate(write);
}

fn compact_whitespace<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    let len = values.len();
    let preserve_space_after_comma = cx
        .value_context
        .is_enabled(ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA);
    let mut read = 0;
    let mut write = 0;
    while read < len {
        if !is_whitespace(&values[read], ast) {
            retain_compacted_value(values, read, &mut write);
            read += 1;
            continue;
        }

        let start = read;
        let was_normalized_space = is_normalized_whitespace(&values[start], ast);
        while read < len && is_whitespace(&values[read], ast) {
            read += 1;
        }

        let keep_space = write > 0
            && read < len
            && (whitespace_is_required(&values[write - 1], &values[read], ast)
                || multiplication_requires_whitespace(
                    &values[write - 1],
                    &values[read],
                    &values[read + 1..],
                    ast,
                )
                || (preserve_space_after_comma && is_comma(&values[write - 1], ast)));
        if !keep_space || read > start + 1 || !was_normalized_space {
            cx.record_value_normalized();
        }
        if keep_space {
            set_normalized_whitespace(&mut values[start], ast);
            retain_compacted_value(values, start, &mut write);
        }
    }
    values.truncate(write);
}

#[inline]
fn retain_compacted_value(values: &mut Vec<'_, TokenOrValue<'_>>, read: usize, write: &mut usize) {
    // The compacted prefix is never revisited. Swapping moves the next
    // retained value into that prefix and pushes one discarded value into the
    // consumed portion, so the tail can be truncated once after the scan.
    if read != *write {
        values.swap(read, *write);
    }
    *write += 1;
}

#[inline]
fn set_normalized_whitespace<'ast>(
    value: &mut TokenOrValue<'ast>,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    let TokenOrValue::Token(token) = value else {
        unreachable!("separator nodes are tokens")
    };
    ast.mutate_node(*token, |token, _| *token = Token::WhiteSpace(" "));
}

#[inline]
fn is_normalized_whitespace(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::WhiteSpace(" ")))
}

fn record_value_normalized(cx: &mut MinifyContext, count: usize) {
    for _ in 0..count {
        cx.record_value_normalized();
    }
}

fn multiplication_requires_whitespace(
    before: &TokenOrValue<'_>,
    after: &TokenOrValue<'_>,
    following: &[TokenOrValue<'_>],
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    if is_delim(before, "*", ast) && is_open_parenthesis(after, ast) {
        return true;
    }
    is_delim(after, "*", ast)
        && following
            .iter()
            .find(|value| !is_whitespace_or_comment(value, ast))
            .is_some_and(|value| is_open_parenthesis(value, ast))
}

fn is_delim(value: &TokenOrValue<'_>, expected: &str, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::Delim(value) if *value == expected))
}

fn is_open_parenthesis(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::ParenthesisBlock))
}

fn minify_transition(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let mut start = 0;
    let mut changed = false;
    loop {
        let end = values[start..]
            .iter()
            .position(|value| is_comma(value, ast))
            .map_or(values.len(), |index| start + index);
        changed |= sort_transition_layer(values, start, end, ast);
        if end == values.len() {
            break;
        }
        start = end + 1;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn sort_transition_layer(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    start: usize,
    end: usize,
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    let Some((items, count)) = collect_layer_items(values, start, end, ast) else {
        return false;
    };
    let mut ranks = [0u8; 16];
    let mut time_count = 0;
    for position in 0..count {
        let value = &values[items[position]];
        ranks[position] = if is_time_value(value, ast) {
            time_count += 1;
            if time_count == 1 { 1 } else { 3 }
        } else if is_timing_value(value, ast) {
            2
        } else if token_ident(value, ast).is_some_and(|value| {
            match_ignore_ascii_case!(value, "normal" | "allow-discrete" => true, _ => false)
        }) {
            4
        } else if is_variable(value, ast) {
            return false;
        } else {
            0
        };
    }
    sort_items_with_ranks(values, items, ranks, count)
}

fn minify_animation(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let mut start = 0;
    let mut changed = false;
    loop {
        let end = values[start..]
            .iter()
            .position(|value| is_comma(value, ast))
            .map_or(values.len(), |index| start + index);
        changed |= sort_animation_layer(values, start, end, ast);
        if end == values.len() {
            break;
        }
        start = end + 1;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn sort_animation_layer(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    start: usize,
    end: usize,
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    let Some((items, count)) = collect_layer_items(values, start, end, ast) else {
        return false;
    };
    let mut ranks = [0u8; 16];
    let mut time_count = 0;
    let mut timing_claimed = false;
    let mut count_claimed = false;
    let mut direction_claimed = false;
    let mut fill_claimed = false;
    let mut play_claimed = false;
    // The first value matching a keyword class claims it; later duplicates are
    // ambiguous and treated as the keyframes name (rank 0), mirroring upstream
    // postcss-ordered-values.
    for position in 0..count {
        let value = &values[items[position]];
        ranks[position] = if is_time_value(value, ast) {
            time_count += 1;
            match time_count {
                1 => 1,
                2 => 3,
                _ => 0,
            }
        } else if is_timing_value(value, ast) && !timing_claimed {
            timing_claimed = true;
            2
        } else if (matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::Number(_)))
            || token_ident(value, ast).is_some_and(
                |value| match_ignore_ascii_case!(value, "infinite" => true, _ => false),
            ))
            && !count_claimed
        {
            count_claimed = true;
            4
        } else if token_ident(value, ast).is_some_and(is_animation_direction) && !direction_claimed
        {
            direction_claimed = true;
            5
        } else if token_ident(value, ast).is_some_and(is_animation_fill_mode) && !fill_claimed {
            fill_claimed = true;
            6
        } else if token_ident(value, ast).is_some_and(is_animation_play_state) && !play_claimed {
            play_claimed = true;
            7
        } else if is_variable(value, ast) {
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

fn is_time_value(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Time(_))
        || matches!(value, TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), Token::Dimension { unit: Unit::Seconds | Unit::Milliseconds, .. }))
}

fn is_timing_value(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    token_ident(value, ast).is_some_and(|value| {
        match_ignore_ascii_case!(
            value,
            "linear" | "ease" | "ease-in" | "ease-out" | "ease-in-out" | "step-start" | "step-end" => true,
            _ => false,
        )
    }) || matches!(value, TokenOrValue::Function(function)
        if matches!(
            ast.ast_context().resolve_node(*function).kind(),
            KnownFunction::Steps
                | KnownFunction::CubicBezier
                | KnownFunction::Linear
                | KnownFunction::Frames
        )
        // A timing function minified to an identifier (e.g. `cubic-bezier(0.25,0.1,0.25,1)`
        // → `ease`) keeps its timing rank even though its kind was reclassified.
        || (ast.ast_context().resolve_node(*function).is_identifier()
            && match_ignore_ascii_case!(
                ast.ast_context().resolve_node(*function).name(),
                "linear" | "ease" | "ease-in" | "ease-out" | "ease-in-out" | "step-start" | "step-end" => true,
                _ => false,
            )))
}

fn collect_layer_items(
    values: &[TokenOrValue<'_>],
    start: usize,
    end: usize,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<([usize; 16], usize)> {
    let mut items = [0usize; 16];
    let mut count = 0;
    for (index, value) in values.iter().enumerate().take(end).skip(start) {
        if is_whitespace(value, ast) {
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

fn minify_grid_auto_flow(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let [first, space, second] = values.as_slice() else {
        return;
    };
    if is_whitespace(space, ast)
        && token_ident(first, ast)
            .is_some_and(|value| match_ignore_ascii_case!(value, "dense" => true, _ => false))
        && token_ident(second, ast).is_some_and(
            |value| match_ignore_ascii_case!(value, "row" | "column" => true, _ => false),
        )
    {
        values.swap(0, 2);
        cx.record_value_normalized();
    }
}

fn minify_grid_gap(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let [first, space, second] = values.as_slice() else {
        return;
    };
    if is_whitespace(space, ast)
        && !token_ident(first, ast)
            .is_some_and(|value| match_ignore_ascii_case!(value, "normal" => true, _ => false))
        && token_ident(second, ast)
            .is_some_and(|value| match_ignore_ascii_case!(value, "normal" => true, _ => false))
    {
        values.swap(0, 2);
        cx.record_value_normalized();
    }
}

fn minify_grid_line(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let mut changed = false;
    let mut index = 0;
    while index + 2 < values.len() {
        if matches!(&values[index], TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::Number(_)))
            && is_whitespace(&values[index + 1], ast)
            && token_ident(&values[index + 2], ast)
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

fn minify_list_style(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    if sort_layer_by_rank(values, 0, values.len(), ast, list_style_rank) {
        cx.record_value_normalized();
    }
}

fn list_style_rank(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> Option<u8> {
    if token_ident(value, ast).is_some_and(
        |value| match_ignore_ascii_case!(value, "inside" | "outside" => true, _ => false),
    ) {
        Some(1)
    } else if matches!(value, TokenOrValue::Url(_))
        || matches!(value, TokenOrValue::Function(function) if ast.ast_context().resolve_node(*function).kind() == KnownFunction::Url)
    {
        Some(2)
    } else if is_variable(value, ast) {
        None
    } else {
        Some(0)
    }
}

fn minify_ordered_columns(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let Some((items, count)) = collect_layer_items(values, 0, values.len(), ast) else {
        return;
    };
    if count == 2
        && token_ident(&values[items[0]], ast)
            .is_some_and(|value| match_ignore_ascii_case!(value, "auto" => true, _ => false))
        && token_ident(&values[items[1]], ast)
            .is_some_and(|value| match_ignore_ascii_case!(value, "auto" => true, _ => false))
    {
        values.truncate(items[0] + 1);
        cx.record_value_normalized();
        return;
    }
    // Mirror upstream postcss-ordered-values: reorder only a two-component
    // value holding exactly one width (has a unit) and one other component;
    // anything else (e.g. `3rem 2 12em`) is left untouched.
    if count == 2
        && !columns_has_unit(&values[items[0]], ast)
        && columns_has_unit(&values[items[1]], ast)
    {
        values.swap(items[0], items[1]);
        cx.record_value_normalized();
    }
}

fn columns_has_unit(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    match value {
        TokenOrValue::Length(_) => true,
        TokenOrValue::Token(token) => {
            matches!(
                ast.ast_context().resolve_node(*token),
                Token::Percentage(_) | Token::Dimension { .. }
            )
        }
        _ => false,
    }
}

fn minify_ordered_border(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    // Extra components of one class (e.g. the widths in `0 0 7px 7px solid
    // black`) keep their relative order instead of being dropped, so invalid
    // values round-trip unchanged like upstream postcss-ordered-values.
    let mut changed = sort_layer_by_rank(values, 0, values.len(), ast, border_value_rank);
    let mut item_count = 0;
    let mut last_item = None;
    for (index, value) in values.iter().enumerate() {
        if !is_whitespace(value, ast) {
            item_count += 1;
            last_item = Some(index);
        }
    }
    if item_count > 1
        && last_item.is_some_and(|index| {
            token_ident(&values[index], ast).is_some_and(
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

fn border_value_rank(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> Option<u8> {
    match value {
        TokenOrValue::Length(_) => Some(0),
        TokenOrValue::Function(function)
            if ast
                .ast_context()
                .resolve_node(*function)
                .kind()
                .is_math_value()
                && !ast
                    .ast_context()
                    .resolve_node(*function)
                    .is_vendor_prefixed() =>
        {
            Some(0)
        }
        TokenOrValue::Function(function)
            if ast.ast_context().resolve_node(*function).kind().is_color() =>
        {
            Some(2)
        }
        TokenOrValue::Color(_) | TokenOrValue::UnresolvedColor(_) => Some(2),
        TokenOrValue::Token(token) => match ast.ast_context().resolve_node(*token) {
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

fn minify_flex_flow(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let [first, space, second] = values.as_slice() else {
        return;
    };
    if !is_whitespace(space, ast)
        || !token_ident(first, ast).is_some_and(is_flex_wrap)
        || !token_ident(second, ast).is_some_and(is_flex_direction)
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

fn minify_box_shadow(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let mut start = 0;
    let mut changed = false;
    loop {
        let end = values[start..]
            .iter()
            .position(|value| is_comma(value, ast))
            .map_or(values.len(), |index| start + index);
        changed |= sort_layer_by_rank(values, start, end, ast, box_shadow_value_rank);
        if end == values.len() {
            break;
        }
        start = end + 1;
    }
    if changed {
        cx.record_value_normalized();
    }
}

fn box_shadow_value_rank(
    value: &TokenOrValue<'_>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<u8> {
    if token_ident(value, ast)
        .is_some_and(|value| match_ignore_ascii_case!(value, "inset" => true, _ => false))
    {
        return Some(0);
    }
    match value {
        TokenOrValue::Length(_) => Some(1),
        TokenOrValue::Function(function)
            if ast
                .ast_context()
                .resolve_node(*function)
                .kind()
                .is_math_value() =>
        {
            Some(1)
        }
        TokenOrValue::Token(token)
            if matches!(
                ast.ast_context().resolve_node(*token),
                Token::Number(_) | Token::Dimension { .. }
            ) =>
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
    ast: &VisitMutContext<'_, '_, '_>,
    rank: fn(&TokenOrValue<'_>, &VisitMutContext<'_, '_, '_>) -> Option<u8>,
) -> bool {
    let mut items = [0usize; 16];
    let mut count = 0;
    for index in start..end {
        if is_whitespace(&values[index], ast) {
            continue;
        }
        if count == items.len() || rank(&values[index], ast).is_none() {
            return false;
        }
        items[count] = index;
        count += 1;
    }
    let mut changed = false;
    for right in 1..count {
        let mut current = right;
        while current > 0
            && rank(&values[items[current - 1]], ast).expect("validated rank")
                > rank(&values[items[current]], ast).expect("validated rank")
        {
            values.swap(items[current - 1], items[current]);
            current -= 1;
            changed = true;
        }
    }
    changed
}

fn minify_box_sides(
    values: &mut Vec<'_, TokenOrValue<'_>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, '_, '_>,
) {
    let count = match values.len() {
        1 => 1,
        3 if is_whitespace(&values[1], ast) => 2,
        5 if is_whitespace(&values[1], ast) && is_whitespace(&values[3], ast) => 3,
        7 if is_whitespace(&values[1], ast)
            && is_whitespace(&values[3], ast)
            && is_whitespace(&values[5], ast) =>
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
        2 if token_or_value_eq(item(0), item(1), ast) => 1,
        3 if token_or_value_eq(item(0), item(1), ast)
            && token_or_value_eq(item(1), item(2), ast) =>
        {
            1
        }
        3 if token_or_value_eq(item(0), item(2), ast) => 2,
        4 if token_or_value_eq(item(0), item(1), ast)
            && token_or_value_eq(item(1), item(2), ast)
            && token_or_value_eq(item(2), item(3), ast) =>
        {
            1
        }
        4 if token_or_value_eq(item(0), item(2), ast)
            && token_or_value_eq(item(1), item(3), ast) =>
        {
            2
        }
        4 if token_or_value_eq(item(1), item(3), ast) => 3,
        _ => count,
    };
    if keep < count {
        values.truncate(keep * 2 - 1);
        cx.record_value_normalized();
    }
}

pub(crate) fn token_or_value_eq(
    left: &TokenOrValue<'_>,
    right: &TokenOrValue<'_>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    std::mem::discriminant(left) == std::mem::discriminant(right)
        && crate::equality::css_values_are_equal(ast.ast_context(), left, right)
}

fn minify_font_weight<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    let [TokenOrValue::Token(token)] = values.as_mut_slice() else {
        return;
    };
    let Token::Ident(value) = ast.ast_context().resolve_node(*token) else {
        return;
    };
    let weight = match_ignore_ascii_case!(
        value,
        "normal" => 400.0,
        "bold" => 700.0,
        _ => return,
    );
    ast.mutate_node(*token, |token, _| *token = Token::Number(weight));
    cx.record_value_normalized();
}

fn minify_font<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    for value in values.iter_mut() {
        let TokenOrValue::Token(token) = value else {
            continue;
        };
        if matches!(ast.ast_context().resolve_node(*token), Token::Ident(value) if match_ignore_ascii_case!(value, "bold" => true, _ => false))
        {
            ast.mutate_node(*token, |token, _| *token = Token::Number(700.0));
            cx.record_value_normalized();
        }
    }

    if let Some(generic) = values
        .iter()
        .position(|value| token_ident(value, ast).is_some_and(is_generic_font_family))
        && values
            .get(generic + 1)
            .is_some_and(|value| is_comma(value, ast))
        && values[..generic].iter().any(|value| is_comma(value, ast))
    {
        values.truncate(generic + 1);
        cx.record_value_normalized();
        return;
    }

    let is_simple_family_list = values.iter().enumerate().all(|(index, value)| {
        if index % 2 == 0 {
            font_family_name(value, ast).is_some()
        } else {
            is_comma(value, ast)
        }
    });
    if !is_simple_family_list {
        return;
    }
    let mut current = 2;
    while current < values.len() {
        let Some((name, generic)) = font_family_name(&values[current], ast) else {
            unreachable!("simple font family entries are names")
        };
        let duplicate = (0..current).step_by(2).any(|previous| {
            font_family_name(&values[previous], ast).is_some_and(|(previous, previous_generic)| {
                previous_generic == generic && previous.eq_ignore_ascii_case(name)
            })
        });
        if duplicate {
            drop(values.drain(current - 1..=current));
            cx.record_value_normalized();
        } else {
            current += 2;
        }
    }
}

fn font_family_name<'a>(
    value: &'a TokenOrValue<'a>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<(&'a str, bool)> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match ast.ast_context().resolve_node(*token) {
        Token::Ident(value) => Some((*value, is_generic_font_family(value))),
        Token::String(value) | Token::UnquotedFont(value) => Some((*value, false)),
        _ => None,
    }
}

fn minify_repeat_style<'ast>(
    values: &mut Vec<'ast, TokenOrValue<'ast>>,
    cx: &mut MinifyContext,
    ast: &mut VisitMutContext<'_, 'ast, '_>,
) {
    let mut index = 0;
    while index + 2 < values.len() {
        let Some(left) = token_ident(&values[index], ast) else {
            index += 1;
            continue;
        };
        if !is_whitespace(&values[index + 1], ast) {
            index += 1;
            continue;
        }
        let Some(right) = token_ident(&values[index + 2], ast) else {
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
        ast.mutate_node(*token, |token, _| *token = Token::Ident(replacement));
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

fn token_ident<'a>(value: &TokenOrValue<'a>, ast: &VisitMutContext<'_, '_, '_>) -> Option<&'a str> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match ast.ast_context().resolve_node(*token) {
        Token::Ident(value) => Some(*value),
        _ => None,
    }
}

fn is_whitespace(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::WhiteSpace(_)))
}

fn is_whitespace_or_comment(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token)
            if matches!(ast.ast_context().resolve_node(*token), Token::WhiteSpace(_) | Token::Comment(_))
    )
}

fn whitespace_is_required(
    left: &TokenOrValue<'_>,
    right: &TokenOrValue<'_>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    !ends_with_open_punctuation(left, ast) && !starts_with_close_punctuation(right, ast)
}

fn ends_with_open_punctuation(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token)
            if matches!(
                ast.ast_context().resolve_node(*token),
                Token::Comma
                    | Token::Colon
                    | Token::Semicolon
                    | Token::ParenthesisBlock
                    | Token::SquareBracketBlock
                    | Token::CurlyBracketBlock
            ) || matches!(ast.ast_context().resolve_node(*token), Token::Delim("/") | Token::Delim("*"))
    )
}

fn starts_with_close_punctuation(
    value: &TokenOrValue<'_>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token)
            if matches!(
                ast.ast_context().resolve_node(*token),
                Token::Comma
                    | Token::Colon
                    | Token::Semicolon
                    | Token::CloseParenthesis
                    | Token::CloseSquareBracket
                    | Token::CloseCurlyBracket
            ) || matches!(ast.ast_context().resolve_node(*token), Token::Delim("/") | Token::Delim("*"))
    )
}
