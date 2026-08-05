# Radix source-order AST IR

## Status

This directory describes the compiler-owned flat AST built on
`rocketcss_common::RadixIndexArena`.

The workload is parse-first and insert-rare. Authored rules stay in a dense
primary vector in lexical preorder. Structural transforms place rare
synthesized rules in sparse Radix sibling storage without moving later primary
nodes.

This is a storage design, not a change to CSS semantics. Selector
compatibility, cascade order, nesting, fallback order, source locations, and
lossless output remain AST responsibilities.

## Documents

- [Storage layout](./storage-layout.md) defines lexical-preorder rule storage,
  subtree spans, tombstones, declaration representations, and the public AST
  traversal boundary.
- [Radix index arena](./radix-index-arena.md) defines primary/sibling storage,
  lookup, local insertion, semantic cursors, relabeling, and overflow rules.
- [ID encoding](./declaration-id-encoding.md) defines compact typed IDs.
- [Effective keys](./effective-key.md) defines parser context and the key owned
  by each declaration block.
- [Minification and commit](./minify-and-reification.md) describes how Nano
  mutates the authoritative AST without rebuilding a second rule topology.

## Pipeline

```text
Compiler::parse
  allocate each rule before descending into its body
  append rules in lexical preorder
  let AST update ancestor subtree counts
  append declaration occurrences to the lexical tape
       ↓
rule-local minify
  mutate rule payloads and declaration blocks through AST transactions
       ↓
cross-rule minify
  query semantic parents/siblings from AST
  insert synthesized rules at final Radix positions
  retire nodes as tombstones
       ↓
code generation / visitors
  traverse root_rules and nested_rules
  never inspect subtree spans or Radix cursor mechanics
```

## Decisions

1. One rule arena is the authoritative tree and source-order sequence.
2. Parsed rules allocate in depth-first lexical preorder.
3. Each `RuleRecord` stores `parent` plus one `u32 nested_rule_count` covering
   all physical descendants; it does not store child lists or links.
4. Direct children are derived by jumping over each child's complete subtree.
5. The stylesheet root is the whole rules arena; no `StyleSheet.root_rules`
   handle or `RuleListId` exists.
6. Tombstones remain inside subtree spans, while semantic iterators filter
   them out.
7. Rare local insertions use sparse sibling Radix trees. Local relabeling
   returns explicit ID remaps that the AST repairs atomically.
8. Tree mechanics are private to the AST crate. Parser, codegen, visitor, and
   Nano use semantic AST operations only.
9. Declaration blocks own their `EffectiveKeyId` and ordered declaration
   representation (`Range`, `Local4`, or `Overflow`).
10. Valid CSS does not depend on optional local insertion capacity; exhausting
    a compact insertion namespace rejects that optimization safely.

## Removed abstractions

- global B+ stores and variable-length semantic order keys;
- `RuleListId`, `RuleListStore`, and per-rule child-list handles;
- first/last/previous/next sibling links;
- previous/next source-order links;
- descendants vectors or per-rule `Vec<RadixRange<_>>` metadata;
- a second flat block identity distinct from `DeclarationBlockId`;
- minify-time EffectiveKey reconstruction; and
- mandatory terminal copying solely to restore rule order.

## Boundaries

- Flattened storage does not flatten native CSS nesting semantics.
- Unsupported at-rule bodies remain opaque when no typed representation
  exists.
- Fingerprints and hashes select interner candidates; exact equality remains
  authoritative.
- Arena addresses stabilize payload storage but are not semantic identity.
