# Cross-rule declaration merging: detailed state machine

## Document map

- [Overall design](./overall.md)
- [S1: same-selector coalescing](./s1-same-selector-coalescing.md)
- [S2: declaration-effect pruning](./s2-declaration-effect-pruning.md)
- [S3: selector partial factoring](./s3-selector-partial-factoring.md)
- [S4: lossless representation planning](./s4-ast-reification-planning.md)
- [S5: terminal commit and cleanup](./s5-ast-reification-commit.md)
- [Non-goals](./non-goal.md)
- [Detailed state machine](./detailed-state-machine.md)
- [Pseudocode](./pseudo-code.md)
- [Flat source-order AST IR](../flat-ast-ir/README.md)

## State ownership

The merge IR preserves effect semantics while the Radix AST remains the
authority for physical ownership, identity, order, and topology:

- authored style rules stay in their original rule lists;
- existing local and ancestor selectors are immutable;
- existing conditional at-rule frames are immutable;
- S1-S3 commit proven local AST changes and update declaration-effect
  liveness/sidecars atomically;
- S1 represents coalescing through an ordered declaration sequence whose active
  output owner is the right rule;
- S3 inserts a synthesized style rule at its final local Radix position;
- S4 incrementally stabilizes lossless declaration representation choices; and
- S5 finishes deferred representation and clears merge-only state.

The target storage uses typed `RadixIndexId` wrappers, primary source-order
vectors, sparse sibling Radix trees, direct rule topology, and a lexical
declaration tape. Ordinary long-lived Rust references to mutable style rules
are not part of the model. Code generation is outside the minify pipeline and
observes committed Radix AST state after S5 cleanup.

## Core identifiers

```rust,ignore
type RuleId = TypedRadixIndexId;
type DeclarationBlockId = TypedRadixIndexId;
type RuleListId = u32;
type RuleListSegmentId = u32;
type DeclarationEntryId = u32;
type DeclarationSequenceId = u32;
type DeclarationOccurrenceId = u32;
type EffectOccurrenceId = u32;
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
    block_state: DenseMap<DeclarationBlockId, BlockPassState>,
    declaration_sequences: DeclarationSequenceTable,
    effect_occurrences: EffectOccurrenceTable<'ast>,
    histories: Map<EffectiveRuleKey<'ast>, EffectiveRuleHistory>,
    current_history_segment: HistorySegmentId,
    candidates: Map<Edge, PartialMergeCandidate<'ast>>,
    dirty_histories: Set<EffectiveRuleKey<'ast>>,
    dirty_same_selector_edges: Set<Edge>,
    dirty_partial_edges: Set<Edge>,
    dirty_s4_plan_items: Set<S4PlanItem>,
    ast_plan: AstReificationPlan<'ast>,
    committed: bool,
}

struct RuleState<'ast> {
    rule: RuleId,
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
    declarations: DeclarationBlockId,
    owning_sequence: DeclarationSequenceId,
    live: bool,
    declaration_revision: Revision,
}

struct DeclarationSequenceState {
    blocks: OrderedSequence<DeclarationEntryId>,
    effects: DeclarationEffectIr,
    live_effect_count: u32,
    aggregate_revision: Revision,
    active_output_owner: RuleId,
}

struct DeclarationEffectIr<'ast> {
    occurrences: Vec<EffectOccurrenceId>,
    index: EffectIndex<'ast>,
    revision: Revision,
}

struct EffectOccurrenceState<'ast> {
    origin: DeclarationOccurrenceId,
    phase: CascadePhaseKey,
    effects: SmallVec<EffectEntry<'ast>>,
    expansion: EffectExpansion,
    live_effects: EffectLiveness,
}

struct EffectEntry<'ast> {
    key: EffectKey<'ast>,
    value: EffectValue<'ast>,
}

enum EffectExpansion {
    Exact,
    VirtualShorthand,
    Opaque,
}

struct AstReificationPlan<'ast> {
    sequences: Map<DeclarationSequenceId, SequenceAstPlan<'ast>>,
    synthesized_rules: Vec<SynthesizedRulePlan<'ast>>,
    removals: Set<RetainedNodeId>,
    complete: bool,
}

struct SequenceAstPlan<'ast> {
    owner: RuleId,
    declarations: AstDeclarationPlan<'ast>,
}

enum AstDeclarationPlan<'ast> {
    ReuseOrigins(Vec<DeclarationOccurrenceId>),
    Materialize(TypedDeclarationPlan<'ast>),
    Mixed {
        retained_origins: Vec<DeclarationOccurrenceId>,
        replacements: Vec<TypedDeclarationPlan<'ast>>,
    },
}

struct EffectiveRuleHistory {
    entries: OrderedMap<DeclarationBlockId, DeclarationEntryId>,
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
    left_only: EffectPlan<'ast>,
    common: EffectPlan<'ast>,
    right_only: EffectPlan<'ast>,
    selectors: SynthesizedSelectorPlan<'ast>,
}
```

