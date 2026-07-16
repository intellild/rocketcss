use std::path::Path;

use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_minify::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};

use crate::{expected_path, fixture_paths, read_fixture};

// Fixtures that still require cross-node analysis, replacement AST allocation,
// or unsupported value transforms remain in the corpus for future work.
#[test]
fn minifies_upstream_fixtures() {
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
    }
}

fn still_requires_unsupported_transform(input: &Path) -> bool {
    let path = input.to_string_lossy();
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
        "/lightningcss/rules/keyframe-merge/",
        "/lightningcss/rules/merge-layer/",
        "/lightningcss/rules/merge-media/",
        "/lightningcss/rules/merge-selectors/",
    ];
    unsupported_cases
        .into_iter()
        .any(|pattern| path.contains(pattern))
}
