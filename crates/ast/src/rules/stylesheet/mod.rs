//! Typed storage and topology for [`StyleSheet`].

use rocketcss_common::{
    Allocator, DenseStore, RadixId, RadixIdRemap, RadixInsertResult, RadixRange,
    TypedRadixIndexArena, define_dense_id,
};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::hash::Hash;

mod declaration;
mod effective_key;
mod mutation;
mod rule;
mod topology;
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
    nested_rule_count: u32,
    declaration_block: Option<DeclarationBlockId<P>>,
    revision: u32,
    live: bool,
}

impl<P> RuleRecord<P> {
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

/// Live topology exposed by retiring one leaf rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetiredRule<P> {
    pub id: RuleId<P>,
    pub previous: Option<RuleId<P>>,
    pub next: Option<RuleId<P>>,
    pub declaration_block: Option<DeclarationBlockId<P>>,
}

/// Result of folding one direct left sibling into the retained right rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MergedAdjacentRuleBlocks<P> {
    pub retired_rule: RuleId<P>,
    pub retired_block: DeclarationBlockId<P>,
    pub retained_rule: RuleId<P>,
    pub retained_block: DeclarationBlockId<P>,
    pub effective_key: EffectiveKeyId,
}

/// Final AST identities allocated by one synthesized rule-and-block
/// transaction.
#[derive(Debug)]
pub struct InsertedRuleWithDeclarationBlock<P> {
    pub rule: RadixInsertResult<RuleId<P>>,
    pub declaration_block: RadixInsertResult<DeclarationBlockId<P>>,
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
        }
    }

    #[inline]
    pub const fn allocator(&self) -> &'ast Allocator {
        self.allocator
    }

    #[inline]
    pub(crate) fn first_rule_in_source(&self) -> Option<RuleId<R>> {
        self.rules.primary_id(0)
    }

    #[inline]
    pub(crate) fn next_rule_in_source(&self, id: RuleId<R>) -> Option<RuleId<R>> {
        self.rules.next_id(id)
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
    pub fn declaration_block_count(&self) -> usize {
        self.declaration_blocks.len()
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
        let mut current = self.first_rule_in_source();
        while let Some(rule) = current {
            current = self.next_rule_in_source(rule);
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

    /// Resolves one semantic declaration position without materializing the
    /// block's ID sequence.
    #[doc(hidden)]
    pub fn declaration_id_at_in_block(
        &self,
        block: DeclarationBlockId<R>,
        index: usize,
    ) -> Result<DeclarationId, MutationError<R>> {
        let declaration_list = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?
            .declarations;
        let index = u32::try_from(index)
            .map_err(|_| MutationError::<R>::NonContiguousDeclarationRange(block))?;
        self.declarations
            .id_at(declaration_list, index)
            .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))
    }

    fn direct_rules(
        &self,
        parent: Option<RuleId<R>>,
    ) -> Result<DirectRuleIter<'_, 'ast, R>, MutationError<R>> {
        let (next, remaining) = if let Some(parent) = parent {
            let record = self
                .rules
                .get(parent)
                .ok_or(MutationError::<R>::UnknownRule(parent))?;
            (
                (record.nested_rule_count != 0)
                    .then(|| self.rules.next_id(parent))
                    .flatten(),
                record.nested_rule_count,
            )
        } else {
            (self.rules.primary_id(0), self.rules.len() as u32)
        };
        Ok(DirectRuleIter {
            rules: &self.rules,
            next,
            remaining,
        })
    }

    /// Iterates live top-level rules without visiting their descendants.
    #[inline]
    pub fn root_rules(&self) -> impl Iterator<Item = (RuleId<R>, &RuleRecord<R>)> {
        DirectRuleIter {
            rules: &self.rules,
            next: self.rules.primary_id(0),
            remaining: self.rules.len() as u32,
        }
    }

    /// Iterates live direct nested rules without visiting deeper descendants.
    pub fn nested_rules(
        &self,
        parent: RuleId<R>,
    ) -> Result<impl Iterator<Item = (RuleId<R>, &RuleRecord<R>)>, MutationError<R>> {
        let record = self
            .rules
            .get(parent)
            .ok_or(MutationError::<R>::UnknownRule(parent))?;
        if !record.live {
            return Err(MutationError::<R>::RetiredRule(parent));
        }
        self.direct_rules(Some(parent))
    }

    /// Iterates the live rules in `rule`'s direct sibling list, including
    /// `rule` itself. Consumers that need adjacency must carry neighboring
    /// IDs while advancing this iterator instead of issuing per-rule lookups.
    pub fn sibling_rules(
        &self,
        rule: RuleId<R>,
    ) -> Result<impl Iterator<Item = (RuleId<R>, &RuleRecord<R>)>, MutationError<R>> {
        let record = self
            .rules
            .get(rule)
            .ok_or(MutationError::<R>::UnknownRule(rule))?;
        if !record.live {
            return Err(MutationError::<R>::RetiredRule(rule));
        }
        self.direct_rules(record.parent)
    }

    /// Returns whether a live rule has at least one live direct nested rule.
    #[inline]
    pub fn has_nested_rules(&self, parent: RuleId<R>) -> Result<bool, MutationError<R>> {
        Ok(self.nested_rules(parent)?.next().is_some())
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

        let id = self.rules.push_primary(RuleRecord {
            payload,
            parent,
            nested_rule_count: 0,
            declaration_block: None,
            revision: 0,
            live: true,
        });
        let mut ancestor = parent;
        while let Some(ancestor_id) = ancestor {
            let record = self
                .rules
                .get_mut(ancestor_id)
                .expect("an appended rule's validated ancestor remains resolvable");
            record.nested_rule_count += 1;
            ancestor = record.parent;
        }
        Ok(id)
    }

    #[doc(hidden)]
    #[inline]
    pub fn authored_rule_count(&self) -> usize {
        self.rules.primary_len()
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
        let is_active_authored_block = !self.declaration_blocks.has_siblings()
            && self
                .declaration_blocks
                .primary_len()
                .checked_sub(1)
                .and_then(|index| self.declaration_blocks.primary_id(index))
                == Some(block);
        if !is_active_authored_block || self.declarations.has_siblings() {
            return Err(MutationError::<R>::NonContiguousDeclarationRange(block));
        }
        let range = block_record.declarations;
        if !range.is_empty()
            && self.declarations.last_id(range)
                != self
                    .declarations
                    .primary_len()
                    .checked_sub(1)
                    .and_then(|index| self.declarations.primary_id(index))
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
            block.declarations.extend_by(1);
        }
        Ok(declaration)
    }
}

/// Direct-sibling iterator over lexical-preorder subtree spans.
struct DirectRuleIter<'comp, 'ast, R: Unpin> {
    rules: &'comp RuleStore<'ast, R>,
    next: Option<RuleId<R>>,
    remaining: u32,
}

impl<'comp, 'ast, R: Unpin> Iterator for DirectRuleIter<'comp, 'ast, R> {
    type Item = (RuleId<R>, &'comp RuleRecord<R>);

    fn next(&mut self) -> Option<Self::Item> {
        while self.remaining != 0 {
            let id = self.next?;
            let rule = self.rules.get(id)?;
            let span = rule.nested_rule_count.checked_add(1)?;
            if span > self.remaining {
                self.remaining = 0;
                self.next = None;
                return None;
            }
            self.remaining -= span;
            self.next = (self.remaining != 0)
                .then(|| self.rules.advance_id(id, span))
                .flatten();
            if rule.live {
                return Some((id, rule));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.remaining as usize))
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/stylesheet/mod.rs"]
mod tests;
