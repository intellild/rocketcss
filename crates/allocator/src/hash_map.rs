//! From https://github.com/oxc-project/oxc/blob/65f9233148aa8bb4ea433130c040675e14255c36/crates/oxc_allocator/src/hash_map.rs
//!
//! A hash map without `Drop` that stores data in arena allocator.
//!
//! By default uses [`FxHasher`] to hash keys. The hasher can be customized via the `S` type
//! parameter (e.g. [`IdentBuildHasher`] for `Ident` keys).
//!
//! See [`HashMap`] for more details.
//!
//! [`FxHasher`]: rustc_hash::FxHasher
//! [`IdentBuildHasher`]: crate::IdentBuildHasher

// All methods which just delegate to `hashbrown::HashMap` methods marked `#[inline(always)]`
#![expect(clippy::inline_always)]

use std::{
    fmt,
    hash::{BuildHasher, Hash},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

use rustc_hash::FxBuildHasher;

// Re-export additional types from `hashbrown`
pub use hashbrown::{
    Equivalent, TryReserveError,
    hash_map::{
        Drain, Entry, EntryRef, ExtractIf, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys,
        OccupiedError, Values, ValuesMut,
    },
};

use crate::{Allocator, vec::Vec};

/// Number of entries an [`AdaptiveHashMap`] scans linearly before promoting
/// itself to a hash table.
pub const DEFAULT_LINEAR_SEARCH_LIMIT: usize = 8;

type InnerHashMap<'alloc, K, V, S> = hashbrown::HashMap<K, V, S, &'alloc Allocator>;

/// A hash map without `Drop` that stores data in arena allocator.
///
/// Uses [`FxHasher`] by default. The hasher can be customized via the `S` type parameter.
///
/// Just a thin wrapper around [`hashbrown::HashMap`], which disables the `Drop` implementation.
///
/// All APIs are the same, except create a [`HashMap`] with
/// either [`new_in`](HashMap::new_in) or [`with_capacity_in`](HashMap::with_capacity_in).
///
/// # No `Drop`s
///
/// Objects allocated into Oxc memory arenas are never [`Dropped`](Drop). Memory is released in bulk
/// when the allocator is dropped, without dropping the individual objects in the arena.
///
/// Therefore, it would produce a memory leak if you allocated [`Drop`] types into the arena
/// which own memory allocations outside the arena.
///
/// Static checks make this impossible to do. [`HashMap::new_in`] and all other methods which create
/// a [`HashMap`] will refuse to compile if either key or value is a [`Drop`] type.
///
/// [`FxHasher`]: rustc_hash::FxHasher
pub struct HashMap<'alloc, K, V, S = FxBuildHasher>(
    pub(crate) ManuallyDrop<InnerHashMap<'alloc, K, V, S>>,
);

enum AdaptiveHashMapRepr<'alloc, K: Unpin, V: Unpin> {
    Linear(Vec<'alloc, (K, V)>),
    Hashed(HashMap<'alloc, K, V>),
}

/// An arena-allocated map that uses linear search for small collections and
/// promotes itself to a hash table once it grows beyond `LINEAR_SEARCH_LIMIT`.
///
/// Unlike a small-map optimization, entries are never stored inline in this
/// value. The linear representation is an arena-allocated [`Vec`], so moving
/// the map does not move its entries. Promotion is one-way because arena
/// allocations are reclaimed in bulk and the hash table can be reused after
/// [`clear`](AdaptiveHashMap::clear).
pub struct AdaptiveHashMap<
    'alloc,
    K: Unpin,
    V: Unpin,
    const LINEAR_SEARCH_LIMIT: usize = DEFAULT_LINEAR_SEARCH_LIMIT,
> {
    allocator: &'alloc Allocator,
    repr: AdaptiveHashMapRepr<'alloc, K, V>,
}

impl<K, V, const LINEAR_SEARCH_LIMIT: usize> fmt::Debug
    for AdaptiveHashMap<'_, K, V, LINEAR_SEARCH_LIMIT>
where
    K: fmt::Debug + Unpin,
    V: fmt::Debug + Unpin,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            AdaptiveHashMapRepr::Linear(entries) => f
                .debug_map()
                .entries(entries.iter().map(|(key, value)| (key, value)))
                .finish(),
            AdaptiveHashMapRepr::Hashed(map) => fmt::Debug::fmt(map, f),
        }
    }
}

impl<'alloc, K, V, const LINEAR_SEARCH_LIMIT: usize>
    AdaptiveHashMap<'alloc, K, V, LINEAR_SEARCH_LIMIT>
