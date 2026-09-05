use std::{
    any::type_name,
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    ptr::NonNull,
};

use rocketcss_common::{Allocator, DenseId, DenseRange, DenseStore, vec::Vec as ArenaVec};

use crate::{DUMMY_SP, Span, Visit, VisitContext, VisitMut, VisitMutContext, Visitor, VisitorMut};

use super::{AstContext, AstNodeClone, AstNodeStorage, ExtraData, ExtraDataCompact};

/// AST node identity. The node type itself is the dense identity domain.
pub type NodeId<'ast, T> = DenseId<'ast, T>;

/// A persistent AST list. The handle stores only its dense `start..end` range;
/// elements are resolved by the owning [`AstContext`](crate::AstContext).
pub type AstVec<'ast, T> = DenseRange<'ast, T>;

enum NodeStorageDomain {}

struct NodeSlot {
    pointer: Option<NonNull<()>>,
    type_name: fn() -> &'static str,
}

#[inline]
fn node_type_name<T>() -> &'static str {
    type_name::<T>()
}

/// Hand-written, heterogeneous node table. Payloads live in the compilation arena while this
/// table keeps their dense identity, borrow state, and aligned span sidecar.
pub(super) struct NodeStore<'ast> {
    allocator: &'ast Allocator,
    nodes: DenseStore<'ast, NodeStorageDomain, NodeSlot>,
    spans: DenseStore<'ast, NodeStorageDomain, Span>,
    active_mutations: usize,
}

enum VecElementStorageDomain {}
enum VecRangeStorageDomain {}

struct VecRangeSlot {
    pointer: Option<NonNull<()>>,
    type_name: fn() -> &'static str,
    start: u32,
    end: u32,
}

/// Persistent list elements remain in one contiguous arena allocation. A compact
/// dense element table maps every range position to one allocation slot, so the
/// pointer, runtime type, and mutation borrow state are stored only once per list.
pub(super) struct VecStore<'ast> {
    allocator: &'ast Allocator,
    elements: DenseStore<'ast, VecElementStorageDomain, DenseId<'ast, VecRangeStorageDomain>>,
    ranges: DenseStore<'ast, VecRangeStorageDomain, VecRangeSlot>,
    active_mutations: usize,
}

pub(crate) struct VecMutation<'ast, T> {
    store: *mut VecStore<'ast>,
    range: AstVec<'ast, T>,
    pointer: NonNull<T>,
}

impl<T> VecMutation<'_, T> {
    #[inline]
    pub(crate) fn values(&mut self) -> &mut [T] {
        // SAFETY: VecStore::mutation made every slot in this range unavailable,
        // and the payload was allocated as one contiguous slice of T.
        unsafe { std::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.range.len()) }
    }
}

impl<T> Drop for VecMutation<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: the owning AstContext outlives this private transaction. The
        // exact range and original contiguous pointer are restored once.
        unsafe { (&mut *self.store).end_mutation(self.range, self.pointer) };
    }
}

pub(crate) struct NodeMutation<'ast, T> {
    store: *mut NodeStore<'ast>,
    id: NodeId<'ast, T>,
    pointer: NonNull<T>,
}

impl<T> NodeMutation<'_, T> {
    #[inline]
    pub(crate) fn value(&mut self) -> &mut T {
        // SAFETY: NodeStore::mutation removed this pointer from its slot, and this transaction is
        // the only owner until Drop restores it.
        unsafe { self.pointer.as_mut() }
    }
}

impl<T> Drop for NodeMutation<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: callers keep the owning AstContext alive for the lexical lifetime of this
        // private transaction. The slot remains unavailable until this guard is dropped.
        unsafe { (&mut *self.store).end_mutation(self.id, self.pointer) };
    }
}

impl<'ast> NodeStore<'ast> {
    pub(super) fn new_in(allocator: &'ast Allocator) -> Self {
        Self {
            allocator,
            nodes: DenseStore::new_in(allocator),
            spans: DenseStore::new_in(allocator),
            active_mutations: 0,
        }
    }

