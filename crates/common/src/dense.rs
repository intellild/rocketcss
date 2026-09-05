//! Typed dense identities and storage for compiler-internal state.

use crate::Allocator;
use std::{
    any::type_name,
    cmp::Ordering,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    num::NonZeroU32,
    ops::{Index, IndexMut},
};

/// A stable typed index produced by a [`DenseStore`].
///
/// The arena lifetime prevents an index from outliving the allocator that
/// anchors its store, while `T` separates independent identity domains.
/// This type intentionally exposes no integer-to-ID conversion.
///
/// ```compile_fail
/// use rocketcss_common::DenseId;
///
/// let _ = DenseId::<u8>::from_index(0);
/// ```
#[repr(transparent)]
pub struct DenseId<'arena, T> {
    inner: NonMaxU32,
    phantom_data: PhantomData<fn() -> &'arena T>,
}

impl<T> DenseId<'_, T> {
    #[inline]
    pub const fn index(self) -> usize {
        self.inner.get() as usize
    }
}

impl<T> Clone for DenseId<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for DenseId<'_, T> {}

impl<T> fmt::Debug for DenseId<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> PartialEq for DenseId<'_, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for DenseId<'_, T> {}

impl<T> PartialOrd for DenseId<'_, T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for DenseId<'_, T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> Hash for DenseId<'_, T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

mod private {
    pub trait SealedDenseId: Sized {
        fn from_index(index: usize) -> Option<Self>;
    }
}

impl<T> private::SealedDenseId for DenseId<'_, T> {
    #[inline]
    fn from_index(index: usize) -> Option<Self> {
        let index = u32::try_from(index).ok()?;
        Some(Self {
            inner: NonMaxU32::new(index)?,
            phantom_data: PhantomData,
        })
    }
}

/// A zero-based `u32` that reserves `u32::MAX` as an invalid niche.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct NonMaxU32(NonZeroU32);

impl NonMaxU32 {
    #[inline]
    const fn new(value: u32) -> Option<Self> {
        match NonZeroU32::new(!value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    #[inline]
    const fn get(self) -> u32 {
        !self.0.get()
    }
}

impl fmt::Debug for NonMaxU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl Hash for NonMaxU32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl PartialOrd for NonMaxU32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonMaxU32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

/// The capacity of a `u32`-backed dense identity has been exhausted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DenseCapacityError;

impl fmt::Display for DenseCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("dense ID capacity exceeds u32::MAX")
    }
}

impl Error for DenseCapacityError {}

/// IDs for one range whose bounds were validated by its originating store.
pub struct DenseIdRange<'arena, Domain> {
    range: std::ops::Range<usize>,
    marker: PhantomData<DenseId<'arena, Domain>>,
}

impl<'arena, Domain: 'arena> Iterator for DenseIdRange<'arena, Domain> {
    type Item = DenseId<'arena, Domain>;

    fn next(&mut self) -> Option<Self::Item> {
        <DenseId<'arena, Domain> as private::SealedDenseId>::from_index(self.range.next()?)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<Domain> ExactSizeIterator for DenseIdRange<'_, Domain> {}

/// An append-only vector that assigns an arena-bound typed ID to each value.
///
/// Stores cannot be created without a real allocator lifetime:
///
/// ```compile_fail
/// use rocketcss_common::DenseStore;
///
/// let _ = DenseStore::<u8, u8>::new();
/// ```
///
/// An ID cannot escape the allocator that anchors its store:
///
/// ```compile_fail
/// use rocketcss_common::{Allocator, DenseId, DenseStore};
///
/// fn escaped() -> DenseId<'static, u8> {
///     let allocator = Allocator::new();
///     DenseStore::<u8, _>::new_in(&allocator).push(1_u8)
/// }
/// ```
///
/// Different domains cannot be mixed even when their payloads are identical:
///
/// ```compile_fail
/// use rocketcss_common::{Allocator, DenseStore};
///
/// enum Left {}
/// enum Right {}
/// let allocator = Allocator::new();
/// let mut left = DenseStore::<Left, _>::new_in(&allocator);
/// let right = DenseStore::<Right, u8>::new_in(&allocator);
/// let left_id = left.push(1_u8);
/// let _ = right[left_id];
/// ```
pub struct DenseStore<'arena, Domain, T> {
    values: std::vec::Vec<T>,
    arena: PhantomData<&'arena Allocator>,
    domain: PhantomData<fn() -> Domain>,
}

