use rocketcss_visitor::prelude::*;

use rocketcss_ast::{
    CssDeclaration, CssDeclarationBlockId as DeclarationBlockId, CssRule, CssRuleId as RuleId,
    DeclarationBlockOwner, DeclarationId, DeclarationRecord, RuleRecord, SelectorValueId,
    StyleSheet, StyleSheetVisitMutContext, StyleSheetVisitor, StyleSheetVisitorMut,
};

fn parse_test_stylesheet<'a>(allocator: &'a Allocator, source: &'a str) -> StyleSheet<'a> {
    GhostToken::scope(|mut token| {
        rocketcss_parser::Compiler::new(allocator)
            .parse(
                source,
                &mut token,
                rocketcss_parser::ParserOptions::default(),
            )
            .unwrap()
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StructuralEvent {
    Rule,
    DeclarationBlock,
    Declaration,
    Descriptor,
}

#[derive(Default)]
struct StructuralRecorder {
    events: std::vec::Vec<StructuralEvent>,
}

impl<'a> StyleSheetVisitor<'a> for StructuralRecorder {
    fn visit_rule(
        &mut self,
        _id: RuleId<'a>,
        _rule: &RuleRecord<CssRule<'a>>,
        _stylesheet: &StyleSheet<'a>,
    ) {
        self.events.push(StructuralEvent::Rule);
    }

    fn visit_declaration_block(
        &mut self,
        _id: DeclarationBlockId<'a>,
        _block: &rocketcss_ast::DeclarationBlock<CssRule<'a>>,
        _stylesheet: &StyleSheet<'a>,
    ) {
        self.events.push(StructuralEvent::DeclarationBlock);
    }

    fn visit_declaration(
        &mut self,
        _block: DeclarationBlockId<'a>,
        _id: DeclarationId,
        _declaration: &DeclarationRecord<CssDeclaration<'a>>,
        _stylesheet: &StyleSheet<'a>,
    ) {
        self.events.push(StructuralEvent::Declaration);
    }

    fn visit_descriptor(
        &mut self,
        _block: DeclarationBlockId<'a>,
        _id: DeclarationId,
        _descriptor: &DeclarationRecord<CssDeclaration<'a>>,
        _stylesheet: &StyleSheet<'a>,
    ) {
        self.events.push(StructuralEvent::Descriptor);
    }
}

#[test]
fn radix_traversal_uses_lexical_rule_and_declaration_order() {
    let allocator = Allocator::new();
    let stylesheet = parse_test_stylesheet(
        &allocator,
        "a{color:red;@media print{b{width:1px}}height:2px}@font-face{font-family:x;src:url(x)}",
    );
    let mut recorder = StructuralRecorder::default();

    stylesheet.visit_stylesheet(&mut recorder).unwrap();

    assert_eq!(
        recorder.events,
        [
            StructuralEvent::Rule,
            StructuralEvent::DeclarationBlock,
            StructuralEvent::Declaration,
            StructuralEvent::Rule,
            StructuralEvent::Rule,
            StructuralEvent::DeclarationBlock,
            StructuralEvent::Declaration,
            StructuralEvent::Rule,
            StructuralEvent::DeclarationBlock,
            StructuralEvent::Declaration,
            StructuralEvent::Rule,
            StructuralEvent::Descriptor,
            StructuralEvent::Descriptor,
        ]
    );
}

struct RadixRewrite<'a> {
    first_rule: RuleId<'a>,
    replacement_selector: SelectorValueId,
    selector_replaced: bool,
    declaration_replaced: bool,
}

impl<'a> StyleSheetVisitorMut<'a> for RadixRewrite<'a> {
    fn visit_rule(&mut self, id: RuleId<'a>, cx: &mut StyleSheetVisitMutContext<'_, 'a>) {
        if id == self.first_rule {
            self.selector_replaced = cx
                .replace_rule_selector_value(id, self.replacement_selector)
                .unwrap();
        }
    }

    fn visit_declaration(
        &mut self,
        block: DeclarationBlockId<'a>,
        id: DeclarationId,
        cx: &mut StyleSheetVisitMutContext<'_, 'a>,
    ) {
        let owner = cx.stylesheet().declaration_block(block).unwrap().owner();
        if owner == DeclarationBlockOwner::Rule(self.first_rule) {
            cx.replace_property_declaration(block, id, Declaration::Tombstone)
                .unwrap();
            self.declaration_replaced = true;
        }
    }
}

#[test]
fn radix_mutable_traversal_uses_selector_and_declaration_transactions() {
    let allocator = Allocator::new();
    let mut stylesheet = parse_test_stylesheet(&allocator, ".before{color:red}.after{width:1px}");
    let rules = stylesheet
        .root_rules()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    let [first_rule, second_rule] = rules.as_slice() else {
        panic!("expected two style rules")
    };
    let CssRule::Style(second_payload) = stylesheet.rule(*second_rule).unwrap().payload() else {
        panic!("expected a style rule")
    };
    let replacement_selector = second_payload.selector_value;
    let mut visitor = RadixRewrite {
        first_rule: *first_rule,
        replacement_selector,
        selector_replaced: false,
        declaration_replaced: false,
    };

    stylesheet.visit_stylesheet_mut(&mut visitor).unwrap();

    assert!(visitor.selector_replaced);
    assert!(visitor.declaration_replaced);
    let first_block = stylesheet
        .rule(*first_rule)
        .unwrap()
        .declaration_block()
        .unwrap();
    let second_block = stylesheet
        .rule(*second_rule)
        .unwrap()
        .declaration_block()
        .unwrap();
    assert_eq!(
        stylesheet.declaration_block(first_block).unwrap().owner(),
        DeclarationBlockOwner::Rule(*first_rule)
    );
    assert_eq!(
        stylesheet
            .declaration_block(first_block)
            .unwrap()
            .effective_key(),
        stylesheet
            .declaration_block(second_block)
            .unwrap()
            .effective_key()
    );
    assert!(matches!(
        stylesheet
            .declarations_in_block(first_block)
            .unwrap()
            .next()
            .unwrap()
            .payload(),
        CssDeclaration::Property(Declaration::Tombstone)
    ));
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[derive(Default)]
struct PanicOnNestedStyle {
    styles_seen: usize,
}

impl<'ast> StyleSheetVisitorMut<'ast> for PanicOnNestedStyle {
    fn visit_rule(&mut self, id: RuleId<'ast>, cx: &mut StyleSheetVisitMutContext<'_, 'ast>) {
        if matches!(
            cx.stylesheet().rule(id).unwrap().payload(),
            CssRule::Style(_)
        ) {
            self.styles_seen += 1;
            assert_ne!(self.styles_seen, 2, "nested style panic");
        }
    }
}

struct NoopVisitor;

impl StyleSheetVisitorMut<'_> for NoopVisitor {}

#[test]
fn mutable_visitor_panic_keeps_nested_rules_attached() {
    let allocator = Allocator::new();
    let mut stylesheet =
        parse_test_stylesheet(&allocator, ".card{color:red;button:hover{color:blue}}");
    let outer = stylesheet.root_rules().next().unwrap().0;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        stylesheet
            .visit_stylesheet_mut(&mut PanicOnNestedStyle::default())
            .unwrap();
    }));
    assert!(result.is_err());

    stylesheet.visit_stylesheet_mut(&mut NoopVisitor).unwrap();
    assert_eq!(stylesheet.nested_rules(outer).unwrap().count(), 1);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}
