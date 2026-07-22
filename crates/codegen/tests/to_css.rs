use rocketcss_allocator::Ref;
use rocketcss_codegen::{Printer, PrinterOptions, ToCss};
use rocketcss_parser::prelude::*;

fn parse_stylesheet(source: &str) -> StyleSheet<'static> {
    let source = std::boxed::Box::leak(source.to_owned().into_boxed_str());
    let allocator = std::boxed::Box::leak(std::boxed::Box::new(Allocator::new()));
    parse(source, allocator, ParserOptions::default()).unwrap()
}
#[test]
#[ignore = "nested custom page regions are not represented in the AST yet"]
fn preserves_unknown_nested_page_regions() {
    const SOURCE: &str = "@page{@footnote{float:bottom}@prince-overlay{content:\"continued\"}}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
fn printer_remains_send_for_a_send_writer() {
    fn assert_send<T: Send>(_: T) {}

    let mut output = String::new();
    assert_send(Printer::new(&mut output, PrinterOptions::default()));
}

#[test]
fn ports_lightningcss_public_to_css_api_cases() {
    let stylesheet = parse_stylesheet(".foo { color: red }");
    let rule = &stylesheet.rules[0];
    assert_eq!(
        rule.to_css_string(PrinterOptions::default()).unwrap(),
        ".foo {\n  color: red;\n}"
    );

    let CssRule::Style(style) = rule else {
        panic!("expected a style rule")
    };
    assert_eq!(
        style.declarations.declarations[0]
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "color: red"
    );
    let stylesheet = parse_stylesheet("@media print{.a{color:red}.b{display:block}}");
    let CssRule::Media(media) = &stylesheet.rules[0] else {
        panic!("expected a media rule")
    };
    assert_eq!(
        media.rules[0]
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        ".a{color:red}"
    );
}

#[test]
fn stylesheet_implements_to_css() {
    let stylesheet = parse_stylesheet(
        ".foo { color: green }\n.bar { color: red; background: pink }\n@media print { .baz { color: green } }",
    );
    assert_eq!(
        stylesheet.to_css_string(PrinterOptions::default()).unwrap(),
        concat!(
            ".foo {\n",
            "  color: green;\n",
            "}\n\n",
            ".bar {\n",
            "  color: red;\n",
            "  background: pink;\n",
            "}\n\n",
            "@media print {\n",
            "  .baz {\n",
            "    color: green;\n",
            "  }\n",
            "}\n"
        )
    );
}

#[test]
#[ignore]
fn supports_conditions_preserve_source_order_deterministically() {
    const SOURCE: &str = "@supports ((foo: bar) or (color: red)) { .a { color: green } }";
    const EXPECTED: &str = "@supports ((foo: bar) or (color: red)){.a{color:green}}";

    for _ in 0..32 {
        let stylesheet = parse_stylesheet(SOURCE);
        let CssRule::Supports(rule) = &stylesheet.rules[0] else {
            panic!("expected a supports rule")
        };

        assert_eq!(
            rule.condition.as_ref(),
            &SupportsCondition::Unknown("((foo: bar) or (color: red))")
        );
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            EXPECTED
        );
    }
}

#[test]
#[ignore]
fn preserves_nonstandard_yahoo_media_query_prelude() {
    let stylesheet = parse_stylesheet("@media screen yahoo { .a { color: red } }");
    let CssRule::Media(rule) = &stylesheet.rules[0] else {
        panic!("expected media rule")
    };
    let query = &rule.query.media_queries[0];
    assert!(matches!(query.media_type, MediaType::All));
    assert!(query.qualifier.is_none());
    assert!(matches!(
        query.condition.as_ref(),
        Some(MediaCondition::Unknown(_))
    ));
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "@media screen yahoo{.a{color:red}}"
    );
}

#[test]
#[ignore]
fn preserves_nonstandard_important_at_rule_as_unknown_syntax() {
    let stylesheet = parse_stylesheet("@important{.card{color:red}.a{color:black}}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "@important{.card{color:red}.a{color:black}}"
    );
}

#[test]
#[ignore]
fn pseudo_classes_are_debuggable_and_serializable() {
    for source in [
        ".foo:hover{color:red}",
        ".foo:disabled{color:red}",
        ".foo:first-child{color:red}",
    ] {
        let stylesheet = parse_stylesheet(source);
        let CssRule::Style(style) = &stylesheet.rules[0] else {
            panic!("expected style rule")
        };
        assert!(format!("{style:#?}").contains("StyleRule"));
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap(),
            source
        );
    }
}

