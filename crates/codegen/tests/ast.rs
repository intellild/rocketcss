use rocketcss_ast::CssRulePayload;
use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::{Allocator, GhostToken};
use rocketcss_parser::{Compiler, ParserOptions};

fn assert_ast_codegen_parity(source: &str) {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let options = ParserOptions::default();
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, options)
            .unwrap();

        for prettify in [false, true] {
            let printer_options = PrinterOptions { prettify };
            let ast_output = ast
                .to_css_string(printer_options, &ToCssContext::new(&token))
                .unwrap();
            let reparsed = Compiler::new(&allocator)
                .parse(&ast_output, &mut token, options)
                .unwrap_or_else(|error| {
                    panic!(
                        "failed to reparse generated CSS for {source:?}: {error:?}\noutput: {ast_output}"
                    )
                });
            let reparsed_output = reparsed
                .to_css_string(printer_options, &ToCssContext::new(&token))
                .unwrap();
            assert_eq!(ast_output, reparsed_output, "source: {source}");
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
        assert_ast_codegen_parity(source);
    }
}

#[test]
fn streams_the_selector_value_published_by_an_ast_transaction() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let options = ParserOptions::default();
        let mut ast = Compiler::new(&allocator)
            .parse("a{x:1}b{x:2}", &mut token, options)
            .unwrap();
        let styles = ast
            .rules_in_source_order()
            .filter_map(|(id, rule)| {
                matches!(rule.payload(), CssRulePayload::Style(_)).then_some(id)
            })
            .collect::<std::vec::Vec<_>>();
        let replacement = match ast.rule(styles[1]).unwrap().payload() {
            CssRulePayload::Style(payload) => payload.selector_value,
            _ => unreachable!(),
        };
        ast.replace_rule_selector_value(styles[0], replacement)
            .unwrap();

        let actual = ast
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
        let mut ast = Compiler::new(&allocator)
            .parse("a{color:red}a{color:blue}", &mut token, options)
            .unwrap();
        let styles = ast
            .rules_in_source_order()
            .filter_map(|(id, rule)| {
                matches!(rule.payload(), CssRulePayload::Style(_)).then_some(id)
            })
            .collect::<std::vec::Vec<_>>();
        ast.merge_adjacent_rule_declaration_blocks(styles[0], styles[1])
            .unwrap();

        let actual = ast
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(actual, "a{color:red;color:blue}");
        assert_eq!(ast.validate_ast(), Ok(()));
    });
}

#[test]
fn ordinary_url_ranges_preserve_text_without_nano_or_storage_growth() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = "a{background-image:url(Assets/Icon.SVG?v=1#part);mask-image:url(路径.svg)}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            let output = ast
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(output, source);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn function_ranges_stream_losslessly_without_nano_or_storage_growth() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = "a{width:CALC(1px + 2px);--x:FuN(路径,VAR(--x, ));future-property:FuN(/*keep*/VAR(--y, ))}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            let output = ast
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(output, source);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn custom_media_types_preserve_text_and_qualifiers_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = "@media only Future媒体{a{color:red}}@media not Future媒体{b{color:blue}}@media (--自定义){c{color:red}}@media (FutureFeature:Value){d{color:blue}}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            assert_eq!(
                ast.to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
                source
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn keyframes_names_resolve_in_the_root_pool_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        for (source, expected) in [
            (
                "a{color:red}@keyframes 路径{from{opacity:0}}@keyframes \"路径\"{to{opacity:1}}",
                "a{color:red}@keyframes 路径{from{opacity:0}}@keyframes \"路径\"{to{opacity:1}}",
            ),
            (
                "a{color:red}@keyframes f\\61 de{from{opacity:0}}",
                "a{color:red}@keyframes fade{from{opacity:0}}",
            ),
        ] {
            let ast = Compiler::new(&allocator)
                .parse(source, &mut token, ParserOptions::default())
                .unwrap();
            let checkpoint = ast.node_checkpoint();
            let bytes = ast.string_pool().extra_len();
            for _ in 0..3 {
                assert_eq!(
                    ast.to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::new(&token)
                    )
                    .unwrap(),
                    expected
                );
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), bytes);
        }
    });
}

