use rocketcss_allocator::{GhostToken, vec::Vec};
use rocketcss_ast::{
    ContainerCondition, CssRule, DeclarationBlock, MediaList, SelectorList, StyleRule, StyleSheet,
    SupportsCondition, VendorPrefix,
};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpaqueConditionalKind {
    Layer,
    MozDocument,
    Scope,
    StartingStyle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SelectorFrameKind {
    Style,
    Nesting,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SelectorFrame<'walk, 'ast> {
    kind: SelectorFrameKind,
    selectors: &'walk SelectorList<'ast>,
    vendor_prefix: VendorPrefix,
}

/// The declaration-history identity known during source-ordered discovery.
///
/// Selector frames are kept as a path because nested-selector resolution is a
/// separate feature. Typed conditional frames use structural equality. At-rule
/// kinds whose semantics are not handled by this pass are isolated by authored
/// wrapper identity.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EffectiveKey<'walk, 'ast> {
    selectors: std::vec::Vec<SelectorFrame<'walk, 'ast>>,
    conditions: std::vec::Vec<ConditionalFrame<'walk, 'ast>>,
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

#[derive(Debug)]
pub(crate) struct DeclarationBlockEntry<'walk, 'ast, 'ghost> {
    pub(crate) declarations: &'walk DeclarationBlock<'ast, 'ghost>,
    pub(crate) effective_key: EffectiveKey<'walk, 'ast>,
    pub(crate) rule_list: RuleListId,
    pub(crate) rule_list_segment: RuleListSegmentId,
    pub(crate) sibling_ordinal: SiblingOrdinal,
}

impl DeclarationBlockEntry<'_, '_, '_> {
    pub(crate) fn is_direct_sibling_of(&self, right: &Self) -> bool {
        self.rule_list == right.rule_list
            && self.rule_list_segment == right.rule_list_segment
            && self.sibling_ordinal.0.checked_add(1) == Some(right.sibling_ordinal.0)
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
) -> std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>> {
    let mut walker = DeclarationBlockWalker::new(token);
    walker.collect_rule_list(&stylesheet.rules);
    walker.declaration_blocks
}

struct DeclarationBlockWalker<'walk, 'ast, 'ghost> {
    token: &'walk GhostToken<'ghost>,
    selectors: std::vec::Vec<SelectorFrame<'walk, 'ast>>,
    conditions: std::vec::Vec<ConditionalFrame<'walk, 'ast>>,
    declaration_blocks: std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
    state: WalkState,
}

impl<'walk, 'ast, 'ghost> DeclarationBlockWalker<'walk, 'ast, 'ghost> {
    fn new(token: &'walk GhostToken<'ghost>) -> Self {
        Self {
            token,
            selectors: std::vec::Vec::new(),
            conditions: std::vec::Vec::new(),
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
                self.declaration_blocks.push(DeclarationBlockEntry {
                    declarations: rule.declarations.as_ref().borrow(self.token).get_ref(),
                    effective_key: self.effective_key(),
                    rule_list: location.rule_list,
                    rule_list_segment: location.rule_list_segment,
                    sibling_ordinal: location.sibling_ordinal,
                });
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
        self.selectors.push(SelectorFrame {
            kind,
            selectors: &rule.selectors,
            vendor_prefix: rule.vendor_prefix,
        });
        self.declaration_blocks.push(DeclarationBlockEntry {
            declarations: rule.declarations.as_ref().borrow(self.token).get_ref(),
            effective_key: self.effective_key(),
            rule_list: location.rule_list,
            rule_list_segment: location.rule_list_segment,
            sibling_ordinal: location.sibling_ordinal,
        });
        self.collect_rule_list(&rule.rules);
        self.selectors.pop();
    }

    fn with_condition(
        &mut self,
        frame: ConditionalFrame<'walk, 'ast>,
        rules: &'walk Vec<'ast, CssRule<'ast, 'ghost>>,
    ) {
        self.conditions.push(frame);
        self.collect_rule_list(rules);
        self.conditions.pop();
    }

    fn effective_key(&self) -> EffectiveKey<'walk, 'ast> {
        EffectiveKey {
            selectors: self.selectors.clone(),
            conditions: self.conditions.clone(),
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

            let blocks = walk_declaration_blocks(&stylesheet, &token);
            assert_eq!(blocks.len(), 5);
            assert_ne!(blocks[0].effective_key, blocks[1].effective_key);
            assert_eq!(blocks[1].effective_key, blocks[2].effective_key);
            assert_ne!(blocks[2].effective_key, blocks[3].effective_key);
            assert_ne!(blocks[2].effective_key, blocks[4].effective_key);
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

            let blocks = walk_declaration_blocks(&stylesheet, &token);
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

            let blocks = walk_declaration_blocks(&stylesheet, &token);
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
