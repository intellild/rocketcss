use super::{
    COMPACT_PRIMARY_CAPACITY, LOCAL_BITS, NO_SIBLING_GROUP, OVERFLOW_CAPACITY,
    RadixAllocationCounts, RadixCapacityError, RadixId, RadixIdKey, RadixIndexArena, RadixLeaf,
    RadixRange, RadixRoot, SIBLING_MASK, TypedRadixIndexArena,
};
use crate::Allocator;

struct TestRuleMarker;
type TestRuleId = RadixId<TestRuleMarker>;

struct ShortExact<I> {
    inner: I,
    reported_len: usize,
}

impl<I> ShortExact<I> {
    fn new(inner: I, reported_len: usize) -> Self {
        Self {
            inner,
            reported_len,
        }
    }
}

impl<I: Iterator> Iterator for ShortExact<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.reported_len, Some(self.reported_len))
    }
}

impl<I: Iterator> ExactSizeIterator for ShortExact<I> {}

fn insert_with_entry<T: Unpin, I: RadixIdKey>(
    values: &mut TypedRadixIndexArena<'_, T, I>,
    primary: I,
    sibling_key: u16,
    value: T,
) -> I {
    values
        .sibling_entry(primary)
        .expect("the primary has sibling capacity")
        .try_insert(sibling_key, value)
        .unwrap_or_else(|_| panic!("the sibling key has capacity"))
}

#[test]
fn primary_ids_directly_address_parse_vector() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);

    let first = values.push_primary(10);
    let second = values.push_primary(20);

    assert_eq!(first.get(), 0);
    assert_eq!(second.get(), 1 << LOCAL_BITS);
    assert_eq!(first.primary_index(), 0);
    assert_eq!(second.primary_index(), 1);
    assert_eq!(values[first], 10);
    assert_eq!(values[second], 20);
}

#[test]
fn siblings_use_direct_and_leaf_storage_and_iterate_by_key() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let first = values.push_primary(0);
    let second = values.push_primary(4);
    values.push_primary(6);

    assert!(values.sibling_primary_indices.is_empty());
    assert!(values.sibling_trees.is_empty());

    let high_branch = insert_with_entry(&mut values, first, 512, 3);
    let low_branch = insert_with_entry(&mut values, first, 1, 1);
    let next_leaf = insert_with_entry(&mut values, first, 32, 2);
    insert_with_entry(&mut values, second, 1, 5);

    assert_eq!(values.sibling_primary_indices.len(), 2);
    assert_eq!(values.sibling_trees.len(), 2);
    assert_eq!(high_branch.sibling_key(), 512);
    assert_eq!(values[low_branch], 1);
    assert_eq!(values[next_leaf], 2);
    assert_eq!(values[high_branch], 3);
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [0, 1, 2, 3, 4, 5, 6]
    );
    assert_eq!(values.len(), 7);
    assert_eq!(values.primary_iter().len(), 3);
    assert_eq!(second.primary_index(), 1);
}

#[test]
fn first_level_direct_value_does_not_allocate_a_leaf() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let direct = insert_with_entry(&mut values, primary, 512, 3);

    assert_eq!(values.get(direct), Some(&3));
    assert_eq!(
        values.sibling_trees[0].allocation_counts(),
        RadixAllocationCounts {
            roots: 1,
            leaves: 0,
            values: 1,
        }
    );
    let root = values.sibling_trees[0].root.as_ref().unwrap();
    assert!(root.direct[16].is_some());
    assert!(root.leaves.iter().all(|leaf| leaf.is_none()));
}

#[test]
fn nonzero_low_value_allocates_only_the_matching_leaf() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    insert_with_entry(&mut values, primary, 513, 4);

    assert_eq!(
        values.sibling_trees[0].allocation_counts(),
        RadixAllocationCounts {
            roots: 1,
            leaves: 1,
            values: 1,
        }
    );
    let root = values.sibling_trees[0].root.as_ref().unwrap();
    assert!(root.direct.iter().all(|value| value.is_none()));
    assert!(root.leaves[16].is_some());
    assert!(root.leaves.iter().filter(|leaf| leaf.is_some()).count() == 1);
}

