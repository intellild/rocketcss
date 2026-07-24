# Cross-rule declaration merging comparison

This fixture concatenates every focused cross-rule declaration-merging scenario
inside an independent named layer. The layers prevent one scenario from
creating histories or adjacency edges with another scenario.

- `output.css`: RocketCSS default minifier output and the fixture expectation.
- `output.lightningcss.css`: local Lightning CSS default minifier output.
- `output.cssnano.css`: local cssnano default-preset output.

All outputs were produced without browser targets. They are comparison
artifacts; only `output.css` is asserted by the Rust fixture harness.

Recorded local revisions:

- RocketCSS: `1525d03`
- Lightning CSS: `91bfa3f`
- cssnano: `da445320`

Regenerate the comparison outputs with each project's default minifier:

```sh
cargo run --manifest-path /path/to/lightningcss/Cargo.toml --features cli \
  --bin lightningcss -- input.css --minify \
  --output-file output.lightningcss.css

node tests/scripts/minify-with-cssnano.mjs \
  tests/fixtures/minify/rocketcss/cross-rule-declaration-merging/state-machine/comprehensive/input.css \
  tests/fixtures/minify/rocketcss/cross-rule-declaration-merging/state-machine/comprehensive/output.cssnano.css
```

Set `CSSNANO_DIR` when the cssnano checkout is not the default sibling
repository.

## Current coverage highlights

| Area | RocketCSS | Lightning CSS | cssnano |
| --- | --- | --- | --- |
| Adjacent equal-selector coalescing | Yes | Yes | Yes |
| Complete equal-declaration factoring | Yes | Yes for supported selectors | Yes |
| Partial declaration factoring | Yes, including candidate invalidation | Generally retained as separate rules | Yes for several simple ranges |
| Same-selector declaration history pruning | Yes | Mostly limited to declarations merged into one rule | Does not model the cross-rule history cases here |
| Nested-declaration barriers and exposed-edge fixed point | Yes | Preserves or rewrites nesting without the same history model | Preserves nesting without the same history model |
| Synthesized selector deduplication | Yes | Case 036 retains a duplicate selector arm | Yes |
| Separately authored same-name layers | Kept as distinct contexts | Coalesced | Coalesced |
| Complex selector materialization | Conservative/deferred | Covers focus-visible and :has examples | Conservative in these examples |

Notable inspection points:

- 021, 022, 034, 044, and 045 exercise candidate invalidation and worklist
  convergence.
- 027–030 exercise retained children and barriers becoming empty.
- 031–032 compare layer and conditional-wrapper ownership.
- 036 compares synthesized selector deduplication.
- 039–040 compare single-declaration partial factoring.
- 043 checks synthesized history insertion before a later declaration.

## Case index

The three minified output files omit ordinary comments, so use this table to map
each `crdm-NNN` layer back to its focused fixture.

| Layer | Focused fixture | Status |
| --- | --- | --- |
| 001 | `declarations/does-not-drop-live-components-of-a-partially-overridden-shorthand` | deferred |
| 002 | `declarations/keeps-case-distinct-custom-properties` | active |
| 003 | `declarations/keeps-fallback-and-importance-chains` | active |
| 004 | `declarations/keeps-logical-and-physical-properties-when-direction-is-not-proven` | active |
| 005 | `declarations/preserves-properties-not-reset-by-all` | active |
| 006 | `declarations/treats-revert-values-conservatively` | active |
| 007 | `nesting/nested-declarations-break-style-rule-adjacency` | active |
| 008 | `nesting/opaque-at-rule-content-keeps-ancestor-style-rule-live` | active |
| 009 | `nesting/preserves-at-nest-wrapper-identity` | active |
| 010 | `nesting/resolves-multiple-and-functional-nesting-selectors` | active |
| 011 | `nesting/treats-custom-vendor-pseudo-elements-as-factoring-barriers` | active |
| 012 | `nesting/treats-pseudo-element-nesting-as-a-conservative-barrier` | active |
| 013 | `real-world/does-not-expand-bootstrap-modal-selectors` | deferred |
| 014 | `real-world/does-not-expand-tailwind-screen-reader-utilities` | deferred |
| 015 | `real-world/factors-tailwind-mask-setup-without-reordering-custom-values` | active |
| 016 | `real-world/keeps-bootstrap-placeholder-vendor-groups-separate` | active |
| 017 | `real-world/merges-bootstrap-focus-visible-sibling-selectors` | deferred |
| 018 | `real-world/merges-tailwind-matching-webkit-details-marker-selectors` | deferred |
| 019 | `review-findings/ast-ownership/assigns-combined-source-span-to-synthesized-rule` | active |
| 020 | `review-findings/ast-ownership/imports-an-existing-previous-merged-chain-on-a-second-minify` | active |
| 021 | `review-findings/ast-ownership/keeps-overlapping-candidate-rule-ids-stable-across-insertion` | active |
| 022 | `review-findings/ast-ownership/mutates-a-shared-block-handle-without-leaving-a-live-alias` | active |
| 023 | `review-findings/ast-ownership/preserves-importance-and-order-when-one-occurrence-becomes-many` | deferred |
| 024 | `review-findings/ast-ownership/s1-emits-a-retired-left-rule-exactly-once` | active |
| 025 | `review-findings/ast-ownership/synthesized-rules-survive-the-minify-scratch-allocator` | active |
| 026 | `review-findings/ast-ownership/transfers-a-non-clone-custom-declaration-into-the-shared-rule` | active |
| 027 | `review-findings/discovery-and-segments/does-not-unlink-a-parent-before-child-discovery-finishes` | active |
| 028 | `review-findings/discovery-and-segments/ignores-an-initially-empty-nested-declarations-barrier` | active |
| 029 | `review-findings/discovery-and-segments/joins-segments-after-a-nested-declarations-barrier-becomes-empty` | active |
| 030 | `review-findings/discovery-and-segments/joins-segments-after-an-empty-conditional-wrapper` | active |
| 031 | `review-findings/semantics/merges-only-within-the-same-authored-layer-context` | active |
| 032 | `review-findings/semantics/merges-rules-across-adjacent-equal-conditional-wrappers` | active |
| 033 | `review-findings/state-machine/reaches-a-fixed-point-after-s4-exposes-a-new-edge` | active |
| 034 | `review-findings/state-machine/reassigns-all-s1-entries-to-the-combined-sequence` | active |
| 035 | `review-findings/state-machine/s3-endpoint-edits-do-not-create-a-transient-bypass-edge` | active |
| 036 | `state-machine/canonicalizes-synthesized-selector-unions-immediately` | active |
| 037 | `state-machine/complete-factoring-reconnects-the-live-chain-through-the-shared-rule` | active |
| 038 | `state-machine/factors-a-complete-equal-run-in-one-stable-transition` | active |
| 039 | `state-machine/factors-single-declaration-with-left-prefix` | active |
| 040 | `state-machine/factors-single-declaration-with-right-prefix` | active |
| 041 | `state-machine/fingerprint-matches-still-require-exact-value-equality` | active |
| 042 | `state-machine/importance-is-part-of-the-declaration-history-context` | active |
| 043 | `state-machine/inserts-synthesized-history-entries-in-semantic-source-order` | active |
| 044 | `state-machine/invalidates-candidates-when-a-predecessor-block-changes` | active |
| 045 | `state-machine/overlapping-partial-candidates-commit-from-left-to-right` | active |
| 046 | `state-machine/rejects-zero-progress-partial-merge-plans` | active |
| 047 | `state-machine/selector-materialization-failure-leaves-both-endpoints-unchanged` | active |
