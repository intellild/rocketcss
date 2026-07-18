pub use rocketcss_allocator::Allocator;
pub use rocketcss_codegen::{PrinterOptions, ToCss};
pub use rocketcss_parser::{ParserOptions, parse};
pub use rocketcss_visitor::{PluginContext, Plugins};

pub use super::*;

mod animations;
mod at_rules;
mod box_model;
mod calc_units;
mod colors;
mod custom_properties;
mod declarations;
mod fonts;
mod options_plugin;
mod prefixes;
mod rule_merge;
mod selectors;
mod transforms;
mod values;

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
