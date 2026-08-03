//! Typed storage and topology for the compiler's persistent Radix AST.

use rocketcss_common::{
    Allocator, DenseId, DenseStore, RadixIdRemap, RadixInsertResult, TypedRadixIndexArena,
    define_dense_id, define_radix_id,
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

define_radix_id!(pub struct RuleId);
define_radix_id!(pub struct DeclarationBlockId);
define_dense_id!(pub struct RuleListId);
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
pub trait RuleIdReferences {
    fn remap_rule_ids(&mut self, remaps: &[RadixIdRemap<RuleId>]);
}

macro_rules! impl_no_rule_id_references {
    ($($type:ty),+ $(,)?) => {
        $(
            impl RuleIdReferences for $type {
                #[inline]
                fn remap_rule_ids(&mut self, _remaps: &[RadixIdRemap<RuleId>]) {}
            }
        )+
    };
}

impl_no_rule_id_references!((), u8, &str);

/// Rules in lexical allocation order plus rare locally inserted siblings.
pub type RuleStore<'ast, P> = TypedRadixIndexArena<'ast, RuleRecord<P>, RuleId>;

/// Declaration blocks in lexical allocation order plus synthesized blocks.
pub type DeclarationBlockStore<'ast> =
    TypedRadixIndexArena<'ast, DeclarationBlockRecord, DeclarationBlockId>;

/// Dense rule-list metadata. Lists own topology, not a second rule vector.
pub type RuleListStore = DenseStore<RuleListId, RuleList>;

/// Interned effective-key records shared by declaration blocks.
pub type EffectiveKeyStore<P> = DenseStore<EffectiveKeyId, P>;

/// Authored declarations in lexical source order.
pub type DeclarationStore<P> = DenseStore<DeclarationId, DeclarationRecord<P>>;

/// Arena-backed complete declaration sequences used when a block no longer
/// maps to one contiguous authored range.
pub type DeclarationOverflowStore<'ast> =
    DenseStore<DeclarationOverflowId, crate::Vec<'ast, DeclarationId>>;

/// Concrete compiler-owned Radix AST.
pub type Compilation<'ast> =
    RadixCompilation<'ast, CssRulePayload<'ast>, DeclarationPayload<'ast>, EffectiveKeyData>;

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

/// The lightweight root of a Radix compilation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StyleSheet {
    root_rules: RuleListId,
}

impl StyleSheet {
    #[inline]
    pub const fn root_rules(self) -> RuleListId {
        self.root_rules
    }
}

/// Direct topology for one authored or synthesized CSS rule.
#[derive(Debug, PartialEq, Eq)]
pub struct RuleRecord<P> {
    payload: P,
    parent: Option<RuleId>,
    parent_list: RuleListId,
    previous_sibling: Option<RuleId>,
    next_sibling: Option<RuleId>,
    previous_in_source: Option<RuleId>,
    next_in_source: Option<RuleId>,
    child_list: Option<RuleListId>,
    declaration_block: Option<DeclarationBlockId>,
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
    pub const fn parent(&self) -> Option<RuleId> {
        self.parent
    }

    #[inline]
    pub const fn parent_list(&self) -> RuleListId {
        self.parent_list
    }

    #[inline]
    pub const fn previous_sibling(&self) -> Option<RuleId> {
        self.previous_sibling
    }

    #[inline]
    pub const fn next_sibling(&self) -> Option<RuleId> {
        self.next_sibling
    }

    #[inline]
    pub const fn previous_in_source(&self) -> Option<RuleId> {
        self.previous_in_source
    }

    #[inline]
    pub const fn next_in_source(&self) -> Option<RuleId> {
        self.next_in_source
    }

    #[inline]
    pub const fn child_list(&self) -> Option<RuleListId> {
        self.child_list
    }

    #[inline]
    pub const fn declaration_block(&self) -> Option<DeclarationBlockId> {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuleList {
    parent: Option<RuleId>,
    first: Option<RuleId>,
    last: Option<RuleId>,
    live_len: u32,
}

impl RuleList {
    #[inline]
    pub const fn parent(self) -> Option<RuleId> {
        self.parent
    }

    #[inline]
    pub const fn first(self) -> Option<RuleId> {
        self.first
    }

    #[inline]
    pub const fn last(self) -> Option<RuleId> {
        self.last
    }

    #[inline]
    pub const fn live_len(self) -> u32 {
        self.live_len
    }
}

/// The unique syntax owner of a declaration block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclarationBlockOwner {
    Rule(RuleId),
}

/// A declaration-list handle plus its persistent AST identity.
#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationBlockRecord {
    declarations: DeclarationList,
    owner: DeclarationBlockOwner,
    effective_key: EffectiveKeyId,
    revision: u32,
    live: bool,
}

