mod context;
mod length;
mod media;
mod options;
mod properties;
mod rules;
mod selector;
mod token;
mod values;

pub mod prelude;

use rs_css_ast::*;
use rs_css_visitor::{BoxError, Plugin, PluginContext, VisitMut, walk_mut};

pub use context::{MinifyContext, MinifyStats};
pub use options::MinifyOptions;

/// Minifies a stylesheet in place and returns transformation statistics.
pub fn minify<'a>(stylesheet: &mut StyleSheet<'a>, options: MinifyOptions) -> MinifyStats {
    let context = MinifyContext::new(options);
    let mut minifier = Minifier { context };
    minifier.visit_style_sheet(stylesheet);
    minifier.context.stats()
}

/// Adapter for running minification in a visitor plugin pipeline.
#[derive(Clone, Copy, Debug, Default)]
pub struct MinifyPlugin {
    options: MinifyOptions,
}

impl MinifyPlugin {
    #[inline]
    pub fn new(options: MinifyOptions) -> Self {
        Self { options }
    }

    #[inline]
    pub fn options(&self) -> MinifyOptions {
        self.options
    }
}

impl<'a> Plugin<'a> for MinifyPlugin {
    fn name(&self) -> &str {
        "minify"
    }

    fn transform(
        &mut self,
        stylesheet: &mut StyleSheet<'a>,
        context: &mut PluginContext<'a>,
    ) -> Result<(), BoxError> {
        let stats = minify(stylesheet, self.options);
        context.insert(stats);
        Ok(())
    }
}

struct Minifier {
    context: MinifyContext,
}

impl<'a> VisitMut<'a> for Minifier {
    fn visit_keyframe_selector(&mut self, node: &mut KeyframeSelector<'a>) {
        walk_mut::walk_keyframe_selector(self, node);
        if self.context.options().normalize_values && rules::minify_keyframe_selector(node) {
            self.context.record_value_normalized();
        }
    }

    fn visit_unparsed_property(&mut self, node: &mut UnparsedProperty<'a>) {
        let previous = self.context.value_context;
        self.context.value_context = properties::value_context(&node.property_id);
        walk_mut::walk_unparsed_property(self, node);
        token::minify_token_list(&mut node.value, &mut self.context);
        self.context.value_context = previous;
    }

    fn visit_custom_property(&mut self, node: &mut CustomProperty<'a>) {
        let previous = self.context.value_context;
        self.context.value_context = properties::custom_property_context(&self.context);
        walk_mut::walk_custom_property(self, node);
        token::minify_token_list(&mut node.value, &mut self.context);
        self.context.value_context = previous;
    }

    fn visit_function(&mut self, node: &mut Function<'a>) {
        let previous = self.context.value_context;
        if is_math_function(node.name) {
            self.context.value_context.allow_unitless_zero = false;
        }
        walk_mut::walk_function(self, node);
        token::minify_token_list(&mut node.arguments, &mut self.context);
        self.context.value_context = previous;
    }

    fn visit_variable(&mut self, node: &mut Variable<'a>) {
        walk_mut::walk_variable(self, node);
        if let Some(fallback) = &mut node.fallback {
            token::minify_token_list(fallback, &mut self.context);
        }
    }

    fn visit_environment_variable(&mut self, node: &mut EnvironmentVariable<'a>) {
        walk_mut::walk_environment_variable(self, node);
        if let Some(fallback) = &mut node.fallback {
            token::minify_token_list(fallback, &mut self.context);
        }
    }

    fn visit_unknown_at_rule(&mut self, node: &mut UnknownAtRule<'a>) {
        let previous = self.context.value_context;
        self.context.value_context = Default::default();
        self.context.value_context.skip_value_transforms = true;
        walk_mut::walk_unknown_at_rule(self, node);
        token::minify_token_list(&mut node.prelude, &mut self.context);
        if let Some(block) = &mut node.block {
            token::minify_token_list(block, &mut self.context);
        }
        self.context.value_context = previous;
    }

    fn visit_token_or_value(&mut self, node: &mut TokenOrValue<'a>) {
        walk_mut::walk_token_or_value(self, node);
        if self.context.options().normalize_values {
            token::minify_token_or_value(node, &mut self.context);
        }
    }

    fn visit_length_value(&mut self, node: &mut LengthValue) {
        walk_mut::walk_length_value(self, node);
        if self.context.options().normalize_values && length::minify_length(node) {
            self.context.record_value_normalized();
        }
    }

    fn visit_angle(&mut self, node: &mut Angle) {
        walk_mut::walk_angle(self, node);
        if self.context.options().normalize_values && length::minify_angle(node) {
            self.context.record_value_normalized();
        }
    }

    fn visit_time(&mut self, node: &mut Time) {
        walk_mut::walk_time(self, node);
        if self.context.options().normalize_values && length::minify_time(node) {
            self.context.record_value_normalized();
        }
    }

