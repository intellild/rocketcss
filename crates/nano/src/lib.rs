mod context;
mod cross_rule_declaration_merging;
mod length;
mod media;
mod options;
mod properties;
mod rules;
mod selector;
mod token;
mod values;

pub mod prelude;

use rocketcss_ast::*;
use rocketcss_common::{DenseId, GhostToken};
use rocketcss_visitor::{BoxError, Plugin, PluginContext, VisitMut, VisitorMut};

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
    let mut cx = MinifyContext::new(options);
    let string_pool = compilation.take_string_pool();
    cx.replace_string_pool(string_pool);
    let origin = compilation.origin();
    let (stylesheet, declaration_blocks, rules) = compilation.all_parts_mut();
    minify_style_sheet(
        stylesheet,
        declaration_blocks,
        rules,
        origin,
        token,
        &mut cx,
    );
    let stats = cx.stats();
    let string_pool = cx.replace_string_pool(Default::default());
    compilation.replace_string_pool(string_pool);
    stats
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
    _stylesheet: &mut StyleSheet<'ast>,
    declaration_block_store: &mut DeclarationBlockStore<'ast>,
    rule_store: &mut RuleStore<'ast>,
    origin: CascadeOrigin,
    token: &mut GhostToken<'ghost>,
    cx: &mut MinifyContext<'cx>,
) where
    'ast: 'cx,
{
    // Move the context into the visitor so it and its scratch IR share one
    // `'cx` lifetime, then restore it after traversal.
    let replacement = MinifyContext::new(cx.options());
    let owned_cx = std::mem::replace(cx, replacement);
    let declaration_blocks = rules::DeclarationBlockMinifier::new();
    let mut minifier = Minifier {
        cx: owned_cx,
        declaration_blocks,
    };
    let mut declaration_block_discovery = minifier
        .cx
        .is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any)
        .then(|| {
            cross_rule_declaration_merging::DeclarationBlockDiscovery::new(rule_store, origin)
        });
    {
        let rule_count = rule_store.len();
        let mut visit_context =
            VisitMutContext::new_with_stores_flat(token, declaration_block_store, rule_store);
        for index in 0..rule_count {
            let rule = RuleId::from_index(index).expect("rule count fits its dense ID domain");
            visit_context.visit_rule(rule, &mut minifier);
            if let Some(discovery) = &mut declaration_block_discovery {
                discovery.observe(rule, visit_context.rule_store());
            }
        }
    }
    let structural_change = declaration_block_discovery.is_some_and(|discovery| {
        cross_rule_declaration_merging::merge_cross_rule_declarations(
            rule_store,
            declaration_block_store,
            discovery.finish(),
            &mut minifier.declaration_blocks,
            &mut minifier.cx,
        )
    });
    let rule_tape_dirty = minifier.cx.rule_tape_dirty() || structural_change;
    if rule_tape_dirty {
        rule_store.compact(declaration_block_store);
    }
    if minifier.cx.declaration_tape_dirty() || rule_tape_dirty {
        // S5 declaration commit: code generation sees one compact,
        // tombstone-free tape and never needs merge chains or logical
        // multi-range state.
        declaration_block_store.compact();
    }
    let Minifier { cx: result, .. } = minifier;
    *cx = result;
}

struct Minifier<'ast, 'cx> {
    cx: MinifyContext<'cx>,
    declaration_blocks: rules::DeclarationBlockMinifier<'ast>,
}

impl<'ast: 'cx, 'cx, 'ghost> VisitorMut<'ast, 'ghost> for Minifier<'ast, 'cx> {
    fn visit_declaration_block_id(
        &mut self,
        declarations: DeclarationBlockId,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        cx.with_declaration_block_store(|store, _| {
            self.declaration_blocks
                .minify(declarations, store, &mut self.cx);
        });
    }

    fn visit_style_rule(
        &mut self,
        node: &mut StyleRule<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
    }

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
            CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name.as_str(),
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
        node: &mut [Selector<'ast>],
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        self.visit_selector_list_children(node, cx);
        selector::minify_selector_list(node, &mut self.cx);
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
