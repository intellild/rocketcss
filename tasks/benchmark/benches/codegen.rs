#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use benchmark::{BENCH_CASES, BenchCase};
use divan::{Bencher, black_box, counter::BytesCount};
use rs_css_allocator::Allocator;
use rs_css_codegen::{PrinterOptions, ToCss};

fn main() {
    divan::main();
}

#[divan::bench(args = BENCH_CASES)]
fn rs_css(bencher: Bencher<'_, '_>, case: BenchCase) {
    let allocator = Allocator::new();
    let stylesheet = rs_css_parser::parse(
        case.source,
        &allocator,
        rs_css_parser::ParserOptions {
            error_recovery: true,
            ..Default::default()
        },
    )
    .unwrap();

    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| {
            black_box(
                stylesheet
                    .to_css_string(PrinterOptions { prettify: false })
                    .unwrap(),
            );
        });
}

#[divan::bench(args = BENCH_CASES)]
fn lightningcss(bencher: Bencher<'_, '_>, case: BenchCase) {
    use lightningcss::stylesheet::{
        ParserOptions, PrinterOptions as LightningPrinterOptions, StyleSheet,
    };

    let stylesheet = StyleSheet::parse(case.source, ParserOptions::default()).unwrap();

    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| {
            black_box(
                stylesheet
                    .to_css(LightningPrinterOptions {
                        minify: true,
                        ..LightningPrinterOptions::default()
                    })
                    .unwrap(),
            );
        });
}
