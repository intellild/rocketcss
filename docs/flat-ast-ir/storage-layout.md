# Storage layout

## Compiler ownership

`Compiler` owns one compilation's source, source map, atom pool, arena, AST
stores, and semantic interners. There are exactly three node stores, one
`RadixIndexArena` per node kind:

```rust,ignore
struct Compilation<'ast> {
    allocator: Allocator,
    string_pool: StringPool<'ast>,

    rules: RadixIndexArena<CssRule<'ast>>,
    declaration_blocks: RadixIndexArena<DeclarationBlock<'ast>>,
    declarations: RadixIndexArena<DeclarationProperty<'ast>>,

    selector_values: SelectorValueInterner,
    context_paths: ContextPathInterner,
    effective_keys: EffectiveKeyInterner,
}
```

Every rule, declaration block, and declaration property is a node in the
corresponding arena with a stable four-byte typed ID. A "list" is never a
separate collection of pointers: it is a range reference into one arena. The
arena is the sequence; the range is a two-word window descriptor.

## Lists are range references

`Vec<CssRule>` and `Vec<Declaration>` do not exist as physical containers.
Both the root rule list and every nested rule list are one range over the
rules arena:

```rust,ignore
struct RuleRange {
    start: RuleId,
    len: u32,
}

struct StyleSheet {
    root_rules: RuleRange,
}
```

A rule owns its direct children as an inline range; no separate rule-list
store is required:

```rust,ignore
struct CssRule<'ast> {
    payload: CssRulePayload<'ast>,
    parent: Option<RuleId>,
    children: Option<RuleRange>,
    declaration_block: Option<DeclarationBlockId>,
    revision: u32,
    live: bool,
}
```

A declaration block references its ordered declarations as one range over the
declaration-property arena:

```rust,ignore
struct DeclarationRange {
    start: DeclarationPropertyId,
    len: u32,
}

struct DeclarationBlock<'ast> {
    owner: RuleId,
    declarations: DeclarationRange,
    effective_key: EffectiveKeyId,
    revision: u32,
    live: bool,
}
```

A range is a window in the arena's semantic ID order: iterate from `start`
forward, taking `len` live nodes. `RadixIndexId` numeric order equals semantic
order (primary node, then its locally inserted siblings, then the next primary),
so the window is a real slice in ID order even after rare local insertions.

There is no `first`/`last`/`previous_sibling`/`next_sibling` topology, no
`previous_in_source`/`next_in_source` chain, and no per-rule child-list
indirection. Direct CSS sibling adjacency of a list is exactly arena adjacency
inside that list's range. Linked lists existed only to paper over noncontiguous
allocation; the allocation invariants below make them unnecessary.

## Allocation invariants

### Rules allocate level by level

Depth-first parsing would interleave a nested rule's descendants between its
parent's direct children, so no single list would be contiguous. Instead the
rules arena is filled in per-list order:

1. append every direct child of the current list contiguously; then
2. queue each appended rule's body; and
3. process the queue so the next level's rules (the children of the rules just
   appended) become the next contiguous region.

Equivalently, the rules arena stores the CSS rule tree in breadth-first order.
Each rule's `children` range is a contiguous primary slice:

```text
level 0  a    media   c              root_rules = (a, 3)
level 1  b  b1   d                   a.children = (b, 2)   media.children = (d, 1)
level 2  e                           b1.children = (e, 1)
```

Arena order is `[a, media, c, b, b1, d, e]`; every range is a contiguous window.

Code generation, visitor walks, and minify traversal read the tree by
recursively following `children` ranges, so source order is preserved even
though the arena is not in lexical preorder. The global arena order is
deterministic; it is simply not a source-order scan.

The parser's nested-body queue replaces immediate `parse_nested_block`
recursion while still reading the source once.

### Declaration ranges never interleave

A block's declarations are appended contiguously in source order. Nested rules
close the current declaration run before their own declarations are parsed, and
post-nesting declarations of the parent go to a distinct
`NestedDeclarationsRule` block. No block's range ever absorbs a descendant's
properties, so the declaration-property arena needs no level ordering: its
primary order is lexical source order and every block range is a contiguous
slice.

## Iteration

Range iteration is store-native:

```rust,ignore
for rule_id in compilation.rules.window(root_rules) {
    consume(rule_id);
}
```

`window(range)` walks the arena's semantic iterator from `start` and takes
`len` live nodes, skipping tombstones. When no local insertion exists, the walk
is a plain primary slice. There is no recursive AST walk that rebuilds a
transient flat vector.

## Mutation

### Insertion

A synthesized rule or block is inserted at its final semantic position with the
arena's local sibling mechanism (`insert_between`/`insert_sibling`). The new
node's ID sorts into the owning range; the range's `len` increments. No primary
node moves, no later ID changes, and no list bookkeeping beyond the range
update is needed.

### Retirement

Retiring a rule or block keeps its ID as a tombstone and decrements the owning
range's `len`. Window iteration skips tombstones. `start` need not move: it
names the first member's stable ID, and iteration filters non-live nodes.

Because direct siblings are arena-adjacent, finding the real neighbors for an
insertion is a short tombstone-skipping scan of the range, not a walk over a
`previous_in_source` chain.

## Effective keys

Effective keys are unchanged by this layout and remain AST data owned by the
block. See [Effective keys](./effective-key.md).

## Code generation

Codegen starts from `StyleSheet.root_rules`, follows each `children` range
recursively, and reads values from the Radix stores. Primary-only ranges use
contiguous traversal; ranges that received a local insertion merge primary
segments with sibling Radix segments.

## Required invariants

- There are exactly three node arenas; no other node collection exists.
- Every live rule and every live declaration block belongs to exactly one range.
- A rule's direct children are one contiguous semantic window (level-order
  allocation).
- A block's declarations are one contiguous semantic window (never interleaved).
- `RuleRange.start` names a stable ID; iteration counts live nodes only.
- Sibling insertion sorts into its range without moving primaries.
- Retirement leaves a tombstone and decrements its range's `len`.
- Direct sibling adjacency equals arena adjacency within one range.
- No arena pointer is used as semantic equality or persistent identity.
