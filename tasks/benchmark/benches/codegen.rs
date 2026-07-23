#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_allocator::Allocator;
use rocketcss_benchmark::{BENCH_CASES, BenchCase, WRITER_CAPACITY_PADDING};
use rocketcss_codegen::{Printer, PrinterOptions, ToCssWithGhost};

fn main() {
    divan::main();
}

#[divan::bench(args = BENCH_CASES)]
fn rocketcss(bencher: Bencher<'_, '_>, case: BenchCase) {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let stylesheet = rocketcss_parser::parse(
            case.source,
            &allocator,
            &mut token,
            rocketcss_parser::ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .unwrap();

        bencher
            .counter(BytesCount::of_str(case.source))
            .bench_local(|| {
                let mut output = String::with_capacity(case.source.len() + WRITER_CAPACITY_PADDING);
                stylesheet
                    .to_css_with_ghost(
                        &token,
                        &mut Printer::new(&mut output, PrinterOptions { prettify: false }),
                    )
                    .unwrap();
                black_box(output);
            });
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
