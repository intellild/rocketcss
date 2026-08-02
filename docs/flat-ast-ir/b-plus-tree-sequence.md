# B+ tree dense stores

## Status

Proposed design.

This document specifies the mutable sequence container used by the flat AST.
Rules, declaration blocks, and declarations are stored in global B+ tree dense
stores rather than nested Rust vectors. Former vector owners hold compact
`start ID + length` headers, and large payloads may remain arena boxed so page
movement copies only compact values.

The declaration key encoding and its rare overflow path are specified
separately in
[Declaration ID encoding](./declaration-id-encoding.md).

## Goals

- Insert or remove an AST entry without moving every later payload.
- Keep common list headers to a start ID plus length.
- Preserve lexical source order during parsing and semantic order during
  minification.
- Make B+ page split and redistribution local, bounded operations.
- Let code generation traverse final store order directly.
- Store candidate aggregates in the sequence so S3 does not need a separate
  ordered heap.
- Keep extreme compact-ID exhaustion correct through an explicit arena-vector
  fallback.

## Non-goals

- This design does not broaden S1-S4 CSS semantics.
- Hashes and fingerprints are never authoritative equality proof.
- It does not interpret unsupported at-rule equivalence.
- It does not require every selector component or declaration child value to
  become an independent dense node.
- It does not promise that valid CSS fits the compact declaration-ID encoding.

## Store shape

```rust,ignore
struct BTreeDenseStore<Id, T> {
    pages: DenseStore<PageId, SequencePage<Id, T>>,
    root: Option<PageId>,
    len: u32,
}

enum SequencePage<Id, T> {
    Internal(InternalPage),
    Leaf(LeafPage<Id, T>),
}

struct InternalPage {
    children: ArrayVec<PageId, INTERNAL_CAPACITY>,
    subtree_lens: ArrayVec<u32, INTERNAL_CAPACITY>,
    candidate_flags: CandidateFlags,
}

struct LeafPage<Id, T> {
    entries: ArrayVec<SequenceEntry<Id, T>, LEAF_CAPACITY>,
    previous: Option<PageId>,
    next: Option<PageId>,
    candidate_flags: CandidateFlags,
}

struct SequenceEntry<Id, T> {
    id: Id,
    value: T,
    flags: EntryFlags,
}
```

Page capacities are benchmark decisions. Internal nodes store subtree lengths
for indexed selection. Linked leaves make full traversal linear without
restarting from the root.

Large rule and declaration payloads use arena boxes:

```rust,ignore
type RuleStore<'ast> =
    BTreeDenseStore<RuleId, ArenaBox<'ast, CssRule<'ast>>>;

type DeclarationStore<'ast> = BTreeDenseStore<
    DeclarationId,
    ArenaBox<'ast, Declaration<'ast>>,
>;
```

Leaf insertion, redistribution, and split therefore move IDs, flags, and arena
pointers rather than complete declaration enums and token/value subtrees.

## Sequence list headers

A former declaration `Vec` is commonly:

```rust,ignore
struct CompactDeclarationList {
    start: Option<DeclarationId>,
    len: u32,
}
```

The header means:

```rust,ignore
declaration_store.iter_from(start).take(len)
```

It does not mean the integer interval `start..start + len`. IDs may have gaps.

The rule equivalent stores the first rule ID and the total preorder interval
length. Direct children remain distinguishable through `next_sibling` and
`subtree_len` topology.

## Why insertion does not invalidate every list

An array header stores a physical offset. Insertion before that offset changes
it even when the list itself is untouched. A B+ list header instead stores an
ID that remains attached to its first semantic entry.

```text
before insertion
  Block A: start=A0, len=2
  Block B: start=B0, len=2

insert inside A with a free ID label
  Block A: start=A0, len=3
  Block B: start=B0, len=2
```

Only insertion before A's first entry changes `A.start`. B is unaffected. A B+
page split changes neither start because the IDs are independent from physical
leaf slots.