#[test]
fn direct_and_leaf_values_coexist_at_one_high_branch() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let direct = insert_with_entry(&mut values, primary, 512, 2);
    let leaf = insert_with_entry(&mut values, primary, 513, 3);

    assert_eq!(values.get(direct), Some(&2));
    assert_eq!(values.get(leaf), Some(&3));
    assert_eq!(
        values
            .iter_enumerated()
            .map(|(id, value)| (id.sibling_key(), *value))
            .collect::<std::vec::Vec<_>>(),
        [(0, 0), (512, 2), (513, 3)]
    );
    assert_eq!(
        values.sibling_trees[0].allocation_counts(),
        RadixAllocationCounts {
            roots: 1,
            leaves: 1,
            values: 2,
        }
    );
}

#[test]
fn iteration_orders_low_boundary_keys_numerically() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    insert_with_entry(&mut values, primary, 511, 511);
    insert_with_entry(&mut values, primary, 513, 513);
    insert_with_entry(&mut values, primary, 512, 512);

    assert_eq!(
        values
            .iter_enumerated()
            .map(|(id, value)| (id.sibling_key(), *value))
            .collect::<std::vec::Vec<_>>(),
        [(0, 0), (511, 511), (512, 512), (513, 513)]
    );
}

#[test]
fn direct_and_leaf_mutation_paths_preserve_masks_and_reuse_rules() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let direct = insert_with_entry(&mut values, primary, 512, 2);
    let leaf = insert_with_entry(&mut values, primary, 513, 3);

    *values.get_mut(direct).unwrap() = 20;
    *values.get_mut(leaf).unwrap() = 30;
    assert_eq!(values.remove_sibling(direct), Some(20));
    let reused_direct = insert_with_entry(&mut values, primary, 512, 200);
    assert_eq!(reused_direct, direct);
    assert_eq!(values.get(reused_direct), Some(&200));

    assert_eq!(values.retire_sibling(leaf), Some(30));
    assert_eq!(values.get(leaf), None);
    assert!(values.sibling_trees[0].is_used(513));
    assert_eq!(values.get(reused_direct), Some(&200));
}

#[test]
fn an_empty_leaf_does_not_hide_a_live_direct_value() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let leaf = insert_with_entry(&mut values, primary, 513, 3);
    assert_eq!(values.retire_sibling(leaf), Some(3));
    let direct = insert_with_entry(&mut values, primary, 512, 2);

    let root = values.sibling_trees[0].root.as_ref().unwrap();
    assert_eq!(root.direct_occupied, 1 << 16);
    assert_eq!(root.leaves[16].as_deref().unwrap().occupied, 0);
    assert_eq!(values.get(direct), Some(&2));
    assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [0, 2]);
}

#[test]
fn retired_direct_id_remains_unavailable() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let direct = insert_with_entry(&mut values, primary, 512, 2);
    assert_eq!(values.retire_sibling(direct), Some(2));
    assert!(values.sibling_trees[0].is_used(512));

    let duplicate = values.sibling_entry(primary).unwrap().try_insert(512, 4);
    assert!(duplicate.is_err());
    assert_eq!(duplicate.unwrap_err().value, 4);
    assert_eq!(values.get(direct), None);
}

#[test]
fn sibling_entry_reuses_one_resolved_group_for_multiple_insertions() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0_u16);

    let (first, second) = {
        let mut entry = values.sibling_entry(primary).unwrap();
        (
            entry.try_insert(128, 1).unwrap(),
            entry.try_insert(512, 2).unwrap(),
        )
    };

    assert_eq!(values.get(first), Some(&1));
    assert_eq!(values.get(second), Some(&2));
    assert_eq!(values.sibling_trees.len(), 1);
}

