# Cross-rule declaration merging: performance

This document records performance observations and optimization candidates. It
does not change the correctness model in the S1-S5 design documents.

## S1 selector matching

### Keep structural equality as the default

An experiment cached a full `FxHasher` fingerprint for every style rule that
participated in an S1 candidate. It regressed the CodSpeed minify benchmarks:

| Workload | Direct equality | Fingerprint | Candidate discovery |
| --- | ---: | ---: | ---: |
| Bootstrap | 1.4 ms | 2.3 ms | 1.2 ms |
| Tailwind | 3.1 ms | 2.8 ms | 1.5 ms |

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
complete stylesheet after collecting `Vec<&StyleRule>` enforces that rule, but
costs another 1-2 ms on the current Bootstrap and Tailwind inputs. It also
performs two pointer-map lookups for every adjacent style-rule pair.

The style-rule walk should instead produce both outputs in one traversal:

```rust,ignore
struct WalkStyleRuleResult<'walk, 'ast, 'ghost> {
    style_rules: Vec<&'walk StyleRule<'ast, 'ghost>>,
    same_selector_candidates: SameSelectorCandidateList,
}
```

When the walker enters a rule list, it records the indices of direct sibling
style rules before recursively visiting child rule lists. This keeps
`SameSelectorCandidateList` responsible for candidate storage while avoiding a
second AST traversal and pointer-to-index lookups.

### Do not repeat validated comparisons during commit

Discovery and scheduling produce an immutable candidate plan. If no earlier
commit can invalidate an S1 pair, commit should consume that validated plan
without repeating selector equality. Debug assertions may retain cheap
invariants. This optimization only affects accepted pairs, so its benefit is
small on stylesheets where equal adjacent selectors are rare, but it adds no
work to rejected candidates.

## Allocation and indexing candidates

- Reserve the style-rule output vector when a cheap rule-list size estimate is
  available. Do not add a counting prepass solely to obtain an exact capacity.
- Generate candidate indices while walking so the S1 path does not require an
  `FxHashMap<*const StyleRule, u32>`. A pointer map may still be appropriate for
  a later commit pass if AST mutation makes direct locations unstable.
- Allocate per-rule caches lazily and only when their reuse is demonstrated.
  Avoid zero-initializing an `Option` entry for every style rule for a cache
  used by only one stage.
- Keep `Candidate(u32, u32)` and compact queue state. Do not store selector
  clones or declaration snapshots in candidates.

## Declaration sequence reuse

`DeclarationBlockMinifier` already has a ghost-backed sequence abstraction for
deduplicating across multiple declaration blocks. S2 can reuse it instead of
adding another declaration-deduplication implementation. Until S2 calls that
path, its ghost constructor and `minify_sequence` entry point are explicitly
marked as reserved dead code.

The minifier should retain only the declaration IR allocated from the scratch
allocator. It does not need to cache a separate allocator field when no caller
reads it.

## Benchmarking method

Evaluate performance changes as isolated commits on CodSpeed. Compare matching
runners where possible and inspect callgraph costs when runner hardware
differs. Local one-shot process timing includes process startup and setup costs
and is not precise enough to validate sub-millisecond changes in the minify
pass.
