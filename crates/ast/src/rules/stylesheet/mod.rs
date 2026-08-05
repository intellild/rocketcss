//! Typed storage and topology for [`StyleSheet`].

use rocketcss_common::{
    Allocator, DenseStore, RadixId, RadixIdRemap, RadixIds, RadixInsertResult, RadixRange,
    SemanticIterEnumerated, TypedRadixIndexArena, define_dense_id,
};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::{collections::VecDeque, hash::Hash};

mod declaration;
mod effective_key;
mod mutation;
mod rule;
mod traversal;
mod validation;
mod value;

pub use declaration::*;
pub use effective_key::*;
pub use rule::*;
pub use traversal::*;
pub use value::*;

/// Stable identity of one rule in the [`RuleStore`]. The type parameter is the
/// rule payload, so `RuleId<P>` can only index an arena storing `RuleRecord<P>`.
pub type RuleId<P> = RadixId<RuleRecord<P>>;

/// Stable identity of one declaration block in the [`DeclarationBlockStore`].
/// The type parameter is the rule payload, so `DeclarationBlockId<P>` can only
/// index an arena storing `DeclarationBlock<P>`.
pub type DeclarationBlockId<P> = RadixId<DeclarationBlock<P>>;

define_dense_id!(pub struct EffectiveKeyId);
define_dense_id!(pub struct SelectorValueId);
define_dense_id!(pub struct SelectorPathId);
define_dense_id!(pub struct ContextValueId);
define_dense_id!(pub struct ContextPathId);
define_dense_id!(pub struct LayerContextId);

/// Repairs payload-local RuleIds after a rare Radix sibling relabel.
#[doc(hidden)]
pub trait RuleIdReferences<P> {
    fn remap_rule_ids(&mut self, remaps: &[RadixIdRemap<RuleId<P>>]);
}

macro_rules! impl_no_rule_id_references {
    ($($type:ty),+ $(,)?) => {
        $(
            impl<P> RuleIdReferences<P> for $type {
                #[inline]
                fn remap_rule_ids(&mut self, _remaps: &[RadixIdRemap<RuleId<P>>]) {}
            }
        )+
    };
}

impl_no_rule_id_references!((), u8, &str);

/// Rules in lexical allocation order plus rare locally inserted siblings.
type RuleStore<'ast, P> = TypedRadixIndexArena<'ast, RuleRecord<P>, RuleId<P>>;

/// Declaration blocks in lexical allocation order plus synthesized blocks.
type DeclarationBlockStore<'ast, P> =
    TypedRadixIndexArena<'ast, DeclarationBlock<P>, DeclarationBlockId<P>>;

/// Stable identity of one declaration occurrence.
pub type DeclarationId = RadixId<DeclarationRecord<()>>;

/// One declaration block's contiguous semantic declaration range.
pub type DeclarationList = RadixRange<DeclarationRecord<()>>;

/// Interned effective-key records shared by declaration blocks.
type EffectiveKeyStore<P> = DenseStore<EffectiveKeyId, P>;

/// Declarations in global semantic order, with rare synthesized siblings.
type DeclarationStore<'ast, P> = TypedRadixIndexArena<'ast, DeclarationRecord<P>, DeclarationId>;

/// Parser-only proof that one authored block still owns the declaration arena
/// append position.
struct AuthoredDeclarationAppend<P> {
    block: DeclarationBlockId<P>,
    last_declaration: Option<DeclarationId>,
}

/// Concrete rule identity for a CSS [`StyleSheet`].
pub type CssRuleId<'ast> = RuleId<CssRule<'ast>>;

/// Concrete declaration-block identity for a CSS [`StyleSheet`].
pub type CssDeclarationBlockId<'ast> = DeclarationBlockId<CssRule<'ast>>;

/// Concrete effective-key payload for a CSS [`StyleSheet`].
pub type CssEffectiveKey<'ast> = EffectiveKeyData<CssRule<'ast>>;

/// Concrete mutation error for a CSS [`StyleSheet`].
pub type StyleSheetMutationError<'ast> = MutationError<CssRule<'ast>>;

impl<'ast> StyleSheetMutationError<'ast> {
    /// Erases the phantom-only arena lifetime so an error can be boxed without
    /// the stylesheet that produced it. The error never stores a rule payload:
    /// every payload-local identity is a `u32` Radix ID with a
    /// `PhantomData<fn() -> T>` type parameter, so no borrowed data is
    /// discarded and the layout is identical across lifetimes.
    #[inline]
    pub fn erase_arena_lifetime(self) -> StyleSheetMutationError<'static> {
        // SAFETY: `MutationError<P>` contains only plain IDs and integers; `P`
        // appears solely inside `PhantomData<fn() -> P>` type parameters.
        unsafe { std::mem::transmute(self) }
    }
}

/// Concrete parser-local semantic context for a CSS [`StyleSheet`].
pub type CssEffectiveContext<'ast> = EffectiveContext<CssRule<'ast>>;

/// Concrete effective-key history segment for a CSS [`StyleSheet`].
pub type CssHistorySegment<'ast> = HistorySegment<CssRule<'ast>>;

/// Initial capacities for the stylesheet's AST stores.
///
/// These are allocation hints only. Every store still grows normally when an
/// input contains more nodes than estimated.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StyleSheetCapacity {
    pub rules: usize,
    pub declaration_blocks: usize,
    pub declarations: usize,
    pub selectors: usize,
    pub contexts: usize,
}

/// One authored or synthesized declaration occurrence and its importance.
#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationRecord<P> {
    payload: P,
    important: bool,
}

impl<P> DeclarationRecord<P> {
    #[inline]
    pub const fn payload(&self) -> &P {
        &self.payload
    }

    #[inline]
    pub fn payload_mut(&mut self) -> &mut P {
        &mut self.payload
    }

    #[inline]
    pub const fn is_important(&self) -> bool {
        self.important
    }
}

/// One authored or synthesized CSS rule in lexical preorder.
#[derive(Debug, PartialEq, Eq)]
pub struct RuleRecord<P> {
    payload: P,
    parent: Option<RuleId<P>>,
    /// Number of physical rule records in this rule's complete lexical
    /// subtree, excluding the rule itself and including retained tombstones.
    descendant_count: u32,
    /// Number of live direct child rules. This is maintained independently
    /// from the physical subtree span so read-only consumers can answer the
    /// common child-existence query without walking retained tombstones.
    nested_rule_count: u32,
    /// Final physical descendant in the lexical subtree. `None` means this is
    /// a leaf and the rule ID itself is the subtree endpoint.
    subtree_last: Option<RuleId<P>>,
    declaration_block: Option<DeclarationBlockId<P>>,
    revision: u32,
    live: bool,
}

impl<P> RuleRecord<P> {
    #[inline]
    fn subtree_range(&self, id: RuleId<P>) -> RadixRange<RuleRecord<P>> {
        RadixRange::new(
            id,
            self.subtree_last.unwrap_or(id),
            self.descendant_count
                .checked_add(1)
                .expect("a rule subtree cannot span u32::MAX descendants"),
        )
    }

    #[inline]
    pub const fn payload(&self) -> &P {
        &self.payload
    }

    #[inline]
    pub fn payload_mut(&mut self) -> &mut P {
        &mut self.payload
    }

    #[inline]
    pub const fn parent(&self) -> Option<RuleId<P>> {
        self.parent
    }

    #[inline]
    pub const fn declaration_block(&self) -> Option<DeclarationBlockId<P>> {
        self.declaration_block
    }

    #[inline]
    pub const fn revision(&self) -> u32 {
        self.revision
    }

    #[inline]
    pub const fn is_live(&self) -> bool {
        self.live
    }
}

/// One live rule yielded by the stylesheet-wide lexical tree cursor.
///
/// The event carries just enough topology for streaming consumers to share
/// one preorder pass without constructing mutation positions or rewalking a
/// direct child list.
pub struct RuleTreeEvent<P> {
    rule: RuleId<P>,
    parent: Option<RuleId<P>>,
    child_count: u32,
}

impl<P> Clone for RuleTreeEvent<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for RuleTreeEvent<P> {}

