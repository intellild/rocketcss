#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::cell::RefCell;
use std::ffi::OsString;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::Duration;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_allocator::{Allocator, GhostToken};
use rocketcss_benchmark::{BENCH_CASES, BenchCase, WRITER_CAPACITY_PADDING};
use rocketcss_codegen::{Printer, PrinterOptions, ToCss, ToCssContext};
use rocketcss_parser::prelude::StyleSheet;

fn main() {
    divan::main();
}

/// Owns a parsed stylesheet together with the allocator that backs its arena
/// storage, so the minify and codegen stages can be measured without parsing.
///
/// Stage benchmarks consume this through `bench_local_refs` so the stylesheet
/// and its arena stay alive until Divan stops timing the sample; dropping them
/// inside the timed section would add teardown time to the stage measurement.
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

    fn minified(source: &'static str, token: &mut GhostToken<'ghost>) -> Self {
        let mut input = Self::new(source, token);
        rocketcss_nano::minify(
            &mut input.stylesheet,
            token,
            rocketcss_nano::MinifyOptions::default(),
        );
        input
    }
}

mod parse {
    use super::*;

    #[divan::bench(args = BENCH_CASES)]
    fn rocketcss(bencher: Bencher<'_, '_>, case: BenchCase) {
        bencher
            .counter(BytesCount::of_str(case.source))
            .bench_local(|| {
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
}

mod minify {
    use super::*;

    #[divan::bench(args = BENCH_CASES)]
    fn rocketcss(bencher: Bencher<'_, '_>, case: BenchCase) {
        rocketcss_allocator::GhostToken::scope(|token| {
            let token = RefCell::new(token);
            bencher
                .counter(BytesCount::of_str(case.source))
                .with_inputs(|| ParsedStyleSheet::new(case.source, &mut token.borrow_mut()))
                .bench_local_refs(|input| {
                    black_box(rocketcss_nano::minify(
                        &mut input.stylesheet,
                        &mut token.borrow_mut(),
                        rocketcss_nano::MinifyOptions::default(),
                    ));
                });
        });
    }

    #[divan::bench(args = BENCH_CASES)]
    fn lightningcss(bencher: Bencher<'_, '_>, case: BenchCase) {
        use lightningcss::stylesheet::{MinifyOptions, ParserOptions, StyleSheet};

        bencher
            .counter(BytesCount::of_str(case.source))
            .with_inputs(|| StyleSheet::parse(case.source, ParserOptions::default()).unwrap())
            .bench_local_refs(|stylesheet| {
                stylesheet.minify(MinifyOptions::default()).unwrap();
                black_box(stylesheet);
            });
    }
}

mod codegen {
    use super::*;

    #[divan::bench(args = BENCH_CASES)]
    fn rocketcss(bencher: Bencher<'_, '_>, case: BenchCase) {
        rocketcss_allocator::GhostToken::scope(|token| {
            let token = RefCell::new(token);
            bencher
                .counter(BytesCount::of_str(case.source))
                .with_inputs(|| ParsedStyleSheet::minified(case.source, &mut token.borrow_mut()))
                .bench_local_refs(|input| {
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
                });
        });
    }

    #[divan::bench(args = BENCH_CASES)]
    fn lightningcss(bencher: Bencher<'_, '_>, case: BenchCase) {
        use lightningcss::stylesheet::{
            MinifyOptions, ParserOptions, PrinterOptions as LightningPrinterOptions, StyleSheet,
        };

        bencher
            .counter(BytesCount::of_str(case.source))
            .with_inputs(|| {
                let mut stylesheet =
                    StyleSheet::parse(case.source, ParserOptions::default()).unwrap();
                stylesheet.minify(MinifyOptions::default()).unwrap();
                stylesheet
            })
            .bench_local_refs(|stylesheet| {
                black_box(
                    stylesheet
                        .to_css(LightningPrinterOptions {
                            minify: true,
                            ..LightningPrinterOptions::default()
                        })
                        .unwrap(),
                );
            });
    }
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

/// End-to-end pipeline measurement for cssnano, which cannot be split into
/// per-stage measurements from Rust. The published tables leave its parse,
/// minify, and codegen cells blank; the rocketcss and Lightning CSS totals are
/// the sums of their measured stages instead of separate runs.
mod total {
    use super::*;

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
}
