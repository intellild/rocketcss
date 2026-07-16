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

use rocketcss_allocator::Allocator;
use rocketcss_ast::{match_ignore_ascii_case, *};
use rocketcss_visitor::{BoxError, Plugin, PluginContext, VisitMut, VisitorMut};

pub use context::{MinifyContext, MinifyStats};
pub use options::{MinifyOptions, Options, OptionsOp};

/// Minifies a syntax-tree node in place.
pub trait Minify {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx;
}

/// Minifies a stylesheet in place and returns transformation statistics.
pub fn minify<'a>(stylesheet: &mut StyleSheet<'a>, options: MinifyOptions) -> MinifyStats {
    let allocator = Allocator::new();
    let mut cx = MinifyContext::new(options, &allocator);
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

pub(crate) fn minify_style_sheet<'ast, 'cx>(
    stylesheet: &mut StyleSheet<'ast>,
    cx: &mut MinifyContext<'cx>,
) where
    'ast: 'cx,
{
    // Minifier IR and transient collections are scratch state. Keep them out
    // of the AST arena so every temporary allocation is released when this
    // minify pass finishes.
    // Move the context into the visitor so it and its scratch IR share one
    // `'cx` lifetime, then restore it after traversal.
    let replacement = MinifyContext::new(cx.options(), cx.allocator());
    let owned_cx = std::mem::replace(cx, replacement);
    let allocator = owned_cx.allocator();
    let declaration_blocks = rules::DeclarationBlockMinifier::new(allocator);
    let mut minifier = Minifier {
        cx: owned_cx,
        declaration_blocks,
    };
    stylesheet.visit_mut(&mut minifier);
    let Minifier { cx: result, .. } = minifier;
    *cx = result;
}

struct Minifier<'ast, 'cx> {
    cx: MinifyContext<'cx>,
    declaration_blocks: rules::DeclarationBlockMinifier<'cx, 'ast>,
}

impl<'ast> VisitorMut<'ast> for Minifier<'ast, '_> {
    fn visit_declaration(&mut self, node: &mut Declaration<'ast>) {
        node.visit_mut_children(self);
        let remove_declaration = if let Declaration::FontFamily(families) = node {
            families.minify(&mut self.cx);
            families.iter().all(FontFamily::is_tombstone)
        } else {
            false
        };
        if remove_declaration {
            *node = Declaration::Tombstone;
            self.cx.record_declaration_removed();
        }
    }

    fn visit_font_family(&mut self, node: &mut FontFamily<'ast>) {
        if !matches!(node, FontFamily::Unparsed(_) | FontFamily::Tombstone) {
            node.visit_mut_children(self);
        }
    }

    fn visit_declaration_block(&mut self, mut node: std::pin::Pin<&mut DeclarationBlock<'ast>>) {
        node.as_mut().visit_mut_children(self);
        // SAFETY: minification mutates fields in place and never moves the
        // pinned declaration block itself.
        self.declaration_blocks
            .minify(unsafe { node.as_mut().get_unchecked_mut() }, &mut self.cx);
    }

    fn visit_keyframe_selector(&mut self, node: &mut KeyframeSelector<'ast>) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_unparsed_property(&mut self, node: &mut UnparsedProperty<'ast>) {
        let previous = self.cx.value_context;
        self.cx.value_context = properties::value_context(
            &node.property_id,
            self.cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
            self.cx
                .is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
        );
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_custom_property(&mut self, node: &mut CustomProperty<'ast>) {
        let previous = self.cx.value_context;
        self.cx.value_context = properties::custom_property_context(&self.cx);
        let name = match &*node.name {
            CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => *name,
        };
        if match_ignore_ascii_case!(name, "--font-family" => true, _ => false) {
            self.cx.value_context.property = context::PropertyContext::Font;
        }
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_function(&mut self, node: &mut Function<'ast>) {
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
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_variable(&mut self, node: &mut Variable<'ast>) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_environment_variable(&mut self, node: &mut EnvironmentVariable<'ast>) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_unknown_at_rule(&mut self, node: &mut UnknownAtRule<'ast>) {
        let previous = self.cx.value_context;
        self.cx.value_context = Default::default();
        self.cx
            .value_context
            .set_enabled(context::ValueContextFlags::SKIP_VALUE_TRANSFORMS, true);
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_token_or_value(&mut self, node: &mut TokenOrValue<'ast>) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_length_value(&mut self, node: &mut LengthValue) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_angle(&mut self, node: &mut Angle) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_time(&mut self, node: &mut Time) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_resolution(&mut self, node: &mut Resolution) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_ratio(&mut self, node: &mut Ratio) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
    }

    fn visit_selector_list(&mut self, node: &mut SelectorList<'ast>) {
        self.visit_selector_list_children(node);
        let allocator = self.cx.allocator();
        selector::minify_selector_list(node, &mut self.cx, allocator);
    }

    fn visit_media_list(&mut self, node: &mut MediaList<'ast>) {
        node.visit_mut_children(self);
        node.minify(&mut self.cx);
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
    fn deduplicates_equivalent_font_families() {
        assert_eq!(
            run("a{font-family:\"A\",Arial,a,sans-serif}"),
            "a{font-family:A,Arial,sans-serif}"
        );
        assert_eq!(
            run("a{font-family:\"serif\",serif}"),
            "a{font-family:\"serif\",serif}"
        );
        assert_eq!(
            run("a{font-family:A,A,serif,Helvetica}"),
            "a{font-family:A,serif}"
        );
        assert_eq!(
            run("a{font-family:monospace,monospace}"),
            "a{font-family:monospace}"
        );
        assert_eq!(
            run("a{font-family:A,var(--family),a,serif}"),
            "a{font-family:A,serif}"
        );

        let allocator = Allocator::new();
        let mut stylesheet = parse(
            "a{font-family:A,var(--family),a,serif}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();
        minify(&mut stylesheet, MinifyOptions::default());
        let CssRule::Style(rule) = &stylesheet.rules[0] else {
            panic!("expected style rule")
        };
        let Declaration::FontFamily(families) = &rule.declarations.declarations[0] else {
            panic!("expected typed font-family declaration")
        };
        assert!(matches!(families[0], FontFamily::Custom("A")));
        assert!(matches!(families[1], FontFamily::Tombstone));
        assert!(matches!(families[2], FontFamily::Tombstone));
        assert!(matches!(families[3], FontFamily::Serif));
    }

    #[test]
    fn font_family_deduplication_is_configurable() {
        let mut options = MinifyOptions::default();
        options.flags.remove(Options::NORMALIZE_VALUES);

        assert_eq!(
            run_with_options("a{font-family:\"A\",Arial,a,sans-serif}", options),
            "a{font-family:A,Arial,a,sans-serif}"
        );
    }

    #[test]
    fn removes_font_family_declarations_containing_only_tombstones() {
        let allocator = Allocator::new();
        let mut stylesheet = parse(
            "a{font-family:var(--family);font-family:slab inherit}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();
        let stats = minify(&mut stylesheet, MinifyOptions::default());
        let CssRule::Style(rule) = &stylesheet.rules[0] else {
            panic!("expected style rule")
        };
        assert!(
            rule.declarations
                .declarations
                .iter()
                .all(Declaration::is_tombstone)
        );
        assert_eq!(stats.declarations_removed, 2);
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            "a{}"
        );
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
    fn preserves_rule_structure() {
        assert_eq!(
            run("a{}a{color:red}a{color:red} @media print{a{}}"),
            "a{}a{color:red}a{color:red}@media print{a{}}"
        );
        assert_eq!(
            run("@charset 'UTF-8'; @import 'theme.css'; a{color:red}"),
            "@charset \"UTF-8\";@import \"theme.css\";a{color:red}"
        );
    }

    #[test]
    fn deduplicates_selectors_with_structural_hashes() {
        assert_eq!(run("h1,h2,h1,h2{color:red}"), "h1,h2{color:red}");
        assert_eq!(
            run("a:custom(1),b,a:custom(1),a:custom(2),b{color:red}"),
            "a:custom(1),b,a:custom(2){color:red}"
        );
        assert_eq!(
            run("a:custom(0),b,a:custom(-0),c,d{color:red}"),
            "a:custom(0),b,c,d{color:red}"
        );
        assert_eq!(
            run("a:is(.x,.x,.y),a:is(.x,.x,.y){color:red}"),
            "a:is(.x,.x,.y){color:red}"
        );
    }

    #[test]
    fn selector_deduplication_is_configurable() {
        let mut options = MinifyOptions::default();
        options.flags.remove(Options::DEDUPLICATE_LISTS);

        assert_eq!(
            run_with_options("h1,h2,h1,h2{color:red}", options),
            "h1,h2,h1,h2{color:red}"
        );
    }

    #[test]
    fn removes_exact_duplicate_declarations_within_one_block() {
        assert_eq!(
            run("h1{font-weight:700;font-weight:700}"),
            "h1{font-weight:700}"
        );
        assert_eq!(
            run("h1{font-weight:bold;font-weight:bold}"),
            "h1{font-weight:700}"
        );
        assert_eq!(
            run("h1{margin:10px 0 10px 0;margin:10px 0}"),
            "h1{margin:10px 0}"
        );
        assert_eq!(
            run("a{width:1px;color:red;width:1px}"),
            "a{color:red;width:1px}"
        );
        assert_eq!(
            run("a{width:1px!important;width:1px!important}"),
            "a{width:1px !important}"
        );
        assert_eq!(
            run("a{-webkit-user-select:none;-webkit-user-select:none}"),
            "a{-webkit-user-select:none}"
        );
        assert_eq!(run("a{--x:1;--x:1}"), "a{--x:1}");
        assert_eq!(run("a{unknown:value;unknown:value}"), "a{unknown:value}");
        assert_eq!(
            run(
                "a{width:1px;height:1px;top:1px;right:1px;bottom:1px;left:1px;color:red;opacity:1;z-index:1;width:1px}"
            ),
            "a{height:1px;top:1px;right:1px;bottom:1px;left:1px;color:red;opacity:1;z-index:1;width:1px}"
        );
        assert_eq!(run("a{width:1px;width:1px;width:1px}"), "a{width:1px}");
        assert_eq!(
            run("a{height:1px;width:1px;width:1px;color:red}"),
            "a{height:1px;width:1px;color:red}"
        );

        let allocator = Allocator::new();
        let mut stylesheet = parse(
            "a{width:1px;color:red;width:1px}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();
        let stats = minify(&mut stylesheet, MinifyOptions::default());
        let CssRule::Style(rule) = &stylesheet.rules[0] else {
            panic!("expected style rule")
        };
        assert_eq!(rule.declarations.len(), 3);
        assert_eq!(rule.declarations.declarations_importance.len(), 3);
        assert!(matches!(
            rule.declarations.declarations[0],
            Declaration::Tombstone
        ));
        assert_eq!(stats.declarations_removed, 1);

        let stats = minify(&mut stylesheet, MinifyOptions::default());
        assert_eq!(stats.declarations_removed, 0);
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            "a{color:red;width:1px}"
        );
    }

    #[test]
    fn preserves_declaration_fallbacks_and_importance() {
        assert_eq!(
            run("a{width:1px;width:2px;width:1px}"),
            "a{width:1px;width:2px;width:1px}"
        );
        assert_eq!(
            run("a{width:1px;width:1px!important}"),
            "a{width:1px;width:1px !important}"
        );
    }

    #[test]
    fn minifies_box_longhands_through_single_pass_ir() {
        assert_eq!(
            run("a{margin-top:10px;margin-right:20px;margin-bottom:10px;margin-left:20px}"),
            "a{margin:10px 20px}"
        );
        assert_eq!(
            run("a{padding-left:4px;padding-top:1px;padding-bottom:3px;padding-right:2px}"),
            "a{padding:1px 2px 3px 4px}"
        );
        assert_eq!(
            run("a{margin-top:1px;margin-right:2px;margin:3px}"),
            "a{margin:3px}"
        );
        assert_eq!(
            run("a{padding:1px;padding-left:2px}"),
            "a{padding:1px 1px 1px 2px}"
        );
        assert_eq!(
            run("a{margin:1px 2px;margin-left:2px}"),
            "a{margin:1px 2px}"
        );
        assert_eq!(
            run(
                "a{margin-top:1px!important;margin-right:1px!important;margin-bottom:1px!important;margin-left:1px!important}"
            ),
            "a{margin:1px !important}"
        );
    }

    #[test]
    fn box_ir_preserves_fallback_and_logical_property_barriers() {
        assert_eq!(
            run("a{margin:inherit;margin-left:1px}"),
            "a{margin:inherit;margin-left:1px}"
        );
        assert_eq!(
            run("a{margin:1px;margin-left:var(--space)}"),
            "a{margin:1px;margin-left:var(--space)}"
        );
        assert_eq!(
            run("a{margin:1px;margin-left:var(--space);margin-left:2px}"),
            "a{margin:1px;margin-left:var(--space);margin-left:2px}"
        );
        assert_eq!(
            run("a{padding:1px;padding-top:var(--space);padding-top:2px}"),
            "a{padding:1px;padding-top:var(--space);padding-top:2px}"
        );
        assert_eq!(
            run("a{margin:1px;margin-left:var(--space);margin-right:2px}"),
            "a{margin:1px 2px 1px 1px;margin-left:var(--space)}"
        );
        assert_eq!(
            run("a{margin-left:1px;margin:invalid}"),
            "a{margin-left:1px;margin:invalid}"
        );
        assert_eq!(
            run("a{padding-left:1px;padding:auto}"),
            "a{padding-left:1px;padding:auto}"
        );
        assert_eq!(
            run(
                "a{margin-top:1px;margin-inline-start:2px;margin-right:3px;margin-bottom:4px;margin-left:5px}"
            ),
            "a{margin-top:1px;margin-inline-start:2px;margin-right:3px;margin-bottom:4px;margin-left:5px}"
        );
        assert_eq!(
            run(
                "a{padding-top:1px!important;padding-right:1px;padding-bottom:1px;padding-left:1px}"
            ),
            "a{padding-top:1px !important;padding-right:1px;padding-bottom:1px;padding-left:1px}"
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
        assert_eq!(columns, PropertyId::Columns(VendorPrefix::NONE));
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
        let mut stylesheet = parse(
            "a{width:16px;width:16px}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();
        let mut plugins = Plugins::new();
        plugins.add(MinifyPlugin::default());
        let mut plugin_context = PluginContext::new(&allocator);
        plugins.run(&mut stylesheet, &mut plugin_context).unwrap();
        let stats = plugin_context.get::<MinifyStats>().unwrap();
        assert_eq!(stats.values_normalized, 2);
        assert_eq!(stats.declarations_removed, 1);
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
