# Cross-rule declaration merging: overall design

## Document map

- [Overall design](./overall.md): goals, semantic model, stage order, and
  correctness boundaries.
- [Non-goals](./non-goal.md): deliberately unsupported transformations and
  deferred implementation choices.
- [Detailed state machine](./detailed-state-machine.md): state ownership,
  transitions, invalidation, fixed-point rules, and regression requirements.
- [Pseudocode](./pseudo-code.md): typed keys, discovery, scheduling, and commit
  algorithms.

## Status

This is a correctness-first design for merging and pruning declarations across
style rules. Runtime, allocation, and hashing optimizations are intentionally
deferred.

The pass operates after selector minification and canonicalization, but before
code generation. All merge-state mutation must finish before code generation
follows RocketCSS's pinned `previous_merged` declaration chains.

## Goals

The pass should:

1. coalesce adjacent rules with exactly equal effective rule keys;
2. remove declarations that are provably dead across rules with exactly equal
   effective rule keys;
3. factor common declarations from adjacent rules with different selectors in
   the same structural and conditional context;
4. apply the same declaration analysis to native CSS nesting without flattening
   the authored style-rule hierarchy;
5. compare rules inside conditional at-rules only when their complete typed
   at-rule context stacks are structurally equal;
6. coalesce adjacent conditional blocks with equal typed frames;
7. remove nodes that become logically empty without losing opaque or retained
   descendants; and
8. reach an idempotent fixed point without changing cascade behavior.

Declaration order, fallback chains, importance, vendor prefixes, selector
validity, source origin, and lossless serialization are correctness inputs. An
unproven transformation is not performed.

## Core identity model

### Selector state

Effective-selector resolution is fallible:

```rust,ignore
enum EffectiveSelectorResult<'ast> {
    Live(EffectiveSelectorKey<'ast>),
    KnownNoMatch,
    OpaqueBarrier,
}
```

- `Live` provides an immutable selector-semantic identity.
- `KnownNoMatch` means the style rule and its entire subtree do not serialize.
- `OpaqueBarrier` preserves the authored subtree but gives it no cross-rule
  identity.

The resolver must handle or conservatively reject:

- explicit and implicit nesting selectors;
- leading combinators;
- multiple or recursively nested `&` occurrences in selector functions;
- compound-position restrictions;
- pseudo-element parents and chaining;
- partially tombstoned selector lists;
- recovered or unparsed selectors; and
- CSS Modules local/global context.

A parsed selector AST is not by itself proof that an effective selector key is
valid. Any earlier selector pass that blindly tombstones recovered syntax must
be corrected before this feature is enabled.

### Parent identity

Leading declarations and `NestedDeclarationsRule` entries use exact parent
matching semantics. Parent matching is different from an explicit `&` child:
an explicit `&` cannot represent pseudo-element arms from the parent selector
list.

For example, these entries must not share a history merely because both can be
described informally as matching the parent:

```css
.item,
.item::before {
  & {
    color: red;
  }

  color: blue;
}
```

### Conditional at-rule context

The initial design supports typed conditional frames for:

- `@media`, including its complete media query list;
- `@supports`, including its complete supports condition; and
- `@container`, including its optional name and complete query.

The ordered context stack is part of rule identity:

```rust,ignore
enum ConditionalAtRuleFrameKey<'ast> {
    Media(MediaQueryListKey<'ast>),
    Supports(SupportsConditionKey<'ast>),
    Container {
        name: Option<ContainerNameKey<'ast>>,
        query: ContainerQueryKey<'ast>,
    },
}

struct ConditionalAtRuleContextKey<'ast> {
    frames: Vec<ConditionalAtRuleFrameKey<'ast>>,
}

struct DeclarationHistoryContextKey<'ast> {
    at_rules: ConditionalAtRuleContextKey<'ast>,
    layer: LayerContextKey,
    origin: CascadeOriginKey,
    phase: CascadePhaseKey,
}

struct EffectiveRuleKey<'ast> {
    history_segment: HistorySegmentId,
    context: DeclarationHistoryContextKey<'ast>,
    selectors: EffectiveSelectorKey<'ast>,
}
```

