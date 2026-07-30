# Cross-rule declaration merging: performance

This document records performance observations and optimization candidates. It
does not change the correctness model in the S1-S5 design documents.

## S1 selector matching

### Keep structural equality as the default

An experiment cached a full `FxHasher` fingerprint for every style rule that
participated in an S1 candidate. It regressed the CodSpeed minify benchmarks:

| Workload  | Direct equality | Fingerprint | Candidate discovery |
| --------- | --------------: | ----------: | ------------------: |
| Bootstrap |          1.4 ms |      2.3 ms |              1.2 ms |
| Tailwind  |          3.1 ms |      2.8 ms |              1.5 ms |

The fingerprint path still had to run structural equality after matching
hashes. Most adjacent selectors differ, so direct equality commonly stops at
the first unequal selector component while hashing must visit every live
selector component. An S1 rule participates in at most two adjacent
comparisons, which is not enough reuse to amortize a full hash.

S1 should therefore use `equal_live_selectors` directly unless a selector
identity is already computed by an earlier pass and can be reused without an
additional selector traversal. A future shared identity must preserve
tombstone filtering and must only be a prefilter; structural equality remains
the collision-safe final check.

### Structural adjacency discovery

S1 candidates must be direct siblings in the same rule list. Rewalking the
complete stylesheet after collecting a flat `Vec<DeclarationBlockEntry>`
enforces that rule at commit time, but costs another 1-2 ms on the current
Bootstrap and Tailwind inputs. It also performs two declaration-block pointer
map lookups for every adjacent style-rule pair.

`DeclarationBlockEntry` now records `RuleListId`, `RuleListSegmentId`, and
`SiblingOrdinal`, and candidate discovery rejects entries whose structural
locations do not form a direct edge.

## Allocation and indexing candidates

- Reserve the declaration-block output vector when a cheap rule-list size
  estimate is available. Do not add a counting prepass solely to obtain an
  exact capacity.
- Allocate per-rule caches lazily and only when their reuse is demonstrated.
  Avoid zero-initializing an `Option` entry for every style rule for a cache
  used by only one stage.
- Keep `Candidate(u32, u32)` and compact queue state for adjacency-based S1/S3
  work. S2 queues a compact history identity instead of a block pair. Do not
  store selector clones or declaration snapshots in either representation.

### Compiler-scoped selector atoms

Selector equality and hashing repeatedly compare identifiers such as type,
class, ID, namespace, attribute, and custom pseudo names. These values are now
stored as `Atom` rather than independent `&str` slices. `Compiler` owns one
compilation-scoped `StringPool`, and the parser interns every selector string
through that pool before constructing the AST.

`StringPool` uses `rocketcss_allocator::HashMap`, so both its entries and hash
table storage have arena lifetime. The first occurrence copies the string into
the arena; subsequent occurrences reuse that allocation. `Atom` equality,
ordering, and hashing use only the canonical string pointer. Comparing an
`Atom` with `str` remains a content comparison for diagnostics and API
boundaries.

Pointer identity is valid only inside one compiler pool. Consequently:

- selector `Atom` constructors are not exposed for arbitrary `&str` values;
- nested selector inputs must reuse the parent `Compiler` string pool;
- mutable visitors must obtain replacement atoms from the owning compiler's
  pool;
- cloning an AST into another arena must re-intern its selector strings in the
  destination compiler; and
- APIs comparing ASTs from different compiler instances must compare
  `Atom::as_str()` explicitly rather than relying on `Atom` equality.

The compiler also owns the single source name and source-map URL associated
with the stylesheet. They are not stored as one-element vectors in
`StyleSheet`.

### Effective-key path cost

The current walker clones both the selector-frame path and conditional-context
path into every `DeclarationBlockEntry`. Even a top-level style rule allocates
one selector-path `Vec`; deeply nested styles copy every ancestor frame again.
This makes discovery allocate and copy `O(blocks * nesting depth)` references
before candidate processing starts.

## Deferred optimization candidates

### Compress initial maximal same-selector runs

After compact effective-rule identities exist, initial discovery can recognize
a maximal run of live-adjacent, S1-eligible rules and create one ordered
declaration-sequence concatenation instead of enqueueing every pair in the run.
For example, `a{} a{} a{} a{}` can become one initial run rather than three
independent candidate items.

This is only an initial-discovery fast path. Edges exposed later by S2/S3/S4
still use the ordinary dirty-edge queue and must be revalidated individually.
The optimization does not improve asymptotic complexity, and ordinary
stylesheets rarely contain long equal-selector runs, so it has lower priority
than compact effective-key IDs, history-based S2 scheduling, and persistent
live-rule state.

The normative pseudocode's `coalesce_same_selector_run` already requires eager
S1 stabilization. This candidate only batches the initially authored maximal
run into less queue and sequence-concatenation work; it does not change that
semantic transition.

Benchmark it only with a dedicated repeated-selector fixture and keep it only
if it reduces total minify time without increasing the common unequal-selector
path.

### Declaration sequence reuse

`DeclarationBlockMinifier` already has a ghost-backed sequence abstraction for
processing multiple declaration blocks. Its full block-local rewrite pipeline
must not be reused by incremental S2: folding a later longhand into an earlier
shorthand, or combining longhands at one block location, can move effects
across an intervening overlapping rule.

The safe incremental API should expose the narrow operation directly:

```rust,ignore
minify(block)                       // full block-local rewrite
deduplicate_exact_sequence(blocks) // cross-block exact pruning only
```

