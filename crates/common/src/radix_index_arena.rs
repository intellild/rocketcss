//! Arena-backed sequence optimized for parse-first, insert-rare workloads.
//!
//! Parsed values live in one linear arena [`Vec`] and use the high 20 bits of
//! [`RadixIndexId`] as their direct index. Rare values inserted after a parsed
//! value live in a lazy two-level radix tree in a sparse sibling vector.
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

use std::{marker::PhantomData, ops::Index, ptr::NonNull};

use crate::{Allocator, vec::Vec};

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
pub struct RadixIndexId(u32);

impl RadixIndexId {
    #[inline]
    fn from_parts(primary_index: usize, sibling_key: u16, property_index: u8) -> Self {
        debug_assert!(primary_index < PRIMARY_CAPACITY);
        debug_assert!(u32::from(sibling_key) <= SIBLING_MASK);
        debug_assert!(u32::from(property_index) <= PROPERTY_MASK);
        Self(
            ((primary_index as u32) << LOCAL_BITS)
                | (u32::from(sibling_key) << SIBLING_SHIFT)
                | u32::from(property_index),
        )
    }

    /// Returns the encoded ID as a `u32`.
    #[inline]
    pub fn get(self) -> u32 {
        self.0
    }

    /// Returns the index of the owning primary value.
    #[inline]
    pub fn primary_index(self) -> usize {
        (self.0 >> LOCAL_BITS) as usize
    }

    /// Returns the ten-bit sibling key, or zero for a primary value.
    #[inline]
    pub fn sibling_key(self) -> u16 {
        ((self.0 >> SIBLING_SHIFT) & SIBLING_MASK) as u16
    }

    /// Returns the two-bit declaration-property index.
    #[inline]
    pub fn property_index(self) -> u8 {
        (self.0 & PROPERTY_MASK) as u8
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
        Self((self.0 & !PROPERTY_MASK) | u32::from(property_index))
    }

    /// Returns whether this ID addresses the primary linear vector.
    #[inline]
    pub fn is_primary(self) -> bool {
        self.sibling_key() == 0
    }
}

struct RadixTree<'arena, T> {
    root: Option<NonNull<RadixRoot<T>>>,
    marker: PhantomData<&'arena RadixRoot<T>>,
}

struct SiblingRadix<'arena, T> {
    primary_index: u32,
    tree: RadixTree<'arena, T>,
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
        assert!(leaf.values[low].is_none(), "duplicate radix sibling key");
        leaf.values[low] = Some(value);
        leaf.occupied |= 1 << low;
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
    fn remove(&mut self, key: u16) -> Option<T> {
        let (high, low) = radix_parts(key);
        let mut root = self.root?;
        // SAFETY: `&mut self` exclusively borrows the tree and arena nodes do
        // not move, so these mutable references cannot alias another access.
        let root = unsafe { root.as_mut() };
        let mut leaf = root.children[high]?;
        let leaf = unsafe { leaf.as_mut() };
        let value = leaf.values[low].take()?;
        leaf.occupied &= !(1 << low);
        if leaf.occupied == 0 {
            root.occupied &= !(1 << high);
        }
        Some(value)
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
}

impl<T> RadixLeaf<T> {
    fn new() -> Self {
        Self {
            values: [const { None }; RADIX_SIZE],
            occupied: 0,
        }
    }
}

/// A sequence with a linear primary store and lazy radix-tree insertions.
///
/// Parsing appends primary values with [`push_primary`](Self::push_primary),
/// which is the same amortized operation as pushing to an arena vector. Rare
/// inserted siblings are addressed by a nonzero ten-bit key and do not move
/// any primary value. Iteration emits each primary value followed by its
/// siblings in ascending key order.
pub struct RadixIndexArena<'arena, T: Unpin> {
    allocator: &'arena Allocator,
    primary: Vec<'arena, T>,
    sibling_groups: Vec<'arena, SiblingRadix<'arena, T>>,
    len: u32,
}

impl<'arena, T: Unpin> RadixIndexArena<'arena, T> {
    /// Creates an empty store in `allocator`.
    pub fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            allocator,
            primary: Vec::new_in(allocator),
            sibling_groups: Vec::new_in(allocator),
            len: 0,
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
            sibling_groups: Vec::new_in(allocator),
            len: 0,
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
    pub fn primary_id(&self, index: usize) -> Option<RadixIndexId> {
        (index < self.primary.len()).then(|| RadixIndexId::from_parts(index, 0, 0))
    }

    /// Iterates only authored primary values in their parse order.
    #[inline]
    pub fn primary_iter(&self) -> std::slice::Iter<'_, T> {
        self.primary.iter()
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