    fn visit_resolution(&mut self, node: &mut Resolution) {
        walk_mut::walk_resolution(self, node);
        if self.context.options().normalize_values && length::minify_resolution(node) {
            self.context.record_value_normalized();
        }
    }

    fn visit_ratio(&mut self, node: &mut Ratio) {
        walk_mut::walk_ratio(self, node);
        if self.context.options().normalize_values && values::minify_ratio(node) {
            self.context.record_value_normalized();
        }
    }

    fn visit_selector_list(&mut self, node: &mut SelectorList<'a>) {
        walk_mut::walk_selector_list(self, node);
        if self.context.options().normalize_values || self.context.options().deduplicate_lists {
            selector::minify_selector_list(node, &mut self.context);
        }
    }

    fn visit_media_list(&mut self, node: &mut MediaList<'a>) {
        walk_mut::walk_media_list(self, node);
        media::minify_media_list(node, &mut self.context);
    }
}

fn is_math_function(name: &str) -> bool {
    [
        "calc", "min", "max", "clamp", "round", "rem", "mod", "abs", "sign", "hypot",
    ]
    .into_iter()
    .any(|candidate| name.eq_ignore_ascii_case(candidate))
}

#[cfg(test)]
mod tests {
    use rs_css_allocator::Allocator;
    use rs_css_codegen::{PrinterOptions, ToCss};
    use rs_css_parser::{ParserOptions, parse};
    use rs_css_visitor::{PluginContext, Plugins};

    use super::*;

    fn run(source: &str) -> String {
        let allocator = Allocator::new();
        let mut stylesheet = parse(source, &allocator, ParserOptions::default()).unwrap();
        minify(&mut stylesheet, MinifyOptions::default());
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap()
    }

    #[test]
    fn normalizes_values_without_rewriting_siblings() {
        assert_eq!(
            run("a{color:yellow;width:16px;width:16px}"),
            "a{color:#ff0;width:1pc;width:1pc}"
        );
        assert_eq!(
            run("a{transition-duration:500ms;transform:rotate(.25turn)}"),
            "a{transition-duration:.5s;transform:rotate(90deg)}"
        );
    }

    #[test]
    fn preserves_rule_structure() {
        assert_eq!(
            run("a{}a{color:red}a{color:red} @media print{a{}}"),
            "a{}a{color:red}a{color:red}@media print{a{}}"
        );
        assert_eq!(run("a{color:red}b{color:red}"), "a{color:red}b{color:red}");
    }

    #[test]
    fn keeps_existing_token_storage() {
        let allocator = Allocator::new();
        let mut stylesheet = parse(
            "a{margin:1px 1px 1px 1px}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();
        let (buffer_before, token_before) = unparsed_value_storage(&stylesheet);

        minify(&mut stylesheet, MinifyOptions::default());

        let (buffer_after, token_after) = unparsed_value_storage(&stylesheet);
        assert_eq!(buffer_after, buffer_before);
        assert_eq!(token_after, token_before);
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            "a{margin:1px}"
        );
    }

    #[test]
    fn preserves_typed_zero_in_calc() {
        assert_eq!(run("a{width:calc(0px + 1em)}"), "a{width:calc(0px + 1em)}");
    }

    #[test]
    fn custom_property_transforms_are_configurable() {
        let allocator = Allocator::new();
        let mut stylesheet = parse(
            "a{--color:rgb(0 0 0);--size:calc(3px * 2)}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();
        minify(
            &mut stylesheet,
            MinifyOptions {
                transform_custom_properties: false,
                ..MinifyOptions::default()
            },
        );
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            "a{--color:rgb(0 0 0);--size:calc(3px * 2)}"
        );
    }

    #[test]
    fn plugin_exposes_local_normalization_stats() {
        let allocator = Allocator::new();
        let mut stylesheet = parse("a{width:16px}", &allocator, ParserOptions::default()).unwrap();
        let mut plugins = Plugins::new();
        plugins.add(MinifyPlugin::default());
        let mut plugin_context = PluginContext::new(&allocator);
        plugins.run(&mut stylesheet, &mut plugin_context).unwrap();
        let stats = plugin_context.get::<MinifyStats>().unwrap();
        assert_eq!(stats.values_normalized, 1);
    }

    fn unparsed_value_storage<'a>(
        stylesheet: &StyleSheet<'a>,
    ) -> (*const TokenOrValue<'a>, *const Token<'a>) {
        let CssRule::Style(rule) = &stylesheet.rules[0] else {
            panic!("expected style rule")
        };
        let Declaration::Unparsed(property) = &rule.declarations.declarations[0] else {
            panic!("expected unparsed property")
        };
        let TokenOrValue::Token(token) = &property.value[0] else {
            panic!("expected token value")
        };
        (property.value.as_ptr(), &**token)
    }
}
