# Effective keys in the flat IR

## Identity layers

Occurrence identity and semantic value identity must not be conflated:

- `SelectorId` identifies one authored or synthesized selector occurrence.
- `SelectorValueId` identifies one canonical selector value.
- `ContextPathId` identifies one canonical parent-linked wrapper path.
- `EffectiveKeyId` identifies the complete exact merge-history key.

Two separately authored `a` rules therefore have different occurrence IDs but
the same selector value ID. Interners use `FxHashMap` buckets and exact typed
equality; a fingerprint is never semantic proof.

```rust,ignore
struct ContextPathNode {
    parent: ContextPathId,
    frame: ContextFrameId,
}

struct EffectiveKeyData {
    selector_path: SelectorPathId,
    condition_path: ContextPathId,
    layer: LayerContextId,
    origin_and_phase: u32,
    history_segment: HistorySegmentId,
}
```

Interning this structure yields `EffectiveKeyId(u32)`. S1/S2 equality then
compares a compact ID, while structural equality is paid once at the interning
boundary.

## Parser and normalization boundary

The parser can compute context-path IDs as it enters and leaves rules. A final
effective key depends on the selector value after rule-local selector
normalization, so it is finalized at one of two safe points:

1. parse stores a context seed, then selector-local minify assigns the final
   canonical selector and `EffectiveKeyId`; or
2. selector nodes are immutable, every replacement appends a new canonical
   value, and the owner immediately recomputes its key.

The implementation must not leave a parse-time key attached to a selector that
was mutated in place. Immutable value nodes plus replacement IDs are the target
model because they also make cache invalidation explicit.

## Context contents

An effective key contains exact layer, origin, cascade phase, conditional path,
selector path, and history-segment identity. The first implementation compares
conditions only when their canonical typed representations are exactly equal.

This branch does not interpret at-rule semantics. A supported typed conditional
frame may use its canonical value ID; an unsupported or opaque wrapper uses its
occurrence `RuleId` as an opaque frame. Thus two declarations never merge merely
because two unmodeled at-rules serialize similarly.

Importance/cascade phase is carried with declaration history entries when a
block contains both normal and important declarations. It must not be erased by
a block-level key. Origin is supplied by compilation/parser configuration.

## Ownership

`EffectiveKeyId` is attached to a declaration occurrence owned by one syntax
position. It may be stored directly in `DeclarationBlockHeader` only after the
flat IR enforces unique block ownership. Until then, the explicit
`DeclarationOccurrence { block, effective_key }` form is required.

This prevents copying a block ID between style rules from silently reusing the
wrong selector or at-rule context.

## Cost model

Key construction is incremental:

```text
enter selector/at-rule
  intern one parent-linked frame
       ↓
encounter declaration run
  combine selector path + condition path + cascade fields
       ↓
  intern EffectiveKeyData
       ↓
store EffectiveKeyId on the declaration occurrence
```

There is no full-stylesheet `walk_declaration_blocks` prepass in the target
pipeline. The parser and selector-local normalization already possess all
required context. S2 history links may be created lazily when an effective key
is observed for the second time.