Keeping separate entry points prevents future property IR from accidentally
participating in cross-block rewrites. The exact sequence path may share the
packed declaration location and exact-property maps, but it must not enter box,
columns, or other relational rewrite state.

The minifier should retain only the declaration IR allocated from the scratch
allocator. It does not need to cache a separate allocator field when no caller
reads it.

### Hash complete conditional contexts before optimizing S2 buckets

S2 candidate discovery buckets declaration blocks by
`EffectiveKey::fingerprint` and then uses exact typed equality inside each
bucket. The current conditional fingerprint is deliberately incomplete:
media queries hash their shape but not the condition contents, while supports
and container conditions mostly hash their discriminants and presence.

Consequently, ordinary stylesheets containing many distinct conditions with
the same shape can place every entry in one bucket:

```css
@media (min-width: 1px) { a { x: 1 } }
@media (min-width: 2px) { a { x: 2 } }
@media (min-width: 3px) { a { x: 3 } }
```

The collision-safe linear equality check then makes discovery quadratic in the
number of distinct contexts even when no S2 history is produced. A later
optimization should hash the complete typed media, supports, and container
condition AST, while retaining exact `EffectiveKey` equality as the final
proof.

This is intentionally deferred until the S2 correctness boundary is fixed.
Fingerprint work must not broaden history identity or make hashes authoritative
for semantic equality. If a compact-ID implementation removes
`EffectiveKey::fingerprint`, apply the same requirement to the interner's frame
hash: hash the complete typed condition before resolving collisions with exact
equality.

## Custom-property token traversal fusion

Status: implemented for the default combined comment-discard and whitespace
normalization path.

Custom properties previously traversed their token trees and top-level token
lists separately:

```text
Minifier::visit_custom_property
  set the custom-property value context
       ↓
  node.visit_mut_children
    recursively visit and minify every TokenOrValue
       ↓
  CustomProperty::minify
       ↓
  Vec<TokenOrValue>::minify
    rescan the top-level list for adjacency, comment/whitespace compaction,
    URL normalization, and context-specific value transforms
```

This guaranteed the required post-order behavior, but read every top-level
custom-property value at least twice. Custom properties with long token lists
or deeply nested functions amplify visitor dispatch and cache traffic.

The specialized custom-property value walker now fuses the recursive visitor
step with the first token-list minify pass. After visiting and minifying an
entry's children, the same outer scan updates the neighbor state used by
function-replacement protection and compacts comments and whitespace through a
write cursor. Transformations that require the finalized whole list still run
afterward.

The general AST visitor remains the fallback for independent option
combinations; the optimization does not blindly fold all
`Vec<TokenOrValue>::minify` operations into the fused scan. Whole-list and
order-sensitive transforms continue to observe the compacted post-order
result. The fused path preserves:

- the custom-property value context, including the `--font-family` special
  case;
- invalid-function and raw-token skip behavior;
- nested function, variable, and environment-variable post-order minification;
- comment and whitespace option combinations and separator identity;
- arena-backed token identity where the current implementation reuses nodes;
  and
- minification statistics.

Benchmark the implementation as an isolated change. The expected benefit
depends on custom-property token volume; removing one traversal does not avoid
the later whole-list passes required by property-specific transforms.

## Single-pass RGB argument validation

Status: optimization candidate.

`collect_tokens_impl` currently builds the complete argument list for every
function. RGB and RGBA functions then call `is_supported_rgb_function`, which
iterates the resulting `TokenOrValue` list again to decide whether the function
can use the typed color path. Typed color parsing and custom-property embedded
value parsing both need this result, so delaying validation until minification
would change the parser's lossless fallback boundary.

RGB syntax can instead be validated while its immediate arguments are being
collected. Use a compact state machine that consumes the original `ValueToken`
stream and recognizes the currently supported forms:

```text
legacy:
  component , component , component
  component , component , component , alpha

modern:
  component component component
  component component component / alpha
```

The state machine must preserve the existing distinctions:

- whitespace is ignored, but comments make the RGB syntax unsupported;
- legacy components must all be numbers or all be percentages;
- modern components may mix numbers, percentages, and `none`;
- legacy alpha accepts a number or percentage, while modern alpha also accepts
  `none`;
- nested functions, variables, grouping blocks, and additional significant
  tokens make the outer RGB function unsupported; and
- collection continues after the state becomes invalid so serialization
  remains lossless.

Only the immediate argument stream belongs to a validator. A nested RGB
function receives its own state machine, while its parent observes a single
function token and rejects it as an RGB component. The completed validation
result should be shared by typed color parsing and the generic function path,
removing both post-collection argument scans.

Do not pass an optional validator through the ordinary collection path and add
a runtime branch for every token. RGB functions are relatively uncommon and
their argument lists are short, so that overhead could exceed the saved
traversal. Prefer a no-op and RGB observer selected through generic or
const-generic dispatch so the ordinary path compiles without validation work.
Cache the function's typed kind for validation and embedded-color
classification rather than repeatedly classifying its name.

Benchmark this as an isolated parser change with both RGB-heavy input and the
Bootstrap and Tailwind workloads. Inspect code size as well as runtime because
specializing the collector can produce a second monomorphized copy. The
optimization is expected to be sub-millisecond on representative inputs; it
should be accepted only if function-heavy inputs without RGB do not regress.
## Benchmarking method

Evaluate performance changes as isolated commits on CodSpeed. Compare matching
runners where possible and inspect callgraph costs when runner hardware
differs. Local one-shot process timing includes process startup and setup costs
and is not precise enough to validate sub-millisecond changes in the minify
pass.
