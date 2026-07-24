# Cross-rule declaration merging: pseudocode

## Document map

- [Overall design](./overall.md)
- [S1: same-selector coalescing](./s1-same-selector-coalescing.md)
- [S2: declaration-effect pruning](./s2-declaration-effect-pruning.md)
- [S3: selector partial factoring](./s3-selector-partial-factoring.md)
- [S4: AST reification planning](./s4-ast-reification-planning.md)
- [S5: AST reification commit](./s5-ast-reification-commit.md)
- [Non-goals](./non-goal.md)
- [Detailed state machine](./detailed-state-machine.md)
- [Pseudocode](./pseudo-code.md)

This file shows control flow only. State ownership and transition requirements
are normative in [Detailed state machine](./detailed-state-machine.md).

## Typed context keys

```rust,ignore
enum EffectiveSelectorResult<'ast> {
    Live(EffectiveSelectorKey<'ast>),
    KnownNoMatch,
    OpaqueBarrier,
}

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

struct EmissionIdentity {
    wrapper_kind: StyleWrapperKind,
    vendor_prefix: VendorPrefix,
    selector_serialization_context: SelectorSerializationContextKey,
}
```

`ConditionalAtRuleContextKey::push` returns a new immutable/interned key. It
does not mutate the caller's stack.

Layer, origin, and phase keys are compared only for exact equality. The merge
pass consumes these opaque keys but does not interpret at-rule or cascade
semantics.

## Declaration-effect IR construction

```rust,ignore
fn build_effect_ir(
    sequence: DeclarationSequenceId,
    state: &MergeState,
) -> DeclarationEffectIr {
    let mut ir = DeclarationEffectIr::new();

    for entry in state.declaration_sequences[sequence].blocks {
        for declaration in authored_occurrences(entry, state) {
            let occurrence = match expand_typed_effects(declaration, state) {
                Lossless(effects) => {
                    let expansion = if effects.len() == 1 {
                        Exact
                    } else {
                        VirtualShorthand
                    };
                    let live_effects = EffectLiveness::all(effects.len());
                    EffectOccurrence {
                        origin: declaration.id,
                        effects,
                        expansion,
                        live_effects,
                    }
                }
                Opaque(affected) => {
                    let effects = affected.as_opaque_effects();
                    let live_effects = EffectLiveness::all(effects.len());
                    EffectOccurrence {
                        origin: declaration.id,
                        effects,
                        expansion: Opaque,
                        live_effects,
                    }
                }
            };

            ir.insert_in_source_order(occurrence);
        }
    }

    ir
}
```

`EffectIndex` maps canonical effect keys to ordered occurrence chains. Known
properties may use dense metadata-generated slots; custom and opaque keys may
use a hash map. Unknown relationships are conflicts, not proof of independence.
Building this IR does not rewrite authored declarations.

## Selector resolution

```rust,ignore
fn resolve_effective_selectors(
    parent: Option<EffectiveSelectorContext>,
    local: LocalSelectorListRef,
) -> EffectiveSelectorResult {
    if selector_list_is_proven_non_output(local) {
        return KnownNoMatch;
    }

    if local.contains_recovered_or_unparsed_syntax()
        || !nesting_positions_are_valid(local)
        || !pseudo_element_nesting_is_valid(parent, local)
        || !css_modules_context_is_resolved(local)
    {
        return OpaqueBarrier;
    }

    let Some(resolved) = resolve_selector_ast(parent, local) else {
        return OpaqueBarrier;
    };
    Live(intern_effective_selector_key(resolved))
}
```

Any unhandled resolver case returns `OpaqueBarrier`, not a guessed identity.

## Source-ordered discovery

