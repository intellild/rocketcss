# Radix source-order AST implementation plan

## Goal

Move RocketCSS rules and declaration blocks into compiler-owned
`RadixIndexArena` stores. Authored nodes remain contiguous primary vectors;
rare structural insertions use local sibling Radix trees. AST IDs provide both
stable identity and semantic order, and each declaration block stores its
`EffectiveKeyId` directly.

The normative design is in [Radix source-order AST IR](../flat-ast-ir/README.md).

## Completion criteria

- Rules and declaration blocks are addressed by typed wrappers over
  `RadixIndexId`.
- Authored primary IDs increase in lexical source order.
- Direct parent/list/sibling topology is stored in the AST; no recursive walk
  is required to answer structural queries.
- Each `DeclarationBlock` owns its current `EffectiveKeyId`.
- Nano consumes AST IDs and keys directly; it has no production
  `DeclarationBlockEntry` collection or separate semantic-order identity.
- S3 inserts synthesized nodes into their final local Radix position and
  enqueues affected work without restarting the scheduler.
- Codegen reads the same stores; a mandatory whole-AST S5 rebuild is absent.
- Compact-ID exhaustion has a non-panicking AST-level fallback.
- Existing parser, Nano, visitor, codegen, source-map, and fixture behavior
  remains lossless.

## Non-negotiable order contract

For authored base nodes in one store:

```text
source position A before source position B
  => primary_index(A) < primary_index(B)
```

For synthesized nodes below the same primary:

```text
sibling_key(A) < sibling_key(B)
  => A is emitted before B
```

Property bits do not participate in base-node ordering. Direct CSS adjacency is
still checked through rule topology rather than numeric proximity.

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
   - more than `2^20` authored nodes;
   - more than 1023 siblings below one primary; and
   - declaration lists larger than `Local4`.
7. Preserve the current SoA layout:

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

### Exit gate

- Primary traversal remains within noise of `Vec`.
- Sibling-only lookup retains the measured SoA improvement.
- Miri/sanitizer testing covers arena pointer safety where supported.

## Phase 2: declaration-block store migration

### Work

1. Replace the current dense declaration-block store with:

   ```rust,ignore
   type DeclarationBlockStore<'ast> =
       RadixIndexArena<'ast, DeclarationBlock<'ast>>;
   ```

2. Define `DeclarationBlockId` as a typed base ID with property bits zero.
3. Add permanent `owner`, `effective_key`, flags, and revision fields to
   `DeclarationBlock`.
4. Replace direct indexing assumptions with store accessors.
5. Preserve `previous_merged` only as a migration adapter; do not reproduce it
   in the final store model.

### Tests

- Authored block IDs follow lexical order across style rules, descriptors,
  keyframes, and nested content.
- Property sub-IDs resolve their owning block but cannot retire it.
- Existing block-local minification and codegen output remains byte-identical.

### Exit gate

- Every declaration-owning syntax node refers to a valid block ID.
- No consumer treats a dense index or borrowed address as a second block ID.

## Phase 3: source-order declarations

### Work

1. Parse declarations as streaming source-order runs rather than accumulating
   a block after recursive child parsing.
2. Introduce explicit declaration representations:

   ```rust,ignore
   enum DeclarationList<'ast> {
       Range(DeclarationRange),
       Local4(LocalPropertySet<'ast>),
       Overflow(ArenaVec<'ast, ArenaBox<'ast, Declaration<'ast>>>),
   }
   ```

3. Close a parent run before parsing a nested rule and begin a later
   `NestedDeclarationsRule` run after it.
4. Port block-local minify to range/local/overflow accessors.
5. Keep importance, source locations, tombstones, and declaration IR aligned
   with the selected representation.

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
- `Local4` upgrades atomically on a fifth property.

## Phase 4: flat rule topology

### Work

1. Move rule values into `RuleStore = RadixIndexArena<CssRule>`.
2. Parse authored rules into the primary store in lexical preorder.
3. Store `parent`, `parent_list`, `previous_sibling`, `next_sibling`, child-list
   identity, flags, and revision.
4. Migrate rule kinds in bounded groups:
   - style and nested declarations;
   - media/supports/container/layer and grouping rules;
   - declaration-owning non-style rules;
   - leaf/custom rules; and
   - keyframes and page-related payloads.
5. Port visitor and codegen traversal to IDs and topology.

### Tests

- Empty, single-child, multiple-sibling, and deeply nested lists.
- `next_sibling` skips a complete preceding subtree.
- Opaque wrapper occurrences retain distinct IDs.
- Recovery creates no dangling topology.
- Inserting a sibling changes only local topology and sparse Radix storage.

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

### Exit gate

- Tailwind performs at most one initialization scan, not one scan per S3
  commit.
- Nano does not allocate a second flat block/effective-key table.
- No source-order heap compares variable-length keys.

## Phase 7: direct S1/S2/S3 mutation

### Work

1. Make S1 commit declaration representation and retire/unlink owners locally.
2. Make S2 update declaration liveness/representation and dirty only affected
   histories/edges.
3. Make S3:
   - validate endpoint topology and movement;
   - intern selector union and EffectiveKey;
   - allocate local sibling keys;
   - insert rule/block values into Radix stores;
   - update topology/history; and
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
