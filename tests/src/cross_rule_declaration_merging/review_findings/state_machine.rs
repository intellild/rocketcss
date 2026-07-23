use super::assert_minifies_idempotently;

// Review finding P1: S3 endpoint edits must not invoke ordinary immediate
// liveness transitions and create a transient p -> q bypass edge.
#[test]
fn s3_endpoint_edits_do_not_create_a_transient_bypass_edge() {
    assert_minifies_idempotently("p{y:1}a{x:1}b{x:1}q{y:1}", "p{y:1}a,b{x:1}q{y:1}");
}

// Review finding P2: every declaration entry from both S1 inputs must point at
// the combined sequence so a later predecessor edit bumps its aggregate
// revision and invalidates the S3 candidate.
#[test]
fn reassigns_all_s1_entries_to_the_combined_sequence() {
    assert_minifies_idempotently("a{x:1}a{y:2}b{y:2}a{x:3}", "a,b{y:2}a{x:3}");
}

// Review finding P2: S4 must be physical-only. If an empty wrapper is still a
// logical barrier when S4 starts, the newly exposed S3 edge must return to the
// stabilization loop in the same pass.
#[test]
fn reaches_a_fixed_point_after_s4_exposes_a_new_edge() {
    assert_minifies_idempotently("a{x:1}@supports (display:grid){}b{x:1}", "a,b{x:1}");
}
