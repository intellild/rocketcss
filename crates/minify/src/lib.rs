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

use rocketcss_ast::*;
use rocketcss_visitor::{BoxError, Plugin, PluginContext, VisitMut, walk_mut};

pub use context::{MinifyContext, MinifyStats};
pub use options::MinifyOptions;

/// Minifies a syntax-tree node in place.
pub trait Minify {
    fn minify(&mut self, context: &mut MinifyContext);
}

/// Minifies a stylesheet in place and returns transformation statistics.
pub fn minify<'a>(stylesheet: &mut StyleSheet<'a>, options: MinifyOptions) -> MinifyStats {
    let mut context = MinifyContext::new(options);
    stylesheet.minify(&mut context);
    context.stats()
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

pub(crate) fn minify_style_sheet<'a>(stylesheet: &mut StyleSheet<'a>, context: &mut MinifyContext) {
    if context.options().discard_license_comments {
        stylesheet.license_comments.clear();
    }
    rules::coalesce_conditional_rules(&mut stylesheet.rules, context);
    Minifier { context }.visit_style_sheet(stylesheet);
    rules::merge_identical_identifier_rules(stylesheet, context);
    rules::discard_unused_definitions(stylesheet, context);
    rules::reduce_keyframe_identifiers(stylesheet, context);
    rules::reduce_counter_style_identifiers(stylesheet, context);
    rules::reduce_counter_identifiers(stylesheet, context);
    rules::reduce_grid_identifiers(stylesheet, context);
    values::reduce_z_indices(stylesheet, context);
    rules::minify_rule_list(&mut stylesheet.rules, context);
}

struct Minifier<'context> {
    context: &'context mut MinifyContext,
}

impl<'a> VisitMut<'a> for Minifier<'_> {
    fn visit_font_face_property(&mut self, node: &mut FontFaceProperty<'a>) {
        walk_mut::walk_font_face_property(self, node);
        rules::minify_font_face_property(node, self.context);
    }

    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        walk_mut::walk_declaration(self, node);
        node.minify(self.context);
    }

    fn visit_keyframe_selector(&mut self, node: &mut KeyframeSelector<'a>) {
        walk_mut::walk_keyframe_selector(self, node);
        node.minify(self.context);
    }

    fn visit_unparsed_property(&mut self, node: &mut UnparsedProperty<'a>) {
        let previous = self.context.value_context;
        self.context.value_context = properties::value_context(
            &node.property_id,
            self.context.options().order_values,
            self.context.options().convert_zero_percentages,
        );
        walk_mut::walk_unparsed_property(self, node);
        node.minify(self.context);
        self.context.value_context = previous;
    }

    fn visit_custom_property(&mut self, node: &mut CustomProperty<'a>) {
        let previous = self.context.value_context;
        self.context.value_context = properties::custom_property_context(self.context);
        let name = match &*node.name {
            CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => *name,
        };
        if name.eq_ignore_ascii_case("--font-family") {
            self.context.value_context.property = context::PropertyContext::Font;
        }
        walk_mut::walk_custom_property(self, node);
        node.minify(self.context);
        self.context.value_context = previous;
    }

    fn visit_function(&mut self, node: &mut Function<'a>) {
        let previous = self.context.value_context;
        if is_math_function(node.name) {
            self.context.value_context.allow_unitless_zero_length = false;
            self.context.value_context.allow_unitless_zero_percentage = false;
            self.context.value_context.property = context::PropertyContext::Generic;
        }
        if node.name.eq_ignore_ascii_case("hwb") {
            self.context.value_context.allow_unitless_zero_length = false;
            self.context.value_context.allow_unitless_zero_percentage = false;
        }
        if node.name.eq_ignore_ascii_case("color-mix") || node.name.eq_ignore_ascii_case("linear") {
            self.context.value_context.allow_unitless_zero_percentage = false;
        }
        if rules::is_gradient_function(node.name) {
            self.context.value_context.property = context::PropertyContext::Generic;
        }
        if node.name.eq_ignore_ascii_case("local") {
            self.context.value_context.minify_colors = false;
        }
        walk_mut::walk_function(self, node);
        node.minify(self.context);
        self.context.value_context = previous;
    }

    fn visit_variable(&mut self, node: &mut Variable<'a>) {
        walk_mut::walk_variable(self, node);
        node.minify(self.context);
    }

    fn visit_environment_variable(&mut self, node: &mut EnvironmentVariable<'a>) {
        walk_mut::walk_environment_variable(self, node);
        node.minify(self.context);
    }

    fn visit_unknown_at_rule(&mut self, node: &mut UnknownAtRule<'a>) {
        let previous = self.context.value_context;
        self.context.value_context = Default::default();
        self.context.value_context.skip_value_transforms = true;
        walk_mut::walk_unknown_at_rule(self, node);
        node.minify(self.context);
        self.context.value_context = previous;
    }

    fn visit_token_or_value(&mut self, node: &mut TokenOrValue<'a>) {
        walk_mut::walk_token_or_value(self, node);
        node.minify(self.context);
    }

    fn visit_length_value(&mut self, node: &mut LengthValue) {
        walk_mut::walk_length_value(self, node);
        node.minify(self.context);
    }

    fn visit_angle(&mut self, node: &mut Angle) {
        walk_mut::walk_angle(self, node);
        node.minify(self.context);
    }

    fn visit_time(&mut self, node: &mut Time) {
        walk_mut::walk_time(self, node);
        node.minify(self.context);
    }

    fn visit_resolution(&mut self, node: &mut Resolution) {
        walk_mut::walk_resolution(self, node);
        node.minify(self.context);
    }

    fn visit_ratio(&mut self, node: &mut Ratio) {
        walk_mut::walk_ratio(self, node);
        node.minify(self.context);
    }

    fn visit_selector_list(&mut self, node: &mut SelectorList<'a>) {
        walk_mut::walk_selector_list(self, node);
        node.minify(self.context);
    }

    fn visit_selector(&mut self, node: &mut Selector<'a>) {
        walk_mut::walk_selector(self, node);
        if selector::minify_selector(node) {
            self.context.record_value_normalized();
        }
    }

    fn visit_media_list(&mut self, node: &mut MediaList<'a>) {
        walk_mut::walk_media_list(self, node);
        node.minify(self.context);
    }
}