Declaration IDs use spaced numeric labels. When a label interval is exhausted,
the store locally relabels entries and uses the encoded block hints to repair
only the few block headers that might contain a changed start. That exceptional
path is not equivalent to shifting every later array offset.

## Basic operations

For page capacity `B` and `N` entries:

| Operation                         | Complexity       |
| --------------------------------- | ---------------- |
| Find an ID/key                    | `O(log_B N)`     |
| Select by global rank             | `O(log_B N)`     |
| Insert or remove                  | `O(log_B N + B)` |
| Split or redistribute one page    | `O(B)`           |
| Pop earliest aggregated candidate | `O(log_B N)`     |
| Iterate linked leaves             | `O(N)`           |

The bounded `O(B)` movement copies compact leaf entries. Declaration value
size does not multiply page-split cost because the leaf holds an arena box.

## Declaration insertion and relabel

The declaration store normally chooses an unused `DeclarationId` between the
left and right semantic neighbors. Existing IDs and unrelated block headers do
not change.

If no key is available:

1. select a block-sized or leaf-sized local interval;
2. redistribute its declaration labels across the available numeric range;
3. return every `(old ID, new ID)` mapping;
4. decode the affected block hints;
5. repair exact matching compact block starts and remaining ID references; and
6. commit page keys and aggregates atomically.

If a compact range cannot be represented efficiently, its owning block
upgrades to an arena vector containing the complete declaration sequence.
Valid input never panics because a fixed bit allocation is full.

## Rule topology

The global rule store uses preorder. A rule stores direct structural links:

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

An inserted direct sibling is placed after the complete left subtree and
before the right sibling. The owning rule-list range, sibling links, and
ancestor subtree lengths update in one transaction. Candidate classification
must use direct sibling links, never mere numeric closeness in preorder.

Opaque wrapper identity uses stable occurrence IDs. Local B+ edits or
declaration relabeling cannot make unsupported conditional contexts equal.

## Candidate aggregation

S1 and S3 candidates are direct-sibling edges. The left rule entry stores edge
state, and every internal B+ page stores the union of candidate flags below it:

```rust,ignore
struct RuleEdgeState {
    flags: CandidateFlags,
    revision: u32,
}
```

To select the semantically earliest S3 candidate, descend through the leftmost
child whose aggregate contains S3, then inspect the first matching leaf entry.
The rule's `next_sibling` supplies the right endpoint.

```rust,ignore
fn pop_first_s3_candidate(&mut self) -> Option<(RuleId, RuleId)> {
    let leaf = self.leftmost_leaf_with(CandidateFlags::S3)?;
    let left = self.leaf(leaf).first_rule_with(CandidateFlags::S3)?;
    let right = self.rules[left].next_sibling?;
    Some((left, right))
}
```

This removes variable-length `SemanticOrderKey` comparison, S3
`BinaryHeap` maintenance, stale heap entries, and heap-membership
deduplication. Structural mutation reclassifies only edges incident to the
edited interval.

S2 histories remain separate semantic chains because they are keyed by
effective declaration history rather than sibling adjacency. They read source
order from the stores and participate in declaration-ID remap when they retain
an encoded ID.

## Declaration-block storage

Declaration blocks themselves live in a global ordered B+ store and carry
their context identity:

```rust,ignore
struct DeclarationBlock<'ast> {
    declarations: DeclarationList<'ast>,
    effective_key: EffectiveKeyId,
    flags: DeclarationBlockFlags,
    revision: u32,
}
```

The parser creates the block in lexical order and stores its context seed. The
selector-local minifier finalizes the canonical selector and
`EffectiveKeyId`. Cross-rule minification consumes that field directly; it does
not walk the recursive rule tree to reconstruct block ownership and wrapper
paths.

## Compact merge

Two compact declaration blocks merge without copying payloads when their
ranges are consecutive in the declaration B+ sequence:

```rust,ignore
merged.start = left.start;
merged.len = left.len + right.len;
```

