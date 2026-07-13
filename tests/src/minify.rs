use std::{borrow::Cow, collections::BTreeMap, path::Path};

use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_minify::{BrowserHackTarget, MinifyOptions, minify};
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
    "packages/postcss-discard-unused/test/index.js:34/547",
    "packages/postcss-discard-unused/test/index.js:52/550",
    "packages/postcss-discard-unused/test/index.js:118/559",
    "packages/postcss-discard-unused/test/index.js:131/560",
    "packages/postcss-discard-unused/test/index.js:144/561",
    "packages/postcss-discard-unused/test/namespace.js:16/564",
    "packages/postcss-discard-unused/test/namespace.js:20/565",
    "packages/postcss-discard-unused/test/namespace.js:46/569",
    "packages/postcss-normalize-url/test/index.js:29/2175",
    "packages/postcss-normalize-url/test/index.js:229/2203",
    "packages/postcss-normalize-url/test/index.js:236/2204",
    "packages/postcss-normalize-url/test/index.js:244/2205",
    "packages/postcss-normalize-url/test/index.js:252/2206",
    "packages/postcss-normalize-url/test/index.js:260/2207",
    "packages/postcss-merge-longhand/test/borders.js:1213/768",
    "packages/cssnano/test/postcss-normalize-url.js:153/275",
    "packages/postcss-merge-rules/test/index.js:409/1056",
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
    "packages/postcss-normalize-url/test/index.js:268/2208",
    "packages/postcss-ordered-values/test/index.js:250/2268",
    "packages/postcss-ordered-values/test/index.js:255/2269",
    "packages/postcss-ordered-values/test/index.js:260/2270",
    "packages/postcss-ordered-values/test/index.js:265/2271",
    "packages/postcss-ordered-values/test/index.js:270/2272",
    "packages/postcss-ordered-values/test/index.js:514/2309",
    "packages/cssnano/test/fixtures.js:231/52",
    "packages/cssnano/test/postcss-normalize-url.js:17/258",
    "packages/cssnano/test/postcss-normalize-url.js:162/276",
    "packages/postcss-merge-rules/test/index.js:854/1103",
];

const CSSNANO_UPSTREAM_SKIPS: &[&str] = &["packages/postcss-minify-params/test/index.js:47/1313"];

const CSSNANO_EXTERNAL_PLUGIN_SKIPS: &[&str] = &[
    "packages/cssnano-preset-default/test/integrations.js:38/30",
    "packages/cssnano/test/fixtures.js:349/61",
    "packages/cssnano/test/postcss-svgo.js:7/292",
    "packages/cssnano/test/postcss-svgo.js:15/293",
    "packages/cssnano/test/postcss-svgo.js:23/294",
    "packages/cssnano/test/postcss-svgo.js:31/295",
    "packages/postcss-svgo/test/index.js:19/3088",
    "packages/postcss-svgo/test/index.js:27/3089",
    "packages/postcss-svgo/test/index.js:35/3090",
    "packages/postcss-svgo/test/index.js:43/3091",
    "packages/postcss-svgo/test/index.js:51/3092",
    "packages/postcss-svgo/test/index.js:59/3093",
    "packages/postcss-svgo/test/index.js:67/3094",
    "packages/postcss-svgo/test/index.js:85/3095",
    "packages/postcss-svgo/test/index.js:93/3096",
    "packages/postcss-svgo/test/index.js:101/3097",
    "packages/postcss-svgo/test/index.js:109/3098",
    "packages/postcss-svgo/test/index.js:117/3099",
    "packages/postcss-svgo/test/index.js:137/3100",
    "packages/postcss-svgo/test/index.js:145/3101",
    "packages/postcss-svgo/test/index.js:163/3103",
    "packages/postcss-svgo/test/index.js:171/3104",
    "packages/postcss-svgo/test/index.js:187/3105",
    "packages/postcss-svgo/test/index.js:195/3106",
    "packages/postcss-svgo/test/index.js:204/3107",
    "packages/postcss-svgo/test/index.js:285/3116",
];

const CSSNANO_EXTERNAL_PREPROCESSOR_SKIPS: &[&str] =
    &["packages/postcss-discard-comments/test/index.js:290/488"];

