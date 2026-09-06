use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use rocketcss_common::{DenseId, DenseRange, vec::Vec as ArenaVec};

use crate::{DUMMY_SP, Span, Visit, VisitContext, VisitMut, VisitMutContext, Visitor, VisitorMut};

use super::{
    AstContext, AstNodeClone, AstNodeStorage, DeclarationId, ExtraData, ExtraDataClone,
    ExtraDataCompact, NodePayload,
};

/// AST node identity. The node type itself is the dense identity domain.
pub type NodeId<'ast, T> = DenseId<'ast, T>;

/// A persistent AST list. The handle stores only its dense `start..end` range;
/// elements are resolved by the owning [`AstContext`](crate::AstContext).
pub type AstVec<'ast, T> = DenseRange<'ast, T>;

/// Value-decoding iterator over one compact `ExtraDataStore` range.
pub(crate) struct EncodedVecIter<'context, 'ast, T> {
    context: &'context AstContext<'ast>,
    range: AstVec<'ast, T>,
    index: usize,
}

impl<'context, 'ast, T: ExtraDataCompact<'ast>> Iterator for EncodedVecIter<'context, 'ast, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.context.encoded_vec_get(self.range, self.index)?;
        self.index += 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.range.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<'context, 'ast, T: ExtraDataCompact<'ast>> ExactSizeIterator
    for EncodedVecIter<'context, 'ast, T>
{
}

/// Tail position of the node table captured by a speculative parser state.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeCheckpoint {
    compact_node_len: usize,
    extra_len: usize,
}

