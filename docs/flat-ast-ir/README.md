# Flat source-order AST IR

## Status

This directory defines the target physical representation for RocketCSS's CSS
AST. Rules, declaration blocks, and declarations live in global mutable B+
dense stores instead of nested Rust vectors. Compact IDs and list headers
recover structure without requiring Rust ownership to reproduce the CSS tree.

A formatted stylesheet with one property per line is a useful mental model:
the parser writes syntax and declarations in lexical order, while B+ sequences,
typed IDs, list headers, and topology describe the final AST.

This is a storage design, not permission to change CSS semantics. Parsing,
minification, visitors, and code generation must preserve lossless syntax,
cascade order, nesting, fallback chains, and source locations.

## Documents

- [Storage layout](./storage-layout.md) specifies compilation ownership, global
  B+ stores, AST list headers, rule topology, arena payloads, and block-owned
  effective keys.
- [B+ tree dense stores](./b-plus-tree-sequence.md) specifies mutable sequence
  pages, list operations, scheduler aggregation, and terminal traversal.
- [Declaration ID encoding](./declaration-id-encoding.md) specifies the compact
  block-hint/local-label key, local relabel repair, and arena-vector overflow
  fallback.
- [Effective keys](./effective-key.md) specifies canonical selector and exact
  context identities stored directly on declaration blocks.
- [Minification over mutable B+ stores](./minify-and-reification.md) maps S1-S5
  onto AST-owned keys, store-local scheduling, range splice, and optional
  terminal cleanup.

The semantic cross-rule algorithm remains specified in
[Cross-rule declaration merging](../cross-rule-declaration-merging/overall.md).
Those documents define which transformations are valid; this directory defines
their target physical storage.

## Target pipeline

```text
Compiler::parse
  insert rules, blocks, and declarations in lexical order
  maintain ContextPathId while entering wrappers
  store compact start_id + len declaration lists
       ↓
rule-local minify
  canonicalize selector values
  finalize DeclarationBlock.effective_key
       ↓
S1/S2/S3/S4
  operate directly on B+ stores
  mutate local ranges and store-owned dirty/history state
  repair rare DeclarationId relabels through BlockHint
       ↓
terminal freeze
  drop merge-only state; optional measured compaction
       ↓
code generation
  traverse linked B+ leaves and overflow vectors
```

There is no required recursive `walk_declaration_blocks` prepass and no
mandatory full AST copy after minification.

## Decisions

1. Rules, declaration blocks, and declarations have global ordered B+ stores.
2. Former AST vectors use compact sequence headers rather than owning nested
   allocations.
3. A common declaration block is `start DeclarationId + length` over the global
   declaration sequence.
4. `DeclarationId` is a spaced order label with an encoded block lookup hint,
   not a physical offset or `PropertyId`.
5. Local ID relabel inspects only hinted declaration blocks and commits exact
   remaps atomically.
6. An extreme declaration block upgrades its complete sequence to a lazy
   arena-allocated vector; valid CSS never panics because compact bits run out.
7. `EffectiveKeyId` is stored directly on a uniquely owned declaration block
   and updated when selector or context identity changes.
8. B+ page movement copies compact IDs and arena boxes, not large declaration
   payloads.
9. Store-owned candidate aggregates replace separate semantic-order heaps.
10. Final B+ order is directly serializable; terminal compaction is optional.

## Deliberate boundaries

- The first version does not interpret at-rule equivalence. Unsupported or
  unmodeled wrappers contribute opaque occurrence identity.
- Flattening storage does not flatten CSS nesting semantics.
- Compact hints narrow candidate lookup but never replace exact equality.
- Selector component storage remains a benchmark decision.
- Arena addresses stabilize large payloads but are never semantic identities.