const CSSNANO_CORPUS_SKIPS: &[&str] = &[
    // This factory excludes autoprefixer, but the exclusion is not present in
    // the recorded per-call options.
    "packages/cssnano-preset-advanced/test/autoprefixer.js:39/3",
    // The standalone sorter expectation is re-minified by the corpus
    // canonicalizer, which applies a different border merge pipeline.
    "packages/cssnano-preset-advanced/test/css-declaration-sorter.js:142/23",
    // This preset factory uses modern Browserslist targets and enables prefix
    // addition, but that factory configuration is absent from the recording.
    "packages/cssnano-preset-advanced/test/integrations.js:36/27",
    // This preset factory uses `env: modern`, but the upstream integration
    // helper does not pass factory configuration to the recorded call.
    "packages/cssnano-preset-default/test/integrations.js:30/29",
    // The standalone plugin expectation intentionally retains whitespace that
    // the complete RocketCSS minifier removes before identifier reduction.
    "packages/postcss-reduce-idents/test/index.js:506/2394",
    // The upstream option is a custom JavaScript encoder function. JSON drops
    // that function, so the runtime corpus cannot reproduce its PREFIX output.
    "packages/postcss-reduce-idents/test/index.js:517/2395",
];

const CSSNANO_OUTPUT_EXPANSION_SKIPS: &[&str] = &[
    // Canonical CSSNano output splits two border declarations into three.
    // The local pass has no spare declaration slot and intentionally does not
    // allocate replacement AST declaration objects.
    "packages/postcss-merge-rules/test/index.js:387/1053",
    "packages/postcss-merge-rules/test/index.js:525/1072",
];

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
    let mut selected = 0usize;
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
        selected += 1;
        let name = case["name"].as_str().expect("case name must be a string");
        if CSSNANO_PARSER_SKIPS.contains(&name) || is_browser_hack_parser_skip(name) {
            eprintln!("skipped CSSNano case outside RocketCSS parser grammar: {name}");
            continue;
        }
        if CSSNANO_UPSTREAM_SKIPS.contains(&name) {
            eprintln!("skipped CSSNano case disabled by the upstream suite: {name}");
            continue;
        }
        if CSSNANO_EXTERNAL_PLUGIN_SKIPS.contains(&name) {
            eprintln!("skipped CSSNano case requiring an external optimizer: {name}");
            continue;
        }
        if CSSNANO_EXTERNAL_PREPROCESSOR_SKIPS.contains(&name) {
            eprintln!("skipped CSSNano case requiring an external preprocessor: {name}");
            continue;
        }
        if CSSNANO_CORPUS_SKIPS.contains(&name) {
            eprintln!("skipped CSSNano case not reproducible by the runtime corpus: {name}");
            continue;
        }
        if CSSNANO_OUTPUT_EXPANSION_SKIPS.contains(&name) {
            eprintln!(
                "skipped CSSNano case requiring more output declarations than reusable AST slots: {name}"
            );
            continue;
        }
        if CSSNANO_MINIFY_SKIPS.contains(&name) || is_browser_hack_lexical_skip(name) {
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
        let upstream_options = &case["options"];
        let browser_hack_target = if plugin == "pluginCreator" {
            match upstream_options["overrideBrowserslist"].as_str() {
                Some("Firefox 2") => Some(BrowserHackTarget::Firefox2),
                Some("IE 6") => Some(BrowserHackTarget::Ie6),
                Some("IE 7") => Some(BrowserHackTarget::Ie7),
                Some("IE 8") => Some(BrowserHackTarget::Ie8),
                Some("IE 9") => Some(BrowserHackTarget::Ie9),
                Some("opera9") => Some(BrowserHackTarget::Opera9),
                Some("Chrome 58") => Some(BrowserHackTarget::Modern),
                _ => None,
            }
        } else {
            None
        };
        let targets_legacy_browsers = upstream_options["overrideBrowserslist"]
            .as_str()
            .is_some_and(|targets| {
                targets.eq_ignore_ascii_case("IE 6") || targets.eq_ignore_ascii_case("IE 11")
            })
            || upstream_options["env"]
                .as_str()
                .is_some_and(|environment| environment.eq_ignore_ascii_case("legacy"));
        let selector_targets_support_is = plugin == "postcss-minify-selectors"
            && upstream_options["overrideBrowserslist"]
                .as_str()
                .is_some_and(|targets| targets.eq_ignore_ascii_case("last 2 Chrome versions"))
            && upstream_options["convertToIs"].as_bool() != Some(false);
        let z_index_start = upstream_options["startIndex"]
            .as_i64()
            .filter(|value| *value != 0)
            .and_then(|value| i32::try_from(value).ok())
            .unwrap_or(1);
        let merge_placeholder_selectors = upstream_options["env"]
            .as_str()
            .is_some_and(|environment| environment.eq_ignore_ascii_case("modern"))
            || upstream_options["overrideBrowserslist"]
                .as_str()
                .is_some_and(|targets| targets.eq_ignore_ascii_case("Chrome 58"));
        let use_hex_alpha_colors = upstream_options["env"]
            .as_str()
            .is_some_and(|environment| environment.eq_ignore_ascii_case("modern"))
            || upstream_options["overrideBrowserslist"]
                .as_str()
                .is_some_and(|targets| targets.eq_ignore_ascii_case("Chrome 62"));
        let options = MinifyOptions {
            // CSSNano's default preset disables absolute length-unit conversion;
            // the standalone postcss-convert-values corpus exercises it enabled.
            convert_length_units: plugin == "postcss-convert-values"
                && upstream_options["length"].as_bool() != Some(false),
            convert_extended_length_units: plugin != "postcss-convert-values",
            length_precision: upstream_options["precision"]
                .as_u64()
                .and_then(|precision| u8::try_from(precision).ok()),
            calc_precision: (plugin == "cssnano").then_some(5),
            preserve_variable_fallback_space: plugin == "cssnano",
            convert_zero_percentages: plugin != "cssnano"
                && (plugin != "postcss-convert-values"
                    || upstream_options["env"]
                        .as_str()
                        .is_some_and(|environment| environment.eq_ignore_ascii_case("modern"))),
            discard_license_comments: upstream_options["removeAll"].as_bool() == Some(true),
            discard_empty_keyframes: plugin == "postcss-convert-values"
                || plugin.starts_with("cssnano"),
            discard_unused_keyframes: plugin == "postcss-discard-unused"
                && upstream_options["keyframes"].as_bool() != Some(false),
            discard_overridden_keyframes: plugin.starts_with("cssnano"),
            discard_unused_counter_styles: plugin == "postcss-discard-unused"
                && upstream_options["counterStyle"].as_bool() != Some(false),
            discard_unused_font_faces: plugin == "postcss-discard-unused"
                && upstream_options["fontFace"].as_bool() != Some(false),
            discard_unused_namespaces: plugin == "postcss-discard-unused"
                && upstream_options["namespace"].as_bool() != Some(false),
            merge_identical_identifiers: plugin == "postcss-merge-idents",
            normalize_urls: plugin == "postcss-normalize-url" || plugin.starts_with("cssnano"),
            order_values: plugin == "postcss-ordered-values"
                || plugin == "postcss-discard-unused"
                || plugin == "postcss-merge-idents"
                || plugin == "postcss-reduce-idents"
                || plugin.starts_with("cssnano"),
            sort_declarations: plugin == "cssnano-preset-advanced",
            discard_obsolete_prefixes: plugin == "cssnano-preset-advanced",
            browser_hack_target,
            order_border_values_with_variables: plugin == "postcss-merge-longhand",
            sort_selectors: plugin == "postcss-minify-selectors"
                || plugin == "postcss-unique-selectors"
                || plugin == "postcss-merge-rules"
                || plugin == "postcss-discard-duplicates"
                || plugin.starts_with("cssnano"),
            merge_selectors: selector_targets_support_is,
            sort_selector_merges: upstream_options["sort"].as_bool() != Some(false),
            reduce_to_initial: plugin == "postcss-reduce-initial" && !targets_legacy_browsers,
            reduce_z_indices: plugin == "postcss-zindex",
            reduce_keyframe_identifiers: plugin == "postcss-reduce-idents"
                && upstream_options["keyframes"].as_bool() != Some(false),
            reduce_counter_style_identifiers: plugin == "postcss-reduce-idents"
                && upstream_options["counterStyle"].as_bool() != Some(false),
            reduce_counter_identifiers: plugin == "postcss-reduce-idents"
                && upstream_options["counter"].as_bool() != Some(false),
            reduce_grid_identifiers: plugin == "postcss-reduce-idents"
                && upstream_options["gridTemplate"].as_bool() != Some(false),
            z_index_start,
            merge_placeholder_selectors,
            use_hex_alpha_colors,
            transform_custom_properties: plugin != "postcss-convert-values"
                || upstream_options["transformCustomProperties"].as_bool() == Some(true),
            normalize_media_queries: !targets_legacy_browsers,
            keep_later_duplicate_declarations: plugin != "postcss-merge-rules",
            preserve_merged_box_initial: plugin == "postcss-merge-longhand",
            ..MinifyOptions::default()
        };
        minify(&mut stylesheet, options);
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

    eprintln!(
        "CSSNano corpus summary: {selected} selected, {executed} executed, {} passed, {} skipped",
        executed - failure_count,
        selected - executed,
    );
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

fn is_browser_hack_parser_skip(name: &str) -> bool {
    let Some(ordinal) = browser_hack_ordinal(name) else {
        return false;
    };
    (3154..=3189).contains(&ordinal) || (3240..=3243).contains(&ordinal)
}

fn is_browser_hack_lexical_skip(name: &str) -> bool {
    let Some(ordinal) = browser_hack_ordinal(name) else {
        return false;
    };
    matches!(ordinal, 3137 | 3139 | 3141 | 3143 | 3145 | 3147 | 3149)
        || (3190..=3193).contains(&ordinal)
        || (3244..=3247).contains(&ordinal)
}

fn browser_hack_ordinal(name: &str) -> Option<usize> {
    name.strip_prefix("unknown:0/")?.parse().ok()
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
