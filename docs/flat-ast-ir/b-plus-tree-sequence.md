# B+ tree source-order sequence and stable rule identity

## Status

Proposed design.

This document refines the flat source-order AST target with a mutable B+ tree
sequence for CSS rules. It keeps rule and declaration-block identities stable,
stores declarations in lexical source order, and replaces variable-length
semantic order keys plus heap-based partial-merge scheduling.

The design is compatible with the declaration tape in
[Storage layout](./storage-layout.md), but changes the rule-storage and terminal
reification strategy described in
[Minification and reification](./minify-and-reification.md). Those documents
must be updated if this design is adopted.

## Goals

- Insert or remove a rule without moving every later AST element.
- Preserve the `CssRuleId` of a retained rule while its payload changes.
- Preserve the `DeclarationBlockId` of a retained declaration block while its
  declaration sequence changes representation.
- Allocate authored declarations in lexical source order across the complete
  stylesheet.
- Represent the common declaration block as one `(offset, len)` pair.
- Concatenate physically adjacent declaration blocks in constant time.
- Replace `SemanticOrderKey` and the S3 `BinaryHeap` with ordering and dirty
  candidate state maintained by the sequence container.
- Keep traversal cache-friendly by storing many compact rule entries in each
  leaf and linking leaves for linear iteration.
- Bound insertion, split, and relabel work to one or a few B+ tree leaves on
  ordinary inputs.

## Non-goals

- This design does not broaden S1-S4 CSS semantics.
- It does not make hashes authoritative for selector, declaration, or
  effective-key equality.
- It does not require every selector component or declaration value to become
  an independent dense node.
- It does not guarantee that every allocation in the compilation is one
  physically contiguous byte range. It instead keeps fixed-size pages and
  dense payload stores locally contiguous.
- It does not require a full AST compaction after every minification.

## Separate stable identity from mutable order

Three identities must remain distinct:

```rust,ignore
/// Stable identity of one rule payload for the compilation lifetime.
struct CssRuleId(NonMaxU32);

/// Stable identity of one declaration-block header.
struct DeclarationBlockId(NonMaxU32);

/// Mutable position label used only by ordered sequence storage.
struct SequenceKey(u32);
```

`CssRuleId` and `DeclarationBlockId` are dense store identities. They do not
encode source position and are never changed by leaf split, local relabel, or
rule insertion.

`SequenceKey` is stored in the B+ tree entry. Its initial high 20 bits encode
the authored source ordinal and its low 12 bits reserve gaps for synthesized
insertions. The complete `u32`, rather than the two fields independently, is
the ordering value.

Keeping these layers separate is required because exhausting a local key gap
may relabel a small set of sequence entries. Relabeling must not invalidate:

- selector or declaration identities;
- declaration histories;
- effective keys;
- opaque wrapper identities;
- source-map ownership; or
- references held by other rule payloads.

## Compilation stores

The target ownership shape is:

```rust,ignore
struct Compilation {
    rules: DenseStore<CssRuleId, CssRuleData>,
    rule_lists: DenseStore<RuleListId, RuleListData>,
    rule_sequence: RuleSequence,
    rule_sequence_pages: RuleSequenceArena,
    rule_positions: DenseMap<CssRuleId, Option<RulePosition>>,

    declaration_blocks:
        DenseStore<DeclarationBlockId, DeclarationBlockData>,
    declarations: DenseStore<DeclarationId, DeclarationSlot>,
    declaration_ranges:
        DenseStore<DeclarationRangeId, DeclarationRange>,

    selectors: DenseStore<SelectorValueId, SelectorData>,
    effective_keys: EffectiveKeyInterner,
}
```

Rule payloads refer to child lists and declaration blocks by stable IDs:

```rust,ignore
struct CssRuleData {
    sequence_key: SequenceKey,
    parent_list: RuleListId,
    previous_sibling: Option<CssRuleId>,
    next_sibling: Option<CssRuleId>,
    payload: CssRulePayload,
    flags: CssRuleFlags,
    revision: u32,
}

enum CssRulePayload {
    Style {
        selectors: SelectorValueId,
        declarations: DeclarationBlockId,
        children: Option<RuleListId>,
    },
    Media {
        query: MediaQueryId,
        children: RuleListId,
    },
    NestedDeclarations {
        declarations: DeclarationBlockId,
    },
    // Other typed rule payloads.
}
```

