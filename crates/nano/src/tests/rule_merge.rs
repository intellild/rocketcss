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
}

#[test]
fn adjacent_rule_merging_is_idempotent() {
    GhostToken::scope(|mut token| {
        let mut stylesheet = parse(
            "a{width:1px}a{height:2px}a{opacity:.5}",
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
fn terminal_reification_compacts_rules_selectors_and_declarations() {
    GhostToken::scope(|mut token| {
        let mut stylesheet = parse(
            "a{width:1px}a{height:2px}b{x:1}b{x:1}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();

        minify(&mut stylesheet, &mut token, MinifyOptions::default());
        stylesheet.validate_flat_ir().unwrap();
        assert_eq!(stylesheet.rule_store().len(), 2);
        assert_eq!(stylesheet.selector_slots().count(), 2);
        assert_eq!(stylesheet.declaration_slots().count(), 3);

        for rule in stylesheet.root_rules().iter() {
            let CssRule::Style(style) = rule else {
                panic!("expected compacted style rule")
            };
            let block = stylesheet.declaration_block(style.declarations);
            assert_eq!(block.ranges().len(), 1);
            assert!(block.effective_key().is_some());
            assert!(
                block
                    .declarations
                    .iter()
                    .all(|declaration| !declaration.is_tombstone())
            );
            assert!(
                stylesheet
                    .selectors(style.selectors)
                    .iter()
                    .all(|selector| !selector.is_tombstone())
            );
        }
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
