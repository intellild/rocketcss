mod context;
mod cross_rule_declaration_merging;
mod equality;
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
use rocketcss_common::{Allocator, GhostToken};
use rocketcss_visitor::{BoxError, Plugin, PluginContext, VisitMut, VisitorMut};

pub use context::{MinifyContext, MinifyStats};
pub use options::{MinifyOptions, Options, OptionsOp};

/// Minifies a syntax-tree node in place.
pub trait Minify {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx;
}

/// Minifies the compiler-owned compilation in place.
pub fn minify<'ast, 'ghost>(
    compilation: &mut AstContext<'ast>,
    token: &mut GhostToken<'ghost>,
    options: MinifyOptions,
) -> MinifyStats {
    try_minify(compilation, token, options)
        .expect("a parsed compilation remains structurally valid while minifying")
}

/// Fallible entry point for structural transforms.
pub fn try_minify<'ast, 'ghost>(
    compilation: &mut AstContext<'ast>,
    token: &mut GhostToken<'ghost>,
    options: MinifyOptions,
) -> Result<MinifyStats, ConcreteMutationError<'ast>> {
    let allocator = Allocator::new();
    // Parser-time context keys use lossless source fingerprints because nested NodeIds have
    // identity equality. Canonicalize them before cross-rule snapshots are published; the final
    // repair below then only observes changes made by this transform.
    compilation.refresh_context_value_identities_with_remaps_by(
        &allocator,
        equality::context_frame_fingerprint,
        equality::context_frames_are_equal,
    )?;
    let cx = MinifyContext::new(options, &allocator);
    let declaration_blocks = rules::DeclarationBlockMinifier::new(&allocator);
    let merge_adjacent_rules = options.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any);
    let mut cross_rule = merge_adjacent_rules
        .then(|| cross_rule_declaration_merging::new_cross_rule_builder(compilation, &allocator));
    let mut minifier = Minifier {
        cx,
        declaration_blocks,
    };
    compilation.transform_selector_values_in(&allocator, |_, selectors, compilation| {
        let mut visit_context = VisitMutContext::with_ast(&mut *token, compilation);
        minifier.visit_selector_list(selectors, &mut visit_context);
    });

    let mut current = compilation.first_rule_in_source();
    while let Some(rule_id) = current {
        let rule = compilation
            .rule(rule_id)
            .ok_or(ConcreteMutationError::<'ast>::UnknownRule(rule_id))?;
        current = rule.next_in_source();
        if !rule.is_live() {
            continue;
        }
        let property_block = rule.payload().owns_property_declarations();
        let block_id = rule.declaration_block();
        compilation.transform_rule_payload_with_context(rule_id, |payload, compilation| {
            let mut visit_context = VisitMutContext::with_ast(&mut *token, compilation);
            minify_rule_payload(payload, &mut minifier, &mut visit_context);
        })?;
        let Some(block_id) = block_id else {
            continue;
        };
        compilation.for_each_declaration_payload_mut_with_context(
            block_id,
            |_, payload, compilation| {
                let mut visit_context = VisitMutContext::with_ast(&mut *token, compilation);
                if property_block {
                    let rocketcss_ast::DeclarationPayload::Property(declaration) = payload else {
                        unreachable!("a property rule owns only property declarations")
                    };
                    declaration.visit_mut(&mut minifier, &mut visit_context);
                } else {
                    minify_descriptor(payload, &mut minifier, &mut visit_context);
                }
            },
        )?;
        if property_block {
            minifier.declaration_blocks.minify_compilation_block(
                compilation,
                block_id,
                &mut minifier.cx,
            );
        }
        if let Some(builder) = cross_rule.as_mut() {
            cross_rule_declaration_merging::publish_cross_rule_block(
                builder,
                compilation,
                block_id,
            )?;
        }
    }

    let context_repair = compilation.refresh_context_value_identities_with_remaps_by(
        &allocator,
        equality::context_frame_fingerprint,
        equality::context_frames_are_equal,
    )?;

    if let Some(builder) = cross_rule {
        let preserve_selector_compatibility = minifier
            .cx
            .is_enabled(Options::PRESERVE_SELECTOR_COMPATIBILITY, OptionsOp::Any);
        let representation_dirty_blocks =
            cross_rule_declaration_merging::stabilize_cross_rule_builder(
                builder,
                compilation,
                preserve_selector_compatibility,
                context_repair.effective_key_remaps(),
            )?;
        for block in representation_dirty_blocks {
            if compilation
                .declaration_block(block)
                .is_some_and(DeclarationBlockRecord::is_live)
            {
                minifier.declaration_blocks.minify_compilation_block(
                    compilation,
                    block,
                    &mut minifier.cx,
                );
            }
        }
    }
    #[cfg(debug_assertions)]
    debug_assert_eq!(compilation.validate_ast(), Ok(()));
    Ok(minifier.cx.stats())
}

