use crate::{Selector, Vec};

use super::*;

#[test]
fn selector_fingerprint_collisions_still_require_exact_equality() {
    let allocator = rocketcss_common::Allocator::new();
    let mut stylesheet = StyleSheet::new_in(&allocator);
    let empty = Vec::new_in(&allocator);
    let mut tombstone = Vec::new_in(&allocator);
    tombstone.push(Selector::Tombstone);

    let first = stylesheet
        .intern_selector_value_with_fingerprint(
            empty,
            SelectorFrameKind::Style,
            VendorPrefix::NONE,
            1,
        )
        .unwrap();
    let second = stylesheet
        .intern_selector_value_with_fingerprint(
            tombstone,
            SelectorFrameKind::Style,
            VendorPrefix::NONE,
            1,
        )
        .unwrap();

    assert_ne!(first, second);
}
