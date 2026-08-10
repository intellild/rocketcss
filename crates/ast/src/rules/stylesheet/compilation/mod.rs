//! Typed dense storage and topology for the compiler's persistent AST.

use rocketcss_common::{Allocator, DenseId, DenseIdRange, DenseStore};
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
/// rule payload, so `RuleId<'ast, P>` can only index an arena storing `RuleRecord<P>`.
pub type RuleId<'ast, P> = DenseId<'ast, RuleRecord<'ast, P>>;

/// Stable identity of one declaration block in the [`DeclarationBlockStore`].
/// The type parameter is the rule payload, so `DeclarationBlockId<'ast, P>` can only
/// index an arena storing `DeclarationBlockRecord<P>`.
pub type DeclarationBlockId<'ast, P> = DenseId<'ast, DeclarationBlockRecord<'ast, P>>;

#[doc(hidden)]
pub enum RuleListDomain {}
#[doc(hidden)]
pub enum EffectiveKeyDomain {}
#[doc(hidden)]
pub enum DeclarationDomain {}
#[doc(hidden)]
pub enum DeclarationOverflowDomain {}
#[doc(hidden)]
pub enum SelectorValueDomain {}
#[doc(hidden)]
pub enum SelectorPathDomain {}
#[doc(hidden)]
pub enum ContextValueDomain {}
#[doc(hidden)]
pub enum ContextPathDomain {}
#[doc(hidden)]
pub enum LayerContextDomain {}

pub type RuleListId<'ast> = DenseId<'ast, RuleListDomain>;
pub type EffectiveKeyId<'ast> = DenseId<'ast, EffectiveKeyDomain>;
pub type DeclarationId<'ast> = DenseId<'ast, DeclarationDomain>;
pub type DeclarationOverflowId<'ast> = DenseId<'ast, DeclarationOverflowDomain>;
pub type SelectorValueId<'ast> = DenseId<'ast, SelectorValueDomain>;
pub type SelectorPathId<'ast> = DenseId<'ast, SelectorPathDomain>;
pub type ContextValueId<'ast> = DenseId<'ast, ContextValueDomain>;
pub type ContextPathId<'ast> = DenseId<'ast, ContextPathDomain>;
pub type LayerContextId<'ast> = DenseId<'ast, LayerContextDomain>;

/// Compilation-local label whose ordering matches semantic source order.
///
/// Unlike [`RuleId`], this is not a storage identity. Local insertions allocate a
/// label between their source neighbors, and the compilation may relabel every
/// rule when a local gap is exhausted. AST references therefore continue to use
/// stable dense IDs while order-sensitive passes can compare this compact key.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceOrderId(u64);

/// Rules in stable dense allocation order. Semantic order is owned by rule
/// topology and the source-order links on each record.
pub type RuleStore<'ast, P> = DenseStore<'ast, RuleRecord<'ast, P>, RuleRecord<'ast, P>>;

/// Declaration blocks in stable dense allocation order.
pub type DeclarationBlockStore<'ast, P> =
    DenseStore<'ast, DeclarationBlockRecord<'ast, P>, DeclarationBlockRecord<'ast, P>>;

/// Dense rule-list metadata. Lists own topology, not a second rule vector.
pub type RuleListStore<'ast, P> = DenseStore<'ast, RuleListDomain, RuleList<'ast, P>>;

/// Interned effective-key records shared by declaration blocks.
pub type EffectiveKeyStore<'ast, P> = DenseStore<'ast, EffectiveKeyDomain, P>;

/// Authored declarations in lexical source order.
pub type DeclarationStore<'ast, P> = DenseStore<'ast, DeclarationDomain, DeclarationRecord<P>>;

/// Arena-backed complete declaration sequences used when a block no longer
/// maps to one contiguous authored range.
pub type DeclarationOverflowStore<'ast> =
    DenseStore<'ast, DeclarationOverflowDomain, crate::Vec<'ast, DeclarationId<'ast>>>;

/// Concrete compiler-owned AST.
pub type Compilation<'ast> = RadixCompilation<
    'ast,
    CssRulePayload<'ast>,
    DeclarationPayload<'ast>,
    EffectiveKeyData<'ast, CssRulePayload<'ast>>,
>;

/// Concrete rule identity for the compiler-owned [`Compilation`].
pub type ConcreteRuleId<'ast> = RuleId<'ast, CssRulePayload<'ast>>;

/// Concrete declaration-block identity for the compiler-owned [`Compilation`].
pub type ConcreteDeclarationBlockId<'ast> = DeclarationBlockId<'ast, CssRulePayload<'ast>>;

/// Concrete effective-key payload for the compiler-owned [`Compilation`].
pub type ConcreteEffectiveKey<'ast> = EffectiveKeyData<'ast, CssRulePayload<'ast>>;

/// Concrete mutation error for the compiler-owned [`Compilation`].
pub type ConcreteMutationError<'ast> = MutationError<'ast, CssRulePayload<'ast>>;

impl<'ast> ConcreteMutationError<'ast> {
    /// Erases the phantom-only arena lifetime so an error can be boxed without
    /// the compilation that produced it. The error never stores a rule payload:
    /// every identity is a packed `u32` whose arena and domain parameters are
    /// carried only by `PhantomData<fn() -> &'ast T>`, so no borrowed data is
    /// discarded and the layout is identical across lifetimes.
    #[inline]
    pub fn erase_arena_lifetime(self) -> ConcreteMutationError<'static> {
        // SAFETY: `MutationError<'ast, P>` contains only packed IDs and
        // integers. All `'ast` occurrences are inside `DenseId` phantom
        // data; the error stores neither an arena reference nor a payload.
        unsafe { std::mem::transmute(self) }
    }
}

