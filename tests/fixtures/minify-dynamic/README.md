# Minify dynamic fixtures

The JSON specs in `cssnano/` are recorded from the upstream CSSNano test
suites. Many upstream cases are built dynamically — helpers wrap values in
declaration templates and suites loop over keyword lists — so they cannot be
captured as static input/output pairs. The recorder in
`.agents/skills/sync-upstream-css-tests/scripts/record-dynamic-cases.mjs`
executes the upstream test files with the test runner stubbed out and stores
every concrete `(input, expected)` pair it sees.

Regenerate the specs after syncing upstream sources:

```sh
cd .agents/skills/sync-upstream-css-tests
node_modules/.bin/zx scripts/record-dynamic-cases.mjs
```

By default the recorder reads the checked-in snapshot at
`tests/upstream-sources/cssnano`; pass `--cssnano /path/to/cssnano` to record
from a live checkout (which also captures cases built from runtime data files
that the snapshot does not contain).

At test time `tests/src/minify_dynamic.rs` expands each case — bare
declarations are wrapped in a rule — and runs it through the same
parse/minify/print pipeline as the static fixtures. Cases that RocketCSS
cannot handle yet are kept in the specs and skipped explicitly in
`still_requires_unsupported_transform`, mirroring `tests/src/minify.rs`.
