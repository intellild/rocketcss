use std::path::Path;

use rocketcss_ast::CssRule;
use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::GhostToken;
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

        assert_minifies_static_fixture(&input);
    }
}

#[test]
fn minifies_enabled_s1_cross_rule_fixtures() {
    let mut fixture_count = 0;
    for input in fixture_paths("minify") {
        let path = fixture_path_key(&input);
        if is_enabled_s1_cross_rule_fixture(&path) {
            assert_minifies_static_fixture(&input);
            fixture_count += 1;
        }
    }
    assert_eq!(fixture_count, 3);
}

#[test]
fn minifies_enabled_s2_only_cross_rule_fixtures() {
    let mut fixture_count = 0;
    for input in fixture_paths("minify") {
        let path = fixture_path_key(&input);
        if is_enabled_s2_only_cross_rule_fixture(&path) {
            assert_minifies_static_fixture(&input);
            fixture_count += 1;
        }
    }
    assert_eq!(fixture_count, 5);
}

fn assert_minifies_static_fixture(input: &Path) {
    let source = read_fixture(input);
    let expected = read_fixture(&expected_path(input));
    GhostToken::scope(|mut token| {
        let mut stylesheet = parse(&source, &mut token, ParserOptions::default())
            .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));

        minify(&mut stylesheet, &mut token, MinifyOptions::default());
        let actual = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

        assert_eq!(actual, expected.trim_end(), "fixture: {}", input.display());

        if is_cross_rule_declaration_merging_fixture(input) {
            minify(&mut stylesheet, &mut token, MinifyOptions::default());
            let twice = stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap_or_else(|error| panic!("{} should print twice: {error}", input.display()));
            assert_eq!(
                twice,
                actual,
                "fixture should be idempotent on the same AST: {}",
                input.display()
            );
        }
    });
}

#[test]
#[ignore = "requires S3 synthesized-rule commit"]
fn synthesized_cross_rule_fixture_preserves_combined_source_span() {
    let input = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "fixtures/minify/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/assigns-combined-source-span-to-synthesized-rule/input.css",
    );
    let source = read_fixture(&input);
    GhostToken::scope(|mut token| {
        let mut stylesheet = parse(&source, &mut token, ParserOptions::default())
            .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));

        minify(&mut stylesheet, &mut token, MinifyOptions::default());

        assert_eq!(stylesheet.root_rules().len(), 1);
        let CssRule::Style(rule) = &stylesheet.root_rules()[0] else {
            panic!("expected one synthesized style rule");
        };
        assert_eq!(rule.span.start, 0);
        assert_eq!(rule.span.end, source.trim_end().len() as u32);
    });
}

fn fixture_path_key(input: &Path) -> String {
    input.to_string_lossy().replace('\\', "/")
}

#[test]
fn normalizes_fixture_paths_for_matching() {
    let input = Path::new(
        r"C:\rocketcss\tests\fixtures\minify\rocketcss\cross-rule-declaration-merging\case\input.css",
    );
    assert!(is_cross_rule_declaration_merging_fixture(input));
}

fn is_cross_rule_declaration_merging_fixture(input: &Path) -> bool {
    fixture_path_key(input).contains("/rocketcss/cross-rule-declaration-merging/")
}

fn still_requires_unsupported_transform(input: &Path) -> bool {
    let path = fixture_path_key(input);
    if is_cross_rule_declaration_merging_fixture(input)
        && !is_enabled_s1_cross_rule_fixture(&path)
        && !is_enabled_s2_only_cross_rule_fixture(&path)
    {
        return true;
    }
    let unsupported_cases = [
        "/cssnano/discard-duplicates/declarations/",
        "/cssnano/discard-duplicates/partial/",
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
        // Lightning CSS normalizes invalid `display: table-cell flow` to
        // `display: table-cell`; RocketCSS preserves the invalid token stream.
        "/lightningcss/values/display/",
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

// Every fixture keeps a retained different-selector rule between equal
// EffectiveKey entries. S1 may discover structural candidates, but none can
// commit; only S2 can change an equal-key history.
fn is_enabled_s2_only_cross_rule_fixture(path: &str) -> bool {
    [
        "/rocketcss/cross-rule-declaration-merging/declarations/removes-exact-duplicate-across-non-adjacent-blocks/",
        "/rocketcss/cross-rule-declaration-merging/declarations/keeps-case-distinct-custom-properties/",
        "/rocketcss/cross-rule-declaration-merging/declarations/keeps-fallback-and-importance-chains/",
        "/rocketcss/cross-rule-declaration-merging/declarations/keeps-logical-and-physical-properties-when-direction-is-not-proven/",
        "/rocketcss/cross-rule-declaration-merging/declarations/treats-revert-values-conservatively/",
    ]
    .into_iter()
    .any(|pattern| path.contains(pattern))
}

fn is_enabled_s1_cross_rule_fixture(path: &str) -> bool {
    [
        "/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/s1-emits-a-retired-left-rule-exactly-once/",
        "/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/imports-an-existing-previous-merged-chain-on-a-second-minify/",
        "/rocketcss/cross-rule-declaration-merging/review-findings/semantics/merges-only-within-the-same-authored-layer-context/",
    ]
    .into_iter()
    .any(|pattern| path.contains(pattern))
}