#[test]
fn radix_pages_do_not_embed_payload_storage() {
    assert_eq!(
        std::mem::size_of::<RadixRoot<'static, u8>>(),
        std::mem::size_of::<RadixRoot<'static, [u8; 4096]>>()
    );
    assert_eq!(
        std::mem::size_of::<RadixLeaf<'static, u8>>(),
        std::mem::size_of::<RadixLeaf<'static, [u8; 4096]>>()
    );
}

#[test]
fn retired_sibling_id_is_not_reused_by_normal_insertion() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(1);
    let sibling = insert_with_entry(&mut values, primary, 17, 2);

    assert_eq!(values.retire_sibling(sibling), Some(2));
    assert_eq!(values.get(sibling), None);
    assert_eq!(
        values
            .semantic_iter()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [1]
    );
    let replacement = values
        .entry_between(primary, None)
        .unwrap()
        .try_insert(3)
        .unwrap();
    assert_ne!(replacement, sibling);
    *values.get_mut(replacement).unwrap() = 4;

    assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [1, 4]);
}

#[test]
fn removed_non_ast_sibling_key_can_be_reused() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(1);
    let sibling = insert_with_entry(&mut values, primary, 17, 2);

    assert_eq!(values.remove_sibling(sibling), Some(2));
    let replacement = insert_with_entry(&mut values, primary, 17, 3);

    assert_eq!(replacement, sibling);
    assert_eq!(values.get(replacement), Some(&3));
}

#[test]
fn sibling_capacity_can_be_preflighted_for_ast_overflow() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    for key in 1..=SIBLING_MASK as u16 {
        insert_with_entry(&mut values, primary, key, key);
    }

    let error = values
        .sibling_entry(primary)
        .unwrap()
        .try_insert(1, u16::MAX)
        .unwrap_err();
    assert_eq!(
        error.error,
        RadixCapacityError::SiblingTreeExhausted { primary }
    );
    assert_eq!(error.value, u16::MAX);
    assert_eq!(values.len(), SIBLING_MASK as usize + 1);
}

#[test]
fn retired_id_exhaustion_selects_the_ast_overflow_path() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    for key in 1..=SIBLING_MASK as u16 {
        let id = insert_with_entry(&mut values, primary, key, key);
        assert_eq!(values.retire_sibling(id), Some(key));
    }

    let error = values
        .sibling_entry(primary)
        .unwrap()
        .try_insert(1, u16::MAX)
        .unwrap_err();
    assert_eq!(
        error.error,
        RadixCapacityError::SiblingTreeExhausted { primary }
    );
    assert_eq!(error.value, u16::MAX);
    assert!(values.entry_between(primary, None).is_none());
    assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [0]);
}

#[test]
fn entry_between_rejects_a_gap_without_an_unused_key() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let next_primary = values.push_primary(4);
    let left = insert_with_entry(&mut values, primary, 1, 1);
    let right = insert_with_entry(&mut values, primary, 2, 3);
    let untouched = insert_with_entry(&mut values, next_primary, 512, 5);

    assert!(values.entry_between(left, Some(right)).is_none());
    assert_eq!(values.get(left), Some(&1));
    assert_eq!(values.get(right), Some(&3));
    assert_eq!(values.get(untouched), Some(&5));
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [0, 1, 3, 4, 5]
    );
}

#[test]
fn entry_between_inserts_without_relabeling() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let first = values.push_primary(0);
    let second = values.push_primary(2);

    let inserted = values
        .entry_between(first, Some(second))
        .unwrap()
        .try_insert(1)
        .unwrap();

    assert_eq!(inserted.sibling_key(), 512);
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [0, 1, 2]
    );
}

