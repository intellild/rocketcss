# S5: terminal commit and cleanup

## Document map

- [Overall design](./overall.md)
- [S1](./s1-same-selector-coalescing.md)
- [S2](./s2-declaration-effect-pruning.md)
- [S3](./s3-selector-partial-factoring.md)
- [S4](./s4-ast-reification-planning.md)
- [Radix AST IR](../flat-ast-ir/README.md)

## Responsibility

S5 is the one-way terminal boundary after the S1-S4 fixed point. In the target
Radix AST it is not a mandatory fresh-store rebuild and does not restore
semantic order. Authored primary nodes and synthesized sibling nodes already
occupy their final positions.

S5:

- materializes any declaration representation deferred by S4;
- finishes planned unlink/removal not already committed locally;
- verifies topology, owners, EffectiveKeys, and Radix order;
- clears histories, summaries, queue membership, revisions, and retired
  pass-local relationships; and
- optionally compacts tombstones/overflow lists when measurement justifies it.

S5 makes no semantic, selector-union, movement, compatibility, or profitability
decision.

## Input state

```text
S1-S4 scheduler fixed point       = true
all history generations consumed = true
all S4 plans complete/current     = true
all synthesized AST IDs final     = true
state.committed                   = false
```

If a plan is incomplete or stale, S5 does not partially finalize. The affected
S4 item or semantic dependency is re-enqueued.

## Visibility before S5

The live Radix AST already contains committed S1-S3 structural changes:

- synthesized S3 rules/blocks are addressable at final IDs;
- locally retired endpoints are absent from live topology;
- block-owned EffectiveKeys are current; and
- affected histories/edges have reached a fixed point.

The AST may still contain tombstoned declarations, retired storage needed by a
deferred plan, or a declaration block whose final representation is pending.

## Declaration commit states

For each S4 plan:

| Plan           | S5 action                                                        |
| -------------- | ---------------------------------------------------------------- |
| `ReuseOrigins` | Point at/coalesce an exact range or copy origins in exact order. |
| `Materialize`  | Build the listed typed declarations and importance bits.         |
| `Mixed`        | Interleave retained origins and typed replacements exactly.      |
| Empty          | Store an empty list; owner may remain for children.              |

The final block chooses `Range`, `Local4`, or complete `Overflow` according to
S4. S5 never lets a compact range absorb a foreign live declaration.

## Structural cleanup

Planned removals are applied post-order so child ownership remains valid. A
retired rule ID is not reused. Sparse sibling groups may remain allocated until
the compiler arena is dropped; removing every empty Radix page is optional and
must not slow the common pass.

Local topology after cleanup must satisfy:

- first/last list endpoints resolve;
- previous/next links are mutual;
- every live child names the correct parent/list;
- no live edge skips a retained barrier; and
- every live rule-owned block points back to that owner.

## Optional compaction

Compaction is not required for correctness or ordering. It may:

- remove declaration tombstones;
- shrink oversized overflow lists;
- discard empty sparse sibling groups from lookup sidecars; or
- merge other pass-local slack.

Do not renumber primary IDs. A local sibling relabel is allowed only through the
normal exact-remap transaction. Measure codegen, memory, and compaction cost
before enabling any cleanup globally.

## Examples

### Pruned declaration

```css
a {
  color: red;
  color: blue;
}
```

S2 proves `color:red` dead and S4 chooses the retained origin. S5 materializes
or points at:

```css
a {
  color: blue;
}
```

### Example 2: committing an S1 owner

For adjacent equal-selector rules, S1 has already selected the right rule as
live owner and unlinked the left endpoint. S5 finishes the chosen declaration
list and drops the retired relationship. No `previous_merged` chain remains for
codegen.

### Synthesized-rule commit

S3 has already inserted:

```css
a,
b {
  color: red;
}
```

at its final sibling ID. S5 only completes its declaration representation and
verifies topology; it neither repositions the rule nor recalculates selectors.

## Ownership and storage output

Every retained declaration sequence has exactly one live AST owner. Retired S1
shells and `previous_merged` adapters are absent from codegen-visible topology.
Synthesized S3 nodes retain the IDs assigned during their atomic commit.

## Output state

```text
live Radix AST exactly represents stable effects
every live block has one owner and current EffectiveKey
all pending declaration plans committed
all work queues and histories dropped
no merge-only relationship observable by codegen
state.committed = true
```

Codegen traverses ordinary rule topology and Radix store segments. It does not
read Nano state or make declaration-effect decisions.

## Invariants

- S5 starts only at the complete fixed point.
- S5 makes no new semantic or profitability choice.
- Synthesized allocation IDs already equal final semantic positions.
- Primary IDs remain stable.
- Cleanup creates no new S1-S4 work.
- Optional compaction is byte-equivalent to uncompacted output.
- Running minify again is idempotent.
