//! Arena-backed sequence optimized for parse-first, insert-rare workloads.
//!
//! Parsed values live in one linear arena [`Vec`]. Its common prefix uses the
//! high primary field of [`RadixId`] as a direct index. If that compact
//! prefix fills, authored values keep appending to the same vector and use a
//! high-bit-tagged dense overflow ID. Rare transformed values inserted after a
//! compact primary live in lazy two-level radix trees indexed by a separate
//! sparse `u32` vector.
//!
//! ```text
//! 31                         12 11                         2 1         0
//! +----------------------------+---------------------------+-----------+
//! |      primary index: 20     |    sibling key: 10        | reserved  |
//! +----------------------------+---------------------------+-----------+
//! ```
//!
//! The highest primary bit is zero for compact Radix IDs. When it is one, the
//! remaining 29 high bits encode a dense overflow index and the low two bits
//! are reserved and always zero for AST node IDs. This keeps IDs four bytes
//! and ordered while allowing valid authored input to outgrow the compact
//! Radix prefix.
//!
//! A zero sibling key addresses the primary linear value. A nonzero key uses
//! a five-bit root branch and, only when its low five bits are nonzero, a
//! second five-bit leaf slot. The two reserved low bits never affect primary
//! or sibling lookup.

use std::{fmt, hash::Hash, marker::PhantomData, ops::Index};

use crate::{Allocator, boxed::Box as ArenaBox, dense::NonMaxU32, vec::Vec};

const PRIMARY_BITS: u32 = 20;
const LOCAL_BITS: u32 = u32::BITS - PRIMARY_BITS;
const SIBLING_BITS: u32 = 10;
const RESERVED_BITS: u32 = LOCAL_BITS - SIBLING_BITS;
const SIBLING_SHIFT: u32 = RESERVED_BITS;
const RADIX_BITS: u32 = 5;
const RADIX_SIZE: usize = 1 << RADIX_BITS;
const SIBLING_MASK: u32 = (1 << SIBLING_BITS) - 1;
// The highest encoded bit distinguishes the rare dense authored overflow
// from compact primary/sibling IDs. Compact IDs remain byte-for-byte
// identical below this boundary.
const OVERFLOW_TAG: u32 = 1 << (u32::BITS - 1);
const COMPACT_PRIMARY_CAPACITY: usize = 1 << (PRIMARY_BITS - 1);
const OVERFLOW_INDEX_BITS: u32 = u32::BITS - RESERVED_BITS - 1;
const OVERFLOW_CAPACITY: usize = (1 << OVERFLOW_INDEX_BITS) - 1;

/// Stable compact identity for a value in [`RadixIndexArena`].
///
/// The `T` parameter is a domain marker: IDs for different AST stores are
/// distinct types even though they share the physical encoding. `u32::MAX` is
/// reserved as the `Option` niche so an optional ID stays four bytes.
///
/// Compact IDs whose [`sibling_key`](Self::sibling_key) is zero address the
/// primary parse-vector prefix directly. Other compact IDs address a rare
/// inserted sibling through a two-level radix tree. High-bit-tagged IDs address
/// the dense authored tail in that same parse vector.
pub struct RadixId<T> {
    inner: NonMaxU32,
    phantom_data: PhantomData<fn() -> T>,
}

impl<T> Clone for RadixId<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RadixId<T> {}

impl<T> fmt::Debug for RadixId<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> PartialEq for RadixId<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for RadixId<T> {}

impl<T> PartialOrd for RadixId<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for RadixId<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> Hash for RadixId<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> RadixId<T> {
    #[inline]
    fn from_parts(primary_index: usize, sibling_key: u16) -> Self {
        debug_assert!(primary_index < COMPACT_PRIMARY_CAPACITY);
        debug_assert!(u32::from(sibling_key) <= SIBLING_MASK);
        let encoded =
            ((primary_index as u32) << LOCAL_BITS) | (u32::from(sibling_key) << SIBLING_SHIFT);
        Self {
            inner: NonMaxU32::new(encoded).expect("u32::MAX is reserved for an invalid radix ID"),
            phantom_data: PhantomData,
        }
    }

    #[inline]
    fn from_overflow_index(index: usize) -> Self {
        debug_assert!(index < OVERFLOW_CAPACITY);
        let encoded = OVERFLOW_TAG | ((index as u32) << RESERVED_BITS);
        Self {
            inner: NonMaxU32::new(encoded).expect("u32::MAX is reserved for an invalid radix ID"),
            phantom_data: PhantomData,
        }
    }

    #[inline]
    fn from_primary_index(index: usize) -> Self {
        if index < COMPACT_PRIMARY_CAPACITY {
            Self::from_parts(index, 0)
        } else {
            Self::from_overflow_index(index - COMPACT_PRIMARY_CAPACITY)
        }
    }

    /// Returns the encoded ID as a `u32`.
    #[inline]
    pub const fn get(self) -> u32 {
        self.inner.get()
    }

    /// Returns the index of the owning primary value.
    #[inline]
    pub const fn primary_index(self) -> usize {
        (self.get() >> LOCAL_BITS) as usize
    }

    #[inline]
    const fn overflow_index(self) -> usize {
        ((self.get() & !OVERFLOW_TAG) >> RESERVED_BITS) as usize
    }

    /// Returns the ten-bit sibling key, or zero for a primary value.
    #[inline]
    pub const fn sibling_key(self) -> u16 {
        ((self.get() >> SIBLING_SHIFT) & SIBLING_MASK) as u16
    }

    /// Returns whether this ID addresses the primary linear vector.
    #[inline]
    pub const fn is_primary(self) -> bool {
        self.is_overflow() || self.sibling_key() == 0
    }

    /// Returns whether this ID addresses the dense authored overflow.
    #[inline]
    pub const fn is_overflow(self) -> bool {
        self.get() & OVERFLOW_TAG != 0
    }
}

/// The arena-facing ID protocol implemented by [`RadixId`].
///
/// Typed arenas use this hidden trait to construct and decode IDs without
/// exposing the raw encoding to other crates. Only [`RadixId<T>`] implements
/// it.
#[doc(hidden)]
pub trait RadixIdKey: Copy + Eq + Ord + Hash + fmt::Debug {
    #[doc(hidden)]
    fn from_parts(primary_index: usize, sibling_key: u16) -> Self;

    #[doc(hidden)]
    fn from_overflow_index(index: usize) -> Self;

    #[doc(hidden)]
    fn from_primary_index(index: usize) -> Self;

    #[doc(hidden)]
    fn get(self) -> u32;

    #[doc(hidden)]
    fn primary_index(self) -> usize;

    #[doc(hidden)]
    fn sibling_key(self) -> u16;

    #[doc(hidden)]
    fn is_primary(self) -> bool;

    #[doc(hidden)]
    fn is_overflow(self) -> bool;

    #[doc(hidden)]
    fn overflow_index(self) -> usize;
}

impl<T> RadixIdKey for RadixId<T> {
    #[inline]
    fn from_parts(primary_index: usize, sibling_key: u16) -> Self {
        Self::from_parts(primary_index, sibling_key)
    }

    #[inline]
    fn from_overflow_index(index: usize) -> Self {
        Self::from_overflow_index(index)
    }

    #[inline]
    fn from_primary_index(index: usize) -> Self {
        Self::from_primary_index(index)
    }

    #[inline]
    fn get(self) -> u32 {
        self.get()
    }

    #[inline]
    fn primary_index(self) -> usize {
        self.primary_index()
    }

    #[inline]
    fn sibling_key(self) -> u16 {
        self.sibling_key()
    }

    #[inline]
    fn is_primary(self) -> bool {
        self.is_primary()
    }

    #[inline]
    fn is_overflow(self) -> bool {
        self.is_overflow()
    }

    #[inline]
    fn overflow_index(self) -> usize {
        self.overflow_index()
    }
}

struct RadixTree<'arena, T> {
    root: Option<ArenaBox<'arena, RadixRoot<'arena, T>>>,
    #[cfg(test)]
    allocations: RadixAllocationCounts,
}

impl<'arena, T> RadixTree<'arena, T> {
    #[inline]
    fn new() -> Self {
        Self {
            root: None,
            #[cfg(test)]
            allocations: RadixAllocationCounts::default(),
        }
    }