#[test]
fn randomized_local_edits_match_a_reference_sequence() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let mut reference = (0_u32..8)
        .map(|value| (values.push_primary(value), value))
        .collect::<std::vec::Vec<_>>();
    let mut random = 0x9e37_79b9_u32;
    let mut next_value = 8_u32;

    for step in 0..256 {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;

        let removable = reference
            .iter()
            .enumerate()
            .filter_map(|(index, (id, _))| (!id.is_primary()).then_some(index))
            .collect::<std::vec::Vec<_>>();
        if step % 5 == 4 && !removable.is_empty() {
            let index = removable[random as usize % removable.len()];
            let (id, value) = reference.remove(index);
            assert_eq!(values.retire_sibling(id), Some(value));
        } else {
            let after_index = random as usize % reference.len();
            let after = reference[after_index].0;
            let before = reference.get(after_index + 1).map(|(id, _)| *id);
            if let Some(entry) = values.entry_between(after, before) {
                let id = entry.try_insert(next_value).unwrap();
                reference.insert(after_index + 1, (id, next_value));
                next_value += 1;
            }
        }

        assert_eq!(
            values
                .iter_enumerated()
                .map(|(id, value)| (id, *value))
                .collect::<std::vec::Vec<_>>(),
            reference
        );
    }
}

#[test]
fn radix_ids_reserve_u32_max_as_the_option_niche() {
    assert_eq!(std::mem::size_of::<RadixId<u8>>(), 4);
    assert_eq!(std::mem::size_of::<Option<RadixId<u8>>>(), 4);

    let highest_overflow_id = RadixId::<u8>::from_overflow_index(OVERFLOW_CAPACITY - 1);
    assert_eq!(highest_overflow_id.get(), u32::MAX - 7);
}

#[test]
fn authored_primary_overflow_preserves_ids_lookup_and_iteration_order() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let mut last_compact = None;
    for _ in 0..COMPACT_PRIMARY_CAPACITY {
        last_compact = Some(values.push_primary(0_u8));
    }

    let overflow = values.push_primary(1_u8);
    let next_overflow = values.push_primary(2_u8);
    let last_compact = last_compact.unwrap();
    let primary_boundary_range = RadixRange::new(last_compact, next_overflow, 3);

    assert_eq!(
        values.primary_slice_in_range(primary_boundary_range),
        Some([0, 1, 2].as_slice())
    );
    assert_eq!(
        values
            .iter_range_enumerated(primary_boundary_range)
            .unwrap()
            .map(|(id, value)| (id, *value))
            .collect::<std::vec::Vec<_>>(),
        [(last_compact, 0), (overflow, 1), (next_overflow, 2)]
    );

    let boundary_sibling = insert_with_entry(&mut values, last_compact, 512, 9_u8);

    assert!(!last_compact.is_overflow());
    assert!(overflow.is_primary());
    assert!(overflow.is_overflow());
    assert!(last_compact < overflow);
    assert!(overflow < next_overflow);
    assert_eq!(overflow.primary_index(), COMPACT_PRIMARY_CAPACITY);
    assert_eq!(next_overflow.primary_index(), COMPACT_PRIMARY_CAPACITY + 1);
    assert_eq!(values.get(overflow), Some(&1));
    assert_eq!(values.get(next_overflow), Some(&2));
    assert_eq!(
        RadixId::from_primary_index(COMPACT_PRIMARY_CAPACITY),
        overflow
    );
    assert_eq!(
        RadixId::from_primary_index(COMPACT_PRIMARY_CAPACITY + 1),
        next_overflow
    );
    assert_eq!(values.primary_iter().len(), COMPACT_PRIMARY_CAPACITY + 2);
    assert_eq!(values.primary_iter().next_back(), Some(&2));
    assert_eq!(values.iter_enumerated().last(), Some((next_overflow, &2)));
    assert_eq!(
        values
            .ids()
            .skip(COMPACT_PRIMARY_CAPACITY - 1)
            .collect::<std::vec::Vec<_>>(),
        [last_compact, boundary_sibling, overflow, next_overflow]
    );
    let boundary_range = RadixRange::new(last_compact, next_overflow, 4);
    assert!(values.primary_slice_in_range(boundary_range).is_none());
    assert_eq!(
        values
            .iter_range(boundary_range)
            .unwrap()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [0, 9, 1, 2]
    );
    assert!(values.sibling_entry(overflow).is_none());
    assert!(values.sibling_entry(next_overflow).is_none());
    assert!(values.entry_between(overflow, None).is_none());
}

