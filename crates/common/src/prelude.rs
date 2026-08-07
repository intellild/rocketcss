//! Common compiler infrastructure.

pub use crate::{
    Allocator, Atom, BTreeIndexArena, DenseId, DenseIdGenerator, DenseMap, DenseStore,
    RadixCapacityError, RadixDirectRangeIter, RadixDirectRangeIterEnumerated, RadixId, RadixIdKey,
    RadixIndexArena, RadixInsertError, RadixRange, RadixRangeItem, RadixRangePushError,
    RadixSiblingEntry, RadixSiblingRangeEntry, RadixSiblingSlotState, RadixVacantEntry, StringPool,
    TypedRadixIndexArena,
    bit_vec::BitVec,
    boxed::Box,
    ghost_cell::{GhostBox, GhostCell, GhostToken},
    hash_map::{AdaptiveHashMap, HashMap},
    hash_set::{AdaptiveHashSet, HashSet},
    reference::Ref,
    vec::Vec,
};
