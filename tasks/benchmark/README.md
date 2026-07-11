# Benchmarks

The benchmark targets use CodSpeed's recommended Divan compatibility layer,
installed as the `divan` crate. They continue to run locally with `cargo bench`
and can be measured by CodSpeed with `cargo codspeed build` and `cargo codspeed run`.

Run the tokenizer comparison with:

```sh
cargo bench -p benchmark --bench tokenizer
```

Run the parser comparison with:

```sh
cargo bench -p benchmark --bench parser
```

Run the code generator comparison with:

```sh
cargo bench -p benchmark --bench codegen
```

Run the Bootstrap minifier comparison with:

```sh
cargo bench -p benchmark --bench minify
```

The minifier benchmark compares `rs-css`, Lightning CSS, and cssnano using the
same unminified `bootstrap.css` input. Each measured iteration includes parsing,
minification, and serialization. cssnano runs in a persistent Node.js process;
its processor is initialized once, and timing is measured inside that process so
Node startup and Rust/Node IPC are excluded.

By default, the benchmark loads cssnano from a sibling `cssnano` checkout. Set
`CSSNANO_DIR` when it lives elsewhere, and set `NODE` to override the Node.js
executable:

```sh
CSSNANO_DIR=/path/to/cssnano NODE=/path/to/node \
  cargo bench -p benchmark --bench minify
```

The tokenizer benchmark compares `rs_css_parser::Tokenizer` with
`css_module_lexer::collect_dependencies`, matching css-module-lexer's own
benchmark entry point. The latter also performs CSS module dependency analysis
because the crate does not expose its raw lexer visitor API.

The parser benchmark stops after each implementation produces its stylesheet
syntax tree. It does not run transformation, minification, or serialization.

The code generator benchmark parses each stylesheet once outside the measured
loop, then compares compact serialization only. Parsing and AST minification
are not included in its timings.
