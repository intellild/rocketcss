# S2: declaration-effect pruning

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

S2 removes declaration effects that are provably dead within one exact
effective-rule history. It operates on virtual semantic effects, not directly
on authored declaration AST nodes.

S2 may:

- mark all effects of an authored longhand occurrence dead;
- mark only selected virtual longhand effects of a shorthand dead; or
- retain the typed effect and origin information S4 needs to choose a lossless
  representation.

S2 does not:

- structurally coalesce equal-selector rules; see
  [S1](./s1-same-selector-coalescing.md);
- move shared effects between different selectors; see
  [S3](./s3-selector-partial-factoring.md);
- choose the final compact AST form; see
  [S4](./s4-ast-reification-planning.md); or
- mutate authored declaration blocks; see
  [S5](./s5-ast-reification-commit.md).

> Incremental implementation contract: until logical effect masks and S4/S5
> reification exist, cross-block S2 is restricted to exact duplicate
> occurrence pruning. It may tombstone an earlier declaration only when the
> later declaration is exactly equal and belongs to the same exact history.
> It must not reuse the full block-local shorthand/longhand or columns rewrite
> pipeline across declaration blocks. Leaf rules made empty by exact pruning
> may still be retired so newly exposed S1 edges reach a fixed point.

## Incremental exact-only boundary

The incremental implementation compares occurrences, not virtual effects. An
earlier occurrence is removable only when all of the following are equal:

```text
EffectiveRuleKey
+ PropertyId, including case-sensitive custom-property identity
+ vendor prefix
+ importance and cascade phase
+ complete declaration value
```

The property index always points to the latest occurrence. A different value
for the same property replaces that index even when it cannot eliminate the
previous fallback. Therefore exact pruning does not search backward through a
different-value compatibility chain:

```css
a { width: 1px }
a { width: 2px }
a { width: 1px }
```

All three occurrences remain in the incremental implementation.

The exact-only path may delete an old occurrence, but it never:

- replaces a declaration at another physical source position;
- folds a later longhand into an earlier shorthand;
- combines longhands from different declaration blocks;
- combines column longhands from different declaration blocks; or
- discovers shorthand, `all`, logical/physical, target, or fallback effect
  relationships.

Those cases return `NoChange` until the complete effect resolver exists. The
complete S2 resolver—not S4—must later decide effect liveness. S4 only chooses
a representation for an already proven S2 result while preserving every
semantic source position.

## History input state

S2 consumes one `EffectiveRuleHistory`:

```rust,ignore
struct EffectiveRuleHistory {
    entries: OrderedMap<SemanticSourceOrderKey, DeclarationEntryId>,
    generation: Generation,
    consumed_generation: Generation,
}
```

All entries have exactly the same:

```text
history segment
+ effective selector semantics
+ ordered conditional context
+ layer, origin, and cascade phase
```

Entries may come from different nesting depths or different structural rule
lists. S2 requires history identity, not local S1/S3 adjacency.

### History generation states

| State                    | Condition                                            | S2 action                           |
| ------------------------ | ---------------------------------------------------- | ----------------------------------- |
| `Stable`                 | `generation == consumed_generation`                  | No work.                            |
| `Dirty`                  | `generation > consumed_generation` and key is queued | Process to a local fixed point.     |
| `MutatedDuringScan`      | A typed edit increments `generation` while scanning. | Continue scanning the same history. |
| `BlockedByOpaqueSegment` | A barrier assigned a different `HistorySegmentId`.   | Never compare across the barrier.   |

Discovery order is not authoritative. `entries` are always read by
`SemanticSourceOrderKey`.

### Scheduling identity

The S2 work queue contains one effective-rule history identity, not a pair of
declaration entries. A compact implementation may intern `EffectiveRuleKey`
values into `HistoryId(u32)` and keep:

```rust,ignore
history_by_key: FxHashMap<EffectiveRuleKey, HistoryId>
dirty_histories: DirtyHistoryQueue<HistoryId>
```

Initial source-ordered discovery appends each declaration entry directly to its
history. The exact-only implementation may defer queueing a history until its
second entry because a one-entry history has no cross-block relationship.
Subsequent S1-S4 mutations increment `generation` and enqueue the same history
identity when it is not already queued.

The exact-only implementation does not require one allocation per history. Its
immutable initial histories may use one-dimensional linked storage:

```rust,ignore
blocks: Vec<Ref<DeclarationBlock>>
next_in_history: Vec<Option<NonZeroU32>>
history_heads: Vec<u32>
```

Only keys with at least two entries need storage. `blocks` retains the
source-ordered references, `next_in_history` links entries with the same
effective-rule identity, and `history_heads` starts each S2 traversal. This is
a physical representation of the same ordered histories; it does not change
their semantic identity or permit comparisons between histories.