Equality is typed structural equality after any earlier canonicalization.
Boolean equivalence, implication, overlap, union, and condition reordering are
not inferred.

Stack order and multiplicity are significant:

```text
[media(A), supports(B)] != [supports(B), media(A)]
[container(A), container(A)] != [container(A)]
```

Repeated nested `@container` frames are not collapsed because each frame may
select a different ancestor container.

### Cascade context

Layer, cascade origin, and cascade phase are declaration-history inputs. Two
declarations may share a history only when all three keys are exactly equal.
The merge pass treats these as opaque structural keys: it does not calculate
layer order, equate separately authored layer blocks by name, move declarations
between layers or origins, or infer relationships between cascade phases.

Within the current author-stylesheet API, the origin key is `Author`. Normal
and important declarations have distinct phase keys. Each authored `@layer`
block receives a distinct traversal identity, including when two blocks use the
same name; recognizing those blocks as one semantic layer is deferred at-rule
work.

### Structural and emission identity

`EffectiveRuleKey` determines semantic history membership. S1 and S3 require
additional structural equality:

```rust,ignore
struct EmissionIdentity {
    wrapper_kind: StyleWrapperKind,
    vendor_prefix: VendorPrefix,
    selector_serialization_context: SelectorSerializationContextKey,
}
```

An S1/S3 edge also requires:

- the same `RuleListId`;
- the same `RuleListSegmentId`;
- equal conditional context;
- equal emission identity; and
- no retained child content on the left endpoint.

Ordinary style rules and legacy `@nest` wrappers have different wrapper kinds.
The initial implementation treats `@nest` as a retained barrier.

## Nesting model

The AST remains nested. Effective selectors are calculated as immutable
semantic keys while every style rule remains owned by its original child list.

Selector resolution is an AST operation, not textual Cartesian expansion:

```css
#a,
b {
  & c {
    color: red;
  }
}
```

The inner selector has the semantics and specificity of `:is(#a, b) c`, not an
ordinary `#a c, b c` selector list.

S2 may compare declaration entries at different nesting depths when their full
effective rule keys match. S1 and S3 remain local to one rule-list segment.
Neither stage mutates an existing local selector or ancestor selector.

When S3 synthesizes a nested rule, it inserts the new rule in the same child
list, materializes a new local selector union, and resolves that union in the
same immutable parent and conditional contexts.

## Adjacency and barrier model

Two independent segment types are used:

- `HistorySegmentId` controls global S2 history. Opaque or unsupported nodes
  end the current history segment.
- `RuleListSegmentId` controls local S1/S3 adjacency. Every retained
  non-endpoint sibling ends the current local segment.

Local barriers include:

- a live `NestedDeclarationsRule`;
- a supported conditional block as seen from its containing list;
- an opaque or unsupported at-rule;
- legacy `@nest`; and
- any other retained node that is not an S1/S3 style-rule endpoint.

“Not an endpoint” never means an edge may skip over the node.

Supported conditional blocks have their own child rule lists. Adjacent blocks
with equal typed frames may first be coalesced into one logical conditional
region by concatenating their child lists in source order:

```css
@media (width >= 600px) {
  a {
    x: 1;
  }
}
@media (width >= 600px) {
  a {
    y: 2;
  }
}
```

This may become one logical media region, after which S1 can coalesce the two
`a` rules. Non-adjacent equal blocks remain structurally separate; their
declarations may share an S2 history, but they do not form an S1/S3 edge.

## Stage order

`S` means stage. The logical order is:

1. **S1: same-selector coalescing**
2. **S2: exact-effective-rule declaration pruning**
3. **S3: selector partial factoring**
4. **S4: empty-node cleanup**

S1 and S2 always have priority over S3 candidate commit. S3 plans may be
discovered early, but remain speculative until all histories capable of
changing their endpoints are stable.

### S1: same-selector coalescing

S1 combines live-adjacent rules with equal effective rule keys and emission
identities.