impl<P> std::fmt::Debug for RuleTreeEvent<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuleTreeEvent")
            .field("rule", &self.rule)
            .field("parent", &self.parent)
            .field("child_count", &self.child_count)
            .finish()
    }
}

impl<P> RuleTreeEvent<P> {
    #[inline]
    pub const fn rule(self) -> RuleId<P> {
        self.rule
    }

    #[inline]
    pub const fn parent(self) -> Option<RuleId<P>> {
        self.parent
    }

    #[inline]
    pub const fn has_children(self) -> bool {
        self.child_count != 0
    }

    #[inline]
    pub const fn child_count(self) -> u32 {
        self.child_count
    }
}

/// Stylesheet-wide preorder events for live rules.
pub struct RuleTreeEventIter<'comp, 'ast, R: Unpin> {
    source: SemanticIterEnumerated<'comp, 'ast, RuleRecord<R>, RuleId<R>>,
}

impl<R: Unpin> Iterator for RuleTreeEventIter<'_, '_, R> {
    type Item = RuleTreeEvent<R>;

    fn next(&mut self) -> Option<Self::Item> {
        for (rule, record) in self.source.by_ref() {
            if record.live {
                return Some(RuleTreeEvent {
                    rule,
                    parent: record.parent,
                    child_count: record.nested_rule_count,
                });
            }
        }
        None
    }
}

/// The unique syntax owner of a declaration block.
pub enum DeclarationBlockOwner<P> {
    Rule(RuleId<P>),
}

impl<P> Clone for DeclarationBlockOwner<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for DeclarationBlockOwner<P> {}

impl<P> std::fmt::Debug for DeclarationBlockOwner<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule(rule) => f.debug_tuple("Rule").field(rule).finish(),
        }
    }
}

impl<P> PartialEq for DeclarationBlockOwner<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rule(left), Self::Rule(right)) => left == right,
        }
    }
}

impl<P> Eq for DeclarationBlockOwner<P> {}

/// A declaration-list handle plus its persistent AST identity.
#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationBlock<P> {
    declarations: DeclarationList,
    owner: DeclarationBlockOwner<P>,
    effective_key: EffectiveKeyId,
    revision: u32,
    live: bool,
}

impl<P> DeclarationBlock<P> {
    #[inline]
    pub const fn declarations(&self) -> DeclarationList {
        self.declarations
    }

    #[inline]
    pub const fn owner(&self) -> DeclarationBlockOwner<P> {
        self.owner
    }

    #[inline]
    pub const fn effective_key(&self) -> EffectiveKeyId {
        self.effective_key
    }

    #[inline]
    pub const fn revision(&self) -> u32 {
        self.revision
    }

    #[inline]
    pub const fn is_live(&self) -> bool {
        self.live
    }
}

/// Capacity or topology error raised before a structural mutation is visible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationError<P> {
    PrimaryRuleCapacityExhausted,
    PrimaryDeclarationBlockCapacityExhausted,
    EffectiveKeyCapacityExhausted,
    SelectorContextCapacityExhausted,
    DeclarationCapacityExhausted,
    UnknownRule(RuleId<P>),
    UnknownEffectiveKey(EffectiveKeyId),
    RetiredRule(RuleId<P>),
    DeclarationBlockAlreadyExists(RuleId<P>),
    UnknownDeclarationBlock(DeclarationBlockId<P>),
    UnknownDeclaration(DeclarationId),
    NonContiguousDeclarationRange(DeclarationBlockId<P>),
    LocalRuleCapacityExhausted(RuleId<P>),
    LocalDeclarationBlockCapacityExhausted(DeclarationBlockId<P>),
    InvalidRuleTopology(RuleId<P>),
    RuleHasChildren(RuleId<P>),
}

/// One live direct-rule adjacency captured by an AST-owned rule-list cursor.
///
/// Queued work retains only the owning list, endpoints, and endpoint
/// revisions. The larger physical Radix window needed to perform a mutation
/// stays inside the AST and is materialized only for a selected mutation.
pub struct DirectRuleEdge<P> {
    parent: Option<RuleId<P>>,
    left: RuleId<P>,
    right: RuleId<P>,
    left_revision: u32,
    right_revision: u32,
}

impl<P> Clone for DirectRuleEdge<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for DirectRuleEdge<P> {}

impl<P> std::fmt::Debug for DirectRuleEdge<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectRuleEdge")
            .field("parent", &self.parent)
            .field("left", &self.left)
            .field("right", &self.right)
            .field("left_revision", &self.left_revision)
            .field("right_revision", &self.right_revision)
            .finish()
    }
}

impl<P> PartialEq for DirectRuleEdge<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent
            && self.left == other.left
            && self.right == other.right
            && self.left_revision == other.left_revision
            && self.right_revision == other.right_revision
    }
}

impl<P> Eq for DirectRuleEdge<P> {}

impl<P> std::hash::Hash for DirectRuleEdge<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
        self.left.hash(state);
        self.right.hash(state);
        self.left_revision.hash(state);
        self.right_revision.hash(state);
    }
}

impl<P> DirectRuleEdge<P> {
    #[inline]
    pub const fn left(&self) -> RuleId<P> {
        self.left
    }

    #[inline]
    pub const fn right(&self) -> RuleId<P> {
        self.right
    }

    /// Returns a validated-position token for a mutation at the left endpoint.
    #[doc(hidden)]
    #[inline]
    pub const fn left_context(&self) -> DirectRuleContext<P> {
        DirectRuleContext {
            parent: self.parent,
            rule: self.left,
            revision: self.left_revision,
        }
    }

    /// Returns a validated-position token for a mutation at the right endpoint.
    #[doc(hidden)]
    #[inline]
    pub const fn right_context(&self) -> DirectRuleContext<P> {
        DirectRuleContext {
            parent: self.parent,
            rule: self.right,
            revision: self.right_revision,
        }
    }

    /// Repairs IDs after a local Radix relabel without exposing construction.
    #[doc(hidden)]
    pub fn remapped(self, remaps: &[RadixIdRemap<RuleId<P>>]) -> Self {
        let remap = |id| {
            remaps
                .iter()
                .find_map(|remap| (remap.old == id).then_some(remap.new))
                .unwrap_or(id)
        };
        Self {
            parent: self.parent.map(remap),
            left: remap(self.left),
            right: remap(self.right),
            ..self
        }
    }
}

/// Compact identity of one live direct rule position.
///
/// The endpoint revision makes retained positions self-invalidating without
/// retaining the physical Radix window used by structural mutations.
pub struct DirectRuleContext<P> {
    parent: Option<RuleId<P>>,
    rule: RuleId<P>,
    revision: u32,
}

/// AST-internal physical window used while committing one structural rule
/// mutation. This never escapes into Nano candidate queues.
struct DirectRuleMutationContext<P> {
    parent: Option<RuleId<P>>,
    previous: Option<RuleId<P>>,
    rule: RuleId<P>,
    next: Option<RuleId<P>>,
    revision: u32,
    incoming_bridge: RadixRange<RuleRecord<P>>,
    subtree: RadixRange<RuleRecord<P>>,
    bridge: RadixRange<RuleRecord<P>>,
    insertion_anchor: RuleId<P>,
    storage_before: Option<RuleId<P>>,
}

impl<P> Clone for DirectRuleMutationContext<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for DirectRuleMutationContext<P> {}

impl<P> std::fmt::Debug for DirectRuleMutationContext<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectRuleMutationContext")
            .field("parent", &self.parent)
            .field("previous", &self.previous)
            .field("rule", &self.rule)
            .field("next", &self.next)
            .field("revision", &self.revision)
            .field("subtree", &self.subtree)
            .field("bridge", &self.bridge)
            .field("insertion_anchor", &self.insertion_anchor)
            .field("storage_before", &self.storage_before)
            .finish_non_exhaustive()
    }
}

impl<P> PartialEq for DirectRuleMutationContext<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent
            && self.previous == other.previous
            && self.rule == other.rule
            && self.next == other.next
            && self.revision == other.revision
            && self.incoming_bridge == other.incoming_bridge
            && self.subtree == other.subtree
            && self.bridge == other.bridge
            && self.insertion_anchor == other.insertion_anchor
            && self.storage_before == other.storage_before
    }
}

