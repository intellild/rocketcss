//! Arena-backed sequence optimized for parse-first, insert-rare workloads.
//!
//! Parsed values live in one linear arena [`Vec`]. Its common prefix uses the
//! high primary field of [`RadixId`] as a direct index. If that compact
//! prefix fills, authored values keep appending to the same vector and use a
//! high-bit-tagged dense overflow ID. Rare transformed values inserted after a
//! compact primary live in lazy two-level radix trees reached through a dense
//! `u32` index sidecar. A separate sorted sparse primary list drives semantic
//! traversal without affecting O(1) lookup.
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
const NO_SIBLING_GROUP: u32 = u32::MAX;

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

    /// Returns the linear parse-vector index of a primary value, or the index
    /// of the owning primary value for an inserted sibling.
    #[inline]
    pub const fn primary_index(self) -> usize {
        if self.is_overflow() {
            COMPACT_PRIMARY_CAPACITY + self.overflow_index()
        } else {
            (self.get() >> LOCAL_BITS) as usize
        }
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

/// A contiguous run in one [`RadixIndexArena`]'s semantic order.
///
/// An empty range is represented solely by `len == 0`. Its endpoint IDs are
/// semantically invalid placeholders and must never be resolved.
pub struct RadixRange<T> {
    start_id: RadixId<T>,
    last_id: RadixId<T>,
    len: u32,
}

/// Describes the contiguous preorder subtree following one range item.
pub trait RadixRangeItem {
    /// Returns the number of contiguous items after this item that belong to
    /// its subtree.
    fn descendants(&self) -> u32;
}

impl<T> Clone for RadixRange<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RadixRange<T> {}

impl<T> fmt::Debug for RadixRange<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RadixRange")
            .field("start_id", &self.start_id)
            .field("last_id", &self.last_id)
            .field("len", &self.len)
            .finish()
    }
}

impl<T> PartialEq for RadixRange<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.start_id == other.start_id && self.last_id == other.last_id && self.len == other.len
    }
}

impl<T> Eq for RadixRange<T> {}

impl<T> RadixRange<T> {
    /// Creates an empty range whose placeholder start must not be resolved.
    #[inline]
    pub fn empty() -> Self {
        Self {
            start_id: RadixId::from_parts(0, 0),
            last_id: RadixId::from_parts(0, 0),
            len: 0,
        }
    }

    /// Creates a semantic range. A zero length is canonicalized to
    /// [`empty`](Self::empty).
    ///
    /// For a non-empty range, the caller must ensure that `start_id`,
    /// `last_id`, and `len` describe the same contiguous semantic run. Arena
    /// range APIs reject combinations that are inconsistent with the arena's
    /// current semantic order.
    #[inline]
    pub fn new(start_id: RadixId<T>, last_id: RadixId<T>, len: u32) -> Self {
        if len == 0 {
            Self::empty()
        } else {
            Self {
                start_id,
                last_id,
                len,
            }
        }
    }

    /// Creates a one-value semantic range.
    #[inline]
    pub fn singleton(id: RadixId<T>) -> Self {
        Self::new(id, id, 1)
    }

    /// Returns the first ID in a non-empty range.
    ///
    /// # Panics
    ///
    /// Panics when the range is empty because its start is only a placeholder.
    #[inline]
    pub fn start_id(self) -> RadixId<T> {
        assert!(self.len != 0, "an empty RadixRange has no start ID");
        self.start_id
    }

    /// Returns the final ID in a non-empty range.
    ///
    /// # Panics
    ///
    /// Panics when the range is empty because its endpoint is only a
    /// placeholder.
    #[inline]
    pub fn last_id(self) -> RadixId<T> {
        assert!(self.len != 0, "an empty RadixRange has no last ID");
        self.last_id
    }

    /// Returns the number of semantic values in the range.
    #[inline]
    pub const fn len(self) -> u32 {
        self.len
    }

    /// Returns whether the range contains no values.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    /// Returns whether `id` is encoded between this range's known endpoints.
    /// Arena insertion preserves encoded semantic order, so no traversal is
    /// required for a range that was constructed from actual insertion IDs.
    #[doc(hidden)]
    #[inline]
    pub fn contains(self, id: RadixId<T>) -> bool {
        !self.is_empty() && self.start_id <= id && id <= self.last_id
    }

    /// Initializes an empty range with its first live ID.
    #[doc(hidden)]
    #[inline]
    pub fn initialize(&mut self, id: RadixId<T>) {
        assert!(
            self.is_empty(),
            "only an empty RadixRange can be initialized"
        );
        *self = Self::singleton(id);
    }

    /// Appends one known semantic ID to a non-empty range.
    #[doc(hidden)]
    #[inline]
    pub fn append(&mut self, id: RadixId<T>) {
        assert!(!self.is_empty(), "an empty RadixRange must be initialized");
        self.last_id = id;
        self.len = self
            .len
            .checked_add(1)
            .expect("RadixRange length exhausted");
    }

    /// Extends this range by a complete, ordered range.
    #[doc(hidden)]
    #[inline]
    pub fn extend(&mut self, next: Self) {
        if next.is_empty() {
            return;
        }
        if self.is_empty() {
            *self = next;
            return;
        }
        self.last_id = next.last_id;
        self.len = self
            .len
            .checked_add(next.len)
            .expect("RadixRange length exhausted");
    }

    /// Clears the range and restores the canonical empty placeholder.
    #[doc(hidden)]
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::empty();
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
    allocator: &'arena Allocator,
    root: Option<ArenaBox<'arena, RadixRoot<'arena, T>>>,
    used_len: u16,
    #[cfg(test)]
    allocations: RadixAllocationCounts,
}

