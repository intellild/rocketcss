use super::{
    COMPACT_PRIMARY_CAPACITY, LOCAL_BITS, NO_SIBLING_GROUP, OVERFLOW_CAPACITY,
    RadixAllocationCounts, RadixId, RadixIndexArena, RadixLeaf, RadixRange, RadixRoot,
    SIBLING_MASK, TypedRadixIndexArena,
};
use crate::Allocator;

struct TestRuleMarker;
type TestRuleId = RadixId<TestRuleMarker>;

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

    let high_branch = values.insert_sibling(first, 512, 3);
    let low_branch = values.insert_sibling(first, 1, 1);
    let next_leaf = values.insert_sibling(first, 32, 2);
    values.insert_sibling(second, 1, 5);

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
    let direct = values.insert_sibling(primary, 512, 3);

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
    values.insert_sibling(primary, 513, 4);

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
    let direct = values.insert_sibling(primary, 512, 2);
    let leaf = values.insert_sibling(primary, 513, 3);

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
    values.insert_sibling(primary, 511, 511);
    values.insert_sibling(primary, 513, 513);
    values.insert_sibling(primary, 512, 512);

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
    let direct = values.insert_sibling(primary, 512, 2);
    let leaf = values.insert_sibling(primary, 513, 3);

    *values.get_mut(direct).unwrap() = 20;
    *values.get_mut(leaf).unwrap() = 30;
    assert_eq!(values.remove_sibling(direct), Some(20));
    let reused_direct = values.insert_sibling(primary, 512, 200);
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
    let leaf = values.insert_sibling(primary, 513, 3);
    assert_eq!(values.retire_sibling(leaf), Some(3));
    let direct = values.insert_sibling(primary, 512, 2);

    let root = values.sibling_trees[0].root.as_ref().unwrap();
    assert_eq!(root.direct_occupied, 1 << 16);
    assert_eq!(root.leaves[16].as_deref().unwrap().occupied, 0);
    assert_eq!(values.get(direct), Some(&2));
    assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [0, 2]);
}

#[test]
fn retired_direct_id_remains_unavailable() {
    use std::panic::AssertUnwindSafe;

    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let direct = values.insert_sibling(primary, 512, 2);
    assert_eq!(values.retire_sibling(direct), Some(2));
    assert!(values.sibling_trees[0].is_used(512));

    let duplicate =
        std::panic::catch_unwind(AssertUnwindSafe(|| values.insert_sibling(primary, 512, 4)));
    assert!(duplicate.is_err());
    assert_eq!(values.get(direct), None);
}

#[test]
fn relabeling_restores_an_unchanged_direct_value() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    for key in 1..=514 {
        values.insert_sibling(primary, key, key);
    }

    let result = values.insert_between(
        RadixId::<u16>::from_parts(0, 513),
        Some(RadixId::<u16>::from_parts(0, 514)),
        10_000,
    );

    assert_eq!(values.get(RadixId::<u16>::from_parts(0, 512)), Some(&512));
    assert!(
        result
            .remaps
            .iter()
            .all(|remap| remap.old.sibling_key() != 512)
    );
    assert_eq!(values.get(result.id), Some(&10_000));
}

#[test]
fn batch_reservation_relabels_one_group_once_for_multiple_insertions() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0_u16);
    values.insert_sibling(primary, 1, 1);
    values.insert_sibling(primary, 2, 2);
    values.insert_sibling(primary, 3, 3);

    let reservation = values
        .reserve_sibling_positions(primary, &[1, 3])
        .expect("one group has capacity for both terminal insertions");
    assert!(!reservation.remaps.is_empty());
    assert_eq!(reservation.reserved.len(), 2);
    assert_eq!(values.get(reservation.reserved[0]), None);
    assert_eq!(values.get(reservation.reserved[1]), None);

    values
        .activate_reserved_sibling(reservation.reserved[0], 10)
        .unwrap();
    values
        .activate_reserved_sibling(reservation.reserved[1], 20)
        .unwrap();
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [0, 1, 10, 2, 20, 3]
    );
}

#[test]
fn relabeling_can_move_a_direct_value_into_a_leaf() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0_u16);
    for key in 1..=512 {
        values.insert_sibling(primary, key, key);
    }
    let old_direct = RadixId::<u16>::from_parts(0, 512);

    let result =
        values.insert_between(RadixId::<u16>::from_parts(0, 511), Some(old_direct), 10_000);

    let remap = result
        .remaps
        .iter()
        .find(|remap| remap.old == old_direct)
        .copied()
        .expect("the direct value crosses into a leaf during relabeling");
    assert_eq!(remap.new.sibling_key(), 514);
    assert_eq!(values.get(old_direct), None);
    assert_eq!(values.get(remap.new), Some(&512));
    assert_eq!(values.get(result.id), Some(&10_000));
    assert!(values.sibling_trees[0].root.as_ref().unwrap().direct[16].is_none());
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
    let sibling = values.insert_sibling(primary, 17, 2);

    assert_eq!(values.retire_sibling(sibling), Some(2));
    assert_eq!(values.get(sibling), None);
    assert_eq!(
        values
            .semantic_iter()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [1]
    );
    let replacement = values.insert_between(primary, None, 3).id;
    assert_ne!(replacement, sibling);
    *values.get_mut(replacement).unwrap() = 4;

    assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [1, 4]);
}

