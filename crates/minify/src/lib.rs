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

/// Adapter for running in-place minification in a visitor plugin pipeline.
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
    rules::merge_adjacent_style_rules(
        &mut stylesheet.rules,
        &mut minifier.declaration_blocks,
        &mut minifier.cx,
    );
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

    fn run_with_error_recovery(source: &str) -> String {
        let allocator = Allocator::new();
        let mut stylesheet = parse(
            source,
            &allocator,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        minify(&mut stylesheet, MinifyOptions::default());
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap()
    }

    #[test]
    fn removes_unparsed_selectors_from_mixed_selector_lists() {
        let allocator = Allocator::new();
        let mut stylesheet = parse(
            ".valid, (font-[family-name:var(--font-*)]), #also-valid { color: red }",
            &allocator,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        let stats = minify(&mut stylesheet, MinifyOptions::default());
        let CssRule::Style(rule) = &stylesheet.rules[0] else {
            panic!("expected style rule")
        };
        assert!(matches!(rule.selectors[0], Selector::Parsed(_)));
        assert!(matches!(rule.selectors[1], Selector::Tombstone));
        assert!(matches!(rule.selectors[2], Selector::Parsed(_)));
        assert_eq!(stats.values_normalized, 1);
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            ".valid,#also-valid{color:red}"
        );
    }

    #[test]
    #[ignore = "unparsed selectors must remain an unforgiving-list barrier"]
    fn preserves_unparsed_selector_list_barriers() {
        assert_eq!(
            run_with_error_recovery(
                ".valid,(font-[family-name:var(--font-*)]),#also-valid{color:red}"
            ),
            ".valid,(font-[family-name:var(--font-*)]),#also-valid{color:red}"
        );
    }

    #[test]
    fn removes_style_rules_containing_only_unparsed_selectors() {
        assert_eq!(
            run_with_error_recovery("(font-[family-name:var(--font-*)]) { color: red }"),
            ""
        );
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
    #[ignore]
    fn leaves_invalid_nonzero_unitless_lengths_unchanged() {
        assert_eq!(run("a{width:100}"), "a{width:100}");
    }

    #[test]
    #[ignore]
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
            "a{font-family:A,var(--family),serif}"
        );
        assert_eq!(
            run("a{font-family:A,serif,Helvetica;font-family:A,serif}"),
            "a{font-family:A,serif}"
        );
        assert_eq!(
            run("a{font-family:Inter,system-ui,sans-serif}"),
            "a{font-family:Inter,system-ui,sans-serif}"
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
        assert!(matches!(families[1], FontFamily::Unparsed(_)));
        assert!(matches!(families[2], FontFamily::Tombstone));
        assert!(matches!(families[3], FontFamily::Serif));
    }

    #[test]
    fn font_family_deduplication_is_configurable() {
        let mut options = MinifyOptions::default();
        options.flags.remove(Options::DEDUPLICATE_LISTS);

        assert_eq!(
            run_with_options("a{font-family:\"A\",Arial,a,sans-serif,Helvetica}", options),
            "a{font-family:A,Arial,a,sans-serif}"
        );

        let mut options = MinifyOptions::default();
        options.flags.remove(Options::NORMALIZE_VALUES);
        assert_eq!(
            run_with_options("a{font-family:A,A,serif,Helvetica}", options),
            "a{font-family:A,serif,Helvetica}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_font_family_declarations_with_unparsed_values() {
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
                .all(|declaration| matches!(declaration, Declaration::FontFamily(_)))
        );
        assert_eq!(stats.declarations_removed, 0);
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            "a{font-family:var(--family);font-family:slab inherit}"
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
    fn preserves_blue_identifiers_in_untyped_values() {
        assert_eq!(
            run("a{--theme:blue;unknown:blue;background:blue;color:blue}"),
            "a{--theme:blue;unknown:blue;background:#00f;color:#00f}"
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
    #[ignore]
    fn preserves_three_dimensional_zero_translation() {
        assert_eq!(
            run("a{transform:translate3d(0,0,0)}"),
            "a{transform:translateZ(0)}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_partial_animation_shorthand_with_infinite_iteration_count() {
        assert_eq!(run(".foo{animation:infinite}"), ".foo{animation:infinite}");
    }

    #[test]
    #[ignore]
    fn preserves_icss_export_syntax_without_module_semantics() {
        assert_eq!(run(":export{rowCount:4}"), ":export{rowCount:4}");
    }

    #[test]
    #[ignore]
    fn preserves_individual_transform_properties_as_independent_declarations() {
        assert_eq!(
            run(".foo{scale:1.5;translate:1rem;transform:skew(-25deg)}"),
            ".foo{scale:1.5;translate:1rem;transform:skew(-25deg)}"
        );
        assert_eq!(
            run(".bar{transform:skew(-25deg);scale:1.5;translate:1rem}"),
            ".bar{transform:skew(-25deg);scale:1.5;translate:1rem}"
        );
        assert_eq!(
            run(
                ".bar{transition:scale .3s linear;transform:skew(10deg);scale:1.5;translate:1em}.bar:hover{scale:2}"
            ),
            ".bar{transition:scale .3s linear;transform:skew(10deg);scale:1.5;translate:1em}.bar:hover{scale:2}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_authored_image_set_fallbacks_without_generating_duplicates() {
        assert_eq!(
            run("a{background-image:image-set(\"a.png\" 1x)}"),
            "a{background-image:image-set(\"a.png\" 1x)}"
        );
        assert_eq!(
            run(
                "a{background-image:-webkit-image-set(\"a.png\" 1x);background-image:image-set(\"a.png\" 1x)}"
            ),
            "a{background-image:-webkit-image-set(\"a.png\" 1x);background-image:image-set(\"a.png\" 1x)}"
        );
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
    fn preserves_rule_boundaries_while_merging_adjacent_styles() {
        assert_eq!(
            run("a{}a{color:red}a{color:red} @media print{a{}}"),
            "a{color:red}@media print{a{}}"
        );
        assert_eq!(
            run("@charset 'UTF-8'; @import 'theme.css'; a{color:red}"),
            "@charset \"UTF-8\";@import \"theme.css\";a{color:red}"
        );
    }

    #[test]
    #[ignore]
    fn accepts_and_minifies_native_nested_rules_without_a_feature_flag() {
        assert_eq!(
            run("h1.b{color:red}h1{.b{color:red}}"),
            "h1.b{color:red}h1{.b{color:red}}"
        );
        assert_eq!(
            run(".top{sub{--prop:value}& .sub{--prop:value}}"),
            ".top{sub{--prop:value}& .sub{--prop:value}}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_unknown_units_in_media_queries() {
        assert_eq!(
            run("@media screen and (max-width:_1000customPx_){.test{color:red}}"),
            "@media screen and (max-width:_1000customPx_){.test{color:red}}"
        );
        assert_eq!(
            run("@media (max-width:1000customPx){.test{color:red}}"),
            "@media (max-width:1000customPx){.test{color:red}}"
        );
        assert_eq!(
            run("@media screen and (min-width:1020 px) and (max-width:739 px){.foo{color:red}}"),
            "@media screen and (min-width:1020 px) and (max-width:739 px){.foo{color:red}}"
        );
        assert_eq!(
            run(
                "@media (min-width:740px) and (max-width:1019px) and (min-width:1020px) and (max-width:1135px){.foo{color:red}}"
            ),
            "@media (width>=740px) and (width<=1019px) and (width>=765pt) and (width<=1135px){.foo{color:red}}"
        );
    }

    #[test]
    fn deduplicates_selectors_with_structural_hashes() {
        assert_eq!(run("h1,h2,h1,h2{color:red}"), "h1,h2{color:red}");
        assert_eq!(
            run(".foo,.bar:baz{color:green}"),
            ".foo,.bar:baz{color:green}"
        );
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
        assert_eq!(run("a{&,&{color:red}}"), "a{&{color:red}}");
    }
    #[test]
    #[ignore]
    fn preserves_rules_with_long_child_selectors() {
        assert_eq!(
            run(".depict.plp .filters .body .input-row > .left{align-items:center;display:flex}"),
            ".depict.plp .filters .body .input-row>.left{align-items:center;display:flex}"
        );
    }

    #[test]
    #[ignore]
    fn avoids_expanding_transition_shorthand_and_property() {
        assert_eq!(
            run(
                ".foo{transition:all .2s cubic-bezier(.4,0,.2,1);transition-property:height,width,transform,max-width,left,right,top,bottom,box-shadow}"
            ),
            ".foo{transition:all .2s cubic-bezier(.4,0,.2,1);transition-property:height,width,transform,max-width,left,right,top,bottom,box-shadow}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_vendor_prefixes_in_supports_conditions() {
        assert_eq!(
            run(
                "@supports ((display:-webkit-box) and (-webkit-box-orient:vertical) and (-webkit-line-clamp:3)){.foo{display:-webkit-box;-webkit-box-orient:vertical;-webkit-line-clamp:3}}"
            ),
            "@supports ((display:-webkit-box) and (-webkit-box-orient:vertical) and (-webkit-line-clamp:3)){.foo{display:-webkit-box;-webkit-box-orient:vertical;-webkit-line-clamp:3}}"
        );
        assert_eq!(
            run(
                "@supports (color:color(display-p3 0 0 0)){:root,:host{--theme:color(display-p3 1 0 0)}}"
            ),
            "@supports (color:color(display-p3 0 0 0)){:root,:host{--theme:color(display-p3 1 0 0)}}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_variables_in_the_all_property() {
        assert_eq!(
            run(".boop{margin:1px;all:var(--all,revert-layer);margin-left:2px}"),
            ".boop{margin:1px;all:var(--all,revert-layer);margin-left:2px}"
        );
    }

    #[test]
    #[ignore = "browser targets are not implemented yet"]
    fn emits_safari_14_safe_zero_media_lengths() {
        assert_eq!(
            run("@media (min-width:0){a{color:red}}"),
            "@media (min-width:0px){a{color:red}}"
        );
    }

    #[test]
    #[ignore = "provably false media query elimination is not implemented"]
    fn removes_provably_false_media_queries() {
        assert_eq!(
            run(
                "@media (min-width:740px) and (max-width:1019px) and (min-width:1020px) and (max-width:1135px){.foo{color:red}}"
            ),
            ""
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
                ".aligncenter{clear:both;clear:both;clip:auto;clip:auto;margin-left:auto;margin-left:auto;margin-right:auto;margin-right:auto;display:block;display:block}"
            ),
            ".aligncenter{clear:both;clip:auto;margin-left:auto;margin-right:auto;display:block}"
        );
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
    #[ignore]
    fn preserves_prefixed_and_standard_backdrop_filter_fallbacks() {
        assert_eq!(
            run(".a{backdrop-filter:blur(10px);-webkit-backdrop-filter:blur(10px)}"),
            ".a{backdrop-filter:blur(10px);-webkit-backdrop-filter:blur(10px)}"
        );
        assert_eq!(
            run(".b{-webkit-backdrop-filter:blur(10px);backdrop-filter:blur(10px)}"),
            ".b{-webkit-backdrop-filter:blur(10px);backdrop-filter:blur(10px)}"
        );
        assert_eq!(
            run("a{-webkit-text-size-adjust:none;text-size-adjust:none}"),
            "a{-webkit-text-size-adjust:none;text-size-adjust:none}"
        );
        assert_eq!(
            run("b{text-size-adjust:none;-webkit-text-size-adjust:none}"),
            "b{text-size-adjust:none;-webkit-text-size-adjust:none}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_authored_text_decoration_prefixes_without_duplication() {
        assert_eq!(
            run("a{color:inherit;-webkit-text-decoration:inherit;text-decoration:inherit}"),
            "a{color:inherit;-webkit-text-decoration:inherit;text-decoration:inherit}"
        );
        assert_eq!(
            run("a{text-decoration:inherit;-webkit-text-decoration:inherit}"),
            "a{text-decoration:inherit;-webkit-text-decoration:inherit}"
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
        assert_eq!(
            run(
                ".foo{color:red;color:var(--my-red);background-color:blue;background-color:var(--my-blue)}"
            ),
            ".foo{color:red;color:var(--my-red);background-color:#00f;background-color:var(--my-blue)}"
        );
        assert_eq!(
            run(".test{font-size:36px;color:green}.test{color:var(--foo,revert-rule)}"),
            ".test{font-size:36px;color:green}.test{color:var(--foo,revert-rule)}"
        );
        assert_eq!(
            run("a{width:-webkit-fill-available;width:-moz-available;width:stretch}"),
            "a{width:-webkit-fill-available;width:-moz-available;width:stretch}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_light_dark_and_color_scheme_without_lowering() {
        assert_eq!(
            run(
                ":root{--background:light-dark(white,black)}p{background:var(--background);color-scheme:dark}"
            ),
            ":root{--background:light-dark(#fff,#000)}p{background:var(--background);color-scheme:dark}"
        );
        assert_eq!(
            run(
                "a{border-bottom:1px solid light-dark(var(--light),var(--dark));border-color:light-dark(white,black)}"
            ),
            "a{border-bottom:1px solid light-dark(var(--light),var(--dark));border-color:light-dark(#fff,#000)}"
        );
        assert_eq!(
            run(
                ".dark{color-scheme:only dark}.light{color-scheme:only light}.alt{color-scheme:dark only}"
            ),
            ".dark{color-scheme:only dark}.light{color-scheme:only light}.alt{color-scheme:dark only}"
        );
        assert_eq!(
            run(":host{color-scheme:inherit}a{color-scheme:normal}"),
            ":host{color-scheme:inherit}a{color-scheme:normal}"
        );
    }

    #[test]
    #[ignore]
    fn minifies_color_keywords_in_variable_fallbacks_by_property_context() {
        assert_eq!(
            run("#foo{color:white}#bar{color:var(--c, white)}"),
            "#foo{color:#fff}#bar{color:var(--c,#fff)}"
        );
        assert_eq!(
            run("a{font-family:var(--family,white)}"),
            "a{font-family:var(--family,white)}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_cascade_sensitive_declaration_order() {
        assert_eq!(
            run(".item{animation:fade both;animation-timeline:scroll(root block)}"),
            ".item{animation:fade both;animation-timeline:scroll(root block)}"
        );
        assert_eq!(
            run(
                ".header{height:1px;height:var(--header-height);block-size:auto;block-size:calc-size(auto)}"
            ),
            ".header{height:1px;height:var(--header-height);block-size:auto;block-size:calc-size(auto)}"
        );
        assert_eq!(
            run(
                ".foo{animation:linear foo;animation-timeline:view();animation-range:entry-crossing 1% exit-crossing 100%}"
            ),
            ".foo{animation:foo linear;animation-timeline:view();animation-range:entry-crossing 1% exit-crossing 100%}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_authored_legacy_grid_fallbacks_without_targets() {
        assert_eq!(
            run(
                ".grid{display:-ms-grid;display:grid;grid-auto-columns:1fr;grid-column-gap:16px;grid-row-gap:16px;-ms-grid-columns:1fr 1fr 1fr;grid-template-columns:1fr 1fr 1fr;-ms-grid-rows:auto;grid-template-rows:auto}"
            ),
            ".grid{display:-ms-grid;display:grid;grid-auto-columns:1fr;grid-column-gap:1pc;grid-row-gap:1pc;-ms-grid-columns:1fr 1fr 1fr;grid-template-columns:1fr 1fr 1fr;-ms-grid-rows:auto;grid-template-rows:auto}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_new_viewport_units_instead_of_approximating_them() {
        assert_eq!(
            run(
                "a{height:100dvh;min-height:100svh;max-height:100lvh;width:100dvw;min-width:100svw;max-width:100lvw}"
            ),
            "a{height:100dvh;min-height:100svh;max-height:100lvh;width:100dvw;min-width:100svw;max-width:100lvw}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_logical_overflow_alongside_physical_fallbacks() {
        assert_eq!(
            run("a{overflow-inline:auto;overflow-block:scroll;overflow-x:hidden}"),
            "a{overflow-inline:auto;overflow-block:scroll;overflow-x:hidden}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_pseudo_elements_inside_where_instead_of_emptying_it() {
        assert_eq!(
            run(".language-diff :where(.inserted::before){content:'+'}"),
            ".language-diff :where(.inserted:before){content:\"+\"}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_oklch_variables_when_fallback_generation_is_unavailable() {
        assert_eq!(
            run(
                ".text-red-200{--tw-text-opacity:1;color:oklch(92.19% .04 20/var(--tw-text-opacity))}"
            ),
            ".text-red-200{--tw-text-opacity:1;color:oklch(92.19% .04 20/var(--tw-text-opacity))}"
        );
        assert_eq!(
            run("a{color:oklch(var(--channels)/var(--alpha))}"),
            "a{color:oklch(var(--channels)/var(--alpha))}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_powerless_color_channels_inside_color_mix() {
        assert_eq!(
            run("a{background:color-mix(in hsl,var(--primary) 40%,hsl(193 100% 100%) 60%)}"),
            "a{background:color-mix(in hsl,var(--primary) 40%,hsl(193 100% 100%) 60%)}"
        );
    }

    #[test]
    #[ignore]
    fn disabling_value_normalization_preserves_authored_color_functions() {
        let mut options = MinifyOptions::default();
        options.flags.remove(Options::NORMALIZE_VALUES);
        assert_eq!(
            run_with_options(
                "a{color:rgb(255 255 255);background:hsl(40 50% 50%)}",
                options
            ),
            "a{color:rgb(255 255 255);background:hsl(40 50% 50%)}"
        );
    }

    #[test]
    #[ignore]
    fn canonicalizes_zero_legacy_media_features_to_safari_safe_ranges() {
        assert_eq!(
            run("@media (min-width:0){a{color:red}}"),
            "@media (width>=0){a{color:red}}"
        );
    }

    #[test]
    fn merges_adjacent_equal_selector_declaration_blocks() {
        assert_eq!(
            run("h1{color:red;background:blue}h1{color:red}"),
            "h1{background:#00f;color:red}"
        );
        assert_eq!(
            run("a{width:1px}a{height:2px}a{opacity:.5}"),
            "a{width:1px;height:2px;opacity:.5}"
        );
    }

    #[test]
    fn runs_box_ir_across_adjacent_blocks() {
        assert_eq!(
            run("a{margin-top:1px;margin-right:2px}a{margin-bottom:3px;margin-left:4px}"),
            "a{margin:1px 2px 3px 4px}"
        );
        assert_eq!(
            run("a{padding:1px}a{padding-left:2px}"),
            "a{padding:1px 1px 1px 2px}"
        );
    }

    #[test]
    fn preserves_cross_block_fallbacks_and_importance() {
        assert_eq!(
            run("a{width:1px}a{width:2px}a{width:1px}"),
            "a{width:1px;width:2px;width:1px}"
        );
        assert_eq!(
            run("a{color:red!important}a{color:blue}"),
            "a{color:red !important;color:#00f}"
        );
    }

    #[test]
    fn respects_nested_content_as_a_forward_merge_barrier() {
        assert_eq!(
            run(".a{color:red;& .child{display:block}}.a{color:blue}"),
            ".a{color:red;& .child{display:block}}.a{color:#00f}"
        );
        assert_eq!(
            run(".a{color:red}.a{color:blue;& .child{display:block}}"),
            ".a{color:red;color:#00f;& .child{display:block}}"
        );
        assert_eq!(
            run(".a{color:red;& .child{display:block};color:green}.a{color:blue}"),
            ".a{color:red;& .child{display:block}color:green}.a{color:#00f}"
        );
    }

    #[test]
    fn merges_only_inside_the_current_sibling_scope() {
        assert_eq!(
            run("a{color:red}b{display:block}a{color:blue}"),
            "a{color:red}b{display:block}a{color:#00f}"
        );
        assert_eq!(
            run("@media print{a{color:red}a{background:blue}}"),
            "@media print{a{color:red;background:#00f}}"
        );
    }

    #[test]
    fn adjacent_rule_merging_is_configurable() {
        let mut options = MinifyOptions::default();
        options.flags.remove(Options::MERGE_ADJACENT_RULES);

        assert_eq!(
            run_with_options("a{color:red}a{background:blue}", options),
            "a{color:red}a{background:#00f}"
        );
    }

    #[test]
    fn adjacent_rule_merging_is_idempotent() {
        let allocator = Allocator::new();
        let mut stylesheet = parse(
            "a{width:1px}a{height:2px}a{width:1px}",
            &allocator,
            ParserOptions::default(),
        )
        .unwrap();

        minify(&mut stylesheet, MinifyOptions::default());
        let once = stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap();
        let second_stats = minify(&mut stylesheet, MinifyOptions::default());
        let twice = stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap();

        assert_eq!(once, "a{height:2px;width:1px}");
        assert_eq!(twice, once);
        assert_eq!(second_stats.declarations_removed, 0);
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
    #[ignore]
    fn folds_static_calc_in_custom_properties_but_preserves_dynamic_terms() {
        assert_eq!(
            run(":root{--static:calc(10px + 20px);--dynamic:calc(10px + var(--bar))}"),
            ":root{--static:30px;--dynamic:calc(10px + var(--bar))}"
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
    #[ignore]
    fn minifies_supported_colors_in_custom_properties() {
        assert_eq!(
            run("a{--white:white;--hex:#FFFFFF;--dynamic:var(--color)}"),
            "a{--white:#fff;--hex:#fff;--dynamic:var(--color)}"
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
    #[test]
    #[ignore]
    fn minifies_nested_calc_groups_without_panicking_or_losing_units() {
        let output = run("a{height:calc((100dvh - 10.5rem) + (4vh + 230px))}");
        for unit in ["100dvh", "10.5rem", "4vh", "230px"] {
            assert!(output.contains(unit), "missing {unit} in {output}");
        }
        assert_eq!(run(&output), output);
    }

    #[test]
    #[ignore]
    fn preserves_dynamic_logical_values_and_user_select_fallbacks() {
        assert_eq!(
            run("a{margin-inline:var(--m);margin-inline:calc(var(--gap) + 1px)}"),
            "a{margin-inline:var(--m);margin-inline:calc(var(--gap) + 1px)}"
        );
        assert_eq!(
            run("a{-webkit-user-select:auto;user-select:all}"),
            "a{-webkit-user-select:auto;user-select:all}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_prefixed_mask_image_variable_fallbacks_without_duplication() {
        const SOURCE: &str = ".foo{-webkit-mask-image:url(./foo.svg);mask-image:url(./foo.svg)}.bar{-webkit-mask-image:var(--foo);mask-image:var(--foo)}.fallback{-webkit-mask-image:var(--foo,url(./fallback.svg));mask-image:var(--foo,url(./fallback.svg))}";
        assert_eq!(run(SOURCE), SOURCE);
    }

    #[test]
    #[ignore]
    fn preserves_nested_layer_statement_and_block_order() {
        const SOURCE: &str = "@layer one,one.a,one.b;@layer one{@layer b{.test1{color:red}}}@layer one.a{.test1{color:green}}";
        assert_eq!(run(SOURCE), SOURCE);
    }

    #[test]
    #[ignore]
    fn preserves_whitespace_between_variables_and_adjacent_values() {
        assert_eq!(
            run("a{margin:var(--x) var(--y);padding:var(--x) 0}"),
            "a{margin:var(--x) var(--y);padding:var(--x) 0}"
        );
    }

    #[test]
    #[ignore]
    fn preserves_distinct_vendor_values_and_negated_supports_conditions() {
        const SOURCE: &str = "a{-webkit-appearance:none;appearance:textfield}b{appearance:textfield;-webkit-appearance:none}@supports not (backdrop-filter:none){c{-webkit-backdrop-filter:none;backdrop-filter:none}}";
        assert_eq!(run(SOURCE), SOURCE);
    }

    #[test]
    #[ignore = "cross-rule selector merging and selector-support proofs are not implemented"]
    fn does_not_merge_adjacent_rules_through_forgiving_selector_wrappers() {
        const SOURCE: &str = "a{color:blue}:unknown{color:blue}";
        assert_eq!(run(SOURCE), SOURCE);
        assert!(!run(SOURCE).contains(":is("));
        assert!(!run(SOURCE).contains(":where("));
    }

    #[test]
    #[ignore]
    fn preserves_scroll_driven_animation_duration_auto_semantics() {
        const SOURCE: &str = ".overflowContainer{animation:--keyframes-top-scroll-border step-end,--keyframes-bottom-scroll-border step-end reverse;animation-timeline:scroll(self)}";
        let output = run(SOURCE);
        assert!(output.contains("animation:--keyframes-top-scroll-border step-end,--keyframes-bottom-scroll-border step-end reverse"));
        assert!(output.contains("animation-timeline:scroll(self)"));
        assert!(!output.contains("animation-duration"));
    }

    #[test]
    #[ignore]
    fn preserves_has_slotted_pseudo_class_through_minification() {
        assert_eq!(
            run("slot:has-slotted{display:none}"),
            "slot:has-slotted{display:none}"
        );
    }
}
