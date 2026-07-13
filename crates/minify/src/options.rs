use bitflags::bitflags;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserHackTarget {
    Firefox2,
    Ie6,
    Ie7,
    Ie8,
    Ie9,
    Opera9,
    Modern,
}

bitflags! {
    /// Individually configurable minification passes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Options: u64 {
        /// Remove ordinary comments from token lists.
        const DISCARD_COMMENTS = 1 << 0;
        /// Normalize token-list whitespace in place.
        const NORMALIZE_WHITESPACE = 1 << 1;
        /// Remove leading license comments in addition to ordinary comments.
        const DISCARD_LICENSE_COMMENTS = 1 << 2;
        /// Normalize values stored by a single AST node.
        const NORMALIZE_VALUES = 1 << 3;
        /// Allow four- and eight-digit hexadecimal alpha colors.
        const USE_HEX_ALPHA_COLORS = 1 << 4;
        /// Put unordered shorthand components into their canonical order.
        const ORDER_VALUES = 1 << 5;
        /// Sort independent declarations while preserving cascade dependencies.
        const SORT_DECLARATIONS = 1 << 6;
        /// Remove vendor-prefixed declarations no longer needed by the targets.
        const DISCARD_OBSOLETE_PREFIXES = 1 << 7;
        /// Order known border width/style components around a trailing variable.
        const ORDER_BORDER_VALUES_WITH_VARIABLES = 1 << 8;
        /// Sort selector lists by their serialized component order.
        const SORT_SELECTORS = 1 << 9;
        /// Normalize redundant `all` prefixes in media-query lists.
        const NORMALIZE_MEDIA_QUERIES = 1 << 10;
        /// Merge selectors with common components using `:is()`.
        const MERGE_SELECTORS = 1 << 11;
        /// Sort the alternatives introduced inside `:is()`.
        const SORT_SELECTOR_MERGES = 1 << 12;
        /// Convert absolute length units when another representation is shorter.
        const CONVERT_LENGTH_UNITS = 1 << 13;
        /// Include metric absolute units (`cm`, `mm`, and `q`) in length conversion.
        const CONVERT_EXTENDED_LENGTH_UNITS = 1 << 14;
        /// Preserve an existing space before a variable fallback value.
        const PRESERVE_VARIABLE_FALLBACK_SPACE = 1 << 15;
        /// Allow target-sensitive percentage zeros to become unitless zeros.
        const CONVERT_ZERO_PERCENTAGES = 1 << 16;
        /// Deduplicate entries contained by a single list node.
        const DEDUPLICATE_LISTS = 1 << 17;
        /// Remove structurally identical style rules within one rule-list segment.
        const DEDUPLICATE_RULES = 1 << 18;
        /// Remove empty rules.
        const DISCARD_EMPTY = 1 << 19;
        /// Remove keyframes definitions that have no animation reference.
        const DISCARD_UNUSED_KEYFRAMES = 1 << 20;
        /// Remove earlier keyframes definitions overridden in the same rule list.
        const DISCARD_OVERRIDDEN_KEYFRAMES = 1 << 21;
        /// Remove counter-style definitions that have no list-style reference.
        const DISCARD_UNUSED_COUNTER_STYLES = 1 << 22;
        /// Remove font-face definitions that have no quoted font reference.
        const DISCARD_UNUSED_FONT_FACES = 1 << 23;
        /// Remove prefixed namespace definitions that no selector references.
        const DISCARD_UNUSED_NAMESPACES = 1 << 24;
        /// Merge identical keyframes and counter-style bodies by identifier.
        const MERGE_IDENTICAL_IDENTIFIERS = 1 << 25;
        /// Normalize URL whitespace, default ports, and dot path segments.
        const NORMALIZE_URLS = 1 << 26;
        /// Keep the later declaration when identical declarations are joined
        /// across style rules with the same selectors.
        const KEEP_LATER_DUPLICATE_DECLARATIONS = 1 << 27;
        /// Preserve an all-`initial` four-side margin/padding group as a shorthand.
        const PRESERVE_MERGED_BOX_INITIAL = 1 << 28;
        /// Merge adjacent style rules that have identical declaration output.
        const MERGE_STYLE_RULES = 1 << 29;
        /// Allow unprefixed `::placeholder` in a merged selector list.
        const MERGE_PLACEHOLDER_SELECTORS = 1 << 30;
        /// Transform values inside custom properties.
        const TRANSFORM_CUSTOM_PROPERTIES = 1 << 31;
        /// Replace known initial values with the `initial` keyword when supported
        /// by the configured output targets.
        const REDUCE_TO_INITIAL = 1 << 32;
        /// Rebase positive z-index values while preserving their relative order.
        const REDUCE_Z_INDICES = 1 << 33;
        /// Shorten keyframe identifiers and all typed animation references.
        const REDUCE_KEYFRAME_IDENTIFIERS = 1 << 34;
        /// Shorten referenced counter-style identifiers.
        const REDUCE_COUNTER_STYLE_IDENTIFIERS = 1 << 35;
        /// Shorten CSS counter identifiers and function references.
        const REDUCE_COUNTER_IDENTIFIERS = 1 << 36;
        /// Shorten grid area and line identifiers.
        const REDUCE_GRID_IDENTIFIERS = 1 << 37;
        /// Remove keyframes rules whose body becomes empty after parsing.
        const DISCARD_EMPTY_KEYFRAMES = 1 << 38;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptionsOp {
    /// Every requested option must be enabled.
    And,
    /// At least one requested option must be enabled.
    Any,
    /// None of the requested options may be enabled.
    None,
}

/// Options controlling local, in-place syntax-tree minification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinifyOptions {
    /// Enabled minification passes.
    pub flags: Options,
    /// Remove legacy browser hacks not needed by this explicit target.
    pub browser_hack_target: Option<BrowserHackTarget>,
    /// Decimal precision for pixel lengths; disables cross-unit conversion.
    pub length_precision: Option<u8>,
    /// Decimal precision applied to folded linear `calc()` terms.
    pub calc_precision: Option<u8>,
    /// First positive value used when rebasing z-index declarations.
    pub z_index_start: i32,
}

impl MinifyOptions {
    #[inline]
    pub const fn is_enabled(&self, options: Options, op: OptionsOp) -> bool {
        match op {
            OptionsOp::And => self.flags.contains(options),
            OptionsOp::Any => self.flags.intersects(options),
            OptionsOp::None => !self.flags.intersects(options),
        }
    }
}

impl Default for MinifyOptions {
    fn default() -> Self {
        Self {
            flags: Options::DISCARD_COMMENTS
                | Options::NORMALIZE_WHITESPACE
                | Options::NORMALIZE_VALUES
                | Options::USE_HEX_ALPHA_COLORS
                | Options::ORDER_VALUES
                | Options::SORT_SELECTORS
                | Options::NORMALIZE_MEDIA_QUERIES
                | Options::MERGE_SELECTORS
                | Options::SORT_SELECTOR_MERGES
                | Options::CONVERT_LENGTH_UNITS
                | Options::CONVERT_EXTENDED_LENGTH_UNITS
                | Options::CONVERT_ZERO_PERCENTAGES
                | Options::DEDUPLICATE_LISTS
                | Options::DEDUPLICATE_RULES
                | Options::DISCARD_EMPTY
                | Options::KEEP_LATER_DUPLICATE_DECLARATIONS
                | Options::MERGE_STYLE_RULES
                | Options::TRANSFORM_CUSTOM_PROPERTIES,
            browser_hack_target: None,
            length_precision: None,
            calc_precision: None,
            z_index_start: 1,
        }
    }
}
