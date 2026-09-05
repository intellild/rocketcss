use std::{any::type_name, ptr::NonNull};

use rocketcss_common::{Allocator, DenseId, DenseStore};

use crate::{DUMMY_SP, Span, Visit, VisitContext, VisitMut, VisitMutContext, Visitor, VisitorMut};

use super::{CssRulePayload, DeclarationPayload, EffectiveKeyData, RadixCompilation};

/// AST node identity. The node type itself is the dense identity domain.
pub type NodeId<'ast, T> = DenseId<'ast, T>;

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
struct NodeStore<'ast> {
    allocator: &'ast Allocator,
    nodes: DenseStore<'ast, NodeStorageDomain, NodeSlot>,
    spans: DenseStore<'ast, NodeStorageDomain, Span>,
    active_mutations: usize,
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
    fn new_in(allocator: &'ast Allocator) -> Self {
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

/// Tail position of the node table captured by a speculative parser state.
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct NodeCheckpoint {
    len: usize,
}

/// Storage extension used by the concrete AST context.
#[doc(hidden)]
pub struct AstNodeStores<'ast> {
    nodes: NodeStore<'ast>,
}

/// Initializes the optional node-storage extension of a generic compilation.
#[doc(hidden)]
pub trait CompilationNodeStores<'ast>: Sized {
    fn new_in(allocator: &'ast Allocator) -> Self;
}

impl<'ast> CompilationNodeStores<'ast> for () {
    fn new_in(_allocator: &'ast Allocator) -> Self {}
}

impl<'ast> CompilationNodeStores<'ast> for AstNodeStores<'ast> {
    fn new_in(allocator: &'ast Allocator) -> Self {
        Self {
            nodes: NodeStore::new_in(allocator),
        }
    }
}

type Context<'ast> = RadixCompilation<
    'ast,
    CssRulePayload<'ast>,
    DeclarationPayload<'ast>,
    EffectiveKeyData<'ast, CssRulePayload<'ast>>,
    AstNodeStores<'ast>,
>;

impl<'ast> Context<'ast> {
    #[doc(hidden)]
    #[inline]
    pub fn node_checkpoint(&self) -> NodeCheckpoint {
        NodeCheckpoint {
            len: self.node_stores.nodes.len(),
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn restore_node_checkpoint(&mut self, checkpoint: NodeCheckpoint) {
        self.node_stores.nodes.truncate(checkpoint.len);
    }

    #[inline]
    pub fn alloc_node<T: 'ast>(&mut self, value: T, span: Span) -> NodeId<'ast, T> {
        self.node_stores.nodes.alloc(value, span)
    }

    #[inline]
    pub fn node<T>(&self, id: NodeId<'ast, T>) -> &T {
        self.node_stores.nodes.get(id)
    }

    /// Resolves a typed ID carried through an API whose AST lifetime is erased.
    #[doc(hidden)]
    #[inline]
    pub fn resolve_node<'id, T>(&self, id: NodeId<'id, T>) -> &T {
        self.node_stores.nodes.get_at(id.index())
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
        let mut mutation = self.node_stores.nodes.mutation(id);
        f(mutation.value(), self)
    }

    #[inline]
    pub fn node_span<T>(&self, id: NodeId<'ast, T>) -> Span {
        self.node_stores.nodes.span(id)
    }

    #[inline]
    pub fn set_node_span<T>(&mut self, id: NodeId<'ast, T>, span: Span) {
        self.node_stores.nodes.set_span(id, span);
    }

    pub(crate) fn node_mutation<T>(&mut self, id: NodeId<'ast, T>) -> NodeMutation<'ast, T> {
        self.node_stores.nodes.mutation(id)
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
