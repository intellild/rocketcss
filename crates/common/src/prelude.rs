//! Common compiler infrastructure.

pub use crate::{
    Allocator, Atom, BTreeIndexArena, DenseId, DenseIdGenerator, DenseMap, DenseStore, RadixId,
    RadixIdRemap, RadixIndexArena, RadixIndexId, RadixInsertResult, StringPool,
    TypedRadixIndexArena,
    bit_vec::BitVec,
    boxed::Box,
    ghost_cell::{GhostBox, GhostCell, GhostToken},
    hash_map::{AdaptiveHashMap, HashMap},
    hash_set::{AdaptiveHashSet, HashSet},
    reference::Ref,
    vec::Vec,
};
