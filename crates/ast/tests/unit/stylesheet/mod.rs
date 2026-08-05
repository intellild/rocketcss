use std::mem::size_of;

use super::*;

fn direct_context<'ast, R: std::fmt::Debug + Unpin, D: Unpin, K>(
    stylesheet: &StyleSheet<'ast, R, D, K>,
    rule: RuleId<R>,
) -> DirectRuleContext<R> {
    match stylesheet.rule(rule).unwrap().parent() {
        Some(parent) => stylesheet
            .nested_rule_contexts(parent)
            .unwrap()
            .find(|context| context.rule() == rule)
            .unwrap(),
        None => stylesheet
            .root_rule_contexts()
            .find(|context| context.rule() == rule)
            .unwrap(),
    }
}

fn direct_edge<'ast, R: std::fmt::Debug + Unpin, D: Unpin, K>(
    stylesheet: &StyleSheet<'ast, R, D, K>,
    left: RuleId<R>,
    right: RuleId<R>,
) -> DirectRuleEdge<R> {
    match stylesheet.rule(left).unwrap().parent() {
        Some(parent) => stylesheet
            .nested_rule_edges(parent)
            .unwrap()
            .find(|edge| edge.left() == left && edge.right() == right)
            .unwrap(),
        None => stylesheet
            .root_rule_edges()
            .find(|edge| edge.left() == left && edge.right() == right)
            .unwrap(),
    }
}

fn declaration_append<'ast, R: Unpin, D: Unpin, K>(
    stylesheet: &StyleSheet<'ast, R, D, K>,
    block: DeclarationBlockId<R>,
) -> DeclarationAppendContext<R> {
    stylesheet
        .declaration_block_positions()
        .find(|position| position.block() == block)
        .unwrap()
        .append_context()
}

#[test]
fn typed_ids_keep_compact_optional_layout() {
    assert_eq!(size_of::<RuleId<&str>>(), size_of::<u32>());
    assert_eq!(size_of::<Option<RuleId<&str>>>(), size_of::<u32>());
    assert_eq!(size_of::<DeclarationId>(), size_of::<u32>());
    assert_eq!(size_of::<Option<DeclarationId>>(), size_of::<u32>());
    assert_eq!(size_of::<DeclarationList>(), size_of::<[u32; 3]>());
    assert_eq!(size_of::<DeclarationBlockId<&str>>(), size_of::<u32>());
    assert_eq!(
        size_of::<Option<DeclarationBlockId<&str>>>(),
        size_of::<u32>()
    );
    assert_eq!(size_of::<DirectRuleContext<&str>>(), size_of::<[u32; 3]>());
    assert_eq!(size_of::<DirectRuleEdge<&str>>(), size_of::<[u32; 5]>());
    assert_eq!(size_of::<RuleMutationDelta<&str>>(), size_of::<[u32; 20]>());
}