```rust,ignore
fn discover(
    rules: &mut RuleList,
    parent: Option<EffectiveSelectorContext>,
    at_rules: ConditionalAtRuleContextKey,
    state: &mut MergeState,
) {
    coalesce_adjacent_equal_conditional_blocks(rules, &at_rules);

    let list = state.lists.register(rules, parent, at_rules);

    for input_rule in rules {
        match input_rule {
            CssRule::Style(style) => {
                discover_style(style, list, parent, at_rules, state);
            }

            CssRule::NestedDeclarations(rule) => {
                discover_nested_declarations(
                    rule,
                    list,
                    parent,
                    at_rules,
                    state,
                );
            }

            CssRule::Media(rule) => {
                end_local_rule_list_segment(list, state);
                discover(
                    &mut rule.rules,
                    parent,
                    at_rules.push(Media(rule.query.key())),
                    state,
                );
            }

            CssRule::Supports(rule) => {
                end_local_rule_list_segment(list, state);
                discover(
                    &mut rule.rules,
                    parent,
                    at_rules.push(Supports(rule.condition.key())),
                    state,
                );
            }

            CssRule::Container(rule) => {
                end_local_rule_list_segment(list, state);
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
                register_retained_unsupported_rule(input_rule, state);
                end_local_rule_list_segment(list, state);
                state.current_history_segment += 1;
            }
        }

        stabilize_s1_and_histories(state);
    }
}
```

## Style-rule discovery

```rust,ignore
fn discover_style(
    style: &mut StyleRule,
    list: RuleListId,
    parent: Option<EffectiveSelectorContext>,
    at_rules: ConditionalAtRuleContextKey,
    state: &mut MergeState,
) {
    match resolve_effective_selectors(parent, style.selectors) {
        KnownNoMatch => {
            register_non_output_subtree(style, state);
        }

        OpaqueBarrier => {
            register_retained_opaque_subtree(style, state);
            end_local_rule_list_segment(list, state);
            state.current_history_segment += 1;
        }

        Live(effective) => {
            let current = state.rules.append_style(
                list,
                current_rule_list_segment(list, state),
                style,
                effective,
                emission_identity(style),
            );

            let key = EffectiveRuleKey {
                history_segment: state.current_history_segment,
                at_rules,
                selectors: effective,
            };

            register_leading_declaration_sequence(
                current,
                key,
                next_source_order_key(state),
                state,
            );
            ensure_effect_ir(current.leading_sequence, state);

            // S1 is eager and has priority.
            let current = coalesce_same_selector_run(current, state);
            register_sequence_occurrences_in_history(current, state);

            discover_style_children(style, effective, at_rules, state);
            refresh_retained_child_count(current, state);

            if let Some(edge) = previous_live_edge(current, state) {
                mark_edge_dirty(edge, state);
            }
        }
    }
}
```

## Nested declaration discovery

```rust,ignore
fn discover_nested_declarations(
    rule: &mut NestedDeclarationsRule,
    list: RuleListId,
    parent: Option<EffectiveSelectorContext>,
    at_rules: ConditionalAtRuleContextKey,
    state: &mut MergeState,
) {
    let key = EffectiveRuleKey {
        history_segment: state.current_history_segment,
        at_rules,
        selectors: parent_match_key(parent),
    };

    let entry = register_nested_declarations(
        rule,
        key,
        next_source_order_key(state),
        state,
    );

    insert_history_entry_by_source_order(key, entry, state);
    dirty_history(key, state);

    // A live NDR participates in S2 but cannot be skipped by S1/S3.
    end_local_rule_list_segment(list, state);
}
```

## History insertion

```rust,ignore
fn insert_history_entry_by_source_order(
    key: EffectiveRuleKey,
    entry: DeclarationEntryId,
    state: &mut MergeState,
) {
    let order = state.declaration_entries[entry].source_order;
    state.histories[key].entries.insert(order, entry);
    state.histories[key].generation += 1;
    state.dirty_histories.insert(key);
}
```

An implementation may rebuild the ordered history instead of supporting
incremental ordered insertion.

## Edge invalidation and classification

