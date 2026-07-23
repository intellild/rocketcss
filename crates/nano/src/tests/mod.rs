pub use rocketcss_allocator::Allocator;
pub use rocketcss_codegen::{PrinterOptions, ToCss, ToCssWithGhost};
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
            .to_css_string(&token, PrinterOptions { prettify: false })
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
            .to_css_string(&token, PrinterOptions { prettify: false })
            .unwrap()
    })
}
