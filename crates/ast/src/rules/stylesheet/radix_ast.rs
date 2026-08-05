//! Typed storage and topology for the compiler's persistent Radix AST.

use rocketcss_common::{
    Allocator, DenseId, DenseStore, RadixId, RadixIdRemap, RadixInsertResult, TypedRadixIndexArena,
    define_dense_id,
};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::hash::Hash;

mod effective_key;
mod mutation;
mod payload;
mod topology;
mod traversal;
mod validation;

pub use effective_key::*;
pub use payload::*;
pub use traversal::*;

/// Stable identity of one rule in the [`RuleStore`]. The type parameter is the
/// rule payload, so `RuleId<P>` can only index an arena storing `RuleRecord<P>`.
pub type RuleId<P> = RadixId<RuleRecord<P>>;

/// Stable identity of one declaration block in the [`DeclarationBlockStore`].
/// The type parameter is the rule payload, so `DeclarationBlockId<P>` can only
/// index an arena storing `DeclarationBlockRecord<P>`.
pub type DeclarationBlockId<P> = RadixId<DeclarationBlockRecord<P>>;

define_dense_id!(pub struct EffectiveKeyId);
define_dense_id!(pub struct DeclarationId);
define_dense_id!(pub struct DeclarationOverflowId);
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
    TypedRadixIndexArena<'ast, DeclarationBlockRecord<P>, DeclarationBlockId<P>>;

/// Interned effective-key records shared by declaration blocks.
type EffectiveKeyStore<P> = DenseStore<EffectiveKeyId, P>;

/// Authored declarations in lexical source order.
type DeclarationStore<P> = DenseStore<DeclarationId, DeclarationRecord<P>>;

/// Arena-backed complete declaration sequences used when a block no longer
/// maps to one contiguous authored range.
type DeclarationOverflowStore<'ast> =
    DenseStore<DeclarationOverflowId, crate::Vec<'ast, DeclarationId>>;

/// Concrete compiler-owned Radix AST.
pub type Compilation<'ast> = RadixCompilation<
    'ast,
    CssRulePayload<'ast>,
    DeclarationPayload<'ast>,
    EffectiveKeyData<CssRulePayload<'ast>>,
>;

/// Concrete rule identity for the compiler-owned [`Compilation`].
pub type ConcreteRuleId<'ast> = RuleId<CssRulePayload<'ast>>;

/// Concrete declaration-block identity for the compiler-owned [`Compilation`].
pub type ConcreteDeclarationBlockId<'ast> = DeclarationBlockId<CssRulePayload<'ast>>;

/// Concrete effective-key payload for the compiler-owned [`Compilation`].
pub type ConcreteEffectiveKey<'ast> = EffectiveKeyData<CssRulePayload<'ast>>;

/// Concrete mutation error for the compiler-owned [`Compilation`].
pub type ConcreteMutationError<'ast> = MutationError<CssRulePayload<'ast>>;

impl<'ast> ConcreteMutationError<'ast> {
    /// Erases the phantom-only arena lifetime so an error can be boxed without
    /// the compilation that produced it. The error never stores a rule payload:
    /// every payload-local identity is a `u32` Radix ID with a
    /// `PhantomData<fn() -> T>` type parameter, so no borrowed data is
    /// discarded and the layout is identical across lifetimes.
    #[inline]
    pub fn erase_arena_lifetime(self) -> ConcreteMutationError<'static> {
        // SAFETY: `MutationError<P>` contains only plain IDs and integers; `P`
        // appears solely inside `PhantomData<fn() -> P>` type parameters.
        unsafe { std::mem::transmute(self) }
    }
}

/// Concrete parser-local semantic context for the compiler-owned [`Compilation`].
pub type ConcreteEffectiveContext<'ast> = EffectiveContext<CssRulePayload<'ast>>;

/// Concrete effective-key history segment for the compiler-owned [`Compilation`].
pub type ConcreteHistorySegment<'ast> = HistorySegment<CssRulePayload<'ast>>;

/// Initial capacities for the compiler-owned AST stores.
///
/// These are allocation hints only. Every store still grows normally when an
/// input contains more nodes than estimated.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompilationCapacity {
    pub rules: usize,
    pub declaration_blocks: usize,
    pub declarations: usize,
    pub selectors: usize,
    pub contexts: usize,
}