    fn alloc<T: 'ast>(&mut self, value: T, span: Span) -> NodeId<'ast, T> {
        assert!(
            self.nodes.has_capacity_for(1) && self.spans.has_capacity_for(1),
            "AST node count exceeds dense ID capacity"
        );
        let pointer = NonNull::from(self.allocator.alloc(value)).cast();
        let raw_id = self.nodes.push(NodeSlot {
            pointer: Some(pointer),
            type_name: node_type_name::<T>,
        });
        let span_id = self.spans.push(span);
        debug_assert_eq!(
            raw_id, span_id,
            "AST node payloads and spans must stay aligned"
        );

        // SAFETY: DenseId is repr(transparent) over the same lifetime-bound integer for every
        // domain. The node table is the sole producer of NodeId values and records T at raw_id.
        unsafe { std::mem::transmute(raw_id) }
    }

    fn validate_type<T>(&self, index: usize) {
        let slot = &self.nodes.as_slice()[index];
        let expected = node_type_name::<T> as fn() -> &'static str;
        assert!(
            std::ptr::fn_addr_eq(slot.type_name, expected)
                || (slot.type_name)() == node_type_name::<T>(),
            "AST node ID used with the wrong payload type"
        );
    }

    fn pointer<T>(&self, index: usize) -> NonNull<T> {
        self.validate_type::<T>(index);
        let slot = &self.nodes.as_slice()[index];
        slot.pointer
            .expect("recursive access to a mutably borrowed AST node")
            .cast()
    }

    fn get<T>(&self, id: NodeId<'ast, T>) -> &T {
        self.get_at(id.index())
    }

    fn get_at<T>(&self, index: usize) -> &T {
        let pointer = self.pointer::<T>(index);
        // SAFETY: only alloc can produce a NodeId<T>, and it records a pointer to T at that
        // dense index. The arena outlives this store and mutation temporarily clears the slot.
        unsafe { &*pointer.as_ptr() }
    }

    fn begin_mutation<T>(&mut self, id: NodeId<'ast, T>) -> NonNull<T> {
        self.validate_type::<T>(id.index());
        let slot = &mut self.nodes.as_mut_slice()[id.index()];
        let pointer = slot
            .pointer
            .take()
            .expect("recursive access to a mutably borrowed AST node")
            .cast();
        self.active_mutations += 1;
        pointer
    }

    #[inline]
    fn mutation<T>(&mut self, id: NodeId<'ast, T>) -> NodeMutation<'ast, T> {
        let pointer = self.begin_mutation(id);
        NodeMutation {
            store: self,
            id,
            pointer,
        }
    }

    fn end_mutation<T>(&mut self, id: NodeId<'ast, T>, pointer: NonNull<T>) {
        self.validate_type::<T>(id.index());
        let slot = &mut self.nodes.as_mut_slice()[id.index()];
        assert!(
            slot.pointer.is_none(),
            "an AST node transaction must restore its original borrowed slot"
        );
        slot.pointer = Some(pointer.cast());
        self.active_mutations -= 1;
    }

    fn span<T>(&self, id: NodeId<'ast, T>) -> Span {
        self.validate_type::<T>(id.index());
        self.spans.as_slice()[id.index()]
    }

    fn set_span<T>(&mut self, id: NodeId<'ast, T>, span: Span) {
        self.validate_type::<T>(id.index());
        self.spans.as_mut_slice()[id.index()] = span;
    }

    fn len(&self) -> usize {
        debug_assert_eq!(self.nodes.len(), self.spans.len());
        self.nodes.len()
    }

    fn truncate(&mut self, len: usize) {
        assert_eq!(
            self.active_mutations, 0,
            "cannot roll back AST storage during a node mutation"
        );
        self.nodes.truncate(len);
        self.spans.truncate(len);
    }
}

impl<'ast> VecStore<'ast> {
    pub(super) fn new_in(allocator: &'ast Allocator) -> Self {
        Self {
            allocator,
            elements: DenseStore::new_in(allocator),
            ranges: DenseStore::new_in(allocator),
            active_mutations: 0,
        }
    }

