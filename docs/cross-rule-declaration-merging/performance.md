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

### Discover structural adjacency during the existing walk

S1 candidates must be direct siblings in the same rule list. Rewalking the
complete stylesheet after collecting a flat `Vec<DeclarationBlockEntry>`
enforces that rule at commit time, but costs another 1-2 ms on the current
Bootstrap and Tailwind inputs. It also performs two declaration-block pointer
map lookups for every adjacent style-rule pair.

The declaration-block walk should instead produce both outputs in one
traversal:

```rust,ignore
struct WalkDeclarationBlockResult<'walk, 'ast, 'ghost> {
    declaration_blocks: Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
    same_selector_candidates: SameSelectorCandidateList,
}
```

When the walker enters a rule list, it records the indices of direct sibling
declaration blocks before recursively visiting child rule lists. This keeps
`SameSelectorCandidateList` responsible for candidate storage while avoiding a
second AST traversal and pointer-to-index lookups.

`DeclarationBlockEntry` now records `RuleListId`, `RuleListSegmentId`, and
`SiblingOrdinal`, and candidate discovery rejects entries whose structural
locations do not form a direct edge. A later optimization can create those
edges while walking instead of collecting a flat vector and testing consecutive
vector entries afterward.

### Do not repeat validated comparisons during commit

Discovery and scheduling produce an immutable candidate plan. If no earlier
commit can invalidate an S1 pair, commit should consume that validated plan
without repeating selector equality. Debug assertions may retain cheap
invariants. This optimization only affects accepted pairs, so its benefit is
small on stylesheets where equal adjacent selectors are rare, but it adds no
work to rejected candidates.

## Allocation and indexing candidates

- Reserve the declaration-block output vector when a cheap rule-list size
  estimate is available. Do not add a counting prepass solely to obtain an
  exact capacity.
- Generate candidate indices while walking so the S1 path does not require an
  `FxHashMap<*const DeclarationBlock, u32>`. A pointer map may still be
  appropriate for a later commit pass if AST mutation makes direct locations
  unstable.
- Allocate per-rule caches lazily and only when their reuse is demonstrated.
  Avoid zero-initializing an `Option` entry for every style rule for a cache
  used by only one stage.
- Keep `Candidate(u32, u32)` and compact queue state. Do not store selector
  clones or declaration snapshots in candidates.

### Share effective-key paths instead of cloning them per block

The current walker clones both the selector-frame path and conditional-context
path into every `DeclarationBlockEntry`. Even a top-level style rule allocates
one selector-path `Vec`; deeply nested styles copy every ancestor frame again.
This makes discovery allocate and copy `O(blocks * nesting depth)` references
before candidate processing starts.

Candidate representations should preserve exact typed equality without owning
these paths repeatedly. Viable implementations include:

- persistent parent-linked selector and conditional-context nodes addressed by
  compact `u32` IDs;
- traversal-local interning with `FxHashMap`, where equal typed paths reuse one
  immutable key; or
- direct structural-edge discovery, with entries retaining only shared
  effective-context identities required by S2 histories.

Do not replace typed frames with serialized strings or make a fingerprint the
sole equality proof. Any intern table or fingerprint must preserve frame order,
multiplicity, authored layer identity, selector tombstone semantics, and exact
conditional AST equality. Benchmark the shared representation independently:
for shallow stylesheets, interning and hashing may cost more than the cloned
references they remove.

## Declaration sequence reuse

`DeclarationBlockMinifier` already has a ghost-backed sequence abstraction for
deduplicating across multiple declaration blocks. S2 can reuse it instead of
adding another declaration-deduplication implementation. Until S2 calls that
path, its ghost constructor and `minify_sequence` entry point are explicitly
marked as reserved dead code.

The minifier should retain only the declaration IR allocated from the scratch
allocator. It does not need to cache a separate allocator field when no caller
reads it.

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

## Benchmarking method

Evaluate performance changes as isolated commits on CodSpeed. Compare matching
runners where possible and inspect callgraph costs when runner hardware
differs. Local one-shot process timing includes process startup and setup costs
and is not precise enough to validate sub-millisecond changes in the minify
pass.