#[test]
fn ids_resolve_their_own_storage_value() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(1);
    let sibling = insert_with_entry(&mut values, primary, 17, 2);

    assert_eq!(values[primary], 1);
    assert_eq!(values[sibling], 2);
    assert_eq!(values.remove_sibling(sibling), Some(2));
}

#[test]
fn typed_ids_isolate_stores_without_changing_layout() {
    assert_eq!(std::mem::size_of::<TestRuleId>(), 4);
    assert_eq!(std::mem::size_of::<Option<TestRuleId>>(), 4);

    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(2, &allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(30);
    let inserted = insert_with_entry(&mut values, first, 512, 20);

    assert_eq!(first.get(), 0);
    assert!(first.is_primary());
    assert!(!first.is_overflow());
    assert_eq!(second.primary_index(), 1);
    assert_eq!(inserted.sibling_key(), 512);
    assert_eq!(values.get(inserted), Some(&20));
}

#[test]
fn semantic_id_cursor_crosses_primary_and_inserted_values() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(3, &allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(30);
    let third = values.push_primary(50);
    let after_first = insert_with_entry(&mut values, first, 512, 20);
    let after_second = insert_with_entry(&mut values, second, 512, 40);

    assert_eq!(
        values.ids().collect::<std::vec::Vec<_>>(),
        [first, after_first, second, after_second, third]
    );

    assert_eq!(values.retire_sibling(after_first), Some(20));
    assert_eq!(
        values.ids().collect::<std::vec::Vec<_>>(),
        [first, second, after_second, third]
    );
}

#[test]
fn stable_group_sidecar_decouples_lookup_from_semantic_group_order() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(3, &allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(30);
    let third = values.push_primary(50);

    let after_third = insert_with_entry(&mut values, third, 512, 60);
    let after_first = insert_with_entry(&mut values, first, 512, 20);

    assert_eq!(values.sibling_primary_indices.as_slice(), [0, 2]);
    assert_eq!(
        values.sibling_group_indices.as_slice(),
        [1, NO_SIBLING_GROUP, 0]
    );
    assert_eq!(values.get(after_first), Some(&20));
    assert_eq!(values.get(after_third), Some(&60));
    assert_eq!(
        values.ids().collect::<std::vec::Vec<_>>(),
        [first, after_first, second, third, after_third]
    );

    let range = RadixRange::new(first, after_third, 5);
    values
        .for_each_in_range_mut(range, |_, value| *value += 1)
        .unwrap();
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [11, 21, 31, 51, 61]
    );
}

#[test]
fn mutable_enumerated_visit_needs_no_id_snapshot() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(2, &allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(30);
    let inserted = insert_with_entry(&mut values, first, 512, 20);
    let mut visited = std::vec::Vec::new();

    values.for_each_enumerated_mut(|id, value| {
        visited.push(id);
        *value += 1;
    });

    assert_eq!(visited, [first, inserted, second]);
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [11, 21, 31]
    );
}

#[test]
fn enumerated_iterators_return_ids_in_storage_order() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::with_capacity_in(3, &allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(30);
    let third = values.push_primary(50);
    let inserted_after_first = insert_with_entry(&mut values, first, 512, 20);
    let inserted_after_second = insert_with_entry(&mut values, second, 512, 40);

    assert_eq!(
        values
            .primary_iter_enumerated()
            .map(|(id, value)| (id, *value))
            .collect::<std::vec::Vec<_>>(),
        [(first, 10), (second, 30), (third, 50)]
    );

    let mut iter = values.iter_enumerated();
    assert_eq!(iter.len(), 5);
    assert_eq!(
        iter.by_ref()
            .map(|(id, value)| (id, *value))
            .collect::<std::vec::Vec<_>>(),
        [
            (first, 10),
            (inserted_after_first, 20),
            (second, 30),
            (inserted_after_second, 40),
            (third, 50),
        ]
    );
    assert_eq!(iter.len(), 0);
}