These tables describe semantic sidecars. Rules and declaration blocks already
live in compilation-owned Radix stores; ordered sequences use AST occurrence
IDs/ranges, and synthesized nodes receive final sibling IDs during S3. Authored
declaration IDs are allocated in lexical source order, including around nested
rules.

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

`DeclarationSequenceId` models semantic declaration order independently from
the physical AST representation:

```text
blocks: ordered declaration-block references
effects: virtual semantic longhand effects for the complete sequence
live_effect_count: aggregate number of live virtual effects
aggregate_revision: changes whenever any referenced block changes
active_output_owner: the style rule that owns the committed AST sequence
```

Every declaration block remains an independent history entry with its original
AST occurrence ID. Structural S1 coalescing does not collapse history
membership into one synthetic occurrence.

`blocks` is a pass-local ordered sequence, not a second AST and not a
requirement to copy declaration payloads when S1 concatenates two sequences.
The committed AST stores the complete ordered declaration sequence as one
coalesced semantic `RadixRange`. There is no `previous_merged` field, owner
chain, or indirect declaration-ID list.

S1 updates the sequence aggregate and commits the chosen declaration-list
representation in one transaction. S2 updates `live_effect_count` when effect
masks change. A zero aggregate plus no retained child content triggers the
logical emptiness transition directly; no predecessor expansion or recursive
AST walk is involved.

The effect IR is an analysis overlay. A losslessly parsed shorthand contributes
virtual canonical longhand effects while retaining one authored declaration
origin. Effect-index slots contain ordered occurrence chains rather than one
last value, so vendor and target fallbacks are not discarded. Variables,
unknown syntax, `all`, and shorthands that cannot be expanded losslessly
produce conservative opaque or wildcard effects.

Property metadata defines the canonical affected-longhand and may-alias sets.
Unknown relationships conflict by default. The concrete effect index may use
dense known-property slots, precomputed hashes, and a separate map for custom
or opaque keys without changing these semantics.

An effect-liveness or replacement-plan edit originating from any predecessor
block increments:

1. its declaration-entry revision;
2. its sequence aggregate revision; and
3. its effect-IR revision; and
4. the generation of its effective-rule history.

S3 candidates snapshot aggregate sequence revisions, so an edit anywhere in
the chain invalidates the candidate.

## Effective-rule histories

History membership is keyed by the complete `EffectiveRuleKey`:

```text
effective selector semantics
+ ordered conditional at-rule context
+ history segment
```

Entries are ordered by `DeclarationBlockId`, never by discovery time. Mid-pass
S3 insertion allocates its final sibling Radix ID at the actual insertion point
and inserts the history entry by that ID.

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
- every residual and synthesized effect sequence has a lossless AST
  reification plan; and
- the declaration movement proof still succeeds.

Candidate commit reruns these checks. Cached validity is not authority.

## S1 transition

For enumerated S1 input/output states and worked examples, see
[S1: same-selector coalescing](./s1-same-selector-coalescing.md).

S1 applies to a live edge with:

- equal effective rule keys;
- equal emission identities; and
- no retained child content on the left endpoint.

It performs one atomic local transition:

1. revalidate the edge;
2. remove all candidates and dirty-edge entries incident to the old endpoints;
3. concatenate the left sequence before the right sequence;
4. keep the right rule as `active_output_owner`;
5. set the combined live-effect count to the exact sum of both sequences;
6. increment the combined aggregate revision;
7. retire the left rule from live adjacency;
8. preserve all declaration history entries and AST occurrence IDs;
9. reconnect `old_previous -> right -> old_next`;
10. dirty the affected effective-rule history; and
11. classify the final incident edges.

The right child list remains owned by the right rule and follows the combined
declaration sequence. The left rule is eligible only when retiring it cannot
hide retained output.

## S2 transition

For enumerated history, occurrence, and resolver states, see
[S2: declaration-effect pruning](./s2-declaration-effect-pruning.md).

S2 processes one complete history in source order until it reaches a local
fixed point.

The typed resolver operates on effect occurrences and returns:

```rust,ignore
enum EffectResolution {
    NoChange,
    Apply(EffectEditPlan),
}

struct EffectEditPlan {
    edits: Vec<EffectOccurrenceEdit>,
}

enum EffectOccurrenceEdit {
    MarkEffectsDead {
        occurrence: EffectOccurrenceId,
        effects: EffectMask,
    },
}
```

Applying an edit:

1. changes only the referenced effect occurrence or its live-effect mask;
2. preserves its authored or synthesized declaration origin;
3. updates the owning sequence's aggregate live-effect count;
4. increments the declaration-entry revision;
5. increments the owning sequence aggregate revision;
6. increments the effect-IR revision;
7. increments the history generation;
8. invalidates incident candidates;
9. dirties affected local edges; and
10. updates retained-content and logical liveness.

When the current resolver changes its own history, it consumes the new
generation within the same local fixed-point loop. Changes originating from
another stage enqueue the history normally.

No AST declaration is rewritten during this transition. Unknown or
non-lossless relationships return `NoChange` or remain represented by opaque
effects.

## S3 candidate creation

For enumerated candidate states and worked examples, see
[S3: selector partial factoring](./s3-selector-partial-factoring.md).

S3 considers a live edge with:

- different selector lists;
- equal conditional and emission contexts;
- no retained child content on the left; and
- declaration sequences with a provably movable common effect subsequence.

The candidate records:

- the edge;
- both aggregate revisions;
- exact occurrence edits for both endpoints;
- immutable selector-arm references;
- an origin span for every selector arm and moved declaration; and
- the endpoint IDs that determine the local Radix insertion interval.

Candidate creation does not mutate the AST or effect IR. Atomic S3 commit does.

## S3 commit

S3 commit is one atomic graph transformation:

1. snapshot the old live neighborhood;
2. remove all old incident candidates and dirty-edge entries;
3. mark the planned common effects dead in both endpoint IRs;
4. increment all affected entry and sequence revisions;
5. materialize/intern the selector union;
6. immediately filter, normalize, deduplicate, and validate the union;
7. resolve its effective selector in the same parent and conditional contexts;
8. insert an independent synthesized rule and block at the final local Radix
   position;
9. initialize its declaration sequence and effect IR;
10. insert the synthesized history entry by `DeclarationBlockId`;
11. recompute endpoint liveness;
12. replace the old live-chain neighborhood with the final neighborhood once;
13. dirty all affected histories; and
14. classify exactly the new incident edges.

The synthesized rule and selector become physical AST nodes during the atomic
S3 commit. All fallible validation/materialization happens before the local
topology transaction becomes observable.

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
6. record any remaining declaration-representation work for S4 and terminal
   unlink/cleanup work for S5.

When an empty `NestedDeclarationsRule` is removed, its two local segments may
join. The newly adjacent endpoints are classified only after the barrier is
logically gone.

A supported conditional wrapper becomes empty only after its entire child list
contains no retained node. Opaque and unsupported nodes never become empty by
inference from this pass.

## S4 lossless representation planning

For enumerated rule-retention and sequence-representation states, see
[S4: lossless representation planning](./s4-ast-reification-planning.md).

S4 processes a plan item when that item's current S1-S3 dependencies are
stable. It may invalidate dependent cost, owner, or representation snapshots
and enqueue new S3/S4 work. Across the stabilization loop it produces a
deterministic `AstReificationPlan` containing:

- logically dead style rules whose effect IR is empty;
- selector-`KnownNoMatch` subtrees;
- empty `NestedDeclarationsRule` nodes;
- supported conditional wrappers with no retained child; and
- retired output-storage nodes that remain pinned until their sequence is
  finalized.

Every logically empty adjacency endpoint must already have been unlinked and
its newly exposed edges classified when its final live effect disappeared. S4
is therefore a verifier and planner, not a late graph mutation pass.

For each non-empty retained declaration sequence, the plan also records:

- the final AST output owner;
- retained authored shorthand and ordered fallback occurrences;
- S4's chosen typed representation for partially live virtual effects;
- any proven equivalent and profitable recombination; and
- synthesized rules, selectors, source positions, and combined origins.

An S2 history remains an analysis index over occurrences in one or more
sequences. It never merges those sequences or selects a common owner. S4
chooses one representation per retained sequence. A changed plan revision
invalidates dependent snapshots before another S3 candidate can commit. Every
S2 edit and S3 candidate must prove reifiability before semantic commit;
otherwise it returns `NoChange` or remains uncommitted.

## S5 terminal commit and cleanup

For enumerated plan-input and AST-output states, see
[S5: terminal commit and cleanup](./s5-ast-reification-commit.md).

