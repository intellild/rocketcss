use super::assert_minifies_idempotently;

// Review finding P1: equal adjacent conditional wrappers need one executable
// ownership model. This expected result is compatible with physical wrapper
// coalescing or a logical region with one active emission owner.
#[test]
fn merges_rules_across_adjacent_equal_conditional_wrappers() {
    assert_minifies_idempotently(
        "@media screen{a{x:1}}@media screen{a{y:2}}",
        "@media screen{a{x:1;y:2}}",
    );
}

#[test]
fn merges_only_within_the_same_authored_layer_context() {
    assert_minifies_idempotently(
        "@layer theme{a{x:1}a{y:2}}@layer theme{a{z:3}}",
        "@layer theme{a{x:1;y:2}}@layer theme{a{z:3}}",
    );
}