#[test]
fn direct_rule_positions_cover_empty_singleton_and_multiple_lists() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, (), ()>::new_in(&allocator);

    assert_eq!(stylesheet.root_rule_positions().count(), 0);

    let first = stylesheet.append_rule(None, 1).unwrap();
    let singleton = stylesheet
        .root_rule_positions()
        .map(|position| (position.context().rule(), position.incoming_edge()))
        .collect::<std::vec::Vec<_>>();
    assert_eq!(singleton, [(first, None)]);

    let second = stylesheet.append_rule(None, 2).unwrap();
    let positions = stylesheet
        .root_rule_positions()
        .map(|position| {
            (
                position.context().rule(),
                position
                    .incoming_edge()
                    .map(|edge| (edge.left(), edge.right())),
            )
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(positions, [(first, None), (second, Some((first, second)))]);
}

#[test]
fn lexical_order_and_direct_topology_are_independent() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<&str, &str, &str>::new_in(&allocator);
    let root = None;

    let outer = stylesheet.append_rule(root, "outer").unwrap();
    let nested = stylesheet.append_rule(Some(outer), "nested").unwrap();
    let following = stylesheet.append_rule(root, "following").unwrap();
    let key = stylesheet.append_effective_key("nested@root").unwrap();
    let block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<&str>::Rule(nested), key)
        .unwrap();
    let declaration = stylesheet
        .append_authored_declaration(block, "color:red", false)
        .unwrap();

    assert_eq!(outer.primary_index(), 0);
    assert_eq!(nested.primary_index(), 1);
    assert_eq!(following.primary_index(), 2);
    assert_eq!(block.primary_index(), 0);
    assert_eq!(declaration.primary_index(), 0);
    assert_eq!(
        stylesheet
            .rules_in_source_order()
            .map(|(_, rule)| *rule.payload())
            .collect::<std::vec::Vec<_>>(),
        ["outer", "nested", "following"]
    );
    assert_eq!(
        stylesheet
            .root_rules()
            .map(|(_, rule)| *rule.payload())
            .collect::<std::vec::Vec<_>>(),
        ["outer", "following"]
    );
    assert_eq!(
        stylesheet.root_rule_ids().collect::<std::vec::Vec<_>>(),
        [outer, following]
    );
    assert_eq!(
        stylesheet
            .nested_rule_ids(outer)
            .unwrap()
            .collect::<std::vec::Vec<_>>(),
        [nested]
    );
    assert_eq!(
        stylesheet
            .rule_tree_events()
            .map(|event| (event.rule(), event.parent(), event.has_children()))
            .collect::<std::vec::Vec<_>>(),
        [
            (outer, None, true),
            (nested, Some(outer), false),
            (following, None, false),
        ]
    );
    assert_eq!(
        stylesheet
            .root_rule_edges()
            .map(|edge| (edge.left(), edge.right()))
            .collect::<std::vec::Vec<_>>(),
        [(outer, following)]
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
fn direct_rule_edges_stay_in_one_parent_and_skip_descendants_and_tombstones() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<&str, (), ()>::new_in(&allocator);
    let parent = stylesheet.append_rule(None, "parent").unwrap();
    let first = stylesheet.append_rule(Some(parent), "first").unwrap();
    stylesheet.append_rule(Some(first), "grandchild").unwrap();
    let retired = stylesheet.append_rule(Some(parent), "retired").unwrap();
    let last = stylesheet.append_rule(Some(parent), "last").unwrap();
    let following = stylesheet.append_rule(None, "following").unwrap();

    let retired_context = direct_context(&stylesheet, retired);
    stylesheet.retire_rule(retired_context).unwrap();

    assert_eq!(
        stylesheet
            .root_rule_edges()
            .map(|edge| (edge.left(), edge.right()))
            .collect::<std::vec::Vec<_>>(),
        [(parent, following)]
    );
    assert_eq!(
        stylesheet
            .nested_rule_edges(parent)
            .unwrap()
            .map(|edge| (edge.left(), edge.right()))
            .collect::<std::vec::Vec<_>>(),
        [(first, last)]
    );
    assert!(
        stylesheet
            .nested_rule_edges(first)
            .unwrap()
            .next()
            .is_none()
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn local_rule_mutations_invalidate_old_edges_and_publish_exact_replacements() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, (), ()>::new_in(&allocator);
    let key = stylesheet.append_effective_key(()).unwrap();
    let left = stylesheet.append_rule(None, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(left), key)
        .unwrap();
    let right = stylesheet.append_rule(None, 2).unwrap();
    stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(right), key)
        .unwrap();
    let next = stylesheet.append_rule(None, 3).unwrap();
    stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(next), key)
        .unwrap();

    let original = direct_edge(&stylesheet, left, right);
    let left_append = declaration_append(&stylesheet, left_block);
    let inserted = stylesheet
        .insert_rule_with_declaration_block_after(original, left_append, 4, key, 0)
        .unwrap();
    let inserted_rule = inserted.rule.id;
    assert!(!stylesheet.is_valid_direct_rule_edge(original));
    assert_eq!(
        inserted
            .delta
            .edges()
            .map(|edge| (edge.left(), edge.right()))
            .collect::<std::vec::Vec<_>>(),
        [(left, inserted_rule), (inserted_rule, right)]
    );
    assert!(
        inserted
            .delta
            .edges()
            .all(|edge| stylesheet.is_valid_direct_rule_edge(edge))
    );

    let merge_edge = inserted
        .delta
        .edges()
        .find(|edge| edge.left() == inserted_rule && edge.right() == right)
        .unwrap();
    let merged = stylesheet
        .merge_adjacent_rule_declaration_blocks(merge_edge)
        .unwrap();
    assert_eq!(
        merged
            .delta
            .edges()
            .map(|edge| (edge.left(), edge.right()))
            .collect::<std::vec::Vec<_>>(),
        [(left, right), (right, next)]
    );

    let right_context = merged
        .delta
        .edges()
        .find(|edge| edge.left() == left && edge.right() == right)
        .unwrap()
        .right_context();
    let retired = stylesheet.retire_rule(right_context).unwrap();
    assert_eq!(
        retired
            .delta
            .edges()
            .map(|edge| (edge.left(), edge.right()))
            .collect::<std::vec::Vec<_>>(),
        [(left, next)]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn local_relabel_repairs_retained_edge_contexts_without_rewalking_the_list() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, (), ()>::new_in(&allocator);
    let left = stylesheet.append_rule(None, 0).unwrap();
    let right = stylesheet.append_rule(None, u8::MAX).unwrap();

    let left_context = direct_context(&stylesheet, left);
    let first = stylesheet
        .insert_rule_after(left_context, 1)
        .unwrap()
        .rule
        .id;
    let left_context = direct_context(&stylesheet, left);
    stylesheet.insert_rule_after(left_context, 2).unwrap();
    let mut retained = direct_edge(&stylesheet, first, right);
    let mut relabeled = false;

    for payload in 3..32 {
        let left_context = direct_context(&stylesheet, left);
        let inserted = stylesheet.insert_rule_after(left_context, payload).unwrap();
        if !inserted.rule.remaps.is_empty() {
            relabeled = true;
            retained = retained.remapped(&inserted.rule.remaps);
        }
        assert!(stylesheet.is_valid_direct_rule_edge(retained));
        assert_eq!(stylesheet.validate_ast(), Ok(()));
    }

    assert!(relabeled);
}

