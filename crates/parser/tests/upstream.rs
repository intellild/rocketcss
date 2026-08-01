use rocketcss_parser::prelude::*;
use serde_json::Value;

const CORPUS: &str = include_str!("upstream/corpus.json");

fn cases<'a>(corpus: &'a Value, upstream: &str) -> &'a [Value] {
    corpus[upstream]["cases"]
        .as_array()
        .expect("upstream corpus must contain a cases array")
}

fn case_string<'a>(case: &'a Value, field: &str) -> &'a str {
    case[field]
        .as_str()
        .unwrap_or_else(|| panic!("case field {field:?} must be a string"))
}

#[test]
fn lightningcss_stylesheet_text_to_ast_corpus() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("upstream corpus must be valid JSON");
    let cases = cases(&corpus, "lightningcss");
    assert_eq!(cases.len(), 4_223, "the audited upstream corpus changed");

    let mut failures = std::vec::Vec::new();
    for case in cases {
        let source = case_string(case, "source");
        let name = case_string(case, "name");
        let allocator = Allocator::new();
        let error = allocator.with_ghost(|mut token| {
            parse(
                source,
                &allocator,
                &mut token,
                ParserOptions {
                    error_recovery: case["error_recovery"].as_bool().unwrap_or(false),
                    ..ParserOptions::default()
                },
            )
            .err()
            .map(|error| format!("{error:?}"))
        });

        if let Some(error) = error {
            failures.push(format!("{name}: {error}\nsource:\n{source}"));
        }
    }

    assert!(
        failures.is_empty(),
        "{} LightningCSS text-to-AST cases disagreed:\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}

#[test]
fn stylo_selector_text_to_ast_corpus() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("upstream corpus must be valid JSON");
    let cases = cases(&corpus, "stylo");
    assert_eq!(cases.len(), 80, "the audited upstream corpus changed");

    let mut failures = std::vec::Vec::new();
    for case in cases {
        let source = case_string(case, "source");
        let name = case_string(case, "name");
        let allocator = Allocator::new();
        let result = SelectorList::parse_string(source, &allocator);

        if let Err(error) = result {
            failures.push(format!("{name}: {error:?}; source: {source:?}"));
        }
    }

    assert!(
        failures.is_empty(),
        "{} Stylo selector text-to-AST cases disagreed:\n\n{}",
        failures.len(),
        failures.join("\n")
    );
}