impl DeclarationBlockRecord {
    #[inline]
    pub const fn declarations(&self) -> DeclarationList {
        self.declarations
    }

    #[inline]
    pub const fn owner(&self) -> DeclarationBlockOwner {
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
pub enum MutationError {
    PrimaryRuleCapacityExhausted,
    PrimaryDeclarationBlockCapacityExhausted,
    RuleListCapacityExhausted,
    EffectiveKeyCapacityExhausted,
    SelectorContextCapacityExhausted,
    DeclarationCapacityExhausted,
    DeclarationOverflowCapacityExhausted,
    UnknownRule(RuleId),
    UnknownRuleList(RuleListId),
    UnknownEffectiveKey(EffectiveKeyId),
    RetiredRule(RuleId),
    ChildListAlreadyExists(RuleId),
    DeclarationBlockAlreadyExists(RuleId),
    UnknownDeclarationBlock(DeclarationBlockId),
    UnknownDeclarationOverflow(DeclarationOverflowId),
    UnknownDeclaration(DeclarationId),
    NonContiguousDeclarationRange(DeclarationBlockId),
    LocalRuleCapacityExhausted(RuleId),
    LocalDeclarationBlockCapacityExhausted(DeclarationBlockId),
    InvalidRuleTopology(RuleId),
    RuleHasChildren(RuleId),
}

/// Live topology exposed by retiring one leaf rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetiredRule {
    pub id: RuleId,
    pub list: RuleListId,
    pub previous: Option<RuleId>,
    pub next: Option<RuleId>,
    pub declaration_block: Option<DeclarationBlockId>,
}

/// Result of folding one direct left sibling into the retained right rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MergedAdjacentRuleBlocks {
    pub retired_rule: RuleId,
    pub retired_block: DeclarationBlockId,
    pub retained_rule: RuleId,
    pub retained_block: DeclarationBlockId,
    pub effective_key: EffectiveKeyId,
}

/// A violated store, ownership, or direct-topology invariant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationError {
    MissingRootRuleList(RuleListId),
    RootRuleListHasParent(RuleId),
    MissingListParent {
        list: RuleListId,
        parent: RuleId,
    },
    RetiredListParent {
        list: RuleListId,
        parent: RuleId,
    },
    ParentDoesNotOwnList {
        list: RuleListId,
        parent: RuleId,
    },
    InvalidListEndpoints(RuleListId),
    MissingRule(RuleId),
    RetiredRuleInList {
        list: RuleListId,
        rule: RuleId,
    },
    RuleHasWrongParentList {
        list: RuleListId,
        rule: RuleId,
    },
    RuleHasWrongParent {
        list: RuleListId,
        rule: RuleId,
    },
    RuleHasWrongPrevious {
        rule: RuleId,
        expected: Option<RuleId>,
    },
    ListDoesNotEndAtLast(RuleListId),
    ListLengthMismatch {
        list: RuleListId,
        expected: u32,
        actual: u32,
    },
    LiveRuleIsNotInOneList(RuleId),
    MissingOwnedChildList {
        rule: RuleId,
        list: RuleListId,
    },
    ChildListHasWrongParent {
        rule: RuleId,
        list: RuleListId,
        actual: Option<RuleId>,
    },
    MissingOwnedDeclarationBlock {
        rule: RuleId,
        block: DeclarationBlockId,
    },
    DeclarationBlockHasWrongOwner {
        rule: RuleId,
        block: DeclarationBlockId,
        actual: RuleId,
    },
    MissingBlockOwner {
        block: DeclarationBlockId,
        owner: RuleId,
    },
    RetiredBlockOwner {
        block: DeclarationBlockId,
        owner: RuleId,
    },
    OwnerDoesNotReferenceBlock {
        block: DeclarationBlockId,
        owner: RuleId,
        actual: Option<DeclarationBlockId>,
    },
    MissingEffectiveKey {
        block: DeclarationBlockId,
        key: EffectiveKeyId,
    },
    InvalidDeclarationRange {
        block: DeclarationBlockId,
        range: DeclarationRange,
    },
    InvalidDeclarationOverflow {
        block: DeclarationBlockId,
        overflow: DeclarationOverflowId,
    },
    InvalidDeclarationReference {
        block: DeclarationBlockId,
        declaration: DeclarationId,
    },
    DuplicateDeclarationOwner {
        declaration: DeclarationId,
        first: DeclarationBlockId,
        second: DeclarationBlockId,
    },
    DeclarationRangeStartsOutOfOrder {
        block: DeclarationBlockId,
        expected: u32,
        actual: u32,
    },
    UnownedDeclarations {
        expected: u32,
        actual: u32,
    },
    InvalidSourcePrevious {
        rule: RuleId,
        expected: Option<RuleId>,
        actual: Option<RuleId>,
    },
    InvalidSourceNext {
        rule: RuleId,
        expected: Option<RuleId>,
        actual: Option<RuleId>,
    },
    InvalidSourceEndpoints,
}

