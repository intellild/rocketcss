# Cross-rule declaration merging design

## Status

This document describes a correctness-first design for merging and pruning
declarations across style rules. It intentionally leaves runtime and memory
optimizations to a later implementation design.

The design applies independently to each sibling rule list. At-rules are out of
scope for the first implementation. A style rule containing nested rules is a
barrier until nested declaration ordering is modeled explicitly.

## Goals

The pass should:

1. coalesce adjacent rules with exactly equal selector lists;
2. remove declarations that are provably dead across rules with exactly equal
   selector lists;
3. factor common declarations out of adjacent rules with different selectors;
4. remove rules that become logically empty; and
5. reach an idempotent fixed point without changing CSS cascade behavior.

The pass must preserve declaration order, fallback chains, importance, vendor
prefixes, and selector validity. If either declaration dominance or selector
compatibility cannot be proven, the authored form is preserved.

## Non-goals

The initial design does not:

- merge through at-rules;
- reason through nested rule boundaries;
- split one selector list into independently owned selector arms;
- merge non-adjacent rules with different selectors;
- infer equivalence from reordered selector lists; or
- specify the final data-layout, hashing, allocation, or work-queue
  optimizations.

## Terminology

- **Live rule**: a rule that still has live declarations or nested content.
- **Live adjacency**: adjacency after logically empty rules are ignored, even
  before physical cleanup.
- **Exact selector list**: structurally equal live selectors in the same order
  and selector context. `a,b` is not equal to `b,a` or to `a`.
- **Selector history**: all rules with one exact selector list, ordered by
  source position.
- **Edge**: one pair of live-adjacent style rules.
- **Candidate**: a speculative selector partial-merge plan attached to an
  edge. A candidate does not mutate the AST until it is committed.

## Rewrite order

The logical priority is:

1. **R1: same-selector coalescing**;
2. **R3: exact-selector declaration pruning**;
3. **R2: selector partial factoring**; and
4. **R4: empty-rule cleanup**.

R2 is speculative while R3 is still capable of changing either endpoint.
Candidates may be discovered early, but they are not committed until all R3
histories that can affect them are stable.

### R1: same-selector coalescing

Two live-adjacent rules with exact selector lists are represented as one rule
whose declaration sequence preserves source order.

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

R1 is safe to apply eagerly because it does not change declaration ownership:
all declarations still belong to the same exact selector list.

R1 is applied repeatedly across a same-selector run before that rule is added
to its selector history.

If an already registered rule is coalesced later because logical adjacency
changed, its old history entry is replaced by the coalesced rule and that
selector history is marked dirty.

### R3: exact-selector declaration pruning

R3 processes a complete selector history as one source-ordered declaration
sequence. It may tombstone declarations in any rule in the history, but it does
not move rules or change selectors.

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

R3 must use typed declaration semantics. A property name match alone is not a
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

### R2: selector partial factoring

R2 considers live-adjacent rules with different selector lists. It factors one
safe common declaration sequence into a shared-selector rule.

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

R2 does not compute a declaration set intersection. A plan is valid only when:

1. common declaration occurrences are equal, including importance and prefix;
2. their relative order is preserved;
3. moving them across residual declarations preserves cascade behavior;
4. shorthand, longhand, fallback, variable, and `all` dependencies remain
   valid;
5. behavior is unchanged for elements matching either or both selector lists;
6. the selector union is valid for all configured targets and contexts; and
7. neither endpoint contains a nested-rule barrier.

The size/profitability policy is separate from semantic validity and can be
specified when the implementation design is finalized.

### R4: empty-rule cleanup

R4 physically removes rules with no live declarations and no nested content.
Logically empty rules stop participating in adjacency immediately; physical
cleanup is deferred until the candidate state is stable.

## Why candidates are required

R2 cannot mutate rules while future R3 updates remain possible. Consider:

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

When the final `a` is processed, R3 needs to delete `color: red` for `a` but
retain it for `b`. That is impossible without splitting the prematurely merged
selector list.

Instead, the first edge records a candidate. When R3 later changes the first
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
type Revision = u32;

struct MergeState<'ast> {
    rules: RuleSequence<'ast>,
    histories: Map<ExactSelectorListKey<'ast>, SelectorHistory>,
    candidates: Map<Edge, PartialMergeCandidate<'ast>>,
    dirty_histories: Set<ExactSelectorListKey<'ast>>,
    dirty_same_selector_edges: Set<Edge>,
    dirty_partial_edges: Set<Edge>,
}

struct RuleState<'ast> {
    rule: StyleRule<'ast>,
    live: bool,
    previous_live: Option<RuleId>,
    next_live: Option<RuleId>,
    selector_revision: Revision,
    declaration_revision: Revision,
}

struct SelectorHistory {
    rules: Vec<RuleId>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    left: RuleId,
    right: RuleId,
}

struct PartialMergeCandidate<'ast> {
    edge: Edge,
    left_selector_revision: Revision,
    right_selector_revision: Revision,
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