impl<P> Eq for DirectRuleMutationContext<P> {}

impl<P> DirectRuleMutationContext<P> {
    fn remapped_with(self, remap: impl Copy + Fn(RuleId<P>) -> RuleId<P>) -> Self {
        let remap_range = |range: RadixRange<RuleRecord<P>>| {
            if range.is_empty() {
                range
            } else {
                RadixRange::new(remap(range.start_id()), remap(range.last_id()), range.len())
            }
        };
        Self {
            parent: self.parent.map(remap),
            previous: self.previous.map(remap),
            rule: remap(self.rule),
            next: self.next.map(remap),
            incoming_bridge: remap_range(self.incoming_bridge),
            subtree: remap_range(self.subtree),
            bridge: remap_range(self.bridge),
            insertion_anchor: remap(self.insertion_anchor),
            storage_before: self.storage_before.map(remap),
            ..self
        }
    }
}

impl<P> Clone for DirectRuleContext<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for DirectRuleContext<P> {}

impl<P> std::fmt::Debug for DirectRuleContext<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectRuleContext")
            .field("parent", &self.parent)
            .field("rule", &self.rule)
            .field("revision", &self.revision)
            .finish()
    }
}

impl<P> PartialEq for DirectRuleContext<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.rule == other.rule && self.revision == other.revision
    }
}

impl<P> Eq for DirectRuleContext<P> {}

impl<P> DirectRuleContext<P> {
    #[inline]
    pub const fn rule(&self) -> RuleId<P> {
        self.rule
    }
}

/// One direct-rule position and the edge entering it from the previous live
/// direct rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectRulePosition<P> {
    context: DirectRuleContext<P>,
    incoming_edge: Option<DirectRuleEdge<P>>,
}

impl<P> DirectRulePosition<P> {
    #[inline]
    pub const fn context(&self) -> DirectRuleContext<P> {
        self.context
    }

    #[inline]
    pub const fn incoming_edge(&self) -> Option<DirectRuleEdge<P>> {
        self.incoming_edge
    }
}

/// Opaque source position for one declaration block and the declaration gap
/// at its semantic end.
pub struct DeclarationBlockPosition<P> {
    order: u32,
    previous: Option<DeclarationBlockId<P>>,
    block: DeclarationBlockId<P>,
    next: Option<DeclarationBlockId<P>>,
    revision: u32,
    live: bool,
    declarations: DeclarationList,
    previous_non_empty_tail: Option<DeclarationId>,
    next_non_empty_start: Option<DeclarationId>,
}

impl<P> Clone for DeclarationBlockPosition<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for DeclarationBlockPosition<P> {}

impl<P> PartialEq for DeclarationBlockPosition<P> {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
            && self.previous == other.previous
            && self.block == other.block
            && self.next == other.next
            && self.revision == other.revision
            && self.live == other.live
            && self.declarations == other.declarations
            && self.previous_non_empty_tail == other.previous_non_empty_tail
            && self.next_non_empty_start == other.next_non_empty_start
    }
}

impl<P> Eq for DeclarationBlockPosition<P> {}

impl<P> std::fmt::Debug for DeclarationBlockPosition<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeclarationBlockPosition")
            .field("order", &self.order)
            .field("block", &self.block)
            .field("revision", &self.revision)
            .field("declarations", &self.declarations)
            .finish_non_exhaustive()
    }
}

impl<P> DeclarationBlockPosition<P> {
    #[inline]
    pub const fn block(&self) -> DeclarationBlockId<P> {
        self.block
    }

    #[doc(hidden)]
    #[inline]
    pub const fn order(&self) -> u32 {
        self.order
    }

    #[inline]
    fn declaration_after(self) -> Option<DeclarationId> {
        if self.declarations.is_empty() {
            self.previous_non_empty_tail
        } else {
            Some(self.declarations.last_id())
        }
    }

    #[inline]
    #[doc(hidden)]
    pub fn append_context(self) -> DeclarationAppendContext<P> {
        DeclarationAppendContext {
            position: self,
            after: self.declaration_after(),
            before: self.next_non_empty_start,
        }
    }

    #[doc(hidden)]
    pub fn remapped(self, remaps: &[RadixIdRemap<DeclarationBlockId<P>>]) -> Self {
        let remap = |id| {
            remaps
                .iter()
                .find_map(|remap| (remap.old == id).then_some(remap.new))
                .unwrap_or(id)
        };
        Self {
            previous: self.previous.map(remap),
            block: remap(self.block),
            next: self.next.map(remap),
            ..self
        }
    }
}

/// Opaque preflight context for appending synthesized declarations to one
/// block without rediscovering declaration neighbors.
pub struct DeclarationAppendContext<P> {
    position: DeclarationBlockPosition<P>,
    after: Option<DeclarationId>,
    before: Option<DeclarationId>,
}

impl<P> Clone for DeclarationAppendContext<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for DeclarationAppendContext<P> {}

impl<P> PartialEq for DeclarationAppendContext<P> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.after == other.after && self.before == other.before
    }
}

impl<P> Eq for DeclarationAppendContext<P> {}

impl<P> std::fmt::Debug for DeclarationAppendContext<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeclarationAppendContext")
            .field("block", &self.position.block)
            .field("revision", &self.position.revision)
            .field("after", &self.after)
            .field("before", &self.before)
            .finish()
    }
}

impl<P> DeclarationAppendContext<P> {
    #[inline]
    pub const fn block(&self) -> DeclarationBlockId<P> {
        self.position.block
    }

    #[doc(hidden)]
    pub fn remapped(self, remaps: &[RadixIdRemap<DeclarationBlockId<P>>]) -> Self {
        Self {
            position: self.position.remapped(remaps),
            ..self
        }
    }

    /// Repairs the predecessor position after `inserted` becomes its direct
    /// declaration-block successor.
    #[doc(hidden)]
    pub fn with_inserted_successor(mut self, inserted: Self) -> Self {
        let next_non_empty_start = if inserted.position.declarations.is_empty() {
            inserted.before
        } else {
            Some(inserted.position.declarations.start_id())
        };
        self.position.next = Some(inserted.position.block);
        self.position.next_non_empty_start = next_non_empty_start;
        self.before = next_non_empty_start;
        self
    }

    /// Repairs the successor position after `inserted` becomes its direct
    /// declaration-block predecessor.
    #[doc(hidden)]
    pub fn with_inserted_predecessor(mut self, inserted: Self) -> Self {
        self.position.previous = Some(inserted.position.block);
        self.position.previous_non_empty_tail = inserted.after;
        if self.position.declarations.is_empty() {
            self.after = inserted.after;
        }
        self
    }
}

/// Fixed-capacity topology work published by one local rule mutation.
pub struct RuleMutationDelta<P> {
    new_edges: [Option<DirectRuleEdge<P>>; 4],
}

impl<P> Clone for RuleMutationDelta<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for RuleMutationDelta<P> {}

impl<P> std::fmt::Debug for RuleMutationDelta<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuleMutationDelta")
            .field("new_edges", &self.new_edges)
            .finish()
    }
}

impl<P> PartialEq for RuleMutationDelta<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.new_edges == other.new_edges
    }
}

impl<P> Eq for RuleMutationDelta<P> {}

impl<P> RuleMutationDelta<P> {
    #[inline]
    pub(super) const fn empty() -> Self {
        Self {
            new_edges: [None, None, None, None],
        }
    }

    #[inline]
    pub fn edges(&self) -> impl Iterator<Item = DirectRuleEdge<P>> + '_ {
        self.new_edges.iter().flatten().copied()
    }
}

/// Live topology exposed by retiring one leaf rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetiredRule<P> {
    pub id: RuleId<P>,
    pub declaration_block: Option<DeclarationBlockId<P>>,
    pub delta: RuleMutationDelta<P>,
    /// Updated cursor state for the next direct rule, when one exists.
    ///
    /// Batch mutation clients can continue through consecutive removals
    /// without restarting the owning direct-rule traversal.
    #[doc(hidden)]
    pub successor_context: Option<DirectRuleContext<P>>,
}

