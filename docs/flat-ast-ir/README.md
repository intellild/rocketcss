# Radix source-order AST IR

## Status

This directory defines the target physical AST representation built on
`rocketcss_common::RadixIndexArena`. It supersedes the earlier global B+ tree
design.

The workload is parse-first and insert-rare. Authored nodes stay in one dense
primary vector. A structural transform inserts rare nodes into a two-level
radix sequence owned by the preceding primary node. Stable compact IDs encode
both authored position and local insertion order.

This is a storage design, not a change to CSS semantics. Selector
compatibility, cascade order, nesting, fallback order, source locations, and
lossless output remain governed by the AST and S1-S5 semantic documents.

## Documents

- [Storage layout](./storage-layout.md) defines compiler-owned rule and block
  stores, topology, declaration lists, and AST-owned effective keys.
- [Radix index arena](./radix-index-arena.md) defines the primary/sibling
  sequence, lookup, insertion, iteration, and overflow rules.
- [ID encoding](./declaration-id-encoding.md) defines the `20 | 10 | 2` layout
  and the distinction between node IDs and declaration-property sub-IDs.
- [Effective keys](./effective-key.md) defines how parser context and canonical
  selectors produce the key stored on each declaration block.
- [Minification and commit](./minify-and-reification.md) describes how Nano
  operates directly on the AST without rebuilding a flat block table or a
  separate semantic-order domain.

The semantic validity of cross-rule transforms remains specified in
[Cross-rule declaration merging](../cross-rule-declaration-merging/overall.md).

## Target pipeline

```text
Compiler::parse
  append authored rules and blocks to primary RadixIndexArena vectors
  build direct parent/list/sibling topology
  intern wrapper/context paths while parsing
  write EffectiveKey seed directly on each DeclarationBlock
       ↓
rule-local minify
  canonicalize selectors
  finalize or replace DeclarationBlock.effective_key in place
       ↓
cross-rule minify
  iterate rule/block stores in RadixIndexId order
  use DeclarationBlockId as identity and source-order key
  insert synthesized nodes into local sibling Radix trees
  enqueue only edges and histories affected by each edit
       ↓
terminal cleanup
  verify queues are stable and discard merge-only sidecars
  optionally compact tombstones when measurement justifies it
       ↓
code generation
  stream primary slices and sibling Radix segments in semantic order
```

There is no production `walk_declaration_blocks` prepass, transient
`Vec<DeclarationBlockEntry>`, minify-time EffectiveKey reconstruction,
variable-length `SemanticOrderKey`, or mandatory full-store S5 rebuild.

## Decisions

1. Authored nodes use the dense primary vector in `RadixIndexArena`.
2. Rare local insertions use sparse sibling Radix trees stored separately from
   primary values.
3. The base node ID is `20-bit primary | 10-bit sibling | 2-bit property`.
4. Numeric base-ID order and `RadixIndexArena::semantic_iter` order are the
   authoritative source/semantic order inside one store.
5. Direct sibling validity still comes from AST topology; numeric proximity is
   not structural adjacency.
6. `EffectiveKeyId` is AST data owned by `DeclarationBlock`, not Nano discovery
   data.
7. S1/S3 insertions allocate a local sibling ID immediately, so newly exposed
   candidates can reference stable AST IDs without rebuilding the stylesheet.
8. Nano sidecars contain only transform state such as liveness, history links,
   summaries, revisions, and queue membership.
9. The low two ID bits are reserved for declaration-property sub-IDs and are
   ignored by base-node storage lookup.
10. Exhausting a local sibling or property encoding takes an explicit fallback;
    valid CSS must not depend on a debug assertion or panic.

## Removed target abstractions

The Radix design makes these earlier target abstractions unnecessary:

- global B+ stores and page aggregates;
- `SemanticOrderKey`, `SemanticSourceOrderKey`, and a source-order heap;
- a recursive AST walk that reconstructs `(block, EffectiveKey)` entries;
- a second flat block identity distinct from `DeclarationBlockId`;
- a logical-only synthesized rule that waits for a complete S5 store rebuild;
- declaration-block owner reconstruction through raw references; and
- mandatory terminal copying solely to restore semantic order.

## Deliberate boundaries

- This branch does not interpret unsupported at-rule semantics. Opaque wrapper
  occurrences remain distinct context frames.
- Flattened storage does not flatten native CSS nesting semantics.
- Fingerprints and hashes only select interner candidates; exact equality is
  still authoritative.
- Selector component storage remains a separate benchmark decision.
- Arena addresses stabilize payloads but are not semantic IDs.
