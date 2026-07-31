use rocketcss_ast::{
    CascadeOrigin, CssRule, DeclarationBlockId, DeclarationBlockStore, EffectiveKeyId, RuleId,
    RuleListId, RuleStore, StyleRule, VendorPrefix,
};
use rocketcss_common::{DenseIdGenerator, DenseStore, define_dense_id};
use rustc_hash::{FxHashMap, FxHasher};
use smallvec::SmallVec;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
enum ConditionalFrame {
    Media(RuleId),
    Supports(RuleId),
    Container(RuleId),
    Layer(RuleId),
    Opaque {
        kind: OpaqueConditionalKind,
        identity: RuleId,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum OpaqueConditionalKind {
    MozDocument,
    Scope,
    StartingStyle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SelectorFrameKind {
    Style,
    Nesting,
}

#[derive(Clone, Copy, Debug)]
struct SelectorFrame {
    kind: SelectorFrameKind,
    value: SelectorValueId,
    vendor_prefix: VendorPrefix,
}

define_dense_id!(struct SelectorValueId);

#[derive(Clone, Copy, Debug)]
struct SelectorValueState {
    rule: RuleId,
    id: SelectorValueId,
}

#[derive(Debug, Default)]
struct SelectorValueInterner {
    buckets: FxHashMap<u64, SmallVec<[SelectorValueState; 1]>>,
    ids: DenseIdGenerator<SelectorValueId>,
}

impl SelectorValueInterner {
    fn intern(&mut self, rule: RuleId, rules: &RuleStore<'_>) -> SelectorValueId {
        let style =
            selector_rule(rule, rules).expect("selector value must reference a style-bearing rule");
        let mut hasher = FxHasher::default();
        rules.selectors(style.selectors).hash(&mut hasher);
        let bucket = self.buckets.entry(hasher.finish()).or_default();
        if let Some(state) = bucket.iter().find(|state| {
            selector_rule(state.rule, rules).map(|rule| rules.selectors(rule.selectors))
                == Some(rules.selectors(style.selectors))
        }) {
            return state.id;
        }
        let id = self.ids.allocate();
        bucket.push(SelectorValueState { rule, id });
        id
    }
}

define_dense_id!(struct SelectorPathId);

#[derive(Clone, Copy, Debug)]
struct SelectorPathNode {
    parent: Option<SelectorPathId>,
    frame: SelectorFrame,
    fingerprint: u64,
}

#[derive(Debug, Default)]
struct SelectorPathStore {
    nodes: DenseStore<SelectorPathId, SelectorPathNode>,
    buckets: FxHashMap<u64, SmallVec<[SelectorPathId; 1]>>,
}

impl SelectorPathStore {
    fn push(&mut self, parent: Option<SelectorPathId>, frame: SelectorFrame) -> SelectorPathId {
        let mut hasher = FxHasher::default();
        self.fingerprint(parent).hash(&mut hasher);
        frame.kind.hash(&mut hasher);
        frame.value.hash(&mut hasher);
        frame.vendor_prefix.hash(&mut hasher);
        let fingerprint = hasher.finish();
        let bucket = self.buckets.entry(fingerprint).or_default();
        if let Some(&id) = bucket.iter().find(|&&id| {
            let node = &self.nodes[id];
            node.parent == parent
                && node.frame.kind == frame.kind
                && node.frame.value == frame.value
                && node.frame.vendor_prefix == frame.vendor_prefix
        }) {
            return id;
        }
        let id = self.nodes.push(SelectorPathNode {
            parent,
            frame,
            fingerprint,
        });
        bucket.push(id);
        id
    }

    fn fingerprint(&self, path: Option<SelectorPathId>) -> u64 {
        path.map_or(0, |path| self.nodes[path].fingerprint)
    }
}

define_dense_id!(struct ConditionalPathId);

#[derive(Clone, Copy, Debug)]
struct ConditionalPathNode {
    parent: Option<ConditionalPathId>,
    frame: ConditionalFrame,
    fingerprint: u64,
}

#[derive(Debug, Default)]
struct ConditionalPathStore {
    nodes: DenseStore<ConditionalPathId, ConditionalPathNode>,
    buckets: FxHashMap<u64, SmallVec<[ConditionalPathId; 1]>>,
}

impl ConditionalPathStore {
    fn push(
        &mut self,
        parent: Option<ConditionalPathId>,
        frame: ConditionalFrame,
        rules: &RuleStore<'_>,
    ) -> ConditionalPathId {
        let mut hasher = FxHasher::default();
        self.fingerprint(parent).hash(&mut hasher);
        hash_conditional_frame(frame, rules, &mut hasher);
        let fingerprint = hasher.finish();
        let bucket = self.buckets.entry(fingerprint).or_default();
        if let Some(&id) = bucket.iter().find(|&&id| {
            let node = &self.nodes[id];
            node.parent == parent && conditional_frames_equal(node.frame, frame, rules)
        }) {
            return id;
        }
        let id = self.nodes.push(ConditionalPathNode {
            parent,
            frame,
            fingerprint,
        });
        bucket.push(id);
        id
    }

    fn fingerprint(&self, path: Option<ConditionalPathId>) -> u64 {
        path.map_or(0, |path| self.nodes[path].fingerprint)
    }
}

define_dense_id!(pub(crate) struct DeclarationBlockEntryId);

#[derive(Clone, Copy, Debug)]
struct EffectiveKeyState {
    selectors: Option<SelectorPathId>,
    conditions: Option<ConditionalPathId>,
    id: EffectiveKeyId,
    last_entry: DeclarationBlockEntryId,
    has_history: bool,
}

#[derive(Clone, Copy, Debug)]
struct EffectiveKeyOccurrence {
    id: EffectiveKeyId,
    previous_entry: Option<DeclarationBlockEntryId>,
    starts_history: bool,
}

#[derive(Debug)]
struct EffectiveKeyInterner {
    buckets: FxHashMap<u64, SmallVec<[EffectiveKeyState; 1]>>,
    ids: DenseIdGenerator<EffectiveKeyId>,
    origin: CascadeOrigin,
}

impl EffectiveKeyInterner {
    fn new(origin: CascadeOrigin) -> Self {
        Self {
            buckets: FxHashMap::default(),
            ids: DenseIdGenerator::default(),
            origin,
        }
    }

    fn intern(
        &mut self,
        selectors: Option<SelectorPathId>,
        conditions: Option<ConditionalPathId>,
        selector_paths: &SelectorPathStore,
        conditional_paths: &ConditionalPathStore,
        current_entry: DeclarationBlockEntryId,
    ) -> EffectiveKeyOccurrence {
        let mut hasher = FxHasher::default();
        selector_paths.fingerprint(selectors).hash(&mut hasher);
        conditional_paths.fingerprint(conditions).hash(&mut hasher);
        self.origin.hash(&mut hasher);
        let bucket = self.buckets.entry(hasher.finish()).or_default();
        if let Some(state) = bucket
            .iter_mut()
            .find(|state| state.selectors == selectors && state.conditions == conditions)
        {
            let occurrence = EffectiveKeyOccurrence {
                id: state.id,
                previous_entry: Some(state.last_entry),
                starts_history: !state.has_history,
            };
            state.last_entry = current_entry;
            state.has_history = true;
            return occurrence;
        }

        let id = self.ids.allocate();
        bucket.push(EffectiveKeyState {
            selectors,
            conditions,
            id,
            last_entry: current_entry,
            has_history: false,
        });
        EffectiveKeyOccurrence {
            id,
            previous_entry: None,
            starts_history: false,
        }
    }
}

define_dense_id!(pub(crate) struct RuleListSegmentId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SiblingOrdinal(u32);

#[derive(Clone, Copy, Debug)]
struct StructuralLocation {
    rule_list: RuleListId,
    rule_list_segment: RuleListSegmentId,
    sibling_ordinal: SiblingOrdinal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DeclarationBlockKind {
    Style {
        has_children: bool,
        has_live_selectors: bool,
    },
    Nesting,
    NestedDeclarations,
}

#[derive(Debug)]
pub(crate) struct DeclarationBlockEntry {
    pub(crate) rule: RuleId,
    pub(crate) declarations: DeclarationBlockId,
    pub(crate) effective_key: EffectiveKeyId,
    pub(crate) kind: DeclarationBlockKind,
    pub(crate) rule_list: RuleListId,
    pub(crate) rule_list_segment: RuleListSegmentId,
    pub(crate) sibling_ordinal: SiblingOrdinal,
    next_in_history: Option<DeclarationBlockEntryId>,
    starts_history: bool,
}

impl DeclarationBlockEntry {
    pub(crate) fn is_direct_sibling_of(&self, right: &Self) -> bool {
        self.rule_list == right.rule_list
            && self.rule_list_segment == right.rule_list_segment
            && self.sibling_ordinal.0.checked_add(1) == Some(right.sibling_ordinal.0)
    }

    pub(crate) fn starts_declaration_history(&self) -> bool {
        self.starts_history
    }

    pub(crate) fn next_declaration_history_entry(&self) -> Option<DeclarationBlockEntryId> {
        self.next_in_history
    }
}

#[derive(Debug, Default)]
struct WalkState {
    rule_list_segments: DenseIdGenerator<RuleListSegmentId>,
}

impl WalkState {
    fn allocate_rule_list_segment(&mut self) -> RuleListSegmentId {
        self.rule_list_segments.allocate()
    }
}

pub(crate) type DeclarationBlockEntries =
    DenseStore<DeclarationBlockEntryId, DeclarationBlockEntry>;

#[derive(Clone, Copy, Debug, Default)]
struct RuleContextState {
    selector_path: Option<SelectorPathId>,
    conditional_path: Option<ConditionalPathId>,
}

struct RuleListScanState {
    segment: RuleListSegmentId,
    sibling_ordinal: u32,
}

pub(crate) struct DeclarationBlockDiscovery {
    selector_values: SelectorValueInterner,
    selector_paths: SelectorPathStore,
    conditional_paths: ConditionalPathStore,
    effective_keys: EffectiveKeyInterner,
    declaration_blocks: DeclarationBlockEntries,
    contexts: DenseStore<RuleId, RuleContextState>,
    list_states: FxHashMap<RuleListId, RuleListScanState>,
    state: WalkState,
}

impl DeclarationBlockDiscovery {
    pub(crate) fn new(rules: &RuleStore<'_>, origin: CascadeOrigin) -> Self {
        Self {
            selector_values: SelectorValueInterner::default(),
            selector_paths: SelectorPathStore::default(),
            conditional_paths: ConditionalPathStore::default(),
            effective_keys: EffectiveKeyInterner::new(origin),
            declaration_blocks: DenseStore::new(),
            contexts: DenseStore::with_capacity(rules.len()),
            list_states: FxHashMap::default(),
            state: WalkState::default(),
        }
        .with_capacity(rules.len())
    }

    fn with_capacity(mut self, rule_count: usize) -> Self {
        self.declaration_blocks.reserve(rule_count);
        self
    }

    pub(crate) fn observe(&mut self, rule_id: RuleId, rules: &RuleStore<'_>) {
        let topology = rules.topology(rule_id);
        let inherited = topology
            .parent
            .map_or_else(RuleContextState::default, |parent| self.contexts[parent]);
        let location = {
            let state =
                self.list_states
                    .entry(topology.list)
                    .or_insert_with(|| RuleListScanState {
                        segment: self.state.allocate_rule_list_segment(),
                        sibling_ordinal: 0,
                    });
            let location = StructuralLocation {
                rule_list: topology.list,
                rule_list_segment: state.segment,
                sibling_ordinal: SiblingOrdinal(state.sibling_ordinal),
            };
            state.sibling_ordinal = state
                .sibling_ordinal
                .checked_add(1)
                .expect("sibling ordinal exceeds u32::MAX");
            location
        };
        let rule = rules.get(rule_id);
        let descendant_context = self.collect_rule(rule_id, rule, inherited, location, rules);
        let inserted = self.contexts.push(descendant_context);
        debug_assert_eq!(inserted, rule_id, "rules are observed in preorder");
        if ends_rule_list_segment(rule, rules) {
            self.list_states.get_mut(&topology.list).unwrap().segment =
                self.state.allocate_rule_list_segment();
        }
    }

    pub(crate) fn finish(self) -> DeclarationBlockEntries {
        self.declaration_blocks
    }

    fn collect_rule<'ast>(
        &mut self,
        rule_id: RuleId,
        rule: &CssRule<'ast>,
        inherited: RuleContextState,
        location: StructuralLocation,
        rules: &RuleStore<'ast>,
    ) -> RuleContextState {
        let mut descendant = inherited;
        match rule {
            CssRule::Media(_) => {
                descendant.conditional_path = Some(self.conditional_paths.push(
                    inherited.conditional_path,
                    ConditionalFrame::Media(rule_id),
                    rules,
                ));
            }
            CssRule::Style(rule) => {
                descendant = self.collect_style_rule(
                    rule_id,
                    rule,
                    SelectorFrameKind::Style,
                    inherited,
                    location,
                    rules,
                );
            }
            CssRule::Supports(_) => {
                descendant.conditional_path = Some(self.conditional_paths.push(
                    inherited.conditional_path,
                    ConditionalFrame::Supports(rule_id),
                    rules,
                ));
            }
            CssRule::MozDocument(_) => {
                descendant.conditional_path = Some(self.conditional_paths.push(
                    inherited.conditional_path,
                    opaque_condition(OpaqueConditionalKind::MozDocument, rule_id),
                    rules,
                ));
            }
            CssRule::Nesting(rule) => {
                descendant = self.collect_style_rule(
                    rule_id,
                    &rule.style,
                    SelectorFrameKind::Nesting,
                    inherited,
                    location,
                    rules,
                );
            }
            CssRule::NestedDeclarations(rule) => {
                self.push_declaration_block(
                    rule_id,
                    rule.declarations,
                    DeclarationBlockKind::NestedDeclarations,
                    inherited,
                    location,
                    rules,
                );
            }
            CssRule::LayerBlock(_) => {
                descendant.conditional_path = Some(self.conditional_paths.push(
                    inherited.conditional_path,
                    ConditionalFrame::Layer(rule_id),
                    rules,
                ));
            }
            CssRule::Container(_) => {
                self.with_condition(ConditionalFrame::Container(rule_id), &mut descendant, rules)
            }
            CssRule::Scope(_) => {
                descendant.conditional_path = Some(self.conditional_paths.push(
                    inherited.conditional_path,
                    opaque_condition(OpaqueConditionalKind::Scope, rule_id),
                    rules,
                ));
            }
            CssRule::StartingStyle(_) => {
                descendant.conditional_path = Some(self.conditional_paths.push(
                    inherited.conditional_path,
                    opaque_condition(OpaqueConditionalKind::StartingStyle, rule_id),
                    rules,
                ));
            }
            _ => {}
        }
        descendant
    }

    fn collect_style_rule<'ast>(
        &mut self,
        rule_id: RuleId,
        rule: &StyleRule<'ast>,
        kind: SelectorFrameKind,
        inherited: RuleContextState,
        location: StructuralLocation,
        rules: &RuleStore<'ast>,
    ) -> RuleContextState {
        let selector_value = self.selector_values.intern(rule_id, rules);
        let selector_path = Some(self.selector_paths.push(
            inherited.selector_path,
            SelectorFrame {
                kind,
                value: selector_value,
                vendor_prefix: rule.vendor_prefix,
            },
        ));
        let has_live_selectors = rules
            .selectors(rule.selectors)
            .iter()
            .any(|selector| !selector.is_tombstone());
        if has_live_selectors {
            self.push_declaration_block(
                rule_id,
                rule.declarations,
                match kind {
                    SelectorFrameKind::Style => DeclarationBlockKind::Style {
                        has_children: !rules.list_is_empty(rule.rules),
                        has_live_selectors,
                    },
                    SelectorFrameKind::Nesting => DeclarationBlockKind::Nesting,
                },
                RuleContextState {
                    selector_path,
                    conditional_path: inherited.conditional_path,
                },
                location,
                rules,
            );
        }
        RuleContextState {
            selector_path,
            conditional_path: inherited.conditional_path,
        }
    }

    fn with_condition(
        &mut self,
        frame: ConditionalFrame,
        descendant: &mut RuleContextState,
        rules: &RuleStore<'_>,
    ) {
        descendant.conditional_path = Some(self.conditional_paths.push(
            descendant.conditional_path,
            frame,
            rules,
        ));
    }

    fn push_declaration_block(
        &mut self,
        rule: RuleId,
        declarations: DeclarationBlockId,
        kind: DeclarationBlockKind,
        context: RuleContextState,
        location: StructuralLocation,
        _rules: &RuleStore<'_>,
    ) {
        let current_entry = self.declaration_blocks.next_id();
        let occurrence = self.effective_keys.intern(
            context.selector_path,
            context.conditional_path,
            &self.selector_paths,
            &self.conditional_paths,
            current_entry,
        );
        let inserted = self.declaration_blocks.push(DeclarationBlockEntry {
            rule,
            declarations,
            effective_key: occurrence.id,
            kind,
            rule_list: location.rule_list,
            rule_list_segment: location.rule_list_segment,
            sibling_ordinal: location.sibling_ordinal,
            next_in_history: None,
            starts_history: false,
        });
        debug_assert_eq!(inserted, current_entry);

        if let Some(previous_entry) = occurrence.previous_entry {
            self.declaration_blocks[previous_entry].next_in_history = Some(current_entry);
            if occurrence.starts_history {
                self.declaration_blocks[previous_entry].starts_history = true;
            }
        }
    }
}

pub(crate) fn attach_effective_keys(
    entries: &DeclarationBlockEntries,
    declaration_store: &mut DeclarationBlockStore<'_>,
) {
    for entry in entries.iter() {
        declaration_store.set_effective_key(entry.declarations, entry.effective_key);
    }
}

fn selector_rule<'store, 'ast>(
    rule: RuleId,
    rules: &'store RuleStore<'ast>,
) -> Option<&'store StyleRule<'ast>> {
    match rules.get(rule) {
        CssRule::Style(rule) => Some(rule),
        CssRule::Nesting(rule) => Some(&rule.style),
        _ => None,
    }
}

fn hash_conditional_frame(
    frame: ConditionalFrame,
    rules: &RuleStore<'_>,
    hasher: &mut impl Hasher,
) {
    std::mem::discriminant(&frame).hash(hasher);
    match frame {
        ConditionalFrame::Media(rule) => {
            let CssRule::Media(media) = rules.get(rule) else {
                unreachable!("media frame must reference a media rule")
            };
            media.query.media_queries.len().hash(hasher);
            for query in &media.query.media_queries {
                query.qualifier.is_some().hash(hasher);
                query.condition.is_some().hash(hasher);
                std::mem::discriminant(&query.media_type).hash(hasher);
            }
        }
        ConditionalFrame::Supports(rule) => {
            let CssRule::Supports(supports) = rules.get(rule) else {
                unreachable!("supports frame must reference a supports rule")
            };
            std::mem::discriminant(&*supports.condition).hash(hasher);
        }
        ConditionalFrame::Container(rule) => {
            let CssRule::Container(container) = rules.get(rule) else {
                unreachable!("container frame must reference a container rule")
            };
            container.name.hash(hasher);
            container.condition.is_some().hash(hasher);
            if let Some(condition) = &container.condition {
                std::mem::discriminant(&**condition).hash(hasher);
            }
        }
        ConditionalFrame::Layer(rule) => {
            let CssRule::LayerBlock(layer) = rules.get(rule) else {
                unreachable!("layer frame must reference a layer block")
            };
            layer.name.as_ref().map(|name| name.len()).hash(hasher);
            if let Some(name) = &layer.name {
                for component in name {
                    component.hash(hasher);
                }
            } else {
                // Every anonymous layer is a distinct cascade context.
                rule.hash(hasher);
            }
        }
        ConditionalFrame::Opaque { kind, identity } => {
            kind.hash(hasher);
            identity.hash(hasher);
        }
    }
}

fn conditional_frames_equal(
    left: ConditionalFrame,
    right: ConditionalFrame,
    rules: &RuleStore<'_>,
) -> bool {
    match (left, right) {
        (ConditionalFrame::Media(left), ConditionalFrame::Media(right)) => {
            let (CssRule::Media(left), CssRule::Media(right)) = (rules.get(left), rules.get(right))
            else {
                return false;
            };
            left.query == right.query
        }
        (ConditionalFrame::Supports(left), ConditionalFrame::Supports(right)) => {
            let (CssRule::Supports(left), CssRule::Supports(right)) =
                (rules.get(left), rules.get(right))
            else {
                return false;
            };
            left.condition == right.condition
        }
        (ConditionalFrame::Container(left), ConditionalFrame::Container(right)) => {
            let (CssRule::Container(left), CssRule::Container(right)) =
                (rules.get(left), rules.get(right))
            else {
                return false;
            };
            left.name == right.name && left.condition == right.condition
        }
        (ConditionalFrame::Layer(left_id), ConditionalFrame::Layer(right_id)) => {
            let (CssRule::LayerBlock(left), CssRule::LayerBlock(right)) =
                (rules.get(left_id), rules.get(right_id))
            else {
                return false;
            };
            match (&left.name, &right.name) {
                (Some(left), Some(right)) => left == right,
                (None, None) => left_id == right_id,
                _ => false,
            }
        }
        (
            ConditionalFrame::Opaque {
                kind: left_kind,
                identity: left_identity,
            },
            ConditionalFrame::Opaque {
                kind: right_kind,
                identity: right_identity,
            },
        ) => left_kind == right_kind && left_identity == right_identity,
        _ => false,
    }
}

fn ends_rule_list_segment(rule: &CssRule<'_>, rules: &RuleStore<'_>) -> bool {
    match rule {
        CssRule::Style(rule) => !rules.list_is_empty(rule.rules),
        _ => true,
    }
}

fn opaque_condition(kind: OpaqueConditionalKind, rule: RuleId) -> ConditionalFrame {
    ConditionalFrame::Opaque {
        kind,
        identity: rule,
    }
}

#[cfg(test)]
pub(crate) fn discover_for_test(rules: &RuleStore<'_>) -> DeclarationBlockEntries {
    let mut discovery = DeclarationBlockDiscovery::new(rules, CascadeOrigin::Author);
    for rule in rules.ids() {
        discovery.observe(rule, rules);
    }
    discovery.finish()
}

#[cfg(test)]
mod tests {
    use rocketcss_common::GhostToken;
    use rocketcss_parser::{ParserOptions, parse};

    use super::discover_for_test;

    #[test]
    fn effective_key_includes_typed_conditional_context() {
        GhostToken::scope(|mut token| {
            let stylesheet = parse(
                "a{x:1}@media print{a{x:2}a{x:3}b{x:4}}@media screen{a{x:5}}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = discover_for_test(stylesheet.rule_store());
            assert_eq!(blocks.len(), 5);
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert_ne!(blocks[0].effective_key, blocks[1].effective_key);
            assert_eq!(blocks[1].effective_key, blocks[2].effective_key);
            assert_ne!(blocks[2].effective_key, blocks[3].effective_key);
            assert_ne!(blocks[2].effective_key, blocks[4].effective_key);
        });
    }

    #[test]
    fn effective_key_canonicalizes_equal_layers_but_isolates_opaque_wrappers() {
        GhostToken::scope(|mut token| {
            let stylesheet = parse(
                "@layer utilities{a{x:1}}@layer utilities{a{x:2}}\
                 @scope (.root){a{x:3}}@scope (.root){a{x:4}}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = discover_for_test(stylesheet.rule_store());
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert_eq!(blocks.len(), 4);
            assert_eq!(blocks[0].effective_key, blocks[1].effective_key);
            assert_ne!(blocks[1].effective_key, blocks[2].effective_key);
            assert_ne!(blocks[2].effective_key, blocks[3].effective_key);
        });
    }

    #[test]
    fn effective_key_keeps_anonymous_layers_distinct() {
        GhostToken::scope(|mut token| {
            let stylesheet = parse(
                "@layer{a{x:1}}@layer{a{x:2}}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = discover_for_test(stylesheet.rule_store());
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert_eq!(blocks.len(), 2);
            assert_ne!(blocks[0].effective_key, blocks[1].effective_key);
        });
    }

    #[test]
    fn declaration_histories_are_linked_only_after_a_key_repeats() {
        GhostToken::scope(|mut token| {
            let stylesheet = parse(
                "a{x:1}b{x:2}a{x:3}c{x:4}a{x:5}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = discover_for_test(stylesheet.rule_store());
            assert_eq!(blocks.len(), 5);
            let ids = blocks.ids().collect::<std::vec::Vec<_>>();
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert!(blocks[0].starts_declaration_history());
            assert_eq!(blocks[0].next_declaration_history_entry(), Some(ids[2]));
            assert_eq!(blocks[2].next_declaration_history_entry(), Some(ids[4]));
            assert_eq!(blocks[4].next_declaration_history_entry(), None);
            assert!(!blocks[1].starts_declaration_history());
            assert_eq!(blocks[1].next_declaration_history_entry(), None);
            assert!(!blocks[3].starts_declaration_history());
            assert_eq!(blocks[3].next_declaration_history_entry(), None);
        });
    }

    #[test]
    fn structural_location_distinguishes_lists_segments_and_siblings() {
        GhostToken::scope(|mut token| {
            let stylesheet = parse(
                "a{x:1}a{x:2}@media print{a{x:3}a{x:4}}a{x:5}@layer utilities;a{x:6}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = discover_for_test(stylesheet.rule_store());
            assert_eq!(blocks.len(), 6);
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert!(blocks[0].is_direct_sibling_of(blocks[1]));
            assert!(blocks[2].is_direct_sibling_of(blocks[3]));
            assert_ne!(blocks[1].rule_list, blocks[2].rule_list);
            assert_ne!(blocks[3].rule_list, blocks[4].rule_list);
            assert_eq!(blocks[4].rule_list, blocks[5].rule_list);
            assert_ne!(blocks[4].rule_list_segment, blocks[5].rule_list_segment);
            assert!(!blocks[4].is_direct_sibling_of(blocks[5]));
        });
    }

    #[test]
    fn nested_declarations_use_the_parent_effective_key() {
        GhostToken::scope(|mut token| {
            let stylesheet = parse(
                ".parent{color:red;.child{x:1}background:blue}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = discover_for_test(stylesheet.rule_store());
            assert_eq!(blocks.len(), 3);
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert_eq!(blocks[0].effective_key, blocks[2].effective_key);
            assert_ne!(blocks[0].rule_list, blocks[2].rule_list);
            assert!(blocks[1].is_direct_sibling_of(blocks[2]));
            assert_ne!(blocks[0].declarations, blocks[2].declarations);
        });
    }
}
