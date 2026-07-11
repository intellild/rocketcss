use crate::MinifyOptions;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MinifyStats {
    pub values_normalized: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum PropertyContext {
    Box,
    FontWeight,
    Repeat,
    #[default]
    Generic,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ValueContext {
    pub allow_unitless_zero: bool,
    pub property: PropertyContext,
    pub skip_value_transforms: bool,
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
}