#[test]
#[ignore]
fn preserves_keyframe_names_in_custom_properties_without_module_linking() {
    let stylesheet = parse_stylesheet(
        ".root{--animation-name:fade-in}@keyframes fade-in{from{opacity:0}to{opacity:1}}",
    );
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        ".root{--animation-name:fade-in}@keyframes fade-in{from{opacity:0}to{opacity:1}}"
    );
}

#[test]
#[ignore]
fn preserves_css_modules_import_syntax_without_compiling_it() {
    let stylesheet = parse_stylesheet(
        "@value button from \"./button.module.css\";:import(\"./button.module.css\"){button:button}",
    );
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "@value button from \"./button.module.css\";:import(\"./button.module.css\"){button:button}"
    );
}

#[test]
#[ignore = "CSS Modules file aliases are preserved but not resolved yet"]
fn preserves_css_modules_file_alias_syntax() {
    const SOURCE: &str = "@alias \"../../../../style/theme/colors.module.css\" as colors;.foobar{color:var(--primary from colors)}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore]
fn preserves_nested_layer_structure_until_lifting_is_implemented() {
    let stylesheet = parse_stylesheet(
        ".foo{@layer utilities{color:red}}.baz{@layer components{color:red}}.bar{@layer utilities{color:red}}",
    );
    for rule in &stylesheet.rules {
        let CssRule::Style(style) = rule else {
            panic!("expected style rule")
        };
        let CssRule::LayerBlock(layer) = &style.rules[0] else {
            panic!("expected nested layer block")
        };
        assert!(matches!(layer.rules[0], CssRule::NestedDeclarations(_)));
    }
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        ".foo{@layer utilities{color:red}}.baz{@layer components{color:red}}.bar{@layer utilities{color:red}}"
    );
}

#[test]
#[ignore]
fn box_sizing_css_wide_keywords_round_trip_as_known_unparsed_values() {
    let stylesheet = parse_stylesheet(
        "a{box-sizing:initial;box-sizing:inherit;box-sizing:unset;box-sizing:revert;box-sizing:revert-layer}",
    );
    let CssRule::Style(rule) = &stylesheet.rules[0] else {
        panic!("expected a style rule")
    };

    assert_eq!(rule.declarations.declarations.len(), 5);
    assert!(
        rule.declarations
            .declarations
            .iter()
            .all(|declaration| matches!(
                declaration,
                Declaration::Unparsed(value)
                    if matches!(
                        &*value.property_id,
                        PropertyId::BoxSizing(VendorPrefix::NONE)
                    )
            ))
    );
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{box-sizing:initial;box-sizing:inherit;box-sizing:unset;box-sizing:revert;box-sizing:revert-layer}"
    );
}

#[test]
fn compact_stylesheet_omits_optional_whitespace() {
    let stylesheet = parse_stylesheet(".foo { color: #ff00ff }");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        ".foo{color:#f0f}"
    );
}

#[test]
fn recovered_unparsed_selectors_round_trip_before_minification() {
    let allocator = Allocator::new();
    let stylesheet = parse(
        ".valid, (font-[family-name:var(--font-*)]), #also-valid { color: red }",
        &allocator,
        ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        },
    )
    .unwrap();

    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        ".valid,(font-[family-name:var(--font-*)]),#also-valid{color:red}"
    );
}

#[test]
#[ignore = "invalid declarations need a lossless raw AST representation"]
fn error_recovery_preserves_tailwind_wildcard_custom_properties() {
    const SOURCE: &str = ":root{--color-*:initial;color:red}";
    let allocator = Allocator::new();
    let stylesheet = parse(
        SOURCE,
        &allocator,
        ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        },
    )
    .unwrap();

    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
fn font_family_lists_skip_tombstones_without_extra_commas() {
    let allocator = Allocator::new();
    let mut families = allocator.vec();
    families.push(FontFamily::Tombstone);
    families.push(FontFamily::Custom("A"));
    families.push(FontFamily::Tombstone);
    families.push(FontFamily::Serif);
    families.push(FontFamily::Tombstone);

    assert_eq!(
        families
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "A,serif"
    );
}

#[test]
fn serializes_typed_multicol_and_legacy_gap_properties() {
    let stylesheet = parse_stylesheet(
        "a { -webkit-column-rule: red solid 1px; columns: 3 10px; grid-column-gap: 10%; grid-row-gap: normal }",
    );
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{-webkit-column-rule:1px solid red;columns:10px 3;grid-column-gap:10%;grid-row-gap:normal}"
    );
}

