---
name: sync-upstream-css-tests
description: Audit, diff, and synchronize RocketCSS test coverage against local Lightning CSS, CSSNano, and Stylo sources. Use when checking whether minify fixtures cover upstream tests, refreshing exact upstream test-source snapshots, inspecting upstream test drift, adding a CI check for unsynchronized CSS minifier tests, or regenerating the Lightning/Stylo parser corpus from instrumented logs and Rust sources.
---

# Sync upstream CSS tests

Use the bundled zx script to keep two distinct facts visible:

1. The upstream test sources copied into `tests/upstream-sources` are current.
2. Upstream behavior has been ported into runnable RocketCSS fixtures.

Never treat source synchronization as a coverage claim. Lightning CSS keeps
many tests inline in Rust, and CSSNano creates cases dynamically through
JavaScript helpers and loops. Static declaration counts are an audit aid, not
the exact number of runtime cases.

## Workflow

Run commands from the RocketCSS repository root:

```sh
pnpm upstream-tests status
pnpm upstream-tests diff --stat
pnpm upstream-tests diff
pnpm upstream-tests sync
pnpm upstream-tests check
```

- Run `status` first. Report local input/output pairs, executed/skipped fixture
  counts, upstream source counts, and snapshot drift separately.
- Run `diff` to inspect changes without modifying the repository. Add
  `--name-only` or `--stat` when a full diff would be too large.
- Run `sync` only when the user intends to update the byte-for-byte source
  snapshot and its SHA-256 manifest. Review the resulting diff afterward.
- Run `check` in CI or validation workflows. It exits non-zero on drift.

By default, locate upstream repositories at `../lightningcss` and `../cssnano`.
Override them with `--lightningcss`, `--cssnano`, `LIGHTNINGCSS_DIR`, or
`CSSNANO_DIR`. Use `--project lightningcss` or `--project cssnano` to limit the
operation. Use `--snapshot` for isolated validation in a temporary directory.

After synchronizing, port relevant cases into `tests/fixtures/minify`. If a
case cannot run yet, preserve it and add an explicit skip in
`tests/src/minify.rs`; do not report it as covered.

The implementation is in `scripts/upstream-tests.mjs`. Keep its file selection
rules broad enough to include `test`, `tests`, and `__tests__` directories,
JavaScript `*.test.*`/`*.spec.*` files, CSSNano test helpers, and Rust files
containing inline tests.

## Parser corpus

Use the bundled zx lexer when the checked-in text-to-AST corpus must be
regenerated:

```sh
pnpm parser-test-corpus \
  --lightningcss /path/to/lightningcss/src/lib.rs \
  --lightningcss-log /path/to/lightningcss-test.log \
  --stylo-selectors /path/to/selectors/parser.rs \
  --output crates/parser/tests/upstream/corpus.json
```

The Lightning log must come from an instrumented single-threaded upstream test
run and contain records in this format:

```text
ROCKETCSS_CORPUS|<source-line>|<ok>|<error-recovery>|<css-modules>|<flags>|<debug-source>|ROCKETCSS_CORPUS_END
```

The generator reads dynamic Lightning parser calls from the log, reads its
custom-parser and serde companion tests from source, and extracts static Stylo
selector cases. It counts parser errors for auditing but omits them from the
AST-producing case arrays. The implementation is
`scripts/generate-parser-test-corpus.mjs`; do not reintroduce a Python runtime
dependency.
