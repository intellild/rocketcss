//! Common compiler infrastructure.

pub use crate::{
    Allocator, AstStr, Atom, DenseId, DenseIdRange, DenseMap, DenseRange, DenseStore, StringPool,
    bit_vec::BitVec,
    boxed::Box,
    ghost_cell::{GhostBox, GhostCell, GhostToken},
    hash_map::{AdaptiveHashMap, HashMap},
    hash_set::{AdaptiveHashSet, HashSet},
    reference::Ref,
    vec::Vec,
};
