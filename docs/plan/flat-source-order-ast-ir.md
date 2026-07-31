# Flat source-order AST IR implementation plan

## Goal

Replace the recursive arena-owned CSS AST with compilation-owned dense stores
and source-order tapes. All stable relationships use typed `u32` IDs or compact
ranges. Parsing, minification, visitors, and code generation operate on those
stores; S5 rebuilds them in final semantic order.

The normative design is in [Flat source-order AST IR](../flat-ast-ir/README.md).
This file is an execution plan and may change as benchmarks expose better
physical layouts.

## Completion criteria

- `CssRule`, every rule payload, selectors, declaration blocks, declarations,
  and remaining aggregate child lists are owned by `Compilation` stores.
- Authored declaration/property IDs increase in lexical source order across the
  complete stylesheet, including before, inside, and after nested rules.
- Direct parent, sibling, and subtree operations require no recursive discovery
  walk.
- `EffectiveKeyId` is computed from canonical selector/context values and is
  attached to a uniquely owned declaration occurrence.
- Cross-rule minify uses IDs, ranges, live links, and rewrite plans; it does not
  use `walk_declaration_blocks` or `previous_merged`.
- S5 produces compact rule/declaration tapes in final semantic order.
- `rocketcss_common::boxed::Box`, `rocketcss_common::Allocator`, allocator-backed
  AST vectors, `Pin`, `PhantomPinned`, and allocator-taking compiler/parser APIs
  are absent from the final AST pipeline.
- `StringPool` owns its backing storage and does not borrow an allocator.
- Existing parser, nano, visitor, codegen, upstream, source-map, and fixture
  behavior remains lossless.

## Non-negotiable property allocation contract

The global declaration store is its own order domain. If declaration
occurrence `A` begins before declaration occurrence `B` in the source, then:

```text
DeclarationId(A) < DeclarationId(B)
```

This applies across top-level rules, conditional rules, nested style rules,
`NestedDeclarationsRule` runs, and recovered syntax that remains in the AST.
Whitespace, comments, rule payload allocation, selector allocation, and later
store growth do not affect declaration IDs.

The parser appends a declaration immediately after parsing that declaration.
It must not accumulate an arena `DeclarationBlock` and allocate/copy the block
after recursively parsing child rules. A block captures `(offset, len)` for
each contiguous declaration run at the moment the run is parsed.

## Phase 0: freeze behavior and expose test inspection

### Work

1. Record baseline type sizes, parse/minify/codegen CodSpeed results, output
   sizes, peak memory, and representative allocation counts.
2. Add test-only read APIs on `Compilation`:

   ```rust,ignore
   fn declaration_slots(&self) -> impl Iterator<Item = (DeclarationId, &Declaration)>;
   fn declaration_block_header(&self, id: DeclarationBlockId) -> &DeclarationBlockHeader;
   fn rule_topology(&self, id: RuleId) -> RuleTopology;
   ```

3. Add invariant validation callable by parser/nano tests:

   ```rust,ignore
   compilation.validate_flat_ir()
   ```

   It checks dense ID bounds, nonoverlapping live ranges, unique block
   ownership, complete topology, source-order declaration allocation, and
   tombstone/range consistency.
4. Keep inspection APIs crate-private or behind `cfg(test)` so physical storage
   is not accidentally frozen as public API.

### Property source-order test gate

Add a dedicated parser test module before changing parser allocation. Use
unique custom-property names so the expected lexical order is unambiguous.

#### Flat top-level order

```css
a { --p0: 0; --p1: 1 }
b { --p2: 2 }
```

Assert the declaration tape is exactly `--p0, --p1, --p2`; all IDs are strictly
increasing and each block range selects exactly its authored values.

#### Parent before nested child

```css
a { --p0: 0 }
a {
  --p1: 1;
  & b { --p2: 2 }
}
```

Assert the tape is `--p0, --p1, --p2`. This catches the current tempting
`--p0, --p2, --p1` post-recursion allocation order.

#### Empty leading run before child

```css
a { --p0: 0 }
a {
  & b { --p1: 1 }
  --p2: 2
}
```

Assert the tape is `--p0, --p1, --p2`; the second `a` leading block is empty at
the cursor before `--p1`; and `--p2` belongs to a later
`NestedDeclarationsRule` block. The empty range must not later absorb `--p1`.

#### Declarations on both sides of nesting

