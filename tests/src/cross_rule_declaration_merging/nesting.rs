use super::assert_minifies_idempotently;

#[test]
fn nested_declarations_break_style_rule_adjacency() {
    assert_minifies_idempotently(
        ":where(.x,.y){&:where(.x){color:red}color:blue;&:where(.y){color:red}}",
        ":where(.x,.y){&:where(.x){color:red}color:#00f;&:where(.y){color:red}}",
    );
}

#[test]
fn opaque_at_rule_content_keeps_ancestor_style_rule_live() {
    assert_minifies_idempotently(
        ".a{@media (min-width:1px){color:red}}",
        ".a{@media (width>=1px){color:red}}",
    );
}

#[test]
fn resolves_multiple_and_functional_nesting_selectors() {
    assert_minifies_idempotently(
        ".a{&+&{color:red}:where(&){color:blue}:has(&){display:block}}",
        ".a{&+&{color:red}:where(&){color:#00f}:has(&){display:block}}",
    );
}

#[test]
fn treats_pseudo_element_nesting_as_a_conservative_barrier() {
    assert_minifies_idempotently(
        ".a::before{&{color:red}}.a::before{color:blue}",
        ".a:before{&{color:red}}.a:before{color:#00f}",
    );
}

#[test]
fn preserves_at_nest_wrapper_identity() {
    assert_minifies_idempotently(
        ".p{@nest & .a{x:1}@nest & .b{x:1}}",
        ".p{@nest & .a{x:1}@nest & .b{x:1}}",
    );
}
