# S4: logical cleanup and AST reification planning

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

S4 converts the stable logical merge result into one complete,
deterministic `AstReificationPlan`.

S4 answers:

- which logical nodes remain or are removed;
- which AST rule owns each retained declaration sequence;
- which authored declaration origins can be reused;
- which partially live effects require typed declarations;
- where synthesized rules and selectors will be inserted; and
- which retired storage must remain available until commit.

S4 does not modify the stylesheet AST. It also does not discover new semantic
merges, new declaration pruning, or new graph edges.

## Input state

S4 starts only at the complete S1-S3 semantic fixed point:

```text
dirty_same_selector_edges = empty
dirty_partial_edges       = empty
dirty_histories           = empty
candidates                = empty
every history:
  generation == consumed_generation
```

Every rule that became logically empty during S1-S3 has already left live
adjacency, and every edge exposed by that removal has already been classified.
If this precondition is false, control returns to the affected stage:

- equal-selector edge → [S1](./s1-same-selector-coalescing.md#edge-states);
- dirty history → [S2](./s2-declaration-effect-pruning.md#history-generation-states);
- different-selector edge → [S3](./s3-selector-partial-factoring.md#candidate-input-state).

### Rule retention states

| Input state                     | S4 plan                                                                                       |
| ------------------------------- | --------------------------------------------------------------------------------------------- |
| `LiveEffects`                   | Retain the rule or its selected active output owner.                                          |
| `EmptyEffectsWithRetainedChild` | Retain the rule shell and child subtree.                                                      |
| `LogicallyEmpty`                | Plan removal when no sequence still references its storage.                                   |
| `RetiredOutputStorage`          | Keep storage pinned until its active sequence is planned; remove the retired shell afterward. |
| `KnownNoMatch`                  | Plan removal of the complete subtree.                                                         |
| `OpaqueBarrier`                 | Retain; S4 cannot infer emptiness.                                                            |
| `SupportedConditionalEmpty`     | Plan wrapper removal after post-order child analysis.                                         |
| `UnsupportedAtRule`             | Retain regardless of merge-state visibility.                                                  |

### Declaration input states

| Input state                         | Available representation                                            |
| ----------------------------------- | ------------------------------------------------------------------- |
| All effects of an origin live       | Reuse the authored origin when order remains exact.                 |
| All effects of an origin dead       | Omit that origin.                                                   |
| Some virtual shorthand effects live | Use a pre-proven typed replacement or another exact representation. |
| Ordered fallback chain live         | Preserve origins and order unless a target proof permits removal.   |
| Opaque occurrence live              | Reuse its authored origin; never synthesize guessed components.     |
| Synthesized exact effect plan       | Materialize typed declarations with recorded origins.               |

### Synthesized-rule input state

Every logical S3 rule supplies:

```text
validated local selector union
+ resolved effective selector key
+ common effect sequence
+ semantic insertion position
+ selector-arm origins
+ declaration origins
+ owning rule list and segment
```

S4 does not rerun the S3 partition. It only selects an AST representation that
realizes this already committed logical state.

## Sequence representation states

For each retained `DeclarationSequenceId`, S4 chooses exactly one:

```rust,ignore
enum AstDeclarationPlan<'ast> {
    ReuseOrigins(Vec<DeclarationOccurrenceId>),
    Materialize(TypedDeclarationPlan<'ast>),
    Mixed {
        retained_origins: Vec<DeclarationOccurrenceId>,
        replacements: Vec<TypedDeclarationPlan<'ast>>,
    },
}
```

| Plan state     | Use case                                                                                      |
| -------------- | --------------------------------------------------------------------------------------------- |
| `ReuseOrigins` | Authored declarations already represent exactly the live effects.                             |
| `Materialize`  | The final effects require fully typed synthesized declarations.                               |
| `Mixed`        | Some authored fallbacks or opaque origins remain, while partial effects require replacements. |

Representation choice may consider size only among proven equivalent choices.
It never changes semantic liveness.

## Output state

S4 produces:

```rust,ignore
struct AstReificationPlan<'ast> {
    sequences: Map<DeclarationSequenceId, SequenceAstPlan<'ast>>,
    synthesized_rules: Vec<SynthesizedRulePlan<'ast>>,
    removals: Set<RetainedNodeId>,
    complete: bool,
}
```

A complete output satisfies:

| Output component        | Requirement                                                                     |
| ----------------------- | ------------------------------------------------------------------------------- |
| Sequence plans          | Every non-empty retained sequence has exactly one AST owner and representation. |
| Synthesized rules       | Every logical S3 rule has one insertion plan.                                   |
| Removals                | Every removable logical node is listed post-order.                              |
| Retained opaque content | Never listed as removable by inference.                                         |
| Retired storage         | Remains addressable until its sequence representation is complete.              |
| AST mutations           | None.                                                                           |
| Completeness            | `ast_plan.complete == true`.                                                    |

S4 planning is infallible for committed semantic state. An inability to build a
lossless plan indicates that S2 or S3 accepted an invalid transformation.

## Transition examples

### Example 1: empty rule removal

Input logical state:

```text
rule#1.effects = []
rule#1.retained_child_count = 0
rule#1.storage_referenced = false
```

S4 output:

```text
ast_plan.removals += rule#1
AST remains unchanged
```

S5 later removes the node and compacts its owning list.

### Example 2: reusing an S1 sequence

S1 produced:

```text
left = RetiredOutputStorage
right = active_output_owner
combined = [origin#left-color, origin#right-background]
```

If both origins remain fully live, S4 emits:

```text
sequences[combined] = {
  owner: right,
  declarations: ReuseOrigins([
    origin#left-color,
    origin#right-background,
  ]),
}
removals += left
```

See [S1 basic coalescing](./s1-same-selector-coalescing.md#example-1-basic-adjacent-coalescing)
and [S5 owner commit](./s5-ast-reification-commit.md#example-2-committing-an-s1-owner).

### Example 3: partially live shorthand

S2 output:

```text
origin#1 margin: 1px
  top/right/bottom = Live
  left             = Dead
origin#2 margin-left: 2px = Live
```

Reusing `margin:1px` would incorrectly revive the dead left component unless
the later override remains and preserves exact order. Depending on the proven
representation and size, S4 may choose:

```text
ReuseOrigins([origin#1, origin#2])
```

when the authored pair is still exact, or:

```text
Materialize([
  margin-top: 1px,
  margin-right: 1px,
  margin-bottom: 1px,
  margin-left: 2px,
])
```

S4 cannot choose a shorter but inequivalent shorthand.

### Example 4: ordered fallback chain

Input effects:

```text
origin#1 display:-webkit-box [Live]
origin#2 display:flex        [Live]
```

S4 output:

```text
ReuseOrigins([origin#1, origin#2])
```

The order is part of the plan. A map-like output containing only the last value
is invalid.

### Example 5: a rule with no declarations but retained children

Input:

```css
a {
  color: red;

  &:hover {
    color: blue;
  }
}
```

Suppose S2 makes the leading `color:red` effect dead. Logical state:

```text
a.leading_effects = []
a.retained_child_count = 1
```

S4 retains the `a` rule shell and nested child. It plans no leading
declarations but does not place `a` in `removals`.

### Example 6: post-order conditional cleanup

Input logical ownership:

```text
@media rule
└── style rule [LogicallyEmpty]
```

S4 first plans the child removal, then observes that the supported conditional
wrapper has no retained child:

```text
removals order:
  1. child style rule
  2. media wrapper
```

An unsupported or opaque wrapper would remain retained.

### Example 7: planning an S3 synthesized rule

S3 output:

```text
left effects   = [margin: 0]
shared effects = [color: red]
right effects  = [padding: 0]
shared selector plan = union(a, b)
```

S4 output contains three sequence plans and one synthesized insertion:

```text
left owner  -> Materialize/Reuse [margin: 0]
shared rule -> Materialize [color: red], insert between left and right
right owner -> Materialize/Reuse [padding: 0]
```

S5 consumes these plans without recomputing the common sequence.

### Example 8: opaque content pins ancestors

Logical input:

```text
style rule effects = []
└── opaque retained child
```

Even though the declaration IR is empty, S4 retains the style-rule ancestor.
The opaque child cannot be declared empty by this pass.

## Effects on other stages

S4 is downstream of S1-S3 and must not create new work for them. If planning
would expose an unclassified edge, revive a dead effect, or require a different
S3 partition, the semantic precondition was violated:

- ownership problem → revisit [S1 output state](./s1-same-selector-coalescing.md#output-state);
- effect problem → revisit [S2 output state](./s2-declaration-effect-pruning.md#output-state);
- synthesized-rule problem → revisit [S3 commit output state](./s3-selector-partial-factoring.md#commit-output-state).

S4's only normal downstream transition is to
[S5](./s5-ast-reification-commit.md#input-state).

## Invariants

- S4 starts only after the S1-S3 fixed point.
- Cleanup walks retained ownership post-order.
- Logical emptiness was already reflected in adjacency before S4.
- Every non-empty sequence receives exactly one final AST owner.
- Every live effect appears exactly once in the planned semantic sequence.
- Dead effects are never revived.
- Authored shorthand, opaque values, and fallback chains are reused only when
  exact.
- Typed replacements come from pre-proven S2/S3 reification paths.
- Every logical synthesized rule receives one insertion plan.
- Opaque or unsupported nodes are never removed by inference.
- Representation profitability is subordinate to equivalence.
- S4 does not mutate the AST or restart S1-S3.
- A complete plan makes S5 non-fallible and decision-free.

## Completion condition

S4 is complete when:

```text
every retained sequence has one SequenceAstPlan
every logical synthesized rule has one SynthesizedRulePlan
every removable node is covered
no retained node is accidentally covered by removal
ast_plan.complete == true
```

The plan then transitions once to
[S5 AST commit](./s5-ast-reification-commit.md).