    fn insert(&mut self, allocator: &'arena Allocator, key: u16, value: T) {
        debug_assert!(key != 0 && u32::from(key) <= SIBLING_MASK);
        let (high, low) = radix_parts(key);
        if self.root.is_none() {
            self.root = Some(ArenaBox::new_in(RadixRoot::new(), allocator));
            #[cfg(test)]
            {
                self.allocations.roots += 1;
            }
        }
        let root = self
            .root
            .as_deref_mut()
            .expect("radix tree always has a root");
        if low == 0 {
            let bit = 1_u32 << high;
            assert_eq!(
                root.direct_used & bit,
                0,
                "radix sibling key was already allocated"
            );
            root.direct[high] = Some(ArenaBox::new_in(value, allocator));
            root.direct_occupied |= bit;
            root.direct_used |= bit;
            root.occupied_branches |= bit;
            #[cfg(test)]
            {
                self.allocations.values += 1;
            }
        } else {
            if root.leaves[high].is_none() {
                root.leaves[high] = Some(ArenaBox::new_in(RadixLeaf::new(), allocator));
                #[cfg(test)]
                {
                    self.allocations.leaves += 1;
                }
            }
            let leaf = root.leaves[high]
                .as_deref_mut()
                .expect("nonzero-low radix key always has a leaf");
            let bit = 1_u32 << low;
            assert_eq!(
                leaf.used & bit,
                0,
                "radix sibling key was already allocated"
            );
            leaf.values[low] = Some(ArenaBox::new_in(value, allocator));
            leaf.occupied |= bit;
            leaf.used |= bit;
            root.occupied_branches |= 1_u32 << high;
            #[cfg(test)]
            {
                self.allocations.values += 1;
            }
        }
    }

    fn restore(&mut self, allocator: &'arena Allocator, key: u16, value: T) {
        let (high, low) = radix_parts(key);
        let root = self
            .root
            .as_deref_mut()
            .expect("a previously allocated key must have a radix root");
        if low == 0 {
            let bit = 1_u32 << high;
            assert_ne!(
                root.direct_used & bit,
                0,
                "restore requires an allocated key"
            );
            assert_eq!(
                root.direct_occupied & bit,
                0,
                "restore requires a retired key"
            );
            assert!(root.direct[high].is_none(), "retired key must be empty");
            root.direct[high] = Some(ArenaBox::new_in(value, allocator));
            root.direct_occupied |= bit;
            root.occupied_branches |= bit;
        } else {
            let leaf = root.leaves[high]
                .as_deref_mut()
                .expect("a previously allocated key must have a radix leaf");
            let bit = 1_u32 << low;
            assert_ne!(leaf.used & bit, 0, "restore requires an allocated key");
            assert_eq!(leaf.occupied & bit, 0, "restore requires a retired key");
            assert!(leaf.values[low].is_none(), "retired key must be empty");
            leaf.values[low] = Some(ArenaBox::new_in(value, allocator));
            leaf.occupied |= bit;
            root.occupied_branches |= 1_u32 << high;
        }
        #[cfg(test)]
        {
            self.allocations.values += 1;
        }
    }

    #[inline]
    fn get(&self, key: u16) -> Option<&T> {
        let (high, low) = radix_parts(key);
        let root = self.root.as_deref()?;
        if low == 0 {
            root.direct[high].as_deref()
        } else {
            root.leaves[high].as_deref()?.values[low].as_deref()
        }
    }

    #[inline]
    fn get_mut(&mut self, key: u16) -> Option<&mut T> {
        let (high, low) = radix_parts(key);
        let root = self.root.as_deref_mut()?;
        if low == 0 {
            root.direct[high].as_deref_mut()
        } else {
            root.leaves[high].as_deref_mut()?.values[low].as_deref_mut()
        }
    }

    #[inline]
    fn take(&mut self, key: u16, reusable: bool) -> Option<T> {
        let (high, low) = radix_parts(key);
        let root = self.root.as_deref_mut()?;
        let value = if low == 0 {
            let bit = 1_u32 << high;
            let value = ArenaBox::into_inner(root.direct[high].take()?);
            root.direct_occupied &= !bit;
            if reusable {
                root.direct_used &= !bit;
            }
            value
        } else {
            let leaf = root.leaves[high].as_deref_mut()?;
            let bit = 1_u32 << low;
            let value = ArenaBox::into_inner(leaf.values[low].take()?);
            leaf.occupied &= !bit;
            if reusable {
                leaf.used &= !bit;
            }
            value
        };
        let branch_bit = 1_u32 << high;
        let branch_occupied = root.direct_occupied & branch_bit != 0
            || root.leaves[high]
                .as_deref()
                .is_some_and(|leaf| leaf.occupied != 0);
        if branch_occupied {
            root.occupied_branches |= branch_bit;
        } else {
            root.occupied_branches &= !branch_bit;
        }
        Some(value)
    }

    #[inline]
    fn is_used(&self, key: u16) -> bool {
        let (high, low) = radix_parts(key);
        let Some(root) = self.root.as_deref() else {
            return false;
        };
        if low == 0 {
            root.direct_used & (1_u32 << high) != 0
        } else {
            root.leaves[high]
                .as_deref()
                .is_some_and(|leaf| leaf.used & (1_u32 << low) != 0)
        }
    }

    fn live_keys(&self) -> std::vec::Vec<u16> {
        let mut keys = std::vec::Vec::new();
        if let Some(mut iter) = self.iter() {
            while let Some((key, _)) = iter.next_enumerated() {
                keys.push(key);
            }
        }
        keys
    }

    fn next_key_after(&self, key: u16) -> Option<u16> {
        let root = self.root.as_deref()?;
        let (high, low) = radix_parts(key);
        if low < RADIX_SIZE - 1 {
            let occupied = root.leaves[high].as_deref().map_or(0, |leaf| leaf.occupied)
                & (u32::MAX << (low + 1));
            if occupied != 0 {
                let next_low = occupied.trailing_zeros() as usize;
                return Some((high * RADIX_SIZE + next_low) as u16);
            }
        }
        let later_branches = if high < RADIX_SIZE - 1 {
            root.occupied_branches & (u32::MAX << (high + 1))
        } else {
            0
        };
        let next_high = later_branches.trailing_zeros() as usize;
        (next_high < RADIX_SIZE).then(|| {
            let branch_bit = 1_u32 << next_high;
            if root.direct_occupied & branch_bit != 0 {
                return (next_high * RADIX_SIZE) as u16;
            }
            let next_low = root.leaves[next_high]
                .as_deref()
                .expect("an occupied non-direct branch has a leaf")
                .occupied
                .trailing_zeros() as usize;
            (next_high * RADIX_SIZE + next_low) as u16
        })
    }

    fn previous_key_before(&self, key: u16) -> Option<u16> {
        let root = self.root.as_deref()?;
        let (high, low) = radix_parts(key);
        if low != 0 {
            let occupied =
                root.leaves[high].as_deref().map_or(0, |leaf| leaf.occupied) & ((1_u32 << low) - 1);
            if occupied != 0 {
                let previous_low = (u32::BITS - 1 - occupied.leading_zeros()) as usize;
                return Some((high * RADIX_SIZE + previous_low) as u16);
            }
            if root.direct_occupied & (1_u32 << high) != 0 {
                return Some((high * RADIX_SIZE) as u16);
            }
        }
        let earlier_branches = if high == 0 {
            0
        } else {
            root.occupied_branches & ((1_u32 << high) - 1)
        };
        if earlier_branches == 0 {
            return None;
        }
        let previous_high = (u32::BITS - 1 - earlier_branches.leading_zeros()) as usize;
        let leaf_occupied = root.leaves[previous_high]
            .as_deref()
            .map_or(0, |leaf| leaf.occupied);
        if leaf_occupied != 0 {
            let previous_low = (u32::BITS - 1 - leaf_occupied.leading_zeros()) as usize;
            Some((previous_high * RADIX_SIZE + previous_low) as u16)
        } else {
            debug_assert_ne!(root.direct_occupied & (1_u32 << previous_high), 0);
            Some((previous_high * RADIX_SIZE) as u16)
        }
    }

    fn last_key(&self) -> Option<u16> {
        let root = self.root.as_deref()?;
        if root.occupied_branches == 0 {
            return None;
        }
        let high = (u32::BITS - 1 - root.occupied_branches.leading_zeros()) as usize;
        let leaf_occupied = root.leaves[high].as_deref().map_or(0, |leaf| leaf.occupied);
        if leaf_occupied != 0 {
            let low = (u32::BITS - 1 - leaf_occupied.leading_zeros()) as usize;
            Some((high * RADIX_SIZE + low) as u16)
        } else {
            debug_assert_ne!(root.direct_occupied & (1_u32 << high), 0);
            Some((high * RADIX_SIZE) as u16)
        }
    }

    fn for_each_enumerated_mut(&mut self, mut visit: impl FnMut(u16, &mut T)) {
        let Some(root) = self.root.as_deref_mut() else {
            return;
        };
        for high in 0..RADIX_SIZE {
            if let Some(value) = root.direct[high].as_deref_mut() {
                visit((high * RADIX_SIZE) as u16, value);
            }
            let Some(leaf) = root.leaves[high].as_deref_mut() else {
                continue;
            };
            for low in 1..RADIX_SIZE {
                if let Some(value) = leaf.values[low].as_deref_mut() {
                    visit((high * RADIX_SIZE + low) as u16, value);
                }
            }
        }
    }