impl<'arena, T> RadixTree<'arena, T> {
    #[inline]
    fn new(allocator: &'arena Allocator) -> Self {
        Self {
            allocator,
            root: None,
            used_len: 0,
            #[cfg(test)]
            allocations: RadixAllocationCounts::default(),
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
                self.used_len -= 1;
            }
            value
        } else {
            let leaf = root.leaves[high].as_deref_mut()?;
            let bit = 1_u32 << low;
            let value = ArenaBox::into_inner(leaf.values[low].take()?);
            leaf.occupied &= !bit;
            if reusable {
                leaf.used &= !bit;
                self.used_len -= 1;
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

    fn reclaim_retired(&mut self, key: u16) -> bool {
        if self.get(key).is_some() || !self.is_used(key) {
            return false;
        }
        let (high, low) = radix_parts(key);
        let root = self
            .root
            .as_deref_mut()
            .expect("a used sibling slot has a radix root");
        if low == 0 {
            root.direct_used &= !(1_u32 << high);
        } else {
            root.leaves[high]
                .as_deref_mut()
                .expect("a used leaf slot has a radix leaf")
                .used &= !(1_u32 << low);
        }
        self.used_len -= 1;
        true
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
        usize::from(self.used_len)
    }

    #[inline]
    fn used_slots(&self, branch: usize) -> u32 {
        let Some(root) = self.root.as_deref() else {
            return 0;
        };
        u32::from(root.direct_used & (1_u32 << branch) != 0)
            | root.leaves[branch]
                .as_deref()
                .map_or(0, |leaf| leaf.used & !1)
    }

    fn insert_unused(&mut self, key: u16, value: T) {
        debug_assert!(key != 0 && u32::from(key) <= SIBLING_MASK);
        debug_assert!(!self.is_used(key));
        if self.root.is_none() {
            self.root = Some(ArenaBox::new_in(RadixRoot::new(), self.allocator));
            #[cfg(test)]
            {
                self.allocations.roots += 1;
            }
        }
        let root = self.root.as_deref_mut().expect("radix tree has a root");
        let (high, low) = radix_parts(key);
        if low == 0 {
            let bit = 1_u32 << high;
            root.direct[high] = Some(ArenaBox::new_in(value, self.allocator));
            root.direct_occupied |= bit;
            root.direct_used |= bit;
            root.occupied_branches |= bit;
        } else {
            if root.leaves[high].is_none() {
                root.leaves[high] = Some(ArenaBox::new_in(RadixLeaf::new(), self.allocator));
                #[cfg(test)]
                {
                    self.allocations.leaves += 1;
                }
            }
            let leaf = root.leaves[high]
                .as_deref_mut()
                .expect("nonzero-low radix key has a leaf");
            let bit = 1_u32 << low;
            leaf.values[low] = Some(ArenaBox::new_in(value, self.allocator));
            leaf.occupied |= bit;
            leaf.used |= bit;
            root.occupied_branches |= 1_u32 << high;
        }
        self.used_len += 1;
        #[cfg(test)]
        {
            self.allocations.values += 1;
        }
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

/// Capacity failures reported before a radix arena mutation writes anything.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadixCapacityError<I> {
    IntervalExhausted {
        primary: I,
        lower: u16,
        upper: u16,
        needed: u32,
        available: u32,
    },
    SiblingTreeExhausted {
        primary: I,
    },
    ArenaExhausted,
}

/// A failed single-value insertion together with the value not written.
#[derive(Debug)]
pub struct RadixInsertError<T, I> {
    pub error: RadixCapacityError<I>,
    pub value: T,
}

/// A failed range insertion together with the iterator not consumed.
#[derive(Debug)]
pub struct RadixRangePushError<Values, I> {
    pub error: RadixCapacityError<I>,
    pub values: Values,
}

/// Storage state of one encoded sibling slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadixSiblingSlotState {
    Vacant,
    Live,
    Retired,
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
    // Stable tree indices make a known sibling ID an O(1) lookup. The sorted
    // primary list is traversal-only and never determines tree storage.
    sibling_group_indices: Vec<'arena, u32>,
    sibling_primary_indices: Vec<'arena, u32>,
    sibling_trees: Vec<'arena, RadixTree<'arena, T>>,
    len: u32,
    id: PhantomData<fn(I) -> I>,
}

/// A validated primary anchor for one or more sibling insertions.
///
/// The arena resolves or creates the sibling group once. The resulting entry
/// borrows only that radix tree and the arena's total-length counter, so
/// repeated insertions do not inspect the arena sidecars again.
pub struct RadixSiblingEntry<'tree, 'arena, T: Unpin, I: RadixIdKey> {
    primary: I,
    tree: &'tree mut RadixTree<'arena, T>,
    arena_len: &'tree mut u32,
}

impl<'tree, 'arena, T: Unpin, I: RadixIdKey> RadixSiblingEntry<'tree, 'arena, T, I> {
    #[inline]
    fn is_used(&self, sibling_key: u16) -> bool {
        self.tree.is_used(sibling_key)
    }

    /// Inserts one unused sibling key below this entry's primary anchor.
    ///
    /// The entry may be reused for additional insertions while it has
    /// capacity. Callers must supply a key in `1..=1023` that has never been
    /// allocated in this sibling group.
    pub fn try_insert(&mut self, sibling_key: u16, value: T) -> Result<I, RadixInsertError<T, I>> {
        debug_assert!(
            sibling_key != 0 && u32::from(sibling_key) <= SIBLING_MASK,
            "radix sibling key must be in 1..=1023"
        );
        if *self.arena_len == u32::MAX {
            return Err(RadixInsertError {
                error: RadixCapacityError::ArenaExhausted,
                value,
            });
        }
        if self.tree.used_len() == SIBLING_MASK as usize {
            return Err(RadixInsertError {
                error: RadixCapacityError::SiblingTreeExhausted {
                    primary: self.primary,
                },
                value,
            });
        }
        if self.is_used(sibling_key) {
            return Err(RadixInsertError {
                error: RadixCapacityError::IntervalExhausted {
                    primary: self.primary,
                    lower: sibling_key - 1,
                    upper: sibling_key + 1,
                    needed: 1,
                    available: 0,
                },
                value,
            });
        }
        self.tree.insert_unused(sibling_key, value);
        *self.arena_len += 1;
        Ok(I::from_parts(self.primary.primary_index(), sibling_key))
    }
}

/// A validated vacant position between two current semantic neighbors.
///
/// The entry owns the resolved sibling group and encoded key, so insertion
/// does not repeat neighbor validation or key lookup.
pub struct RadixVacantEntry<'tree, 'arena, T: Unpin, I: RadixIdKey> {
    sibling: RadixSiblingEntry<'tree, 'arena, T, I>,
    sibling_key: u16,
}

impl<'tree, 'arena, T: Unpin, I: RadixIdKey> RadixVacantEntry<'tree, 'arena, T, I> {
    /// Inserts the value into this vacant position.
    pub fn try_insert(mut self, value: T) -> Result<I, RadixInsertError<T, I>> {
        self.sibling.try_insert(self.sibling_key, value)
    }
}

#[derive(Clone, Copy, Debug)]
struct RadixVacancyCursor {
    branch: u8,
    free_slots: u32,
}

/// A sibling-only gap that can receive one semantic range.
pub struct RadixSiblingRangeEntry<'tree, 'arena, T: Unpin, M> {
    sibling: RadixSiblingEntry<'tree, 'arena, T, RadixId<M>>,
    lower: u16,
    upper: u16,
    vacancy: RadixVacancyCursor,
    capacity: u32,
}

impl<'tree, 'arena, T: Unpin, M> RadixSiblingRangeEntry<'tree, 'arena, T, M> {
    /// Returns the number of values that fit in this gap without changing an
    /// existing ID.
    #[inline]
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn try_push<Values>(
        mut self,
        values: Values,
    ) -> Result<RadixRange<M>, RadixRangePushError<Values::IntoIter, RadixId<M>>>
    where
        Values: IntoIterator<Item = T>,
        Values::IntoIter: ExactSizeIterator,
    {
        let mut values = values.into_iter();
        let needed = u32::try_from(values.len()).unwrap_or(u32::MAX);
        if needed > self.capacity {
            return Err(RadixRangePushError {
                error: RadixCapacityError::IntervalExhausted {
                    primary: self.sibling.primary,
                    lower: self.lower,
                    upper: self.upper,
                    needed,
                    available: self.capacity,
                },
                values,
            });
        }
        if needed > u32::MAX - *self.sibling.arena_len {
            return Err(RadixRangePushError {
                error: RadixCapacityError::ArenaExhausted,
                values,
            });
        }

        let mut endpoints = None;
        let mut inserted = 0_u32;
        for _ in 0..needed {
            let Some(value) = values.next() else { break };
            let key = self
                .vacancy
                .next(&self.sibling, self.lower, self.upper)
                .expect("range capacity was preflighted");
            self.sibling.tree.insert_unused(key, value);
            *self.sibling.arena_len += 1;
            let id = RadixId::from_parts(self.sibling.primary.primary_index(), key);
            endpoints = Some((endpoints.map_or(id, |(start, _)| start), id));
            inserted += 1;
        }
        Ok(endpoints.map_or_else(RadixRange::empty, |(start, last)| {
            RadixRange::new(start, last, inserted)
        }))
    }
}

impl RadixVacancyCursor {
    #[inline]
    fn free_slots_in_branch<T>(
        tree: Option<&RadixTree<'_, T>>,
        branch: usize,
        lower: u16,
        upper: u16,
    ) -> u32 {
        let branch_start = (branch * RADIX_SIZE) as u16;
        if branch_start >= upper {
            return 0;
        }
        let first = lower.saturating_add(1).saturating_sub(branch_start).min(32);
        let end = upper.saturating_sub(branch_start).min(32);
        let after_lower = u32::MAX.checked_shl(u32::from(first)).unwrap_or(0);
        let before_upper = u32::MAX.checked_shr(u32::from(32 - end)).unwrap_or(0);
        after_lower & before_upper & !tree.map_or(0, |tree| tree.used_slots(branch))
    }

    fn next<T: Unpin, I: RadixIdKey>(
        &mut self,
        sibling: &RadixSiblingEntry<'_, '_, T, I>,
        lower: u16,
        upper: u16,
    ) -> Option<u16> {
        loop {
            if self.free_slots != 0 {
                let low = self.free_slots.trailing_zeros() as u16;
                self.free_slots &= self.free_slots - 1;
                let branch = self.branch;
                if self.free_slots == 0 {
                    self.branch += 1;
                }
                return Some(u16::from(branch) * RADIX_SIZE as u16 + low);
            }
            let branch = usize::from(self.branch);
            if branch >= RADIX_SIZE {
                return None;
            }
            if (branch * RADIX_SIZE) as u16 >= upper {
                return None;
            }
            self.free_slots =
                Self::free_slots_in_branch(Some(&*sibling.tree), branch, lower, upper);
            if self.free_slots == 0 {
                self.branch += 1;
            }
        }
    }
}

impl<'arena, T: Unpin, I: RadixIdKey> TypedRadixIndexArena<'arena, T, I> {
    /// Creates an empty store in `allocator`.
    pub fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            allocator,
            primary: Vec::new_in(allocator),
            sibling_group_indices: Vec::new_in(allocator),
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
            sibling_group_indices: Vec::with_capacity_in(capacity, allocator),
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

    /// Mutably visits every value with its typed ID in semantic arena order.
    ///
    /// This is the mutation counterpart to [`iter_enumerated`](Self::iter_enumerated)
    /// for callers that need to update arena records without first collecting
    /// their IDs into a temporary allocation.
    pub fn for_each_enumerated_mut(&mut self, mut visit: impl FnMut(I, &mut T)) {
        let mut next_group = 0;
        let sibling_primary_indices = &self.sibling_primary_indices;
        let sibling_group_indices = &self.sibling_group_indices;
        let sibling_trees = &mut self.sibling_trees;
        for (primary_index, value) in self.primary.iter_mut().enumerate() {
            visit(I::from_primary_index(primary_index), value);
            if sibling_primary_indices.get(next_group).copied() == Some(primary_index as u32) {
                let tree_index = sibling_group_indices[primary_index] as usize;
                sibling_trees[tree_index].for_each_enumerated_mut(|key, value| {
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
            back: self.primary.len(),
            id: PhantomData,
        }
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
        self.sibling_group_indices.push(NO_SIBLING_GROUP);
        self.len += 1;
        id
    }

    /// Returns a cached insertion entry for `primary`, or `None` when the
    /// primary is invalid.
    ///
    /// This is the AST wrapper's preflight for choosing its overflow fallback
    /// before moving a value into the compact store. Keeping the entry through
    /// insertion avoids resolving the sibling group twice.
    pub fn sibling_entry(&mut self, primary: I) -> Option<RadixSiblingEntry<'_, 'arena, T, I>> {
        if primary.is_overflow()
            || primary.sibling_key() != 0
            || self.primary.get(primary.primary_index()).is_none()
        {
            return None;
        }
        Some(self.resolve_sibling_entry(primary.primary_index()))
    }

    /// Returns a validated vacant entry between two current semantic
    /// neighbors without reusing any retired ID.
    pub fn entry_between(
        &mut self,
        after: I,
        before: Option<I>,
    ) -> Option<RadixVacantEntry<'_, 'arena, T, I>> {
        if self.len == u32::MAX {
            return None;
        }
        let (primary_index, lower, upper) = self.insertion_point(after, before)?;
        let sibling_key = self.unused_key_between(primary_index, lower, upper)?;
        Some(RadixVacantEntry {
            sibling: self.resolve_sibling_entry(primary_index),
            sibling_key,
        })
    }

    fn resolve_sibling_entry(
        &mut self,
        primary_index: usize,
    ) -> RadixSiblingEntry<'_, 'arena, T, I> {
        let group_index = match self.sibling_group_index(primary_index) {
            Some(index) => index,
            None => {
                let semantic_index = self
                    .sibling_group_position(primary_index)
                    .expect_err("a missing sibling group has no traversal entry");
                self.sibling_primary_indices
                    .insert(semantic_index, primary_index as u32);
                let index = self.sibling_trees.len();
                self.sibling_trees.push(RadixTree::new(self.allocator));
                self.sibling_group_indices[primary_index] = index as u32;
                index
            }
        };
        RadixSiblingEntry {
            primary: I::from_primary_index(primary_index),
            tree: &mut self.sibling_trees[group_index],
            arena_len: &mut self.len,
        }
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
        let group_index = self.sibling_group_index(id.primary_index())?;
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
        let group_index = self.sibling_group_index(id.primary_index())?;
        let value = self
            .sibling_trees
            .get_mut(group_index)?
            .take(id.sibling_key(), false)?;
        self.len -= 1;
        Some(value)
    }

    /// Reports whether one encoded sibling slot is vacant, live, or retained
    /// as a non-reusable tombstone. This exposes storage facts without choosing
    /// an AST rebalance policy.
    pub fn sibling_slot_state(
        &self,
        primary: I,
        sibling_key: u16,
    ) -> Option<RadixSiblingSlotState> {
        if !primary.is_primary()
            || primary.is_overflow()
            || sibling_key == 0
            || u32::from(sibling_key) > SIBLING_MASK
            || self.primary.get(primary.primary_index()).is_none()
        {
            return None;
        }
        let Some(group_index) = self.sibling_group_index(primary.primary_index()) else {
            return Some(RadixSiblingSlotState::Vacant);
        };
        let tree = &self.sibling_trees[group_index];
        Some(if tree.get(sibling_key).is_some() {
            RadixSiblingSlotState::Live
        } else if tree.is_used(sibling_key) {
            RadixSiblingSlotState::Retired
        } else {
            RadixSiblingSlotState::Vacant
        })
    }

    /// Explicitly makes one retired sibling slot reusable.
    ///
    /// Callers must first prove that every persistent reference to the retired
    /// ID has been repaired. Normal insertion and retirement never call this
    /// method implicitly.
    pub fn reclaim_retired_sibling(&mut self, id: I) -> bool {
        if id.is_primary() || id.is_overflow() {
            return false;
        }
        self.sibling_group_index(id.primary_index())
            .and_then(|group| self.sibling_trees.get_mut(group))
            .is_some_and(|tree| tree.reclaim_retired(id.sibling_key()))
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
            let group_index = self.sibling_group_index(id.primary_index())?;
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
            let group_index = self.sibling_group_index(id.primary_index())?;
            self.sibling_trees
                .get_mut(group_index)?
                .get_mut(id.sibling_key())
        }
    }

    /// Iterates all values in semantic order.
    #[inline]
    pub fn iter(&self) -> Iter<'_, 'arena, T> {
        let kind = if self.sibling_primary_indices.is_empty() {
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
            sibling_group_indices: &self.sibling_group_indices,
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

    /// Iterates typed IDs in semantic order without resolving each ID again.
    #[inline]
    pub fn ids(&self) -> RadixIds<'_, 'arena, T, I> {
        RadixIds {
            arena: self,
            cursor: RadixIdsCursor::Arena(DetachedRadixIds::at_start(self)),
            remaining: self.len,
            expected_last: None,
        }
    }

    /// Creates detached semantic-ID cursor state for mutation-safe AST
    /// traversal. The cursor holds no arena borrow; callers pass the arena only
    /// while advancing it and must not structurally mutate the arena between
    /// steps.
    #[doc(hidden)]
    #[inline]
    pub fn detached_ids(&self) -> DetachedRadixIds {
        DetachedRadixIds::at_start(self)
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
            sibling_group_indices: &self.sibling_group_indices,
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
    fn sibling_group_index(&self, primary_index: usize) -> Option<usize> {
        let index = *self.sibling_group_indices.get(primary_index)?;
        (index != NO_SIBLING_GROUP).then_some(index as usize)
    }

    #[inline]
    fn sibling_group_position(&self, primary_index: usize) -> Result<usize, usize> {
        self.sibling_primary_indices
            .binary_search(&(primary_index as u32))
    }

    fn insertion_point(&self, after: I, before: Option<I>) -> Option<(usize, u16, u16)> {
        if after.is_overflow() {
            return None;
        }
        let mut cursor = DetachedRadixRangeCursor::at_id(self, after)?;
        if cursor.advance(self) != Some(after) || cursor.advance(self) != before {
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
            Some(before) if before.is_primary() && before.primary_index() == primary_index + 1 => {
                (SIBLING_MASK + 1) as u16
            }
            None => (SIBLING_MASK + 1) as u16,
            _ => return None,
        };
        Some((primary_index, lower, upper))
    }

    fn unused_key_between(&self, primary_index: usize, lower: u16, upper: u16) -> Option<u16> {
        if upper <= lower + 1 {
            return None;
        }
        let tree = self
            .sibling_group_index(primary_index)
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
}

impl<'arena, T: Unpin, M> TypedRadixIndexArena<'arena, T, RadixId<M>> {
    /// Tries to append a batch of primary values without consuming it on
    /// capacity failure.
    pub fn try_push_primary_range<Values>(
        &mut self,
        values: Values,
    ) -> Result<RadixRange<M>, RadixRangePushError<Values::IntoIter, RadixId<M>>>
    where
        Values: IntoIterator<Item = T>,
        Values::IntoIter: ExactSizeIterator,
    {
        let values = values.into_iter();
        let reported_len = u32::try_from(values.len()).unwrap_or(u32::MAX);
        let has_capacity = self.len.checked_add(reported_len).is_some()
            && self
                .primary
                .len()
                .checked_add(reported_len as usize)
                .is_some_and(|primary_len| {
                    primary_len <= COMPACT_PRIMARY_CAPACITY + OVERFLOW_CAPACITY
                });
        if !has_capacity {
            return Err(RadixRangePushError {
                error: RadixCapacityError::ArenaExhausted,
                values,
            });
        }
        let mut endpoints = None;
        let mut pushed = 0_u32;
        for value in values {
            let id = self.push_primary(value);
            endpoints = Some((endpoints.map_or(id, |(start, _)| start), id));
            pushed += 1;
        }
        Ok(endpoints.map_or_else(RadixRange::empty, |(start, last)| {
            RadixRange::new(start, last, pushed)
        }))
    }

    /// Appends a batch of primary values and returns their semantic range.
    pub fn push_primary_range<Values>(&mut self, values: Values) -> RadixRange<M>
    where
        Values: IntoIterator<Item = T>,
        Values::IntoIter: ExactSizeIterator,
    {
        match self.try_push_primary_range(values) {
            Ok(range) => range,
            Err(_) => panic!("RadixIndexArena primary range capacity exhausted"),
        }
    }

    /// Returns the IDs in `range`, or `None` when the complete range is not
    /// present in this arena. Empty ranges never resolve their placeholder ID.
    pub fn ids_in_range(
        &self,
        range: RadixRange<M>,
    ) -> Option<RadixIds<'_, 'arena, T, RadixId<M>>> {
        let cursor = DetachedRadixRangeCursor::from_range(self, range)?;
        Some(RadixIds {
            arena: self,
            cursor: RadixIdsCursor::Range(cursor),
            remaining: range.len(),
            expected_last: (!range.is_empty()).then(|| range.last_id()),
        })
    }

    /// Creates a detached cursor over one semantic range.
    #[doc(hidden)]
    pub fn detached_ids_in_range(
        &self,
        range: RadixRange<M>,
    ) -> Option<DetachedRadixRangeIds<RadixId<M>>> {
        Some(DetachedRadixRangeIds {
            cursor: DetachedRadixRangeCursor::from_range(self, range)?,
            remaining: range.len(),
            expected_last: (!range.is_empty()).then(|| range.last_id()),
        })
    }

    /// Returns a primary-vector slice when the complete semantic range is
    /// provably a contiguous run of primary values.
    pub fn primary_slice_in_range(&self, range: RadixRange<M>) -> Option<&[T]> {
        if range.is_empty() {
            return Some(&self.primary[..0]);
        }

        let start = range.start_id();
        let last = range.last_id();
        if !start.is_primary() || !last.is_primary() {
            return None;
        }

        let start = start.primary_index();
        let last = last.primary_index();
        let primary_len = last.checked_sub(start)?.checked_add(1)?;
        if primary_len != range.len() as usize {
            return None;
        }

        let next_group = self
            .sibling_primary_indices
            .binary_search(&(start as u32))
            .unwrap_or_else(|next_group| next_group);
        if self
            .sibling_primary_indices
            .get(next_group)
            .is_some_and(|&primary| (primary as usize) < last)
        {
            return None;
        }

        self.primary.get(start..=last)
    }

    /// Iterates values in one valid semantic range.
    pub fn iter_range(&self, range: RadixRange<M>) -> Option<RadixRangeIter<'_, 'arena, T, M>> {
        let kind = match self.primary_slice_in_range(range) {
            Some(primary) => RadixRangeIterKind::Primary(primary.iter()),
            None => RadixRangeIterKind::Semantic(self.ids_in_range(range)?),
        };
        Some(RadixRangeIter { kind })
    }

    /// Iterates typed IDs and values in one valid semantic range.
    pub fn iter_range_enumerated(
        &self,
        range: RadixRange<M>,
    ) -> Option<RadixRangeIterEnumerated<'_, 'arena, T, M>> {
        let kind = match self.primary_slice_in_range(range) {
            Some(primary) => RadixRangeIterEnumeratedKind::Primary {
                values: primary.iter(),
                next_primary: if range.is_empty() {
                    0
                } else {
                    range.start_id().primary_index()
                },
            },
            None => RadixRangeIterEnumeratedKind::Semantic(self.ids_in_range(range)?),
        };
        Some(RadixRangeIterEnumerated { kind })
    }

    /// Iterates only the direct items in one preorder semantic range.
    ///
    /// Descendant spans are skipped by the range cursor without resolving or
    /// visiting each descendant. Unlike [`iter_range`](Self::iter_range), this
    /// iterator does not pre-scan sibling ranges to validate their endpoint.
    pub fn iter_direct_range(
        &self,
        range: RadixRange<M>,
    ) -> Option<RadixDirectRangeIter<'_, 'arena, T, M>>
    where
        T: RadixRangeItem,
    {
        RadixDirectRangeIter::from_range(self, range)
    }

    /// Iterates typed IDs and direct items in one preorder semantic range.
    ///
    /// This has the same descendant-aware and no-pre-scan behavior as
    /// [`iter_direct_range`](Self::iter_direct_range).
    pub fn iter_direct_range_enumerated(
        &self,
        range: RadixRange<M>,
    ) -> Option<RadixDirectRangeIterEnumerated<'_, 'arena, T, M>>
    where
        T: RadixRangeItem,
    {
        Some(RadixDirectRangeIterEnumerated {
            inner: RadixDirectRangeIter::from_range(self, range)?,
        })
    }

    /// Mutably visits one valid semantic range without materializing its IDs.
    pub fn for_each_in_range_mut(
        &mut self,
        range: RadixRange<M>,
        mut visit: impl FnMut(RadixId<M>, &mut T),
    ) -> Option<()> {
        let mut cursor = DetachedRadixRangeCursor::from_range(self, range)?;
        for offset in 0..range.len() {
            let id = cursor.advance(self)?;
            if offset + 1 == range.len() {
                debug_assert_eq!(id, range.last_id());
            }
            let value = self.get_mut(id)?;
            visit(id, value);
        }
        Some(())
    }

    /// Returns an entry for inserting one semantic range between current
    /// neighbors. Existing IDs are never relabeled.
    pub fn range_entry_between(
        &mut self,
        after: RadixId<M>,
        before: Option<RadixId<M>>,
    ) -> Option<RadixSiblingRangeEntry<'_, 'arena, T, M>> {
        let (primary_index, lower, upper) = self.insertion_point(after, before)?;
        let tree = self
            .sibling_group_index(primary_index)
            .and_then(|group| self.sibling_trees.get(group));
        let capacity = (usize::from(lower / RADIX_SIZE as u16)..RADIX_SIZE)
            .map(|branch| {
                RadixVacancyCursor::free_slots_in_branch(tree, branch, lower, upper).count_ones()
            })
            .sum::<u32>();
        let capacity = capacity.min(u32::MAX - self.len);
        Some(RadixSiblingRangeEntry {
            capacity,
            sibling: self.resolve_sibling_entry(primary_index),
            lower,
            upper,
            vacancy: RadixVacancyCursor {
                branch: (lower / RADIX_SIZE as u16) as u8,
                free_slots: 0,
            },
        })
    }
}

/// Detached traversal state for semantic IDs.
///
/// The cursor locates the starting sibling group once. Advancing thereafter
/// walks primary segments and copied radix occupancy masks without resolving
/// the previously returned ID.
pub struct DetachedRadixIds {
    next_primary: usize,
    next_group: usize,
    current: RadixIdSegment,
}

/// Detached bounded traversal state for one [`RadixRange`].
pub struct DetachedRadixRangeIds<I: RadixIdKey> {
    cursor: DetachedRadixRangeCursor,
    remaining: u32,
    expected_last: Option<I>,
}

impl<I: RadixIdKey> DetachedRadixRangeIds<I> {
    /// Advances the bounded cursor once using a short-lived arena borrow.
    #[doc(hidden)]
    pub fn next<T: Unpin>(&mut self, arena: &TypedRadixIndexArena<'_, T, I>) -> Option<I> {
        if self.remaining == 0 {
            return None;
        }
        let id = self.cursor.advance(arena)?;
        self.remaining -= 1;
        if self.remaining == 0 {
            debug_assert_eq!(Some(id), self.expected_last);
        }
        Some(id)
    }
}

/// Range-local cursor initialized directly from a [`RadixRange`]'s known
/// first ID. It locates the next sibling group once and advances from there;
/// it never scans from the beginning of the range to resume at an ID.
struct DetachedRadixRangeCursor {
    current: RadixRangeIdSegment,
    next_group: usize,
    last_id: Option<u32>,
}

enum RadixRangeIdSegment {
    Primary {
        next: usize,
    },
    Siblings {
        primary_index: usize,
        tree_index: usize,
        cursor: RadixTreeKeyCursor,
    },
    Done,
}

impl DetachedRadixRangeCursor {
    fn from_range<T: Unpin, M>(
        arena: &TypedRadixIndexArena<'_, T, RadixId<M>>,
        range: RadixRange<M>,
    ) -> Option<Self> {
        if range.is_empty() {
            return Some(Self {
                current: RadixRangeIdSegment::Done,
                next_group: 0,
                last_id: None,
            });
        }
        let start_id = range.start_id();
        if arena.primary_slice_in_range(range).is_some() {
            return Self::at_id(arena, start_id);
        }

        let mut validator = Self::at_id(arena, start_id)?;
        let mut actual_last = None;
        for _ in 0..range.len() {
            actual_last = Some(validator.advance(arena)?);
        }
        if actual_last != Some(range.last_id()) {
            return None;
        }
        Self::at_id(arena, start_id)
    }

    fn at_id<T: Unpin, I: RadixIdKey>(
        arena: &TypedRadixIndexArena<'_, T, I>,
        id: I,
    ) -> Option<Self> {
        arena.get(id)?;
        let group_position = arena
            .sibling_primary_indices
            .partition_point(|&primary| (primary as usize) < id.primary_index());
        let (current, next_group) = if id.is_primary() {
            (
                RadixRangeIdSegment::Primary {
                    next: id.primary_index(),
                },
                group_position,
            )
        } else {
            let primary_index = id.primary_index();
            let tree_index = arena.sibling_group_index(primary_index)?;
            let cursor = RadixTreeKeyCursor::from_key(
                arena.sibling_trees.get(tree_index)?,
                id.sibling_key(),
            )?;
            (
                RadixRangeIdSegment::Siblings {
                    primary_index,
                    tree_index,
                    cursor,
                },
                group_position + 1,
            )
        };
        Some(Self {
            current,
            next_group,
            last_id: None,
        })
    }

    #[inline]
    fn advance<T: Unpin, I: RadixIdKey>(
        &mut self,
        arena: &TypedRadixIndexArena<'_, T, I>,
    ) -> Option<I> {
        loop {
            match &mut self.current {
                RadixRangeIdSegment::Primary { next } => {
                    let primary_index = *next;
                    arena.primary.get(primary_index)?;
                    let id = I::from_primary_index(primary_index);
                    *next += 1;
                    self.last_id = Some(id.get());
                    let has_sibling_group = !id.is_overflow()
                        && arena
                            .sibling_primary_indices
                            .get(self.next_group)
                            .is_some_and(|&primary| primary as usize == primary_index);
                    if has_sibling_group {
                        let tree_index = arena
                            .sibling_group_index(primary_index)
                            .expect("a traversal sibling primary has a stable tree index");
                        self.next_group += 1;
                        if let Some(cursor) =
                            RadixTreeKeyCursor::at_start(&arena.sibling_trees[tree_index])
                        {
                            self.current = RadixRangeIdSegment::Siblings {
                                primary_index,
                                tree_index,
                                cursor,
                            };
                        }
                    }
                    return Some(id);
                }
                RadixRangeIdSegment::Siblings {
                    primary_index,
                    tree_index,
                    cursor,
                } => {
                    if let Some(key) = cursor.next_key(&arena.sibling_trees[*tree_index]) {
                        let id = I::from_parts(*primary_index, key);
                        self.last_id = Some(id.get());
                        return Some(id);
                    }
                    let next = *primary_index + 1;
                    self.current = RadixRangeIdSegment::Primary { next };
                }
                RadixRangeIdSegment::Done => return None,
            }
        }
    }

    /// Consumes up to `count` IDs using segment-level arithmetic. Returns
    /// whether the complete count was available.
    fn advance_by<T: Unpin, I: RadixIdKey>(
        &mut self,
        arena: &TypedRadixIndexArena<'_, T, I>,
        mut count: u32,
    ) -> bool {
        while count != 0 {
            match &mut self.current {
                RadixRangeIdSegment::Primary { next } => {
                    if arena.primary.get(*next).is_none() {
                        self.current = RadixRangeIdSegment::Done;
                        return false;
                    }
                    let group_primary = arena
                        .sibling_primary_indices
                        .get(self.next_group)
                        .map(|&primary| primary as usize);
                    let end = group_primary
                        .map_or(arena.primary.len(), |primary| primary + 1)
                        .min(arena.primary.len());
                    let available = end.saturating_sub(*next);
                    let take = available.min(count as usize);
                    if take != 0 {
                        let last = *next + take - 1;
                        self.last_id = Some(I::from_primary_index(last).get());
                        *next += take;
                        count -= take as u32;
                    }
                    if *next != end {
                        continue;
                    }

                    if group_primary.is_some_and(|primary| primary + 1 == end) {
                        let primary_index = end - 1;
                        let tree_index = arena
                            .sibling_group_index(primary_index)
                            .expect("a traversal sibling primary has a stable tree index");
                        self.next_group += 1;
                        if let Some(cursor) =
                            RadixTreeKeyCursor::at_start(&arena.sibling_trees[tree_index])
                        {
                            self.current = RadixRangeIdSegment::Siblings {
                                primary_index,
                                tree_index,
                                cursor,
                            };
                        } else {
                            self.current = RadixRangeIdSegment::Primary { next: end };
                        }
                    } else {
                        self.current = RadixRangeIdSegment::Done;
                    }
                }
                RadixRangeIdSegment::Siblings {
                    primary_index,
                    tree_index,
                    cursor,
                } => {
                    let (skipped, last_key) =
                        cursor.advance_by(&arena.sibling_trees[*tree_index], count);
                    if let Some(last_key) = last_key {
                        self.last_id = Some(I::from_parts(*primary_index, last_key).get());
                    }
                    count -= skipped;
                    if count != 0 {
                        let next = *primary_index + 1;
                        self.current = RadixRangeIdSegment::Primary { next };
                    }
                }
                RadixRangeIdSegment::Done => return false,
            }
        }
        true
    }
}

enum RadixIdSegment {
    Primary {
        next: usize,
        end: usize,
    },
    Siblings {
        primary_index: usize,
        tree_index: usize,
        cursor: RadixTreeKeyCursor,
    },
    Done,
}

impl DetachedRadixIds {
    #[inline]
    fn at_start<T: Unpin, I: RadixIdKey>(arena: &TypedRadixIndexArena<'_, T, I>) -> Self {
        let end = arena
            .sibling_primary_indices
            .first()
            .map_or(arena.primary.len(), |&primary_index| {
                primary_index as usize + 1
            });
        Self {
            next_primary: end,
            next_group: 0,
            current: if end == 0 {
                RadixIdSegment::Done
            } else {
                RadixIdSegment::Primary { next: 0, end }
            },
        }
    }

    #[inline]
    fn advance<T: Unpin, I: RadixIdKey>(
        &mut self,
        arena: &TypedRadixIndexArena<'_, T, I>,
    ) -> Option<I> {
        loop {
            match &mut self.current {
                RadixIdSegment::Primary { next, end } => {
                    if *next < *end {
                        let id = I::from_primary_index(*next);
                        *next += 1;
                        return Some(id);
                    }
                }
                RadixIdSegment::Siblings {
                    primary_index,
                    tree_index,
                    cursor,
                } => {
                    let tree = &arena.sibling_trees[*tree_index];
                    if let Some(key) = cursor.next_key(tree) {
                        return Some(I::from_parts(*primary_index, key));
                    }
                }
                RadixIdSegment::Done => return None,
            }
            self.advance_segment(arena);
        }
    }

    /// Advances the cursor once. Cursor state is updated before control
    /// returns to the caller.
    #[doc(hidden)]
    #[inline]
    pub fn next<T: Unpin, I: RadixIdKey>(
        &mut self,
        arena: &TypedRadixIndexArena<'_, T, I>,
    ) -> Option<I> {
        self.advance(arena)
    }

    #[inline]
    fn advance_segment<T: Unpin, I: RadixIdKey>(&mut self, arena: &TypedRadixIndexArena<'_, T, I>) {
        if matches!(self.current, RadixIdSegment::Primary { .. })
            && let Some(&primary_index) = arena.sibling_primary_indices.get(self.next_group)
            && self.next_primary == primary_index as usize + 1
        {
            let tree_index = arena
                .sibling_group_index(primary_index as usize)
                .expect("a traversal sibling primary has a stable tree index");
            let tree = &arena.sibling_trees[tree_index];
            self.next_group += 1;
            if let Some(cursor) = RadixTreeKeyCursor::at_start(tree) {
                self.current = RadixIdSegment::Siblings {
                    primary_index: primary_index as usize,
                    tree_index,
                    cursor,
                };
                return;
            }
        }

        let start = self.next_primary;
        let end = arena
            .sibling_primary_indices
            .get(self.next_group)
            .map_or(arena.primary.len(), |&primary_index| {
                primary_index as usize + 1
            });
        self.next_primary = end;
        self.current = if start == end {
            RadixIdSegment::Done
        } else {
            RadixIdSegment::Primary { next: start, end }
        };
    }
}

struct RadixTreeKeyCursor {
    branches: u32,
    slots: u32,
    branch: usize,
    direct_pending: bool,
    first_low: Option<usize>,
}

impl RadixTreeKeyCursor {
    #[inline]
    fn at_start<T>(tree: &RadixTree<'_, T>) -> Option<Self> {
        let root = tree.root.as_deref()?;
        (root.occupied_branches != 0).then_some(Self {
            branches: root.occupied_branches,
            slots: 0,
            branch: 0,
            direct_pending: false,
            first_low: None,
        })
    }

    #[inline]
    fn from_key<T>(tree: &RadixTree<'_, T>, key: u16) -> Option<Self> {
        let root = tree.root.as_deref()?;
        let (high, low) = radix_parts(key);
        let branches = root.occupied_branches & (u32::MAX << high);
        (branches != 0).then_some(Self {
            branches,
            slots: 0,
            branch: high,
            direct_pending: false,
            first_low: Some(low),
        })
    }

    #[inline]
    fn next_key<T>(&mut self, tree: &RadixTree<'_, T>) -> Option<u16> {
        let root = tree.root.as_deref()?;
        loop {
            if self.direct_pending {
                self.direct_pending = false;
                return Some((self.branch << RADIX_BITS) as u16);
            }

            if self.slots != 0 {
                let slot = self.slots.trailing_zeros() as usize;
                self.slots &= self.slots - 1;
                return Some(((self.branch << RADIX_BITS) | slot) as u16);
            }

            let branch = self.branches.trailing_zeros() as usize;
            if branch == u32::BITS as usize {
                return None;
            }
            self.branches &= self.branches - 1;
            self.branch = branch;
            let first_low = self.first_low.take().unwrap_or(0);
            let branch_bit = 1_u32 << branch;
            self.direct_pending = first_low == 0 && root.direct_occupied & branch_bit != 0;
            let low_mask = if first_low <= 1 {
                !1_u32
            } else {
                u32::MAX << first_low
            };
            self.slots = root.leaves[branch]
                .as_deref()
                .map_or(0, |leaf| leaf.occupied & low_mask);
        }
    }

    /// Consumes sibling keys with one bitmap operation per crossed radix
    /// branch instead of one operation per live sibling.
    fn advance_by<T>(&mut self, tree: &RadixTree<'_, T>, mut count: u32) -> (u32, Option<u16>) {
        let Some(root) = tree.root.as_deref() else {
            return (0, None);
        };
        let requested = count;
        let mut last_key = None;
        while count != 0 {
            if self.direct_pending {
                self.direct_pending = false;
                last_key = Some((self.branch << RADIX_BITS) as u16);
                count -= 1;
                continue;
            }

            let slots = self.slots.count_ones();
            if slots != 0 {
                if count >= slots {
                    let low = u32::BITS - 1 - self.slots.leading_zeros();
                    last_key = Some(((self.branch << RADIX_BITS) | low as usize) as u16);
                    self.slots = 0;
                    count -= slots;
                    continue;
                }

                let low = nth_set_bit(self.slots, count - 1);
                let after = u32::MAX.checked_shl(low + 1).unwrap_or(0);
                self.slots &= after;
                last_key = Some(((self.branch << RADIX_BITS) | low as usize) as u16);
                count = 0;
                continue;
            }

            let branch = self.branches.trailing_zeros() as usize;
            if branch == u32::BITS as usize {
                break;
            }
            self.branches &= self.branches - 1;
            self.branch = branch;
            let first_low = self.first_low.take().unwrap_or(0);
            let branch_bit = 1_u32 << branch;
            self.direct_pending = first_low == 0 && root.direct_occupied & branch_bit != 0;
            let low_mask = if first_low <= 1 {
                !1_u32
            } else {
                u32::MAX << first_low
            };
            self.slots = root.leaves[branch]
                .as_deref()
                .map_or(0, |leaf| leaf.occupied & low_mask);
        }
        (requested - count, last_key)
    }
}

#[inline]
fn nth_set_bit(mut bits: u32, mut nth: u32) -> u32 {
    while nth != 0 {
        bits &= bits - 1;
        nth -= 1;
    }
    bits.trailing_zeros()
}

/// Iterator over IDs in one arena or [`RadixRange`].
pub struct RadixIds<'tree, 'arena, T: Unpin, I: RadixIdKey> {
    arena: &'tree TypedRadixIndexArena<'arena, T, I>,
    cursor: RadixIdsCursor,
    remaining: u32,
    expected_last: Option<I>,
}

enum RadixIdsCursor {
    Arena(DetachedRadixIds),
    Range(DetachedRadixRangeCursor),
}

impl<T: Unpin, I: RadixIdKey> Iterator for RadixIds<'_, '_, T, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let id = match &mut self.cursor {
            RadixIdsCursor::Arena(cursor) => cursor.advance(self.arena)?,
            RadixIdsCursor::Range(cursor) => cursor.advance(self.arena)?,
        };
        self.remaining -= 1;
        if self.remaining == 0
            && let Some(expected_last) = self.expected_last
        {
            debug_assert_eq!(id, expected_last);
        }
        Some(id)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining as usize;
        (remaining, Some(remaining))
    }
}

impl<T: Unpin, I: RadixIdKey> RadixIds<'_, '_, T, I> {
    /// Returns the semantic ID immediately following an exhausted bounded
    /// iterator, using the iterator's existing cursor state.
    ///
    /// Whole-arena iterators and bounded iterators that have not been fully
    /// consumed return `None`.
    #[doc(hidden)]
    #[inline]
    pub fn following(&mut self) -> Option<I> {
        if self.remaining != 0 {
            return None;
        }
        self.expected_last.take()?;
        match &mut self.cursor {
            RadixIdsCursor::Arena(cursor) => cursor.advance(self.arena),
            RadixIdsCursor::Range(cursor) => cursor.advance(self.arena),
        }
    }
}

impl<T: Unpin, I: RadixIdKey> ExactSizeIterator for RadixIds<'_, '_, T, I> {}
impl<T: Unpin, I: RadixIdKey> std::iter::FusedIterator for RadixIds<'_, '_, T, I> {}

/// Iterator over the values in one [`RadixRange`].
pub struct RadixRangeIter<'tree, 'arena, T: Unpin, M> {
    kind: RadixRangeIterKind<'tree, 'arena, T, M>,
}

enum RadixRangeIterKind<'tree, 'arena, T: Unpin, M> {
    Primary(std::slice::Iter<'tree, T>),
    Semantic(RadixIds<'tree, 'arena, T, RadixId<M>>),
}

impl<'tree, T: Unpin, M> Iterator for RadixRangeIter<'tree, '_, T, M> {
    type Item = &'tree T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            RadixRangeIterKind::Primary(primary) => primary.next(),
            RadixRangeIterKind::Semantic(ids) => ids.next().map(|id| {
                ids.arena
                    .get(id)
                    .expect("a preflighted RadixRange remains valid")
            }),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.kind {
            RadixRangeIterKind::Primary(primary) => primary.size_hint(),
            RadixRangeIterKind::Semantic(ids) => ids.size_hint(),
        }
    }
}

impl<T: Unpin, M> ExactSizeIterator for RadixRangeIter<'_, '_, T, M> {}
impl<T: Unpin, M> std::iter::FusedIterator for RadixRangeIter<'_, '_, T, M> {}

/// Iterator over typed IDs and values in one [`RadixRange`].
pub struct RadixRangeIterEnumerated<'tree, 'arena, T: Unpin, M> {
    kind: RadixRangeIterEnumeratedKind<'tree, 'arena, T, M>,
}

enum RadixRangeIterEnumeratedKind<'tree, 'arena, T: Unpin, M> {
    Primary {
        values: std::slice::Iter<'tree, T>,
        next_primary: usize,
    },
    Semantic(RadixIds<'tree, 'arena, T, RadixId<M>>),
}

impl<'tree, T: Unpin, M> Iterator for RadixRangeIterEnumerated<'tree, '_, T, M> {
    type Item = (RadixId<M>, &'tree T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            RadixRangeIterEnumeratedKind::Primary {
                values,
                next_primary,
            } => {
                let value = values.next()?;
                let id = RadixId::from_primary_index(*next_primary);
                *next_primary += 1;
                Some((id, value))
            }
            RadixRangeIterEnumeratedKind::Semantic(ids) => {
                let id = ids.next()?;
                let value = ids
                    .arena
                    .get(id)
                    .expect("a preflighted RadixRange remains valid");
                Some((id, value))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.kind {
            RadixRangeIterEnumeratedKind::Primary { values, .. } => values.size_hint(),
            RadixRangeIterEnumeratedKind::Semantic(ids) => ids.size_hint(),
        }
    }
}

impl<T: Unpin, M> ExactSizeIterator for RadixRangeIterEnumerated<'_, '_, T, M> {}
impl<T: Unpin, M> std::iter::FusedIterator for RadixRangeIterEnumerated<'_, '_, T, M> {}

/// Iterator over direct values in a preorder [`RadixRange`].
pub struct RadixDirectRangeIter<'tree, 'arena, T: Unpin, M> {
    arena: &'tree TypedRadixIndexArena<'arena, T, RadixId<M>>,
    cursor: DetachedRadixRangeCursor,
    current: Option<RadixId<M>>,
    next: Option<RadixId<M>>,
    remaining_span: u32,
    expected_last: Option<RadixId<M>>,
}

impl<'tree, 'arena, T: Unpin + RadixRangeItem, M> RadixDirectRangeIter<'tree, 'arena, T, M> {
    fn from_range(
        arena: &'tree TypedRadixIndexArena<'arena, T, RadixId<M>>,
        range: RadixRange<M>,
    ) -> Option<Self> {
        if range.is_empty() {
            return Some(Self {
                arena,
                cursor: DetachedRadixRangeCursor {
                    current: RadixRangeIdSegment::Done,
                    next_group: 0,
                    last_id: None,
                },
                current: None,
                next: None,
                remaining_span: 0,
                expected_last: None,
            });
        }

        let mut cursor = DetachedRadixRangeCursor::at_id(arena, range.start_id())?;
        let current = cursor.advance(arena)?;
        let mut state = Self {
            arena,
            cursor,
            current: Some(current),
            next: None,
            remaining_span: range.len() - 1,
            expected_last: Some(range.last_id()),
        };
        state.prepare_next();
        Some(state)
    }

    fn prepare_next(&mut self) {
        let Some(current) = self.current else {
            return;
        };
        let descendants = self
            .arena
            .get(current)
            .expect("a direct-range cursor ID remains valid")
            .descendants();
        if descendants > self.remaining_span {
            let available = self.remaining_span;
            let advanced = self.cursor.advance_by(self.arena, available);
            self.remaining_span = 0;
            self.next = None;
            self.check_endpoint(advanced);
            debug_assert!(
                false,
                "RadixRangeItem descendant span exceeds its containing RadixRange"
            );
            return;
        }

        if descendants != 0 {
            let advanced = self.cursor.advance_by(self.arena, descendants);
            self.remaining_span -= descendants;
            if !advanced {
                self.remaining_span = 0;
                self.next = None;
                debug_assert!(advanced, "RadixRange ended before its descendant span");
                return;
            }
        }

        if self.remaining_span == 0 {
            self.next = None;
            self.check_endpoint(true);
            return;
        }

        self.next = self.cursor.advance(self.arena);
        self.remaining_span -= u32::from(self.next.is_some());
        if self.next.is_none() {
            self.remaining_span = 0;
            debug_assert!(
                self.next.is_some(),
                "RadixRange ended before its declared span"
            );
        }
    }

    #[inline]
    fn check_endpoint(&self, advanced: bool) {
        debug_assert!(advanced, "RadixRange ended before its declared span");
        debug_assert_eq!(
            self.cursor.last_id,
            self.expected_last.map(RadixId::get),
            "RadixRange endpoint does not match its declared span"
        );
    }

    fn next_id(&mut self) -> Option<RadixId<M>> {
        let current = self.current?;
        self.current = self.next.take();
        if self.current.is_some() {
            self.prepare_next();
        }
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let current = usize::from(self.current.is_some());
        let upper = current + usize::from(self.next.is_some()) + self.remaining_span as usize;
        (current, Some(upper))
    }
}

impl<'tree, T: Unpin + RadixRangeItem, M> Iterator for RadixDirectRangeIter<'tree, '_, T, M> {
    type Item = &'tree T;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.next_id()?;
        Some(
            self.arena
                .get(id)
                .expect("a direct-range cursor ID remains valid"),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        RadixDirectRangeIter::size_hint(self)
    }
}

impl<T: Unpin + RadixRangeItem, M> std::iter::FusedIterator for RadixDirectRangeIter<'_, '_, T, M> {}

/// Iterator over typed IDs and direct values in a preorder [`RadixRange`].
pub struct RadixDirectRangeIterEnumerated<'tree, 'arena, T: Unpin, M> {
    inner: RadixDirectRangeIter<'tree, 'arena, T, M>,
}

impl<'tree, T: Unpin + RadixRangeItem, M> Iterator
    for RadixDirectRangeIterEnumerated<'tree, '_, T, M>
{
    type Item = (RadixId<M>, &'tree T);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.inner.next_id()?;
        let value = self
            .inner
            .arena
            .get(id)
            .expect("a direct-range cursor ID remains valid");
        Some((id, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T: Unpin + RadixRangeItem, M> std::iter::FusedIterator
    for RadixDirectRangeIterEnumerated<'_, '_, T, M>
{
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
    sibling_group_indices: &'tree [u32],
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
    sibling_group_indices: &'tree [u32],
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
                let tree_index = self.sibling_group_indices[primary_index as usize] as usize;
                let tree = &self.sibling_trees[tree_index];
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
                let tree_index = self.sibling_group_indices[primary_index as usize] as usize;
                let tree = &self.sibling_trees[tree_index];
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
#[path = "radix_index_arena/tests/mod.rs"]
mod tests;
