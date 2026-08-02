# Cross-rule declaration merging: performance

This document records performance observations and optimization candidates. It
does not change the correctness model in the S1-S5 design documents.

## S1 selector matching

### Keep structural equality until canonical IDs already exist

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

S1 should therefore use `equal_live_selectors` directly in the current nested
AST. The [Radix source-order AST IR](../flat-ast-ir/README.md) changes the cost
boundary: selector/context canonicalization happens once during parse and
rule-local normalization, then S1 compares `EffectiveKeyId` values. Exact
structural equality remains the collision-safe check inside the interner, not
on every S1 edge.

### Structural adjacency discovery

S1 candidates must be direct siblings in the same rule list. Rewalking the
complete stylesheet after collecting a flat `Vec<DeclarationBlockEntry>`
enforces that rule at commit time, but costs another 1-2 ms on the current
Bootstrap and Tailwind inputs. It also performs two declaration-block pointer
map lookups for every adjacent style-rule pair.

The transitional `DeclarationBlockEntry` records structural location. The
target Radix AST removes the entry entirely: rule topology stores parent/list
and direct-sibling links, eliminating both the rewalk and pointer map.

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

`StringPool` uses `rocketcss_common::HashMap`, so both its entries and hash
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

The current minify traversal builds compact parent-linked selector and
conditional paths after rule-local normalization. In the target Radix AST,
parsing maintains those paths and writes `EffectiveKeyId` directly on every
declaration block; selector replacement recomputes it immediately.

The compilation-owned interner retains canonical records so an S3 selector
union can receive a key before insertion. Histories are ordered by
`DeclarationBlockId`, including synthesized sibling IDs. Nano does not own a
collector/path store and performs no key-reconstruction traversal.

### Persistent S1-S3 scheduling

The former nested-AST adapter committed one S3 candidate, rebuilt all
declaration-block and S1/S2 state, and restarted source-order discovery. On the
Tailwind fixture this performed 4,020 collections for 4,019 S3 commits and took
about 10.39 seconds locally.

The persistent scheduler keeps block liveness, ordered EffectiveKey histories,
endpoint revisions, and candidate queues across commits. In the target Radix
AST, S3 updates the changed edge/histories and inserts the shared node directly
at its final sibling ID; no later global reification pass restores order. Local
release samples completed Bootstrap in
about 1.0-1.2 ms and Tailwind in about 17-21 ms. Minified output for both
fixtures was byte-identical to the pre-refactor S3 branch.

## Deferred optimization candidates

### Selector storage granularity

The target design requires canonical selector value IDs for effective-key
construction, but does not require every selector component to be independently
interned. Most complete selectors in Bootstrap and Tailwind are unique, so a
hash table for all mutable occurrence objects may cost more than it saves.

Benchmark a flat component tape with `(offset, len)` selector headers against
per-component dense IDs. Keep immutable canonical values at the effective-key
boundary and replace an occurrence ID when selector minification changes it.
Do not retain mutable shared selector nodes or a compatibility layer of arena
references.

## Radix storage and allocation

Current representative type sizes are `CssRule = 16`, `StyleRule = 64`,
`DeclarationBlock = 64`, `Declaration = 32`, `Selector = 32`, and
`SelectorComponent = 40` bytes. A declaration-block header containing two
range words, an effective-key ID, and compact flags can approach 16 bytes,
while declaration payloads remain in a contiguous tape.

The expected wins are no recursive declaration-block discovery, AST-owned
EffectiveKeys, compact ID comparisons, contiguous authored traversal, and
local structural insertion. The risks are semantic-iterator overhead after
insertions, sparse-group lookup, local key exhaustion, and payload
indirection. Measure parse, minify, codegen, peak memory, and output size
separately.

`RadixIndexArena` and large AST payload boxes deliberately use the
compiler-owned allocator. Arena addresses are not semantic IDs, but arena
allocation keeps sparse Radix pages and large payloads stable and cheap to
discard together. Removing allocator infrastructure is no longer a goal of
this storage design; benchmark individual payloads before changing inline
versus boxed ownership.

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

The declaration-block walker buckets effective-key paths by a rolling
fingerprint and then uses exact typed path equality inside each collision
bucket. The current conditional-frame fingerprint is deliberately incomplete:
media queries hash their shape but not the condition contents, while supports
and container conditions mostly hash their discriminants and presence.

Consequently, ordinary stylesheets containing many distinct conditions with
the same shape can place every entry in one bucket:

```css
@media (min-width: 1px) {
  a {
    x: 1;
  }
}
@media (min-width: 2px) {
  a {
    x: 2;
  }
}
@media (min-width: 3px) {
  a {
    x: 3;
  }
}
```

