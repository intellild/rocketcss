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
    assert_eq!(run("a{unknown:value;unknown:value}"), "a{unknown:value}");
    assert_eq!(
        run(
            ".aligncenter{clear:both;clear:both;clip:auto;clip:auto;margin-left:auto;margin-left:auto;margin-right:auto;margin-right:auto;display:block;display:block}"
        ),
        ".aligncenter{clear:both;clip:auto;margin-left:auto;margin-right:auto;display:block}"
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

    let allocator = Allocator::new();
    let mut stylesheet = parse(
        "a{width:1px;color:red;width:1px}",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let stats = minify(&mut stylesheet, MinifyOptions::default());
    let CssRule::Style(rule) = &stylesheet.rules[0] else {
        panic!("expected style rule")
    };
    assert_eq!(rule.declarations.len(), 3);
    assert_eq!(rule.declarations.declarations_importance.len(), 3);
    assert!(matches!(
        rule.declarations.declarations[0],
        Declaration::Tombstone
    ));
    assert_eq!(stats.declarations_removed, 1);

    let stats = minify(&mut stylesheet, MinifyOptions::default());
    assert_eq!(stats.declarations_removed, 0);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{color:red;width:1px}"
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
fn preserves_cross_block_fallbacks_and_removes_overridden_normal_values() {
    assert_eq!(
        run("a{width:1px}a{width:2px}a{width:1px}"),
        "a{width:1px;width:2px;width:1px}"
    );
    assert_eq!(
        run("a{color:red!important}a{color:blue}"),
        "a{color:red !important}"
    );
}
