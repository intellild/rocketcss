# S1: same-selector coalescing

## Document map

- [Overall design](./overall.md)
- [S1: same-selector coalescing](./s1-same-selector-coalescing.md)
- [S2: declaration-effect pruning](./s2-declaration-effect-pruning.md)
- [S3: selector partial factoring](./s3-selector-partial-factoring.md)
- [S4: AST reification planning](./s4-ast-reification-planning.md)
- [S5: AST reification commit](./s5-ast-reification-commit.md)
- [Detailed state machine](./detailed-state-machine.md)
- [Pseudocode](./pseudo-code.md)
- [Non-goals](./non-goal.md)

## Responsibility

S1 coalesces two live-adjacent style-rule endpoints when they have exactly the
same effective rule identity and emission identity.

S1 changes logical ownership and adjacency only. It does not:

- remove overridden declaration effects; that is
  [S2](./s2-declaration-effect-pruning.md);
- factor declarations shared by different selectors; that is
  [S3](./s3-selector-partial-factoring.md);
- choose the final declaration AST representation; that is
  [S4](./s4-ast-reification-planning.md); or
- write or remove AST nodes; that is
  [S5](./s5-ast-reification-commit.md).

The right endpoint becomes the active output owner. The left declaration
sequence is linked before the right declaration sequence, and the left
endpoint is retired from the live adjacency graph.

## Input state

S1 consumes one current live edge:

```rust,ignore
struct Edge {
    list: RuleListId,
    segment: RuleListSegmentId,
    left: RuleId,
    right: RuleId,
}
```

### Endpoint states

| State                  | Meaning                                                    | S1 treatment                                  |
| ---------------------- | ---------------------------------------------------------- | --------------------------------------------- |
| `Live`                 | The rule is a current adjacency endpoint.                  | May participate.                              |
| `LogicallyEmpty`       | The rule has no live effects or retained children.         | Must already be unlinked; cannot participate. |
| `RetiredOutputStorage` | Its declaration storage is referenced by another sequence. | Not an endpoint; cannot participate again.    |
| `KnownNoMatch`         | The complete subtree is proven not to output.              | Never participates.                           |
| `OpaqueBarrier`        | The subtree is retained but has no cross-rule identity.    | Splits adjacency and history.                 |

Both endpoints must be `Live`.

### Edge states

An observed pair is classified into exactly one of these states:

