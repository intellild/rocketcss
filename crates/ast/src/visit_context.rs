use rocketcss_common::{GhostCell, GhostToken, Ref};
use std::{marker::PhantomData, pin::Pin};

/// Shared GhostCell access carried through immutable value-AST traversal.
pub struct VisitContext<'token, 'ast, 'ghost> {
    token: &'token GhostToken<'ghost>,
    ast: PhantomData<&'ast ()>,
}

impl<'token, 'ast, 'ghost> VisitContext<'token, 'ast, 'ghost> {
    #[inline]
    pub const fn new(token: &'token GhostToken<'ghost>) -> Self {
        Self {
            token,
            ast: PhantomData,
        }
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

/// Unique GhostCell access carried through mutable value-AST traversal.
pub struct VisitMutContext<'token, 'ast, 'ghost> {
    state: VisitMutState<'token, 'ghost>,
    ast: PhantomData<&'ast mut ()>,
}

enum VisitMutState<'token, 'ghost> {
    Available(&'token mut GhostToken<'ghost>),
    Borrowed,
}

impl<'token, 'ast, 'ghost> VisitMutContext<'token, 'ast, 'ghost> {
    #[inline]
    pub const fn new(token: &'token mut GhostToken<'ghost>) -> Self {
        Self {
            state: VisitMutState::Available(token),
            ast: PhantomData,
        }
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