#[allow(dead_code)]
impl<'ast> AstContext<'ast> {
    #[inline]
    pub(crate) fn encoded_declaration_id_at(&self, index: usize) -> DeclarationId<'ast> {
        self.declarations
            .id_at_offset(0, index)
            .expect("declaration ID does not belong to this context")
    }

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
        unsafe { T::decode(self.nodes.payload(id, T::KIND), self) }
    }

    /// Reads a kind-checked payload for an AST module's typed field accessor.
    #[inline]
    pub(crate) fn node_payload<'id, T: AstNodeStorage<'id>>(
        &self,
        id: NodeId<'id, T>,
    ) -> NodePayload {
        self.nodes.payload(id, T::KIND)
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
        let mut value = unsafe { T::decode(current, self) };
        let result = catch_unwind(AssertUnwindSafe(|| f(&mut value, self)));
        let payload = unsafe { value.encode_existing(current, self) };
        self.nodes.finish_mutation(id, T::KIND, payload);
        match result {
            Ok(result) => result,
            Err(payload) => resume_unwind(payload),
        }
    }

    /// Starts a visitor-owned node transaction without retaining a context
    /// borrow while the visitor recursively accesses other stored values.
    pub(crate) fn begin_visit_node_mutation<T: AstNodeStorage<'ast>>(
        &mut self,
        id: NodeId<'ast, T>,
    ) -> (NodePayload, T) {
        let current = self.nodes.begin_mutation(id, T::KIND);
        let value = unsafe { T::decode(current, self) };
        (current, value)
    }

    /// Commits a visitor-owned node transaction under its original identity.
    pub(crate) fn finish_visit_node_mutation<T: AstNodeStorage<'ast>>(
        &mut self,
        id: NodeId<'ast, T>,
        current: NodePayload,
        value: T,
    ) {
        let payload = unsafe { value.encode_existing(current, self) };
        self.nodes.finish_mutation(id, T::KIND, payload);
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
        self.extra.alloc(values.map(T::encode_extra))
    }

    #[inline]
    pub(crate) fn encoded_vec_get<T: ExtraDataCompact<'ast>>(
        &self,
        range: AstVec<'ast, T>,
        index: usize,
    ) -> Option<T> {
        self.extra
            .get(range, index)
            .map(|data| unsafe { T::decode_extra(data) })
    }

    #[inline]
    pub(crate) fn encoded_vec_iter<T: ExtraDataCompact<'ast>>(
        &self,
        range: AstVec<'ast, T>,
    ) -> EncodedVecIter<'_, 'ast, T> {
        EncodedVecIter {
            context: self,
            range,
            index: 0,
        }
    }

    #[inline]
    pub(crate) fn encoded_vec_set<T: ExtraDataCompact<'ast>>(
        &mut self,
        range: AstVec<'ast, T>,
        index: usize,
        value: T,
    ) {
        let data = value.encode_extra();
        self.extra.set(range, index, data);
    }

    /// Deep-clones a compact list through each element's context-aware codec.
    pub(crate) fn clone_encoded_vec<T>(&mut self, range: AstVec<'ast, T>) -> AstVec<'ast, T>
    where
        T: ExtraDataCompact<'ast> + ExtraDataClone<'ast>,
    {
        let values = self.encoded_vec_iter(range).collect::<std::vec::Vec<_>>();
        let cloned = values
            .into_iter()
            .map(|value| value.clone_extra(self))
            .collect::<std::vec::Vec<_>>();
        self.alloc_encoded_vec(cloned.into_iter())
    }

    /// Mutates a compact list without changing its range.
    pub(crate) fn mutate_encoded_vec<T, R>(
        &mut self,
        range: AstVec<'ast, T>,
        f: impl FnOnce(&mut [T], &mut Self) -> R,
    ) -> R
    where
        T: ExtraDataCompact<'ast>,
    {
        let mut values = self.encoded_vec_iter(range).collect::<std::vec::Vec<_>>();
        let result = catch_unwind(AssertUnwindSafe(|| f(&mut values, self)));
        for (index, value) in values.into_iter().enumerate() {
            self.encoded_vec_set(range, index, value);
        }
        match result {
            Ok(result) => result,
            Err(payload) => resume_unwind(payload),
        }
    }

    /// Applies a length-changing edit and publishes a fresh compact range.
    pub(crate) fn rewrite_encoded_vec<T, R>(
        &mut self,
        range: &mut AstVec<'ast, T>,
        f: impl FnOnce(&mut ArenaVec<'ast, T>, &mut Self) -> R,
    ) -> R
    where
        T: ExtraDataCompact<'ast> + Unpin + 'ast,
    {
        let mut values = ArenaVec::from_iter_in(self.encoded_vec_iter(*range), self.allocator);
        let result = catch_unwind(AssertUnwindSafe(|| f(&mut values, self)));
        *range = self.extra.alloc(values.into_iter().map(T::encode_extra));
        match result {
            Ok(result) => result,
            Err(payload) => resume_unwind(payload),
        }
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

    #[cfg(test)]
    pub(crate) fn encoded_node_len(&self) -> usize {
        self.nodes.len()
    }

    #[doc(hidden)]
    #[inline]
    pub fn node_checkpoint(&self) -> NodeCheckpoint {
        NodeCheckpoint {
            compact_node_len: self.nodes.len(),
            extra_len: self.extra.len(),
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn restore_node_checkpoint(&mut self, checkpoint: NodeCheckpoint) {
        self.nodes.truncate(checkpoint.compact_node_len);
        self.extra.truncate(checkpoint.extra_len);
    }

    #[inline]
    pub fn alloc_node<T: AstNodeStorage<'ast>>(&mut self, value: T, span: Span) -> NodeId<'ast, T> {
        self.alloc_encoded_node(value, span)
    }

    /// Commits a construction-time arena vector to persistent AST range storage.
    #[inline]
    pub fn alloc_vec<T: ExtraDataCompact<'ast> + 'ast + Unpin>(
        &mut self,
        values: ArenaVec<'ast, T>,
    ) -> AstVec<'ast, T> {
        self.alloc_encoded_vec(values.into_iter())
    }

    /// Returns the number of values in a persistent AST range.
    #[inline]
    pub const fn vec_len<T>(&self, range: AstVec<'_, T>) -> usize {
        range.len()
    }

    /// Decodes one value from a persistent AST range.
    #[inline]
    pub fn vec_get<'id, T: ExtraDataCompact<'id>>(
        &self,
        range: AstVec<'id, T>,
        index: usize,
    ) -> Option<T> {
        self.extra.get(range, index).map(|data| {
            // SAFETY: this typed range identifies slots written for T. Decoding
            // copies the stored value; it does not borrow from the context.
            unsafe { T::decode_extra(data) }
        })
    }

    /// Iterates decoded values from a persistent AST range.
    #[inline]
    pub fn vec_iter<'context, 'id, T>(
        &'context self,
        range: AstVec<'id, T>,
    ) -> impl ExactSizeIterator<Item = T> + 'context
    where
        'id: 'context,
        T: ExtraDataCompact<'id> + 'context,
    {
        (0..range.len()).map(move |index| {
            self.vec_get(range, index)
                .expect("AST range index validated by its length")
        })
    }

    /// Materializes a decoded range for diagnostics and assertions.
    /// Performance-sensitive code should consume [`Self::vec_iter`] directly.
    #[doc(hidden)]
    pub fn vec_snapshot<'id, T>(&self, range: AstVec<'id, T>) -> std::vec::Vec<T>
    where
        T: ExtraDataCompact<'id>,
    {
        self.vec_iter(range).collect()
    }

    /// Replaces one value without changing a persistent AST range's bounds.
    #[inline]
    pub fn vec_set<T: ExtraDataCompact<'ast>>(
        &mut self,
        range: AstVec<'ast, T>,
        index: usize,
        value: T,
    ) {
        self.encoded_vec_set(range, index, value);
    }

    /// Clones a persistent list into a fresh dense range.
    pub fn clone_vec<T>(&mut self, range: AstVec<'ast, T>) -> AstVec<'ast, T>
    where
        T: ExtraDataCompact<'ast> + ExtraDataClone<'ast>,
    {
        self.clone_encoded_vec(range)
    }

    /// Mutates the elements of a persistent list without changing its length.
    pub fn mutate_vec<T: ExtraDataCompact<'ast>, R>(
        &mut self,
        range: AstVec<'ast, T>,
        f: impl FnOnce(&mut [T], &mut Self) -> R,
    ) -> R {
        self.mutate_encoded_vec(range, f)
    }

    /// Runs a length-changing list mutation and publishes a replacement range.
    /// If the callback unwinds, its partially updated values are still committed
    /// before the panic resumes, matching ordinary Vec mutation semantics.
    pub fn rewrite_vec<T: ExtraDataCompact<'ast> + Unpin + 'ast, R>(
        &mut self,
        range: &mut AstVec<'ast, T>,
        f: impl FnOnce(&mut ArenaVec<'ast, T>, &mut Self) -> R,
    ) -> R {
        self.rewrite_encoded_vec(range, f)
    }

    #[inline]
    pub fn node<T: AstNodeStorage<'ast>>(&self, id: NodeId<'ast, T>) -> T {
        self.encoded_node(id)
    }

    /// Resolves a typed ID carried through an API whose AST lifetime is erased.
    #[doc(hidden)]
    #[inline]
    pub fn resolve_node<'id, T: AstNodeStorage<'id>>(&self, id: NodeId<'id, T>) -> T {
        let payload = self.nodes.payload(id, T::KIND);
        // SAFETY: this is the lifetime-erased resolution boundary used by APIs
        // such as code generation. A NodeId is only valid with its owning
        // AstContext; all references decoded here originate in that context's
        // allocator and are merely restored to the ID's erased lifetime.
        let context = unsafe { &*(self as *const Self).cast::<AstContext<'id>>() };
        unsafe { T::decode(payload, context) }
    }

    /// Compares stored node payloads rather than their dense identities.
    #[doc(hidden)]
    #[inline]
    pub fn nodes_eq<'id, T: PartialEq + AstNodeStorage<'id>>(
        &self,
        first: NodeId<'id, T>,
        second: NodeId<'id, T>,
    ) -> bool {
        self.resolve_node(first)
            .eq_in_context(&self.resolve_node(second), self)
    }

    /// Clones a stored payload into a fresh dense ID and carries its detached span.
    pub fn clone_node<T: AstNodeClone<'ast>>(&mut self, id: NodeId<'ast, T>) -> NodeId<'ast, T> {
        self.clone_encoded_node(id)
    }

    /// Mutates a stored payload transactionally while keeping all access on the context.
    pub fn mutate_node<T: AstNodeStorage<'ast>, R>(
        &mut self,
        id: NodeId<'ast, T>,
        f: impl FnOnce(&mut T, &mut Self) -> R,
    ) -> R {
        self.mutate_encoded_node(id, f)
    }

    #[inline]
    pub fn node_span<T: AstNodeStorage<'ast>>(&self, id: NodeId<'ast, T>) -> Span {
        self.encoded_node_span(id)
    }

    #[inline]
    pub fn set_node_span<T: AstNodeStorage<'ast>>(&mut self, id: NodeId<'ast, T>, span: Span) {
        self.set_encoded_node_span(id, span);
    }

    #[inline]
    pub fn alloc_node_without_span<T: AstNodeStorage<'ast>>(
        &mut self,
        value: T,
    ) -> NodeId<'ast, T> {
        self.alloc_node(value, DUMMY_SP)
    }
}

impl<'ast, 'ghost, T> Visit<'ast, 'ghost> for NodeId<'ast, T>
where
    T: AstNodeStorage<'ast> + Visit<'ast, 'ghost>,
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
    T: AstNodeStorage<'ast> + VisitMut<'ast, 'ghost>,
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
    T: ExtraDataCompact<'ast> + Visit<'ast, 'ghost>,
{
    fn visit<VisitorT: ?Sized + Visitor<'ast, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ast, 'ghost>,
    ) {
        for value in cx.ast_context().vec_iter(*self) {
            value.visit(visitor, cx);
        }
    }
}

impl<'ast, 'ghost, T> VisitMut<'ast, 'ghost> for AstVec<'ast, T>
where
    T: ExtraDataCompact<'ast> + VisitMut<'ast, 'ghost>,
{
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'ast, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        let len = cx.ast_context().vec_len(*self);
        for index in 0..len {
            cx.mutate_vec_element(*self, index, |value, cx| {
                value.visit_mut(visitor, cx);
            });
        }
    }
}
