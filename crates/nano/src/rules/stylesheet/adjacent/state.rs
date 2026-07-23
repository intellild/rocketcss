use std::{collections::VecDeque, hash::Hasher};

use rocketcss_allocator::Ref;
use rocketcss_ast::{DeclarationBlock, VendorPrefix};

macro_rules! dense_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(super) struct $name(pub(super) u32);

        impl $name {
            pub(super) fn index(self) -> usize {
                self.0 as usize
            }
        }
    };
}

dense_id!(RuleId);
dense_id!(SequenceId);
dense_id!(HistoryId);
dense_id!(EdgeId);
dense_id!(DeclarationEntryId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct SegmentId(pub(super) u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct CascadeScope {
    pub(super) layer: Option<LayerContextId>,
    pub(super) origin: CascadeOrigin,
}

impl CascadeScope {
    pub(super) const AUTHOR: Self = Self {
        layer: None,
        origin: CascadeOrigin::AUTHOR,
    };

    pub(super) fn in_layer(self, layer: LayerContextId) -> Self {
        Self {
            layer: Some(layer),
            ..self
        }
    }

    pub(super) fn declaration_context(self, important: bool) -> DeclarationHistoryContext {
        DeclarationHistoryContext {
            layer: self.layer,
            origin: self.origin,
            phase: if important {
                CascadePhase::Important
            } else {
                CascadePhase::Normal
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct DeclarationHistoryContext {
    pub(super) layer: Option<LayerContextId>,
    pub(super) origin: CascadeOrigin,
    pub(super) phase: CascadePhase,
}

impl DeclarationHistoryContext {
    pub(super) fn is_important(self) -> bool {
        matches!(self.phase, CascadePhase::Important)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct LayerContextId(pub(super) u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct CascadeOrigin(u8);

impl CascadeOrigin {
    const AUTHOR: Self = Self(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum CascadePhase {
    Normal,
    Important,
}

#[derive(Default)]
pub(super) struct HistoryTraversal {
    next_layer_context: u32,
}

impl HistoryTraversal {
    pub(super) fn next_layer_context(&mut self) -> LayerContextId {
        let context = LayerContextId(self.next_layer_context);
        self.next_layer_context += 1;
        context
    }
}

pub(super) struct RuleState {
    pub(super) ast_slot: usize,
    pub(super) selector_summary: SelectorSummary,
    pub(super) live: bool,
    pub(super) previous_live: Option<RuleId>,
    pub(super) next_live: Option<RuleId>,
    pub(super) previous_edge: Option<EdgeId>,
    pub(super) next_edge: Option<EdgeId>,
    pub(super) segment: SegmentId,
    pub(super) sequence: SequenceId,
    pub(super) history: HistoryId,
    pub(super) retained_child_count: u32,
    pub(super) order_label: u64,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct SelectorSummary {
    pub(super) hash: u64,
    pub(super) live_len: u32,
    pub(super) vendor_prefixes: u8,
    pub(super) materializable: bool,
}

#[derive(Default)]
pub(super) struct PrecomputedHasher {
    hash: Option<u64>,
}

impl Hasher for PrecomputedHasher {
    #[inline]
    fn write(&mut self, _: &[u8]) {
        unreachable!("precomputed hash map keys must write one u64")
    }

    #[inline]
    fn write_u64(&mut self, hash: u64) {
        debug_assert!(self.hash.is_none());
        self.hash = Some(hash);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash.expect("precomputed hash was not written")
    }
}

pub(super) struct SequenceState<'ast> {
    pub(super) blocks: std::vec::Vec<DeclarationEntryId>,
    pub(super) revision: u32,
    pub(super) fingerprint: Option<CachedFingerprint<'ast>>,
    pub(super) owner: RuleId,
}

pub(super) struct CachedFingerprint<'ast> {
    pub(super) revision: u32,
    pub(super) summary: SequenceSummary<'ast>,
}

pub(super) struct SequenceSummary<'ast> {
    pub(super) live_len: u32,
    pub(super) shape_hash: u64,
    pub(super) occurrences: std::vec::Vec<DeclarationOccurrence<'ast>>,
}

#[derive(Clone, Copy)]
pub(super) struct DeclarationOccurrence<'ast> {
    pub(super) block: Ref<'ast, DeclarationBlock<'ast>>,
    pub(super) entry: DeclarationEntryId,
    pub(super) index: usize,
    pub(super) history_context: DeclarationHistoryContext,
    pub(super) shape_hash: u64,
}

pub(super) struct DeclarationEntryState<'ast> {
    pub(super) block: Ref<'ast, DeclarationBlock<'ast>>,
    pub(super) sequence: SequenceId,
}

pub(super) struct HistoryState {
    pub(super) entries: std::vec::Vec<DeclarationEntryId>,
    pub(super) representative: RuleId,
    pub(super) vendor_prefix: VendorPrefix,
    pub(super) generation: u32,
    pub(super) consumed_generation: u32,
    pub(super) queued: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EdgeStatus {
    DirtySameSelector,
    DirtyPartial,
    Candidate(u32),
    Stable,
    Stale,
}

pub(super) struct EdgeState {
    pub(super) left: RuleId,
    pub(super) right: RuleId,
    pub(super) same_selector: bool,
    pub(super) status: EdgeStatus,
}

#[derive(Default)]
pub(super) struct WorkQueues {
    pub(super) same_selector: VecDeque<EdgeId>,
    pub(super) histories: VecDeque<HistoryId>,
    pub(super) partial: VecDeque<EdgeId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declaration_history_context_distinguishes_every_cascade_dimension() {
        let base = DeclarationHistoryContext {
            layer: Some(LayerContextId(1)),
            origin: CascadeOrigin::AUTHOR,
            phase: CascadePhase::Normal,
        };

        assert_ne!(
            base,
            DeclarationHistoryContext {
                layer: Some(LayerContextId(2)),
                ..base
            }
        );
        assert_ne!(
            base,
            DeclarationHistoryContext {
                origin: CascadeOrigin(1),
                ..base
            }
        );
        assert_ne!(
            base,
            DeclarationHistoryContext {
                phase: CascadePhase::Important,
                ..base
            }
        );
    }
}