```rust,ignore
fn mark_edge_dirty(edge: Edge, state: &mut MergeState) {
    state.candidates.remove(&edge);
    state.dirty_same_selector_edges.remove(&edge);
    state.dirty_partial_edges.remove(&edge);

    if !is_live_edge(edge, state)
        || !same_rule_list_segment(edge, state)
        || !same_emission_identity(edge, state)
    {
        return;
    }

    if state.rules[edge.left].retained_child_count != 0 {
        return;
    }

    if exact_effective_rule_keys_equal(edge, state) {
        state.dirty_same_selector_edges.insert(edge);
    } else {
        state.dirty_partial_edges.insert(edge);
    }
}
```

## S1 eligibility and commit

```rust,ignore
fn is_s1_eligible_now(edge: Edge, state: &MergeState) -> bool {
    is_live_edge(edge, state)
        && same_rule_list_segment(edge, state)
        && same_emission_identity(edge, state)
        && exact_effective_rule_keys_equal(edge, state)
        && state.rules[edge.left].retained_child_count == 0
}

fn commit_s1(edge: Edge, state: &mut MergeState) {
    if !is_s1_eligible_now(edge, state) {
        return;
    }

    let neighborhood = snapshot_neighborhood(edge, state);
    remove_old_incident_work(neighborhood, state);

    let left_sequence = state.rules[edge.left].leading_sequence.unwrap();
    let right_sequence = state.rules[edge.right].leading_sequence.unwrap();

    let combined = concatenate_sequences(
        left_sequence,
        right_sequence,
        edge.right,
        state,
    );

    state.rules[edge.right].leading_sequence = Some(combined);
    retire_as_output_storage(edge.left, state);
    reconnect_with_right_owner(neighborhood, edge.right, state);

    dirty_sequence_history(combined, state);
    classify_final_incident_edges(edge.right, state);
}
```

History occurrences and their semantic source-order keys are preserved.

## Declaration resolver

```rust,ignore
enum EffectResolution<'ast> {
    NoChange,
    Apply(EffectEditPlan<'ast>),
}

struct EffectEditPlan<'ast> {
    edits: Vec<EffectOccurrenceEdit<'ast>>,
}

enum EffectOccurrenceEdit<'ast> {
    MarkEffectsDead {
        occurrence: EffectOccurrenceId,
        effects: EffectMask,
    },
    PlanReplacement {
        occurrence: EffectOccurrenceId,
        replacement: TypedEffectPlan<'ast>,
    },
}
```

```rust,ignore
fn apply_effect_edit(
    edit: EffectOccurrenceEdit,
    state: &mut MergeState,
) {
    let entry = owning_entry(edit, state);
    let sequence = state.declaration_entries[entry].owning_sequence;
    let key = state.declaration_entries[entry].effective_rule;

    apply_typed_lossless_effect_edit(edit, state);

    state.declaration_entries[entry].declaration_revision += 1;
    state.declaration_sequences[sequence].aggregate_revision += 1;
    state.declaration_sequences[sequence].effects.revision += 1;
    state.histories[key].generation += 1;

    invalidate_sequence_candidates(sequence, state);
    dirty_sequence_incident_edges(sequence, state);
    update_logical_liveness(entry, state);
}
```

`apply_effect_edit` changes semantic liveness and replacement plans only. The
authored declaration AST remains intact until S5.

## S2 local fixed point

```rust,ignore
fn prune_effective_rule_history_to_local_fixed_point(
    key: EffectiveRuleKey,
    state: &mut MergeState,
) -> HistoryPruneResult {
    let mut changed = Set::new();

    loop {
        let start_generation = state.histories[key].generation;
        let ordered_entries =
            state.histories[key].entries.in_semantic_order();

        for relationship in effect_relationships(ordered_entries, state) {
            match resolve_typed_effects(relationship, state) {
                NoChange => {}
                Apply(plan) => {
                    for edit in plan.edits {
                        changed.insert(owning_entry(edit, state));
                        apply_effect_edit(edit, state);
                    }
                }
            }
        }

        if state.histories[key].generation == start_generation {
            break;
        }
    }

    HistoryPruneResult {
        changed_entries: changed,
        final_generation: state.histories[key].generation,
    }
}
```

Lossless shorthand expansion is one-way within this pass.

## S3 candidate validity

