# S4: lossless representation planning

## Document map

- [Overall design](./overall.md)
- [S1](./s1-same-selector-coalescing.md)
- [S2](./s2-declaration-effect-pruning.md)
- [S3](./s3-selector-partial-factoring.md)
- [S5](./s5-ast-reification-commit.md)
- [Radix AST IR](../flat-ast-ir/README.md)

## Responsibility

S4 chooses a lossless declaration representation for effect liveness already
proved by S2. It does not decide liveness, selector unions, movement safety, or
topology.

The current AST stores declarations as linked chains owned directly by
`DeclarationBlockRecord`. Rules and blocks already have stable IDs and final
semantic positions, so S4 never chooses a `Range`, `Local4`, `Overflow`, or a
new insertion position. The authored declaration origin is the insertion
position.

```text
S2 updates typed live-effect masks
       ↓
partially live box shorthand enters dirty_s4_plan_items
       ↓
S1-S3 queues reach a fixed point
       ↓
S4 snapshots owner/block/effect revisions
       ↓
S5 replaces the shorthand at its origin
```

## Supported effect domain

The first implemented relational domain is typed physical `margin` and
`padding`:

- shorthand effects use a four-bit top/right/bottom/left mask;
- physical longhands use the corresponding single bit;
- normal and important declarations use independent histories;
- logical properties, `all`, CSS-wide forms, variables, unparsed values, and
  nested-declaration boundaries clear the relevant proof history; and
- other property families retain the existing exact-only behavior.

These barriers produce `NoChange`; S4 never converts an opaque occurrence into
a typed declaration.

## Plan state

```rust,ignore
enum AstDeclarationPlanKind {
    MaterializeBoxLonghands {
        family: BoxFamily,
        live_effects: u8,
    },
}

struct AstDeclarationPlan<'ast> {
    origin: DeclarationId<'ast>,
    owner: DeclarationBlockId<'ast>,
    block_revision: u32,
    effect_revision: u32,
    important: bool,
    kind: AstDeclarationPlanKind,
}
```

`dirty_s4_plan_items` is deduplicated by declaration origin. A queued item may
become fully dead before S4 runs; such an item is discarded because S2 already
tombstoned it. An all-live origin is reused and needs no deferred plan.

For a partially live typed shorthand, S4 records the live mask and current
dependencies. It does not extract values or allocate AST declarations. Values
remain owned by the authored shorthand until terminal commit.

## Representation follow-up

S1 concatenation and S5 one-to-many replacement can expose new block-local
shorthand opportunities. Those owner blocks are recorded in a deduplicated
representation-dirty list. After S5, the existing local declaration minifier
runs only for those blocks. This can recombine four adjacent live physical
longhands while preserving non-adjacent authored origins.

For example:

```css
a { margin-top: 1px; margin-right: 2px }
a { margin-bottom: 3px; margin-left: 4px }
```

S1 first creates one declaration chain. The block-local follow-up then emits:

```css
a { margin: 1px 2px 3px 4px }
```

For a non-adjacent partial override, the earlier shorthand remains at its
original rule and S4 plans only its surviving longhands:

```css
.a { margin: 1px }
.middle { display: block }
.a { margin-left: 2px }
```

becomes:

```css
.a { margin-top: 1px; margin-right: 1px; margin-bottom: 1px }
.middle { display: block }
.a { margin-left: 2px }
```

## Completion and invariants

S4 is complete when its dirty queue is empty and every stored plan has current
owner, block, effect, and importance snapshots.

- S4 never changes semantic liveness or AST topology.
- Every replacement is typed and derived from the authored shorthand.
- Replacement order is top, right, bottom, left.
- Fallback and opaque occurrence order remains unchanged.
- No plan stores a semantic insertion position.
- S5 can apply every plan without a semantic or profitability decision.
