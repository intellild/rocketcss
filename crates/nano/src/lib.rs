mod context;
mod cross_rule_declaration_merging;
mod length;
mod media;
mod options;
mod properties;
mod rules;
mod selector;
mod token;
mod utils;
mod values;

pub mod prelude;

use rocketcss_ast::*;
use rocketcss_common::{Allocator, GhostToken};
use rocketcss_visitor::{BoxError, Plugin, PluginContext, VisitMut, VisitorMut};
use std::pin::Pin;

pub use context::{MinifyContext, MinifyStats};
pub use options::{MinifyOptions, Options, OptionsOp};

/// Minifies a syntax-tree node in place.
pub trait Minify {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx;
}

/// Minifies a stylesheet in place and returns transformation statistics.
pub fn minify<'a, 'ghost>(
    compilation: &mut Compilation<'a>,
    token: &mut GhostToken<'ghost>,
    options: MinifyOptions,
) -> MinifyStats {
    let allocator = Allocator::new();
    let mut cx = MinifyContext::new(options, &allocator);
    let (stylesheet, declaration_blocks) = compilation.parts_mut();
    minify_style_sheet(stylesheet, declaration_blocks, token, &mut cx);
    cx.stats()
}

/// Adapter for running in-place minification in a visitor plugin pipeline.
#[derive(Clone, Copy, Debug, Default)]
pub struct MinifyPlugin {
    options: MinifyOptions,
}

impl MinifyPlugin {
    #[inline]
    pub fn new(options: MinifyOptions) -> Self {
        Self { options }
    }

    #[inline]
    pub fn options(&self) -> MinifyOptions {
        self.options
    }
}

impl<'a, 'ghost> Plugin<'a, 'ghost> for MinifyPlugin {
    fn name(&self) -> &str {
        "minify"
    }

    fn transform(
        &mut self,
        compilation: &mut Compilation<'a>,
        cx: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError> {
        let stats = minify(compilation, cx.ghost_token(), self.options);
        cx.insert(stats);
        Ok(())
    }
}

pub(crate) fn minify_style_sheet<'ast, 'ghost, 'cx>(
    stylesheet: &mut StyleSheet<'ast>,
    declaration_block_store: &mut DeclarationBlockStore<'ast>,
    token: &mut GhostToken<'ghost>,
    cx: &mut MinifyContext<'cx>,
) where
    'ast: 'cx,
{
    // Minifier IR and transient collections are scratch state. Keep them out
    // of the AST arena so every temporary allocation is released when this
    // minify pass finishes.
    // Move the context into the visitor so it and its scratch IR share one
    // `'cx` lifetime, then restore it after traversal.
    let replacement = MinifyContext::new(cx.options(), cx.allocator());
    let owned_cx = std::mem::replace(cx, replacement);
    let allocator = owned_cx.allocator();
    let declaration_blocks = rules::DeclarationBlockMinifier::new(allocator);
    let collect_cross_rule_state =
        owned_cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any);
    let cross_rule_declaration_ir = if collect_cross_rule_state {
        cross_rule_declaration_merging::FrozenDeclarationIrStore::with_block_capacity(
            declaration_block_store.len(),
        )
    } else {
        Default::default()
    };
    let mut minifier = Minifier {
        cx: owned_cx,
        declaration_blocks,
        cross_rule_declaration_ir,
    };
    let discovery = {
        let mut visit_context =
            VisitMutContext::new_with_declaration_blocks(token, declaration_block_store);
        if collect_cross_rule_state {
            Some(minifier.minify_and_collect_style_sheet(stylesheet, &mut visit_context))
        } else {
            stylesheet.visit_mut(&mut minifier, &mut visit_context);
            None
        }
    };
    if let Some(discovery) = discovery {
        let declaration_ir = std::mem::take(&mut minifier.cross_rule_declaration_ir);
        let plan = cross_rule_declaration_merging::stabilize_cross_rule_declarations(
            discovery,
            declaration_block_store,
            declaration_ir,
            &mut minifier.declaration_blocks,
            &mut minifier.cx,
        );
        plan.apply(stylesheet);
    }
    let Minifier { cx: result, .. } = minifier;
    *cx = result;
}

struct Minifier<'ast, 'cx> {
    cx: MinifyContext<'cx>,
    declaration_blocks: rules::DeclarationBlockMinifier<'cx, 'ast>,
    cross_rule_declaration_ir: cross_rule_declaration_merging::FrozenDeclarationIrStore<'ast>,
}

