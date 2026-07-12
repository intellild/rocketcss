#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::ffi::OsString;
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::OnceLock;
use std::time::Duration;

use divan::{Bencher, black_box, counter::BytesCount};
use rocketcss_allocator::Allocator;
use rocketcss_ast::StyleSheet as RocketStyleSheet;
use rocketcss_codegen::{PrinterOptions, ToCss};

const BOOTSTRAP: &str = include_str!("../files/bootstrap.css");
static CONDITIONAL_RUN: OnceLock<String> = OnceLock::new();

fn main() {
    divan::main();
}

fn conditional_run() -> &'static str {
    CONDITIONAL_RUN.get_or_init(|| conditional_run_fixture(256))
}

struct RocketMinifyInput {
    stylesheet: RocketStyleSheet<'static>,
    _allocator: Box<Allocator>,
}

impl RocketMinifyInput {
    fn parse(source: &'static str) -> Self {
        let allocator = Box::new(Allocator::new());

        // The allocator has a stable heap address and is kept alive by this
        // input until after the stylesheet is dropped.
        let allocator_ref: &'static Allocator = unsafe { &*std::ptr::from_ref(&*allocator) };
        let stylesheet = rocketcss_parser::parse(
            source,
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

#[divan::bench]
fn rocketcss_parse(bencher: Bencher<'_, '_>) {
    bencher
        .counter(BytesCount::of_str(BOOTSTRAP))
        .bench_local(|| {
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
        });
}

#[divan::bench]
fn rocketcss_parse_minify(bencher: Bencher<'_, '_>) {
    bencher
        .counter(BytesCount::of_str(BOOTSTRAP))
        .bench_local(|| {
            let allocator = Allocator::new();
            let mut stylesheet = rocketcss_parser::parse(
                black_box(BOOTSTRAP),
                &allocator,
                rocketcss_parser::ParserOptions {
                    error_recovery: true,
                    ..Default::default()
                },
            )
            .unwrap();
            rocketcss_minify::minify(&mut stylesheet, rocketcss_minify::MinifyOptions::default());
            black_box(stylesheet);
        });
}

#[divan::bench]
fn rocketcss_minify_only(bencher: Bencher<'_, '_>) {
    bencher
        .counter(BytesCount::of_str(BOOTSTRAP))
        .with_inputs(|| RocketMinifyInput::parse(BOOTSTRAP))
        .bench_local_refs(|input| {
            black_box(rocketcss_minify::minify(
                &mut input.stylesheet,
                rocketcss_minify::MinifyOptions::default(),
            ));
        });
}

#[divan::bench]
fn rocketcss(bencher: Bencher<'_, '_>) {
    bencher
        .counter(BytesCount::of_str(BOOTSTRAP))
        .bench_local(|| {
            let allocator = Allocator::new();
            let mut stylesheet = rocketcss_parser::parse(
                black_box(BOOTSTRAP),
                &allocator,
                rocketcss_parser::ParserOptions {
                    error_recovery: true,
                    ..Default::default()
                },
            )
            .unwrap();
            rocketcss_minify::minify(&mut stylesheet, rocketcss_minify::MinifyOptions::default());
            let output = stylesheet
                .to_css_string(PrinterOptions { prettify: false })
                .unwrap();
            black_box(output);
        });
}

#[divan::bench]
fn rocketcss_conditional_run(bencher: Bencher<'_, '_>) {
    let source = conditional_run();
    bencher.counter(BytesCount::of_str(source)).bench_local(|| {
        let allocator = Allocator::new();
        let mut stylesheet = rocketcss_parser::parse(
            black_box(source),
            &allocator,
            rocketcss_parser::ParserOptions {
                error_recovery: true,
                ..Default::default()
            },
        )
        .unwrap();
        rocketcss_minify::minify(&mut stylesheet, rocketcss_minify::MinifyOptions::default());
        let output = stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap();
        black_box(output);
    });
}

#[divan::bench]
fn rocketcss_conditional_run_minify_only(bencher: Bencher<'_, '_>) {
    let source = conditional_run();
    bencher
        .counter(BytesCount::of_str(source))
        .with_inputs(|| RocketMinifyInput::parse(source))
        .bench_local_refs(|input| {
            black_box(rocketcss_minify::minify(
                &mut input.stylesheet,
                rocketcss_minify::MinifyOptions::default(),
            ));
        });
}

#[divan::bench]
fn lightningcss(bencher: Bencher<'_, '_>) {
    use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};

    bencher
        .counter(BytesCount::of_str(BOOTSTRAP))
        .bench_local(|| {
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

#[divan::bench]
fn lightningcss_conditional_run(bencher: Bencher<'_, '_>) {
    use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};

    let source = conditional_run();
    bencher.counter(BytesCount::of_str(source)).bench_local(|| {
        let mut stylesheet =
            StyleSheet::parse(black_box(source), ParserOptions::default()).unwrap();
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

#[divan::bench]
fn lightningcss_minify_only(bencher: Bencher<'_, '_>) {
    use lightningcss::stylesheet::{MinifyOptions, ParserOptions, StyleSheet};

    bencher
        .counter(BytesCount::of_str(BOOTSTRAP))
        .with_inputs(|| StyleSheet::parse(BOOTSTRAP, ParserOptions::default()).unwrap())
        .bench_local_refs(|stylesheet| {
            stylesheet.minify(MinifyOptions::default()).unwrap();
            black_box(stylesheet);
        });
}

#[divan::bench]
fn lightningcss_conditional_run_minify_only(bencher: Bencher<'_, '_>) {
    use lightningcss::stylesheet::{MinifyOptions, ParserOptions, StyleSheet};

    let source = conditional_run();
    bencher
        .counter(BytesCount::of_str(source))
        .with_inputs(|| StyleSheet::parse(source, ParserOptions::default()).unwrap())
        .bench_local_refs(|stylesheet| {
            stylesheet.minify(MinifyOptions::default()).unwrap();
            black_box(stylesheet);
        });
}

// Copied from Lightning CSS's `stylesheet/transform[conditional-run]`
// benchmark, introduced by parcel-bundler/lightningcss#1271 and optimized by
// parcel-bundler/lightningcss#1263.
fn conditional_run_fixture(rule_count: usize) -> String {
    let mut css = String::with_capacity(rule_count * 720);

    for i in 0..rule_count {
        let hue = (i * 17) % 360;
        writeln!(
            css,
            r#"
@media (min-width: 768px) {{
  .media-card-{i} {{
    color: hsl({hue}deg 52% 28%);
    display: grid;
    gap: {gap}px;
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }}
}}
"#,
            gap = 8 + (i % 12),
        )
        .unwrap();
    }

    for i in 0..rule_count {
        let width = 420 + (i % 24);
        writeln!(
            css,
            r#"

@supports (container-type: inline-size) {{
  .supports-card-{i} {{
    container-type: inline-size;
    inline-size: min(100%, {width}px);
  }}
}}
"#,
        )
        .unwrap();
    }

    for i in 0..rule_count {
        writeln!(
            css,
            r#"

@container (min-width: 32rem) {{
  .container-card-{i} {{
    padding: {padding}px;
  }}
}}
"#,
            padding = 12 + (i % 10),
        )
        .unwrap();
    }

    css
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

#[divan::bench]
fn cssnano(bencher: Bencher<'_, '_>) {
    let mut cssnano = CssnanoWorker::spawn();
    bencher
        .counter(BytesCount::of_str(BOOTSTRAP))
        .bench_local(|| black_box(cssnano.run(1)));
}
