use bitflags::bitflags;
use rocketcss_common::Allocator;

use crate::{MinifyOptions, Options, OptionsOp};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MinifyStats {
    pub values_normalized: u32,
    pub declarations_removed: u32,
    pub initial_scans: u32,
    pub scheduler_ast_mutations: u32,
    pub reification_passes: u32,
    pub live_endpoint_reuses: u32,
    pub rule_tombstone_reuses: u32,
    pub block_tombstone_reuses: u32,
    pub declaration_tombstone_reuses: u32,
    pub residual_rule_inserts: u32,
    pub residual_declaration_inserts: u32,
    pub radix_relabel_groups: u32,
}

#[allow(dead_code)]
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
        const SKIP_RAW_TOKEN_TRANSFORMS = 1 << 5;
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
pub struct MinifyContext<'cx> {
    allocator: &'cx Allocator,
    options: MinifyOptions,
    stats: MinifyStats,
    pub(crate) value_context: ValueContext,
}

impl<'cx> MinifyContext<'cx> {
    /// Creates a minification context backed by the scratch allocator shared
    /// for the whole minification pass.
    pub fn new(options: MinifyOptions, allocator: &'cx Allocator) -> Self {
        Self {
            allocator,
            options,
            stats: MinifyStats::default(),
            value_context: ValueContext::default(),
        }
    }

    /// Returns the scratch allocator shared by this minification pass.
    #[inline]
    pub fn allocator(&self) -> &'cx Allocator {
        self.allocator
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

    #[inline]
    pub(crate) fn record_declaration_removed(&mut self) {
        self.stats.declarations_removed += 1;
    }

    pub(crate) fn record_cross_rule_stats(
        &mut self,
        stats: crate::cross_rule_declaration_merging::CrossRuleStats,
    ) {
        self.stats.initial_scans += stats.initial_scans;
        self.stats.scheduler_ast_mutations += stats.scheduler_ast_mutations;
        self.stats.reification_passes += stats.reification_passes;
        self.stats.live_endpoint_reuses += stats.live_endpoint_reuses;
        self.stats.rule_tombstone_reuses += stats.rule_tombstone_reuses;
        self.stats.block_tombstone_reuses += stats.block_tombstone_reuses;
        self.stats.declaration_tombstone_reuses += stats.declaration_tombstone_reuses;
        self.stats.residual_rule_inserts += stats.residual_rule_inserts;
        self.stats.residual_declaration_inserts += stats.residual_declaration_inserts;
        self.stats.radix_relabel_groups += stats.radix_relabel_groups;
    }
}
