use std::hash::{Hash, Hasher};

use ahash::AHasher;
use rocketcss_allocator::{Allocator, boxed::Box, prelude::AdaptiveHashSet};
use rocketcss_ast::{
    Combinator, CssRule, Declaration, DeclarationBlock, EqIgnoringTombstones, PropertyId,
    PseudoElement, Selector, SelectorComponent, SelectorList, Span, StyleRule,
};

use super::{
    scheduler::Stabilizer,
    state::{
        CachedFingerprint, DeclarationEntryId, DeclarationEntryState, DeclarationOccurrence,
        EdgeId, EdgeStatus, HistoryId, RuleId, RuleState, SequenceId, SequenceState,
        SequenceSummary,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct FactorCandidateId(u32);

impl FactorCandidateId {
    fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone)]
pub(super) struct FactorCandidate {
    edge: EdgeId,
    order_label: u64,
    plan: FactorPlan,
}

#[derive(Clone)]
enum FactorPlan {
    Complete {
        members: std::vec::Vec<RuleId>,
        revisions: std::vec::Vec<u32>,
    },
    Partial {
        left: RuleId,
        right: RuleId,
        left_revision: u32,
        right_revision: u32,
        left_common_start: usize,
        right_common_start: usize,
        common_len: usize,
    },
}

impl<'list, 'scratch, 'ast> Stabilizer<'list, 'scratch, 'ast>
where
    'ast: 'scratch,
{
    pub(super) fn process_partial_edge(&mut self, edge: EdgeId) {
        if self.edges[edge.index()].status != EdgeStatus::DirtyPartial {
            return;
        }
        if !self.edge_is_current(edge) {
            self.edges[edge.index()].status = EdgeStatus::Stale;
            return;
        }
        let Some(candidate) = self.prepare_factor_candidate(edge) else {
            self.edges[edge.index()].status = EdgeStatus::Stable;
            return;
        };
        let id = FactorCandidateId(self.candidates.len() as u32);
        let order_label = candidate.order_label;
        self.candidates.push(Some(candidate));
        self.edges[edge.index()].status = EdgeStatus::Candidate(id.0);
        self.candidate_order
            .push(std::cmp::Reverse((order_label, id)));
    }

    pub(super) fn commit_leftmost_candidate(&mut self) -> bool {
        while let Some(std::cmp::Reverse((_, id))) = self.candidate_order.pop() {
            let Some(candidate) = self.candidates[id.index()].as_ref().cloned() else {
                continue;
            };
            if self.edges[candidate.edge.index()].status != EdgeStatus::Candidate(id.0)
                || !self.candidate_is_valid(&candidate)
            {
                self.candidates[id.index()] = None;
                continue;
            }
            let edge = candidate.edge;
            if self.commit_factor_candidate(candidate) {
                self.candidates[id.index()] = None;
                return true;
            }
            self.edges[edge.index()].status = EdgeStatus::Stable;
            self.candidates[id.index()] = None;
        }
        false
    }

    fn prepare_factor_candidate(&mut self, edge: EdgeId) -> Option<FactorCandidate> {
        let (left, right) = {
            let state = &self.edges[edge.index()];
            (state.left, state.right)
        };
        if !self.pair_is_eligible(edge) {
            return None;
        }
        let left_sequence = self.rule_states[left.index()].sequence;
        let right_sequence = self.rule_states[right.index()].sequence;
        self.ensure_sequence_summary(left_sequence);
        self.ensure_sequence_summary(right_sequence);

        let left_live_len = self.sequences[left_sequence.index()]
            .fingerprint
            .as_ref()
            .expect("summary is initialized")
            .summary
            .live_len;
        let empty_rules_can_factor = self.rule_states[left.index()].retained_child_count == 0
            && self.rule_states[right.index()].retained_child_count == 0;
        if (left_live_len != 0 || empty_rules_can_factor)
            && self.sequence_shapes_match(left_sequence, right_sequence)
            && self.exact_sequence_equal(left_sequence, right_sequence)
        {
            let mut members = std::vec![left, right];
            let mut current = right;
            while let Some(next) = self.rule_states[current.index()].next_live {
                let next_edge = self.rule_states[current.index()]
                    .next_edge
                    .expect("live neighbors have an edge");
                debug_assert_eq!(self.edges[next_edge.index()].right, next);
                if !self.pair_is_eligible(next_edge) {
                    break;
                }
                let sequence = self.rule_states[next.index()].sequence;
                self.ensure_sequence_summary(sequence);
                if left_live_len == 0 && self.rule_states[next.index()].retained_child_count != 0 {
                    break;
                }
                if !self.sequence_shapes_match(left_sequence, sequence)
                    || !self.exact_sequence_equal(left_sequence, sequence)
                {
                    break;
                }
                members.push(next);
                current = next;
            }
            let revisions = members
                .iter()
                .map(|member| {
                    let sequence = self.rule_states[member.index()].sequence;
                    self.sequences[sequence.index()].revision
                })
                .collect();
            return Some(FactorCandidate {
                edge,
                order_label: self.rule_states[left.index()].order_label,
                plan: FactorPlan::Complete { members, revisions },
            });
        }

        let (left_common_start, right_common_start, common_len) =
            self.best_common_range(left_sequence, right_sequence)?;
        Some(FactorCandidate {
            edge,
            order_label: self.rule_states[left.index()].order_label,
            plan: FactorPlan::Partial {
                left,
                right,
                left_revision: self.sequences[left_sequence.index()].revision,
                right_revision: self.sequences[right_sequence.index()].revision,
                left_common_start,
                right_common_start,
                common_len,
            },
        })
    }

    fn candidate_is_valid(&mut self, candidate: &FactorCandidate) -> bool {
        match &candidate.plan {
            FactorPlan::Complete { members, revisions } => {
                if members.len() != revisions.len() {
                    return false;
                }
                for (index, (&member, &revision)) in
                    members.iter().zip(revisions.iter()).enumerate()
                {
                    if !self.rule_states[member.index()].live {
                        return false;
                    }
                    let sequence = self.rule_states[member.index()].sequence;
                    if self.sequences[sequence.index()].revision != revision {
                        return false;
                    }
                    if index > 0 {
                        let previous = members[index - 1];
                        let Some(edge) = self.rule_states[previous.index()].next_edge else {
                            return false;
                        };
                        if self.rule_states[previous.index()].next_live != Some(member)
                            || self.edges[edge.index()].right != member
                            || !self.pair_is_eligible(edge)
                        {
                            return false;
                        }
                    }
                }
                let base = self.rule_states[members[0].index()].sequence;
                let base_is_empty = self.sequences[base.index()]
                    .fingerprint
                    .as_ref()
                    .expect("summary is initialized")
                    .summary
                    .live_len
                    == 0;
                members[1..].iter().all(|member| {
                    let sequence = self.rule_states[member.index()].sequence;
                    self.ensure_sequence_summary(sequence);
                    (!base_is_empty || self.rule_states[member.index()].retained_child_count == 0)
                        && self.sequence_shapes_match(base, sequence)
                        && self.exact_sequence_equal(base, sequence)
                })
            }
            FactorPlan::Partial {
                left,
                right,
                left_revision,
                right_revision,
                ..
            } => {
                self.edge_is_current(candidate.edge)
                    && self.pair_is_eligible(candidate.edge)
                    && self.sequences[self.rule_states[left.index()].sequence.index()].revision
                        == *left_revision
                    && self.sequences[self.rule_states[right.index()].sequence.index()].revision
                        == *right_revision
            }
        }
    }

    fn pair_is_eligible(&self, edge: EdgeId) -> bool {
        let edge = &self.edges[edge.index()];
        let left = edge.left;
        let right = edge.right;
        let left_rule = self.rule(left);
        let right_rule = self.rule(right);
        let left_summary = self.rule_states[left.index()].selector_summary;
        let right_summary = self.rule_states[right.index()].selector_summary;
        left_rule.rules.is_empty()
            && left_rule.vendor_prefix == right_rule.vendor_prefix
            && !edge.same_selector
            && left_summary.vendor_prefixes == right_summary.vendor_prefixes
            && left_summary.materializable
            && right_summary.materializable
    }

    fn ensure_sequence_summary(&mut self, sequence: SequenceId) {
        let revision = self.sequences[sequence.index()].revision;
        if self.sequences[sequence.index()]
            .fingerprint
            .as_ref()
            .is_some_and(|cached| cached.revision == revision)
        {
            return;
        }
        let entries = self.sequences[sequence.index()].blocks.clone();
        let mut occurrences = std::vec::Vec::new();
        let mut hasher = AHasher::default();
        for entry in entries {
            let block = self.entry_block(entry);
            for (index, (declaration, important)) in block.get().get_ref().iter().enumerate() {
                if declaration.is_tombstone() {
                    continue;
                }
                let history_context = self.cascade_scope.declaration_context(important);
                let mut occurrence_hasher = AHasher::default();
                history_context.hash(&mut occurrence_hasher);
                if let Some((property, prefix)) = declaration.known_id_and_prefix() {
                    0u8.hash(&mut occurrence_hasher);
                    property.hash(&mut occurrence_hasher);
                    prefix.bits().hash(&mut occurrence_hasher);
                } else if let Some(property) = declaration.property_id() {
                    1u8.hash(&mut occurrence_hasher);
                    property.hash(&mut occurrence_hasher);
                }
                let shape_hash = occurrence_hasher.finish();
                shape_hash.hash(&mut hasher);
                occurrences.push(DeclarationOccurrence {
                    block,
                    entry,
                    index,
                    history_context,
                    shape_hash,
                });
            }
        }
        let live_len = occurrences.len() as u32;
        live_len.hash(&mut hasher);
        self.sequences[sequence.index()].fingerprint = Some(CachedFingerprint {
            revision,
            summary: SequenceSummary {
                live_len,
                shape_hash: hasher.finish(),
                occurrences,
            },
        });
    }

    fn sequence_shapes_match(&self, left: SequenceId, right: SequenceId) -> bool {
        let left = &self.sequences[left.index()]
            .fingerprint
            .as_ref()
            .expect("summary is initialized")
            .summary;
        let right = &self.sequences[right.index()]
            .fingerprint
            .as_ref()
            .expect("summary is initialized")
            .summary;
        left.live_len == right.live_len && left.shape_hash == right.shape_hash
    }

    fn exact_sequence_equal(&self, left: SequenceId, right: SequenceId) -> bool {
        let left = &self.sequences[left.index()]
            .fingerprint
            .as_ref()
            .expect("summary is initialized")
            .summary
            .occurrences;
        let right = &self.sequences[right.index()]
            .fingerprint
            .as_ref()
            .expect("summary is initialized")
            .summary
            .occurrences;
        left.len() == right.len()
            && left
                .iter()
                .zip(right)
                .all(|(&left, &right)| self.occurrences_equal(left, right))
    }

    fn best_common_range(
        &self,
        left: SequenceId,
        right: SequenceId,
    ) -> Option<(usize, usize, usize)> {
        let left = &self.sequences[left.index()]
            .fingerprint
            .as_ref()
            .expect("summary is initialized")
            .summary
            .occurrences;
        let right = &self.sequences[right.index()]
            .fingerprint
            .as_ref()
            .expect("summary is initialized")
            .summary
            .occurrences;
        if left.is_empty() || right.is_empty() {
            return None;
        }
        if left.len() == 1 {
            return right
                .iter()
                .enumerate()
                .find_map(|(right_start, &right_occurrence)| {
                    (self.occurrences_equal(left[0], right_occurrence)
                        && self.movement_is_safe(left, right, 0, right_start, 1))
                    .then_some((0, right_start, 1))
                });
        }
        if right.len() == 1 {
            return left
                .iter()
                .enumerate()
                .find_map(|(left_start, &left_occurrence)| {
                    (self.occurrences_equal(left_occurrence, right[0])
                        && self.movement_is_safe(left, right, left_start, 0, 1))
                    .then_some((left_start, 0, 1))
                });
        }
        let mut previous = std::vec![0usize; right.len() + 1];
        let mut current = std::vec![0usize; right.len() + 1];
        let mut best_len = 0;
        let mut best = std::vec::Vec::new();
        for left_end in 1..=left.len() {
            current.fill(0);
            for right_end in 1..=right.len() {
                if self.occurrences_equal(left[left_end - 1], right[right_end - 1]) {
                    let len = previous[right_end - 1] + 1;
                    current[right_end] = len;
                    let candidate = (left_end - len, right_end - len, len);
                    if len > best_len {
                        best_len = len;
                        best.clear();
                        best.push(candidate);
                    } else if len == best_len {
                        best.push(candidate);
                    }
                }
            }
            std::mem::swap(&mut previous, &mut current);
        }
        best.sort_unstable();
        best.dedup();
        best.into_iter().find(|&(left_start, right_start, len)| {
            len != 0 && self.movement_is_safe(left, right, left_start, right_start, len)
        })
    }

    fn occurrences_equal(
        &self,
        left: DeclarationOccurrence<'ast>,
        right: DeclarationOccurrence<'ast>,
    ) -> bool {
        left.shape_hash == right.shape_hash
            && left.history_context == right.history_context
            && self
                .occurrence_declaration(left)
                .eq_ignoring_tombstones(self.occurrence_declaration(right))
    }

    fn movement_is_safe(
        &self,
        left: &[DeclarationOccurrence<'ast>],
        right: &[DeclarationOccurrence<'ast>],
        left_start: usize,
        right_start: usize,
        len: usize,
    ) -> bool {
        let common = &left[left_start..left_start + len];
        let crossed_left = &left[left_start + len..];
        let crossed_right = &right[..right_start];
        !common.iter().any(|&common| {
            crossed_left
                .iter()
                .chain(crossed_right)
                .any(|&crossed| self.declarations_conflict(common, crossed))
        })
    }

    fn declarations_conflict(
        &self,
        left: DeclarationOccurrence<'ast>,
        right: DeclarationOccurrence<'ast>,
    ) -> bool {
        if left.history_context != right.history_context {
            return true;
        }
        let left = self.occurrence_declaration(left);
        let right = self.occurrence_declaration(right);
        let (Some(left_id), Some(right_id)) = (left.property_id(), right.property_id()) else {
            return true;
        };
        if left_id == right_id
            || matches!(left_id, PropertyId::All)
            || matches!(right_id, PropertyId::All)
        {
            return true;
        }
        if matches!(left, Declaration::Custom(_)) || matches!(right, Declaration::Custom(_)) {
            return false;
        }
        if matches!(left_id, PropertyId::Custom(_) | PropertyId::Unparsed)
            || matches!(right_id, PropertyId::Custom(_) | PropertyId::Unparsed)
        {
            return true;
        }
        property_relation(left_id)
            .zip(property_relation(right_id))
            .is_some_and(|(left, right)| {
                left.family == right.family && (left.shorthand || right.shorthand)
            })
    }

    fn occurrence_declaration(
        &self,
        occurrence: DeclarationOccurrence<'ast>,
    ) -> &Declaration<'ast> {
        &occurrence.block.get().get_ref().declarations[occurrence.index]
    }

    fn commit_factor_candidate(&mut self, candidate: FactorCandidate) -> bool {
        let members = match &candidate.plan {
            FactorPlan::Complete { members, .. } => members.clone(),
            FactorPlan::Partial { left, right, .. } => std::vec![*left, *right],
        };
        let allocator = self.rules.bump();
        let selector_lists = members
            .iter()
            .map(|&member| &*self.rule(member).selectors)
            .collect::<std::vec::Vec<_>>();
        let Some(selectors) = clone_selector_union(&selector_lists, allocator) else {
            return false;
        };

        let mut shared_declarations = DeclarationBlock::new(allocator);
        match &candidate.plan {
            FactorPlan::Complete { members, .. } => {
                let base_sequence = self.rule_states[members[0].index()].sequence;
                let common_len = self.sequences[base_sequence.index()]
                    .fingerprint
                    .as_ref()
                    .expect("validated candidate has a summary")
                    .summary
                    .occurrences
                    .len();
                for offset in 0..common_len {
                    let base = self.sequence_occurrence(base_sequence, offset);
                    let declaration = self.replace_occurrence(base, Declaration::Tombstone);
                    shared_declarations.push(declaration, base.history_context.is_important());
                    for &member in &members[1..] {
                        let sequence = self.rule_states[member.index()].sequence;
                        let occurrence = self.sequence_occurrence(sequence, offset);
                        self.replace_occurrence(occurrence, Declaration::Tombstone);
                        self.cx.record_declaration_removed();
                    }
                }
            }
            FactorPlan::Partial {
                left,
                right,
                left_common_start,
                right_common_start,
                common_len,
                ..
            } => {
                let left_sequence = self.rule_states[left.index()].sequence;
                let right_sequence = self.rule_states[right.index()].sequence;
                for offset in 0..*common_len {
                    let left = self.sequence_occurrence(left_sequence, left_common_start + offset);
                    let right =
                        self.sequence_occurrence(right_sequence, right_common_start + offset);
                    let declaration = self.replace_occurrence(left, Declaration::Tombstone);
                    self.replace_occurrence(right, Declaration::Tombstone);
                    shared_declarations.push(declaration, left.history_context.is_important());
                    self.cx.record_declaration_removed();
                }
            }
        }

        let first = members[0];
        let second = members[1];
        let last = *members.last().expect("factor candidate has members");
        let span = Span {
            start: members
                .iter()
                .map(|&member| self.rule(member).span.start)
                .min()
                .expect("factor candidate has members"),
            end: members
                .iter()
                .map(|&member| self.rule(member).span.end)
                .max()
                .expect("factor candidate has members"),
        };
        let vendor_prefix = self.rule(first).vendor_prefix;
        let order_label = self.allocate_order_between(first, second);
        let slot = self.storage.len();
        let selector_summary = super::summarize_selectors(&selectors);
        self.storage
            .push(Some(CssRule::Style(allocator.boxed(StyleRule {
                declarations: allocator.pinned(shared_declarations),
                span,
                rules: allocator.vec(),
                selectors,
                vendor_prefix,
            }))));
        self.slot_order_labels.push(order_label);
        self.slot_to_rule.push(None);

        let shared = RuleId(self.rule_states.len() as u32);
        let sequence = SequenceId(self.sequences.len() as u32);
        let entry = DeclarationEntryId(self.declaration_entries.len() as u32);
        let block = {
            let CssRule::Style(rule) = self.storage[slot].as_ref().expect("shared rule stored")
            else {
                unreachable!()
            };
            rocketcss_allocator::Ref::from_pinned_box(&rule.declarations)
        };
        self.declaration_entries
            .push(DeclarationEntryState { block, sequence });
        self.sequences.push(SequenceState {
            blocks: std::vec![entry],
            revision: 0,
            fingerprint: None,
            owner: shared,
        });
        self.rule_states.push(RuleState {
            ast_slot: slot,
            selector_summary,
            live: true,
            previous_live: None,
            next_live: None,
            previous_edge: None,
            next_edge: None,
            segment: self.rule_states[first.index()].segment,
            sequence,
            history: HistoryId(u32::MAX),
            retained_child_count: 0,
            order_label,
        });
        self.slot_to_rule[slot] = Some(shared);
        let history = self.register_rule_history(shared);
        self.rule_states[shared.index()].history = history;
        self.queue_history(history);

        let mut changed_sequences = std::collections::HashSet::new();
        let mut changed_histories = std::collections::HashSet::new();
        for &member in &members {
            changed_sequences.insert(self.rule_states[member.index()].sequence);
            changed_histories.insert(self.rule_states[member.index()].history);
        }
        for sequence in changed_sequences {
            self.mark_sequence_changed(sequence);
        }
        for history in changed_histories {
            self.history_changed(history);
        }

        let mut replacement = std::vec::Vec::new();
        match &candidate.plan {
            FactorPlan::Complete { members, .. } => {
                for &member in members {
                    let sequence = self.rule_states[member.index()].sequence;
                    let empty = self.sequence_is_empty(sequence);
                    let keep = !empty || self.rule_states[member.index()].retained_child_count != 0;
                    self.rule_states[member.index()].live = keep;
                }
                replacement.push(shared);
                if self.rule_states[last.index()].live {
                    replacement.push(last);
                }
            }
            FactorPlan::Partial { left, right, .. } => {
                for &member in [left, right] {
                    let sequence = self.rule_states[member.index()].sequence;
                    let empty = self.sequence_is_empty(sequence);
                    let keep = !empty || self.rule_states[member.index()].retained_child_count != 0;
                    self.rule_states[member.index()].live = keep;
                }
                if self.rule_states[left.index()].live {
                    replacement.push(*left);
                }
                replacement.push(shared);
                if self.rule_states[right.index()].live {
                    replacement.push(*right);
                }
            }
        }
        self.atomic_reconnect(&members, &replacement);
        self.changed = true;
        true
    }

    fn sequence_occurrence(
        &self,
        sequence: SequenceId,
        index: usize,
    ) -> DeclarationOccurrence<'ast> {
        self.sequences[sequence.index()]
            .fingerprint
            .as_ref()
            .expect("validated candidate has a summary")
            .summary
            .occurrences[index]
    }

    fn replace_occurrence(
        &mut self,
        mut occurrence: DeclarationOccurrence<'ast>,
        replacement: Declaration<'ast>,
    ) -> Declaration<'ast> {
        debug_assert_eq!(
            self.declaration_entries[occurrence.entry.index()].block,
            occurrence.block
        );
        // SAFETY: candidate commit is the only mutation phase, and every
        // occurrence selected by a validated plan is replaced exactly once.
        let block = unsafe { occurrence.block.get_mut_unchecked().get_unchecked_mut() };
        std::mem::replace(&mut block.declarations[occurrence.index], replacement)
    }

    fn sequence_is_empty(&self, sequence: SequenceId) -> bool {
        self.sequences[sequence.index()]
            .blocks
            .iter()
            .all(|&entry| {
                self.entry_block(entry)
                    .get()
                    .get_ref()
                    .declarations
                    .iter()
                    .all(Declaration::is_tombstone)
            })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PropertyFamily {
    Background,
    Border,
    Flex,
    Inset,
    Margin,
    Overflow,
    Padding,
    PlaceContent,
}

#[derive(Clone, Copy)]
struct PropertyRelation {
    family: PropertyFamily,
    shorthand: bool,
}

fn property_relation(property: PropertyId<'_>) -> Option<PropertyRelation> {
    let relation = match property {
        PropertyId::Background => (PropertyFamily::Background, true),
        PropertyId::BackgroundAttachment
        | PropertyId::BackgroundClip(_)
        | PropertyId::BackgroundColor
        | PropertyId::BackgroundImage
        | PropertyId::BackgroundOrigin
        | PropertyId::BackgroundPosition
        | PropertyId::BackgroundPositionX
        | PropertyId::BackgroundPositionY
        | PropertyId::BackgroundRepeat
        | PropertyId::BackgroundSize => (PropertyFamily::Background, false),
        PropertyId::Border
        | PropertyId::BorderBlock
        | PropertyId::BorderBlockColor
        | PropertyId::BorderBlockEnd
        | PropertyId::BorderBlockEndColor
        | PropertyId::BorderBlockEndStyle
        | PropertyId::BorderBlockEndWidth
        | PropertyId::BorderBlockStart
        | PropertyId::BorderBlockStartColor
        | PropertyId::BorderBlockStartStyle
        | PropertyId::BorderBlockStartWidth
        | PropertyId::BorderBlockStyle
        | PropertyId::BorderBlockWidth
        | PropertyId::BorderBottom
        | PropertyId::BorderBottomColor
        | PropertyId::BorderBottomStyle
        | PropertyId::BorderBottomWidth
        | PropertyId::BorderColor
        | PropertyId::BorderInline
        | PropertyId::BorderInlineColor
        | PropertyId::BorderInlineEnd
        | PropertyId::BorderInlineEndColor
        | PropertyId::BorderInlineEndStyle
        | PropertyId::BorderInlineEndWidth
        | PropertyId::BorderInlineStart
        | PropertyId::BorderInlineStartColor
        | PropertyId::BorderInlineStartStyle
        | PropertyId::BorderInlineStartWidth
        | PropertyId::BorderInlineStyle
        | PropertyId::BorderInlineWidth
        | PropertyId::BorderLeft
        | PropertyId::BorderLeftColor
        | PropertyId::BorderLeftStyle
        | PropertyId::BorderLeftWidth
        | PropertyId::BorderRight
        | PropertyId::BorderRightColor
        | PropertyId::BorderRightStyle
        | PropertyId::BorderRightWidth
        | PropertyId::BorderStyle
        | PropertyId::BorderTop
        | PropertyId::BorderTopColor
        | PropertyId::BorderTopStyle
        | PropertyId::BorderTopWidth
        | PropertyId::BorderWidth => (PropertyFamily::Border, true),
        PropertyId::Flex(_) => (PropertyFamily::Flex, true),
        PropertyId::FlexBasis(_) | PropertyId::FlexGrow(_) | PropertyId::FlexShrink(_) => {
            (PropertyFamily::Flex, false)
        }
        PropertyId::Inset | PropertyId::InsetBlock | PropertyId::InsetInline => {
            (PropertyFamily::Inset, true)
        }
        PropertyId::InsetBlockEnd
        | PropertyId::InsetBlockStart
        | PropertyId::InsetInlineEnd
        | PropertyId::InsetInlineStart
        | PropertyId::Bottom
        | PropertyId::Left
        | PropertyId::Right
        | PropertyId::Top => (PropertyFamily::Inset, false),
        PropertyId::Margin | PropertyId::MarginBlock | PropertyId::MarginInline => {
            (PropertyFamily::Margin, true)
        }
        PropertyId::MarginBlockEnd
        | PropertyId::MarginBlockStart
        | PropertyId::MarginBottom
        | PropertyId::MarginInlineEnd
        | PropertyId::MarginInlineStart
        | PropertyId::MarginLeft
        | PropertyId::MarginRight
        | PropertyId::MarginTop => (PropertyFamily::Margin, false),
        PropertyId::Overflow => (PropertyFamily::Overflow, true),
        PropertyId::OverflowX | PropertyId::OverflowY => (PropertyFamily::Overflow, false),
        PropertyId::Padding | PropertyId::PaddingBlock | PropertyId::PaddingInline => {
            (PropertyFamily::Padding, true)
        }
        PropertyId::PaddingBlockEnd
        | PropertyId::PaddingBlockStart
        | PropertyId::PaddingBottom
        | PropertyId::PaddingInlineEnd
        | PropertyId::PaddingInlineStart
        | PropertyId::PaddingLeft
        | PropertyId::PaddingRight
        | PropertyId::PaddingTop => (PropertyFamily::Padding, false),
        PropertyId::PlaceContent => (PropertyFamily::PlaceContent, true),
        PropertyId::AlignContent(_) | PropertyId::JustifyContent(_) => {
            (PropertyFamily::PlaceContent, false)
        }
        _ => return None,
    };
    Some(PropertyRelation {
        family: relation.0,
        shorthand: relation.1,
    })
}

fn clone_selector_union<'ast>(
    lists: &[&SelectorList<'ast>],
    allocator: &'ast Allocator,
) -> Option<Box<'ast, SelectorList<'ast>>> {
    let mut union = allocator.vec();
    let mut seen = AdaptiveHashSet::<_, 4>::new_in(allocator);
    for selector in lists
        .iter()
        .flat_map(|selectors| selectors.iter())
        .filter(|selector| !selector.is_tombstone())
    {
        if !seen.insert(selector) {
            continue;
        }
        union.push(clone_selector(selector, allocator)?);
    }
    Some(allocator.boxed(union))
}

fn clone_selector<'ast>(
    selector: &Selector<'ast>,
    allocator: &'ast Allocator,
) -> Option<Selector<'ast>> {
    let Selector::Parsed(components) = selector else {
        return None;
    };
    let mut cloned = allocator.vec();
    for component in components {
        cloned.push(clone_selector_component(component, allocator)?);
    }
    Some(Selector::Parsed(cloned))
}

