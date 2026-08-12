use std::mem::size_of;

use super::*;

#[test]
fn typed_ids_keep_compact_optional_layout() {
    assert_eq!(size_of::<RuleId<'_, &str>>(), size_of::<u32>());
    assert_eq!(size_of::<Option<RuleId<'_, &str>>>(), size_of::<u32>());
    assert_eq!(size_of::<DeclarationBlockId<'_, &str>>(), size_of::<u32>());
    assert_eq!(
        size_of::<Option<DeclarationBlockId<'_, &str>>>(),
        size_of::<u32>()
    );
    assert_eq!(size_of::<RuleListId<'_>>(), size_of::<u32>());
    assert_eq!(size_of::<EffectiveKeyId<'_>>(), size_of::<u32>());
    assert_eq!(size_of::<DeclarationId<'_>>(), size_of::<u32>());
    assert_eq!(
        size_of::<ScopedDeclarationHandle<'static, 'static>>(),
        size_of::<u32>()
    );
    assert_eq!(size_of::<SelectorValueId<'_>>(), size_of::<u32>());
    assert_eq!(size_of::<SelectorPathId<'_>>(), size_of::<u32>());
    assert_eq!(size_of::<ContextValueId<'_>>(), size_of::<u32>());
    assert_eq!(size_of::<ContextPathId<'_>>(), size_of::<u32>());
    assert_eq!(size_of::<LayerContextId<'_>>(), size_of::<u32>());
    assert_eq!(size_of::<SourceOrderId>(), size_of::<u64>());
    assert!(size_of::<DeclarationRecord<'static, DeclarationPayload<'static>>>() <= 56);
    assert!(size_of::<DeclarationBlockRecord<'static, CssRulePayload<'static>>>() <= 28);
}

fn append_test_block<'ast>(
    compilation: &mut RadixCompilation<'ast, u8, u8, &'static str>,
    root: RuleListId<'ast>,
    key: EffectiveKeyId<'ast>,
    rule_payload: u8,
    declarations: &[u8],
) -> (RuleId<'ast, u8>, DeclarationBlockId<'ast, u8>) {
    let rule = compilation.append_rule(root, rule_payload).unwrap();
    let block = compilation
        .append_declaration_block(DeclarationBlockOwner::Rule(rule), key)
        .unwrap();
    for &payload in declarations {
        compilation
            .append_declaration(block, payload, false)
            .unwrap();
    }
    (rule, block)
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
        .append_declaration_block(DeclarationBlockOwner::<&str>::Rule(nested), key)
        .unwrap();
    let declaration = compilation
        .append_declaration(block, "color:red", false)
        .unwrap();

    assert_eq!(outer.index(), 0);
    assert_eq!(nested.index(), 1);
    assert_eq!(following.index(), 2);
    assert_eq!(block.index(), 0);
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
        DeclarationBlockOwner::<&str>::Rule(nested)
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
fn direct_declaration_endpoints_iterators_and_links_agree() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let (_, empty) = append_test_block(&mut compilation, root, key, 0, &[]);
    let (_, block) = append_test_block(&mut compilation, root, key, 1, &[10, 20, 30]);

    let empty_record = compilation.declaration_block(empty).unwrap();
    assert_eq!(empty_record.first_declaration, None);
    assert_eq!(empty_record.last_declaration, None);
    assert_eq!(empty_record.declaration_count(), 0);

    let ids = compilation
        .declaration_ids_in_block(block)
        .unwrap()
        .collect::<std::vec::Vec<_>>();
    let occurrences = compilation
        .declaration_occurrences_in_block(block)
        .unwrap()
        .map(|(occurrence, record)| (occurrence.declaration(), *record.payload()))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(occurrences, [(ids[0], 10), (ids[1], 20), (ids[2], 30)]);
    assert_eq!(
        compilation.declarations.get(ids[0]).next_in_block,
        Some(ids[1])
    );
    assert_eq!(
        compilation.declarations.get(ids[1]).next_in_block,
        Some(ids[2])
    );
    assert_eq!(compilation.declarations.get(ids[2]).next_in_block, None);
    let block_record = compilation.declaration_block(block).unwrap();
    assert_eq!(block_record.first_declaration, Some(ids[0]));
    assert_eq!(block_record.last_declaration, Some(ids[2]));
    assert_eq!(block_record.declaration_count(), 3);
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn occurrence_tokens_preserve_retained_members_and_reject_retired_blocks() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let (left, left_block) = append_test_block(&mut compilation, root, key, 0, &[10]);
    let (right, right_block) = append_test_block(&mut compilation, root, key, 1, &[20]);
    let left_occurrence = compilation
        .declaration_occurrences_in_block(left_block)
        .unwrap()
        .next()
        .unwrap()
        .0;
    let right_occurrence = compilation
        .declaration_occurrences_in_block(right_block)
        .unwrap()
        .next()
        .unwrap()
        .0;

    assert_eq!(
        compilation.replace_declaration(right_block, left_occurrence.declaration(), 11),
        Err(MutationError::UnknownDeclaration(
            left_occurrence.declaration()
        ))
    );
    compilation
        .merge_adjacent_rule_declaration_blocks(left, right)
        .unwrap();
    assert_eq!(
        compilation.validated_declaration_mut(left_occurrence),
        Err(MutationError::UnknownDeclarationBlock(left_block))
    );
    *compilation
        .validated_declaration_mut(right_occurrence)
        .unwrap()
        .0 = 21;
    assert_eq!(
        compilation
            .declarations_in_block(right_block)
            .unwrap()
            .map(|record| *record.payload())
            .collect::<std::vec::Vec<_>>(),
        [10, 21]
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
fn validation_rejects_non_monotonic_source_order_ids() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, (), ()>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let first = compilation.append_rule(root, 1).unwrap();
    let second = compilation.append_rule(root, 2).unwrap();

    let first_order = compilation.rule(first).unwrap().source_order_id();
    compilation.rule_mut(second).unwrap().source_order_id = first_order;

    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::InvalidSourceOrder {
            previous: first,
            next: second,
        })
    );
}

#[test]
fn declaration_block_source_iteration_skips_an_unresolved_block() {
    let allocator = Allocator::new();
    let mut foreign = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let foreign_root = foreign.stylesheet().root_rules();
    let foreign_key = foreign.append_effective_key("foreign").unwrap();
    append_test_block(&mut foreign, foreign_root, foreign_key, 0, &[0]);
    let (_, unresolved) = append_test_block(&mut foreign, foreign_root, foreign_key, 1, &[1]);

    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("target").unwrap();
    let broken_rule = compilation.append_rule(root, 0).unwrap();
    let (_, valid_block) = append_test_block(&mut compilation, root, key, 1, &[1]);
    compilation.rule_mut(broken_rule).unwrap().declaration_block = Some(unresolved);

    assert_eq!(
        compilation
            .declaration_blocks_in_source_order()
            .map(|(block, _)| block)
            .collect::<std::vec::Vec<_>>(),
        [valid_block]
    );
}

#[test]
fn validation_rejects_a_declaration_cycle() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let (_, block) = append_test_block(&mut compilation, root, key, 0, &[0]);
    let declaration = compilation
        .declaration_block(block)
        .unwrap()
        .first_declaration
        .unwrap();
    compilation.declarations.get_mut(declaration).next_in_block = Some(declaration);

    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::DeclarationCycle { block, declaration })
    );
}

#[test]
fn validation_rejects_a_dangling_declaration_link() {
    let allocator = Allocator::new();
    let mut foreign = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let foreign_root = foreign.stylesheet().root_rules();
    let foreign_key = foreign.append_effective_key("foreign").unwrap();
    append_test_block(&mut foreign, foreign_root, foreign_key, 0, &[0]);
    let (_, foreign_block) = append_test_block(&mut foreign, foreign_root, foreign_key, 1, &[1]);
    let dangling = foreign
        .declaration_block(foreign_block)
        .unwrap()
        .first_declaration
        .unwrap();

    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("target").unwrap();
    let (_, block) = append_test_block(&mut compilation, root, key, 0, &[0]);
    let declaration = compilation
        .declaration_block(block)
        .unwrap()
        .first_declaration
        .unwrap();
    compilation.declarations.get_mut(declaration).next_in_block = Some(dangling);

    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::InvalidDeclarationReference {
            block,
            declaration: dangling,
        })
    );
}

#[test]
fn validation_rejects_declaration_count_and_endpoint_mismatches() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let (_, block) = append_test_block(&mut compilation, root, key, 0, &[0]);

    compilation
        .declaration_block_mut(block)
        .unwrap()
        .declaration_count = 2;
    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::DeclarationCountMismatch {
            block,
            expected: 2,
            actual: 1,
        })
    );

    let block_record = compilation.declaration_block_mut(block).unwrap();
    block_record.declaration_count = 1;
    block_record.last_declaration = None;
    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::InvalidDeclarationEndpoints { block })
    );
}

