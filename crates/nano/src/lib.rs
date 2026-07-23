mod context;
mod length;
mod media;
mod options;
mod properties;
mod rules;
mod selector;
mod token;
mod values;

pub mod prelude;

use rocketcss_allocator::{Allocator, GhostToken};
use rocketcss_ast::{match_ignore_ascii_case, *};
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
    stylesheet: &mut StyleSheet<'a, 'ghost>,
    token: &mut GhostToken<'ghost>,
    options: MinifyOptions,
) -> MinifyStats {
    let allocator = Allocator::new();
    let mut cx = MinifyContext::new(options, &allocator);
    minify_style_sheet(stylesheet, token, &mut cx);
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
        stylesheet: &mut StyleSheet<'a, 'ghost>,
        cx: &mut PluginContext<'a, '_, 'ghost>,
    ) -> Result<(), BoxError> {
        let stats = minify(stylesheet, cx.ghost_token(), self.options);
        cx.insert(stats);
        Ok(())
    }
}

pub(crate) fn minify_style_sheet<'ast, 'ghost, 'cx>(
    stylesheet: &mut StyleSheet<'ast, 'ghost>,
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
    let mut minifier = Minifier {
        cx: owned_cx,
        declaration_blocks,
    };
    let mut visit_context = VisitMutContext::new(token);
    stylesheet.visit_mut(&mut minifier, &mut visit_context);
    rules::merge_adjacent_style_rules(
        &mut stylesheet.rules,
        token,
        &mut minifier.declaration_blocks,
        &mut minifier.cx,
    );
    let Minifier { cx: result, .. } = minifier;
    *cx = result;
}

struct Minifier<'ast, 'cx> {
    cx: MinifyContext<'cx>,
    declaration_blocks: rules::DeclarationBlockMinifier<'cx, 'ast>,
}

impl<'ast, 'ghost> VisitorMut<'ast, 'ghost> for Minifier<'ast, '_> {
    fn visit_declaration(
        &mut self,
        node: &mut Declaration<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
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
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        if !matches!(node, FontFamily::Unparsed(_) | FontFamily::Tombstone) {
            node.visit_mut_children(self, cx);
        }
    }

    fn visit_declaration_block(
        &mut self,
        node: &mut DeclarationBlock<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        self.declaration_blocks.minify(node, &mut self.cx);
    }

    fn visit_keyframe_selector(
        &mut self,
        node: &mut KeyframeSelector,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_animation(
        &mut self,
        node: &mut Animation<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_unparsed_property(
        &mut self,
        node: &mut UnparsedProperty<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let previous = self.cx.value_context;
        self.cx.value_context = properties::value_context(
            &node.property_id,
            self.cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
            self.cx
                .is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
        );
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_custom_property(
        &mut self,
        node: &mut CustomProperty<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let previous = self.cx.value_context;
        self.cx.value_context = properties::custom_property_context(&self.cx);
        let name = match &*node.name {
            CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => *name,
        };
        if match_ignore_ascii_case!(name, "--font-family" => true, _ => false) {
            self.cx.value_context.property = context::PropertyContext::Font;
        }
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
        self.cx.value_context = previous;
    }

    fn visit_function(&mut self, node: &mut Function<'ast>, cx: &mut VisitMutContext<'_, 'ghost>) {
        let previous = self.cx.value_context;
        let kind = node.kind();
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

    fn visit_variable(&mut self, node: &mut Variable<'ast>, cx: &mut VisitMutContext<'_, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_environment_variable(
        &mut self,
        node: &mut EnvironmentVariable<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_unknown_at_rule(
        &mut self,
        node: &mut UnknownAtRule<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
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
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_length_value(&mut self, node: &mut LengthValue, cx: &mut VisitMutContext<'_, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_angle(&mut self, node: &mut Angle, cx: &mut VisitMutContext<'_, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_time(&mut self, node: &mut Time, cx: &mut VisitMutContext<'_, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_resolution(&mut self, node: &mut Resolution, cx: &mut VisitMutContext<'_, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_ratio(&mut self, node: &mut Ratio, cx: &mut VisitMutContext<'_, 'ghost>) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }

    fn visit_selector_list(
        &mut self,
        node: &mut SelectorList<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_selector_list_children(node, cx);
        let allocator = self.cx.allocator();
        selector::minify_selector_list(node, &mut self.cx, allocator);
    }

    fn visit_media_list(
        &mut self,
        node: &mut MediaList<'ast>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        node.visit_mut_children(self, cx);
        node.minify(&mut self.cx);
    }
}

#[cfg(test)]
mod tests;
