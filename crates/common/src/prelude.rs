//! Common compiler infrastructure.

pub use crate::{
    Allocator, Atom, BTreeIndexArena, DenseId, DenseIdGenerator, DenseMap, DenseStore,
    PriorityQueue, RadixId, RadixIdKey, RadixIdRemap, RadixIndexArena, RadixInsertResult,
    RadixRange, StringPool, TypedRadixIndexArena,
    bit_vec::BitVec,
    boxed::Box,
    ghost_cell::{GhostBox, GhostCell, GhostToken},
    hash_map::{AdaptiveHashMap, HashMap},
    hash_set::{AdaptiveHashSet, HashSet},
    reference::Ref,
    vec::Vec,
};