```rust,ignore
fn candidate_is_valid(
    candidate: &PartialMergeCandidate,
    state: &MergeState,
) -> bool {
    let edge = candidate.edge;
    let left = state.rules.get(edge.left);
    let right = state.rules.get(edge.right);

    let (Some(left_id), Some(right_id)) = (
        left.leading_sequence,
        right.leading_sequence,
    ) else {
        return false;
    };

    let left_sequence = state.declaration_sequences.get(left_id);
    let right_sequence = state.declaration_sequences.get(right_id);

    left.live
        && right.live
        && left.owning_list == edge.list
        && right.owning_list == edge.list
        && left.list_segment == edge.segment
        && right.list_segment == edge.segment
        && left.next_live == Some(edge.right)
        && right.previous_live == Some(edge.left)
        && left.at_rule_context == right.at_rule_context
        && left.emission_identity == right.emission_identity
        && left.retained_child_count == 0
        && left_sequence.aggregate_revision
            == candidate.left_sequence_revision
        && right_sequence.aggregate_revision
            == candidate.right_sequence_revision
        && effect_plan_is_still_valid(candidate.plan, state)
        && effect_plan_is_losslessly_reifiable(candidate.plan, state)
        && selector_plan_can_be_materialized(candidate.plan.selectors, state)
}
```

## Atomic S3 commit

```rust,ignore
fn commit_partial_merge(
    candidate: PartialMergeCandidate,
    state: &mut MergeState,
) {
    if !candidate_is_valid(&candidate, state) {
        mark_edge_dirty(candidate.edge, state);
        return;
    }

    // Complete every fallible selector operation before endpoint mutation.
    let local_selectors = deep_materialize_selector_union(
        candidate.plan.selectors,
        state.allocator,
    );
    let Some(local_selectors) =
        canonicalize_and_validate_synthesized_selectors(local_selectors)
    else {
        invalidate_candidate(candidate, state);
        return;
    };

    let effective = resolve_effective_selectors(
        parent_context(candidate.edge, state),
        local_selectors,
    );
    let Live(effective) = effective else {
        invalidate_candidate(candidate, state);
        return;
    };

    let old = snapshot_neighborhood(candidate.edge, state);
    remove_old_incident_work(old, state);
    apply_endpoint_effect_plans(&candidate.plan, state);

    let shared = create_logical_synthesized_style_rule(
        candidate.edge,
        local_selectors,
        effective,
        candidate.plan.common,
        candidate.plan.insertion_order,
        state,
    );

    insert_synthesized_history_entry_by_source_order(shared, state);
    recompute_endpoint_liveness(candidate.edge, state);
    reconnect_final_neighborhood_once(old, shared, state);

    dirty_all_affected_histories(candidate.edge, shared, state);
    classify_all_final_incident_edges(old, shared, state);
}
```

The concrete implementation prepares every fallible selector operation before
applying endpoint effect edits. A transactional implementation is also valid if
abort cannot leave partial semantic mutation. The synthesized rule is inserted
into the AST only during S5.

## Logical empty transition

```rust,ignore
fn rule_became_logically_empty(rule: RuleId, state: &mut MergeState) {
    let previous = state.rules[rule].previous_live;
    let next = state.rules[rule].next_live;

    remove_edge_work(previous, Some(rule), state);
    remove_edge_work(Some(rule), next, state);
    unlink_live_rule(rule, state);

    if let (Some(previous), Some(next)) = (previous, next)
        && state.rules[previous].list_segment
            == state.rules[next].list_segment
    {
        mark_edge_dirty(
            Edge::new_between(previous, next, state),
            state,
        );
    }

    propagate_retained_child_change_to_ancestors(rule, state);
    state.pending_s4_cleanup.insert(rule);
}
```

S3 complete factoring uses its atomic commit path instead of invoking this
function separately for both endpoints.

## Stabilization

