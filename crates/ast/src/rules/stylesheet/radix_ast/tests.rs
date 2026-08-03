use std::mem::size_of;

use super::*;

#[test]
fn typed_ids_keep_compact_optional_layout() {
    assert_eq!(size_of::<RuleId>(), size_of::<u32>());
    assert_eq!(size_of::<Option<RuleId>>(), size_of::<u32>());
    assert_eq!(size_of::<DeclarationBlockId>(), size_of::<u32>());
    assert_eq!(size_of::<Option<DeclarationBlockId>>(), size_of::<u32>());
}

#[test]
fn lexical_order_and_direct_topology_are_independent() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<&str, &str, &str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();

    let outer = compilation.append_rule(root, "outer").unwrap();
    let children = compilation.create_child_list(outer).unwrap();
    let nested = compilation.append_rule(children, "nested").unwrap();
    let following = compilation.append_rule(root, "following").unwrap();
    let key = compilation.append_effective_key("nested@root").unwrap();
    let block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(nested), key)
        .unwrap();
    let declaration = compilation
        .append_declaration(block, "color:red", false)
        .unwrap();

    assert_eq!(outer.primary_index(), 0);
    assert_eq!(nested.primary_index(), 1);
    assert_eq!(following.primary_index(), 2);
    assert_eq!(block.primary_index(), 0);
    assert_eq!(declaration.index(), 0);
    assert_eq!(
        compilation
            .rules_in_source_order()
            .map(|(_, rule)| *rule.payload())
            .collect::<std::vec::Vec<_>>(),
        ["outer", "nested", "following"]
    );
    assert_eq!(
        compilation
            .rules_in_list(root)
            .unwrap()
            .map(|(_, rule)| *rule.payload())
            .collect::<std::vec::Vec<_>>(),
        ["outer", "following"]
    );
    assert_eq!(
        compilation.rule(outer).unwrap().next_sibling(),
        Some(following)
    );
    assert_eq!(
        compilation.rule(following).unwrap().previous_sibling(),
        Some(outer)
    );
    assert_eq!(compilation.rule(nested).unwrap().parent(), Some(outer));
    assert_eq!(
        compilation.declaration_block(block).unwrap().owner(),
        DeclarationBlockOwner::Rule(nested)
    );
    assert_eq!(
        compilation.rule(nested).unwrap().declaration_block(),
        Some(block)
    );
    assert_eq!(
        compilation
            .declaration_block(block)
            .unwrap()
            .effective_key(),
        key
    );
    assert_eq!(compilation.effective_key(key), Some(&"nested@root"));
    assert_eq!(
        compilation
            .declarations_in_block(block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        ["color:red"]
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn validation_rejects_a_broken_mutual_link() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, (), ()>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let first = compilation.append_rule(root, 1).unwrap();
    let second = compilation.append_rule(root, 2).unwrap();

    compilation.rule_mut(second).unwrap().previous_sibling = None;

    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::RuleHasWrongPrevious {
            rule: second,
            expected: Some(first),
        })
    );
}

#[test]
fn child_list_is_owned_once() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<(), (), ()>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let parent = compilation.append_rule(root, ()).unwrap();
    compilation.create_child_list(parent).unwrap();

    assert_eq!(
        compilation.create_child_list(parent),
        Err(MutationError::ChildListAlreadyExists(parent))
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn validation_rejects_a_child_list_owned_by_another_rule() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<(), (), ()>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let parent = compilation.append_rule(root, ()).unwrap();
    let other = compilation.append_rule(root, ()).unwrap();
    let children = compilation.create_child_list(parent).unwrap();

    compilation.rule_mut(other).unwrap().child_list = Some(children);

    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::ChildListHasWrongParent {
            rule: other,
            list: children,
            actual: Some(parent),
        })
    );
}