/// Result of folding one direct left sibling into the retained right rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MergedAdjacentRuleBlocks<P> {
    pub retired_rule: RuleId<P>,
    pub retired_block: DeclarationBlockId<P>,
    pub retained_rule: RuleId<P>,
    pub retained_block: DeclarationBlockId<P>,
    pub effective_key: EffectiveKeyId,
    /// Refreshed append cursor for the retained declaration representation.
    #[doc(hidden)]
    pub declaration_append: DeclarationAppendContext<P>,
    pub delta: RuleMutationDelta<P>,
}

/// Final rule identity and topology work produced by one local insertion.
#[derive(Debug)]
pub struct InsertedRule<P> {
    pub rule: RadixInsertResult<RuleId<P>>,
    pub delta: RuleMutationDelta<P>,
}

/// Final AST identities allocated by one synthesized rule-and-block
/// transaction.
#[derive(Debug)]
pub struct InsertedRuleWithDeclarationBlock<P> {
    pub rule: RadixInsertResult<RuleId<P>>,
    pub declaration_block: RadixInsertResult<DeclarationBlockId<P>>,
    /// Refreshed append cursor for the block immediately before the insertion.
    #[doc(hidden)]
    pub predecessor_declaration_append: DeclarationAppendContext<P>,
    pub declaration_append: DeclarationAppendContext<P>,
    pub delta: RuleMutationDelta<P>,
}

/// Payload and local append state returned by one declaration replacement.
#[derive(Debug)]
pub struct ReplacedDeclaration<P, D> {
    pub previous: D,
    /// Refreshed append cursor for the affected declaration block.
    #[doc(hidden)]
    pub declaration_append: DeclarationAppendContext<P>,
}

/// A violated store, ownership, or direct-topology invariant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationError<P> {
    RuleHasWrongParent {
        parent: Option<RuleId<P>>,
        rule: RuleId<P>,
    },
    NestedRuleCountMismatch {
        rule: RuleId<P>,
        expected: u32,
        actual: u32,
    },
    DescendantCountMismatch {
        rule: RuleId<P>,
        expected: u32,
        actual: u32,
    },
    MissingOwnedDeclarationBlock {
        rule: RuleId<P>,
        block: DeclarationBlockId<P>,
    },
    DeclarationBlockHasWrongOwner {
        rule: RuleId<P>,
        block: DeclarationBlockId<P>,
        actual: RuleId<P>,
    },
    MissingBlockOwner {
        block: DeclarationBlockId<P>,
        owner: RuleId<P>,
    },
    RetiredBlockOwner {
        block: DeclarationBlockId<P>,
        owner: RuleId<P>,
    },
    OwnerDoesNotReferenceBlock {
        block: DeclarationBlockId<P>,
        owner: RuleId<P>,
        actual: Option<DeclarationBlockId<P>>,
    },
    MissingEffectiveKey {
        block: DeclarationBlockId<P>,
        key: EffectiveKeyId,
    },
    InvalidDeclarationRange {
        block: DeclarationBlockId<P>,
        range: DeclarationList,
    },
    DeclarationRangeStartsOutOfOrder {
        block: DeclarationBlockId<P>,
        expected: DeclarationId,
        actual: DeclarationId,
    },
    UnownedDeclarations {
        expected: u32,
        actual: u32,
    },
}

/// A stylesheet backed by source-ordered Radix arenas.
///
/// The type parameters default to RocketCSS's CSS rule, declaration, and
/// effective-key nodes. Alternate payloads are supported for storage tests.
pub struct StyleSheet<
    'ast,
    R: Unpin = CssRule<'ast>,
    D: Unpin = CssDeclaration<'ast>,
    K = EffectiveKeyData<CssRule<'ast>>,
> {
    allocator: &'ast Allocator,
    license_comments: crate::Vec<'ast, &'ast str>,
    rules: RuleStore<'ast, R>,
    declaration_blocks: DeclarationBlockStore<'ast, R>,
    declarations: DeclarationStore<'ast, D>,
    authored_declaration_append: Option<AuthoredDeclarationAppend<R>>,
    effective_keys: EffectiveKeyStore<K>,
    effective_key_ids: FxHashMap<K, EffectiveKeyId>,
    selector_values: DenseStore<SelectorValueId, SelectorValueRecord<'ast>>,
    selector_value_buckets: FxHashMap<u64, SmallVec<[SelectorValueId; 1]>>,
    selector_paths: DenseStore<SelectorPathId, SelectorPathRecord>,
    root_selector_paths: std::vec::Vec<Option<SelectorPathId>>,
    selector_path_ids: FxHashMap<SelectorPathKey, SelectorPathId>,
    context_values: DenseStore<ContextValueId, ContextValueRecord<R>>,
    context_value_buckets: FxHashMap<u64, SmallVec<[ContextValueState<R>; 1]>>,
    context_paths: DenseStore<ContextPathId, ContextPathRecord>,
    context_path_ids: FxHashMap<ContextPathKey, ContextPathId>,
    layer_contexts: DenseStore<LayerContextId, LayerContextRecord<R>>,
    layer_context_ids: FxHashMap<LayerContextKey<R>, LayerContextId>,
    rule_mutation_contexts: FxHashMap<RuleId<R>, DirectRuleMutationContext<R>>,
}

impl<'ast, R: Unpin, D: Unpin, K> StyleSheet<'ast, R, D, K> {
    /// Creates an empty stylesheet.
    pub fn new_in(allocator: &'ast Allocator) -> Self {
        Self::with_capacity_in(allocator, StyleSheetCapacity::default())
    }

    /// Creates an empty stylesheet with capacity for the expected authored
    /// AST shape.
    pub fn with_capacity_in(allocator: &'ast Allocator, capacity: StyleSheetCapacity) -> Self {
        Self {
            allocator,
            license_comments: allocator.vec(),
            rules: RuleStore::with_capacity_in(capacity.rules, allocator),
            declaration_blocks: DeclarationBlockStore::with_capacity_in(
                capacity.declaration_blocks,
                allocator,
            ),
            declarations: DeclarationStore::with_capacity_in(capacity.declarations, allocator),
            authored_declaration_append: None,
            effective_keys: EffectiveKeyStore::with_capacity(capacity.declaration_blocks),
            effective_key_ids: FxHashMap::with_capacity_and_hasher(
                capacity.declaration_blocks,
                Default::default(),
            ),
            selector_values: DenseStore::with_capacity(capacity.selectors),
            selector_value_buckets: FxHashMap::with_capacity_and_hasher(
                capacity.selectors,
                Default::default(),
            ),
            selector_paths: DenseStore::with_capacity(capacity.selectors),
            root_selector_paths: std::vec::Vec::with_capacity(capacity.selectors),
            selector_path_ids: FxHashMap::with_capacity_and_hasher(
                capacity.selectors,
                Default::default(),
            ),
            context_values: DenseStore::with_capacity(capacity.contexts),
            context_value_buckets: FxHashMap::with_capacity_and_hasher(
                capacity.contexts,
                Default::default(),
            ),
            context_paths: DenseStore::with_capacity(capacity.contexts),
            context_path_ids: FxHashMap::with_capacity_and_hasher(
                capacity.contexts,
                Default::default(),
            ),
            layer_contexts: DenseStore::with_capacity(capacity.contexts),
            layer_context_ids: FxHashMap::with_capacity_and_hasher(
                capacity.contexts,
                Default::default(),
            ),
            rule_mutation_contexts: FxHashMap::with_capacity_and_hasher(
                capacity.rules,
                Default::default(),
            ),
        }
    }

