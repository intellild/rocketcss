use rocketcss_ast_next::{
    CssRule, CssRulePayloadData, DeclarationBlockData, SelectorData, StyleRuleData,
};
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
                .to_css_string(printer_options, &ToCssContext::new(&token, radix.ast()))
                .unwrap();
            let reparsed = Compiler::new(&allocator)
                .parse(&radix_output, &mut token, options)
                .unwrap_or_else(|error| {
                    panic!(
                        "failed to reparse generated CSS for {source:?}: {error:?}\noutput: {radix_output}"
                    )
                });
            let reparsed_output = reparsed
                .to_css_string(printer_options, &ToCssContext::new(&token, reparsed.ast()))
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
        let styles = style_rules(&radix);
        let replacement = match styles[1].payload(radix.ast()).data(radix.ast()) {
            CssRulePayloadData::Style(payload) => payload.selectors(radix.ast()),
            _ => unreachable!(),
        };
        let style = match styles[0].payload(radix.ast()).data(radix.ast()) {
            CssRulePayloadData::Style(payload) => payload,
            _ => unreachable!(),
        };
        let vendor_prefix = style.vendor_prefix(radix.ast());
        radix.ast_mut().set_data(
            style,
            StyleRuleData {
                selectors: replacement,
                vendor_prefix,
            },
        );

        let actual = radix
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token, radix.ast()),
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
        let styles = style_rules(&radix);
        let first_block = styles[0].declarations(radix.ast()).unwrap();
        let second_block = styles[1].declarations(radix.ast()).unwrap();
        let (merged, merged_important) = {
            let ast = radix.ast();
            let mut declarations = first_block.declarations(ast).to_vec();
            declarations.extend_from_slice(second_block.declarations(ast));
            let mut important = first_block.important(ast).to_vec();
            important.extend_from_slice(second_block.important(ast));
            (ast.alloc_slice(&declarations), ast.alloc_slice(&important))
        };
        radix.ast_mut().set_data(
            first_block,
            DeclarationBlockData {
                declarations: merged,
                important: merged_important,
            },
        );

        let tombstone = radix
            .ast_mut()
            .alloc_selector(SelectorData::Tombstone, Default::default());
        let tombstone_selectors = radix.ast().alloc_slice(&[tombstone]);
        let second_style = match styles[1].payload(radix.ast()).data(radix.ast()) {
            CssRulePayloadData::Style(payload) => payload,
            _ => unreachable!(),
        };
        let vendor_prefix = second_style.vendor_prefix(radix.ast());
        radix.ast_mut().set_data(
            second_style,
            StyleRuleData {
                selectors: tombstone_selectors,
                vendor_prefix,
            },
        );

        let actual = radix
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token, radix.ast()),
            )
            .unwrap();
        assert_eq!(actual, "a{color:red;color:blue}");
    });
}

fn style_rules<'ast>(compilation: &rocketcss_ast_next::Compilation<'ast>) -> Vec<CssRule<'ast>> {
    let ast = compilation.ast();
    compilation
        .style_sheet()
        .rules(ast)
        .iter()
        .copied()
        .filter(|rule| matches!(rule.payload(ast).data(ast), CssRulePayloadData::Style(_)))
        .collect()
}