fn is_math_function(name: &str) -> bool {
    let name = name
        .strip_prefix('-')
        .and_then(|name| name.split_once('-').map(|(_, name)| name))
        .unwrap_or(name);
    [
        "calc", "min", "max", "clamp", "round", "rem", "mod", "abs", "sign", "hypot",
    ]
    .into_iter()
    .any(|candidate| name.eq_ignore_ascii_case(candidate))
}

#[cfg(test)]
mod tests {
    use rocketcss_allocator::Allocator;
    use rocketcss_codegen::{PrinterOptions, ToCss};
    use rocketcss_parser::{ParserOptions, parse};
    use rocketcss_visitor::{PluginContext, Plugins};

    use super::*;

    fn run(source: &str) -> String {
        run_with_options(source, MinifyOptions::default())
    }

    fn run_with_options(source: &str, options: MinifyOptions) -> String {
        let allocator = Allocator::new();
        let mut stylesheet = parse(source, &allocator, ParserOptions::default()).unwrap();
        minify(&mut stylesheet, options);
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap()
    }

    #[test]
    fn normalizes_values_and_removes_exact_duplicate_declarations() {
        assert_eq!(
            run("a{color:yellow;width:16px;width:16px}"),
            "a{color:#ff0;width:1pc}"
        );
        assert_eq!(
            run("a{width:1px;color:red;width:1px}"),
            "a{color:red;width:1px}"
        );
        assert_eq!(
            run("a{width:1px;width:2px;width:1px}"),
            "a{width:1px;width:2px;width:1px}"
        );
        assert_eq!(
            run("a{transition-duration:500ms;transform:rotate(.25turn)}"),
            "a{transition-duration:.5s;transform:rotate(90deg)}"
        );
    }

    #[test]
    fn merges_adjacent_style_rules() {
        assert_eq!(
            run("a{}a{color:red}a{color:red} @media print{a{}}"),
            "a{color:red}"
        );
        assert_eq!(run("a{color:red}b{color:red}"), "a,b{color:red}");
        assert_eq!(
            run("a{padding-left:1px}a{padding-right:2px}"),
            "a{padding-left:1px;padding-right:2px}"
        );
    }

    #[test]
    fn treats_at_rules_as_style_merge_barriers() {
        assert_eq!(
            run("a{padding-left:1px}@media print{a{padding-left:3px}}a{padding-right:2px}"),
            "a{padding-left:1px}@media print{a{padding-left:3px}}a{padding-right:2px}"
        );
    }

    #[test]
    fn coalesces_adjacent_conditional_rules_before_minifying_children() {
        assert_eq!(
            run("@media print{a{padding-left:1px}}@media print{a{padding-right:2px}}"),
            "@media print{a{padding-left:1px;padding-right:2px}}"
        );
        assert_eq!(
            run("@media print{a{color:red}}@media screen{a{color:red}}"),
            "@media print{a{color:red}}@media screen{a{color:red}}"
        );
        assert_eq!(
            run(
                "@media print{a{padding-top:1px}}@media print{a{padding-right:2px}}@media print{a{padding-bottom:3px}}"
            ),
            "@media print{a{padding-top:1px;padding-right:2px;padding-bottom:3px}}"
        );
    }

    #[test]
    fn repeated_minification_keeps_block_links_stable() {
        let allocator = Allocator::new();
        let mut stylesheet = parse(
            "a{color:red}a{color:red;padding-left:1px}a{padding-right:2px}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();

        minify(&mut stylesheet, MinifyOptions::default());
        minify(&mut stylesheet, MinifyOptions::default());

        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            "a{color:red;padding-left:1px;padding-right:2px}"
        );
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
    fn rebases_positive_z_indices_in_place() {
        let options = MinifyOptions {
            reduce_z_indices: true,
            ..MinifyOptions::default()
        };
        assert_eq!(
            run_with_options(
                "a{z-index:600}b{z-index:350}c{z-index:150}d{z-index:0}e{z-index:auto}",
                options,
            ),
            "a{z-index:3}b{z-index:2}c{z-index:1}d{z-index:0}e{z-index:auto}"
        );
        assert_eq!(
            run_with_options(
                "a{z-index:8}b{z-index:-2}c{z-index:10}d{z-index:6}",
                options,
            ),
            "a{z-index:8}b{z-index:-2}c{z-index:10}d{z-index:6}"
        );
        assert_eq!(
            run_with_options(
                "a{z-index:20}",
                MinifyOptions {
                    z_index_start: 15,
                    ..options
                },
            ),
            "a{z-index:15}"
        );
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