/// `R` and `D` keep rule and declaration payloads independent from storage.
pub struct RadixCompilation<'ast, R: Unpin, D, K> {
    allocator: &'ast Allocator,
    stylesheet: StyleSheet,
    license_comments: crate::Vec<'ast, &'ast str>,
    rules: RuleStore<'ast, R>,
    rule_lists: RuleListStore,
    declaration_blocks: DeclarationBlockStore<'ast>,
    declarations: DeclarationStore<D>,
    declaration_overflows: DeclarationOverflowStore<'ast>,
    effective_keys: EffectiveKeyStore<K>,
    effective_key_ids: FxHashMap<K, EffectiveKeyId>,
    selector_values: DenseStore<SelectorValueId, SelectorValueRecord<'ast>>,
    selector_value_buckets: FxHashMap<u64, SmallVec<[SelectorValueId; 1]>>,
    selector_paths: DenseStore<SelectorPathId, SelectorPathRecord>,
    root_selector_paths: std::vec::Vec<Option<SelectorPathId>>,
    selector_path_ids: FxHashMap<SelectorPathKey, SelectorPathId>,
    context_values: DenseStore<ContextValueId, ContextValueRecord>,
    context_value_buckets: FxHashMap<u64, SmallVec<[ContextValueState; 1]>>,
    context_paths: DenseStore<ContextPathId, ContextPathRecord>,
    context_path_ids: FxHashMap<ContextPathKey, ContextPathId>,
    layer_contexts: DenseStore<LayerContextId, LayerContextRecord>,
    layer_context_ids: FxHashMap<LayerContextKey, LayerContextId>,
    first_rule_in_source: Option<RuleId>,
    last_rule_in_source: Option<RuleId>,
}

impl<'ast, R: Unpin, D, K> RadixCompilation<'ast, R, D, K> {
    /// Creates an empty compilation with one root rule list.
    pub fn new_in(allocator: &'ast Allocator) -> Self {
        Self::with_capacity_in(allocator, CompilationCapacity::default())
    }

