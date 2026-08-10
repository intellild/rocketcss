//! Selector-union compatibility and materialization shared by S3.

mod selector;

pub(super) use self::selector::materialize_selector_union;
