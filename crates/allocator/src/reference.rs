use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    pin::Pin,
    ptr::NonNull,
};

use crate::{GhostToken, boxed::Box, ghost_cell::InvariantLifetime};

/// A token-controlled reference to a pinned arena value.
///
/// `Ref` consumes the pinned box handle and is thereafter the only safe access
/// path to the pointee. [`GhostToken`] prevents overlapping mutable access,
/// while the consumed `Pin<Box<T>>` establishes the stable-address contract.
#[repr(transparent)]
pub struct Ref<'a, 'ghost, T: ?Sized> {
    pointer: NonNull<T>,
    _pin: PhantomData<Pin<&'a mut T>>,
    _brand: InvariantLifetime<'ghost>,
}

impl<'a, 'ghost, T> Ref<'a, 'ghost, T> {
    #[inline]
    pub fn from_pinned_box(value: Pin<Box<'a, T>>) -> Self {
        // SAFETY: extracting the box does not move its pointee. The box is
        // consumed immediately below, transferring its unique pointer into
        // this handle while `Ref` preserves the pinning contract.
        let value = unsafe { Pin::into_inner_unchecked(value) };
        let pointer =
            NonNull::new(Box::into_raw(value)).expect("arena boxes always contain a value");
        // The arena box has no destructor and its allocation remains live for
        // `'a`. Consuming it here removes the ordinary owner access path.
        Self {
            pointer,
            _pin: PhantomData,
            _brand: PhantomData,
        }
    }
}

impl<'a, 'ghost, T: ?Sized> Ref<'a, 'ghost, T> {
    #[inline]
    pub fn get<'cell>(&self, _token: &'cell GhostToken<'ghost>) -> Pin<&'cell T>
    where
        'a: 'cell,
    {
        // SAFETY: construction consumes a Pin<Box<T>>, and shared token access
        // cannot move the pointee.
        unsafe { Pin::new_unchecked(self.pointer.as_ref()) }
    }

    #[inline]
    pub fn get_mut<'cell>(&self, _token: &'cell mut GhostToken<'ghost>) -> Pin<&'cell mut T>
    where
        'a: 'cell,
    {
        // SAFETY: construction consumes a Pin<Box<T>>, and the unique token
        // prevents any overlapping access while this borrow is live.
        unsafe { Pin::new_unchecked(&mut *self.pointer.as_ptr()) }
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
        std::ptr::eq(self.pointer.as_ptr(), other.pointer.as_ptr())
    }
}

impl<T: ?Sized> Eq for Ref<'_, '_, T> {}

impl<T: ?Sized> Hash for Ref<'_, '_, T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer.hash(state);
    }
}

impl<T: ?Sized> fmt::Debug for Ref<'_, '_, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("Ref").field(&self.pointer).finish()
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
    fn token_controls_a_consumed_pinned_box() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let reference = Ref::from_pinned_box(allocator.pinned(PinnedValue {
                value: 42,
                _pin: PhantomPinned,
            }));

            assert_eq!(reference.get(&token).value, 42);
            // SAFETY: assigning a field does not move the pinned value.
            unsafe { reference.get_mut(&mut token).get_unchecked_mut() }.value = 7;
            assert_eq!(reference.get(&token).value, 7);
        });
    }
}
