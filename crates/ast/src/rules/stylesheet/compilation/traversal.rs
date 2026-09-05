use super::*;

/// Storage-neutral callbacks over the semantic AST.
///
/// Traversal owns ordering. Callbacks receive typed IDs and may resolve any
/// additional immutable data through `compilation` without depending on store
/// layout or retaining references across a later mutation.
pub trait CompilationVisitor<'ast> {
    fn visit_rule(
        &mut self,
        _id: ConcreteRuleId<'ast>,
        _rule: &RuleRecord<'ast, CssRulePayload<'ast>>,
        _compilation: &Compilation<'ast>,
    ) {
    }

    fn visit_declaration_block(
        &mut self,
        _id: ConcreteDeclarationBlockId<'ast>,
        _block: &DeclarationBlockRecord<'ast, CssRulePayload<'ast>>,
        _compilation: &Compilation<'ast>,
    ) {
    }

    fn visit_declaration(
        &mut self,
        _block: ConcreteDeclarationBlockId<'ast>,
        _id: DeclarationId<'ast>,
        _declaration: &DeclarationRecord<'ast, DeclarationPayload<'ast>>,
        _compilation: &Compilation<'ast>,
    ) {
    }

    fn visit_descriptor(
        &mut self,
        _block: ConcreteDeclarationBlockId<'ast>,
        _id: DeclarationId<'ast>,
        _descriptor: &DeclarationRecord<'ast, DeclarationPayload<'ast>>,
        _compilation: &Compilation<'ast>,
    ) {
    }
}

/// Mutable AST traversal callbacks.
///
/// The context deliberately exposes semantic replacement transactions rather
/// than `&mut` store entries. A callback therefore cannot retain a reference
/// that an insertion or interner update would invalidate.
pub trait CompilationVisitorMut<'ast> {
    fn visit_selector_value(
        &mut self,
        _id: SelectorValueId<'ast>,
        _selectors: &mut crate::SelectorList<'ast>,
        _compilation: &mut Compilation<'ast>,
    ) {
    }

    fn visit_rule(
        &mut self,
        _id: ConcreteRuleId<'ast>,
        _cx: &mut CompilationVisitMutContext<'_, 'ast>,
    ) {
    }

    fn visit_declaration_block(
        &mut self,
        _id: ConcreteDeclarationBlockId<'ast>,
        _cx: &mut CompilationVisitMutContext<'_, 'ast>,
    ) {
    }

    fn visit_declaration(
        &mut self,
        _block: ConcreteDeclarationBlockId<'ast>,
        _id: DeclarationId<'ast>,
        _cx: &mut CompilationVisitMutContext<'_, 'ast>,
    ) {
    }

    fn visit_descriptor(
        &mut self,
        _block: ConcreteDeclarationBlockId<'ast>,
        _id: DeclarationId<'ast>,
        _cx: &mut CompilationVisitMutContext<'_, 'ast>,
    ) {
    }
}

/// Mutation access available during [`CompilationVisitorMut`] traversal.
pub struct CompilationVisitMutContext<'comp, 'ast> {
    compilation: &'comp mut Compilation<'ast>,
}

impl<'comp, 'ast> CompilationVisitMutContext<'comp, 'ast> {
    #[inline]
    pub fn compilation(&self) -> &Compilation<'ast> {
        self.compilation
    }

    #[inline]
    pub fn replace_rule_selector_value(
        &mut self,
        rule: ConcreteRuleId<'ast>,
        selector: SelectorValueId<'ast>,
    ) -> Result<bool, ConcreteMutationError<'ast>> {
        self.compilation.replace_rule_selector_value(rule, selector)
    }

    #[inline]
    pub fn replace_property_declaration(
        &mut self,
        block: ConcreteDeclarationBlockId<'ast>,
        declaration: DeclarationId<'ast>,
        replacement: crate::Declaration<'ast>,
    ) -> Result<crate::Declaration<'ast>, ConcreteMutationError<'ast>> {
        if !matches!(
            self.compilation
                .declaration(declaration)
                .map(DeclarationRecord::payload),
            Some(DeclarationPayload::Property(_))
        ) {
            return Err(ConcreteMutationError::UnknownDeclaration(declaration));
        }
        let previous = self.compilation.replace_declaration(
            block,
            declaration,
            DeclarationPayload::Property(replacement),
        )?;
        let DeclarationPayload::Property(previous) = previous else {
            unreachable!("the declaration kind was validated before replacement")
        };
        Ok(previous)
    }

    #[inline]
    pub fn intern_selector_value(
        &mut self,
        selectors: crate::SelectorList<'ast>,
        kind: SelectorFrameKind,
        vendor_prefix: crate::VendorPrefix,
    ) -> Result<SelectorValueId<'ast>, ConcreteMutationError<'ast>> {
        self.compilation
            .intern_selector_value(selectors, kind, vendor_prefix)
    }
}

