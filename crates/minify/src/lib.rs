//! Arena AST minification for rs-css.
//!
//! Minification is split into AST-aligned modules. It mutates an existing
//! stylesheet; compact whitespace serialization remains the code generator's
//! responsibility through `PrinterOptions::minify`.

mod color;
mod context;
mod css_rule;
mod length;
mod media;
mod options;
mod properties;
mod rules;
mod selector;
mod token;
mod values;

pub mod prelude;

use rs_css_allocator::Allocator;
use rs_css_ast::*;
use rs_css_visitor::{BoxError, Plugin, PluginContext, VisitMut, walk_mut};

pub use context::{MinifyAnalysis, MinifyContext, MinifyStats};
pub use options::MinifyOptions;

/// Minifies a stylesheet in place and returns transformation statistics.
pub fn minify<'a>(
    stylesheet: &mut StyleSheet<'a>,
    allocator: &'a Allocator,
    options: MinifyOptions,
) -> MinifyStats {
    let context = MinifyContext::new(allocator, stylesheet, options);
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
        let stats = minify(stylesheet, context.allocator(), self.options);
        context.insert(stats);
        Ok(())
    }
}

struct Minifier<'a> {
    context: MinifyContext<'a>,
}

impl<'a> VisitMut<'a> for Minifier<'a> {
    fn visit_style_sheet(&mut self, node: &mut StyleSheet<'a>) {
        walk_mut::walk_style_sheet(self, node);
        rules::minify_rule_list(&mut node.rules, &mut self.context);
    }

    fn visit_declaration_block(&mut self, node: &mut DeclarationBlock<'a>) {
        walk_mut::walk_declaration_block(self, node);
        properties::minify_declaration_block(node, &mut self.context);
    }

    fn visit_keyframe_selector(&mut self, node: &mut KeyframeSelector<'a>) {
        walk_mut::walk_keyframe_selector(self, node);
        if self.context.options().normalize_values && rules::minify_keyframe_selector(node) {
            self.context.record_value_normalized();
        }
    }

    fn visit_keyframes_rule(&mut self, node: &mut KeyframesRule<'a>) {
        walk_mut::walk_keyframes_rule(self, node);
        rules::minify_keyframes(node, &mut self.context);
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
        if matches!(
            node.name.to_ascii_lowercase().as_str(),
            "calc" | "min" | "max" | "clamp" | "round" | "rem" | "mod" | "abs" | "sign" | "hypot"
        ) {
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
        if self.context.options().normalize_values || self.context.options().discard_duplicates {
            selector::minify_selector_list(node, &mut self.context);
        }
    }

    fn visit_media_list(&mut self, node: &mut MediaList<'a>) {
        walk_mut::walk_media_list(self, node);
        media::minify_media_list(node, &mut self.context);
    }
}

#[cfg(test)]
mod tests {
    use rs_css_codegen::{PrinterOptions, ToCss};
    use rs_css_parser::{ParserOptions, parse};
    use rs_css_visitor::{PluginContext, Plugins};

    use super::*;

    fn run(source: &str) -> String {
        let allocator = Allocator::new();
        let mut stylesheet = parse(source, &allocator, ParserOptions::default()).unwrap();
        minify(&mut stylesheet, &allocator, MinifyOptions::default());
        stylesheet
            .to_css_string(PrinterOptions { minify: true })
            .unwrap()
    }

    #[test]
    fn normalizes_values_and_discards_duplicates() {
        assert_eq!(
            run("a{color:yellow;width:16px;width:16px}"),
            "a{color:#ff0;width:1pc}"
        );
        assert_eq!(
            run("a{transition-duration:500ms;transform:rotate(.25turn)}"),
            "a{transition-duration:.5s;transform:rotate(90deg)}"
        );
    }

    #[test]
    fn merges_compatible_rules_and_removes_empty_rules() {
        assert_eq!(
            run("a{}a{color:red}a{color:red} @media print{a{}}"),
            "a{color:red}"
        );
        assert_eq!(run("a{color:red}b{color:red}"), "a,b{color:red}");
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
            &allocator,
            MinifyOptions {
                transform_custom_properties: false,
                ..MinifyOptions::default()
            },
        );
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { minify: true })
                .unwrap(),
            "a{--color:rgb(0 0 0);--size:calc(3px * 2)}"
        );
    }

    #[test]
    fn plugin_exposes_stats_and_precomputed_analysis() {
        let allocator = Allocator::new();
        let mut stylesheet = parse(
            "a{color:red;color:red}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();
        let analysis =
            MinifyContext::new(&allocator, &stylesheet, MinifyOptions::default()).analysis();
        assert_eq!(analysis.rules, 1);
        assert_eq!(analysis.declarations, 2);

        let mut plugins = Plugins::new();
        plugins.add(MinifyPlugin::default());
        let mut plugin_context = PluginContext::new(&allocator);
        plugins.run(&mut stylesheet, &mut plugin_context).unwrap();
        let stats = plugin_context.get::<MinifyStats>().unwrap();
        assert_eq!(stats.declarations_removed, 1);
    }
}