fn clone_selector_component<'ast>(
    component: &SelectorComponent<'ast>,
    allocator: &'ast Allocator,
) -> Option<SelectorComponent<'ast>> {
    Some(match component {
        SelectorComponent::Combinator(Combinator::Descendant) => {
            SelectorComponent::Combinator(Combinator::Descendant)
        }
        SelectorComponent::ExplicitAnyNamespace => SelectorComponent::ExplicitAnyNamespace,
        SelectorComponent::ExplicitNoNamespace => SelectorComponent::ExplicitNoNamespace,
        SelectorComponent::DefaultNamespace(value) => SelectorComponent::DefaultNamespace(value),
        SelectorComponent::Namespace { prefix, url } => {
            SelectorComponent::Namespace { prefix, url }
        }
        SelectorComponent::ExplicitUniversalType => SelectorComponent::ExplicitUniversalType,
        SelectorComponent::LocalName { name, lower_name } => {
            SelectorComponent::LocalName { name, lower_name }
        }
        SelectorComponent::Id(value) => SelectorComponent::Id(value),
        SelectorComponent::Class(value) => SelectorComponent::Class(value),
        SelectorComponent::Root => SelectorComponent::Root,
        SelectorComponent::Empty => SelectorComponent::Empty,
        SelectorComponent::Scope => SelectorComponent::Scope,
        SelectorComponent::Nth(value) => SelectorComponent::Nth(*value),
        SelectorComponent::PseudoElement(value) => {
            SelectorComponent::PseudoElement(allocator.boxed(clone_simple_pseudo_element(value)?))
        }
        SelectorComponent::Part(values) => {
            let mut cloned = allocator.vec();
            cloned.extend_from_slice_copy(values);
            SelectorComponent::Part(cloned)
        }
        SelectorComponent::Nesting => SelectorComponent::Nesting,
        _ => return None,
    })
}