```rust,ignore
fn stabilize(state: &mut MergeState) {
    loop {
        if let Some(edge) = state.dirty_same_selector_edges.pop() {
            if is_s1_eligible_now(edge, state) {
                commit_s1(edge, state);
            }
            continue;
        }

        if let Some(key) = state.dirty_histories.pop() {
            let result =
                prune_effective_rule_history_to_local_fixed_point(key, state);

            for entry in result.changed_entries {
                declaration_entry_changed_after_history_fixpoint(
                    entry,
                    state,
                );
            }

            state.histories[key].consumed_generation =
                result.final_generation;

            if state.histories[key].generation
                == state.histories[key].consumed_generation
            {
                state.dirty_histories.remove(&key);
            }

            continue;
        }

        if let Some(edge) = state.dirty_partial_edges.pop() {
            if is_s3_eligible_now(edge, state) {
                recompute_partial_candidate(edge, state);
            }
            continue;
        }

        let Some(candidate) = leftmost_candidate(state) else {
            break;
        };

        if !candidate_is_valid(candidate, state) {
            mark_edge_dirty(candidate.edge, state);
            continue;
        }

        commit_partial_merge(candidate, state);
    }

    assert_all_history_generations_consumed(state);
    finalize_s4_ast_plan(state);
}
```

## S4 logical cleanup and AST reification planning

```rust,ignore
fn finalize_s4_ast_plan(state: &mut MergeState) {
    assert_semantic_fixed_point(state);

    for node in state.ownership.post_order() {
        match node {
            Style(rule)
                if effect_ir_is_empty(rule, state)
                    && retained_child_count(rule, state) == 0
                    && !is_retired_sequence_storage(rule, state) =>
            {
                state.ast_plan.remove(rule);
            }
            NestedDeclarations(rule)
                if effect_ir_is_empty(rule, state) =>
            {
                state.ast_plan.remove(rule);
            }
            SupportedConditional(rule)
                if retained_child_count(rule, state) == 0 =>
            {
                state.ast_plan.remove(rule);
            }
            KnownNoMatchSubtree(rule) => {
                state.ast_plan.remove_subtree(rule);
            }
            _ => {}
        }
    }

    for sequence in retained_sequences_in_semantic_order(state) {
        let representation = choose_lossless_ast_representation(
            state.declaration_sequences[sequence].effects,
            authored_origins(sequence, state),
        )
        .expect("committed effect state is losslessly reifiable");
        state.ast_plan.represent(
            state.declaration_sequences[sequence].active_output_owner,
            representation,
        );
    }

    state.ast_plan.include_synthesized_rules(state);
    assert_ast_plan_does_not_expose_unclassified_edges(state);
}
```

## S5 AST reification commit

```rust,ignore
fn reify_ast(state: &mut MergeState) {
    assert_semantic_fixed_point(state);
    assert_ast_plan_complete(state);

    apply_planned_declarations_and_importance(state);
    apply_planned_synthesized_rules(state);
    apply_planned_removals_and_compact_rule_lists(state);
    clear_merge_only_storage_and_revisions(state);
    state.reified = true;
}
```

The minify-stage entry point sequences the semantic pass and AST commit:

```rust,ignore
fn run_cross_rule_merge(stylesheet: &mut Stylesheet) {
    let mut state = discover_merge_state(stylesheet);
    stabilize(&mut state);
    reify_ast(&mut state);
    assert!(is_minify_complete(&state));
}
```

S4 may plan an authored shorthand plus later overrides, typed longhands for
partially live virtual effects, or another proven equivalent representation.
S5 makes no new semantic or profitability decision: it writes that plan into
the final stylesheet AST. Code generation is a later, independent consumer of
that AST.

## Completion invariant

```rust,ignore
fn is_semantically_complete(state: &MergeState) -> bool {
    state.dirty_same_selector_edges.is_empty()
        && state.dirty_partial_edges.is_empty()
        && state.dirty_histories.is_empty()
        && state.candidates.is_empty()
        && state.histories.values().all(|history| {
            history.generation == history.consumed_generation
        })
}

fn is_minify_complete(state: &MergeState) -> bool {
    is_semantically_complete(state)
        && state.ast_plan.is_complete()
        && state.reified
}
```
