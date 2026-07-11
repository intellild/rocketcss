#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use benchmark::{BENCH_CASES, BenchCase};
use css_module_lexer::{Mode, collect_dependencies};
use divan::{Bencher, black_box, counter::BytesCount};
use rs_css_parser::Tokenizer;

fn main() {
    divan::main();
}

fn tokenize(source: &str) -> usize {
    let mut tokenizer = Tokenizer::new(source);
    let mut token_count = 0;
    while let Ok(token) = tokenizer.next() {
        black_box(token);
        token_count += 1;
    }
    token_count
}

#[divan::bench(args = BENCH_CASES)]
fn css_module_lexer(bencher: Bencher<'_, '_>, case: BenchCase) {
    // This follows css-module-lexer's own benchmark. It includes dependency
    // collection in addition to lexical scanning because its raw Visitor
    // interface is not public.
    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| {
            black_box(collect_dependencies(black_box(case.source), Mode::Local));
        });
}

#[divan::bench(args = BENCH_CASES)]
fn rs_css(bencher: Bencher<'_, '_>, case: BenchCase) {
    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| tokenize(black_box(case.source)));
}
