# Cross-rule declaration merging: detailed state machine

## Document map

- [Overall design](./overall.md)
- [Non-goals](./non-goal.md)
- [Detailed state machine](./detailed-state-machine.md)
- [Pseudocode](./pseudo-code.md)

## State ownership

The merge IR preserves AST ownership:

- authored style rules stay in their original rule lists;
- existing local and ancestor selectors are immutable;
- existing conditional at-rule frames are immutable;
- S2 edits declaration occurrences without moving rules;
- S1 represents coalescing through an ordered declaration sequence whose active
  output owner is the right rule;
- S3 creates a new independent style rule in the owning rule-list segment; and
- S4 physically removes nodes only after logical liveness is stable.

Pinned declaration blocks and stable identifiers should be reused. Ordinary
long-lived Rust references to mutable style rules are not part of the model.
Mutation and code generation are separate phases.

## Core identifiers

```rust,ignore
type RuleId = u32;
type RuleListId = u32;
type RuleListSegmentId = u32;
type DeclarationEntryId = u32;
type DeclarationSequenceId = u32;
type DeclarationOccurrenceId = u32;
type HistorySegmentId = u32;
type Revision = u32;
type Generation = u32;
```

`HistorySegmentId` prevents S2 from comparing through opaque or unsupported
semantics. `RuleListSegmentId` prevents S1/S3 from skipping retained
non-endpoint siblings.

## Conceptual state

```rust,ignore
struct MergeState<'ast> {
    lists: Map<RuleListId, RuleSequence<'ast>>,
    rules: RuleTable<'ast>,
    declaration_entries: DeclarationEntryTable<'ast>,
    declaration_sequences: DeclarationSequenceTable,
    histories: Map<EffectiveRuleKey<'ast>, EffectiveRuleHistory>,
    current_history_segment: HistorySegmentId,
    candidates: Map<Edge, PartialMergeCandidate<'ast>>,
    dirty_histories: Set<EffectiveRuleKey<'ast>>,
    dirty_same_selector_edges: Set<Edge>,
    dirty_partial_edges: Set<Edge>,
}

struct RuleState<'ast> {
    rule: StyleRule<'ast>,
    owning_list: RuleListId,
    list_segment: RuleListSegmentId,
    selector_state: SelectorState<'ast>,
    local_selectors: LocalSelectorListRef<'ast>,
    at_rule_context: ConditionalAtRuleContextKey<'ast>,
    cascade_context: CascadeContextKey,
    emission_identity: EmissionIdentity,
    leading_sequence: Option<DeclarationSequenceId>,
    live: bool,
    retained_child_count: u32,
    retired_output_storage: bool,
    previous_live: Option<RuleId>,
    next_live: Option<RuleId>,
}

struct DeclarationEntryState<'ast> {
    origin: DeclarationOrigin,
    effective_rule: EffectiveRuleKey<'ast>,
    cascade_phase: CascadePhaseKey,
    declarations: DeclarationBlockRef<'ast>,
    source_order: SemanticSourceOrderKey,
    owning_sequence: DeclarationSequenceId,
    live: bool,
    declaration_revision: Revision,
}

struct DeclarationSequenceState {
    blocks: Vec<DeclarationEntryId>,
    aggregate_revision: Revision,
    active_output_owner: RuleId,
}

struct EffectiveRuleHistory {
    entries: OrderedMap<SemanticSourceOrderKey, DeclarationEntryId>,
    generation: Generation,
    consumed_generation: Generation,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    list: RuleListId,
    segment: RuleListSegmentId,
    left: RuleId,
    right: RuleId,
}

struct PartialMergeCandidate<'ast> {
    edge: Edge,
    left_sequence_revision: Revision,
    right_sequence_revision: Revision,
    plan: PartialMergePlan<'ast>,
}

struct PartialMergePlan<'ast> {
    left_only: DeclarationPlan<'ast>,
    common: DeclarationPlan<'ast>,
    right_only: DeclarationPlan<'ast>,
    selectors: SynthesizedSelectorPlan<'ast>,
    insertion_order: SemanticSourceOrderKey,
}
```

