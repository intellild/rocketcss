//! Arena-owned AST context shared by every flattened node.

use rocketcss_common::Allocator;

use crate::{
    NodeId, NodeKind, NodePayload, NodeStorage, Span,
    storage::{inline_data_size, NodePayload as Payload},
};

/// Compiler-owned storage for one flattened syntax tree.
///
/// Nodes live in three parallel columns indexed by the same [`NodeId`]. Small
/// node data is stored inline in the payload column; larger data is allocated
/// once in the arena and the payload stores its stable address.
pub struct Ast<'ast> {
    allocator: &'ast Allocator,
    spans: Vec<Span>,
    kinds: Vec<NodeKind>,
    payloads: Vec<NodePayload>,
}

impl<'ast> Ast<'ast> {
    #[inline]
    pub fn new_in(allocator: &'ast Allocator) -> Self {
        Self {
            allocator,
            spans: Vec::new(),
            kinds: Vec::new(),
            payloads: Vec::new(),
        }
    }

    /// Allocates one node and returns its typed handle.
    #[inline]
    pub fn alloc<T: NodeStorage<'ast>>(&mut self, data: T::Data, span: Span) -> T {
        let payload = if inline_data_size(std::mem::size_of::<T::Data>()) {
            Payload::inline(data)
        } else {
            Payload::spilled(self.allocator.alloc(data))
        };
        let raw = self.spans.len() as u32;
        self.spans.push(span);
        self.kinds.push(T::KIND);
        self.payloads.push(payload);
        // SAFETY: the payload and kind column were just written for `T`.
        unsafe { T::from_node_id(NodeId::from_raw(raw)) }
    }

    /// Returns the logical data of one node.
    #[inline]
    pub fn data<T: NodeStorage<'ast>>(&self, id: T) -> T::Data {
        let raw = id.node_id();
        self.check_kind(raw, T::KIND);
        let payload = self.payloads[raw.raw() as usize];
        if inline_data_size(std::mem::size_of::<T::Data>()) {
            // SAFETY: `check_kind` established that this slot was written for
            // `T`, and the payload was stored inline because it fits.
            unsafe { payload.read_inline::<T::Data>() }
        } else {
            // SAFETY: the kind check established that this payload stores a
            // spilled `T::Data` allocation owned by the context allocator.
            unsafe { *payload.read_spilled::<T::Data>() }
        }
    }

    /// Replaces the logical data of an existing node without changing its ID.
    #[inline]
    pub fn set_data<T: NodeStorage<'ast>>(&mut self, id: T, data: T::Data) {
        let raw = id.node_id();
        self.check_kind(raw, T::KIND);
        if inline_data_size(std::mem::size_of::<T::Data>()) {
            self.payloads[raw.raw() as usize] = Payload::inline(data);
        } else {
            // Mutate the existing allocation in place so pointers embedded in
            // other nodes remain valid.
            let payload = self.payloads[raw.raw() as usize];
            // SAFETY: the kind check established that this payload stores a
            // spilled `T::Data` allocation owned by the context allocator.
            let target = unsafe { payload.read_spilled::<T::Data>() } as *const T::Data;
            // SAFETY: the allocation is exclusively owned by this context and
            // is not borrowed across this call.
            unsafe { (target as *mut T::Data).write(data) };
        }
    }

    #[inline]
    pub fn span<T: NodeStorage<'ast>>(&self, id: T) -> Span {
        let raw = id.node_id();
        self.check_kind(raw, T::KIND);
        self.spans[raw.raw() as usize]
    }

    #[inline]
    pub fn set_span<T: NodeStorage<'ast>>(&mut self, id: T, span: Span) {
        let raw = id.node_id();
        self.check_kind(raw, T::KIND);
        self.spans[raw.raw() as usize] = span;
    }

    /// Copies a list into the arena and returns it as a borrowed slice.
    #[inline]
    pub fn alloc_slice<T: Copy>(&self, values: &[T]) -> &'ast [T] {
        self.allocator.alloc_slice_copy(values)
    }

    /// Copies bytes into the arena.
    #[inline]
    pub fn alloc_str(&self, value: &str) -> &'ast str {
        self.allocator.alloc_str(value)
    }

    #[inline]
    pub fn allocator(&self) -> &'ast Allocator {
        self.allocator
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.spans.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    #[inline]
    pub fn kind_raw<T: NodeStorage<'ast>>(&self, id: T) -> NodeKind {
        self.kinds[id.node_id().raw() as usize]
    }

    #[inline]
    fn check_kind<T>(&self, raw: NodeId<'ast, T>, expected: NodeKind) {
        debug_assert_eq!(
            self.kinds[raw.raw() as usize],
            expected,
            "AST node used with the wrong node kind"
        );
    }
}

impl std::fmt::Debug for Ast<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Ast")
            .field("nodes", &self.spans.len())
            .finish()
    }
}
