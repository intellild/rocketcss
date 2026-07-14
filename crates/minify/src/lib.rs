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

use rocketcss_ast::{match_ignore_ascii_case, *};
use rocketcss_visitor::{BoxError, Plugin, PluginContext, VisitMut, VisitorMut};

pub use context::{MinifyContext, MinifyStats};
pub use options::{MinifyOptions, Options, OptionsOp};

/// Minifies a syntax-tree node in place.
pub trait Minify {
    fn minify(&mut self, cx: &mut MinifyContext);
}

/// Minifies a stylesheet in place and returns transformation statistics.
pub fn minify<'a>(stylesheet: &mut StyleSheet<'a>, options: MinifyOptions) -> MinifyStats {
    let mut cx = MinifyContext::new(options);
    stylesheet.minify(&mut cx);
    cx.stats()
}

/// Adapter for running node-local minification in a visitor plugin pipeline.
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
        cx: &mut PluginContext<'a>,
    ) -> Result<(), BoxError> {
        let stats = minify(stylesheet, self.options);
        cx.insert(stats);
        Ok(())
    }
}

pub(crate) fn minify_style_sheet<'a>(stylesheet: &mut StyleSheet<'a>, cx: &mut MinifyContext) {
    stylesheet.visit_mut(&mut Minifier { cx });
}

struct Minifier<'cx> {
    cx: &'cx mut MinifyContext,
}