A retained style rule continues to own the same `DeclarationBlockId` after
S1, S2, S3 endpoint reuse, or S4 representation changes. Only a real split
that introduces another simultaneously live output rule allocates another
rule and block identity.

## B+ tree rule sequence

### Page storage

The compilation owns one global preorder rule sequence. It is an
order-statistic B+ tree with fixed-capacity pages allocated from a
compiler-owned page arena:

```rust,ignore
struct RuleSequenceArena {
    pages: DenseStore<SequencePageId, SequencePage>,
}

enum SequencePage {
    Internal(InternalPage),
    Leaf(LeafPage),
}

struct InternalPage {
    children: ArrayVec<SequencePageId, INTERNAL_CAPACITY>,
    subtree_lens: ArrayVec<u32, INTERNAL_CAPACITY>,
    candidate_flags: CandidateFlags,
}

struct LeafPage {
    entries: ArrayVec<RuleSequenceEntry, LEAF_CAPACITY>,
    previous: Option<SequencePageId>,
    next: Option<SequencePageId>,
    candidate_flags: CandidateFlags,
}

struct RuleSequenceEntry {
    key: SequenceKey,
    rule: CssRuleId,
    candidate_edge: EdgeState,
}
```

The exact capacities are benchmark decisions. Initial candidates should keep
one page near a cache-line or small-page multiple and store only compact IDs,
keys, and flags in the sequence. Large rule payloads stay in typed dense
stores and are not moved during leaf insertion or split.

The page arena is allocation-only during one compilation and reserves from a
source-size estimate. This keeps pages densely allocated without requiring
stable Rust addresses. Page relationships use `SequencePageId`, so growing the
backing store may relocate its buffer safely.

### Rule positions

Stable rule IDs locate their current leaf entry through a dense sidecar:

```rust,ignore
struct RulePosition {
    leaf: SequencePageId,
    slot: u16,
}
```

Insertion, deletion, redistribution, or split refreshes positions only for
entries in the affected pages:

```rust,ignore
fn refresh_leaf_positions(&mut self, leaf: SequencePageId) {
    for (slot, entry) in self.leaf(leaf).entries.iter().enumerate() {
        self.rule_positions[entry.rule] = Some(RulePosition {
            leaf,
            slot: slot as u16,
        });
    }
}
```

Retiring a rule removes its sequence entry and stores `None` in the sidecar.
The dense rule payload slot remains allocated and marked retired. IDs are not
reused during the compilation.

### Basic complexity

For leaf capacity `B` and `N` sequence entries:

| Operation                         | Complexity       |
| --------------------------------- | ---------------- |
| Locate by position                | `O(log_B N)`     |
| Insert or remove                  | `O(log_B N + B)` |
| Split or redistribute one leaf    | `O(B)`           |
| Refresh affected rule positions   | `O(B)`           |
| Pop earliest dirty candidate      | `O(log_B N)`     |
| Ordered code-generation traversal | `O(N)`           |

The bounded `O(B)` movement copies compact sequence entries, not complete AST
payloads.

## Sequence-key allocation

### Initial authored keys

Authored rules start with a 12-bit gap:

```rust,ignore
const INSERTION_BITS: u32 = 12;
const INSERTION_STRIDE: u32 = 1 << INSERTION_BITS;

fn authored_key(source_ordinal: u32) -> SequenceKey {
    SequenceKey(
        source_ordinal
            .checked_mul(INSERTION_STRIDE)
            .expect("authored rule order exceeds compact sequence capacity"),
    )
}
```

For example:

```text
authored A = 0x001000
authored B = 0x002000
```

The values between them are available for synthesized entries. They do not
reserve empty `CssRuleData` slots because the B+ tree stores only existing
entries.

### Insertion between entries

The common path takes the midpoint of two neighboring labels:

```rust,ignore
fn between(left: SequenceKey, right: SequenceKey) -> Option<SequenceKey> {
    let distance = right.0.checked_sub(left.0)?;
    (distance > 1).then(|| SequenceKey(left.0 + distance / 2))
}
```