#[test]
fn validation_rejects_last_mismatch_and_duplicate_declaration_ownership() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let (_, left_block) = append_test_block(&mut compilation, root, key, 0, &[0, 1]);
    let (_, right_block) = append_test_block(&mut compilation, root, key, 1, &[]);
    let left_first = compilation
        .declaration_block(left_block)
        .unwrap()
        .first_declaration
        .unwrap();

    compilation
        .declaration_block_mut(left_block)
        .unwrap()
        .last_declaration = Some(left_first);
    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::DeclarationLastMismatch {
            block: left_block,
            expected: Some(left_first),
            actual: compilation.declarations.get(left_first).next_in_block,
        })
    );

    let left_last = compilation
        .declarations
        .get(left_first)
        .next_in_block
        .unwrap();
    compilation
        .declaration_block_mut(left_block)
        .unwrap()
        .last_declaration = Some(left_last);
    let right = compilation.declaration_block_mut(right_block).unwrap();
    right.first_declaration = Some(left_first);
    right.last_declaration = Some(left_last);
    right.declaration_count = 2;
    assert_eq!(
        compilation.validate_ast(),
        Err(ValidationError::DuplicateDeclarationOwner {
            declaration: left_first,
            first: left_block,
            second: right_block,
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
        Err(MutationError::<()>::ChildListAlreadyExists(parent))
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
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    compilation
        .append_declaration(left_block, 10, false)
        .unwrap();
    let right = compilation.append_rule(root, 2).unwrap();
    let right_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(right), key)
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
fn synthesized_rule_and_block_keep_dense_ids_with_appended_declarations() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("shared").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    compilation
        .append_declaration(left_block, 10, false)
        .unwrap();
    let right = compilation.append_rule(root, 2).unwrap();
    let right_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(right), key)
        .unwrap();
    compilation
        .append_declaration(right_block, 20, false)
        .unwrap();

    let inserted_rule = compilation.insert_rule_after(left, 3).unwrap();
    let inserted_block = compilation
        .insert_declaration_block(inserted_rule, key)
        .unwrap();
    compilation
        .append_declaration(inserted_block, 30, false)
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
            DeclarationBlockOwner::<u8>::Rule(left),
            DeclarationBlockOwner::<u8>::Rule(inserted_rule),
            DeclarationBlockOwner::<u8>::Rule(right),
        ]
    );
    assert!(inserted_rule > right);
    assert!(
        compilation.rule(left).unwrap().source_order_id()
            < compilation.rule(inserted_rule).unwrap().source_order_id()
    );
    assert!(
        compilation.rule(inserted_rule).unwrap().source_order_id()
            < compilation.rule(right).unwrap().source_order_id()
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}