/// One contiguous run in the authored declaration tape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeclarationRange {
    start: u32,
    len: u32,
}

impl DeclarationRange {
    #[inline]
    pub const fn start(self) -> u32 {
        self.start
    }

    #[inline]
    pub const fn len(self) -> u32 {
        self.len
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }
}

/// Up to four declaration occurrences stored directly in a block record.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LocalPropertySet {
    declarations: [Option<DeclarationId>; 4],
    len: u8,
}

impl LocalPropertySet {
    fn from_ids(ids: &[DeclarationId]) -> Option<Self> {
        if ids.len() > 4 {
            return None;
        }
        let mut declarations = [None; 4];
        declarations[..ids.len()]
            .iter_mut()
            .zip(ids.iter().copied())
            .for_each(|(slot, id)| *slot = Some(id));
        Some(Self {
            declarations,
            len: ids.len() as u8,
        })
    }

    #[inline]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = DeclarationId> + '_ {
        self.declarations[..usize::from(self.len)]
            .iter()
            .map(|id| id.expect("Local4 entries before len are initialized"))
    }
}

/// Ordered declaration representation owned by one syntax position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclarationList {
    Range(DeclarationRange),
    Local4(LocalPropertySet),
    Overflow(DeclarationOverflowId),
}

impl DeclarationList {
    #[inline]
    pub const fn as_range(self) -> Option<DeclarationRange> {
        match self {
            Self::Range(range) => Some(range),
            Self::Local4(_) | Self::Overflow(_) => None,
        }
    }
}

/// One authored declaration occurrence and its cascade importance.
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
pub struct DeclarationBlockRecord<P> {
    declarations: DeclarationList,
    owner: DeclarationBlockOwner<P>,
    effective_key: EffectiveKeyId,
    revision: u32,
    live: bool,
}

impl<P> DeclarationBlockRecord<P> {
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
    DeclarationOverflowCapacityExhausted,
    UnknownRule(RuleId<P>),
    UnknownEffectiveKey(EffectiveKeyId),
    RetiredRule(RuleId<P>),
    DeclarationBlockAlreadyExists(RuleId<P>),
    UnknownDeclarationBlock(DeclarationBlockId<P>),
    UnknownDeclarationOverflow(DeclarationOverflowId),
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
        range: DeclarationRange,
    },
    InvalidDeclarationOverflow {
        block: DeclarationBlockId<P>,
        overflow: DeclarationOverflowId,
    },
    InvalidDeclarationReference {
        block: DeclarationBlockId<P>,
        declaration: DeclarationId,
    },
    DuplicateDeclarationOwner {
        declaration: DeclarationId,
        first: DeclarationBlockId<P>,
        second: DeclarationBlockId<P>,
    },
    DeclarationRangeStartsOutOfOrder {
        block: DeclarationBlockId<P>,
        expected: u32,
        actual: u32,
    },
    UnownedDeclarations {
        expected: u32,
        actual: u32,
    },
}

/// `R` and `D` keep rule and declaration payloads independent from storage.
pub struct RadixCompilation<'ast, R: Unpin, D, K> {
    allocator: &'ast Allocator,
    license_comments: crate::Vec<'ast, &'ast str>,
    rules: RuleStore<'ast, R>,
    declaration_blocks: DeclarationBlockStore<'ast, R>,
    declarations: DeclarationStore<D>,
    declaration_overflows: DeclarationOverflowStore<'ast>,
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

impl<'ast, R: Unpin, D, K> RadixCompilation<'ast, R, D, K> {
    /// Creates an empty compilation.
    pub fn new_in(allocator: &'ast Allocator) -> Self {
        Self::with_capacity_in(allocator, CompilationCapacity::default())
    }