The initial 12-bit gap is not a claim that arbitrary midpoint insertion can
continue 4095 times without maintenance. A repeatedly one-sided insertion may
consume the immediate midpoint space after a small number of operations. B+
tree locality makes this a bounded relabel event rather than a global AST
move.

### Local relabel

When no integer exists between adjacent keys:

1. select their leaf and, if necessary, one neighboring leaf;
2. find the exclusive lower and upper boundary keys outside that local set;
3. redistribute the selected entries evenly in the available integer range;
4. update `CssRuleData::sequence_key` for those entries;
5. refresh their dense `RulePosition` sidecars; and
6. reclassify only edges incident to the relabeled pages.

```rust,ignore
fn relabel_entries(
    entries: &mut [RuleSequenceEntry],
    lower: SequenceKey,
    upper: SequenceKey,
) -> bool {
    let available = upper.0 - lower.0 - 1;
    if available < entries.len() as u32 {
        return false;
    }

    let step = (upper.0 - lower.0) / (entries.len() as u32 + 1);
    for (index, entry) in entries.iter_mut().enumerate() {
        entry.key = SequenceKey(lower.0 + step * (index as u32 + 1));
    }
    true
}
```

If one leaf has insufficient surrounding key space, expand to a neighboring
leaf or split and redistribute. A full global-sequence relabel is the final
compact fallback. This path is expected to be extremely rare, but it must be
explicit rather than panic or silently reorder nodes.

The initial high-20/low-12 layout supports about one million authored labels
before fallback. If a valid input exceeds that common representation, the
implementation must use a checked slow path, such as relabeling the complete
global sequence with denser initial spacing or promoting the sequence to a
wider key. Valid CSS must not wrap the compact key.

## Replacing `SemanticOrderKey` and `BinaryHeap`

An S1 or S3 candidate is an edge between two direct live siblings in the same
rule list. Those siblings are not necessarily physically adjacent in the
global preorder sequence because the left sibling's subtree may lie between
them. Store the candidate state on the left rule's global sequence entry
rather than copying an endpoint snapshot into a global heap:

```rust,ignore
bitflags::bitflags! {
    struct EdgeFlags: u8 {
        const DIRTY = 1 << 0;
        const S1 = 1 << 1;
        const S3 = 1 << 2;
    }
}

struct EdgeState {
    flags: EdgeFlags,
    revision: u32,
}
```

Every internal B+ page stores the union of candidate flags below it. To pop the
semantically earliest S3 candidate, descend through the leftmost child whose
aggregate includes `S3`, then select the first S3 rule entry in its leaf. The
entry's stable `next_sibling` supplies the right endpoint.

```rust,ignore
fn pop_first_s3_candidate(&mut self) -> Option<(CssRuleId, CssRuleId)> {
    let leaf = self.leftmost_leaf_with(CandidateFlags::S3)?;
    let slot = self.leaf(leaf).first_s3_entry()?;
    let left = self.leaf(leaf).entries[slot].rule;
    Some((left, self.rules[left].next_sibling?))
}
```

Structural edits dirty only these edges:

- the predecessor to the first edited entry;
- edges between inserted, retained, or removed entries; and
- the last edited entry to its successor.

This removes:

- variable-length `SemanticOrderKey` allocation and comparison;
- `BinaryHeap<Reverse<PartialCandidateKey>>` maintenance;
- stale heap entries created by endpoint revisions; and
- a separate hash set used only to deduplicate heap membership.

Endpoint revisions may remain useful for semantic payload changes, but the
candidate state is cleared and recomputed whenever either endpoint or their
direct-sibling link changes. It cannot remain attached to a retired pair after
local reclassification.

S2 dirty effective-key histories remain separate work items because they are
not adjacency edges. Their occurrence order reads the current sequence key or
uses stable history links maintained during the same local mutation.

## Rule-list and tree topology

All rules live in one global preorder B+ sequence so `SequenceKey` is a global
semantic order and local relabel never collides with a separately maintained
list. `RuleListId` describes structural ownership and direct-sibling links. A
rule payload that contains child rules owns another `RuleListId`; it does not
own a nested Rust vector or another independent B+ tree.

