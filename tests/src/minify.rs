use std::path::Path;

use rocketcss_ast::CssRule;
use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::Allocator;
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

#[test]
fn minifies_enabled_s3_cross_rule_fixtures() {
    let mut fixture_count = 0;
    for input in fixture_paths("minify") {
        let path = fixture_path_key(&input);
        if is_enabled_s3_cross_rule_fixture(&path) {
            assert_minifies_static_fixture(&input);
            fixture_count += 1;
        }
    }
    assert_eq!(fixture_count, 17);
}

fn assert_minifies_static_fixture(input: &Path) {
    let source = read_fixture(input);
    let expected = read_fixture(&expected_path(input));
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(&source, &allocator, &mut token, ParserOptions::default())
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
fn synthesized_cross_rule_fixture_preserves_combined_source_span() {
    let input = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "fixtures/minify/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/assigns-combined-source-span-to-synthesized-rule/input.css",
    );
    let source = read_fixture(&input);
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(&source, &allocator, &mut token, ParserOptions::default())
            .unwrap_or_else(|error| panic!("{} should parse: {error:?}", input.display()));

        minify(&mut stylesheet, &mut token, MinifyOptions::default());

        let rules = stylesheet
            .rules_in_list(stylesheet.stylesheet_root().root_rules())
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(rules.len(), 1);
        let CssRule::Style(rule) = rules[0].1.payload() else {
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
        && !is_enabled_s3_cross_rule_fixture(&path)
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
        // The current Background AST only covers the color-only shorthand;
        // gradient shorthand values remain intentionally opaque until the
        // full layer grammar is implemented.
        "/cssnano/colormin/gradient/",
        // Box-shadow and text-shadow are intentionally unparsed, so their
        // nested colors cannot be normalized across the opaque barrier.
        "/cssnano/colormin/rgb/",
        "/cssnano/colormin/text-shadow/",
        // Percentage zero is retained in typed margin values because it has
        // different computed-value semantics from a unitless length zero.
        "/cssnano/convert-values/zero-lengths/",
        // Gradient shorthand direction normalization requires the complete
        // Background layer grammar and is opaque in the current AST.
        "/cssnano/minify-gradients/",
        // Position keyword-to-percentage normalization is not part of the
        // typed position AST minifier yet.
        "/cssnano/normalize-positions/",
        "/lightningcss/values/background-position/",
        // Background shorthand repeat folding still needs the complete layer
        // grammar; the fallback remains lossless.
        "/cssnano/normalize-repeat/",
        // URL normalization in the background shorthand is likewise deferred
        // until its image-layer parser is complete.
        "/cssnano/normalize-url/",
        // Matrix and transform-function reduction is separate from typed
        // parsing and remains intentionally unimplemented.
        "/cssnano/reduce-transforms/",
        // Comment-containing values are lossless opaque fallbacks by design.
        "/cssnano-extra/normalize-whitespace/comments/",
        // Static calc evaluation is not part of the typed property coverage
        // work; retain these upstream math cases as explicit skips.
        "/lightningcss/math/",
        // `vertical-align` is still an opaque fallback, so its authored
        // numeric spelling is preserved instead of applying Lightning's
        // leading-zero normalization.
        "/lightningcss/values/leading-zero/",
        // The upstream multi-layer mask case uses target-aware position and
        // length normalization that is not part of this fixture harness.
        "/lightningcss/values/mask-multilayer/",
        // These cases require target-browser prefix expansion, which is not
        // enabled by RocketCSS's default minifier.
        "/lightningcss/values/margin-inline-prefix/",
        "/lightningcss/values/padding-inline-prefix/",
        "/lightningcss/values/mask-composite-prefix/",
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

fn is_enabled_s3_cross_rule_fixture(path: &str) -> bool {
    [
        "/rocketcss/cross-rule-declaration-merging/real-world/factors-tailwind-mask-setup-without-reordering-custom-values/",
        "/rocketcss/cross-rule-declaration-merging/real-world/keeps-bootstrap-placeholder-vendor-groups-separate/",
        "/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/assigns-combined-source-span-to-synthesized-rule/",
        "/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/keeps-overlapping-candidate-rule-ids-stable-across-insertion/",
        "/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/synthesized-rules-survive-the-minify-scratch-allocator/",
        "/rocketcss/cross-rule-declaration-merging/review-findings/ast-ownership/transfers-a-non-clone-custom-declaration-into-the-shared-rule/",
        "/rocketcss/cross-rule-declaration-merging/review-findings/state-machine/s3-endpoint-edits-do-not-create-a-transient-bypass-edge/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/canonicalizes-synthesized-selector-unions-immediately/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/complete-factoring-reconnects-the-live-chain-through-the-shared-rule/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/factors-a-complete-equal-run-in-one-stable-transition/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/factors-single-declaration-with-left-prefix/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/factors-single-declaration-with-right-prefix/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/fingerprint-matches-still-require-exact-value-equality/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/importance-is-part-of-the-declaration-history-context/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/overlapping-partial-candidates-commit-from-left-to-right/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/rejects-zero-progress-partial-merge-plans/",
        "/rocketcss/cross-rule-declaration-merging/state-machine/selector-materialization-failure-leaves-both-endpoints-unchanged/",
    ]
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
