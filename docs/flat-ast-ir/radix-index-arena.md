# Radix index arena

## Workload

The AST is parsed once, then structurally edited relatively few times. A plain
`Vec` is optimal for parsing and traversal but expensive for middle insertion.
A B+ tree improves insertion but makes every parse, lookup, and traversal pay
tree costs.

`RadixIndexArena` keeps the common representation linear and moves only rare
inserted siblings into sparse radix storage:

```rust,ignore
struct RadixIndexArena<'arena, T> {
    allocator: &'arena Allocator,
    primary: ArenaVec<'arena, T>,
    sibling_primary_indices: ArenaVec<'arena, u32>,
    sibling_trees: ArenaVec<'arena, RadixTree<'arena, T>>,
    len: u32,
}
```

The single `primary` vector has two ID regions but one payload allocation:

```text
primary[0 .. 2^19)       compact Radix primary IDs
primary[2^19 .. end)     high-bit-tagged dense overflow IDs
```

Normal files never enter the second region. Very large valid inputs retain the
same contiguous traversal and payload layout; only optional local insertion
inside the overflow tail is unavailable.

The two sibling vectors are a structure-of-arrays pair. Binary search touches
only compact `u32` primary indices. The matching tree pointer is loaded after a
match. Both vectors have identical length and corresponding indices.

## ID and sequence model

Every compact primary receives a direct ID from its vector index. An inserted
value receives the same primary index plus a nonzero local sibling key.

```text
primary P, sibling key 0      authored node P
primary P, sibling key 1..N   inserted nodes immediately after P
primary P+1, sibling key 0    next authored node
overflow O                    authored node after the compact prefix
```

Within one arena, base-ID numeric order is semantic order. The low two bits are
reserved and always zero for AST node IDs; see
[ID encoding](./declaration-id-encoding.md).

## Sparse two-level Radix

The sibling key is ten bits split into two five-bit levels:

```text
sibling key
  high 5 bits -> root branch
  low  5 bits == 0 -> root direct value
  low  5 bits != 0 -> leaf slot
```

The low-zero value is stored directly in the root branch. A second-level leaf
is allocated only for a nonzero low part, and its slot zero is unused. Root,
leaf, and value storage are arena-owned boxes, so page size is independent of
the payload type. Root and leaf pages use separate live and used masks: a
retired key remains unavailable while an empty leaf may stay allocated for its
used mask. A primary that never receives an insertion has no sibling entry and
allocates no Radix page.

The sparse branch shape is therefore:

```rust,ignore
struct RadixRoot<'arena, T> {
    direct: [Option<ArenaBox<'arena, T>>; 32],
    leaves: [Option<ArenaBox<'arena, RadixLeaf<'arena, T>>>; 32],
    direct_occupied: u32,
    direct_used: u32,
    occupied_branches: u32,
}

struct RadixLeaf<'arena, T> {
    values: [Option<ArenaBox<'arena, T>>; 32],
    occupied: u32,
    used: u32,
}
```

The sorted `sibling_primary_indices` vector locates the Radix tree for lookup or
mutation. This is intentionally sparse: a `Vec<Option<TreePointer>>` would add
one pointer-sized slot for every authored node and force parse-time
initialization even when no transform inserts anything.

## Operations

Let `P` be the authored-node count, `G` the number of primary nodes with at
least one inserted sibling, and `S` the number of siblings below one primary.

| Operation                               | Cost                                       |
| --------------------------------------- | ------------------------------------------ |
| Append authored primary                 | amortized `O(1)`                           |
| Get primary ID                          | `O(1)`                                     |
| Find a sibling group                    | `O(log G)` over compact `u32` values       |
| Get inserted sibling                    | `O(log G + 1)` with two fixed Radix levels |
| Insert a sibling into an existing group | fixed-depth Radix insertion                |
| Create a new sibling group              | `O(G)` compact SoA insertion               |
| Iterate primary-only store              | `O(P)` contiguous slice traversal          |
| Iterate semantic sequence               | `O(P + inserted)` segmented traversal      |

