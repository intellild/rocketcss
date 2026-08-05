//! Selector-union compatibility and materialization shared by Radix S3.

mod selector;

pub(super) use self::selector::materialize_selector_union;
