/// Options controlling local, in-place syntax-tree minification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinifyOptions {
    /// Remove ordinary comments and normalize token-list whitespace in place.
    pub normalize_tokens: bool,
    /// Remove leading license comments in addition to ordinary comments.
    pub discard_license_comments: bool,
    /// Normalize values stored by a single AST node.
    pub normalize_values: bool,
    /// Allow four- and eight-digit hexadecimal alpha colors.
    pub use_hex_alpha_colors: bool,
    /// Put unordered shorthand components into their canonical order.
    pub order_values: bool,
    /// Order known border width/style components around a trailing variable.
    pub order_border_values_with_variables: bool,
    /// Sort selector lists by their serialized component order.
    pub sort_selectors: bool,
    /// Normalize redundant `all` prefixes in media-query lists.
    pub normalize_media_queries: bool,
    /// Merge selectors with common components using `:is()`.
    pub merge_selectors: bool,
    /// Sort the alternatives introduced inside `:is()`.
    pub sort_selector_merges: bool,
    /// Convert absolute length units when another representation is shorter.
    pub convert_length_units: bool,
    /// Include metric absolute units (`cm`, `mm`, and `q`) in length conversion.
    pub convert_extended_length_units: bool,
    /// Decimal precision for pixel lengths; disables cross-unit conversion.
    pub length_precision: Option<u8>,
    /// Allow target-sensitive percentage zeros to become unitless zeros.
    pub convert_zero_percentages: bool,
    /// Deduplicate entries contained by a single list node.
    pub deduplicate_lists: bool,
    /// Remove structurally identical style rules within one rule-list segment.
    pub deduplicate_rules: bool,
    /// Remove keyframes rules whose body is empty after parsing.
    pub discard_empty_keyframes: bool,
    /// Remove keyframes definitions that have no animation reference.
    pub discard_unused_keyframes: bool,
    /// Remove earlier keyframes definitions overridden in the same rule list.
    pub discard_overridden_keyframes: bool,
    /// Remove counter-style definitions that have no list-style reference.
    pub discard_unused_counter_styles: bool,
    /// Remove font-face definitions that have no quoted font reference.
    pub discard_unused_font_faces: bool,
    /// Remove prefixed namespace definitions that no selector references.
    pub discard_unused_namespaces: bool,
    /// Merge identical keyframes and counter-style bodies by identifier.
    pub merge_identical_identifiers: bool,
    /// Normalize URL whitespace, default ports, and dot path segments.
    pub normalize_urls: bool,
    /// Keep the later declaration when identical declarations are joined
    /// across style rules with the same selectors.
    pub keep_later_duplicate_declarations: bool,
    /// Preserve an all-`initial` four-side margin/padding group as a shorthand.
    pub preserve_merged_box_initial: bool,
    /// Merge adjacent style rules that have identical declaration output.
    pub merge_style_rules: bool,
    /// Allow unprefixed `::placeholder` in a merged selector list.
    pub merge_placeholder_selectors: bool,
    /// Transform values inside custom properties.
    pub transform_custom_properties: bool,
    /// Replace known initial values with the `initial` keyword when supported
    /// by the configured output targets.
    pub reduce_to_initial: bool,
    /// Rebase positive z-index values while preserving their relative order.
    pub reduce_z_indices: bool,
    /// Shorten keyframe identifiers and all typed animation references.
    pub reduce_keyframe_identifiers: bool,
    /// Shorten referenced counter-style identifiers.
    pub reduce_counter_style_identifiers: bool,
    /// Shorten CSS counter identifiers and function references.
    pub reduce_counter_identifiers: bool,
    /// Shorten grid area and line identifiers.
    pub reduce_grid_identifiers: bool,
    /// First positive value used when rebasing z-index declarations.
    pub z_index_start: i32,
}

impl Default for MinifyOptions {
    fn default() -> Self {
        Self {
            normalize_tokens: true,
            discard_license_comments: false,
            normalize_values: true,
            use_hex_alpha_colors: true,
            order_values: true,
            order_border_values_with_variables: false,
            sort_selectors: true,
            normalize_media_queries: true,
            merge_selectors: true,
            sort_selector_merges: true,
            convert_length_units: true,
            convert_extended_length_units: true,
            length_precision: None,
            convert_zero_percentages: true,
            deduplicate_lists: true,
            deduplicate_rules: true,
            discard_empty_keyframes: false,
            discard_unused_keyframes: false,
            discard_overridden_keyframes: false,
            discard_unused_counter_styles: false,
            discard_unused_font_faces: false,
            discard_unused_namespaces: false,
            merge_identical_identifiers: false,
            normalize_urls: false,
            keep_later_duplicate_declarations: true,
            preserve_merged_box_initial: false,
            merge_style_rules: true,
            merge_placeholder_selectors: false,
            transform_custom_properties: true,
            reduce_to_initial: false,
            reduce_z_indices: false,
            reduce_keyframe_identifiers: false,
            reduce_counter_style_identifiers: false,
            reduce_counter_identifiers: false,
            reduce_grid_identifiers: false,
            z_index_start: 1,
        }
    }
}
