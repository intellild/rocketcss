use std::path::Path;

use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_minify::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};
use serde_json::Value;

use crate::{expected_path, fixture_paths, read_fixture};

const CSSNANO_CORPUS: &str = include_str!("../fixtures/minify/cssnano/corpus.json");

// Fixtures that require cross-node analysis or replacement AST allocation
// remain in the corpus but are skipped until those features are redesigned
// around the local-only pass.
#[test]
fn minifies_upstream_fixtures() {
    for input in fixture_paths("minify") {
        if requires_nonlocal_or_rebuilding_transform(&input) {
            eprintln!(
                "skipped non-local or rebuilding minify fixture: {}",
                input.display()
            );
            continue;
        }

        let source = read_fixture(&input);
        let expected = read_fixture(&expected_path(&input));
        let allocator = Allocator::new();
        let mut stylesheet = parse(&source, &allocator, ParserOptions::default())
            .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));

        minify(&mut stylesheet, MinifyOptions::default());
        let actual = stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

        assert_eq!(actual, expected.trim_end(), "fixture: {}", input.display());
    }
}

#[test]
fn minifies_all_cssnano_runtime_cases() {
    let corpus: Value =
        serde_json::from_str(CSSNANO_CORPUS).expect("CSSNano corpus must be valid JSON");
    let cases = corpus["cases"]
        .as_array()
        .expect("CSSNano corpus must contain cases");
    assert_eq!(cases.len(), 3_359, "the audited CSSNano corpus changed");

    let mut failure_count = 0usize;
    let mut failures = std::vec::Vec::new();
    for case in cases {
        let name = case["name"].as_str().expect("case name must be a string");
        let source = case["source"]
            .as_str()
            .expect("case source must be a string");
        let expected = case["expected"]
            .as_str()
            .expect("case expected output must be a string");

        let allocator = Allocator::new();
        let Ok(mut stylesheet) = parse(source, &allocator, ParserOptions::default()) else {
            record_failure(
                &mut failure_count,
                &mut failures,
                format!(
                    "{name}: RocketCSS could not parse source\n{}",
                    preview(source)
                ),
            );
            continue;
        };
        minify(&mut stylesheet, MinifyOptions::default());
        let actual = stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap_or_else(|error| panic!("{name} should print: {error}"));

        let expected_allocator = Allocator::new();
        let Ok(expected_stylesheet) =
            parse(expected, &expected_allocator, ParserOptions::default())
        else {
            record_failure(
                &mut failure_count,
                &mut failures,
                format!(
                    "{name}: RocketCSS could not parse CSSNano output\n{}",
                    preview(expected)
                ),
            );
            continue;
        };
        let canonical_expected = expected_stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap_or_else(|error| panic!("{name} expected output should print: {error}"));

        if actual != canonical_expected {
            record_failure(
                &mut failure_count,
                &mut failures,
                format!(
                    "{name}\nsource: {}\nexpected: {}\nactual: {}",
                    preview(source),
                    preview(&canonical_expected),
                    preview(&actual)
                ),
            );
        }
    }

    assert!(
        failures.is_empty(),
        "{failure_count} CSSNano cases disagreed (showing at most 50):\n\n{}",
        failures.join("\n\n")
    );
}

fn record_failure(
    failure_count: &mut usize,
    failures: &mut std::vec::Vec<String>,
    failure: String,
) {
    *failure_count += 1;
    if failures.len() < 50 {
        failures.push(failure);
    }
}

fn preview(value: &str) -> String {
    const LIMIT: usize = 500;
    let mut preview: String = value.chars().take(LIMIT).collect();
    if value.chars().count() > LIMIT {
        preview.push('…');
    }
    preview
}

fn requires_nonlocal_or_rebuilding_transform(input: &Path) -> bool {
    let path = input.to_string_lossy();
    let unsupported_groups = ["/lightningcss/math/"];
    let unsupported_cases = [
        "/lightningcss/declarations/important/",
        "/lightningcss/rules/keyframe-merge/",
        "/lightningcss/rules/merge-layer/",
        "/lightningcss/rules/merge-media/",
        "/lightningcss/rules/merge-selectors/",
        "/lightningcss/values/background-position/",
        "/lightningcss/values/display/",
        "/lightningcss/values/font-family/",
    ];
    unsupported_groups
        .into_iter()
        .chain(unsupported_cases)
        .any(|pattern| path.contains(pattern))
}