```rust,ignore
struct RuleListData {
    first_child: Option<CssRuleId>,
    last_child: Option<CssRuleId>,
    len: u32,
    parent_rule: Option<CssRuleId>,
}
```

The global sequence supplies deterministic source order; explicit sibling
links supply structural adjacency. An inserted direct sibling is placed after
the complete left subtree and before the right sibling, while its
`parent_list`, predecessor, and successor links are updated atomically. It
cannot accidentally cross a nested list, rule-list segment, or retained
barrier. Candidate classification must never infer direct adjacency from
global numeric closeness alone.

Because relabel operates on one globally ordered tree, the lower and upper
boundary keys account for nested rules and entries from every list. A local
relabel therefore preserves one total order without coordinating independent
per-list key spaces.

Opaque conditional identity uses a stable `OpaqueContextId`, not a mutable
`SequenceKey`. Local relabel therefore cannot merge histories that were
previously isolated by unsupported at-rule semantics.

## Source-ordered declaration tape

### Allocation invariant

The parser appends a declaration slot when it encounters the property in the
source. Declaration IDs increase in lexical order across the complete
stylesheet, including nested rules and later `NestedDeclarationsRule` runs.

```rust,ignore
struct DeclarationSlot {
    value: Declaration,
    source: SourceLocationId,
    flags: DeclarationFlags,
}

struct DeclarationRange32 {
    offset: u32,
    len: u32,
}
```

Parser tests must inspect declaration IDs directly. Generated output is not
sufficient evidence because serialization could hide an incorrect allocation
order.

### Stable declaration-block headers

The common block representation is exactly one range:

```rust,ignore
struct DeclarationBlockData {
    owner: CssRuleId,
    sequence: DeclarationSequence,
    effective_key: EffectiveKeyId,
    flags: DeclarationBlockFlags,
}

enum DeclarationSequence {
    Range(DeclarationRange32),
    Segmented {
        head: DeclarationRangeId,
        tail: DeclarationRangeId,
        live_len: u32,
    },
}
```

Importance, tombstones, and other booleans use packed sidecars or flags rather
than inflating each aligned declaration value.

### Constant-time adjacent concatenation

Two declaration ranges can become one range when:

```text
left.offset + left.len == right.offset
```

and every slot has compatible unique ownership. S1 then retains the right rule
and right block IDs and mutates only the right header:

```rust,ignore
fn prepend_adjacent(
    left: DeclarationRange32,
    right: &mut DeclarationRange32,
) {
    debug_assert_eq!(left.offset + left.len, right.offset);
    right.offset = left.offset;
    right.len += left.len;
}
```

There is no declaration copy, range-vector prefix copy, owner-chain walk, or
new declaration-block identity.

### Segmented fallback

A contiguous `(offset, len)` cannot represent every legal transformation. For
example:

```css
a {
  color: red;
  margin: 0;
}
b {
  color: red;
  padding: 0;
}
```

S3 may retain `margin`, synthesize shared `color`, and retain `padding`. These
three live sequences cannot all own disjoint contiguous views of the two
authored ranges.

The uncommon fallback stores an intrusive range chain:

```rust,ignore
struct DeclarationRange {
    range: DeclarationRange32,
    next: Option<DeclarationRangeId>,
}
```

Because a live declaration sequence has one semantic owner, concatenating two
segmented sequences can splice `left.tail` to `right.head` in constant time.
The retained `DeclarationBlockId` upgrades its header in place. A new shared
block is allocated only when the transformation produces an additional live
output owner.

## Stable identity through S1-S4

The deterministic survivor policy is:

| Transition                                      | Rule identity                       | Declaration-block identity           |
| ----------------------------------------------- | ----------------------------------- | ------------------------------------ |
| S1 merges equal adjacent selectors              | retain right; retire left           | retain right; retire left            |
| S2 removes declarations but rule remains        | unchanged                           | unchanged                            |
| S2 empties a rule without children              | retire original                     | retire original                      |
| S2 empties declarations but children remain     | retain original shell               | retain original empty header         |
| S3 exhausts left and reuses its representation  | mutate left into shared             | retain left                          |
| S3 retains right residual declarations          | right unchanged                     | right unchanged                      |
| S3 retains both residual endpoints              | allocate one additional shared rule | allocate one additional shared block |
| S4 changes shorthand or physical representation | unchanged                           | mutate existing header               |
| B+ leaf split or key relabel                    | unchanged                           | unchanged                            |

