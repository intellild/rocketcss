use std::mem::size_of;

use super::*;

#[test]
fn typed_ids_keep_compact_optional_layout() {
    assert_eq!(size_of::<RuleId<&str>>(), size_of::<u32>());
    assert_eq!(size_of::<Option<RuleId<&str>>>(), size_of::<u32>());
    assert_eq!(size_of::<DeclarationBlockId<&str>>(), size_of::<u32>());
    assert_eq!(
        size_of::<Option<DeclarationBlockId<&str>>>(),
        size_of::<u32>()
    );
}

#[test]
fn lexical_order_and_direct_topology_are_independent() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<&str, &str, &str>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();

    let outer = stylesheet.append_rule(root, "outer").unwrap();
    let children = stylesheet.create_child_list(outer).unwrap();
    let nested = stylesheet.append_rule(children, "nested").unwrap();
    let following = stylesheet.append_rule(root, "following").unwrap();
    let key = stylesheet.append_effective_key("nested@root").unwrap();
    let block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<&str>::Rule(nested), key)
        .unwrap();
    let declaration = stylesheet
        .append_declaration(block, "color:red", false)
        .unwrap();

    assert_eq!(outer.primary_index(), 0);
    assert_eq!(nested.primary_index(), 1);
    assert_eq!(following.primary_index(), 2);
    assert_eq!(block.primary_index(), 0);
    assert_eq!(declaration.index(), 0);
    assert_eq!(
        stylesheet
            .rules_in_source_order()
            .map(|(_, rule)| *rule.payload())
            .collect::<std::vec::Vec<_>>(),
        ["outer", "nested", "following"]
    );
    assert_eq!(
        stylesheet
            .rules_in_list(root)
            .unwrap()
            .map(|(_, rule)| *rule.payload())
            .collect::<std::vec::Vec<_>>(),
        ["outer", "following"]
    );
    assert_eq!(
        stylesheet.rule(outer).unwrap().next_sibling(),
        Some(following)
    );
    assert_eq!(
        stylesheet.rule(following).unwrap().previous_sibling(),
        Some(outer)
    );
    assert_eq!(stylesheet.rule(nested).unwrap().parent(), Some(outer));
    assert_eq!(
        stylesheet.declaration_block(block).unwrap().owner(),
        DeclarationBlockOwner::<&str>::Rule(nested)
    );
    assert_eq!(
        stylesheet.rule(nested).unwrap().declaration_block(),
        Some(block)
    );
    assert_eq!(
        stylesheet.declaration_block(block).unwrap().effective_key(),
        key
    );
    assert_eq!(stylesheet.effective_key(key), Some(&"nested@root"));
    assert_eq!(
        stylesheet
            .declarations_in_block(block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        ["color:red"]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn validation_rejects_a_broken_mutual_link() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, (), ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let first = stylesheet.append_rule(root, 1).unwrap();
    let second = stylesheet.append_rule(root, 2).unwrap();

    stylesheet.rule_mut(second).unwrap().previous_sibling = None;

    assert_eq!(
        stylesheet.validate_ast(),
        Err(ValidationError::<u8>::RuleHasWrongPrevious {
            rule: second,
            expected: Some(first),
        })
    );
}

#[test]
fn validation_rejects_a_corrupt_descendant_span_before_direct_iteration() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, (), ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let parent = stylesheet.append_rule(root, 1).unwrap();
    let children = stylesheet.create_child_list(parent).unwrap();
    stylesheet.append_rule(children, 2).unwrap();
    stylesheet.finalize_parsed_rule_ranges();

    stylesheet.rule_mut(parent).unwrap().descendants = 2;

    assert_eq!(
        stylesheet.validate_ast(),
        Err(ValidationError::<u8>::RuleDescendantsMismatch {
            rule: parent,
            expected: 1,
            actual: 2,
        })
    );
}

