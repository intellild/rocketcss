use std::path::{Path, PathBuf};

use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_minify::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};

use crate::read_fixture;

// Dynamic fixtures are JSON specs recorded from the upstream CSSNano test
// suites by `.agents/skills/sync-upstream-css-tests/scripts/record-dynamic-cases.mjs`.
// Each case is expanded here — bare declarations are wrapped in a rule — and
// then run through the same parse/minify/print pipeline as the static
// fixtures. The expected value is normalized through the printer as well, so
// comparisons are insensitive to hand-formatting in the upstream sources.
#[test]
fn minifies_dynamic_fixtures() {
    let mut failures = 0usize;
    let mut executed = 0usize;
    let mut skipped = 0usize;

    for spec_path in dynamic_spec_paths("minify-dynamic") {
        let spec_name = spec_display_name(&spec_path);
        let spec: serde_json::Value = serde_json::from_str(&read_fixture(&spec_path))
            .unwrap_or_else(|error| {
                panic!("{} should be valid JSON: {error}", spec_path.display())
            });
        let cases = spec["cases"]
            .as_array()
            .unwrap_or_else(|| panic!("{} should contain a cases array", spec_path.display()));

        for (index, case) in cases.iter().enumerate() {
            let test_name = case["test"].as_str().unwrap_or("<unnamed>");
            let input = case["input"].as_str().unwrap_or_default();
            let expected = case["expected"].as_str().unwrap_or_default();

            if case["upstreamSkip"].as_bool() == Some(true) {
                skipped += 1;
                eprintln!(
                    "skipped upstream-disabled dynamic fixture: {spec_name} case {index} \
                     ({test_name})"
                );
                continue;
            }
            if let Some(reason) =
                still_requires_unsupported_transform(&spec_name, test_name, input, expected)
            {
                skipped += 1;
                eprintln!(
                    "skipped unsupported dynamic fixture: {spec_name} case {index} \
                     ({test_name}): {reason}"
                );
                continue;
            }
            executed += 1;

            let actual = minify_css(&wrap_rule(input));
            // An empty expectation means the minifier is expected to remove
            // the input entirely; do not wrap it into an empty rule.
            let reference = if expected.trim().is_empty() {
                Ok(String::new())
            } else {
                print_css(&wrap_rule(expected))
            };
            // Upstream expectations are hand-written with arbitrary spacing
            // inside values (`matrix(20, 20, ...)`) while the printer keeps
            // value tokens verbatim, so compare with whitespace stripped.
            match (actual, reference) {
                (Ok(actual), Ok(reference)) if squash(&actual) == squash(&reference) => {}
                (actual, reference) => {
                    failures += 1;
                    eprintln!(
                        "dynamic fixture mismatch: {spec_name} case {index} ({test_name})\n  \
                         input:     {input}\n  expected:  {expected}\n  actual:    \
                         {}\n  reference: {}",
                        actual.as_deref().unwrap_or("<error>"),
                        reference.as_deref().unwrap_or("<error>"),
                    );
                }
            }
        }
    }

    assert_eq!(
        failures, 0,
        "{failures} dynamic fixture(s) mismatched ({executed} executed, {skipped} skipped)"
    );
}

// Bare declarations such as `animation:fade 3s ease` are wrapped so they can
// be parsed as a stylesheet, mirroring how cssnano's processCSS accepts both
// declarations and full CSS. At-rules and rule lists are used as-is.
fn wrap_rule(css: &str) -> String {
    let trimmed = css.trim();
    if trimmed.contains('{') || trimmed.starts_with('@') {
        trimmed.to_string()
    } else {
        format!("h1{{{trimmed}}}")
    }
}

fn squash(css: &str) -> String {
    css.chars().filter(|c| !c.is_whitespace()).collect()
}

fn minify_css(source: &str) -> Result<String, String> {
    let allocator = Allocator::new();
    let mut stylesheet = parse(source, &allocator, ParserOptions::default())
        .map_err(|error| format!("parse: {error:?}"))?;
    minify(&mut stylesheet, MinifyOptions::default());
    stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .map_err(|error| error.to_string())
}

