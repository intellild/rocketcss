# Minification directly over the Radix AST

## Physical model

The semantic S1-S5 documents still define which declaration effects and
selector transforms are valid. The Radix AST removes several physical adapter
layers previously needed by the recursive AST:

```text
RuleStore: RadixIndexArena<CssRule> + direct topology
  stable RuleId, semantic order, direct sibling edges

DeclarationBlockStore: RadixIndexArena<DeclarationBlock>
  stable DeclarationBlockId + AST-owned EffectiveKeyId

Declaration storage
  authored source-order ranges + Local4/overflow transformed lists

Nano sidecars
  only liveness, summaries, histories, revisions, and queue membership
```

Nano does not build a `Vec<DeclarationBlockEntry>`, reconstruct EffectiveKeys,
or assign a separate semantic-order key.

## Initialization

```text
Parser/local minify
  AST stores already contain final owner, topology, and EffectiveKey fields
       ↓
CrossRuleState::initialize
  iterate DeclarationBlockStore once in Radix order
  lazily create summaries and repeated-key histories
  classify direct rule edges from topology
       ↓
unified scheduler
```

Initialization is a store scan, not an AST discovery walk. It neither allocates
a second flat block list nor hashes selector/context structures again.

If block-local minification already visits every block, it may seed summaries
and histories incrementally. That is an optional fusion optimization; the AST
remains authoritative either way.

## Identity and ordering

Candidate types use AST IDs directly:

```rust,ignore
struct SameSelectorCandidate(DeclarationBlockId, DeclarationBlockId);
struct PartialMergeCandidate(DeclarationBlockId, DeclarationBlockId);
struct DeclarationOverrideTask(EffectiveKeyId);
```

Within a rule list, endpoint `RuleId`/`DeclarationBlockId` order is the
deterministic source-order priority. Synthesized Radix siblings are inserted at
their final local semantic position, so a separate `SemanticOrderKey`,
`SemanticInsertionPosition`, `BTreeMap`, or source-order `BinaryHeap` is not
required.

Candidate queues may still use a compact ordered data structure when multiple
ready tasks exist, but its key is the fixed-width AST ID. It is queue policy,
not another semantic identity.

Direct adjacency always revalidates rule topology. Numeric closeness alone is
not proof across nested subtrees or rule-list barriers.

## Unified scheduler

The scheduler keeps the existing priority:

```rust,ignore
while state.has_work() {
    if let Some(candidate) = state.s1.pop_first() {
        state.commit_s1(candidate, ast);
        continue;
    }
    if let Some(history) = state.s2.pop_dirty_history() {
        state.commit_s2(history, ast);
        continue;
    }
    if let Some(candidate) = state.s3.pop_first() {
        state.commit_s3(candidate, ast);
    }
}
```

Each candidate captures endpoint revisions. A pop rejects stale work if IDs no
longer resolve, endpoints are retired, topology changed, revisions differ, or
current EffectiveKey equality no longer matches the stage.

## S1

S1 reads equal selector/effective-key facts from the two AST blocks and commits
the chosen declaration representation directly:

- consecutive authored declaration ranges may coalesce;
- small synthesized results may use `Local4`;
- nonconsecutive or larger results use a complete overflow sequence; and
- the retired rule/block is unlinked from live topology but retains its ID
  until cleanup.

The local topology transaction reclassifies newly exposed edges and dirties
affected S2 histories. It does not rebuild all blocks or restart S1/S2.

## S2

S2 histories are keyed by `(EffectiveKeyId, PropertyId, importance phase)` and
ordered by `DeclarationBlockId`. History records hold intrusive predecessor and
successor block IDs plus revisions.

Declaration deletion or effect replacement updates the AST block immediately
when the lossless representation is already proven. S4 remains responsible for
choosing between equivalent declaration encodings when the choice is not yet
known; that pending representation state is attached to the block, not stored
in a second flat AST.

An empty block causes local owner/topology updates and queue insertion. It does
not trigger `walk_declaration_blocks`.

## S3

S3 commits a shared rule at its final Radix position:

```text
validate adjacent endpoint IDs and declaration movement
       ↓
intern selector union and EffectiveKeyId
       ↓
choose sibling key between local semantic neighbors
       ↓
insert synthesized CssRule and DeclarationBlock into Radix stores
       ↓
update rule topology and residual declaration lists
       ↓
insert new block into its EffectiveKey history by block ID
       ↓
enqueue affected S1/S2/S3 edges
```

The insertion does not move later primary values and does not invalidate AST
identity. Therefore S3 does not need a logical-only node, a variable semantic
position, or a later global rule-list reification pass.

If local sibling labels require relabeling, the store returns an exact remap and
the transaction repairs the few affected topology/history/queue references.

## S4 and S5 under this storage model

S4 remains a semantic representation planner where effect-level transforms
need to choose a lossless CSS encoding. Its result is attached to the affected
AST node or committed immediately when complete.

S5 is no longer a mandatory fresh-store rebuild. It is a terminal commit and
cleanup boundary:

1. assert S1-S4 queues and history generations are stable;
2. finish any deferred declaration representation chosen by S4;
3. unlink or tombstone retired nodes not already finalized;
4. discard histories, summaries, revisions, and queue state; and
5. optionally compact declaration overflow/tombstones when benchmarks justify
   the copy.

S5 makes no new semantic decision and does not restore order—the Radix stores
already carry final semantic order.

## Code generation

Codegen follows root rule-list topology and resolves IDs through the AST stores.
The underlying store iterator processes primary slices and inserted Radix
segments. No merge-only `previous_merged` chain or reified copy is required.

## State removed from Nano

The target deletes:

- `DeclarationBlockDiscovery` and production `walk_declaration_blocks`;
- `DeclarationBlockEntry`/`DeclarationBlockEntryId` flat ownership;
- Nano-owned EffectiveKey paths and interner reconstruction;
- `owner_by_block` reconstructed from borrowed AST references;
- `SemanticOrderKey` and `SemanticSourceOrderKey`;
- ordered maps keyed by variable-length semantic positions;
- global scheduler restart after an S3 physical edit;
- mutation plans needed only because the recursive AST lacked stable insertion
  IDs; and
- mandatory S5 allocation/copy of complete rule and declaration stores.

Nano retains:

- declaration IR/effect summaries;
- lazy EffectiveKey histories and property Bloom filters;
- liveness and endpoint revisions;
- S1/S2/S3 queues and rejection counters;
- exact movement and selector compatibility proofs; and
- S4 lossless representation decisions.

## Correctness invariants

- Every live block has one AST owner and one current EffectiveKey.
- Candidate order uses fixed-width AST ID order and is deterministic.
- Direct edges are validated through topology.
- A synthesized node has its final semantic Radix position before it is
  enqueued.
- S2 histories are ordered by current block IDs and exact context equality.
- Declaration effects and fallback order remain losslessly representable.
- Unsupported at-rule semantics remain opaque and unequal across occurrences.
- Terminal cleanup does not create new S1-S4 work.

## Performance gates

- No production recursive block discovery or EffectiveKey reconstruction.
- One initialization store scan at most; fusion with local minify is optional.
- S3 work grows with changed local edges, not stylesheet size times commit
  count.
- Stylesheets with no structural transforms retain primary-Vec parse and
  traversal performance.
- Track parse, minify, codegen, peak memory, sibling-group count, local relabel
  count, and semantic-iterator time separately.