The concrete containers may change. The ownership and dependency relationships
must not.

## Selector and output states

A style rule has one of three selector states:

- `Live(key)`: participates in output and may receive a cross-rule identity.
- `KnownNoMatch`: the rule and its complete subtree do not output.
- `OpaqueBarrier`: the rule and subtree remain output, receive no cross-rule
  identity, and split both history and local adjacency.

S1 also has a distinct storage state:

- `retired_output_storage`: the rule is no longer a live adjacency endpoint,
  but its pinned declaration block remains referenced by another sequence's
  active output owner.

Selector-dead rules, ordinary live rules, opaque retained rules, and retired
storage must never be conflated.

## Retained child content

`retained_child_count` includes every child node that must still serialize:

- a selector-live nested style rule;
- a live `NestedDeclarationsRule`;
- a supported conditional block containing retained nodes;
- an opaque selector subtree;
- an unsupported at-rule;
- an `@nest` wrapper; or
- another retained non-style node.

Any retained child pins every style-rule ancestor. Losing the final retained
child may make an endpoint eligible for S1/S3 and therefore dirties its
incident edges.

## Rule-list segments

Only style-rule endpoints inhabit a local adjacency segment. Each retained
non-endpoint node ends the current `RuleListSegmentId`.

A live `NestedDeclarationsRule` is an S2 history entry and a local barrier. It
does not become an S1/S3 endpoint and cannot be skipped by an edge.

Supported conditional wrappers end the containing list's local segment. Their
child lists receive independent segments. Adjacent wrappers with equal typed
frames may be represented as one logical region before child-edge discovery.

Opaque or unsupported nodes end both the local rule-list segment and the
current global history segment.

## Declaration sequences

`DeclarationSequenceId` models RocketCSS's existing `previous_merged` chain:

```text
blocks: ordered declaration-block references
aggregate_revision: changes whenever any referenced block changes
active_output_owner: the style rule whose codegen emits the full chain
```

Every declaration block remains an independent history entry with its original
`SemanticSourceOrderKey`. Structural S1 coalescing does not collapse history
membership into one synthetic occurrence.

An edit inside any predecessor block increments:

1. its declaration-entry revision;
2. its sequence aggregate revision; and
3. the generation of its effective-rule history.

S3 candidates snapshot aggregate sequence revisions, so an edit anywhere in
the chain invalidates the candidate.

## Effective-rule histories

History membership is keyed by the complete `EffectiveRuleKey`:

```text
effective selector semantics
+ ordered conditional at-rule context
+ history segment
```

Entries are ordered by `SemanticSourceOrderKey`, never by discovery time.
Mid-pass S3 insertion allocates a source-order key at the actual insertion
point and inserts or rebuilds the history accordingly.

Each history tracks:

- `generation`: latest membership or declaration mutation;
- `consumed_generation`: latest generation processed to a local S2 fixed point.

A history is stable only when these generations are equal.

## Candidate validity

A candidate is valid only when all of the following remain true:

- both endpoints are live;
- both endpoints remain in the candidate's list and segment;
- the endpoints remain mutually adjacent in the live chain;
- conditional contexts remain equal;
- emission identities remain equal;
- the left endpoint has no retained child content;
- both declaration sequences still exist;
- both aggregate sequence revisions match the candidate snapshots;
- the selector union can still be materialized, canonicalized, and validated;
  and
- the declaration movement proof still succeeds.

Candidate commit reruns these checks. Cached validity is not authority.

## S1 transition

S1 applies to a live edge with:

- equal effective rule keys;
- equal emission identities; and
- no retained child content on the left endpoint.

It performs one atomic local transition:

1. revalidate the edge;
2. remove all candidates and dirty-edge entries incident to the old endpoints;
3. concatenate the left sequence before the right sequence;
4. keep the right rule as `active_output_owner`;
5. increment the combined aggregate revision;
6. retire the left rule from live adjacency;
7. preserve all declaration history entries and source-order keys;
8. reconnect `old_previous -> right -> old_next`;
9. dirty the affected effective-rule history; and
10. classify the final incident edges.

