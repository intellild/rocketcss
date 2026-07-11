use rs_css_codegen::{PrinterOptions, ToCss};
use rs_css_parser::prelude::*;

fn parse_stylesheet(source: &str) -> StyleSheet<'static> {
    let source = std::boxed::Box::leak(source.to_owned().into_boxed_str());
    let allocator = std::boxed::Box::leak(std::boxed::Box::new(Allocator::new()));
    parse(source, allocator, ParserOptions::default()).unwrap()
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
}
