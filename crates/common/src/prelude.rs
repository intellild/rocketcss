//! Common compiler infrastructure.

pub use crate::{
    Allocator, Atom, BTreeIndexArena, DenseId, DenseIdGenerator, DenseMap, DenseStore,
    RadixCapacityError, RadixId, RadixIdKey, RadixIndexArena, RadixInsertError, RadixRange,
    RadixRangePushError, RadixSiblingEntry, RadixSiblingRangeEntry, RadixSiblingSlotState,
    RadixVacantEntry, StringPool, TypedRadixIndexArena,
    bit_vec::BitVec,
    boxed::Box,
    ghost_cell::{GhostBox, GhostCell, GhostToken},
    hash_map::{AdaptiveHashMap, HashMap},
    hash_set::{AdaptiveHashSet, HashSet},
    reference::Ref,
    vec::Vec,
};