The retained rule may reference the block header whose identity best preserves
the start ID's block-hint mapping. Rule identity and declaration-block identity
are independent.

If the ranges are not consecutive, the minifier may splice a B+ range into
semantic position. When that is not cheap or would complicate compact ID
repair, it upgrades the result to the arena-vector fallback. A compact range
never includes a live foreign declaration.

## Terminal freeze and code generation

The B+ stores already hold final semantic order. Code generation walks linked
leaves and follows list lengths:

```rust,ignore
for rule in compilation.rules.iter() {
    emit_rule(rule);
}
```

Compact declaration blocks iterate the global declaration store from `start`;
overflow blocks iterate their arena vector. A mandatory S5 copy into fresh flat
vectors is unnecessary.

The terminal phase may:

1. verify that candidate aggregates and dirty work are empty;
2. remove merge-only histories and flags;
3. merge underfull neighboring pages; and
4. compact overflow blocks only when benchmarks justify the copy.

These are cleanup choices, not prerequisites for correct serialization.

## Memory behavior

Preserve locality through these rules:

- use fixed-capacity pages and compact entries;
- allocate page stores from source-size estimates;
- keep large declaration and rule payloads behind arena boxes;
- link leaves for parse/minify/codegen traversal;
- avoid a heap allocation for every candidate or history edge;
- keep overflow vectors lazy; and
- benchmark page capacities and source-size reserve multipliers.

Deletion may leave underfull pages. Reuse retired pages through a store-owned
free list only when page initialization is provably safe; otherwise retain
them until the compilation is dropped.

## Required invariants

- B+ sequence order is the authoritative physical output order.
- Rule topology, not preorder adjacency, determines direct siblings.
- Every compact list start resolves to its first live sequence entry.
- Every compact declaration block owns exactly `len` successive live entries.
- Page split or redistribution alone does not change declaration IDs.
- Declaration relabel emits and commits a complete remap transaction.
- Block hints narrow repair candidates but exact start equality proves an
  update.
- Overflow blocks preserve complete semantic order and lossless values.
- Effective keys are valid for the current selector and exact context.
- Code generation emits each live rule and declaration exactly once.

## Implementation sequence

### Phase 1: generic B+ store

1. Implement fixed-capacity pages, subtree lengths, linked leaves, insertion,
   removal, split, and redistribution.
2. Benchmark inline values against arena-boxed values.
3. Add source-size capacity estimation and operation counters.

### Phase 2: rule and block stores

1. Move rule and declaration-block ownership into global B+ stores.
2. Replace nested vectors with compact list headers and explicit topology.
3. Port visitors and code generation to store accessors.

### Phase 3: declaration store and encoding

1. Store arena-boxed declarations in one global B+ store.
2. Implement compact `DeclarationId`, block-hint lookup, local relabel, and
   exact remap repair.
3. Add the lazy full-block overflow vector.
4. Test lexical allocation order and extreme valid blocks.

### Phase 4: minify integration

1. Store `EffectiveKeyId` directly on declaration blocks.
2. Remove recursive block collection and key reconstruction.
3. Move S1/S3 edge scheduling into B+ candidate aggregates.
4. Move remaining history and summary state into store-owned sidecars.
5. Remove mandatory full AST reification.

## Benchmark requirements

- Bootstrap and Tailwind parse/minify/codegen pipelines;
- no-op minification over many unique rules;
- declaration-heavy blocks with no structural changes;
- repeated middle insertion with and without a free ID gap;
- block-local relabel and hint-based repair;
- compact-to-overflow upgrade;
- long S1 runs and repeated S3 factoring;
- deeply nested rules and many small rule lists;
- one very large flat rule list; and
- code generation directly from linked leaves.

Count page entries moved, page splits, relabeled declaration IDs, block
candidates inspected per relabel, overflow upgrades, allocation count, bytes
copied, and complete compilation time. The design succeeds only if it removes
suffix movement and global rescans without making ordinary traversal slower
than the saved work.
