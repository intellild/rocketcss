use super::assert_minifies_idempotently;

#[test]
#[ignore = "S2 and S3 cross-rule processing is not implemented"]
fn inserts_synthesized_history_entries_in_semantic_source_order() {
    assert_minifies_idempotently("a{x:1}b{x:1}a,b{x:2}", "a,b{x:2}");
}

#[test]
#[ignore = "S3 selector factoring is not implemented"]
fn canonicalizes_synthesized_selector_unions_immediately() {
    assert_minifies_idempotently("a,b{x:1}b,c{x:1}", "a,b,c{x:1}");
}

#[test]
#[ignore = "S2 and S3 cross-rule processing is not implemented"]
fn invalidates_candidates_when_a_predecessor_block_changes() {
    assert_minifies_idempotently("a{x:1}a{y:2}b{y:2}a{x:3}", "a,b{y:2}a{x:3}");
}

#[test]
#[ignore = "atomic S3 live-chain replacement is not implemented"]
fn complete_factoring_reconnects_the_live_chain_through_the_shared_rule() {
    assert_minifies_idempotently("p{y:1}a{x:1}b{x:1}q{y:1}", "p{y:1}a,b{x:1}q{y:1}");
}
