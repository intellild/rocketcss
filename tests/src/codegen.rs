use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCssWithGhost};
use rocketcss_parser::{ParserOptions, parse};

use crate::{expected_path, fixture_paths, read_fixture};

#[test]
fn prints_expected_css() {
    for input in fixture_paths("codegen") {
        let source = read_fixture(&input);
        let expected = read_fixture(&expected_path(&input));
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(&source, &allocator, &mut token, ParserOptions::default())
                .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));

            let actual = stylesheet
                .to_css_string(&token, PrinterOptions::default())
                .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

            assert_eq!(actual, expected, "fixture: {}", input.display());
        });
    }
}

#[test]
#[ignore]
fn preserves_leading_license_comments_in_all_output_modes() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let stylesheet = parse(
            "/*! first */ /*! second */ /* ordinary */ a { color: red; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .expect("stylesheet should parse");

        let pretty = stylesheet
            .to_css_string(&token, PrinterOptions { prettify: true })
            .expect("stylesheet should print in pretty mode");
        let compact = stylesheet
            .to_css_string(&token, PrinterOptions { prettify: false })
            .expect("stylesheet should print in compact mode");

        assert!(pretty.starts_with("/*! first */\n/*! second */\n"));
        assert!(compact.starts_with("/*! first *//*! second */"));
        assert!(!pretty.contains("ordinary"));
        assert!(!compact.contains("ordinary"));
    });
}