where
    K: Unpin,
    V: Unpin,
{
    /// Creates an empty adaptive map in its linear representation.
    #[inline]
    pub fn new_in(allocator: &'alloc Allocator) -> Self {
        Self {
            allocator,
            repr: AdaptiveHashMapRepr::Linear(Vec::new_in(allocator)),
        }
    }

    /// Returns the number of entries in the map.
    #[inline]
    pub fn len(&self) -> usize {
        match &self.repr {
            AdaptiveHashMapRepr::Linear(entries) => entries.len(),
            AdaptiveHashMapRepr::Hashed(map) => map.len(),
        }
    }

    /// Returns `true` if the map contains no entries.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes all entries while retaining the current representation and its
    /// arena-allocated capacity for reuse.
    #[inline]
    pub fn clear(&mut self) {
        match &mut self.repr {
            AdaptiveHashMapRepr::Linear(entries) => entries.clear(),
            AdaptiveHashMapRepr::Hashed(map) => map.clear(),
        }
    }

    #[cfg(test)]
    pub(crate) fn uses_hash_table(&self) -> bool {
        matches!(self.repr, AdaptiveHashMapRepr::Hashed(_))
    }
}

impl<'alloc, K, V, const LINEAR_SEARCH_LIMIT: usize>
    AdaptiveHashMap<'alloc, K, V, LINEAR_SEARCH_LIMIT>
where
    K: Eq + Hash + Unpin,
    V: Unpin,
{
    /// Returns a reference to the value corresponding to `key`.
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        match &self.repr {
            AdaptiveHashMapRepr::Linear(entries) => entries
                .iter()
                .find_map(|(candidate, value)| (candidate == key).then_some(value)),
            AdaptiveHashMapRepr::Hashed(map) => map.get(key),
        }
    }

    /// Returns `true` if the map contains `key`.
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the key was already present, replaces its value and returns the old
    /// value. A new unique key promotes a full linear representation before it
    /// is inserted into the hash table.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.repr {
            AdaptiveHashMapRepr::Linear(entries) => {
                if let Some((_, existing)) =
                    entries.iter_mut().find(|(candidate, _)| candidate == &key)
                {
                    return Some(std::mem::replace(existing, value));
                }

                if entries.len() < LINEAR_SEARCH_LIMIT {
                    entries.push((key, value));
                    return None;
                }
            }
            AdaptiveHashMapRepr::Hashed(map) => return map.insert(key, value),
        }

        self.promote();
        let AdaptiveHashMapRepr::Hashed(map) = &mut self.repr else {
            unreachable!();
        };
        map.insert(key, value)
    }

    #[cold]
    fn promote(&mut self) {
        if matches!(self.repr, AdaptiveHashMapRepr::Hashed(_)) {
            return;
        }

        let old_repr = std::mem::replace(
            &mut self.repr,
            AdaptiveHashMapRepr::Linear(Vec::new_in(self.allocator)),
        );
        let AdaptiveHashMapRepr::Linear(entries) = old_repr else {
            unreachable!();
        };
        let mut map = HashMap::with_capacity_in(entries.len() + 1, self.allocator);
        for (key, value) in entries {
            let previous = map.insert(key, value);
            debug_assert!(previous.is_none());
        }
        self.repr = AdaptiveHashMapRepr::Hashed(map);
    }
}

impl<K: fmt::Debug, V: fmt::Debug, S> fmt::Debug for HashMap<'_, K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

/// SAFETY: Even though `Arena` is not `Sync`, we can make `HashMap<K, V>` `Sync` if both `K` and `V`
/// are `Sync` because:
///
/// 1. No public methods allow access to the `&Arena` that `HashMap` contains (in `hashbrown::HashMap`),
///    so user cannot illegally obtain 2 `&Arena`s on different threads via `HashMap`.
///
/// 2. All internal methods which access the `&Arena` take a `&mut self`.
///    `&mut HashMap` cannot be transferred across threads, and nor can an owned `HashMap`
///    (`HashMap` is not `Send`).
///    Therefore these methods taking `&mut self` can be sure they're not operating on a `HashMap`
///    which has been moved across threads.
///
/// Note: `HashMap` CANNOT be `Send`, even if `K` and `V` are `Send`, because that would allow 2 `HashMap`s
/// on different threads to both allocate into same arena simultaneously. `Arena` is not thread-safe,
/// and this would be undefined behavior.
///
/// ### Soundness holes
///
/// This is not actually fully sound. There are 2 holes I (@overlookmotel) am aware of:
///
/// 1. `allocator` method, which does allow access to the `&Arena` that `HashMap` contains.
/// 2. `Clone` impl on `hashbrown::HashMap`, which may perform allocations in the arena, given only a
///    `&self` reference.
///
/// [`HashMap::allocator`] prevents accidental access to the underlying method of `hashbrown::HashMap`,
/// and `clone` called on a `&HashMap` clones the `&HashMap` reference, not the `HashMap` itself (harmless).
/// But both can be accessed via explicit `Deref` (`hash_map.deref().allocator()` or `hash_map.deref().clone()`),
/// so we don't have complete soundness.
///
/// To close these holes we need to remove `Deref` and `DerefMut` impls on `HashMap`, and instead add
/// methods to `HashMap` itself which pass on calls to the inner `hashbrown::HashMap`.
///
/// TODO: Fix these holes.
/// TODO: Remove any other methods that currently allow performing allocations with only a `&self` reference.
unsafe impl<K: Sync, V: Sync, S: Sync> Sync for HashMap<'_, K, V, S> {}

