use crate::MinifyOptions;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MinifyStats {
    pub values_normalized: usize,
    pub declarations_removed: usize,
    pub style_rules_merged: usize,
    pub conditional_rules_merged: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum PropertyContext {
    Animation,
    Border,
    Box,
    BoxShadow,
    Columns,
    Display,
    FlexFlow,
    Font,
    FontWeight,
    GridAutoFlow,
    GridGap,
    GridLine,
    ListStyle,
    Outline,
    Position,
    Repeat,
    TimingFunction,
    Transition,
    Transform,
    #[default]
    Generic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ValueContext {
    pub allow_unitless_zero_length: bool,
    pub allow_unitless_zero_percentage: bool,
    pub minify_colors: bool,
    pub preserve_space_after_comma: bool,
    pub property: PropertyContext,
    pub skip_value_transforms: bool,
}

impl Default for ValueContext {
    fn default() -> Self {
        Self {
            allow_unitless_zero_length: false,
            allow_unitless_zero_percentage: false,
            minify_colors: true,
            preserve_space_after_comma: false,
            property: PropertyContext::Generic,
            skip_value_transforms: false,
        }
    }
}

/// Shared state for local, in-place node minification.
pub struct MinifyContext {
    options: MinifyOptions,
    stats: MinifyStats,
    pub(crate) value_context: ValueContext,
}

impl MinifyContext {
    pub fn new(options: MinifyOptions) -> Self {
        Self {
            options,
            stats: MinifyStats::default(),
            value_context: ValueContext::default(),
        }
    }

    #[inline]
    pub fn options(&self) -> MinifyOptions {
        self.options
    }

    #[inline]
    pub fn stats(&self) -> MinifyStats {
        self.stats
    }

    #[inline]
    pub(crate) fn record_value_normalized(&mut self) {
        self.stats.values_normalized += 1;
    }

    #[inline]
    pub(crate) fn record_declaration_removed(&mut self) {
        self.stats.declarations_removed += 1;
    }

    #[inline]
    pub(crate) fn record_style_rule_merged(&mut self) {
        self.stats.style_rules_merged += 1;
    }

    #[inline]
    pub(crate) fn record_conditional_rule_merged(&mut self) {
        self.stats.conditional_rules_merged += 1;
    }
}
