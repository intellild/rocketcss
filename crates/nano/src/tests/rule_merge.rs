use super::*;

#[test]
fn merges_adjacent_equal_selector_declaration_blocks() {
    assert_eq!(
        run("h1{color:red}h1{background:blue}"),
        "h1{color:red;background:#00f}"
    );
    assert_eq!(
        run("a{width:1px}a{height:2px}a{opacity:.5}"),
        "a{width:1px;height:2px;opacity:.5}"
    );
}

#[test]
fn factors_common_declarations_between_different_selectors() {
    assert_eq!(run("a{color:red}b{color:red}"), "a,b{color:red}");
    assert_eq!(
        run("a{color:red;margin:0}b{color:red;padding:0}"),
        "a{margin:0}a,b{color:red}b{padding:0}"
    );
    assert_eq!(
        run("a{width:1px;color:red}b{color:red}"),
        "a{width:1px}a,b{color:red}"
    );
    assert_eq!(
        run("a{color:red}b{width:1px;color:red}"),
        "a,b{color:red}b{width:1px}"
    );
}

#[test]
fn stabilizes_overlapping_partial_selector_candidates() {
    assert_eq!(
        run("a{color:red}b{color:red;width:1px}c{width:1px}"),
        "a,b{color:red}b,c{width:1px}"
    );
    assert_eq!(
        run("a{opacity:.5}b{opacity:.5}c{opacity:.5}d{opacity:.5}"),
        "a,b,c,d{opacity:.5}"
    );
    assert_eq!(run("a,b{opacity:.5}b,c{opacity:.5}"), "a,b,c{opacity:.5}");
    assert_eq!(
        run("a{opacity:.5}b{opacity:.5}a,b{opacity:.5}"),
        "a,b{opacity:.5}"
    );
}

#[test]
fn scheduler_propagates_work_between_s1_s2_and_s3() {
    // S3 creates a same-selector edge that must immediately return to S1.
    assert_eq!(
        run("a{color:red}b{color:red}a,b{width:1px}"),
        "a,b{color:red;width:1px}"
    );

    // The synthesized a,b occurrence must enter the existing non-adjacent S2
    // history without rebuilding declaration blocks from the AST.
    assert_eq!(
        run("a,b{width:1px}c{display:block}a{width:1px}b{width:1px}"),
        "c{display:block}a,b{width:1px}"
    );

    // S2 removes the first b rule and exposes an a/c edge for S3.
    assert_eq!(
        run("a{color:red}b{display:block}c{color:red;width:1px}b{display:block}"),
        "a,c{color:red}c{width:1px}b{display:block}"
    );
}

#[test]
fn synthesized_history_occurrence_keeps_semantic_source_order() {
    assert_eq!(
        run("a,b{opacity:.5}@layer first;a{opacity:.5}b{opacity:.5}@layer second;a,b{opacity:.5}"),
        "@layer first;@layer second;a,b{opacity:.5}"
    );
}

#[test]
fn rejects_unproven_partial_selector_candidates() {
    assert_eq!(
        run("a{color:red!important}b{color:red}"),
        "a{color:red !important}b{color:red}"
    );
    assert_eq!(
        run("a{color:red}b{color:blue}"),
        "a{color:red}b{color:#00f}"
    );
    assert_eq!(
        run("a:has(.x){color:red}b{color:red}"),
        "a:has(.x){color:red}b{color:red}"
    );
    assert_eq!(
        run("code::-webkit-selection{color:red}code::-moz-selection{color:red}"),
        "code::-webkit-selection{color:red}code::-moz-selection{color:red}"
    );
    assert_eq!(
        run("a{display:-webkit-box;display:flex}b{display:flex;display:-webkit-box}"),
        "a{display:-webkit-box;display:flex}b{display:flex;display:-webkit-box}"
    );
    assert_eq!(
        run("a{margin:var(--m);margin-left:1px}b{margin:var(--m);color:red}"),
        "a{margin:var(--m);margin-left:1px}b{margin:var(--m);color:red}"
    );
    assert_eq!(
        run("a{font-size:1rem;font:serif}b{font-size:1rem;color:red}"),
        "a{font-size:1rem;font:serif}b{font-size:1rem;color:red}"
    );
    assert_eq!(
        run("a:hover{color:red}b:focus-visible{color:red}"),
        "a:hover{color:red}b:focus-visible{color:red}"
    );
}

#[test]
fn selector_compatibility_guard_is_configurable() {
    let mut options = MinifyOptions::default();
    options
        .flags
        .remove(Options::PRESERVE_SELECTOR_COMPATIBILITY);

    assert_eq!(
        run_with_options("a:hover{color:red}b:focus-visible{color:red}", options),
        "a:hover,b:focus-visible{color:red}"
    );
    assert_eq!(
        run_with_options("div{color:red}a>b{color:red}", options),
        "div,a>b{color:red}"
    );
    assert_eq!(
        run_with_options(
            "code::-webkit-selection{color:red}code::-moz-selection{color:red}",
            options,
        ),
        "code::-webkit-selection{color:red}code::-moz-selection{color:red}"
    );
}

#[test]
fn merges_selectors_with_the_same_compatibility_features() {
    assert_eq!(
        run("a:hover{color:red}b:hover{color:red}"),
        "a:hover,b:hover{color:red}"
    );
    assert_eq!(
        run("a[href]{color:red}b[href]{color:red}"),
        "a[href],b[href]{color:red}"
    );
    assert_eq!(
        run("a:has(.x){color:red}b:has(.y){color:red}"),
        "a:has(.x),b:has(.y){color:red}"
    );
}

