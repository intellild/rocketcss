# Benchmarks

Run the tokenizer comparison with:

```sh
cargo bench -p benchmark --bench tokenizer
```

Run the parser comparison with:

```sh
cargo bench -p benchmark --bench parser
```

The tokenizer benchmark compares `rs_css_parser::Tokenizer` with
`css_module_lexer::collect_dependencies`, matching css-module-lexer's own
benchmark entry point. The latter also performs CSS module dependency analysis
because the crate does not expose its raw lexer visitor API.

The parser benchmark stops after each implementation produces its stylesheet
syntax tree. It does not run transformation, minification, or serialization.
