# Radix node and declaration-property ID encoding

## Layout

`RadixIndexId` is a non-negative `u32` with this layout:

```text
31                         12 11                         2 1         0
+----------------------------+---------------------------+-----------+
|      primary index: 20     |    sibling key: 10        | property  |
+----------------------------+---------------------------+-----------+
```

- `primary index` addresses the dense parse vector and supports `2^20`
  authored entries per store.
- `sibling key` addresses a local inserted node below that primary. Zero means
  the primary itself; `1..=1023` address the two-level Radix tree.
- `property` is a two-bit declaration-property sub-index. It is zero for base
  rule/block IDs and can encode four local declaration-property identities.

Typed wrappers prevent accidental interchange of rule, block, and declaration
IDs even though they share the physical encoding.

## Base IDs and sub-IDs

A base AST node ID has property bits zero:

```rust,ignore
struct RuleId(RadixIndexId);
struct DeclarationBlockId(RadixIndexId);

assert_eq!(rule_id.property_index(), 0);
assert_eq!(block_id.property_index(), 0);
```

`with_property_index(0..=3)` creates a sub-ID with the same base storage
location. Base lookup masks the low two bits, so these all identify the same
owning block while retaining a compact property identity:

```text
block ID       P | S | 00
property #0    P | S | 00
property #1    P | S | 01
property #2    P | S | 10
property #3    P | S | 11
```

Destructive base-node operations must reject a nonzero property index. A
property sub-ID must never remove or retire its whole declaration block.

## The low two bits are not the general declaration-list limit

RocketCSS must represent declaration blocks larger than four properties. The
two property bits are a compact identity for small synthesized/local cases and
an emergency local representation when a block is already using the rare
sibling address space. They do not impose a four-property CSS limit.

The common authored declaration run remains a compact range in the global
source-order declaration tape. A transformed block may use:

1. a consecutive range in that tape;
2. a small local set addressable by the two property bits; or
3. a lazy arena overflow list containing the complete ordered declaration
   sequence.

The chosen representation is explicit in `DeclarationList`; consumers never
guess from ID bit patterns alone.

```rust,ignore
enum DeclarationList<'ast> {
    Range(DeclarationRange),
    Local4(LocalPropertySet<'ast>),
    Overflow(ArenaVec<'ast, ArenaBox<'ast, Declaration<'ast>>>),
}
```

`Local4` is valid only when every live property has a unique `0..=3` index.
Adding a fifth upgrades the complete list to `Range` or `Overflow` before the
mutation commits.

## Ordering

Compare base node IDs after masking property bits. For two values in the same
`RadixIndexArena`:

```text
(primary A, sibling X) < (primary B, sibling Y)
```

exactly when the first node appears earlier in the arena's semantic sequence.
Property indices order properties only inside a `Local4` block; they do not
reorder declaration ranges or fallback chains stored elsewhere.

This makes `DeclarationBlockId` sufficient as Nano's stable source-order key.
There is no separate variable-length `SemanticOrderKey`.

## Initial allocation

Parsing appends authored base nodes with sibling and property bits zero:

```rust,ignore
let block = declaration_blocks.push_primary(parsed_block);
```

Structural insertion chooses a nonzero sibling key below a primary anchor:

```rust,ignore
let inserted = declaration_blocks.insert_sibling(anchor, sibling_key, block);
```

Keys should initially leave numeric gaps when multiple synthesized siblings
may be inserted in one interval. Correctness does not depend on the heuristic.

## Local relabel

If no key exists between two sibling neighbors, relabel only the siblings
under their shared primary. The store returns an exact remap:

```rust,ignore
struct RadixIdRemap {
    old: RadixIndexId,
    new: RadixIndexId,
}
```

The transaction repairs:

- rule parent/list/previous/next topology;
- declaration-block owner IDs;
- candidate endpoints and queue-membership indices;
- S2 history links;
- source-map or diagnostic references that persist past mutation; and
- any small property sub-ID by preserving its low two bits.

Primary IDs never relabel. No later authored node moves because a local
insertion exhausted its gaps.

## Capacity fallback

There are three independent capacity limits:

- at most `2^20` primary values in one store;
- at most 1023 locally encoded siblings below one primary; and
- at most four `Local4` properties in one block.

These are representation limits, not CSS validity limits. The owning compiler
must choose a documented fallback, such as another store segment or an arena
overflow sequence, before reaching the limit. Public parsing/minification must
not panic on valid input.

The current `RadixIndexArena` primitive enforces its compact limits. AST store
wrappers own the non-panicking segmentation/overflow policy because only they
know rule-list and declaration-list semantics.

## Required tests

- Primary, sibling, and property fields round-trip at minimum and maximum
  values.
- Numeric base-ID ordering matches semantic iteration.
- Property sub-IDs resolve the owning storage value.
- Destructive base mutation rejects a property sub-ID.
- Multiple insertions between the same neighbors preserve order.
- Local relabel preserves property bits and repairs every persistent reference.
- A fifth local property upgrades without changing output order.
- Primary, sibling, and property capacity overflow take the explicit fallback.
