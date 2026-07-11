#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::ffi::OsString;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::Duration;

use criterion::{Bencher, Criterion, Throughput, black_box, criterion_group, criterion_main};
use rs_css_allocator::Allocator;
use rs_css_codegen::{PrinterOptions, ToCss};

const BOOTSTRAP: &str = include_str!("../files/bootstrap.css");

fn bench_rs_css(b: &mut Bencher<'_>) {
    b.iter(|| {
        let allocator = Allocator::new();
        let mut stylesheet = rs_css_parser::parse(
            black_box(BOOTSTRAP),
            &allocator,
            rs_css_parser::ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .unwrap();
        rs_css_minify::minify(&mut stylesheet, rs_css_minify::MinifyOptions::default());
        let output = stylesheet
            .to_css_string(PrinterOptions { minify: true })
            .unwrap();
        black_box(output);
    });
}

fn bench_lightningcss(b: &mut Bencher<'_>) {
    use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};

    b.iter(|| {
        let mut stylesheet =
            StyleSheet::parse(black_box(BOOTSTRAP), ParserOptions::default()).unwrap();
        stylesheet.minify(MinifyOptions::default()).unwrap();
        let output = stylesheet
            .to_css(PrinterOptions {
                minify: true,
                ..PrinterOptions::default()
            })
            .unwrap();
        black_box(output);
    });
}

struct CssnanoWorker {
    child: Child,
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

impl CssnanoWorker {
    fn spawn() -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let node = std::env::var_os("NODE").unwrap_or_else(|| OsString::from("node"));
        let mut child = Command::new(node)
            .arg(manifest_dir.join("scripts/cssnano-worker.cjs"))
            .arg(manifest_dir.join("files/bootstrap.css"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to start the cssnano Node.js worker");
        let stdin = BufWriter::new(child.stdin.take().expect("cssnano worker has no stdin"));
        let mut stdout = BufReader::new(child.stdout.take().expect("cssnano worker has no stdout"));

        let mut ready = String::new();
        stdout
            .read_line(&mut ready)
            .expect("failed to read cssnano worker startup status");
        assert_eq!(ready.trim(), "ready", "cssnano worker failed to initialize");

        Self {
            child,
            stdin,
            stdout,
        }
    }

    fn run(&mut self, iterations: u64) -> Duration {
        writeln!(self.stdin, "{iterations}").expect("failed to request a cssnano benchmark sample");
        self.stdin
            .flush()
            .expect("failed to flush a cssnano benchmark request");

        let mut response = String::new();
        self.stdout
            .read_line(&mut response)
            .expect("failed to read a cssnano benchmark sample");
        let mut fields = response.split_ascii_whitespace();
        let elapsed_nanos = fields
            .next()
            .expect("cssnano worker returned an empty response")
            .parse()
            .expect("cssnano worker returned an invalid duration");
        let output_len: usize = fields
            .next()
            .expect("cssnano worker did not return an output length")
            .parse()
            .expect("cssnano worker returned an invalid output length");
        assert!(fields.next().is_none(), "invalid cssnano worker response");
        black_box(output_len);
        Duration::from_nanos(elapsed_nanos)
    }
}

impl Drop for CssnanoWorker {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn bench_bootstrap(c: &mut Criterion) {
    let mut cssnano = CssnanoWorker::spawn();
    let mut group = c.benchmark_group("bootstrap/minify");
    group.throughput(Throughput::Bytes(BOOTSTRAP.len() as u64));

    group.bench_function("rs-css", bench_rs_css);
    group.bench_function("lightningcss", bench_lightningcss);
    group.bench_function("cssnano", |b| {
        b.iter_custom(|iterations| cssnano.run(iterations));
    });

    group.finish();
}

criterion_group!(benches, bench_bootstrap);
criterion_main!(benches);
