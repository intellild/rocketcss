use std::path::PathBuf;

use rs_css_allocator::Allocator;
use rs_css_codegen::{PrinterOptions, ToCss};
use rs_css_minify::{MinifyOptions, minify};
use rs_css_parser::{ParserOptions, parse};
use rstest::rstest;

use crate::{expected_path, read_fixture};

#[rstest]
// The glob expands every upstream input/output pair into an independent case.
// Keep this source as the single runner when adding new upstream fixtures and
// their original input/output semantics; changing this comment also makes
// Cargo re-expand the compile-time fixture glob after new files are added. The
// fixture path itself records the upstream package or Lightning CSS area, so
// failures identify both the pass and its source suite. All cases run through
// the same parser/minifier/code-generator pipeline, including math and font
// value normalization.
fn minifies_upstream_fixture(
    #[base_dir = "fixtures"]
    #[files("minify/**/input.css")]
    input: PathBuf,
) {
    let source = read_fixture(&input);
    let expected = read_fixture(&expected_path(&input));
    let allocator = Allocator::new();
    let mut stylesheet = parse(&source, &allocator, ParserOptions::default())
        .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));

    minify(&mut stylesheet, &allocator, MinifyOptions::default());
    let actual = stylesheet
        .to_css_string(PrinterOptions { minify: true })
        .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

    assert_eq!(actual, expected.trim_end(), "fixture: {}", input.display());
}
