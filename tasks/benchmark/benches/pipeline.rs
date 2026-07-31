#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_benchmark::{BENCH_CASES, BenchCase, WRITER_CAPACITY_PADDING};
use rocketcss_codegen::{Printer, PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::GhostToken;
use rocketcss_parser::prelude::Compilation;
use std::cell::RefCell;

fn main() {
    divan::main();
}

struct ParsedStyleSheet<'ghost> {
    stylesheet: Compilation<'ghost>,
}

impl<'ghost> ParsedStyleSheet<'ghost> {
    fn new(source: &'static str, token: &mut GhostToken<'ghost>) -> Self {
        let stylesheet = rocketcss_parser::parse(
            source,
            token,
            rocketcss_parser::ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .unwrap();

        Self { stylesheet }
    }
}

fn processed_bytes(case: BenchCase) -> BytesCount {
    BytesCount::new(case.source.len() * case.pipeline_iterations)
}

#[divan::bench(args = BENCH_CASES)]
fn parse(bencher: Bencher<'_, '_>, case: BenchCase) {
    bencher.counter(processed_bytes(case)).bench_local(|| {
        for _ in 0..case.pipeline_iterations {
            GhostToken::scope(|mut token| {
                let stylesheet = rocketcss_parser::parse(
                    black_box(case.source),
                    &mut token,
                    rocketcss_parser::ParserOptions {
                        error_recovery: true,
                        ..Default::default()
                    },
                )
                .unwrap();
                black_box(stylesheet);
            });
        }
    });
}

#[divan::bench(args = BENCH_CASES)]
fn minify(bencher: Bencher<'_, '_>, case: BenchCase) {
    rocketcss_common::GhostToken::scope(|token| {
        let token = RefCell::new(token);
        bencher
            .counter(processed_bytes(case))
            .with_inputs(|| {
                std::iter::repeat_with(|| {
                    ParsedStyleSheet::new(case.source, &mut token.borrow_mut())
                })
                .take(case.pipeline_iterations)
                .collect::<Vec<_>>()
            })
            .bench_local_values(|mut inputs| {
                for input in &mut inputs {
                    black_box(rocketcss_nano::minify(
                        &mut input.stylesheet,
                        &mut token.borrow_mut(),
                        rocketcss_nano::MinifyOptions::default(),
                    ));
                }
            });
    });
}

#[divan::bench(args = BENCH_CASES)]
fn codegen(bencher: Bencher<'_, '_>, case: BenchCase) {
    rocketcss_common::GhostToken::scope(|token| {
        let token = RefCell::new(token);
        bencher
            .counter(processed_bytes(case))
            .with_inputs(|| {
                let mut input = ParsedStyleSheet::new(case.source, &mut token.borrow_mut());
                rocketcss_nano::minify(
                    &mut input.stylesheet,
                    &mut token.borrow_mut(),
                    rocketcss_nano::MinifyOptions::default(),
                );
                input
            })
            .bench_local_values(|input| {
                for _ in 0..case.pipeline_iterations {
                    let mut output =
                        String::with_capacity(case.source.len() + WRITER_CAPACITY_PADDING);
                    input
                        .stylesheet
                        .to_css(
                            &mut Printer::new(&mut output, PrinterOptions { prettify: false }),
                            &ToCssContext::new(&token.borrow()),
                        )
                        .unwrap();
                    black_box(output);
                }
            });
    });
}
