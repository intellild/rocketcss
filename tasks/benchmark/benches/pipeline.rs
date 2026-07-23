#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_allocator::{Allocator, GhostToken};
use rocketcss_benchmark::{BENCH_CASES, BenchCase, WRITER_CAPACITY_PADDING};
use rocketcss_codegen::{Printer, PrinterOptions, ToCssWithGhost};
use rocketcss_parser::prelude::StyleSheet;
use std::cell::RefCell;

fn main() {
    divan::main();
}

struct ParsedStyleSheet<'ghost> {
    // Fields are dropped in declaration order, so the stylesheet is dropped
    // before the allocator that owns its arena storage.
    stylesheet: StyleSheet<'ghost, 'ghost>,
    _allocator: Box<Allocator>,
}

impl<'ghost> ParsedStyleSheet<'ghost> {
    fn new(source: &'static str, token: &mut GhostToken<'ghost>) -> Self {
        let allocator = Box::new(Allocator::new());

        // The allocator remains at a stable heap address and is owned by this
        // input for at least as long as the stylesheet.
        let allocator_ref: &'ghost Allocator = unsafe { &*std::ptr::from_ref(&*allocator) };
        let stylesheet = rocketcss_parser::parse(
            source,
            allocator_ref,
            token,
            rocketcss_parser::ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            stylesheet,
            _allocator: allocator,
        }
    }
}

fn processed_bytes(case: BenchCase) -> BytesCount {
    BytesCount::new(case.source.len() * case.pipeline_iterations)
}

#[divan::bench(args = BENCH_CASES)]
fn parse(bencher: Bencher<'_, '_>, case: BenchCase) {
    bencher.counter(processed_bytes(case)).bench_local(|| {
        for _ in 0..case.pipeline_iterations {
            let allocator = Allocator::new();
            allocator.with_ghost(|mut token| {
                let stylesheet = rocketcss_parser::parse(
                    black_box(case.source),
                    &allocator,
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
    rocketcss_allocator::GhostToken::scope(|token| {
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
    rocketcss_allocator::GhostToken::scope(|token| {
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
                        .to_css_with_ghost(
                            &token.borrow(),
                            &mut Printer::new(&mut output, PrinterOptions { prettify: false }),
                        )
                        .unwrap();
                    black_box(output);
                }
            });
    });
}