S5 starts only when the work-coverage invariant holds, every history generation
is consumed, the candidate and dirty-S4-plan sets are empty, every plan
dependency revision is current, and S4 has produced complete lossless
representation choices.

S5 applies that plan without making new semantic choices:

1. writes planned declarations and importance bits to active AST output owners;
2. materializes planned typed longhand replacements;
3. verifies synthesized rules/selectors already occupy their final Radix
   positions;
4. finishes planned removals and optionally compacts tombstones; and
5. clears merge-only references, revisions, and retired-storage ownership.

S5 is a one-way commit. It must preserve the exact semantic effect sequence and
must not create new S1-S4 work. After it completes, code generation uses only
the ordinary stylesheet AST.

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
commit uses deterministic `DeclarationBlockId` order.

Committing `(a, b)` invalidates all old edges incident to `a`, `b`, and the new
shared rule before another candidate is selected.

## Scheduling priority

The global priority is:

```text
dirty S1 edges
before dirty S2 histories
before dirty S3 edges
before dirty S4 plan items
before any S3 candidate commit
before the one-way S5 terminal commit
```

S1 remains higher priority throughout the fixed point. S2 or S3 may remove an
intervening node and expose a new same-selector edge after initial discovery.

## Persistent stabilization state

The merge state lives for the complete semantic fixed point. Discovery builds
the initial rule table, live sibling links, declaration sequences, histories,
and dirty work once.

Logically retiring one endpoint splices `previous_live`/`next_live` and
classifies only the final newly exposed edge. It must not compact the AST,
re-walk every declaration block, or rebuild every effective-rule history to
discover that edge. Each later semantic mutation updates the corresponding
edge, history generation, sequence revision, and S4 work item in place.

Local rule-list insertion/unlinking happens during proven S1-S3 transactions.
Optional compaction happens once in S5 after the Stabilizer is empty. This
persistent-state requirement bounds work by the initial state plus actual
mutations rather than by the number of global rediscovery rounds.

## Work-coverage invariant

Every mutation must leave the work graph in this state:

```text
all changed histories have generation > consumed_generation and are dirty
all removed incident edges are absent from candidates and dirty sets
all newly possible incident edges are classified into exactly one dirty set
all unchanged candidates still match endpoint aggregate revisions
all changed sequence representations have dirty S4 plan items
all unchanged S4 plans match sequence and dependency revisions
```

The pass stops only when:

- both dirty-edge sets are empty;
- the dirty-history set is empty;
- the dirty-S4-plan set is empty;
- every history generation is consumed; and
- the candidate map is empty; and
- every S4 plan revision matches its dependencies.

At that point the AST representation plan is stable and complete. Minification
is complete only after S5 has finalized it and set `committed = true`.

## Termination

S1-S4 do not reverse a committed factorization or physically fold virtual
longhand effects back into shorthand.

A finite lexicographic progress measure exists:

- each typed shorthand occurrence is virtually expanded at most once per effect
  IR revision;
- S2 otherwise removes live effects or retains occurrences;
- S3 replaces two non-empty common copies with one;
- S1 reduces live output owners; and
- S4 reduces the logically retained output plan.

Candidate invalidation and recomputation do not mutate the AST; successful
S1-S3 commits do so atomically. S5 performs terminal finalization after the
semantic fixed point and does not participate in the progress loop. Between
semantic mutations, each dirty set contains an edge or history key at most
once.

The progress measure plus the work-coverage invariant yields a complete,
idempotent fixed point.

## Required regression matrix

### Declaration semantics

- earlier important versus later normal declarations;
- all earlier/later `!important` combinations;
- vendor fallback chains;
- partial shorthand overrides that retain unaffected components;
- variables and unknown shorthand syntax;
- virtual shorthand expansion retains authored origins;
- partial shorthand liveness reifies to an equivalent AST representation;
- ordered vendor and target fallback chains survive effect indexing;
- case-sensitive custom properties such as `--x` versus `--X`;
- `all` exceptions: custom properties, `direction`, and `unicode-bidi`;
- logical/physical conflicts with unknown writing direction;
- `revert` and `revert-layer`; and
- enabled, disabled, and second-pass idempotence.

### AST reification

- code generation receives no merge-state or effect-IR dependency;
- an empty effect IR plus no retained child produces no AST rule;
- a non-empty effect IR is written to exactly one active AST output owner;
- authored shorthand plus overrides is retained when it is still exact;
- partially live shorthand effects materialize lossless typed longhands;
- synthesized selectors and rules are inserted at their semantic source
  positions;
- required fallback order and importance bits survive reification; and
- no merge-only retired storage remains observable after S5.

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
