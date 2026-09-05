use super::*;
use super::{
    calc::{
        calc_linear_expression, minify_flat_calc_operations, remove_redundant_calc_parentheses,
        simple_calc_value,
    },
    color::{minify_hsl_function, minify_rgb_function},
    gradient::{
        minify_gradient_direction, minify_gradient_stops, rollback_gradient_color_replacements,
    },
    timing::{minify_cubic_bezier, minify_steps},
    transform::minify_transform_function,
    url::can_unquote_url,
};

pub(crate) fn minify_function<'ast, 'cx, 'ghost>(
    function: &mut Function<'ast>,
    cx: &mut MinifyContext<'cx>,
    ast: &mut VisitMutContext<'_, 'ast, 'ghost>,
) where
    'ast: 'cx,
{
    if cx
        .value_context
        .is_enabled(ValueContextFlags::SKIP_VALUE_TRANSFORMS)
    {
        return;
    }
    if matches!(function.kind(), KnownFunction::Rgb | KnownFunction::Rgba)
        && !function.is_valid_rgb()
    {
        return;
    }
    if function.kind().is_color() {
        if cx
            .value_context
            .is_enabled(ValueContextFlags::MINIFY_COLORS)
            && let Some(color) = minify_rgb_function(function, cx, ast)
                .or_else(|| minify_hsl_function(function, cx, ast))
        {
            function.replacement = Some(color);
            cx.record_value_normalized();
        }
        return;
    }
    let is_gradient = function.kind().is_gradient();
    let gradient_contains_variable = is_gradient
        && function
            .arguments
            .iter()
            .any(|value| token_or_value_contains_variable(value, ast));
    if gradient_contains_variable {
        rollback_gradient_color_replacements(&mut function.arguments, ast);
    }
    let preserve_space_after_comma = cx
        .value_context
        .is_enabled(ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA);
    cx.value_context.set_enabled(
        ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA,
        cx.is_enabled(Options::PRESERVE_VARIABLE_FALLBACK_SPACE, OptionsOp::Any)
            && function.kind().is_variable(),
    );
    crate::token::minify_token_values(&mut function.arguments, cx, ast);
    cx.value_context.set_enabled(
        ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA,
        preserve_space_after_comma,
    );
    if is_gradient
        && !gradient_contains_variable
        && (minify_gradient_direction(&mut function.arguments, ast)
            | minify_gradient_stops(&mut function.arguments, ast))
    {
        cx.record_value_normalized();
    }
    if function.kind() == KnownFunction::Calc && !function.is_vendor_prefixed() {
        if let Some(linear) = calc_linear_expression(&function.arguments, ast)
            .map(|linear| linear.round(cx.options().calc_precision))
            && linear.write_to(function, ast)
        {
            cx.record_value_normalized();
            if function.replacement.is_some() {
                return;
            }
        }
        if remove_redundant_calc_parentheses(&mut function.arguments, ast) {
            cx.record_value_normalized();
        }
        if minify_flat_calc_operations(&mut function.arguments, ast) {
            cx.record_value_normalized();
        }
        if let Some(value) = simple_calc_value(&function.arguments, ast) {
            function.replacement = Some(value);
            function.arguments.clear();
            cx.record_value_normalized();
            return;
        }
    }
    if function.kind() == KnownFunction::Url {
        if cx.is_enabled(Options::NORMALIZE_URLS, OptionsOp::Any) {
            function.set_name("url");
            let allocator = function.arguments.bump();
            if let [TokenOrValue::Token(token)] = function.arguments.as_mut_slice() {
                let mut normalized_value = None;
                let mut unquoted_url = false;
                ast.mutate_node(*token, |token, _| {
                    let Token::String(value) = token else {
                        return;
                    };
                    if let Some(normalized) = normalize_url_text(value) {
                        *value = allocator.alloc_str(&normalized);
                        normalized_value = Some(());
                    }
                    unquoted_url = !value.get(..5).is_some_and(
                        |prefix| match_ignore_ascii_case!(prefix, "data:" => true, _ => false),
                    ) && can_unquote_url(value);
                });
                if normalized_value.is_some() {
                    cx.record_value_normalized();
                }
                function.set_unquoted_url(unquoted_url);
            }
        } else if matches!(function.arguments.as_slice(), [TokenOrValue::Token(token)]
                if matches!(ast.ast_context().resolve_node(*token), Token::String(value)
                    if !value.get(..5).is_some_and(|prefix| {
                        match_ignore_ascii_case!(prefix, "data:" => true, _ => false)
                    })
                        && can_unquote_url(value)))
        {
            function.set_unquoted_url(true);
            cx.record_value_normalized();
        }
    }
    if cx.value_context.property == crate::context::PropertyContext::Transform
        && minify_transform_function(function, ast)
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

    let replacement = match function.kind() {
        KnownFunction::CubicBezier => minify_cubic_bezier(&function.arguments, ast),
        KnownFunction::Steps => minify_steps(&mut function.arguments, ast),
        _ => None,
    };
    if let Some(replacement) = replacement {
        function.set_name(replacement);
        function.arguments.clear();
        function.set_identifier(true);
        cx.record_value_normalized();
    }
}
