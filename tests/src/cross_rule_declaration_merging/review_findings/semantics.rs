use super::assert_minifies_idempotently;

// Review finding P1: a selector that never matches cannot discard
// selector-independent global/name-defining at-rules in its subtree.
#[test]
#[ignore = "KnownNoMatch subtree classification does not inspect global at-rule descendants"]
fn preserves_global_at_rules_below_a_known_no_match_selector() {
    assert_minifies_idempotently(
        ":not(*){@media (width>=0px){@keyframes spin{to{opacity:0}}}}.target{animation:spin 1s}",
        ":not(*){@media (width>=0px){@keyframes spin{to{opacity:0}}}}.target{animation:spin 1s}",
    );
}

// Review finding P1: equal adjacent conditional wrappers need one executable
// ownership model. This expected result is compatible with physical wrapper
// coalescing or a logical region with one active emission owner.
#[test]
#[ignore = "equal conditional-wrapper region coalescing is not implemented"]
fn merges_rules_across_adjacent_equal_conditional_wrappers() {
    assert_minifies_idempotently(
        "@media screen{a{x:1}}@media screen{a{y:2}}",
        "@media screen{a{x:1;y:2}}",
    );
}