Reusing an authored rule ID for a synthesized selector union means the ID is a
stable storage identity, not an immutable statement that the payload remains
authored. Source-origin metadata must explicitly retain the contributing
authored spans.

## S1 simplification

S1 no longer builds `previous_merged` chains or rewrites `owner_by_block` for
every physical predecessor:

```rust,ignore
fn commit_s1(left_rule: CssRuleId, right_rule: CssRuleId) {
    let left_block = rules[left_rule].declarations();
    let right_block = rules[right_rule].declarations();

    declaration_blocks[right_block]
        .sequence
        .prepend(declaration_blocks[left_block].sequence);

    retire_block(left_block);
    rule_sequence.remove(left_rule);
    retire_rule(left_rule);
}
```

The right rule and block remain stable. Declaration history occurrences refer
to the retained block identity or are redirected once as part of the atomic S1
transition. They do not require replaying an ever-growing declaration chain.

## Terminal commit and code generation

The B+ sequence is already the final physical rule order. Code generation can
walk linked leaves and fetch stable rule payloads:

```rust,ignore
for leaf in rule_sequence.leaves() {
    for entry in &leaf.entries {
        emit_rule(&rules[entry.rule]);
    }
}
```

A mandatory S5 copy of every surviving rule back into one `Vec<CssRule>` is
therefore unnecessary. The terminal phase instead:

1. verifies that S1-S4 work and candidate aggregates are empty;
2. removes merge-only edge and history sidecars;
3. optionally merges underfull neighboring leaves;
4. optionally compacts declaration ranges when fragmentation exceeds a
   measured threshold; and
5. freezes the compilation for immutable code generation.

Declaration compaction remains independent from rule-sequence compaction. A
token-heavy declaration payload may be more expensive to copy than a compact
rule entry, so compaction must be benchmark-driven rather than unconditional.

## Memory behavior

The B+ tree eliminates large suffix moves, but it is not as physically compact
as one flat vector. Preserve locality through these rules:

- use fixed-capacity pages with compact entry arrays;
- allocate pages from one pre-sized `RuleSequenceArena`;
- keep large typed rule payloads out of leaf entries;
- link leaves for sequential parse/minify/codegen traversal;
- avoid per-entry heap allocation;
- keep parent and child references as dense IDs;
- estimate initial page and declaration capacity from source byte length; and
- benchmark leaf capacities on Bootstrap, Tailwind, deeply nested input, and
  candidate-heavy generated input.

Deletion may leave underfull pages because an allocation-only arena cannot
return individual pages. Reuse retired page IDs through an internal free list
only if the compilation permits safe page reinitialization; otherwise merge
pages opportunistically and retain unused capacity until the compilation is
dropped.

## Required invariants

- `CssRuleId` remains stable until its rule is retired.
- `DeclarationBlockId` remains stable until its block is retired.
- `SequenceKey` may change only through a sequence-owned relabel operation.
- B+ leaf order and `SequenceKey` order are identical.
- `rule_positions[rule]` identifies the leaf entry containing every live rule.
- Relabel never changes selector, declaration, effective-key, source, or opaque
  context identity.
- Each live declaration slot has exactly one semantic owner.
- A contiguous block range never absorbs a live foreign declaration.
- Direct adjacency never crosses a `RuleListId`, segment, nested subtree, or
  retained barrier.
- Candidate aggregate flags exactly equal the union of live candidates below
  each internal page.
- Synthesized allocation time does not override semantic insertion position.
- Code generation emits each live rule and declaration exactly once.

## Implementation sequence

### Phase 1: sequence prototype

1. Implement a generic arena-backed B+ sequence of compact stable IDs.
2. Add leaf links, subtree lengths, rule-position sidecars, insert, remove,
   split, and redistribution.
3. Implement spaced `SequenceKey` allocation and local relabel.
4. Test adversarial one-sided insertion and boundary insertion.
5. Benchmark against `Vec<CssRuleId>` and the current live sibling graph.

### Phase 2: stable rule/list stores