    fn used_len(&self) -> usize {
        let Some(root) = self.root.as_deref() else {
            return 0;
        };
        root.direct_used.count_ones() as usize
            + root
                .leaves
                .iter()
                .flatten()
                .map(|leaf| leaf.used.count_ones() as usize)
                .sum::<usize>()
    }

    fn drain_live_retaining_ids(&mut self) -> std::vec::Vec<(u16, T)> {
        let keys = self.live_keys();
        keys.into_iter()
            .map(|key| {
                let value = self
                    .take(key, false)
                    .expect("a live radix key must contain a value");
                (key, value)
            })
            .collect()
    }

    fn iter(&self) -> Option<RadixTreeIter<'_, 'arena, T>> {
        let root = self.root.as_deref()?;
        if root.occupied_branches == 0 {
            return None;
        }
        Some(RadixTreeIter {
            root,
            branches: root.occupied_branches,
            slots: 0,
            branch: 0,
            direct_pending: false,
        })
    }

    #[cfg(test)]
    fn allocation_counts(&self) -> RadixAllocationCounts {
        self.allocations
    }
}

struct RadixRoot<'arena, T> {
    direct: [Option<ArenaBox<'arena, T>>; RADIX_SIZE],
    leaves: [Option<ArenaBox<'arena, RadixLeaf<'arena, T>>>; RADIX_SIZE],
    direct_occupied: u32,
    direct_used: u32,
    occupied_branches: u32,
}

impl<'arena, T> RadixRoot<'arena, T> {
    fn new() -> Self {
        Self {
            direct: [const { None }; RADIX_SIZE],
            leaves: [const { None }; RADIX_SIZE],
            direct_occupied: 0,
            direct_used: 0,
            occupied_branches: 0,
        }
    }
}

struct RadixLeaf<'arena, T> {
    // Slot zero belongs to `RadixRoot::direct` and is never used here.
    values: [Option<ArenaBox<'arena, T>>; RADIX_SIZE],
    occupied: u32,
    used: u32,
}

