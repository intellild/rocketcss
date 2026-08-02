# Storage layout

## Compilation-owned stores

`Compiler` owns the source, source map, atom pool, arena, and the mutable stores
for one parsed compilation. `StyleSheet` is a compact range header and metadata;
it does not own a recursive Rust object graph.

```rust,ignore
struct Compilation<'ast> {
    arena: Allocator,

    rules: BTreeDenseStore<RuleId, ArenaBox<'ast, CssRule<'ast>>>,
    declaration_blocks:
        BTreeDenseStore<DeclarationBlockId, DeclarationBlock<'ast>>,
    declarations:
        BTreeDenseStore<DeclarationId, ArenaBox<'ast, Declaration<'ast>>>,

    selectors: DenseStore<SelectorValueId, SelectorData<'ast>>,
    context_paths: ContextPathInterner,
    effective_keys: EffectiveKeyInterner,
    declaration_block_hints: DeclarationBlockHintIndex<'ast>,
}

struct StyleSheet {
    rules: RuleList,
    source: SourceId,
}
```

Rules, declaration blocks, and declarations have one global ordered store per
entity kind. Former `Vec<T>` owners hold compact list headers into those stores.
The B+ pages supply mutable sequence order; dense IDs and typed accessors keep
relationships compact.

## `BTreeDenseStore`

The common sequence abstraction combines:

- fixed-capacity B+ pages allocated from compiler-owned storage;
- compact typed IDs or keys in leaf entries;
- order-statistic subtree lengths for indexed access;
- linked leaves for linear iteration; and
- values stored inline only when small, otherwise behind arena boxes.

```rust,ignore
struct BTreeDenseStore<Id, T> {
    pages: DenseStore<PageId, SequencePage<Id, T>>,
    root: Option<PageId>,
    len: u32,
}
```

The exact internal lookup strategy may differ by ID kind. `DeclarationId` is a
special order-maintenance key described in
[Declaration ID encoding](./declaration-id-encoding.md). Rule and block stores
may use stable dense IDs plus a compact position sidecar if benchmarks favor
that representation.

Moving or splitting a leaf containing `ArenaBox<Declaration>` copies only a
pointer-sized value. The declaration enum and its nested value data remain at
their arena address. This is why a mutable global sequence does not imply
moving every declaration payload after an insertion.

## List headers replace AST vectors

The common declaration list is:

```rust,ignore
struct CompactDeclarationList {
    start: Option<DeclarationId>,
    len: u32,
}
```

`start` is a sequence key, not a physical array offset. `start + len` means
locate `start` in the declaration B+ store and consume `len` successive
entries. Insertion before an unrelated block therefore does not change this
header.

Rule lists use the equivalent concept:

```rust,ignore
struct RuleList {
    start: Option<RuleId>,
    subtree_len: u32,
}
```

The rule store is global preorder. `subtree_len` counts the complete preorder
interval owned by the list, including descendant rules. Direct siblings are
followed through topology metadata rather than by treating every entry in the
interval as a direct child.

## Flat rule topology

Preorder alone cannot answer direct-sibling queries because descendants are
interleaved. Each rule stores explicit topology:

```rust,ignore
struct CssRule<'ast> {
    parent: Option<RuleId>,
    parent_list: RuleListId,
    previous_sibling: Option<RuleId>,
    next_sibling: Option<RuleId>,
    subtree_len: u32,
    payload: CssRulePayload<'ast>,
    flags: CssRuleFlags,
    revision: u32,
}
```

A structural insertion updates the modified rule list, direct-sibling links,
and ancestor subtree lengths. It does not shift every later rule payload or
rebuild the recursive AST.

Rule relationships use IDs and store accessors. Visitors do not retain mutable
references while a B+ store may split or grow; structural visitors submit
local rewrite operations through the owning `Compilation`.

## Source-order allocation invariant

The parser inserts authored rules, declaration blocks, and declarations when
it encounters them in lexical source order.

```css
a {
  x: 1;
}
a {
  y: 1;
  & b {
    z: 1;
  }
}
```

The authored declaration order is `x, y, z`. The parent declaration run must
not wait until recursive child parsing finishes, which would incorrectly
produce `x, z, y`.

An empty leading declaration run captures its sequence cursor before a nested
child:

```css
a {
  & b {
    z: 1;
  }
  w: 1;
}
```

The parent has an empty leading block before `z`; `w` belongs to a later
`NestedDeclarationsRule` block. No `start + len` extension may absorb `z` into
either parent declaration run.

Parser tests must inspect the store order, block starts, and lengths directly.
Serialized output alone can hide an allocation-order bug.

## Declaration blocks