impl<'ast, 'cx> Minifier<'ast, 'cx> {
    fn minify_and_collect_style_sheet<'tree, 'ghost>(
        &mut self,
        stylesheet: &'tree mut StyleSheet<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) -> utils::DeclarationBlockDiscovery<'tree, 'ast>
    where
        'ast: 'tree,
    {
        let mut collector = utils::DeclarationBlockCollector::new();
        self.minify_and_collect_rule_list(&mut stylesheet.rules, &mut collector, cx);
        collector.finish()
    }

    fn minify_and_collect_rule_list<'tree, 'ghost>(
        &mut self,
        rules: &'tree mut rocketcss_common::vec::Vec<'ast, CssRule<'ast>>,
        collector: &mut utils::DeclarationBlockCollector<'tree, 'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) where
        'ast: 'tree,
    {
        let rule_list = collector.allocate_rule_list();
        let mut rule_list_segment = collector.allocate_rule_list_segment();
        for (sibling, rule) in rules.iter_mut().enumerate() {
            let ends_segment = utils::ends_rule_list_segment(rule);
            self.minify_and_collect_rule(
                rule,
                utils::StructuralLocation {
                    rule_list,
                    rule_list_segment,
                    sibling_ordinal: utils::SiblingOrdinal::from_index(sibling),
                },
                collector,
                cx,
            );
            if ends_segment {
                rule_list_segment = collector.allocate_rule_list_segment();
            }
        }
    }

    fn minify_and_collect_rule<'tree, 'ghost>(
        &mut self,
        rule: &'tree mut CssRule<'ast>,
        location: utils::StructuralLocation,
        collector: &mut utils::DeclarationBlockCollector<'tree, 'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) where
        'ast: 'tree,
    {
        match rule {
            CssRule::Media(rule) => {
                let MediaRule { query, rules, .. } = &mut **rule;
                query.visit_mut(self, cx);
                let parent = collector.enter_condition(utils::ConditionalFrame::Media(query));
                self.minify_and_collect_rule_list(rules, collector, cx);
                collector.leave_condition(parent);
            }
            CssRule::Style(rule) => self.minify_and_collect_style_rule(
                rule.as_mut(),
                utils::SelectorFrameKind::Style,
                location,
                collector,
                cx,
            ),
            CssRule::Supports(rule) => {
                let SupportsRule {
                    condition, rules, ..
                } = &mut **rule;
                condition.visit_mut(self, cx);
                let parent =
                    collector.enter_condition(utils::ConditionalFrame::Supports(condition));
                self.minify_and_collect_rule_list(rules, collector, cx);
                collector.leave_condition(parent);
            }
            CssRule::MozDocument(rule) => {
                let parent =
                    collector.enter_opaque_condition(utils::OpaqueConditionalKind::MozDocument);
                self.minify_and_collect_rule_list(&mut rule.rules, collector, cx);
                collector.leave_condition(parent);
            }
            CssRule::Nesting(rule) => self.minify_and_collect_style_rule(
                rule.style.as_mut(),
                utils::SelectorFrameKind::Nesting,
                location,
                collector,
                cx,
            ),
            CssRule::NestedDeclarations(rule) => {
                cx.with_declaration_block(rule.declarations, |block, cx| {
                    block.visit_mut(self, cx);
                    self.cross_rule_declaration_ir
                        .freeze_physical_block(rule.declarations, block);
                });
                collector.push_declaration_block(
                    rule.declarations,
                    utils::DeclarationBlockKind::NestedDeclarations,
                    location,
                );
            }
            CssRule::LayerBlock(rule) => {
                let parent = collector.enter_opaque_condition(utils::OpaqueConditionalKind::Layer);
                self.minify_and_collect_rule_list(&mut rule.rules, collector, cx);
                collector.leave_condition(parent);
            }
            CssRule::Container(rule) => {
                let ContainerRule {
                    condition,
                    name,
                    rules,
                    ..
                } = &mut **rule;
                if let Some(condition) = condition.as_mut() {
                    condition.visit_mut(self, cx);
                }
                let parent = collector.enter_condition(utils::ConditionalFrame::Container {
                    name: *name,
                    condition: condition.as_deref(),
                });
                self.minify_and_collect_rule_list(rules, collector, cx);
                collector.leave_condition(parent);
            }
            CssRule::Scope(rule) => {
                if let Some(selectors) = &mut rule.scope_start {
                    self.visit_selector_list(selectors, cx);
                }
                if let Some(selectors) = &mut rule.scope_end {
                    self.visit_selector_list(selectors, cx);
                }
                let parent = collector.enter_opaque_condition(utils::OpaqueConditionalKind::Scope);
                self.minify_and_collect_rule_list(&mut rule.rules, collector, cx);
                collector.leave_condition(parent);
            }
            CssRule::StartingStyle(rule) => {
                let parent =
                    collector.enter_opaque_condition(utils::OpaqueConditionalKind::StartingStyle);
                self.minify_and_collect_rule_list(&mut rule.rules, collector, cx);
                collector.leave_condition(parent);
            }
            _ => rule.visit_mut_children(self, cx),
        }
    }

    fn minify_and_collect_style_rule<'tree, 'ghost>(
        &mut self,
        rule: Pin<&'tree mut StyleRule<'ast>>,
        kind: utils::SelectorFrameKind,
        location: utils::StructuralLocation,
        collector: &mut utils::DeclarationBlockCollector<'tree, 'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) where
        'ast: 'tree,
    {
        let (declarations, span, vendor_prefix) = {
            let rule = rule.as_ref().get_ref();
            (rule.declarations, rule.span, rule.vendor_prefix)
        };
        let (selectors, rules) = rule.selectors_and_rules_mut();
        self.visit_selector_list(selectors, cx);
        cx.with_declaration_block(declarations, |block, cx| {
            block.visit_mut(self, cx);
            self.cross_rule_declaration_ir
                .freeze_physical_block(declarations, block);
        });

        let parent = collector.enter_selector(kind, selectors, vendor_prefix);
        collector.push_declaration_block(
            declarations,
            match kind {
                utils::SelectorFrameKind::Style => utils::DeclarationBlockKind::Style {
                    selectors,
                    span,
                    vendor_prefix,
                    has_children: !rules.is_empty(),
                    has_live_selectors: selectors.iter().any(|selector| !selector.is_tombstone()),
                },
                utils::SelectorFrameKind::Nesting => utils::DeclarationBlockKind::Nesting,
            },
            location,
        );
        if !rules.is_empty() {
            self.minify_and_collect_rule_list(rules, collector, cx);
        }
        collector.leave_selector(parent);
    }
}

impl<'ast, 'ghost> VisitorMut<'ast, 'ghost> for Minifier<'ast, '_> {
    fn visit_declaration(
        &mut self,
        node: &mut Declaration<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        let remove_declaration = if let Declaration::FontFamily(families) = node {
            families.minify(&mut self.cx);
            families.iter().all(FontFamily::is_tombstone)
        } else {
            false
        };
        if remove_declaration {
            *node = Declaration::Tombstone;
            self.cx.record_declaration_removed();
        }
    }

