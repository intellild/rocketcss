use rocketcss_ast::{
    ContainerCondition, CssRule, DeclarationBlockId, MediaList, SelectorList, Span,
    SupportsCondition, VendorPrefix,
};
#[cfg(test)]
use rocketcss_ast::{StyleRule, StyleSheet};
#[cfg(test)]
use rocketcss_common::vec::Vec;
use rocketcss_common::{DenseIdGenerator, DenseStore, define_dense_id};
use rustc_hash::{FxHashMap, FxHasher};
use smallvec::SmallVec;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ConditionalFrame<'walk, 'ast> {
    Media(&'walk MediaList<'ast>),
    Supports(&'walk SupportsCondition<'ast>),
    Container {
        name: Option<&'ast str>,
        condition: Option<&'walk ContainerCondition<'ast>>,
    },
    Opaque {
        kind: OpaqueConditionalKind,
        identity: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum OpaqueConditionalKind {
    Layer,
    MozDocument,
    Scope,
    StartingStyle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SelectorFrameKind {
    Style,
    Nesting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SelectorFrame<'walk, 'ast> {
    kind: SelectorFrameKind,
    selectors: &'walk SelectorList<'ast>,
    vendor_prefix: VendorPrefix,
}

define_dense_id!(pub(crate) struct SelectorPathId);

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

define_dense_id!(pub(crate) struct ConditionalPathId);

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
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct EffectiveKeyRecord {
    selectors: Option<SelectorPathId>,
    conditions: Option<ConditionalPathId>,
}

#[derive(Debug, Default)]
struct EffectiveKeyInterner {
    buckets: FxHashMap<u64, SmallVec<[EffectiveKeyState; 1]>>,
    ids: DenseIdGenerator<EffectiveKeyId>,
    records: DenseStore<EffectiveKeyId, EffectiveKeyRecord>,
}

impl EffectiveKeyInterner {
    fn intern(
        &mut self,
        selectors: Option<SelectorPathId>,
        conditions: Option<ConditionalPathId>,
        selector_paths: &SelectorPathStore<'_, '_>,
        conditional_paths: &ConditionalPathStore<'_, '_>,
    ) -> EffectiveKeyId {
        let mut hasher = FxHasher::default();
        selector_paths.fingerprint(selectors).hash(&mut hasher);
        conditional_paths.fingerprint(conditions).hash(&mut hasher);
        let bucket = self.buckets.entry(hasher.finish()).or_default();
        if let Some(state) = bucket.iter().find(|state| {
            selector_paths.equals(state.selectors, selectors)
                && conditional_paths.equals(state.conditions, conditions)
        }) {
            return state.id;
        }

        let id = self.ids.allocate();
        let inserted = self.records.push(EffectiveKeyRecord {
            selectors,
            conditions,
        });
        debug_assert_eq!(inserted, id);
        bucket.push(EffectiveKeyState {
            selectors,
            conditions,
            id,
        });
        id
    }
}

/// Persistent effective-context data produced by declaration-block discovery.
///
/// Keeping the path stores and reverse records alive lets later scheduler
/// phases intern keys for synthesized declaration blocks without walking the
/// stylesheet again.
#[derive(Debug, Default)]
pub(crate) struct EffectiveKeyStore<'walk, 'ast> {
    selector_paths: SelectorPathStore<'walk, 'ast>,
    conditional_paths: ConditionalPathStore<'walk, 'ast>,
    interner: EffectiveKeyInterner,
}

impl<'walk, 'ast> EffectiveKeyStore<'walk, 'ast> {
    pub(crate) fn record(&self, id: EffectiveKeyId) -> &EffectiveKeyRecord {
        &self.interner.records[id]
    }

    pub(crate) fn intern_selector_union(
        &mut self,
        left: EffectiveKeyId,
        right: EffectiveKeyId,
        selectors: &'walk SelectorList<'ast>,
        vendor_prefix: VendorPrefix,
    ) -> Option<EffectiveKeyId> {
        let left = *self.record(left);
        let right = *self.record(right);
        if !self
            .conditional_paths
            .equals(left.conditions, right.conditions)
        {
            return None;
        }

        let (Some(left_selectors), Some(right_selectors)) = (left.selectors, right.selectors)
        else {
            return None;
        };
        let left_node = self.selector_paths.nodes[left_selectors];
        let right_node = self.selector_paths.nodes[right_selectors];
        if left_node.frame.kind != SelectorFrameKind::Style
            || right_node.frame.kind != SelectorFrameKind::Style
            || left_node.frame.vendor_prefix != vendor_prefix
            || right_node.frame.vendor_prefix != vendor_prefix
            || !self
                .selector_paths
                .equals(left_node.parent, right_node.parent)
        {
            return None;
        }

        let selector_path = self.selector_paths.push(
            left_node.parent,
            SelectorFrame {
                kind: SelectorFrameKind::Style,
                selectors,
                vendor_prefix,
            },
        );
        Some(self.interner.intern(
            Some(selector_path),
            left.conditions,
            &self.selector_paths,
            &self.conditional_paths,
        ))
    }
}

define_dense_id!(pub(crate) struct RuleListId);
define_dense_id!(pub(crate) struct RuleListSegmentId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SiblingOrdinal(u32);

impl SiblingOrdinal {
    pub(crate) fn from_index(index: usize) -> Self {
        Self(u32::try_from(index).expect("sibling ordinal exceeds u32::MAX"))
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct StructuralLocation {
    pub(crate) rule_list: RuleListId,
    pub(crate) rule_list_segment: RuleListSegmentId,
    pub(crate) sibling_ordinal: SiblingOrdinal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DeclarationBlockKind<'walk, 'ast> {
    Style {
        selectors: &'walk SelectorList<'ast>,
        span: Span,
        vendor_prefix: VendorPrefix,
        has_children: bool,
        has_live_selectors: bool,
    },
    Nesting,
    NestedDeclarations,
}

#[derive(Debug)]
pub(crate) struct DeclarationBlockEntry<'walk, 'ast> {
    pub(crate) declarations: DeclarationBlockId,
    pub(crate) effective_key: EffectiveKeyId,
    pub(crate) kind: DeclarationBlockKind<'walk, 'ast>,
    pub(crate) rule_list: RuleListId,
    pub(crate) rule_list_segment: RuleListSegmentId,
    pub(crate) sibling_ordinal: SiblingOrdinal,
}

impl DeclarationBlockEntry<'_, '_> {
    pub(crate) fn is_direct_sibling_of(&self, right: &Self) -> bool {
        self.rule_list == right.rule_list
            && self.rule_list_segment == right.rule_list_segment
            && self.sibling_ordinal.0.checked_add(1) == Some(right.sibling_ordinal.0)
    }
}

#[derive(Debug, Default)]
pub(crate) struct WalkState {
    rule_lists: DenseIdGenerator<RuleListId>,
    rule_list_segments: DenseIdGenerator<RuleListSegmentId>,
    opaque_conditions: u32,
}

impl WalkState {
    pub(crate) fn allocate_rule_list(&mut self) -> RuleListId {
        self.rule_lists.allocate()
    }

    pub(crate) fn allocate_rule_list_segment(&mut self) -> RuleListSegmentId {
        self.rule_list_segments.allocate()
    }

    fn allocate_opaque_condition(&mut self) -> u32 {
        let identity = self.opaque_conditions;
        self.opaque_conditions = self
            .opaque_conditions
            .checked_add(1)
            .expect("opaque conditional wrapper count exceeds u32::MAX");
        identity
    }
}

pub(crate) type DeclarationBlockEntries<'walk, 'ast> =
    DenseStore<DeclarationBlockEntryId, DeclarationBlockEntry<'walk, 'ast>>;

#[derive(Debug)]
pub(crate) struct DeclarationBlockDiscovery<'walk, 'ast> {
    pub(crate) declaration_blocks: DeclarationBlockEntries<'walk, 'ast>,
    pub(crate) effective_keys: EffectiveKeyStore<'walk, 'ast>,
}

#[cfg(test)]
pub(crate) fn discover_declaration_blocks<'walk, 'ast>(
    stylesheet: &'walk StyleSheet<'ast>,
) -> DeclarationBlockDiscovery<'walk, 'ast> {
    let mut walker = DeclarationBlockCollector::new();
    walker.collect_rule_list(&stylesheet.rules);
    walker.finish()
}

#[cfg(test)]
pub(crate) fn walk_declaration_blocks<'walk, 'ast>(
    stylesheet: &'walk StyleSheet<'ast>,
) -> DeclarationBlockEntries<'walk, 'ast> {
    discover_declaration_blocks(stylesheet).declaration_blocks
}

pub(crate) struct DeclarationBlockCollector<'walk, 'ast> {
    selector_path: Option<SelectorPathId>,
    conditional_path: Option<ConditionalPathId>,
    effective_keys: EffectiveKeyStore<'walk, 'ast>,
    declaration_blocks: DeclarationBlockEntries<'walk, 'ast>,
    state: WalkState,
}

impl<'walk, 'ast> DeclarationBlockCollector<'walk, 'ast> {
    pub(crate) fn new() -> Self {
        Self {
            selector_path: None,
            conditional_path: None,
            effective_keys: EffectiveKeyStore::default(),
            declaration_blocks: DenseStore::new(),
            state: WalkState::default(),
        }
    }

    pub(crate) fn finish(self) -> DeclarationBlockDiscovery<'walk, 'ast> {
        DeclarationBlockDiscovery {
            declaration_blocks: self.declaration_blocks,
            effective_keys: self.effective_keys,
        }
    }

    pub(crate) fn allocate_rule_list(&mut self) -> RuleListId {
        self.state.allocate_rule_list()
    }

    pub(crate) fn allocate_rule_list_segment(&mut self) -> RuleListSegmentId {
        self.state.allocate_rule_list_segment()
    }

    pub(crate) fn enter_condition(
        &mut self,
        frame: ConditionalFrame<'walk, 'ast>,
    ) -> Option<ConditionalPathId> {
        let parent = self.conditional_path;
        self.conditional_path = Some(self.effective_keys.conditional_paths.push(parent, frame));
        parent
    }

    pub(crate) fn enter_opaque_condition(
        &mut self,
        kind: OpaqueConditionalKind,
    ) -> Option<ConditionalPathId> {
        let identity = self.state.allocate_opaque_condition();
        self.enter_condition(ConditionalFrame::Opaque { kind, identity })
    }

    pub(crate) fn leave_condition(&mut self, parent: Option<ConditionalPathId>) {
        self.conditional_path = parent;
    }

    pub(crate) fn enter_selector(
        &mut self,
        kind: SelectorFrameKind,
        selectors: &'walk SelectorList<'ast>,
        vendor_prefix: VendorPrefix,
    ) -> Option<SelectorPathId> {
        let parent = self.selector_path;
        self.selector_path = Some(self.effective_keys.selector_paths.push(
            parent,
            SelectorFrame {
                kind,
                selectors,
                vendor_prefix,
            },
        ));
        parent
    }

    pub(crate) fn leave_selector(&mut self, parent: Option<SelectorPathId>) {
        self.selector_path = parent;
    }

    #[cfg(test)]
    fn collect_rule_list(&mut self, rules: &'walk Vec<'ast, CssRule<'ast>>) {
        let rule_list = self.state.allocate_rule_list();
        let mut rule_list_segment = self.state.allocate_rule_list_segment();
        self.declaration_blocks.reserve(rules.len());

        for (sibling_ordinal, rule) in rules.iter().enumerate() {
            let sibling_ordinal = SiblingOrdinal::from_index(sibling_ordinal);
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

    #[cfg(test)]
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
            CssRule::MozDocument(rule) => {
                self.with_opaque_condition(OpaqueConditionalKind::MozDocument, &rule.rules)
            }
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
            CssRule::LayerBlock(rule) => {
                self.with_opaque_condition(OpaqueConditionalKind::Layer, &rule.rules)
            }
            CssRule::Container(rule) => self.with_condition(
                ConditionalFrame::Container {
                    name: rule.name,
                    condition: rule.condition.as_deref(),
                },
                &rule.rules,
            ),
            CssRule::Scope(rule) => {
                self.with_opaque_condition(OpaqueConditionalKind::Scope, &rule.rules)
            }
            CssRule::StartingStyle(rule) => {
                self.with_opaque_condition(OpaqueConditionalKind::StartingStyle, &rule.rules)
            }
            _ => {}
        }
    }

    #[cfg(test)]
    fn collect_style_rule(
        &mut self,
        rule: &'walk StyleRule<'ast>,
        kind: SelectorFrameKind,
        location: StructuralLocation,
    ) {
        let parent_selector_path = self.enter_selector(kind, &rule.selectors, rule.vendor_prefix);
        self.push_declaration_block(
            rule.declarations,
            match kind {
                SelectorFrameKind::Style => DeclarationBlockKind::Style {
                    selectors: &rule.selectors,
                    span: rule.span,
                    vendor_prefix: rule.vendor_prefix,
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
        if !rule.rules.is_empty() {
            self.collect_rule_list(&rule.rules);
        }
        self.leave_selector(parent_selector_path);
    }

    #[cfg(test)]
    fn with_condition(
        &mut self,
        frame: ConditionalFrame<'walk, 'ast>,
        rules: &'walk Vec<'ast, CssRule<'ast>>,
    ) {
        let parent_conditional_path = self.conditional_path;
        self.conditional_path = Some(
            self.effective_keys
                .conditional_paths
                .push(parent_conditional_path, frame),
        );
        self.collect_rule_list(rules);
        self.conditional_path = parent_conditional_path;
    }

    #[cfg(test)]
    fn with_opaque_condition(
        &mut self,
        kind: OpaqueConditionalKind,
        rules: &'walk Vec<'ast, CssRule<'ast>>,
    ) {
        let parent = self.enter_opaque_condition(kind);
        self.collect_rule_list(rules);
        self.leave_condition(parent);
    }

    pub(crate) fn push_declaration_block(
        &mut self,
        declarations: DeclarationBlockId,
        kind: DeclarationBlockKind<'walk, 'ast>,
        location: StructuralLocation,
    ) {
        let effective_key = self.effective_keys.interner.intern(
            self.selector_path,
            self.conditional_path,
            &self.effective_keys.selector_paths,
            &self.effective_keys.conditional_paths,
        );
        self.declaration_blocks.push(DeclarationBlockEntry {
            declarations,
            effective_key,
            kind,
            rule_list: location.rule_list,
            rule_list_segment: location.rule_list_segment,
            sibling_ordinal: location.sibling_ordinal,
        });
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

pub(crate) fn ends_rule_list_segment(rule: &CssRule<'_>) -> bool {
    match rule {
        CssRule::Style(rule) => !rule.as_ref().get_ref().rules.is_empty(),
        _ => true,
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
    fn repeated_effective_keys_are_stable_in_source_order() {
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
            let blocks = blocks.iter().collect::<std::vec::Vec<_>>();
            assert_eq!(blocks[0].effective_key, blocks[2].effective_key);
            assert_eq!(blocks[2].effective_key, blocks[4].effective_key);
            assert_ne!(blocks[0].effective_key, blocks[1].effective_key);
            assert_ne!(blocks[0].effective_key, blocks[3].effective_key);
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