fn clone_simple_pseudo_element<'ast>(pseudo: &PseudoElement<'ast>) -> Option<PseudoElement<'ast>> {
    Some(match pseudo {
        PseudoElement::After => PseudoElement::After,
        PseudoElement::Before => PseudoElement::Before,
        PseudoElement::FirstLine => PseudoElement::FirstLine,
        PseudoElement::FirstLetter => PseudoElement::FirstLetter,
        PseudoElement::DetailsContent => PseudoElement::DetailsContent,
        PseudoElement::TargetText => PseudoElement::TargetText,
        PseudoElement::SearchText => PseudoElement::SearchText,
        PseudoElement::Selection(prefix) => PseudoElement::Selection(*prefix),
        PseudoElement::Placeholder(prefix) => PseudoElement::Placeholder(*prefix),
        PseudoElement::HighlightFunction { name } => PseudoElement::HighlightFunction { name },
        PseudoElement::Marker => PseudoElement::Marker,
        PseudoElement::Backdrop(prefix) => PseudoElement::Backdrop(*prefix),
        PseudoElement::FileSelectorButton(prefix) => PseudoElement::FileSelectorButton(*prefix),
        PseudoElement::WebKitScrollbar(value) => PseudoElement::WebKitScrollbar(*value),
        PseudoElement::Cue => PseudoElement::Cue,
        PseudoElement::CueRegion => PseudoElement::CueRegion,
        PseudoElement::ViewTransition => PseudoElement::ViewTransition,
        PseudoElement::PickerFunction { identifier } => {
            PseudoElement::PickerFunction { identifier }
        }
        PseudoElement::PickerIcon => PseudoElement::PickerIcon,
        PseudoElement::Checkmark => PseudoElement::Checkmark,
        PseudoElement::GrammarError => PseudoElement::GrammarError,
        PseudoElement::SpellingError => PseudoElement::SpellingError,
        _ => return None,
    })
}