    fn alloc<T: 'ast + Unpin>(&mut self, values: ArenaVec<'ast, T>) -> AstVec<'ast, T> {
        assert!(
            !std::mem::needs_drop::<T>(),
            "persistent AST list elements cannot require Drop"
        );
        let start = self.elements.len();
        let len = values.len();
        assert!(
            self.elements.has_capacity_for(len) && self.ranges.has_capacity_for(1),
            "AST list element count exceeds dense range capacity"
        );
        if len == 0 {
            return AstVec::empty();
        }
        self.elements.reserve(len);
        self.ranges.reserve(1);
        let values = values.into_bump_slice_mut();
        let end = start + values.len();
        let range_id = self.ranges.push(VecRangeSlot {
            pointer: Some(NonNull::from(&mut values[0]).cast::<()>()),
            type_name: node_type_name::<T>,
            start: u32::try_from(start).expect("AST list start exceeds dense range capacity"),
            end: u32::try_from(end).expect("AST list end exceeds dense range capacity"),
        });
        for _ in 0..values.len() {
            self.elements.push(range_id);
        }
        debug_assert_eq!(end, self.elements.len());
        // SAFETY: the just-appended slots point at one contiguous allocation of
        // T values, and VecStore remains owned by the same arena-bound context.
        unsafe { AstVec::from_indices_unchecked(start, end) }
    }

    fn range_id<T>(&self, range: AstVec<'_, T>) -> Option<DenseId<'ast, VecRangeStorageDomain>> {
        if range.is_empty() {
            return None;
        }
        assert!(
            range.start_index() <= range.end_index() && range.end_index() <= self.elements.len(),
            "AST list range does not belong to this context"
        );
        let range_id = self.elements.as_slice()[range.start_index()];
        let slot = &self.ranges.as_slice()[range_id.index()];
        assert!(
            slot.start as usize == range.start_index() && slot.end as usize == range.end_index(),
            "AST list range does not identify a complete allocation"
        );
        let expected = node_type_name::<T> as fn() -> &'static str;
        assert!(
            std::ptr::fn_addr_eq(slot.type_name, expected)
                || (slot.type_name)() == node_type_name::<T>(),
            "AST list range used with the wrong element type"
        );
        Some(range_id)
    }

    fn pointer<T>(&self, range: AstVec<'_, T>) -> Option<NonNull<T>> {
        let range_id = self.range_id(range)?;
        let pointer = self.ranges.as_slice()[range_id.index()]
            .pointer
            .expect("recursive access to a mutably borrowed or retired AST list")
            .cast::<T>();
        Some(pointer)
    }

    fn get<T>(&self, range: AstVec<'_, T>) -> &[T] {
        let Some(pointer) = self.pointer(range) else {
            return &[];
        };
        // SAFETY: alloc records the contiguous slice behind this typed range,
        // and validate rejects active mutations or retired slots.
        unsafe { std::slice::from_raw_parts(pointer.as_ptr(), range.len()) }
    }

    fn begin_mutation<T>(&mut self, range: AstVec<'ast, T>) -> NonNull<T> {
        let Some(range_id) = self.range_id(range) else {
            return NonNull::dangling();
        };
        let pointer = self.ranges.as_mut_slice()[range_id.index()]
            .pointer
            .take()
            .expect("recursive access to a mutably borrowed or retired AST list")
            .cast::<T>();
        self.active_mutations += 1;
        pointer
    }

    fn mutation<T>(&mut self, range: AstVec<'ast, T>) -> VecMutation<'ast, T> {
        let pointer = self.begin_mutation(range);
        VecMutation {
            store: self,
            range,
            pointer,
        }
    }

    fn end_mutation<T>(&mut self, range: AstVec<'ast, T>, pointer: NonNull<T>) {
        if range.is_empty() {
            return;
        }
        let range_id = self
            .range_id(range)
            .expect("a non-empty AST list has an allocation slot");
        let slot = &mut self.ranges.as_mut_slice()[range_id.index()];
        assert!(
            slot.pointer.is_none(),
            "an AST list transaction must restore its original borrowed range"
        );
        slot.pointer = Some(pointer.cast());
        self.active_mutations -= 1;
    }

    fn take<T: 'ast + Unpin>(&mut self, range: AstVec<'ast, T>) -> ArenaVec<'ast, T> {
        let Some(range_id) = self.range_id(range) else {
            return ArenaVec::new_in(self.allocator);
        };
        let pointer = self.ranges.as_mut_slice()[range_id.index()]
            .pointer
            .take()
            .expect("recursive access to a mutably borrowed or retired AST list")
            .cast::<T>();
        let mut values = ArenaVec::with_capacity_in(range.len(), self.allocator);
        for offset in 0..range.len() {
            // SAFETY: the old range is now retired, so moving each no-Drop AST
            // value out exactly once cannot race another context access.
            values.push(unsafe { pointer.as_ptr().add(offset).read() });
        }
        values
    }

    fn element_len(&self) -> usize {
        self.elements.len()
    }

    fn range_len(&self) -> usize {
        self.ranges.len()
    }

    fn truncate(&mut self, element_len: usize, range_len: usize) {
        assert_eq!(
            self.active_mutations, 0,
            "cannot roll back AST range storage during a list mutation"
        );
        self.elements.truncate(element_len);
        self.ranges.truncate(range_len);
    }
}