`ExactSelectorListKey` represents the complete live selector list and its
relevant context. It never indexes individual arms of a multi-selector rule.

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

    left.live
        && right.live
        && left.next_live == Some(candidate.edge.right)
        && right.previous_live == Some(candidate.edge.left)
        && left.selector_revision == candidate.left_selector_revision
        && right.selector_revision == candidate.right_selector_revision
        && left.declaration_revision == candidate.left_declaration_revision
        && right.declaration_revision == candidate.right_declaration_revision
}
```

Even a revision-valid cached plan may be revalidated immediately before commit
as a correctness guard.

## State transitions

### Declaration change

When R1 or R3 changes declarations in a rule:

```text
increment declaration_revision
invalidate candidates on both incident edges
classify and mark both incident edges dirty
if the rule became logically empty, update live adjacency
```

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

The initial `a`/`b` candidate has `x: red` as `left_only`. After R3 removes
`x: red`, that plan is stale. Recomputing the edge discovers complete equality
for `y: 1`.

### Selector change

When R2 commits and creates a shared selector list:

```text
increment selector_revision
remove the affected rules from their old selector histories
insert the new rule into its exact-selector history
mark the old and new histories dirty
invalidate incident candidates
classify and mark the new incident edges dirty
```

The new selector history must be processed by R3 before another candidate that
depends on it is committed.

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

Committing the first candidate creates a new `a,b` history. R3 can then prove
that the earlier `x: 1` is dead.

### Rule becomes logically empty

An empty rule is unlinked from the live adjacency chain immediately:

```text
invalidate (previous, empty) and (empty, next)
create and dirty (previous, next)
defer physical deletion to R4
```

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
same exact selector list
    -> apply R1 eagerly
    -> dirty the selector history and affected edges

different selector lists
    -> remove any old plan for this edge
    -> compute a fresh R2 candidate, if one is valid
```

This classification is required after R2 commits and after an empty rule is
unlinked. Either operation can make two same-selector rules newly adjacent.
R1 therefore remains higher priority than candidate creation throughout the
whole fixed-point process, not only during initial discovery.

Conceptually, edge invalidation routes the edge to the appropriate queue:

```rust,ignore
fn mark_edge_dirty(edge: Edge, state: &mut MergeState) {
    state.candidates.remove(&edge);

    if !is_live_edge(edge, state) {
        return;
    }

    if exact_selectors_equal(edge, state) {
        state.dirty_same_selector_edges.insert(edge);
    } else {
        state.dirty_partial_edges.insert(edge);
    }
}
```

## Processing algorithm

### Forward discovery

```rust,ignore
fn discover(rules: &mut RuleList, state: &mut MergeState) {
    for input_rule in rules {
        let current = state.rules.append(input_rule);

        // R1 is eager because exact-selector ownership does not change.
        let current = coalesce_same_selector_run(current, state);

        let key = exact_selector_list_key(current, state);
        register_coalesced_rule(key, current, state);

        // R3 may mutate current or earlier rules in this history.
        state.dirty_histories.insert(key);
        stabilize_r1_and_histories(state);

        // Discover, but do not commit, the current live-adjacency plan.
        if let Some(edge) = previous_live_edge(current, state) {
            mark_edge_dirty(edge, state);
        }
    }
}
```

Future R3 changes automatically stale candidates through endpoint revisions.

### Stabilization and commit

```rust,ignore
fn stabilize(state: &mut MergeState) {
    loop {
        // R1 always has priority and may change declaration histories.
        if let Some(edge) = state.dirty_same_selector_edges.pop() {
            coalesce_same_selector_edge(edge, state);
            continue;
        }

        // R3 always has priority over R2.
        if let Some(key) = state.dirty_histories.pop() {
            let changed_rules = prune_selector_history(key, state);
            for rule in changed_rules {
                declaration_changed(rule, state);
            }
            continue;
        }

        // Different-selector edges only produce speculative R2 candidates.
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

        // R2 commits one plan. This creates dirty histories and edges.
        commit_partial_merge(candidate, state);
    }

    // R4 runs only after no history or candidate can change the logical graph.
    remove_logically_empty_rules(state);
}
```

The important scheduling invariant is:

```text
dirty same-selector edges (R1)
    before dirty R3 histories
    before dirty partial edges
    before any R2 candidate commit
```

## Termination and idempotence

The pass never reverses a committed factorization. Progress is monotonic:

- R3 reduces the number of live declaration occurrences;
- R2 replaces two copies of a non-empty common sequence with one copy;
- R1 and R4 reduce the number of live rules when declaration count is
  unchanged.

Candidate invalidation and recomputation do not alter the AST. Therefore the
pass reaches a fixed point, and a second execution should make no changes.

## Required regression scenarios

### R1 and R3

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

If R3 empties the first `.empty`, the new live edge must be considered before
R4 physically removes it.

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

### Feature flag and idempotence

The enabled path must reach the expected fixed point. The disabled path must
preserve rule boundaries, and running the enabled pass twice must not change
the second output or removal statistics.

## Open implementation questions

The following belong to the detailed implementation design:

- the stable `RuleId` and live-adjacency representation;
- whether selector histories are updated eagerly or lazily;
- the concrete declaration movement/dependency proof used by R2;
- target-aware selector compatibility until RocketCSS exposes a targets model;
- profitability measurement for partial factoring;
- source-span and preserved-comment ownership;
- physical compaction versus output-time tombstones; and
- packed state, small-vector thresholds, and hash-table choices.