impl<'a> VisitorMut<'a> for Minifier<'_> {
    fn visit_keyframe_selector(&mut self, node: &mut KeyframeSelector<'a>) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_unparsed_property(&mut self, node: &mut UnparsedProperty<'a>) {
        let previous = self.cx.value_context;
        self.cx.value_context = properties::value_context(
            &node.property_id,
            self.cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
            self.cx
                .is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
        );
        node.visit_mut_children(self);
        node.minify(self.cx);
        self.cx.value_context = previous;
    }

    fn visit_custom_property(&mut self, node: &mut CustomProperty<'a>) {
        let previous = self.cx.value_context;
        self.cx.value_context = properties::custom_property_context(self.cx);
        let name = match &*node.name {
            CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => *name,
        };
        if match_ignore_ascii_case!(name, "--font-family" => true, _ => false) {
            self.cx.value_context.property = context::PropertyContext::Font;
        }
        node.visit_mut_children(self);
        node.minify(self.cx);
        self.cx.value_context = previous;
    }

    fn visit_function(&mut self, node: &mut Function<'a>) {
        let previous = self.cx.value_context;
        let kind = node.kind();
        if kind.is_math() {
            self.cx.value_context.set_enabled(
                context::ValueContextFlags::ALLOW_UNITLESS_ZERO_LENGTH
                    | context::ValueContextFlags::ALLOW_UNITLESS_ZERO_PERCENTAGE,
                false,
            );
            self.cx.value_context.property = context::PropertyContext::Generic;
        }
        match kind {
            KnownFunction::Hwb => self.cx.value_context.set_enabled(
                context::ValueContextFlags::ALLOW_UNITLESS_ZERO_LENGTH
                    | context::ValueContextFlags::ALLOW_UNITLESS_ZERO_PERCENTAGE,
                false,
            ),
            KnownFunction::ColorMix | KnownFunction::Linear => self.cx.value_context.set_enabled(
                context::ValueContextFlags::ALLOW_UNITLESS_ZERO_PERCENTAGE,
                false,
            ),
            _ => {}
        }
        if kind.is_gradient() {
            self.cx.value_context.property = context::PropertyContext::Generic;
        }
        if kind == KnownFunction::Local {
            self.cx
                .value_context
                .set_enabled(context::ValueContextFlags::MINIFY_COLORS, false);
        }
        node.visit_mut_children(self);
        node.minify(self.cx);
        self.cx.value_context = previous;
    }

    fn visit_variable(&mut self, node: &mut Variable<'a>) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_environment_variable(&mut self, node: &mut EnvironmentVariable<'a>) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_unknown_at_rule(&mut self, node: &mut UnknownAtRule<'a>) {
        let previous = self.cx.value_context;
        self.cx.value_context = Default::default();
        self.cx
            .value_context
            .set_enabled(context::ValueContextFlags::SKIP_VALUE_TRANSFORMS, true);
        node.visit_mut_children(self);
        node.minify(self.cx);
        self.cx.value_context = previous;
    }

    fn visit_token_or_value(&mut self, node: &mut TokenOrValue<'a>) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_length_value(&mut self, node: &mut LengthValue) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_angle(&mut self, node: &mut Angle) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_time(&mut self, node: &mut Time) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_resolution(&mut self, node: &mut Resolution) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_ratio(&mut self, node: &mut Ratio) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }

    fn visit_selector_list(&mut self, node: &mut SelectorList<'a>) {
        self.visit_selector_list_children(node);
        node.minify(self.cx);
    }

    fn visit_media_list(&mut self, node: &mut MediaList<'a>) {
        node.visit_mut_children(self);
        node.minify(self.cx);
    }
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
    fn normalizes_numbers_colors_and_lengths() {
        assert_eq!(
            run("a{color:rgb(255 0 0);border-color:#ffffff;width:16px}"),
            "a{color:red;border-color:#fff;width:1pc}"
        );
        assert_eq!(
            run("a{transition-duration:500ms;transform:rotate(.25turn)}"),
            "a{transition-duration:.5s;transform:rotate(90deg)}"
        );
        assert_eq!(run("a{MARGIN:1px 1px 1px 1px}"), "a{margin:1px}");
    }

    #[test]
    fn serializes_rgb_channels_as_integers() {
        let mut options = MinifyOptions::default();
        options.flags.remove(Options::USE_HEX_ALPHA_COLORS);

        assert_eq!(
            run_with_options("a{color:rgba(1 2 3/.5)}", options),
            "a{color:rgba(1,2,3,.5)}"
        );
    }

    #[test]
    fn folds_calc_values_in_the_existing_function_node() {
        let mut options = MinifyOptions::default();
        options
            .flags
            .remove(Options::CONVERT_LENGTH_UNITS | Options::CONVERT_EXTENDED_LENGTH_UNITS);
        assert_eq!(
            run_with_options(
                "a{width:calc(3px * 2);height:calc(100px + 50px - 25px)}",
                options,
            ),
            "a{width:6px;height:125px}"
        );
        assert_eq!(run("a{width:calc(0px + 1em)}"), "a{width:calc(0px + 1em)}");
    }

    #[test]
    fn dispatches_known_functions_without_repeated_name_matching() {
        assert_eq!(
            run("a{color:RGB(255 0 0);transform:ROTATEZ(1turn)}"),
            "a{color:red;transform:rotate(1turn)}"
        );
        assert_eq!(
            run("a{width:-WEBKIT-CALC(3px * 2)}"),
            "a{width:-WEBKIT-CALC(3px*2)}"
        );
    }

    #[test]
    fn preserves_rule_and_declaration_structure() {
        assert_eq!(
            run("a{}a{color:red}a{color:red} @media print{a{}}"),
            "a{}a{color:red}a{color:red}@media print{a{}}"
        );
        assert_eq!(run("a{width:1px;width:1px}"), "a{width:1px;width:1px}");
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
    fn custom_property_transforms_are_configurable() {
        let mut options = MinifyOptions::default();
        options.flags.remove(Options::TRANSFORM_CUSTOM_PROPERTIES);
        assert_eq!(
            run_with_options("a{--color:rgb(0 0 0);--size:calc(3px * 2)}", options),
            "a{--color:rgb(0 0 0);--size:calc(3px * 2)}"
        );
    }

    #[test]
    fn option_operations_are_explicit() {
        let options = MinifyOptions::default();
        assert!(options.is_enabled(
            Options::NORMALIZE_VALUES | Options::NORMALIZE_WHITESPACE,
            OptionsOp::And,
        ));
        assert!(options.is_enabled(
            Options::NORMALIZE_VALUES | Options::NORMALIZE_URLS,
            OptionsOp::Any,
        ));
        assert!(options.is_enabled(Options::NORMALIZE_URLS, OptionsOp::None));
    }

    #[test]
    fn property_context_dispatches_by_property_id() {
        let animation = PropertyId::Animation(VendorPrefix::WEBKIT);
        assert_eq!(
            properties::value_context(&animation, true, true).property,
            context::PropertyContext::Animation
        );
        assert_eq!(
            properties::value_context(&animation, false, true).property,
            context::PropertyContext::TimingFunction
        );

        let border = PropertyId::Border;
        assert_eq!(
            properties::value_context(&border, true, true).property,
            context::PropertyContext::Border
        );
        assert_eq!(
            properties::value_context(&border, false, true).property,
            context::PropertyContext::Generic
        );

        let columns = PropertyId::from_name("CoLuMnS");
        assert_eq!(columns, PropertyId::Columns);
        assert_eq!(
            properties::value_context(&columns, true, true).property,
            context::PropertyContext::Columns
        );
        assert_eq!(
            properties::value_context(&columns, false, true).property,
            context::PropertyContext::Generic
        );

        let prefixed_animation = PropertyId::from_name("-WebKit-ANIMATION");
        assert_eq!(
            prefixed_animation,
            PropertyId::Animation(VendorPrefix::WEBKIT)
        );
        assert_eq!(
            prefixed_animation
                .to_css_string(PrinterOptions::default())
                .unwrap(),
            "-webkit-animation"
        );
        assert_eq!(
            properties::value_context(&prefixed_animation, true, true).property,
            context::PropertyContext::Animation
        );
        assert_eq!(
            properties::value_context(&prefixed_animation, false, true).property,
            context::PropertyContext::TimingFunction
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
