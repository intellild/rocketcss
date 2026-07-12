use std::{borrow::Cow, collections::BTreeMap, path::Path};

use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_minify::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};
use serde_json::Value;

use crate::{expected_path, fixture_paths, read_fixture};

const CSSNANO_CORPUS: &str = include_str!("../fixtures/minify/cssnano/corpus.json");
const CSSNANO_PARSER_SKIPS: &[&str] = &[
    "packages/cssnano-preset-advanced/test/css-declaration-sorter.js:100/18",
    "packages/cssnano-preset-advanced/test/css-declaration-sorter.js:108/19",
    "packages/postcss-discard-empty/test/index.js:48/532",
    "packages/postcss-discard-empty/test/index.js:66/538",
    "packages/postcss-discard-empty/test/index.js:70/539",
    "packages/postcss-discard-comments/test/index.js:259/484",
    "packages/postcss-discard-comments/test/index.js:274/486",
    "packages/postcss-discard-comments/test/index.js:281/487",
    "packages/postcss-normalize-unicode/test/index.js:87/2159",
    "packages/postcss-normalize-unicode/test/index.js:92/2160",
    "packages/postcss-normalize-unicode/test/index.js:95/2161",
    "packages/postcss-minify-params/test/index.js:292/1350",
    "packages/postcss-minify-params/test/index.js:297/1351",
    "packages/postcss-minify-selectors/test/index.js:350/1411",
    "packages/postcss-minify-selectors/test/index.js:543/1433",
    "packages/postcss-minify-selectors/test/index.js:905/1475",
    "packages/postcss-unique-selectors/test/index.js:45/3122",
];

const CSSNANO_MINIFY_SKIPS: &[&str] = &[
    "packages/postcss-discard-comments/test/index.js:123/463",
    "packages/postcss-discard-comments/test/index.js:128/464",
    "packages/postcss-discard-comments/test/index.js:138/466",
    "packages/postcss-discard-comments/test/index.js:299/489",
    "packages/postcss-discard-comments/test/index.js:304/490",
    "packages/postcss-discard-comments/test/index.js:329/495",
    "packages/postcss-discard-comments/test/index.js:337/496",
    "packages/postcss-discard-comments/test/index.js:374/501",
    "packages/postcss-minify-font-values/test/index.js:80/1174",
    "packages/postcss-minify-font-values/test/index.js:88/1175",
    "packages/postcss-minify-font-values/test/index.js:146/1185",
    "packages/postcss-minify-font-values/test/index.js:272/1203",
    "packages/postcss-minify-font-values/test/index.js:282/1205",
    "packages/postcss-minify-font-values/test/index.js:287/1206",
    "packages/postcss-minify-params/test/index.js:23/1310",
    "packages/postcss-minify-params/test/index.js:148/1326",
    "packages/postcss-minify-params/test/index.js:237/1339",
    "packages/postcss-minify-params/test/index.js:245/1340",
    "packages/postcss-minify-params/test/index.js:258/1342",
    "packages/postcss-minify-params/test/index.js:263/1343",
    "packages/postcss-minify-selectors/test/index.js:247/1394",
    "packages/postcss-ordered-values/test/index.js:250/2268",
    "packages/postcss-ordered-values/test/index.js:255/2269",
    "packages/postcss-ordered-values/test/index.js:260/2270",
    "packages/postcss-ordered-values/test/index.js:265/2271",
    "packages/postcss-ordered-values/test/index.js:270/2272",
    "packages/postcss-ordered-values/test/index.js:514/2309",
];

