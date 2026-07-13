use rocketcss_allocator::{Allocator, atom::Atom};

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
pub struct MinifyContext<'a> {
    allocator: &'a Allocator,
    options: MinifyOptions,
    stats: MinifyStats,
    pub(crate) value_context: ValueContext,
}

impl<'a> MinifyContext<'a> {
    pub fn new(allocator: &'a Allocator, options: MinifyOptions) -> Self {
        Self {
            allocator,
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
    pub fn allocator(&self) -> &'a Allocator {
        self.allocator
    }

    #[inline]
    pub(crate) fn alloc_str(&self, value: &str) -> Atom<'a> {
        self.allocator.alloc_str(value)
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
