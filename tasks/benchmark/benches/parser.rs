#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_allocator::Allocator;
use rocketcss_benchmark::{BENCH_CASES, BenchCase};

fn main() {
    divan::main();
}

#[divan::bench(args = BENCH_CASES)]
fn rocketcss(bencher: Bencher<'_, '_>, case: BenchCase) {
    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| {
            let allocator = Allocator::new();
            let stylesheet = rocketcss_parser::parse(
                black_box(case.source),
                &allocator,
                rocketcss_parser::ParserOptions {
                    error_recovery: true,
                    ..Default::default()
                },
            )
            .unwrap();
            black_box(stylesheet);
        });
}

#[divan::bench(args = BENCH_CASES)]
fn lightningcss(bencher: Bencher<'_, '_>, case: BenchCase) {
    use lightningcss::stylesheet::{ParserOptions, StyleSheet};

    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| {
            let stylesheet =
                StyleSheet::parse(black_box(case.source), ParserOptions::default()).unwrap();
            black_box(stylesheet);
        });
}

#[divan::bench(args = BENCH_CASES)]
fn stylo(bencher: Bencher<'_, '_>, case: BenchCase) {
    use style::context::QuirksMode;
    use style::shared_lock::SharedRwLock;
    use style::stylesheets::{AllowImportRules, Origin, StylesheetContents, UrlExtraData};

    let shared_lock = SharedRwLock::new();
    let url_data = UrlExtraData::from(url::Url::parse("about:blank").unwrap());

    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| {
            let stylesheet = StylesheetContents::from_str(
                black_box(case.source),
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
