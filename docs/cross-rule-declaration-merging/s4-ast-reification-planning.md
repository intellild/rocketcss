# S4: lossless representation planning

## Document map

- [Overall design](./overall.md)
- [S1](./s1-same-selector-coalescing.md)
- [S2](./s2-declaration-effect-pruning.md)
- [S3](./s3-selector-partial-factoring.md)
- [S5](./s5-ast-reification-commit.md)
- [Radix AST IR](../flat-ast-ir/README.md)

## Responsibility

S4 chooses a lossless CSS declaration representation for effect states already
proven by S1-S3. It does not discover new effect liveness, selector unions,
movement, or cascade relationships.

Under the Radix AST design, S4 is not a complete-AST reification planner.
Rules/blocks already have stable IDs and final semantic positions, and S3 has
already inserted synthesized nodes. S4 plans only representations that could
not be committed immediately.

S4 answers:

- which authored declaration origins can remain unchanged;
- which fully dead origins can be omitted;
- whether partially live shorthand effects require typed longhands;
- how retained fallback/opaque origins interleave with replacements;
- which output block owns the resulting declaration list; and
- whether a block uses a source range, `Local4`, or complete overflow list.

## Input state

S4 work may run when one block/sequence's current dependencies are stable. It
participates in the scheduler because changing a representation cost or owner
revision may invalidate an S3 profitability snapshot.

```text
S1/S2/S3 effect decision is committed
all source origins remain available
current block/sequence revisions are known
selector/wrapper topology is already final for the local edit
```

### Rule retention states

| State                             | S4 action                                               |
| --------------------------------- | ------------------------------------------------------- |
| Live effects                      | Retain current AST owner.                               |
| Empty effects with retained child | Retain rule shell and subtree.                          |
| Logically empty                   | Record terminal unlink/removal if not already unlinked. |
| Known no match                    | Record removal of the complete subtree.                 |
| Opaque barrier                    | Retain; do not infer emptiness.                         |
| Empty supported conditional       | Record post-order removal.                              |

Topology may already exclude a logically retired endpoint from live adjacency.
S4 must not expose edges late; the S1-S3 commit that caused emptiness performs
the local unlink and queue insertion.

### Declaration input states

| State                        | Available representation                                     |
| ---------------------------- | ------------------------------------------------------------ |
| All effects of origin live   | Reuse authored origin.                                       |
| All effects dead             | Omit origin.                                                 |
| Partial typed shorthand live | Reuse an exact equivalent sequence or emit proven longhands. |
| Ordered fallback chain       | Preserve order unless a target proof permits removal.        |
| Opaque/recovered occurrence  | Reuse authored origin.                                       |
| Synthesized exact effects    | Materialize typed declarations.                              |

If S2 returned `NoChange`, S4 cannot turn the same relationship into an
optimization. Representation planning does not prove liveness.

### Synthesized-rule input state

Every committed S3 rule supplies its final `RuleId`, `DeclarationBlockId`,
validated selector union, EffectiveKey, common effect sequence, selector-arm
origins, declaration origins, and owning rule list/segment. S4 does not rerun
the partition or choose an insertion position.

## Declaration plan

```rust,ignore
enum AstDeclarationPlan<'ast> {
    ReuseOrigins(SmallVec<[DeclarationOccurrenceId; 4]>),
    Materialize(TypedDeclarationPlan<'ast>),
    Mixed {
        retained_origins: SmallVec<[DeclarationOccurrenceId; 4]>,
        replacements: SmallVec<[TypedDeclarationPlan<'ast>; 4]>,
    },
}
```

The plan also names its current `DeclarationBlockId`, owner revision, effect-IR
revision, and required declaration-list representation. It does not contain a
semantic insertion position: the block ID already encodes that position.

## Sequence representation states

Choose the smallest lossless option:

1. retain/coalesce an exact consecutive authored `Range`;
2. use `Local4` for at most four small local/synthesized declarations whose
   sub-IDs fit the low two property bits; or
3. allocate a complete ordered `Overflow` list.

A range must never include a live declaration owned by a nested or neighboring
block. `Overflow` contains the complete sequence, not only the nonconsecutive
suffix.

Representation choice may consider output size only among already proven
equivalent choices.

## Direct versus deferred commit

When the representation is already known during S1-S3, commit it in that
stage's atomic AST transaction and mark S4 complete for the block. Otherwise
store one deferred plan in pass-local block state. S5 materializes it after the
fixed point.

This avoids a second AST graph while keeping fallible/complex representation
work outside partially committed mutations.

## Examples

### Example 1: exact pruning

```css
a {
  color: red;
  color: blue;
}
```

S2 marks the first occurrence dead. S4 chooses
`ReuseOrigins([color:blue])`; the block can retain a compact range or materialize
the one live origin.

### Example 2: reusing an S1 sequence

S1 selects the right rule as the live owner of the left-then-right declaration
sequence. If both authored ranges remain exact and consecutive, S4 coalesces
them. Otherwise it chooses one complete overflow sequence without changing
their AST occurrence order.

### Example 3: partially live shorthand

```css
a {
  margin: 1px;
  margin-left: 2px;
}
```

If the effect resolver proves only three shorthand components remain live, S4
either retains an exactly equivalent authored sequence or materializes the
three proven longhands plus the authored `margin-left` in exact order.

### Example 4: ordered fallback chain

When multiple authored values form a compatibility fallback chain, S4 retains
their origin order unless S2 already proved an occurrence dead for the active
target policy. Representation planning never sorts or deduplicates fallbacks.

### Example 5: synthesized S3 block

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

S3 has already inserted the `a,b` rule/block at its Radix position. S4 chooses
the declaration list for its `color:red` effect; it does not choose where the
rule goes.

## Completion condition

S4 is complete when every retained non-empty effect sequence has a current
lossless representation and every pending removal has a current owner/topology
revision.

Any plan revision change enqueues affected dependencies before terminal commit.
An inability to produce a lossless plan means S2 or S3 accepted invalid state;
S4 must not guess.

## Invariants

- S4 never changes semantic liveness.
- Every retained fallback and opaque origin preserves exact order.
- Every typed replacement comes from proven effects.
- Every plan is keyed by AST IDs and current revisions.
- No plan contains `SemanticOrderKey` or synthesized insertion position.
- S4 does not rebuild rules, declaration blocks, or EffectiveKeys.
- S5 can apply a complete plan without making semantic choices.
