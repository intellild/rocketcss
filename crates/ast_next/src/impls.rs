//! Hand-written behavior for generated tree nodes.

use crate::{
    Ast, CssRulePayload, CssRulePayloadKind, Declaration, DeclarationPayload, DeclarationPayloadKind,
};

impl<'ast> CssRulePayload<'ast> {
    /// Returns whether this rule owns ordinary CSS property declarations.
    #[inline]
    pub fn owns_property_declarations(self, ast: &Ast<'ast>) -> bool {
        matches!(
            self.kind(ast),
            CssRulePayloadKind::Style
                | CssRulePayloadKind::CounterStyle
                | CssRulePayloadKind::Viewport
                | CssRulePayloadKind::PositionTry
                | CssRulePayloadKind::Keyframe
                | CssRulePayloadKind::PageMargin
                | CssRulePayloadKind::PageDeclarations
                | CssRulePayloadKind::Nesting
                | CssRulePayloadKind::NestedDeclarations
        )
    }
}

impl<'ast> DeclarationPayload<'ast> {
    /// Returns the property declaration held by this payload, if any.
    #[inline]
    pub fn as_property(self, ast: &Ast<'ast>) -> Option<Declaration<'ast>> {
        match self.kind(ast) {
            DeclarationPayloadKind::Property => match self.data(ast) {
                crate::DeclarationPayloadData::Property(declaration) => Some(declaration),
                _ => None,
            },
            _ => None,
        }
    }
}
