use super::*;

#[test]
fn removes_unparsed_selectors_from_mixed_selector_lists() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            ".valid, (font-[family-name:var(--font-*)]), #also-valid { color: red }",
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        let stats = minify(&mut stylesheet, &mut token, MinifyOptions::default());
        let CssRule::Style(rule) = &stylesheet.rules[0] else {
            panic!("expected style rule")
        };
        let rule = rule.get(&token);
        let rule = rule.get_ref();
        assert!(matches!(rule.selectors[0], Selector::Parsed(_)));
        assert!(matches!(rule.selectors[1], Selector::Tombstone));
        assert!(matches!(rule.selectors[2], Selector::Parsed(_)));
        assert_eq!(stats.values_normalized, 1);
        assert_eq!(
            stylesheet
                .to_css_string(&token, PrinterOptions { prettify: false })
                .unwrap(),
            ".valid,#also-valid{color:red}"
        );
    });
}

#[test]
#[ignore = "unparsed selectors must remain an unforgiving-list barrier"]
fn preserves_unparsed_selector_list_barriers() {
    assert_eq!(
        run_with_error_recovery(".valid,(font-[family-name:var(--font-*)]),#also-valid{color:red}"),
        ".valid,(font-[family-name:var(--font-*)]),#also-valid{color:red}"
    );
}

#[test]
fn removes_style_rules_containing_only_unparsed_selectors() {
    assert_eq!(
        run_with_error_recovery("(font-[family-name:var(--font-*)]) { color: red }"),
        ""
    );
}

#[test]
fn deduplicates_selectors_with_structural_hashes() {
    assert_eq!(run("h1,h2,h1,h2{color:red}"), "h1,h2{color:red}");
    assert_eq!(
        run(".foo,.bar:baz{color:green}"),
        ".foo,.bar:baz{color:green}"
    );
    assert_eq!(
        run("a:custom(1),b,a:custom(1),a:custom(2),b{color:red}"),
        "a:custom(1),b,a:custom(2){color:red}"
    );
    assert_eq!(
        run("a:custom(0),b,a:custom(-0),c,d{color:red}"),
        "a:custom(0),b,c,d{color:red}"
    );
    assert_eq!(
        run("a:is(.x,.x,.y),a:is(.x,.x,.y){color:red}"),
        "a:is(.x,.x,.y){color:red}"
    );
    assert_eq!(run("a{&,&{color:red}}"), "a{&{color:red}}");
}

#[test]
#[ignore]
fn preserves_rules_with_long_child_selectors() {
    assert_eq!(
        run(".depict.plp .filters .body .input-row > .left{align-items:center;display:flex}"),
        ".depict.plp .filters .body .input-row>.left{align-items:center;display:flex}"
    );
}

#[test]
fn selector_deduplication_is_configurable() {
    let mut options = MinifyOptions::default();
    options.flags.remove(Options::DEDUPLICATE_LISTS);
    assert_eq!(
        run_with_options("h1,h2,h1,h2{color:red}", options),
        "h1,h2,h1,h2{color:red}"
    );
}

#[test]
#[ignore]
fn preserves_pseudo_elements_inside_where_instead_of_emptying_it() {
    assert_eq!(
        run(".language-diff :where(.inserted::before){content:'+'}"),
        ".language-diff :where(.inserted:before){content:\"+\"}"
    );
}

#[test]
#[ignore = "cross-rule selector merging and selector-support proofs are not implemented"]
fn does_not_merge_adjacent_rules_through_forgiving_selector_wrappers() {
    const SOURCE: &str = "a{color:blue}:unknown{color:blue}";
    assert_eq!(run(SOURCE), SOURCE);
    assert!(!run(SOURCE).contains(":is("));
    assert!(!run(SOURCE).contains(":where("));
}

#[test]
#[ignore]
fn preserves_has_slotted_pseudo_class_through_minification() {
    assert_eq!(
        run("slot:has-slotted{display:none}"),
        "slot:has-slotted{display:none}"
    );
}
