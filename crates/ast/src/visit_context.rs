use rocketcss_allocator::{GhostCell, GhostToken, Ref};
use std::pin::Pin;

use crate::{StyleRule, VisitMut, VisitorMut};

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
        cell: &GhostCell<'ghost, T>,
        f: impl FnOnce(&T, &Self) -> R,
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
        cell: &GhostCell<'ghost, T>,
        f: impl FnOnce(&mut T, &mut Self) -> R,
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

    /// Visits a pinned style one allocation at a time.
    ///
    /// The style's declaration cell and nested rule list are visited only
    /// after releasing the mutable borrow of the pinned style itself.
    pub fn visit_ref<'a, VisitorT>(
        &mut self,
        reference: Ref<'a, 'ghost, StyleRule<'a, 'ghost>>,
        visitor: &mut VisitorT,
    ) where
        VisitorT: ?Sized + VisitorMut<'a, 'ghost>,
    {
        visitor.enter_node(crate::AstType::StyleRule);
        self.with_ref(reference, |value, cx| {
            visitor.visit_style_rule(value, cx);
        });

        let declarations = self.with_ref(reference, |value, _| value.declarations);
        self.with_cell(declarations, |value, cx| {
            VisitMut::visit_mut(value, visitor, cx);
        });

        let mut rules = self.with_ref(reference, |mut value, _| {
            // SAFETY: replacing a field does not move the pinned style.
            let value = unsafe { value.as_mut().get_unchecked_mut() };
            let empty = rocketcss_allocator::vec::Vec::new_in(value.rules.bump());
            std::mem::replace(&mut value.rules, empty)
        });
        VisitMut::visit_mut(&mut rules, visitor, self);
        self.with_ref(reference, |mut value, _| {
            // SAFETY: restoring a field does not move the pinned style.
            let value = unsafe { value.as_mut().get_unchecked_mut() };
            debug_assert!(value.rules.is_empty());
            value.rules = rules;
        });

        self.with_ref(reference, |mut value, cx| {
            // SAFETY: visiting these fields does not move the pinned style.
            let value = unsafe { value.as_mut().get_unchecked_mut() };
            visitor.visit_selector_list(&mut value.selectors, cx);
            VisitMut::visit_mut(&mut value.vendor_prefix, visitor, cx);
        });
        visitor.leave_node(crate::AstType::StyleRule);
    }
}
