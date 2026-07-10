/// Options controlling syntax-tree minification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinifyOptions {
    /// Remove ordinary comments and normalize token-list whitespace.
    pub normalize_tokens: bool,
    /// Normalize colors, dimensions, angles, times, and ratios.
    pub normalize_values: bool,
    /// Remove empty and duplicate rules and declarations.
    pub discard_duplicates: bool,
    /// Merge adjacent compatible rules.
    pub merge_rules: bool,
    /// Transform values inside custom properties.
    pub transform_custom_properties: bool,
}

impl Default for MinifyOptions {
    fn default() -> Self {
        Self {
            normalize_tokens: true,
            normalize_values: true,
            discard_duplicates: true,
            merge_rules: true,
            transform_custom_properties: true,
        }
    }
}
