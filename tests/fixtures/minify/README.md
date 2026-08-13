# Minify upstream fixtures

These input/output pairs are direct Rust fixture adaptations of tests from the
local upstream checkouts:

- `cssnano/`: `/data00/home/jinzhixin/rstack/cssnano/packages/*/test`
- `lightningcss/`: `/data00/home/jinzhixin/rstack/lightningcss/src/lib.rs`
- `rocketcss/`: RocketCSS-owned regression fixtures organized with the same
  `category/case/input.css` and `output.css` layout.

The original JavaScript/Rust test runners are replaced by the repository's
shared `rstest` harness. CSS input and expected output are otherwise kept as
the authoritative parts of each upstream test.

`cssnano-extra/` holds RocketCSS-only fixtures that have no counterpart in the
upstream cssnano test suites. Keep `cssnano/` itself limited to cases ported
from upstream.

`rocketcss/cross-rule-declaration-merging/` contains the local declaration
state-machine regressions. Its `state-machine/comprehensive/` fixture also
records Lightning CSS and cssnano output next to RocketCSS's expected output so
the three tools' current coverage can be compared directly.

All copied fixtures supported by the current pipeline run by default. Precise
static gaps remain visible in `tests/src/minify.rs`; dynamic CSSNano cases keep
their skip reasons beside each case in `minify-dynamic/cssnano/*.json`.

The 2026-08-13 audit runs 35 of 53 CSSNano static pairs and 15 of 32 Lightning
CSS static pairs. It also runs 1239 of 2105 recorded CSSNano dynamic cases,
with one upstream-disabled case and 865 explicit RocketCSS gaps. S5-specific
guards cover CSSNano declaration deduplication and Lightning selector merging
so a future broad skip cannot silently disable those groups.
