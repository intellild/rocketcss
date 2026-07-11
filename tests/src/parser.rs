use rocketcss_allocator::Allocator;
use rocketcss_parser::{ParserOptions, parse};

use crate::{fixture_paths, read_fixture};

#[test]
fn parses_valid_fixtures() {
    for path in fixture_paths("parser/pass") {
        let source = read_fixture(&path);
        let allocator = Allocator::new();

        parse(&source, &allocator, ParserOptions::default()).unwrap_or_else(|error| {
            panic!("{} should parse successfully: {error:?}", path.display())
        });
    }
}

#[test]
fn rejects_invalid_fixtures() {
    for path in fixture_paths("parser/fail") {
        let source = read_fixture(&path);
        let allocator = Allocator::new();

        assert!(
            parse(&source, &allocator, ParserOptions::default()).is_err(),
            "{} should fail to parse",
            path.display()
        );
    }
}