#[test]
fn empty_range_never_resolves_its_placeholder_start() {
    let allocator = Allocator::new();
    let values = TypedRadixIndexArena::<u8, TestRuleId>::new_in(&allocator);
    let range = RadixRange::empty();

    assert!(range.is_empty());
    assert_eq!(values.ids_in_range(range).unwrap().len(), 0);
    assert_eq!(values.iter_range(range).unwrap().len(), 0);
    assert_eq!(values.iter_range_enumerated(range).unwrap().len(), 0);
}

#[test]
fn primary_range_slice_proves_contiguous_ranges_without_resolving_ids() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(20);
    let third = values.push_primary(30);

    assert_eq!(
        values.primary_slice_in_range(RadixRange::empty()),
        Some([].as_slice())
    );
    assert_eq!(
        values.primary_slice_in_range(RadixRange::singleton(second)),
        Some([20].as_slice())
    );

    let range = RadixRange::new(first, third, 3);
    assert_eq!(
        values.primary_slice_in_range(range),
        Some([10, 20, 30].as_slice())
    );
    assert!(
        values
            .primary_slice_in_range(RadixRange::new(first, third, 2))
            .is_none()
    );

    let mut iter = values.iter_range(range).unwrap();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.len(), 2);

    let mut enumerated = values.iter_range_enumerated(range).unwrap();
    assert_eq!(enumerated.len(), 3);
    assert_eq!(enumerated.next(), Some((first, &10)));
    assert_eq!(enumerated.next(), Some((second, &20)));
    assert_eq!(enumerated.next(), Some((third, &30)));
    assert_eq!(enumerated.len(), 0);

    let stale = range;
    insert_with_entry(&mut values, second, 512, 25);
    assert!(values.primary_slice_in_range(stale).is_none());
    assert!(values.ids_in_range(stale).is_none());
    assert!(values.detached_ids_in_range(stale).is_none());
    assert!(values.iter_range(stale).is_none());
    assert!(values.iter_range_enumerated(stale).is_none());

    let before = values.iter().copied().collect::<std::vec::Vec<_>>();
    assert!(
        values
            .for_each_in_range_mut(stale, |_, value| *value += 1)
            .is_none()
    );
    assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), before);
}

#[test]
fn primary_ranges_use_the_number_of_values_actually_pushed() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);

    let empty = values.push_primary_range(ShortExact::new(std::iter::empty::<u8>(), 2));
    assert!(empty.is_empty());

    let range = values.push_primary_range(ShortExact::new([10].into_iter(), 2));
    assert_eq!(range.len(), 1);
    assert_eq!(
        values
            .iter_range(range)
            .unwrap()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [10]
    );
}

#[test]
fn primary_range_capacity_failure_preserves_the_iterator() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    values.len = u32::MAX;

    let error = values.try_push_primary_range([20, 30]).unwrap_err();
    assert_eq!(error.error, RadixCapacityError::ArenaExhausted);
    assert_eq!(error.values.collect::<std::vec::Vec<_>>(), [20, 30]);
    assert_eq!(
        values.primary_iter().copied().collect::<std::vec::Vec<_>>(),
        [10]
    );
    assert_eq!(values.get(first), Some(&10));
    assert_eq!(values.len, u32::MAX);
}

#[test]
fn ranges_follow_semantic_order_across_siblings_and_primaries() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(30);
    let inserted = insert_with_entry(&mut values, first, 512, 20);
    let range = RadixRange::new(first, second, 3);

    assert!(values.primary_slice_in_range(range).is_none());
    assert_eq!(range.start_id(), first);
    assert_eq!(range.last_id(), second);
    assert_eq!(values.ids_in_range(range).unwrap().nth(1), Some(inserted));
    assert_eq!(
        values
            .iter_range(range)
            .unwrap()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [10, 20, 30]
    );
    let mut iter = values.iter_range(range).unwrap();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&30));
    let mut enumerated = values.iter_range_enumerated(range).unwrap();
    assert_eq!(enumerated.len(), 3);
    assert_eq!(enumerated.next(), Some((first, &10)));
    assert_eq!(enumerated.len(), 2);
    assert_eq!(
        enumerated.collect::<std::vec::Vec<_>>(),
        [(inserted, &20), (second, &30)]
    );
    values
        .for_each_in_range_mut(range, |_, value| *value += 1)
        .unwrap();
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [11, 21, 31]
    );
}

