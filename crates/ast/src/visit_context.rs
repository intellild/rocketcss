use rocketcss_allocator::{GhostCell, GhostToken, Ref};
use std::pin::Pin;

/// Shared GhostCell access carried through immutable AST traversal.
pub struct VisitContext<'token, 'ghost> {
    token: &'token GhostToken<'ghost>,
}

impl<'token, 'ghost> VisitContext<'token, 'ghost> {
    #[inline]
    pub fn new(token: &'token GhostToken<'ghost>) -> Self {
        Self { token }
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
pub struct VisitMutContext<'token, 'ghost> {
    state: VisitMutState<'token, 'ghost>,
}

enum VisitMutState<'token, 'ghost> {
    Available(&'token mut GhostToken<'ghost>),
    Borrowed,
}

impl<'token, 'ghost> VisitMutContext<'token, 'ghost> {
    #[inline]
    pub fn new(token: &'token mut GhostToken<'ghost>) -> Self {
        Self {
            state: VisitMutState::Available(token),
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
