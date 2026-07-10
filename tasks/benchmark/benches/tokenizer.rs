#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use css_module_lexer::{Mode, collect_dependencies};
use rs_css_parser::Tokenizer;

const BENCH_CASES: &[(&str, &str)] = &[
    ("bootstrap", include_str!("../files/bootstrap.css")),
    ("bootstrap.min", include_str!("../files/bootstrap.min.css")),
];

fn tokenize(source: &str) -> usize {
    let mut tokenizer = Tokenizer::new(source);
    let mut token_count = 0;
    while let Ok(token) = tokenizer.next() {
        black_box(token);
        token_count += 1;
    }
    token_count
}

fn bench_files(c: &mut Criterion) {
    for &(name, source) in BENCH_CASES {
        let mut group = c.benchmark_group(format!("{name}/tokenizer"));
        group.throughput(Throughput::Bytes(source.len() as u64));

        // This follows css-module-lexer's own benchmark. It includes dependency
        // collection in addition to lexical scanning because its raw Visitor
        // interface is not public.
        group.bench_function("css-module-lexer", |b| {
            b.iter(|| collect_dependencies(black_box(source), Mode::Local))
        });
        group.bench_function("rs-css", |b| b.iter(|| tokenize(black_box(source))));

        group.finish();
    }
}

criterion_group!(benches, bench_files);
criterion_main!(benches);