/// Concrete parser-local semantic context for the compiler-owned [`Compilation`].
pub type ConcreteEffectiveContext<'ast> = EffectiveContext<'ast, CssRulePayload<'ast>>;

/// Concrete effective-key history segment for the compiler-owned [`Compilation`].
pub type ConcreteHistorySegment<'ast> = HistorySegment<'ast, CssRulePayload<'ast>>;

/// Initial capacities for the compiler-owned AST stores.
///
/// These are allocation hints only. Every store still grows normally when an
/// input contains more nodes than estimated.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompilationCapacity {
    pub rules: usize,
    pub rule_lists: usize,
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
pub struct LocalPropertySet<'ast> {
    declarations: [Option<DeclarationId<'ast>>; 4],
    len: u8,
}

impl<'ast> LocalPropertySet<'ast> {
    fn from_ids(ids: &[DeclarationId<'ast>]) -> Option<Self> {
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
    pub fn iter(&self) -> impl ExactSizeIterator<Item = DeclarationId<'ast>> + '_ {
        self.declarations[..usize::from(self.len)]
            .iter()
            .map(|id| id.expect("Local4 entries before len are initialized"))
    }
}

/// Ordered declaration representation owned by one syntax position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclarationList<'ast> {
    Range(DeclarationRange),
    Local4(LocalPropertySet<'ast>),
    Overflow(DeclarationOverflowId<'ast>),
}

impl DeclarationList<'_> {
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

/// The lightweight root of a compilation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StyleSheet<'ast> {
    root_rules: RuleListId<'ast>,
}

impl<'ast> StyleSheet<'ast> {
    #[inline]
    pub const fn root_rules(self) -> RuleListId<'ast> {
        self.root_rules
    }
}

/// Direct topology for one authored or synthesized CSS rule.
#[derive(Debug, PartialEq, Eq)]
pub struct RuleRecord<'ast, P> {
    payload: P,
    source_order_id: SourceOrderId,
    parent: Option<RuleId<'ast, P>>,
    parent_list: RuleListId<'ast>,
    previous_sibling: Option<RuleId<'ast, P>>,
    next_sibling: Option<RuleId<'ast, P>>,
    previous_in_source: Option<RuleId<'ast, P>>,
    next_in_source: Option<RuleId<'ast, P>>,
    child_list: Option<RuleListId<'ast>>,
    declaration_block: Option<DeclarationBlockId<'ast, P>>,
    revision: u32,
    live: bool,
}

impl<'ast, P> RuleRecord<'ast, P> {
    #[inline]
    pub const fn payload(&self) -> &P {
        &self.payload
    }

    #[inline]
    pub fn payload_mut(&mut self) -> &mut P {
        &mut self.payload
    }

    #[inline]
    pub const fn source_order_id(&self) -> SourceOrderId {
        self.source_order_id
    }

    #[inline]
    pub const fn parent(&self) -> Option<RuleId<'ast, P>> {
        self.parent
    }

    #[inline]
    pub const fn parent_list(&self) -> RuleListId<'ast> {
        self.parent_list
    }

    #[inline]
    pub const fn previous_sibling(&self) -> Option<RuleId<'ast, P>> {
        self.previous_sibling
    }

    #[inline]
    pub const fn next_sibling(&self) -> Option<RuleId<'ast, P>> {
        self.next_sibling
    }

    #[inline]
    pub const fn previous_in_source(&self) -> Option<RuleId<'ast, P>> {
        self.previous_in_source
    }

    #[inline]
    pub const fn next_in_source(&self) -> Option<RuleId<'ast, P>> {
        self.next_in_source
    }

    #[inline]
    pub const fn child_list(&self) -> Option<RuleListId<'ast>> {
        self.child_list
    }

    #[inline]
    pub const fn declaration_block(&self) -> Option<DeclarationBlockId<'ast, P>> {
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

/// Endpoints and ownership of one direct CSS rule list.
pub struct RuleList<'ast, P> {
    parent: Option<RuleId<'ast, P>>,
    first: Option<RuleId<'ast, P>>,
    last: Option<RuleId<'ast, P>>,
    live_len: u32,
}

impl<P> Clone for RuleList<'_, P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for RuleList<'_, P> {}

impl<P> std::fmt::Debug for RuleList<'_, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuleList")
            .field("parent", &self.parent)
            .field("first", &self.first)
            .field("last", &self.last)
            .field("live_len", &self.live_len)
            .finish()
    }
}

impl<P> PartialEq for RuleList<'_, P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent
            && self.first == other.first
            && self.last == other.last
            && self.live_len == other.live_len
    }
}

impl<P> Eq for RuleList<'_, P> {}

impl<'ast, P> RuleList<'ast, P> {
    #[inline]
    pub const fn parent(self) -> Option<RuleId<'ast, P>> {
        self.parent
    }

    #[inline]
    pub const fn first(self) -> Option<RuleId<'ast, P>> {
        self.first
    }

    #[inline]
    pub const fn last(self) -> Option<RuleId<'ast, P>> {
        self.last
    }

    #[inline]
    pub const fn live_len(self) -> u32 {
        self.live_len
    }
}

/// The unique syntax owner of a declaration block.
pub enum DeclarationBlockOwner<'ast, P> {
    Rule(RuleId<'ast, P>),
}

impl<P> Clone for DeclarationBlockOwner<'_, P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for DeclarationBlockOwner<'_, P> {}

impl<P> std::fmt::Debug for DeclarationBlockOwner<'_, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule(rule) => f.debug_tuple("Rule").field(rule).finish(),
        }
    }
}

impl<P> PartialEq for DeclarationBlockOwner<'_, P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rule(left), Self::Rule(right)) => left == right,
        }
    }
}

