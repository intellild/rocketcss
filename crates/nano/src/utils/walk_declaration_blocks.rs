use std::hash::{Hash, Hasher};
use std::num::NonZeroU32;

use rocketcss_allocator::{Allocator, GhostToken, Ref, hash_map::HashMap, vec::Vec};
use rocketcss_ast::{
    ContainerCondition, CssRule, DeclarationBlock, MediaList, SelectorList, StyleRule, StyleSheet,
    SupportsCondition, VendorPrefix,
};
use rustc_hash::FxHasher;

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct SelectorPathId(u32);

impl SelectorPathId {
    fn index(self) -> usize {
        usize::try_from(self.0 - 1).expect("selector path ID fits usize")
    }
}

#[derive(Clone, Copy, Debug)]
struct SelectorPathNode<'walk, 'ast> {
    parent: SelectorPathId,
    frame: SelectorFrame<'walk, 'ast>,
    fingerprint: u64,
}

#[derive(Debug, Default)]
struct SelectorPathStore<'walk, 'ast> {
    nodes: std::vec::Vec<SelectorPathNode<'walk, 'ast>>,
}

impl<'walk, 'ast> SelectorPathStore<'walk, 'ast> {
    fn push(
        &mut self,
        parent: SelectorPathId,
        frame: SelectorFrame<'walk, 'ast>,
    ) -> SelectorPathId {
        let mut hasher = FxHasher::default();
        self.fingerprint(parent).hash(&mut hasher);
        frame.hash(&mut hasher);
        let id = SelectorPathId(
            u32::try_from(self.nodes.len())
                .expect("selector path count exceeds u32::MAX")
                .checked_add(1)
                .expect("selector path count exceeds u32::MAX"),
        );
        self.nodes.push(SelectorPathNode {
            parent,
            frame,
            fingerprint: hasher.finish(),
        });
        id
    }

    fn fingerprint(&self, path: SelectorPathId) -> u64 {
        if path == SelectorPathId::default() {
            0
        } else {
            self.nodes[path.index()].fingerprint
        }
    }

