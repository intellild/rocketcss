use super::*;

#[test]
fn removes_exact_duplicate_declarations_within_one_block() {
    assert_eq!(
        run("h1{font-weight:700;font-weight:700}"),
        "h1{font-weight:700}"
    );
    assert_eq!(
        run("h1{font-weight:bold;font-weight:bold}"),
        "h1{font-weight:700}"
    );
    assert_eq!(
        run("h1{margin:10px 0 10px 0;margin:10px 0}"),
        "h1{margin:10px 0}"
    );
    assert_eq!(
        run("a{width:1px;color:red;width:1px}"),
        "a{color:red;width:1px}"
    );
    assert_eq!(
        run("a{width:1px!important;width:1px!important}"),
        "a{width:1px !important}"
    );
    assert_eq!(
        run("a{-webkit-user-select:none;-webkit-user-select:none}"),
        "a{-webkit-user-select:none}"
    );
    assert_eq!(run("a{--x:1;--x:1}"), "a{--x:1}");
    assert_eq!(
        run("a{unknown:value;unknown:value}"),
        "a{unknown:value;unknown:value}"
    );
    assert_eq!(
        run(
            ".aligncenter{clear:both;clear:both;clip:auto;clip:auto;margin-left:auto;margin-left:auto;margin-right:auto;margin-right:auto;display:block;display:block}"
        ),
        ".aligncenter{clear:both;clear:both;clip:auto;clip:auto;margin-left:auto;margin-right:auto;display:block}"
    );
    assert_eq!(
        run(
            "a{width:1px;height:1px;top:1px;right:1px;bottom:1px;left:1px;color:red;opacity:1;z-index:1;width:1px}"
        ),
        "a{height:1px;top:1px;right:1px;bottom:1px;left:1px;color:red;opacity:1;z-index:1;width:1px}"
    );
    assert_eq!(run("a{width:1px;width:1px;width:1px}"), "a{width:1px}");
    assert_eq!(
        run("a{height:1px;width:1px;width:1px;color:red}"),
        "a{height:1px;width:1px;color:red}"
    );
    GhostToken::scope(|mut token| {
        let mut stylesheet = parse(
            "a{width:1px;color:red;width:1px}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let stats = minify(&mut stylesheet, &mut token, MinifyOptions::default());
        let CssRule::Style(rule) = &stylesheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let declarations = stylesheet.declaration_block(rule.declarations);
        assert_eq!(declarations.len(), 2);
        assert_eq!(declarations.declarations_importance.len(), 2);
        assert!(
            declarations
                .declarations
                .iter()
                .all(|declaration| !matches!(declaration, Declaration::Tombstone))
        );
        assert_eq!(stats.declarations_removed, 1);

        let stats = minify(&mut stylesheet, &mut token, MinifyOptions::default());
        assert_eq!(stats.declarations_removed, 0);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{color:red;width:1px}"
        );
    });
}

#[test]
fn preserves_unresolved_functional_colors_as_history_barriers() {
    assert_eq!(
        run("a{color:red;color:rgb(foo)}"),
        "a{color:red;color:rgb(foo)}"
    );
    assert_eq!(
        run("a{color:red;color:RGB(0px 0 0)}"),
        "a{color:red;color:RGB(0px 0 0)}"
    );
    assert_eq!(
        run("a{color:red;color:rgb(0,,0,0);color:rgb(0/0/0)}"),
        "a{color:red;color:rgb(0,,0,0);color:rgb(0/0/0)}"
    );
    assert_eq!(
        run("a{color:rgb(0 0 0);color:blue}"),
        "a{color:#000;color:#00f}"
    );
}

#[test]
fn preserves_declaration_fallbacks_and_importance() {
    assert_eq!(
        run("a{width:1px;width:2px;width:1px}"),
        "a{width:1px;width:2px;width:1px}"
    );
    assert_eq!(
        run("a{width:1px;width:1px!important}"),
        "a{width:1px;width:1px !important}"
    );
    assert_eq!(
        run(
            ".foo{color:red;color:var(--my-red);background-color:blue;background-color:var(--my-blue)}"
        ),
        ".foo{color:red;color:var(--my-red);background-color:#00f;background-color:var(--my-blue)}"
    );
    assert_eq!(
        run("a{width:-webkit-fill-available;width:-moz-available;width:stretch}"),
        "a{width:-webkit-fill-available;width:-moz-available;width:stretch}"
    );
}

#[test]
fn preserves_cross_block_fallbacks_and_separates_cascade_phases() {
    assert_eq!(
        run("a{width:1px}@layer barrier-1;a{width:2px}@layer barrier-2;a{width:1px}"),
        "a{width:1px}@layer barrier-1;a{width:2px}@layer barrier-2;a{width:1px}"
    );
    assert_eq!(
        run("a{color:red!important}@layer barrier;a{color:blue}"),
        "a{color:red !important}@layer barrier;a{color:#00f}"
    );
}

#[test]
fn prunes_exact_effects_across_non_adjacent_blocks_in_one_history() {
    assert_eq!(run("a{width:1px}b{x:1}a{width:1px}"), "b{x:1}a{width:1px}");
}

#[test]
fn parent_declaration_segments_share_an_s2_history() {
    assert_eq!(
        run(".parent{color:red;.child{x:1}color:red}"),
        ".parent{.child{x:1}color:red}"
    );
}

#[test]
fn s2_requires_exactly_equal_conditional_contexts() {
    GhostToken::scope(|mut token| {
        let mut stylesheet = parse(
            "@media (width:1px){a{x:1}}@media (width:2px){a{x:1}}@media (width:1px){a{x:1}}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();

        minify(&mut stylesheet, &mut token, MinifyOptions::default());

        let blocks = crate::cross_rule_declaration_merging::discovery::discover_for_test(
            stylesheet.rule_store(),
        );
        assert_eq!(blocks.len(), 2);
        let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
        assert_ne!(blocks[0].effective_key, blocks[1].effective_key);
        assert!(
            !stylesheet
                .declaration_block(blocks[0].declarations)
                .declarations[0]
                .is_tombstone()
        );
        assert!(
            !stylesheet
                .declaration_block(blocks[1].declarations)
                .declarations[0]
                .is_tombstone()
        );
    });
}

#[test]
fn s2_emptying_a_rule_exposes_a_new_s1_edge() {
    assert_eq!(
        run("a{x:1}b{y:1}a{z:1}b{y:1}a{w:1}"),
        "a{x:1;z:1}b{y:1}a{w:1}"
    );
}

#[test]
fn live_graph_prepends_into_an_existing_s1_sequence() {
    assert_eq!(
        run("a{x:1}b{q:1}a{y:1}a{z:1}b{q:1}"),
        "a{x:1;y:1;z:1}b{q:1}"
    );
}

#[test]
fn live_graph_removes_multiple_s2_barriers_before_stabilizing_s1() {
    assert_eq!(
        run("a{x:1}b{q:1}c{r:1}a{y:1}b{q:1}c{r:1}"),
        "a{x:1;y:1}b{q:1}c{r:1}"
    );
}
