use std::path::{Path, PathBuf};

use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::Allocator;
use rocketcss_nano::{MinifyOptions, minify};
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
            if let Some(reason) = still_requires_unsupported_transform(case) {
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
    assert_eq!(
        (executed, skipped),
        (1239, 866),
        "dynamic fixture coverage changed; audit every newly recorded or reclassified case"
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
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(source, &allocator, &mut token, ParserOptions::default())
            .map_err(|error| format!("parse: {error:?}"))?;
        minify(&mut stylesheet, &mut token, MinifyOptions::default());
        stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .map_err(|error| error.to_string())
    })
}

fn print_css(source: &str) -> Result<String, String> {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let stylesheet = parse(source, &allocator, &mut token, ParserOptions::default())
            .map_err(|error| format!("parse: {error:?}"))?;
        stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .map_err(|error| error.to_string())
    })
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

// Unsupported cases stay next to their recorded input and expectation so broad
// plugin-level patterns cannot hide already supported pass-through cases.
fn still_requires_unsupported_transform(case: &serde_json::Value) -> Option<&str> {
    case["rocketcssSkip"].as_str()
}