RocketCSS keeps the right rule as the active output owner. The left
declaration sequence is linked before the right sequence through
`previous_merged`, and the left endpoint is retired from adjacency:

```text
left declaration blocks
right leading declaration blocks
right child sequence
```

The left endpoint must have no retained child content. The right endpoint may
retain children because it remains the owner and its children stay after the
combined declarations.

Histories contain ordered declaration entries, not one entry per surviving
style-rule owner. S1 therefore preserves the semantic source-order keys of both
sequences.

### S2: exact-effective-rule declaration pruning

S2 processes all declaration entries with one exact `EffectiveRuleKey` in
semantic source order. It only applies typed, lossless declaration edit plans.

The resolver considers:

- typed `PropertyId` and vendor prefix;
- importance;
- target and fallback compatibility;
- shorthand/longhand relationships;
- logical/physical property interactions;
- `all`, including its exceptions;
- variables and case-sensitive custom properties;
- `revert` and `revert-layer`; and
- unknown or recovered syntax.

The resolver returns a concrete edit plan:

```rust,ignore
enum DeclarationResolution<'ast> {
    NoChange,
    Apply(DeclarationEditPlan<'ast>),
}
```

A partial shorthand override may replace the shorthand with still-live typed
longhands only when the replacement is lossless. Variables, recovered syntax,
unknown values, and unproven fallback behavior produce `NoChange`.

Each history is processed to a local fixed point before S3 can commit.

### S3: selector partial factoring

S3 factors a safe common declaration sequence from two live-adjacent rules:

```text
SL { DL }
SR { DR }

becomes

SL    { left_only }
SL,SR { common }
SR    { right_only }
```

Complete declaration equality is the degenerate case where both residual
sequences are empty.

Validity requires:

1. equal declaration occurrences, including importance and prefix;
2. preserved declaration order and fallback behavior;
3. unchanged behavior for elements matching either or both selector lists;
4. safe shorthand, logical/physical, variable, and `all` dependencies;
5. valid target-compatible selector union;
6. immediate selector filtering, normalization, and deduplication;
7. source origin for every selector arm and moved declaration; and
8. no retained child content on the left endpoint.

Endpoint selector ASTs are immutable and cannot be moved. Commit requires an
arena-aware deep materialization of the selector union. If materialization or
validation is unavailable, S3 is disabled.

### S4: empty-node cleanup

S4 removes a selector-live style rule only when it has:

- no live declarations;
- no retained child content; and
- no declaration block referenced by another active output owner's merged
  sequence.

Cleanup is post-order. It also removes:

- selector-`KnownNoMatch` subtrees;
- empty `NestedDeclarationsRule` nodes; and
- supported conditional blocks whose complete child list is empty.

Opaque or unsupported nodes are never declared empty by this pass. Any retained
opaque content pins every style-rule ancestor.

## Candidate requirement

S3 cannot mutate rules while S2 may still change either endpoint:

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

The first edge initially appears fully factorable. If it were committed
immediately, later S2 processing would need to remove `color:red` for `a` while
retaining it for `b`, which would require splitting the synthesized selector
list.

Instead, the edge stores a candidate with both aggregate declaration-sequence
revisions. Any endpoint change invalidates and recomputes the candidate.

## Global correctness invariants

- Existing local selectors, ancestor selectors, and at-rule conditions are
  immutable.
- S2 tombstones or losslessly replaces declarations but never moves rules.
- S1 and S3 never cross a rule-list segment.
- Histories are ordered by semantic source position, not discovery time.
- Editing any block in a merged sequence increments its aggregate revision.
- Every retained non-endpoint node prevents an edge from skipping over it.
- S3 commit is one atomic live-graph transition.
- Synthesized selectors are canonicalized during the same pass.
- Dirty queues are hints; consumers revalidate current eligibility.
- The pass stops only at a complete work-queue and history-generation fixed
  point.

Implementation details and unsupported transformations are listed in
[Non-goals](./non-goal.md). State ownership and transitions are specified in
[Detailed state machine](./detailed-state-machine.md).
