# Declaration ID encoding

## Status

Proposed design.

This document specifies the compact `DeclarationId` used by the mutable global
declaration `BTreeDenseStore`. The encoding is an order-maintenance label and a
reverse lookup hint. It is not a physical array offset and it is not the CSS
property kind represented by `PropertyId`.

The common representation lets a `DeclarationBlock` store only the first
declaration ID and the number of consecutive B+ sequence entries. An
arena-allocated vector is the correctness fallback for inputs or transforms
that exceed the compact encoding.

## Required distinction

The following types have different meanings:

```rust,ignore
/// Stable identity of a declaration-block header.
struct DeclarationBlockId(NonMaxU32);

/// Position label and block lookup hint for one declaration occurrence.
struct DeclarationId(NonMaxU32);

/// High-bit reverse lookup hint. Its value is limited to 20 bits.
struct BlockHint(u32);

/// Low-bit label inside one hinted declaration region.
struct LocalLabel(u16);

/// Semantic CSS property identity, such as `color` or `margin`.
enum PropertyId {
    // Generated known properties and custom-property fallback.
}
```

`DeclarationId` may change during a local order-label relabel. A
`DeclarationBlockId` does not change until its header is retired.

## Common 20/12 layout

The initial candidate is:

```text
31                         12 11                         0
+----------------------------+---------------------------+
|       BlockHint: 20        |       LocalLabel: 12      |
+----------------------------+---------------------------+
```

```rust,ignore
const LOCAL_BITS: u32 = 12;
const LOCAL_MASK: u32 = (1 << LOCAL_BITS) - 1;

impl DeclarationId {
    fn from_parts(
        block_hint: BlockHint,
        local: LocalLabel,
    ) -> Option<Self> {
        if block_hint.0 >= 1 << 20 || u32::from(local.0) >= 1 << LOCAL_BITS {
            return None;
        }
        Self::new((block_hint.0 << LOCAL_BITS) | u32::from(local.0))
    }

    fn block_hint(self) -> BlockHint {
        BlockHint(self.get() >> LOCAL_BITS)
    }

    fn local_label(self) -> LocalLabel {
        LocalLabel((self.get() & LOCAL_MASK) as u16)
    }
}
```

This gives approximately one million compact block-hint values and 4096 local
labels per hint. Representative stylesheets are expected to have fewer than
60,000 declaration blocks and fewer than 100 declarations in one block. Those
numbers are sizing evidence, not validity limits.

## `BlockHint` is a reverse index

The high bits identify the small set of `DeclarationBlock` headers that may
need inspection when IDs with that hint are relabeled. They must not require a
scan over every block.

```rust,ignore
struct DeclarationBlockHintIndex<'ast> {
    // A direct compact lookup table; an absent hint stores the NonMax niche.
    primary_by_hint: Box<[Option<DeclarationBlockId>]>,
    // Allocated only when a split gives one hint multiple live candidates.
    aliases: FxHashMap<
        BlockHint,
        ArenaVec<'ast, DeclarationBlockId>,
    >,
}
```

The common lookup is one indexed load and has no per-hint allocation. With 20
hint bits and a niche-packed optional `u32` ID, the complete primary table is
approximately 4 MiB. Benchmark it against a two-level paged table; do not
replace it with a standard-library randomized hash map in the compiler hot
path. A split or other rare transform may make multiple live block starts share
one historical hint; only then is an alias vector allocated. Merging or
retiring a block removes or redirects that candidate.

The hint is not authoritative ownership. Exact proof is always:

```rust,ignore
declaration_blocks[candidate].declarations.start() == old_id
```

This distinction is necessary because S1 or S3 can change which live block
owns a declaration sequence without immediately rewriting every declaration
ID.

## Initial label spacing

Authored block hints and local labels are assigned in lexical source order with
gaps. If a block contains `count` declarations, its local labels can be spread
over the 12-bit space:

```rust,ignore
fn initial_local_label(index: u32, count: u32) -> Option<LocalLabel> {
    let step = (1 << LOCAL_BITS) / (count + 1);
    (step != 0).then(|| LocalLabel((step * (index + 1)) as u16))
}
```

For 100 declarations, the initial step is about 40. A block-hint allocator can
likewise use the observed or estimated block count to leave gaps between
authored blocks. The final heuristic must be selected with parse and minify
benchmarks; correctness must not depend on the estimate.

The parser may assign provisional IDs while reading a block and finalize its
spaced IDs when the contiguous declaration run closes. This is still parser
work and does not require a later recursive `walk_declaration_blocks` pass.

## Sequence semantics

A compact declaration block is:

```rust,ignore
struct CompactDeclarationList {
    start: Option<DeclarationId>,
    len: u32,
}
```

`start + len` is sequence notation, not integer arithmetic. Iteration locates
`start` in the global B+ store and consumes `len` successive live entries:

```rust,ignore
block
    .start
    .into_iter()
    .flat_map(|start| declarations.iter_from(start).take(block.len))
```

IDs between adjacent declarations may contain large numeric gaps. Physical B+
page splits and entry movement do not change IDs. Only exhaustion of the
numeric interval between semantic neighbors requires relabeling.

## Common insertion