#[test]
fn child_list_is_owned_once() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<(), (), ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let parent = stylesheet.append_rule(root, ()).unwrap();
    stylesheet.create_child_list(parent).unwrap();

    assert_eq!(
        stylesheet.create_child_list(parent),
        Err(MutationError::<()>::ChildListAlreadyExists(parent))
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn finalized_rule_ranges_describe_nested_preorder() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<&str, (), ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let a = stylesheet.append_rule(root, "a").unwrap();
    let a_children = stylesheet.create_child_list(a).unwrap();
    let a1 = stylesheet.append_rule(a_children, "a1").unwrap();
    let a1_children = stylesheet.create_child_list(a1).unwrap();
    let a11 = stylesheet.append_rule(a1_children, "a11").unwrap();
    let a2 = stylesheet.append_rule(a_children, "a2").unwrap();
    let empty_children = stylesheet.create_child_list(a2).unwrap();
    let b = stylesheet.append_rule(root, "b").unwrap();

    let topology_ids = stylesheet
        .rules_in_list(a_children)
        .unwrap()
        .map(|(id, _)| id)
        .collect::<std::vec::Vec<_>>();

    stylesheet.finalize_parsed_rule_ranges();

    assert_eq!(stylesheet.rule(a).unwrap().descendants(), 3);
    assert_eq!(stylesheet.rule(a1).unwrap().descendants(), 1);
    assert_eq!(stylesheet.rule_list(a1_children).unwrap().range().len(), 1);
    assert_eq!(stylesheet.rule_list(a_children).unwrap().range().len(), 3);
    assert!(
        stylesheet
            .rule_list(empty_children)
            .unwrap()
            .range()
            .is_empty()
    );
    assert_eq!(stylesheet.rules_in_list(empty_children).unwrap().len(), 0);
    let root_range = stylesheet.rule_list(root).unwrap().range();
    assert_eq!(root_range.start_id(), a);
    assert_eq!(root_range.last_id(), b);
    assert_eq!(root_range.len(), 5);
    assert_eq!(
        stylesheet
            .rules_in_list(a_children)
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        topology_ids
    );
    assert_eq!(
        stylesheet
            .rules_in_list(a1_children)
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [a11]
    );

    let a12 = stylesheet.insert_rule_after(a11, "a12").unwrap().id;
    assert_eq!(stylesheet.rule(a1).unwrap().descendants(), 2);
    assert_eq!(stylesheet.rule(a).unwrap().descendants(), 4);
    assert_eq!(stylesheet.rule_list(a1_children).unwrap().range().len(), 2);
    assert_eq!(stylesheet.rule_list(a_children).unwrap().range().len(), 4);
    assert_eq!(stylesheet.rule_list(root).unwrap().range().len(), 6);
    assert_eq!(
        stylesheet
            .rules_in_list(a1_children)
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [a11, a12]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn append_after_finalization_updates_the_range_projection() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<&str, (), ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let parent = stylesheet.append_rule(root, "parent").unwrap();
    let children = stylesheet.create_child_list(parent).unwrap();
    stylesheet.finalize_parsed_rule_ranges();

    let child = stylesheet.append_rule(children, "child").unwrap();

    assert_eq!(stylesheet.rule(parent).unwrap().descendants(), 1);
    assert_eq!(stylesheet.rule_list(children).unwrap().range().len(), 1);
    let root_range = stylesheet.rule_list(root).unwrap().range();
    assert_eq!(root_range.len(), 2);
    assert_eq!(root_range.last_id(), child);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn mutation_preserves_physical_rule_ranges() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, (), ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let first = stylesheet.append_rule(root, 1).unwrap();
    let last = stylesheet.append_rule(root, 2).unwrap();
    stylesheet.finalize_parsed_rule_ranges();

    let middle = stylesheet.insert_rule_after(first, 3).unwrap().id;
    assert_eq!(stylesheet.rule_list(root).unwrap().range().len(), 3);
    stylesheet.retire_rule(middle).unwrap();
    assert_eq!(stylesheet.rule_list(root).unwrap().range().len(), 3);
    assert_eq!(
        stylesheet
            .rules_in_list(root)
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [first, last]
    );

    let new_last = stylesheet.insert_rule_after(last, 4).unwrap().id;
    let root_range = stylesheet.rule_list(root).unwrap().range();
    assert_eq!(root_range.len(), 4);
    assert_eq!(root_range.last_id(), new_last);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn validation_rejects_a_child_list_owned_by_another_rule() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<(), (), ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let parent = stylesheet.append_rule(root, ()).unwrap();
    let other = stylesheet.append_rule(root, ()).unwrap();
    let children = stylesheet.create_child_list(parent).unwrap();

    stylesheet.rule_mut(other).unwrap().child_list = Some(children);

    assert_eq!(
        stylesheet.validate_ast(),
        Err(ValidationError::<()>::ChildListHasWrongParent {
            rule: other,
            list: children,
            actual: Some(parent),
        })
    );
}

#[test]
fn adjacent_equal_key_blocks_merge_without_a_previous_merged_chain() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let key = stylesheet.append_effective_key("same").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    stylesheet
        .append_declaration(left_block, 10, false)
        .unwrap();
    let right = stylesheet.append_rule(root, 2).unwrap();
    let right_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(right), key)
        .unwrap();
    stylesheet
        .append_declaration(right_block, 20, false)
        .unwrap();

    let merged = stylesheet
        .merge_adjacent_rule_declaration_blocks(left, right)
        .unwrap();

    assert_eq!(merged.retired_block, left_block);
    assert_eq!(merged.retained_block, right_block);
    assert!(!stylesheet.rule(left).unwrap().is_live());
    assert!(stylesheet.rule(right).unwrap().is_live());
    assert_eq!(
        stylesheet
            .declarations_in_block(left_block)
            .unwrap()
            .count(),
        0
    );
    assert_eq!(
        stylesheet
            .declarations_in_block(right_block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 20]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn synthesized_rule_and_block_use_final_radix_ids_with_appended_declarations() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let key = stylesheet.append_effective_key("shared").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    stylesheet
        .append_declaration(left_block, 10, false)
        .unwrap();
    let right = stylesheet.append_rule(root, 2).unwrap();
    let right_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(right), key)
        .unwrap();
    stylesheet
        .append_declaration(right_block, 20, false)
        .unwrap();

    let inserted_rule = stylesheet.insert_rule_after(left, 3).unwrap();
    assert!(inserted_rule.remaps.is_empty());
    let inserted_block = stylesheet
        .insert_declaration_block_between(left_block, Some(right_block), inserted_rule.id, key)
        .unwrap();
    assert!(inserted_block.remaps.is_empty());
    stylesheet
        .append_declaration(inserted_block.id, 30, false)
        .unwrap();

    assert_eq!(
        stylesheet
            .rules_in_list(root)
            .unwrap()
            .map(|(_, rule)| *rule.payload())
            .collect::<std::vec::Vec<_>>(),
        [1, 3, 2]
    );
    assert_eq!(
        stylesheet
            .declaration_blocks_in_source_order()
            .map(|(_, block)| block.owner())
            .collect::<std::vec::Vec<_>>(),
        [
            DeclarationBlockOwner::<u8>::Rule(left),
            DeclarationBlockOwner::<u8>::Rule(inserted_rule.id),
            DeclarationBlockOwner::<u8>::Rule(right),
        ]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn noncontiguous_small_merge_uses_local4_without_copying_declarations() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let key = stylesheet.append_effective_key("same").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let first = stylesheet
        .append_declaration(left_block, 10, false)
        .unwrap();
    let following = stylesheet.append_rule(root, 2).unwrap();
    let following_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    stylesheet
        .append_declaration(following_block, 20, false)
        .unwrap();

    let inserted = stylesheet.insert_rule_after(left, 3).unwrap().id;
    let inserted_block = stylesheet
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    let second = stylesheet
        .append_declaration(inserted_block, 30, false)
        .unwrap();
    stylesheet
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();

    assert!(matches!(
        stylesheet
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Local4(_)
    ));
    assert_eq!(
        stylesheet
            .declaration_ids_in_block(inserted_block)
            .unwrap()
            .collect::<std::vec::Vec<_>>(),
        [first, second]
    );
    assert_eq!(
        stylesheet
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 30]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn noncontiguous_large_merge_uses_arena_overflow_without_copying_declarations() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let key = stylesheet.append_effective_key("same").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    for value in [10, 11, 12] {
        stylesheet
            .append_declaration(left_block, value, false)
            .unwrap();
    }
    let following = stylesheet.append_rule(root, 2).unwrap();
    let following_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    stylesheet
        .append_declaration(following_block, 20, false)
        .unwrap();

    let inserted = stylesheet.insert_rule_after(left, 3).unwrap().id;
    let inserted_block = stylesheet
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    for value in [30, 31, 32] {
        stylesheet
            .append_declaration(inserted_block, value, false)
            .unwrap();
    }
    stylesheet
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();

    assert!(matches!(
        stylesheet
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Overflow(_)
    ));
    assert_eq!(
        stylesheet
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 11, 12, 30, 31, 32]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn fifth_local_declaration_promotes_the_complete_sequence_to_overflow() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let key = stylesheet.append_effective_key("same").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    for value in [10, 11] {
        stylesheet
            .append_declaration(left_block, value, false)
            .unwrap();
    }
    let following = stylesheet.append_rule(root, 2).unwrap();
    let following_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    stylesheet
        .append_declaration(following_block, 20, false)
        .unwrap();
    let inserted = stylesheet.insert_rule_after(left, 3).unwrap().id;
    let inserted_block = stylesheet
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    for value in [30, 31] {
        stylesheet
            .append_declaration(inserted_block, value, false)
            .unwrap();
    }
    stylesheet
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();
    assert!(matches!(
        stylesheet
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Local4(_)
    ));

    stylesheet
        .append_transformed_declaration(inserted_block, 32, false)
        .unwrap();

    assert!(matches!(
        stylesheet
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Overflow(_)
    ));
    assert_eq!(
        stylesheet
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 11, 30, 31, 32]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn streaming_declaration_mutation_preserves_range_local4_and_overflow_order() {
    let allocator = Allocator::new();

    let mut range = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = range.stylesheet_root().root_rules();
    let key = range.append_effective_key("range").unwrap();
    let rule = range.append_rule(root, 0).unwrap();
    let block = range
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(rule), key)
        .unwrap();
    let range_ids = [
        range.append_declaration(block, 1, false).unwrap(),
        range.append_declaration(block, 2, false).unwrap(),
        range.append_declaration(block, 3, false).unwrap(),
    ];
    let mut visited = std::vec::Vec::new();
    range
        .for_each_declaration_mut(block, |id, record| {
            visited.push(id);
            *record.payload_mut() += 10;
        })
        .unwrap();
    assert_eq!(visited, range_ids);
    assert_eq!(
        range
            .declarations_in_block(block)
            .unwrap()
            .map(|record| *record.payload())
            .collect::<std::vec::Vec<_>>(),
        [11, 12, 13]
    );

    let mut local4 = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = local4.stylesheet_root().root_rules();
    let key = local4.append_effective_key("local4").unwrap();
    let left = local4.append_rule(root, 0).unwrap();
    let left_block = local4
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let first = local4.append_declaration(left_block, 1, false).unwrap();
    let following = local4.append_rule(root, 1).unwrap();
    let following_block = local4
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    local4
        .append_declaration(following_block, 2, false)
        .unwrap();
    let inserted = local4.insert_rule_after(left, 2).unwrap().id;
    let inserted_block = local4
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    let second = local4.append_declaration(inserted_block, 3, false).unwrap();
    local4
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();
    assert!(matches!(
        local4
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Local4(_)
    ));
    visited.clear();
    local4
        .for_each_declaration_mut(inserted_block, |id, record| {
            visited.push(id);
            *record.payload_mut() += 10;
        })
        .unwrap();
    assert_eq!(visited, [first, second]);
    assert_eq!(
        local4
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|record| *record.payload())
            .collect::<std::vec::Vec<_>>(),
        [11, 13]
    );

    let mut overflow = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = overflow.stylesheet_root().root_rules();
    let key = overflow.append_effective_key("overflow").unwrap();
    let left = overflow.append_rule(root, 0).unwrap();
    let left_block = overflow
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let mut expected = std::vec::Vec::new();
    for value in [1, 2, 3] {
        expected.push(
            overflow
                .append_declaration(left_block, value, false)
                .unwrap(),
        );
    }
    let following = overflow.append_rule(root, 1).unwrap();
    let following_block = overflow
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    overflow
        .append_declaration(following_block, 4, false)
        .unwrap();
    let inserted = overflow.insert_rule_after(left, 2).unwrap().id;
    let inserted_block = overflow
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    for value in [5, 6, 7] {
        expected.push(
            overflow
                .append_declaration(inserted_block, value, false)
                .unwrap(),
        );
    }
    overflow
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();
    assert!(matches!(
        overflow
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Overflow(_)
    ));
    visited.clear();
    overflow
        .for_each_declaration_mut(inserted_block, |id, record| {
            visited.push(id);
            *record.payload_mut() += 10;
        })
        .unwrap();
    assert_eq!(visited, expected);
    assert_eq!(
        overflow
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|record| *record.payload())
            .collect::<std::vec::Vec<_>>(),
        [11, 12, 13, 15, 16, 17]
    );
}

