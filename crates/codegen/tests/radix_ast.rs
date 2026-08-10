use rocketcss_ast::CssRulePayload;
use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::{Allocator, GhostToken};
use rocketcss_parser::{Compiler, ParserOptions};

fn assert_radix_codegen_parity(source: &str) {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let options = ParserOptions::default();
        let radix = Compiler::new(&allocator)
            .parse(source, &mut token, options)
            .unwrap();

        for prettify in [false, true] {
            let printer_options = PrinterOptions { prettify };
            let radix_output = radix
                .to_css_string(printer_options, &ToCssContext::new(&token))
                .unwrap();
            let reparsed = Compiler::new(&allocator)
                .parse(&radix_output, &mut token, options)
                .unwrap_or_else(|error| {
                    panic!(
                        "failed to reparse generated CSS for {source:?}: {error:?}\noutput: {radix_output}"
                    )
                });
            let reparsed_output = reparsed
                .to_css_string(printer_options, &ToCssContext::new(&token))
                .unwrap();
            assert_eq!(radix_output, reparsed_output, "source: {source}");
        }
    });
}

#[test]
fn streams_flat_rules_and_declarations_without_reification() {
    for source in [
        "/*!keep*/@charset 'UTF-8';@layer reset,theme;@import url(a.css) layer(theme) screen;@namespace svg url(http://www.w3.org/2000/svg);@custom-media --narrow (width < 30em);a{}",
        "a{color:red;& b{margin:0;padding:1px}color:blue}c{display:block}",
        "@supports (display:grid){a{color:red}}b{@starting-style{opacity:0;&:hover{opacity:.5}}opacity:1}",
        "@layer app{a{color:red}}@container card (width>1px){@scope (.card) to (.end){b{color:blue}}}@-moz-document url-prefix(){c{display:block}}",
        "@foo screen and (x:y);a{color:red;@bar one{two:3;nested(x)}color:blue}",
        "@counter-style marker{system:cyclic;symbols:'x'}@viewport{width:device-width}@position-try --fallback{top:0;left:1px}",
        "@font-face{font-family:Demo;src:url(demo.woff2);unicode-range:U+0-7F}@font-palette-values --theme{font-family:Demo;base-palette:1;override-colors:0 red}@view-transition{navigation:auto;types:foo bar}",
        "@-webkit-keyframes fade{from{opacity:0}50%,to{opacity:1;transform:none}}",
        "@page invoice:left{size:A4;@top-left{content:'x'}margin:0;@bottom-right{content:counter(page)}color:red}",
        "a{@nest & .b{color:red;@media (width>1px){color:green}}color:blue}",
        "@font-feature-values 'Demo'{@styleset{nice:1 2;alt:3}@swash{fancy:4}}",
        "@property --space{syntax:'<length>';unknown:foo;syntax:'*';inherits:false;initial-value:10px}",
    ] {
        assert_radix_codegen_parity(source);
    }
}

#[test]
fn streams_the_selector_value_published_by_an_ast_transaction() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let options = ParserOptions::default();
        let mut radix = Compiler::new(&allocator)
            .parse("a{x:1}b{x:2}", &mut token, options)
            .unwrap();
        let styles = radix
            .rules_in_source_order()
            .filter_map(|(id, rule)| {
                matches!(rule.payload(), CssRulePayload::Style(_)).then_some(id)
            })
            .collect::<std::vec::Vec<_>>();
        let replacement = match radix.rule(styles[1]).unwrap().payload() {
            CssRulePayload::Style(payload) => payload.selector_value,
            _ => unreachable!(),
        };
        radix
            .replace_rule_selector_value(styles[0], replacement)
            .unwrap();

        let actual = radix
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(actual, "b{x:1}b{x:2}");
    });
}

#[test]
fn streams_an_adjacent_block_merge_directly_from_live_topology() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let options = ParserOptions::default();
        let mut radix = Compiler::new(&allocator)
            .parse("a{color:red}a{color:blue}", &mut token, options)
            .unwrap();
        let styles = radix
            .rules_in_source_order()
            .filter_map(|(id, rule)| {
                matches!(rule.payload(), CssRulePayload::Style(_)).then_some(id)
            })
            .collect::<std::vec::Vec<_>>();
        radix
            .merge_adjacent_rule_declaration_blocks(styles[0], styles[1])
            .unwrap();

        let actual = radix
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(actual, "a{color:red;color:#00f}");
        assert_eq!(radix.validate_ast(), Ok(()));
    });
}
