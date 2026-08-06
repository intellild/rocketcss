use super::*;
use rocketcss_parser::Compiler;

fn run_radix(source: &str) -> String {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        try_minify(&mut stylesheet, &mut token, MinifyOptions::default()).unwrap();
        assert_eq!(stylesheet.validate_ast(), Ok(()));
        stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap()
    })
}

fn assert_radix_minify_parity(source: &str) {
    assert_eq!(run_radix(source), run(source), "source: {source}");
}

#[test]
fn radix_local_declaration_minify_preserves_pipeline_output() {
    for source in [
        "a{color:rgb(255,0,0);color:red}",
        "a{margin-top:0;margin-right:0;margin-bottom:0;margin-left:0}",
        "a{display:-webkit-box;display:flex;display:flex}",
        "a{columns:auto auto;width:0px;opacity:0.50}",
    ] {
        assert_radix_minify_parity(source);
    }
}

#[test]
fn radix_s1_s2_and_s3_preserve_pipeline_output() {
    for source in [
        "a{color:red}a{background:blue}",
        "*a{color:red}a{background:blue}",
        "a{color:red}b{display:block}a{color:red}",
        "a{color:red;margin:0}b{color:red;padding:0}",
        "a{color:red}b{color:red;width:1px}c{width:1px}",
    ] {
        assert_radix_minify_parity(source);
    }
}

#[test]
fn radix_minify_is_idempotent() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = Compiler::new(&allocator)
            .parse(
                "*a{color:rgb(255,0,0)}a{margin:0;color:red}b{margin:0}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
        try_minify(&mut stylesheet, &mut token, MinifyOptions::default()).unwrap();
        let once = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        try_minify(&mut stylesheet, &mut token, MinifyOptions::default()).unwrap();
        let twice = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();

        assert_eq!(twice, once);
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    });
}

#[test]
fn radix_nested_property_blocks_preserve_pipeline_output() {
    for source in [
        "@media print{a{color:red}b{color:red}}",
        "@supports (display:grid){a{opacity:.5}b{opacity:.5}}",
        "a{color:red;&:hover{color:blue}}b{color:red}",
        "a{color:red}b{color:red;&:hover{color:blue}}",
    ] {
        assert_radix_minify_parity(source);
    }
}

#[test]
fn radix_rule_local_values_and_descriptors_preserve_pipeline_output() {
    for source in [
        "a,a{color:red}",
        "*:first-child{color:red}",
        "@media screen,screen{a{color:red}}",
        "@keyframes fade{from{opacity:0}100%{opacity:1}}",
        "@foo x /* discard */ y;",
        "@property --accent{syntax:'<color>';inherits:false;initial-value:rgb(255,0,0)}",
    ] {
        assert_radix_minify_parity(source);
    }
}