#[test]
fn repeated_local_insertions_relabel_source_order_without_remapping_dense_ids() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, (), ()>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let left = compilation.append_rule(root, 0).unwrap();
    let right = compilation.append_rule(root, u8::MAX).unwrap();

    let inserted = (1..=40)
        .map(|payload| compilation.insert_rule_after(left, payload).unwrap())
        .collect::<std::vec::Vec<_>>();

    assert!(inserted.iter().all(|id| id.index() > right.index()));
    assert_eq!(compilation.validate_ast(), Ok(()));
    let source_order = compilation
        .rules_in_source_order()
        .map(|(_, rule)| rule.source_order_id())
        .collect::<std::vec::Vec<_>>();
    assert!(source_order.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn noncontiguous_small_merge_links_declarations_without_copying_payloads() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let first = compilation
        .append_declaration(left_block, 10, false)
        .unwrap();
    let following = compilation.append_rule(root, 2).unwrap();
    let following_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    compilation
        .append_declaration(following_block, 20, false)
        .unwrap();

    let inserted = compilation.insert_rule_after(left, 3).unwrap();
    let inserted_block = compilation.insert_declaration_block(inserted, key).unwrap();
    let second = compilation
        .append_declaration(inserted_block, 30, false)
        .unwrap();
    compilation
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();

    let mut ids = compilation
        .declaration_ids_in_block(inserted_block)
        .unwrap();
    assert_eq!(ids.len(), 2);
    assert_eq!(ids.next(), Some(first));
    assert_eq!(ids.len(), 1);
    assert_eq!(ids.next(), Some(second));
    assert_eq!(ids.len(), 0);
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
fn noncontiguous_large_merge_links_declarations_without_copying_payloads() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    for value in [10, 11, 12] {
        compilation
            .append_declaration(left_block, value, false)
            .unwrap();
    }
    let following = compilation.append_rule(root, 2).unwrap();
    let following_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    compilation
        .append_declaration(following_block, 20, false)
        .unwrap();

    let inserted = compilation.insert_rule_after(left, 3).unwrap();
    let inserted_block = compilation.insert_declaration_block(inserted, key).unwrap();
    for value in [30, 31, 32] {
        compilation
            .append_declaration(inserted_block, value, false)
            .unwrap();
    }
    compilation
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();

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
fn transformed_append_extends_the_noncontiguous_direct_chain() {
    let allocator = Allocator::new();
    let mut compilation = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let key = compilation.append_effective_key("same").unwrap();
    let left = compilation.append_rule(root, 1).unwrap();
    let left_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    for value in [10, 11] {
        compilation
            .append_declaration(left_block, value, false)
            .unwrap();
    }
    let following = compilation.append_rule(root, 2).unwrap();
    let following_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    compilation
        .append_declaration(following_block, 20, false)
        .unwrap();
    let inserted = compilation.insert_rule_after(left, 3).unwrap();
    let inserted_block = compilation.insert_declaration_block(inserted, key).unwrap();
    for value in [30, 31] {
        compilation
            .append_declaration(inserted_block, value, false)
            .unwrap();
    }
    compilation
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();
    let appended = compilation
        .append_transformed_declaration(inserted_block, 32, false)
        .unwrap();
    assert_eq!(
        compilation
            .declaration_block(inserted_block)
            .unwrap()
            .last_declaration,
        Some(appended)
    );
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
fn streaming_declaration_mutation_preserves_direct_chain_order() {
    let allocator = Allocator::new();

    let mut range = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = range.stylesheet().root_rules();
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

    let mut small_split = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = small_split.stylesheet().root_rules();
    let key = small_split.append_effective_key("small-split").unwrap();
    let left = small_split.append_rule(root, 0).unwrap();
    let left_block = small_split
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let first = small_split
        .append_declaration(left_block, 1, false)
        .unwrap();
    let following = small_split.append_rule(root, 1).unwrap();
    let following_block = small_split
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    small_split
        .append_declaration(following_block, 2, false)
        .unwrap();
    let inserted = small_split.insert_rule_after(left, 2).unwrap();
    let inserted_block = small_split.insert_declaration_block(inserted, key).unwrap();
    let second = small_split
        .append_declaration(inserted_block, 3, false)
        .unwrap();
    small_split
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();
    visited.clear();
    small_split
        .for_each_declaration_mut(inserted_block, |id, record| {
            visited.push(id);
            *record.payload_mut() += 10;
        })
        .unwrap();
    assert_eq!(visited, [first, second]);
    assert_eq!(
        small_split
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|record| *record.payload())
            .collect::<std::vec::Vec<_>>(),
        [11, 13]
    );

    let mut large_split = RadixCompilation::<u8, u8, &'static str>::new_in(&allocator);
    let root = large_split.stylesheet().root_rules();
    let key = large_split.append_effective_key("large-split").unwrap();
    let left = large_split.append_rule(root, 0).unwrap();
    let left_block = large_split
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let mut expected = std::vec::Vec::new();
    for value in [1, 2, 3] {
        expected.push(
            large_split
                .append_declaration(left_block, value, false)
                .unwrap(),
        );
    }
    let following = large_split.append_rule(root, 1).unwrap();
    let following_block = large_split
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    large_split
        .append_declaration(following_block, 4, false)
        .unwrap();
    let inserted = large_split.insert_rule_after(left, 2).unwrap();
    let inserted_block = large_split.insert_declaration_block(inserted, key).unwrap();
    for value in [5, 6, 7] {
        expected.push(
            large_split
                .append_declaration(inserted_block, value, false)
                .unwrap(),
        );
    }
    large_split
        .merge_adjacent_rule_declaration_blocks(left, inserted)
        .unwrap();
    visited.clear();
    large_split
        .for_each_declaration_mut(inserted_block, |id, record| {
            visited.push(id);
            *record.payload_mut() += 10;
        })
        .unwrap();
    assert_eq!(visited, expected);
    assert_eq!(
        large_split
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
    let mut compilation = RadixCompilation::<(), (), ()>::new_in(&allocator);
    let root = compilation.stylesheet().root_rules();
    let owner = compilation.append_rule(root, ()).unwrap();
    let key = compilation.append_effective_key(()).unwrap();
    compilation
        .append_declaration_block(DeclarationBlockOwner::<()>::Rule(owner), key)
        .unwrap();

    assert_eq!(
        compilation.append_declaration_block(DeclarationBlockOwner::<()>::Rule(owner), key),
        Err(MutationError::<()>::DeclarationBlockAlreadyExists(owner))
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
        .append_declaration_block(DeclarationBlockOwner::<()>::Rule(outer), key)
        .unwrap();
    compilation
        .append_declaration(outer_block, "before", false)
        .unwrap();
    let nested_block = compilation
        .append_declaration_block(DeclarationBlockOwner::<()>::Rule(nested), key)
        .unwrap();
    compilation
        .append_declaration(nested_block, "nested", false)
        .unwrap();

    assert_eq!(
        compilation.append_declaration(outer_block, "after", false),
        Err(MutationError::<()>::NonContiguousDeclarationRange(
            outer_block
        ))
    );
    assert_eq!(compilation.validate_ast(), Ok(()));
}
