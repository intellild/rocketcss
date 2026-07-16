/// Compares AST values after omitting tombstoned child slots.
///
/// A tombstone value is omitted by the container that owns its slot. Comparing
/// the tombstone value itself remains structural, which keeps this relation
/// transitive and leaves [`PartialEq`] unchanged.
pub trait EqIgnoringTombstones {
    fn eq_ignoring_tombstones(&self, other: &Self) -> bool;
}
