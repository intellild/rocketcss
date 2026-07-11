use rs_css_allocator::vec::Vec;
use rs_css_ast::{Token, TokenOrValue};

use crate::{MinifyContext, context::PropertyContext, length};

/// Normalizes one token node in place. The surrounding `TokenOrValue` variant
/// and its arena allocation are preserved.
pub(crate) fn minify_token_or_value(value: &mut TokenOrValue<'_>, context: &mut MinifyContext) {
    if context.value_context.skip_value_transforms {
        return;
    }

    let TokenOrValue::Token(token) = value else {
        return;
    };
    match &mut **token {
        Token::Dimension { unit, value } => {
            if *value == 0.0 && context.value_context.allow_unitless_zero && unit.is_length() {
                **token = Token::Number(0.0);
                context.record_value_normalized();
            } else if let Some((number, normalized_unit)) = length::minify_dimension(*value, *unit)
                && (number != *value || normalized_unit != *unit)
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

/// Removes comments and redundant whitespace by compacting the existing arena
/// vector. Separator tokens are reused rather than allocated again.
pub(crate) fn minify_token_list<'a>(
    values: &mut Vec<'a, TokenOrValue<'a>>,
    context: &mut MinifyContext,
) {
    if context.options().normalize_tokens {
        normalize_separators(values, context);
    }
    if !context.options().normalize_values || context.value_context.skip_value_transforms {
        return;
    }

    match context.value_context.property {
        PropertyContext::Box => minify_box_sides(values, context),
        PropertyContext::FontWeight => minify_font_weight(values, context),
        PropertyContext::Repeat => minify_repeat_style(values, context),
        PropertyContext::Generic => {}
    }
}

fn normalize_separators(values: &mut Vec<'_, TokenOrValue<'_>>, context: &mut MinifyContext) {
    let mut index = 0;
    while index < values.len() {
        if !is_whitespace_or_comment(&values[index]) {
            index += 1;
            continue;
        }

        let start = index;
        let mut end = start + 1;
        while end < values.len() && is_whitespace_or_comment(&values[end]) {
            end += 1;
        }

        let keep_space = start > 0
            && end < values.len()
            && whitespace_is_required(&values[start - 1], &values[end]);
        if keep_space {
            let TokenOrValue::Token(token) = &mut values[start] else {
                unreachable!("separator nodes are tokens")
            };
            let was_normalized_space = matches!(**token, Token::WhiteSpace(" "));
            **token = Token::WhiteSpace(" ");
            if end > start + 1 {
                drop(values.drain(start + 1..end));
                context.record_value_normalized();
            } else if !was_normalized_space {
                context.record_value_normalized();
            }
            index = start + 1;
        } else {
            drop(values.drain(start..end));
            context.record_value_normalized();
            index = start;
        }
    }
}

fn minify_box_sides(values: &mut Vec<'_, TokenOrValue<'_>>, context: &mut MinifyContext) {
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
        context.record_value_normalized();
    }
}

fn minify_font_weight(values: &mut Vec<'_, TokenOrValue<'_>>, context: &mut MinifyContext) {
    let [TokenOrValue::Token(token)] = values.as_mut_slice() else {
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
    **token = Token::Number(weight);
    context.record_value_normalized();
}

fn minify_repeat_style(values: &mut Vec<'_, TokenOrValue<'_>>, context: &mut MinifyContext) {
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

        let replacement = if left.eq_ignore_ascii_case("repeat")
            && right.eq_ignore_ascii_case("no-repeat")
        {
            Some("repeat-x")
        } else if left.eq_ignore_ascii_case("no-repeat") && right.eq_ignore_ascii_case("repeat") {
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
        context.record_value_normalized();
    }
}

fn canonical_repeat(value: &str) -> Option<&'static str> {
    ["repeat", "space", "round", "no-repeat"]
        .into_iter()
        .find(|candidate| value.eq_ignore_ascii_case(candidate))
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