/// Tail position of the node table captured by a speculative parser state.
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct NodeCheckpoint {
    compact_node_len: usize,
    extra_len: usize,
    string_len: usize,
    atom_len: usize,
    node_len: usize,
    vec_element_len: usize,
    vec_range_len: usize,
}

#[allow(dead_code)]
impl<'ast> AstContext<'ast> {
    /// Allocates one value through its hand-written fixed-width codec.
    pub(crate) fn alloc_encoded_node<T: AstNodeStorage<'ast>>(
        &mut self,
        value: T,
        span: Span,
    ) -> NodeId<'ast, T> {
        let payload = value.encode_new(self);
        self.nodes.alloc(T::KIND, payload, span)
    }

    /// Decodes one value through its hand-written fixed-width codec.
    #[inline]
    pub(crate) fn encoded_node<T: AstNodeStorage<'ast>>(&self, id: NodeId<'ast, T>) -> T {
        T::decode(self.nodes.payload(id, T::KIND), self)
    }

    /// Reconstructs a typed identity stored inside another encoded payload.
    #[inline]
    pub(crate) fn encoded_node_id_at<T>(&self, index: usize) -> NodeId<'ast, T> {
        self.nodes.id_at(index)
    }

    /// Mutates an encoded value transactionally without changing its identity.
    pub(crate) fn mutate_encoded_node<T: AstNodeStorage<'ast>, R>(
        &mut self,
        id: NodeId<'ast, T>,
        f: impl FnOnce(&mut T, &mut Self) -> R,
    ) -> R {
        let current = self.nodes.begin_mutation(id, T::KIND);
        let mut value = T::decode(current, self);
        let result = catch_unwind(AssertUnwindSafe(|| f(&mut value, self)));
        let payload = value.encode_existing(current, self);
        self.nodes.finish_mutation(id, T::KIND, payload);
        match result {
            Ok(result) => result,
            Err(payload) => resume_unwind(payload),
        }
    }

    /// Deep-clones one encoded node through the owning context.
    pub(crate) fn clone_encoded_node<T: AstNodeClone<'ast>>(
        &mut self,
        id: NodeId<'ast, T>,
    ) -> NodeId<'ast, T> {
        let span = self.encoded_node_span(id);
        let value = self.encoded_node(id).clone_in_context(self);
        self.alloc_encoded_node(value, span)
    }

    #[inline]
    pub(crate) fn encoded_node_span<T: AstNodeStorage<'ast>>(&self, id: NodeId<'ast, T>) -> Span {
        self.nodes.span(id, T::KIND)
    }

    #[inline]
    pub(crate) fn set_encoded_node_span<T: AstNodeStorage<'ast>>(
        &mut self,
        id: NodeId<'ast, T>,
        span: Span,
    ) {
        self.nodes.set_span(id, T::KIND, span);
    }

    pub(crate) fn alloc_encoded_vec<T: ExtraDataCompact<'ast>>(
        &mut self,
        values: impl ExactSizeIterator<Item = T>,
    ) -> AstVec<'ast, T> {
        let slots = values
            .map(|value| value.encode_extra(self))
            .collect::<std::vec::Vec<_>>();
        self.extra.alloc(slots.into_iter())
    }

    #[inline]
    pub(crate) fn encoded_vec_get<T: ExtraDataCompact<'ast>>(
        &self,
        range: AstVec<'ast, T>,
        index: usize,
    ) -> Option<T> {
        self.extra
            .get(range, index)
            .map(|data| T::decode_extra(data, self))
    }

    #[inline]
    pub(crate) fn encoded_vec_set<T: ExtraDataCompact<'ast>>(
        &mut self,
        range: AstVec<'ast, T>,
        index: usize,
        value: T,
    ) {
        let data = value.encode_extra(self);
        self.extra.set(range, index, data);
    }

    #[inline]
    pub(crate) fn store_string(&mut self, value: &'ast str) -> u32 {
        self.strings.store_string(value)
    }

    #[inline]
    pub(crate) fn resolve_string(&self, index: u64) -> &'ast str {
        self.strings.resolve_string(index)
    }

    #[inline]
    pub(crate) fn store_atom(&mut self, value: rocketcss_common::Atom<'ast>) -> u32 {
        self.strings.store_atom(value)
    }

    #[inline]
    pub(crate) fn resolve_atom(&self, index: u64) -> rocketcss_common::Atom<'ast> {
        self.strings.resolve_atom(index)
    }

    pub(crate) fn alloc_extra_slots<const N: usize>(&mut self, slots: [ExtraData; N]) -> usize {
        self.extra
            .alloc::<ExtraData>(slots.into_iter())
            .start_index()
    }

    #[inline]
    pub(crate) fn extra_slot(&self, index: usize) -> ExtraData {
        self.extra.get_at(index)
    }

    #[inline]
    pub(crate) fn set_extra_slot(&mut self, index: usize, value: ExtraData) {
        self.extra.set_at(index, value);
    }

    #[inline]
    pub(crate) fn encoded_vec_range<T>(&self, start: usize, end: usize) -> AstVec<'ast, T> {
        // SAFETY: callers are hand-written codecs decoding bounds that were
        // previously obtained from an AstVec owned by this same context.
        unsafe { AstVec::from_indices_unchecked(start, end) }
    }

    #[cfg(test)]
    pub(crate) fn encoded_extra_len(&self) -> usize {
        self.extra.len()
    }

    #[doc(hidden)]
    #[inline]
    pub fn node_checkpoint(&self) -> NodeCheckpoint {
        let (string_len, atom_len) = self.strings.lengths();
        NodeCheckpoint {
            compact_node_len: self.nodes.len(),
            extra_len: self.extra.len(),
            string_len,
            atom_len,
            node_len: self.node_store.len(),
            vec_element_len: self.vec_store.element_len(),
            vec_range_len: self.vec_store.range_len(),
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn restore_node_checkpoint(&mut self, checkpoint: NodeCheckpoint) {
        self.nodes.truncate(checkpoint.compact_node_len);
        self.extra.truncate(checkpoint.extra_len);
        self.strings
            .truncate(checkpoint.string_len, checkpoint.atom_len);
        self.node_store.truncate(checkpoint.node_len);
        self.vec_store
            .truncate(checkpoint.vec_element_len, checkpoint.vec_range_len);
    }

    #[inline]
    pub fn alloc_node<T: 'ast>(&mut self, value: T, span: Span) -> NodeId<'ast, T> {
        self.node_store.alloc(value, span)
    }

    /// Commits a construction-time arena vector to persistent AST range storage.
    #[inline]
    pub fn alloc_vec<T: 'ast + Unpin>(&mut self, values: ArenaVec<'ast, T>) -> AstVec<'ast, T> {
        self.vec_store.alloc(values)
    }

    /// Resolves a persistent AST range to its contiguous elements.
    #[inline]
    pub fn vec<'id, T>(&self, range: AstVec<'id, T>) -> &[T] {
        self.vec_store.get(range)
    }

    /// Clones a persistent list into a fresh dense range.
    pub fn clone_vec<T: Clone + Unpin + 'ast>(
        &mut self,
        range: AstVec<'ast, T>,
    ) -> AstVec<'ast, T> {
        let values = ArenaVec::from_iter_in(self.vec(range).iter().cloned(), self.allocator);
        self.alloc_vec(values)
    }

    /// Mutates the elements of a persistent list without changing its length.
    pub fn mutate_vec<T, R>(
        &mut self,
        range: AstVec<'ast, T>,
        f: impl FnOnce(&mut [T], &mut Self) -> R,
    ) -> R {
        let mut mutation = self.vec_store.mutation(range);
        f(mutation.values(), self)
    }

    /// Runs a length-changing list mutation and publishes a replacement range.
    /// If the callback unwinds, its partially updated values are still committed
    /// before the panic resumes, matching ordinary Vec mutation semantics.
    pub fn rewrite_vec<T: Unpin + 'ast, R>(
        &mut self,
        range: &mut AstVec<'ast, T>,
        f: impl FnOnce(&mut ArenaVec<'ast, T>, &mut Self) -> R,
    ) -> R {
        let mut values = self.vec_store.take(*range);
        let result = catch_unwind(AssertUnwindSafe(|| f(&mut values, self)));
        *range = self.vec_store.alloc(values);
        match result {
            Ok(result) => result,
            Err(payload) => resume_unwind(payload),
        }
    }

    #[inline]
    pub fn node<T>(&self, id: NodeId<'ast, T>) -> &T {
        self.node_store.get(id)
    }

    /// Resolves a typed ID carried through an API whose AST lifetime is erased.
    #[doc(hidden)]
    #[inline]
    pub fn resolve_node<'id, T>(&self, id: NodeId<'id, T>) -> &T {
        self.node_store.get_at(id.index())
    }

    /// Compares stored node payloads rather than their dense identities.
    #[doc(hidden)]
    #[inline]
    pub fn nodes_eq<'id, T: PartialEq>(
        &self,
        first: NodeId<'id, T>,
        second: NodeId<'id, T>,
    ) -> bool {
        self.resolve_node(first) == self.resolve_node(second)
    }

    /// Clones a stored payload into a fresh dense ID and carries its detached span.
    pub fn clone_node<T: Clone + 'ast>(&mut self, id: NodeId<'ast, T>) -> NodeId<'ast, T> {
        let span = self.node_span(id);
        let value = self.node(id).clone();
        self.alloc_node(value, span)
    }

    /// Mutates a stored payload transactionally while keeping all access on the context.
    pub fn mutate_node<T, R>(
        &mut self,
        id: NodeId<'ast, T>,
        f: impl FnOnce(&mut T, &mut Self) -> R,
    ) -> R {
        let mut mutation = self.node_store.mutation(id);
        f(mutation.value(), self)
    }

    #[inline]
    pub fn node_span<T>(&self, id: NodeId<'ast, T>) -> Span {
        self.node_store.span(id)
    }

    #[inline]
    pub fn set_node_span<T>(&mut self, id: NodeId<'ast, T>, span: Span) {
        self.node_store.set_span(id, span);
    }

    pub(crate) fn node_mutation<T>(&mut self, id: NodeId<'ast, T>) -> NodeMutation<'ast, T> {
        self.node_store.mutation(id)
    }

    pub(crate) fn vec_mutation<T>(&mut self, range: AstVec<'ast, T>) -> VecMutation<'ast, T> {
        self.vec_store.mutation(range)
    }

    pub(crate) fn take_vec<T: Unpin + 'ast>(
        &mut self,
        range: AstVec<'ast, T>,
    ) -> ArenaVec<'ast, T> {
        self.vec_store.take(range)
    }

    #[inline]
    pub fn alloc_node_without_span<T: 'ast>(&mut self, value: T) -> NodeId<'ast, T> {
        self.alloc_node(value, DUMMY_SP)
    }
}