The collision-safe linear equality check then makes discovery quadratic in the
number of distinct contexts even when no S2 history is produced. A later
optimization should hash the complete typed media, supports, and container
condition AST, while retaining exact conditional-frame equality as the final
proof.

This is intentionally deferred until the S2 correctness boundary is fixed.
Fingerprint work must not broaden history identity or make hashes authoritative
for semantic equality. The compact-ID implementation preserves that boundary:
improve the interner's frame hash without removing exact equality as the final
collision check.

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

## PR #50 CodSpeed history

The PR history contains several storage and scheduling experiments, but many
adjacent runs used different CodSpeed Simulation CPU models. Treat a reported
change as causal only when CodSpeed says the relevant benchmarks ran in the
same environment.

The useful signals are:

- storing declaration blocks behind dense IDs improved Bootstrap minify by
  5.01% on matching runners; the other five pipeline benchmarks were
  unchanged;
- moving the dense store abstraction into `rocketcss_common` was unchanged in
  all six benchmarks on matching runners;
- building S1 indices on demand after avoiding non-structural S2 rescans was
  unchanged in all six benchmarks on matching runners, although both minify
  inputs moved slightly in the expected direction;
- arena-backed and flat effective-key bucket layouts were unchanged and the
  flat layout was reverted; and
- the all-at-once flat source-order AST experiment appeared to regress parse by
  24-26%, minify by 40-48%, and codegen by 11-37%, but all six comparisons
  crossed from an AMD EPYC 7763 runner to an AMD EPYC 9V74 runner. Those
  percentages are not a valid A/B measurement and require a same-runner
  reproduction.

This history favors isolated migrations over representation-wide rewrites.
Keep the [Radix AST design](../flat-ast-ir/README.md) as a target, but require a
benchmark gate after each independently useful storage boundary.

### Migrate the declaration tape before the complete rule tree

The only confirmed storage win in this history is the narrow dense
declaration-block change. Implement the global source-order declaration tape
and exact block ranges independently of flat rule, selector, and value
storage. This removes declaration-block pointer chasing and enables range-based
S1/S4 work without forcing parse and codegen through an ID lookup for every AST
node.

The phase must preserve the source-order property allocation contract and
measure parse, minify, codegen, output size, and peak memory. Keep it only if
the isolated result remains positive. A generic `DenseStore` wrapper is useful
for type safety, but its abstraction alone is not a performance optimization.

### Flatten recursive token ownership separately

The flat-AST Tailwind callgraphs show that declaration and rule topology is not
the dominant remaining cost for token-heavy input:

- parse spends 38.27% below `collect_tokens_impl`, while compilation teardown
  accounts for 9.43%;
- minify spends 25.62% below `visit_custom_property`, and recursive
  `TokenOrValue` destruction contributes materially to the measured lifetime;
  and
- codegen spends 34.22% below `write_token_list`, while compilation teardown
  accounts for 35.22%.

These percentages come from the hardware-contaminated flat-AST run and are
hotspot attribution, not a comparison against the base commit. They identify a
separate candidate: store generic/custom-property token trees in an owned token
tape with compact child ranges, or otherwise remove per-function `Box` ownership
and recursive drop glue. Parser collection, nano traversal, codegen, and
destruction must consume the same representation directly; adding a flat token
copy beside the recursive representation would make all four phases worse.

Benchmark total compilation lifetime, including destruction. A benchmark that
uses `ManuallyDrop` may help attribute teardown cost, but must not be used to
claim an end-to-end improvement.

### Cache identifier serialization metadata with interned atoms

In the Tailwind codegen callgraph,
`cssparser::serializer::serialize_name` accounts for 14.70% total time and
8.03% self time. Compiler-scoped atoms already canonicalize repeated selector
identifiers. Extend the atom entry, or a codegen side cache keyed by atom ID,
with identifier-serialization metadata:

- an ASCII-safe fast-path bit for names that can be emitted unchanged; and
- lazily cached escaped output only for names that require escaping.

Do not eagerly serialize every interned name during parse. Many atoms may never
reach output after minification, and most CSS identifiers should take the
single-bit fast path. Benchmark this independently on selector-heavy input and
verify escaped identifiers, non-ASCII names, custom property names, and source
maps.

### Deprioritize effective-key container tuning

Changing effective-key histories among nested vectors, arena buckets, and a
flat interner repeatedly remained within the unchanged range. Further container
substitution is lower priority than reducing the number of effective-key
operations or attaching a canonical ID while context is already available.
Revisit bucket layout only with a dedicated workload that demonstrates a
collision, allocation, or cache-locality bottleneck.

## Benchmarking method

Evaluate performance changes as isolated commits on CodSpeed. Compare matching
runners where possible and inspect callgraph costs when runner hardware
differs. Local one-shot process timing includes process startup and setup costs
and is not precise enough to validate sub-millisecond changes in the minify
pass.
