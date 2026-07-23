use super::assert_minifies_idempotently;

// These cases are reduced from the Bootstrap and Tailwind pipeline fixtures.
// Active tests lock in behavior supported by the current state model. Ignored
// tests record output-profitability and selector-materialization gaps that
// require the corresponding deferred design decisions.

#[test]
fn factors_tailwind_mask_setup_without_reordering_custom_values() {
    assert_minifies_idempotently(
        ".mask-a{mask-image:var(--tw-mask-linear);mask-composite:intersect;--tw-mask-linear:var(--tw-mask-left);--tw-mask-top-from-color:red}.mask-b{mask-image:var(--tw-mask-linear);mask-composite:intersect;--tw-mask-linear:var(--tw-mask-left);--tw-mask-top-from-color:blue}",
        ".mask-a{--tw-mask-top-from-color:red}.mask-a,.mask-b{mask-image:var(--tw-mask-linear);mask-composite:intersect;--tw-mask-linear:var(--tw-mask-left)}.mask-b{--tw-mask-top-from-color:blue}",
    );
}

#[test]
fn keeps_bootstrap_placeholder_vendor_groups_separate() {
    assert_minifies_idempotently(
        ".form-control::-moz-placeholder{color:var(--bs-secondary-color);opacity:1}.form-control::placeholder{color:var(--bs-secondary-color);opacity:1}",
        ".form-control::-moz-placeholder{color:var(--bs-secondary-color);opacity:1}.form-control::placeholder{color:var(--bs-secondary-color);opacity:1}",
    );
}

#[test]
#[ignore = "profitability thresholds are a deferred design choice"]
fn does_not_expand_tailwind_screen_reader_utilities() {
    assert_minifies_idempotently(
        ".sr-only{position:absolute;padding:0}.not-sr-only{position:static;padding:0}",
        ".sr-only{position:absolute;padding:0}.not-sr-only{position:static;padding:0}",
    );
}

#[test]
#[ignore = "profitability thresholds are a deferred design choice"]
fn does_not_expand_bootstrap_modal_selectors() {
    assert_minifies_idempotently(
        ".modal-fullscreen .modal-content{color:red;border-radius:0}.modal-fullscreen .modal-header,.modal-fullscreen .modal-footer{border-radius:0;color:blue}",
        ".modal-fullscreen .modal-content{color:red;border-radius:0}.modal-fullscreen .modal-header,.modal-fullscreen .modal-footer{border-radius:0;color:#00f}",
    );
}

#[test]
#[ignore = "selector compatibility keys and recursive arena cloning need a design decision"]
fn merges_bootstrap_focus_visible_sibling_selectors() {
    assert_minifies_idempotently(
        ".btn:focus-visible{border-color:var(--x);outline:0;box-shadow:var(--s)}.btn-check:focus-visible+.btn{border-color:var(--x);outline:0;box-shadow:var(--s)}",
        ".btn:focus-visible,.btn-check:focus-visible+.btn{border-color:var(--x);outline:0;box-shadow:var(--s)}",
    );
}

#[test]
#[ignore = "custom vendor pseudo-elements remain opaque until compatibility is modeled"]
fn merges_tailwind_matching_webkit_details_marker_selectors() {
    assert_minifies_idempotently(
        "a{& *::marker{display:flex}&::marker{display:flex}& *::-webkit-details-marker{display:flex}&::-webkit-details-marker{display:flex}}",
        "a{& *::marker,&::marker{display:flex}& *::-webkit-details-marker,&::-webkit-details-marker{display:flex}}",
    );
}
