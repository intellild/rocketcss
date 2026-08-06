//! Persistent cross-rule scheduler over transform-local state.
//!
//! The scheduler performs one source-order AST scan, then runs S1/S2/S3 only
//! against dense local nodes. AST mutations are described by a terminal
//! final-state reification plan and are applied after the fixed point.

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, VecDeque},
};

use rocketcss_ast::{
    CssDeclaration, CssDeclarationBlockId as DeclarationBlockId, CssRule, CssRuleId as RuleId,
    Declaration, EffectiveKeyId, EqIgnoringTombstones, FinalReificationRule,
    FinalReificationRuleList, FinalReificationSelector,
    PendingReificationEffectiveKey as PendingEffectiveKey,
    ReificationDeclarationId as DeclarationNodeId,
    ReificationEffectiveKey as TransformEffectiveKey, ReificationPlan, ReificationRuleId as NodeId,
    ReificationSelectorId as LocalSelectorId, ReificationStep, SelectorFrameKind, SelectorList,
    SelectorValueId, Span, StyleSheet, StyleSheetMutationError as MutationError, VendorPrefix,
};
use rocketcss_common::Allocator;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use super::{
    declaration_ir::{CompactPropertyKey, DeclarationIrClassifier, MovementDomain, PropertyBloom},
    partial_selector::materialize_selector_union,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct CrossRuleStats {
    pub(crate) initial_scans: u32,
    pub(crate) scheduler_ast_mutations: u32,
    pub(crate) reification_passes: u32,
    pub(crate) live_endpoint_reuses: u32,
    pub(crate) rule_tombstone_reuses: u32,
    pub(crate) block_tombstone_reuses: u32,
    pub(crate) declaration_tombstone_reuses: u32,
    pub(crate) residual_rule_inserts: u32,
    pub(crate) residual_declaration_inserts: u32,
    pub(crate) radix_relabel_groups: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SemanticOrderKey(SmallVec<[u32; 2]>);

impl SemanticOrderKey {
    fn initial(index: usize) -> Self {
        let value = u32::try_from(index)
            .ok()
            .and_then(|index| index.checked_add(1))
            .expect("rule count exceeds u32::MAX");
        Self(SmallVec::from_slice(&[value]))
    }

    fn between(left: &Self, right: &Self) -> Self {
        debug_assert!(left < right);
        let mut digits = SmallVec::new();
        let mut index = 0;
        let mut upper_is_open = false;
        loop {
            let lower = left.0.get(index).copied().unwrap_or(0);
            let upper = if upper_is_open {
                u32::MAX
            } else {
                right.0.get(index).copied().unwrap_or(u32::MAX)
            };
            if upper - lower > 1 {
                digits.push(lower + (upper - lower) / 2);
                return Self(digits);
            }
            digits.push(lower);
            upper_is_open |= lower < upper;
            index += 1;
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum SelectorSource {
    Existing(SelectorValueId),
    Synthesized(LocalSelectorId),
}

#[derive(Clone, Copy, Debug)]
struct StyleState {
    selector: SelectorSource,
    kind: SelectorFrameKind,
    vendor_prefix: VendorPrefix,
    span: Span,
}

#[derive(Debug)]
struct LiveRule<'ast> {
    source_rule: Option<RuleId<'ast>>,
    source_block: Option<DeclarationBlockId<'ast>>,
    parent: Option<NodeId>,
    previous_live: Option<NodeId>,
    next_live: Option<NodeId>,
    order: SemanticOrderKey,
    effective_key: Option<TransformEffectiveKey>,
    declarations: std::vec::Vec<DeclarationNodeId>,
    style: Option<StyleState>,
    has_children: bool,
    live: bool,
    generation: u32,
}

#[derive(Clone, Copy, Debug)]
struct VirtualDeclaration {
    source: rocketcss_ast::DeclarationId,
    property_key: Option<CompactPropertyKey>,
    movement_domain: Option<MovementDomain>,
    live: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct EdgeCandidate {
    left: NodeId,
    right: NodeId,
    left_generation: u32,
    right_generation: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PartialCandidateKey {
    order: SemanticOrderKey,
    candidate: EdgeCandidate,
}

impl PartialOrd for EdgeCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EdgeCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.left
            .cmp(&other.left)
            .then_with(|| self.right.cmp(&other.right))
            .then_with(|| self.left_generation.cmp(&other.left_generation))
            .then_with(|| self.right_generation.cmp(&other.right_generation))
    }
}

#[derive(Default)]
struct CandidateQueue {
    pending: VecDeque<EdgeCandidate>,
    queued: FxHashSet<EdgeCandidate>,
}

impl CandidateQueue {
    fn push(&mut self, candidate: EdgeCandidate) {
        if self.queued.insert(candidate) {
            self.pending.push_back(candidate);
        }
    }

    fn pop(&mut self) -> Option<EdgeCandidate> {
        let candidate = self.pending.pop_front()?;
        self.queued.remove(&candidate);
        Some(candidate)
    }
}

#[derive(Default)]
struct PartialCandidateQueue {
    pending: BinaryHeap<Reverse<PartialCandidateKey>>,
    queued: FxHashSet<EdgeCandidate>,
}

impl PartialCandidateQueue {
    fn push(&mut self, candidate: EdgeCandidate, order: SemanticOrderKey) {
        if self.queued.insert(candidate) {
            self.pending
                .push(Reverse(PartialCandidateKey { order, candidate }));
        }
    }

    fn pop(&mut self) -> Option<EdgeCandidate> {
        let candidate = self.pending.pop()?.0.candidate;
        self.queued.remove(&candidate);
        Some(candidate)
    }
}

#[derive(Default)]
struct DirtyEffectiveKeys {
    pending: VecDeque<TransformEffectiveKey>,
    queued: FxHashSet<TransformEffectiveKey>,
}

impl DirtyEffectiveKeys {
    fn push(&mut self, key: TransformEffectiveKey) {
        if self.queued.insert(key) {
            self.pending.push_back(key);
        }
    }

    fn pop(&mut self) -> Option<TransformEffectiveKey> {
        let key = self.pending.pop_front()?;
        self.queued.remove(&key);
        Some(key)
    }
}

pub(super) struct CrossRuleScheduler<'arena, 'ast> {
    allocator: &'arena Allocator,
    nodes: std::vec::Vec<LiveRule<'ast>>,
    declarations: std::vec::Vec<VirtualDeclaration>,
    selectors: std::vec::Vec<SelectorList<'ast>>,
    pending_keys: std::vec::Vec<PendingEffectiveKey>,
    histories: FxHashMap<TransformEffectiveKey, BTreeMap<SemanticOrderKey, NodeId>>,
    same_selector_candidates: CandidateQueue,
    declaration_override_candidates: DirtyEffectiveKeys,
    partial_merge_candidates: PartialCandidateQueue,
    steps: std::vec::Vec<ReificationStep>,
    stats: CrossRuleStats,
}

impl<'arena, 'ast> CrossRuleScheduler<'arena, 'ast> {
    pub(super) fn from_stylesheet(
        stylesheet: &StyleSheet<'ast>,
        allocator: &'arena Allocator,
    ) -> Result<Self, MutationError<'ast>> {
        let mut classifier =
            DeclarationIrClassifier::with_capacity_in(stylesheet.declaration_count(), allocator);
        let mut nodes: std::vec::Vec<LiveRule<'ast>> =
            std::vec::Vec::with_capacity(stylesheet.rule_count());
        let mut declarations = std::vec::Vec::with_capacity(stylesheet.declaration_count());
        let mut source_nodes = FxHashMap::default();
        let mut previous_by_parent = FxHashMap::default();
        let mut histories: FxHashMap<_, BTreeMap<_, _>> = FxHashMap::default();

        for entry in stylesheet.rule_tree_entries() {
            let source_rule = entry.rule();
            let parent = entry
                .parent()
                .map(|parent| {
                    source_nodes
                        .get(&parent)
                        .copied()
                        .ok_or(MutationError::InvalidRuleTopology(parent))
                })
                .transpose()?;
            let id = NodeId(u32::try_from(nodes.len()).expect("rule count exceeds u32::MAX"));
            let previous_live = previous_by_parent.insert(parent, id);
            if let Some(previous) = previous_live {
                nodes[previous.index()].next_live = Some(id);
            }

            let record = entry.record();
            let source_block = record.declaration_block();
            let mut declaration_nodes = std::vec::Vec::new();
            let mut effective_key = None;
            if let Some(block) = source_block {
                let block_record = stylesheet
                    .declaration_block(block)
                    .ok_or(MutationError::UnknownDeclarationBlock(block))?;
                effective_key = Some(TransformEffectiveKey::Existing(
                    block_record.effective_key(),
                ));
                for (declaration, declaration_record) in
                    stylesheet.declaration_occurrences_in_block(block)?
                {
                    let declaration_id = DeclarationNodeId(
                        u32::try_from(declarations.len())
                            .expect("declaration count exceeds u32::MAX"),
                    );
                    let (property_key, movement_domain, live) = match declaration_record.payload() {
                        CssDeclaration::Property(value) => {
                            let live = !matches!(value, Declaration::Tombstone);
                            (
                                live.then(|| {
                                    classifier
                                        .property_key(value, declaration_record.is_important())
                                })
                                .flatten(),
                                live.then(|| classifier.movement_domain(value)).flatten(),
                                live,
                            )
                        }
                        _ => (None, None, true),
                    };
                    declarations.push(VirtualDeclaration {
                        source: declaration,
                        property_key,
                        movement_domain,
                        live,
                    });
                    declaration_nodes.push(declaration_id);
                }
            }

            let style = style_state(record.payload());
            let order = SemanticOrderKey::initial(nodes.len());
            nodes.push(LiveRule {
                source_rule: Some(source_rule),
                source_block,
                parent,
                previous_live,
                next_live: None,
                order: order.clone(),
                effective_key,
                declarations: declaration_nodes,
                style,
                has_children: entry.event().has_children(),
                live: true,
                generation: 0,
            });
            source_nodes.insert(source_rule, id);
            if let Some(key) = effective_key {
                histories.entry(key).or_default().insert(order, id);
            }
        }

        let mut scheduler = Self {
            allocator,
            nodes,
            declarations,
            selectors: std::vec::Vec::new(),
            pending_keys: std::vec::Vec::new(),
            histories,
            same_selector_candidates: CandidateQueue::default(),
            declaration_override_candidates: DirtyEffectiveKeys::default(),
            partial_merge_candidates: PartialCandidateQueue::default(),
            steps: std::vec::Vec::new(),
            stats: CrossRuleStats {
                initial_scans: 1,
                ..CrossRuleStats::default()
            },
        };
        for index in 0..scheduler.nodes.len() {
            let left = NodeId(index as u32);
            if let Some(right) = scheduler.nodes[index].next_live {
                scheduler.enqueue_edge(left, right);
            }
        }
        let dirty = scheduler
            .histories
            .iter()
            .filter_map(|(&key, history)| (history.len() > 1).then_some(key))
            .collect::<std::vec::Vec<_>>();
        for key in dirty {
            scheduler.declaration_override_candidates.push(key);
        }
        Ok(scheduler)
    }

    pub(super) fn stabilize(
        mut self,
        stylesheet: &StyleSheet<'ast>,
        preserve_selector_compatibility: bool,
    ) -> (ReificationPlan<'ast>, CrossRuleStats) {
        loop {
            if let Some(candidate) = self.same_selector_candidates.pop() {
                self.commit_s1(candidate);
                continue;
            }
            if let Some(key) = self.declaration_override_candidates.pop() {
                self.commit_s2(stylesheet, key);
                continue;
            }
            if let Some(candidate) = self.partial_merge_candidates.pop() {
                self.commit_s3(stylesheet, candidate, preserve_selector_compatibility);
                continue;
            }
            break;
        }
        let mut affected_nodes = FxHashSet::default();
        for step in &self.steps {
            match step {
                ReificationStep::MergeAdjacent { left, right } => {
                    affected_nodes.insert(*left);
                    affected_nodes.insert(*right);
                }
                ReificationStep::RemoveDeclaration { owner, .. } => {
                    affected_nodes.insert(*owner);
                }
                ReificationStep::RetireRule { rule } => {
                    affected_nodes.insert(*rule);
                }
                ReificationStep::ReuseLeftForPartialMerge { left, right, .. } => {
                    affected_nodes.insert(*left);
                    affected_nodes.insert(*right);
                }
                ReificationStep::InsertPartialMerge {
                    left,
                    right,
                    synthesized,
                    ..
                } => {
                    affected_nodes.insert(*left);
                    affected_nodes.insert(*right);
                    affected_nodes.insert(*synthesized);
                }
            }
        }
        let affected_parents = affected_nodes
            .iter()
            .map(|node| self.nodes[node.index()].parent)
            .collect::<FxHashSet<_>>();
        let mut final_rule_lists = std::vec::Vec::with_capacity(affected_parents.len());
        for parent in affected_parents {
            let mut rules = self
                .nodes
                .iter()
                .enumerate()
                .filter(|(_, node)| node.live && node.parent == parent)
                .map(|(index, node)| (node.order.clone(), NodeId(index as u32)))
                .collect::<std::vec::Vec<_>>();
            rules.sort_unstable_by(|left, right| left.0.cmp(&right.0));
            let rules = rules
                .into_iter()
                .map(|(_, rule)| rule)
                .collect::<std::vec::Vec<_>>();
            affected_nodes.extend(rules.iter().copied());
            final_rule_lists.push(FinalReificationRuleList { parent, rules });
        }
        let mut final_rule_ids = affected_nodes.into_iter().collect::<std::vec::Vec<_>>();
        final_rule_ids.sort_unstable();
        let final_rules = final_rule_ids
            .into_iter()
            .map(|logical_id| {
                let node = &self.nodes[logical_id.index()];
                FinalReificationRule {
                    logical_id,
                    source_rule: node.source_rule,
                    source_block: node.source_block,
                    parent: node.parent,
                    live: node.live,
                    effective_key: node.effective_key,
                    selector: node.style.map(|style| match style.selector {
                        SelectorSource::Existing(selector) => {
                            FinalReificationSelector::Existing(selector)
                        }
                        SelectorSource::Synthesized(selector) => {
                            FinalReificationSelector::Synthesized(selector)
                        }
                    }),
                    span: node.style.map(|style| style.span),
                    declarations: node
                        .declarations
                        .iter()
                        .copied()
                        .filter(|declaration| self.declarations[declaration.index()].live)
                        .collect(),
                }
            })
            .collect();
        let plan = ReificationPlan {
            source_rules: self.nodes.iter().map(|node| node.source_rule).collect(),
            source_blocks: self.nodes.iter().map(|node| node.source_block).collect(),
            rule_orders: self
                .nodes
                .iter()
                .map(|node| node.order.0.iter().copied().collect())
                .collect(),
            declaration_sources: self
                .declarations
                .iter()
                .map(|declaration| declaration.source)
                .collect(),
            selectors: self.selectors,
            pending_keys: self.pending_keys,
            final_rules,
            final_rule_lists,
            steps: self.steps,
        };
        (plan, self.stats)
    }

    fn commit_s1(&mut self, candidate: EdgeCandidate) {
        let Some((left, right)) = self.validate_edge(candidate) else {
            return;
        };
        if self.nodes[left.index()].style.is_none()
            || self.nodes[right.index()].style.is_none()
            || self.nodes[left.index()].has_children
            || self.nodes[left.index()].effective_key != self.nodes[right.index()].effective_key
        {
            return;
        }
        let key = self.nodes[left.index()]
            .effective_key
            .expect("an S1 style block has an effective key");
        let left_declarations = std::mem::take(&mut self.nodes[left.index()].declarations);
        let mut merged = left_declarations;
        merged.append(&mut self.nodes[right.index()].declarations);
        self.nodes[right.index()].declarations = merged;
        self.steps
            .push(ReificationStep::MergeAdjacent { left, right });
        self.remove_history(key, left);
        self.retire_and_relink(left);
        self.bump_and_enqueue(right);
        self.declaration_override_candidates.push(key);
    }

    fn commit_s2(&mut self, stylesheet: &StyleSheet<'ast>, key: TransformEffectiveKey) {
        let Some(history) = self.histories.get(&key) else {
            return;
        };
        let owners = history.values().copied().collect::<std::vec::Vec<_>>();
        let mut previous_by_property = FxHashMap::default();
        let mut removals = std::vec::Vec::new();
        for owner in owners {
            for &declaration in &self.nodes[owner.index()].declarations {
                let state = self.declarations[declaration.index()];
                let Some(property_key) = state.live.then_some(state.property_key).flatten() else {
                    continue;
                };
                if let Some(&(previous_owner, previous)) = previous_by_property.get(&property_key)
                    && self.declarations_are_exactly_equal(stylesheet, previous, declaration)
                {
                    removals.push((previous_owner, previous));
                }
                previous_by_property.insert(property_key, (owner, declaration));
            }
        }
        let mut affected = FxHashSet::default();
        for (owner, declaration) in removals {
            if !self.declarations[declaration.index()].live {
                continue;
            }
            self.declarations[declaration.index()].live = false;
            self.steps
                .push(ReificationStep::RemoveDeclaration { owner, declaration });
            affected.insert(owner);
        }
        for owner in affected {
            if self.node_live_declaration_count(owner) == 0
                && !self.nodes[owner.index()].has_children
            {
                self.steps.push(ReificationStep::RetireRule { rule: owner });
                self.remove_history(key, owner);
                self.retire_and_relink(owner);
            } else {
                self.bump_and_enqueue(owner);
            }
        }
    }

    fn commit_s3(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
        candidate: EdgeCandidate,
        preserve_selector_compatibility: bool,
    ) {
        let Some((left, right)) = self.validate_edge(candidate) else {
            return;
        };
        let (Some(left_style), Some(right_style)) = (
            self.nodes[left.index()].style,
            self.nodes[right.index()].style,
        ) else {
            return;
        };
        if self.nodes[left.index()].has_children
            || self.nodes[left.index()].effective_key == self.nodes[right.index()].effective_key
            || left_style.kind != right_style.kind
            || left_style.vendor_prefix != right_style.vendor_prefix
        {
            return;
        }

        let left_live = self.live_declarations(left);
        let right_live = self.live_declarations(right);
        if left_live.is_empty() || right_live.is_empty() {
            return;
        }
        let mut matched_right = FxHashSet::default();
        let mut common = std::vec::Vec::new();
        for &left_declaration in &left_live {
            let left_state = self.declarations[left_declaration.index()];
            let Some(property_key) = left_state.property_key else {
                continue;
            };
            if left_state.movement_domain.is_some_and(|domain| {
                self.has_opaque_domain_conflict(domain, &left_live)
                    || self.has_opaque_domain_conflict(domain, &right_live)
            }) {
                continue;
            }
            if let Some((right_order, &right_declaration)) =
                right_live
                    .iter()
                    .enumerate()
                    .find(|(_, right_declaration)| {
                        !matched_right.contains(*right_declaration)
                            && self.declarations[right_declaration.index()].property_key
                                == Some(property_key)
                            && self.declarations_have_equal_effect(
                                stylesheet,
                                left_declaration,
                                **right_declaration,
                            )
                    })
            {
                matched_right.insert(right_declaration);
                common.push((left_declaration, right_declaration, right_order));
            }
        }
        if common.is_empty() {
            return;
        }
        let matched_left = common
            .iter()
            .map(|(left, _, _)| *left)
            .collect::<FxHashSet<_>>();
        let left_residual = left_live
            .iter()
            .copied()
            .filter(|declaration| !matched_left.contains(declaration))
            .collect::<std::vec::Vec<_>>();
        let right_residual = right_live
            .iter()
            .copied()
            .filter(|declaration| !matched_right.contains(declaration))
            .collect::<std::vec::Vec<_>>();
        if !self.partial_movement_is_safe(&common, &left_residual, &right_residual) {
            return;
        }

        let selectors = {
            let left_selectors = self.selector_list(stylesheet, left_style.selector);
            let right_selectors = self.selector_list(stylesheet, right_style.selector);
            materialize_selector_union(
                left_selectors,
                right_selectors,
                preserve_selector_compatibility,
                self.allocator,
            )
        };
        let Some(selectors) = selectors else {
            return;
        };
        let selector = LocalSelectorId(
            u32::try_from(self.selectors.len()).expect("selector count exceeds u32::MAX"),
        );
        self.selectors.push(selectors);
        let left_key = self.nodes[left.index()]
            .effective_key
            .expect("an S3 endpoint has an effective key");
        let right_key = self.nodes[right.index()]
            .effective_key
            .expect("an S3 endpoint has an effective key");
        let Some(shared_key) = self.intern_pending_key(stylesheet, left_key, right_key, selector)
        else {
            self.selectors.pop();
            return;
        };
        let span = Span::new(left_style.span.start, right_style.span.end);

        if left_residual.is_empty() {
            let right_removed = common
                .iter()
                .map(|(_, right, _)| *right)
                .collect::<std::vec::Vec<_>>();
            for &declaration in &right_removed {
                self.declarations[declaration.index()].live = false;
            }
            self.remove_history(left_key, left);
            self.nodes[left.index()].effective_key = Some(shared_key);
            self.nodes[left.index()].style = Some(StyleState {
                selector: SelectorSource::Synthesized(selector),
                span,
                ..left_style
            });
            self.insert_history(shared_key, left);
            let retire_right = right_residual.is_empty() && !self.nodes[right.index()].has_children;
            self.steps.push(ReificationStep::ReuseLeftForPartialMerge {
                left,
                right,
                selector,
                effective_key: shared_key,
                right_removed,
                retire_right,
                span,
            });
            if retire_right {
                self.remove_history(right_key, right);
                self.retire_and_relink(right);
            } else {
                self.bump_and_enqueue(right);
            }
            self.bump_and_enqueue(left);
            self.declaration_override_candidates.push(left_key);
            self.declaration_override_candidates.push(right_key);
            self.declaration_override_candidates.push(shared_key);
            self.stats.live_endpoint_reuses += 1;
            return;
        }

        let synthesized =
            NodeId(u32::try_from(self.nodes.len()).expect("rule count exceeds u32::MAX"));
        let mut moved = std::vec::Vec::with_capacity(common.len());
        let mut synthesized_declarations = std::vec::Vec::with_capacity(common.len());
        for (left_declaration, right_declaration, _) in common {
            self.declarations[left_declaration.index()].live = false;
            self.declarations[right_declaration.index()].live = false;
            let mut declaration = self.declarations[left_declaration.index()];
            declaration.live = true;
            let synthesized_declaration = DeclarationNodeId(
                u32::try_from(self.declarations.len()).expect("declaration count exceeds u32::MAX"),
            );
            self.declarations.push(declaration);
            synthesized_declarations.push(synthesized_declaration);
            moved.push((synthesized_declaration, left_declaration, right_declaration));
        }
        let order = SemanticOrderKey::between(
            &self.nodes[left.index()].order,
            &self.nodes[right.index()].order,
        );
        self.nodes.push(LiveRule {
            source_rule: None,
            source_block: None,
            parent: self.nodes[left.index()].parent,
            previous_live: Some(left),
            next_live: Some(right),
            order: order.clone(),
            effective_key: Some(shared_key),
            declarations: synthesized_declarations,
            style: Some(StyleState {
                selector: SelectorSource::Synthesized(selector),
                kind: left_style.kind,
                vendor_prefix: left_style.vendor_prefix,
                span,
            }),
            has_children: false,
            live: true,
            generation: 0,
        });
        self.nodes[left.index()].next_live = Some(synthesized);
        self.nodes[right.index()].previous_live = Some(synthesized);
        self.insert_history(shared_key, synthesized);
        let retire_right = right_residual.is_empty() && !self.nodes[right.index()].has_children;
        self.steps.push(ReificationStep::InsertPartialMerge {
            left,
            right,
            synthesized,
            selector,
            effective_key: shared_key,
            declarations: moved,
            retire_right,
        });
        if retire_right {
            self.remove_history(right_key, right);
            self.retire_and_relink(right);
        } else {
            self.bump_and_enqueue(right);
        }
        self.bump_and_enqueue(left);
        self.enqueue_edge(left, synthesized);
        if self.nodes[synthesized.index()].live
            && let Some(next) = self.nodes[synthesized.index()].next_live
        {
            self.enqueue_edge(synthesized, next);
        }
        self.declaration_override_candidates.push(left_key);
        self.declaration_override_candidates.push(right_key);
        self.declaration_override_candidates.push(shared_key);
    }

    fn validate_edge(&self, candidate: EdgeCandidate) -> Option<(NodeId, NodeId)> {
        let left = self.nodes.get(candidate.left.index())?;
        let right = self.nodes.get(candidate.right.index())?;
        (left.live
            && right.live
            && left.next_live == Some(candidate.right)
            && right.previous_live == Some(candidate.left)
            && left.parent == right.parent
            && left.generation == candidate.left_generation
            && right.generation == candidate.right_generation)
            .then_some((candidate.left, candidate.right))
    }

    fn enqueue_edge(&mut self, left: NodeId, right: NodeId) {
        let left_node = &self.nodes[left.index()];
        let right_node = &self.nodes[right.index()];
        if !left_node.live
            || !right_node.live
            || left_node.next_live != Some(right)
            || left_node.style.is_none()
            || right_node.style.is_none()
        {
            return;
        }
        let candidate = EdgeCandidate {
            left,
            right,
            left_generation: left_node.generation,
            right_generation: right_node.generation,
        };
        if left_node.effective_key == right_node.effective_key {
            self.same_selector_candidates.push(candidate);
        } else if self
            .property_bloom(left)
            .may_share_declaration(self.property_bloom(right))
        {
            self.partial_merge_candidates
                .push(candidate, left_node.order.clone());
        }
    }

    fn enqueue_incident_edges(&mut self, node: NodeId) {
        if !self.nodes[node.index()].live {
            return;
        }
        if let Some(previous) = self.nodes[node.index()].previous_live {
            self.enqueue_edge(previous, node);
        }
        if let Some(next) = self.nodes[node.index()].next_live {
            self.enqueue_edge(node, next);
        }
    }

    fn bump_and_enqueue(&mut self, node: NodeId) {
        self.nodes[node.index()].generation = self.nodes[node.index()].generation.wrapping_add(1);
        self.enqueue_incident_edges(node);
    }

    fn retire_and_relink(&mut self, node: NodeId) {
        let previous = self.nodes[node.index()].previous_live;
        let next = self.nodes[node.index()].next_live;
        self.nodes[node.index()].live = false;
        self.nodes[node.index()].generation = self.nodes[node.index()].generation.wrapping_add(1);
        if let Some(previous) = previous {
            self.nodes[previous.index()].next_live = next;
            self.nodes[previous.index()].generation =
                self.nodes[previous.index()].generation.wrapping_add(1);
        }
        if let Some(next) = next {
            self.nodes[next.index()].previous_live = previous;
            self.nodes[next.index()].generation =
                self.nodes[next.index()].generation.wrapping_add(1);
        }
        if let (Some(previous), Some(next)) = (previous, next) {
            self.enqueue_edge(previous, next);
        }
    }

    fn live_declarations(&self, node: NodeId) -> std::vec::Vec<DeclarationNodeId> {
        self.nodes[node.index()]
            .declarations
            .iter()
            .copied()
            .filter(|declaration| self.declarations[declaration.index()].live)
            .collect()
    }

    fn node_live_declaration_count(&self, node: NodeId) -> usize {
        self.nodes[node.index()]
            .declarations
            .iter()
            .filter(|declaration| self.declarations[declaration.index()].live)
            .count()
    }

    fn property_bloom(&self, node: NodeId) -> PropertyBloom {
        let mut bloom = PropertyBloom::default();
        for declaration in self.live_declarations(node) {
            if let Some(key) = self.declarations[declaration.index()].property_key {
                bloom.insert(key);
            }
        }
        bloom
    }

    fn selector_list<'comp>(
        &'comp self,
        stylesheet: &'comp StyleSheet<'ast>,
        selector: SelectorSource,
    ) -> &'comp SelectorList<'ast> {
        match selector {
            SelectorSource::Existing(selector) => stylesheet
                .selector_value(selector)
                .expect("a scanned selector remains resolvable")
                .selectors(),
            SelectorSource::Synthesized(selector) => &self.selectors[selector.index()],
        }
    }

    fn representative_key(&self, key: TransformEffectiveKey) -> EffectiveKeyId {
        match key {
            TransformEffectiveKey::Existing(key) => key,
            TransformEffectiveKey::Synthesized(key) => {
                self.pending_keys[key as usize].representative
            }
        }
    }

    fn intern_pending_key(
        &mut self,
        stylesheet: &StyleSheet<'ast>,
        left: TransformEffectiveKey,
        right: TransformEffectiveKey,
        selector: LocalSelectorId,
    ) -> Option<TransformEffectiveKey> {
        let left = self.representative_key(left);
        let right = self.representative_key(right);
        if !stylesheet.selector_union_effective_keys_are_compatible(left, right) {
            return None;
        }
        for node in &self.nodes {
            let (Some(TransformEffectiveKey::Existing(existing)), Some(style)) =
                (node.effective_key, node.style)
            else {
                continue;
            };
            if stylesheet.selector_union_effective_keys_are_compatible(left, existing)
                && self.selector_list(stylesheet, style.selector)
                    == &self.selectors[selector.index()]
            {
                return Some(TransformEffectiveKey::Existing(existing));
            }
        }
        for (index, pending) in self.pending_keys.iter().enumerate() {
            if stylesheet.selector_union_effective_keys_are_compatible(left, pending.representative)
                && self.selectors[pending.selector.index()] == self.selectors[selector.index()]
            {
                return Some(TransformEffectiveKey::Synthesized(index as u32));
            }
        }
        let index =
            u32::try_from(self.pending_keys.len()).expect("effective key count exceeds u32::MAX");
        self.pending_keys.push(PendingEffectiveKey {
            representative: left,
            selector,
        });
        Some(TransformEffectiveKey::Synthesized(index))
    }

    fn insert_history(&mut self, key: TransformEffectiveKey, node: NodeId) {
        let history = self.histories.entry(key).or_default();
        history.insert(self.nodes[node.index()].order.clone(), node);
        if history.len() == 2 {
            self.declaration_override_candidates.push(key);
        }
    }

    fn remove_history(&mut self, key: TransformEffectiveKey, node: NodeId) {
        let remove_key = if let Some(history) = self.histories.get_mut(&key) {
            history.remove(&self.nodes[node.index()].order);
            history.is_empty()
        } else {
            false
        };
        if remove_key {
            self.histories.remove(&key);
        }
    }

    fn declarations_are_exactly_equal(
        &self,
        stylesheet: &StyleSheet<'ast>,
        left: DeclarationNodeId,
        right: DeclarationNodeId,
    ) -> bool {
        let Some(left) = stylesheet.declaration(self.declarations[left.index()].source) else {
            return false;
        };
        let Some(right) = stylesheet.declaration(self.declarations[right.index()].source) else {
            return false;
        };
        left.is_important() == right.is_important() && left.payload() == right.payload()
    }

    fn declarations_have_equal_effect(
        &self,
        stylesheet: &StyleSheet<'ast>,
        left: DeclarationNodeId,
        right: DeclarationNodeId,
    ) -> bool {
        let Some(left) = stylesheet.declaration(self.declarations[left.index()].source) else {
            return false;
        };
        let Some(right) = stylesheet.declaration(self.declarations[right.index()].source) else {
            return false;
        };
        if left.is_important() != right.is_important() {
            return false;
        }
        matches!(
            (left.payload(), right.payload()),
            (CssDeclaration::Property(left), CssDeclaration::Property(right))
                if left.eq_ignoring_tombstones(right)
        )
    }

    fn has_opaque_domain_conflict(
        &self,
        domain: MovementDomain,
        declarations: &[DeclarationNodeId],
    ) -> bool {
        declarations.iter().any(|declaration| {
            let declaration = self.declarations[declaration.index()];
            declaration.property_key.is_none()
                && declaration
                    .movement_domain
                    .is_some_and(|opaque| domain.overlaps(&opaque))
        })
    }

    fn partial_movement_is_safe(
        &self,
        common: &[(DeclarationNodeId, DeclarationNodeId, usize)],
        left_residual: &[DeclarationNodeId],
        right_residual: &[DeclarationNodeId],
    ) -> bool {
        if left_residual.is_empty() && right_residual.is_empty() {
            if common
                .iter()
                .any(|(left, _, _)| self.declarations[left.index()].movement_domain.is_none())
            {
                return common.windows(2).all(|pair| pair[0].2 < pair[1].2);
            }
            return self.common_effect_order_is_safe(common);
        }
        for (common, _, _) in common {
            let Some(common_domain) = self.declarations[common.index()].movement_domain else {
                return false;
            };
            for residual in left_residual.iter().chain(right_residual) {
                let Some(residual_domain) = self.declarations[residual.index()].movement_domain
                else {
                    return false;
                };
                if common_domain.overlaps(&residual_domain) {
                    return false;
                }
            }
        }
        self.common_effect_order_is_safe(common)
    }

    fn common_effect_order_is_safe(
        &self,
        common: &[(DeclarationNodeId, DeclarationNodeId, usize)],
    ) -> bool {
        for left in 0..common.len() {
            for right in left + 1..common.len() {
                if common[left].2 > common[right].2 {
                    let Some(left_domain) =
                        self.declarations[common[left].0.index()].movement_domain
                    else {
                        return false;
                    };
                    let Some(right_domain) =
                        self.declarations[common[right].0.index()].movement_domain
                    else {
                        return false;
                    };
                    if left_domain.overlaps(&right_domain) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn style_state(payload: &CssRule<'_>) -> Option<StyleState> {
    match payload {
        CssRule::Style(payload) => Some(StyleState {
            selector: SelectorSource::Existing(payload.selector_value),
            kind: SelectorFrameKind::Style,
            vendor_prefix: payload.vendor_prefix,
            span: payload.span,
        }),
        CssRule::Nesting(payload) => Some(StyleState {
            selector: SelectorSource::Existing(payload.selector_value),
            kind: SelectorFrameKind::Nesting,
            vendor_prefix: VendorPrefix::NONE,
            span: payload.span,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
    use rocketcss_parser::{ParserOptions, parse};

    use super::*;

    #[test]
    fn semantic_order_supports_repeated_insertions() {
        let left = SemanticOrderKey::initial(0);
        let right = SemanticOrderKey::initial(1);
        let mut previous = left.clone();
        for _ in 0..256 {
            let current = SemanticOrderKey::between(&previous, &right);
            assert!(previous < current && current < right);
            previous = current;
        }
    }

    #[test]
    fn scheduler_build_and_fixed_point_leave_ast_unchanged() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut stylesheet = parse(
                "a{color:red;margin:0}b{color:red;padding:0}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let before = stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap();
            let counts = (
                stylesheet.rule_count(),
                stylesheet.declaration_block_count(),
                stylesheet.declaration_count(),
            );

            let scheduler = CrossRuleScheduler::from_stylesheet(&stylesheet, &allocator).unwrap();
            let (plan, scheduler_stats) = scheduler.stabilize(&stylesheet, false);

            assert_eq!(scheduler_stats.initial_scans, 1);
            assert_eq!(scheduler_stats.scheduler_ast_mutations, 0);
            assert!(!plan.steps.is_empty());
            assert_eq!(
                counts,
                (
                    stylesheet.rule_count(),
                    stylesheet.declaration_block_count(),
                    stylesheet.declaration_count(),
                )
            );
            assert_eq!(
                stylesheet
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::new(&token),
                    )
                    .unwrap(),
                before
            );

            let stats = stylesheet.apply_reification_plan(plan, &allocator).unwrap();
            assert_eq!(stats.reification_passes, 1);
            assert_eq!(stylesheet.validate_ast(), Ok(()));
        });
    }
}
