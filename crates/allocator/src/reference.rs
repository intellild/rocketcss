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
///
/// # Soundness trade-off
///
/// `Ref` is a detached handle rather than a Rust borrow. Pinning guarantees a
/// stable address, but the type system cannot prevent the owning pinned box
/// from being mutably accessed while a shared reference returned by [`Ref::get`]
/// is alive. Current users rely on phase separation: declaration blocks are
/// mutated while minifying and only read through merge links while generating
/// code. Using `Ref` outside such an access discipline requires redesigning the
/// ownership boundary or making shared dereferencing unsafe.
///
/// `Ref` is a shared handle and intentionally does not provide safe mutable access:
///
/// ```compile_fail
/// use rocketcss_allocator::{Allocator, Ref};
///
/// let allocator = Allocator::new();
/// let mut value = allocator.pinned(1);
/// let mut reference = Ref::from_pin(value.as_mut());
/// let _ = reference.get_mut();
/// ```
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

    /// Returns mutable pinned access to the pointee.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other handle or reference accesses the pointee
    /// for the duration of the returned borrow.
    #[inline]
    pub unsafe fn get_mut_unchecked(&mut self) -> Pin<&mut T> {
        // SAFETY: the caller guarantees unique access, and the arena allocation
        // keeps the pointee at a stable address.
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

    #[test]
    fn permits_explicitly_unsafe_unique_mutation() {
        let allocator = Allocator::new();
        let mut value = allocator.pinned(PinnedValue {
            value: 42,
            _pin: PhantomPinned,
        });
        let mut reference = Ref::from_pin(value.as_mut());

        // SAFETY: this is the only handle used to access the pointee here.
        let pinned = unsafe { reference.get_mut_unchecked() };
        // SAFETY: changing the integer field does not move the pinned value.
        unsafe { pinned.get_unchecked_mut() }.value = 7;

        assert_eq!(reference.get().value, 7);
    }
}