#[test]
fn serializes_charset_rules() {
    let stylesheet =
        parse_stylesheet("@charset 'UTF-8'; @import 'theme.css'; .foo { color: green }");

    assert_eq!(
        stylesheet.to_css_string(PrinterOptions::default()).unwrap(),
        concat!(
            "@charset \"UTF-8\";\n",
            "@import \"theme.css\";\n\n",
            ".foo {\n",
            "  color: green;\n",
            "}\n"
        )
    );
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "@charset \"UTF-8\";@import \"theme.css\";.foo{color:green}"
    );
}

#[test]
fn function_codegen_uses_known_identity_and_preserves_original_name() {
    let stylesheet = parse_stylesheet("a{color:VAR(--x,);width:CuStOm(1)}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{color:VAR(--x, );width:CuStOm(1)}"
    );
}

#[test]
fn serializes_packed_rgb_and_rgba_hex_values() {
    for (color, expected) in [
        (
            RGBA {
                red: 0xaa,
                green: 0xbb,
                blue: 0xcc,
                alpha: 0xff,
            },
            "#abc",
        ),
        (
            RGBA {
                red: 0x12,
                green: 0x34,
                blue: 0x56,
                alpha: 0xff,
            },
            "#123456",
        ),
        (
            RGBA {
                red: 0xaa,
                green: 0xbb,
                blue: 0xcc,
                alpha: 0xdd,
            },
            "#abcd",
        ),
        (
            RGBA {
                red: 0x12,
                green: 0x34,
                blue: 0x56,
                alpha: 0x78,
            },
            "#12345678",
        ),
    ] {
        assert_eq!(
            color.to_css_string(PrinterOptions::default()).unwrap(),
            expected
        );
    }
}

#[test]
fn serializes_typed_and_unknown_dimension_units() {
    assert_eq!(
        Token::Dimension {
            value: 2.0,
            unit: Unit::Length(LengthUnit::Px),
        }
        .to_css_string(PrinterOptions::default())
        .unwrap(),
        "2px"
    );
    assert_eq!(
        Token::UnknownDimension {
            value: 2.0,
            unit: "furlong",
        }
        .to_css_string(PrinterOptions::default())
        .unwrap(),
        "2furlong"
    );
}

#[test]
fn declaration_block_preserves_importance_bits() {
    let stylesheet = parse_stylesheet(".foo { color: red !important; opacity: .5 }");
    let CssRule::Style(style) = &stylesheet.rules[0] else {
        panic!("expected a style rule")
    };
    assert_eq!(
        style
            .declarations
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "color: red !important;\nopacity: .5"
    );
}

#[test]
fn declaration_block_skips_tombstones() {
    let allocator = Allocator::new();
    let mut declarations = DeclarationBlock::new(&allocator);

    declarations.push(Declaration::Tombstone, true);
    assert!(declarations.is_output_empty());
    assert_eq!(
        declarations
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        ""
    );

    declarations.push(Declaration::All(CSSWideKeyword::Initial), false);
    declarations.push(Declaration::Tombstone, true);
    declarations.push(Declaration::All(CSSWideKeyword::Inherit), true);
    declarations.push(Declaration::Tombstone, false);
    assert!(!declarations.is_output_empty());
    assert_eq!(
        declarations
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "all: initial;\nall: inherit !important"
    );
}

#[test]
fn merged_declaration_blocks_serialize_from_chain_head() {
    let mut stylesheet = parse_stylesheet("a{width:1px}a{height:2px}");
    let [CssRule::Style(first), CssRule::Style(second)] = &mut stylesheet.rules[..] else {
        panic!("expected two style rules")
    };
    let previous = Ref::from_pinned_box(first);
    second.as_mut().set_previous_merged(Some(previous));
    for selector in first.as_mut().selectors_mut().iter_mut() {
        *selector = Selector::Tombstone;
    }

    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{width:1px;height:2px}"
    );

    let pretty = stylesheet
        .to_css_string(PrinterOptions { prettify: true })
        .unwrap();
    assert_eq!(pretty.matches("a {").count(), 1);
    assert!(pretty.trim_start().starts_with("a {"));
    assert!(!pretty.starts_with('\n'));
}