#[test]
fn validation_rejects_an_invalid_descendant_count() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, (), ()>::new_in(&allocator);
    let root = None;
    let first = stylesheet.append_rule(root, 1).unwrap();
    stylesheet.append_rule(Some(first), 2).unwrap();
    stylesheet.append_rule(root, 3).unwrap();

    stylesheet.rule_mut(first).unwrap().descendant_count = 3;

    assert_eq!(
        stylesheet.validate_ast(),
        Err(ValidationError::<u8>::DescendantCountMismatch {
            rule: first,
            expected: 2,
            actual: 3,
        })
    );
}

#[test]
fn validation_rejects_an_invalid_cached_direct_rule_count() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, (), ()>::new_in(&allocator);
    let parent = stylesheet.append_rule(None, 1).unwrap();
    stylesheet.append_rule(Some(parent), 2).unwrap();

    stylesheet.rule_mut(parent).unwrap().nested_rule_count = 2;

    assert_eq!(
        stylesheet.validate_ast(),
        Err(ValidationError::<u8>::NestedRuleCountMismatch {
            rule: parent,
            expected: 1,
            actual: 2,
        })
    );
}

#[test]
fn cached_rule_counts_track_physical_descendants_and_live_direct_children() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<(), (), ()>::new_in(&allocator);
    let root = None;
    let parent = stylesheet.append_rule(root, ()).unwrap();
    let child = stylesheet.append_rule(Some(parent), ()).unwrap();
    stylesheet.append_rule(Some(child), ()).unwrap();

    assert_eq!(stylesheet.rule(parent).unwrap().descendant_count, 2);
    assert_eq!(stylesheet.rule(parent).unwrap().nested_rule_count, 1);
    assert_eq!(stylesheet.rule(child).unwrap().descendant_count, 1);
    assert_eq!(stylesheet.rule(child).unwrap().nested_rule_count, 1);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn insertion_after_a_nested_subtree_updates_every_ancestor_span() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<&str, (), ()>::new_in(&allocator);
    let parent = stylesheet.append_rule(None, "parent").unwrap();
    let child = stylesheet.append_rule(Some(parent), "child").unwrap();
    let grandchild = stylesheet.append_rule(Some(child), "grandchild").unwrap();
    let following = stylesheet.append_rule(None, "following").unwrap();

    let parent_context = direct_context(&stylesheet, parent);
    let child_context = direct_context(&stylesheet, child);
    let inserted = stylesheet
        .insert_rule_after(child_context, "inserted")
        .unwrap()
        .rule
        .id;

    assert_eq!(
        stylesheet
            .nested_rules(parent)
            .unwrap()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [child, inserted]
    );
    assert_eq!(
        stylesheet
            .rules_in_source_order()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [parent, child, grandchild, inserted, following]
    );
    assert_eq!(stylesheet.rule(parent).unwrap().descendant_count, 3);
    assert_eq!(stylesheet.rule(parent).unwrap().nested_rule_count, 2);
    assert_eq!(stylesheet.rule(child).unwrap().descendant_count, 1);
    assert_eq!(stylesheet.rule(child).unwrap().nested_rule_count, 1);
    assert!(matches!(
        stylesheet.rule_edges_at_context(parent_context),
        Err(MutationError::InvalidRuleTopology(id)) if id == parent
    ));
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn compact_position_rehydrates_after_an_unrelated_subtree_boundary_change() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<&str, (), ()>::new_in(&allocator);
    let parent = stylesheet.append_rule(None, "parent").unwrap();
    let child = stylesheet.append_rule(Some(parent), "child").unwrap();
    let following = stylesheet.append_rule(None, "following").unwrap();
    let stale_following = direct_context(&stylesheet, following);

    stylesheet
        .insert_rule_after(direct_context(&stylesheet, child), "inserted")
        .unwrap();

    assert_eq!(
        stylesheet
            .rule_edges_at_context(stale_following)
            .unwrap()
            .edges()
            .map(|edge| (edge.left(), edge.right()))
            .collect::<std::vec::Vec<_>>(),
        [(parent, following)]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn retired_nested_tombstones_stay_in_the_span_but_not_semantic_traversal() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<&str, (), ()>::new_in(&allocator);
    let parent = stylesheet.append_rule(None, "parent").unwrap();
    let child = stylesheet.append_rule(Some(parent), "child").unwrap();
    let following = stylesheet.append_rule(None, "following").unwrap();

    let child_context = direct_context(&stylesheet, child);
    stylesheet.retire_rule(child_context).unwrap();
    assert!(!stylesheet.has_nested_rules(parent).unwrap());
    let parent_context = direct_context(&stylesheet, parent);
    stylesheet.retire_rule(parent_context).unwrap();

    assert_eq!(
        stylesheet
            .root_rules()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>(),
        [following]
    );
    assert_eq!(
        stylesheet
            .rule_tree_events()
            .map(|event| event.rule())
            .collect::<std::vec::Vec<_>>(),
        [following]
    );
    assert_eq!(stylesheet.rule(parent).unwrap().descendant_count, 1);
    assert_eq!(stylesheet.rule(parent).unwrap().nested_rule_count, 0);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn validation_rejects_a_wrong_preorder_parent() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<(), (), ()>::new_in(&allocator);
    let root = None;
    let parent = stylesheet.append_rule(root, ()).unwrap();
    let child = stylesheet.append_rule(Some(parent), ()).unwrap();
    let other = stylesheet.append_rule(root, ()).unwrap();

    stylesheet.rule_mut(child).unwrap().parent = Some(other);

    assert_eq!(
        stylesheet.validate_ast(),
        Err(ValidationError::<()>::RuleHasWrongParent {
            parent: Some(parent),
            rule: child,
        })
    );
}

