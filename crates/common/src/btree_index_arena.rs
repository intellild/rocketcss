//! Arena-backed order-statistic B+ sequence.
//!
//! [`BTreeIndexArena`] keeps values in fixed-capacity leaf pages and stores the
//! length of every child subtree in internal pages. Tree pages are allocated
//! directly in the compiler [`Allocator`], so traversal does not need a
//! separate node table or node IDs.

use std::{
    cell::Cell,
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use crate::{Allocator, boxed::Box, vec::Vec};

const DEFAULT_PAGE_CAPACITY: usize = 32;

struct Leaf<'arena, T: Unpin> {
    values: Vec<'arena, T>,
    previous: Cell<Option<NonNull<Leaf<'arena, T>>>>,
    next: Cell<Option<NonNull<Leaf<'arena, T>>>>,
}

struct Internal<'arena, T: Unpin> {
    children: Vec<'arena, Box<'arena, Node<'arena, T>>>,
    subtree_lens: Vec<'arena, u32>,
}

enum Node<'arena, T: Unpin> {
    Leaf(Leaf<'arena, T>),
    Internal(Internal<'arena, T>),
}

impl<T: Unpin> Node<'_, T> {
    #[inline]
    fn len(&self) -> u32 {
        match self {
            Self::Leaf(leaf) => leaf.values.len() as u32,
            Self::Internal(internal) => internal.subtree_lens.iter().copied().sum(),
        }
    }
}

/// An arena-backed B+ sequence with order-statistic indexing.
///
/// Values are stored directly in leaf pages. Callers remain free to choose a
/// pointer type such as [`Box`] for `T` when stable value addresses are needed.
/// Internal pages store arena-owned child pages and subtree lengths, so lookup,
/// insertion, and removal descend in `O(log_B N)` time and move at most `O(B)`
/// values in one page. Empty pages are unlinked, while underfull nonempty pages
/// are retained for the compilation lifetime.
///
/// `PAGE_CAPACITY` controls the maximum number of leaf values or internal
/// children before a split. It must be at least four.
pub struct BTreeIndexArena<'arena, T: Unpin, const PAGE_CAPACITY: usize = DEFAULT_PAGE_CAPACITY> {
    allocator: &'arena Allocator,
    root: Option<Box<'arena, Node<'arena, T>>>,
    first_leaf: NonNull<Leaf<'arena, T>>,
    len: u32,
}

impl<'arena, T: Unpin, const PAGE_CAPACITY: usize> BTreeIndexArena<'arena, T, PAGE_CAPACITY> {
    /// Creates an empty sequence in `allocator`.
    pub fn new_in(allocator: &'arena Allocator) -> Self {
        assert!(
            PAGE_CAPACITY >= 4,
            "BTreeIndexArena page capacity must be at least four"
        );

        let mut root = new_leaf::<T, PAGE_CAPACITY>(allocator);
        let first_leaf = match &mut *root {
            Node::Leaf(leaf) => NonNull::from(leaf),
            Node::Internal(_) => unreachable!(),
        };

        Self {
            allocator,
            root: Some(root),
            first_leaf,
            len: 0,
        }
    }

    /// Returns the number of values in the sequence.
    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns whether the sequence contains no values.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Appends `value` to the sequence.
    #[inline]
    pub fn push(&mut self, value: T) {
        self.insert(self.len(), value);
    }

