pub use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
pub use rocketcss_common::Allocator;
pub use rocketcss_parser::{ParserOptions, parse};
pub use rocketcss_visitor::{PluginContext, Plugins};

pub use super::*;

mod animations;
mod at_rules;
mod box_model;
mod calc_units;
mod colors;
mod columns;
mod custom_properties;
mod declarations;
mod fonts;
mod options_plugin;
mod prefixes;
mod radix_pipeline;
mod rule_merge;
mod selectors;
mod transforms;
mod values;

fn run(source: &str) -> String {
    run_with_options(source, MinifyOptions::default())
}

fn run_with_options(source: &str, options: MinifyOptions) -> String {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet =
            parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        minify(&mut stylesheet, &mut token, options);
        stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap()
    })
}

fn run_with_error_recovery(source: &str) -> String {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            source,
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        minify(&mut stylesheet, &mut token, MinifyOptions::default());
        stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap()
    })
}

fn first_rule_id<'ast>(stylesheet: &StyleSheet<'ast>) -> CssRuleId<'ast> {
    stylesheet
        .rules_in_list(stylesheet.stylesheet_root().root_rules())
        .expect("the root rule list remains valid")
        .map(|(rule, _)| rule)
        .next()
        .expect("expected at least one rule")
}

fn first_declaration_block_id<'ast>(stylesheet: &StyleSheet<'ast>) -> CssDeclarationBlockId<'ast> {
    stylesheet
        .rule(first_rule_id(stylesheet))
        .and_then(|rule| rule.declaration_block())
        .expect("expected the first rule to own declarations")
}

fn first_property_declaration<'tree, 'ast>(
    stylesheet: &'tree StyleSheet<'ast>,
) -> &'tree Declaration<'ast> {
    stylesheet
        .declarations_in_block(first_declaration_block_id(stylesheet))
        .expect("the first declaration block remains valid")
        .find_map(|declaration| match declaration.payload() {
            CssDeclaration::Property(declaration) => Some(declaration),
            _ => None,
        })
        .expect("expected a property declaration")
}
