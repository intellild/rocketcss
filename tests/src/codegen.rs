use std::path::PathBuf;

use rs_css_allocator::Allocator;
use rs_css_codegen::{PrinterOptions, ToCss};
use rs_css_parser::{ParserOptions, parse};
use rstest::rstest;

use crate::{expected_path, read_fixture};

#[rstest]
fn prints_expected_css(
    #[base_dir = "fixtures"]
    #[files("codegen/**/input.css")]
    input: PathBuf,
) {
    let source = read_fixture(&input);
    let expected = read_fixture(&expected_path(&input));
    let allocator = Allocator::new();
    let stylesheet = parse(&source, &allocator, ParserOptions::default())
        .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));

    let actual = stylesheet
        .to_css_string(PrinterOptions::default())
        .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

    assert_eq!(actual, expected, "fixture: {}", input.display());
}