    /// Creates an empty compilation with capacity for the expected authored
    /// AST shape.
    pub fn with_capacity_in(allocator: &'ast Allocator, capacity: CompilationCapacity) -> Self {
        let mut rule_lists = RuleListStore::with_capacity(capacity.rule_lists.max(1));
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
            rules: RuleStore::with_capacity_in(capacity.rules, allocator),
            rule_lists,
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
            first_rule_in_source: None,
            last_rule_in_source: None,
        }
    }

    #[inline]
    pub const fn stylesheet(&self) -> StyleSheet {
        self.stylesheet
    }

    #[inline]
    pub const fn allocator(&self) -> &'ast Allocator {
        self.allocator
    }

    #[inline]
    pub const fn first_rule_in_source(&self) -> Option<RuleId> {
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
    pub fn rule(&self, id: RuleId) -> Option<&RuleRecord<R>> {
        self.rules.get(id)
    }

    #[inline]
    pub fn rule_mut(&mut self, id: RuleId) -> Option<&mut RuleRecord<R>> {
        self.rules.get_mut(id)
    }

    #[inline]
    pub fn rule_list(&self, id: RuleListId) -> Option<&RuleList> {
        self.rule_lists.try_get(id)
    }

    #[inline]
    pub fn declaration_block(&self, id: DeclarationBlockId) -> Option<&DeclarationBlockRecord> {
        self.declaration_blocks.get(id)
    }

    #[inline]
    pub fn declaration_block_mut(
        &mut self,
        id: DeclarationBlockId,
    ) -> Option<&mut DeclarationBlockRecord> {
        self.declaration_blocks.get_mut(id)
    }

    #[inline]
    pub fn declaration(&self, id: DeclarationId) -> Option<&DeclarationRecord<D>> {
        self.declarations.try_get(id)
    }

    #[inline]
    pub fn effective_key(&self, id: EffectiveKeyId) -> Option<&K> {
        self.effective_keys.try_get(id)
    }

    /// Iterates authored and synthesized rules in global semantic order.
    #[inline]
    pub fn rules_in_source_order(&self) -> impl Iterator<Item = (RuleId, &RuleRecord<R>)> {
        self.rules.iter_enumerated()
    }

    /// Iterates authored and synthesized blocks in global semantic order.
    #[inline]
    pub fn declaration_blocks_in_source_order(
        &self,
    ) -> impl Iterator<Item = (DeclarationBlockId, &DeclarationBlockRecord)> {
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
        block: DeclarationBlockId,
    ) -> Result<DeclarationIter<'_, D>, MutationError> {
        let declarations = self
            .declaration_blocks
            .get(block)
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
        block: DeclarationBlockId,
    ) -> Result<DeclarationOccurrenceIter<'_, D>, MutationError> {
        let declarations = self
            .declaration_blocks
            .get(block)
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
        block: DeclarationBlockId,
    ) -> Result<DeclarationIdIter<'_>, MutationError> {
        let block = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?;
        let kind = match block.declarations {
            DeclarationList::Range(range) => DeclarationIdIterKind::Range(
                range.start as usize..range.start as usize + range.len as usize,
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

    /// Iterates direct siblings in one rule list without walking descendants.
    pub fn rules_in_list(
        &self,
        list: RuleListId,
    ) -> Result<RuleListIter<'_, 'ast, R>, MutationError> {
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
    pub fn append_rule(&mut self, list: RuleListId, payload: R) -> Result<RuleId, MutationError> {
        let (parent, previous_sibling) = self
            .rule_lists
            .try_get(list)
            .map(|list| (list.parent, list.last))
            .ok_or(MutationError::UnknownRuleList(list))?;
        if !self.rules.can_push_primary() {
            return Err(MutationError::PrimaryRuleCapacityExhausted);
        }

        let id = self.rules.push_primary(RuleRecord {
            payload,
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
        });
        if let Some(previous) = self.last_rule_in_source {
            self.rules
                .get_mut(previous)
                .expect("the global source tail remains resolvable")
                .next_in_source = Some(id);
        } else {
            self.first_rule_in_source = Some(id);
        }
        self.last_rule_in_source = Some(id);
        if let Some(previous) = previous_sibling {
            self.rules
                .get_mut(previous)
                .expect("the list tail was validated when it was published")
                .next_sibling = Some(id);
        }
        let list_record = self.rule_lists.get_mut(list);
        list_record.first.get_or_insert(id);
        list_record.last = Some(id);
        list_record.live_len += 1;
        Ok(id)
    }

    /// Creates the one direct child list owned by `parent`.
    pub fn create_child_list(&mut self, parent: RuleId) -> Result<RuleListId, MutationError> {
        let parent_record = self
            .rules
            .get(parent)
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
            .get_mut(parent)
            .expect("the parent was validated before allocating its child list")
            .child_list = Some(list);
        Ok(list)
    }

    /// Appends a key record. W6 routes this operation through exact interning.
    pub fn append_effective_key(&mut self, key: K) -> Result<EffectiveKeyId, MutationError>
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
        owner: DeclarationBlockOwner,
        effective_key: EffectiveKeyId,
    ) -> Result<DeclarationBlockId, MutationError> {
        let DeclarationBlockOwner::Rule(owner_rule) = owner;
        let owner_record = self
            .rules
            .get(owner_rule)
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
        if !self.declaration_blocks.can_push_primary() {
            return Err(MutationError::PrimaryDeclarationBlockCapacityExhausted);
        }
        let block = self
            .declaration_blocks
            .push_primary(DeclarationBlockRecord {
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
        block: DeclarationBlockId,
        payload: D,
        important: bool,
    ) -> Result<DeclarationId, MutationError> {
        let range = self
            .declaration_blocks
            .get(block)
            .ok_or(MutationError::UnknownDeclarationBlock(block))?
            .declarations
            .as_range()
            .ok_or(MutationError::NonContiguousDeclarationRange(block))?;
        let next = self
            .declarations
            .try_next_id()
            .map_err(|_| MutationError::DeclarationCapacityExhausted)?;
        if range.start as usize + range.len as usize != next.index() {
            return Err(MutationError::NonContiguousDeclarationRange(block));
        }
        let declaration = self
            .declarations
            .try_push(DeclarationRecord { payload, important })
            .map_err(|_| MutationError::DeclarationCapacityExhausted)?;
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

/// Direct-sibling iterator backed only by the rule store and topology links.
pub struct RuleListIter<'comp, 'ast, R: Unpin> {
    rules: &'comp RuleStore<'ast, R>,
    next: Option<RuleId>,
    remaining: u32,
}

impl<'comp, 'ast, R: Unpin> Iterator for RuleListIter<'comp, 'ast, R> {
    type Item = (RuleId, &'comp RuleRecord<R>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let id = self.next?;
        let rule = self.rules.get(id)?;
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