```css
a {
  --p0: 0;
  & b { --p1: 1; --p2: 2 }
  --p3: 3;
  @media (width > 1px) { --p4: 4 }
  --p5: 5
}
```

Assert the global tape is exactly `--p0` through `--p5`. Assert every
declaration run has a separate exact range and the enclosing rule's ranges do
not include nested declarations.

#### Conditional and non-style blocks

Cover `@media`, `@supports`, `@container`, `@font-face`, keyframe declaration
blocks, page declarations, and any other grammar that contributes a declaration
run. Assert one global lexical order even when those blocks use different typed
payload stores.

#### Recovery, comments, and importance

- Put valid declarations around a recovered invalid declaration and assert the
  retained slots remain in source order.
- Put comments and whitespace between every token and assert they do not create
  or reorder declaration slots.
- Mix normal and `!important` declarations and assert importance sidecars align
  with the same declaration IDs.
- Include custom-property raw tokens and typed properties to ensure parse path
  selection cannot change allocation order.

#### Global invariants for every fixture

- Every live authored declaration occurs in exactly one live block range.
- Live ranges do not overlap.
- A range contains no live declaration owned by another rule/run.
- Iterating ranges in semantic source order yields the same authored property
  order as iterating the global declaration tape and filtering tombstones.
- Parsing the same source twice assigns the same declaration order.
- Parse then codegen then parse preserves property order.

Do not accept serialization-only tests for this contract. Incorrect physical
allocation can serialize correctly before S1 range coalescing exposes it.

### Exit gate

- Tests fail against any deliberate post-recursion allocation implementation.
- Baselines and invariant helpers are committed independently.

## Phase 1: owned storage kernel

### Work

1. Extend the common dense abstraction with all required typed operations:
   checked `u32` capacity, `DenseRange { offset, len }`, slice access, append,
   and ID-preserving sidecars. Do not add parallel hand-written ID wrappers.
2. Make `Compilation` the only owner of AST stores. Introduce compact store
   types in the owning AST modules, following the existing module layout.
3. Change `StringPool` to owned storage. Interning remains collision-safe and
   compiler-local; `Atom` comparison remains constant-time. Do not solve this
   by retaining `Allocator` inside the pool.
4. Introduce a compact syntax-node header and compare two payload layouts:
   tagged `RulePayloadId` plus per-kind stores versus one inline rule enum.
   Select using memory and end-to-end benchmarks, not enum aesthetics.
5. Add generation/debug ownership assertions if stale cross-compilation IDs are
   otherwise hard to diagnose. Do not enlarge release IDs beyond `u32` without
   measured need.

### Exit gate

- Dense stores reject overflow deterministically.
- Store/range property tests cover empty, boundary, append, tombstone, and
  compaction cases.
- Owned `StringPool` tests cover deduplication, exact collision handling,
  compiler isolation, and stable serialization.

## Phase 2: source-order declaration tape

This phase lands before the complete rule-store migration because its order
contract is required by every later range optimization.

### Work

1. Replace `DeclarationBlock { Vec<Declaration>, BitVec, previous_merged }`
   with a block header, global `DenseStore<DeclarationId, Declaration>`, and
   aligned importance/liveness sidecars.
2. Rewrite `parse_style_contents` and every descriptor/declaration parser as a
   streaming run builder:

   ```text
   begin run -> remember next DeclarationId
   parse one property -> append slot immediately
   encounter nested rule/end -> finish exact range
   parse nested rule
   begin later NestedDeclarations run at current cursor
   ```

3. Ensure error recovery either appends one lossless recovered declaration at
   its lexical point or appends nothing. It must never defer a retained value.
4. Convert block-local minify to mutate/tombstone global slots through IDs.
   Preserve a narrow slice/range façade temporarily for algorithms that do not
   yet need structural rewrites.
5. Store source locations and importance by declaration ID. Add assertions that
   sidecar lengths equal the declaration store length.

### Exit gate

- Every Phase 0 property-order fixture passes.
- S1 regression: merging the two `a` rules in the parent/child fixtures never
  includes `--p1`/`--p2` from the nested child.
- Parser, declaration minifier, codegen, source-map, and upstream tests pass.
- No `previous_merged` behavior is added to the new header.

## Phase 3: flat rule topology and typed payload stores

### Work

