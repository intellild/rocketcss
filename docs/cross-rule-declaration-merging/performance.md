# Cross-rule declaration merging performance

This document records optimization candidates that are intentionally deferred
while the S1-S5 design is implemented incrementally.

## Implemented

- Compiler-internal hash collections use `rustc_hash`.
- The S1 FIFO does not maintain a redundant duplicate-tracking set.
- StyleRule collection reserves capacity while entering each rule list.
- The S1 candidate list discovers only structurally committable sibling pairs.
- Selector fingerprints are cached lazily and used as collision-safe rejection
  filters.
- S1 commit trusts the immutable candidate plan and does not repeat selector
  equality.

## Deferred AST and traversal work

### Stable StyleRule indices in the AST traversal model

The current implementation builds a pointer-to-`u32` map after collecting
StyleRules. A future AST traversal/indexing refactor should assign stable
StyleRule indices while walking the tree and expose them to S1-S3. This would
remove the pointer map without creating another parallel identity system.

### Streaming commit by candidate ordinal

Accepted candidates are ordered by StyleRule index, but commit currently walks
the AST and performs hash lookups for both endpoints and the candidate pair.
After stable AST indices exist, commit can advance one candidate cursor during
the mutable tree walk and eliminate the commit-side maps and sets.

### Collect cross-rule metadata during normal minification

Selector fingerprints, live-selector counts, effective-rule context, and
emission identity can eventually be recorded after rule-local selector
normalization. Reusing that metadata would remove the separate read-only
StyleRule collection pass. This should be done as part of the AST/indexing
refactor rather than by coupling the scanner to visitor internals.

## Future S2 and S3 queue work

- Re-evaluate whether S2 histories need candidate-pair queues or a dirty-history
  queue keyed by the effective-rule history identifier.
- Benchmark the S3 `BTreeSet` before activation. If ordered extraction and
  duplicate suppression are both required at scale, compare it with a compact
  priority heap plus `FxHashSet`, or an index-addressed state table.
- Share cached effective-rule and declaration-effect fingerprints across S1,
  S2, and S3 instead of maintaining phase-specific hashes.
- Remove the dormant multi-block `DeclarationBlockMinifier::minify_sequence`
  path if S2 owns cross-block effect pruning, so the compiler does not retain
  two competing cross-block declaration algorithms.

## Unrelated minifier hotspots

Token/value minification remains a larger whole-pipeline hotspot than S1.
Potential work includes fusing repeated token-vector scans, caching variable
presence checks, and reducing recursive function visitation. These changes
should be measured and implemented independently from cross-rule merging.
