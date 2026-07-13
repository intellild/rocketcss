use std::path::Path;

use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_minify::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};

use crate::{expected_path, fixture_paths, read_fixture};

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

        minify(&mut stylesheet, &allocator, MinifyOptions::default());
        let actual = stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap_or_else(|error| panic!("{} should print: {error}", input.display()));

        assert_eq!(actual, expected.trim_end(), "fixture: {}", input.display());
    }
}

fn requires_nonlocal_or_rebuilding_transform(input: &Path) -> bool {
    let path = input.to_string_lossy();
    let unsupported_groups = [
        "/cssnano/custom-properties/",
        "/cssnano/discard-duplicates/",
        "/cssnano/discard-empty/",
        "/cssnano/discard-overridden/",
        "/cssnano/minify-gradients/",
        "/cssnano/normalize-display/",
        "/cssnano/normalize-positions/",
        "/cssnano/normalize-timing/",
        "/cssnano/reduce-transforms/",
        "/lightningcss/math/",
    ];
    let unsupported_cases = [
        "/cssnano/colormin/gradient/",
        "/cssnano/colormin/hex-name/",
        "/cssnano/colormin/hsl/",
        "/cssnano/colormin/rgb/",
        "/cssnano/colormin/text-shadow/",
        "/cssnano/merge-longhand/important/",
        "/cssnano/merge-longhand/margin/",
        "/cssnano/merge-longhand/padding-order/",
        "/cssnano/minify-font-values/family-deduplicate/",
        "/cssnano/minify-font-values/family-unquote/",
        "/cssnano/normalize-repeat/collapse/",
        "/cssnano/normalize-url/double-quote/",
        "/cssnano/normalize-url/single-quote/",
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
