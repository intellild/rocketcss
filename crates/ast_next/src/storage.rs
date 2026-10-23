//! Flat node identity and storage columns.

use std::{fmt, marker::PhantomData};

/// Stable four-byte identity of one AST node in an [`Ast`](crate::Ast).
///
/// `T` separates identity domains, and `'ast` ties the handle to the context
/// that produced it. The handle never dereferences itself; all node data is
/// read through the owning context.
#[repr(transparent)]
pub struct NodeId<'ast, T> {
    raw: u32,
    _domain: PhantomData<fn() -> (&'ast (), T)>,
}

impl<T> Clone for NodeId<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for NodeId<'_, T> {}

impl<T> PartialEq for NodeId<'_, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for NodeId<'_, T> {}

impl<T> PartialOrd for NodeId<'_, T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for NodeId<'_, T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T> std::hash::Hash for NodeId<'_, T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> fmt::Debug for NodeId<'_, T> {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("NodeId").field(&self.raw).finish()
    }
}

impl<'ast, T> NodeId<'ast, T> {
    #[inline]
    pub(crate) const fn from_raw(raw: u32) -> Self {
        Self {
            raw,
            _domain: PhantomData,
        }
    }

    #[inline]
    pub(crate) const fn raw(self) -> u32 {
        self.raw
    }
}

/// Logical storage contract implemented by every generated node handle.
pub trait NodeStorage<'ast>: Copy + Sized {
    /// Concrete kind stored for this handle.
    const KIND: NodeKind;

    /// Copyable logical data stored in the payload or the arena spill table.
    type Data: Copy;

    /// Returns the raw identity behind this typed handle.
    fn node_id(self) -> NodeId<'ast, Self::Data>;

    /// Creates a typed handle after the caller has verified the node kind.
    ///
    /// # Safety
    /// `id` must refer to a node of exactly this handle's logical type.
    unsafe fn from_node_id(id: NodeId<'ast, Self::Data>) -> Self;
}

/// Compact discriminator for one logical node type.
///
/// Zero is reserved, so a zeroed payload can never describe a published node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NodeKind(u16);

impl NodeKind {
    #[inline]
    pub(crate) const fn new(value: u16) -> Self {
        assert!(value != 0);
        Self(value)
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn value(self) -> u16 {
        self.0
    }
}

/// A fixed-width node payload. Small node data is stored inline; larger data
/// is bump-allocated and the payload stores its stable address.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub(crate) struct NodePayload {
    words: [u64; 2],
}

const _: () = assert!(std::mem::size_of::<NodePayload>() == 16);

impl NodePayload {
    #[inline]
    pub(crate) fn inline<T: Copy>(value: T) -> Self {
        assert!(std::mem::size_of::<T>() <= 16);
        let mut words = [0u64; 2];
        // SAFETY: the value fits and `words` provides at least its size.
        unsafe {
            words.as_mut_ptr().cast::<T>().write_unaligned(value);
        }
        Self { words }
    }

    #[inline]
    pub(crate) fn spilled<T>(value: &T) -> Self {
        Self {
            words: [(value as *const T).cast::<u64>() as usize as u64, 0],
        }
    }

    /// # Safety
    /// The payload must have been written by [`Self::inline`] for `T`.
    #[inline]
    pub(crate) unsafe fn read_inline<T: Copy>(self) -> T {
        assert!(std::mem::size_of::<T>() <= 16);
        // SAFETY: the caller guarantees the stored value has type `T`.
        unsafe { self.words.as_ptr().cast::<T>().read_unaligned() }
    }

    /// # Safety
    /// The payload must have been written by [`Self::spilled`] for `T`, and
    /// the pointed-to value must still be live.
    #[inline]
    pub(crate) unsafe fn read_spilled<'a, T>(self) -> &'a T {
        // SAFETY: the caller guarantees the pointer was written for `T` and
        // refers to an arena allocation that outlives the context.
        unsafe { &*(self.words[0] as usize as *const T) }
    }
}

/// True when node data of this size is stored directly in the payload.
#[inline]
pub(crate) const fn inline_data_size(size: usize) -> bool {
    size <= 16
}