1. Introduce `RuleId`, `SyntaxNode`, and per-rule-kind dense payload stores in
   `crates/ast/src/css_rule.rs` and the corresponding `rules/*` modules.
2. Parse rules directly into preorder slots. Maintain a small open-parent stack
   and finalize `next_sibling` and `subtree_end` when a rule/list closes.
3. Replace child `Vec<CssRule>` fields with topology. Preserve list identity and
   barriers required by S1/S3; direct children must be iterable without scanning
   descendants.
4. Migrate rule kinds in bounded groups:
   - style and nested-declarations;
   - media/supports/container/layer and other grouping rules;
   - declaration-owning non-style rules;
   - leaf and unknown/custom rules;
   - keyframes and nested keyframe payloads.
5. Keep every migration commit compiling all consumers. A temporary accessor
   façade may return IDs/views, but it must not recreate boxed tree ownership.

### Tests

- Topology table tests for empty lists, one child, multiple siblings, nested
  descendants, opaque rules, and maximum supported nesting depth.
- Preorder IDs follow rule source order.
- `next_sibling` skips the complete preceding subtree.
- `subtree_end` is the first rule after a subtree.
- Recovery never creates dangling parent/sibling IDs.
- Existing nesting and at-rule serialization fixtures remain byte-equivalent.

### Exit gate

- Recursive rule discovery is unnecessary for parent/sibling/subtree queries.
- No `Pin`/`PhantomPinned` remains on style rules.

## Phase 4: selector/value tapes and canonical effective keys

### Work

1. Replace selector-owned arena vectors with store headers and component ranges.
   Benchmark this default against per-component IDs before committing the latter.
2. Separate selector occurrence identity from canonical selector value identity.
   Intern exact normalized values with `FxHashMap` buckets plus exact equality.
3. Build parent-linked conditional/layer paths while parsing rules. Unsupported
   at-rules contribute their occurrence `RuleId` as opaque frames; do not add
   at-rule semantics in this branch.
4. Finalize `EffectiveKeyId` after rule-local selector normalization. Replacing
   an immutable selector value must immediately recompute the owner key.
5. Attach the key to a uniquely owned declaration occurrence/header. Split
   normal and important history phases as required; do not hide mixed phases in
   one block-level scalar.
6. Migrate remaining arena-backed aggregate value lists to owned dense ranges or
   ordinary owned values, module by module. The end state cannot retain an
   allocator merely for value children.

### Tests

- Equal separately authored selectors have different occurrence IDs and equal
  canonical value IDs.
- Different conditional stack order/multiplicity produces different key IDs.
- Equal typed media/supports/container paths produce equal IDs only after exact
  structural equality.
- Layer, origin, and phase mismatches never share a history.
- Opaque at-rule occurrences remain distinct even with identical text.
- Selector minification replacement invalidates/recomputes the effective key.
- Hash collision tests still perform exact comparison.

### Exit gate

- Cross-rule equality hot paths compare compact IDs.
- No complete-path vector is allocated per declaration occurrence.

## Phase 5: consumers and mutation API

### Work

1. Change visitor callbacks to receive typed IDs and compilation/store views.
   Remove transparent callbacks below the useful property/selector granularity
   as already planned; do not reproduce the old node explosion with ID APIs.
2. Separate payload mutation from structural mutation:
   - fixed-size replacement updates a store slot or replacement ID;
   - removal/insertion/reparenting emits a `RewriteOp`;
   - callers cannot hold a mutable payload reference while growing its store.
3. Port nano modules along AST module boundaries. Use shared range and matching
   abstractions instead of local store-index arithmetic.
4. Port codegen to scan syntax topology and declaration ranges directly.
5. Port clone/equality/debug/test helpers. Replace `CloneIn`, `FromIn`, and
   `IntoIn` with ordinary owned cloning or compilation-aware ID reification.

### Exit gate

- Visitor, nano, and codegen suites pass without tree reconstruction.
- No public API exposes store element addresses as stable identity.
- No consumer requires an arena lifetime merely to traverse the AST.

## Phase 6: cross-rule minify and terminal reification

### Work

1. Replace `walk_declaration_blocks` with direct source-order syntax/declaration
   scanning. Build lazy S2 history links when an `EffectiveKeyId` repeats.
2. Port live-sibling state to `RuleId` topology. S1/S3 candidates remain compact
   `(u32, u32)` edge identities and revalidate current liveness before commit.