#[test]
fn adjacent_equal_key_blocks_merge_without_a_previous_merged_chain() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = None;
    let key = stylesheet.append_effective_key("same").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(left_block, 10, false)
        .unwrap();
    let right = stylesheet.append_rule(root, 2).unwrap();
    let right_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(right), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(right_block, 20, false)
        .unwrap();

    let edge = direct_edge(&stylesheet, left, right);
    let merged = stylesheet
        .merge_adjacent_rule_declaration_blocks(edge)
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
    let root = None;
    let key = stylesheet.append_effective_key("shared").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(left_block, 10, false)
        .unwrap();
    let right = stylesheet.append_rule(root, 2).unwrap();
    let right_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(right), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(right_block, 20, false)
        .unwrap();

    let edge = direct_edge(&stylesheet, left, right);
    let left_append = declaration_append(&stylesheet, left_block);
    let inserted = stylesheet
        .insert_rule_with_declaration_block_after(edge, left_append, 3, key, 1)
        .unwrap();
    assert!(inserted.rule.remaps.is_empty());
    assert!(inserted.declaration_block.remaps.is_empty());
    stylesheet
        .insert_transformed_declarations_at_block_end(inserted.declaration_block.id, [(30, false)])
        .unwrap();

    assert_eq!(
        stylesheet
            .root_rules()
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
            DeclarationBlockOwner::<u8>::Rule(inserted.rule.id),
            DeclarationBlockOwner::<u8>::Rule(right),
        ]
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn synthesized_declaration_is_inserted_between_neighbor_block_ranges() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = None;
    let key = stylesheet.append_effective_key("same").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let first = stylesheet
        .append_authored_declaration(left_block, 10, false)
        .unwrap();
    let following = stylesheet.append_rule(root, 2).unwrap();
    let following_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(following_block, 20, false)
        .unwrap();

    let left_context = direct_context(&stylesheet, left);
    let inserted = stylesheet
        .insert_rule_after(left_context, 3)
        .unwrap()
        .rule
        .id;
    let inserted_block = stylesheet
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    stylesheet
        .insert_transformed_declarations_at_block_end(inserted_block, [(30, false)])
        .unwrap();
    let second = stylesheet
        .declaration_ids_in_block(inserted_block)
        .unwrap()
        .next()
        .unwrap();
    let following = stylesheet
        .declaration_ids_in_block(following_block)
        .unwrap()
        .next()
        .unwrap();
    assert!(
        stylesheet
            .declarations
            .ids()
            .collect::<std::vec::Vec<_>>()
            .windows(3)
            .any(|ids| ids == [first, second, following])
    );
    let edge = direct_edge(&stylesheet, left, inserted);
    stylesheet
        .merge_adjacent_rule_declaration_blocks(edge)
        .unwrap();

    assert_eq!(
        stylesheet
            .declaration_block(inserted_block)
            .unwrap()
            .declarations()
            .len(),
        2
    );
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
fn large_synthesized_merge_remains_one_radix_range() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = None;
    let key = stylesheet.append_effective_key("same").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    for value in [10, 11, 12] {
        stylesheet
            .append_authored_declaration(left_block, value, false)
            .unwrap();
    }
    let following = stylesheet.append_rule(root, 2).unwrap();
    let following_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(following_block, 20, false)
        .unwrap();

    let left_context = direct_context(&stylesheet, left);
    let inserted = stylesheet
        .insert_rule_after(left_context, 3)
        .unwrap()
        .rule
        .id;
    let inserted_block = stylesheet
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    stylesheet
        .insert_transformed_declarations_at_block_end(
            inserted_block,
            [(30, false), (31, false), (32, false)],
        )
        .unwrap();
    let edge = direct_edge(&stylesheet, left, inserted);
    stylesheet
        .merge_adjacent_rule_declaration_blocks(edge)
        .unwrap();

    assert_eq!(
        stylesheet
            .declaration_block(inserted_block)
            .unwrap()
            .declarations()
            .len(),
        6
    );
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
fn appending_another_transformed_batch_extends_the_same_range() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = None;
    let key = stylesheet.append_effective_key("same").unwrap();
    let left = stylesheet.append_rule(root, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    for value in [10, 11] {
        stylesheet
            .append_authored_declaration(left_block, value, false)
            .unwrap();
    }
    let following = stylesheet.append_rule(root, 2).unwrap();
    let following_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(following_block, 20, false)
        .unwrap();
    let left_context = direct_context(&stylesheet, left);
    let inserted = stylesheet
        .insert_rule_after(left_context, 3)
        .unwrap()
        .rule
        .id;
    let inserted_block = stylesheet
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    stylesheet
        .insert_transformed_declarations_at_block_end(inserted_block, [(30, false), (31, false)])
        .unwrap();
    let edge = direct_edge(&stylesheet, left, inserted);
    stylesheet
        .merge_adjacent_rule_declaration_blocks(edge)
        .unwrap();
    stylesheet
        .insert_transformed_declarations_at_block_end(inserted_block, [(32, false)])
        .unwrap();

    assert_eq!(
        stylesheet
            .declaration_block(inserted_block)
            .unwrap()
            .declarations()
            .len(),
        5
    );
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
fn streaming_declaration_mutation_preserves_radix_range_order() {
    let allocator = Allocator::new();

    let mut range = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = None;
    let key = range.append_effective_key("range").unwrap();
    let rule = range.append_rule(root, 0).unwrap();
    let block = range
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(rule), key)
        .unwrap();
    let range_ids = [
        range.append_authored_declaration(block, 1, false).unwrap(),
        range.append_authored_declaration(block, 2, false).unwrap(),
        range.append_authored_declaration(block, 3, false).unwrap(),
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

    let mut inserted_range = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = None;
    let key = inserted_range.append_effective_key("inserted").unwrap();
    let left = inserted_range.append_rule(root, 0).unwrap();
    let left_block = inserted_range
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let first = inserted_range
        .append_authored_declaration(left_block, 1, false)
        .unwrap();
    let following = inserted_range.append_rule(root, 1).unwrap();
    let following_block = inserted_range
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    inserted_range
        .append_authored_declaration(following_block, 2, false)
        .unwrap();
    let left_context = direct_context(&inserted_range, left);
    let inserted = inserted_range
        .insert_rule_after(left_context, 2)
        .unwrap()
        .rule
        .id;
    let inserted_block = inserted_range
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    inserted_range
        .insert_transformed_declarations_at_block_end(inserted_block, [(3, false)])
        .unwrap();
    let second = inserted_range
        .declaration_ids_in_block(inserted_block)
        .unwrap()
        .next()
        .unwrap();
    let edge = direct_edge(&inserted_range, left, inserted);
    inserted_range
        .merge_adjacent_rule_declaration_blocks(edge)
        .unwrap();
    visited.clear();
    inserted_range
        .for_each_declaration_mut(inserted_block, |id, record| {
            visited.push(id);
            *record.payload_mut() += 10;
        })
        .unwrap();
    assert_eq!(visited, [first, second]);
    assert_eq!(
        inserted_range
            .declarations_in_block(inserted_block)
            .unwrap()
            .map(|record| *record.payload())
            .collect::<std::vec::Vec<_>>(),
        [11, 13]
    );

    let mut large_range = StyleSheet::<u8, u8, &'static str>::new_in(&allocator);
    let root = None;
    let key = large_range.append_effective_key("large").unwrap();
    let left = large_range.append_rule(root, 0).unwrap();
    let left_block = large_range
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(left), key)
        .unwrap();
    let mut expected = std::vec::Vec::new();
    for value in [1, 2, 3] {
        expected.push(
            large_range
                .append_authored_declaration(left_block, value, false)
                .unwrap(),
        );
    }
    let following = large_range.append_rule(root, 1).unwrap();
    let following_block = large_range
        .append_declaration_block(DeclarationBlockOwner::<u8>::Rule(following), key)
        .unwrap();
    large_range
        .append_authored_declaration(following_block, 4, false)
        .unwrap();
    let left_context = direct_context(&large_range, left);
    let inserted = large_range
        .insert_rule_after(left_context, 2)
        .unwrap()
        .rule
        .id;
    let inserted_block = large_range
        .insert_declaration_block_between(left_block, Some(following_block), inserted, key)
        .unwrap()
        .id;
    large_range
        .insert_transformed_declarations_at_block_end(
            inserted_block,
            [(5, false), (6, false), (7, false)],
        )
        .unwrap();
    expected.extend(
        large_range
            .declaration_ids_in_block(inserted_block)
            .unwrap(),
    );
    let edge = direct_edge(&large_range, left, inserted);
    large_range
        .merge_adjacent_rule_declaration_blocks(edge)
        .unwrap();
    visited.clear();
    large_range
        .for_each_declaration_mut(inserted_block, |id, record| {
            visited.push(id);
            *record.payload_mut() += 10;
        })
        .unwrap();
    assert_eq!(visited, expected);
    assert_eq!(
        large_range
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
    let root = None;
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
fn empty_declaration_list_initializes_from_its_first_authored_declaration() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<(), u8, ()>::new_in(&allocator);
    let rule = stylesheet.append_rule(None, ()).unwrap();
    let key = stylesheet.append_effective_key(()).unwrap();
    let block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(rule), key)
        .unwrap();

    assert!(
        stylesheet
            .declaration_block(block)
            .unwrap()
            .declarations()
            .is_empty()
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));

    let declaration = stylesheet
        .append_authored_declaration(block, 1, false)
        .unwrap();
    let declarations = stylesheet.declaration_block(block).unwrap().declarations();
    assert_eq!(declarations.len(), 1);
    assert_eq!(declarations.start_id(), declaration);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn merging_empty_declaration_ranges_uses_only_block_order_and_lengths() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, ()>::new_in(&allocator);
    let key = stylesheet.append_effective_key(()).unwrap();
    let left = stylesheet.append_rule(None, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(left), key)
        .unwrap();
    let right = stylesheet.append_rule(None, 2).unwrap();
    let right_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(right), key)
        .unwrap();
    let declaration = stylesheet
        .append_authored_declaration(right_block, 3, false)
        .unwrap();

    let edge = direct_edge(&stylesheet, left, right);
    stylesheet
        .merge_adjacent_rule_declaration_blocks(edge)
        .unwrap();

    assert!(
        stylesheet
            .declaration_block(left_block)
            .unwrap()
            .declarations()
            .is_empty()
    );
    let retained = stylesheet
        .declaration_block(right_block)
        .unwrap()
        .declarations();
    assert_eq!(retained.len(), 1);
    assert_eq!(retained.start_id(), declaration);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn declaration_block_positions_bridge_consecutive_empty_ranges_once() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, ()>::new_in(&allocator);
    let key = stylesheet.append_effective_key(()).unwrap();

    let left_rule = stylesheet.append_rule(None, 1).unwrap();
    let left = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(left_rule), key)
        .unwrap();
    let left_declaration = stylesheet
        .append_authored_declaration(left, 10, false)
        .unwrap();
    let first_empty_rule = stylesheet.append_rule(None, 2).unwrap();
    let first_empty = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(first_empty_rule), key)
        .unwrap();
    let second_empty_rule = stylesheet.append_rule(None, 3).unwrap();
    let second_empty = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(second_empty_rule), key)
        .unwrap();
    let right_rule = stylesheet.append_rule(None, 4).unwrap();
    let right = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(right_rule), key)
        .unwrap();
    let right_declaration = stylesheet
        .append_authored_declaration(right, 20, false)
        .unwrap();

    let positions = stylesheet
        .declaration_block_positions()
        .map(|position| {
            let append = position.append_context();
            (
                position.block,
                position.previous,
                position.next,
                append.after,
                append.before,
            )
        })
        .collect::<std::vec::Vec<_>>();
    assert_eq!(
        positions,
        [
            (
                left,
                None,
                Some(first_empty),
                Some(left_declaration),
                Some(right_declaration)
            ),
            (
                first_empty,
                Some(left),
                Some(second_empty),
                Some(left_declaration),
                Some(right_declaration),
            ),
            (
                second_empty,
                Some(first_empty),
                Some(right),
                Some(left_declaration),
                Some(right_declaration),
            ),
            (
                right,
                Some(second_empty),
                None,
                Some(right_declaration),
                None
            ),
        ]
    );
}