    fn equals(&self, mut left: SelectorPathId, mut right: SelectorPathId) -> bool {
        if left == right {
            return true;
        }
        loop {
            if left == SelectorPathId::default() || right == SelectorPathId::default() {
                return left == right;
            }
            let left_node = &self.nodes[left.index()];
            let right_node = &self.nodes[right.index()];
            if left_node.frame != right_node.frame {
                return false;
            }
            left = left_node.parent;
            right = right_node.parent;
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct ConditionalPathId(u32);

impl ConditionalPathId {
    fn index(self) -> usize {
        usize::try_from(self.0 - 1).expect("conditional path ID fits usize")
    }
}

#[derive(Clone, Copy, Debug)]
struct ConditionalPathNode<'walk, 'ast> {
    parent: ConditionalPathId,
    frame: ConditionalFrame<'walk, 'ast>,
    fingerprint: u64,
}

#[derive(Debug, Default)]
struct ConditionalPathStore<'walk, 'ast> {
    nodes: std::vec::Vec<ConditionalPathNode<'walk, 'ast>>,
}

impl<'walk, 'ast> ConditionalPathStore<'walk, 'ast> {
    fn push(
        &mut self,
        parent: ConditionalPathId,
        frame: ConditionalFrame<'walk, 'ast>,
    ) -> ConditionalPathId {
        let mut hasher = FxHasher::default();
        self.fingerprint(parent).hash(&mut hasher);
        hash_conditional_frame(&frame, &mut hasher);
        let id = ConditionalPathId(
            u32::try_from(self.nodes.len())
                .expect("conditional path count exceeds u32::MAX")
                .checked_add(1)
                .expect("conditional path count exceeds u32::MAX"),
        );
        self.nodes.push(ConditionalPathNode {
            parent,
            frame,
            fingerprint: hasher.finish(),
        });
        id
    }

    fn fingerprint(&self, path: ConditionalPathId) -> u64 {
        if path == ConditionalPathId::default() {
            0
        } else {
            self.nodes[path.index()].fingerprint
        }
    }

    fn equals(&self, mut left: ConditionalPathId, mut right: ConditionalPathId) -> bool {
        if left == right {
            return true;
        }
        loop {
            if left == ConditionalPathId::default() || right == ConditionalPathId::default() {
                return left == right;
            }
            let left_node = &self.nodes[left.index()];
            let right_node = &self.nodes[right.index()];
            if left_node.frame != right_node.frame {
                return false;
            }
            left = left_node.parent;
            right = right_node.parent;
        }
    }
}

/// The dense declaration-history identity computed during source-ordered
/// discovery.
///
/// Selector and condition paths are tracked independently because nested
/// selector resolution is a separate feature. Typed conditional frames use
/// structural equality. At-rule kinds whose semantics are not handled by this
/// pass are isolated by authored wrapper identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct EffectiveKeyId(u32);

#[derive(Clone, Copy, Debug)]
struct EffectiveKeyState {
    selectors: SelectorPathId,
    conditions: ConditionalPathId,
    id: EffectiveKeyId,
    last_entry: u32,
    has_history: bool,
}

#[derive(Clone, Copy, Debug)]
struct EffectiveKeyOccurrence {
    id: EffectiveKeyId,
    previous_entry: Option<u32>,
    starts_history: bool,
}

struct EffectiveKeyInterner<'scratch> {
    allocator: &'scratch Allocator,
    buckets: HashMap<'scratch, u64, Vec<'scratch, EffectiveKeyState>>,
    next_id: u32,
}

impl<'scratch> EffectiveKeyInterner<'scratch> {
    fn new(allocator: &'scratch Allocator) -> Self {
        Self {
            allocator,
            buckets: HashMap::new_in(allocator),
            next_id: 0,
        }
    }

    fn intern(
        &mut self,
        selectors: SelectorPathId,
        conditions: ConditionalPathId,
        selector_paths: &SelectorPathStore<'_, '_>,
        conditional_paths: &ConditionalPathStore<'_, '_>,
        current_entry: u32,
    ) -> EffectiveKeyOccurrence {
        let mut hasher = FxHasher::default();
        selector_paths.fingerprint(selectors).hash(&mut hasher);
        conditional_paths.fingerprint(conditions).hash(&mut hasher);
        let allocator = self.allocator;
        let bucket = self
            .buckets
            .entry(hasher.finish())
            .or_insert_with(|| Vec::new_in(allocator));
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

        let id = EffectiveKeyId(self.next_id);
        self.next_id = self
            .next_id
            .checked_add(1)
            .expect("effective key count exceeds u32::MAX");
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct RuleListId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct RuleListSegmentId(u32);

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
pub(crate) struct DeclarationBlockEntry<'walk, 'ast, 'ghost> {
    pub(crate) declarations: &'walk DeclarationBlock<'ast, 'ghost>,
    pub(crate) declaration_ref: Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
    pub(crate) effective_key: EffectiveKeyId,
    pub(crate) kind: DeclarationBlockKind,
    pub(crate) rule_list: RuleListId,
    pub(crate) rule_list_segment: RuleListSegmentId,
    pub(crate) sibling_ordinal: SiblingOrdinal,
    next_in_history: Option<NonZeroU32>,
    starts_history: bool,
}

impl DeclarationBlockEntry<'_, '_, '_> {
    pub(crate) fn is_direct_sibling_of(&self, right: &Self) -> bool {
        self.rule_list == right.rule_list
            && self.rule_list_segment == right.rule_list_segment
            && self.sibling_ordinal.0.checked_add(1) == Some(right.sibling_ordinal.0)
    }

    pub(crate) fn starts_declaration_history(&self) -> bool {
        self.starts_history
    }

    pub(crate) fn next_declaration_history_entry(&self) -> Option<usize> {
        self.next_in_history.map(|next| {
            usize::try_from(next.get() - 1).expect("declaration block index fits usize")
        })
    }
}

#[derive(Debug, Default)]
struct WalkState {
    next_rule_list: u32,
    next_rule_list_segment: u32,
}

impl WalkState {
    fn allocate_rule_list(&mut self) -> RuleListId {
        let id = RuleListId(self.next_rule_list);
        self.next_rule_list = self
            .next_rule_list
            .checked_add(1)
            .expect("rule list count exceeds u32::MAX");
        id
    }

    fn allocate_rule_list_segment(&mut self) -> RuleListSegmentId {
        let id = RuleListSegmentId(self.next_rule_list_segment);
        self.next_rule_list_segment = self
            .next_rule_list_segment
            .checked_add(1)
            .expect("rule list segment count exceeds u32::MAX");
        id
    }
}

pub(crate) fn walk_declaration_blocks<'walk, 'ast, 'ghost>(
    stylesheet: &'walk StyleSheet<'ast, 'ghost>,
    token: &'walk GhostToken<'ghost>,
    allocator: &Allocator,
) -> std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>> {
    let mut walker = DeclarationBlockWalker::new(token, allocator);
    walker.collect_rule_list(&stylesheet.rules);
    walker.declaration_blocks
}

struct DeclarationBlockWalker<'scratch, 'walk, 'ast, 'ghost> {
    token: &'walk GhostToken<'ghost>,
    selector_path: SelectorPathId,
    selector_paths: SelectorPathStore<'walk, 'ast>,
    conditional_path: ConditionalPathId,
    conditional_paths: ConditionalPathStore<'walk, 'ast>,
    effective_keys: EffectiveKeyInterner<'scratch>,
    declaration_blocks: std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
    state: WalkState,
}

impl<'scratch, 'walk, 'ast, 'ghost> DeclarationBlockWalker<'scratch, 'walk, 'ast, 'ghost> {
    fn new(token: &'walk GhostToken<'ghost>, allocator: &'scratch Allocator) -> Self {
        Self {
            token,
            selector_path: SelectorPathId::default(),
            selector_paths: SelectorPathStore::default(),
            conditional_path: ConditionalPathId::default(),
            conditional_paths: ConditionalPathStore::default(),
            effective_keys: EffectiveKeyInterner::new(allocator),
            declaration_blocks: std::vec::Vec::new(),
            state: WalkState::default(),
        }
    }

