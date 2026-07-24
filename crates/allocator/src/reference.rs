use std::{
    fmt,
    hash::{Hash, Hasher},
    pin::Pin,
};

use crate::{GhostBox, GhostCell, GhostToken};

/// A token-controlled reference to a pinned arena value.
///
/// The arena [`GhostBox`] establishes the stable-address contract, while
/// [`GhostToken`] prevents overlapping mutable access.
#[repr(transparent)]
pub struct Ref<'a, 'ghost, T: ?Sized> {
    cell: Pin<&'a GhostCell<'ghost, T>>,
}

impl<'a, 'ghost, T> From<&GhostBox<'a, 'ghost, T>> for Ref<'a, 'ghost, T> {
    #[inline]
    fn from(owner: &GhostBox<'a, 'ghost, T>) -> Self {
        let cell = owner.as_ref().get_ref() as *const GhostCell<'ghost, T>;
        // SAFETY: the custom arena Box never moves or drops its pointee, and
        // its `'a` lifetime is tied to the arena allocation. GhostCell is
        // !Unpin, so safe code cannot extract it from the pinned Box.
        let cell: &'a GhostCell<'ghost, T> = unsafe { &*cell };
        // SAFETY: the owner is pinned and the arena allocation remains at the
        // same address for `'a`.
        let cell = unsafe { Pin::new_unchecked(cell) };
        Self { cell }
    }
}

impl<'a, 'ghost, T: ?Sized> Ref<'a, 'ghost, T> {
    #[inline]
    pub fn get<'cell>(&self, _token: &'cell GhostToken<'ghost>) -> Pin<&'cell T>
    where
        'a: 'cell,
    {
        self.cell.borrow(_token)
    }

    #[inline]
    pub fn get_mut<'cell>(&self, _token: &'cell mut GhostToken<'ghost>) -> Pin<&'cell mut T>
    where
        'a: 'cell,
    {
        self.cell.borrow_mut(_token)
    }
}

impl<T: ?Sized> Clone for Ref<'_, '_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Ref<'_, '_, T> {}

impl<T: ?Sized> PartialEq for Ref<'_, '_, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.cell.get_ref(), other.cell.get_ref())
    }
}

impl<T: ?Sized> Eq for Ref<'_, '_, T> {}

impl<T: ?Sized> Hash for Ref<'_, '_, T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.cell.get_ref(), state);
    }
}

impl<T: ?Sized> fmt::Debug for Ref<'_, '_, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Ref")
            .field(&std::ptr::from_ref(self.cell.get_ref()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomPinned;

    use super::*;
    use crate::Allocator;

    #[derive(Debug)]
    struct PinnedValue {
        value: u32,
        _pin: PhantomPinned,
    }

    #[test]
    fn token_controls_a_reference_to_a_pinned_owner() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let owner = allocator.pinned(GhostCell::new(PinnedValue {
                value: 42,
                _pin: PhantomPinned,
            }));
            let reference = Ref::from(&owner);

            assert_eq!(reference.get(&token).value, 42);
            // SAFETY: assigning a field does not move the pinned value.
            unsafe { reference.get_mut(&mut token).get_unchecked_mut() }.value = 7;
            assert_eq!(reference.get(&token).value, 7);
        });
    }
}