#[test]
fn ports_lightningcss_typed_value_serialization_cases() {
    assert_eq!(
        Time::Milliseconds(100.0)
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        ".1s"
    );
    assert_eq!(
        EasingFunction::CubicBezier {
            x1: 0.42,
            y1: 0.0,
            x2: 1.0,
            y2: 1.0,
        }
        .to_css_string(PrinterOptions::default())
        .unwrap(),
        "ease-in"
    );
    assert_eq!(
        UnicodeRange {
            start: 0x400,
            end: 0x4ff,
        }
        .to_css_string(PrinterOptions::default())
        .unwrap(),
        "U+4??"
    );
    assert_eq!(
        FontFormat::Woff
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "\"woff\""
    );
    assert_eq!(
        FamilyName("Fancy Font Name")
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "Fancy Font Name"
    );
    assert_eq!(
        FontFamily::SansSerif
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "sans-serif"
    );
    assert_eq!(
        FontFamily::Custom("serif")
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "\"serif\""
    );
    assert_eq!(
        FontFamily::Custom("Fancy Font")
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "Fancy Font"
    );
    assert_eq!(
        FontFamily::Custom("A  B")
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "\"A  B\""
    );
    assert_eq!(
        FontFamily::Custom("1")
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "\"1\""
    );
    assert_eq!(
        FontFamily::Custom("slab serif")
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "\"slab serif\""
    );
    assert_eq!(
        FontFamily::Custom("slab inherit")
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        "\"slab inherit\""
    );
}

#[test]
#[ignore = "pseudo-elements inside :is() need lossless diagnostics"]
fn preserves_pseudo_elements_inside_is() {
    let stylesheet = parse_stylesheet(".foo:is(::before){color:green}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        ".foo:is(:before){color:green}"
    );
}

