# Radix source-order AST IR

> **Deprecated historical design.** The current AST storage design is
> [`docs/ast-storage.md`](../ast-storage.md). New implementation work must use
> responsibility-based `AstContext`, `NodeData`, and `ExtraData` terminology;
> the names in this directory are retained only to describe the superseded
> proposal.

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

- [Storage layout](./storage-layout.md) defines the three compiler-owned
  arenas, range references, level-order rule allocation, and AST-owned
  effective keys.
- [Radix index arena](./radix-index-arena.md) defines the primary/sibling
  sequence, lookup, insertion, iteration, and overflow rules.
- [ID encoding](./declaration-id-encoding.md) defines the `19 | 10 | 2` layout
  and the typed rule/block/declaration IDs over one encoding.
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
  append authored rules level by level and blocks in source order
  build children and declaration ranges over the three RadixIndexArena stores
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
  extend ranges and enqueue only edges and histories affected by each edit
       ↓
terminal cleanup
  verify queues are stable and discard merge-only sidecars
  optionally compact tombstones when measurement justifies it
       ↓
code generation
  walk children ranges recursively and stream primary slices and sibling
  Radix segments in semantic order
```

There is no production `walk_declaration_blocks` prepass, transient
`Vec<DeclarationBlockEntry>`, minify-time EffectiveKey reconstruction,
variable-length `SemanticOrderKey`, or mandatory full-store S5 rebuild.

## Decisions

1. There are exactly three node arenas: `RadixIndexArena<CssRule>`,
   `RadixIndexArena<DeclarationBlock>`, and
   `RadixIndexArena<DeclarationProperty>`. No other node collection exists.
2. A list is a range reference: `start` ID + `len`, a two-word window over one
   arena. The arena is the sequence; the range is the window.
3. Rules allocate in per-list order, level by level, so every rule's direct
   children form one contiguous primary range. Declaration blocks and their
   properties are single-range sequences by construction.
4. Rare local insertions use sparse sibling Radix trees stored separately from
   primary values.
5. Compact base IDs are `0 + 19-bit primary | 10-bit sibling | 2 reserved`;
   overflow base IDs are `1 + 29-bit dense index | 2 reserved`. The reserved
   bits are always zero for AST node IDs.
6. Numeric base-ID order and `RadixIndexArena::semantic_iter` order are the
   authoritative semantic order inside one store; a range is a window in that
   order.
7. Direct sibling adjacency equals arena adjacency inside one range; there is
   no first/last/previous/next linked-list topology.
8. `EffectiveKeyId` is AST data owned by `DeclarationBlock`, not Nano discovery
   data.
9. S1/S3 insertions allocate a local sibling ID immediately, so newly exposed
   candidates can reference stable AST IDs without rebuilding the stylesheet.
10. Nano sidecars contain only transform state such as liveness, history links,
    summaries, revisions, and queue membership.
11. Exhausting the compact primary prefix continues in the dense overflow
    tail. Exhausting a local sibling skips the optional structural transform;
    valid CSS never depends on a debug assertion or panic.

## Removed target abstractions

The Radix design makes these earlier target abstractions unnecessary:

- global B+ stores and page aggregates;
- `SemanticOrderKey`, `SemanticSourceOrderKey`, and a source-order heap;
- a recursive AST walk that reconstructs `(block, EffectiveKey)` entries;
- a second flat block identity distinct from `DeclarationBlockId`;
- a logical-only synthesized rule that waits for a complete S5 store rebuild;
- declaration-block owner reconstruction through raw references;
- linked-list rule topology and source-order chains;
- the tri-state declaration list (`Range`/`Local4`/`Overflow`) and the local
  property sub-ID bits; and
- mandatory terminal copying solely to restore semantic order.

## Deliberate boundaries

- This branch does not interpret unsupported at-rule semantics. Opaque wrapper
  occurrences remain distinct context frames.
- Flattened storage does not flatten native CSS nesting semantics.
- Fingerprints and hashes only select interner candidates; exact equality is
  still authoritative.
- Selector component storage remains a separate benchmark decision.
- Arena addresses stabilize payloads but are not semantic IDs.
