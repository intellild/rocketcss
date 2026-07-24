use std::path::Path;

use rocketcss_allocator::Allocator;
use rocketcss_ast::CssRule;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_nano::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};

use crate::{expected_path, fixture_paths, read_fixture};

// Fixtures that still require cross-node analysis, replacement AST allocation,
// or unsupported value transforms remain in the corpus for future work.
#[test]
fn minifies_static_fixtures() {
    for input in fixture_paths("minify") {
        if still_requires_unsupported_transform(&input) {
            eprintln!("skipped unsupported minify fixture: {}", input.display());
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

        if is_cross_rule_declaration_merging_fixture(&input) {
            minify(&mut stylesheet, MinifyOptions::default());
            let twice = stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap_or_else(|error| panic!("{} should print twice: {error}", input.display()));
            assert_eq!(
                twice,
                actual,
                "fixture should be idempotent on the same AST: {}",
                input.display()
            );
        }
    }
}

#[test]
fn synthesized_cross_rule_fixture_preserves_combined_source_span() {
    let input = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "fixtures/minify/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/assigns-combined-source-span-to-synthesized-rule/input.css",
    );
    let source = read_fixture(&input);
    let allocator = Allocator::new();
    let mut stylesheet = parse(&source, &allocator, ParserOptions::default())
        .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));

    minify(&mut stylesheet, MinifyOptions::default());

    assert_eq!(stylesheet.rules.len(), 1);
    let CssRule::Style(rule) = &stylesheet.rules[0] else {
        panic!("expected one synthesized style rule");
    };
    assert_eq!(rule.span.start, 0);
    assert_eq!(rule.span.end, source.trim_end().len() as u32);
}

fn is_cross_rule_declaration_merging_fixture(input: &Path) -> bool {
    input
        .to_string_lossy()
        .contains("/rocketcss/cross-rule-declaration-merging/")
}

fn still_requires_unsupported_transform(input: &Path) -> bool {
    let path = input.to_string_lossy();
    let unsupported_cases = [
        "/cssnano/discard-empty/rules/",
        "/cssnano/discard-overridden/counter-style/",
        "/cssnano/discard-overridden/keyframes/",
        "/cssnano/normalize-timing/step-start/",
        "/lightningcss/math/color-abs/",
        "/lightningcss/math/color-hypot/",
        "/lightningcss/math/color-max/",
        "/lightningcss/math/color-sign/",
        "/lightningcss/math/opacity-filter/",
        "/lightningcss/math/width-max/",
        "/lightningcss/rules/keyframe-merge/",
        "/lightningcss/rules/merge-layer/",
        "/lightningcss/rules/merge-media/",
        "/lightningcss/rules/merge-selectors/",
        "/rocketcss/cross-rule-declaration-merging/declarations/does-not-drop-live-components-of-a-partially-overridden-shorthand/",
        "/rocketcss/cross-rule-declaration-merging/real-world/does-not-expand-bootstrap-modal-selectors/",
        "/rocketcss/cross-rule-declaration-merging/real-world/does-not-expand-tailwind-screen-reader-utilities/",
        "/rocketcss/cross-rule-declaration-merging/real-world/merges-bootstrap-focus-visible-sibling-selectors/",
        "/rocketcss/cross-rule-declaration-merging/real-world/merges-tailwind-matching-webkit-details-marker-selectors/",
        "/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/preserves-importance-and-order-when-one-occurrence-becomes-many/",
    ];
    unsupported_cases
        .into_iter()
        .any(|pattern| path.contains(pattern))
}
