use super::{assert_minifies_idempotently, assert_minifies_same_ast_twice};
use rocketcss_allocator::Allocator;
use rocketcss_ast::CssRule;
use rocketcss_nano::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};

// Review finding P0: S1 must suppress the retired left rule in real AST
// emission while the right rule emits the previous_merged chain exactly once.
#[test]
fn s1_emits_a_retired_left_rule_exactly_once() {
    assert_minifies_idempotently("a{x:1}a{y:2}", "a{x:1;y:2}");
}

// Review finding P0: synthesized AST nodes must outlive nano's scratch
// allocator. Serializing after minify returns exercises that lifetime boundary.
#[test]
#[ignore = "S3 synthesis with an explicit AST allocator is not implemented"]
fn synthesized_rules_survive_the_minify_scratch_allocator() {
    assert_minifies_idempotently("a{color:red}b{color:red}", "a,b{color:red}");
}

// Review finding P1: non-Clone declarations must be taken into the shared
// block before endpoint copies are tombstoned.
#[test]
#[ignore = "owned transfer of common non-Clone declarations is not implemented"]
fn transfers_a_non_clone_custom_declaration_into_the_shared_rule() {
    assert_minifies_idempotently(
        "a{--theme:var(--x,red)}b{--theme:var(--x,red)}",
        "a,b{--theme:var(--x,red)}",
    );
}

// Review finding P1: a second minify of the same AST must bootstrap the full
// persisted previous_merged chain rather than registering only its tail.
#[test]
fn imports_an_existing_previous_merged_chain_on_a_second_minify() {
    assert_minifies_same_ast_twice(
        "a{width:1px}a{height:2px}a{opacity:.5}",
        "a{width:1px;height:2px;opacity:.5}",
    );
}

// Review finding P1: inserting the first synthesized rule must not invalidate
// stable identities used by the overlapping second candidate.
#[test]
#[ignore = "stable RuleId handling across synthesized AST insertion is not implemented"]
fn keeps_overlapping_candidate_rule_ids_stable_across_insertion() {
    assert_minifies_idempotently("a{x:1}b{x:1;y:2}c{y:2}", "a,b{x:1}b,c{y:2}");
}

// Review finding P1: histories and sequences may hold the same opaque block
// handle, but scoped mutation must still invalidate every dependent candidate.
#[test]
#[ignore = "scoped unique access to shared declaration-block handles is not implemented"]
fn mutates_a_shared_block_handle_without_leaving_a_live_alias() {
    assert_minifies_same_ast_twice("a{x:1}a{y:2}b{y:2}a{x:3}", "a,b{y:2}a{x:3}");
}

// Review finding P1: until selector-arm/declaration spans exist, a synthesized
// rule at minimum needs a non-dummy combined rule-level origin.
#[test]
#[ignore = "the synthesized rule source-origin policy is not implemented"]
fn assigns_a_combined_source_span_to_a_synthesized_rule() {
    let source = "a{color:red}b{color:red}";
    let allocator = Allocator::new();
    let mut stylesheet = parse(source, &allocator, ParserOptions::default()).unwrap();

    minify(&mut stylesheet, MinifyOptions::default());

    assert_eq!(stylesheet.rules.len(), 1);
    let CssRule::Style(rule) = &stylesheet.rules[0] else {
        panic!("expected one synthesized style rule");
    };
    assert_eq!(rule.span.start, 0);
    assert_eq!(rule.span.end, source.len() as u32);
}

// Review finding P1: replacing one shorthand occurrence with several
// longhands must preserve importance and not invalidate later occurrence IDs.
#[test]
#[ignore = "stable one-to-many declaration occurrence replacement is not implemented"]
fn preserves_importance_and_order_when_one_occurrence_becomes_many() {
    assert_minifies_idempotently(
        "a{margin:1px!important;color:red}.middle{display:block}a{margin-left:2px!important;color:blue}",
        "a{margin-top:1px !important;margin-right:1px !important;margin-bottom:1px !important}.middle{display:block}a{margin-left:2px !important;color:#00f}",
    );
}
