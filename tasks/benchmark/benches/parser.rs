#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use criterion::{Bencher, Criterion, Throughput, black_box, criterion_group, criterion_main};
use rs_css_allocator::Allocator;

const BENCH_CASES: &[(&str, &str)] = &[
    ("bootstrap", include_str!("../files/bootstrap.css")),
    ("bootstrap.min", include_str!("../files/bootstrap.min.css")),
];

fn bench_rs_css(b: &mut Bencher<'_>, source: &'static str) {
    b.iter(|| {
        let allocator = Allocator::new();
        let stylesheet = rs_css_parser::parse(
            black_box(source),
            &allocator,
            rs_css_parser::ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .unwrap();
        black_box(stylesheet);
    });
}

fn bench_lightningcss(b: &mut Bencher<'_>, source: &'static str) {
    use lightningcss::stylesheet::{ParserOptions, StyleSheet};

    b.iter(|| {
        let stylesheet = StyleSheet::parse(black_box(source), ParserOptions::default()).unwrap();
        black_box(stylesheet);
    });
}

fn bench_stylo(b: &mut Bencher<'_>, source: &'static str) {
    use style::context::QuirksMode;
    use style::shared_lock::SharedRwLock;
    use style::stylesheets::{AllowImportRules, Origin, StylesheetContents, UrlExtraData};

    let shared_lock = SharedRwLock::new();
    let url_data = UrlExtraData::from(url::Url::parse("about:blank").unwrap());

    b.iter(|| {
        let stylesheet = StylesheetContents::from_str(
            black_box(source),
            url_data.clone(),
            Origin::Author,
            &shared_lock,
            None,
            None,
            QuirksMode::NoQuirks,
            AllowImportRules::Yes,
            None,
        );
        black_box(stylesheet);
    });
}

fn bench_files(c: &mut Criterion) {
    for &(name, source) in BENCH_CASES {
        let mut group = c.benchmark_group(format!("{name}/parser"));
        group.throughput(Throughput::Bytes(source.len() as u64));

        group.bench_function("rs-css", |b| bench_rs_css(b, source));
        group.bench_function("lightningcss", |b| bench_lightningcss(b, source));
        group.bench_function("stylo", |b| bench_stylo(b, source));

        group.finish();
    }
}

criterion_group!(benches, bench_files);
criterion_main!(benches);
