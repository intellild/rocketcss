# Radix node ID encoding

## Layout

The common compact `RadixIndexId` is a non-negative `u32` with this layout:

```text
31                         12 11                         2 1         0
+----------------------------+---------------------------+-----------+
| 0 | primary index: 19      |    sibling key: 10        | reserved  |
+----------------------------+---------------------------+-----------+
```

- `primary index` directly addresses the first `2^19` authored entries.
- `sibling key` addresses a local inserted node below that primary. Zero means
  the primary itself; `1..=1023` address the two-level Radix tree.
- the two low bits are reserved and always zero for AST node IDs.

When that compact prefix fills, parsing keeps appending to the same physical
arena vector and switches to a dense authored-overflow layout:

```text
31 30                                                2 1         0
+--+---------------------------------------------------+-----------+
| 1|              dense overflow index: 29            | reserved  |
+--+---------------------------------------------------+-----------+
```

Overflow base IDs remain four bytes, sort after every compact ID, and keep the
reserved low bits zero. They intentionally have no local sibling key: optional
structural insertion whose endpoint is in the overflow tail is rejected, while
parsing, lookup, visitors, and codegen continue normally.

## Three typed arenas

There is exactly one arena per node kind. Typed wrappers prevent accidental
interchange of rule, block, and declaration IDs even though they share the
physical encoding:

```rust,ignore
struct RuleId(RadixIndexId);
struct DeclarationBlockId(RadixIndexId);
struct DeclarationPropertyId(RadixIndexId);

let rules: RadixIndexArena<CssRule>;
let declaration_blocks: RadixIndexArena<DeclarationBlock>;
let declarations: RadixIndexArena<DeclarationProperty>;
```

A list is a two-word range reference into its arena:

```rust,ignore
struct RuleRange { start: RuleId, len: u32 }
struct DeclarationRange { start: DeclarationPropertyId, len: u32 }
```

The `RuleRange` lives on the owning rule (`children`); the `DeclarationRange`
lives on the block. Neither is a separate collection of pointers.

## Ordering

Compare base node IDs directly; reserved bits are always zero. Compact
primary/sibling IDs sort before dense overflow IDs; within either region
numeric order is semantic order. For compact values in the same
`RadixIndexArena`:

```text
(primary A, sibling X) < (primary B, sibling Y)
```

exactly when the first node appears earlier in the arena's semantic sequence.

A range is a window in that semantic order: iterate from `start` forward,
taking `len` live nodes. This makes `DeclarationBlockId` sufficient as Nano's
stable source-order key. There is no separate variable-length
`SemanticOrderKey`.

## Initial allocation

Parsing appends authored base nodes with the reserved bits zero:

```rust,ignore
let rule = rules.push_primary(parsed_rule);
let block = declaration_blocks.push_primary(parsed_block);
let property = declarations.push_primary(parsed_property);
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

The transaction repairs every persistent reference to the relabeled IDs:
declaration-block owners, rule parents, candidate endpoints, history links, and
source-map or diagnostic references. Primary IDs never relabel. No later
authored node moves because a local insertion exhausted its gaps.

## Capacity fallback

There are two independent compact-representation limits:

- `2^19` authored primary values in the compact Radix prefix, followed by a
  dense overflow tail of almost `2^29` base IDs; and
- at most 1023 locally encoded siblings below one primary.

These are representation limits, not CSS validity limits. `RadixIndexArena`
automatically uses its dense authored tail when the compact primary prefix is
full, and AST transactions treat local sibling exhaustion as an optional
transform rejection. Public parsing/minification therefore does not panic or
reject otherwise valid CSS at the compact boundaries.

## Required tests

- Primary and sibling fields round-trip at minimum and maximum values.
- Numeric base-ID ordering matches semantic iteration.
- Multiple insertions between the same neighbors preserve order.
- Local relabel repairs every persistent reference.
- Sibling insertion sorts into its owning range without moving primaries.
- Retirement leaves a tombstone and decrements its range's `len`.
- Primary and sibling capacity overflow take the explicit fallback.
