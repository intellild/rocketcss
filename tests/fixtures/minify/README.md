# Minify upstream fixtures

These input/output pairs are direct Rust fixture adaptations of tests from the
local upstream checkouts:

- `cssnano/`: `/data00/home/jinzhixin/rstack/cssnano/packages/*/test`
- `lightningcss/`: `/data00/home/jinzhixin/rstack/lightningcss/src/lib.rs`

The original JavaScript/Rust test runners are replaced by the repository's
shared `rstest` harness. CSS input and expected output are otherwise kept as
the authoritative parts of each upstream test.

The current minifier intentionally supports only simple, in-place transforms
within one AST node. Fixtures that require cross-rule/declaration analysis or
replacement AST allocation are kept for future redesign work and skipped by
`tests/src/minify.rs` rather than deleted.