    /// Inserts `value` at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`, the sequence exceeds `u32::MAX` entries, or
    /// `PAGE_CAPACITY` is smaller than four.
    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len(), "insertion index out of bounds");
        assert!(self.len < u32::MAX, "BTreeIndexArena capacity exhausted");

        let split = insert_node::<T, PAGE_CAPACITY>(
            self.allocator,
            self.root.as_deref_mut().expect("tree always has a root"),
            index,
            value,
        );
        if let Some(right) = split {
            let left = self.root.take().expect("tree always has a root");
            let mut children = Vec::with_capacity_in(PAGE_CAPACITY + 1, self.allocator);
            let mut subtree_lens = Vec::with_capacity_in(PAGE_CAPACITY + 1, self.allocator);
            subtree_lens.push(left.len());
            subtree_lens.push(right.len());
            children.push(left);
            children.push(right);
            self.root = Some(self.allocator.boxed(Node::Internal(Internal {
                children,
                subtree_lens,
            })));
        }
        self.len += 1;
    }

    /// Removes and returns the value at `index`.
    ///
    /// Empty leaf pages are unlinked. Nonempty underfull pages are retained to
    /// avoid paying merge costs in mutation-heavy compiler passes.
    ///
    /// # Panics
    ///
    /// Panics if `index >= len`.
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len(), "removal index out of bounds");
        let (value, _) = remove_node(
            self.root.as_deref_mut().expect("tree always has a root"),
            index,
            true,
            &mut self.first_leaf,
        );
        self.len -= 1;
        self.collapse_root();
        value
    }

    /// Returns a shared reference to the value at `index`.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        get_node(self.root.as_deref().expect("tree always has a root"), index)
    }

    /// Returns a mutable reference to the value at `index`.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            return None;
        }
        get_node_mut(
            self.root.as_deref_mut().expect("tree always has a root"),
            index,
        )
    }

    /// Iterates values in sequence order.
    #[inline]
    pub fn iter(&self) -> Iter<'_, 'arena, T, PAGE_CAPACITY> {
        Iter {
            _tree: self,
            leaf: Some(self.first_leaf),
            offset: 0,
            remaining: self.len,
        }
    }

    fn collapse_root(&mut self) {
        loop {
            let only_child = match self.root.as_deref_mut().expect("tree always has a root") {
                Node::Internal(internal) if internal.children.len() == 1 => {
                    Some(internal.children.remove(0))
                }
                _ => None,
            };
            match only_child {
                Some(child) => self.root = Some(child),
                None => break,
            }
        }

        if self.len == 0
            && !matches!(
                self.root.as_deref().expect("tree always has a root"),
                Node::Leaf(_)
            )
        {
            let mut root = new_leaf::<T, PAGE_CAPACITY>(self.allocator);
            self.first_leaf = match &mut *root {
                Node::Leaf(leaf) => NonNull::from(leaf),
                Node::Internal(_) => unreachable!(),
            };
            self.root = Some(root);
        }
    }
}

impl<T: Unpin, const PAGE_CAPACITY: usize> Index<usize> for BTreeIndexArena<'_, T, PAGE_CAPACITY> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .expect("BTreeIndexArena index out of bounds")
    }
}

impl<T: Unpin, const PAGE_CAPACITY: usize> IndexMut<usize>
    for BTreeIndexArena<'_, T, PAGE_CAPACITY>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .expect("BTreeIndexArena index out of bounds")
    }
}

/// Iterator over a [`BTreeIndexArena`] in sequence order.
pub struct Iter<'tree, 'arena, T: Unpin, const PAGE_CAPACITY: usize = DEFAULT_PAGE_CAPACITY> {
    _tree: &'tree BTreeIndexArena<'arena, T, PAGE_CAPACITY>,
    leaf: Option<NonNull<Leaf<'arena, T>>>,
    offset: usize,
    remaining: u32,
}

impl<'tree, 'arena: 'tree, T: Unpin, const PAGE_CAPACITY: usize> Iterator
    for Iter<'tree, 'arena, T, PAGE_CAPACITY>
{
    type Item = &'tree T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.remaining != 0 {
            let leaf_pointer = self.leaf?;
            // SAFETY: leaf pages are arena allocated and therefore do not move.
            // `_tree` keeps the tree immutably borrowed for the iterator's
            // lifetime, and empty leaves are unlinked before they can be removed
            // from the tree.
            let leaf: &'tree Leaf<'arena, T> = unsafe { leaf_pointer.as_ref() };
            if let Some(value) = leaf.values.get(self.offset) {
                self.offset += 1;
                self.remaining -= 1;
                return Some(value);
            }
            self.leaf = leaf.next.get();
            self.offset = 0;
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining as usize;
        (remaining, Some(remaining))
    }
}

impl<T: Unpin, const PAGE_CAPACITY: usize> ExactSizeIterator for Iter<'_, '_, T, PAGE_CAPACITY> {}

impl<T: Unpin, const PAGE_CAPACITY: usize> std::iter::FusedIterator
    for Iter<'_, '_, T, PAGE_CAPACITY>
{
}

impl<'tree, 'arena: 'tree, T: Unpin, const PAGE_CAPACITY: usize> IntoIterator
    for &'tree BTreeIndexArena<'arena, T, PAGE_CAPACITY>
{
    type Item = &'tree T;
    type IntoIter = Iter<'tree, 'arena, T, PAGE_CAPACITY>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn new_leaf<'arena, T: Unpin, const PAGE_CAPACITY: usize>(
    allocator: &'arena Allocator,
) -> Box<'arena, Node<'arena, T>> {
    allocator.boxed(Node::Leaf(Leaf {
        values: Vec::with_capacity_in(PAGE_CAPACITY + 1, allocator),
        previous: Cell::new(None),
        next: Cell::new(None),
    }))
}

