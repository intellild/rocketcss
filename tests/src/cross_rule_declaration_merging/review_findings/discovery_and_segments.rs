use super::assert_minifies_idempotently;

// Review finding P0: a parent must remain pinned while its child list is still
// being discovered. S2 may empty the leading declaration before the child has
// incremented retained_child_count.
#[test]
fn does_not_unlink_a_parent_before_child_discovery_finishes() {
    assert_minifies_idempotently(
        ".a{color:red!important}.middle{display:block}.a{color:blue;.child{display:block}}",
        ".a{color:red !important}.middle{display:block}.a{.child{display:block}}",
    );
}

// Review finding P0/P2: removing a live NestedDeclarationsRule barrier must
// join its two RuleListSegmentIds before the newly exposed edge is classified.
#[test]
fn joins_segments_after_a_nested_declarations_barrier_becomes_empty() {
    assert_minifies_idempotently(
        ".parent{color:red!important;.a{x:1}color:blue;.a{y:2}}",
        ".parent{color:red !important;.a{x:1;y:2}}",
    );
}

// Review finding P0/P2: an empty supported conditional wrapper must retire as
// a logical barrier before S4, so S1 sees the newly adjacent rules in the same
// pass.
#[test]
fn joins_segments_after_an_empty_conditional_wrapper() {
    assert_minifies_idempotently("a{x:1}@media screen{}a{y:2}", "a{x:1;y:2}");
}

// Review finding P2: a barrier that is already output-empty when discovery
// begins must not survive until a post-stabilization S4 cleanup.
#[test]
fn ignores_an_initially_empty_nested_declarations_barrier() {
    assert_minifies_idempotently(
        ".parent{.a{x:1}font-family:var(--family);font-family:slab inherit;.b{x:1}}",
        ".parent{.a,.b{x:1}}",
    );
}