The `O(G)` new-group insertion is accepted because structural insertion is
rare and `G` is normally tiny compared with `P`. If profiles show many distinct
edited primary nodes, add a sparse group index without changing primary
storage.

## Segmented iteration

Semantic traversal uses two cursors:

- the next primary position; and
- the next sparse sibling group.

The iterator alternates whole primary slices and sibling Radix iterators:

```text
primary[start .. group.primary + 1]
       ↓
group.sibling_tree in key order
       ↓
next primary slice
```

`Primary`, `Siblings`, and `Done` are iterator-internal states. Sibling lookup
and binary search do not run once per primary element. State changes occur only
when the current segment is exhausted.

Codegen and passes that know no siblings exist use `primary_iter()` directly.
Passes that need transformed semantic order use `semantic_iter()`.

## AST sequences and ranges

Rules allocate in per-list order, level by level, so each rule's direct
children form one contiguous primary range; declaration blocks and declaration
properties are single-range sequences by construction. See
[Storage layout](./storage-layout.md) for the allocation invariants. The Radix
ID supplies stable identity and ordering; the range supplies the sequence.

When Nano inserts a synthesized direct sibling:

1. choose a sibling key between its local semantic neighbors;
2. insert the value into the owning primary's Radix tree;
3. update the affected rule or declaration range (`len += 1`); and
4. bind any owned declaration block and EffectiveKey immediately.

No global AST walk or renumbering of later primary nodes is required.

## Local key allocation

Sibling key zero is reserved for the primary. Keys `1..=1023` are assigned with
gaps so insertion between existing siblings normally chooses a midpoint.

When an interval has no free key, the arena may relabel only the siblings below
that primary and return an exact old-to-new ID remap. Candidate queues and the
few ranges/references to those rare IDs are repaired in the same transaction.

If one compact primary requires more than 1023 live inserted siblings, the AST
transaction rejects that optional structural optimization and preserves the
unmerged CSS. Overflow-primary endpoints likewise reject local insertion.
Valid CSS never fails because the local label space is full.

## Mutation and stale work

Candidates carry endpoint IDs and revisions. A local edit increments affected
revisions and enqueues newly exposed work. When popped, a candidate validates:

- both IDs still resolve and are live;
- they remain mutual direct siblings in the same rule list and segment;
- the stored revisions still match; and
- current EffectiveKey equality still classifies the edge for S1 or S3.

Retired nodes may remain as tombstones until terminal cleanup. Their IDs are not
reused during the compilation.

Direct siblings are arena-adjacent inside one range, so finding the real
neighbors for an insertion is a short tombstone-skipping scan of that range;
there is no `previous_in_source`/`next_in_source` chain to walk. The range is
unaffected by tombstones in sibling regions and does not record an insertion
position beyond its window.

## Measured trade-off

For 65,536 primary pointer-sized values with one inserted sibling every 256
values, local measurements showed:

| Operation                 |         `Vec` |        B+ tree |         Radix |
| ------------------------- | ------------: | -------------: | ------------: |
| Primary build             |   about 31 us |  about 2.05 ms |   about 36 us |
| Primary traversal         | about 4.33 us | about 25.16 us | about 4.12 us |
| Sparse build              |  about 924 us |  about 2.13 ms |   about 60 us |
| Sparse semantic traversal | about 3.81 us | about 22.58 us | about 30.5 us |

Splitting `primary_index` and `RadixTree` into two sibling arrays reduced a
sibling-only lookup workload from about 17.62 us to 13.91 us. The target keeps
this SoA layout.

The remaining cost is semantic iteration after insertions. It is paid only by
consumers that must observe inserted siblings and does not affect primary parse
or primary-only traversal.

## Required invariants

- Primary IDs are stable for the lifetime of the compilation.
- Base node IDs always have the reserved bits zero.
- Sibling groups are sorted by primary index and the SoA lengths match.
- Sibling key order equals semantic insertion order below one primary.
- A rule's direct children are one contiguous semantic window (level-order
  allocation); a block's declarations are one contiguous semantic window.
- A local relabel repairs all persistent references atomically.
- Valid input has a non-panicking overflow path.
