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
    // Stable tree indices make a known sibling ID an O(1) lookup. The sorted
    // primary list is traversal-only and never determines tree storage.
    sibling_group_indices: Vec<'arena, u32>,
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
            Some(index) => index,
            None => {
                let semantic_index = self.sibling_group_position(primary_index).unwrap_err();
                self.sibling_primary_indices
                    .insert(semantic_index, primary_index as u32);
                let index = self.sibling_trees.len();
                self.sibling_trees.push(RadixTree::new());
                self.sibling_group_indices[primary_index] = index as u32;
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
        let Some(group) = self.sibling_group_index(primary_index) else {
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
    /// to select their non-panicking fallback or rejection path first.
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

impl<'arena, T: Unpin, M> TypedRadixIndexArena<'arena, T, RadixId<M>> {
    /// Returns whether `len` additional primary values can be appended.
    pub fn can_push_primary_range(&self, len: u32) -> bool {
        self.len.checked_add(len).is_some()
            && self
                .primary
                .len()
                .checked_add(len as usize)
                .is_some_and(|primary_len| {
                    primary_len <= COMPACT_PRIMARY_CAPACITY + OVERFLOW_CAPACITY
                })
    }

    /// Appends a batch of primary values and returns their semantic range.
    pub fn push_primary_range<Values>(&mut self, values: Values) -> RadixRange<M>
    where
        Values: IntoIterator<Item = T>,
        Values::IntoIter: ExactSizeIterator,
    {
        let values = values.into_iter();
        let len = u32::try_from(values.len()).expect("Radix primary range length exceeds u32");
        assert!(
            self.can_push_primary_range(len),
            "RadixIndexArena primary range capacity exhausted"
        );
        let mut endpoints = None;
        for value in values {
            let id = self.push_primary(value);
            endpoints = Some((endpoints.map_or(id, |(start, _)| start), id));
        }
        endpoints.map_or_else(RadixRange::empty, |(start, last)| {
            RadixRange::new(start, last, len)
        })
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

    /// Preflights a stable batch insertion between current semantic neighbors.
    /// Existing IDs are never relabeled.
    pub fn can_insert_stable_range_between(
        &self,
        after: RadixId<M>,
        before: Option<RadixId<M>>,
        len: u32,
    ) -> bool {
        if len == 0 {
            return self.insertion_gap_is_current(after, before);
        }
        if self.len.checked_add(len).is_none() {
            return false;
        }
        if before.is_none() && self.insertion_gap_is_current(after, None) {
            return self.can_push_primary_range(len);
        }
        let Some((primary_index, lower, upper, _)) = self.insertion_point(after, before) else {
            return false;
        };
        self.stable_unused_keys_between(primary_index, lower, upper, len)
            .is_some()
    }

    /// Inserts a stable batch between current semantic neighbors and returns
    /// its contiguous semantic range. Existing IDs are not relabeled.
    ///
    /// # Panics
    ///
    /// Panics unless the full insertion passes
    /// [`can_insert_stable_range_between`](Self::can_insert_stable_range_between).
    pub fn insert_stable_range_between<Values>(
        &mut self,
        after: RadixId<M>,
        before: Option<RadixId<M>>,
        values: Values,
    ) -> RadixRange<M>
    where
        Values: IntoIterator<Item = T>,
        Values::IntoIter: ExactSizeIterator,
    {
        let values = values.into_iter();
        let len = u32::try_from(values.len()).expect("Radix stable range length exceeds u32");
        assert!(
            self.can_insert_stable_range_between(after, before, len),
            "RadixIndexArena stable range capacity exhausted"
        );
        if len == 0 {
            return RadixRange::empty();
        }

        if before.is_none() && self.insertion_gap_is_current(after, None) {
            return self.push_primary_range(values);
        }

        let (primary_index, lower, upper, _) = self
            .insertion_point(after, before)
            .expect("stable range endpoints were preflighted");
        let keys = self
            .stable_unused_keys_between(primary_index, lower, upper, len)
            .expect("stable range key capacity was preflighted");
        let primary = RadixId::from_parts(primary_index, 0);
        let mut endpoints = None;
        for (key, value) in keys.into_iter().zip(values) {
            let id = self.insert_sibling(primary, key, value);
            endpoints = Some((endpoints.map_or(id, |(start, _)| start), id));
        }
        let (start, last) = endpoints.expect("a non-empty batch has endpoints");
        RadixRange::new(start, last, len)
    }

    fn stable_unused_keys_between(
        &self,
        primary_index: usize,
        lower: u16,
        upper: u16,
        len: u32,
    ) -> Option<std::vec::Vec<u16>> {
        let needed = len as usize;
        let tree = self
            .sibling_group_index(primary_index)
            .and_then(|group| self.sibling_trees.get(group));
        let mut keys = std::vec::Vec::with_capacity(needed);
        for key in lower + 1..upper {
            if tree.is_none_or(|tree| !tree.is_used(key)) {
                keys.push(key);
                if keys.len() == needed {
                    return Some(keys);
                }
            }
        }
        (needed == 0).then_some(keys)
    }

    fn insertion_gap_is_current(&self, after: RadixId<M>, before: Option<RadixId<M>>) -> bool {
        let Some(mut cursor) = DetachedRadixIds::at_id(self, after) else {
            return false;
        };
        cursor.advance(self) == Some(after) && cursor.advance(self) == before
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
/// first ID. Unlike whole-arena detached traversal, it needs neither the
/// sorted sibling-group sidecar nor a binary search to resume at that ID.
struct DetachedRadixRangeCursor {
    current: RadixRangeIdSegment,
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
            });
        }
        Self::at_id(arena, range.start_id())
    }

    fn at_id<T: Unpin, I: RadixIdKey>(
        arena: &TypedRadixIndexArena<'_, T, I>,
        id: I,
    ) -> Option<Self> {
        arena.get(id)?;
        let current = if id.is_primary() {
            RadixRangeIdSegment::Primary {
                next: id.primary_index(),
            }
        } else {
            let primary_index = id.primary_index();
            let tree_index = arena.sibling_group_index(primary_index)?;
            let cursor = RadixTreeKeyCursor::from_key(
                arena.sibling_trees.get(tree_index)?,
                id.sibling_key(),
            )?;
            RadixRangeIdSegment::Siblings {
                primary_index,
                tree_index,
                cursor,
            }
        };
        Some(Self { current })
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
                    if !id.is_overflow()
                        && let Some(tree_index) = arena.sibling_group_index(primary_index)
                        && let Some(cursor) =
                            RadixTreeKeyCursor::at_start(&arena.sibling_trees[tree_index])
                    {
                        self.current = RadixRangeIdSegment::Siblings {
                            primary_index,
                            tree_index,
                            cursor,
                        };
                    }
                    return Some(id);
                }
                RadixRangeIdSegment::Siblings {
                    primary_index,
                    tree_index,
                    cursor,
                } => {
                    if let Some(key) = cursor.next_key(&arena.sibling_trees[*tree_index]) {
                        return Some(I::from_parts(*primary_index, key));
                    }
                    let next = *primary_index + 1;
                    self.current = RadixRangeIdSegment::Primary { next };
                }
                RadixRangeIdSegment::Done => return None,
            }
        }
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

    fn at_id<T: Unpin, I: RadixIdKey>(
        arena: &TypedRadixIndexArena<'_, T, I>,
        id: I,
    ) -> Option<Self> {
        let primary_index = id.primary_index();
        if id.is_overflow() {
            arena.primary.get(primary_index)?;
            return Some(Self {
                next_primary: arena.primary.len(),
                next_group: arena.sibling_primary_indices.len(),
                current: RadixIdSegment::Primary {
                    next: primary_index,
                    end: arena.primary.len(),
                },
            });
        }

        arena.primary.get(primary_index)?;
        let group_position = arena.sibling_group_position(primary_index);
        if id.is_primary() {
            let next_group = group_position.unwrap_or_else(|next_group| next_group);
            let end = arena
                .sibling_primary_indices
                .get(next_group)
                .map_or(arena.primary.len(), |&primary_index| {
                    primary_index as usize + 1
                });
            return Some(Self {
                next_primary: end,
                next_group,
                current: RadixIdSegment::Primary {
                    next: primary_index,
                    end,
                },
            });
        }

        let group = group_position.ok()?;
        let tree_index = arena.sibling_group_index(primary_index)?;
        let tree = &arena.sibling_trees[tree_index];
        tree.get(id.sibling_key())?;
        Some(Self {
            next_primary: primary_index + 1,
            next_group: group + 1,
            current: RadixIdSegment::Siblings {
                primary_index,
                tree_index,
                cursor: RadixTreeKeyCursor::from_key(tree, id.sibling_key())?,
            },
        })
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
mod tests {
    use super::{
        COMPACT_PRIMARY_CAPACITY, LOCAL_BITS, NO_SIBLING_GROUP, OVERFLOW_CAPACITY,
        RadixAllocationCounts, RadixId, RadixIndexArena, RadixLeaf, RadixRange, RadixRoot,
        SIBLING_MASK, TypedRadixIndexArena,
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
        assert_eq!(values.primary_iter().len(), 3);
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
        let next_overflow = values.push_primary(2_u8);
        let last_compact = last_compact.unwrap();
        let primary_boundary_range = RadixRange::new(last_compact, next_overflow, 3);

        assert_eq!(
            values.primary_slice_in_range(primary_boundary_range),
            Some([0, 1, 2].as_slice())
        );
        assert_eq!(
            values
                .iter_range_enumerated(primary_boundary_range)
                .unwrap()
                .map(|(id, value)| (id, *value))
                .collect::<std::vec::Vec<_>>(),
            [(last_compact, 0), (overflow, 1), (next_overflow, 2)]
        );

        let boundary_sibling = values.insert_sibling(last_compact, 512, 9_u8);

        assert!(!last_compact.is_overflow());
        assert!(overflow.is_primary());
        assert!(overflow.is_overflow());
        assert!(last_compact < overflow);
        assert!(overflow < next_overflow);
        assert_eq!(overflow.primary_index(), COMPACT_PRIMARY_CAPACITY);
        assert_eq!(next_overflow.primary_index(), COMPACT_PRIMARY_CAPACITY + 1);
        assert_eq!(values.get(overflow), Some(&1));
        assert_eq!(values.get(next_overflow), Some(&2));
        assert_eq!(
            RadixId::from_primary_index(COMPACT_PRIMARY_CAPACITY),
            overflow
        );
        assert_eq!(
            RadixId::from_primary_index(COMPACT_PRIMARY_CAPACITY + 1),
            next_overflow
        );
        assert_eq!(values.primary_iter().len(), COMPACT_PRIMARY_CAPACITY + 2);
        assert_eq!(values.primary_iter().next_back(), Some(&2));
        assert_eq!(values.iter_enumerated().last(), Some((next_overflow, &2)));
        assert_eq!(
            values
                .ids()
                .skip(COMPACT_PRIMARY_CAPACITY - 1)
                .collect::<std::vec::Vec<_>>(),
            [last_compact, boundary_sibling, overflow, next_overflow]
        );
        let boundary_range = RadixRange::new(last_compact, next_overflow, 4);
        assert!(values.primary_slice_in_range(boundary_range).is_none());
        assert_eq!(
            values
                .iter_range(boundary_range)
                .unwrap()
                .copied()
                .collect::<std::vec::Vec<_>>(),
            [0, 9, 1, 2]
        );
        assert!(!values.can_insert_sibling(overflow));
        assert!(!values.can_insert_sibling(next_overflow));
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
    fn semantic_id_cursor_crosses_primary_and_inserted_values() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(3, &allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(30);
        let third = values.push_primary(50);
        let after_first = values.insert_sibling(first, 512, 20);
        let after_second = values.insert_sibling(second, 512, 40);

        assert_eq!(
            values.ids().collect::<std::vec::Vec<_>>(),
            [first, after_first, second, after_second, third]
        );

        assert_eq!(values.retire_sibling(after_first), Some(20));
        assert_eq!(
            values.ids().collect::<std::vec::Vec<_>>(),
            [first, second, after_second, third]
        );
    }

    #[test]
    fn stable_group_sidecar_decouples_lookup_from_semantic_group_order() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(3, &allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(30);
        let third = values.push_primary(50);

        let after_third = values.insert_sibling(third, 512, 60);
        let after_first = values.insert_sibling(first, 512, 20);

        assert_eq!(values.sibling_primary_indices.as_slice(), [0, 2]);
        assert_eq!(
            values.sibling_group_indices.as_slice(),
            [1, NO_SIBLING_GROUP, 0]
        );
        assert_eq!(values.get(after_first), Some(&20));
        assert_eq!(values.get(after_third), Some(&60));
        assert_eq!(
            values.ids().collect::<std::vec::Vec<_>>(),
            [first, after_first, second, third, after_third]
        );

        let range = RadixRange::new(first, after_third, 5);
        values
            .for_each_in_range_mut(range, |_, value| *value += 1)
            .unwrap();
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [11, 21, 31, 51, 61]
        );
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

    #[test]
    fn empty_range_never_resolves_its_placeholder_start() {
        let allocator = Allocator::new();
        let values = TypedRadixIndexArena::<u8, TestRuleId>::new_in(&allocator);
        let range = RadixRange::empty();

        assert!(range.is_empty());
        assert_eq!(values.ids_in_range(range).unwrap().len(), 0);
        assert_eq!(values.iter_range(range).unwrap().len(), 0);
        assert_eq!(values.iter_range_enumerated(range).unwrap().len(), 0);
    }

    #[test]
    fn primary_range_slice_proves_contiguous_ranges_without_resolving_ids() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(20);
        let third = values.push_primary(30);

        assert_eq!(
            values.primary_slice_in_range(RadixRange::empty()),
            Some([].as_slice())
        );
        assert_eq!(
            values.primary_slice_in_range(RadixRange::singleton(second)),
            Some([20].as_slice())
        );

        let range = RadixRange::new(first, third, 3);
        assert_eq!(
            values.primary_slice_in_range(range),
            Some([10, 20, 30].as_slice())
        );
        assert!(
            values
                .primary_slice_in_range(RadixRange::new(first, third, 2))
                .is_none()
        );

        let mut iter = values.iter_range(range).unwrap();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.len(), 2);

        let mut enumerated = values.iter_range_enumerated(range).unwrap();
        assert_eq!(enumerated.len(), 3);
        assert_eq!(enumerated.next(), Some((first, &10)));
        assert_eq!(enumerated.next(), Some((second, &20)));
        assert_eq!(enumerated.next(), Some((third, &30)));
        assert_eq!(enumerated.len(), 0);

        let stale = range;
        values.insert_sibling(second, 512, 25);
        assert!(values.primary_slice_in_range(stale).is_none());
    }

    #[test]
    fn ranges_follow_semantic_order_across_siblings_and_primaries() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(30);
        let inserted = values.insert_sibling(first, 512, 20);
        let range = RadixRange::new(first, second, 3);

        assert!(values.primary_slice_in_range(range).is_none());
        assert_eq!(range.start_id(), first);
        assert_eq!(range.last_id(), second);
        assert_eq!(values.ids_in_range(range).unwrap().nth(1), Some(inserted));
        assert_eq!(
            values
                .iter_range(range)
                .unwrap()
                .copied()
                .collect::<std::vec::Vec<_>>(),
            [10, 20, 30]
        );
        let mut iter = values.iter_range(range).unwrap();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&30));
        let mut enumerated = values.iter_range_enumerated(range).unwrap();
        assert_eq!(enumerated.len(), 3);
        assert_eq!(enumerated.next(), Some((first, &10)));
        assert_eq!(enumerated.len(), 2);
        assert_eq!(
            enumerated.collect::<std::vec::Vec<_>>(),
            [(inserted, &20), (second, &30)]
        );
        values
            .for_each_in_range_mut(range, |_, value| *value += 1)
            .unwrap();
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [11, 21, 31]
        );
    }

    #[test]
    fn a_sibling_after_the_last_primary_does_not_block_the_slice_fast_path() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(20);
        let after_second = values.insert_sibling(second, 512, 30);

        let ending_at_primary = RadixRange::new(first, second, 2);
        assert_eq!(
            values.primary_slice_in_range(ending_at_primary),
            Some([10, 20].as_slice())
        );

        let starting_at_sibling = RadixRange::new(after_second, after_second, 1);
        assert!(values.primary_slice_in_range(starting_at_sibling).is_none());
        assert_eq!(
            values
                .iter_range(starting_at_sibling)
                .unwrap()
                .copied()
                .collect::<std::vec::Vec<_>>(),
            [30]
        );
    }

    #[test]
    fn bounded_range_cursor_resumes_at_a_sibling_and_retains_its_following_id() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(40);
        let third = values.push_primary(70);
        let after_first = values.insert_sibling(first, 256, 20);
        let later_after_first = values.insert_sibling(first, 768, 30);
        let after_second = values.insert_sibling(second, 512, 50);
        let range = RadixRange::new(after_first, second, 3);

        let mut ids = values.ids_in_range(range).unwrap();
        assert_eq!(
            ids.by_ref().collect::<std::vec::Vec<_>>(),
            [after_first, later_after_first, second]
        );
        assert_eq!(ids.following(), Some(after_second));
        assert_eq!(ids.following(), None);

        let mut detached = values.detached_ids_in_range(range).unwrap();
        assert_eq!(detached.next(&values), Some(after_first));
        assert_eq!(detached.next(&values), Some(later_after_first));
        assert_eq!(detached.next(&values), Some(second));
        assert_eq!(detached.next(&values), None);

        let mutable_range = RadixRange::new(after_first, after_second, 4);
        values
            .for_each_in_range_mut(mutable_range, |_, value| *value += 1)
            .unwrap();
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [10, 21, 31, 41, 51, 70]
        );
        assert_eq!(third.primary_index(), 2);
    }

    #[test]
    fn stable_batch_insertion_preserves_existing_ids_and_returns_one_range() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(40);

        assert!(values.can_insert_stable_range_between(first, Some(second), 2));
        let inserted = values.insert_stable_range_between(first, Some(second), [20, 30]);

        assert_eq!(values.get(first), Some(&10));
        assert_eq!(values.get(second), Some(&40));
        assert_eq!(inserted.len(), 2);
        assert_eq!(
            values
                .iter_range(inserted)
                .unwrap()
                .copied()
                .collect::<std::vec::Vec<_>>(),
            [20, 30]
        );
        assert_eq!(inserted.last_id().sibling_key(), 2);
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [10, 20, 30, 40]
        );
    }

    #[test]
    fn stable_batch_capacity_is_preflighted_for_the_complete_range() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
        let first = values.push_primary(10);
        let second = values.push_primary(50);
        let boundary = values.insert_sibling(first, 3, 40);
        let before_len = values.len();

        assert!(values.can_insert_stable_range_between(first, Some(boundary), 2));
        assert!(!values.can_insert_stable_range_between(first, Some(boundary), 3));
        assert_eq!(values.len(), before_len);
        assert_eq!(
            values.ids().collect::<std::vec::Vec<_>>(),
            [first, boundary, second]
        );
    }

    #[test]
    fn stable_batch_at_the_arena_tail_appends_primaries() {
        let allocator = Allocator::new();
        let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
        let first = values.push_primary(10);
        let inserted = values.insert_stable_range_between(first, None, [20, 30]);

        assert!(inserted.start_id().is_primary());
        assert_eq!(values.primary_iter().len(), 3);
        assert_eq!(
            values
                .iter_range(inserted)
                .unwrap()
                .copied()
                .collect::<std::vec::Vec<_>>(),
            [20, 30]
        );
    }
}