#[test]
fn a_sibling_after_the_last_primary_does_not_block_the_slice_fast_path() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(20);
    let after_second = insert_with_entry(&mut values, second, 512, 30);

    let ending_at_primary = RadixRange::new(first, second, 2);
    assert_eq!(
        values.primary_slice_in_range(ending_at_primary),
        Some([10, 20].as_slice())
    );

    let starting_at_sibling = RadixRange::new(after_second, after_second, 1);
    assert!(values.primary_slice_in_range(starting_at_sibling).is_none());
    assert_eq!(
        values
            .iter_range(starting_at_sibling)
            .unwrap()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [30]
    );
}

#[test]
fn bounded_range_cursor_resumes_at_a_sibling_and_retains_its_following_id() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(40);
    let third = values.push_primary(70);
    let after_first = insert_with_entry(&mut values, first, 256, 20);
    let later_after_first = insert_with_entry(&mut values, first, 768, 30);
    let after_second = insert_with_entry(&mut values, second, 512, 50);
    let range = RadixRange::new(after_first, second, 3);

    let mut ids = values.ids_in_range(range).unwrap();
    assert_eq!(
        ids.by_ref().collect::<std::vec::Vec<_>>(),
        [after_first, later_after_first, second]
    );
    assert_eq!(ids.following(), Some(after_second));
    assert_eq!(ids.following(), None);

    let mut detached = values.detached_ids_in_range(range).unwrap();
    assert_eq!(detached.next(&values), Some(after_first));
    assert_eq!(detached.next(&values), Some(later_after_first));
    assert_eq!(detached.next(&values), Some(second));
    assert_eq!(detached.next(&values), None);

    let mutable_range = RadixRange::new(after_first, after_second, 4);
    values
        .for_each_in_range_mut(mutable_range, |_, value| *value += 1)
        .unwrap();
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [10, 21, 31, 41, 51, 70]
    );
    assert_eq!(third.primary_index(), 2);
}

#[test]
fn stable_batch_insertion_preserves_existing_ids_and_returns_one_range() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(40);

    let entry = values.range_entry_between(first, Some(second)).unwrap();
    assert!(entry.capacity() >= 2);
    let inserted = entry.try_push([20, 30]).unwrap();

    assert_eq!(values.get(first), Some(&10));
    assert_eq!(values.get(second), Some(&40));
    assert_eq!(inserted.len(), 2);
    assert_eq!(
        values
            .iter_range(inserted)
            .unwrap()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [20, 30]
    );
    assert_eq!(inserted.last_id().sibling_key(), 2);
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [10, 20, 30, 40]
    );
}

#[test]
fn stable_ranges_use_the_number_of_values_actually_inserted() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(40);

    let empty = values
        .range_entry_between(first, Some(second))
        .unwrap()
        .try_push(ShortExact::new(std::iter::empty::<i32>(), 1))
        .unwrap_or_else(|_| panic!("the range has capacity"));
    assert!(empty.is_empty());

    let inserted = values
        .range_entry_between(first, Some(second))
        .unwrap()
        .try_push(ShortExact::new([20].into_iter(), 2))
        .unwrap_or_else(|_| panic!("the range has capacity"));
    assert_eq!(inserted.len(), 1);
    assert_eq!(
        values
            .iter_range(inserted)
            .unwrap()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [20]
    );
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [10, 20, 40]
    );
}

#[test]
fn stable_batch_capacity_is_preflighted_for_the_complete_range() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(50);
    let boundary = insert_with_entry(&mut values, first, 3, 40);
    let before_len = values.len();

    assert_eq!(
        values
            .range_entry_between(first, Some(boundary))
            .unwrap()
            .capacity(),
        2
    );
    assert_eq!(values.len(), before_len);
    assert_eq!(
        values.ids().collect::<std::vec::Vec<_>>(),
        [first, boundary, second]
    );
}

