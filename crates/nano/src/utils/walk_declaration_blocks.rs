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

#[derive(Debug)]
pub(crate) struct DeclarationBlockEntry<'walk, 'ast, 'ghost> {
    pub(crate) declarations: &'walk DeclarationBlock<'ast, 'ghost>,
    pub(crate) effective_key: EffectiveKey<'walk, 'ast>,
}

pub(crate) fn walk_declaration_blocks<'walk, 'ast, 'ghost>(
    stylesheet: &'walk StyleSheet<'ast, 'ghost>,
    token: &'walk GhostToken<'ghost>,
) -> std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>> {
    let mut declaration_blocks = std::vec::Vec::new();
    let mut selectors = std::vec::Vec::new();
    let mut conditions = std::vec::Vec::new();
    collect_rule_list(
        &stylesheet.rules,
        token,
        &mut selectors,
        &mut conditions,
        &mut declaration_blocks,
    );
    declaration_blocks
}

fn collect_rule_list<'walk, 'ast, 'ghost>(
    rules: &'walk Vec<'ast, CssRule<'ast, 'ghost>>,
    token: &'walk GhostToken<'ghost>,
    selectors: &mut std::vec::Vec<SelectorFrame<'walk, 'ast>>,
    conditions: &mut std::vec::Vec<ConditionalFrame<'walk, 'ast>>,
    declaration_blocks: &mut std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
) {
    declaration_blocks.reserve(rules.len());
    for rule in rules {
        collect_rule(rule, token, selectors, conditions, declaration_blocks);
    }
}

fn collect_rule<'walk, 'ast, 'ghost>(
    rule: &'walk CssRule<'ast, 'ghost>,
    token: &'walk GhostToken<'ghost>,
    selectors: &mut std::vec::Vec<SelectorFrame<'walk, 'ast>>,
    conditions: &mut std::vec::Vec<ConditionalFrame<'walk, 'ast>>,
    declaration_blocks: &mut std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
) {
    match rule {
        CssRule::Media(rule) => with_condition(
            ConditionalFrame::Media(&rule.query),
            &rule.rules,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        CssRule::Style(rule) => collect_style_rule(
            rule.as_ref().get_ref(),
            SelectorFrameKind::Style,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        CssRule::Supports(rule) => with_condition(
            ConditionalFrame::Supports(&rule.condition),
            &rule.rules,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        CssRule::MozDocument(rule) => with_condition(
            opaque_condition(OpaqueConditionalKind::MozDocument, rule.as_ref()),
            &rule.rules,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        CssRule::Nesting(rule) => collect_style_rule(
            rule.style.as_ref().get_ref(),
            SelectorFrameKind::Nesting,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        CssRule::NestedDeclarations(rule) => {
            declaration_blocks.push(DeclarationBlockEntry {
                declarations: rule.declarations.as_ref().borrow(token).get_ref(),
                effective_key: EffectiveKey {
                    selectors: selectors.clone(),
                    conditions: conditions.clone(),
                },
            });
        }
        CssRule::LayerBlock(rule) => with_condition(
            opaque_condition(OpaqueConditionalKind::Layer, rule.as_ref()),
            &rule.rules,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        CssRule::Container(rule) => with_condition(
            ConditionalFrame::Container {
                name: rule.name,
                condition: rule.condition.as_deref(),
            },
            &rule.rules,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        CssRule::Scope(rule) => with_condition(
            opaque_condition(OpaqueConditionalKind::Scope, rule.as_ref()),
            &rule.rules,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        CssRule::StartingStyle(rule) => with_condition(
            opaque_condition(OpaqueConditionalKind::StartingStyle, rule.as_ref()),
            &rule.rules,
            token,
            selectors,
            conditions,
            declaration_blocks,
        ),
        _ => {}
    }
}

fn collect_style_rule<'walk, 'ast, 'ghost>(
    rule: &'walk StyleRule<'ast, 'ghost>,
    kind: SelectorFrameKind,
    token: &'walk GhostToken<'ghost>,
    selectors: &mut std::vec::Vec<SelectorFrame<'walk, 'ast>>,
    conditions: &mut std::vec::Vec<ConditionalFrame<'walk, 'ast>>,
    declaration_blocks: &mut std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
) {
    selectors.push(SelectorFrame {
        kind,
        selectors: &rule.selectors,
        vendor_prefix: rule.vendor_prefix,
    });
    declaration_blocks.push(DeclarationBlockEntry {
        declarations: rule.declarations.as_ref().borrow(token).get_ref(),
        effective_key: EffectiveKey {
            selectors: selectors.clone(),
            conditions: conditions.clone(),
        },
    });
    collect_rule_list(
        &rule.rules,
        token,
        selectors,
        conditions,
        declaration_blocks,
    );
    selectors.pop();
}

fn with_condition<'walk, 'ast, 'ghost>(
    frame: ConditionalFrame<'walk, 'ast>,
    rules: &'walk Vec<'ast, CssRule<'ast, 'ghost>>,
    token: &'walk GhostToken<'ghost>,
    selectors: &mut std::vec::Vec<SelectorFrame<'walk, 'ast>>,
    conditions: &mut std::vec::Vec<ConditionalFrame<'walk, 'ast>>,
    declaration_blocks: &mut std::vec::Vec<DeclarationBlockEntry<'walk, 'ast, 'ghost>>,
) {
    conditions.push(frame);
    collect_rule_list(rules, token, selectors, conditions, declaration_blocks);
    conditions.pop();
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
            assert_ne!(
                std::ptr::from_ref(blocks[0].declarations),
                std::ptr::from_ref(blocks[2].declarations)
            );
        });
    }
}
