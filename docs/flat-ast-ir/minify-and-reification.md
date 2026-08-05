# Minification directly over the Radix AST

## Physical model

Nano operates on the authoritative AST rather than rebuilding a second rule
topology:

```text
RuleStore: RadixIndexArena<RuleRecord>
  stable RuleId, lexical preorder, AST-private subtree spans

DeclarationBlockStore: RadixIndexArena<DeclarationBlock>
  stable DeclarationBlockId + owner + AST-owned EffectiveKeyId

DeclarationStore: RadixIndexArena<DeclarationRecord>
  primary parse order + stable synthesized siblings
  each block owns one RadixRange

Nano sidecars
  declaration IR, summaries, histories, revisions, and queue membership
```

Nano may retain transform-specific graphs, but it does not own parents,
children, sibling links, or source-order links.

AST stores `parent` plus the preorder subtree span. A parent-list traversal
produces direct edges; Nano retains that edge context only for the duration of
the transform and does not store AST sibling links.

## AST boundary

Nano asks the AST semantic questions:

- iterate all rule IDs in lexical order;
- test or iterate live nested rules;
- iterate opaque direct-rule edges for a root or nested parent list;
- insert a synthesized direct sibling; and
- merge or retire rules through validated transactions.

It does not read `nested_rule_count`, call Radix `advance_id`, or repair ID
remaps. Those operations remain inside the AST crate.

## Initialization

```text
Parser / local minify
  AST already owns parents, source order, declarations, and EffectiveKeys
       ↓
CrossRuleState::from_stylesheet
  scan declaration blocks in deterministic AST order
  create summaries and repeated-key histories
  consume AST root_rule_edges / nested_rule_edges
       ↓
unified scheduler
```

Initialization is a store scan, not topology reconstruction.

## Identity and ordering

Candidates use fixed-width AST IDs directly. A candidate is valid only while
its endpoints still resolve, remain live direct siblings under the same parent,
retain the captured revisions, and satisfy the stage's current EffectiveKey and
selector proofs.

Numeric ID closeness is not adjacency. A rule's descendants and locally
inserted Radix records may lie between two direct siblings in physical order;
the AST derives and validates the relationship.

## Scheduler

The scheduler keeps the semantic stage priority:

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

### S1

S1 validates the captured direct edge, commits the chosen declaration
representation into the retained block, and retires the other rule through the
AST. The merge transaction returns `RuleMutationDelta`; Nano classifies those
new edges without an adjacency query or a full rule-tree rebuild.

### S2

S2 histories are keyed by effective context, property identity, and importance
phase. Declaration deletion and effect replacement update the owning AST block.
Affected owners are grouped by parent; each direct-rule list is traversed once
to capture rule contexts. Empty rules are retired as a batch, and Nano filters
their mutation deltas after every retirement so only final live edges publish.

### S3

S3 creates shared syntax through AST transactions:

```text
validate semantic endpoints and declaration movement
       ↓
intern selector union and EffectiveKeyId
       ↓
AST insert_rule_after
  consume DirectRuleEdge context
  locate preceding subtree tail
  insert at final Radix position
  repair local ID remaps
  update ancestor subtree spans
       ↓
AST insert declaration block and bind owner
       ↓
consume RuleMutationDelta.new_edges
       ↓
publish histories and reclassified candidates
```

The synthesized rule has its final stable ID before it is published to queues.

## Retirement and tombstones

Retired rules and blocks remain as physical tombstones so IDs and subtree spans
stay stable. AST semantic iterators skip them. A parent whose nested syntax has
all been retired can itself retire without shrinking its physical descendant
count.

## Code generation

Codegen starts with `root_rules` and recursively asks for `nested_rules`.
It never observes tombstones, raw subtree spans, or Radix cursor mechanics. No
merge-only source chain or reified rule copy is required.

## Correctness invariants

- Every live block has one AST owner and one current EffectiveKey.
- Direct edges are validated through AST topology operations.
- A synthesized node has its final semantic position before queue publication.
- Tombstones preserve physical spans but do not appear in semantic traversal.
- Declaration effects and fallback order remain losslessly representable.
- Unsupported at-rule semantics remain opaque and unequal across occurrences.
- Terminal cleanup does not create new S1-S4 work.

## Performance gates

- No production recursive block-discovery copy or EffectiveKey reconstruction.
- Structural work scales with changed local edges rather than stylesheet size
  times commit count.
- Primary-only stylesheets retain vector-like parse and traversal behavior.
- Track parse, minify, codegen, peak memory, sibling-group count, local relabel
  count, and semantic-iterator time separately.