// TODO: `IntoIter`, `Drain`, and other consuming iterators provided by `hashbrown` are `Drop`.
// Wrap them in `ManuallyDrop` to prevent that.

impl<'alloc, K, V, S> HashMap<'alloc, K, V, S> {
    /// Const assertions that `K` and `V` are not `Drop`.
    /// Must be referenced in all methods which create a `HashMap`.
    const ASSERT_K_AND_V_ARE_NOT_DROP: () = {
        assert!(
            !std::mem::needs_drop::<K>(),
            "Cannot create a HashMap<K, V> where K is a Drop type"
        );
        assert!(
            !std::mem::needs_drop::<V>(),
            "Cannot create a HashMap<K, V> where V is a Drop type"
        );
    };

    /// Creates an empty [`HashMap`] with the given hasher. It will be allocated with the given allocator.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate
    /// until it is first inserted into.
    #[inline(always)]
    pub fn with_hasher_in(hasher: S, allocator: &'alloc Allocator) -> Self {
        const { Self::ASSERT_K_AND_V_ARE_NOT_DROP };

        let inner = InnerHashMap::with_hasher_in(hasher, allocator);
        Self(ManuallyDrop::new(inner))
    }

    /// Creates an empty [`HashMap`] with the specified capacity and hasher.
    /// It will be allocated with the given allocator.
    ///
    /// The hash map will be able to hold at least capacity elements without reallocating.
    /// If capacity is 0, the hash map will not allocate.
    #[inline(always)]
    pub fn with_capacity_and_hasher_in(
        capacity: usize,
        hasher: S,
        allocator: &'alloc Allocator,
    ) -> Self {
        const { Self::ASSERT_K_AND_V_ARE_NOT_DROP };

        let inner = InnerHashMap::with_capacity_and_hasher_in(capacity, hasher, allocator);
        Self(ManuallyDrop::new(inner))
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    ///
    /// The map cannot be used after calling this. The iterator element type is `K`.
    #[inline(always)]
    pub fn into_keys(self) -> IntoKeys<K, V, &'alloc Allocator> {
        let inner = ManuallyDrop::into_inner(self.0);
        inner.into_keys()
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    ///
    /// The map cannot be used after calling this. The iterator element type is `V`.
    #[inline(always)]
    pub fn into_values(self) -> IntoValues<K, V, &'alloc Allocator> {
        let inner = ManuallyDrop::into_inner(self.0);
        inner.into_values()
    }

    /// Calling this method produces a compile-time panic.
    ///
    /// This method would be unsound, because [`HashMap`] is `Sync`, and the underlying allocator
    /// (`Arena`) is not `Sync`.
    ///
    /// This method exists only to block access as much as possible to the underlying
    /// `hashbrown::HashMap::allocator` method. That method can still be accessed via explicit `Deref`
    /// (`hash_map.deref().allocator()`), but that's unsound.
    ///
    /// We'll prevent access to it completely and remove this method as soon as we can.
    // TODO: Do that!
    #[allow(unfulfilled_lint_expectations)]
    #[expect(clippy::unused_self)]
    pub fn allocator(&self) -> &'alloc Allocator {
        const { panic!("This method cannot be called") };
        unreachable!();
    }
}

/// Methods for any hasher that implements [`Default`].
///
/// This includes [`FxBuildHasher`] and any custom hasher (e.g. `IdentBuildHasher`).
impl<'alloc, K, V, S: Default> HashMap<'alloc, K, V, S> {
    /// Creates an empty [`HashMap`]. It will be allocated with the given allocator.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate
    /// until it is first inserted into.
    #[inline(always)]
    pub fn new_in(allocator: &'alloc Allocator) -> Self {
        Self::with_hasher_in(S::default(), allocator)
    }

    /// Creates an empty [`HashMap`] with the specified capacity. It will be allocated with the given allocator.
    ///
    /// The hash map will be able to hold at least capacity elements without reallocating.
    /// If capacity is 0, the hash map will not allocate.
    #[inline(always)]
    pub fn with_capacity_in(capacity: usize, allocator: &'alloc Allocator) -> Self {
        Self::with_capacity_and_hasher_in(capacity, S::default(), allocator)
    }