impl<'ast> Compilation<'ast> {
    /// Visits live rules in lexical preorder, followed by each rule's owned
    /// declaration block and ordered declaration occurrences.
    pub fn visit_compilation<V: ?Sized + CompilationVisitor<'ast>>(
        &self,
        visitor: &mut V,
    ) -> Result<(), ConcreteMutationError<'ast>> {
        for (rule_id, rule) in self.rules_in_source_order() {
            if !rule.is_live() {
                continue;
            }
            visitor.visit_rule(rule_id, rule, self);
            let Some(block_id) = rule.declaration_block() else {
                continue;
            };
            let block = self
                .declaration_block(block_id)
                .ok_or(ConcreteMutationError::UnknownDeclarationBlock(block_id))?;
            if !block.is_live() {
                return Err(ConcreteMutationError::InvalidRuleTopology(rule_id));
            }
            let property_block = rule.payload().owns_property_declarations();
            if property_block {
                visitor.visit_declaration_block(block_id, block, self);
            }
            for (occurrence, declaration) in self.declaration_occurrences_in_block(block_id)? {
                let declaration_id = occurrence.declaration();
                if property_block {
                    visitor.visit_declaration(block_id, declaration_id, declaration, self);
                } else {
                    visitor.visit_descriptor(block_id, declaration_id, declaration, self);
                }
            }
        }
        Ok(())
    }

    /// Mutably visits the same semantic sequence as [`Self::visit_compilation`].
    ///
    /// Only selector and declaration replacement transactions are exposed to
    /// callbacks. Structural mutations stay scheduler-owned, so the captured
    /// source successor and declaration identities remain valid for the callback.
    pub fn visit_compilation_mut<V: ?Sized + CompilationVisitorMut<'ast>>(
        &mut self,
        visitor: &mut V,
    ) -> Result<(), ConcreteMutationError<'ast>> {
        let allocator = self.allocator;
        self.transform_selector_values_in(allocator, |id, selectors, compilation| {
            visitor.visit_selector_value(id, selectors, compilation);
        });
        let mut current = self.first_rule_in_source;
        while let Some(rule_id) = current {
            let rule = self
                .rule(rule_id)
                .ok_or(ConcreteMutationError::UnknownRule(rule_id))?;
            current = rule.next_in_source();
            if !rule.is_live() {
                continue;
            }

            visitor.visit_rule(
                rule_id,
                &mut CompilationVisitMutContext { compilation: self },
            );
            let rule = self
                .rule(rule_id)
                .ok_or(ConcreteMutationError::UnknownRule(rule_id))?;
            let property_block = rule.payload().owns_property_declarations();
            let Some(block_id) = rule.declaration_block() else {
                continue;
            };
            let block = self
                .declaration_block(block_id)
                .ok_or(ConcreteMutationError::UnknownDeclarationBlock(block_id))?;
            if !block.is_live() {
                return Err(ConcreteMutationError::InvalidRuleTopology(rule_id));
            }
            if property_block {
                visitor.visit_declaration_block(
                    block_id,
                    &mut CompilationVisitMutContext { compilation: self },
                );
            }
            let mut declaration_cursor = self.declaration_cursor(block_id)?;
            while let Some(declaration_id) = self.next_declaration_id(&mut declaration_cursor)? {
                if property_block {
                    visitor.visit_declaration(
                        block_id,
                        declaration_id,
                        &mut CompilationVisitMutContext { compilation: self },
                    );
                } else {
                    visitor.visit_descriptor(
                        block_id,
                        declaration_id,
                        &mut CompilationVisitMutContext { compilation: self },
                    );
                }
            }
        }
        Ok(())
    }
}
