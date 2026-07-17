# Minify upstream fixtures

These input/output pairs are direct Rust fixture adaptations of tests from the
local upstream checkouts:

- `cssnano/`: `/data00/home/jinzhixin/rstack/cssnano/packages/*/test`
- `lightningcss/`: `/data00/home/jinzhixin/rstack/lightningcss/src/lib.rs`

The original JavaScript/Rust test runners are replaced by the repository's
shared `rstest` harness. CSS input and expected output are otherwise kept as
the authoritative parts of each upstream test.

`cssnano-extra/` holds RocketCSS-only fixtures that have no counterpart in the
upstream cssnano test suites. Keep `cssnano/` itself limited to cases ported
from upstream.

The current minifier emphasizes simple, in-place transforms within one AST
node. Fixtures that still require cross-rule/declaration analysis, replacement
AST allocation, or unsupported value transforms are kept for future work and
skipped by `tests/src/minify.rs` rather than deleted.