    #[inline]
    pub const fn allocator(&self) -> &'ast Allocator {
        self.allocator
    }

    #[inline]
    pub fn license_comments(&self) -> &[&'ast str] {
        &self.license_comments
    }

    #[inline]
    pub fn push_license_comment(&mut self, comment: &'ast str) {
        self.license_comments.push(comment);
    }

    #[inline]
    pub fn rule(&self, id: RuleId<R>) -> Option<&RuleRecord<R>> {
        self.rules.get(id)
    }

    #[inline]
    pub fn rule_mut(&mut self, id: RuleId<R>) -> Option<&mut RuleRecord<R>> {
        self.rules.get_mut(id)
    }

    #[inline]
    pub fn declaration_block(&self, id: DeclarationBlockId<R>) -> Option<&DeclarationBlock<R>> {
        self.declaration_blocks.get(id)
    }

    #[inline]
    pub fn declaration_block_mut(
        &mut self,
        id: DeclarationBlockId<R>,
    ) -> Option<&mut DeclarationBlock<R>> {
        self.declaration_blocks.get_mut(id)
    }

    #[doc(hidden)]
    #[inline]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    #[doc(hidden)]
    #[inline]
    pub fn declaration_block_count(&self) -> usize {
        self.declaration_blocks.len()
    }

    #[doc(hidden)]
    #[inline]
    pub fn declaration_count(&self) -> usize {
        self.declarations.len()
    }

    #[doc(hidden)]
    #[inline]
    pub fn effective_key_count(&self) -> usize {
        self.effective_keys.len()
    }

    #[doc(hidden)]
    #[inline]
    pub fn selector_value_count(&self) -> usize {
        self.selector_values.len()
    }

    #[doc(hidden)]
    #[inline]
    pub fn context_value_count(&self) -> usize {
        self.context_values.len()
    }

    #[inline]
    pub fn declaration(&self, id: DeclarationId) -> Option<&DeclarationRecord<D>> {
        self.declarations.get(id)
    }

    #[inline]
    pub fn effective_key(&self, id: EffectiveKeyId) -> Option<&K> {
        self.effective_keys.try_get(id)
    }

    /// Iterates live authored and synthesized rules in global lexical order.
    #[inline]
    pub fn rules_in_source_order(&self) -> impl Iterator<Item = (RuleId<R>, &RuleRecord<R>)> {
        self.rules
            .iter_enumerated()
            .filter(|(_, record)| record.live)
    }

    /// Iterates every live rule once in lexical preorder with its direct-tree
    /// relationship. Streaming consumers should prefer this over recursively
    /// opening a new direct-list iterator for every rule.
    #[inline]
    pub fn rule_tree_events(&self) -> RuleTreeEventIter<'_, 'ast, R> {
        RuleTreeEventIter {
            source: self.rules.iter_enumerated(),
        }
    }

    /// Runs a fallible non-structural transform over live rules in lexical
    /// order without materializing their IDs.
    ///
    /// The callback may mutate rule payloads and declarations through the
    /// stylesheet, but must not insert, retire, or relabel rules while this
    /// source-order cursor is active.
    #[doc(hidden)]
    pub fn try_for_each_rule_in_source_order<E>(
        &mut self,
        mut visit: impl FnMut(RuleId<R>, &mut Self) -> Result<(), E>,
    ) -> Result<(), E> {
        let mut cursor = self.rules.detached_ids();
        while let Some(rule) = cursor.next(&self.rules) {
            if self.rules.get(rule).is_some_and(|record| record.live) {
                visit(rule, self)?;
            }
        }
        Ok(())
    }

    /// Iterates authored and synthesized blocks in global semantic order.
    #[inline]
    pub fn declaration_blocks_in_source_order(
        &self,
    ) -> impl Iterator<Item = (DeclarationBlockId<R>, &DeclarationBlock<R>)> {
        self.declaration_blocks.iter_enumerated()
    }

    /// Compares two known declaration-block IDs in semantic source order.
    ///
    /// Radix insertion preserves encoded order, but that representation detail
    /// stays owned by the AST rather than leaking into transformation state.
    #[doc(hidden)]
    pub fn declaration_block_is_before(
        &self,
        left: DeclarationBlockId<R>,
        right: DeclarationBlockId<R>,
    ) -> Result<bool, MutationError<R>> {
        self.declaration_blocks
            .get(left)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(left))?;
        self.declaration_blocks
            .get(right)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(right))?;
        Ok(left < right)
    }

    /// Iterates declaration-block positions and their block/declaration gaps
    /// in one source-order pass.
    #[doc(hidden)]
    pub fn declaration_block_positions(
        &self,
    ) -> impl Iterator<Item = DeclarationBlockPosition<R>> + '_ {
        DeclarationBlockPositionIter::new(self.declaration_blocks.iter_enumerated())
    }

    fn declaration_append_context(
        &self,
        block: DeclarationBlockId<R>,
    ) -> Result<DeclarationAppendContext<R>, MutationError<R>> {
        self.declaration_block_positions()
            .find(|position| position.block == block)
            .filter(|position| position.live)
            .map(DeclarationBlockPosition::append_context)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))
    }

    /// Iterates authored and synthesized declarations in global semantic order.
    #[inline]
    pub fn declarations_in_source_order(
        &self,
    ) -> impl ExactSizeIterator<Item = (DeclarationId, &DeclarationRecord<D>)> {
        self.declarations.iter_enumerated()
    }

    /// Iterates the declarations owned by `block` in semantic order.
    pub fn declarations_in_block(
        &self,
        block: DeclarationBlockId<R>,
    ) -> Result<impl ExactSizeIterator<Item = &DeclarationRecord<D>> + '_, MutationError<R>> {
        let range = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?
            .declarations;
        self.declarations
            .iter_range(range)
            .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))
    }

    /// Iterates the typed declaration IDs and records owned by `block`.
    ///
    /// Consumers that keep sidecars should key them by these source-order IDs
    /// instead of rebuilding a second occurrence identity.
    pub fn declaration_occurrences_in_block(
        &self,
        block: DeclarationBlockId<R>,
    ) -> Result<
        impl ExactSizeIterator<Item = (DeclarationId, &DeclarationRecord<D>)> + '_,
        MutationError<R>,
    > {
        let range = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?
            .declarations;
        self.declarations
            .iter_range_enumerated(range)
            .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))
    }

    /// Iterates only declaration identities for one block representation.
    pub fn declaration_ids_in_block(
        &self,
        block: DeclarationBlockId<R>,
    ) -> Result<impl ExactSizeIterator<Item = DeclarationId> + '_, MutationError<R>> {
        let record = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?;
        self.declarations
            .ids_in_range(record.declarations)
            .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))
    }

    /// Visits declaration records in the semantic order owned by `block`.
    ///
    /// The representation is resolved once and the callback receives each
    /// typed occurrence directly. In particular, a contiguous [`RadixRange`] is
    /// streamed from the semantic declaration arena instead of first copying
    /// its IDs into a temporary vector. The block revision is advanced once
    /// for every visited occurrence, matching the scoped mutation helpers.
    #[doc(hidden)]
    pub fn for_each_declaration_mut(
        &mut self,
        block: DeclarationBlockId<R>,
        mut visit: impl FnMut(DeclarationId, &mut DeclarationRecord<D>),
    ) -> Result<usize, MutationError<R>> {
        let declaration_list = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?
            .declarations;
        if !self
            .declaration_blocks
            .get(block)
            .is_some_and(|record| record.live)
        {
            return Err(MutationError::<R>::UnknownDeclarationBlock(block));
        }

        let visited = declaration_list.len() as usize;
        self.declarations
            .for_each_in_range_mut(declaration_list, |id, record| visit(id, record))
            .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))?;

        if visited != 0 {
            let revision =
                u32::try_from(visited).expect("a block cannot contain u32::MAX declarations");
            let block = self
                .declaration_blocks
                .get_mut(block)
                .expect("the live block remains resolvable");
            block.revision = block.revision.wrapping_add(revision);
        }
        Ok(visited)
    }

    fn direct_rules(
        &self,
        parent: Option<RuleId<R>>,
    ) -> Result<(DirectRuleIter<'_, 'ast, R>, Option<u32>), MutationError<R>> {
        let (source, remaining, parent_revision) = if let Some(parent) = parent {
            let record = self
                .rules
                .get(parent)
                .ok_or(MutationError::<R>::UnknownRule(parent))?;
            if !record.live {
                return Err(MutationError::<R>::RetiredRule(parent));
            }
            let mut source = self
                .rules
                .ids_in_range(record.subtree_range(parent))
                .ok_or(MutationError::<R>::InvalidRuleTopology(parent))?;
            if source.next() != Some(parent) {
                return Err(MutationError::<R>::InvalidRuleTopology(parent));
            }
            (source, record.descendant_count, Some(record.revision))
        } else {
            (self.rules.ids(), self.rules.len() as u32, None)
        };
        Ok((
            DirectRuleIter {
                rules: &self.rules,
                source,
                remaining,
            },
            parent_revision,
        ))
    }

    fn direct_rule_contexts(
        &self,
        parent: Option<RuleId<R>>,
    ) -> Result<DirectRuleContextIter<'_, 'ast, R>, MutationError<R>> {
        let (rules, _) = self.direct_rules(parent)?;
        Ok(DirectRuleContextIter::new(parent, rules))
    }

    fn direct_rule_mutation_contexts(
        &self,
        parent: Option<RuleId<R>>,
    ) -> Result<DirectRuleMutationContextIter<'_, 'ast, R>, MutationError<R>> {
        let (rules, _) = self.direct_rules(parent)?;
        Ok(DirectRuleMutationContextIter::new(parent, rules))
    }

    /// Builds the AST-owned physical mutation sidecar and all live direct
    /// adjacencies in one whole-tree pass.
    ///
    /// Later candidate checks keep compact edges and expand only a selected
    /// endpoint through the sidecar.
    #[doc(hidden)]
    pub fn prepare_direct_rule_mutation_contexts(
        &mut self,
    ) -> Result<std::vec::Vec<DirectRuleEdge<R>>, MutationError<R>> {
        struct ListState<P> {
            previous: Option<DirectRuleMutationContext<P>>,
            pending_gap: RadixRange<RuleRecord<P>>,
        }

        let mut contexts =
            FxHashMap::with_capacity_and_hasher(self.rule_count(), Default::default());
        let mut lists: FxHashMap<Option<RuleId<R>>, ListState<R>> =
            FxHashMap::with_capacity_and_hasher(self.rule_count(), Default::default());
        let mut following =
            FxHashMap::with_capacity_and_hasher(self.rule_count(), Default::default());
        let mut edges = std::vec::Vec::with_capacity(self.rule_count());
        let mut previous_physical = None;

        for (rule, record) in self.rules.iter_enumerated() {
            if let Some(previous) = previous_physical {
                following.insert(previous, rule);
            }
            previous_physical = Some(rule);

            let state = lists.entry(record.parent).or_insert(ListState {
                previous: None,
                pending_gap: RadixRange::empty(),
            });
            if !record.live {
                state.pending_gap.extend(record.subtree_range(rule));
                continue;
            }

            let incoming_bridge = state.pending_gap;
            state.pending_gap.clear();
            let previous = state.previous.take();
            if let Some(mut previous) = previous {
                previous.next = Some(rule);
                previous.bridge = incoming_bridge;
                previous.insertion_anchor = if incoming_bridge.is_empty() {
                    previous.subtree.last_id()
                } else {
                    incoming_bridge.last_id()
                };
                previous.storage_before = Some(rule);
                edges.push(DirectRuleEdge {
                    parent: record.parent,
                    left: previous.rule,
                    right: rule,
                    left_revision: previous.revision,
                    right_revision: record.revision,
                });
                contexts.insert(previous.rule, previous);
            }
            state.previous = Some(DirectRuleMutationContext {
                parent: record.parent,
                previous: previous.map(|previous| previous.rule),
                rule,
                next: None,
                revision: record.revision,
                incoming_bridge,
                subtree: record.subtree_range(rule),
                bridge: RadixRange::empty(),
                insertion_anchor: record.subtree_range(rule).last_id(),
                storage_before: None,
            });
        }

        for (parent, state) in lists {
            let Some(mut last) = state.previous else {
                continue;
            };
            last.bridge = state.pending_gap;
            last.insertion_anchor = if last.bridge.is_empty() {
                last.subtree.last_id()
            } else {
                last.bridge.last_id()
            };
            last.storage_before = parent.and_then(|parent_id| {
                let parent = self
                    .rules
                    .get(parent_id)
                    .expect("a direct-list owner remains resolvable");
                following
                    .get(&parent.subtree_range(parent_id).last_id())
                    .copied()
            });
            contexts.insert(last.rule, last);
        }
        self.rule_mutation_contexts = contexts;
        Ok(edges)
    }

    fn direct_rule_ids(
        &self,
        parent: Option<RuleId<R>>,
    ) -> Result<DirectRuleIdIter<'_, 'ast, R>, MutationError<R>> {
        let (rules, _) = self.direct_rules(parent)?;
        Ok(DirectRuleIdIter { rules })
    }

    /// Iterates only the IDs of live top-level rules.
    #[inline]
    pub fn root_rule_ids(&self) -> impl Iterator<Item = RuleId<R>> + '_ {
        self.direct_rule_ids(None)
            .unwrap_or_else(|_| unreachable!("the root rule list always has a cursor"))
    }

    /// Iterates only the IDs of live direct nested rules.
    pub fn nested_rule_ids(
        &self,
        parent: RuleId<R>,
    ) -> Result<impl Iterator<Item = RuleId<R>> + '_, MutationError<R>> {
        self.direct_rule_ids(Some(parent))
    }

    /// Iterates live top-level rules without visiting their descendants.
    #[inline]
    pub fn root_rules(&self) -> impl Iterator<Item = (RuleId<R>, &RuleRecord<R>)> {
        let rules = &self.rules;
        self.root_rule_ids().map(move |id| {
            (
                id,
                rules
                    .get(id)
                    .expect("a cursor-produced live rule remains resolvable"),
            )
        })
    }

    /// Iterates live direct nested rules without visiting deeper descendants.
    pub fn nested_rules(
        &self,
        parent: RuleId<R>,
    ) -> Result<impl Iterator<Item = (RuleId<R>, &RuleRecord<R>)>, MutationError<R>> {
        let rules = &self.rules;
        Ok(self.nested_rule_ids(parent)?.map(move |id| {
            (
                id,
                rules
                    .get(id)
                    .expect("a cursor-produced live rule remains resolvable"),
            )
        }))
    }

    /// Iterates top-level rule positions and their incoming edges in one pass.
    #[inline]
    pub fn root_rule_positions(&self) -> impl Iterator<Item = DirectRulePosition<R>> + '_ {
        DirectRuleContextIter::new(
            None,
            self.direct_rules(None)
                .unwrap_or_else(|_| unreachable!("the root rule list always has a cursor"))
                .0,
        )
    }

    /// Iterates one nested direct list and its incoming edges in one pass.
    pub fn nested_rule_positions(
        &self,
        parent: RuleId<R>,
    ) -> Result<impl Iterator<Item = DirectRulePosition<R>> + '_, MutationError<R>> {
        self.direct_rule_contexts(Some(parent))
    }

    /// Iterates live top-level adjacencies in source order.
    #[inline]
    pub fn root_rule_edges(&self) -> impl Iterator<Item = DirectRuleEdge<R>> + '_ {
        self.root_rule_positions()
            .filter_map(|position| position.incoming_edge)
    }

    /// Iterates live adjacencies in one direct nested rule list.
    pub fn nested_rule_edges(
        &self,
        parent: RuleId<R>,
    ) -> Result<impl Iterator<Item = DirectRuleEdge<R>> + '_, MutationError<R>> {
        Ok(self
            .nested_rule_positions(parent)?
            .filter_map(|position| position.incoming_edge))
    }

    /// Iterates opaque mutation positions for top-level rules.
    #[doc(hidden)]
    #[inline]
    pub fn root_rule_contexts(&self) -> impl Iterator<Item = DirectRuleContext<R>> + '_ {
        self.root_rule_positions().map(|position| position.context)
    }

    /// Iterates opaque mutation positions for one direct nested rule list.
    #[doc(hidden)]
    pub fn nested_rule_contexts(
        &self,
        parent: RuleId<R>,
    ) -> Result<impl Iterator<Item = DirectRuleContext<R>> + '_, MutationError<R>> {
        Ok(self
            .nested_rule_positions(parent)?
            .map(|position| position.context))
    }

    /// Checks whether both endpoints and their owning direct list still match
    /// an AST-produced edge context.
    #[doc(hidden)]
    pub fn is_valid_direct_rule_edge(&self, edge: DirectRuleEdge<R>) -> bool {
        self.rules.get(edge.left).is_some_and(|left| {
            left.live
                && left.parent == edge.parent
                && left.revision == edge.left_revision
                && self.rules.get(edge.right).is_some_and(|right| {
                    right.live
                        && right.parent == edge.parent
                        && right.revision == edge.right_revision
                })
        })
    }

    /// Returns whether a live rule has at least one live direct nested rule.
    #[inline]
    pub fn has_nested_rules(&self, parent: RuleId<R>) -> Result<bool, MutationError<R>> {
        let parent_record = self
            .rules
            .get(parent)
            .ok_or(MutationError::<R>::UnknownRule(parent))?;
        if !parent_record.live {
            return Err(MutationError::<R>::RetiredRule(parent));
        }
        Ok(parent_record.nested_rule_count != 0)
    }

    /// Appends one authored rule below `parent` in lexical parse order.
    ///
    /// All fallible checks happen before either the store or topology changes.
    pub fn append_rule(
        &mut self,
        parent: Option<RuleId<R>>,
        payload: R,
    ) -> Result<RuleId<R>, MutationError<R>> {
        if let Some(parent_id) = parent {
            let parent = self
                .rules
                .get(parent_id)
                .ok_or(MutationError::<R>::UnknownRule(parent_id))?;
            if !parent.live {
                return Err(MutationError::<R>::RetiredRule(parent_id));
            }
        }
        if !self.rules.can_push_primary() {
            return Err(MutationError::<R>::PrimaryRuleCapacityExhausted);
        }

        self.authored_declaration_append = None;
        let id = self.rules.push_primary(RuleRecord {
            payload,
            parent,
            descendant_count: 0,
            nested_rule_count: 0,
            subtree_last: None,
            declaration_block: None,
            revision: 0,
            live: true,
        });
        let direct_parent = parent;
        let mut ancestor = parent;
        while let Some(ancestor_id) = ancestor {
            let record = self
                .rules
                .get_mut(ancestor_id)
                .expect("an appended rule's validated ancestor remains resolvable");
            record.descendant_count += 1;
            if Some(ancestor_id) == direct_parent {
                record.nested_rule_count += 1;
            }
            record.subtree_last = Some(id);
            record.revision = record.revision.wrapping_add(1);
            ancestor = record.parent;
        }
        Ok(id)
    }

    #[doc(hidden)]
    #[inline]
    pub fn authored_rule_count(&self) -> usize {
        self.rules.primary_iter().len()
    }

    /// Appends a key record. W6 routes this operation through exact interning.
    pub fn append_effective_key(&mut self, key: K) -> Result<EffectiveKeyId, MutationError<R>>
    where
        K: Copy + Eq + Hash,
    {
        if let Some(&id) = self.effective_key_ids.get(&key) {
            return Ok(id);
        }
        let id = self
            .effective_keys
            .try_push(key)
            .map_err(|_| MutationError::<R>::EffectiveKeyCapacityExhausted)?;
        self.effective_key_ids.insert(key, id);
        Ok(id)
    }

    /// Appends one authored declaration block and binds its unique owner.
    pub fn append_declaration_block(
        &mut self,
        owner: DeclarationBlockOwner<R>,
        effective_key: EffectiveKeyId,
    ) -> Result<DeclarationBlockId<R>, MutationError<R>> {
        let DeclarationBlockOwner::<R>::Rule(owner_rule) = owner;
        let owner_record = self
            .rules
            .get(owner_rule)
            .ok_or(MutationError::<R>::UnknownRule(owner_rule))?;
        if !owner_record.live {
            return Err(MutationError::<R>::RetiredRule(owner_rule));
        }
        if owner_record.declaration_block.is_some() {
            return Err(MutationError::<R>::DeclarationBlockAlreadyExists(
                owner_rule,
            ));
        }
        if self.effective_keys.try_get(effective_key).is_none() {
            return Err(MutationError::<R>::UnknownEffectiveKey(effective_key));
        }
        if !self.declaration_blocks.can_push_primary() {
            return Err(MutationError::<R>::PrimaryDeclarationBlockCapacityExhausted);
        }
        let block = self.declaration_blocks.push_primary(DeclarationBlock::<R> {
            declarations: DeclarationList::empty(),
            owner,
            effective_key,
            revision: 0,
            live: true,
        });
        self.rules
            .get_mut(owner_rule)
            .expect("the block owner was validated before allocation")
            .declaration_block = Some(block);
        self.authored_declaration_append = Some(AuthoredDeclarationAppend {
            block,
            last_declaration: None,
        });
        Ok(block)
    }

    /// Appends one declaration occurrence to the active end of `block`.
    ///
    /// Descendant declarations close an ancestor's range. Continuing that
    /// older range is rejected so the parser must first allocate a distinct
    /// `NestedDeclarations` syntax position.
    pub fn append_authored_declaration(
        &mut self,
        block: DeclarationBlockId<R>,
        payload: D,
        important: bool,
    ) -> Result<DeclarationId, MutationError<R>> {
        let block_record = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?;
        if !block_record.live {
            return Err(MutationError::<R>::UnknownDeclarationBlock(block));
        }
        let range = block_record.declarations;
        let expected_last = (!range.is_empty()).then(|| range.last_id());
        if !self
            .authored_declaration_append
            .as_ref()
            .is_some_and(|append| append.block == block && append.last_declaration == expected_last)
        {
            return Err(MutationError::<R>::NonContiguousDeclarationRange(block));
        }
        if !self.declarations.can_push_primary() {
            return Err(MutationError::<R>::DeclarationCapacityExhausted);
        }
        let declaration = self
            .declarations
            .push_primary(DeclarationRecord { payload, important });
        let block = self
            .declaration_blocks
            .get_mut(block)
            .expect("the block was validated before appending its declaration");
        if block.declarations.is_empty() {
            block.declarations.initialize(declaration);
        } else {
            block.declarations.append(declaration);
        }
        self.authored_declaration_append
            .as_mut()
            .expect("the authored append cursor was validated")
            .last_declaration = Some(declaration);
        Ok(declaration)
    }
}

