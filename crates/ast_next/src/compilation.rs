//! Compiler-owned result of parsing one stylesheet.

use crate::{Ast, StyleSheet};

/// The flattened stylesheet tree together with the context that owns it.
pub struct Compilation<'ast> {
    ast: Ast<'ast>,
    style_sheet: StyleSheet<'ast>,
}

impl<'ast> Compilation<'ast> {
    #[inline]
    pub fn new(ast: Ast<'ast>, style_sheet: StyleSheet<'ast>) -> Self {
        Self { ast, style_sheet }
    }

    #[inline]
    pub fn ast(&self) -> &Ast<'ast> {
        &self.ast
    }

    #[inline]
    pub fn ast_mut(&mut self) -> &mut Ast<'ast> {
        &mut self.ast
    }

    #[inline]
    pub fn style_sheet(&self) -> StyleSheet<'ast> {
        self.style_sheet
    }

    #[inline]
    pub fn into_parts(self) -> (Ast<'ast>, StyleSheet<'ast>) {
        (self.ast, self.style_sheet)
    }
}

impl std::fmt::Debug for Compilation<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Compilation")
            .field("nodes", &self.ast.len())
            .finish()
    }
}
