# Repository test suite

This workspace crate contains cross-crate fixture tests. `rstest` discovers each
`input.css` file at compile time and exposes it as an individual Rust test case.

Fixtures are grouped by the stage under test:

- `fixtures/parser/pass`: inputs that must parse successfully.
- `fixtures/parser/fail`: inputs that must produce a parser error.
- `fixtures/codegen`: `input.css` and `output.css` serialization pairs.
- `fixtures/visitor`: `input.css` and `output.css` plugin transformation pairs.

Run only this suite with:

```sh
cargo test -p rs_css_tests
```
