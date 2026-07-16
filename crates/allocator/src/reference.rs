use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    pin::Pin,
    ptr::NonNull,
};

/// A stable reference to a pinned value stored in an arena.
///
/// The arena owns the referenced allocation, so copying or dropping this
/// handle never affects the pointee. Accessors preserve the pointee's pinning
/// guarantee.
#[repr(transparent)]
pub struct Ref<'a, T: 'a> {
    pointer: NonNull<T>,
    marker: PhantomData<Pin<&'a T>>,
}

impl<'a, T> Ref<'a, T> {
    #[inline]
    pub fn from_pin(value: Pin<&mut T>) -> Self {
        Self {
            pointer: NonNull::from(value.as_ref().get_ref()),
            marker: PhantomData,
        }
    }

    /// Creates a stable arena-lifetime reference from an arena-owned pinned box.
    #[inline]
    pub fn from_pinned_box(value: &Pin<crate::boxed::Box<'a, T>>) -> Self {
        Self {
            pointer: NonNull::from(value.as_ref().get_ref()),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn get(self) -> Pin<&'a T> {
        // SAFETY: the arena keeps the pointee alive for `'a`, and `Ref` is only
        // constructed from an already pinned value.
        unsafe { Pin::new_unchecked(self.pointer.as_ref()) }
    }

    #[inline]
    pub fn get_mut(&mut self) -> Pin<&mut T> {
        // SAFETY: mutable access is tied to the exclusive borrow of this
        // handle, and the pointee remains pinned in its arena allocation.
        unsafe { Pin::new_unchecked(self.pointer.as_mut()) }
    }
}

impl<T> Clone for Ref<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ref<'_, T> {}

impl<T> PartialEq for Ref<'_, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.pointer == other.pointer
    }
}

impl<T> Eq for Ref<'_, T> {}

impl<T> Hash for Ref<'_, T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer.hash(state);
    }
}

impl<T> fmt::Debug for Ref<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Ref").field(&self.pointer).finish()
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
    fn preserves_the_pinned_pointee() {
        let allocator = Allocator::new();
        let mut value = allocator.pinned(PinnedValue {
            value: 42,
            _pin: PhantomPinned,
        });
        let pointer = value.as_ref().get_ref() as *const PinnedValue;
        let reference = Ref::from_pin(value.as_mut());

        assert_eq!(reference.get().value, 42);
        assert_eq!(reference.get().get_ref() as *const PinnedValue, pointer);
    }

    #[test]
    fn borrows_a_pinned_arena_box_for_the_arena_lifetime() {
        let allocator = Allocator::new();
        let value = allocator.pinned(PinnedValue {
            value: 42,
            _pin: PhantomPinned,
        });
        let reference = Ref::from_pinned_box(&value);

        assert_eq!(reference.get().value, 42);
    }
}
