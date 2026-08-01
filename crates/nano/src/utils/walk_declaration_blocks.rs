use rocketcss_ast::{
    ContainerCondition, CssRule, DeclarationBlockId, MediaList, SelectorList, StyleRule,
    StyleSheet, SupportsCondition, VendorPrefix,
};
use rocketcss_common::{DenseIdGenerator, DenseStore, define_dense_id, vec::Vec};
use rustc_hash::{FxHashMap, FxHasher};
use smallvec::SmallVec;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq)]
enum ConditionalFrame<'walk, 'ast> {
    Media(&'walk MediaList<'ast>),
    Supports(&'walk SupportsCondition<'ast>),
    Container {
        name: Option<&'ast str>,
        condition: Option<&'walk ContainerCondition<'ast>>,
    },
    Opaque {
        kind: OpaqueConditionalKind,
        identity: *const (),
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum OpaqueConditionalKind {
    Layer,
    MozDocument,
    Scope,
    StartingStyle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SelectorFrameKind {
    Style,
    Nesting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SelectorFrame<'walk, 'ast> {
    kind: SelectorFrameKind,
    selectors: &'walk SelectorList<'ast>,
    vendor_prefix: VendorPrefix,
}

define_dense_id!(struct SelectorPathId);

#[derive(Clone, Copy, Debug)]
struct SelectorPathNode<'walk, 'ast> {
    parent: Option<SelectorPathId>,
    frame: SelectorFrame<'walk, 'ast>,
    fingerprint: u64,
}

#[derive(Debug, Default)]
struct SelectorPathStore<'walk, 'ast> {
    nodes: DenseStore<SelectorPathId, SelectorPathNode<'walk, 'ast>>,
}

impl<'walk, 'ast> SelectorPathStore<'walk, 'ast> {
    fn push(
        &mut self,
        parent: Option<SelectorPathId>,
        frame: SelectorFrame<'walk, 'ast>,
    ) -> SelectorPathId {
        let mut hasher = FxHasher::default();
        self.fingerprint(parent).hash(&mut hasher);
        frame.hash(&mut hasher);
        self.nodes.push(SelectorPathNode {
            parent,
            frame,
            fingerprint: hasher.finish(),
        })
    }

    fn fingerprint(&self, path: Option<SelectorPathId>) -> u64 {
        path.map_or(0, |path| self.nodes[path].fingerprint)
    }

    fn equals(&self, mut left: Option<SelectorPathId>, mut right: Option<SelectorPathId>) -> bool {
        if left == right {
            return true;
        }
        loop {
            let (left_id, right_id) = match (left, right) {
                (Some(left), Some(right)) => (left, right),
                (None, None) => return true,
                _ => return false,
            };
            let left_node = &self.nodes[left_id];
            let right_node = &self.nodes[right_id];
            if left_node.frame != right_node.frame {
                return false;
            }
            left = left_node.parent;
            right = right_node.parent;
        }
    }
}

define_dense_id!(struct ConditionalPathId);

#[derive(Clone, Copy, Debug)]
struct ConditionalPathNode<'walk, 'ast> {
    parent: Option<ConditionalPathId>,
    frame: ConditionalFrame<'walk, 'ast>,
    fingerprint: u64,
}

#[derive(Debug, Default)]
struct ConditionalPathStore<'walk, 'ast> {
    nodes: DenseStore<ConditionalPathId, ConditionalPathNode<'walk, 'ast>>,
}

impl<'walk, 'ast> ConditionalPathStore<'walk, 'ast> {
    fn push(
        &mut self,
        parent: Option<ConditionalPathId>,
        frame: ConditionalFrame<'walk, 'ast>,
    ) -> ConditionalPathId {
        let mut hasher = FxHasher::default();
        self.fingerprint(parent).hash(&mut hasher);
        hash_conditional_frame(&frame, &mut hasher);
        self.nodes.push(ConditionalPathNode {
            parent,
            frame,
            fingerprint: hasher.finish(),
        })
    }

    fn fingerprint(&self, path: Option<ConditionalPathId>) -> u64 {
        path.map_or(0, |path| self.nodes[path].fingerprint)
    }

    fn equals(
        &self,
        mut left: Option<ConditionalPathId>,
        mut right: Option<ConditionalPathId>,
    ) -> bool {
        if left == right {
            return true;
        }
        loop {
            let (left_id, right_id) = match (left, right) {
                (Some(left), Some(right)) => (left, right),
                (None, None) => return true,
                _ => return false,
            };
            let left_node = &self.nodes[left_id];
            let right_node = &self.nodes[right_id];
            if left_node.frame != right_node.frame {
                return false;
            }
            left = left_node.parent;
            right = right_node.parent;
        }
    }
}

define_dense_id!(
    /// The dense declaration-history identity computed during source-ordered
    /// discovery.
    ///
    /// Selector and condition paths are tracked independently because nested
    /// selector resolution is a separate feature. Typed conditional frames use
    /// structural equality. At-rule kinds whose semantics are not handled by
    /// this pass are isolated by authored wrapper identity.
    pub(crate) struct EffectiveKeyId;
);
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

#[derive(Debug, Default)]
struct EffectiveKeyInterner {
    buckets: FxHashMap<u64, SmallVec<[EffectiveKeyState; 1]>>,
    ids: DenseIdGenerator<EffectiveKeyId>,
}

impl EffectiveKeyInterner {
    fn intern(
        &mut self,
        selectors: Option<SelectorPathId>,
        conditions: Option<ConditionalPathId>,
        selector_paths: &SelectorPathStore<'_, '_>,
        conditional_paths: &ConditionalPathStore<'_, '_>,
        current_entry: DeclarationBlockEntryId,
    ) -> EffectiveKeyOccurrence {
        let mut hasher = FxHasher::default();
        selector_paths.fingerprint(selectors).hash(&mut hasher);
        conditional_paths.fingerprint(conditions).hash(&mut hasher);
        let bucket = self.buckets.entry(hasher.finish()).or_default();
        if let Some(state) = bucket.iter_mut().find(|state| {
            selector_paths.equals(state.selectors, selectors)
                && conditional_paths.equals(state.conditions, conditions)
        }) {
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

define_dense_id!(pub(crate) struct RuleListId);
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
    rule_lists: DenseIdGenerator<RuleListId>,
    rule_list_segments: DenseIdGenerator<RuleListSegmentId>,
}

impl WalkState {
    fn allocate_rule_list(&mut self) -> RuleListId {
        self.rule_lists.allocate()
    }

    fn allocate_rule_list_segment(&mut self) -> RuleListSegmentId {
        self.rule_list_segments.allocate()
    }
}

pub(crate) type DeclarationBlockEntries =
    DenseStore<DeclarationBlockEntryId, DeclarationBlockEntry>;

pub(crate) fn walk_declaration_blocks<'walk, 'ast>(
    stylesheet: &'walk StyleSheet<'ast>,
) -> DeclarationBlockEntries {
    let mut walker = DeclarationBlockWalker::new();
    walker.collect_rule_list(&stylesheet.rules);
    walker.declaration_blocks
}

struct DeclarationBlockWalker<'walk, 'ast> {
    selector_path: Option<SelectorPathId>,
    selector_paths: SelectorPathStore<'walk, 'ast>,
    conditional_path: Option<ConditionalPathId>,
    conditional_paths: ConditionalPathStore<'walk, 'ast>,
    effective_keys: EffectiveKeyInterner,
    declaration_blocks: DeclarationBlockEntries,
    state: WalkState,
}

impl<'walk, 'ast> DeclarationBlockWalker<'walk, 'ast> {
    fn new() -> Self {
        Self {
            selector_path: None,
            selector_paths: SelectorPathStore::default(),
            conditional_path: None,
            conditional_paths: ConditionalPathStore::default(),
            effective_keys: EffectiveKeyInterner::default(),
            declaration_blocks: DenseStore::new(),
            state: WalkState::default(),
        }
    }

    fn collect_rule_list(&mut self, rules: &'walk Vec<'ast, CssRule<'ast>>) {
        let rule_list = self.state.allocate_rule_list();
        let mut rule_list_segment = self.state.allocate_rule_list_segment();
        self.declaration_blocks.reserve(rules.len());

        for (sibling_ordinal, rule) in rules.iter().enumerate() {
            let sibling_ordinal = SiblingOrdinal(
                u32::try_from(sibling_ordinal).expect("sibling ordinal exceeds u32::MAX"),
            );
            self.collect_rule(
                rule,
                StructuralLocation {
                    rule_list,
                    rule_list_segment,
                    sibling_ordinal,
                },
            );
            if ends_rule_list_segment(rule) {
                rule_list_segment = self.state.allocate_rule_list_segment();
            }
        }
    }

    fn collect_rule(&mut self, rule: &'walk CssRule<'ast>, location: StructuralLocation) {
        match rule {
            CssRule::Media(rule) => {
                self.with_condition(ConditionalFrame::Media(&rule.query), &rule.rules)
            }
            CssRule::Style(rule) => {
                self.collect_style_rule(rule.as_ref().get_ref(), SelectorFrameKind::Style, location)
            }
            CssRule::Supports(rule) => {
                self.with_condition(ConditionalFrame::Supports(&rule.condition), &rule.rules)
            }
            CssRule::MozDocument(rule) => self.with_condition(
                opaque_condition(OpaqueConditionalKind::MozDocument, rule.as_ref()),
                &rule.rules,
            ),
            CssRule::Nesting(rule) => self.collect_style_rule(
                rule.style.as_ref().get_ref(),
                SelectorFrameKind::Nesting,
                location,
            ),
            CssRule::NestedDeclarations(rule) => {
                self.push_declaration_block(
                    rule.declarations,
                    DeclarationBlockKind::NestedDeclarations,
                    location,
                );
            }
            CssRule::LayerBlock(rule) => self.with_condition(
                opaque_condition(OpaqueConditionalKind::Layer, rule.as_ref()),
                &rule.rules,
            ),
            CssRule::Container(rule) => self.with_condition(
                ConditionalFrame::Container {
                    name: rule.name,
                    condition: rule.condition.as_deref(),
                },
                &rule.rules,
            ),
            CssRule::Scope(rule) => self.with_condition(
                opaque_condition(OpaqueConditionalKind::Scope, rule.as_ref()),
                &rule.rules,
            ),
            CssRule::StartingStyle(rule) => self.with_condition(
                opaque_condition(OpaqueConditionalKind::StartingStyle, rule.as_ref()),
                &rule.rules,
            ),
            _ => {}
        }
    }

    fn collect_style_rule(
        &mut self,
        rule: &'walk StyleRule<'ast>,
        kind: SelectorFrameKind,
        location: StructuralLocation,
    ) {
        let parent_selector_path = self.selector_path;
        self.selector_path = Some(self.selector_paths.push(
            parent_selector_path,
            SelectorFrame {
                kind,
                selectors: &rule.selectors,
                vendor_prefix: rule.vendor_prefix,
            },
        ));
        self.push_declaration_block(
            rule.declarations,
            match kind {
                SelectorFrameKind::Style => DeclarationBlockKind::Style {
                    has_children: !rule.rules.is_empty(),
                    has_live_selectors: rule
                        .selectors
                        .iter()
                        .any(|selector| !selector.is_tombstone()),
                },
                SelectorFrameKind::Nesting => DeclarationBlockKind::Nesting,
            },
            location,
        );
        self.collect_rule_list(&rule.rules);
        self.selector_path = parent_selector_path;
    }

    fn with_condition(
        &mut self,
        frame: ConditionalFrame<'walk, 'ast>,
        rules: &'walk Vec<'ast, CssRule<'ast>>,
    ) {
        let parent_conditional_path = self.conditional_path;
        self.conditional_path = Some(self.conditional_paths.push(parent_conditional_path, frame));
        self.collect_rule_list(rules);
        self.conditional_path = parent_conditional_path;
    }

    fn push_declaration_block(
        &mut self,
        declarations: DeclarationBlockId,
        kind: DeclarationBlockKind,
        location: StructuralLocation,
    ) {
        let current_entry = self.declaration_blocks.next_id();
        let occurrence = self.effective_keys.intern(
            self.selector_path,
            self.conditional_path,
            &self.selector_paths,
            &self.conditional_paths,
            current_entry,
        );
        let inserted = self.declaration_blocks.push(DeclarationBlockEntry {
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

fn hash_conditional_frame(frame: &ConditionalFrame<'_, '_>, hasher: &mut impl Hasher) {
    std::mem::discriminant(frame).hash(hasher);
    match frame {
        ConditionalFrame::Media(media) => {
            media.media_queries.len().hash(hasher);
            for query in &media.media_queries {
                query.qualifier.is_some().hash(hasher);
                query.condition.is_some().hash(hasher);
                std::mem::discriminant(&query.media_type).hash(hasher);
            }
        }
        ConditionalFrame::Supports(condition) => {
            std::mem::discriminant(*condition).hash(hasher);
        }
        ConditionalFrame::Container { name, condition } => {
            name.hash(hasher);
            condition.is_some().hash(hasher);
            if let Some(condition) = condition {
                std::mem::discriminant(*condition).hash(hasher);
            }
        }
        ConditionalFrame::Opaque { kind, identity } => {
            kind.hash(hasher);
            identity.hash(hasher);
        }
    }
}

fn ends_rule_list_segment(rule: &CssRule<'_>) -> bool {
    match rule {
        CssRule::Style(rule) => !rule.as_ref().get_ref().rules.is_empty(),
        _ => true,
    }
}

fn opaque_condition<'walk, 'ast, T>(
    kind: OpaqueConditionalKind,
    rule: &'walk T,
) -> ConditionalFrame<'walk, 'ast> {
    ConditionalFrame::Opaque {
        kind,
        identity: std::ptr::from_ref(rule).cast(),
    }
}

#[cfg(test)]
mod tests {
    use rocketcss_common::Allocator;
    use rocketcss_parser::{ParserOptions, parse};

    use super::walk_declaration_blocks;

    #[test]
    fn effective_key_includes_typed_conditional_context() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                "a{x:1}@media print{a{x:2}a{x:3}b{x:4}}@media screen{a{x:5}}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = walk_declaration_blocks(&stylesheet);
            assert_eq!(blocks.len(), 5);
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert_ne!(blocks[0].effective_key, blocks[1].effective_key);
            assert_eq!(blocks[1].effective_key, blocks[2].effective_key);
            assert_ne!(blocks[2].effective_key, blocks[3].effective_key);
            assert_ne!(blocks[2].effective_key, blocks[4].effective_key);
        });
    }

    #[test]
    fn declaration_histories_are_linked_only_after_a_key_repeats() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                "a{x:1}b{x:2}a{x:3}c{x:4}a{x:5}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = walk_declaration_blocks(&stylesheet);
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
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                "a{x:1}a{x:2}@media print{a{x:3}a{x:4}}a{x:5}@layer utilities;a{x:6}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = walk_declaration_blocks(&stylesheet);
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
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                ".parent{color:red;.child{x:1}background:blue}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();

            let blocks = walk_declaration_blocks(&stylesheet);
            assert_eq!(blocks.len(), 3);
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert_eq!(blocks[0].effective_key, blocks[2].effective_key);
            assert_ne!(blocks[0].rule_list, blocks[2].rule_list);
            assert!(blocks[1].is_direct_sibling_of(blocks[2]));
            assert_ne!(blocks[0].declarations, blocks[2].declarations);
        });
    }
}