impl<P> Eq for DeclarationBlockOwner<'_, P> {}

/// A declaration-list handle plus its persistent AST identity.
#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationBlockRecord<'ast, P> {
    declarations: DeclarationList<'ast>,
    owner: DeclarationBlockOwner<'ast, P>,
    effective_key: EffectiveKeyId<'ast>,
    revision: u32,
    live: bool,
}

impl<'ast, P> DeclarationBlockRecord<'ast, P> {
    #[inline]
    pub const fn declarations(&self) -> DeclarationList<'ast> {
        self.declarations
    }

    #[inline]
    pub const fn owner(&self) -> DeclarationBlockOwner<'ast, P> {
        self.owner
    }

    #[inline]
    pub const fn effective_key(&self) -> EffectiveKeyId<'ast> {
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
pub enum MutationError<'ast, P> {
    RuleCapacityExhausted,
    DeclarationBlockCapacityExhausted,
    RuleListCapacityExhausted,
    EffectiveKeyCapacityExhausted,
    SelectorContextCapacityExhausted,
    DeclarationCapacityExhausted,
    DeclarationOverflowCapacityExhausted,
    UnknownRule(RuleId<'ast, P>),
    UnknownRuleList(RuleListId<'ast>),
    UnknownEffectiveKey(EffectiveKeyId<'ast>),
    RetiredRule(RuleId<'ast, P>),
    ChildListAlreadyExists(RuleId<'ast, P>),
    DeclarationBlockAlreadyExists(RuleId<'ast, P>),
    UnknownDeclarationBlock(DeclarationBlockId<'ast, P>),
    UnknownDeclarationOverflow(DeclarationOverflowId<'ast>),
    UnknownDeclaration(DeclarationId<'ast>),
    NonContiguousDeclarationRange(DeclarationBlockId<'ast, P>),
    InvalidRuleTopology(RuleId<'ast, P>),
    InvalidSourceTopology,
    RuleHasChildren(RuleId<'ast, P>),
}

/// Live topology exposed by retiring one leaf rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetiredRule<'ast, P> {
    pub id: RuleId<'ast, P>,
    pub list: RuleListId<'ast>,
    pub previous: Option<RuleId<'ast, P>>,
    pub next: Option<RuleId<'ast, P>>,
    pub declaration_block: Option<DeclarationBlockId<'ast, P>>,
}

/// Result of folding one direct left sibling into the retained right rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MergedAdjacentRuleBlocks<'ast, P> {
    pub retired_rule: RuleId<'ast, P>,
    pub retired_block: DeclarationBlockId<'ast, P>,
    pub retained_rule: RuleId<'ast, P>,
    pub retained_block: DeclarationBlockId<'ast, P>,
    pub effective_key: EffectiveKeyId<'ast>,
}

/// A violated store, ownership, or direct-topology invariant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationError<'ast, P> {
    MissingRootRuleList(RuleListId<'ast>),
    RootRuleListHasParent(RuleId<'ast, P>),
    MissingListParent {
        list: RuleListId<'ast>,
        parent: RuleId<'ast, P>,
    },
    RetiredListParent {
        list: RuleListId<'ast>,
        parent: RuleId<'ast, P>,
    },
    ParentDoesNotOwnList {
        list: RuleListId<'ast>,
        parent: RuleId<'ast, P>,
    },
    InvalidListEndpoints(RuleListId<'ast>),
    MissingRule(RuleId<'ast, P>),
    RetiredRuleInList {
        list: RuleListId<'ast>,
        rule: RuleId<'ast, P>,
    },
    RuleHasWrongParentList {
        list: RuleListId<'ast>,
        rule: RuleId<'ast, P>,
    },
    RuleHasWrongParent {
        list: RuleListId<'ast>,
        rule: RuleId<'ast, P>,
    },
    RuleHasWrongPrevious {
        rule: RuleId<'ast, P>,
        expected: Option<RuleId<'ast, P>>,
    },
    ListDoesNotEndAtLast(RuleListId<'ast>),
    ListLengthMismatch {
        list: RuleListId<'ast>,
        expected: u32,
        actual: u32,
    },
    LiveRuleIsNotInOneList(RuleId<'ast, P>),
    MissingOwnedChildList {
        rule: RuleId<'ast, P>,
        list: RuleListId<'ast>,
    },
    ChildListHasWrongParent {
        rule: RuleId<'ast, P>,
        list: RuleListId<'ast>,
        actual: Option<RuleId<'ast, P>>,
    },
    MissingOwnedDeclarationBlock {
        rule: RuleId<'ast, P>,
        block: DeclarationBlockId<'ast, P>,
    },
    DeclarationBlockHasWrongOwner {
        rule: RuleId<'ast, P>,
        block: DeclarationBlockId<'ast, P>,
        actual: RuleId<'ast, P>,
    },
    MissingBlockOwner {
        block: DeclarationBlockId<'ast, P>,
        owner: RuleId<'ast, P>,
    },
    RetiredBlockOwner {
        block: DeclarationBlockId<'ast, P>,
        owner: RuleId<'ast, P>,
    },
    OwnerDoesNotReferenceBlock {
        block: DeclarationBlockId<'ast, P>,
        owner: RuleId<'ast, P>,
        actual: Option<DeclarationBlockId<'ast, P>>,
    },
    MissingEffectiveKey {
        block: DeclarationBlockId<'ast, P>,
        key: EffectiveKeyId<'ast>,
    },
    InvalidDeclarationRange {
        block: DeclarationBlockId<'ast, P>,
        range: DeclarationRange,
    },
    InvalidDeclarationOverflow {
        block: DeclarationBlockId<'ast, P>,
        overflow: DeclarationOverflowId<'ast>,
    },
    InvalidDeclarationReference {
        block: DeclarationBlockId<'ast, P>,
        declaration: DeclarationId<'ast>,
    },
    DuplicateDeclarationOwner {
        declaration: DeclarationId<'ast>,
        first: DeclarationBlockId<'ast, P>,
        second: DeclarationBlockId<'ast, P>,
    },
    DeclarationRangeStartsOutOfOrder {
        block: DeclarationBlockId<'ast, P>,
        expected: u32,
        actual: u32,
    },
    UnownedDeclarations {
        expected: u32,
        actual: u32,
    },
    InvalidSourcePrevious {
        rule: RuleId<'ast, P>,
        expected: Option<RuleId<'ast, P>>,
        actual: Option<RuleId<'ast, P>>,
    },
    InvalidSourceNext {
        rule: RuleId<'ast, P>,
        expected: Option<RuleId<'ast, P>>,
        actual: Option<RuleId<'ast, P>>,
    },
    InvalidSourceOrder {
        previous: RuleId<'ast, P>,
        next: RuleId<'ast, P>,
    },
    InvalidSourceEndpoints,
}

/// `R` and `D` keep rule and declaration payloads independent from storage.
pub struct RadixCompilation<'ast, R: Unpin, D, K> {
    allocator: &'ast Allocator,
    stylesheet: StyleSheet<'ast>,
    license_comments: crate::Vec<'ast, &'ast str>,
    rules: RuleStore<'ast, R>,
    rule_lists: RuleListStore<'ast, R>,
    declaration_blocks: DeclarationBlockStore<'ast, R>,
    declarations: DeclarationStore<'ast, D>,
    declaration_overflows: DeclarationOverflowStore<'ast>,
    effective_keys: EffectiveKeyStore<'ast, K>,
    effective_key_ids: FxHashMap<K, EffectiveKeyId<'ast>>,
    selector_values: DenseStore<'ast, SelectorValueDomain, SelectorValueRecord<'ast>>,
    selector_value_buckets: FxHashMap<u64, SmallVec<[SelectorValueId<'ast>; 1]>>,
    selector_paths: DenseStore<'ast, SelectorPathDomain, SelectorPathRecord<'ast>>,
    root_selector_paths: std::vec::Vec<Option<SelectorPathId<'ast>>>,
    selector_path_ids: FxHashMap<SelectorPathKey<'ast>, SelectorPathId<'ast>>,
    context_values: DenseStore<'ast, ContextValueDomain, ContextValueRecord<'ast, R>>,
    context_value_buckets: FxHashMap<u64, SmallVec<[ContextValueState<'ast, R>; 1]>>,
    context_paths: DenseStore<'ast, ContextPathDomain, ContextPathRecord<'ast>>,
    context_path_ids: FxHashMap<ContextPathKey<'ast>, ContextPathId<'ast>>,
    layer_contexts: DenseStore<'ast, LayerContextDomain, LayerContextRecord<'ast, R>>,
    layer_context_ids: FxHashMap<LayerContextKey<'ast, R>, LayerContextId<'ast>>,
    first_rule_in_source: Option<RuleId<'ast, R>>,
    last_rule_in_source: Option<RuleId<'ast, R>>,
}

impl<'ast, R: Unpin, D, K> RadixCompilation<'ast, R, D, K> {
    const SOURCE_ORDER_STRIDE: u64 = 1_u64 << 32;

    /// Creates an empty compilation with one root rule list.
    pub fn new_in(allocator: &'ast Allocator) -> Self {
        Self::with_capacity_in(allocator, CompilationCapacity::default())
    }

    /// Creates an empty compilation with capacity for the expected authored
    /// AST shape.
    pub fn with_capacity_in(allocator: &'ast Allocator, capacity: CompilationCapacity) -> Self {
        let mut rule_lists = RuleListStore::with_capacity_in(allocator, capacity.rule_lists.max(1));
        let root_rules = rule_lists.push(RuleList {
            parent: None,
            first: None,
            last: None,
            live_len: 0,
        });
        Self {
            allocator,
            stylesheet: StyleSheet { root_rules },
            license_comments: allocator.vec(),
            rules: RuleStore::with_capacity_in(allocator, capacity.rules),
            rule_lists,
            declaration_blocks: DeclarationBlockStore::with_capacity_in(
                allocator,
                capacity.declaration_blocks,
            ),
            declarations: DeclarationStore::with_capacity_in(allocator, capacity.declarations),
            declaration_overflows: DeclarationOverflowStore::new_in(allocator),
            effective_keys: EffectiveKeyStore::with_capacity_in(
                allocator,
                capacity.declaration_blocks,
            ),
            effective_key_ids: FxHashMap::with_capacity_and_hasher(
                capacity.declaration_blocks,
                Default::default(),
            ),
            selector_values: DenseStore::with_capacity_in(allocator, capacity.selectors),
            selector_value_buckets: FxHashMap::with_capacity_and_hasher(
                capacity.selectors,
                Default::default(),
            ),
            selector_paths: DenseStore::with_capacity_in(allocator, capacity.selectors),
            root_selector_paths: std::vec::Vec::with_capacity(capacity.selectors),
            selector_path_ids: FxHashMap::with_capacity_and_hasher(
                capacity.selectors,
                Default::default(),
            ),
            context_values: DenseStore::with_capacity_in(allocator, capacity.contexts),
            context_value_buckets: FxHashMap::with_capacity_and_hasher(
                capacity.contexts,
                Default::default(),
            ),
            context_paths: DenseStore::with_capacity_in(allocator, capacity.contexts),
            context_path_ids: FxHashMap::with_capacity_and_hasher(
                capacity.contexts,
                Default::default(),
            ),
            layer_contexts: DenseStore::with_capacity_in(allocator, capacity.contexts),
            layer_context_ids: FxHashMap::with_capacity_and_hasher(
                capacity.contexts,
                Default::default(),
            ),
            first_rule_in_source: None,
            last_rule_in_source: None,
        }
    }

    #[inline]
    pub const fn stylesheet(&self) -> StyleSheet<'ast> {
        self.stylesheet
    }

    #[inline]
    pub const fn allocator(&self) -> &'ast Allocator {
        self.allocator
    }

    #[inline]
    pub const fn first_rule_in_source(&self) -> Option<RuleId<'ast, R>> {
        self.first_rule_in_source
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
    pub fn rule(&self, id: RuleId<'ast, R>) -> Option<&RuleRecord<'ast, R>> {
        self.rules.try_get(id)
    }

    #[inline]
    pub fn rule_mut(&mut self, id: RuleId<'ast, R>) -> Option<&mut RuleRecord<'ast, R>> {
        self.rules.try_get_mut(id)
    }

    #[inline]
    pub fn rule_list(&self, id: RuleListId<'ast>) -> Option<&RuleList<'ast, R>> {
        self.rule_lists.try_get(id)
    }

    #[inline]
    pub fn declaration_block(
        &self,
        id: DeclarationBlockId<'ast, R>,
    ) -> Option<&DeclarationBlockRecord<'ast, R>> {
        self.declaration_blocks.try_get(id)
    }

    #[inline]
    pub fn declaration_block_mut(
        &mut self,
        id: DeclarationBlockId<'ast, R>,
    ) -> Option<&mut DeclarationBlockRecord<'ast, R>> {
        self.declaration_blocks.try_get_mut(id)
    }

    #[doc(hidden)]
    #[inline]
    pub fn declaration_block_count(&self) -> usize {
        self.declaration_blocks.len()
    }

    #[doc(hidden)]
    #[inline]
    pub fn rule_list_count(&self) -> usize {
        self.rule_lists.len()
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
    pub fn declaration(&self, id: DeclarationId<'ast>) -> Option<&DeclarationRecord<D>> {
        self.declarations.try_get(id)
    }

    #[inline]
    pub fn effective_key(&self, id: EffectiveKeyId<'ast>) -> Option<&K> {
        self.effective_keys.try_get(id)
    }

    /// Iterates authored and synthesized rules in global semantic order.
    #[inline]
    pub fn rules_in_source_order(&self) -> RuleSourceIter<'ast, '_, R> {
        RuleSourceIter {
            rules: &self.rules,
            next: self.first_rule_in_source,
            remaining: self.rules.len(),
        }
    }

    /// Iterates authored and synthesized blocks in global semantic order.
    #[inline]
    pub fn declaration_blocks_in_source_order(&self) -> DeclarationBlockSourceIter<'ast, '_, R> {
        DeclarationBlockSourceIter {
            rules: self.rules_in_source_order(),
            blocks: &self.declaration_blocks,
        }
    }

    /// Returns the semantic source-order label of a declaration block through
    /// its owning rule.
    #[inline]
    pub fn declaration_block_source_order_id(
        &self,
        block: DeclarationBlockId<'ast, R>,
    ) -> Option<SourceOrderId> {
        let DeclarationBlockOwner::Rule(owner) = self.declaration_blocks.try_get(block)?.owner;
        self.rules.try_get(owner).map(RuleRecord::source_order_id)
    }

    /// Iterates authored declarations in lexical source order.
    #[inline]
    pub fn declarations_in_source_order(
        &self,
    ) -> impl ExactSizeIterator<Item = (DeclarationId<'ast>, &DeclarationRecord<D>)> {
        self.declarations.iter_enumerated()
    }

    /// Iterates the declarations owned by `block` in semantic order.
    pub fn declarations_in_block(
        &self,
        block: DeclarationBlockId<'ast, R>,
    ) -> Result<DeclarationIter<'ast, '_, D>, MutationError<'ast, R>> {
        let declarations = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?
            .declarations;
        let kind = match declarations {
            DeclarationList::Range(range) => {
                let start = range.start as usize;
                let end = start + range.len as usize;
                let records = self
                    .declarations
                    .as_slice()
                    .get(start..end)
                    .ok_or(MutationError::NonContiguousDeclarationRange(block))?;
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
                    .ok_or(MutationError::UnknownDeclarationOverflow(overflow))?;
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
        block: DeclarationBlockId<'ast, R>,
    ) -> Result<DeclarationOccurrenceIter<'ast, '_, D>, MutationError<'ast, R>> {
        let declarations = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?
            .declarations;
        let kind = match declarations {
            DeclarationList::Range(range) => {
                let start = range.start as usize;
                let end = start + range.len as usize;
                let records = self
                    .declarations
                    .as_slice()
                    .get(start..end)
                    .ok_or(MutationError::NonContiguousDeclarationRange(block))?;
                DeclarationOccurrenceIterKind::Range {
                    ids: self
                        .declarations
                        .ids_in_range(start, range.len as usize)
                        .ok_or(MutationError::NonContiguousDeclarationRange(block))?,
                    records: records.iter(),
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
                    .ok_or(MutationError::UnknownDeclarationOverflow(overflow))?;
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
        block: DeclarationBlockId<'ast, R>,
    ) -> Result<DeclarationIdIter<'ast, '_>, MutationError<'ast, R>> {
        let block_record = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?;
        let kind = match block_record.declarations {
            DeclarationList::Range(range) => DeclarationIdIterKind::Range(
                self.declarations
                    .ids_in_range(range.start as usize, range.len as usize)
                    .ok_or(MutationError::NonContiguousDeclarationRange(block))?,
            ),
            DeclarationList::Local4(local) => DeclarationIdIterKind::Local4 { local, index: 0 },
            DeclarationList::Overflow(overflow) => DeclarationIdIterKind::Overflow(
                self.declaration_overflows
                    .try_get(overflow)
                    .ok_or(MutationError::UnknownDeclarationOverflow(overflow))?
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
        block: DeclarationBlockId<'ast, R>,
        mut visit: impl FnMut(DeclarationId<'ast>, &mut DeclarationRecord<D>),
    ) -> Result<usize, MutationError<'ast, R>> {
        let declaration_list = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?
            .declarations;
        if !self
            .declaration_blocks
            .try_get(block)
            .is_some_and(|record| record.live)
        {
            return Err(MutationError::UnknownDeclarationBlock(block));
        }

        let mut visited = 0usize;
        match declaration_list {
            DeclarationList::Range(range) => {
                let start = range.start as usize;
                let records = self
                    .declarations
                    .iter_enumerated_range_mut(start, range.len as usize)
                    .ok_or(MutationError::NonContiguousDeclarationRange(block))?;
                for (declaration, record) in records {
                    visit(declaration, record);
                    visited += 1;
                }
            }
            DeclarationList::Local4(local) => {
                for declaration in local.iter() {
                    let record = self
                        .declarations
                        .try_get_mut(declaration)
                        .ok_or(MutationError::UnknownDeclaration(declaration))?;
                    visit(declaration, record);
                    visited += 1;
                }
            }
            DeclarationList::Overflow(overflow) => {
                let ids = self
                    .declaration_overflows
                    .try_get(overflow)
                    .ok_or(MutationError::UnknownDeclarationOverflow(overflow))?;
                // The overflow ID tape and the declaration store are
                // independent fields, so the ID slice can stay borrowed while
                // the corresponding declaration records are mutated.
                for &declaration in ids.iter() {
                    let record = self
                        .declarations
                        .try_get_mut(declaration)
                        .ok_or(MutationError::UnknownDeclaration(declaration))?;
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
                .try_get_mut(block)
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
        block: DeclarationBlockId<'ast, R>,
        index: usize,
    ) -> Result<DeclarationId<'ast>, MutationError<'ast, R>> {
        let declaration_list = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?
            .declarations;
        match declaration_list {
            DeclarationList::Range(range) => {
                if index >= range.len as usize {
                    return Err(MutationError::NonContiguousDeclarationRange(block));
                }
                self.declarations
                    .id_at_offset(range.start as usize, index)
                    .ok_or(MutationError::NonContiguousDeclarationRange(block))
            }
            DeclarationList::Local4(local) => local
                .iter()
                .nth(index)
                .ok_or(MutationError::NonContiguousDeclarationRange(block)),
            DeclarationList::Overflow(overflow) => self
                .declaration_overflows
                .try_get(overflow)
                .ok_or(MutationError::UnknownDeclarationOverflow(overflow))?
                .get(index)
                .copied()
                .ok_or(MutationError::NonContiguousDeclarationRange(block)),
        }
    }

    /// Iterates direct siblings in one rule list without walking descendants.
    pub fn rules_in_list(
        &self,
        list: RuleListId<'ast>,
    ) -> Result<RuleListIter<'ast, '_, R>, MutationError<'ast, R>> {
        let list = self
            .rule_lists
            .try_get(list)
            .ok_or(MutationError::UnknownRuleList(list))?;
        Ok(RuleListIter {
            rules: &self.rules,
            next: list.first,
            remaining: list.live_len,
        })
    }

    /// Appends one authored rule to `list` in lexical parse order.
    ///
    /// All fallible checks happen before either the store or topology changes.
    pub fn append_rule(
        &mut self,
        list: RuleListId<'ast>,
        payload: R,
    ) -> Result<RuleId<'ast, R>, MutationError<'ast, R>> {
        let (parent, previous_sibling) = self
            .rule_lists
            .try_get(list)
            .map(|list| (list.parent, list.last))
            .ok_or(MutationError::UnknownRuleList(list))?;
        if !self.rules.has_capacity_for(1) {
            return Err(MutationError::RuleCapacityExhausted);
        }
        let source_order_id = self.source_order_id_between(self.last_rule_in_source, None)?;
        let id = self
            .rules
            .try_push(RuleRecord {
                payload,
                source_order_id,
                parent,
                parent_list: list,
                previous_sibling,
                next_sibling: None,
                previous_in_source: self.last_rule_in_source,
                next_in_source: None,
                child_list: None,
                declaration_block: None,
                revision: 0,
                live: true,
            })
            .map_err(|_| MutationError::RuleCapacityExhausted)?;
        if let Some(previous) = self.last_rule_in_source {
            self.rules
                .try_get_mut(previous)
                .expect("the global source tail remains resolvable")
                .next_in_source = Some(id);
        } else {
            self.first_rule_in_source = Some(id);
        }
        self.last_rule_in_source = Some(id);
        if let Some(previous) = previous_sibling {
            self.rules
                .try_get_mut(previous)
                .expect("the list tail was validated when it was published")
                .next_sibling = Some(id);
        }
        let list_record = self.rule_lists.get_mut(list);
        list_record.first.get_or_insert(id);
        list_record.last = Some(id);
        list_record.live_len += 1;
        Ok(id)
    }

    fn source_order_id_between(
        &mut self,
        previous: Option<RuleId<'ast, R>>,
        next: Option<RuleId<'ast, R>>,
    ) -> Result<SourceOrderId, MutationError<'ast, R>> {
        if let Some(id) = self.source_order_id_between_without_relabel(previous, next)? {
            return Ok(id);
        }
        self.relabel_source_order_ids()?;
        self.source_order_id_between_without_relabel(previous, next)?
            .ok_or_else(|| {
                MutationError::InvalidRuleTopology(
                    previous
                        .or(next)
                        .expect("a nonempty source chain has an insertion neighbor"),
                )
            })
    }

    fn source_order_id_between_without_relabel(
        &self,
        previous: Option<RuleId<'ast, R>>,
        next: Option<RuleId<'ast, R>>,
    ) -> Result<Option<SourceOrderId>, MutationError<'ast, R>> {
        let previous_order = previous
            .map(|id| {
                self.rules
                    .try_get(id)
                    .map(RuleRecord::source_order_id)
                    .ok_or(MutationError::InvalidRuleTopology(id))
            })
            .transpose()?;
        let next_order = next
            .map(|id| {
                self.rules
                    .try_get(id)
                    .map(RuleRecord::source_order_id)
                    .ok_or(MutationError::InvalidRuleTopology(id))
            })
            .transpose()?;
        let order =
            match (previous_order, next_order) {
                (None, None) => Some(Self::SOURCE_ORDER_STRIDE),
                (Some(previous), None) => previous
                    .0
                    .checked_add(Self::SOURCE_ORDER_STRIDE)
                    .or_else(|| {
                        let gap = u64::MAX - previous.0;
                        (gap > 1).then_some(previous.0 + gap / 2)
                    }),
                (None, Some(next)) => (next.0 > 1).then_some(next.0 / 2),
                (Some(previous), Some(next)) if previous < next && previous.0 + 1 < next.0 => {
                    Some(previous.0 + (next.0 - previous.0) / 2)
                }
                (Some(previous), Some(next)) if previous < next => None,
                (Some(_), Some(_)) => {
                    return Err(MutationError::InvalidRuleTopology(
                        previous.expect("the invalid pair has a previous rule"),
                    ));
                }
            };
        Ok(order.map(SourceOrderId))
    }

    fn relabel_source_order_ids(&mut self) -> Result<(), MutationError<'ast, R>> {
        let step = u64::MAX / (self.rules.len() as u64 + 1);
        let mut current = self.first_rule_in_source;
        let mut ordinal = 1_u64;
        while let Some(id) = current {
            let next = self
                .rules
                .try_get(id)
                .ok_or(MutationError::InvalidRuleTopology(id))?
                .next_in_source;
            self.rules
                .try_get_mut(id)
                .expect("the source rule was resolved above")
                .source_order_id = SourceOrderId(step * ordinal);
            ordinal += 1;
            current = next;
        }
        if ordinal - 1 != self.rules.len() as u64 {
            return Err(MutationError::InvalidSourceTopology);
        }
        Ok(())
    }

    /// Creates the one direct child list owned by `parent`.
    pub fn create_child_list(
        &mut self,
        parent: RuleId<'ast, R>,
    ) -> Result<RuleListId<'ast>, MutationError<'ast, R>> {
        let parent_record = self
            .rules
            .try_get(parent)
            .ok_or(MutationError::UnknownRule(parent))?;
        if !parent_record.live {
            return Err(MutationError::RetiredRule(parent));
        }
        if parent_record.child_list.is_some() {
            return Err(MutationError::ChildListAlreadyExists(parent));
        }

        let list = self
            .rule_lists
            .try_push(RuleList {
                parent: Some(parent),
                first: None,
                last: None,
                live_len: 0,
            })
            .map_err(|_| MutationError::RuleListCapacityExhausted)?;
        self.rules
            .try_get_mut(parent)
            .expect("the parent was validated before allocating its child list")
            .child_list = Some(list);
        Ok(list)
    }

    /// Appends a key record. W6 routes this operation through exact interning.
    pub fn append_effective_key(
        &mut self,
        key: K,
    ) -> Result<EffectiveKeyId<'ast>, MutationError<'ast, R>>
    where
        K: Copy + Eq + Hash,
    {
        if let Some(&id) = self.effective_key_ids.get(&key) {
            return Ok(id);
        }
        let id = self
            .effective_keys
            .try_push(key)
            .map_err(|_| MutationError::EffectiveKeyCapacityExhausted)?;
        self.effective_key_ids.insert(key, id);
        Ok(id)
    }

    /// Appends one authored declaration block and binds its unique owner.
    pub fn append_declaration_block(
        &mut self,
        owner: DeclarationBlockOwner<'ast, R>,
        effective_key: EffectiveKeyId<'ast>,
    ) -> Result<DeclarationBlockId<'ast, R>, MutationError<'ast, R>> {
        let DeclarationBlockOwner::Rule(owner_rule) = owner;
        let owner_record = self
            .rules
            .try_get(owner_rule)
            .ok_or(MutationError::UnknownRule(owner_rule))?;
        if !owner_record.live {
            return Err(MutationError::RetiredRule(owner_rule));
        }
        if owner_record.declaration_block.is_some() {
            return Err(MutationError::DeclarationBlockAlreadyExists(owner_rule));
        }
        if self.effective_keys.try_get(effective_key).is_none() {
            return Err(MutationError::UnknownEffectiveKey(effective_key));
        }
        let block = self
            .declaration_blocks
            .try_push(DeclarationBlockRecord {
                declarations: DeclarationList::Range(DeclarationRange {
                    start: self.declarations.len() as u32,
                    len: 0,
                }),
                owner,
                effective_key,
                revision: 0,
                live: true,
            })
            .map_err(|_| MutationError::DeclarationBlockCapacityExhausted)?;
        self.rules
            .try_get_mut(owner_rule)
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
        block: DeclarationBlockId<'ast, R>,
        payload: D,
        important: bool,
    ) -> Result<DeclarationId<'ast>, MutationError<'ast, R>> {
        let range = self
            .declaration_blocks
            .try_get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?
            .declarations
            .as_range()
            .ok_or(MutationError::NonContiguousDeclarationRange(block))?;
        if range.start as usize + range.len as usize != self.declarations.len() {
            return Err(MutationError::NonContiguousDeclarationRange(block));
        }
        let declaration = self
            .declarations
            .try_push(DeclarationRecord { payload, important })
            .map_err(|_| MutationError::DeclarationCapacityExhausted)?;
        let block = self
            .declaration_blocks
            .try_get_mut(block)
            .expect("the block was validated before appending its declaration");
        let DeclarationList::Range(range) = &mut block.declarations else {
            unreachable!("the declaration representation was validated before append")
        };
        range.len += 1;
        Ok(declaration)
    }
}

/// Ordered declaration identities resolved from one block representation.
pub struct DeclarationIdIter<'ast, 'comp> {
    kind: DeclarationIdIterKind<'ast, 'comp>,
}

enum DeclarationIdIterKind<'ast, 'comp> {
    Range(DenseIdRange<'ast, DeclarationDomain>),
    Local4 {
        local: LocalPropertySet<'ast>,
        index: u8,
    },
    Overflow(std::slice::Iter<'comp, DeclarationId<'ast>>),
}

impl<'ast> Iterator for DeclarationIdIter<'ast, '_> {
    type Item = DeclarationId<'ast>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            DeclarationIdIterKind::Range(range) => range.next(),
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

impl ExactSizeIterator for DeclarationIdIter<'_, '_> {}

/// Ordered declaration payloads resolved from one block representation.
pub struct DeclarationIter<'ast, 'comp, D> {
    kind: DeclarationIterKind<'ast, 'comp, D>,
}

enum DeclarationIterKind<'ast, 'comp, D> {
    Range(std::slice::Iter<'comp, DeclarationRecord<D>>),
    Indirect {
        ids: DeclarationIdIter<'ast, 'comp>,
        declarations: &'comp DeclarationStore<'ast, D>,
    },
}

impl<'ast, 'comp, D> Iterator for DeclarationIter<'ast, 'comp, D> {
    type Item = &'comp DeclarationRecord<D>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            DeclarationIterKind::Range(records) => records.next(),
            DeclarationIterKind::Indirect { ids, declarations } => Some(&declarations[ids.next()?]),
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

impl<D> ExactSizeIterator for DeclarationIter<'_, '_, D> {}

/// Ordered declaration identities and payloads resolved from one block.
pub struct DeclarationOccurrenceIter<'ast, 'comp, D> {
    kind: DeclarationOccurrenceIterKind<'ast, 'comp, D>,
}

enum DeclarationOccurrenceIterKind<'ast, 'comp, D> {
    Range {
        ids: DenseIdRange<'ast, DeclarationDomain>,
        records: std::slice::Iter<'comp, DeclarationRecord<D>>,
    },
    Indirect {
        ids: DeclarationIdIter<'ast, 'comp>,
        declarations: &'comp DeclarationStore<'ast, D>,
    },
}

impl<'ast, 'comp, D> Iterator for DeclarationOccurrenceIter<'ast, 'comp, D> {
    type Item = (DeclarationId<'ast>, &'comp DeclarationRecord<D>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            DeclarationOccurrenceIterKind::Range { ids, records } => {
                let record = records.next()?;
                let id = ids.next()?;
                Some((id, record))
            }
            DeclarationOccurrenceIterKind::Indirect { ids, declarations } => {
                let id = ids.next()?;
                Some((id, &declarations[id]))
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

impl<D> ExactSizeIterator for DeclarationOccurrenceIter<'_, '_, D> {}

/// Global source-order iterator backed by the explicit source topology.
pub struct RuleSourceIter<'ast, 'comp, R: Unpin> {
    rules: &'comp RuleStore<'ast, R>,
    next: Option<RuleId<'ast, R>>,
    remaining: usize,
}

impl<'ast, 'comp, R: Unpin> Iterator for RuleSourceIter<'ast, 'comp, R> {
    type Item = (RuleId<'ast, R>, &'comp RuleRecord<'ast, R>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let id = self.next?;
        let rule = self.rules.try_get(id)?;
        self.next = rule.next_in_source;
        self.remaining -= 1;
        Some((id, rule))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.remaining))
    }
}

/// Declaration blocks projected from the global rule source topology.
pub struct DeclarationBlockSourceIter<'ast, 'comp, R: Unpin> {
    rules: RuleSourceIter<'ast, 'comp, R>,
    blocks: &'comp DeclarationBlockStore<'ast, R>,
}

impl<'ast, 'comp, R: Unpin> Iterator for DeclarationBlockSourceIter<'ast, 'comp, R> {
    type Item = (
        DeclarationBlockId<'ast, R>,
        &'comp DeclarationBlockRecord<'ast, R>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        for (_, rule) in self.rules.by_ref() {
            let Some(id) = rule.declaration_block else {
                continue;
            };
            return self.blocks.try_get(id).map(|block| (id, block));
        }
        None
    }
}

/// Direct-sibling iterator backed only by the rule store and topology links.
pub struct RuleListIter<'ast, 'comp, R: Unpin> {
    rules: &'comp RuleStore<'ast, R>,
    next: Option<RuleId<'ast, R>>,
    remaining: u32,
}

impl<'ast, 'comp, R: Unpin> Iterator for RuleListIter<'ast, 'comp, R> {
    type Item = (RuleId<'ast, R>, &'comp RuleRecord<'ast, R>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let id = self.next?;
        let rule = self.rules.try_get(id)?;
        self.next = rule.next_sibling;
        self.remaining -= 1;
        Some((id, rule))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining as usize;
        (remaining, Some(remaining))
    }
}

impl<R: Unpin> ExactSizeIterator for RuleListIter<'_, '_, R> {}

#[cfg(test)]
mod tests;