fn insert_node<'arena, T: Unpin, const PAGE_CAPACITY: usize>(
    allocator: &'arena Allocator,
    node: &mut Node<'arena, T>,
    index: usize,
    value: T,
) -> Option<Box<'arena, Node<'arena, T>>> {
    match node {
        Node::Leaf(leaf) => leaf.values.insert(index, value),
        Node::Internal(internal) => {
            let (child_index, child_offset) = locate_child(&internal.subtree_lens, index, true);
            let child_split = insert_node::<_, PAGE_CAPACITY>(
                allocator,
                &mut internal.children[child_index],
                child_offset,
                value,
            );
            internal.subtree_lens[child_index] = internal.children[child_index].len();
            if let Some(right) = child_split {
                let right_len = right.len();
                internal.children.insert(child_index + 1, right);
                internal.subtree_lens.insert(child_index + 1, right_len);
            }
        }
    }

    split_if_needed::<_, PAGE_CAPACITY>(allocator, node)
}

fn split_if_needed<'arena, T: Unpin, const PAGE_CAPACITY: usize>(
    allocator: &'arena Allocator,
    node: &mut Node<'arena, T>,
) -> Option<Box<'arena, Node<'arena, T>>> {
    match node {
        Node::Leaf(leaf) if leaf.values.len() > PAGE_CAPACITY => {
            let middle = leaf.values.len() / 2;
            let right_values = leaf.values.split_off(middle);
            let old_next = leaf.next.get();
            let left_pointer = NonNull::from(&mut *leaf);
            let mut right = allocator.boxed(Node::Leaf(Leaf {
                values: right_values,
                previous: Cell::new(Some(left_pointer)),
                next: Cell::new(old_next),
            }));
            let right_pointer = match &mut *right {
                Node::Leaf(right_leaf) => NonNull::from(right_leaf),
                Node::Internal(_) => unreachable!(),
            };
            leaf.next.set(Some(right_pointer));
            if let Some(old_next) = old_next {
                // SAFETY: every link points to a distinct arena-allocated leaf.
                // Leaves do not move, and link mutation uses `Cell`, so no
                // mutable reference aliases the active `leaf` reference.
                unsafe { old_next.as_ref() }
                    .previous
                    .set(Some(right_pointer));
            }
            Some(right)
        }
        Node::Internal(internal) if internal.children.len() > PAGE_CAPACITY => {
            let middle = internal.children.len() / 2;
            Some(allocator.boxed(Node::Internal(Internal {
                children: internal.children.split_off(middle),
                subtree_lens: internal.subtree_lens.split_off(middle),
            })))
        }
        Node::Leaf(_) | Node::Internal(_) => None,
    }
}

fn remove_node<'arena, T: Unpin>(
    node: &mut Node<'arena, T>,
    index: usize,
    is_root: bool,
    first_leaf: &mut NonNull<Leaf<'arena, T>>,
) -> (T, bool) {
    match node {
        Node::Leaf(leaf) => {
            let value = leaf.values.remove(index);
            let empty = leaf.values.is_empty();
            if empty && !is_root {
                unlink_leaf(NonNull::from(leaf), first_leaf);
            }
            (value, empty)
        }
        Node::Internal(internal) => {
            let (child_index, child_offset) = locate_child(&internal.subtree_lens, index, false);
            let (value, child_empty) = remove_node(
                &mut internal.children[child_index],
                child_offset,
                false,
                first_leaf,
            );
            if child_empty {
                internal.children.remove(child_index);
                internal.subtree_lens.remove(child_index);
            } else {
                internal.subtree_lens[child_index] = internal.children[child_index].len();
            }
            (value, internal.children.is_empty())
        }
    }
}

fn unlink_leaf<'arena, T: Unpin>(
    leaf_pointer: NonNull<Leaf<'arena, T>>,
    first_leaf: &mut NonNull<Leaf<'arena, T>>,
) {
    // SAFETY: leaf links are created only from arena-allocated leaf pages and
    // are updated before an empty leaf is detached from its parent.
    let leaf = unsafe { leaf_pointer.as_ref() };
    let previous = leaf.previous.get();
    let next = leaf.next.get();

    if let Some(previous) = previous {
        // SAFETY: `previous` is a live leaf in the same arena. Link fields use
        // interior mutability and do not alias leaf values.
        unsafe { previous.as_ref() }.next.set(next);
    } else if let Some(next) = next {
        *first_leaf = next;
    }
    if let Some(next) = next {
        // SAFETY: `next` is a live leaf in the same arena. Link fields use
        // interior mutability and do not alias leaf values.
        unsafe { next.as_ref() }.previous.set(previous);
    }
}