    /// Create a new [`HashMap`] whose elements are taken from an iterator and
    /// allocated in the given `allocator`.
    ///
    /// This is behaviorally identical to [`FromIterator::from_iter`].
    #[inline]
    pub fn from_iter_in<I: IntoIterator<Item = (K, V)>>(
        iter: I,
        allocator: &'alloc Allocator,
    ) -> Self
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        const { Self::ASSERT_K_AND_V_ARE_NOT_DROP };

        let iter = iter.into_iter();

        // Use the iterator's lower size bound.
        // This follows `hashbrown::HashMap`'s `from_iter` implementation.
        //
        // This is a trade-off:
        // * Negative: If lower bound is too low, the `HashMap` may have to grow and reallocate during `for_each` loop.
        // * Positive: Avoids potential large over-allocation for iterators where upper bound may be a large over-estimate
        //   e.g. filter iterators.
        let capacity = iter.size_hint().0;
        let map = InnerHashMap::with_capacity_and_hasher_in(capacity, S::default(), allocator);
        // Wrap in `ManuallyDrop` *before* calling `for_each`, so compiler doesn't insert unnecessary code
        // to drop the `FxHashMap` in case of a panic in iterator's `next` method
        let mut map = ManuallyDrop::new(map);

        iter.for_each(|(k, v)| {
            map.insert(k, v);
        });

        Self(map)
    }
}

// Provide access to all `hashbrown::HashMap`'s methods via deref
impl<'alloc, K, V, S> Deref for HashMap<'alloc, K, V, S> {
    type Target = InnerHashMap<'alloc, K, V, S>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'alloc, K, V, S> DerefMut for HashMap<'alloc, K, V, S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut InnerHashMap<'alloc, K, V, S> {
        &mut self.0
    }
}

impl<'alloc, K, V, S> IntoIterator for HashMap<'alloc, K, V, S> {
    type IntoIter = IntoIter<K, V, &'alloc Allocator>;
    type Item = (K, V);

    /// Creates a consuming iterator, that is, one that moves each key-value pair out of the map
    /// in arbitrary order.
    ///
    /// The map cannot be used after calling this.
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        let inner = ManuallyDrop::into_inner(self.0);
        // TODO: `hashbrown::hash_map::IntoIter` is `Drop`.
        // Wrap it in `ManuallyDrop` to prevent that.
        inner.into_iter()
    }
}

impl<'alloc, 'i, K, V, S> IntoIterator for &'i HashMap<'alloc, K, V, S> {
    type IntoIter = <&'i InnerHashMap<'alloc, K, V, S> as IntoIterator>::IntoIter;
    type Item = (&'i K, &'i V);

    /// Creates an iterator over the entries of a `HashMap` in arbitrary order.
    ///
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// Return the same [`Iter`] struct as by the `iter` method on [`HashMap`].
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'alloc, 'i, K, V, S> IntoIterator for &'i mut HashMap<'alloc, K, V, S> {
    type IntoIter = <&'i mut InnerHashMap<'alloc, K, V, S> as IntoIterator>::IntoIter;
    type Item = (&'i K, &'i mut V);

    /// Creates an iterator over the entries of a `HashMap` in arbitrary order
    /// with mutable references to the values.
    ///
    /// The iterator element type is `(&'a K, &'a mut V)`.
    ///
    /// Return the same [`IterMut`] struct as by the `iter_mut` method on [`HashMap`].
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<K, V, S> PartialEq for HashMap<'_, K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<K, V, S> Eq for HashMap<'_, K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}

// Note: `Index` and `Extend` are implemented via `Deref`

#[cfg(test)]
mod tests {
    use super::{AdaptiveHashMap, Allocator};

    #[test]
    fn promotes_only_after_exceeding_unique_key_limit() {
        let allocator = Allocator::new();
        let mut map = AdaptiveHashMap::<u32, u32, 4>::new_in(&allocator);

        for key in 0..4 {
            assert_eq!(map.insert(key, key * 10), None);
        }
        assert!(!map.uses_hash_table());

        assert_eq!(map.insert(2, 25), Some(20));
        assert!(!map.uses_hash_table());
        assert_eq!(map.len(), 4);

        assert_eq!(map.insert(4, 40), None);
        assert!(map.uses_hash_table());
        assert_eq!(map.len(), 5);
        assert_eq!(map.get(&2), Some(&25));
        assert_eq!(map.get(&4), Some(&40));
    }

    #[test]
    fn clear_reuses_promoted_hash_table() {
        let allocator = Allocator::new();
        let mut map = AdaptiveHashMap::<u32, u32, 1>::new_in(&allocator);
        map.insert(0, 0);
        map.insert(1, 10);
        assert!(map.uses_hash_table());

        map.clear();
        assert!(map.is_empty());
        assert!(map.uses_hash_table());

        assert_eq!(map.insert(2, 20), None);
        assert_eq!(map.get(&2), Some(&20));
        assert!(map.uses_hash_table());
    }
}