The right child list remains owned by the right rule and follows the combined
declaration sequence. The left rule is eligible only when retiring it cannot
hide retained output.

## S2 transition

S2 processes one complete history in source order until it reaches a local
fixed point.

The typed resolver returns:

```rust,ignore
enum DeclarationResolution<'ast> {
    NoChange,
    Apply(DeclarationEditPlan<'ast>),
}

struct DeclarationEditPlan<'ast> {
    edits: Vec<DeclarationOccurrenceEdit<'ast>>,
}

enum DeclarationOccurrenceEdit<'ast> {
    Tombstone(DeclarationOccurrenceId),
    ReplaceWith {
        occurrence: DeclarationOccurrenceId,
        declarations: TypedDeclarationPlan<'ast>,
    },
}
```

Applying an edit:

1. changes only the referenced declaration occurrence;
2. preserves or records its source origin;
3. increments the declaration-entry revision;
4. increments the owning sequence aggregate revision;
5. increments the history generation;
6. invalidates incident candidates;
7. dirties affected local edges; and
8. updates retained-content and logical liveness.

When the current resolver changes its own history, it consumes the new
generation within the same local fixed-point loop. Changes originating from
another stage enqueue the history normally.

Unknown or non-lossless relationships return `NoChange`.

## S3 candidate creation

S3 considers a live edge with:

- different selector lists;
- equal conditional and emission contexts;
- no retained child content on the left; and
- declaration sequences with a provably movable common subsequence.

The candidate records:

- the edge;
- both aggregate revisions;
- exact occurrence edits for both endpoints;
- immutable selector-arm references;
- an origin span for every selector arm and moved declaration; and
- the semantic insertion position.

Candidate creation does not mutate the AST.

## S3 commit

S3 commit is one atomic graph transformation:

1. snapshot the old live neighborhood;
2. remove all old incident candidates and dirty-edge entries;
3. tombstone the planned common occurrences;
4. increment all affected entry and sequence revisions;
5. deep-materialize the selector union in the arena;
6. immediately filter, normalize, deduplicate, and validate the union;
7. resolve its effective selector in the same parent and conditional contexts;
8. create an independent synthesized style rule and declaration sequence;
9. allocate a semantic source-order key at the edge insertion point;
10. insert the synthesized history entry by semantic order;
11. recompute endpoint liveness;
12. replace the old live-chain neighborhood with the final neighborhood once;
13. dirty all affected histories; and
14. classify exactly the new incident edges.

For:

```text
p -> left -> right -> q
```

complete factoring that empties both endpoints must directly produce:

```text
p -> shared -> q
```

It must never enqueue a temporary `p -> q` bypass edge. If the right endpoint
retains children, the final order is:

```text
p -> shared -> right -> q
```

## Logical emptiness transition

When an ordinary style-rule endpoint becomes logically empty:

1. remove `(previous, empty)` and `(empty, next)` from candidates and dirty
   sets;
2. unlink the endpoint;
3. create `(previous, next)` only when both endpoints are in the same
   `RuleListSegmentId`;
4. dirty that new edge;
5. propagate retained-content changes toward ancestors; and
6. defer physical removal to S4.

When an empty `NestedDeclarationsRule` is removed, its two local segments may
join. The newly adjacent endpoints are classified only after the barrier is
logically gone.

A supported conditional wrapper becomes empty only after its entire child list
contains no retained node. Opaque and unsupported nodes never become empty by
inference from this pass.

## Dirty-edge processing

Marking an edge dirty first removes all stale representations:

```text
remove candidate(edge)
remove dirty_same_selector(edge)
remove dirty_partial(edge)
```

The edge is then reclassified only if:

- it is a current live edge;
- both endpoints have the same list and segment;
- emission identities match; and
- the left endpoint has no retained child content.

Equal effective rule keys route to S1. Different selector lists route to S3
candidate computation.

