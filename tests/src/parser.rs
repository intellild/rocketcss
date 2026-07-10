use std::path::PathBuf;

use rs_css_allocator::Allocator;
use rs_css_parser::{ParserOptions, parse};
use rstest::rstest;

use crate::read_fixture;

#[rstest]
fn parses_valid_fixtures(
    #[base_dir = "fixtures"]
    #[files("parser/pass/**/input.css")]
    path: PathBuf,
) {
    let source = read_fixture(&path);
    let allocator = Allocator::new();

    parse(&source, &allocator, ParserOptions::default())
        .unwrap_or_else(|error| panic!("{} should parse successfully: {error:?}", path.display()));
}

#[rstest]
fn rejects_invalid_fixtures(
    #[base_dir = "fixtures"]
    #[files("parser/fail/**/input.css")]
    path: PathBuf,
) {
    let source = read_fixture(&path);
    let allocator = Allocator::new();

    assert!(
        parse(&source, &allocator, ParserOptions::default()).is_err(),
        "{} should fail to parse",
        path.display()
    );
}