    /// Creates an empty compilation with capacity for the expected authored
    /// AST shape.
    pub fn with_capacity_in(allocator: &'ast Allocator, capacity: CompilationCapacity) -> Self {
        Self {
            allocator,
            license_comments: allocator.vec(),
            rules: RuleStore::with_capacity_in(capacity.rules, allocator),
            declaration_blocks: DeclarationBlockStore::with_capacity_in(
                capacity.declaration_blocks,
                allocator,
            ),
            declarations: DeclarationStore::with_capacity(capacity.declarations),
            declaration_overflows: DeclarationOverflowStore::new(),
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
    pub fn declaration_block(
        &self,
        id: DeclarationBlockId<R>,
    ) -> Option<&DeclarationBlockRecord<R>> {
        self.declaration_blocks.get(id)
    }

    #[inline]
    pub fn declaration_block_mut(
        &mut self,
        id: DeclarationBlockId<R>,
    ) -> Option<&mut DeclarationBlockRecord<R>> {
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
        self.declarations.try_get(id)
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
    /// compilation, but must not insert, retire, or relabel rules while this
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
    ) -> impl Iterator<Item = (DeclarationBlockId<R>, &DeclarationBlockRecord<R>)> {
        self.declaration_blocks.iter_enumerated()
    }

    /// Iterates authored declarations in lexical source order.
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
    ) -> Result<DeclarationIter<'_, D>, MutationError<R>> {
        let declarations = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?
            .declarations;
        let kind = match declarations {
            DeclarationList::Range(range) => {
                let start = range.start as usize;
                let end = start + range.len as usize;
                let records = self
                    .declarations
                    .as_slice()
                    .get(start..end)
                    .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))?;
                DeclarationIterKind::Range(records.iter())
            }
            DeclarationList::Local4(local) => DeclarationIterKind::Indirect {
                ids: DeclarationIdIter {
                    kind: DeclarationIdIterKind::Local4 { local, index: 0 },
                },
                declarations: &self.declarations,
            },
            DeclarationList::Overflow(overflow) => {
                let ids = self
                    .declaration_overflows
                    .try_get(overflow)
                    .ok_or(MutationError::<R>::UnknownDeclarationOverflow(overflow))?;
                DeclarationIterKind::Indirect {
                    ids: DeclarationIdIter {
                        kind: DeclarationIdIterKind::Overflow(ids.iter()),
                    },
                    declarations: &self.declarations,
                }
            }
        };
        Ok(DeclarationIter { kind })
    }

    /// Iterates the typed declaration IDs and records owned by `block`.
    ///
    /// Consumers that keep sidecars should key them by these source-order IDs
    /// instead of rebuilding a second occurrence identity.
    pub fn declaration_occurrences_in_block(
        &self,
        block: DeclarationBlockId<R>,
    ) -> Result<DeclarationOccurrenceIter<'_, D>, MutationError<R>> {
        let declarations = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?
            .declarations;
        let kind = match declarations {
            DeclarationList::Range(range) => {
                let start = range.start as usize;
                let end = start + range.len as usize;
                let records = self
                    .declarations
                    .as_slice()
                    .get(start..end)
                    .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))?;
                DeclarationOccurrenceIterKind::Range {
                    records: records.iter(),
                    next: start,
                }
            }
            DeclarationList::Local4(local) => DeclarationOccurrenceIterKind::Indirect {
                ids: DeclarationIdIter {
                    kind: DeclarationIdIterKind::Local4 { local, index: 0 },
                },
                declarations: &self.declarations,
            },
            DeclarationList::Overflow(overflow) => {
                let ids = self
                    .declaration_overflows
                    .try_get(overflow)
                    .ok_or(MutationError::<R>::UnknownDeclarationOverflow(overflow))?;
                DeclarationOccurrenceIterKind::Indirect {
                    ids: DeclarationIdIter {
                        kind: DeclarationIdIterKind::Overflow(ids.iter()),
                    },
                    declarations: &self.declarations,
                }
            }
        };
        Ok(DeclarationOccurrenceIter { kind })
    }

    /// Iterates only declaration identities for one block representation.
    pub fn declaration_ids_in_block(
        &self,
        block: DeclarationBlockId<R>,
    ) -> Result<DeclarationIdIter<'_>, MutationError<R>> {
        let block = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?;
        let kind = match block.declarations {
            DeclarationList::Range(range) => DeclarationIdIterKind::Range(
                range.start as usize..range.start as usize + range.len as usize,
            ),
            DeclarationList::Local4(local) => DeclarationIdIterKind::Local4 { local, index: 0 },
            DeclarationList::Overflow(overflow) => DeclarationIdIterKind::Overflow(
                self.declaration_overflows
                    .try_get(overflow)
                    .ok_or(MutationError::<R>::UnknownDeclarationOverflow(overflow))?
                    .iter(),
            ),
        };
        Ok(DeclarationIdIter { kind })
    }

    /// Visits declaration records in the semantic order owned by `block`.
    ///
    /// The representation is resolved once and the callback receives each
    /// typed occurrence directly. In particular, a contiguous [`Range`] is
    /// streamed from the authored declaration store instead of first copying
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

        let mut visited = 0usize;
        match declaration_list {
            DeclarationList::Range(range) => {
                let start = range.start as usize;
                let end = start
                    .checked_add(range.len as usize)
                    .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))?;
                let records = self
                    .declarations
                    .as_mut_slice()
                    .get_mut(start..end)
                    .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))?;
                for (offset, record) in records.iter_mut().enumerate() {
                    let declaration = DeclarationId::from_index(start + offset)
                        .expect("the declaration store length fits its typed ID");
                    visit(declaration, record);
                    visited += 1;
                }
            }
            DeclarationList::Local4(local) => {
                for declaration in local.iter() {
                    let record = self
                        .declarations
                        .try_get_mut(declaration)
                        .ok_or(MutationError::<R>::UnknownDeclaration(declaration))?;
                    visit(declaration, record);
                    visited += 1;
                }
            }
            DeclarationList::Overflow(overflow) => {
                let ids = self
                    .declaration_overflows
                    .try_get(overflow)
                    .ok_or(MutationError::<R>::UnknownDeclarationOverflow(overflow))?;
                // The overflow ID tape and the declaration store are
                // independent fields, so the ID slice can stay borrowed while
                // the corresponding declaration records are mutated.
                for &declaration in ids.iter() {
                    let record = self
                        .declarations
                        .try_get_mut(declaration)
                        .ok_or(MutationError::<R>::UnknownDeclaration(declaration))?;
                    visit(declaration, record);
                    visited += 1;
                }
            }
        }

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
        match declaration_list {
            DeclarationList::Range(range) => {
                if index >= range.len as usize {
                    return Err(MutationError::<R>::NonContiguousDeclarationRange(block));
                }
                let index = u32::try_from(index)
                    .map_err(|_| MutationError::<R>::NonContiguousDeclarationRange(block))?;
                range
                    .start
                    .checked_add(index)
                    .and_then(|index| DeclarationId::from_index(index as usize))
                    .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))
            }
            DeclarationList::Local4(local) => local
                .iter()
                .nth(index)
                .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block)),
            DeclarationList::Overflow(overflow) => self
                .declaration_overflows
                .try_get(overflow)
                .ok_or(MutationError::<R>::UnknownDeclarationOverflow(overflow))?
                .get(index)
                .copied()
                .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block)),
        }
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

    pub fn previous_sibling(&self, id: RuleId<R>) -> Option<RuleId<R>> {
        let parent = self.rules.get(id)?.parent;
        let mut previous = None;
        for (candidate, _) in self.direct_rules(parent).ok()? {
            if candidate == id {
                return previous;
            }
            previous = Some(candidate);
        }
        None
    }

    pub fn next_sibling(&self, id: RuleId<R>) -> Option<RuleId<R>> {
        let parent = self.rules.get(id)?.parent;
        let mut rules = self.direct_rules(parent).ok()?;
        while let Some((candidate, _)) = rules.next() {
            if candidate == id {
                return rules.next().map(|(next, _)| next);
            }
        }
        None
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
        let block = self
            .declaration_blocks
            .push_primary(DeclarationBlockRecord::<R> {
                declarations: DeclarationList::Range(DeclarationRange {
                    start: self.declarations.len() as u32,
                    len: 0,
                }),
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
    pub fn append_declaration(
        &mut self,
        block: DeclarationBlockId<R>,
        payload: D,
        important: bool,
    ) -> Result<DeclarationId, MutationError<R>> {
        let range = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::<R>::UnknownDeclarationBlock(block))?
            .declarations
            .as_range()
            .ok_or(MutationError::<R>::NonContiguousDeclarationRange(block))?;
        let next = self
            .declarations
            .try_next_id()
            .map_err(|_| MutationError::<R>::DeclarationCapacityExhausted)?;
        if range.start as usize + range.len as usize != next.index() {
            return Err(MutationError::<R>::NonContiguousDeclarationRange(block));
        }
        let declaration = self
            .declarations
            .try_push(DeclarationRecord { payload, important })
            .map_err(|_| MutationError::<R>::DeclarationCapacityExhausted)?;
        let block = self
            .declaration_blocks
            .get_mut(block)
            .expect("the block was validated before appending its declaration");
        let DeclarationList::Range(range) = &mut block.declarations else {
            unreachable!("the declaration representation was validated before append")
        };
        range.len += 1;
        Ok(declaration)
    }
}

/// Ordered declaration identities resolved from one block representation.
pub struct DeclarationIdIter<'comp> {
    kind: DeclarationIdIterKind<'comp>,
}