```rust,ignore
struct DeclarationBlock<'ast> {
    declarations: DeclarationList<'ast>,
    effective_key: EffectiveKeyId,
    flags: DeclarationBlockFlags,
    revision: u32,
}

struct DeclarationList<'ast> {
    start: Option<DeclarationId>,
    len: u32,
    overflow: Option<
        ArenaBox<
            'ast,
            ArenaVec<'ast, ArenaBox<'ast, Declaration<'ast>>>,
        >,
    >,
}
```

The common block has one compact range over the global declaration B+ store.
Its overflow field is null. The arena vector is allocated only for an extreme
block that exceeds the compact ID space, repeatedly exhausts label gaps, or
cannot cheaply remain one contiguous B+ interval. When nonnull, it stores the
complete semantic declaration sequence, `start` is cleared, and `len` mirrors
the vector length. Arbitrary insertion and serialization therefore do not
merge two ordering schemes.

The exact Rust layout may replace the arena box with a compact
`OverflowDeclarationListId` if that keeps the common block smaller. The
semantic distinction between a null compact range and a nonnull complete
overflow vector must remain explicit.

Each live block header has one semantic owning syntax position. Because
ownership is unique, the exact selector and cascade context can be cached
directly as `effective_key`; no `DeclarationOccurrence` wrapper or minify-time
owner reconstruction is required.

Importance remains declaration-local when a block mixes normal and important
declarations. A history key combines the block's `EffectiveKeyId` with the
declaration's importance phase.

## Compact range operations

Two compact blocks can coalesce without copying declaration values when their
ranges are consecutive in global B+ order and their semantic order matches the
physical order:

```rust,ignore
merged.start = left.start;
merged.len = left.len + right.len;
```

If the ranges are not consecutive, the transform must either:

1. splice the relevant B+ ranges into consecutive semantic order; or
2. fill the result's complete overflow vector.

A compact `start + len` must never include a live declaration belonging to a
nested or neighboring block.

Deletion of the first declaration advances `start` to the next live sequence
entry and moves the block between old/new hint-index candidates when necessary.
Middle deletion removes or tombstones the entry according to the active minify
phase and decrements the block length when the physical sequence is committed.

## Declaration ID repair

B+ page split and redistribution do not invalidate compact declaration IDs.
Only exhaustion of an order-label interval triggers local ID relabeling. The
encoded block hint narrows repair to the few block headers that could start at
an affected ID:

```text
local DeclarationId remap
  decode old/new BlockHint values
       ↓
DeclarationBlockHintIndex
  return one common candidate or rare aliases
       ↓
exactly compare candidate.start with old ID
       ↓
update matching compact headers and remaining ID references
```

The complete encoding, alias, remap, and overflow rules are specified in
[Declaration ID encoding](./declaration-id-encoding.md).

## Effective-key ownership

The parser already knows the conditional path, layer, origin, and containing
selector when it creates a declaration block. It interns a context seed at
that point and stores the final `EffectiveKeyId` directly on the block after
selector-local normalization.

Selector replacement, S3 selector union, or movement into a different context
must recompute the affected key immediately. Declaration-only changes leave it
unchanged. Unsupported at-rules contribute opaque occurrence identity; this
design does not infer their semantics.

## Selectors and values

Selector names continue to use compiler-scoped `Atom` values. Complete
selector values may be hash-consed to a canonical `SelectorValueId`, with exact
equality after hash-bucket selection. Occurrence IDs and canonical value IDs
remain distinct.

Flattening every selector component into an independent node may cost more
indirection than it saves. A selector component sequence with a compact list
header should be benchmarked against per-component IDs before fixing the
public representation.

## Arena ownership

The B+ stores own sequence pages and compact headers; the compiler arena owns
large AST payloads and the rare overflow vectors. Arena allocation provides
stable payload addresses, while all graph relationships still use typed IDs.

The arena must not become an implicit identity system. Growing, splitting, or
relabeling a B+ sequence is expressed through store APIs, and no consumer may
derive semantic identity from an arena pointer. `StringPool` remains a
compiler-owned interner whose atoms follow its own documented identity rules.

## Required invariants

- Rule, block, and declaration store order initially follows lexical source
  order.
- Every compact declaration block owns exactly `len` successive live entries
  beginning at `start`.
- Every overflow block owns the complete ordered vector stored in its fallback.
- Direct sibling traversal uses topology and never skips a nested subtree.
- B+ page movement copies compact entries or arena pointers, not declaration
  payloads.
- Effective-key equality is compact ID equality backed by exact canonical
  interning.
- A local ID relabel repairs all persistent references atomically.
- Valid CSS exceeding a compact encoding limit takes a fallback and does not
  panic.
