use super::*;

pub(super) fn minify_cubic_bezier(
    arguments: &[TokenOrValue<'_>],
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<&'static str> {
    let [a, comma_1, b, comma_2, c, comma_3, d] = arguments else {
        return None;
    };
    if !is_comma(comma_1, ast) || !is_comma(comma_2, ast) || !is_comma(comma_3, ast) {
        return None;
    }
    match (
        token_number(a, ast)?,
        token_number(b, ast)?,
        token_number(c, ast)?,
        token_number(d, ast)?,
    ) {
        (0.25, 0.1, 0.25, 1.0) => Some("ease"),
        (0.0, 0.0, 1.0, 1.0) => Some("linear"),
        (0.42, 0.0, 1.0, 1.0) => Some("ease-in"),
        (0.0, 0.0, 0.58, 1.0) => Some("ease-out"),
        (0.42, 0.0, 0.58, 1.0) => Some("ease-in-out"),
        _ => None,
    }
}

pub(super) fn minify_steps(
    arguments: &mut rocketcss_common::vec::Vec<'_, TokenOrValue<'_>>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<&'static str> {
    let [count, comma, position] = arguments.as_slice() else {
        return None;
    };
    if !is_comma(comma, ast) {
        return None;
    }
    let position = token_ident(position, ast)?;
    let is_start = match_ignore_ascii_case!(position, "start" | "jump-start" => true, _ => false);
    let is_end = match_ignore_ascii_case!(position, "end" | "jump-end" => true, _ => false);
    if token_number(count, ast) == Some(1.0) {
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
