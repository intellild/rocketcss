# Benchmarks

The benchmark targets use CodSpeed's recommended Divan compatibility layer,
installed as the `divan` crate. They continue to run locally with `cargo bench`
and can be measured by CodSpeed with `cargo codspeed build` and `cargo codspeed run`.

CodSpeed tracks the `pipeline` target, which measures Rocket CSS parsing,
minification, and code generation as separate stages. Input preparation for the
minify and codegen stages is excluded from their measurements. Each measured
sample runs its stage 10 times to reduce fixed-cost noise in CodSpeed's
single-execution simulation mode.

Run the same pipeline benchmark locally with:

```sh
cargo bench -p rocketcss_benchmark --bench pipeline
```

Run the tokenizer comparison with:

```sh
cargo bench -p rocketcss_benchmark --bench tokenizer
```

Run the parser comparison with:

```sh
cargo bench -p rocketcss_benchmark --bench parser
```

Run the code generator comparison with:

```sh
cargo bench -p rocketcss_benchmark --bench codegen
```

Run the Bootstrap minifier comparison with:

```sh
cargo bench -p rocketcss_benchmark --bench minify
```

The minifier benchmark compares `rocketcss`, Lightning CSS, and cssnano using the
same unminified `bootstrap.css` input. Each measured iteration includes parsing,
minification, and serialization. cssnano runs in a persistent Node.js process;
its processor is initialized once, so Node startup is excluded. The cssnano
measurement includes the Rust/Node IPC round trip because Divan measures the
worker request from the Rust side.

`rocketcss` currently runs only node-local, in-place normalization, while the other
tools include broader cross-rule passes. Treat this as an implementation-cost
comparison rather than feature-equivalent minifier throughput.

By default, the benchmark loads cssnano from a sibling `cssnano` checkout. When
that checkout is not available (for example in CI, including the CodSpeed run),
the cssnano comparison is skipped instead of failing the benchmark run. Set
`CSSNANO_DIR` when it lives elsewhere, and set `NODE` to override the Node.js
executable:

```sh
CSSNANO_DIR=/path/to/cssnano NODE=/path/to/node \
  cargo bench -p rocketcss_benchmark --bench minify
```

The tokenizer benchmark compares `rocketcss_parser::Tokenizer` with
`css_module_lexer::collect_dependencies`, matching css-module-lexer's own
benchmark entry point. The latter also performs CSS module dependency analysis
because the crate does not expose its raw lexer visitor API.

The parser benchmark stops after each implementation produces its stylesheet
syntax tree. It does not run transformation, minification, or serialization.

The code generator benchmark parses each stylesheet once outside the measured
loop, then compares compact serialization only. Parsing and AST minification
are not included in its timings.
