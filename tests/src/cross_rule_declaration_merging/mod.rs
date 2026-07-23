use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_nano::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};

mod declarations;
mod nesting;
mod real_world;
mod review_findings;
mod state_machine;

fn run(source: &str) -> String {
    let allocator = Allocator::new();
    let mut stylesheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    minify(&mut stylesheet, MinifyOptions::default());
    stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap()
}

fn assert_minifies_idempotently(source: &str, expected: &str) {
    let once = run(source);
    assert_eq!(once, expected);
    assert_eq!(run(&once), once);
}

fn assert_minifies_same_ast_twice(source: &str, expected: &str) {
    let allocator = Allocator::new();
    let mut stylesheet = parse(source, &allocator, ParserOptions::default()).unwrap();

    minify(&mut stylesheet, MinifyOptions::default());
    let once = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();

    minify(&mut stylesheet, MinifyOptions::default());
    let twice = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();

    assert_eq!(once, expected);
    assert_eq!(twice, once);
}
