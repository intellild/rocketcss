# Cross-rule declaration merging design

## Status

This document describes a correctness-first design for merging and pruning
declarations across style rules. It intentionally leaves runtime and memory
optimizations to a later implementation design.

S1 and S3 apply independently to each sibling rule list. S2 uses one global
effective-rule history over the source-ordered style-rule traversal. Supported
conditional at-rules contribute an immutable, ordered context key. Entries
participate in the same history only when both their effective selector lists
and their complete conditional at-rule context stacks are structurally equal.
Unsupported at-rules remain history and traversal barriers.

Native CSS nesting remains represented in the AST. While building the IR, the
pass resolves each nested selector against its ancestor selector context and
stores an immutable effective-selector key. It does not flatten nested rules,
rewrite ancestor selectors, or move synthesized rules out of their owning
sibling list.

## Goals

The pass should:

1. coalesce adjacent rules with exactly equal effective rule keys;
2. remove declarations that are provably dead across rules with exactly equal
   effective rule keys;
3. factor common declarations out of adjacent rules with different selectors
   in the same conditional at-rule context;
4. apply the same declaration analysis to nested rules by using their resolved
   effective selectors;
5. apply the same analysis inside structurally equal conditional at-rule
   contexts;
6. coalesce adjacent conditional blocks whose at-rule kind, complete prelude,
   and surrounding at-rule context are structurally equal;
7. preserve authored style-rule nesting while removing rules that become
   logically empty; and
8. reach an idempotent fixed point without changing CSS cascade behavior.

The pass must preserve declaration order, fallback chains, importance, vendor
prefixes, and selector validity. If either declaration dominance or selector
compatibility cannot be proven, the authored form is preserved.

## Non-goals

The initial design does not:

- compare rules whose complete conditional at-rule contexts differ;
- infer condition equivalence, implication, overlap, union, or negation;
- reorder condition terms or commute nested at-rule frames;
- lift a style rule into or out of a conditional at-rule;
- merge non-adjacent conditional at-rule blocks;
- collapse repeated nested `@container` frames, even when their text is equal;
- model non-conditional cascade contexts such as `@layer`, `@scope`, or
  `@starting-style`;
- merge name-defining, descriptor, or global at-rules such as `@keyframes`,
  `@font-face`, `@property`, `@page`, `@import`, or `@namespace`;
- flatten nesting or lift nested rules into an ancestor sibling list;
- create S1 or S3 edges across different sibling rule lists;
- mutate an existing rule's local or ancestor selectors during S1, S2, or S3;
- split one selector list into independently owned selector arms;
- merge non-adjacent rules with different selectors;
- infer equivalence from reordered selector lists; or
- specify the final data-layout, hashing, allocation, or work-queue
  optimizations.

## Terminology

- **Local selector list**: the selector list stored on a `StyleRule`. For a
  nested rule it may contain `&` or an implicit relative selector.
- **Effective selector list**: the immutable selector semantics obtained by
  resolving a local selector list through its ancestor selector context. It
  preserves nesting specificity and matching behavior rather than performing
  a textual Cartesian expansion.
- **Parent identity**: the effective-selector identity used by a parent rule's
  leading declarations and its `NestedDeclarationsRule` entries. It preserves
  the parent's exact matching behavior, including pseudo-elements, and is not
  interchangeable with an explicit `&` rule.
- **Live rule**: a rule that still has live declarations or at least one live
  nested descendant.
- **Live adjacency**: adjacency after logically empty rules are ignored, even
  before physical cleanup. Adjacency never crosses a sibling-list boundary.
- **Exact effective selector list**: structurally equal effective selectors in
  the same order and selector context. `a,b` is not equal to `b,a` or to `a`.
- **Conditional at-rule frame**: one typed `@media`, `@supports`, or
  `@container` frame. Its key contains the at-rule kind and complete parsed
  prelude; a container frame includes both its optional name and query.
- **Conditional at-rule context**: the ordered stack of conditional at-rule
  frames surrounding a declaration-bearing entry. Structural equality is
  required. The stack `[media(A), supports(B)]` is different from
  `[supports(B), media(A)]`, and `[container(A), container(A)]` is different
  from `[container(A)]`.
- **Effective rule key**: one exact effective selector list together with one
  exact conditional at-rule context and the current unsupported-at-rule
  barrier segment.
