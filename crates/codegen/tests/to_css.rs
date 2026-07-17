use rocketcss_allocator::Ref;
use rocketcss_codegen::{Printer, PrinterOptions, ToCss};
use rocketcss_parser::prelude::*;

fn parse_stylesheet(source: &str) -> StyleSheet<'static> {
    let source = std::boxed::Box::leak(source.to_owned().into_boxed_str());
    let allocator = std::boxed::Box::leak(std::boxed::Box::new(Allocator::new()));
    parse(source, allocator, ParserOptions::default()).unwrap()
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
    let previous = Ref::from_pinned_box(&first.declarations);
    second
        .declarations
        .as_mut()
        .set_previous_merged(Some(previous));
    for selector in first.selectors.iter_mut() {
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
