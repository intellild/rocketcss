//! Typed traversal over the flattened AST.

use rocketcss_ast_next::{
    Ast, Compilation, CssRule, CssRulePayloadData, DeclarationBlock, DeclarationEntry,
    DeclarationPayloadData,
};

/// Immutable callbacks invoked while traversing a compilation.
pub trait Visitor<'ast> {
    fn visit_rule(&mut self, _rule: CssRule<'ast>, _ast: &Ast<'ast>) {}
    fn visit_declaration_block(&mut self, _block: DeclarationBlock<'ast>, _ast: &Ast<'ast>) {}
    fn visit_declaration(&mut self, _entry: DeclarationEntry<'ast>, _ast: &Ast<'ast>) {}
    fn visit_descriptor(&mut self, _entry: DeclarationEntry<'ast>, _ast: &Ast<'ast>) {}
    fn visit_selector(&mut self, _selector: rocketcss_ast_next::Selector<'ast>, _ast: &Ast<'ast>) {}
}

/// Mutable callbacks invoked while traversing a compilation.
pub trait VisitorMut<'ast> {
    fn visit_rule(&mut self, _rule: CssRule<'ast>, _cx: &mut VisitMutContext<'_, 'ast>) {}
    fn visit_declaration(
        &mut self,
        _entry: DeclarationEntry<'ast>,
        _cx: &mut VisitMutContext<'_, 'ast>,
    ) {
    }
    fn visit_descriptor(
        &mut self,
        _entry: DeclarationEntry<'ast>,
        _cx: &mut VisitMutContext<'_, 'ast>,
    ) {
    }
    fn visit_selector(
        &mut self,
        _selector: rocketcss_ast_next::Selector<'ast>,
        _cx: &mut VisitMutContext<'_, 'ast>,
    ) {
    }
}

/// Mutable traversal state exposing the owning context.
pub struct VisitMutContext<'a, 'ast> {
    ast: &'a mut Ast<'ast>,
}

impl<'a, 'ast> VisitMutContext<'a, 'ast> {
    #[inline]
    pub fn new(ast: &'a mut Ast<'ast>) -> Self {
        Self { ast }
    }

    #[inline]
    pub fn ast(&self) -> &Ast<'ast> {
        self.ast
    }

    #[inline]
    pub fn ast_mut(&mut self) -> &mut Ast<'ast> {
        self.ast
    }
}

/// Visits every rule, declaration block, declaration, descriptor, and selector.
pub fn visit_compilation<'ast, V: Visitor<'ast>>(compilation: &Compilation<'ast>, visitor: &mut V) {
    visit_rules(
        compilation.ast(),
        compilation.style_sheet().rules(compilation.ast()),
        visitor,
    );
}

fn visit_rules<'ast, V: Visitor<'ast>>(ast: &Ast<'ast>, rules: &[CssRule<'ast>], visitor: &mut V) {
    for rule in rules {
        visitor.visit_rule(*rule, ast);
        let payload = rule.payload(ast).data(ast);
        match payload {
            CssRulePayloadData::Style(style) => {
                for selector in style.selectors(ast) {
                    visitor.visit_selector(*selector, ast);
                }
            }
            CssRulePayloadData::Nesting(nesting) => {
                for selector in nesting.selectors(ast) {
                    visitor.visit_selector(*selector, ast);
                }
            }
            _ => {}
        }

        if let Some(block) = rule.declarations(ast) {
            let entries = block
                .declarations(ast)
                .iter()
                .zip(block.important(ast))
                .map(|(payload, important)| DeclarationEntry {
                    payload: *payload,
                    important: *important,
                });
            if is_descriptor_family(payload) {
                for entry in entries {
                    visitor.visit_descriptor(entry, ast);
                }
            } else {
                visitor.visit_declaration_block(block, ast);
                for entry in entries {
                    if matches!(entry.payload.data(ast), DeclarationPayloadData::Property(_)) {
                        visitor.visit_declaration(entry, ast);
                    } else {
                        visitor.visit_descriptor(entry, ast);
                    }
                }
            }
        }

        visit_rules(ast, rule.children(ast), visitor);
    }
}

/// Visits every rule, declaration, descriptor, and selector with mutable access.
pub fn visit_compilation_mut<'ast, V: VisitorMut<'ast>>(
    compilation: &mut Compilation<'ast>,
    visitor: &mut V,
) {
    let rules = compilation.style_sheet().rules(compilation.ast()).to_vec();
    visit_rules_mut(compilation, &rules, visitor);
}

fn visit_rules_mut<'ast, V: VisitorMut<'ast>>(
    compilation: &mut Compilation<'ast>,
    rules: &[CssRule<'ast>],
    visitor: &mut V,
) {
    for rule in rules {
        let mut cx = VisitMutContext::new(compilation.ast_mut());
        visitor.visit_rule(*rule, &mut cx);

        let payload = rule.payload(cx.ast()).data(cx.ast());
        match payload {
            CssRulePayloadData::Style(style) => {
                for selector in style.selectors(cx.ast()).to_vec() {
                    visitor.visit_selector(selector, &mut cx);
                }
            }
            CssRulePayloadData::Nesting(nesting) => {
                for selector in nesting.selectors(cx.ast()).to_vec() {
                    visitor.visit_selector(selector, &mut cx);
                }
            }
            _ => {}
        }

        if let Some(block) = rule.declarations(cx.ast()) {
            let entries: Vec<_> = block
                .declarations(cx.ast())
                .iter()
                .zip(block.important(cx.ast()))
                .map(|(payload, important)| DeclarationEntry {
                    payload: *payload,
                    important: *important,
                })
                .collect();
            for entry in entries {
                if is_descriptor_family(payload) {
                    visitor.visit_descriptor(entry, &mut cx);
                } else if matches!(
                    entry.payload.data(cx.ast()),
                    DeclarationPayloadData::Property(_)
                ) {
                    visitor.visit_declaration(entry, &mut cx);
                } else {
                    visitor.visit_descriptor(entry, &mut cx);
                }
            }
        }

        let children = rule.children(cx.ast()).to_vec();
        visit_rules_mut(compilation, &children, visitor);
    }
}

fn is_descriptor_family(payload: CssRulePayloadData<'_>) -> bool {
    matches!(
        payload,
        CssRulePayloadData::FontFace(_)
            | CssRulePayloadData::FontPaletteValues(_)
            | CssRulePayloadData::ViewTransition(_)
            | CssRulePayloadData::Property(_)
            | CssRulePayloadData::FontFeatureSubrule(_)
    )
}
