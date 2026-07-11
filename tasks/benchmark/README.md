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

The tokenizer benchmark compares `rs_css_parser::Tokenizer` with
`css_module_lexer::collect_dependencies`, matching css-module-lexer's own
benchmark entry point. The latter also performs CSS module dependency analysis
because the crate does not expose its raw lexer visitor API.

The parser benchmark stops after each implementation produces its stylesheet
syntax tree. It does not run transformation, minification, or serialization.

The code generator benchmark parses each stylesheet once outside the measured
loop, then compares compact serialization only. Parsing and AST minification
are not included in its timings.