| State                | Conditions                                                                                                                  | Next action                                                                                  |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `S1Eligible`         | Same list and segment, mutually adjacent, equal effective rule keys, equal emission identities, left has no retained child. | Commit S1.                                                                                   |
| `S3Eligible`         | Same structural constraints, but selectors differ and a common effect sequence may exist.                                   | Route to [S3 candidate discovery](./s3-selector-partial-factoring.md#candidate-input-state). |
| `BlockedByLeftChild` | Left has retained child content.                                                                                            | No movement across the child.                                                                |
| `SegmentMismatch`    | A retained sibling separates the endpoints.                                                                                 | No edge exists.                                                                              |
| `EmissionMismatch`   | Wrapper, prefix, or serialization context differs.                                                                          | No merge.                                                                                    |
| `Stale`              | The pair is no longer current live adjacency.                                                                               | Discard the work item.                                                                       |

Equality means the complete `EffectiveRuleKey`, not textual selector equality:

```text
history segment
+ cascade and conditional context
+ effective selector semantics
```

The additional `EmissionIdentity` must also be equal.

### Declaration-sequence input

Each endpoint owns an ordered sequence:

```text
left.blocks  = [L1, L2, ...]
right.blocks = [R1, R2, ...]
```

Every block keeps its original `SemanticSourceOrderKey` and history entry.
S1 must not collapse the sequence into one property map or one synthetic
history occurrence.

## Output state

A successful S1 transition produces:

```text
combined.blocks = [L1, L2, ..., R1, R2, ...]
combined.active_output_owner = right

left.state = RetiredOutputStorage
right.state = Live
right.leading_sequence = combined
```

The output states are:

| State component    | Before                                                     | After                                                |
| ------------------ | ---------------------------------------------------------- | ---------------------------------------------------- |
| Live endpoints     | `left`, `right`                                            | `right` only                                         |
| Output owner       | `left` and `right` separately                              | `right`                                              |
| Declaration order  | Left block order, then right block order in the stylesheet | One sequence with the same order                     |
| History membership | One entry per original block                               | Unchanged                                            |
| Sequence revision  | Revisions of two sequences                                 | New incremented aggregate revision                   |
| AST                | Two authored rules                                         | Unchanged until [S5](./s5-ast-reification-commit.md) |

The local live chain changes atomically:

```text
before: previous -> left -> right -> next
after:  previous -> right -> next
```

Only the final incident edges are classified. A temporary
`previous -> next` bypass edge is never exposed.

## Transition examples

### Example 1: basic adjacent coalescing

Input AST:

```css
a {
  color: red;
}

a {
  background: white;
}
```

Input logical state:

```text
edge = (rule#1, rule#2)
effective_key(rule#1) == effective_key(rule#2)
emission_identity(rule#1) == emission_identity(rule#2)
rule#1.retained_child_count == 0

sequence#1 = [color: red]
sequence#2 = [background: white]
```

S1 output:

```text
rule#1 = RetiredOutputStorage
rule#2 = Live, active_output_owner
sequence#3 = [color: red] -> [background: white]
```

The AST still contains both authored rules at this point. S4 later decides how
`sequence#3` is represented, and S5 writes the decision. A possible final AST
is:

```css
a {
  color: red;
  background: white;
}
```

See [S4 example: reusing an S1 sequence](./s4-ast-reification-planning.md#example-2-reusing-an-s1-sequence)
and [S5 example: committing removals and ownership](./s5-ast-reification-commit.md#example-2-committing-an-s1-owner).

### Example 2: declaration order remains observable

Input:

```css
a {
  color: red;
}

a {
  color: blue;
}
```

S1 does not decide that `color:red` is dead. It produces:

```text
combined = [color: red] -> [color: blue]
```

That exact ordered sequence is passed to
[S2](./s2-declaration-effect-pruning.md#example-1-exact-longhand-override),
which may mark the earlier effect dead.

### Example 3: the right endpoint retains children

Input:

```css
a {
  color: red;
}

a {
  background: white;

  &:hover {
    color: blue;
  }
}
```

The left endpoint has no retained child, so it may be retired. The right
endpoint remains the owner because its nested child must stay after the
combined leading declarations:

```text
right output order:
  left declaration blocks
  right leading declaration blocks
  right child sequence
```

S1 output:

```text
right.leading_sequence =
  [color: red] -> [background: white]
right.retained_child_count = 1
```

The nested rule does not move.

### Example 4: a retained child on the left blocks S1

Input:

```css
a {
  color: red;

  &:hover {
    color: blue;
  }
}

a {
  background: white;
}
```

Moving the left declarations into the right rule would also change their
position relative to the retained nested rule. The edge state is
`BlockedByLeftChild`; S1 produces no change.

### Example 5: a local barrier prevents adjacency

Input:

```css
a {
  color: red;
}

@supports (display: grid) {
  .grid {
    display: grid;
  }
}

a {
  background: white;
}
```

The supported conditional wrapper ends the containing list segment. The two
`a` rules are not an S1 edge even though their effective selectors may compare
equal in another analysis context.

If their histories remain comparable, S2 may still reason about declaration
effects. Structural coalescing remains forbidden. See
[S2 history input](./s2-declaration-effect-pruning.md#history-input-state).

### Example 6: S2 exposes a new S1 edge

Input:

```css
a {
  color: red;
}

b {
  color: red;
}

a {
  background: white;
}

b {
  color: blue;
}
```

S2 can prove the first `b { color:red }` effect dead because the later
equal-history `b { color:blue }` overrides it. Its logical-empty transition
unlinks the first `b` rule:

```text
before: a#1 -> b -> a#2
after:  a#1 -> a#2
```

The new edge is classified immediately and routed back to S1. S4 is not
allowed to discover this adjacency late. See
[S2 effect edits and liveness](./s2-declaration-effect-pruning.md#output-state)
and [S4 input precondition](./s4-ast-reification-planning.md#input-state).

### Example 7: equal text but unequal emission identity

Input:

```text
left.wrapper_kind  = StyleRule
right.wrapper_kind = LegacyNestRule
selectors(left)    = a
selectors(right)   = a
```

The selector text is equal, but the wrappers serialize differently. The edge
state is `EmissionMismatch`; no S1 merge occurs.

## Effects on other stages

### Effect on S2

S1 preserves the individual history entries but changes their owning aggregate
sequence. The affected history is dirtied so S2 reprocesses it in semantic
source order. See
[S2 history generations](./s2-declaration-effect-pruning.md#history-generation-states).

### Effect on S3

All candidates incident to either old endpoint are invalidated. Any candidate
using the right endpoint must snapshot the new aggregate sequence revision.
See [S3 candidate invalidation](./s3-selector-partial-factoring.md#candidate-invalidation).

### Effect on S4

S1 creates a retired storage node and one active output owner. S4 must assign
the complete sequence to the right owner and plan removal of the retired AST
shell only after no plan depends on its storage. See
[S4 sequence representation states](./s4-ast-reification-planning.md#sequence-representation-states).

### Effect on S5

S5 writes the combined declaration representation to the right AST owner,
removes or compacts the retired left rule according to the S4 plan, and clears
the merge-only storage link. See
[S5 ownership commit](./s5-ast-reification-commit.md#ownership-and-storage-output).

## Invariants

- Both endpoints are revalidated immediately before commit.
- S1 never crosses a `RuleListSegmentId`.
- Effective rule keys and emission identities use exact equality.
- The left endpoint has no retained child content.
- The right endpoint is the active output owner.
- Declaration blocks and source-order keys retain their original order.
- History entries are preserved rather than collapsed.
- The combined aggregate revision changes.
- Old incident edge work and S3 candidates are removed before reconnection.
- Only final live edges are classified.
- No authored declaration or selector AST is rewritten in S1.
- Every successful output has a lossless S4 representation path.

## Completion condition

S1 is locally complete when `dirty_same_selector_edges` is empty. It is not
globally complete until S2 and S3 can no longer expose a new eligible edge.
The semantic pass reaches that fixed point before
[S4 begins](./s4-ast-reification-planning.md#input-state).