    fn collect_rule_list(&mut self, rules: &'walk Vec<'ast, CssRule<'ast, 'ghost>>) {
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

    fn collect_rule(&mut self, rule: &'walk CssRule<'ast, 'ghost>, location: StructuralLocation) {
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
                    rule.declarations.as_ref().borrow(self.token).get_ref(),
                    Ref::from(&rule.declarations),
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
        rule: &'walk StyleRule<'ast, 'ghost>,
        kind: SelectorFrameKind,
        location: StructuralLocation,
    ) {
        let parent_selector_path = self.selector_path;
        self.selector_path = self.selector_paths.push(
            parent_selector_path,
            SelectorFrame {
                kind,
                selectors: &rule.selectors,
                vendor_prefix: rule.vendor_prefix,
            },
        );
        self.push_declaration_block(
            rule.declarations.as_ref().borrow(self.token).get_ref(),
            Ref::from(&rule.declarations),
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
        rules: &'walk Vec<'ast, CssRule<'ast, 'ghost>>,
    ) {
        let parent_conditional_path = self.conditional_path;
        self.conditional_path = self.conditional_paths.push(parent_conditional_path, frame);
        self.collect_rule_list(rules);
        self.conditional_path = parent_conditional_path;
    }

    fn push_declaration_block(
        &mut self,
        declarations: &'walk DeclarationBlock<'ast, 'ghost>,
        declaration_ref: Ref<'ast, 'ghost, DeclarationBlock<'ast, 'ghost>>,
        kind: DeclarationBlockKind,
        location: StructuralLocation,
    ) {
        let current_entry = u32::try_from(self.declaration_blocks.len())
            .expect("declaration block count exceeds u32::MAX");
        let occurrence = self.effective_keys.intern(
            self.selector_path,
            self.conditional_path,
            &self.selector_paths,
            &self.conditional_paths,
            current_entry,
        );
        self.declaration_blocks.push(DeclarationBlockEntry {
            declarations,
            declaration_ref,
            effective_key: occurrence.id,
            kind,
            rule_list: location.rule_list,
            rule_list_segment: location.rule_list_segment,
            sibling_ordinal: location.sibling_ordinal,
            next_in_history: None,
            starts_history: false,
        });

        if let Some(previous_entry) = occurrence.previous_entry {
            let previous_entry =
                usize::try_from(previous_entry).expect("declaration block index fits usize");
            let current_link = NonZeroU32::new(
                current_entry
                    .checked_add(1)
                    .expect("declaration block count exceeds u32::MAX"),
            )
            .expect("encoded declaration block index is non-zero");
            self.declaration_blocks[previous_entry].next_in_history = Some(current_link);
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

fn ends_rule_list_segment(rule: &CssRule<'_, '_>) -> bool {
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
    use rocketcss_allocator::Allocator;
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

            let blocks = walk_declaration_blocks(&stylesheet, &token, &allocator);
            assert_eq!(blocks.len(), 5);
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

            let blocks = walk_declaration_blocks(&stylesheet, &token, &allocator);
            assert_eq!(blocks.len(), 5);
            assert!(blocks[0].starts_declaration_history());
            assert_eq!(blocks[0].next_declaration_history_entry(), Some(2));
            assert_eq!(blocks[2].next_declaration_history_entry(), Some(4));
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

            let blocks = walk_declaration_blocks(&stylesheet, &token, &allocator);
            assert_eq!(blocks.len(), 6);
            assert!(blocks[0].is_direct_sibling_of(&blocks[1]));
            assert!(blocks[2].is_direct_sibling_of(&blocks[3]));
            assert_ne!(blocks[1].rule_list, blocks[2].rule_list);
            assert_ne!(blocks[3].rule_list, blocks[4].rule_list);
            assert_eq!(blocks[4].rule_list, blocks[5].rule_list);
            assert_ne!(blocks[4].rule_list_segment, blocks[5].rule_list_segment);
            assert!(!blocks[4].is_direct_sibling_of(&blocks[5]));
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

            let blocks = walk_declaration_blocks(&stylesheet, &token, &allocator);
            assert_eq!(blocks.len(), 3);
            assert_eq!(blocks[0].effective_key, blocks[2].effective_key);
            assert_ne!(blocks[0].rule_list, blocks[2].rule_list);
            assert!(blocks[1].is_direct_sibling_of(&blocks[2]));
            assert_ne!(
                std::ptr::from_ref(blocks[0].declarations),
                std::ptr::from_ref(blocks[2].declarations)
            );
        });
    }
}