- **Effective-rule history**: all declaration-bearing IR entries with one
  effective rule key, ordered by semantic source position. Histories may
  contain entries from different nesting depths and distinct occurrences of
  the same conditional block, but never cross an unsupported-at-rule barrier.
- **Conditional region**: a sibling-list region owned by one conditional
  at-rule block. Two adjacent blocks may be coalesced into one region only when
  their typed frames and surrounding conditional contexts are structurally
  equal.
- **Edge**: one pair of live-adjacent style rules in one sibling rule list. The
  edge therefore also identifies the list and insertion position for an S3
  result.
- **Candidate**: a speculative selector partial-merge plan attached to an
  edge. A candidate does not mutate the AST until it is committed.

## Stage order

`S` means **Stage**. The stages execute in this logical order:

1. **S1: same-selector coalescing**;
2. **S2: exact-effective-rule declaration pruning**;
3. **S3: selector partial factoring**; and
4. **S4: empty-rule cleanup**.

S3 is speculative while S2 is still capable of changing either endpoint.
Candidates may be discovered early, but they are not committed until all S2
histories that can affect them are stable.

## Conditional at-rule model

The first implementation supports only conditional group rules with typed AST
representations:

- `@media`, keyed by its complete media query list;
- `@supports`, keyed by its complete supports condition; and
- `@container`, keyed by its optional container name and complete query.

Equality is structural equality after the parser and any earlier
canonicalization pass. Whitespace and comments are not part of the condition,
but Boolean equivalence is not inferred. For example, reordered media queries,
rewritten range syntax, or reordered `and` operands are equal only if an
earlier canonicalization pass has already produced equal typed ASTs.

The context is an ordered stack:

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

