# S5: terminal commit and cleanup

## Document map

- [Overall design](./overall.md)
- [S1](./s1-same-selector-coalescing.md)
- [S2](./s2-declaration-effect-pruning.md)
- [S3](./s3-selector-partial-factoring.md)
- [S4](./s4-ast-reification-planning.md)
- [Radix AST IR](../flat-ast-ir/README.md)

## Responsibility

S5 is the one-way boundary after the S1-S4 fixed point. Rules, blocks,
EffectiveKeys, and synthesized S3 nodes already occupy their final AST
positions. S5 only validates and commits deferred declaration representations,
then consumes all merge-only sidecars.

S5 makes no liveness, selector, movement, compatibility, or profitability
decision.

## Preconditions

```text
same-selector queue empty
declaration-history queue empty
partial-factor queue empty
dirty_s4_plan_items empty
every plan dependency revision current
all additional declaration capacity available
```

All plans are validated before the first mutation. S5 verifies:

- the owner block is live and its revision matches the snapshot;
- the declaration origin still belongs to that owner;
- the effect revision and live mask are current;
- the origin's importance matches the plan; and
- the declaration store can allocate the sum of all additional records.

If validation or capacity preflight fails, no plan is committed.

## Declaration-chain transaction

`RadixCompilation::rewrite_declaration_with_sequence` replaces one origin with
a nonempty ordered sequence. It:

1. validates the complete owning chain and origin membership;
2. preflights count overflow and all additional records;
3. transfers the non-`Clone` authored payload to an infallible rewrite
   callback;
4. reuses the origin ID for the first replacement;
5. links the replacement tail to the origin's former successor;
6. updates first/last/count as required; and
7. advances the block revision once.

For a partial `margin` or `padding` shorthand, the callback moves only the live
top/right/bottom/left values into typed longhands and copies the authored
importance bit to every emitted declaration.

The transaction works for first, middle, and last origins and for declaration
chains that became non-contiguous after structural merging. Capacity failure
leaves payloads, links, importance, counts, and revisions unchanged.

## Terminal flow

```text
validate every S4 snapshot
       ↓
sum and preflight declaration growth
       ↓
rewrite each origin through the declaration-chain transaction
       ↓
debug validate_ast()
       ↓
drop histories, queues, revisions, and plans
       ↓
rerun local declaration representation only for dirty blocks
```

The final local representation pass is outside semantic stabilization. It may
recombine already equivalent physical longhands but cannot enqueue S1-S4 work.

## Output state and invariants

```text
ordinary Radix AST losslessly represents stable effects
every live declaration block has one owner and current EffectiveKey
all deferred plans are committed
all merge-only state is consumed
codegen reads no Nano sidecars
```

- S5 starts only at the complete fixed point.
- Primary declaration origins remain stable for the first replacement.
- Generated declarations stay at the authored origin position.
- Every generated declaration preserves importance.
- S5 creates no new S1-S4 work.
- `Compilation::validate_ast()` succeeds after commit.
- Running minify again is byte-idempotent.