#[test]
#[ignore = "CSS Modules composition is not implemented"]
fn preserves_composes_inside_layers_until_module_compilation() {
    const SOURCE: &str = ".default{color:red}.button{composes:default}@layer components{.foo{composes:bar from \"./other.module.css\"}}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore = "CSS Modules grid symbol transforms are not implemented"]
fn preserves_dynamic_grid_symbols_until_module_compilation() {
    const SOURCE: &str =
        ".test{grid-template:\"test\" var(--foo);grid-template:\"test\" 1fr}.item{grid-area:test}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore = "CSS Modules dashed-ident resolution is not implemented"]
fn preserves_imported_dashed_idents_in_nested_values_and_rules() {
    const SOURCE: &str = ".x{background-color:rgb(var(--blue from \"./colors.module.css\"));&.info{border-color:var(--border);color:var(--red from \"./colors.module.css\")}}@media (min-width:10px){.x{color:var(--red from \"./colors.module.css\")}}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore = "module-qualified custom-property definitions are not represented"]
fn preserves_module_qualified_custom_property_definitions() {
    const SOURCE: &str = ".other-button{composes:button from \"./button.module.css\";--accent from \"./button.module.css\":blue}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore = "CSS custom functions and mixins are preserved but not implemented"]
fn preserves_css_custom_functions_and_mixins_draft() {
    const SOURCE: &str = "@function --negative(--value <length>) returns <length>{result:calc(-1 * var(--value))}.foo{margin:--negative(1px)}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore = "custom at-rule visitor expansion is not implemented"]
fn expands_mixins_at_the_apply_position_without_reordering_declarations() {
    const SOURCE: &str = "@mixin card{background:var(--bg-card);border-radius:var(--border-radius-md);padding:var(--spacing-5)}.quote{@apply card;transition:background var(--duration);margin-block-end:0;border-top-left-radius:0;border-bottom-left-radius:0;border-left-width:5px;border-left-color:var(--color-gray-400)}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        ".quote{background:var(--bg-card);border-radius:var(--border-radius-md);padding:var(--spacing-5);transition:background var(--duration);margin-block-end:0;border-top-left-radius:0;border-bottom-left-radius:0;border-left-width:5px;border-left-color:var(--color-gray-400)}"
    );
}

#[test]
#[ignore = "CSS Modules scoped keyframe names are not represented"]
fn preserves_global_keyframe_names_until_module_compilation() {
    const SOURCE: &str = "@keyframes :global(jump){0%{transform:translateY(0)}50%{transform:translateY(-10px)}100%{transform:translateY(0)}}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore = "target-aware light-dark lowering is not implemented"]
fn preserves_light_dark_when_a_child_changes_color_scheme() {
    const SOURCE: &str = ":root{--background:light-dark(white,black);--text:light-dark(black,white)}p{color:var(--text);background:var(--background);color-scheme:dark}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output, SOURCE);
    assert!(!output.contains("--lightningcss-light"));
    assert!(!output.contains("--lightningcss-dark"));
}

#[test]
#[ignore = "pseudo-element nesting validation and lowering are not implemented"]
fn preserves_nested_pseudo_element_rules_without_invalid_flattening() {
    const SOURCE: &str = ".input::placeholder{&:not(.noAdaptiveTypography){font-size:inherit}}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output, SOURCE);
    assert!(!output.contains(".input::placeholder:not("));
}

#[test]
#[ignore = "target-aware vendor prefix generation is not implemented"]
fn does_not_duplicate_authored_text_decoration_when_prefixing_for_targets() {
    const SOURCE: &str = "a{color:inherit;-webkit-text-decoration:inherit;text-decoration:inherit}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output, SOURCE);
    assert_eq!(output.matches("-webkit-text-decoration:inherit").count(), 1);
}

#[test]
#[ignore = "CSS Modules scoped selector compilation and cross-rule merging are not implemented"]
fn combines_resolved_local_and_global_css_module_selectors() {
    const SOURCE: &str = ".a{color:red}.b{color:red}:global(.c){color:red}:global(.d){color:red}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output.matches("{color:red}").count(), 1);
    assert!(!output.contains(":global"));
}

#[test]
#[ignore = "target-aware supports fallback generation is not implemented"]
fn preserves_root_and_host_when_generating_supports_fallbacks() {
    const SOURCE: &str = ":root,:host{--theme:color(display-p3 1 0 0)}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(
        output,
        "@supports (color:color(display-p3 0 0 0)){:root,:host{--theme:color(display-p3 1 0 0)}}"
    );
}

#[test]
#[ignore = "target-driven user-select prefix generation is not implemented"]
fn generates_user_select_prefix_for_safari_targets() {
    const SOURCE: &str = "a{user-select:all}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{-webkit-user-select:all;user-select:all}"
    );
}

#[test]
#[ignore = "target-driven logical property lowering is not implemented"]
fn does_not_partially_lower_dynamic_logical_shorthands() {
    const SOURCE: &str = "a{margin-inline:var(--m);color:red}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore]
fn preserves_svg_data_urls_with_opposite_quote_styles() {
    const SOURCE: &str = r#".a{background:url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg"></svg>')}.b{background:url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>")}"#;
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output.matches("data:image/svg+xml").count(), 2);
    assert!(output.contains("xmlns"));
    let _ = parse_stylesheet(&output);
}

#[test]
#[ignore]
fn preserves_unescaped_exponent_like_unknown_units() {
    const SOURCE: &str = r"a{height:0e;height:0E;height:0\65}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert!(output.contains("height:0e"));
    assert!(output.contains("height:0E"));
    assert!(!output.contains(r"0\65"));
}

#[test]
#[ignore]
fn retains_more_than_six_significant_digits_when_serializing_numbers() {
    let stylesheet = parse_stylesheet("a{line-height:1.3333333333;width:33.333333%}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{line-height:1.3333334;width:33.333332%}"
    );
}

#[test]
#[ignore = "custom-media expansion after stylesheet replacement is not implemented"]
fn expands_custom_media_after_a_stylesheet_replacement() {
    const SOURCE: &str = "@custom-media --narrow (max-width:30em);@media (--narrow){.a{color:red}}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "@media (max-width:30em){.a{color:red}}"
    );
}

#[test]
#[ignore = "iOS-target text-size-adjust prefix generation is not implemented"]
fn generates_text_size_adjust_prefix_for_ios_safari() {
    let stylesheet = parse_stylesheet("a{text-size-adjust:none}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{-webkit-text-size-adjust:none;text-size-adjust:none}"
    );
}

#[test]
#[ignore = "browser-target diagnostics for unlowerable selectors are not implemented"]
fn preserves_where_specificity_when_a_legacy_target_requires_a_diagnostic() {
    const SOURCE: &str = ":where(.button,#danger){color:red}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output, SOURCE);
    assert!(!output.contains(":is("));
}

#[test]
#[ignore]
fn preserves_property_rules_inside_layer_blocks() {
    const SOURCE: &str = "@layer base{@property --radialprogress{syntax:\"<percentage>\";inherits:true;initial-value:0%}}";
    let stylesheet = parse_stylesheet(SOURCE);
    let CssRule::LayerBlock(layer) = &stylesheet.rules[0] else {
        panic!("expected layer block")
    };
    assert!(matches!(layer.rules[0], CssRule::Property(_)));
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore]
fn preserves_numeric_oklch_property_initial_values() {
    const SOURCE: &str =
        "@property --accent{syntax:\"<color>\";inherits:false;initial-value:oklch(.5 0 0)}";
    let stylesheet = parse_stylesheet(SOURCE);
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        SOURCE
    );
}

#[test]
#[ignore]
fn preserves_attr_type_angle_brackets_without_inserted_whitespace() {
    const SOURCE: &str = "a{max-width:attr(data-max-width type(<length>)|fit-content)}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert!(output.contains("type(<length>)"));
    assert!(!output.contains("< length>"));
    let _ = parse_stylesheet(&output);
}

#[test]
#[ignore = "target-aware nesting lowering is not implemented"]
fn avoids_invalid_is_wrapping_for_nested_pseudo_element_media_rules() {
    let stylesheet = parse_stylesheet(".foo::after,.bar::after{@media screen{color:red}}");
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output, "@media screen{.foo:after,.bar:after{color:red}}");
    assert!(!output.contains(":is("));
}

