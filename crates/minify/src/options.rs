use bitflags::bitflags;

bitflags! {
    /// Individually configurable minification passes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Options: u32 {
        /// Remove ordinary comments from token lists.
        const DISCARD_COMMENTS = 1 << 0;
        /// Normalize token-list whitespace in place.
        const NORMALIZE_WHITESPACE = 1 << 1;
        /// Normalize values stored by a single AST node.
        const NORMALIZE_VALUES = 1 << 2;
        /// Allow four- and eight-digit hexadecimal alpha colors.
        const USE_HEX_ALPHA_COLORS = 1 << 3;
        /// Put unordered components in a single value into canonical order.
        const ORDER_VALUES = 1 << 4;
        /// Convert absolute length units when another representation is shorter.
        const CONVERT_LENGTH_UNITS = 1 << 5;
        /// Include metric absolute units (cm, mm, and q) in length conversion.
        const CONVERT_EXTENDED_LENGTH_UNITS = 1 << 6;
        /// Preserve an existing space before a variable fallback value.
        const PRESERVE_VARIABLE_FALLBACK_SPACE = 1 << 7;
        /// Allow target-sensitive percentage zeros to become unitless zeros.
        const CONVERT_ZERO_PERCENTAGES = 1 << 8;
        /// Deduplicate entries contained by a single list node.
        const DEDUPLICATE_LISTS = 1 << 9;
        /// Normalize URL whitespace, default ports, and dot path segments.
        const NORMALIZE_URLS = 1 << 10;
        /// Transform values inside custom properties.
        const TRANSFORM_CUSTOM_PROPERTIES = 1 << 11;
        /// Merge physically adjacent style rules with equal selectors.
        const MERGE_ADJACENT_RULES = 1 << 12;
        /// Allow a ratio with denominator 1 to be written as a bare number
        /// (`1/1` → `1`). Disabled by default because some browsers
        /// misinterpret the bare-number form.
        const CONVERT_RATIOS = 1 << 13;
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

/// Options controlling in-place syntax-tree minification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinifyOptions {
    /// Enabled minification passes.
    pub flags: Options,
    /// Decimal precision for pixel lengths; disables cross-unit conversion.
    pub length_precision: Option<u8>,
    /// Decimal precision applied to folded linear calc() terms.
    pub calc_precision: Option<u8>,
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
                | Options::CONVERT_LENGTH_UNITS
                | Options::CONVERT_EXTENDED_LENGTH_UNITS
                | Options::CONVERT_ZERO_PERCENTAGES
                | Options::DEDUPLICATE_LISTS
                | Options::TRANSFORM_CUSTOM_PROPERTIES
                | Options::MERGE_ADJACENT_RULES,
            length_precision: None,
            calc_precision: None,
        }
    }
}
