use rs_css_allocator::Allocator;
use rs_css_ast::StyleSheet;
use rs_css_visitor::{AstType, Visit};

use crate::MinifyOptions;

/// Facts collected before mutation begins.
///
/// Passes that need ancestor-dependent information should add it here during
/// analysis instead of looking up parents while traversing the mutable AST.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MinifyAnalysis {
    pub declarations: usize,
    pub nested_rules: usize,
    pub rules: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MinifyStats {
    pub declarations_removed: usize,
    pub rules_merged: usize,
    pub rules_removed: usize,
    pub values_normalized: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum PropertyContext {
    Box,
    Display,
    FontFamily,
    FontWeight,
    Position,
    Repeat,
    Timing,
    Transform,
    #[default]
    Generic,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ValueContext {
    pub allow_color: bool,
    pub allow_unitless_zero: bool,
    pub property: PropertyContext,
    pub skip_value_transforms: bool,
}

/// Shared state for all minification passes.
pub struct MinifyContext<'a> {
    allocator: &'a Allocator,
    analysis: MinifyAnalysis,
    options: MinifyOptions,
    stats: MinifyStats,
    pub(crate) value_context: ValueContext,
}

impl<'a> MinifyContext<'a> {
    pub fn new(
        allocator: &'a Allocator,
        stylesheet: &StyleSheet<'a>,
        options: MinifyOptions,
    ) -> Self {
        Self {
            allocator,
            analysis: analyze(stylesheet),
            options,
            stats: MinifyStats::default(),
            value_context: ValueContext::default(),
        }
    }

    #[inline]
    pub fn allocator(&self) -> &'a Allocator {
        self.allocator
    }

    #[inline]
    pub fn analysis(&self) -> MinifyAnalysis {
        self.analysis
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
    pub(crate) fn record_declarations_removed(&mut self, count: usize) {
        self.stats.declarations_removed += count;
    }

    #[inline]
    pub(crate) fn record_rule_merged(&mut self) {
        self.stats.rules_merged += 1;
    }

    #[inline]
    pub(crate) fn record_rule_removed(&mut self) {
        self.stats.rules_removed += 1;
    }

    #[inline]
    pub(crate) fn record_value_normalized(&mut self) {
        self.stats.values_normalized += 1;
    }
}

#[derive(Default)]
struct AnalysisVisitor {
    analysis: MinifyAnalysis,
}

impl<'a> Visit<'a> for AnalysisVisitor {
    fn enter_node(&mut self, kind: AstType) {
        match kind {
            AstType::CssRule => self.analysis.rules += 1,
            AstType::Declaration => self.analysis.declarations += 1,
            AstType::NestingRule | AstType::NestedDeclarationsRule => {
                self.analysis.nested_rules += 1;
            }
            _ => {}
        }
    }
}

fn analyze(stylesheet: &StyleSheet<'_>) -> MinifyAnalysis {
    let mut visitor = AnalysisVisitor::default();
    visitor.visit_style_sheet(stylesheet);
    visitor.analysis
}