struct DeclarationBlockCursorEntry<'comp, R> {
    id: DeclarationBlockId<R>,
    record: &'comp DeclarationBlock<R>,
}

impl<R> Clone for DeclarationBlockCursorEntry<'_, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for DeclarationBlockCursorEntry<'_, R> {}

struct DeclarationBlockPositionIter<'comp, 'ast, R: Unpin> {
    source: SemanticIterEnumerated<'comp, 'ast, DeclarationBlock<R>, DeclarationBlockId<R>>,
    buffered: VecDeque<DeclarationBlockCursorEntry<'comp, R>>,
    next_non_empty: Option<(DeclarationBlockId<R>, DeclarationId)>,
    source_exhausted: bool,
    previous: Option<DeclarationBlockId<R>>,
    previous_non_empty_tail: Option<DeclarationId>,
    order: u32,
}

impl<'comp, 'ast, R: Unpin> DeclarationBlockPositionIter<'comp, 'ast, R> {
    fn new(
        source: SemanticIterEnumerated<'comp, 'ast, DeclarationBlock<R>, DeclarationBlockId<R>>,
    ) -> Self {
        Self {
            source,
            buffered: VecDeque::new(),
            next_non_empty: None,
            source_exhausted: false,
            previous: None,
            previous_non_empty_tail: None,
            order: 0,
        }
    }

