# S5: AST reification commit

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

S5 is the only stage that writes the stable cross-rule merge result back into
the stylesheet AST.

It consumes the complete S4 plan and performs a one-way commit:

- write declarations and importance bits;
- materialize typed replacement declarations;
- insert synthesized rules and selector lists;
- remove planned nodes and compact rule lists; and
- release merge-only storage and revision state.

S5 makes no semantic, candidate, selector-union, or profitability decision.
Code generation is outside this minify pipeline and later consumes only the
ordinary rewritten AST.

## Input state

S5 requires:

```text
S1-S3 semantic fixed point = true
S4 ast_plan.complete       = true
state.reified              = false
```

If the plan is incomplete, S5 does not partially commit. The missing decision
belongs to [S4](./s4-ast-reification-planning.md#completion-condition).

### Plan input states

| Plan item         | Required state                                                                 |
| ----------------- | ------------------------------------------------------------------------------ |
| Sequence          | One final AST owner and one declaration representation.                        |
| Reused origin     | Points to retained authored storage and has exact planned order.               |
| Typed replacement | Fully materializable without semantic analysis.                                |
| Synthesized rule  | Has validated selectors, declaration plan, owner list, and insertion position. |
| Removal           | Refers to a logically dead node and respects post-order ownership.             |
| Retired storage   | Remains pinned until all consumers are written.                                |

### AST visibility states

Before commit, the AST may still contain:

| State                      | Meaning                                                         |
| -------------------------- | --------------------------------------------------------------- |
| Authored stale declaration | Its effect may be dead in IR, but the AST node still exists.    |
| Retired S1 rule shell      | It is absent from live adjacency but still owns pinned storage. |
| Missing S3 shared rule     | The logical synthesized rule is not yet in the AST.             |
| Logically dead node        | S4 plans removal, but the AST still owns it.                    |

S5 resolves all of these differences.

## Declaration commit states

For each `AstDeclarationPlan`:

| Input plan     | S5 action                                                                             | Output AST state                                                   |
| -------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `ReuseOrigins` | Move or clone the planned authored occurrences into the final owner in planned order. | Exact authored syntax and fallback order retained.                 |
| `Materialize`  | Build the listed typed AST declarations and importance bits.                          | Exact typed replacement sequence present.                          |
| `Mixed`        | Interleave retained origins and replacements according to the complete order plan.    | Opaque/fallback origins and typed replacements coexist losslessly. |
| Empty sequence | Write no leading declarations.                                                        | Owner may remain only for retained children.                       |

S5 does not ask whether a declaration is dead; that decision was made by S2
and encoded by S4.

## Output state

After successful commit:

| State component           | Required output                                                |
| ------------------------- | -------------------------------------------------------------- |
| Stylesheet AST            | Exactly represents the stable live effect and ownership state. |
| Declaration owners        | Each retained sequence is written to exactly one AST owner.    |
| Synthesized rules         | Present at their semantic insertion positions.                 |
| Planned removals          | Physically absent; owning lists compacted.                     |
| Retired storage           | No longer externally observable.                               |
| Merge IR                  | Cleared or dropped.                                            |
| Revisions and work queues | Cleared or dropped.                                            |
| `state.reified`           | `true`.                                                        |
| Codegen dependency        | AST only.                                                      |

### Ownership and storage output

For an S1 sequence:

```text
before S5:
  left AST shell owns pinned authored block
  right is logical active_output_owner

after S5:
  right AST rule owns the complete planned declaration sequence
  left AST shell is removed if planned
  previous_merged/retired-storage links are cleared
```

No merge-only reference may be needed by code generation.

## Commit ordering

The conceptual commit order is:

1. materialize declaration representations while all referenced origins remain
   available;
2. write declarations to active owners;
3. insert synthesized selectors and rules;
4. apply post-order removals and compact rule lists;
5. clear merge-only storage and revisions; and
6. set `reified = true`.

An implementation may batch these operations differently if it preserves the
same dependency order and cannot expose a partially committed AST.

## Transition examples

### Example 1: committing a pruned declaration

Input AST before S5:

```css
a {
  color: red;
  color: blue;
}
```

S4 plan:

```text
owner = a
declarations = ReuseOrigins([color: blue])
```

S5 output AST:

```css
a {
  color: blue;
}
```

S5 did not calculate the override; it only applied the plan produced from
[S2](./s2-declaration-effect-pruning.md#example-1-exact-longhand-override).

### Example 2: committing an S1 owner

Input AST before S5:

```css
a {
  color: red;
}

a {
  background: white;
}
```

S4 plan:

```text
sequence owner = second a rule
declarations = ReuseOrigins([
  first.color,
  second.background,
])
removals = [first a rule]
```

S5 output:

```css
a {
  color: red;
  background: white;
}
```

The first rule's storage is released only after its declaration origin has
been written to the right owner.

### Example 3: typed replacement for a partial shorthand

S4 plan:

```text
Materialize([
  margin-top: 1px,
  margin-right: 1px,
  margin-bottom: 1px,
  margin-left: 2px,
])
```

S5 output:

```css
a {
  margin-top: 1px;
  margin-right: 1px;
  margin-bottom: 1px;
  margin-left: 2px;
}
```

S5 preserves planned order and importance bits. It does not attempt to
recombine the declarations into `margin`.

### Example 4: synthesized-rule commit

Before S5, S3 and S4 logically describe:

```text
a   -> margin: 0
a,b -> color: red
b   -> padding: 0
```

S5 inserts the shared AST rule at its planned position:

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

The selector union was validated in
[S3](./s3-selector-partial-factoring.md#example-9-invalid-selector-union-rejects-the-candidate)
and placed in the plan by
[S4](./s4-ast-reification-planning.md#example-7-planning-an-s3-synthesized-rule).
S5 does not resolve it again.

### Example 5: retaining a child-only rule

S4 plan says the parent has no leading declaration sequence but has a retained
child:

```css
a {
  &:hover {
    color: blue;
  }
}
```

S5 writes an empty leading declaration block if required by the AST shape and
retains the parent shell. It does not remove `a`.

### Example 6: post-order removal and compaction

Input AST:

```css
@media (width >= 600px) {
  a {
    color: red;
  }
}

b {
  color: blue;
}
```

If S4 plans removal of both `a` and the now-empty supported media wrapper, S5
removes the child before compacting away the wrapper:

```css
b {
  color: blue;
}
```

### Example 7: preserving fallback order

S4 plan:

```text
ReuseOrigins([
  display: -webkit-box,
  display: flex,
])
```

S5 output:

```css
a {
  display: -webkit-box;
  display: flex;
}
```

Origin order is copied exactly. S5 never stores these declarations in an
unordered single-value property map.

## Synthesized-rule commit

For each synthesized plan, S5:

1. uses the already materialized and validated local selector list;
2. builds the final style-rule AST wrapper with the planned emission identity;
3. writes the planned declarations and importance bits;
4. attaches source spans or combined origins;
5. inserts the rule into its owning list at the semantic position; and
6. preserves surrounding retained child and wrapper order.

Any operation that could semantically reject the selector or declaration plan
must have occurred before S5. Allocation failure handling is an implementation
concern, not a new optimization choice.

## Effects on other stages

S5 is terminal for this minify invocation:

- it does not return to [S1](./s1-same-selector-coalescing.md);
- it does not rerun [S2](./s2-declaration-effect-pruning.md);
- it does not recompute [S3](./s3-selector-partial-factoring.md); and
- it does not revise [S4](./s4-ast-reification-planning.md).

If S5 would need any such decision, the S4 plan is incomplete and commit must
not begin.

After S5, the next consumer is ordinary code generation:

```text
rewritten stylesheet AST -> code generation
```

Code generation has no link back to merge IR, effect indexes, candidates,
histories, or retired storage.

## Invariants

- S5 starts only with a complete S4 plan.
- S5 is the only stage that mutates AST ownership for this feature.
- Every retained sequence is written exactly once.
- Planned declaration order, fallback order, prefixes, and importance survive.
- Synthesized selectors and rules use their already validated plans.
- Removals occur only for nodes listed by S4.
- Retained child-only, opaque, and unsupported nodes remain present.
- Referenced authored storage is consumed before retired shells are removed.
- S5 makes no semantic or profitability decision.
- S5 does not create new S1-S4 work.
- Merge-only state is unobservable after commit.
- Code generation receives only the ordinary AST.

## Completion condition

Minification is complete when:

```text
semantic fixed point is still true
ast_plan.complete == true
state.reified == true
no merge-only storage is observable from the AST
```

A second minify invocation may rediscover state from the rewritten AST, but the
first invocation never loops from S5 back into S1-S4.
