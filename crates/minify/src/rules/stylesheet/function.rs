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

impl Minify for Function<'_> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
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
