# Radix source-order AST implementation plan

## Goal

Move RocketCSS rules, declaration blocks, and declaration properties into
compiler-owned `RadixIndexArena` stores: exactly three node arenas. Authored
nodes remain contiguous primary vectors; rare structural insertions use local
sibling Radix trees. A list is a range reference (`start` + `len`) into one
arena, never a separate pointer collection. AST IDs provide both stable
identity and semantic order, and each declaration block stores its
`EffectiveKeyId` directly.

The normative design is in [Radix source-order AST IR](../flat-ast-ir/README.md):

- [Storage layout](../flat-ast-ir/storage-layout.md)
- [Radix node ID encoding](../flat-ast-ir/declaration-id-encoding.md)
- [Radix index arena](../flat-ast-ir/radix-index-arena.md)
- [Minification and reification](../flat-ast-ir/minify-and-reification.md)
- [Effective keys](../flat-ast-ir/effective-key.md)

## Completion criteria

- Rules, declaration blocks, and declaration properties are addressed by
  typed wrappers over `RadixIndexId`, one arena per kind.
- Every list is a two-word range reference; no first/last/previous/next
  topology exists.
- A rule's direct children form one contiguous primary range (level-order
  allocation); a block's declarations form one contiguous range.
- Each `DeclarationBlock` owns its current `EffectiveKeyId`.
- Nano consumes AST IDs and keys directly; it has no production
  `DeclarationBlockEntry` collection or separate semantic-order identity.
- S3 inserts synthesized nodes into their final local Radix position and
  extends the affected range without restarting the scheduler.
- Codegen reads the same stores; a mandatory whole-AST S5 rebuild is absent.
- Compact-ID exhaustion has a non-panicking AST-level fallback.
- Existing parser, Nano, visitor, codegen, source-map, and fixture behavior
  remains lossless.

## Non-negotiable order contract

Rules allocate in per-list order, level by level, so each rule's direct
children form one contiguous primary range. Declaration blocks and their
properties are single-range sequences. A list is a two-word range reference
(`start` + `len`) into one arena, never a collection of pointers:

```text
sibling_key(A) < sibling_key(B) below one primary
  => A is emitted before B
```

A range is a window in the arena's semantic ID order; iteration counts live
nodes. Direct CSS adjacency equals arena adjacency inside one range. The global
arena order is deterministic per-list (breadth-first) order, not lexical
preorder; codegen and visitors follow `children` ranges to recover source
order.

## Phase 0: freeze behavior and benchmarks

### Work

1. Record parse/minify/codegen CodSpeed results, peak memory, type sizes, and
   allocation counts.
2. Keep the current `RadixIndexArena` microbenchmarks for:
   - primary build and traversal;
   - sparse build and semantic traversal;
   - primary, mixed, and sibling-only lookup; and
   - repeated local insertion/removal.
3. Add test-only AST inspection:

   ```rust,ignore
   compilation.validate_radix_ast();
   compilation.rule_store_ids();
   compilation.declaration_block_store_ids();
   ```

4. Validate SoA sibling index/tree lengths, ID masks, topology, owner links,
   source order, and EffectiveKey presence.

### Exit gate

- A deliberate primary-order or topology error fails tests.
- Baseline benchmark output is recorded independently from AST migration.

## Phase 1: finish the common Radix primitive

### Work

1. Add typed-ID adapters so AST stores do not expose raw `RadixIndexId`.
2. Add ID-aware primary and semantic iterators yielding `(Id, &T)`.
3. Add an insertion-key allocator that leaves gaps and supports insertion
   between local sibling IDs.
4. Define local relabel as an explicit transaction returning exact ID remaps.
5. Add AST-wrapper hooks for repairing persistent references after a rare
   relabel.
6. Define the non-panicking fallback for:
   - more than `2^19` authored nodes;
   - more than 1023 siblings below one primary; and
   - local sibling exhaustion at an overflow primary.