    /// Appends a parsed value and returns its direct primary ID.
    ///
    /// # Panics
    ///
    /// Panics after `2^20` primary values or `u32::MAX` total values.
    pub fn push_primary(&mut self, value: T) -> RadixIndexId {
        assert!(
            self.primary.len() < PRIMARY_CAPACITY,
            "RadixIndexArena primary capacity exhausted"
        );
        assert!(self.len < u32::MAX, "RadixIndexArena capacity exhausted");
        let id = RadixIndexId::from_parts(self.primary.len(), 0, 0);
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
    pub fn insert_sibling(
        &mut self,
        primary: RadixIndexId,
        sibling_key: u16,
        value: T,
    ) -> RadixIndexId {
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
                self.sibling_groups.insert(
                    index,
                    SiblingRadix {
                        primary_index: primary_index as u32,
                        tree: RadixTree::new(),
                    },
                );
                index
            }
        };
        self.sibling_groups[group_index]
            .tree
            .insert(self.allocator, sibling_key, value);
        self.len += 1;
        RadixIndexId::from_parts(primary_index, sibling_key, 0)
    }

    /// Removes an inserted sibling. Primary parsed values cannot be removed.
    pub fn remove_sibling(&mut self, id: RadixIndexId) -> Option<T> {
        if id.is_primary() || id.property_index() != 0 {
            return None;
        }
        let group_index = self.sibling_group_index(id.primary_index()).ok()?;
        let value = self
            .sibling_groups
            .get_mut(group_index)?
            .tree
            .remove(id.sibling_key())?;
        self.len -= 1;
        Some(value)
    }

    /// Resolves an ID to its value.
    #[inline]
    pub fn get(&self, id: RadixIndexId) -> Option<&T> {
        if id.is_primary() {
            self.primary.get(id.primary_index())
        } else {
            let group_index = self.sibling_group_index(id.primary_index()).ok()?;
            self.sibling_groups
                .get(group_index)?
                .tree
                .get(id.sibling_key())
        }
    }

    /// Mutably resolves an ID to its value.
    #[inline]
    pub fn get_mut(&mut self, id: RadixIndexId) -> Option<&mut T> {
        if id.is_primary() {
            self.primary.get_mut(id.primary_index())
        } else {
            let group_index = self.sibling_group_index(id.primary_index()).ok()?;
            self.sibling_groups
                .get_mut(group_index)?
                .tree
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
        let chunk_end = self
            .sibling_groups
            .first()
            .map_or(self.primary.len(), |group| group.primary_index as usize + 1);
        SemanticIter {
            primary: &self.primary,
            next_primary: 0,
            sibling_groups: &self.sibling_groups,
            next_group: 0,
            chunk_end,
            siblings: None,
        }
    }

    #[inline]
    fn sibling_group_index(&self, primary_index: usize) -> Result<usize, usize> {
        self.sibling_groups
            .binary_search_by_key(&(primary_index as u32), |group| group.primary_index)
    }
}

impl<T: Unpin> Index<RadixIndexId> for RadixIndexArena<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, id: RadixIndexId) -> &Self::Output {
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
    sibling_groups: &'tree [SiblingRadix<'arena, T>],
    next_group: usize,
    chunk_end: usize,
    siblings: Option<RadixTreeIter<'tree, T>>,
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
            if let Some(siblings) = &mut self.siblings
                && let Some(value) = siblings.next()
            {
                return Some(value);
            }
            self.siblings = None;

            if self.next_primary < self.chunk_end {
                let value = &self.primary[self.next_primary];
                self.next_primary += 1;
                return Some(value);
            }

            let group = self.sibling_groups.get(self.next_group)?;
            debug_assert_eq!(self.chunk_end, group.primary_index as usize + 1);
            self.siblings = group.tree.iter();
            self.next_group += 1;
            self.chunk_end = self
                .sibling_groups
                .get(self.next_group)
                .map_or(self.primary.len(), |group| group.primary_index as usize + 1);
        }
    }
}

impl<'tree, 'arena: 'tree, T: Unpin> IntoIterator for &'tree RadixIndexArena<'arena, T> {
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
}

impl<'tree, T> Iterator for RadixTreeIter<'tree, T> {
    type Item = &'tree T;

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
    use super::{LOCAL_BITS, RadixIndexArena, RadixIndexId};
    use crate::Allocator;

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

        assert!(values.sibling_groups.is_empty());

        let high_branch = values.insert_sibling(first, 512, 3);
        let low_branch = values.insert_sibling(first, 1, 1);
        let next_leaf = values.insert_sibling(first, 32, 2);

        assert_eq!(values.sibling_groups.len(), 1);
        assert_eq!(high_branch.sibling_key(), 512);
        assert_eq!(values[low_branch], 1);
        assert_eq!(values[next_leaf], 2);
        assert_eq!(values[high_branch], 3);
        assert_eq!(
            values.iter().copied().collect::<std::vec::Vec<_>>(),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(values.len(), 5);
        assert_eq!(values.primary_len(), 2);
        assert_eq!(second.primary_index(), 1);
    }

    #[test]
    fn remove_and_reuse_sibling_slot() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(1);
        let sibling = values.insert_sibling(primary, 17, 2);

        assert_eq!(values.remove_sibling(sibling), Some(2));
        assert_eq!(values.get(sibling), None);
        let replacement = values.insert_sibling(primary, 17, 3);
        *values.get_mut(replacement).unwrap() = 4;

        assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [1, 4]);
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
    fn property_sub_ids_resolve_the_same_storage_value() {
        let allocator = Allocator::new();
        let mut values = RadixIndexArena::new_in(&allocator);
        let primary = values.push_primary(1);
        let sibling = values.insert_sibling(primary, 17, 2);

        assert_eq!(values[primary.with_property_index(3)], 1);
        assert_eq!(values[sibling.with_property_index(2)], 2);
        assert_eq!(values.remove_sibling(sibling.with_property_index(1)), None);
    }
}