impl<'ast, 'ghost, T> Visit<'ast, 'ghost> for NodeId<'ast, T>
where
    T: Visit<'ast, 'ghost>,
{
    fn visit<VisitorT: ?Sized + Visitor<'ast, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ast, 'ghost>,
    ) {
        cx.ast_context().node(*self).visit(visitor, cx);
    }
}

impl<'ast, 'ghost, T> VisitMut<'ast, 'ghost> for NodeId<'ast, T>
where
    T: VisitMut<'ast, 'ghost>,
{
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'ast, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        cx.mutate_node(*self, |node, cx| node.visit_mut(visitor, cx));
    }
}

impl<'ast, 'ghost, T> Visit<'ast, 'ghost> for AstVec<'ast, T>
where
    T: Visit<'ast, 'ghost>,
{
    fn visit<VisitorT: ?Sized + Visitor<'ast, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ast, 'ghost>,
    ) {
        for value in cx.ast_context().vec(*self) {
            value.visit(visitor, cx);
        }
    }
}

impl<'ast, 'ghost, T> VisitMut<'ast, 'ghost> for AstVec<'ast, T>
where
    T: VisitMut<'ast, 'ghost>,
{
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'ast, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        cx.mutate_vec(*self, |values, cx| {
            for value in values {
                value.visit_mut(visitor, cx);
            }
        });
    }
}
