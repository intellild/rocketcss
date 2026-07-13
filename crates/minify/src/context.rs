use bitflags::bitflags;

use crate::{MinifyOptions, Options, OptionsOp};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MinifyStats {
    pub values_normalized: u32,
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

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) struct ValueContextFlags: u8 {
        const ALLOW_UNITLESS_ZERO_LENGTH = 1 << 0;
        const ALLOW_UNITLESS_ZERO_PERCENTAGE = 1 << 1;
        const MINIFY_COLORS = 1 << 2;
        const PRESERVE_SPACE_AFTER_COMMA = 1 << 3;
        const SKIP_VALUE_TRANSFORMS = 1 << 4;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ValueContext {
    flags: ValueContextFlags,
    pub property: PropertyContext,
}

impl ValueContext {
    #[inline]
    pub(crate) const fn new(property: PropertyContext) -> Self {
        Self {
            flags: ValueContextFlags::MINIFY_COLORS,
            property,
        }
    }

    #[inline]
    pub(crate) const fn is_enabled(&self, option: ValueContextFlags) -> bool {
        self.flags.contains(option)
    }

    #[inline]
    pub(crate) fn set_enabled(&mut self, option: ValueContextFlags, enabled: bool) {
        self.flags.set(option, enabled);
    }
}

impl Default for ValueContext {
    fn default() -> Self {
        Self::new(PropertyContext::Generic)
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
    pub fn is_enabled(&self, options: Options, op: OptionsOp) -> bool {
        self.options.is_enabled(options, op)
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