`Candidate(left, right)` is an edge representation for S1/S3. It must not be
used to reconstruct an S2 history through pair chaining, tail maps, or another
candidate pass. A history is marked consumed only after the complete ordered
history reaches its local fixed point.

### Effect-occurrence input states

Each authored declaration occurrence has one expansion state:

| Expansion state    | Example                | Meaning                                                                                     |
| ------------------ | ---------------------- | ------------------------------------------------------------------------------------------- |
| `Exact`            | `color: red`           | One typed declaration has an exact modeled effect set.                                      |
| `VirtualShorthand` | `margin: 1px`          | One authored origin contributes multiple canonical longhand effects.                        |
| `Opaque`           | `margin: var(--space)` | The exact component values are unavailable; conservatively conflicts with its affected set. |

Each effect in an occurrence has one liveness state:

| Liveness state | Meaning                                 |
| -------------- | --------------------------------------- |
| `Live`         | Required to preserve current semantics. |
| `Dead`         | Proven unobservable in this history.    |

The effect index is an ordered multimap. Multiple live occurrences for one
effect key may be a required fallback chain.

## Resolver output states

For every analyzed relationship, the resolver returns:

```rust,ignore
enum EffectResolution {
    NoChange,
    Apply(EffectEditPlan),
}
```

| Output                   | Meaning                                                   |
| ------------------------ | --------------------------------------------------------- |
| `NoChange`               | Safety, compatibility, or reifiability is not proven.     |
| `Apply(MarkEffectsDead)` | The effect mask is provably dead and remains reifiable.   |

Applying a plan changes semantic state:

```text
occurrence live-effect mask
+ declaration-entry revision
+ sequence aggregate revision
+ effect-IR revision
+ history generation
+ rule logical liveness, if the final effect disappears
```

It does not change the authored AST.

## Output state

After S2 reaches a local fixed point:

| State component    | Output requirement                                                                   |
| ------------------ | ------------------------------------------------------------------------------------ |
| Effect masks       | Every proven dead effect is marked dead.                                             |
| Authored origins   | Preserved for every live or dead occurrence until S5.                                |
| Typed effect data   | Retained with origin and owner identity so S4 can choose a representation.            |
| History generation | `generation == consumed_generation`.                                                 |
| Sequence revisions | Incremented for every affected aggregate sequence.                                   |
| Sequence liveness  | Aggregate live-effect counts match the remaining live masks.                         |
| S3 candidates      | Incident stale candidates invalidated.                                               |
| Logical adjacency  | Rules that became empty are already unlinked and newly exposed edges are classified. |
| AST                | Unchanged.                                                                           |

S2 output is conservative. `NoChange` is a valid stable result.

## Transition examples

The examples below describe the complete effect-IR design. Under the
incremental exact-only boundary, only examples involving exactly equal
occurrences are pruned. Different-value overrides and partial shorthand
relationships remain unchanged.

### Example 1: exact longhand override

Input:

```css
a {
  color: red;
}

a {
  color: blue;
}
```

After S1, the history still contains two ordered occurrences:

```text
occurrence#1 color = red  [Live]
occurrence#2 color = blue [Live]
```

S2 proves the earlier normal declaration dead:

```text
occurrence#1 color = red  [Dead]
occurrence#2 color = blue [Live]
history.generation == history.consumed_generation
```

No AST declaration is deleted yet. S4 plans a representation containing only
the live effect, and S5 writes it.

### Example 2: partial shorthand override

Input:

```css
a {
  margin: 1px;
  margin-left: 2px;
}
```

Effect IR before S2:

```text
origin#1 margin: 1px [VirtualShorthand]
  margin-top    = 1px [Live]
  margin-right  = 1px [Live]
  margin-bottom = 1px [Live]
  margin-left   = 1px [Live]

origin#2 margin-left: 2px [Exact]
  margin-left   = 2px [Live]
```

S2 output:

```text
origin#1:
  top/right/bottom [Live]
  left             [Dead]

origin#2:
  left             [Live]
```

