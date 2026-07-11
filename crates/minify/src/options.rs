/// Options controlling local, in-place syntax-tree minification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinifyOptions {
    /// Remove ordinary comments and normalize token-list whitespace in place.
    pub normalize_tokens: bool,
    /// Normalize values stored by a single AST node.
    pub normalize_values: bool,
    /// Deduplicate entries contained by a single list node.
    pub deduplicate_lists: bool,
    /// Transform values inside custom properties.
    pub transform_custom_properties: bool,
}

impl Default for MinifyOptions {
    fn default() -> Self {
        Self {
            normalize_tokens: true,
            normalize_values: true,
            deduplicate_lists: true,
            transform_custom_properties: true,
        }
    }
}