3. Implement S1 range behavior:
   - exactly adjacent ranges: coalesce header;
   - gap containing tombstones only: coalesce after proof;
   - gap containing a live foreign slot: retain ordered ranges until S5.
4. Port S2 exact-declaration pruning to declaration IDs and tombstone sidecars.
5. Make S3/S4 append synthesized payloads plus semantic insertion positions;
   allocation order must never become output order.
6. Implement S5 as one fresh-store rebuild that copies retained origins,
   materializes replacements, writes topology, compacts declaration ranges, and
   drops tombstones/retired state.
7. Delete `previous_merged` and its codegen traversal only after S5 output is
   covered by equivalence tests.

### Tests

- All S1-S5 fixtures and enabled cssnano fixtures.
- Property-order fixtures before and after S1/S2/S5.
- S1 cannot widen a parent range across a nested child's live declaration.
- Synthesized S3 rule serializes at its planned position even when its payload
  ID was allocated last.
- S5 output has no tombstones, overlapping ranges, retired live links, or
  dangling IDs.
- Running minify twice is idempotent.
- Tailwind and Bootstrap input/output comparisons detect wrong and missed
  optimization separately.

### Exit gate

- `walk_declaration_blocks` and `previous_merged` are deleted.
- Codegen consumes only committed flat stores.
- CodSpeed compares the full pipeline and per-stage timings against every major
  migration baseline.

## Phase 7: remove legacy allocation infrastructure

### Work

1. Remove `allocator: &Allocator` from `Compiler`, parser entry points, parser
   helpers, AST constructors, nano entry points, codegen helpers, and tests.
2. Delete remaining uses of `rocketcss_common::boxed::Box`, allocator-backed
   `vec::Vec`, arena hash collections, `CloneIn`, `FromIn`, and `IntoIn` from the
   AST pipeline.
3. Remove `Allocator` and arena container modules from `rocketcss_common` after
   repository-wide `rg` proves there are no consumers. If an unrelated consumer
   remains, separate it first; do not keep the AST API for compatibility.
4. Replace `Allocator::with_ghost` with direct ghost-token scoping where the
   brand is still required, then remove ghost state that existed only for arena
   aliasing.
5. Update examples, README/API documentation, benchmarks, size tooling, and
   downstream integration fixtures.

### Exit gate

The following searches return no AST-pipeline matches:

```text
rg "rocketcss_common::boxed::Box|Allocator|CloneIn|FromIn|IntoIn" crates
rg "Pin<|PhantomPinned|previous_merged|walk_declaration_blocks" crates
```

Any intentional remaining match must be outside AST ownership and documented
with its own measured requirement.

## Commit and validation strategy

- Keep storage primitives, parser order changes, rule-kind migrations, consumer
  ports, cross-rule changes, and allocator removal in independent commits.
- Never combine a semantic minify change with a physical-layout benchmark
  change unless the semantic behavior is already locked by tests.
- Run `cargo fmt --all`, all Rust tests, and clippy at every phase boundary.
- Run targeted parser/nano/codegen tests after each bounded migration commit.
- Compare CodSpeed after Phases 2, 3, 4, 6, and 7. Retain changes only with
  explained wins or with a necessary architectural role whose measured cost is
  explicitly recorded.
- Track parse, minify, and codegen separately: faster codegen must not hide a
  parser regression, and a source-order bug must never be accepted for speed.

## Primary risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Parser recursion allocates parent declarations late | Phase 0 physical ID-order tests fail before serialization can mask it |
| Flat preorder is mistaken for direct-child range | Store `next_sibling` and `subtree_end`; test nested siblings explicitly |
| S1 range union swallows nested declarations | Require adjacency/tombstone proof and run parent/child range fixtures |
| Canonical ID hash collision changes semantics | Exact typed equality inside every interner bucket |
| Selector mutation leaves stale effective key | Immutable selector values and replacement-triggered key recomputation |
| Per-node IDs add excessive indirection | Benchmark inline payload, per-kind store, and range-tape alternatives |
| S3 append order changes output order | Explicit semantic insertion positions and S5-only physical reorder |
| Compatibility façade preserves old allocator indefinitely | Phase exit gates ban boxed/tree reconstruction and final `rg` checks |
| S5 copy cost erases locality wins | Stage-level CodSpeed and allocation/peak-memory measurement |