1. Move rule payloads into typed dense stores behind `CssRuleId`.
2. Give every child rule list a stable `RuleListId`, first/last child, and
   direct-sibling links.
3. Replace nested rule vectors with rule-list IDs.
4. Route visitors and code generation through leaf iteration.
5. Preserve source spans and lossless output before enabling structural edits.

### Phase 3: declaration tape

1. Append declarations globally at the parser encounter point.
2. Add stable block headers with the common range representation.
3. Add source-order allocation tests for nested and segmented declarations.
4. Add the segmented sequence fallback without changing common block size.
5. Port declaration code generation to range and range-chain iteration.

### Phase 4: scheduler integration

1. Attach S1/S3 edge state to B+ leaf entries.
2. Add internal aggregate flags and leftmost dirty-candidate descent.
3. Remove `SemanticOrderKey`, `PartialCandidateQueue`, and its `BinaryHeap`.
4. Mutate retained rules and blocks in place according to the survivor table.
5. Remove `previous_merged` and whole-chain `owner_by_block` maintenance.
6. Keep S2 history work independent, but order occurrences through stable rule
   positions or incrementally maintained history links.

### Phase 5: terminal freeze

1. Remove mandatory full-rule reification.
2. Freeze B+ sequences for immutable code generation.
3. Add optional page and declaration compaction thresholds.
4. Compare total parse, minify, codegen, and drop time with the current flat
   vector and nested AST implementations.

## Verification matrix

### Identity

- S1 retains the right `CssRuleId` and `DeclarationBlockId`.
- S2 deletion does not replace the owning IDs.
- exhausted-left S3 retains the left IDs while changing selectors and the
  effective key;
- S3 with both residuals allocates exactly one additional rule and block;
- leaf split and relabel preserve every stable rule and block ID; and
- retired IDs are never reused during one compilation.

### Ordering

- authored keys increase in lexical source order;
- insertion at the beginning, middle, and end preserves output order;
- repeated one-sided insertion triggers local relabel without output changes;
- nested lists remain isolated;
- overlapping S3 candidates commit in the same deterministic semantic order;
- relabel does not change which candidate is selected first; and
- code generation follows leaf order exactly.

### Declaration storage

- declaration IDs match lexical property order across the entire stylesheet;
- two adjacent authored blocks concatenate by updating one range header;
- a tombstone-only gap is consumed only after proof;
- a live foreign gap forces segmented representation;
- partial factoring never creates overlapping live ownership;
- important and normal phases remain distinct; and
- unparsed declarations remain lossless and outside unsafe movement proofs.

### Complexity instrumentation

Count, rather than infer from wall time:

- sequence entries moved per insertion;
- leaves split, redistributed, and relabeled;
- maximum and total relabeled entries;
- rule-position sidecars refreshed;
- candidate-tree descent steps;
- stale candidate records retained outside the tree;
- declaration ranges copied or chained by S1; and
- complete AST or declaration compaction passes.

For `N` repeated insertions, no operation may move `O(N)` complete rule
payloads. For `N` adjacent same-selector rules, S1 declaration concatenation
must not copy an accumulated `1 + 2 + ... + N` range prefix.

## Benchmark requirements

- Bootstrap and Tailwind parse/minify/codegen pipelines;
- no-op minification over many unique rules;
- repeated exhausted-left S3 factoring;
- repeated S3 factoring where both endpoints retain residuals;
- long runs of identical selectors exercising S1;
- adversarial midpoint exhaustion in one authored gap;
- many small nested rule lists;
- one very large flat rule list;
- declaration-heavy rules with no structural changes; and
- code generation directly from linked leaves.

Measure memory, page occupancy, branch misses, cache misses, bytes copied,
allocation count, and complete compilation lifetime. The B+ design should be
accepted only if bounded insertion and scheduler simplification outweigh the
extra indirection on ordinary parse and code-generation traversal.

## Open benchmark decisions

- leaf and internal page capacities;
- whether pages use one enum store or separate leaf/internal stores;
- source-byte heuristics for initial page and declaration capacity;
- when to redistribute instead of split;
- how many neighboring leaves local relabel may inspect;
- the compact-key overflow representation;
- whether S2 histories use sequence keys or intrusive history links; and
- declaration and page fragmentation thresholds for optional compaction.