impl<'arena, Domain: 'arena, T> DenseStore<'arena, Domain, T> {
    #[inline]
    pub fn new_in(_allocator: &'arena Allocator) -> Self {
        Self {
            values: std::vec::Vec::new(),
            arena: PhantomData,
            domain: PhantomData,
        }
    }

    #[inline]
    pub fn with_capacity_in(_allocator: &'arena Allocator, capacity: usize) -> Self {
        assert!(
            capacity <= u32::MAX as usize,
            "dense store capacity exceeds u32::MAX"
        );
        Self {
            values: std::vec::Vec::with_capacity(capacity),
            arena: PhantomData,
            domain: PhantomData,
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) -> DenseId<'arena, Domain> {
        self.try_push(value)
            .unwrap_or_else(|_| panic!("{} count exceeds u32::MAX", type_name::<Domain>()))
    }

    #[inline]
    pub fn try_push(&mut self, value: T) -> Result<DenseId<'arena, Domain>, DenseCapacityError> {
        if !self.has_capacity_for(1) {
            return Err(DenseCapacityError);
        }
        self.values.push(value);
        Ok(self.id_for_existing_index(self.values.len() - 1))
    }

    #[inline]
    pub fn has_capacity_for(&self, additional: usize) -> bool {
        self.values
            .len()
            .checked_add(additional)
            .is_some_and(|len| len <= u32::MAX as usize)
    }

    #[inline]
    fn id_for_existing_index(&self, index: usize) -> DenseId<'arena, Domain> {
        debug_assert!(index < self.values.len());
        <DenseId<'arena, Domain> as private::SealedDenseId>::from_index(index)
            .expect("a DenseStore index always fits its ID domain")
    }

    #[inline]
    pub fn get(&self, id: DenseId<'arena, Domain>) -> &T {
        &self.values[id.index()]
    }

    #[inline]
    pub fn get_mut(&mut self, id: DenseId<'arena, Domain>) -> &mut T {
        &mut self.values[id.index()]
    }

    #[inline]
    pub fn try_get(&self, id: DenseId<'arena, Domain>) -> Option<&T> {
        self.values.get(id.index())
    }

    #[inline]
    pub fn try_get_mut(&mut self, id: DenseId<'arena, Domain>) -> Option<&mut T> {
        self.values.get_mut(id.index())
    }

    pub fn get_two_mut(
        &mut self,
        left: DenseId<'arena, Domain>,
        right: DenseId<'arena, Domain>,
    ) -> Option<(&mut T, &mut T)> {
        if left == right {
            return None;
        }
        let (low, high, reversed) = if left.index() < right.index() {
            (left.index(), right.index(), false)
        } else {
            (right.index(), left.index(), true)
        };
        let (before_high, high_and_after) = self.values.split_at_mut(high);
        let low = before_high.get_mut(low)?;
        let high = high_and_after.first_mut()?;
        if reversed {
            Some((high, low))
        } else {
            Some((low, high))
        }
    }

    #[inline]
    pub fn ids(&self) -> impl ExactSizeIterator<Item = DenseId<'arena, Domain>> + '_ {
        (0..self.values.len()).map(|index| self.id_for_existing_index(index))
    }

    #[inline]
    pub fn id_at_offset(&self, start: usize, offset: usize) -> Option<DenseId<'arena, Domain>> {
        let index = start.checked_add(offset)?;
        (index < self.values.len()).then(|| self.id_for_existing_index(index))
    }

    #[inline]
    pub fn ids_in_range(&self, start: usize, len: usize) -> Option<DenseIdRange<'arena, Domain>> {
        let end = start.checked_add(len)?;
        (end <= self.values.len()).then_some(DenseIdRange {
            range: start..end,
            marker: PhantomData,
        })
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.values
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.values.iter_mut()
    }

    #[inline]
    pub fn iter_enumerated(
        &self,
    ) -> impl ExactSizeIterator<Item = (DenseId<'arena, Domain>, &T)> + '_ {
        self.values
            .iter()
            .enumerate()
            .map(|(index, value)| (self.id_for_existing_index(index), value))
    }

    #[inline]
    pub fn iter_enumerated_mut(
        &mut self,
    ) -> impl ExactSizeIterator<Item = (DenseId<'arena, Domain>, &mut T)> + '_ {
        self.values.iter_mut().enumerate().map(|(index, value)| {
            let id = <DenseId<'arena, Domain> as private::SealedDenseId>::from_index(index)
                .expect("a DenseStore index always fits its ID domain");
            (id, value)
        })
    }

    #[inline]
    pub fn iter_enumerated_range_mut(
        &mut self,
        start: usize,
        len: usize,
    ) -> Option<impl ExactSizeIterator<Item = (DenseId<'arena, Domain>, &mut T)> + '_> {
        let end = start.checked_add(len)?;
        let values = self.values.get_mut(start..end)?;
        Some(values.iter_mut().enumerate().map(move |(offset, value)| {
            let id =
                <DenseId<'arena, Domain> as private::SealedDenseId>::from_index(start + offset)
                    .expect("a DenseStore index always fits its ID domain");
            (id, value)
        }))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        assert!(
            self.has_capacity_for(additional),
            "dense store capacity exceeds u32::MAX"
        );
        self.values.reserve(additional);
    }

    /// Discards entries allocated after `len` while preserving all earlier IDs.
    #[doc(hidden)]
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.values.truncate(len);
    }
}

impl<Domain, T: fmt::Debug> fmt::Debug for DenseStore<'_, Domain, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.values).finish()
    }
}

