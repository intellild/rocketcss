//! Arena-backed sequence optimized for parse-first, insert-rare workloads.
//!
//! Parsed values live in one linear arena [`Vec`] and use the high 20 bits of
//! [`RadixIndexId`] as their direct index. Rare values inserted after a parsed
//! value live in lazy two-level radix trees indexed by a separate sparse `u32`
//! vector.
//!
//! ```text
//! 31                         12 11                         2 1         0
//! +----------------------------+---------------------------+-----------+
//! |      primary index: 20     |    sibling key: 10        | property  |
//! +----------------------------+---------------------------+-----------+
//! ```
//!
//! A zero sibling key addresses the primary linear value. A nonzero key uses
//! two five-bit radix levels. The low two bits encode up to four declaration
//! properties without affecting primary or sibling lookup.

use std::{fmt, hash::Hash, marker::PhantomData, ops::Index, ptr::NonNull};

use crate::{Allocator, dense::NonMaxU32, vec::Vec};

const PRIMARY_BITS: u32 = 20;
const LOCAL_BITS: u32 = u32::BITS - PRIMARY_BITS;
const SIBLING_BITS: u32 = 10;
const PROPERTY_BITS: u32 = LOCAL_BITS - SIBLING_BITS;
const SIBLING_SHIFT: u32 = PROPERTY_BITS;
const RADIX_BITS: u32 = 5;
const RADIX_SIZE: usize = 1 << RADIX_BITS;
const SIBLING_MASK: u32 = (1 << SIBLING_BITS) - 1;
const PROPERTY_MASK: u32 = (1 << PROPERTY_BITS) - 1;
const PRIMARY_CAPACITY: usize = 1 << PRIMARY_BITS;

/// Stable compact identity for a value in [`RadixIndexArena`].
///
/// IDs whose [`sibling_key`](Self::sibling_key) is zero address the primary
/// parse vector directly. Other IDs address a rare inserted sibling through a
/// two-level radix tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RadixIndexId(NonMaxU32);

impl RadixIndexId {
    #[inline]
    fn from_parts(primary_index: usize, sibling_key: u16, property_index: u8) -> Self {
        debug_assert!(primary_index < PRIMARY_CAPACITY);
        debug_assert!(u32::from(sibling_key) <= SIBLING_MASK);
        debug_assert!(u32::from(property_index) <= PROPERTY_MASK);
        let encoded = ((primary_index as u32) << LOCAL_BITS)
            | (u32::from(sibling_key) << SIBLING_SHIFT)
            | u32::from(property_index);
        Self(NonMaxU32::new(encoded).expect("u32::MAX is reserved for an invalid radix ID"))
    }

    /// Returns the encoded ID as a `u32`.
    #[inline]
    pub const fn get(self) -> u32 {
        self.0.get()
    }

    /// Returns the index of the owning primary value.
    #[inline]
    pub const fn primary_index(self) -> usize {
        (self.get() >> LOCAL_BITS) as usize
    }

    /// Returns the ten-bit sibling key, or zero for a primary value.
    #[inline]
    pub const fn sibling_key(self) -> u16 {
        ((self.get() >> SIBLING_SHIFT) & SIBLING_MASK) as u16
    }

    /// Returns the two-bit declaration-property index.
    #[inline]
    pub const fn property_index(self) -> u8 {
        (self.get() & PROPERTY_MASK) as u8
    }

    /// Returns the same storage location with a declaration-property index.
    ///
    /// Storage lookup ignores these low two bits. Mutation APIs require the
    /// zero property index so a property sub-ID cannot remove an entire value.
    #[inline]
    pub fn with_property_index(self, property_index: u8) -> Self {
        assert!(
            u32::from(property_index) <= PROPERTY_MASK,
            "property index must be in 0..=3"
        );
        let encoded = (self.get() & !PROPERTY_MASK) | u32::from(property_index);
        Self(NonMaxU32::new(encoded).expect("u32::MAX is reserved for an invalid radix ID"))
    }

    /// Returns whether this ID addresses the primary linear vector.
    #[inline]
    pub const fn is_primary(self) -> bool {
        self.sibling_key() == 0
    }
}

