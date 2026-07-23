use super::assert_minifies_idempotently;

#[test]
fn keeps_case_distinct_custom_properties() {
    assert_minifies_idempotently("a{--x:red}a{--X:blue}", "a{--x:red;--X:blue}");
}

#[test]
#[ignore = "S2 all-property exception analysis is not implemented"]
fn preserves_properties_not_reset_by_all() {
    assert_minifies_idempotently(
        "a{--x:red;direction:rtl;unicode-bidi:isolate;color:red}a{all:initial}",
        "a{--x:red;direction:rtl;unicode-bidi:isolate;all:initial}",
    );
}

#[test]
fn keeps_logical_and_physical_properties_when_direction_is_not_proven() {
    assert_minifies_idempotently(
        "a{direction:rtl;margin-left:1px}a{margin-inline-end:2px}",
        "a{direction:rtl;margin-left:1px;margin-inline-end:2px}",
    );
}

#[test]
#[ignore = "S2 importance analysis across declaration blocks is not implemented"]
fn keeps_fallback_and_importance_chains() {
    assert_minifies_idempotently(
        "a{display:-webkit-box;display:flex;color:red!important}a{color:blue}",
        "a{display:-webkit-box;display:flex;color:red !important}",
    );
}

#[test]
#[ignore = "partial shorthand replacement plans are not implemented"]
fn does_not_drop_live_components_of_a_partially_overridden_shorthand() {
    assert_minifies_idempotently(
        "a{margin:1px}a{margin-left:2px}",
        "a{margin:1px;margin-left:2px}",
    );
}

#[test]
#[ignore = "S2 revert and revert-layer analysis is not implemented"]
fn treats_revert_values_conservatively() {
    assert_minifies_idempotently(
        "a{color:red}a{color:revert}a{background:blue}a{background:revert-layer}",
        "a{color:revert;background:revert-layer}",
    );
}