fn minify_rule_payload<'ast: 'cx, 'cx, 'ghost>(
    payload: &mut CssRulePayload<'ast>,
    minifier: &mut Minifier<'cx>,
    cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
) {
    use CssRulePayload;

    match payload {
        CssRulePayload::Media(payload) => payload.query.visit_mut(minifier, cx),
        CssRulePayload::Supports(payload) => payload.condition.visit_mut(minifier, cx),
        CssRulePayload::Container(payload) => {
            if let Some(condition) = &mut payload.condition {
                condition.visit_mut(minifier, cx);
            }
        }
        CssRulePayload::Scope(payload) => {
            if let Some(selectors) = &mut payload.scope_start {
                selectors.visit_mut(minifier, cx);
            }
            if let Some(selectors) = &mut payload.scope_end {
                selectors.visit_mut(minifier, cx);
            }
        }
        CssRulePayload::Unknown(payload) => {
            minifier.minify_unknown_token_lists(&mut payload.prelude, payload.block.as_mut(), cx)
        }
        CssRulePayload::Import(payload) => payload.visit_mut(minifier, cx),
        CssRulePayload::CustomMedia(payload) => payload.query.visit_mut(minifier, cx),
        CssRulePayload::Keyframes(payload) => payload.name.visit_mut(minifier, cx),
        CssRulePayload::Keyframe(payload) => payload.selectors.visit_mut(minifier, cx),
        CssRulePayload::Page(payload) => payload.selectors.visit_mut(minifier, cx),
        CssRulePayload::FontFeatureValues(payload) => payload.name.visit_mut(minifier, cx),
        CssRulePayload::Style(_)
        | CssRulePayload::StartingStyle(_)
        | CssRulePayload::LayerStatement(_)
        | CssRulePayload::LayerBlock(_)
        | CssRulePayload::MozDocument(_)
        | CssRulePayload::CounterStyle(_)
        | CssRulePayload::Viewport(_)
        | CssRulePayload::PositionTry(_)
        | CssRulePayload::FontFace(_)
        | CssRulePayload::FontPaletteValues(_)
        | CssRulePayload::ViewTransition(_)
        | CssRulePayload::Charset(_)
        | CssRulePayload::Namespace(_)
        | CssRulePayload::PageMargin(_)
        | CssRulePayload::PageDeclarations(_)
        | CssRulePayload::Nesting(_)
        | CssRulePayload::FontFeatureSubrule(_)
        | CssRulePayload::Property(_)
        | CssRulePayload::NestedDeclarations(_) => {}
    }
}

fn minify_descriptor<'ast: 'cx, 'cx, 'ghost>(
    descriptor: &mut DeclarationPayload<'ast>,
    minifier: &mut Minifier<'cx>,
    cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
) {
    use {DeclarationPayload, PropertyRuleDescriptor};

    match descriptor {
        DeclarationPayload::Property(declaration) => declaration.visit_mut(minifier, cx),
        DeclarationPayload::FontFace(property) => property.visit_mut(minifier, cx),
        DeclarationPayload::FontPaletteValues(property) => property.visit_mut(minifier, cx),
        DeclarationPayload::ViewTransition(property) => property.visit_mut(minifier, cx),
        DeclarationPayload::FontFeature(declaration) => declaration.visit_mut(minifier, cx),
        DeclarationPayload::PropertyRule(descriptor) => match descriptor {
            PropertyRuleDescriptor::Syntax(syntax) => syntax.visit_mut(minifier, cx),
            PropertyRuleDescriptor::Inherits(_) => {}
            PropertyRuleDescriptor::InitialValue(value) => value.visit_mut(minifier, cx),
            PropertyRuleDescriptor::Unknown(property) => property.visit_mut(minifier, cx),
        },
    }
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
        compilation: &mut AstContext<'a>,
        cx: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError> {
        let stats = minify(compilation, cx.ghost_token(), self.options);
        cx.insert(stats);
        Ok(())
    }
}

struct Minifier<'cx> {
    cx: MinifyContext<'cx>,
    declaration_blocks: rules::DeclarationBlockMinifier<'cx>,
}

impl<'cx> Minifier<'cx> {
    fn minify_unknown_token_lists<'ast: 'cx, 'ghost>(
        &mut self,
        prelude: &mut rocketcss_ast::Vec<'ast, TokenOrValue<'ast>>,
        mut block: Option<&mut rocketcss_ast::Vec<'ast, TokenOrValue<'ast>>>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        let previous = self.cx.value_context;
        self.cx.value_context = Default::default();
        self.cx
            .value_context
            .set_enabled(context::ValueContextFlags::SKIP_VALUE_TRANSFORMS, true);
        if let Some(block) = block.as_deref_mut() {
            cx.rewrite_vec(block, |block, cx| {
                block.visit_mut(self, cx);
            });
        }
        cx.rewrite_vec(prelude, |prelude, cx| {
            prelude.visit_mut(self, cx);
            token::minify_token_values(prelude, &mut self.cx, cx);
        });
        if let Some(block) = block {
            cx.rewrite_vec(block, |block, cx| {
                token::minify_token_values(block, &mut self.cx, cx);
            });
        }
        self.cx.value_context = previous;
    }
}