#[test]
fn adjacent_equal_key_blocks_merge_without_a_previous_merged_chain() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(left), key)
        .unwrap();
    compilation
        .append_declaration(left_block, 10, false)
        .unwrap();
    let right = compilation.append_rule(root, 2).unwrap();
    let right_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(right), key)
        .unwrap();
    compilation
        .append_declaration(right_block, 20, false)
        .unwrap();

    let merged = compilation
        .merge_adjacent_rule_declaration_blocks(left, right)
        .unwrap();

    assert_eq!(merged.retired_block, left_block);
    assert_eq!(merged.retained_block, right_block);
    assert!(!compilation.rule(left).unwrap().is_live());
    assert!(compilation.rule(right).unwrap().is_live());
    assert_eq!(
        compilation
            .declarations_in_block(left_block)
            .unwrap()
            .count(),
        0
    );
    assert_eq!(
        compilation
            .declarations_in_block(right_block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 20]
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn synthesized_rule_and_block_use_final_radix_ids_with_appended_declarations() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("shared").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(left), key)
        .unwrap();
    compilation
        .append_declaration(left_block, 10, false)
        .unwrap();
    let right = compilation.append_rule(root, 2).unwrap();
    let right_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(right), key)
        .unwrap();
    compilation
        .append_declaration(right_block, 20, false)
        .unwrap();

    let inserted_rule = compilation.insert_rule_after(left, 3).unwrap();
    assert!(inserted_rule.remaps.is_empty());
    let inserted_block = compilation
        .insert_declaration_block_between(left_block, Some(right_block), inserted_rule.id, key)
        .unwrap();
    assert!(inserted_block.remaps.is_empty());
    compilation
        .append_declaration(inserted_block.id, 30, false)
        .unwrap();

    assert_eq!(
        compilation
            .rules_in_list(root)
            .unwrap()
            .map(|(_, rule)| *rule.payload())
            .collect::<std::vec::Vec<_>>(),
        [1, 3, 2]
    );
    assert_eq!(
        compilation
            .declaration_blocks_in_source_order()
            .map(|(_, block)| block.owner())
            .collect::<std::vec::Vec<_>>(),
        [
            DeclarationBlockOwner::Rule(left),
            DeclarationBlockOwner::Rule(inserted_rule.id),
            DeclarationBlockOwner::Rule(right),
        ]
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn noncontiguous_small_merge_uses_local4_without_copying_declarations() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(left), key)
        .unwrap();
    let first = compilation
        .append_declaration(left_block, 10, false)
        .unwrap();
    let following = compilation.append_rule(root, 2).unwrap();
    let following_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(following), key)
        .unwrap();
    compilation
        .append_declaration(following_block, 20, false)
        .unwrap();

    let inserted = compilation.insert_rule_after(left, 3).unwrap().id;
    let inserted_block = compilation
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    let second = compilation
        .append_declaration(inserted_block, 30, false)
        .unwrap();
    compilation
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();

    assert!(matches!(
        compilation
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Local4(_)
    ));
    assert_eq!(
        compilation
            .declaration_ids_in_block(inserted_block)
            .unwrap()
            .collect::<std::vec::Vec<_>>(),
        [first, second]
    );
    assert_eq!(
        compilation
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 30]
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn noncontiguous_large_merge_uses_arena_overflow_without_copying_declarations() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(left), key)
        .unwrap();
    for value in [10, 11, 12] {
        compilation
            .append_declaration(left_block, value, false)
            .unwrap();
    }
    let following = compilation.append_rule(root, 2).unwrap();
    let following_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(following), key)
        .unwrap();
    compilation
        .append_declaration(following_block, 20, false)
        .unwrap();

    let inserted = compilation.insert_rule_after(left, 3).unwrap().id;
    let inserted_block = compilation
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    for value in [30, 31, 32] {
        compilation
            .append_declaration(inserted_block, value, false)
            .unwrap();
    }
    compilation
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();

    assert!(matches!(
        compilation
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Overflow(_)
    ));
    assert_eq!(
        compilation
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 11, 12, 30, 31, 32]
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn fifth_local_declaration_promotes_the_complete_sequence_to_overflow() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(left), key)
        .unwrap();
    for value in [10, 11] {
        compilation
            .append_declaration(left_block, value, false)
            .unwrap();
    }
    let following = compilation.append_rule(root, 2).unwrap();
    let following_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(following), key)
        .unwrap();
    compilation
        .append_declaration(following_block, 20, false)
        .unwrap();
    let inserted = compilation.insert_rule_after(left, 3).unwrap().id;
    let inserted_block = compilation
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    for value in [30, 31] {
        compilation
            .append_declaration(inserted_block, value, false)
            .unwrap();
    }
    compilation
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();
    assert!(matches!(
        compilation
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Local4(_)
    ));

    compilation
        .append_transformed_declaration(inserted_block, 32, false)
        .unwrap();

    assert!(matches!(
        compilation
            .declaration_block(inserted_block)
            .unwrap()
            .declarations(),
        DeclarationList::Overflow(_)
    ));
    assert_eq!(
        compilation
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|declaration| *declaration.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 11, 30, 31, 32]
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn a_rule_owns_at_most_one_declaration_block() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<(), (), ()>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let owner = compilation.append_rule(root, ()).unwrap();
    let key = compilation.append_effective_key(()).unwrap();
    compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(owner), key)
        .unwrap();

    assert_eq!(
        compilation.append_declaration_block(DeclarationBlockOwner::Rule(owner), key),
        Err(MutationError::DeclarationBlockAlreadyExists(owner))
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn a_declaration_range_cannot_cross_a_nested_allocation() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<(), &str, ()>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let outer = compilation.append_rule(root, ()).unwrap();
    let nested = compilation.append_rule(root, ()).unwrap();
    let key = compilation.append_effective_key(()).unwrap();
    let outer_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(outer), key)
        .unwrap();
    compilation
        .append_declaration(outer_block, "before", false)
        .unwrap();
    let nested_block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(nested), key)
        .unwrap();
    compilation
        .append_declaration(nested_block, "nested", false)
        .unwrap();

    assert_eq!(
        compilation.append_declaration(outer_block, "after", false),
        Err(MutationError::NonContiguousDeclarationRange(outer_block))
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}