7. Store zero-low sibling keys directly in the first Radix level. A sibling
   key with `low == 0` (the gap allocator's initial key `512 = high 16,
   low 0`) currently allocates a full 32-slot leaf; keep a per-branch direct
   value in the root and allocate the second-level leaf only when a nonzero-low
   key appears in that branch. The second-level leaf's slot zero stays unused.
   This was a ~15 ms `RadixTree::insert` `memset` hotspot in the Tailwind
   minify flame graph. Use `rocketcss_common::boxed::Box` for arena pages and
   keep page sizes independent of `size_of::<T>()`.
8. Preserve the current SoA layout:

   ```rust,ignore
   sibling_primary_indices: Vec<u32>
   sibling_trees: Vec<RadixTree<T>>
   ```

### Tests

- Property tests comparing random append/insert/remove/iterate sequences with
  a reference `Vec`.
- Boundary tests for every ID bit field.
- Multiple midpoint insertions and local relabel repair.
- Overflow/fallback behavior without panic.
- Semantic iteration after empty sibling groups and tombstones.
- Key `512` stored as `(high 16, low 0)` without allocating a leaf; key `513`
  allocating the leaf at branch 16; both co-existing and iterating in order.
- Relabeling across a `low == 0` boundary.
- Test-only allocation counters proving inserting key `512` allocates one
  root, one value, and zero leaves.

### Exit gate

- Primary traversal remains within noise of `Vec`.
- Sibling-only lookup retains the measured SoA improvement.
- The Tailwind `RadixTree<RuleRecord>::insert` `memset` hotspot is removed.
- Miri/sanitizer testing covers arena pointer safety where supported.

## Phase 2: declaration-block store migration

### Work

1. Replace the current dense declaration-block store with:

   ```rust,ignore
   type DeclarationBlockStore<'ast> =
       RadixIndexArena<'ast, DeclarationBlock<'ast>>;
   ```

2. Define `DeclarationBlockId` and `DeclarationPropertyId` as typed base IDs
   over the shared Radix encoding.
3. Add permanent `owner`, `effective_key`, `declarations` range, and revision
   fields to `DeclarationBlock`.
4. Replace direct indexing assumptions with store accessors.
5. Preserve `previous_merged` only as a migration adapter; do not reproduce it
   in the final store model.

### Tests

- Authored block IDs follow the declaration arena's per-block order.
- A block's declaration range covers exactly its own declarations.
- Existing block-local minification and codegen output remains byte-identical.

### Exit gate

- Every declaration-owning syntax node refers to a valid block ID.
- No consumer treats a dense index or borrowed address as a second block ID.

## Phase 3: source-order declarations

### Work

1. Parse declarations as streaming source-order runs rather than accumulating
   a block after recursive child parsing.
2. Store each block's declarations as one range over the declaration-property
   arena:

   ```rust,ignore
   struct DeclarationRange {
       start: DeclarationPropertyId,
       len: u32,
   }
   ```

3. Close a parent run before parsing a nested rule and begin a later
   `NestedDeclarationsRule` run after it.
4. Port block-local minify to range accessors.
5. Keep importance, source locations, tombstones, and declaration IR aligned
   with the range representation.

### Required source-order fixtures

```css
a {
  --p0: 0;
  --p1: 1;
}
b {
  --p2: 2;
}
```

```css
a {
  --p0: 0;
  & b {
    --p1: 1;
  }
  --p2: 2;
}
```

```css
a {
  & b {
    --p0: 0;
  }
  --p1: 1;
  @media (width > 1px) {
    --p2: 2;
  }
  --p3: 3;
}
```

Tests inspect declaration allocation and exact block ranges, not only output.
Cover comments, recovery, custom properties, normal/important values, and
non-style declaration owners.

### Exit gate

- Every authored declaration occurs in exactly one authored range.
- Parent ranges never contain nested declarations.
- A block range stays contiguous after any local declaration insertion.

## Phase 4: flat rule topology

### Work

1. Move rule values into `RuleStore = RadixIndexArena<CssRule>`.
2. Parse authored rules in per-list order, level by level, so each rule's
   direct children are one contiguous primary range.
3. Store `parent`, an inline `children` range (`start` + `len`), flags, and
   revision; no first/last/previous/next links.
4. Migrate rule kinds in bounded groups:
   - style and nested declarations;
   - media/supports/container/layer and grouping rules;
   - declaration-owning non-style rules;
   - leaf/custom rules; and
   - keyframes and page-related payloads.
5. Port visitor and codegen traversal to `children` ranges.

### Nano topology coupling

Two Nano helpers still walk linked-list rule topology and move to this phase,
not to declaration handling:

- `retire_rule` returns `RetiredRule { previous, next }`; and
- `first_block_after_rule_in_source` walks the `next_in_source` chain.

Under the range model both become short tombstone-skipping scans of the owning
range.

### Tests

- Empty, single-child, multiple-sibling, and deeply nested lists.
- A nested rule's descendants never appear in its parent's range.
- Opaque wrapper occurrences retain distinct IDs.
- Recovery creates no dangling range.
- Inserting a sibling extends only its owning range and sparse Radix storage.

### Exit gate

- Parent/sibling/subtree queries need no recursive discovery.
- Primary-only codegen retains contiguous traversal.

## Phase 5: AST-owned EffectiveKeys

### Work

1. Keep `ContextPathInterner` and `EffectiveKeyInterner` on `Compilation`.
2. Build parent-linked selector/wrapper/layer/origin context while parsing.
3. Write a context seed or final key directly on each new block.
4. Canonicalize selectors during rule-local minify and replace the owned key
   immediately if selector identity changes.
5. Make S3 selector-union creation return the final canonical selector and
   EffectiveKey before AST insertion.
6. Treat unsupported wrappers as opaque occurrence frames.

### Tests

- Exact selector/context equality shares a key.
- Layer, origin, phase, wrapper path/order, and opaque occurrence mismatches do
  not share keys.
- Selector replacement updates all owned blocks.
- Declaration-only changes leave keys unchanged.

### Exit gate

- Cross-rule equality compares compact AST-owned IDs.
- Production code contains no full EffectiveKey reconstruction pass.

## Phase 6: simplify Nano around AST IDs

### Work

1. Initialize scheduler sidecars by store iteration or fuse them into existing
   block-local minify traversal.
2. Delete production `DeclarationBlockDiscovery`,
   `DeclarationBlockEntry`, `DeclarationBlockEntryId`, and owner reconstruction.
3. Replace `SemanticOrderKey`/`SemanticSourceOrderKey` with fixed-width
   `DeclarationBlockId` or `RuleId` ordering.
4. Key S1 and S3 candidates by AST endpoint IDs and revisions.
5. Build S2 histories lazily from `block.effective_key`, ordered by block ID.
6. Keep declaration IR, property Bloom filters, rejection counters, and exact
   movement proofs as sidecars.
7. Remove global scheduler restart after S3.

### Declaration IR addressing

`DeclarationOccurrenceIr` lives in `occurrences: Vec<Option<...>>` addressed by
`DeclarationId.index()` (declaration_ir.rs), sized by
`declarations_in_source_order().len()`. That dense integer addressing is only
valid while declarations are one dense primary vector. A `DeclarationId` is a
Radix primary/sibling-split encoding, not a dense index, so the cache must be
re-keyed by `DeclarationPropertyId`:

- a sparse map (`FxHashMap<DeclarationPropertyId, DeclarationOccurrenceIr>`), or
- a primary-only dense region plus a sibling/overflow map, or
- derivation on demand during S2/S3.

Property identity itself is already decoupled: `CompactPropertyKey` and the
property Bloom filters are built from property names, not from ID bits, so
removing the property sub-ID and Local4 encoding requires no property-key
change. Block-ID-sorted histories (`histories.binary_search(&block)`) are
already range-compatible and work unchanged.

### Minify hot-path consolidation

The common minify path is dominated by selector/context canonicalization,
declaration scanning, IR construction, and rejected S3 candidates, not by
reallocations. Fold these performance phases in with the AST-ID work; each
lands as its own commit with a same-runner benchmark gate:

1. Migrate transient scheduler state (tapes, histories, indexes, ordered
   heaps, scratch) to the one minify-scope arena; FIFO candidate lists retain
   `std::collections::VecDeque`. Do not use `SmallVec` or system-heap
   collections for minify-only state.
2. Add no-change fast paths: detect unchanged selector/context identities and
   skip the global interner/key/block-revision rebuild entirely.
3. Publish declaration IR once during block-local minify instead of a second
   `CrossRuleState::from_compilation` declaration scan.
4. Remove the final block-scan by repairing EffectiveKey references from exact
   selector/wrapper remaps and appending finalized blocks to histories in
   source order.
5. Index S3 declarations by property above a measured block-size threshold;
   reject disjoint-PropertyBloom candidates before matching; keep output order
   from the source-ordered occurrence metadata.

### Tests and gate

- Tailwind performs at most one initialization scan, not one scan per S3
  commit.
- Nano does not allocate a second flat block/effective-key table.
- No source-order heap compares variable-length keys.
- A no-op minify changes no selector/context ID or revision.
- Published IR matches a test-only summary rebuilt from the final AST.
- Bootstrap and Tailwind output remains byte-identical.

### Exit gate

- No temporary declaration-ID `Vec` is allocated for ordinary blocks.
- All minify-only allocations have one explicit arena owner.
- S3 matching no longer searches every left declaration against every right
  declaration.

## Phase 7: direct S1/S2/S3 mutation

### Work

1. Make S1 commit declaration representation and retire/unlink owners locally.
2. Make S2 update declaration liveness/representation and dirty only affected
   histories/edges.
3. Make S3:
   - validate endpoint ranges and movement;
   - intern selector union and EffectiveKey;
   - allocate local sibling keys;
   - insert rule/block values into Radix stores;
   - extend the affected children/declaration ranges; and
   - enqueue affected S1/S2/S3 work.
4. Apply exact ID remaps if a rare local relabel occurs.
5. Reject stale candidates by endpoint revisions.

### Tests

- S3 exposes an earlier S1 edge and it is scheduled deterministically.
- S3 creates a non-adjacent same-key S2 history occurrence.
- Overlapping partial merges stabilize without global restart.
- Multiple insertions in one authored interval preserve old deterministic
  output.
- Nested lists and wrapper barriers remain isolated.

### Exit gate

- Scheduler work grows with changed edges/histories, not total blocks times
  committed candidates.
- Synthesized AST nodes already occupy final semantic positions.

## Phase 8: reduce S4/S5 to representation and cleanup

### Work

1. Keep S4 only for lossless declaration-representation choices that cannot be
   committed earlier.
2. Attach deferred representation plans to affected block state.
3. Make S5 finalize those plans, verify stable queues, unlink remaining retired
   nodes, and drop merge-only sidecars.
4. Remove fresh-store rebuilding used only to recover order.
5. Delete `previous_merged` and its codegen traversal.
6. Benchmark optional tombstone/overflow compaction separately.

### Exit gate

- Codegen consumes the live Radix AST directly.
- S5 performs no semantic choice and creates no new S1-S4 work.
- Compaction is optional and justified by measurement.

## Validation strategy

At every phase:

```text
cargo fmt --all
cargo test -p affected-crate
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

At AST/Nano boundaries run all Rust tests, enabled cssnano fixtures, Tailwind
and Bootstrap byte comparisons, and full CodSpeed. Track parse, minify, codegen,
peak memory, primary length, sibling-group count, local relabels, and overflow
upgrades independently.

## Commit strategy

- Keep common-store primitives, AST migration, parser order, EffectiveKey
  ownership, Nano simplification, and direct mutation in separate commits.
- Do not combine semantic merge changes with physical-layout experiments.
- Retain a layout optimization only when a focused benchmark exposes the hot
  operation, as with the sibling-index/tree SoA split.
- Preserve test-only structural walkers as independent oracles if useful, but
  never call them from the production minify path.