fn print_css(source: &str) -> Result<String, String> {
    let allocator = Allocator::new();
    let stylesheet = parse(source, &allocator, ParserOptions::default())
        .map_err(|error| format!("parse: {error:?}"))?;
    stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .map_err(|error| error.to_string())
}

fn spec_display_name(spec_path: &Path) -> String {
    spec_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| spec_path.display().to_string())
}

// Dynamic specs are `*.json` files rather than `input.css`/`output.css` pairs,
// so they need their own directory walk.
fn dynamic_spec_paths(relative_dir: &str) -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(relative_dir);
    let mut paths = Vec::new();
    collect_spec_paths(&root, &mut paths);
    paths.sort();
    assert!(!paths.is_empty(), "no specs found in {}", root.display());
    paths
}

fn collect_spec_paths(dir: &Path, paths: &mut Vec<PathBuf>) {
    let entries = std::fs::read_dir(dir).unwrap_or_else(|error| {
        panic!(
            "failed to read fixture directory {}: {error}",
            dir.display()
        )
    });

    for entry in entries {
        let entry = entry
            .unwrap_or_else(|error| panic!("failed to read entry in {}: {error}", dir.display()));
        let path = entry.path();
        if path.is_dir() {
            collect_spec_paths(&path, paths);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            paths.push(path);
        }
    }
}