For neighboring IDs `left` and `right`, the common insertion chooses their
midpoint:

```rust,ignore
fn between(left: DeclarationId, right: DeclarationId) -> Option<DeclarationId> {
    let distance = right.get().checked_sub(left.get())?;
    (distance > 1)
        .then(|| left.get() + distance / 2)
        .and_then(DeclarationId::new)
}
```

Insertion inside a compact block normally changes only `len`. Insertion before
its first declaration also changes `start`:

```rust,ignore
fn commit_insert(block: &mut DeclarationBlock, index: u32, new: DeclarationId) {
    if index == 0 {
        block_hint_index.move_candidate(
            block.id,
            block.declarations.start().map(DeclarationId::block_hint),
            new.block_hint(),
        );
        block.declarations.set_start(new);
    }
    block.declarations.increment_len();
}
```

No later declaration block is updated merely because a B+ page inserted or
split.

## Local relabel and block repair

When no numeric value exists between two neighbors, the declaration store
relabels the smallest useful local interval and returns the exact mapping:

```rust,ignore
struct DeclarationIdRemap {
    old: DeclarationId,
    new: DeclarationId,
}
```

Repair groups the remaps by the decoded old and new block hints, then inspects
only the candidates registered for those hints:

```rust,ignore
fn repair_block_starts(remaps: &[DeclarationIdRemap]) {
    for remap in remaps {
        for block in block_hint_index.candidates(remap.old.block_hint()) {
            if declaration_blocks[block].declarations.start() == Some(remap.old) {
                block_hint_index.move_candidate(
                    block,
                    Some(remap.old.block_hint()),
                    remap.new.block_hint(),
                );
                declaration_blocks[block]
                    .declarations
                    .set_start(remap.new);
            }
        }
    }
}
```

Insertion or deletion that changes a compact block's first ID performs the same
hint-index move even when no relabel occurred. The same remap transaction
updates any remaining declaration-ID references, such as an S2 history endpoint
or dirty work item. Such consumers must not silently retain an old ID. Prefer
storing merge state in the declaration store so the relabel operation owns as
many repairs as possible.

Relabel is local to a block-sized range when possible. With the expected block
size, changing roughly 100 compact keys is cheaper and more predictable than
updating the starts of every later block after an array insertion.

## Overflow representation

Valid CSS is not limited to 4096 declarations per block or one million blocks.
Repeated adversarial insertion can also exhaust local gaps even when few
declarations remain live. These cases must not panic.

`DeclarationBlock` therefore has a lazy arena-allocated vector field:

```rust,ignore
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

`overflow = None` selects the common global B+ range. A nonempty overflow
vector contains the complete declaration sequence for that block, not only a
suffix; `start` is cleared and `len` mirrors the vector length. A complete
upgrade keeps arbitrary middle insertion, deletion, iteration, and
serialization unambiguous. Moving an arena box moves only its pointer; the
declaration value retains its arena address.

Upgrade atomically removes the block from the compact hint index and either
repairs or invalidates declaration-ID histories for that block. Later minify
work addresses overflow entries through the block and vector position rather
than pretending that they still have compact global IDs.

Upgrade is permitted when:

- the compact block-hint or local-label space cannot represent the sequence;
- repeated relabel exceeds a measured threshold;
- a transformation cannot cheaply preserve one contiguous global B+ range; or
- hint aliases cease to be a small exceptional set.

The common path reads two integers and observes a null overflow pointer. A
compact `OverflowDeclarationListId` may replace the pointer if layout
benchmarks show that the optional field inflates every block excessively.

## Merge and split behavior

Two compact block ranges that are consecutive in the global declaration B+
sequence can merge by retaining the first start and adding their lengths. The
retained style rule may point at whichever stable block header the survivor
policy selects; the block-hint index is updated atomically.

Splitting a compact range creates another block header whose start is the first
ID in the new range. If its decoded hint still points to the original block,
the new block is registered as a rare alias for that hint. A later local
relabel may give the split range a dedicated hint and remove the alias.

Nonconsecutive semantic sequences must not be described as one `start + len`
range. The minifier must first splice them into consecutive B+ order or fill
the affected block's overflow vector.

## Invariants

- B+ sequence order and numeric `DeclarationId` order are identical for
  compact declarations.
- A compact block owns exactly `len` successive live entries beginning at
  `start`.
- `BlockHint` narrows lookup but never proves ownership.
- Page split or redistribution does not change declaration IDs.
- Every ID relabel is returned as an exact remap and committed atomically with
  all remaining references.
- Overflow blocks preserve complete declaration order and lossless values.
- No valid CSS input triggers a panic because the compact encoding is full.

## Required tests

- parser-assigned declaration IDs increase in lexical source order;
- local labels contain gaps for representative blocks;
- middle insertion with an available gap changes no existing ID;
- front insertion changes only the owning block start;
- gap exhaustion relabels locally and repairs the correct block starts;
- one hint with split aliases repairs every exact matching start;
- S1 merge updates hint candidates without scanning all blocks;
- compact-to-overflow upgrade preserves declaration and source-map order;
- more than 4096 declarations in one valid block does not panic; and
- B+ page split without label exhaustion changes no declaration ID.
