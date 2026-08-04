#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_benchmark::{BENCH_CASES, BenchCase, WRITER_CAPACITY_PADDING};
use rocketcss_codegen::{Printer, PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::{Allocator, GhostToken};
use rocketcss_parser::prelude::Compilation;
use std::cell::RefCell;

fn main() {
    divan::main();
}

struct ParsedStyleSheet<'ghost> {
    // Fields are dropped in declaration order, so the stylesheet is dropped
    // before the allocator that owns its arena storage.
    stylesheet: Compilation<'ghost>,
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

const EXHAUSTED_S3_ENDPOINT_CHAIN: &str = concat!(
    ".r00{opacity:.5}",
    ".r01{opacity:.5}",
    ".r02{opacity:.5}",
    ".r03{opacity:.5}",
    ".r04{opacity:.5}",
    ".r05{opacity:.5}",
    ".r06{opacity:.5}",
    ".r07{opacity:.5}",
    ".r08{opacity:.5}",
    ".r09{opacity:.5}",
    ".r10{opacity:.5}",
    ".r11{opacity:.5}",
    ".r12{opacity:.5}",
    ".r13{opacity:.5}",
    ".r14{opacity:.5}",
    ".r15{opacity:.5}",
    ".r16{opacity:.5}",
    ".r17{opacity:.5}",
    ".r18{opacity:.5}",
    ".r19{opacity:.5}",
    ".r20{opacity:.5}",
    ".r21{opacity:.5}",
    ".r22{opacity:.5}",
    ".r23{opacity:.5}",
    ".r24{opacity:.5}",
    ".r25{opacity:.5}",
    ".r26{opacity:.5}",
    ".r27{opacity:.5}",
    ".r28{opacity:.5}",
    ".r29{opacity:.5}",
    ".r30{opacity:.5}",
    ".r31{opacity:.5}",
);

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

#[divan::bench]
fn minify_exhausted_s3_endpoint_chain(bencher: Bencher<'_, '_>) {
    rocketcss_common::GhostToken::scope(|token| {
        let token = RefCell::new(token);
        bencher
            .counter(BytesCount::new(EXHAUSTED_S3_ENDPOINT_CHAIN.len()))
            .with_inputs(|| {
                ParsedStyleSheet::new(EXHAUSTED_S3_ENDPOINT_CHAIN, &mut token.borrow_mut())
            })
            .bench_local_values(|mut input| {
                black_box(rocketcss_nano::minify(
                    &mut input.stylesheet,
                    &mut token.borrow_mut(),
                    rocketcss_nano::MinifyOptions::default(),
                ));
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