#[test]
fn reorders_only_proven_independent_common_effects() {
    assert_eq!(
        run("a{color:red;width:2px}b{width:2px;color:red}"),
        "a,b{color:red;width:2px}"
    );
}

#[test]
fn factors_only_within_one_rule_list_and_preserves_nested_children() {
    assert_eq!(
        run("@media print{a{color:red}b{color:red}}"),
        "@media print{a,b{color:red}}"
    );
    assert_eq!(
        run("a{color:red}b{color:red;&:hover{color:blue}}"),
        "a,b{color:red}b{&:hover{color:#00f}}"
    );
    assert_eq!(
        run("a{color:red;&:hover{color:blue}}b{color:red}"),
        "a{color:red;&:hover{color:#00f}}b{color:red}"
    );
    assert_eq!(
        run("@media print{a{color:red}}@media print{b{color:red}}"),
        "@media print{a{color:red}}@media print{b{color:red}}"
    );
    assert_eq!(
        run("@layer theme{a{color:red}}@layer theme{b{color:red}}"),
        "@layer theme{a{color:red}}@layer theme{b{color:red}}"
    );
}

#[test]
fn factors_within_each_supported_condition_context_only() {
    assert_eq!(
        run("@supports (display:grid){a{opacity:.5}b{opacity:.5}}"),
        "@supports (display:grid){a,b{opacity:.5}}"
    );
    assert_eq!(
        run("@container (width>1px){a{opacity:.5}b{opacity:.5}}"),
        "@container (width>1px){a,b{opacity:.5}}"
    );
    assert_eq!(
        run("@scope (.root){a{opacity:.5}b{opacity:.5}}"),
        "@scope (.root){a,b{opacity:.5}}"
    );
    assert_eq!(
        run("@starting-style{a{opacity:.5}b{opacity:.5}}"),
        "@starting-style{a,b{opacity:.5}}"
    );

    // Structurally separate wrappers remain isolated even when their textual
    // prelude is identical. This branch does not model at-rule equivalence.
    assert_eq!(
        run("@scope (.root){a{x:1}}@scope (.root){b{x:1}}"),
        "@scope (.root){a{x:1}}@scope (.root){b{x:1}}"
    );
}

#[test]
fn merges_only_inside_the_current_sibling_scope() {
    assert_eq!(
        run("a{color:red}b{display:block}a{color:blue}"),
        "a{color:red}b{display:block}a{color:#00f}"
    );
    assert_eq!(
        run("@media print{a{color:red}a{background:blue}}"),
        "@media print{a{color:red;background:#00f}}"
    );
}

#[test]
fn adjacent_rule_merging_is_configurable() {
    let mut options = MinifyOptions::default();
    options.flags.remove(Options::MERGE_ADJACENT_RULES);

    assert_eq!(
        run_with_options("a{color:red}a{background:blue}", options),
        "a{color:red}a{background:#00f}"
    );
    assert_eq!(
        run_with_options("a{color:red;&{x:1}color:blue}", options),
        "a{color:red;&{x:1}color:#00f}"
    );
    assert_eq!(
        run_with_options("a{width:1px}@layer barrier;a{width:1px}", options),
        "a{width:1px}@layer barrier;a{width:1px}"
    );
    assert_eq!(
        run_with_options("a{color:red}b{color:red}", options),
        "a{color:red}b{color:red}"
    );
}

#[test]
fn adjacent_rule_merging_is_idempotent() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            "a{width:1px}a{height:2px}a{opacity:.5}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();

        minify(&mut stylesheet, &mut token, MinifyOptions::default());
        let once = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        let second_stats = minify(&mut stylesheet, &mut token, MinifyOptions::default());
        let twice = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();

        assert_eq!(once, "a{width:1px;height:2px;opacity:.5}");
        assert_eq!(twice, once);
        assert_eq!(second_stats.declarations_removed, 0);
    });
}

#[test]
fn partial_factoring_imports_s1_history_and_is_idempotent() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            "a{color:red}a{margin:0}b{color:red;padding:0}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();

        minify(&mut stylesheet, &mut token, MinifyOptions::default());
        let once = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        let second_stats = minify(&mut stylesheet, &mut token, MinifyOptions::default());
        let twice = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();

        assert_eq!(once, "a{margin:0}a,b{color:red}b{padding:0}");
        assert_eq!(twice, once);
        assert_eq!(second_stats.declarations_removed, 0);
    });
}

#[test]
fn respects_nested_content_as_a_forward_merge_barrier() {
    assert_eq!(
        run(".a{color:red;& .child{display:block}}.a{color:blue}"),
        ".a{color:red;& .child{display:block}}.a{color:#00f}"
    );
    assert_eq!(
        run(".a{color:red}.a{color:blue;& .child{display:block}}"),
        ".a{color:red;color:#00f;& .child{display:block}}"
    );
    assert_eq!(
        run(".a{color:red;& .child{display:block};color:green}.a{color:blue}"),
        ".a{color:red;& .child{display:block}color:green}.a{color:#00f}"
    );
}

#[test]
fn retained_rule_boundaries_are_not_adjacent_style_rules() {
    assert_eq!(
        run("a{color:red}@layer utilities;a{background:blue}"),
        "a{color:red}@layer utilities;a{background:#00f}"
    );
    assert_eq!(
        run("@media print{a{color:red}}a{background:blue}"),
        "@media print{a{color:red}}a{background:#00f}"
    );
    assert_eq!(
        run(".a{&{color:red}}.a{background:blue}"),
        ".a{&{color:red}}.a{background:#00f}"
    );
}