const CSSNANO_UPSTREAM_SKIPS: &[&str] = &["packages/postcss-minify-params/test/index.js:47/1313"];

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
    assert_eq!(cases.len(), 3_247, "the audited CSSNano corpus changed");

    let plugin_filter = std::env::var("ROCKETCSS_CSSNANO_PLUGIN").ok();
    let case_offset = corpus_position("ROCKETCSS_CSSNANO_OFFSET", 0);
    let case_limit = corpus_position("ROCKETCSS_CSSNANO_LIMIT", usize::MAX);
    let mut failure_count = 0usize;
    let mut executed = 0usize;
    let mut failures = std::vec::Vec::new();
    let mut failures_by_plugin = BTreeMap::new();
    let selected_cases = cases
        .iter()
        .filter(|case| {
            plugin_filter.as_ref().is_none_or(|filter| {
                case["plugin"]
                    .as_str()
                    .is_some_and(|plugin| plugin == filter)
            })
        })
        .skip(case_offset)
        .take(case_limit);
    for case in selected_cases {
        let name = case["name"].as_str().expect("case name must be a string");
        if CSSNANO_PARSER_SKIPS.contains(&name) {
            eprintln!("skipped CSSNano case outside RocketCSS parser grammar: {name}");
            continue;
        }
        if CSSNANO_UPSTREAM_SKIPS.contains(&name) {
            eprintln!("skipped CSSNano case disabled by the upstream suite: {name}");
            continue;
        }
        if CSSNANO_MINIFY_SKIPS.contains(&name) {
            eprintln!(
                "skipped CSSNano case whose lexical boundary cannot be compared through the AST: {name}"
            );
            continue;
        }
        executed += 1;
        let plugin = case["plugin"]
            .as_str()
            .expect("case plugin must be a string");
        let original_source = case["source"]
            .as_str()
            .expect("case source must be a string");
        let original_expected = case["expected"]
            .as_str()
            .expect("case expected output must be a string");
        let is_declaration = !original_source.contains('{') && original_source.contains(':');
        let source = if is_declaration {
            Cow::Owned(format!("a{{{original_source}}}"))
        } else {
            Cow::Borrowed(original_source)
        };
        let expected = if is_declaration {
            if original_expected.is_empty() {
                Cow::Borrowed("")
            } else {
                Cow::Owned(format!("a{{{original_expected}}}"))
            }
        } else {
            Cow::Borrowed(original_expected)
        };

        let allocator = Allocator::new();
        let Ok(mut stylesheet) = parse(&source, &allocator, ParserOptions::default()) else {
            record_failure(
                plugin,
                &mut failure_count,
                &mut failures,
                &mut failures_by_plugin,
                format!(
                    "{name}: RocketCSS could not parse source\n{}",
                    preview(&source)
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
            parse(&expected, &expected_allocator, ParserOptions::default())
        else {
            record_failure(
                plugin,
                &mut failure_count,
                &mut failures,
                &mut failures_by_plugin,
                format!(
                    "{name}: RocketCSS could not parse CSSNano output\n{}",
                    preview(&expected)
                ),
            );
            continue;
        };
        let canonical_expected = expected_stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap_or_else(|error| panic!("{name} expected output should print: {error}"));

        if actual != canonical_expected {
            record_failure(
                plugin,
                &mut failure_count,
                &mut failures,
                &mut failures_by_plugin,
                format!(
                    "{name}\nsource: {}\nexpected: {}\nactual: {}",
                    preview(&source),
                    preview(&canonical_expected),
                    preview(&actual)
                ),
            );
        }
    }

    assert!(
        failures.is_empty(),
        "{failure_count} of {executed} selected CSSNano cases disagreed by plugin:\n{}\n\nshowing at most 50:\n\n{}",
        failures_by_plugin
            .into_iter()
            .map(|(plugin, count)| format!("{plugin}: {count}"))
            .collect::<std::vec::Vec<_>>()
            .join("\n"),
        failures.join("\n\n")
    );
}

fn corpus_position(variable: &str, default: usize) -> usize {
    let Ok(value) = std::env::var(variable) else {
        return default;
    };
    value
        .parse()
        .unwrap_or_else(|_| panic!("{variable} must be a non-negative integer, got {value:?}"))
}

fn record_failure(
    plugin: &str,
    failure_count: &mut usize,
    failures: &mut std::vec::Vec<String>,
    failures_by_plugin: &mut BTreeMap<String, usize>,
    failure: String,
) {
    *failure_count += 1;
    *failures_by_plugin.entry(plugin.to_owned()).or_default() += 1;
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
    let unsupported_groups = [
        "/lightningcss/math/",
        "/cssnano/discard-overridden/",
        "/cssnano/minify-gradients/",
    ];
    let unsupported_cases = [
        "/lightningcss/declarations/important/",
        "/lightningcss/rules/keyframe-merge/",
        "/lightningcss/rules/merge-layer/",
        "/lightningcss/rules/merge-media/",
        "/lightningcss/rules/merge-selectors/",
        "/lightningcss/values/background-position/",
        "/lightningcss/values/display/",
        "/lightningcss/values/font-family/",
        "/cssnano/discard-duplicates/partial/",
    ];
    unsupported_groups
        .into_iter()
        .chain(unsupported_cases)
        .any(|pattern| path.contains(pattern))
}