impl<'arena, T> RadixLeaf<'arena, T> {
    fn new() -> Self {
        Self {
            values: [const { None }; RADIX_SIZE],
            occupied: 0,
            used: 0,
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RadixAllocationCounts {
    roots: usize,
    leaves: usize,
    values: usize,
}

/// One ID change produced by a local sibling relabel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadixIdRemap<I> {
    pub old: I,
    pub new: I,
}

/// The result of inserting a value between two semantic neighbors.
#[derive(Debug)]
pub struct RadixInsertResult<I> {
    pub id: I,
    pub remaps: std::vec::Vec<RadixIdRemap<I>>,
}

/// A sequence with a linear primary store and lazy radix-tree insertions.
///
/// Parsing appends primary values with [`push_primary`](Self::push_primary),
/// which is the same amortized operation as pushing to an arena vector. Rare
/// inserted siblings are addressed by a nonzero ten-bit key and do not move
/// any primary value. After the compact prefix fills, dense overflow primary
/// values remain contiguous but reject optional local insertion. Iteration
/// emits each compact primary and its siblings, followed by the authored
/// overflow tail.
pub type RadixIndexArena<'arena, T> = TypedRadixIndexArena<'arena, T, RadixId<T>>;

/// A [`RadixIndexArena`] whose IDs are isolated to one domain-specific type.
///
/// ```compile_fail
/// use rocketcss_common::{Allocator, TypedRadixIndexArena, RadixId};
///
/// struct RuleMarker;
/// struct BlockMarker;
/// type RuleId = RadixId<RuleMarker>;
/// type BlockId = RadixId<BlockMarker>;
///
/// let allocator = Allocator::new();
/// let mut rules = TypedRadixIndexArena::<_, RuleId>::new_in(&allocator);
/// let blocks = TypedRadixIndexArena::<_, BlockId>::new_in(&allocator);
/// let rule = rules.push_primary(1_u8);
/// let _ = blocks.get(rule);
/// ```
pub struct TypedRadixIndexArena<'arena, T: Unpin, I: RadixIdKey> {
    allocator: &'arena Allocator,
    primary: Vec<'arena, T>,
    // These structure-of-arrays vectors share indices. Binary search touches
    // only compact primary IDs; tree pointers are loaded after a match.
    sibling_primary_indices: Vec<'arena, u32>,
    sibling_trees: Vec<'arena, RadixTree<'arena, T>>,
    len: u32,
    id: PhantomData<fn(I) -> I>,
}

impl<'arena, T: Unpin, I: RadixIdKey> TypedRadixIndexArena<'arena, T, I> {
    /// Creates an empty store in `allocator`.
    pub fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            allocator,
            primary: Vec::new_in(allocator),
            sibling_primary_indices: Vec::new_in(allocator),
            sibling_trees: Vec::new_in(allocator),
            len: 0,
            id: PhantomData,
        }
    }

    /// Creates an empty store with space for `capacity` primary values.
    pub fn with_capacity_in(capacity: usize, allocator: &'arena Allocator) -> Self {
        assert!(capacity <= COMPACT_PRIMARY_CAPACITY + OVERFLOW_CAPACITY);
        Self {
            allocator,
            primary: Vec::with_capacity_in(capacity, allocator),
            sibling_primary_indices: Vec::new_in(allocator),
            sibling_trees: Vec::new_in(allocator),
            len: 0,
            id: PhantomData,
        }
    }

    /// Returns the number of primary and inserted values.
    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns the number of values stored in the primary parse vector.
    #[inline]
    pub fn primary_len(&self) -> usize {
        self.primary.len()
    }

    /// Returns the direct primary ID for `index` when it exists.
    #[inline]
    pub fn primary_id(&self, index: usize) -> Option<I> {
        if index < self.primary.len() {
            Some(I::from_primary_index(index))
        } else {
            None
        }
    }

    /// Returns the next value ID in semantic arena order.
    pub fn next_id(&self, id: I) -> Option<I> {
        self.get(id)?;
        if id.is_overflow() {
            let index = COMPACT_PRIMARY_CAPACITY + id.overflow_index();
            return self.primary_id(index + 1);
        }

        let primary_index = id.primary_index();
        if let Some(key) = self
            .sibling_group_index(primary_index)
            .ok()
            .and_then(|group| self.sibling_trees[group].next_key_after(id.sibling_key()))
        {
            return Some(I::from_parts(primary_index, key));
        }
        self.primary_id(primary_index + 1)
    }

    /// Returns the previous value ID in semantic arena order.
    pub fn previous_id(&self, id: I) -> Option<I> {
        self.get(id)?;
        if !id.is_overflow() && id.sibling_key() != 0 {
            let primary_index = id.primary_index();
            return Some(
                self.sibling_group_index(primary_index)
                    .ok()
                    .and_then(|group| {
                        self.sibling_trees[group].previous_key_before(id.sibling_key())
                    })
                    .map_or_else(
                        || I::from_parts(primary_index, 0),
                        |key| I::from_parts(primary_index, key),
                    ),
            );
        }

        let primary_index = if id.is_overflow() {
            COMPACT_PRIMARY_CAPACITY + id.overflow_index()
        } else {
            id.primary_index()
        };
        let previous_primary = primary_index.checked_sub(1)?;
        if previous_primary < COMPACT_PRIMARY_CAPACITY
            && let Some(key) = self
                .sibling_group_index(previous_primary)
                .ok()
                .and_then(|group| self.sibling_trees[group].last_key())
        {
            return Some(I::from_parts(previous_primary, key));
        }
        self.primary_id(previous_primary)
    }

    /// Advances `steps` values in semantic arena order.
    ///
    /// Authored primary-only arenas use direct indexing. Once a rare sibling
    /// exists, the bounded radix cursor preserves semantic order.
    pub fn advance_id(&self, id: I, steps: u32) -> Option<I> {
        self.get(id)?;
        if steps == 0 {
            return Some(id);
        }
        if !self.has_siblings() && id.is_primary() {
            let index = if id.is_overflow() {
                COMPACT_PRIMARY_CAPACITY + id.overflow_index()
            } else {
                id.primary_index()
            };
            return self.primary_id(index.checked_add(steps as usize)?);
        }
        let mut current = id;
        for _ in 0..steps {
            current = self.next_id(current)?;
        }
        Some(current)
    }

    /// Mutably visits every value with its typed ID in semantic arena order.
    ///
    /// This is the mutation counterpart to [`iter_enumerated`](Self::iter_enumerated)
    /// for callers that need to update arena records without first collecting
    /// their IDs into a temporary allocation.
    pub fn for_each_enumerated_mut(&mut self, mut visit: impl FnMut(I, &mut T)) {
        let mut next_group = 0;
        let sibling_primary_indices = &self.sibling_primary_indices;
        let sibling_trees = &mut self.sibling_trees;
        for (primary_index, value) in self.primary.iter_mut().enumerate() {
            visit(I::from_primary_index(primary_index), value);
            if sibling_primary_indices.get(next_group).copied() == Some(primary_index as u32) {
                sibling_trees[next_group].for_each_enumerated_mut(|key, value| {
                    visit(I::from_parts(primary_index, key), value);
                });
                next_group += 1;
            }
        }
        debug_assert_eq!(next_group, sibling_primary_indices.len());
    }

    /// Iterates only authored primary values in their parse order.
    #[inline]
    pub fn primary_iter(&self) -> std::slice::Iter<'_, T> {
        self.primary.iter()
    }

    /// Iterates typed IDs and authored primary values in parse order.
    #[inline]
    pub fn primary_iter_enumerated(&self) -> PrimaryIterEnumerated<'_, T, I> {
        PrimaryIterEnumerated {
            primary: &self.primary,
            front: 0,
            back: self.primary_len(),
            id: PhantomData,
        }
    }

    /// Returns whether at least one inserted sibling is live.
    #[inline]
    pub fn has_siblings(&self) -> bool {
        self.len() != self.primary.len()
    }

    /// Returns whether the store contains no values.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns whether another primary value fits the compact representation.
    #[inline]
    pub fn can_push_primary(&self) -> bool {
        self.len < u32::MAX && self.primary.len() < COMPACT_PRIMARY_CAPACITY + OVERFLOW_CAPACITY
    }

    /// Appends a parsed value and returns its direct primary ID.
    ///
    /// # Panics
    ///
    /// Panics only after the compact and dense-overflow namespaces are both
    /// exhausted, or after `u32::MAX` total values.
    pub fn push_primary(&mut self, value: T) -> I {
        assert!(
            self.can_push_primary(),
            "RadixIndexArena primary capacity exhausted"
        );
        let id = I::from_primary_index(self.primary.len());
        self.primary.push(value);
        self.len += 1;
        id
    }

    /// Inserts a rare sibling after a primary value.
    ///
    /// Siblings are emitted in ascending `sibling_key` order. Key zero is
    /// reserved for the primary value; valid sibling keys are `1..=1023`.
    ///
    /// # Panics
    ///
    /// Panics for a non-primary owner, an invalid or duplicate key, an unknown
    /// primary ID, or `u32::MAX` total values.
    pub fn insert_sibling(&mut self, primary: I, sibling_key: u16, value: T) -> I {
        assert!(
            primary.is_primary() && !primary.is_overflow(),
            "sibling owner must be a compact primary ID"
        );
        assert!(
            sibling_key != 0 && u32::from(sibling_key) <= SIBLING_MASK,
            "radix sibling key must be in 1..=1023"
        );
        assert!(self.len < u32::MAX, "RadixIndexArena capacity exhausted");
        let primary_index = primary.primary_index();
        self.primary
            .get(primary_index)
            .expect("unknown primary radix ID");
        let group_index = match self.sibling_group_index(primary_index) {
            Ok(index) => index,
            Err(index) => {
                self.sibling_primary_indices
                    .insert(index, primary_index as u32);
                self.sibling_trees.insert(index, RadixTree::new());
                index
            }
        };
        self.sibling_trees[group_index].insert(self.allocator, sibling_key, value);
        self.len += 1;
        I::from_parts(primary_index, sibling_key)
    }

    /// Returns whether another live sibling can be represented below `primary`.
    ///
    /// This is the AST wrapper's preflight for choosing its overflow fallback
    /// before moving a value into the compact store.
    pub fn can_insert_sibling(&self, primary: I) -> bool {
        if !primary.is_primary()
            || primary.is_overflow()
            || self.primary.get(primary.primary_index()).is_none()
        {
            return false;
        }
        self.len < u32::MAX
            && self
                .sibling_group_index(primary.primary_index())
                .ok()
                .and_then(|group| self.sibling_trees.get(group))
                .is_none_or(|tree| tree.used_len() < SIBLING_MASK as usize)
    }

    /// Returns whether a compact ID can be assigned between two current
    /// semantic neighbors without reusing any retired ID.
    pub fn can_insert_between(&self, after: I, before: Option<I>) -> bool {
        if self.len == u32::MAX {
            return false;
        }
        let Some((primary_index, lower, upper, live_keys)) = self.insertion_point(after, before)
        else {
            return false;
        };
        if self
            .unused_key_between(primary_index, lower, upper)
            .is_some()
        {
            return true;
        }
        let Some(group) = self.sibling_group_index(primary_index).ok() else {
            return false;
        };
        let insertion_index = live_keys.partition_point(|key| *key <= lower);
        Self::relabel_keys_for_insert(&self.sibling_trees[group], &live_keys, insertion_index)
            .is_some()
    }

    /// Inserts a value between two current semantic neighbors.
    ///
    /// The new value is stored below `after`'s primary anchor. When no unused
    /// encoded key remains in the interval, this method relabels only the live
    /// siblings below that primary and returns every changed ID. Callers must
    /// repair persistent references in the same transaction before publishing
    /// the result.
    ///
    /// `before` is `None` only after the final value in the arena. This method
    /// validates that `after` and `before` are current semantic neighbors.
    ///
    /// # Panics
    ///
    /// Panics for invalid/non-neighbor IDs or when compact IDs are exhausted.
    /// AST wrappers must use [`can_insert_between`](Self::can_insert_between)
    /// to select their non-panicking overflow representation first.
    pub fn insert_between(
        &mut self,
        after: I,
        before: Option<I>,
        value: T,
    ) -> RadixInsertResult<I> {
        assert!(self.len < u32::MAX, "RadixIndexArena capacity exhausted");
        let (primary_index, lower, upper, live_keys) = self
            .insertion_point(after, before)
            .expect("after and before must be current base-ID semantic neighbors");

        if let Some(key) = self.unused_key_between(primary_index, lower, upper) {
            let primary = I::from_parts(primary_index, 0);
            let id = self.insert_sibling(primary, key, value);
            return RadixInsertResult {
                id,
                remaps: std::vec::Vec::new(),
            };
        }

        let group = self
            .sibling_group_index(primary_index)
            .expect("a gapless interval requires an existing sibling group");
        let insertion_index = live_keys.partition_point(|key| *key <= lower);
        let assigned_keys =
            Self::relabel_keys_for_insert(&self.sibling_trees[group], &live_keys, insertion_index)
                .expect("RadixIndexArena local sibling ID space exhausted");
        self.relabel_and_insert(primary_index, insertion_index, assigned_keys, value)
    }

    /// Removes an inserted sibling and makes its key reusable.
    ///
    /// Primary parsed values cannot be removed. AST nodes whose identities may
    /// still be referenced must use [`retire_sibling`](Self::retire_sibling)
    /// instead.
    pub fn remove_sibling(&mut self, id: I) -> Option<T> {
        if id.is_primary() {
            return None;
        }
        let group_index = self.sibling_group_index(id.primary_index()).ok()?;
        let value = self
            .sibling_trees
            .get_mut(group_index)?
            .take(id.sibling_key(), true)?;
        self.len -= 1;
        Some(value)
    }

    /// Retires an inserted sibling without making its encoded ID reusable.
    pub fn retire_sibling(&mut self, id: I) -> Option<T> {
        if id.is_primary() {
            return None;
        }
        let group_index = self.sibling_group_index(id.primary_index()).ok()?;
        let value = self
            .sibling_trees
            .get_mut(group_index)?
            .take(id.sibling_key(), false)?;
        self.len -= 1;
        Some(value)
    }

    /// Resolves an ID to its value.
    #[inline]
    pub fn get(&self, id: I) -> Option<&T> {
        if id.is_overflow() {
            self.primary
                .get(COMPACT_PRIMARY_CAPACITY + id.overflow_index())
        } else if id.is_primary() {
            self.primary.get(id.primary_index())
        } else {
            let group_index = self.sibling_group_index(id.primary_index()).ok()?;
            self.sibling_trees.get(group_index)?.get(id.sibling_key())
        }
    }

    /// Mutably resolves an ID to its value.
    #[inline]
    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        if id.is_overflow() {
            self.primary
                .get_mut(COMPACT_PRIMARY_CAPACITY + id.overflow_index())
        } else if id.is_primary() {
            self.primary.get_mut(id.primary_index())
        } else {
            let group_index = self.sibling_group_index(id.primary_index()).ok()?;
            self.sibling_trees
                .get_mut(group_index)?
                .get_mut(id.sibling_key())
        }
    }

    /// Iterates all values in semantic order.
    #[inline]
    pub fn iter(&self) -> Iter<'_, 'arena, T> {
        let kind = if !self.has_siblings() {
            IterKind::Primary(self.primary_iter())
        } else {
            IterKind::Expanded(self.semantic_iter())
        };
        Iter {
            kind,
            remaining: self.len,
        }
    }

    /// Iterates primary values and inserted siblings in semantic order.
    ///
    /// Callers that already know siblings exist can use this method to keep the
    /// primary/semantic dispatch outside the per-value loop.
    #[inline]
    pub fn semantic_iter(&self) -> SemanticIter<'_, 'arena, T> {
        let primary_end = self
            .sibling_primary_indices
            .first()
            .map_or(self.primary.len(), |&primary_index| {
                primary_index as usize + 1
            });
        SemanticIter {
            primary: &self.primary,
            next_primary: primary_end,
            sibling_primary_indices: &self.sibling_primary_indices,
            sibling_trees: &self.sibling_trees,
            next_group: 0,
            current: SemanticSegment::Primary(self.primary[..primary_end].iter()),
        }
    }

    /// Iterates typed IDs and values in semantic order.
    #[inline]
    pub fn iter_enumerated(&self) -> SemanticIterEnumerated<'_, 'arena, T, I> {
        self.semantic_iter_enumerated()
    }

    /// Iterates typed IDs and values by merging primary slices with sparse
    /// sibling radix trees.
    #[inline]
    pub fn semantic_iter_enumerated(&self) -> SemanticIterEnumerated<'_, 'arena, T, I> {
        let primary_end = self
            .sibling_primary_indices
            .first()
            .map_or(self.primary.len(), |&primary_index| {
                primary_index as usize + 1
            });
        SemanticIterEnumerated {
            primary: &self.primary,
            next_primary: primary_end,
            current_primary: 0,
            sibling_primary_indices: &self.sibling_primary_indices,
            sibling_trees: &self.sibling_trees,
            next_group: 0,
            sibling_primary: 0,
            current: SemanticSegment::Primary(self.primary[..primary_end].iter()),
            remaining: self.len,
            id: PhantomData,
        }
    }

    #[inline]
    fn sibling_group_index(&self, primary_index: usize) -> Result<usize, usize> {
        self.sibling_primary_indices
            .binary_search(&(primary_index as u32))
    }

    fn insertion_point(
        &self,
        after: I,
        before: Option<I>,
    ) -> Option<(usize, u16, u16, std::vec::Vec<u16>)> {
        if after.is_overflow() || self.get(after).is_none() {
            return None;
        }
        if before.is_some_and(|before| self.get(before).is_none()) {
            return None;
        }

        let primary_index = after.primary_index();
        let lower = after.sibling_key();
        let upper = match before {
            Some(before)
                if before.primary_index() == primary_index && before.sibling_key() > lower =>
            {
                before.sibling_key()
            }
            Some(before)
                if before.is_overflow()
                    && before.overflow_index() == 0
                    && primary_index + 1 == COMPACT_PRIMARY_CAPACITY =>
            {
                (SIBLING_MASK + 1) as u16
            }
            Some(before) if before.is_primary() && before.primary_index() == primary_index + 1 => {
                (SIBLING_MASK + 1) as u16
            }
            None if self.primary.len() <= COMPACT_PRIMARY_CAPACITY
                && primary_index + 1 == self.primary.len() =>
            {
                (SIBLING_MASK + 1) as u16
            }
            _ => return None,
        };

        let live_keys = self
            .sibling_group_index(primary_index)
            .ok()
            .and_then(|group| self.sibling_trees.get(group))
            .map_or_else(std::vec::Vec::new, RadixTree::live_keys);
        let no_live_between = live_keys.iter().all(|&key| key <= lower || key >= upper);
        let after_is_final =
            upper != (SIBLING_MASK + 1) as u16 || live_keys.iter().all(|&key| key <= lower);
        (no_live_between && after_is_final).then_some((primary_index, lower, upper, live_keys))
    }

    fn unused_key_between(&self, primary_index: usize, lower: u16, upper: u16) -> Option<u16> {
        if upper <= lower + 1 {
            return None;
        }
        let tree = self
            .sibling_group_index(primary_index)
            .ok()
            .and_then(|group| self.sibling_trees.get(group));
        let middle = lower + (upper - lower) / 2;
        let width = upper - lower - 1;
        for distance in 0..width {
            let lower_candidate = middle.checked_sub(distance);
            if let Some(candidate) = lower_candidate
                && candidate > lower
                && candidate < upper
                && tree.is_none_or(|tree| !tree.is_used(candidate))
            {
                return Some(candidate);
            }
            let upper_candidate = middle + distance;
            if upper_candidate > lower
                && upper_candidate < upper
                && tree.is_none_or(|tree| !tree.is_used(upper_candidate))
            {
                return Some(upper_candidate);
            }
        }
        None
    }

    fn relabel_keys_for_insert(
        tree: &RadixTree<'_, T>,
        live_keys: &[u16],
        insertion_index: usize,
    ) -> Option<std::vec::Vec<u16>> {
        let mut old_keys = live_keys
            .iter()
            .copied()
            .map(Some)
            .collect::<std::vec::Vec<_>>();
        old_keys.insert(insertion_index, None);
        let mut assigned_keys = std::vec::Vec::with_capacity(old_keys.len());
        let mut previous = 0_u16;
        for old_key in old_keys {
            let retained = match old_key {
                Some(old_key) if old_key > previous => Some(old_key),
                _ => None,
            };
            let unused =
                ((previous + 1)..=SIBLING_MASK as u16).find(|&candidate| !tree.is_used(candidate));
            let key = match (retained, unused) {
                (Some(retained), Some(unused)) => retained.min(unused),
                (Some(retained), None) => retained,
                (None, Some(unused)) => unused,
                (None, None) => return None,
            };
            assigned_keys.push(key);
            previous = key;
        }
        Some(assigned_keys)
    }

    fn relabel_and_insert(
        &mut self,
        primary_index: usize,
        insertion_index: usize,
        assigned_keys: std::vec::Vec<u16>,
        value: T,
    ) -> RadixInsertResult<I> {
        let group = self
            .sibling_group_index(primary_index)
            .expect("relabeling requires an existing sibling group");
        let tree = &mut self.sibling_trees[group];
        let mut entries = tree.drain_live_retaining_ids();
        entries.insert(insertion_index, (0, value));
        let mut remaps = std::vec::Vec::with_capacity(entries.len().saturating_sub(1));
        let mut inserted = None;
        for (index, ((old_key, value), key)) in entries.into_iter().zip(assigned_keys).enumerate() {
            if index != insertion_index && old_key == key {
                tree.restore(self.allocator, key, value);
            } else {
                tree.insert(self.allocator, key, value);
            }
            let new = I::from_parts(primary_index, key);
            if index == insertion_index {
                inserted = Some(new);
            } else if old_key != key {
                remaps.push(RadixIdRemap {
                    old: I::from_parts(primary_index, old_key),
                    new,
                });
            }
        }
        self.len += 1;
        RadixInsertResult {
            id: inserted.expect("the inserted entry is part of the rebuilt sibling tree"),
            remaps,
        }
    }
}

