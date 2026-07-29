# S3: selector partial factoring

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

S3 factors a provably movable common effect sequence from two live-adjacent
rules with different selectors:

```text
SL { DL }                 SL    { left_only }
SR { DR }       ->        SL,SR { common }
                          SR    { right_only }
```

S3 is split into speculative candidate discovery and atomic candidate commit.
Discovery does not mutate semantic state. Commit occurs only after S1 and S2
work capable of changing either endpoint is stable.

S3 does not choose the physical AST declaration representation or insert a
physical AST rule. Those responsibilities belong to
[S4](./s4-ast-reification-planning.md) and
[S5](./s5-ast-reification-commit.md).

## Candidate input state

S3 consumes one current live edge whose endpoints have different effective
selector keys.

### Structural input states

| State                  | Requirement                                                             |
| ---------------------- | ----------------------------------------------------------------------- |
| `CurrentLiveEdge`      | Endpoints are mutually adjacent and live.                               |
| `SameListAndSegment`   | Both endpoints belong to the same `RuleListId` and `RuleListSegmentId`. |
| `SameContext`          | Conditional and cascade contexts are exactly equal.                     |
| `SameEmissionIdentity` | Wrapper kind, prefix, and selector serialization context are equal.     |
| `MovableLeft`          | The left endpoint has no retained child content.                        |
| `DifferentSelectors`   | Otherwise the edge belongs to [S1](./s1-same-selector-coalescing.md).   |

Failure of any requirement produces no candidate.

### Declaration input states

Each endpoint sequence is partitioned by a typed movement proof:

```rust,ignore
struct PartialMergePlan<'ast> {
    left_only: EffectPlan<'ast>,
    common: EffectPlan<'ast>,
    right_only: EffectPlan<'ast>,
    selectors: SynthesizedSelectorPlan<'ast>,
    insertion_order: SemanticSourceOrderKey,
}
```

The partition has one of these states:

| State                 | Meaning                                                                           |
| --------------------- | --------------------------------------------------------------------------------- |
| `NoCommonEffects`     | Nothing can be factored.                                                          |
| `UnsafeCommonEffects` | Values look equal, but order, aliases, fallback, or overlap behavior is unproven. |
| `PartialCommon`       | `common` is non-empty and at least one residual is non-empty.                     |
| `CompleteCommon`      | Both residual sequences are empty.                                                |

Effect equality includes importance, prefix, target compatibility, fallback
position, and value semantics. Property-name equality is insufficient.

### Candidate stabilization states

| Candidate state       | Meaning                                                          | Action                         |
| --------------------- | ---------------------------------------------------------------- | ------------------------------ |
| `Discovered`          | A plan and endpoint revisions were captured.                     | Keep speculative.              |
| `WaitingForHistories` | An endpoint history can still change.                            | Do not commit.                 |
| `Ready`               | All higher-priority work is stable and snapshots still match.    | Revalidate and commit.         |
| `Stale`               | Adjacency, revision, liveness, or context changed.               | Remove and recompute the edge. |
| `Rejected`            | Selector materialization, movement, or reification proof failed. | Produce no semantic change.    |

## Candidate output state

Candidate discovery records:

```text
edge identity
+ left aggregate revision
+ right aggregate revision
+ exact endpoint effect edits
+ common effect plan
+ immutable selector-arm origins
+ synthesized selector plan
+ semantic insertion position
+ proof that all three result sequences are losslessly reifiable
```

It changes neither the effect IR nor the AST.

## Commit output state

A successful atomic commit:

1. marks the common endpoint effects dead;
2. creates an independent logical synthesized rule and effect sequence;
3. materializes and validates the selector union for semantic use;
4. inserts the synthesized history occurrence in semantic order;
5. recomputes endpoint liveness;
6. reconnects the final live neighborhood once; and
7. dirties every affected history and final incident edge.

Possible final neighborhood states are:

| Residual state                             | Final live chain                              |
| ------------------------------------------ | --------------------------------------------- |
| Both residuals live                        | `previous -> left -> shared -> right -> next` |
| Left empty, right live                     | `previous -> shared -> right -> next`         |
| Left live, right empty                     | `previous -> left -> shared -> next`          |
| Both empty                                 | `previous -> shared -> next`                  |
| Right has retained children but no effects | `previous -> shared -> right -> next`         |

The synthesized rule is logical state only. It becomes an AST node in S5.

## Transition examples

### Example 1: partial factoring

Input:

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

Candidate partition:

```text
left_only  = [margin: 0]
common     = [color: red]
right_only = [padding: 0]
selectors  = union(a, b)
```

Logical commit output:

```text
a   -> effects [margin: 0]
a,b -> effects [color: red]  (logical synthesized rule)
b   -> effects [padding: 0]
```

A possible S5 AST result is:

```css
a {
  margin: 0;
}

a,
b {
  color: red;
}

b {
  padding: 0;
}
```

### Example 2: complete factoring

Input:

```css
a {
  color: red;
}

b {
  color: red;
}
```

Candidate state is `CompleteCommon`:

```text
left_only  = []
common     = [color: red]
right_only = []
```

Atomic output:

```text
before: previous -> a -> b -> next
after:  previous -> shared(a,b) -> next
```

S3 must not expose `previous -> next` between retiring the endpoints and
inserting `shared`.

### Example 3: overlap semantics must remain unchanged

Input:

```css
.x {
  color: red;
}

.x.y {
  color: red;
}
```

An element may match both selectors. Factoring is safe only if the common
sequence appears at a position that preserves the same value, importance, and
relative ordering for:

```text
matches left only
matches right only
matches both
```

The movement proof must cover all three domains. If it cannot, the candidate is
`UnsafeCommonEffects` and is rejected.

### Example 4: an S2 edit invalidates the candidate

Input:

```css
a {
  color: red;
}

b {
  color: red;
}

a {
  color: blue;
}
```

The first edge initially suggests a complete common sequence. But S2 may later
mark the first `a { color:red }` effect dead because of the final `a` rule.
S3 therefore stores endpoint revisions and waits.

```text
candidate snapshots: left.rev = 1, right.rev = 1
S2 edits left:        left.rev = 2
candidate state:      Stale
```

The candidate is discarded instead of requiring a later split of an already
synthesized `a,b` rule. See
[S2 effect on S3](./s2-declaration-effect-pruning.md#effect-on-s3).

### Example 5: an empty endpoint exposes a new candidate

Logical input:

```text
a -> emptying-rule -> b
```

After S2 removes the last effect of the middle rule:

```text
a -> b
```

If `a` and `b` have different selectors and compatible contexts, the new edge
is routed to S3 candidate discovery. This happens during the fixed point, not
during S4 cleanup.

### Example 6: retained left children block movement

Input:

```css
a {
  color: red;

  &:hover {
    color: blue;
  }
}

b {
  color: red;
}
```

The apparent common declaration cannot move past the nested child. The
structural state is not `MovableLeft`, so no candidate is created.

### Example 7: right children retain the right endpoint

Input:

```css
a {
  color: red;
}

b {
  color: red;

  &:hover {
    color: blue;
  }
}
```

Factoring can retire the declaration effects of both endpoints, but the right
rule still owns retained child content:

```text
previous -> shared(a,b) -> b-with-child -> next
```

S4 plans an empty leading declaration block for `b` without removing its
retained subtree. See
[S4 rule retention states](./s4-ast-reification-planning.md#rule-retention-states).

### Example 8: fallback position prevents naive matching

Input:

```css
a {
  display: -webkit-box;
  display: flex;
}

b {
  display: flex;
}
```

The final textual `display:flex` values match, but the declarations occupy
different fallback sequences. S3 may not extract `display:flex` unless the
typed movement proof shows that doing so preserves behavior for all configured
targets. Otherwise the candidate is `UnsafeCommonEffects`.

### Example 9: invalid selector union rejects the candidate

Logical input:

```text
left selectors  = a valid selector list
right selectors = another valid selector list
common effects  = proven movable
```

If the combined selector list cannot be deep-materialized or is invalid in the
current nesting or target context, commit stops before endpoint mutation:

```text
candidate = Rejected
left/right effect IR = unchanged
AST = unchanged
```

### Example 10: synthesized history is inserted by semantic order

Given:

```text
source order: left@10, right@20, later-entry@30
shared insertion position: between left and right
```

The shared occurrence receives an order key between the endpoints and is
inserted before `later-entry`, even if that later entry was discovered first.
S2 then sees the correct cascade order.

## Candidate invalidation

A candidate becomes stale when any of these changes:

- an endpoint is no longer live;
- the endpoints are no longer mutually adjacent;
- list, segment, conditional context, or emission identity differs;
- the left endpoint gains retained child content;
- either sequence aggregate revision changes;
- the effect movement proof no longer succeeds;
- the selector union is no longer valid; or
- a lossless AST reification path is no longer available.

Invalidation removes the cached candidate and reclassifies the current edge.
Cached validity is never authority.

## Effects on other stages

### Effect on S1

Endpoint retirement or insertion of the shared rule creates final incident
edges. Equal-key edges are routed to
[S1](./s1-same-selector-coalescing.md#edge-states).

### Effect on S2

S3 edits endpoint effect masks and inserts a synthesized history occurrence.
All affected histories become dirty and must reach a new local fixed point.
See [S2 history generation states](./s2-declaration-effect-pruning.md#history-generation-states).

### Effect on S4

S3 supplies three logical effect plans, selector origins, insertion order, and
final owners. S4 must plan a lossless AST representation for every retained
residual and the shared sequence. See
[S4 synthesized-rule input](./s4-ast-reification-planning.md#synthesized-rule-input-state).

### Effect on S5

S5 inserts the validated synthesized selector and rule at the exact planned
AST position. It cannot recalculate the union or change the partition. See
[S5 synthesized-rule commit](./s5-ast-reification-commit.md#synthesized-rule-commit).

## Invariants

- S1 and S2 work has priority over candidate commit.
- S3 never crosses a rule-list segment.
- The left endpoint has no retained child content.
- Common-effect equality includes all cascade and compatibility semantics.
- Behavior is preserved for left-only, right-only, and overlap matches.
- Candidate discovery is non-mutating.
- Every candidate snapshots both sequence revisions.
- Every residual and shared sequence is proven reifiable before commit.
- All fallible selector work completes before endpoint effect mutation.
- Selector unions are filtered, normalized, deduplicated, and validated
  immediately.
- Existing endpoint and ancestor selector ASTs remain immutable.
- Commit is one atomic live-graph transition.
- Synthesized histories use semantic insertion order.
- Physical AST insertion is deferred to S5.

## Completion condition

S3 is idle when `dirty_partial_edges` and the candidate map are empty. Because
an S3 commit can dirty S1 edges and S2 histories, global completion is the
shared S1-S3 fixed point. Only then may
[S4](./s4-ast-reification-planning.md#input-state) run.