#[test]
fn declaration_mutation_results_refresh_append_contexts() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u8, ()>::new_in(&allocator);
    let key = stylesheet.append_effective_key(()).unwrap();
    let rule = stylesheet.append_rule(None, 1).unwrap();
    let block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(rule), key)
        .unwrap();
    let first = stylesheet
        .append_authored_declaration(block, 10, false)
        .unwrap();
    let second = stylesheet
        .append_authored_declaration(block, 20, false)
        .unwrap();
    let original = declaration_append(&stylesheet, block);

    let replaced = stylesheet
        .replace_declaration_with_context(original, first, 11)
        .unwrap();
    assert_eq!(replaced.previous, 10);
    assert!(matches!(
        stylesheet.replace_declaration_with_context(original, second, 21),
        Err(MutationError::NonContiguousDeclarationRange(id)) if id == block
    ));
    let replaced = stylesheet
        .replace_declaration_with_context(replaced.declaration_append, second, 21)
        .unwrap();
    assert_eq!(replaced.previous, 20);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn transformed_range_capacity_failure_does_not_partially_mutate_the_ast() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<u8, u16, ()>::new_in(&allocator);
    let key = stylesheet.append_effective_key(()).unwrap();
    let left = stylesheet.append_rule(None, 1).unwrap();
    let left_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(left), key)
        .unwrap();
    let first = stylesheet
        .append_authored_declaration(left_block, 0, false)
        .unwrap();
    let right = stylesheet.append_rule(None, 2).unwrap();
    let right_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::Rule(right), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(right_block, u16::MAX, false)
        .unwrap();
    let mut sibling_range = DeclarationList::empty();
    for sibling_key in 1..=1023 {
        let id = stylesheet.declarations.insert_sibling(
            first,
            sibling_key,
            DeclarationRecord {
                payload: sibling_key,
                important: false,
            },
        );
        if sibling_range.is_empty() {
            sibling_range.initialize(id);
        } else {
            sibling_range.append(id);
        }
    }
    stylesheet
        .declaration_block_mut(left_block)
        .unwrap()
        .declarations
        .extend(sibling_range);
    assert_eq!(stylesheet.validate_ast(), Ok(()));
    let before = stylesheet
        .declaration_block(left_block)
        .unwrap()
        .declarations();

    assert_eq!(
        stylesheet.insert_transformed_declarations_at_block_end(left_block, [(42, false)]),
        Err(MutationError::DeclarationCapacityExhausted)
    );
    assert_eq!(
        stylesheet
            .declaration_block(left_block)
            .unwrap()
            .declarations(),
        before
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}

#[test]
fn a_declaration_range_cannot_cross_a_nested_allocation() {
    let allocator = Allocator::new();
    let mut stylesheet = StyleSheet::<(), &str, ()>::new_in(&allocator);
    let root = None;
    let outer = stylesheet.append_rule(root, ()).unwrap();
    let nested = stylesheet.append_rule(root, ()).unwrap();
    let key = stylesheet.append_effective_key(()).unwrap();
    let outer_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<()>::Rule(outer), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(outer_block, "before", false)
        .unwrap();
    let nested_block = stylesheet
        .append_declaration_block(DeclarationBlockOwner::<()>::Rule(nested), key)
        .unwrap();
    stylesheet
        .append_authored_declaration(nested_block, "nested", false)
        .unwrap();

    assert_eq!(
        stylesheet.append_authored_declaration(outer_block, "after", false),
        Err(MutationError::<()>::NonContiguousDeclarationRange(
            outer_block
        ))
    );
    assert_eq!(stylesheet.validate_ast(), Ok(()));
}
