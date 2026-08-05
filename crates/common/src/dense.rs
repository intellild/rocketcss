//! Typed dense identities and storage for compiler-internal state.

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

/// The shared behavior implemented by domain-specific dense IDs.
///
/// Implement IDs with [`define_dense_id!`] rather than implementing this
/// trait manually. Its construction method is public only so the generated
/// implementation can live in another crate.
#[doc(hidden)]
pub trait DenseId: Copy + Eq + Hash + fmt::Debug {
    #[doc(hidden)]
    fn from_index(index: usize) -> Option<Self>;

    fn index(self) -> usize;
}

/// A zero-based `u32` that reserves `u32::MAX` as an invalid niche.
///
/// The complemented representation lets `Option<NonMaxU32>` reuse the
/// `NonZeroU32` niche without unstable layout attributes.
#[doc(hidden)]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NonMaxU32(NonZeroU32);

impl NonMaxU32 {
    #[doc(hidden)]
    #[inline]
    pub const fn new(value: u32) -> Option<Self> {
        match NonZeroU32::new(!value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub const fn get(self) -> u32 {
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

/// Declares an opaque, zero-based dense ID type.
#[macro_export]
macro_rules! define_dense_id {
    ($(#[$attribute:meta])* $visibility:vis struct $name:ident $(;)?) => {
        $(#[$attribute])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $visibility struct $name($crate::dense::NonMaxU32);

        impl $name {
            #[inline]
            $visibility const fn index(self) -> usize {
                self.0.get() as usize
            }
        }

        impl $crate::dense::DenseId for $name {
            #[inline]
            fn from_index(index: usize) -> Option<Self> {
                let index = u32::try_from(index).ok()?;
                $crate::dense::NonMaxU32::new(index).map(Self)
            }

            #[inline]
            fn index(self) -> usize {
                self.index()
            }
        }
    };
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

/// An append-only vector that assigns a typed ID to each inserted value.
///
/// IDs from different domains cannot be mixed, even when both stores use the
/// same value type:
///
/// ```compile_fail
/// use rocketcss_common::{DenseStore, define_dense_id};
///
/// define_dense_id!(struct LeftId);
/// define_dense_id!(struct RightId);
///
/// let mut left = DenseStore::<LeftId, _>::new();
/// let right = DenseStore::<RightId, _>::new();
/// let left_id = left.push(1_u8);
/// let _ = right[left_id];
/// ```
pub struct DenseStore<I: DenseId, T> {
    values: std::vec::Vec<T>,
    marker: PhantomData<fn(I) -> I>,
}

impl<I: DenseId, T> DenseStore<I, T> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            values: std::vec::Vec::new(),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: std::vec::Vec::with_capacity(capacity),
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) -> I {
        self.try_push(value)
            .unwrap_or_else(|_| panic!("{} count exceeds u32::MAX", type_name::<I>()))
    }

    #[inline]
    pub fn try_push(&mut self, value: T) -> Result<I, DenseCapacityError> {
        let id = self.try_next_available()?;
        self.values.push(value);
        Ok(id)
    }

    #[inline]
    pub fn next_available(&self) -> I {
        self.try_next_available()
            .unwrap_or_else(|_| panic!("{} count exceeds u32::MAX", type_name::<I>()))
    }

    #[inline]
    pub fn try_next_available(&self) -> Result<I, DenseCapacityError> {
        I::from_index(self.values.len()).ok_or(DenseCapacityError)
    }

    #[inline]
    pub fn get(&self, id: I) -> &T {
        &self.values[id.index()]
    }

    #[inline]
    pub fn get_mut(&mut self, id: I) -> &mut T {
        &mut self.values[id.index()]
    }

    #[inline]
    pub fn try_get(&self, id: I) -> Option<&T> {
        self.values.get(id.index())
    }

    #[inline]
    pub fn try_get_mut(&mut self, id: I) -> Option<&mut T> {
        self.values.get_mut(id.index())
    }

    pub fn get_two_mut(&mut self, left: I, right: I) -> Option<(&mut T, &mut T)> {
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
    pub fn ids(&self) -> impl ExactSizeIterator<Item = I> + '_ {
        (0..self.values.len()).map(|index| {
            I::from_index(index).expect("a DenseStore length always fits its ID domain")
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
    pub fn iter_enumerated(&self) -> impl ExactSizeIterator<Item = (I, &T)> + '_ {
        self.values.iter().enumerate().map(|(index, value)| {
            (
                I::from_index(index).expect("a DenseStore length always fits its ID domain"),
                value,
            )
        })
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
        self.values.reserve(additional);
    }
}

impl<I: DenseId, T> Default for DenseStore<I, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: DenseId, T: fmt::Debug> fmt::Debug for DenseStore<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.values).finish()
    }
}

impl<I: DenseId, T: PartialEq> PartialEq for DenseStore<I, T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<I: DenseId, T: Eq> Eq for DenseStore<I, T> {}

impl<I: DenseId, T> Index<I> for DenseStore<I, T> {
    type Output = T;

    #[inline]
    fn index(&self, id: I) -> &Self::Output {
        self.get(id)
    }
}

impl<I: DenseId, T> IndexMut<I> for DenseStore<I, T> {
    #[inline]
    fn index_mut(&mut self, id: I) -> &mut Self::Output {
        self.get_mut(id)
    }
}

/// Secondary state indexed by IDs allocated by a [`DenseStore`].
///
/// A map copies the source store's shape and does not borrow it. Growing the
/// source store does not grow an existing map; consumers must rebuild the map
/// before indexing it with newly allocated IDs.
pub struct DenseMap<I: DenseId, T> {
    values: std::vec::Vec<T>,
    marker: PhantomData<fn(I) -> I>,
}

impl<I: DenseId, T> DenseMap<I, T> {
    #[inline]
    pub fn from_store<U>(store: &DenseStore<I, U>, mut init: impl FnMut(I) -> T) -> Self {
        let values = store.ids().map(&mut init).collect();
        Self {
            values,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self, id: I) -> &T {
        &self.values[id.index()]
    }

    #[inline]
    pub fn get_mut(&mut self, id: I) -> &mut T {
        &mut self.values[id.index()]
    }

    #[inline]
    pub fn try_get(&self, id: I) -> Option<&T> {
        self.values.get(id.index())
    }

    #[inline]
    pub fn try_get_mut(&mut self, id: I) -> Option<&mut T> {
        self.values.get_mut(id.index())
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.values.iter_mut()
    }

    #[inline]
    pub fn iter_enumerated(&self) -> impl ExactSizeIterator<Item = (I, &T)> + '_ {
        self.values.iter().enumerate().map(|(index, value)| {
            (
                I::from_index(index).expect("a DenseMap length always fits its ID domain"),
                value,
            )
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<I: DenseId, T: fmt::Debug> fmt::Debug for DenseMap<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.values).finish()
    }
}

impl<I: DenseId, T: PartialEq> PartialEq for DenseMap<I, T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

impl<I: DenseId, T: Eq> Eq for DenseMap<I, T> {}

impl<I: DenseId, T> Index<I> for DenseMap<I, T> {
    type Output = T;

    #[inline]
    fn index(&self, id: I) -> &Self::Output {
        self.get(id)
    }
}

impl<I: DenseId, T> IndexMut<I> for DenseMap<I, T> {
    #[inline]
    fn index_mut(&mut self, id: I) -> &mut Self::Output {
        self.get_mut(id)
    }
}

/// Allocates typed IDs when there is no value store for the identity domain.
pub struct DenseIdGenerator<I: DenseId> {
    next: usize,
    marker: PhantomData<fn(I) -> I>,
}

impl<I: DenseId> DenseIdGenerator<I> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            next: 0,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn allocate(&mut self) -> I {
        self.try_allocate()
            .unwrap_or_else(|_| panic!("{} count exceeds u32::MAX", type_name::<I>()))
    }

    #[inline]
    pub fn try_allocate(&mut self) -> Result<I, DenseCapacityError> {
        let id = I::from_index(self.next).ok_or(DenseCapacityError)?;
        self.next += 1;
        Ok(id)
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.next
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.next == 0
    }
}

impl<I: DenseId> Default for DenseIdGenerator<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: DenseId> fmt::Debug for DenseIdGenerator<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DenseIdGenerator")
            .field("next", &self.next)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::hash_map::DefaultHasher, mem::size_of};

    define_dense_id!(struct TestId);
    define_dense_id!(struct OtherId);

    #[test]
    fn non_max_round_trips_and_preserves_order() {
        for value in [0, 1, u32::MAX - 1] {
            let encoded = NonMaxU32::new(value).expect("value fits non-max integer");
            assert_eq!(encoded.get(), value);
        }
        assert!(NonMaxU32::new(u32::MAX).is_none());
        assert!(NonMaxU32::new(1).unwrap() < NonMaxU32::new(2).unwrap());
        assert_eq!(format!("{:?}", NonMaxU32::new(7).unwrap()), "7");
    }

    #[test]
    fn dense_ids_use_the_non_max_option_niche() {
        assert_eq!(size_of::<TestId>(), size_of::<u32>());
        assert_eq!(size_of::<Option<TestId>>(), size_of::<u32>());
        assert_eq!(TestId::from_index(0).unwrap().index(), 0);
        assert_eq!(TestId::from_index(1).unwrap().index(), 1);
        assert_eq!(
            TestId::from_index((u32::MAX - 1) as usize).unwrap().index(),
            (u32::MAX - 1) as usize
        );
        assert!(TestId::from_index(u32::MAX as usize).is_none());
    }

    #[test]
    fn dense_id_hashes_the_logical_value() {
        let id = TestId::from_index(42).unwrap();
        let mut left = DefaultHasher::new();
        id.hash(&mut left);
        let mut right = DefaultHasher::new();
        NonMaxU32::new(42).unwrap().hash(&mut right);
        assert_eq!(left.finish(), right.finish());
    }

    #[test]
    fn dense_store_assigns_stable_typed_ids() {
        let mut store = DenseStore::<TestId, _>::with_capacity(1);
        let first = store.push("first");
        let second = store.push("second");
        assert_eq!(first.index(), 0);
        assert_eq!(second.index(), 1);
        assert_eq!(store[first], "first");
        assert_eq!(store[second], "second");
        assert_eq!(store.ids().collect::<std::vec::Vec<_>>(), [first, second]);
        assert_eq!(
            store
                .iter_enumerated()
                .map(|(id, value)| (id.index(), *value))
                .collect::<std::vec::Vec<_>>(),
            [(0, "first"), (1, "second")]
        );
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct TinyId(usize);

    impl DenseId for TinyId {
        fn from_index(index: usize) -> Option<Self> {
            (index < 2).then_some(Self(index))
        }

        fn index(self) -> usize {
            self.0
        }
    }

    #[test]
    fn dense_store_rejects_capacity_before_insertion() {
        let mut store = DenseStore::<TinyId, _>::new();
        store.push("first");
        store.push("second");
        assert_eq!(store.try_push("third"), Err(DenseCapacityError));
        assert_eq!(store.len(), 2);
        assert_eq!(
            store.iter().copied().collect::<std::vec::Vec<_>>(),
            ["first", "second"]
        );
    }

    #[test]
    fn dense_store_borrows_two_distinct_values_in_caller_order() {
        let mut store = DenseStore::<TestId, _>::default();
        let first = store.push(1);
        let second = store.push(2);
        let (right, left) = store.get_two_mut(second, first).unwrap();
        *right = 20;
        *left = 10;
        assert_eq!(store[first], 10);
        assert_eq!(store[second], 20);
        assert!(store.get_two_mut(first, first).is_none());
    }

    #[test]
    fn dense_map_uses_the_source_store_shape() {
        let mut store = DenseStore::<TestId, _>::default();
        let first = store.push("first");
        let second = store.push("second");
        let mut map = DenseMap::from_store(&store, |id| id.index());
        assert_eq!(map.len(), store.len());
        assert_eq!(map[first], 0);
        assert_eq!(map[second], 1);
        map[second] = 7;
        assert_eq!(map[second], 7);
    }

    #[test]
    fn dense_generator_allocates_zero_based_ids() {
        let mut generator = DenseIdGenerator::<OtherId>::default();
        assert_eq!(generator.allocate().index(), 0);
        assert_eq!(generator.allocate().index(), 1);
        assert_eq!(generator.len(), 2);
    }

    #[test]
    fn dense_generator_rejects_the_reserved_value_without_advancing() {
        let mut generator = DenseIdGenerator::<OtherId> {
            next: u32::MAX as usize,
            marker: PhantomData,
        };
        assert_eq!(generator.try_allocate(), Err(DenseCapacityError));
        assert_eq!(generator.len(), u32::MAX as usize);
    }
}