    fn fill_lookahead(&mut self) {
        if self.buffered.is_empty() && !self.source_exhausted {
            if let Some((id, record)) = self.source.next() {
                self.buffered
                    .push_back(DeclarationBlockCursorEntry { id, record });
            } else {
                self.source_exhausted = true;
            }
        }
        if self
            .buffered
            .front()
            .is_some_and(|front| self.next_non_empty.is_some_and(|(id, _)| id == front.id))
        {
            self.next_non_empty = None;
        }
        while self.next_non_empty.is_none() && !self.source_exhausted {
            let Some((id, record)) = self.source.next() else {
                self.source_exhausted = true;
                break;
            };
            self.buffered
                .push_back(DeclarationBlockCursorEntry { id, record });
            if !record.declarations.is_empty() {
                self.next_non_empty = Some((id, record.declarations.start_id()));
            }
        }
    }
}

impl<R: Unpin> Iterator for DeclarationBlockPositionIter<'_, '_, R> {
    type Item = DeclarationBlockPosition<R>;

    fn next(&mut self) -> Option<Self::Item> {
        self.fill_lookahead();
        let current = self.buffered.pop_front()?;
        let next = self.buffered.front().map(|entry| entry.id);
        let next_non_empty_start = self.next_non_empty.map(|(_, declaration)| declaration);
        let position = DeclarationBlockPosition {
            order: self.order,
            previous: self.previous,
            block: current.id,
            next,
            revision: current.record.revision,
            live: current.record.live,
            declarations: current.record.declarations,
            previous_non_empty_tail: self.previous_non_empty_tail,
            next_non_empty_start,
        };
        self.order = self.order.wrapping_add(1);
        self.previous = Some(current.id);
        if !current.record.declarations.is_empty() {
            self.previous_non_empty_tail = Some(current.record.declarations.last_id());
        }
        Some(position)
    }
}