fn get_node<'tree, T: Unpin>(mut node: &'tree Node<'_, T>, mut index: usize) -> Option<&'tree T> {
    loop {
        match node {
            Node::Leaf(leaf) => return leaf.values.get(index),
            Node::Internal(internal) => {
                let (child_index, child_offset) =
                    locate_child(&internal.subtree_lens, index, false);
                node = &internal.children[child_index];
                index = child_offset;
            }
        }
    }
}

fn get_node_mut<'tree, T: Unpin>(
    mut node: &'tree mut Node<'_, T>,
    mut index: usize,
) -> Option<&'tree mut T> {
    loop {
        match node {
            Node::Leaf(leaf) => return leaf.values.get_mut(index),
            Node::Internal(internal) => {
                let (child_index, child_offset) =
                    locate_child(&internal.subtree_lens, index, false);
                node = &mut internal.children[child_index];
                index = child_offset;
            }
        }
    }
}

fn locate_child(subtree_lens: &[u32], index: usize, allow_end: bool) -> (usize, usize) {
    debug_assert!(!subtree_lens.is_empty());
    let mut remaining = index;
    for (child_index, &child_len) in subtree_lens.iter().enumerate() {
        let child_len = child_len as usize;
        if remaining < child_len
            || (allow_end && child_index + 1 == subtree_lens.len() && remaining == child_len)
        {
            return (child_index, remaining);
        }
        remaining -= child_len;
    }
    unreachable!("index is inside the node subtree")
}

#[cfg(test)]
mod tests {
    use super::BTreeIndexArena;
    use crate::Allocator;

    #[test]
    fn insert_get_mut_and_iterate() {
        let allocator = Allocator::new();
        let mut tree = BTreeIndexArena::<_, 4>::new_in(&allocator);

        tree.push(1);
        tree.push(3);
        tree.insert(1, 2);
        tree.insert(0, 0);
        tree.push(4);

        assert_eq!(tree.len(), 5);
        assert_eq!(tree.get(5), None);
        tree[2] = 20;
        assert_eq!(
            tree.iter().copied().collect::<std::vec::Vec<_>>(),
            [0, 1, 20, 3, 4]
        );
    }

    #[test]
    fn remove_across_empty_leaves_and_collapse_root() {
        let allocator = Allocator::new();
        let mut tree = BTreeIndexArena::<_, 4>::new_in(&allocator);
        for value in 0..128 {
            tree.push(value);
        }

        for expected in (0..128).rev() {
            assert_eq!(tree.remove(tree.len() - 1), expected);
        }

        assert!(tree.is_empty());
        tree.push(7);
        assert_eq!(tree[0], 7);
    }

    #[test]
    fn randomized_edits_match_vec() {
        let allocator = Allocator::new();
        let mut tree = BTreeIndexArena::<_, 4>::new_in(&allocator);
        let mut expected = std::vec::Vec::new();
        let mut state = 0x1234_5678_u32;

        for value in 0..4_000_u32 {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            if !expected.is_empty() && state.is_multiple_of(4) {
                let index = state as usize % expected.len();
                assert_eq!(tree.remove(index), expected.remove(index));
            } else {
                let index = state as usize % (expected.len() + 1);
                tree.insert(index, value);
                expected.insert(index, value);
            }

            assert_eq!(tree.len(), expected.len());
            assert_eq!(tree.iter().copied().collect::<std::vec::Vec<_>>(), expected);
            for (index, &value) in expected.iter().enumerate() {
                assert_eq!(tree.get(index), Some(&value));
            }
        }
    }

    #[test]
    fn caller_can_choose_arena_boxed_values() {
        let allocator = Allocator::new();
        let mut tree = BTreeIndexArena::<_, 4>::new_in(&allocator);

        tree.push(allocator.boxed(1));
        tree.push(allocator.boxed(2));

        assert_eq!(**tree.get(0).unwrap(), 1);
        assert_eq!(**tree.get(1).unwrap(), 2);
    }
}