Both consumers revalidate current eligibility before mutating state. Dirty
membership is a work hint, not proof.

## Overlapping candidates

Edges `(a, b)` and `(b, c)` cannot commit from the same snapshots. Candidate
commit uses a deterministic semantic source-order policy.

Committing `(a, b)` invalidates all old edges incident to `a`, `b`, and the new
shared rule before another candidate is selected.

## Scheduling priority

The global priority is:

```text
dirty S1 edges
before dirty S2 histories
before dirty S3 edges
before any S3 candidate commit
before S4 physical cleanup
```

S1 remains higher priority throughout the fixed point. S2 or S3 may remove an
intervening node and expose a new same-selector edge after initial discovery.

## Work-coverage invariant

Every mutation must leave the work graph in this state:

```text
all changed histories have generation > consumed_generation and are dirty
all removed incident edges are absent from candidates and dirty sets
all newly possible incident edges are classified into exactly one dirty set
all unchanged candidates still match endpoint aggregate revisions
```

The pass stops only when:

- both dirty-edge sets are empty;
- the dirty-history set is empty;
- every history generation is consumed; and
- the candidate map is empty.

## Termination

The pass does not reverse a committed factorization or fold generated
longhands back into shorthand.

A finite lexicographic progress measure exists:

- each typed shorthand occurrence can be expanded at most once;
- S2 otherwise tombstones or retains occurrences;
- S3 replaces two non-empty common copies with one;
- S1 reduces live output owners; and
- S4 reduces retained AST nodes.

Candidate invalidation and recomputation do not mutate the AST. Between AST
mutations, each dirty set contains an edge or history key at most once.

The progress measure plus the work-coverage invariant yields a complete,
idempotent fixed point.

## Required regression matrix

### Declaration semantics

- earlier important versus later normal declarations;
- all earlier/later `!important` combinations;
- vendor fallback chains;
- partial shorthand overrides that retain unaffected components;
- variables and unknown shorthand syntax;
- case-sensitive custom properties such as `--x` versus `--X`;
- `all` exceptions: custom properties, `direction`, and `unicode-bidi`;
- logical/physical conflicts with unknown writing direction;
- `revert` and `revert-layer`; and
- enabled, disabled, and second-pass idempotence.

### Candidate and history state

- later S2 mutation invalidates an earlier S3 candidate;
- candidate recomputation discovers a different common sequence;
- editing a predecessor block in an S1 sequence invalidates an S3 candidate;
- synthesized history insertion occurs before an already discovered later
  entry;
- overlapping candidates commit deterministically;
- complete factoring reconnects the live chain atomically; and
- a stale dirty edge is harmless after its endpoints cease to be adjacent.

### Selector and emission state

- selector-list ownership is never split by S2;
- synthesized selector unions deduplicate immediately;
- target-incompatible selector unions remain unmerged;
- `KnownNoMatch` removes the complete subtree;
- recovered or invalid nesting selectors remain opaque barriers;
- pseudo-element parent matching differs from explicit `&`;
- multiple and recursive nesting selectors preserve specificity;
- CSS Modules contexts participate in emission identity;
- different vendor prefixes do not merge; and
- legacy `@nest` wrappers remain barriers.

### Nesting and barriers

- a live `NestedDeclarationsRule` breaks local S1/S3 adjacency;
- a left endpoint with retained nested content blocks movement;
- a right endpoint with children remains the S1 active output owner;
- an opaque child pins all ancestors;
- empty nested descendants are cleaned post-order;
- removal of an empty `NestedDeclarationsRule` exposes and classifies the new
  edge; and
- a selector-retired S1 storage rule remains available to its active output
  owner.

### Conditional at-rules

- adjacent equal media/supports/container blocks form one logical region;
- non-adjacent equal contexts share S2 history without wrapper movement;
- different conditions never compare;
- reordered conditional stacks never compare;
- repeated nested container frames are retained;
- unsupported at-rules split history and local adjacency; and
- an empty supported conditional wrapper is removed only after its complete
  child list is empty.