#[test]
fn font_feature_names_share_the_root_pool_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source =
            "a{color:red}@font-feature-values \"D\\65 mo Sans\",字体{@styleset{f\\65 ature:1 2}}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let expected = "a{color:red}@font-feature-values Demo Sans,字体{@styleset{feature:1 2}}";
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            assert_eq!(
                ast.to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
                expected
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn page_selector_ranges_preserve_names_and_pseudo_classes_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = "a{color:red}@page I\\6e voice:left{margin:1px}@page :right{margin:2px}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            assert_eq!(
                ast.to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
                "a{color:red}@page Invoice:left{margin:1px}@page :right{margin:2px}"
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn image_set_file_type_ranges_round_trip_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = "a{background-image:image-set(url(a) 2dpi type(\"image/路径\"),url(b) 3dpi type(\"\"),url(c) 4dpi)}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            assert_eq!(
                ast.to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
                source
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn animation_and_environment_ranges_preserve_text_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source =
            "@media (width<=env(ViewportCustom 2 4,10px)){a{animation-name:SlideIn,\"none\",路径}}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            let output = ast
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(output, source);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn ordinary_property_names_and_raw_values_remain_lossless_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = "a{FuTuRe-PROP:Fn(01.00PX,/*x*/'Y') !important;--THÈME:var(--X,0px);transition-property:opacity,FuTuRe-PROP}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            let output = ast
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            assert_eq!(output, source);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn import_and_nested_layer_ranges_preserve_text_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = "@import \"路径.css\";@import \"路径.css\" layer;@import \"路径.css\" layer(主题.组件);@layer 主题.组件,主题.组件;@layer 主题.组件{a{color:red}}@layer{b{color:blue}}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            assert_eq!(
                ast.to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
                source
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn rule_text_ranges_preserve_authored_text_without_nano() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = "/*! 许可 */@charset \"UTF-8\";@namespace svg \"路径\";@namespace \"默认\";@custom-media --窄屏 (width<10px);@container 面板 (width>1px){a{color:red}}@未来 数据;@counter-style 序号{system:cyclic;symbols:\"甲\"}@position-try --位置{left:0}@font-palette-values --调色{font-family:字体;base-palette:0}@property --间距{syntax:\"<length>\";inherits:false;initial-value:0px}";
        let ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            assert_eq!(
                ast.to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
                source
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
        assert_eq!(
            ast.license_comments()
                .iter()
                .map(|text| ast.str(*text))
                .collect::<std::vec::Vec<_>>(),
            ["! 许可 "]
        );
    });
}

#[test]
fn empty_typed_visits_preserve_string_storage_and_serialization() {
    use rocketcss_ast::{AstStr, Atom, DeclarationPayload, VisitMut, VisitMutContext, VisitorMut};

    #[derive(Default)]
    struct ObserveStrings {
        ordinary: usize,
        atoms: usize,
    }
    impl<'a, 'ghost> VisitorMut<'a, 'ghost> for ObserveStrings {
        fn visit_ast_str(&mut self, _: &mut AstStr<'a>, _: &mut VisitMutContext<'_, 'a, 'ghost>) {
            self.ordinary += 1;
        }
        fn visit_atom(&mut self, _: &mut Atom<'a>, _: &mut VisitMutContext<'_, 'a, 'ghost>) {
            self.atoms += 1;
        }
    }

    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = r#"a[data-X="Y"]{background:linear-gradient(red,blue);width:calc(1px + var(--x, ));content:"é";--x:FuN(/*keep*/url(路径.svg));future-prop:var(--x, 01PX);font-family:Demo,serif}"#;
        let mut ast = Compiler::new(&allocator)
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        let options = PrinterOptions { prettify: false };
        let expected = ast
            .to_css_string(options, &ToCssContext::new(&token))
            .unwrap();
        let rule = ast.first_rule_in_source().unwrap();
        let block = ast.rule(rule).unwrap().declaration_block().unwrap();
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        let interned = ast.string_pool().len();
        let mut visitor = ObserveStrings::default();
        for _ in 0..3 {
            ast.transform_selector_values_in(&allocator, |_, selectors, ast| {
                selectors.visit_mut(
                    &mut visitor,
                    &mut VisitMutContext::with_ast(&mut token, ast),
                );
            });
            ast.for_each_declaration_payload_mut_with_context(block, |_, payload, ast| {
                let DeclarationPayload::Property(declaration) = payload else {
                    panic!()
                };
                declaration.visit_mut(
                    &mut visitor,
                    &mut VisitMutContext::with_ast(&mut token, ast),
                );
            })
            .unwrap();
            assert_eq!(
                ast.to_css_string(options, &ToCssContext::new(&token))
                    .unwrap(),
                expected
            );
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), bytes);
            assert_eq!(ast.string_pool().len(), interned);
        }
        assert!(visitor.ordinary > 0);
        assert!(visitor.atoms > 0);
    });
}
