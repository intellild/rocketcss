# Repository test suite

This workspace crate contains cross-crate fixture tests. The standard Rust tests
discover each `input.css` file recursively and validate it against the relevant stage.

Fixtures are grouped by the stage under test:

- `fixtures/parser/pass`: inputs that must parse successfully.
- `fixtures/parser/fail`: inputs that must produce a parser error.
- `fixtures/codegen`: `input.css` and `output.css` serialization pairs.
- `fixtures/minify/cssnano`: fixtures adapted from local cssnano package tests.
- `fixtures/minify/lightningcss`: fixtures adapted from local Lightning CSS tests.
- `fixtures/visitor`: `input.css` and `output.css` plugin transformation pairs.

Run only this suite with:

```sh
cargo test -p rs_css_tests
```
