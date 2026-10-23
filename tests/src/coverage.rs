use rocketcss_ast_next::{CssRule, DeclarationData, UnparsedPropertyReason};
use rocketcss_common::Allocator;
use rocketcss_parser::{ParserOptions, parse};
use rustc_hash::FxHashMap;

const BOOTSTRAP: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../tasks/benchmark/files/bootstrap.css"
));
const TAILWIND: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../tasks/benchmark/files/tailwind.css"
));

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum UnparsedBucket {
    UnsupportedGrammar,
    OpaqueValue,
    UnknownProperty,
    InvalidValue,
}

impl From<UnparsedPropertyReason> for UnparsedBucket {
    fn from(reason: UnparsedPropertyReason) -> Self {
        match reason {
            UnparsedPropertyReason::UnsupportedGrammar => Self::UnsupportedGrammar,
            UnparsedPropertyReason::OpaqueValue => Self::OpaqueValue,
            UnparsedPropertyReason::UnknownProperty => Self::UnknownProperty,
            UnparsedPropertyReason::InvalidValue => Self::InvalidValue,
        }
    }
}

#[derive(Default)]
struct Coverage {
    typed_by_property: FxHashMap<String, usize>,
    custom_properties: usize,
    unparsed: FxHashMap<UnparsedBucket, usize>,
}

impl Coverage {
    fn unparsed_count(&self, bucket: UnparsedBucket) -> usize {
        self.unparsed.get(&bucket).copied().unwrap_or_default()
    }

    fn typed_count(&self, property: &str) -> usize {
        self.typed_by_property
            .get(property)
            .copied()
            .unwrap_or_default()
    }

    fn print(&self, name: &str) {
        let mut typed = self.typed_by_property.iter().collect::<Vec<_>>();
        typed.sort_unstable_by(|left, right| left.0.cmp(right.0));

        println!(
            "{name}: typed={}, custom={}, unsupported={}, opaque={}, unknown={}, invalid={}",
            typed.iter().map(|(_, count)| **count).sum::<usize>(),
            self.custom_properties,
            self.unparsed_count(UnparsedBucket::UnsupportedGrammar),
            self.unparsed_count(UnparsedBucket::OpaqueValue),
            self.unparsed_count(UnparsedBucket::UnknownProperty),
            self.unparsed_count(UnparsedBucket::InvalidValue),
        );
        for (property, count) in typed {
            println!("  typed {property}: {count}");
        }
    }
}

fn collect_coverage(source: &str) -> Coverage {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let stylesheet = parse(
            source,
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .expect("benchmark stylesheet should parse");

        let mut coverage = Coverage::default();
        visit_rules(
            stylesheet.ast(),
            stylesheet.style_sheet().rules(stylesheet.ast()),
            &mut coverage,
        );
        coverage
    })
}

fn visit_rules<'ast>(
    ast: &rocketcss_ast_next::Ast<'ast>,
    rules: &[CssRule<'ast>],
    coverage: &mut Coverage,
) {
    for rule in rules {
        if let Some(block) = rule.declarations(ast) {
            for payload in block.declarations(ast) {
                let Some(declaration) = payload.as_property(ast) else {
                    continue;
                };
                match declaration.data(ast) {
                    DeclarationData::Custom(_) => coverage.custom_properties += 1,
                    DeclarationData::Unparsed(property) => {
                        *coverage
                            .unparsed
                            .entry(property.reason(ast).into())
                            .or_default() += 1;
                    }
                    DeclarationData::Tombstone => {}
                    _ => {
                        *coverage
                            .typed_by_property
                            .entry(declaration.name(ast).to_owned())
                            .or_default() += 1;
                    }
                }
            }
        }
        visit_rules(ast, rule.children(ast), coverage);
    }
}

fn assert_typed(coverage: &Coverage, properties: &[&str]) {
    for property in properties {
        assert!(
            coverage.typed_count(property) > 0,
            "benchmark must construct typed declarations for {property}"
        );
    }
}

#[test]
fn reports_benchmark_property_coverage_and_regression_thresholds() {
    let bootstrap = collect_coverage(BOOTSTRAP);
    let tailwind = collect_coverage(TAILWIND);
    bootstrap.print("bootstrap");
    tailwind.print("tailwind");

    // These are the parse-only baseline counts recorded in the implementation
    // plan. Opaque values are intentionally not a reduction target.
    assert!(
        bootstrap.unparsed_count(UnparsedBucket::UnsupportedGrammar) < 3_042,
        "Bootstrap unsupported grammar count regressed"
    );
    assert!(
        tailwind.unparsed_count(UnparsedBucket::UnsupportedGrammar) < 22_337,
        "Tailwind unsupported grammar count regressed"
    );
    assert!(
        bootstrap.unparsed_count(UnparsedBucket::UnknownProperty) < 194,
        "Bootstrap unknown-property count regressed"
    );
    assert!(
        tailwind.unparsed_count(UnparsedBucket::UnknownProperty) < 817,
        "Tailwind unknown-property count regressed"
    );

    assert_typed(
        &tailwind,
        &[
            "border-color",
            "mask-composite",
            "mask-image",
            "translate",
            "fill",
            "stroke",
            "accent-color",
        ],
    );
}