impl<T: Unpin, I: RadixIdKey> Index<I> for TypedRadixIndexArena<'_, T, I> {
    type Output = T;

    #[inline]
    fn index(&self, id: I) -> &Self::Output {
        self.get(id).expect("RadixIndexArena ID does not exist")
    }
}

/// Iterator over a [`RadixIndexArena`] in semantic order.
pub struct Iter<'tree, 'arena, T: Unpin> {
    kind: IterKind<'tree, 'arena, T>,
    remaining: u32,
}

enum IterKind<'tree, 'arena, T: Unpin> {
    Primary(std::slice::Iter<'tree, T>),
    Expanded(SemanticIter<'tree, 'arena, T>),
}

/// Iterator over authored IDs and values in parse order.
pub struct PrimaryIterEnumerated<'tree, T, I: RadixIdKey> {
    primary: &'tree [T],
    front: usize,
    back: usize,
    id: PhantomData<fn(I) -> I>,
}

impl<'tree, T, I: RadixIdKey> PrimaryIterEnumerated<'tree, T, I> {
    #[inline]
    fn item(&self, index: usize) -> (I, &'tree T) {
        (I::from_primary_index(index), &self.primary[index])
    }
}

impl<'tree, T, I: RadixIdKey> Iterator for PrimaryIterEnumerated<'tree, T, I> {
    type Item = (I, &'tree T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let index = self.front;
        self.front += 1;
        Some(self.item(index))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.back - self.front;
        (remaining, Some(remaining))
    }
}