// Cases whose transforms RocketCSS does not support yet stay recorded in the
// specs for future work, mirroring the static skip list in `minify.rs`. An
// entry matches when the spec file name contains `spec_pattern` and the test
// name, case input, or expected output contains `text_pattern`.
fn still_requires_unsupported_transform(
    spec_name: &str,
    test_name: &str,
    input: &str,
    expected: &str,
) -> Option<&'static str> {
    let unsupported_cases: &[(&str, &str, &str)] = &[
        // Plugins that RocketCSS does not implement at all yet.
        (
            "reduce-initial",
            "",
            "initial-value substitution not implemented",
        ),
        ("merge-rules", "", "cross-rule merging not implemented"),
        (
            "merge-idents",
            "",
            "identifier merge/rename not implemented",
        ),
        ("reduce-idents", "", "identifier renaming not implemented"),
        (
            "discard-unused",
            "",
            "unused at-rule removal not implemented",
        ),
        ("zindex", "", "z-index rebasing not implemented"),
        ("svgo", "", "SVG optimization out of scope"),
        // postcss-ordered-values: the ordering transform is implemented, but
        // the recorded expectations come from the plugin in isolation while
        // the harness runs the full minify pipeline.
        (
            "ordered-values",
            "should order border consistently",
            "pipeline dedups identical declarations",
        ),
        (
            "ordered-values",
            "color functions",
            "pipeline converts colors",
        ),
        (
            "ordered-values",
            "currentColor",
            "pipeline strips initial currentcolor",
        ),
        (
            "ordered-values",
            "calc width in borders",
            "pipeline folds calc",
        ),
        (
            "ordered-values",
            "box-shadow consistently (11)",
            "pipeline converts colors",
        ),
        (
            "ordered-values",
            "invalid box-shadow",
            "pipeline converts colors",
        ),
        (
            "ordered-values",
            "important comments (border)",
            "upstream aborts ordering on comments",
        ),
        (
            "ordered-values",
            "important comments (transition)",
            "upstream aborts ordering on comments",
        ),
        ("ordered-values", "border-block", "pipeline converts colors"),
        (
            "ordered-values",
            "assigns keyframe name last",
            "upstream reorders ambiguous keyword names into a non-round-trip-safe order",
        ),
        (
            "ordered-values",
            "list-style 9",
            "pipeline strips url quotes",
        ),
        (
            "ordered-values",
            "list-style 12",
            "pipeline strips url quotes",
        ),
        ("borders", "", "border longhand merging not implemented"),
        ("columns", "", "column merging not implemented"),
        // Upstream's testPassthrough(t, fixture) helper is called without the
        // fixture argument, so the recorded expectation is literally
        // "undefined"; these cases carry no upstream signal.
        (
            "timing-functions",
            "undefined",
            "upstream helper emits undefined",
        ),
        // normalize-positions: the recorded expectations come from the
        // positions plugin alone, but the harness runs the full pipeline.
        ("positions", "#f1ff", "pipeline also minifies #f1ff to #f1f"),
        ("positions", "function", "pipeline folds calc/min/max/clamp"),
        (
            "positions",
            "with var",
            "pipeline converts lengths/quotes around var()",
        ),
        (
            "positions",
            "background size",
            "pipeline further minifies the value",
        ),
        (
            "positions",
            "multiple background",
            "pipeline further minifies the value",
        ),
        (
            "positions",
            "handle 0",
            "pipeline further minifies the value",
        ),
        // postcss-minify-selectors gaps.
        (
            "minify-selectors",
            "universal selector",
            "universal selector removal not implemented",
        ),
        (
            "minify-selectors",
            "should sort",
            "selector sorting not implemented",
        ),
        (
            "minify-selectors",
            ":not",
            ":not() simplification not implemented",
        ),
        (
            "minify-selectors",
            "::before to :before (2)",
            "pseudo-element case differs",
        ),
        (
            "minify-selectors",
            "polymer mixins",
            "parser: mixin selectors",
        ),
        (
            "minify-selectors",
            "namespace",
            "parser: namespaced attribute selector",
        ),
        ("minify-selectors", "fold:", ":is() folding not implemented"),
        (
            "minify-selectors",
            "no-fold:",
            "selector sorting not implemented",
        ),
        ("minify-selectors", "mixin-like", "parser: mixin selectors"),
        // postcss-convert-values gaps and option-dependent behavior.
        (
            "convert-values",
            "custom properties",
            "custom property conversion is option-gated upstream",
        ),
        (
            "convert-values",
            "strip the units from length",
            "pipeline collapses the shorthand further",
        ),
        (
            "convert-values",
            "flex basis",
            "0% stripping policy differs",
        ),
        ("convert-values", "calc", "calc arithmetic differs"),
        (
            "convert-values",
            "comma separated",
            "pipeline further minifies the value",
        ),
        (
            "convert-values",
            "angle units",
            "declaration dedup pipeline differs",
        ),
        (
            "convert-values",
            "length units",
            "option-dependent upstream",
        ),
        ("convert-values", "time units", "option-dependent upstream"),
        ("convert-values", "hsl", "pipeline converts colors"),
        (
            "convert-values",
            "percentage from 0",
            "0% stripping policy differs",
        ),
        (
            "convert-values",
            "round pixel",
            "value rounding not implemented",
        ),
        (
            "convert-values",
            "clamp",
            "opacity clamping not implemented",
        ),
        (
            "convert-values",
            "SVG properties",
            "0% stripping on stroke-* differs",
        ),
        (
            "convert-values",
            "border-image-width",
            "0% stripping policy differs",
        ),
        (
            "convert-values",
            "color-mix",
            "pipeline minifies color-mix colors",
        ),
        ("convert-values", "box-shadow", "alpha rounding differs"),
        // postcss-minify-gradients gaps.
        (
            "minify-gradients",
            "uppercase",
            "property/function case handling differs",
        ),
        (
            "minify-gradients",
            "\"at\"",
            "pipeline also minifies gradient colors",
        ),
        (
            "minify-gradients",
            "-webkit",
            "prefixed gradient variants differ",
        ),
        (
            "minify-gradients",
            "floating point",
            "property/function case handling differs",
        ),
        (
            "minify-gradients",
            "custom property references",
            "pipeline normalizes positions around var()",
        ),
        (
            "minify-gradients",
            "last is zero",
            "color case handling differs",
        ),
        // postcss-colormin gaps.
        ("colormin", "color values (2)", "hex8 alpha output differs"),
        (
            "colormin",
            "color values (11)",
            "declaration dedup pipeline",
        ),
        (
            "colormin",
            "gradients",
            "name-vs-hex choice in gradients differs",
        ),
        (
            "colormin",
            "color stops",
            "-webkit-gradient handling differs",
        ),
        ("colormin", "calc", "pipeline folds calc"),
        (
            "colormin",
            "extra spaces",
            "rgb-in-gradient spacing policy differs",
        ),
        (
            "colormin",
            "transparent",
            "rgba(0,0,0,0) conversion differs",
        ),
        ("colormin", "custom properties", "option-dependent upstream"),
        ("colormin", "Browserslist", "option-dependent upstream"),
        ("colormin", "slash alpha", "hex8 alpha output differs"),
        // postcss-discard-duplicates gaps.
        (
            "discard-duplicates",
            "font-weight:bold",
            "pipeline converts font-weight",
        ),
        ("discard-duplicates", "@charset", "parser: @charset"),
        (
            "discard-duplicates",
            "@media queries",
            "cross-rule dedup not implemented",
        ),
        (
            "discard-duplicates",
            "different vendors",
            "pipeline normalizes 100% to `to`",
        ),
        (
            "discard-duplicates",
            "duplicate rules (2)",
            "non-adjacent rule dedup not implemented",
        ),
        (
            "discard-duplicates",
            "differently ordered",
            "reordered-declaration dedup not implemented",
        ),
        (
            "discard-duplicates",
            "partial duplicates",
            "partial dedup not implemented",
        ),
        (
            "discard-duplicates",
            "normalising declarations",
            "pipeline merges equivalent declarations",
        ),
        (
            "discard-duplicates",
            "duplicate rules and declarations",
            "non-adjacent rule dedup not implemented",
        ),
        // postcss-discard-comments gaps.
        (
            "discard-comments",
            "special comments",
            "important-comment preservation not implemented",
        ),
        (
            "discard-comments",
            "single line comments",
            "parser: line comments",
        ),
        (
            "discard-comments",
            "with a flag",
            "option-dependent upstream",
        ),
        (
            "discard-comments",
            "@rule param",
            "parser: comments in at-rule prelude",
        ),
        (
            "discard-comments",
            "at rules without comments",
            "parser: @page nested rules",
        ),
        (
            "discard-comments",
            "from other plugins",
            "comment AST retention not implemented",
        ),
        (
            "discard-comments",
            "URL strings",
            "comment-in-string handling differs",
        ),
        (
            "discard-comments",
            "string comments",
            "comment-in-string handling differs",
        ),
        (
            "discard-comments",
            "multiple strings",
            "comment-in-string handling differs",
        ),
        (
            "discard-comments",
            "keyframe names with strings",
            "comment-in-string handling differs",
        ),
        // postcss-merge-longhand boxBase: full-pipeline divergence and gaps.
        (
            "boxBase",
            "nesting level",
            "pipeline converts absolute lengths",
        ),
        (
            "boxBase",
            "save fallbacks",
            "var() fallback merging not implemented",
        ),
        (
            "boxBase",
            "custom properties",
            "custom property case preservation",
        ),
        // postcss-minify-font-values gaps.
        (
            "minify-font-values",
            "one character",
            "single-char font unquoting edge case",
        ),
        (
            "minify-font-values",
            "space at the end",
            "font name unquoting edge case",
        ),
        (
            "minify-font-values",
            "not remove duplicates",
            "case-sensitive family dedup differs",
        ),
        (
            "minify-font-values",
            "not dedupe",
            "case-sensitive family dedup differs",
        ),
        (
            "minify-font-values",
            "custom props",
            "custom property handling differs",
        ),
        (
            "minify-font-values",
            "font property #3",
            "font shorthand minification gap",
        ),
        (
            "minify-font-values",
            "font property #4",
            "font shorthand minification gap",
        ),
        (
            "minify-font-values",
            "css variables",
            "var() in font-family differs",
        ),
        // postcss-discard-empty gaps (mirrors the static skip list).
        (
            "discard-empty",
            "@ rules",
            "empty at-rule removal needs cascade",
        ),
        (
            "discard-empty",
            "empty rules",
            "cascading empty removal not implemented",
        ),
        (
            "discard-empty",
            "empty declarations",
            "empty declaration removal not implemented",
        ),
        ("discard-empty", "null selectors", "parser: null selector"),
        (
            "discard-empty",
            "empty media queries",
            "cascading empty removal not implemented",
        ),
        // postcss-normalize-url gaps.
        (
            "normalize-url",
            "default port",
            "port removal not implemented",
        ),
        (
            "normalize-url",
            "traversal",
            "path normalization not implemented",
        ),
        (
            "normalize-url",
            "current directory",
            "path normalization not implemented",
        ),
        (
            "normalize-url",
            "stripping quotes",
            "path normalization not implemented",
        ),
        (
            "normalize-url",
            "special characters",
            "path normalization not implemented",
        ),
        (
            "normalize-url",
            "whitespace",
            "url whitespace trimming not implemented",
        ),
        (
            "normalize-url",
            "multiple backgrounds",
            "path normalization not implemented",
        ),
        ("normalize-url", "embedded fonts", "parser: data url"),
        (
            "normalize-url",
            "protocol relative",
            "port removal not implemented",
        ),
        (
            "normalize-url",
            "doesn't find a url",
            "pipeline converts color/font-weight",
        ),
        (
            "normalize-url",
            "uppercase URL",
            "url case handling differs",
        ),
        // postcss-normalize-repeat-style: full-pipeline divergence.
        (
            "repeat-style",
            "with var",
            "pipeline converts lengths/quotes around var()",
        ),
        (
            "repeat-style",
            "multiple background",
            "pipeline normalizes positions/quotes",
        ),
        // postcss-normalize-whitespace gaps.
        (
            "normalize-whitespace",
            "nested functions",
            "pipeline folds calc",
        ),
        (
            "normalize-whitespace",
            "css variables",
            "pipeline folds var() fallbacks",
        ),
        (
            "normalize-whitespace",
            "env variables",
            "pipeline folds env() fallbacks",
        ),
        // postcss-normalize-unicode: parser gaps.
        (
            "normalize-unicode",
            "css variables",
            "parser: unicode-range var()",
        ),
        (
            "normalize-unicode",
            "env variables",
            "parser: unicode-range env()",
        ),
        (
            "normalize-unicode",
            "initial",
            "parser: unicode-range initial",
        ),
        // postcss-reduce-transforms: full-pipeline calc folding.
        (
            "reduce-transforms",
            "difference fallback",
            "pipeline folds var() fallbacks",
        ),
        (
            "reduce-transforms",
            "with calc",
            "pipeline folds calc in transform args",
        ),
        // postcss-unique-selectors gaps.
        (
            "unique-selectors",
            "sort",
            "selector sorting not implemented",
        ),
        (
            "unique-selectors",
            "comments",
            "comment-in-selector handling differs",
        ),
        // postcss-normalize-charset gaps.
        (
            "normalize-charset",
            "move up first",
            "@charset reordering not implemented",
        ),
        (
            "normalize-charset",
            "remove all charset",
            "@charset removal not implemented",
        ),
        (
            "normalize-charset",
            "add a charset",
            "@charset insertion not implemented",
        ),
        (
            "normalize-charset",
            "remove extra charset",
            "@charset dedup not implemented",
        ),
        (
            "normalize-charset",
            "add on",
            "@charset insertion not implemented",
        ),
        // postcss-normalize-string gaps.
        (
            "normalize-string",
            "backslashes",
            "unescape vs percent-encode policy differs",
        ),
        // postcss-minify-params gaps.
        (
            "minify-params",
            "uppercase",
            "@media keyword case handling differs",
        ),
        (
            "minify-params",
            "normalise @media queries (2)",
            "pipeline converts media query values",
        ),
        (
            "minify-params",
            "normalise \"all\"",
            "@media all removal not implemented",
        ),
        ("minify-params", "Browserslist", "option-dependent upstream"),
        (
            "minify-params",
            "not all and",
            "pipeline converts media query values",
        ),
    ];
    unsupported_cases
        .iter()
        .find(|(spec_pattern, text_pattern, _)| {
            spec_name.contains(spec_pattern)
                && (test_name.contains(text_pattern)
                    || input.contains(text_pattern)
                    || expected.contains(text_pattern))
        })
        .map(|(_, _, reason)| *reason)
}
