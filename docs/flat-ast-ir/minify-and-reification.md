# Minification and reification

## Semantic IR versus physical storage

The S1-S5 documents define semantic declaration sequences, histories, live
adjacency, and candidates. The flat IR supplies their physical substrate:

```text
syntax tape + topology
  gives owning lists, direct siblings, subtree boundaries, and source order

declaration tape + block headers
  gives declaration occurrences and cheap contiguous ranges

canonical value/context stores
  give SelectorValueId, ContextPathId, and EffectiveKeyId

sidecars
  give liveness, history links, revisions, and rewrite-plan state
```

The target scanner consumes these stores directly. It does not reconstruct
source order with recursive AST walking and does not use node addresses as
keys.

## Stage mapping

### S1: same-selector coalescing

S1 compares direct live siblings and one `EffectiveKeyId` plus emission
identity. It retires the left syntax endpoint and logically concatenates the
left declaration sequence before the right sequence.

If declaration ranges are physically adjacent, the right sequence may adopt
one combined range. If a gap contains only tombstones, it may also be consumed
after proving that invariant. Otherwise S1 records an ordered range sequence;
it never widens a range across live declarations belonging to a nested or
neighboring rule.

`previous_merged` is a transitional encoding and is absent from the target IR.

### S2: declaration-effect pruning

S2 histories are one-dimensional source-ordered chains keyed by
`EffectiveKeyId`. A declaration occurrence is linked when its key repeats.
Exact duplicate removal tombstones only the old declaration slot. More complete
effect analysis maintains masks and origins in sidecar state; it does not move
authored slots during stabilization.

### S3: selector partial factoring

S3 produces logical synthesized rules, selectors, and declaration sequences.
New payloads may be appended to stores, but append position is allocation order,
not output order. Every synthesized rule therefore carries a semantic insertion
position in the rewrite plan.

### S4: representation planning

S4 selects retained authored ranges, typed replacements, synthesized nodes, and
final sequence ownership. Its output is a flat-IR reification plan even if the
semantic type keeps the historical `AstReificationPlan` name during migration.

S4 may enqueue more S1-S4 work when a representation or structural revision
changes. It never partially rearranges the main tapes.

### S5: commit

S5 allocates fresh dense stores and scans the semantic output order once. It:

1. copies retained authored payloads and materializes planned replacements;
2. writes declarations in final semantic order and creates compact block
   ranges;
3. writes rules in preorder and fills `parent`, `next_sibling`, and
   `subtree_end`;
4. installs canonical selector/context IDs and recomputes invalidated effective
   keys;
5. drops tombstones, retired shells, and merge-only sidecars; and
6. swaps the compact stores into the compilation.

Code generation consumes only this committed flat IR. It never follows merge
chains or interprets candidate state.

## Scheduling epochs

Because S3/S4 may introduce new candidates, one invocation may have multiple
semantic scheduling epochs:

```text
flat authored IR
  -> local replacement/tombstone edits
  -> S1/S2 fixed point
  -> S3/S4 rewrite planning
  -> newly exposed work? return to S1/S2/S3/S4
  -> complete plan
  -> one terminal S5 compacting commit
```

Only S5 changes the physical order of surviving entities. Dirty queues and
live links describe logical order until that terminal commit.

## Required invariants

- Authored declaration IDs initially increase in lexical source order.
- Each live authored declaration belongs to exactly one live block/occurrence.
- Block ranges do not overlap and never include a live foreign declaration.
- Direct sibling checks use topology metadata and cannot skip a subtree or
  retained barrier.
- Effective-key equality is compact ID equality backed by exact canonical
  interning.
- Tombstones are never serialized and are the only slots a range may absorb as
  a physical gap.
- Synthesized allocation order never substitutes for semantic insertion order.
- Reification preserves source origins for diagnostics and source maps.
- After S5, no `Box`, AST arena pointer, `previous_merged`, or merge-only
  reference is required by code generation.

