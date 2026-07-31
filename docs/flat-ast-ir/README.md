# Flat source-order AST IR

## Status

This directory defines the target physical representation for RocketCSS's CSS
AST. It replaces the current tree of boxed and arena-allocated nodes with dense
stores and source-order tapes addressed by compact `u32` IDs.

The semantic CSS tree still exists. Parent, sibling, and subtree metadata encode
that tree without making Rust ownership reproduce it. A formatted stylesheet
with one property per line is a useful mental model: parsing appends syntax and
properties in lexical order, while IDs and ranges recover structure.

This is a storage design, not permission to change CSS semantics. Parsing,
minification, visitors, and code generation must preserve lossless syntax,
cascade order, nesting, fallback chains, and source locations.

## Documents

- [Storage layout](./storage-layout.md) specifies dense stores, source-order
  allocation, tree topology, declaration ranges, and allocator removal.
- [Effective keys](./effective-key.md) specifies canonical context identities
  stored with declaration occurrences.
- [Minification and reification](./minify-and-reification.md) maps the S1-S5
  cross-rule merge design onto the flat representation.

The cross-rule algorithm remains specified in
[Cross-rule declaration merging](../cross-rule-declaration-merging/overall.md).
Those documents describe semantic states; this directory defines their target
physical storage.

## Target pipeline

```text
Compiler::parse
  append source-order syntax nodes and typed payloads
       ↓
  append every property to one declaration tape in lexical order
       ↓
  finalize selector/context IDs and declaration-block headers
       ↓
local minify
  replace payload IDs or tombstone slots; do not move live source occurrences
       ↓
S1/S2/S3/S4
  operate on IDs, ranges, live links, effects, and rewrite plans
       ↓
S5
  rebuild compact tapes in final semantic order
       ↓
code generation
  scan compact syntax and declaration tapes
```

## Decisions

1. Every stable AST entity is addressed by a typed dense ID. An ID identifies
   a store slot, not a memory address.
2. Authored rule and property allocation order is lexical source order.
3. Tree relationships are explicit metadata, not Rust pointer ownership.
4. A declaration block is a header over a range in the global declaration
   tape. Importance and liveness use compact sidecar storage.
5. Effective selector and conditional identities are canonical dense IDs.
6. Structural rewrites are planned and committed by reification; appending a
   synthesized node does not determine its semantic output position.
7. The current AST `Box` values and AST arena allocator have no role in the
   target representation and are removed after all owning nodes migrate.

## Deliberate boundaries

- The first version does not interpret at-rule equivalence. Unsupported or
  unmodeled wrappers contribute an opaque occurrence ID to the context.
- Flattening storage does not flatten CSS nesting semantics.
- It is not yet decided whether every selector component deserves its own ID.
  A component tape plus ranges may have better locality than per-component
  indirection and must be benchmarked.
- A compiler may retain a scratch allocator for short-lived analyses and a
  string pool for atoms. Neither is the old AST ownership allocator.

