use rocketcss_common::{GhostCell, GhostToken, Ref};
use std::{
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    pin::Pin,
};

use crate::{Compilation, NodeId};

/// Shared GhostCell access carried through immutable value-AST traversal.
pub struct VisitContext<'token, 'ast, 'ghost> {
    token: &'token GhostToken<'ghost>,
    ast: Option<&'token Compilation<'ast>>,
}

impl<'token, 'ast, 'ghost> VisitContext<'token, 'ast, 'ghost> {
    #[inline]
    pub const fn new(token: &'token GhostToken<'ghost>) -> Self {
        Self { token, ast: None }
    }

    #[inline]
    pub const fn with_ast(
        token: &'token GhostToken<'ghost>,
        ast: &'token Compilation<'ast>,
    ) -> Self {
        Self {
            token,
            ast: Some(ast),
        }
    }

    #[inline]
    pub const fn token(&self) -> &'token GhostToken<'ghost> {
        self.token
    }

    #[inline]
    pub fn ast_context(&self) -> &'token Compilation<'ast> {
        self.ast.expect("visiting a NodeId requires its AstContext")
    }

    #[inline]
    pub fn with_cell<T: ?Sized, R>(
        &self,
        cell: Pin<&GhostCell<'ghost, T>>,
        f: impl FnOnce(Pin<&T>, &Self) -> R,
    ) -> R {
        f(cell.borrow(self.token), self)
    }

    #[inline]
    pub fn with_ref<'a, T: ?Sized, R>(
        &self,
        reference: Ref<'a, 'ghost, T>,
        f: impl FnOnce(Pin<&T>, &Self) -> R,
    ) -> R {
        f(reference.get(self.token), self)
    }
}

/// Unique GhostCell access carried through mutable value-AST traversal.
pub struct VisitMutContext<'token, 'ast, 'ghost> {
    state: VisitMutState<'token, 'ast, 'ghost>,
}

enum VisitMutState<'token, 'ast, 'ghost> {
    Available {
        token: &'token mut GhostToken<'ghost>,
        ast: Option<&'token mut Compilation<'ast>>,
    },
    Borrowed,
}

impl<'token, 'ast, 'ghost> VisitMutContext<'token, 'ast, 'ghost> {
    #[inline]
    pub const fn new(token: &'token mut GhostToken<'ghost>) -> Self {
        Self {
            state: VisitMutState::Available { token, ast: None },
        }
    }

    #[inline]
    pub const fn with_ast(
        token: &'token mut GhostToken<'ghost>,
        ast: &'token mut Compilation<'ast>,
    ) -> Self {
        Self {
            state: VisitMutState::Available {
                token,
                ast: Some(ast),
            },
        }
    }

    #[inline]
    pub fn ast_allocator(&self) -> &'ast rocketcss_common::Allocator {
        let VisitMutState::Available { ast: Some(ast), .. } = &self.state else {
            panic!("visiting a NodeId requires its available AstContext");
        };
        ast.allocator()
    }

    /// Returns the AST context while no nested mutable node transaction is active.
    #[inline]
    pub fn ast_context(&self) -> &Compilation<'ast> {
        let VisitMutState::Available { ast: Some(ast), .. } = &self.state else {
            panic!("visiting a NodeId requires its available AstContext");
        };
        ast
    }

    /// Returns unique access to the AST context while no nested transaction is active.
    #[inline]
    pub fn ast_context_mut(&mut self) -> &mut Compilation<'ast> {
        let VisitMutState::Available { ast: Some(ast), .. } = &mut self.state else {
            panic!("visiting a NodeId requires its available AstContext");
        };
        ast
    }

    #[inline]
    pub fn with_cell<T: ?Sized, R>(
        &mut self,
        cell: Pin<&GhostCell<'ghost, T>>,
        f: impl FnOnce(Pin<&mut T>, &mut Self) -> R,
    ) -> R {
        let VisitMutState::Available { token, ast } =
            std::mem::replace(&mut self.state, VisitMutState::Borrowed)
        else {
            panic!("nested mutable GhostCell access");
        };
        let result = catch_unwind(AssertUnwindSafe(|| {
            let value = cell.borrow_mut(&mut *token);
            f(value, self)
        }));
        self.state = VisitMutState::Available { token, ast };
        match result {
            Ok(result) => result,
            Err(payload) => resume_unwind(payload),
        }
    }

    #[inline]
    pub fn with_ref<'a, T: ?Sized, R>(
        &mut self,
        reference: Ref<'a, 'ghost, T>,
        f: impl FnOnce(Pin<&mut T>, &mut Self) -> R,
    ) -> R {
        let VisitMutState::Available { token, ast } =
            std::mem::replace(&mut self.state, VisitMutState::Borrowed)
        else {
            panic!("nested mutable Ref access");
        };
        let result = catch_unwind(AssertUnwindSafe(|| {
            let value = reference.get_mut(&mut *token);
            f(value, self)
        }));
        self.state = VisitMutState::Available { token, ast };
        match result {
            Ok(result) => result,
            Err(payload) => resume_unwind(payload),
        }
    }

    pub fn mutate_node<T, R>(
        &mut self,
        id: NodeId<'ast, T>,
        visit: impl FnOnce(&mut T, &mut Self) -> R,
    ) -> R {
        let VisitMutState::Available { ast: Some(ast), .. } = &mut self.state else {
            panic!("visiting a NodeId requires its available AstContext");
        };
        let ast = std::ptr::NonNull::from(&mut **ast);
        // SAFETY: the temporary unique borrow above has ended. The private node transaction keeps
        // this ID unavailable while the callback accesses the context through `self`; its guard
        // only uses the raw context pointer after the callback returns or starts unwinding.
        let mut mutation = unsafe { (*ast.as_ptr()).node_mutation(id) };
        visit(mutation.value(), self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutable_cell_access_restores_the_context_after_unwind() {
        GhostToken::scope(|mut token| {
            let cell = std::pin::pin!(GhostCell::new(1_u8));
            let mut context = VisitMutContext::new(&mut token);

            let result = catch_unwind(AssertUnwindSafe(|| {
                context.with_cell(cell.as_ref(), |_, _| panic!("stop cell mutation"));
            }));

            assert!(result.is_err());
            context.with_cell(cell.as_ref(), |mut value, _| *value = 2);
        });
    }
}