impl<T, I: RadixIdKey> DoubleEndedIterator for PrimaryIterEnumerated<'_, T, I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        Some(self.item(self.back))
    }
}

impl<T, I: RadixIdKey> ExactSizeIterator for PrimaryIterEnumerated<'_, T, I> {}
impl<T, I: RadixIdKey> std::iter::FusedIterator for PrimaryIterEnumerated<'_, T, I> {}

/// Iterator that merges primary values with sparse sibling radix trees.
pub struct SemanticIter<'tree, 'arena, T: Unpin> {
    primary: &'tree [T],
    next_primary: usize,
    sibling_primary_indices: &'tree [u32],
    sibling_trees: &'tree [RadixTree<'arena, T>],
    next_group: usize,
    current: SemanticSegment<'tree, 'arena, T>,
}

/// Iterator over typed IDs and values in semantic order.
pub struct SemanticIterEnumerated<'tree, 'arena, T: Unpin, I: RadixIdKey> {
    primary: &'tree [T],
    next_primary: usize,
    current_primary: usize,
    sibling_primary_indices: &'tree [u32],
    sibling_trees: &'tree [RadixTree<'arena, T>],
    next_group: usize,
    sibling_primary: usize,
    current: SemanticSegment<'tree, 'arena, T>,
    remaining: u32,
    id: PhantomData<fn(I) -> I>,
}

enum SemanticSegment<'tree, 'arena, T> {
    Primary(std::slice::Iter<'tree, T>),
    Siblings(RadixTreeIter<'tree, 'arena, T>),
    Done,
}

impl<'tree, 'arena: 'tree, T: Unpin> Iterator for Iter<'tree, 'arena, T> {
    type Item = &'tree T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = match &mut self.kind {
            IterKind::Primary(primary) => primary.next()?,
            IterKind::Expanded(expanded) => expanded.next()?,
        };
        self.remaining -= 1;
        Some(value)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining as usize;
        (remaining, Some(remaining))
    }
}

impl<T: Unpin> ExactSizeIterator for Iter<'_, '_, T> {}
impl<T: Unpin> std::iter::FusedIterator for Iter<'_, '_, T> {}

impl<'tree, 'arena: 'tree, T: Unpin> Iterator for SemanticIter<'tree, 'arena, T> {
    type Item = &'tree T;

    #[inline]
    fn next(&mut self) -> Option<&'tree T> {
        loop {
            match &mut self.current {
                SemanticSegment::Primary(primary) => {
                    if let Some(value) = primary.next() {
                        return Some(value);
                    }
                }
                SemanticSegment::Siblings(siblings) => {
                    if let Some(value) = siblings.next() {
                        return Some(value);
                    }
                }
                SemanticSegment::Done => return None,
            }

            self.advance_segment();
        }
    }
}

impl<'tree, 'arena: 'tree, T: Unpin> SemanticIter<'tree, 'arena, T> {
    #[inline]
    fn advance_segment(&mut self) {
        match self.current {
            SemanticSegment::Primary(_) => {
                let Some(&primary_index) = self.sibling_primary_indices.get(self.next_group) else {
                    self.current = SemanticSegment::Done;
                    return;
                };
                debug_assert_eq!(self.next_primary, primary_index as usize + 1);
                let tree = &self.sibling_trees[self.next_group];
                self.next_group += 1;
                if let Some(siblings) = tree.iter() {
                    self.current = SemanticSegment::Siblings(siblings);
                    return;
                }
            }
            SemanticSegment::Siblings(_) => {}
            SemanticSegment::Done => return,
        }

        let primary_end = self
            .sibling_primary_indices
            .get(self.next_group)
            .map_or(self.primary.len(), |&primary_index| {
                primary_index as usize + 1
            });
        let primary = self.primary[self.next_primary..primary_end].iter();
        self.next_primary = primary_end;
        self.current = SemanticSegment::Primary(primary);
    }
}

impl<'tree, 'arena: 'tree, T: Unpin, I: RadixIdKey> Iterator
    for SemanticIterEnumerated<'tree, 'arena, T, I>
{
    type Item = (I, &'tree T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.current {
                SemanticSegment::Primary(primary) => {
                    if let Some(value) = primary.next() {
                        let id = I::from_primary_index(self.current_primary);
                        self.current_primary += 1;
                        self.remaining -= 1;
                        return Some((id, value));
                    }
                }
                SemanticSegment::Siblings(siblings) => {
                    if let Some((key, value)) = siblings.next_enumerated() {
                        let id = I::from_parts(self.sibling_primary, key);
                        self.remaining -= 1;
                        return Some((id, value));
                    }
                }
                SemanticSegment::Done => return None,
            }

            self.advance_segment();
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining as usize;
        (remaining, Some(remaining))
    }
}

impl<T: Unpin, I: RadixIdKey> ExactSizeIterator for SemanticIterEnumerated<'_, '_, T, I> {}
impl<T: Unpin, I: RadixIdKey> std::iter::FusedIterator for SemanticIterEnumerated<'_, '_, T, I> {}

impl<'tree, 'arena: 'tree, T: Unpin, I: RadixIdKey> SemanticIterEnumerated<'tree, 'arena, T, I> {
    #[inline]
    fn advance_segment(&mut self) {
        match self.current {
            SemanticSegment::Primary(_) => {
                let Some(&primary_index) = self.sibling_primary_indices.get(self.next_group) else {
                    self.current = SemanticSegment::Done;
                    return;
                };
                debug_assert_eq!(self.next_primary, primary_index as usize + 1);
                let tree = &self.sibling_trees[self.next_group];
                self.next_group += 1;
                if let Some(siblings) = tree.iter() {
                    self.sibling_primary = primary_index as usize;
                    self.current = SemanticSegment::Siblings(siblings);
                    return;
                }
            }
            SemanticSegment::Siblings(_) => {}
            SemanticSegment::Done => return,
        }

        let primary_start = self.next_primary;
        let primary_end = self
            .sibling_primary_indices
            .get(self.next_group)
            .map_or(self.primary.len(), |&primary_index| {
                primary_index as usize + 1
            });
        self.current_primary = primary_start;
        self.next_primary = primary_end;
        self.current = SemanticSegment::Primary(self.primary[primary_start..primary_end].iter());
    }
}

impl<'tree, 'arena: 'tree, T: Unpin, I: RadixIdKey> IntoIterator
    for &'tree TypedRadixIndexArena<'arena, T, I>
{
    type Item = &'tree T;
    type IntoIter = Iter<'tree, 'arena, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct RadixTreeIter<'tree, 'arena, T> {
    root: &'tree RadixRoot<'arena, T>,
    branches: u32,
    slots: u32,
    branch: usize,
    direct_pending: bool,
}

impl<'tree, 'arena, T> RadixTreeIter<'tree, 'arena, T> {
    #[inline]
    fn next_enumerated(&mut self) -> Option<(u16, &'tree T)> {
        loop {
            if self.direct_pending {
                self.direct_pending = false;
                let value = self.root.direct[self.branch]
                    .as_deref()
                    .expect("occupied direct branch always contains a value");
                let key = (self.branch << RADIX_BITS) as u16;
                return Some((key, value));
            }

            if self.slots != 0 {
                let slot = self.slots.trailing_zeros() as usize;
                self.slots &= self.slots - 1;
                let leaf = self.root.leaves[self.branch]
                    .as_deref()
                    .expect("occupied leaf branch always contains a leaf");
                let value = leaf.values[slot]
                    .as_deref()
                    .expect("occupied leaf slot always contains a value");
                let key = ((self.branch << RADIX_BITS) | slot) as u16;
                return Some((key, value));
            }

            let branch = self.branches.trailing_zeros() as usize;
            if branch == u32::BITS as usize {
                return None;
            }
            self.branches &= self.branches - 1;
            self.branch = branch;
            let branch_bit = 1_u32 << branch;
            self.direct_pending = self.root.direct_occupied & branch_bit != 0;
            self.slots = self.root.leaves[branch]
                .as_deref()
                .map_or(0, |leaf| leaf.occupied & !1_u32);
        }
    }
}

impl<'tree, 'arena, T> Iterator for RadixTreeIter<'tree, 'arena, T> {
    type Item = &'tree T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_enumerated().map(|(_, value)| value)
    }
}

#[inline]
fn radix_parts(key: u16) -> (usize, usize) {
    let key = usize::from(key);
    (key >> RADIX_BITS, key & (RADIX_SIZE - 1))
}

