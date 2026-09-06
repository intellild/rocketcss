use rocketcss_visitor::prelude::*;

use rocketcss_ast::{
    AstContext, CompilationVisitMutContext, CompilationVisitor, CompilationVisitorMut,
    ConcreteDeclarationBlockId as DeclarationBlockId, ConcreteRuleId as RuleId, CssRulePayload,
    DeclarationBlockOwner, DeclarationId, DeclarationPayload, DeclarationRecord, RuleRecord,
    SelectorValueId,
};

fn parse_test_compilation<'a>(allocator: &'a Allocator, source: &'a str) -> AstContext<'a> {
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

impl<'a> CompilationVisitor<'a> for StructuralRecorder {
    fn visit_rule(
        &mut self,
        _id: RuleId<'a>,
        _rule: &RuleRecord<'a, CssRulePayload<'a>>,
        _compilation: &AstContext<'a>,
    ) {
        self.events.push(StructuralEvent::Rule);
    }

    fn visit_declaration_block(
        &mut self,
        _id: DeclarationBlockId<'a>,
        _block: &rocketcss_ast::DeclarationBlockRecord<CssRulePayload<'a>>,
        _compilation: &AstContext<'a>,
    ) {
        self.events.push(StructuralEvent::DeclarationBlock);
    }

    fn visit_declaration(
        &mut self,
        _block: DeclarationBlockId<'a>,
        _id: DeclarationId<'a>,
        _declaration: &DeclarationRecord<'a, DeclarationPayload<'a>>,
        _compilation: &AstContext<'a>,
    ) {
        self.events.push(StructuralEvent::Declaration);
    }

    fn visit_descriptor(
        &mut self,
        _block: DeclarationBlockId<'a>,
        _id: DeclarationId<'a>,
        _descriptor: &DeclarationRecord<'a, DeclarationPayload<'a>>,
        _compilation: &AstContext<'a>,
    ) {
        self.events.push(StructuralEvent::Descriptor);
    }
}

#[test]
fn ast_traversal_uses_lexical_rule_and_declaration_order() {
    let allocator = Allocator::new();
    let compilation = parse_test_compilation(
        &allocator,
        "a{color:red;@media print{b{width:1px}}height:2px}@font-face{font-family:x;src:url(x)}",
    );
    let mut recorder = StructuralRecorder::default();

    compilation.visit_compilation(&mut recorder).unwrap();

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

struct ContextRewrite<'a> {
    first_rule: RuleId<'a>,
    replacement_selector: SelectorValueId<'a>,
    selector_replaced: bool,
    declaration_replaced: bool,
}

impl<'a> CompilationVisitorMut<'a> for ContextRewrite<'a> {
    fn visit_rule(&mut self, id: RuleId<'a>, cx: &mut CompilationVisitMutContext<'_, 'a>) {
        if id == self.first_rule {
            self.selector_replaced = cx
                .replace_rule_selector_value(id, self.replacement_selector)
                .unwrap();
        }
    }

    fn visit_declaration(
        &mut self,
        block: DeclarationBlockId<'a>,
        id: DeclarationId<'a>,
        cx: &mut CompilationVisitMutContext<'_, 'a>,
    ) {
        let owner = cx.compilation().declaration_block(block).unwrap().owner();
        if owner == DeclarationBlockOwner::Rule(self.first_rule) {
            cx.replace_property_declaration(block, id, Declaration::Tombstone)
                .unwrap();
            self.declaration_replaced = true;
        }
    }
}

#[test]
fn ast_mutable_traversal_uses_selector_and_declaration_transactions() {
    let allocator = Allocator::new();
    let mut compilation = parse_test_compilation(&allocator, ".before{color:red}.after{width:1px}");
    let rules = compilation
        .rules_in_list(compilation.stylesheet().root_rules())
        .unwrap()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();
    let [first_rule, second_rule] = rules.as_slice() else {
        panic!("expected two style rules")
    };
    let CssRulePayload::Style(second_payload) = compilation.rule(*second_rule).unwrap().payload()
    else {
        panic!("expected a style rule")
    };
    let replacement_selector = second_payload.selector_value;
    let mut visitor = ContextRewrite {
        first_rule: *first_rule,
        replacement_selector,
        selector_replaced: false,
        declaration_replaced: false,
    };

    compilation.visit_compilation_mut(&mut visitor).unwrap();

    assert!(visitor.selector_replaced);
    assert!(visitor.declaration_replaced);
    let first_block = compilation
        .rule(*first_rule)
        .unwrap()
        .declaration_block()
        .unwrap();
    let second_block = compilation
        .rule(*second_rule)
        .unwrap()
        .declaration_block()
        .unwrap();
    assert_eq!(
        compilation.declaration_block(first_block).unwrap().owner(),
        DeclarationBlockOwner::Rule(*first_rule)
    );
    assert_eq!(
        compilation
            .declaration_block(first_block)
            .unwrap()
            .effective_key(),
        compilation
            .declaration_block(second_block)
            .unwrap()
            .effective_key()
    );
    assert!(matches!(
        compilation
            .declarations_in_block(first_block)
            .unwrap()
            .next()
            .unwrap()
            .payload(),
        DeclarationPayload::Property(Declaration::Tombstone)
    ));
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[derive(Default)]
struct PanicOnNestedStyle {
    styles_seen: usize,
}

impl<'ast> CompilationVisitorMut<'ast> for PanicOnNestedStyle {
    fn visit_rule(&mut self, id: RuleId<'ast>, cx: &mut CompilationVisitMutContext<'_, 'ast>) {
        if matches!(
            cx.compilation().rule(id).unwrap().payload(),
            CssRulePayload::Style(_)
        ) {
            self.styles_seen += 1;
            assert_ne!(self.styles_seen, 2, "nested style panic");
        }
    }
}

struct NoopVisitor;

impl CompilationVisitorMut<'_> for NoopVisitor {}

#[test]
fn mutable_visitor_panic_keeps_nested_rules_attached() {
    let allocator = Allocator::new();
    let mut compilation =
        parse_test_compilation(&allocator, ".card{color:red;button:hover{color:blue}}");
    let outer = compilation
        .rules_in_list(compilation.stylesheet().root_rules())
        .unwrap()
        .next()
        .unwrap()
        .0;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        compilation
            .visit_compilation_mut(&mut PanicOnNestedStyle::default())
            .unwrap();
    }));
    assert!(result.is_err());

    compilation.visit_compilation_mut(&mut NoopVisitor).unwrap();
    let child_list = compilation.rule(outer).unwrap().child_list().unwrap();
    assert_eq!(compilation.rules_in_list(child_list).unwrap().count(), 1);
    assert_eq!(compilation.validate_ast(), Ok(()));
}