/// A domain-specific identity backed by [`RadixIndexId`].
///
/// Implement IDs with [`define_radix_id!`] instead of implementing this trait
/// manually. Its conversion methods are public only so the generated
/// implementation can live in another crate.
#[doc(hidden)]
pub trait RadixId: Copy + Eq + Ord + Hash + fmt::Debug {
    #[doc(hidden)]
    fn from_radix_index(id: RadixIndexId) -> Self;

    #[doc(hidden)]
    fn radix_index(self) -> RadixIndexId;
}

impl RadixId for RadixIndexId {
    #[inline]
    fn from_radix_index(id: RadixIndexId) -> Self {
        id
    }

    #[inline]
    fn radix_index(self) -> RadixIndexId {
        self
    }
}

/// Declares an opaque typed ID for a [`RadixIndexArena`].
///
/// The generated type preserves the four-byte encoded representation while
/// preventing IDs from different AST stores from being mixed.
#[macro_export]
macro_rules! define_radix_id {
    ($(#[$attribute:meta])* $visibility:vis struct $name:ident $(;)?) => {
        $(#[$attribute])*
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $visibility struct $name($crate::radix_index_arena::RadixIndexId);

        impl $name {
            /// Returns the complete encoded ID.
            #[inline]
            $visibility const fn get(self) -> u32 {
                self.0.get()
            }

            /// Returns the owning primary-vector index.
            #[inline]
            $visibility const fn primary_index(self) -> usize {
                self.0.primary_index()
            }

            /// Returns zero for a primary value or the local sibling key.
            #[inline]
            $visibility const fn sibling_key(self) -> u16 {
                self.0.sibling_key()
            }

            /// Returns the low declaration-property sub-index.
            #[inline]
            $visibility const fn property_index(self) -> u8 {
                self.0.property_index()
            }

            /// Returns whether this ID addresses a primary value.
            #[inline]
            $visibility const fn is_primary(self) -> bool {
                self.0.is_primary()
            }

            /// Returns the same storage location with a property sub-index.
            #[inline]
            $visibility fn with_property_index(self, property_index: u8) -> Self {
                Self(self.0.with_property_index(property_index))
            }
        }

        impl $crate::radix_index_arena::RadixId for $name {
            #[inline]
            fn from_radix_index(id: $crate::radix_index_arena::RadixIndexId) -> Self {
                Self(id)
            }

            #[inline]
            fn radix_index(self) -> $crate::radix_index_arena::RadixIndexId {
                self.0
            }
        }
    };
}

struct RadixTree<'arena, T> {
    root: Option<NonNull<RadixRoot<T>>>,
    marker: PhantomData<&'arena RadixRoot<T>>,
}

impl<T> RadixTree<'_, T> {
    #[inline]
    fn new() -> Self {
        Self {
            root: None,
            marker: PhantomData,
        }
    }

    fn insert(&mut self, allocator: &Allocator, key: u16, value: T) {
        debug_assert!(key != 0 && u32::from(key) <= SIBLING_MASK);
        let (high, low) = radix_parts(key);
        let root = match self.root {
            Some(mut root) => {
                // SAFETY: roots are allocated in `allocator`, never moved, and
                // `&mut self` prevents another access through this tree.
                unsafe { root.as_mut() }
            }
            None => {
                let root = NonNull::from(allocator.alloc(RadixRoot::new()));
                self.root = Some(root);
                // SAFETY: the pointer was just allocated and is uniquely
                // reachable through `&mut self`.
                unsafe { &mut *root.as_ptr() }
            }
        };
        let leaf = match root.children[high] {
            Some(mut leaf) => {
                // SAFETY: leaves are allocated in `allocator`, never moved, and
                // `&mut self` prevents another access through this tree.
                unsafe { leaf.as_mut() }
            }
            None => {
                let leaf = NonNull::from(allocator.alloc(RadixLeaf::new()));
                root.children[high] = Some(leaf);
                // SAFETY: the pointer was just allocated and is uniquely
                // reachable through `&mut self`.
                unsafe { &mut *leaf.as_ptr() }
            }
        };
        assert_eq!(
            leaf.used & (1 << low),
            0,
            "radix sibling key was already allocated"
        );
        leaf.values[low] = Some(value);
        leaf.occupied |= 1 << low;
        leaf.used |= 1 << low;
        root.occupied |= 1 << high;
    }

    fn restore(&mut self, key: u16, value: T) {
        let (high, low) = radix_parts(key);
        let mut root = self
            .root
            .expect("a previously allocated key must have a radix root");
        // SAFETY: `&mut self` uniquely borrows the tree and its arena pages.
        let root = unsafe { root.as_mut() };
        let mut leaf =
            root.children[high].expect("a previously allocated key must have a radix leaf");
        // SAFETY: `&mut self` uniquely borrows the tree and its arena pages.
        let leaf = unsafe { leaf.as_mut() };
        let bit = 1 << low;
        assert_ne!(leaf.used & bit, 0, "restore requires an allocated key");
        assert_eq!(leaf.occupied & bit, 0, "restore requires a retired key");
        assert!(leaf.values[low].is_none(), "retired key must be empty");
        leaf.values[low] = Some(value);
        leaf.occupied |= bit;
        root.occupied |= 1 << high;
    }

    #[inline]
    fn get(&self, key: u16) -> Option<&T> {
        let (high, low) = radix_parts(key);
        // SAFETY: root and leaf pointers refer to arena allocations that remain
        // live for `'arena`; the returned reference is bounded by `&self`.
        let root = unsafe { self.root?.as_ref() };
        let leaf = unsafe { root.children[high]?.as_ref() };
        leaf.values[low].as_ref()
    }

    #[inline]
    fn get_mut(&mut self, key: u16) -> Option<&mut T> {
        let (high, low) = radix_parts(key);
        let mut root = self.root?;
        // SAFETY: `&mut self` exclusively borrows the tree and arena nodes do
        // not move, so these mutable references cannot alias another access.
        let root = unsafe { root.as_mut() };
        let mut leaf = root.children[high]?;
        let leaf = unsafe { leaf.as_mut() };
        leaf.values[low].as_mut()
    }

    #[inline]
    fn take(&mut self, key: u16, reusable: bool) -> Option<T> {
        let (high, low) = radix_parts(key);
        let mut root = self.root?;
        // SAFETY: `&mut self` exclusively borrows the tree and arena nodes do
        // not move, so these mutable references cannot alias another access.
        let root = unsafe { root.as_mut() };
        let mut leaf = root.children[high]?;
        let leaf = unsafe { leaf.as_mut() };
        let value = leaf.values[low].take()?;
        leaf.occupied &= !(1 << low);
        if reusable {
            leaf.used &= !(1 << low);
        }
        if leaf.occupied == 0 {
            root.occupied &= !(1 << high);
        }
        Some(value)
    }

    #[inline]
    fn is_used(&self, key: u16) -> bool {
        let (high, low) = radix_parts(key);
        // SAFETY: root and leaf pointers remain live for the arena lifetime and
        // the references are bounded by `&self`.
        let Some(root) = (unsafe { self.root.map(|root| root.as_ref()) }) else {
            return false;
        };
        let Some(leaf) = (unsafe { root.children[high].map(|leaf| leaf.as_ref()) }) else {
            return false;
        };
        leaf.used & (1 << low) != 0
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

    fn used_len(&self) -> usize {
        let Some(root) = (unsafe { self.root.map(|root| root.as_ref()) }) else {
            return 0;
        };
        let mut len = 0;
        for leaf in root.children.iter().flatten() {
            // SAFETY: leaves are arena allocated and remain live for the tree.
            len += unsafe { leaf.as_ref() }.used.count_ones() as usize;
        }
        len
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

    fn iter(&self) -> Option<RadixTreeIter<'_, T>> {
        // SAFETY: the root is arena allocated and the produced reference is
        // bounded by the borrow of this tree.
        let root = unsafe { self.root?.as_ref() };
        if root.occupied == 0 {
            return None;
        }
        Some(RadixTreeIter {
            root,
            branches: root.occupied,
            leaf: None,
            slots: 0,
            branch: 0,
        })
    }
}

struct RadixRoot<T> {
    children: [Option<NonNull<RadixLeaf<T>>>; RADIX_SIZE],
    occupied: u32,
}

impl<T> RadixRoot<T> {
    fn new() -> Self {
        Self {
            children: [None; RADIX_SIZE],
            occupied: 0,
        }
    }
}

struct RadixLeaf<T> {
    values: [Option<T>; RADIX_SIZE],
    occupied: u32,
    used: u32,
}

impl<T> RadixLeaf<T> {
    fn new() -> Self {
        Self {
            values: [const { None }; RADIX_SIZE],
            occupied: 0,
            used: 0,
        }
    }
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
/// any primary value. Iteration emits each primary value followed by its
/// siblings in ascending key order.
pub type RadixIndexArena<'arena, T> = TypedRadixIndexArena<'arena, T, RadixIndexId>;

/// A [`RadixIndexArena`] whose IDs are isolated to one domain-specific type.
///
/// ```compile_fail
/// use rocketcss_common::{Allocator, TypedRadixIndexArena, define_radix_id};
///
/// define_radix_id!(struct RuleId);
/// define_radix_id!(struct BlockId);
///
/// let allocator = Allocator::new();
/// let mut rules = TypedRadixIndexArena::<_, RuleId>::new_in(&allocator);
/// let blocks = TypedRadixIndexArena::<_, BlockId>::new_in(&allocator);
/// let rule = rules.push_primary(1_u8);
/// let _ = blocks.get(rule);
/// ```
pub struct TypedRadixIndexArena<'arena, T: Unpin, I: RadixId> {
    allocator: &'arena Allocator,
    primary: Vec<'arena, T>,
    // These structure-of-arrays vectors share indices. Binary search touches
    // only compact primary IDs; tree pointers are loaded after a match.
    sibling_primary_indices: Vec<'arena, u32>,
    sibling_trees: Vec<'arena, RadixTree<'arena, T>>,
    len: u32,
    id: PhantomData<fn(I) -> I>,
}

impl<'arena, T: Unpin, I: RadixId> TypedRadixIndexArena<'arena, T, I> {
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
        assert!(
            capacity <= PRIMARY_CAPACITY,
            "RadixIndexArena primary capacity exhausted"
        );
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
        (index < self.primary.len())
            .then(|| I::from_radix_index(RadixIndexId::from_parts(index, 0, 0)))
    }

    /// Iterates only authored primary values in their parse order.
    #[inline]
    pub fn primary_iter(&self) -> std::slice::Iter<'_, T> {
        self.primary.iter()
    }

    /// Iterates typed IDs and authored primary values in parse order.
    #[inline]
    pub fn primary_iter_enumerated(&self) -> impl ExactSizeIterator<Item = (I, &T)> + '_ {
        self.primary.iter().enumerate().map(|(index, value)| {
            (
                I::from_radix_index(RadixIndexId::from_parts(index, 0, 0)),
                value,
            )
        })
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
        self.primary.len() < PRIMARY_CAPACITY && self.len < u32::MAX
    }

    /// Appends a parsed value and returns its direct primary ID.
    ///
    /// # Panics
    ///
    /// Panics after `2^20` primary values or `u32::MAX` total values.
    pub fn push_primary(&mut self, value: T) -> I {
        assert!(
            self.can_push_primary(),
            "RadixIndexArena primary capacity exhausted"
        );
        let id = RadixIndexId::from_parts(self.primary.len(), 0, 0);
        self.primary.push(value);
        self.len += 1;
        I::from_radix_index(id)
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
        let primary = primary.radix_index();
        assert!(primary.is_primary(), "sibling owner must be a primary ID");
        assert_eq!(
            primary.property_index(),
            0,
            "sibling owner must not be a property sub-ID"
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
        I::from_radix_index(RadixIndexId::from_parts(primary_index, sibling_key, 0))
    }

    /// Returns whether another live sibling can be represented below `primary`.
    ///
    /// This is the AST wrapper's preflight for choosing its overflow fallback
    /// before moving a value into the compact store.
    pub fn can_insert_sibling(&self, primary: I) -> bool {
        let primary = primary.radix_index();
        if !primary.is_primary()
            || primary.property_index() != 0
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
            let primary = I::from_radix_index(RadixIndexId::from_parts(primary_index, 0, 0));
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
        let id = id.radix_index();
        if id.is_primary() || id.property_index() != 0 {
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
        let id = id.radix_index();
        if id.is_primary() || id.property_index() != 0 {
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
        let id = id.radix_index();
        if id.is_primary() {
            self.primary.get(id.primary_index())
        } else {
            let group_index = self.sibling_group_index(id.primary_index()).ok()?;
            self.sibling_trees.get(group_index)?.get(id.sibling_key())
        }
    }

    /// Mutably resolves an ID to its value.
    #[inline]
    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        let id = id.radix_index();
        if id.is_primary() {
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
            IterKind::Primary(self.primary.iter())
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
        let after = after.radix_index();
        let before = before.map(RadixId::radix_index);
        if after.property_index() != 0 || self.get(I::from_radix_index(after)).is_none() {
            return None;
        }
        if before.is_some_and(|before| {
            before.property_index() != 0 || self.get(I::from_radix_index(before)).is_none()
        }) {
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
            None if primary_index + 1 == self.primary.len() => (SIBLING_MASK + 1) as u16,
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
                tree.restore(key, value);
            } else {
                tree.insert(self.allocator, key, value);
            }
            let new = I::from_radix_index(RadixIndexId::from_parts(primary_index, key, 0));
            if index == insertion_index {
                inserted = Some(new);
            } else if old_key != key {
                remaps.push(RadixIdRemap {
                    old: I::from_radix_index(RadixIndexId::from_parts(primary_index, old_key, 0)),
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

impl<T: Unpin, I: RadixId> Index<I> for TypedRadixIndexArena<'_, T, I> {
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

/// Iterator that merges primary values with sparse sibling radix trees.
pub struct SemanticIter<'tree, 'arena, T: Unpin> {
    primary: &'tree [T],
    next_primary: usize,
    sibling_primary_indices: &'tree [u32],
    sibling_trees: &'tree [RadixTree<'arena, T>],
    next_group: usize,
    current: SemanticSegment<'tree, T>,
}

/// Iterator over typed IDs and values in semantic order.
pub struct SemanticIterEnumerated<'tree, 'arena, T: Unpin, I: RadixId> {
    primary: &'tree [T],
    next_primary: usize,
    current_primary: usize,
    sibling_primary_indices: &'tree [u32],
    sibling_trees: &'tree [RadixTree<'arena, T>],
    next_group: usize,
    sibling_primary: usize,
    current: SemanticSegment<'tree, T>,
    remaining: u32,
    id: PhantomData<fn(I) -> I>,
}

enum SemanticSegment<'tree, T> {
    Primary(std::slice::Iter<'tree, T>),
    Siblings(RadixTreeIter<'tree, T>),
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

impl<'tree, 'arena: 'tree, T: Unpin, I: RadixId> Iterator
    for SemanticIterEnumerated<'tree, 'arena, T, I>
{
    type Item = (I, &'tree T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.current {
                SemanticSegment::Primary(primary) => {
                    if let Some(value) = primary.next() {
                        let id = RadixIndexId::from_parts(self.current_primary, 0, 0);
                        self.current_primary += 1;
                        self.remaining -= 1;
                        return Some((I::from_radix_index(id), value));
                    }
                }
                SemanticSegment::Siblings(siblings) => {
                    if let Some((key, value)) = siblings.next_enumerated() {
                        let id = RadixIndexId::from_parts(self.sibling_primary, key, 0);
                        self.remaining -= 1;
                        return Some((I::from_radix_index(id), value));
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

impl<T: Unpin, I: RadixId> ExactSizeIterator for SemanticIterEnumerated<'_, '_, T, I> {}
impl<T: Unpin, I: RadixId> std::iter::FusedIterator for SemanticIterEnumerated<'_, '_, T, I> {}

impl<'tree, 'arena: 'tree, T: Unpin, I: RadixId> SemanticIterEnumerated<'tree, 'arena, T, I> {
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

impl<'tree, 'arena: 'tree, T: Unpin, I: RadixId> IntoIterator
    for &'tree TypedRadixIndexArena<'arena, T, I>
{
    type Item = &'tree T;
    type IntoIter = Iter<'tree, 'arena, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct RadixTreeIter<'tree, T> {
    root: &'tree RadixRoot<T>,
    branches: u32,
    leaf: Option<&'tree RadixLeaf<T>>,
    slots: u32,
    branch: usize,
}

impl<'tree, T> RadixTreeIter<'tree, T> {
    #[inline]
    fn next_enumerated(&mut self) -> Option<(u16, &'tree T)> {
        loop {
            if self.slots != 0 {
                let slot = self.slots.trailing_zeros() as usize;
                self.slots &= self.slots - 1;
                let leaf = self.leaf?;
                let key = ((self.current_branch() << RADIX_BITS) | slot) as u16;
                return leaf.values[slot].as_ref().map(|value| (key, value));
            }

            let branch = self.branches.trailing_zeros() as usize;
            if branch == u32::BITS as usize {
                return None;
            }
            self.branches &= self.branches - 1;
            let leaf_pointer =
                self.root.children[branch].expect("occupied radix branch always contains a leaf");
            // SAFETY: leaves are arena allocated and the iterator is bounded by
            // the immutable borrow of its owning radix tree.
            let leaf: &'tree RadixLeaf<T> = unsafe { leaf_pointer.as_ref() };
            self.slots = leaf.occupied;
            self.leaf = Some(leaf);
            self.branch = branch;
        }
    }

    #[inline]
    fn current_branch(&self) -> usize {
        self.branch
    }
}

impl<'tree, T> Iterator for RadixTreeIter<'tree, T> {
    type Item = &'tree T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.slots != 0 {
                let slot = self.slots.trailing_zeros() as usize;
                self.slots &= self.slots - 1;
                return self.leaf?.values[slot].as_ref();
            }

            let branch = self.branches.trailing_zeros() as usize;
            if branch == u32::BITS as usize {
                return None;
            }
            self.branches &= self.branches - 1;
            let leaf_pointer =
                self.root.children[branch].expect("occupied radix branch always contains a leaf");
            // SAFETY: leaves are arena allocated and the iterator is bounded by
            // the immutable borrow of its owning radix tree.
            let leaf: &'tree RadixLeaf<T> = unsafe { leaf_pointer.as_ref() };
            self.slots = leaf.occupied;
            self.leaf = Some(leaf);
        }
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
        LOCAL_BITS, PRIMARY_CAPACITY, RadixIndexArena, RadixIndexId, SIBLING_MASK,
        TypedRadixIndexArena,
    };
    use crate::Allocator;

    crate::define_radix_id!(struct TestRuleId);

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
    fn siblings_use_two_radix_levels_and_iterate_by_key() {
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
    fn property_index_uses_the_low_two_bits() {
        let id = RadixIndexId::from_parts(0xabcde, 0x2aa, 3);
        assert_eq!(id.primary_index(), 0xabcde);
        assert_eq!(id.sibling_key(), 0x2aa);
        assert_eq!(id.property_index(), 3);
        assert_eq!(id.get() & 0b11, 3);
        assert_eq!((id.get() >> 2) & 0x3ff, 0x2aa);
    }

    #[test]
    fn radix_ids_reserve_u32_max_as_the_option_niche() {
        assert_eq!(std::mem::size_of::<RadixIndexId>(), 4);
        assert_eq!(std::mem::size_of::<Option<RadixIndexId>>(), 4);

        let highest_property_sub_id =
            RadixIndexId::from_parts(PRIMARY_CAPACITY - 1, SIBLING_MASK as u16, 2);
        assert_eq!(highest_property_sub_id.get(), u32::MAX - 1);
    }

    #[test]
    fn property_sub_ids_resolve_the_same_storage_value() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(1);
        let sibling = values.insert_sibling(primary, 17, 2);

        assert_eq!(values[primary.with_property_index(3)], 1);
        assert_eq!(values[sibling.with_property_index(2)], 2);
        assert_eq!(values.remove_sibling(sibling.with_property_index(1)), None);
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
        assert_eq!(second.primary_index(), 1);
        assert_eq!(inserted.sibling_key(), 512);
        assert_eq!(values.get(inserted), Some(&20));
        assert_eq!(inserted.with_property_index(3).property_index(), 3);
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