struct EffectiveRuleKey<'ast> {
    barrier_segment: BarrierSegmentId,
    at_rules: ConditionalAtRuleContextKey<'ast>,
    selectors: EffectiveSelectorKey<'ast>,
}
```

The at-rule stack is semantic context, not part of the selector. Selector
resolution and condition identity therefore remain independently typed.

IR discovery pushes a supported frame before visiting its child rule list and
pops it afterward. It never distributes, combines, removes, or reorders stack
frames. In particular, identical nested `@container` frames remain two frames
because each query may select a different ancestor container.

Adjacent conditional blocks in the same owning sibling list may be coalesced
when their typed frame keys are exactly equal. Coalescing concatenates their
child rule lists in source order and creates one logical sibling list before
S1/S3 edge discovery:

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

may first be viewed as:

```css
@media (width >= 600px) {
  a {
    x: 1;
  }
  a {
    y: 2;
  }
}
```

S1 can then coalesce the two `a` rules. This exact-block coalescing is part of
IR region construction, not a new declaration-merging stage. It is limited to
adjacent blocks, so it does not need a rule-movement or insertion-point proof.

Non-adjacent occurrences of an equal conditional context remain separate AST
blocks. Their declarations may join the same S2 effective-rule history because
S2 only tombstones declarations and does not move rules. They do not form an
S1 or S3 edge across their separate sibling lists.

All other at-rules are unsupported context in this design. Encountering one
creates a new `BarrierSegmentId`; declarations on opposite sides cannot share
an effective-rule history. Their children may still be traversed by a future
at-rule-specific design, but this pass makes no cross-boundary inference.

## Nesting IR construction

Selector minification and canonicalization run before this pass. Local and
ancestor selectors are immutable for the lifetime of the merge state.

The AST remains nested. IR construction walks style rules in semantic source
order while carrying the effective selector of the nearest ancestor style
rule. Resolving a nested selector is a selector-AST operation, not string
concatenation:

- an explicit `&` represents the parent selector list with the specificity
  behavior of `:is(parent)`;
- an implicit relative selector receives the implied nesting selector and
  combinator;
- multiple nesting levels resolve recursively; and
- invalid or recovered selectors do not receive a cross-rule identity and
  remain conservative barriers.

For example, the effective selector of the inner rule is `:is(#a, b) c`, not
the textual Cartesian expansion `#a c, b c`:

```css
#a,
b {
  & c {
    color: red;
  }
}
```

The `:is()` shape matters because the nesting selector uses the largest
specificity in the parent selector list for every match.

Leading declarations on a `StyleRule` use the exact effective identity of that
style rule. Declarations which occur after a nested rule are represented by a
`NestedDeclarationsRule`; they use the same parent identity and retain their
position among child rules.

An explicit `&` child is different from a `NestedDeclarationsRule`. `&` cannot
represent pseudo-elements from the parent selector list, while nested
declarations match exactly the same elements and pseudo-elements as their
parent. The effective-selector key must preserve that difference:

```css
.item,
.item::before {
  & {
    color: red;
  }

  color: blue;
}
```

The red and blue declarations above must not be placed in the same history
merely by labeling both entries as `&`.

Conceptually, traversal emits declaration-bearing entries in this order:

```text
visit StyleRule
    register leading declarations with:
        EffectiveRuleKey(at-rule context, ParentMatch(effective selectors))
    for each child in source order
        nested StyleRule       -> recurse with the current effective selectors
        NestedDeclarationsRule -> register with ParentMatch(effective selectors)
        supported condition    -> push its typed frame and recurse
        unsupported at-rule    -> create a barrier segment
```

This is a logical IR traversal only. The child remains owned by its original
AST parent. Encountering an unsupported at-rule ends the current history
segment, blocks local adjacency, and assigns subsequent entries a new
`BarrierSegmentId`; the first implementation does not attempt to compare
declarations on either side of that barrier.

### Nesting and structural rewrites

S2 only tombstones declarations, so its global histories may contain entries
from different nesting depths and distinct conditional block occurrences when
their complete effective-rule keys match.

S1 and S3 remain local to one sibling list. A nested sibling edge carries both
forms of selector identity:

- effective selectors are used for semantic equality, histories, and cascade
  proofs; and
- local selectors are retained for serialization and selector union.

When S3 commits on nested siblings, the synthesized rule is inserted at that
edge in the same child-rule list. Its local selector list is the union of the
two endpoint local selector lists, and its effective selector is resolved in
the same immutable parent and conditional at-rule contexts. No ancestor
selector, endpoint selector, or at-rule condition is modified.

A left endpoint with live nested content is a forward structural barrier for
S1 and S3. Its declarations originally occur before its children; moving any
of them into a following shared or coalesced sibling rule would move them past
those children. A right endpoint may contain live nested content because the
resulting declarations still occur before those children.

`NestedDeclarationsRule` entries participate in S2 histories but do not form
S1 or S3 style-rule edges in the initial implementation.

### S1: same-selector coalescing

Two live-adjacent rules in the same sibling list with exact effective rule keys
are represented as one rule whose declaration sequence preserves source order.
The left rule must not have live nested content. Rules in the same sibling list
already have the same conditional at-rule context; checking the full key keeps
the invariant explicit.

```css
a {
  x: 1;
}
a {
  y: 2;
}
```

becomes:

```css
a {
  x: 1;
  y: 2;
}
```

S1 is safe to apply eagerly because it does not change declaration ownership:
all declarations still belong to the same exact selector and conditional
context. S1 does not modify the surviving rule's selector, any ancestor
selector, or any at-rule frame.

S1 is applied repeatedly across a same-key run before that rule is added to its
effective-rule history.

If an already registered rule is coalesced later because logical adjacency
changed, its old effective-rule history entry is replaced by the coalesced rule
and that history is marked dirty.

### S2: exact-effective-rule declaration pruning

S2 processes a complete effective-rule history as one source-ordered
declaration sequence. It may tombstone declarations in any entry in the
history, including entries at different nesting depths or in separate
occurrences of the same conditional block, but it does not move rules or
change local selectors, effective selectors, ancestor selectors, or at-rule
conditions.

```css
h1 {
  color: red !important;
}
.middle {
  display: block;
}
h1 {
  color: blue;
}
```

The second `color` is dead because the earlier known-valid important
declaration wins for the same selector:

```css
h1 {
  color: red !important;
}
.middle {
  display: block;
}
h1 {
}
```

S2 must use typed declaration semantics. A property name match alone is not a
proof of dominance. The decision includes at least:

- `PropertyId` and vendor prefix;
- declaration importance;
- target/value compatibility and fallback requirements;
- shorthand/longhand relationships;
- logical/physical property barriers;
- `all`;
- variables and custom properties; and
- unknown or recovered declaration syntax.

The declaration resolver returns a semantic outcome rather than an equality
boolean:

```rust,ignore
enum DeclarationResolution {
    PreviousDead,
    CurrentDead,
    BothLive,
    Fold,
}
```

Unknown cases produce `BothLive`.

### S3: selector partial factoring

S3 considers live-adjacent rules with different selector lists in one sibling
list. It factors one safe common declaration sequence into an independent
shared-selector rule inserted at that edge.

Both endpoints necessarily have the same complete conditional at-rule context.
S3 never synthesizes a rule across two separate at-rule blocks unless adjacent
equal blocks were already coalesced into one logical conditional region during
IR construction.

Given:

```text
SL { DL }
SR { DR }
```

a plan has this shape:

```text
SL    { left_only }
SL,SR { common }
SR    { right_only }
```

For example:

```css
h1 {
  color: red;
  text-align: right;
}
h2 {
  text-align: right;
  color: blue;
}
```

may become:

```css
h1 {
  color: red;
}
h1,
h2 {
  text-align: right;
}
h2 {
  color: blue;
}
```

Complete declaration equality is the degenerate partial-merge case:

```text
left_only  = empty
common     = the complete declaration block
right_only = empty
```

S3 does not compute a declaration set intersection. A plan is valid only when:

1. common declaration occurrences are equal, including importance and prefix;
2. their relative order is preserved;
3. moving them across residual declarations preserves cascade behavior;
4. shorthand, longhand, fallback, variable, and `all` dependencies remain
   valid;
5. behavior is unchanged for elements matching either or both selector lists;
6. the local-selector union and its effective resolution are valid for all
   configured targets and contexts; and
7. the left endpoint has no live nested content.

The size/profitability policy is separate from semantic validity and can be
specified when the implementation design is finalized.

### S4: empty-rule cleanup

S4 physically removes rules with no live declarations and no live nested
descendants. Logically empty rules stop participating in adjacency
immediately; physical cleanup is deferred until the candidate state is stable.

Cleanup is post-order. A parent cannot become logically empty until all of its
deep descendants have become logically empty. Therefore every ancestor used
to calculate a live descendant's effective selector remains present, and no
ancestor-selector revision or defensive selector copy is needed.

## Why candidates are required

S3 cannot mutate rules while future S2 updates remain possible. Consider:

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

The first two rules initially appear fully mergeable. Committing them early
would create:

```css
a,
b {
  color: red;
}
a {
  color: blue;
}
```

When the final `a` is processed, S2 needs to delete `color: red` for `a` but
retain it for `b`. That is impossible without splitting the prematurely merged
selector list.

Instead, the first edge records a candidate. When S2 later changes the first
`a`, its declaration revision changes and the candidate becomes stale. The
final result is:

```css
b {
  color: red;
}
a {
  color: blue;
}
```

Candidate invalidation alone is safe but incomplete. A changed endpoint may
still support a different plan, so its incident edges must also be marked dirty
and recomputed.

## Conceptual data structures

These structures define ownership and dependencies. Their final packed or
arena-backed representation is deferred.

```rust,ignore
type RuleId = u32;
type RuleListId = u32;
type DeclarationEntryId = u32;
type BarrierSegmentId = u32;
type Revision = u32;

struct MergeState<'ast> {
    lists: Map<RuleListId, RuleSequence<'ast>>,
    rules: RuleTable<'ast>,
    declaration_entries: DeclarationEntryTable<'ast>,
    histories: Map<EffectiveRuleKey<'ast>, EffectiveRuleHistory>,
    current_barrier_segment: BarrierSegmentId,
    candidates: Map<Edge, PartialMergeCandidate<'ast>>,
    dirty_histories: Set<EffectiveRuleKey<'ast>>,
    dirty_same_selector_edges: Set<Edge>,
    dirty_partial_edges: Set<Edge>,
}

struct RuleState<'ast> {
    rule: StyleRule<'ast>,
    owning_list: RuleListId,
    local_selectors: LocalSelectorListRef<'ast>,
    effective_selectors: EffectiveSelectorKey<'ast>,
    at_rule_context: ConditionalAtRuleContextKey<'ast>,
    leading_declarations: Option<DeclarationEntryId>,
    live: bool,
    has_live_nested_content: bool,
    previous_live: Option<RuleId>,
    next_live: Option<RuleId>,
}

struct DeclarationEntryState<'ast> {
    origin: DeclarationOrigin,
    effective_rule: EffectiveRuleKey<'ast>,
    declarations: DeclarationBlockRef<'ast>,
    live: bool,
    declaration_revision: Revision,
}

struct EffectiveRuleHistory {
    entries: Vec<DeclarationEntryId>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    list: RuleListId,
    left: RuleId,
    right: RuleId,
}

struct PartialMergeCandidate<'ast> {
    edge: Edge,
    left_declaration_revision: Revision,
    right_declaration_revision: Revision,
    plan: PartialMergePlan<'ast>,
}

struct PartialMergePlan<'ast> {
    left_only: DeclarationPlan<'ast>,
    common: DeclarationPlan<'ast>,
    right_only: DeclarationPlan<'ast>,
}
```

`EffectiveSelectorKey` represents the complete resolved selector semantics. It
never indexes individual arms of a multi-selector rule. Parent-match entries
preserve exact parent matching, while explicit or implicit nesting selectors
preserve their `:is()`-like specificity and pseudo-element restrictions.

Parent match is a selector-semantic mode, not the identity of one parent AST
node. Parent declarations from distinct rules may share a history when their
complete parent-match keys are structurally equal.

`EffectiveRuleKey` combines selector semantics with the complete ordered
conditional at-rule context and an unsupported-at-rule barrier segment. This
keeps rule identity global across nested style-rule depths and repeated equal
conditional blocks without joining entries whose conditions differ.

`DeclarationOrigin` distinguishes a style rule's leading declaration block, a
`NestedDeclarationsRule`, and a synthesized S3 rule. A style rule edge refers
to the revision of each endpoint's leading declaration entry. A nested
declaration entry has a revision for S2 history processing but never becomes an
S1 or S3 endpoint.

Local selectors, effective selectors, and conditional at-rule contexts have no
revision. Selector and at-rule canonicalization passes run before IR
construction, and S1, S2, and S3 never mutate them. S3 creates a new immutable
local/effective selector pair for its synthesized rule and inherits the owning
list's immutable conditional context.

The stored `StyleRule` and collection types above are conceptual. In RocketCSS,
pinned declaration blocks and stable rule identifiers or references should be
reused rather than introducing long-lived ordinary Rust references to mutable
style rules.

## Candidate validity

A candidate is valid only if:

```rust,ignore
fn candidate_is_valid(state: &MergeState, candidate: &PartialMergeCandidate) -> bool {
    let left = state.rules.get(candidate.edge.left);
    let right = state.rules.get(candidate.edge.right);
    let (Some(left_entry), Some(right_entry)) = (
        left.leading_declarations,
        right.leading_declarations,
    ) else {
        return false;
    };
    let left_declarations = state.declaration_entries.get(left_entry);
    let right_declarations = state.declaration_entries.get(right_entry);

    left.live
        && right.live
        && left_declarations.live
        && right_declarations.live
        && left.owning_list == candidate.edge.list
        && right.owning_list == candidate.edge.list
        && left.at_rule_context == right.at_rule_context
        && left.next_live == Some(candidate.edge.right)
        && right.previous_live == Some(candidate.edge.left)
        && !left.has_live_nested_content
        && left_declarations.declaration_revision
            == candidate.left_declaration_revision
        && right_declarations.declaration_revision
            == candidate.right_declaration_revision
}
```

Even a revision-valid cached plan may be revalidated immediately before commit
as a correctness guard.

## State transitions

### Declaration change

When S1 or S2 changes a leading declaration entry:

```text
increment declaration_revision
invalidate candidates on both incident edges
classify and mark both incident edges dirty
if the rule became logically empty, update live adjacency
```

When S2 changes a `NestedDeclarationsRule`, increment that entry's declaration
revision and propagate any subtree-liveness change toward its ancestors. It has
no incident S1 or S3 edge of its own.

Invalidating without marking dirty is conservative but misses optimizations.

For example:

```css
a {
  x: red;
  y: 1;
}
b {
  y: 1;
}
a {
  x: blue;
}
```

The initial `a`/`b` candidate has `x: red` as `left_only`. After S2 removes
`x: red`, that plan is stale. Recomputing the edge discovers complete equality
for `y: 1`.

### S3 shared-rule insertion

S3 does not change either endpoint selector. When it commits:

```text
tombstone the planned common declarations in the left and right entries
increment both endpoint declaration revisions
create an independent StyleRule at the candidate edge
give it the union of the endpoint local selector lists
resolve its immutable effective selector in the owning list's parent context
inherit the owning list's immutable conditional at-rule context
register its leading declaration entry in the effective-rule history
mark the endpoint and synthesized histories dirty
invalidate and classify every affected edge in the owning sibling list
```

The endpoint local selectors, all ancestor selectors, and the conditional
at-rule context remain unchanged. The new effective-rule history must be
processed by S2 before another candidate that depends on it is committed.

For example:

```css
a {
  x: 1;
}
b {
  x: 1;
}
a,
b {
  x: 2;
}
```

Committing the first candidate creates a new independent rule and a new `a,b`
history. S2 can then prove that the earlier `x: 1` is dead.

### Rule becomes logically empty

An empty rule is unlinked from the live adjacency chain immediately:

```text
invalidate (previous, empty) and (empty, next)
create and dirty (previous, next)
defer physical deletion to S4
```

If the empty rule is nested, update `has_live_nested_content` on its ancestors
in post-order. An ancestor is unlinked only after both its own declarations and
its entire descendant subtree are logically empty. If an ancestor loses its
last live nested child but keeps declarations, its incident edges are dirtied
because it may no longer be a forward structural barrier.

### Overlapping candidates

Two adjacent edges may share a rule:

```text
(a, b) and (b, c)
```

They cannot be committed from the same snapshots. Candidate commits use a
deterministic source-order policy. Committing `(a, b)` invalidates and dirties
all edges incident to the resulting rules before another candidate is chosen.

### Dirty edge processing

Every dirty live edge is classified again from current rule state:

```text
same exact effective rule key and S1 is structurally eligible
    -> apply S1 eagerly
    -> dirty the effective-rule history and affected edges

different selector lists and S3 is structurally eligible
    -> remove any old plan for this edge
    -> compute a fresh S3 candidate, if one is valid

left endpoint has live nested content
    -> remove any old plan and keep the edge as a structural barrier
```

This classification is required after S3 commits and after an empty rule is
unlinked. Either operation can make two same-selector rules newly adjacent.
S1 therefore remains higher priority than candidate creation throughout the
whole fixed-point process, not only during initial discovery.

Conceptually, edge invalidation routes the edge to the appropriate queue:

```rust,ignore
fn mark_edge_dirty(edge: Edge, state: &mut MergeState) {
    state.candidates.remove(&edge);

    if !is_live_edge(edge, state) {
        return;
    }

    if state.rules[edge.left].has_live_nested_content {
        return;
    }

    if exact_effective_rule_keys_equal(edge, state) {
        state.dirty_same_selector_edges.insert(edge);
    } else {
        state.dirty_partial_edges.insert(edge);
    }
}
```

## Processing algorithm

### Source-ordered recursive discovery

```rust,ignore
fn discover(
    rules: &mut RuleList,
    parent: Option<EffectiveSelectorContext>,
    at_rules: ConditionalAtRuleContextKey,
    state: &mut MergeState,
) {
    // This only joins adjacent blocks with equal typed frame keys.
    coalesce_adjacent_equal_conditional_blocks(rules, &at_rules);

    let list = state.lists.register(rules, parent, at_rules);

    for input_rule in rules {
        match input_rule {
            CssRule::Style(style) => {
                let effective = resolve_effective_selectors(
                    parent,
                    style.selectors,
                );
                let current = state.rules.append_style(
                    list,
                    style,
                    effective,
                );

                let key = EffectiveRuleKey {
                    barrier_segment: state.current_barrier_segment,
                    at_rules,
                    selectors: effective,
                };
                register_leading_declarations(current, key, state);

                // S1 is local and eager when the left endpoint has no live
                // nested content.
                let current = coalesce_same_selector_run(current, state);

                if let Some(entry) = leading_declaration_entry(current, state) {
                    let key = effective_rule_key(entry, state);
                    register_history_entry(key, entry, state);
                    state.dirty_histories.insert(key);
                }

                // Nested entries are visited after the parent's leading
                // declarations and before the next sibling.
                discover_style_children(
                    style,
                    effective,
                    at_rules,
                    state,
                );
                refresh_subtree_liveness(current, state);

                // Discover, but do not commit, a plan on this local edge.
                if let Some(edge) = previous_live_edge(current, state) {
                    mark_edge_dirty(edge, state);
                }
            }
            CssRule::NestedDeclarations(rule) => {
                let effective = parent_match_key(parent);
                let key = EffectiveRuleKey {
                    barrier_segment: state.current_barrier_segment,
                    at_rules,
                    selectors: effective,
                };
                let entry = register_nested_declarations(rule, key, state);
                let key = effective_rule_key(entry, state);
                register_history_entry(key, entry, state);
                state.dirty_histories.insert(key);
            }
            CssRule::Media(rule) => {
                discover(
                    &mut rule.rules,
                    parent,
                    at_rules.push(Media(rule.query.key())),
                    state,
                );
            }
            CssRule::Supports(rule) => {
                discover(
                    &mut rule.rules,
                    parent,
                    at_rules.push(Supports(rule.condition.key())),
                    state,
                );
            }
            CssRule::Container(rule) => {
                discover(
                    &mut rule.rules,
                    parent,
                    at_rules.push(Container {
                        name: rule.name.key(),
                        query: rule.condition.key(),
                    }),
                    state,
                );
            }
            _ => {
                register_unsupported_at_rule_barrier(input_rule, state);
                state.current_barrier_segment += 1;
            }
        }

        stabilize_s1_and_histories(state);
    }
}
```

The pseudocode uses value-like interned context keys; `push` returns a new key
and does not mutate the caller's stack. `discover_style_children` visits the
existing child-rule list while carrying the same conditional context; it does
not detach or flatten it. Future S2 changes automatically stale candidates
through endpoint declaration revisions.

### Stabilization and commit

```rust,ignore
fn stabilize(state: &mut MergeState) {
    loop {
        // S1 always has priority and may change declaration histories.
        if let Some(edge) = state.dirty_same_selector_edges.pop() {
            coalesce_same_selector_edge(edge, state);
            continue;
        }

        // S2 always has priority over S3.
        if let Some(key) = state.dirty_histories.pop() {
            let changed_entries = prune_effective_rule_history(key, state);
            for entry in changed_entries {
                declaration_entry_changed(entry, state);
            }
            continue;
        }

        // Different-selector edges only produce speculative S3 candidates.
        if let Some(edge) = state.dirty_partial_edges.pop() {
            recompute_partial_candidate(edge, state);
            continue;
        }

        let Some(candidate) = leftmost_candidate(state) else {
            break;
        };

        if !candidate_is_valid(state, candidate) {
            mark_edge_dirty(candidate.edge, state);
            continue;
        }

        // S3 commits one plan. This creates dirty histories and edges.
        commit_partial_merge(candidate, state);
    }

    // S4 runs only after no history or candidate can change the logical graph.
    remove_logically_empty_rules_post_order(state);
}
```

The important scheduling invariant is:

```text
dirty same-selector edges (S1)
    before dirty S2 histories
    before dirty partial edges
    before any S3 candidate commit
```

## Termination and idempotence

The pass never reverses a committed factorization. Progress is monotonic:

- S2 reduces the number of live declaration occurrences;
- S3 replaces two copies of a non-empty common sequence with one copy;
- S1 and S4 reduce the number of live rules when declaration count is
  unchanged.

Candidate invalidation and recomputation do not alter the AST. Therefore the
pass reaches a fixed point, and a second execution should make no changes.

## Required regression scenarios

### S1 and S2

```css
a {
  x: 1;
}
a {
  y: 2;
}

h1 {
  color: red !important;
}
.middle {
  display: block;
}
h1 {
  color: blue;
}
```

### Candidate invalidation

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

### Candidate recomputation

```css
a {
  x: red;
  y: 1;
}
b {
  y: 1;
}
a {
  x: blue;
}
```

### Selector-list ownership

```css
a,
b {
  color: red;
}
a {
  color: blue;
}
```

The first rule's declaration must not be removed for `b`.

### Overlapping candidates

```css
a {
  x: 1;
}
b {
  x: 1;
  y: 2;
}
c {
  y: 2;
}
```

The output must be deterministic and cascade-equivalent regardless of stale
candidate discovery order.

### Fallback and shorthand barriers

```css
a {
  display: -webkit-box;
}
.middle {
  display: block;
}
a {
  display: flex;
}

a {
  margin: 1px;
}
.middle {
  display: block;
}
a {
  margin-left: 2px;
}
```

### Empty-rule adjacency

```css
a {
  x: 1;
}
.empty {
  color: red;
}
b {
  x: 1;
}
.empty {
  color: blue;
}
```

If S2 empties the first `.empty`, the new live edge must be considered before
S4 physically removes it.

### Selector compatibility

```css
a {
  color: red;
}
:unknown {
  color: red;
}
```

The rules must not be combined unless target compatibility proves the selector
union safe.

### Nested effective selectors and specificity

```css
#a,
b {
  & c {
    color: blue;
  }
}
```

The effective selector must preserve the semantics and specificity of
`:is(#a, b) c`. It must not be treated as the ordinary expanded selector list
`#a c, b c`.

### Global S2 history through nested declaration order

```css
.a {
  color: red !important;

  .child {
    display: block;
  }

  color: blue;
}
```

The trailing `color: blue` is represented by a `NestedDeclarationsRule`. Its
parent-match key joins the leading `.a` declaration history, so S2 can remove
it without moving either declaration or the nested child.

### Parent identity versus explicit nesting selector

```css
.item,
.item::before {
  & {
    color: red;
  }

  color: blue;
}
```

The explicit `&` entry and the trailing nested-declarations entry must not be
put into one history: `&` cannot represent the parent's pseudo-element arm,
while the nested declarations match it exactly.

### Nested sibling partial factoring

```css
.parent {
  .a {
    color: red;
    display: block;
  }

  .b {
    display: block;
    color: blue;
  }
}
```

S3 may synthesize a `.a, .b` rule between these nested siblings inside
`.parent`. The original `.parent`, `.a`, and `.b` selectors remain unchanged;
the new rule receives a local selector union and an independently resolved
effective-selector key.

### Nested forward barrier

```css
.parent {
  .a {
    display: block;

    .child {
      color: red;
    }
  }

  .b {
    display: block;
  }
}
```

S3 must not move the first `display: block` into a shared sibling after `.a`,
because that would move it past `.a .child`. The edge may become eligible only
if the entire `.child` subtree later becomes logically empty.

### Post-order nested cleanup

```css
.outer {
  .middle {
    .inner {
      color: red;
    }
  }
}
```

If S2 removes the only declaration, logical emptiness propagates from `.inner`
to `.middle` and then `.outer`. Physical removal occurs in the same post-order.
A parent with any live descendant must remain live.

### Adjacent equal conditional blocks

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

The adjacent media blocks may form one logical conditional region. S1 can then
coalesce the two `a` rules while preserving their declaration and child-rule
order.

### Global S2 history across equal conditional blocks

```css
@supports (display: grid) {
  h1 {
    color: red !important;
  }
}

.middle {
  display: block;
}

@supports (display: grid) {
  h1 {
    color: blue;
  }
}
```

The two `h1` entries have equal effective rule keys, so S2 may remove
`color: blue`. The non-adjacent `@supports` blocks remain structurally
separate.

### Different conditional contexts

```css
@media (width >= 600px) {
  a {
    color: red;
  }
}

@media (width >= 800px) {
  a {
    color: blue;
  }
}
```

No declaration pruning, S1, S3, condition implication, or block merge occurs
between these rules.

### Ordered conditional stacks

```css
@media (width >= 600px) {
  @supports (display: grid) {
    a {
      display: grid;
    }
  }
}

@supports (display: grid) {
  @media (width >= 600px) {
    a {
      display: grid;
    }
  }
}
```

The ordered at-rule stacks differ, so the rules do not share an effective rule
key even if the two conditions happen to be simultaneously true.

### Repeated container frames

```css
@container card (width >= 300px) {
  @container card (width >= 300px) {
    a {
      display: block;
    }
  }
}
```

The two frames must remain in the context stack. They are not collapsed into
one frame because the nested queries may select different ancestor containers.

### Unsupported at-rule barrier

```css
a {
  color: red;
}

@layer theme {
  a {
    color: green;
  }
}

a {
  color: blue;
}
```

`@layer` is not represented as a conditional frame in this design. It creates
a barrier, and the two unlayered `a` rules are not compared across it.

### Feature flag and idempotence

The enabled path must reach the expected fixed point. The disabled path must
preserve rule boundaries, and running the enabled pass twice must not change
the second output or removal statistics.

## Open implementation questions

The following belong to the detailed implementation design:

- the stable `RuleId` and live-adjacency representation;
- the concrete immutable representation and interning policy for effective
  selectors, parent-match keys, and conditional at-rule context keys;
- whether effective-rule histories are updated eagerly or lazily;
- whether adjacent equal conditional blocks are physically coalesced during IR
  construction or represented as one logical region until emission;
- the concrete declaration movement/dependency proof used by S3;
- target-aware selector compatibility until RocketCSS exposes a targets model;
- profitability measurement for partial factoring;
- source-span and preserved-comment ownership;
- physical compaction versus output-time tombstones; and
- packed state, small-vector thresholds, and hash-table choices.