impl<Domain, T: PartialEq> PartialEq for DenseStore<'_, Domain, T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<Domain, T: Eq> Eq for DenseStore<'_, Domain, T> {}

impl<'arena, Domain: 'arena, T> Index<DenseId<'arena, Domain>> for DenseStore<'arena, Domain, T> {
    type Output = T;

    fn index(&self, id: DenseId<'arena, Domain>) -> &Self::Output {
        self.get(id)
    }
}

impl<'arena, Domain: 'arena, T> IndexMut<DenseId<'arena, Domain>>
    for DenseStore<'arena, Domain, T>
{
    fn index_mut(&mut self, id: DenseId<'arena, Domain>) -> &mut Self::Output {
        self.get_mut(id)
    }
}

/// Secondary state indexed by IDs allocated by a [`DenseStore`].
pub struct DenseMap<'arena, Domain, T> {
    values: std::vec::Vec<T>,
    marker: PhantomData<DenseId<'arena, Domain>>,
}

impl<'arena, Domain: 'arena, T> DenseMap<'arena, Domain, T> {
    pub fn from_store<U>(
        store: &DenseStore<'arena, Domain, U>,
        mut init: impl FnMut(DenseId<'arena, Domain>) -> T,
    ) -> Self {
        let values = store.ids().map(&mut init).collect();
        Self {
            values,
            marker: PhantomData,
        }
    }

    pub fn get(&self, id: DenseId<'arena, Domain>) -> &T {
        &self.values[id.index()]
    }
    pub fn get_mut(&mut self, id: DenseId<'arena, Domain>) -> &mut T {
        &mut self.values[id.index()]
    }
    pub fn try_get(&self, id: DenseId<'arena, Domain>) -> Option<&T> {
        self.values.get(id.index())
    }
    pub fn try_get_mut(&mut self, id: DenseId<'arena, Domain>) -> Option<&mut T> {
        self.values.get_mut(id.index())
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.values.iter_mut()
    }
    pub fn iter_enumerated(
        &self,
    ) -> impl ExactSizeIterator<Item = (DenseId<'arena, Domain>, &T)> + '_ {
        self.values.iter().enumerate().map(|(index, value)| {
            let id = <DenseId<'arena, Domain> as private::SealedDenseId>::from_index(index)
                .expect("a DenseMap index always fits its ID domain");
            (id, value)
        })
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<Domain, T: fmt::Debug> fmt::Debug for DenseMap<'_, Domain, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.values).finish()
    }
}

impl<Domain, T: PartialEq> PartialEq for DenseMap<'_, Domain, T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<Domain, T: Eq> Eq for DenseMap<'_, Domain, T> {}

impl<'arena, Domain: 'arena, T> Index<DenseId<'arena, Domain>> for DenseMap<'arena, Domain, T> {
    type Output = T;
    fn index(&self, id: DenseId<'arena, Domain>) -> &Self::Output {
        self.get(id)
    }
}

impl<'arena, Domain: 'arena, T> IndexMut<DenseId<'arena, Domain>> for DenseMap<'arena, Domain, T> {
    fn index_mut(&mut self, id: DenseId<'arena, Domain>) -> &mut Self::Output {
        self.get_mut(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    enum TestDomain {}

    #[test]
    fn dense_ids_are_compact_and_store_owned() {
        let allocator = Allocator::new();
        let mut store = DenseStore::<TestDomain, _>::with_capacity_in(&allocator, 2);
        let first = store.push("first");
        let second = store.push("second");

        assert_eq!(size_of::<DenseId<'_, TestDomain>>(), size_of::<u32>());
        assert_eq!(
            size_of::<Option<DenseId<'_, TestDomain>>>(),
            size_of::<u32>()
        );
        assert_eq!(store[first], "first");
        assert_eq!(store[second], "second");
        assert_eq!(store.ids().collect::<std::vec::Vec<_>>(), [first, second]);
    }

    #[test]
    fn enumerated_and_range_access_only_return_existing_ids() {
        let allocator = Allocator::new();
        let mut store = DenseStore::<TestDomain, _>::new_in(&allocator);
        let first = store.push(1);
        let second = store.push(2);

        for (id, value) in store.iter_enumerated_mut() {
            *value += id.index();
        }
        assert_eq!(store[first], 1);
        assert_eq!(store[second], 3);
        assert!(store.ids_in_range(1, 1).is_some());
        assert!(store.ids_in_range(1, 2).is_none());
        assert_eq!(store.id_at_offset(0, 1), Some(second));
        assert_eq!(store.id_at_offset(1, 1), None);
    }

    #[test]
    fn store_reports_representable_capacity_without_producing_an_id() {
        let allocator = Allocator::new();
        let store = DenseStore::<TestDomain, ()>::new_in(&allocator);
        assert!(store.has_capacity_for(u32::MAX as usize));
        assert!(!store.has_capacity_for(u32::MAX as usize + 1));
    }
}