#[test]
fn removed_non_ast_sibling_key_can_be_reused() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(1);
    let sibling = values.insert_sibling(primary, 17, 2);

    assert_eq!(values.remove_sibling(sibling), Some(2));
    let replacement = values.insert_sibling(primary, 17, 3);

    assert_eq!(replacement, sibling);
    assert_eq!(values.get(replacement), Some(&3));
}

#[test]
fn sibling_capacity_can_be_preflighted_for_ast_overflow() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    for key in 1..=SIBLING_MASK as u16 {
        values.insert_sibling(primary, key, key);
    }

    assert!(!values.can_insert_sibling(primary));
    assert_eq!(values.len(), SIBLING_MASK as usize + 1);
}

#[test]
fn retired_id_exhaustion_selects_the_ast_overflow_path() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    for key in 1..=SIBLING_MASK as u16 {
        let id = values.insert_sibling(primary, key, key);
        assert_eq!(values.retire_sibling(id), Some(key));
    }

    assert!(!values.can_insert_sibling(primary));
    assert!(!values.can_insert_between(primary, None));
    assert_eq!(values.iter().copied().collect::<std::vec::Vec<_>>(), [0]);
}

#[test]
fn insert_between_relabels_only_one_local_sibling_group() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(0);
    let next_primary = values.push_primary(4);
    let left = values.insert_sibling(primary, 1, 1);
    let right = values.insert_sibling(primary, 2, 3);
    let untouched = values.insert_sibling(next_primary, 512, 5);

    let result = values.insert_between(left, Some(right), 2);

    assert_eq!(result.remaps.len(), 1);
    assert_eq!(result.remaps[0].old, right);
    assert_eq!(values.get(left), Some(&1));
    assert_eq!(values.get(right), None);
    assert_eq!(values.get(untouched), Some(&5));
    assert_eq!(
        values.iter().copied().collect::<std::vec::Vec<_>>(),
        [0, 1, 2, 3, 4, 5]
    );
}

#[test]
fn insert_between_uses_a_gap_without_relabeling() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let first = values.push_primary(0);
    let second = values.push_primary(2);

    let result = values.insert_between(first, Some(second), 1);

    assert!(result.remaps.is_empty());
    assert_eq!(result.id.sibling_key(), 512);
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
            let result = values.insert_between(after, before, next_value);
            for remap in result.remaps {
                let entry = reference
                    .iter_mut()
                    .find(|(id, _)| *id == remap.old)
                    .expect("every remapped ID exists in the reference sequence");
                entry.0 = remap.new;
            }
            reference.insert(after_index + 1, (result.id, next_value));
            next_value += 1;
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

    let boundary_sibling = values.insert_sibling(last_compact, 512, 9_u8);

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
    assert!(!values.can_insert_sibling(overflow));
    assert!(!values.can_insert_sibling(next_overflow));
    assert!(!values.can_insert_between(overflow, None));
}

#[test]
fn ids_resolve_their_own_storage_value() {
    let allocator = Allocator::new();
    let mut values = RadixIndexArena::new_in(&allocator);
    let primary = values.push_primary(1);
    let sibling = values.insert_sibling(primary, 17, 2);

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
    let inserted = values.insert_sibling(first, 512, 20);

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
    let after_first = values.insert_sibling(first, 512, 20);
    let after_second = values.insert_sibling(second, 512, 40);

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

    let after_third = values.insert_sibling(third, 512, 60);
    let after_first = values.insert_sibling(first, 512, 20);

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
    let inserted = values.insert_sibling(first, 512, 20);
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
    let inserted_after_first = values.insert_sibling(first, 512, 20);
    let inserted_after_second = values.insert_sibling(second, 512, 40);

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
    values.insert_sibling(second, 512, 25);
    assert!(values.primary_slice_in_range(stale).is_none());
}

#[test]
fn ranges_follow_semantic_order_across_siblings_and_primaries() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(30);
    let inserted = values.insert_sibling(first, 512, 20);
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
    let after_second = values.insert_sibling(second, 512, 30);

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
    let after_first = values.insert_sibling(first, 256, 20);
    let later_after_first = values.insert_sibling(first, 768, 30);
    let after_second = values.insert_sibling(second, 512, 50);
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

    assert!(values.can_insert_stable_range_between(first, Some(second), 2));
    let inserted = values.insert_stable_range_between(first, Some(second), [20, 30]);

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
fn stable_batch_capacity_is_preflighted_for_the_complete_range() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let second = values.push_primary(50);
    let boundary = values.insert_sibling(first, 3, 40);
    let before_len = values.len();

    assert!(values.can_insert_stable_range_between(first, Some(boundary), 2));
    assert!(!values.can_insert_stable_range_between(first, Some(boundary), 3));
    assert_eq!(values.len(), before_len);
    assert_eq!(
        values.ids().collect::<std::vec::Vec<_>>(),
        [first, boundary, second]
    );
}

#[test]
fn stable_batch_at_the_arena_tail_appends_primaries() {
    let allocator = Allocator::new();
    let mut values = TypedRadixIndexArena::<_, TestRuleId>::new_in(&allocator);
    let first = values.push_primary(10);
    let inserted = values.insert_stable_range_between(first, None, [20, 30]);

    assert!(inserted.start_id().is_primary());
    assert_eq!(values.primary_iter().len(), 3);
    assert_eq!(
        values
            .iter_range(inserted)
            .unwrap()
            .copied()
            .collect::<std::vec::Vec<_>>(),
        [20, 30]
    );
}
