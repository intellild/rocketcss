# Minification over mutable B+ stores

## Semantic IR versus physical storage

The S1-S5 documents define semantic declaration effects, histories, live
adjacency, and candidates. The flat AST stores the physical facts required by
those stages directly:

```text
global rule B+ store + topology
  direct siblings, subtree boundaries, and final rule order

global declaration-block B+ store
  source-order block iteration and AST-owned EffectiveKeyId

global declaration B+ store
  compact start_id + len ranges and local sequence edits

store-owned flags and links
  liveness, revisions, histories, and dirty candidate state
```

The minifier does not recursively rebuild a `Vec<DeclarationBlockEntry>` and
does not perform a full `walk_declaration_blocks` pass to rediscover selector or
at-rule context. The parser and selector-local normalization have already
stored the final context identity on each declaration block.

## Initialization

The target initialization flow is:

```text
Parser
  append rules, blocks, and declarations in lexical order
  intern ContextPathId while entering wrappers
  store a context seed on each DeclarationBlock
       ↓
rule-local minify
  canonicalize selector values
  finalize DeclarationBlock.effective_key
       ↓
cross-rule minify
  iterate B+ stores directly
  create histories only when a key/property repeats
  classify direct sibling edges locally
```

This removes the current full-tree collection and EffectiveKey reconstruction
cost. A synthesized or structurally moved rule must update its owned block keys
at the mutation point instead of relying on another global scan.

## Store-owned scheduling

S1 and S3 candidates are edges between direct live sibling rules. Candidate
flags live on the rule sequence entry for the left endpoint, and B+ internal
pages aggregate whether any descendant contains an S1 or S3 edge.

```rust,ignore
bitflags::bitflags! {
    struct CandidateFlags: u8 {
        const DIRTY = 1 << 0;
        const S1 = 1 << 1;
        const S3 = 1 << 2;
    }
}
```

Selecting the semantically earliest candidate descends through the leftmost
page whose aggregate contains the requested flag. This replaces a separate
`SemanticOrderKey` and `BinaryHeap` while preserving source-order scheduling.

Structural mutation dirties only the predecessor edge, edges inside the edited
range, and the successor edge. S4 may create new work, so store mutation APIs
must enqueue or mark those local edges before returning.

S2 histories are not adjacency edges. They remain source-ordered property
chains keyed by `(EffectiveKeyId, PropertyId, importance phase)`, but links and
summary state should live with the declaration/block stores instead of in a
second reconstructed block IR.

## S1: same-selector coalescing

S1 compares direct live siblings, exact selector/emission identity, and the
AST-owned `EffectiveKeyId`.

For consecutive compact declaration ranges:

```rust,ignore
fn merge_compact_blocks(left: DeclarationBlockId, right: DeclarationBlockId) {
    debug_assert!(declarations.are_consecutive(
        declaration_blocks[left].declarations.compact_range(),
        declaration_blocks[right].declarations.compact_range(),
    ));

    let merged_start = declaration_blocks[left].declarations.start();
    let merged_len = declaration_blocks[left].declarations.len()
        + declaration_blocks[right].declarations.len();

    let survivor = choose_block_survivor(left, right, merged_start);
    declaration_blocks[survivor]
        .declarations
        .set_compact(merged_start, merged_len);
    redirect_rule_to_block(right_rule, survivor);
    retire_other_block(left, right, survivor);
}
```

The retained `CssRuleId` and retained `DeclarationBlockId` do not have to come
from the same authored endpoint. Prefer retaining the block header identified
by the merged start's block hint when that avoids a hint redirect. If another
survivor is required, update `DeclarationBlockHintIndex` atomically.

If compact ranges are not consecutive, S1 either splices their B+ ranges into
semantic order or fills the survivor's complete arena overflow vector. It never
widens `len` across a live foreign declaration.

`previous_merged`, accumulated owner chains, and prefix copying are absent from
the target representation.

## S2: declaration-effect pruning

Each declaration block already supplies:

- `EffectiveKeyId`;
- its direct compact or overflow declaration iterator;
- revision and liveness flags; and
- source order through its B+ store position.

S2 therefore builds a history entry lazily only when an effective
key/property/phase tuple repeats. An exact duplicate tombstones or removes the
old declaration according to the current mutation epoch.

Deleting from a compact block follows these rules:

- deleting the first entry advances `start` and decrements `len`;
- deleting a middle entry either removes it from the B+ range immediately or
  leaves a store-owned tombstone until the local commit;
- an empty block has `start = None` and `len = 0`; and
- an overflow block mutates its arena vector directly.

Histories that retain `DeclarationId` participate in every local relabel remap.
An alternative intrusive history link stored on the declaration entry moves
with that entry and is preferred if it avoids remap work.

## S3: selector partial factoring

S3 compares declaration IR derived from the two endpoint iterators. Property
Bloom filters, fingerprints, and live counts may be cached on the block header
and invalidated by its revision; exact declaration equality remains the final
proof.

The commit path edits B+ ranges directly:

```text
choose shared declarations and residuals
       ↓
splice or retain declaration entries in semantic order
       ↓
mutate exhausted endpoint block in place when possible
       ↓
allocate only genuinely additional rule/block output
       ↓
compute EffectiveKeyId for synthesized selector union
       ↓
repair BlockHint aliases and dirty adjacent rule edges
```

If both residual endpoints survive, S3 allocates one additional shared rule and
block. If one endpoint is exhausted, it reuses that endpoint's rule/block
storage where compatible. A compact result must be one consecutive global B+
range; otherwise it uses an explicit splice or the overflow fallback.

Allocation order never substitutes for output order. The rule B+ store inserts
the synthesized rule at its semantic position immediately, so no separate
variable-length `SemanticOrderKey` is required.

## S4: representation planning

S4 changes shorthand/longhand representation, declaration spelling, and other
physical choices through the same block mutation API. It may:

- replace an arena-boxed declaration value without changing its ID;
- remove or insert a declaration entry;
- fill a compact list's lazy overflow vector; or
- create new S1-S3 work by changing an endpoint revision.

Every structural or declaration mutation distinguishes its change class:

```rust,ignore
enum ChangeKind {
    None,
    Declaration,
    Structural,
}
```

Declaration-only changes invalidate local summaries and S2 history state.
Structural changes additionally reclassify affected sibling edges. Neither
kind implies an unconditional whole-stylesheet second round.

## Declaration ID relabel transaction

Insertion normally selects an unused `DeclarationId` between the neighboring
keys. When the interval is exhausted:

```text
Declaration B+ store
  relabel smallest useful local interval
       ↓
emit SmallVec<DeclarationIdRemap>
       ↓
decode BlockHint values
       ↓
inspect only registered candidate blocks
       ↓
repair exact matching compact starts
       ↓
repair remaining history/dirty references
       ↓
commit B+ keys and aggregate state atomically
```

The special ID encoding exists to bound this repair work. It does not permit
skipping exact comparison, and it does not make a stale ID acceptable.

An input that exceeds compact capacity upgrades the affected block to its
arena vector. It never panics merely because valid CSS contains many
declarations.

## Terminal phase

The mutable B+ stores already represent final semantic order. S5 is therefore
a freeze/cleanup phase rather than a mandatory complete AST rebuild:

1. verify that S1-S4 dirty work and candidate aggregates are empty;
2. remove merge-only history and candidate state;
3. optionally merge underfull neighboring B+ pages;
4. optionally compact overflow or fragmented ranges when a measured threshold
   justifies it; and
5. freeze stores for immutable code generation.

Code generation walks linked B+ leaves for compact sequences and dispatches to
the block's arena vector for overflow sequences. It does not follow
`previous_merged`, interpret a rewrite plan, or rerun cross-rule semantics.

## Scheduling epochs

S3 and S4 may expose more local work, but the scheduler remains incremental:

```text
AST-owned effective keys and B+ sequences
  -> pop earliest dirty S1/S2/S3/S4 work
  -> mutate one local rule/block/declaration range
  -> mark directly affected work
  -> fixed point
  -> terminal freeze
```

There is no rule that restarts by walking every rule or declaration block after
each structural change.

## Removed transitional state

The target representation removes or avoids:

- recursive `walk_declaration_blocks` collection;
- `Vec<DeclarationBlockEntry>` source-order reconstruction;
- minify-time `EffectiveKey` path rebuilding;
- declaration-block owner reconstruction;
- `SemanticOrderKey` and the S3 `BinaryHeap`;
- `previous_merged` chains;
- mandatory full AST reification; and
- duplicate ordering structures whose only purpose is to mirror the AST.

The following semantic state remains necessary, but belongs in store-owned
compact fields or sidecars:

- liveness and revisions;
- S1/S3 edge candidate flags and page aggregates;
- S2 property-history links;
- property Bloom/fingerprint/live-count summaries;
- declaration ID remap participants; and
- overflow representation state.

## Required invariants

- `DeclarationBlock.effective_key` is valid for its current selector and exact
  context.
- Compact blocks own one consecutive B+ range and overflow blocks own one
  complete ordered arena vector.
- B+ page movement alone never changes declaration IDs.
- Every label relabel repairs block starts and remaining ID references in one
  transaction.
- Direct sibling checks use topology and cannot cross a nested subtree or
  retained barrier.
- Candidate selection preserves deterministic semantic source order.
- Declaration-only changes do not force a structural global rescan.
- Synthesized allocation time does not determine output position.
- Unparsed declarations remain lossless and outside unsafe typed-value proofs.
- Code generation emits every live rule and declaration exactly once.

## Verification

Tests must cover:

- no `walk_declaration_blocks` prepass in the target minify pipeline;
- parse/local-minify construction and invalidation of `EffectiveKeyId`;
- S1 compact adjacency and nonadjacent overflow/splice fallback;
- S2 first/middle/last deletion in compact and overflow blocks;
- S3 exhausted endpoint reuse and both-residual insertion;
- S4-generated candidates returning to the local scheduler;
- local declaration relabel repairing only hinted candidate blocks;
- deterministic candidate order after B+ insertion and page split; and
- output equivalence before and after optional terminal compaction.