enum DeclarationIdIterKind<'comp> {
    Range(std::ops::Range<usize>),
    Local4 { local: LocalPropertySet, index: u8 },
    Overflow(std::slice::Iter<'comp, DeclarationId>),
}

impl Iterator for DeclarationIdIter<'_> {
    type Item = DeclarationId;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            DeclarationIdIterKind::Range(range) => DeclarationId::from_index(range.next()?),
            DeclarationIdIterKind::Local4 { local, index } => {
                let declaration = local.declarations.get(usize::from(*index))?.as_ref()?;
                *index += 1;
                Some(*declaration)
            }
            DeclarationIdIterKind::Overflow(ids) => ids.next().copied(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match &self.kind {
            DeclarationIdIterKind::Range(range) => range.len(),
            DeclarationIdIterKind::Local4 { local, index } => {
                usize::from(local.len.saturating_sub(*index))
            }
            DeclarationIdIterKind::Overflow(ids) => ids.len(),
        };
        (len, Some(len))
    }
}

impl ExactSizeIterator for DeclarationIdIter<'_> {}

/// Ordered declaration payloads resolved from one block representation.
pub struct DeclarationIter<'comp, D> {
    kind: DeclarationIterKind<'comp, D>,
}

enum DeclarationIterKind<'comp, D> {
    Range(std::slice::Iter<'comp, DeclarationRecord<D>>),
    Indirect {
        ids: DeclarationIdIter<'comp>,
        declarations: &'comp DeclarationStore<D>,
    },
}

