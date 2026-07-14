#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::ffi::OsString;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::Duration;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_allocator::Allocator;
use rocketcss_benchmark::{BENCH_CASES, BenchCase, WRITER_CAPACITY_PADDING};
use rocketcss_codegen::{Printer, PrinterOptions, ToCss};

fn main() {
    divan::main();
}

#[divan::bench(args = BENCH_CASES)]
fn rocketcss(bencher: Bencher<'_, '_>, case: BenchCase) {
    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| {
            let allocator = Allocator::new();
            let mut stylesheet = rocketcss_parser::parse(
                black_box(case.source),
                &allocator,
                rocketcss_parser::ParserOptions {
                    error_recovery: true,
                    ..Default::default()
                },
            )
            .unwrap();
            rocketcss_minify::minify(&mut stylesheet, rocketcss_minify::MinifyOptions::default());
            let mut output = String::with_capacity(case.source.len() + WRITER_CAPACITY_PADDING);
            stylesheet
                .to_css(&mut Printer::new(
                    &mut output,
                    PrinterOptions { prettify: false },
                ))
                .unwrap();
            black_box(output);
        });
}

#[divan::bench(args = BENCH_CASES)]
fn lightningcss(bencher: Bencher<'_, '_>, case: BenchCase) {
    use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};

    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| {
            let mut stylesheet =
                StyleSheet::parse(black_box(case.source), ParserOptions::default()).unwrap();
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
    /// Returns the directory that holds the cssnano checkout, if it is available.
    ///
    /// The cssnano comparison relies on an external Node.js checkout of cssnano.
    /// When it is missing, the benchmark is skipped instead of failing the whole
    /// run.
    fn cssnano_dir() -> Option<PathBuf> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let cssnano_dir = std::env::var_os("CSSNANO_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| manifest_dir.join("../../../cssnano"));
        cssnano_dir
            .join("packages/cssnano/src/index.js")
            .is_file()
            .then_some(cssnano_dir)
    }

    fn spawn(case: BenchCase) -> Option<Self> {
        Self::cssnano_dir()?;

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let node = std::env::var_os("NODE").unwrap_or_else(|| OsString::from("node"));
        let mut child = Command::new(node)
            .arg(manifest_dir.join("scripts/cssnano-worker.mjs"))
            .arg(manifest_dir.join("files").join(case.file_name))
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

        Some(Self {
            child,
            stdin,
            stdout,
        })
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

#[divan::bench(args = BENCH_CASES)]
fn cssnano(bencher: Bencher<'_, '_>, case: BenchCase) {
    let Some(mut cssnano) = CssnanoWorker::spawn(case) else {
        // Skip the comparison instead of failing the benchmark run when the
        // cssnano checkout is unavailable. Set CSSNANO_DIR to enable it locally.
        eprintln!(
            "skipping cssnano benchmark: cssnano checkout not found (set CSSNANO_DIR to enable)"
        );
        return;
    };
    bencher
        .counter(BytesCount::of_str(case.source))
        .bench_local(|| black_box(cssnano.run(1)));
}
