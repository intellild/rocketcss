use crate::{DeclarationBlockId, DeclarationBlockStore};
use rocketcss_common::{GhostCell, GhostToken, Ref};
use std::pin::Pin;

/// Shared GhostCell access carried through immutable AST traversal.
pub struct VisitContext<'token, 'ast, 'ghost> {
    token: &'token GhostToken<'ghost>,
    declaration_blocks: Option<&'token DeclarationBlockStore<'ast>>,
}

impl<'token, 'ast, 'ghost> VisitContext<'token, 'ast, 'ghost> {
    #[inline]
    pub fn new(token: &'token GhostToken<'ghost>) -> Self {
        Self {
            token,
            declaration_blocks: None,
        }
    }

    #[inline]
    pub fn new_with_declaration_blocks(
        token: &'token GhostToken<'ghost>,
        declaration_blocks: &'token DeclarationBlockStore<'ast>,
    ) -> Self {
        Self {
            token,
            declaration_blocks: Some(declaration_blocks),
        }
    }

    #[inline]
    pub fn with_declaration_block<R>(
        &self,
        id: DeclarationBlockId,
        f: impl FnOnce(&crate::DeclarationBlock<'ast>, &Self) -> R,
    ) -> R {
        let blocks = self
            .declaration_blocks
            .expect("declaration block traversal requires a compilation store");
        f(blocks.get(id), self)
    }

    #[inline]
    pub const fn token(&self) -> &'token GhostToken<'ghost> {
        self.token
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

/// Unique GhostCell access carried through mutable AST traversal.
pub struct VisitMutContext<'token, 'ast, 'ghost> {
    state: VisitMutState<'token, 'ghost>,
    declaration_blocks: DeclarationBlockVisitState<'ast>,
}

enum VisitMutState<'token, 'ghost> {
    Available(&'token mut GhostToken<'ghost>),
    Borrowed,
}

enum DeclarationBlockVisitState<'ast> {
    Unavailable,
    Available(*mut DeclarationBlockStore<'ast>),
    Borrowed,
}

struct DeclarationBlockScopeReset<'ast>(*mut DeclarationBlockVisitState<'ast>);

impl Drop for DeclarationBlockScopeReset<'_> {
    fn drop(&mut self) {
        // SAFETY: the guard never outlives the `VisitMutContext` field it points
        // to, and the callback cannot move that context while it is borrowed.
        unsafe { *self.0 = DeclarationBlockVisitState::Unavailable };
    }
}

impl<'token, 'ast, 'ghost> VisitMutContext<'token, 'ast, 'ghost> {
    #[inline]
    pub fn new(token: &'token mut GhostToken<'ghost>) -> Self {
        Self {
            state: VisitMutState::Available(token),
            declaration_blocks: DeclarationBlockVisitState::Unavailable,
        }
    }

    #[inline]
    pub fn new_with_declaration_blocks(
        token: &'token mut GhostToken<'ghost>,
        declaration_blocks: &'token mut DeclarationBlockStore<'ast>,
    ) -> Self {
        Self {
            state: VisitMutState::Available(token),
            declaration_blocks: DeclarationBlockVisitState::Available(declaration_blocks),
        }
    }

    #[inline]
    pub fn with_declaration_block<R>(
        &mut self,
        id: DeclarationBlockId,
        f: impl FnOnce(&mut crate::DeclarationBlock<'ast>, &mut Self) -> R,
    ) -> R {
        let DeclarationBlockVisitState::Available(blocks) = std::mem::replace(
            &mut self.declaration_blocks,
            DeclarationBlockVisitState::Borrowed,
        ) else {
            panic!("declaration block traversal requires an available compilation store");
        };
        // SAFETY: `Available` is installed only from a live exclusive borrow.
        // Replacing it with `Borrowed` prevents nested access while `f` runs.
        let blocks = unsafe { &mut *blocks };
        let result = f(blocks.get_mut(id), self);
        self.declaration_blocks = DeclarationBlockVisitState::Available(blocks);
        result
    }

    pub fn with_declaration_blocks<R>(
        &mut self,
        declaration_blocks: &mut DeclarationBlockStore<'ast>,
        callback: impl FnOnce(&mut Self) -> R,
    ) -> R {
        assert!(
            matches!(
                self.declaration_blocks,
                DeclarationBlockVisitState::Unavailable
            ),
            "declaration block store is already available"
        );
        self.declaration_blocks = DeclarationBlockVisitState::Available(declaration_blocks);
        let _reset = DeclarationBlockScopeReset(&mut self.declaration_blocks);
        callback(self)
    }

    #[inline]
    pub fn with_cell<T: ?Sized, R>(
        &mut self,
        cell: Pin<&GhostCell<'ghost, T>>,
        f: impl FnOnce(Pin<&mut T>, &mut Self) -> R,
    ) -> R {
        let VisitMutState::Available(token) =
            std::mem::replace(&mut self.state, VisitMutState::Borrowed)
        else {
            panic!("nested mutable GhostCell access");
        };
        let result = {
            let value = cell.borrow_mut(&mut *token);
            f(value, self)
        };
        self.state = VisitMutState::Available(token);
        result
    }

    #[inline]
    pub fn with_ref<'a, T: ?Sized, R>(
        &mut self,
        reference: Ref<'a, 'ghost, T>,
        f: impl FnOnce(Pin<&mut T>, &mut Self) -> R,
    ) -> R {
        let VisitMutState::Available(token) =
            std::mem::replace(&mut self.state, VisitMutState::Borrowed)
        else {
            panic!("nested mutable Ref access");
        };
        let result = {
            let value = reference.get_mut(&mut *token);
            f(value, self)
        };
        self.state = VisitMutState::Available(token);
        result
    }
}
