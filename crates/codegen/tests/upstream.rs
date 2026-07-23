use lightningcss::stylesheet::{
    ParserOptions as LightningParserOptions, PrinterOptions as LightningPrinterOptions,
    StyleSheet as LightningStyleSheet,
};
use rocketcss_codegen::{PrinterOptions, ToCssWithGhost};
use rocketcss_parser::prelude::*;
use serde_json::Value;

const CORPUS: &str = include_str!("../../parser/tests/upstream/corpus.json");

#[test]
fn lightningcss_stylesheet_ast_to_css_corpus() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("upstream corpus must be valid JSON");
    let cases = corpus["lightningcss"]["cases"]
        .as_array()
        .expect("Lightning CSS corpus must contain cases");
    assert_eq!(cases.len(), 4_223, "the audited upstream corpus changed");

    let mut compared = 0;
    let mut failures = std::vec::Vec::new();
    for case in cases {
        let source = case["source"]
            .as_str()
            .expect("case source must be a string");
        let name = case["name"].as_str().expect("case name must be a string");
        let error_recovery = case["error_recovery"].as_bool().unwrap_or(false);

        let Ok(lightning) = LightningStyleSheet::parse(
            source,
            LightningParserOptions {
                error_recovery,
                ..LightningParserOptions::default()
            },
        ) else {
            // The parser corpus includes two custom-at-rule parser cases. They
            // cannot be parsed by Lightning CSS's default at-rule parser.
            continue;
        };
        let expected = lightning
            .to_css(LightningPrinterOptions::default())
            .expect("upstream AST should serialize")
            .code;

        let allocator = Allocator::new();
        let actual = allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                source,
                &allocator,
                &mut token,
                ParserOptions {
                    error_recovery,
                    ..ParserOptions::default()
                },
            )
            .unwrap_or_else(|error| {
                panic!("{name}: parser corpus stopped producing an AST: {error:?}")
            });
            stylesheet
                .to_css_string(&token, PrinterOptions::default())
                .expect("rocketcss AST should serialize")
        });
        compared += 1;

        let canonical_actual = LightningStyleSheet::parse(
            &actual,
            LightningParserOptions {
                error_recovery,
                ..LightningParserOptions::default()
            },
        )
        .ok()
        .and_then(|stylesheet| stylesheet.to_css(LightningPrinterOptions::default()).ok());
        let Some(canonical_actual) = canonical_actual else {
            failures.push(format!(
                "{name}\nsource:\n{source}\nrocketcss output could not be reparsed:\n{actual}"
            ));
            if failures.len() == 50 {
                break;
            }
            continue;
        };

        if canonical_actual.code != expected {
            failures.push(format!(
                "{name}\nsource:\n{source}\nexpected canonical CSS:\n{expected}\nrocketcss output:\n{actual}\ncanonical rocketcss output:\n{}",
                canonical_actual.code
            ));
            if failures.len() == 50 {
                break;
            }
        }
    }

    assert_eq!(
        compared, 4_220,
        "the audited upstream serialization corpus changed"
    );
    assert!(
        failures.is_empty(),
        "{} Lightning CSS AST-to-CSS cases disagreed (showing at most 50):\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}