#[test]
#[ignore = "target-aware vendor prefix generation is not implemented"]
fn retains_authored_vendor_values_when_generating_missing_prefixes() {
    let stylesheet = parse_stylesheet("a{-webkit-appearance:none;appearance:textfield}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{-webkit-appearance:none;-moz-appearance:textfield;appearance:textfield}"
    );
}

#[test]
#[ignore]
fn preserves_three_length_text_shadows_without_inserting_a_spread() {
    let stylesheet = parse_stylesheet(".foo{text-shadow:0 .02rem 0 rgba(0,0,0,.05)}");
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert!(output.contains("text-shadow:0 .02rem 0 rgba(0,0,0,.05)"));
    assert!(!output.contains("text-shadow:0 .02rem 0 0"));
    let _ = parse_stylesheet(&output);
}

#[test]
#[ignore]
fn preserves_unknown_media_calc_symbols_and_rule_bodies() {
    let stylesheet =
        parse_stylesheet("@media (min-width:calc(baseUnit * 1)){.className{color:red}}");
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert!(output.contains("baseUnit * 1"));
    assert!(output.contains(".className{color:red}"));
    assert_eq!(parse_stylesheet(&output).rules.len(), 1);
}

#[test]
#[ignore = "target-aware nesting lowering is not implemented"]
fn preserves_pseudo_elements_when_lowering_nested_parent_selectors() {
    let stylesheet = parse_stylesheet("#b::after{&{color:green}}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "#b::after{color:green}"
    );
}

#[test]
#[ignore = "pseudo-element chaining validation and source spelling preservation are not implemented"]
fn preserves_valid_before_and_after_marker_chains() {
    let stylesheet = parse_stylesheet("li::before::marker,li::after::marker{content:\"\"}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "li::before::marker,li::after::marker{content:\"\"}"
    );
}

#[test]
#[ignore = "browser-target selector lowering is not implemented"]
fn avoids_legacy_any_fallbacks_when_targets_support_selector_list_not() {
    let stylesheet = parse_stylesheet(":not(a,block){color:red}");
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output, ":not(a,block){color:red}");
    assert!(!output.contains("-webkit-any"));
    assert!(!output.contains("-moz-any"));
}

#[test]
#[ignore]
fn printer_options_are_copy_clone_and_debuggable() {
    fn assert_clone<T: Clone>() {}

    assert_clone::<PrinterOptions>();
    let options = PrinterOptions { prettify: false };
    let copied = options;
    assert_eq!(options, copied);
    assert_eq!(format!("{options:?}"), "PrinterOptions { prettify: false }");
}

#[test]
#[ignore = "an explicit quirks-mode color parser is not implemented"]
fn normalizes_legacy_bare_hex_colors_only_in_quirks_mode() {
    let stylesheet = parse_stylesheet("a{background-color:333333}");
    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{background-color:#333}"
    );
}

#[test]
#[ignore = "target-aware logical-property lowering is not implemented"]
fn avoids_specificity_increases_when_lowering_logical_margins() {
    const SOURCE: &str = ".ms-0{margin-inline-start:0}@media(min-width:1536px){.two-xl\\:mx-auto{margin-inline:auto}}";
    let stylesheet = parse_stylesheet(SOURCE);
    let output = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    assert_eq!(output, SOURCE);
    assert!(!output.contains(":lang("));
}