#[test]
fn sibling_interval_exact_fill_succeeds_and_one_more_is_atomic() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(50);
    let boundary = insert_with_entry(&mut values, first, 3, 40);

    let inserted = values
        .range_entry_between(first, Some(boundary))
        .unwrap()
        .try_push([20, 30])
        .unwrap();
    assert_eq!(inserted.len(), 2);
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [10, 20, 30, 40, 50]
    );

    let before_ids = values.ids().collect::<std::vec::Vec<_>>();
    let before_values = values.iter().copied().collect::<std::vec::Vec<_>>();
    let before_len = values.len();
    let error = values
        .range_entry_between(inserted.last_id(), Some(boundary))
        .unwrap()
        .try_push([60])
        .unwrap_err();
    assert_eq!(
        error.error,
        RadixCapacityError::IntervalExhausted {
            primary: first,
            lower: 2,
            upper: 3,
            needed: 1,
            available: 0,
        }
    );
    assert_eq!(error.values.collect::<std::vec::Vec<_>>(), [60]);
    assert_eq!(values.ids().collect::<std::vec::Vec<_>>(), before_ids);
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        before_values
    );
    assert_eq!(values.len(), before_len);
    assert_eq!(values.get(second), Some(&50));
}

#[test]
fn retired_tombstones_count_against_interval_capacity() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let boundary = insert_with_entry(&mut values, first, 3, 40);
    let retired = insert_with_entry(&mut values, first, 2, 30);
    assert_eq!(values.retire_sibling(retired), Some(30));

    let before_ids = values.ids().collect::<std::vec::Vec<_>>();
    let before_len = values.len();
    let error = values
        .range_entry_between(first, Some(boundary))
        .unwrap()
        .try_push([20, 25])
        .unwrap_err();
    assert_eq!(
        error.error,
        RadixCapacityError::IntervalExhausted {
            primary: first,
            lower: 0,
            upper: 3,
            needed: 2,
            available: 1,
        }
    );
    assert_eq!(error.values.collect::<std::vec::Vec<_>>(), [20, 25]);
    assert_eq!(values.ids().collect::<std::vec::Vec<_>>(), before_ids);
    assert_eq!(values.len(), before_len);
    assert!(values.sibling_trees[0].is_used(2));

    assert!(values.reclaim_retired_sibling(retired));
    let inserted = values
        .range_entry_between(first, Some(boundary))
        .unwrap()
        .try_push([20, 30])
        .unwrap();
    assert_eq!(inserted.len(), 2);
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [10, 20, 30, 40]
    );
}

#[test]
fn arena_exhaustion_returns_the_unwritten_value() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(10);
    values.len = u32::MAX;
    let before = values.iter().copied().collect::<std::vec::Vec<_>>();

    let error = values
        .sibling_entry(primary)
        .unwrap()
        .try_insert(1, 20)
        .unwrap_err();
    assert_eq!(error.error, RadixCapacityError::ArenaExhausted);
    assert_eq!(error.value, 20);
    assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), before);
    assert_eq!(values.len, u32::MAX);
}

#[test]
fn sibling_range_crosses_direct_and_leaf_slots_in_constant_time_steps() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let lower = insert_with_entry(&mut values, primary, 31, 31);
    let upper = insert_with_entry(&mut values, primary, 34, 34);

    let inserted = values
        .range_entry_between(lower, Some(upper))
        .unwrap()
        .try_push([32, 33])
        .unwrap();
    assert_eq!(inserted.start_id().sibling_key(), 32);
    assert_eq!(inserted.last_id().sibling_key(), 33);
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [0, 31, 32, 33, 34]
    );
}

#[test]
fn stable_batch_at_the_arena_tail_uses_siblings() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let inserted = values
        .range_entry_between(first, None)
        .unwrap()
        .try_push([20, 30])
        .unwrap();

    assert!(!inserted.start_id().is_primary());
    assert_eq!(values.primary_iter().len(), 1);
    assert_eq!(
        values
            .iter_range(inserted)
            .unwrap()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [20, 30]
    );
}