impl<'comp, D> Iterator for DeclarationIter<'comp, D> {
    type Item = &'comp DeclarationRecord<D>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            DeclarationIterKind::Range(records) => records.next(),
            DeclarationIterKind::Indirect { ids, declarations } => {
                Some(declarations.get(ids.next()?))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match &self.kind {
            DeclarationIterKind::Range(records) => records.len(),
            DeclarationIterKind::Indirect { ids, .. } => ids.len(),
        };
        (len, Some(len))
    }
}

impl<D> ExactSizeIterator for DeclarationIter<'_, D> {}

/// Ordered declaration identities and payloads resolved from one block.
pub struct DeclarationOccurrenceIter<'comp, D> {
    kind: DeclarationOccurrenceIterKind<'comp, D>,
}

enum DeclarationOccurrenceIterKind<'comp, D> {
    Range {
        records: std::slice::Iter<'comp, DeclarationRecord<D>>,
        next: usize,
    },
    Indirect {
        ids: DeclarationIdIter<'comp>,
        declarations: &'comp DeclarationStore<D>,
    },
}

impl<'comp, D> Iterator for DeclarationOccurrenceIter<'comp, D> {
    type Item = (DeclarationId, &'comp DeclarationRecord<D>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            DeclarationOccurrenceIterKind::Range { records, next } => {
                let record = records.next()?;
                let id = DeclarationId::from_index(*next)
                    .expect("a declaration range is backed by its dense store");
                *next += 1;
                Some((id, record))
            }
            DeclarationOccurrenceIterKind::Indirect { ids, declarations } => {
                let id = ids.next()?;
                Some((id, declarations.get(id)))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match &self.kind {
            DeclarationOccurrenceIterKind::Range { records, .. } => records.len(),
            DeclarationOccurrenceIterKind::Indirect { ids, .. } => ids.len(),
        };
        (len, Some(len))
    }
}

impl<D> ExactSizeIterator for DeclarationOccurrenceIter<'_, D> {}

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
mod tests;