#[test]
fn a_rule_owns_at_most_one_declaration_block() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<(), (), ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let owner = stylesheet.append_rule(root, ()).unwrap();
    let key = stylesheet.append_effective_key(()).unwrap();
    stylesheet
        .append_declaration_block(DeclarationBlockOwner::<()>::Rule(owner), key)
        .unwrap();

    assert_eq!(
        stylesheet.append_declaration_block(DeclarationBlockOwner::<()>::Rule(owner), key),
        Err(MutationError::<()>::DeclarationBlockAlreadyExists(owner))
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn a_declaration_range_cannot_cross_a_nested_allocation() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<(), &str, ()>::new_in(&allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let outer = stylesheet.append_rule(root, ()).unwrap();
    let nested = stylesheet.append_rule(root, ()).unwrap();
    let key = stylesheet.append_effective_key(()).unwrap();
    let outer_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<()>::Rule(outer), key)
        .unwrap();
    stylesheet
        .append_declaration(outer_block, "before", false)
        .unwrap();
    let nested_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<()>::Rule(nested), key)
        .unwrap();
    stylesheet
        .append_declaration(nested_block, "nested", false)
        .unwrap();

    assert_eq!(
        stylesheet.append_declaration(outer_block, "after", false),
        Err(MutationError::<()>::NonContiguousDeclarationRange(
            outer_block
        ))
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}