struct DirectRuleSlot<R> {
    id: RuleId<R>,
    revision: u32,
    subtree: RadixRange<RuleRecord<R>>,
    live: bool,
}

impl<R> Clone for DirectRuleSlot<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for DirectRuleSlot<R> {}

/// Direct-sibling iterator over one source-order ID cursor. Descendant IDs are
/// consumed from the same cursor instead of re-locating the rule after each
/// subtree span.
struct DirectRuleIter<'comp, 'ast, R: Unpin> {
    rules: &'comp RuleStore<'ast, R>,
    source: RadixIds<'comp, 'ast, RuleRecord<R>, RuleId<R>>,
    remaining: u32,
}

impl<R: Unpin> DirectRuleIter<'_, '_, R> {
    fn next_slot(&mut self) -> Option<DirectRuleSlot<R>> {
        if self.remaining == 0 {
            return None;
        }

        let id = self.source.next()?;
        let rule = self.rules.get(id)?;
        let subtree = rule.subtree_range(id);
        if subtree.len() > self.remaining {
            self.remaining = 0;
            return None;
        }
        let mut actual_last = id;
        for _ in 1..subtree.len() {
            actual_last = self.source.next()?;
        }
        if actual_last != subtree.last_id() {
            self.remaining = 0;
            return None;
        }
        self.remaining -= subtree.len();
        Some(DirectRuleSlot {
            id,
            revision: rule.revision,
            subtree,
            live: rule.live,
        })
    }

    #[inline]
    fn following(&mut self) -> Option<RuleId<R>> {
        (self.remaining == 0)
            .then(|| self.source.following())
            .flatten()
    }
}

/// Read-only direct-rule iterator. It retains only the shared physical cursor
/// and never constructs mutation positions or adjacency windows.
struct DirectRuleIdIter<'comp, 'ast, R: Unpin> {
    rules: DirectRuleIter<'comp, 'ast, R>,
}

impl<R: Unpin> Iterator for DirectRuleIdIter<'_, '_, R> {
    type Item = RuleId<R>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let slot = self.rules.next_slot()?;
            if slot.live {
                return Some(slot.id);
            }
        }
    }
}

struct LiveDirectRule<R> {
    id: RuleId<R>,
    revision: u32,
    subtree: RadixRange<RuleRecord<R>>,
    gap_before: RadixRange<RuleRecord<R>>,
}

impl<R> Clone for LiveDirectRule<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for LiveDirectRule<R> {}

/// Lightweight direct-rule positions and adjacencies. It advances the shared
/// physical cursor once and retains no Radix mutation window.
struct DirectRuleContextIter<'comp, 'ast, R: Unpin> {
    parent: Option<RuleId<R>>,
    rules: DirectRuleIter<'comp, 'ast, R>,
    previous: Option<DirectRuleContext<R>>,
    current: Option<DirectRuleContext<R>>,
}

impl<'comp, 'ast, R: Unpin> DirectRuleContextIter<'comp, 'ast, R> {
    fn new(parent: Option<RuleId<R>>, rules: DirectRuleIter<'comp, 'ast, R>) -> Self {
        let mut result = Self {
            parent,
            rules,
            previous: None,
            current: None,
        };
        result.current = result.next_live();
        result
    }

    fn next_live(&mut self) -> Option<DirectRuleContext<R>> {
        loop {
            let slot = self.rules.next_slot()?;
            if slot.live {
                return Some(DirectRuleContext {
                    parent: self.parent,
                    rule: slot.id,
                    revision: slot.revision,
                });
            }
        }
    }
}

impl<R: Unpin> Iterator for DirectRuleContextIter<'_, '_, R> {
    type Item = DirectRulePosition<R>;

    fn next(&mut self) -> Option<Self::Item> {
        let context = self.current?;
        let incoming_edge = self.previous.map(|left| DirectRuleEdge {
            parent: self.parent,
            left: left.rule,
            right: context.rule,
            left_revision: left.revision,
            right_revision: context.revision,
        });
        self.previous = Some(context);
        self.current = self.next_live();
        Some(DirectRulePosition {
            context,
            incoming_edge,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            0,
            Some(self.rules.remaining as usize + usize::from(self.current.is_some())),
        )
    }
}

/// Sliding direct-rule window. It advances every physical rule ID exactly once
/// and retains the local physical gap required by later mutations.
struct DirectRuleMutationContextIter<'comp, 'ast, R: Unpin> {
    parent: Option<RuleId<R>>,
    rules: DirectRuleIter<'comp, 'ast, R>,
    storage_after: Option<RuleId<R>>,
    pending_gap: RadixRange<RuleRecord<R>>,
    previous: Option<LiveDirectRule<R>>,
    current: Option<LiveDirectRule<R>>,
    next: Option<LiveDirectRule<R>>,
}

impl<'comp, 'ast, R: Unpin> DirectRuleMutationContextIter<'comp, 'ast, R> {
    fn new(parent: Option<RuleId<R>>, rules: DirectRuleIter<'comp, 'ast, R>) -> Self {
        let mut result = Self {
            parent,
            rules,
            storage_after: None,
            pending_gap: RadixRange::empty(),
            previous: None,
            current: None,
            next: None,
        };
        result.current = result.next_live();
        result.next = result.next_live();
        result
    }

    fn next_live(&mut self) -> Option<LiveDirectRule<R>> {
        while let Some(slot) = self.rules.next_slot() {
            if slot.live {
                let gap_before = self.pending_gap;
                self.pending_gap.clear();
                return Some(LiveDirectRule {
                    id: slot.id,
                    revision: slot.revision,
                    subtree: slot.subtree,
                    gap_before,
                });
            }
            self.pending_gap.extend(slot.subtree);
        }
        if self.storage_after.is_none() {
            self.storage_after = self.rules.following();
        }
        None
    }
}

impl<R: Unpin> Iterator for DirectRuleMutationContextIter<'_, '_, R> {
    type Item = DirectRuleMutationContext<R>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        let bridge = self.next.map_or(self.pending_gap, |next| next.gap_before);
        let insertion_anchor = if bridge.is_empty() {
            current.subtree.last_id()
        } else {
            bridge.last_id()
        };
        let storage_before = self.next.map(|next| next.id).or(self.storage_after);
        let context = DirectRuleMutationContext {
            parent: self.parent,
            previous: self.previous.map(|previous| previous.id),
            rule: current.id,
            next: self.next.map(|next| next.id),
            revision: current.revision,
            incoming_bridge: current.gap_before,
            subtree: current.subtree,
            bridge,
            insertion_anchor,
            storage_before,
        };
        self.previous = Some(current);
        self.current = self.next;
        self.next = self.next_live();
        Some(context)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let buffered = usize::from(self.current.is_some()) + usize::from(self.next.is_some());
        (0, Some(self.rules.remaining as usize + buffered))
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/stylesheet/mod.rs"]
mod tests;
