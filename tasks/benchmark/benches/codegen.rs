#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use rs_css_allocator::Allocator;
use rs_css_codegen::{PrinterOptions, ToCss};

const BENCH_CASES: &[(&str, &str)] = &[
    ("bootstrap", include_str!("../files/bootstrap.css")),
    ("bootstrap.min", include_str!("../files/bootstrap.min.css")),
];

fn bench_files(c: &mut Criterion) {
    use lightningcss::stylesheet::{
        ParserOptions, PrinterOptions as LightningPrinterOptions, StyleSheet,
    };

    for &(name, source) in BENCH_CASES {
        let allocator = Allocator::new();
        let stylesheet = rs_css_parser::parse(
            source,
            &allocator,
            rs_css_parser::ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .unwrap();
        let lightning_stylesheet = StyleSheet::parse(source, ParserOptions::default()).unwrap();

        let mut group = c.benchmark_group(format!("{name}/codegen"));
        group.throughput(Throughput::Bytes(source.len() as u64));

        group.bench_function("rs-css", |b| {
            b.iter(|| {
                black_box(
                    stylesheet
                        .to_css_string(PrinterOptions { prettify: false })
                        .unwrap(),
                );
            });
        });
        group.bench_function("lightningcss", |b| {
            b.iter(|| {
                black_box(
                    lightning_stylesheet
                        .to_css(LightningPrinterOptions {
                            minify: true,
                            ..LightningPrinterOptions::default()
                        })
                        .unwrap(),
                );
            });
        });

        group.finish();
    }
}

criterion_group!(benches, bench_files);
criterion_main!(benches);
