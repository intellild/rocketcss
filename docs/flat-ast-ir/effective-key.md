# Effective keys as AST data

## Purpose

`EffectiveKeyId` identifies the exact selector and cascade context in which a
declaration block participates in cross-rule history. Equality is intentionally
strict: this branch merges only when all modeled conditions are identical and
does not interpret at-rule equivalence.

## Identity layers

Occurrence identity and canonical value identity remain separate:

- `RuleId` identifies one authored or synthesized rule occurrence.
- `SelectorValueId` identifies one canonical selector value.
- `ContextPathId` identifies one canonical parent-linked wrapper path.
- `EffectiveKeyId` identifies the complete exact history key.

Two separately authored `a` rules have different `RuleId` values but may share
the same selector value and EffectiveKey.

```rust,ignore
struct EffectiveKeyData {
    selector_path: SelectorPathId,
    condition_path: ContextPathId,
    layer: LayerContextId,
    origin: OriginId,
    cascade_phase: CascadePhase,
    history_segment: HistorySegmentId,
}
```

The interner uses `FxHashMap` buckets and exact typed equality. A fingerprint is
only a candidate lookup key.

## Construction belongs to parsing and local normalization

The parser already has the complete parent selector, wrapper, layer, origin,
and cascade context when it creates a declaration block. It builds
parent-linked context IDs incrementally and writes a key seed directly to the
AST block.

```text
enter selector or wrapper
  update parent-linked context IDs
       ↓
parse declaration run
  append DeclarationBlock to RadixIndexArena
  store context seed/final EffectiveKeyId on the block
       ↓
selector-local minify
  canonicalize selector value
  replace the block's EffectiveKeyId if selector identity changed
```

There is no later recursive walk that rediscovers block owners and rebuilds
keys into a transient flat vector.

The preferred representation makes canonical selector values immutable. A
selector transform creates or reuses another `SelectorValueId` and updates all
owned declaration-block keys in the same operation. If in-place selector
mutation remains temporarily, its API must invalidate/recompute keys before
returning.

## AST ownership

Every live declaration block owns its key:

```rust,ignore
struct DeclarationBlock<'ast> {
    owner: DeclarationBlockOwner,
    declarations: DeclarationList<'ast>,
    effective_key: EffectiveKeyId,
    revision: u32,
    flags: DeclarationBlockFlags,
}
```

Nano consumes `block.effective_key` directly. It does not own an
`EffectiveKeyStore` whose IDs disappear after discovery, and it does not create
`DeclarationOccurrence { block, effective_key }` wrappers.

The interner and canonical key records are compilation-owned because S3 must
intern a selector union. A synthesized block receives its final key before its
Radix insertion becomes visible to queues.

## Exact context contents

The key includes:

- canonical selector path;
- exact conditional/wrapper path;
- layer;
- origin;
- cascade phase/history segment; and
- any other modeled context required by CSS cascade equality.

Importance remains declaration-local when one block mixes normal and important
declarations. S2 histories use `(EffectiveKeyId, PropertyId, importance)` and
must not erase the phase through one block-level scalar.

Supported typed wrappers may intern canonical values. Unsupported or unmodeled
wrappers contribute their occurrence `RuleId` as an opaque frame. Identical
serialization does not make two opaque occurrences equal.

## Invalidation rules

| Mutation                                         | EffectiveKey action                      |
| ------------------------------------------------ | ---------------------------------------- |
| Declaration value/delete/tombstone               | unchanged                                |
| Selector canonical replacement                   | recompute immediately                    |
| S3 selector union                                | intern and assign before insertion       |
| Move to another wrapper/list/layer/origin        | recompute immediately                    |
| Local Radix ID relabel without semantic movement | unchanged                                |
| Rule retirement                                  | key remains available until pass cleanup |

Block revision increments are separate from key identity. A declaration-only
mutation may invalidate effect summaries without changing EffectiveKey.

## Nano histories

Nano builds histories lazily from AST values:

```text
iterate DeclarationBlockStore in Radix order
       ↓
read block.effective_key
       ↓
first occurrence: remember endpoint
second occurrence: create intrusive history
later occurrence: insert by DeclarationBlockId order
```

Because `DeclarationBlockId` already encodes semantic position, histories do
not allocate or compare `SemanticSourceOrderKey` values.

## Required tests

- Equal selectors and exact equal contexts intern the same key.
- Layer, origin, cascade phase, wrapper order, or wrapper multiplicity mismatch
  produces a different key.
- Opaque wrapper occurrences remain distinct even with identical text.
- Selector replacement updates every owned block key before Nano runs.
- S3 assigns the synthesized block key before enqueueing its histories/edges.
- Declaration-only changes do not rebuild the key.
- Hash collisions still require exact structural equality.
- Production Nano performs no full key-reconstruction walk.
