#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_parser::prelude::StyleSheet;

const BOOTSTRAP: &str = include_str!("../files/bootstrap.css");
const ITERATIONS: usize = 10;

fn main() {
    divan::main();
}

struct ParsedStyleSheet {
    // Fields are dropped in declaration order, so the stylesheet is dropped
    // before the allocator that owns its arena storage.
    stylesheet: StyleSheet<'static>,
    _allocator: Box<Allocator>,
}

impl ParsedStyleSheet {
    fn new() -> Self {
        let allocator = Box::new(Allocator::new());

        // The allocator remains at a stable heap address and is owned by this
        // input for at least as long as the stylesheet.
        let allocator_ref: &'static Allocator = unsafe { &*std::ptr::from_ref(&*allocator) };
        let stylesheet = rocketcss_parser::parse(
            BOOTSTRAP,
            allocator_ref,
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

fn processed_bytes() -> BytesCount {
    BytesCount::new(BOOTSTRAP.len() * ITERATIONS)
}

#[divan::bench]
fn parse(bencher: Bencher<'_, '_>) {
    bencher.counter(processed_bytes()).bench_local(|| {
        for _ in 0..ITERATIONS {
            let allocator = Allocator::new();
            let stylesheet = rocketcss_parser::parse(
                black_box(BOOTSTRAP),
                &allocator,
                rocketcss_parser::ParserOptions {
                    error_recovery: true,
                    ..Default::default()
                },
            )
            .unwrap();
            black_box(stylesheet);
        }
    });
}

#[divan::bench]
fn minify(bencher: Bencher<'_, '_>) {
    bencher
        .counter(processed_bytes())
        .with_inputs(|| std::array::from_fn::<_, ITERATIONS, _>(|_| ParsedStyleSheet::new()))
        .bench_local_values(|mut inputs| {
            for input in &mut inputs {
                black_box(rocketcss_minify::minify(
                    &mut input.stylesheet,
                    rocketcss_minify::MinifyOptions::default(),
                ));
            }
        });
}

#[divan::bench]
fn codegen(bencher: Bencher<'_, '_>) {
    bencher
        .counter(processed_bytes())
        .with_inputs(|| {
            let mut input = ParsedStyleSheet::new();
            rocketcss_minify::minify(
                &mut input.stylesheet,
                rocketcss_minify::MinifyOptions::default(),
            );
            input
        })
        .bench_local_values(|input| {
            for _ in 0..ITERATIONS {
                black_box(
                    input
                        .stylesheet
                        .to_css_string(PrinterOptions { prettify: false })
                        .unwrap(),
                );
            }
        });
}