S2 does not physically expand `margin`. S4 chooses between an exact authored
form, typed longhands, or another proven equivalent representation. See
[S4 example: partially live shorthand](./s4-ast-reification-planning.md#example-3-partially-live-shorthand).

### Example 3: ordered fallback chain is retained

Input:

```css
a {
  display: -webkit-box;
  display: flex;
}
```

A single-value property map would keep only `display:flex`, but S2's ordered
effect chain contains both occurrences:

```text
display:
  1. -webkit-box [Live for a compatibility domain]
  2. flex        [Live]
```

Unless the configured targets prove the first value unnecessary, S2 returns
`NoChange`. Both origins remain available to
[S4 fallback planning](./s4-ast-reification-planning.md#example-4-ordered-fallback-chain).

### Example 4: normal and important phases do not share a history

Input:

```css
a {
  color: red !important;
  color: blue;
}
```

Importance participates in cascade phase identity. The later normal
declaration cannot kill the earlier important declaration:

```text
important history: color:red  [Live]
normal history:    color:blue [Live]
```

S2 does not compare them as one simple property chain.

### Example 5: opaque shorthand prevents an unsafe edit

Input:

```css
a {
  margin: var(--space);
  margin-left: 2px;
}
```

The shorthand origin cannot be expanded into known component values:

```text
origin#1 = Opaque(affected = all margin longhands)
origin#2 = Exact(margin-left)
```

S2 knows the declarations may interact, but cannot derive a lossless
replacement for the unaffected shorthand components. It returns `NoChange`
for the unsafe partial edit.

### Example 6: logical and physical aliases remain conservative

Input:

```css
a {
  margin-left: 1px;
  margin-inline-start: 2px;
}
```

Without a proof for writing mode and direction, S2 cannot say the second
declaration always overrides the first. Metadata records a possible alias, so
the resolver retains both effects.

### Example 7: `all` has exceptions

Input:

```css
a {
  color: red;
  --theme: dark;
  direction: rtl;
  all: initial;
}
```

The `all` occurrence may reset `color`, but not the custom property,
`direction`, or `unicode-bidi`. Its wildcard effect therefore has explicit
exceptions:

```text
color       -> may become Dead
--theme     -> remains Live
direction   -> remains Live
all         -> remains Live
```

If the resolver cannot encode the exact affected set, it returns `NoChange`.

### Example 8: cross-structure history without structural merge

Input:

```css
a {
  color: red;
}

@media (width >= 600px) {
  b {
    color: black;
  }
}

a {
  color: blue;
}
```

The conditional wrapper blocks a local S1/S3 edge in the outer list. Because a
supported conditional wrapper does not itself split the global history
segment, the two outer `a` entries share an S2 history and S2 may prune the
earlier color. S1 still cannot coalesce the rules structurally.

### Example 9: final effect removal changes adjacency

Logical input:

```text
previous -> rule#emptying -> next
rule#emptying.effects = [effect#1 Live]
rule#emptying.retained_child_count = 0
```

After S2 marks `effect#1` dead:

```text
rule#emptying = LogicallyEmpty
previous -> next
new edge(previous, next) = classified
```

If the new endpoints have equal effective rule keys, the edge jumps to
[S1](./s1-same-selector-coalescing.md#example-6-s2-exposes-a-new-s1-edge).
If their selectors differ, it jumps to
[S3](./s3-selector-partial-factoring.md#example-5-an-empty-endpoint-exposes-a-new-candidate).

## Effects on other stages

### Effect on S1

Removing the final live effect can unlink a rule and expose a new local edge.
That edge is classified immediately; an equal-key edge is added to
`dirty_same_selector_edges`. See
[S1 edge states](./s1-same-selector-coalescing.md#edge-states).

### Effect on S3

Every affected sequence revision changes, invalidating candidates that
snapshot the old revision. S3 recomputes the common sequence only after the
history is stable. See
[S3 candidate stabilization states](./s3-selector-partial-factoring.md#candidate-stabilization-states).

### Effect on S4

S2 supplies live-effect masks, authored origins, owner identity, and typed
effect data. S4 chooses how to represent that state without changing semantic
liveness or source order. See
[S4 declaration input states](./s4-ast-reification-planning.md#declaration-input-states).

### Effect on S5

S5 never interprets effect relationships. It only applies the S4 declaration
plan derived from S2's stable output. See
[S5 declaration commit states](./s5-ast-reification-commit.md#declaration-commit-states).

## Invariants

- Only entries with an exactly equal `EffectiveRuleKey` share a history.
- Histories are processed in semantic source order.
- Importance, prefix, compatibility, and fallback position are semantic input.
- The effect index is an ordered multimap.
- Known shorthands are expanded virtually, not eagerly in the AST.
- Opaque and wildcard effects conflict conservatively.
- Unknown property relationships default to `NoChange`.
- Every applied edit is proven reifiable from retained typed effects and
  origins, but S2 does not choose the representation.
- Authored or synthesized origins remain attached until S5.
- Every semantic edit increments all dependent revisions and generations.
- Incident S3 candidates are invalidated.
- A newly empty endpoint is unlinked before S4 and newly exposed edges are
  classified immediately.
- S2 reaches a local fixed point before its history is marked consumed.

## Completion condition

One history is complete when its `generation` equals
`consumed_generation`. S2 is globally idle when `dirty_histories` is empty and
all histories are consumed.

S2 may become dirty again after [S1 changes sequence ownership](./s1-same-selector-coalescing.md#effect-on-s2)
or [S3 inserts a synthesized history occurrence](./s3-selector-partial-factoring.md#effect-on-s2).
S4 cannot start until all such work is stable.