impl<'ast: 'cx, 'cx, 'ghost> VisitorMut<'ast, 'ghost> for Minifier<'cx> {
    fn visit_url(&mut self, node: &mut Url<'ast>, cx: &mut VisitMutContext<'_, 'ast, 'ghost>) {
        if self.cx.is_enabled(Options::NORMALIZE_URLS, OptionsOp::Any)
            && let Some(normalized) = rules::normalize_url_text(cx.ast_context().str(node.url))
        {
            node.url = cx.ast_context_mut().add_str(&normalized);
            self.cx.record_value_normalized();
        }
    }

    fn visit_css_color(
        &mut self,
        node: &mut CssColor<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        if self
            .cx
            .is_enabled(Options::NORMALIZE_VALUES, OptionsOp::Any)
            && self
                .cx
                .value_context
                .is_enabled(context::ValueContextFlags::MINIFY_COLORS)
            && !self
                .cx
                .value_context
                .is_enabled(context::ValueContextFlags::SKIP_VALUE_TRANSFORMS)
            && let CssColor::Known(color) = *node
        {
            *node = CssColor::Rgba(color.rgba());
            self.cx.record_value_normalized();
        }
        node.visit_mut_children(self, cx);
    }

    fn visit_declaration(
        &mut self,
        node: &mut Declaration<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        let remove_declaration = if let Declaration::FontFamily(families) = node {
            let ids = cx
                .ast_context()
                .vec_iter(*families)
                .collect::<std::vec::Vec<_>>();
            let mut values = ids
                .iter()
                .map(|id| cx.ast_context().resolve_node(*id))
                .collect::<std::vec::Vec<_>>();
            crate::values::font::minify_font_families(&mut values, &mut self.cx, cx.ast_context());
            let remove = values.iter().all(FontFamily::is_tombstone);
            for (id, value) in ids.into_iter().zip(values) {
                cx.ast_context_mut()
                    .mutate_node(id, |node, _| *node = value);
            }
            remove
        } else {
            false
        };
        if remove_declaration {
            *node = Declaration::Tombstone;
            self.cx.record_declaration_removed();
        }
    }

    fn visit_animation(
        &mut self,
        node: &mut Animation<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        values::animation::minify_animation(node, &mut self.cx, cx);
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

    fn visit_font_weight(
        &mut self,
        node: &mut FontWeight,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_unparsed_property(
        &mut self,
        _node: &mut UnparsedProperty<'ast>,
        _cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        // Every unparsed declaration is a lossless minification barrier. The
        // parser retained it because semantic validation was not possible, so
        // even token-level transforms could change authored behavior.
    }

    fn visit_custom_property(
        &mut self,
        node: &mut CustomProperty<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        let previous = self.cx.value_context;
        self.cx.value_context = properties::custom_property_context(&self.cx);
        let name = match cx.ast_context().resolve_node(node.name) {
            CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => {
                cx.ast_context().str(name)
            }
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
            cx.rewrite_vec(&mut node.value, |values, cx| {
                for value in values.iter_mut() {
                    value.visit_mut(self, cx);
                }
                let normalized =
                    token::compact_comments_and_whitespace(values, preserve_space_after_comma, cx);
                for _ in 0..normalized {
                    self.cx.record_value_normalized();
                }
                token::minify_compacted_token_values(values, &mut self.cx, cx);
            });
        } else {
            node.visit_mut_children(self, cx);
            cx.rewrite_vec(&mut node.value, |values, cx| {
                token::minify_token_values(values, &mut self.cx, cx)
            });
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
            rules::minify_function(node, &mut self.cx, cx);
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
        rules::minify_function(node, &mut self.cx, cx);
        self.cx.value_context = previous;
    }

    fn visit_variable(
        &mut self,
        node: &mut Variable<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        if let Some(fallback) = &mut node.fallback {
            cx.rewrite_vec(fallback, |fallback, cx| {
                token::minify_token_values(fallback, &mut self.cx, cx)
            });
        }
    }

    fn visit_environment_variable(
        &mut self,
        node: &mut EnvironmentVariable<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        if let Some(fallback) = &mut node.fallback {
            cx.rewrite_vec(fallback, |fallback, cx| {
                token::minify_token_values(fallback, &mut self.cx, cx)
            });
        }
    }

    fn visit_token_or_value(
        &mut self,
        node: &mut TokenOrValue<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
    }

    fn visit_token(&mut self, node: &mut Token<'ast>, cx: &mut VisitMutContext<'_, 'ast, 'ghost>) {
        node.visit_mut_children(self, cx);
        token::minify_token(node, &mut self.cx, cx.ast_context_mut());
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
        selector::minify_selector_list(node, &mut self.cx, allocator, cx);
    }

    fn visit_media_list(
        &mut self,
        node: &mut MediaList<'ast>,
        cx: &mut VisitMutContext<'_, 'ast, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        media::minify_media_list(node, &mut self.cx, cx);
    }
}

#[cfg(test)]
mod tests;
