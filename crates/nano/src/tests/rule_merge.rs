use super::*;

#[test]
fn merges_adjacent_equal_selector_declaration_blocks() {
    assert_eq!(
        run("h1{color:red;background:blue}h1{color:red}"),
        "h1{background:#00f;color:red}"
    );
    assert_eq!(
        run("a{width:1px}a{height:2px}a{opacity:.5}"),
        "a{width:1px;height:2px;opacity:.5}"
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
}

#[test]
fn adjacent_rule_merging_is_idempotent() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            "a{width:1px}a{height:2px}a{width:1px}",
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

        assert_eq!(once, "a{height:2px;width:1px}");
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
