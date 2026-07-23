use super::assert_minifies_idempotently;

#[test]
fn inserts_synthesized_history_entries_in_semantic_source_order() {
    assert_minifies_idempotently("a{x:1}b{x:1}a,b{x:2}", "a,b{x:2}");
}

#[test]
fn canonicalizes_synthesized_selector_unions_immediately() {
    assert_minifies_idempotently("a,b{x:1}b,c{x:1}", "a,b,c{x:1}");
}

#[test]
fn invalidates_candidates_when_a_predecessor_block_changes() {
    assert_minifies_idempotently("a{x:1}a{y:2}b{y:2}a{x:3}", "a,b{y:2}a{x:3}");
}

#[test]
fn complete_factoring_reconnects_the_live_chain_through_the_shared_rule() {
    assert_minifies_idempotently("p{y:1}a{x:1}b{x:1}q{y:1}", "p{y:1}a,b{x:1}q{y:1}");
}

#[test]
fn rejects_zero_progress_partial_merge_plans() {
    assert_minifies_idempotently("a{}b{c{x:1}}", "a{}b{c{x:1}}");
}

#[test]
fn factors_a_complete_equal_run_in_one_stable_transition() {
    assert_minifies_idempotently("a{x:1}b{x:1}c{x:1}d{x:1}", "a,b,c,d{x:1}");
}

#[test]
fn fingerprint_matches_still_require_exact_value_equality() {
    assert_minifies_idempotently("a{color:red}b{color:blue}", "a{color:red}b{color:#00f}");
}

#[test]
fn factors_a_single_declaration_sequence_without_dense_dp_rows() {
    assert_minifies_idempotently(
        "a{color:red}b{width:1px;color:red}",
        "a,b{color:red}b{width:1px}",
    );
    assert_minifies_idempotently(
        "a{width:1px;color:red}b{color:red}",
        "a{width:1px}a,b{color:red}",
    );
}

#[test]
fn importance_is_part_of_the_declaration_history_context() {
    assert_minifies_idempotently(
        "a{color:red!important}b{color:red}",
        "a{color:red !important}b{color:red}",
    );
}

#[test]
fn selector_materialization_failure_leaves_both_endpoints_unchanged() {
    assert_minifies_idempotently(
        ".a:has(.x){color:red}.b{color:red}",
        ".a:has(.x){color:red}.b{color:red}",
    );
}

#[test]
fn overlapping_partial_candidates_commit_from_left_to_right() {
    assert_minifies_idempotently(
        "a{color:red;background:red}b{color:red;width:1px}c{width:1px}",
        "a{background:red}a,b{color:red}b,c{width:1px}",
    );
}