    fn visit_font_family(
        &mut self,
        node: &mut FontFamily<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        if !matches!(node, FontFamily::Unparsed(_) | FontFamily::Tombstone) {
            node.visit_mut_children(self, cx);
        }
    }

    fn visit_declaration_block(
        &mut self,
        node: &mut DeclarationBlock<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        self.declaration_blocks.minify(node, &mut self.cx);
    }

    fn visit_keyframe_selector(
        &mut self,
        node: &mut KeyframeSelector,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_animation(
        &mut self,
        node: &mut Animation<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_unparsed_property(
        &mut self,
        node: &mut UnparsedProperty<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        if matches!(
            node.reason,
            UnparsedPropertyReason::UnknownProperty | UnparsedPropertyReason::InvalidValue
        ) {
            return;
        }
        let previous = self.cx.value_context;
        self.cx.value_context = properties::value_context(
            &node.property_id,
            self.cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
            self.cx
                .is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
        );
        if matches!(node.reason, UnparsedPropertyReason::UnsupportedGrammar) {
            node.visit_mut_children(self, cx);
        } else {
            self.cx
                .value_context
                .set_enabled(context::ValueContextFlags::SKIP_RAW_TOKEN_TRANSFORMS, true);
        }
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_custom_property(
        &mut self,
        node: &mut CustomProperty<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        let previous = self.cx.value_context;
        self.cx.value_context = properties::custom_property_context(&self.cx);
        let name = match &*node.name {
            CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => *name,
        };
        if match_ignore_ascii_case!(name, "--font-family" => true, _ => false) {
            self.cx.value_context.property = context::PropertyContext::Font;
        }
        let fuse_token_compaction = !self
            .cx
            .value_context
            .is_enabled(context::ValueContextFlags::SKIP_VALUE_TRANSFORMS)
            && self.cx.is_enabled(
                Options::DISCARD_COMMENTS | Options::NORMALIZE_WHITESPACE,
                OptionsOp::And,
            );
        if fuse_token_compaction {
            node.name.visit_mut(self, cx);
            let preserve_space_after_comma = self
                .cx
                .value_context
                .is_enabled(context::ValueContextFlags::PRESERVE_SPACE_AFTER_COMMA);
            let normalized = token::visit_and_compact_comments_and_whitespace(
                &mut node.value,
                preserve_space_after_comma,
                |value| value.visit_mut(self, cx),
            );
            for _ in 0..normalized {
                self.cx.record_value_normalized();
            }
            token::minify_compacted_token_values(&mut node.value, &mut self.cx);
        } else {
            node.visit_mut_children(self, cx);
            node.minify(&mut self.cx);
        }
        self.cx.value_context = previous;
    }

    fn visit_function(
        &mut self,
        node: &mut Function<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        let previous = self.cx.value_context;
        let kind = node.kind();
        if matches!(kind, KnownFunction::Rgb | KnownFunction::Rgba) && !node.is_valid_rgb() {
            return;
        }
        if kind.is_color() {
            self.cx
                .value_context
                .set_enabled(context::ValueContextFlags::SKIP_VALUE_TRANSFORMS, true);
            node.visit_mut_children(self, cx);
            self.cx.value_context = previous;
            self.cx
                .value_context
                .set_enabled(context::ValueContextFlags::SKIP_RAW_TOKEN_TRANSFORMS, true);
            node.minify(&mut self.cx);
            self.cx.value_context = previous;
            return;
        }
        if kind.is_math() {
            self.cx.value_context.set_enabled(
                context::ValueContextFlags::ALLOW_UNITLESS_ZERO_LENGTH
                    | context::ValueContextFlags::ALLOW_UNITLESS_ZERO_PERCENTAGE,
                false,
            );
            self.cx.value_context.property = context::PropertyContext::Generic;
        }
        match kind {
            KnownFunction::Hwb => self.cx.value_context.set_enabled(
                context::ValueContextFlags::ALLOW_UNITLESS_ZERO_LENGTH
                    | context::ValueContextFlags::ALLOW_UNITLESS_ZERO_PERCENTAGE,
                false,
            ),
            KnownFunction::ColorMix | KnownFunction::Linear => self.cx.value_context.set_enabled(
                context::ValueContextFlags::ALLOW_UNITLESS_ZERO_PERCENTAGE,
                false,
            ),
            _ => {}
        }
        if kind.is_gradient() {
            self.cx.value_context.property = context::PropertyContext::Generic;
        }
        if kind == KnownFunction::Local {
            self.cx
                .value_context
                .set_enabled(context::ValueContextFlags::MINIFY_COLORS, false);
        }
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_variable(
        &mut self,
        node: &mut Variable<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_environment_variable(
        &mut self,
        node: &mut EnvironmentVariable<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_unknown_at_rule(
        &mut self,
        node: &mut UnknownAtRule<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        let previous = self.cx.value_context;
        self.cx.value_context = Default::default();
        self.cx
            .value_context
            .set_enabled(context::ValueContextFlags::SKIP_VALUE_TRANSFORMS, true);
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_token_or_value(
        &mut self,
        node: &mut TokenOrValue<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_length_value(
        &mut self,
        node: &mut LengthValue,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_angle(&mut self, node: &mut Angle, cx: &mut VisitMutContext<'_, 'ast, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_time(&mut self, node: &mut Time, cx: &mut VisitMutContext<'_, 'ast, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_resolution(
        &mut self,
        node: &mut Resolution,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_ratio(&mut self, node: &mut Ratio, cx: &mut VisitMutContext<'_, 'ast, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_selector_list(
        &mut self,
        node: &mut SelectorList<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        self.visit_selector_list_children(node, cx);
        let allocator = self.cx.allocator();
        selector::minify_selector_list(node, &mut self.cx, allocator);
    }

    fn visit_media_list(
        &mut self,
        node: &mut MediaList<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }
}

#[cfg(test)]
mod tests;
