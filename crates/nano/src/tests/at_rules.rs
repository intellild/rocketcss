use super::*;

#[test]
fn preserves_rule_boundaries_while_merging_adjacent_styles() {
    assert_eq!(
        run("a{}a{color:red}a{color:red} @media print{a{}}"),
        "a{color:red}@media print{a{}}"
    );
    assert_eq!(
        run("@charset 'UTF-8'; @import 'theme.css'; a{color:red}"),
        "@charset \"UTF-8\";@import \"theme.css\";a{color:red}"
    );
}

#[test]
#[ignore]
fn preserves_unknown_units_in_media_queries() {
    assert_eq!(
        run("@media screen and (max-width:_1000customPx_){.test{color:red}}"),
        "@media screen and (max-width:_1000customPx_){.test{color:red}}"
    );
    assert_eq!(
        run("@media (max-width:1000customPx){.test{color:red}}"),
        "@media (max-width:1000customPx){.test{color:red}}"
    );
    assert_eq!(
        run("@media screen and (min-width:1020 px) and (max-width:739 px){.foo{color:red}}"),
        "@media screen and (min-width:1020 px) and (max-width:739 px){.foo{color:red}}"
    );
    assert_eq!(
        run(
            "@media (min-width:740px) and (max-width:1019px) and (min-width:1020px) and (max-width:1135px){.foo{color:red}}"
        ),
        "@media (width>=740px) and (width<=1019px) and (width>=765pt) and (width<=1135px){.foo{color:red}}"
    );
}

#[test]
#[ignore = "browser targets are not implemented yet"]
fn emits_safari_14_safe_zero_media_lengths() {
    assert_eq!(
        run("@media (min-width:0){a{color:red}}"),
        "@media (min-width:0px){a{color:red}}"
    );
}

#[test]
#[ignore = "provably false media query elimination is not implemented"]
fn removes_provably_false_media_queries() {
    assert_eq!(
        run(
            "@media (min-width:740px) and (max-width:1019px) and (min-width:1020px) and (max-width:1135px){.foo{color:red}}"
        ),
        ""
    );
}

#[test]
#[ignore]
fn canonicalizes_zero_legacy_media_features_to_safari_safe_ranges() {
    assert_eq!(
        run("@media (min-width:0){a{color:red}}"),
        "@media (width>=0){a{color:red}}"
    );
}

#[test]
#[ignore]
fn preserves_icss_export_syntax_without_module_semantics() {
    assert_eq!(run(":export{rowCount:4}"), ":export{rowCount:4}");
}

#[test]
#[ignore]
fn accepts_and_minifies_native_nested_rules_without_a_feature_flag() {
    assert_eq!(
        run("h1.b{color:red}h1{.b{color:red}}"),
        "h1.b{color:red}h1{.b{color:red}}"
    );
    assert_eq!(
        run(".top{sub{--prop:value}& .sub{--prop:value}}"),
        ".top{sub{--prop:value}& .sub{--prop:value}}"
    );
}

#[test]
#[ignore]
fn preserves_nested_layer_statement_and_block_order() {
    const SOURCE: &str = "@layer one,one.a,one.b;@layer one{@layer b{.test1{color:red}}}@layer one.a{.test1{color:green}}";
    assert_eq!(run(SOURCE), SOURCE);
}