#[cfg(test)]
mod tests {
    use super::{
        COMPACT_PRIMARY_CAPACITY, LOCAL_BITS, OVERFLOW_CAPACITY, RadixAllocationCounts, RadixId,
        RadixIndexArena, RadixLeaf, RadixRoot, SIBLING_MASK, TypedRadixIndexArena,
    };
    use crate::Allocator;

    struct TestRuleMarker;
    type TestRuleId = RadixId<TestRuleMarker>;

    #[test]
    fn primary_ids_directly_address_parse_vector() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);

        let first = values.push_primary(10);
        let second = values.push_primary(20);

        assert_eq!(first.get(), 0);
        assert_eq!(second.get(), 1 << LOCAL_BITS);
        assert_eq!(first.primary_index(), 0);
        assert_eq!(second.primary_index(), 1);
        assert_eq!(values[first], 10);
        assert_eq!(values[second], 20);
    }

    #[test]
    fn siblings_use_direct_and_leaf_storage_and_iterate_by_key() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let first = values.push_primary(0);
        let second = values.push_primary(4);
        values.push_primary(6);

        assert!(values.sibling_primary_indices.is_empty());
        assert!(values.sibling_trees.is_empty());

        let high_branch = values.insert_sibling(first, 512, 3);
        let low_branch = values.insert_sibling(first, 1, 1);
        let next_leaf = values.insert_sibling(first, 32, 2);
        values.insert_sibling(second, 1, 5);

        assert_eq!(values.sibling_primary_indices.len(), 2);
        assert_eq!(values.sibling_trees.len(), 2);
        assert_eq!(high_branch.sibling_key(), 512);
        assert_eq!(values[low_branch], 1);
        assert_eq!(values[next_leaf], 2);
        assert_eq!(values[high_branch], 3);
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [0, 1, 2, 3, 4, 5, 6]
        );
        assert_eq!(values.len(), 7);
        assert_eq!(values.primary_len(), 3);
        assert_eq!(second.primary_index(), 1);
    }

    #[test]
    fn first_level_direct_value_does_not_allocate_a_leaf() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        let direct = values.insert_sibling(primary, 512, 3);

        assert_eq!(values.get(direct), Some(&3));
        assert_eq!(
            values.sibling_trees[0].allocation_counts(),
            RadixAllocationCounts {
                roots: 1,
                leaves: 0,
                values: 1,
            }
        );
        let root = values.sibling_trees[0].root.as_ref().unwrap();
        assert!(root.direct[16].is_some());
        assert!(root.leaves.iter().all(|leaf| leaf.is_none()));
    }

    #[test]
    fn nonzero_low_value_allocates_only_the_matching_leaf() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        values.insert_sibling(primary, 513, 4);

        assert_eq!(
            values.sibling_trees[0].allocation_counts(),
            RadixAllocationCounts {
                roots: 1,
                leaves: 1,
                values: 1,
            }
        );
        let root = values.sibling_trees[0].root.as_ref().unwrap();
        assert!(root.direct.iter().all(|value| value.is_none()));
        assert!(root.leaves[16].is_some());
        assert!(root.leaves.iter().filter(|leaf| leaf.is_some()).count() == 1);
    }

    #[test]
    fn direct_and_leaf_values_coexist_at_one_high_branch() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        let direct = values.insert_sibling(primary, 512, 2);
        let leaf = values.insert_sibling(primary, 513, 3);

        assert_eq!(values.get(direct), Some(&2));
        assert_eq!(values.get(leaf), Some(&3));
        assert_eq!(
            values
                .iter_enumerated()
                .map(|(id, value)| (id.sibling_key(), *value))
                .collect::<std::vec::Vec<_>>(),
            [(0, 0), (512, 2), (513, 3)]
        );
        assert_eq!(
            values.sibling_trees[0].allocation_counts(),
            RadixAllocationCounts {
                roots: 1,
                leaves: 1,
                values: 2,
            }
        );
    }

    #[test]
    fn iteration_orders_low_boundary_keys_numerically() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        values.insert_sibling(primary, 511, 511);
        values.insert_sibling(primary, 513, 513);
        values.insert_sibling(primary, 512, 512);

        assert_eq!(
            values
                .iter_enumerated()
                .map(|(id, value)| (id.sibling_key(), *value))
                .collect::<std::vec::Vec<_>>(),
            [(0, 0), (511, 511), (512, 512), (513, 513)]
        );
    }

    #[test]
    fn direct_and_leaf_mutation_paths_preserve_masks_and_reuse_rules() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        let direct = values.insert_sibling(primary, 512, 2);
        let leaf = values.insert_sibling(primary, 513, 3);

        *values.get_mut(direct).unwrap() = 20;
        *values.get_mut(leaf).unwrap() = 30;
        assert_eq!(values.remove_sibling(direct), Some(20));
        let reused_direct = values.insert_sibling(primary, 512, 200);
        assert_eq!(reused_direct, direct);
        assert_eq!(values.get(reused_direct), Some(&200));

        assert_eq!(values.retire_sibling(leaf), Some(30));
        assert_eq!(values.get(leaf), None);
        assert!(values.sibling_trees[0].is_used(513));
        assert_eq!(values.get(reused_direct), Some(&200));
    }

    #[test]
    fn an_empty_leaf_does_not_hide_a_live_direct_value() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        let leaf = values.insert_sibling(primary, 513, 3);
        assert_eq!(values.retire_sibling(leaf), Some(3));
        let direct = values.insert_sibling(primary, 512, 2);

        let root = values.sibling_trees[0].root.as_ref().unwrap();
        assert_eq!(root.direct_occupied, 1 << 16);
        assert_eq!(root.leaves[16].as_deref().unwrap().occupied, 0);
        assert_eq!(values.get(direct), Some(&2));
        assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [0, 2]);
    }

    #[test]
    fn retired_direct_id_remains_unavailable() {
        use std::panic::AssertUnwindSafe;

        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        let direct = values.insert_sibling(primary, 512, 2);
        assert_eq!(values.retire_sibling(direct), Some(2));
        assert!(values.sibling_trees[0].is_used(512));

        let duplicate =
            std::panic::catch_unwind(AssertUnwindSafe(|| values.insert_sibling(primary, 512, 4)));
        assert!(duplicate.is_err());
        assert_eq!(values.get(direct), None);
    }

    #[test]
    fn relabeling_restores_an_unchanged_direct_value() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        for key in 1..=514 {
            values.insert_sibling(primary, key, key);
        }

        let result = values.insert_between(
            RadixId::<u16>::from_parts(0, 513),
            Some(RadixId::<u16>::from_parts(0, 514)),
            10_000,
        );

        assert_eq!(values.get(RadixId::<u16>::from_parts(0, 512)), Some(&512));
        assert!(
            result
                .remaps
                .iter()
                .all(|remap| remap.old.sibling_key() != 512)
        );
        assert_eq!(values.get(result.id), Some(&10_000));
    }

    #[test]
    fn relabeling_can_move_a_direct_value_into_a_leaf() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0_u16);
        for key in 1..=512 {
            values.insert_sibling(primary, key, key);
        }
        let old_direct = RadixId::<u16>::from_parts(0, 512);

        let result =
            values.insert_between(RadixId::<u16>::from_parts(0, 511), Some(old_direct), 10_000);

        let remap = result
            .remaps
            .iter()
            .find(|remap| remap.old == old_direct)
            .copied()
            .expect("the direct value crosses into a leaf during relabeling");
        assert_eq!(remap.new.sibling_key(), 514);
        assert_eq!(values.get(old_direct), None);
        assert_eq!(values.get(remap.new), Some(&512));
        assert_eq!(values.get(result.id), Some(&10_000));
        assert!(values.sibling_trees[0].root.as_ref().unwrap().direct[16].is_none());
    }

    #[test]
    fn radix_pages_do_not_embed_payload_storage() {
        assert_eq!(
            std::mem::size_of::<RadixRoot<'static, u8>>(),
            std::mem::size_of::<RadixRoot<'static, [u8; 4096]>>()
        );
        assert_eq!(
            std::mem::size_of::<RadixLeaf<'static, u8>>(),
            std::mem::size_of::<RadixLeaf<'static, [u8; 4096]>>()
        );
    }

    #[test]
    fn retired_sibling_id_is_not_reused_by_normal_insertion() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(1);
        let sibling = values.insert_sibling(primary, 17, 2);

        assert_eq!(values.retire_sibling(sibling), Some(2));
        assert_eq!(values.get(sibling), None);
        assert_eq!(
            values
                .semantic_iter()
                .copied()
                .collect::<std::vec::Vec<_>>(),
            [1]
        );
        let replacement = values.insert_between(primary, None, 3).id;
        assert_ne!(replacement, sibling);
        *values.get_mut(replacement).unwrap() = 4;

        assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [1, 4]);
    }

    #[test]
    fn removed_non_ast_sibling_key_can_be_reused() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(1);
        let sibling = values.insert_sibling(primary, 17, 2);

        assert_eq!(values.remove_sibling(sibling), Some(2));
        let replacement = values.insert_sibling(primary, 17, 3);

        assert_eq!(replacement, sibling);
        assert_eq!(values.get(replacement), Some(&3));
    }

    #[test]
    fn sibling_capacity_can_be_preflighted_for_ast_overflow() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        for key in 1..=SIBLING_MASK as u16 {
            values.insert_sibling(primary, key, key);
        }

        assert!(!values.can_insert_sibling(primary));
        assert_eq!(values.len(), SIBLING_MASK as usize + 1);
    }

    #[test]
    fn retired_id_exhaustion_selects_the_ast_overflow_path() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        for key in 1..=SIBLING_MASK as u16 {
            let id = values.insert_sibling(primary, key, key);
            assert_eq!(values.retire_sibling(id), Some(key));
        }

        assert!(!values.can_insert_sibling(primary));
        assert!(!values.can_insert_between(primary, None));
        assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [0]);
    }

    #[test]
    fn insert_between_relabels_only_one_local_sibling_group() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(0);
        let next_primary = values.push_primary(4);
        let left = values.insert_sibling(primary, 1, 1);
        let right = values.insert_sibling(primary, 2, 3);
        let untouched = values.insert_sibling(next_primary, 512, 5);

        let result = values.insert_between(left, Some(right), 2);

        assert_eq!(result.remaps.len(), 1);
        assert_eq!(result.remaps[0].old, right);
        assert_eq!(values.get(left), Some(&1));
        assert_eq!(values.get(right), None);
        assert_eq!(values.get(untouched), Some(&5));
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [0, 1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn insert_between_uses_a_gap_without_relabeling() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let first = values.push_primary(0);
        let second = values.push_primary(2);

        let result = values.insert_between(first, Some(second), 1);

        assert!(result.remaps.is_empty());
        assert_eq!(result.id.sibling_key(), 512);
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [0, 1, 2]
        );
    }

    #[test]
    fn randomized_local_edits_match_a_reference_sequence() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let mut reference = (0_u32..8)
            .map(|value| (values.push_primary(value), value))
            .collect::<std::vec::Vec<_>>();
        let mut random = 0x9e37_79b9_u32;
        let mut next_value = 8_u32;

        for step in 0..256 {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;

            let removable = reference
                .iter()
                .enumerate()
                .filter_map(|(index, (id, _))| (!id.is_primary()).then_some(index))
                .collect::<std::vec::Vec<_>>();
            if step % 5 == 4 && !removable.is_empty() {
                let index = removable[random as usize % removable.len()];
                let (id, value) = reference.remove(index);
                assert_eq!(values.retire_sibling(id), Some(value));
            } else {
                let after_index = random as usize % reference.len();
                let after = reference[after_index].0;
                let before = reference.get(after_index + 1).map(|(id, _)| *id);
                let result = values.insert_between(after, before, next_value);
                for remap in result.remaps {
                    let entry = reference
                        .iter_mut()
                        .find(|(id, _)| *id == remap.old)
                        .expect("every remapped ID exists in the reference sequence");
                    entry.0 = remap.new;
                }
                reference.insert(after_index + 1, (result.id, next_value));
                next_value += 1;
            }

            assert_eq!(
                values
                    .iter_enumerated()
                    .map(|(id, value)| (id, *value))
                    .collect::<std::vec::Vec<_>>(),
                reference
            );
        }
    }

    #[test]
    fn radix_ids_reserve_u32_max_as_the_option_niche() {
        assert_eq!(std::mem::size_of::<RadixId<u8>>(), 4);
        assert_eq!(std::mem::size_of::<Option<RadixId<u8>>>(), 4);

        let highest_overflow_id = RadixId::<u8>::from_overflow_index(OVERFLOW_CAPACITY - 1);
        assert_eq!(highest_overflow_id.get(), u32::MAX - 7);
    }

    #[test]
    fn authored_primary_overflow_preserves_ids_lookup_and_iteration_order() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let mut last_compact = None;
        for _ in 0..COMPACT_PRIMARY_CAPACITY {
            last_compact = Some(values.push_primary(0_u8));
        }

        let overflow = values.push_primary(1_u8);
        let last_compact = last_compact.unwrap();

        assert!(!last_compact.is_overflow());
        assert!(overflow.is_primary());
        assert!(overflow.is_overflow());
        assert!(last_compact < overflow);
        assert_eq!(values.get(overflow), Some(&1));
        assert_eq!(values.primary_id(COMPACT_PRIMARY_CAPACITY), Some(overflow));
        assert_eq!(values.primary_len(), COMPACT_PRIMARY_CAPACITY + 1);
        assert_eq!(values.primary_iter().next_back(), Some(&1));
        assert_eq!(values.iter_enumerated().last(), Some((overflow, &1)));
        assert!(!values.can_insert_sibling(overflow));
        assert!(!values.can_insert_between(overflow, None));
    }

    #[test]
    fn ids_resolve_their_own_storage_value() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(1);
        let sibling = values.insert_sibling(primary, 17, 2);

        assert_eq!(values[primary], 1);
        assert_eq!(values[sibling], 2);
        assert_eq!(values.remove_sibling(sibling), Some(2));
    }

    #[test]
    fn typed_ids_isolate_stores_without_changing_layout() {
        assert_eq!(std::mem::size_of::<TestRuleId>(), 4);
        assert_eq!(std::mem::size_of::<Option<TestRuleId>>(), 4);

        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(2, &allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(30);
        let inserted = values.insert_sibling(first, 512, 20);

        assert_eq!(first.get(), 0);
        assert!(first.is_primary());
        assert!(!first.is_overflow());
        assert_eq!(second.primary_index(), 1);
        assert_eq!(inserted.sibling_key(), 512);
        assert_eq!(values.get(inserted), Some(&20));
    }

    #[test]
    fn semantic_cursors_cross_primary_and_inserted_values() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(3, &allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(30);
        let third = values.push_primary(50);
        let after_first = values.insert_sibling(first, 512, 20);
        let after_second = values.insert_sibling(second, 512, 40);

        assert_eq!(values.next_id(first), Some(after_first));
        assert_eq!(values.next_id(after_first), Some(second));
        assert_eq!(values.next_id(second), Some(after_second));
        assert_eq!(values.next_id(after_second), Some(third));
        assert_eq!(values.next_id(third), None);

        assert_eq!(values.previous_id(third), Some(after_second));
        assert_eq!(values.previous_id(after_second), Some(second));
        assert_eq!(values.previous_id(second), Some(after_first));
        assert_eq!(values.previous_id(after_first), Some(first));
        assert_eq!(values.previous_id(first), None);

        assert_eq!(values.advance_id(first, 0), Some(first));
        assert_eq!(values.advance_id(first, 2), Some(second));
        assert_eq!(values.advance_id(first, 4), Some(third));
        assert_eq!(values.advance_id(first, 5), None);

        assert_eq!(values.retire_sibling(after_first), Some(20));
        assert_eq!(values.next_id(first), Some(second));
        assert_eq!(values.previous_id(second), Some(first));
        assert_eq!(values.advance_id(first, 2), Some(after_second));
    }

    #[test]
    fn mutable_enumerated_visit_needs_no_id_snapshot() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(2, &allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(30);
        let inserted = values.insert_sibling(first, 512, 20);
        let mut visited = std::vec::Vec::new();

        values.for_each_enumerated_mut(|id, value| {
            visited.push(id);
            *value += 1;
        });

        assert_eq!(visited, [first, inserted, second]);
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [11, 21, 31]
        );
    }

    #[test]
    fn enumerated_iterators_return_ids_in_storage_order() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(3, &allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(30);
        let third = values.push_primary(50);
        let inserted_after_first = values.insert_sibling(first, 512, 20);
        let inserted_after_second = values.insert_sibling(second, 512, 40);

        assert_eq!(
            values
                .primary_iter_enumerated()
                .map(|(id, value)| (id, *value))
                .collect::<std::vec::Vec<_>>(),
            [(first, 10), (second, 30), (third, 50)]
        );

        let mut iter = values.iter_enumerated();
        assert_eq!(iter.len(), 5);
        assert_eq!(
            iter.by_ref()
                .map(|(id, value)| (id, *value))
                .collect::<std::vec::Vec<_>>(),
            [
                (first, 10),
                (inserted_after_first, 20),
                (second, 30),
                (inserted_after_second, 40),
                (third, 50),
            ]
        );
        assert_eq!(iter.len(), 0);
    }
}
