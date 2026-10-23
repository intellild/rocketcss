use rocketcss_visitor::prelude::*;

use rocketcss_ast_next::{
    Compilation, CssRule, CssRulePayloadData, DeclarationData, DeclarationEntry,
    DeclarationPayloadData, Selector, StyleRuleData,
};

fn parse_test_compilation<'a>(allocator: &'a Allocator, source: &'a str) -> Compilation<'a> {
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

impl<'ast> Visitor<'ast> for StructuralRecorder {
    fn visit_rule(&mut self, _rule: CssRule<'ast>, _ast: &Ast<'ast>) {
        self.events.push(StructuralEvent::Rule);
    }

    fn visit_declaration_block(
        &mut self,
        _block: rocketcss_ast_next::DeclarationBlock<'ast>,
        _ast: &Ast<'ast>,
    ) {
        self.events.push(StructuralEvent::DeclarationBlock);
    }

    fn visit_declaration(&mut self, _entry: DeclarationEntry<'ast>, _ast: &Ast<'ast>) {
        self.events.push(StructuralEvent::Declaration);
    }

    fn visit_descriptor(&mut self, _entry: DeclarationEntry<'ast>, _ast: &Ast<'ast>) {
        self.events.push(StructuralEvent::Descriptor);
    }
}

#[test]
fn traversal_uses_lexical_rule_and_declaration_order() {
    let allocator = Allocator::new();
    let compilation = parse_test_compilation(
        &allocator,
        "a{color:red;@media print{b{width:1px}}height:2px}@font-face{font-family:x;src:url(x)}",
    );
    let mut recorder = StructuralRecorder::default();

    visit_compilation(&compilation, &mut recorder);

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

struct SelectorRewrite<'ast> {
    first_rule: CssRule<'ast>,
    replacement: &'ast [Selector<'ast>],
    selector_replaced: bool,
    declaration_replaced: bool,
}

impl<'ast> VisitorMut<'ast> for SelectorRewrite<'ast> {
    fn visit_rule(&mut self, rule: CssRule<'ast>, cx: &mut VisitMutContext<'_, 'ast>) {
        if rule == self.first_rule {
            let ast = cx.ast();
            let CssRulePayloadData::Style(style) = rule.payload(ast).data(ast) else {
                return;
            };
            let vendor_prefix = style.vendor_prefix(ast);
            cx.ast_mut().set_data(
                style,
                StyleRuleData {
                    selectors: self.replacement,
                    vendor_prefix,
                },
            );
            self.selector_replaced = true;
        }
    }

    fn visit_declaration(
        &mut self,
        entry: DeclarationEntry<'ast>,
        cx: &mut VisitMutContext<'_, 'ast>,
    ) {
        let ast = cx.ast();
        let Some(declaration) = entry.payload.as_property(ast) else {
            return;
        };
        let Some(block) = self.first_rule.declarations(ast) else {
            return;
        };
        if block.declarations(ast).contains(&entry.payload) {
            cx.ast_mut()
                .set_data(declaration, DeclarationData::Tombstone);
            self.declaration_replaced = true;
        }
    }
}

#[test]
fn mutable_traversal_uses_selector_and_declaration_transactions() {
    let allocator = Allocator::new();
    let mut compilation = parse_test_compilation(&allocator, ".before{color:red}.after{width:1px}");
    let rules = compilation.style_sheet().rules(compilation.ast()).to_vec();
    let [first_rule, second_rule] = rules.as_slice() else {
        panic!("expected two style rules")
    };
    let ast = compilation.ast();
    let CssRulePayloadData::Style(second_payload) = second_rule.payload(ast).data(ast) else {
        panic!("expected a style rule")
    };
    let replacement = second_payload.selectors(ast);
    let mut visitor = SelectorRewrite {
        first_rule: *first_rule,
        replacement,
        selector_replaced: false,
        declaration_replaced: false,
    };

    visit_compilation_mut(&mut compilation, &mut visitor);

    assert!(visitor.selector_replaced);
    assert!(visitor.declaration_replaced);
    let ast = compilation.ast();
    let first_block = first_rule.declarations(ast).unwrap();
    assert!(matches!(
        first_block.declarations(ast)[0].data(ast),
        DeclarationPayloadData::Property(declaration)
            if matches!(declaration.data(ast), DeclarationData::Tombstone)
    ));
}

#[derive(Default)]
struct PanicOnNestedStyle {
    styles_seen: usize,
}

impl<'ast> VisitorMut<'ast> for PanicOnNestedStyle {
    fn visit_rule(&mut self, rule: CssRule<'ast>, cx: &mut VisitMutContext<'_, 'ast>) {
        if matches!(
            rule.payload(cx.ast()).data(cx.ast()),
            CssRulePayloadData::Style(_)
        ) {
            self.styles_seen += 1;
            assert_ne!(self.styles_seen, 2, "nested style panic");
        }
    }
}

struct NoopVisitor;

impl VisitorMut<'_> for NoopVisitor {}

#[test]
fn mutable_visitor_panic_keeps_nested_rules_attached() {
    let allocator = Allocator::new();
    let mut compilation =
        parse_test_compilation(&allocator, ".card{color:red;button:hover{color:blue}}");
    let outer = compilation.style_sheet().rules(compilation.ast())[0];

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        visit_compilation_mut(&mut compilation, &mut PanicOnNestedStyle::default());
    }));
    assert!(result.is_err());

    visit_compilation_mut(&mut compilation, &mut NoopVisitor);
    assert_eq!(outer.children(compilation.ast()).len(), 1);
}
