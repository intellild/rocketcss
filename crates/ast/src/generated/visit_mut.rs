//! Generated typed visitor API. Regenerate with `cargo run -p rocketcss_ast_tools`.
#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use crate::*;
use std::pin::Pin;
/// Typed callbacks invoked while traversing CSS AST nodes.
pub trait VisitorMut<'a, 'ghost> {
    #[inline]
    fn enter_node(&mut self, _kind: AstType) {}
    #[inline]
    fn leave_node(&mut self, _kind: AstType) {}
    #[inline]
    fn visit_str(&mut self, _value: &mut &'a str, _cx: &mut VisitMutContext<'_, 'ghost>) {}
    #[inline]
    fn visit_css_color(&mut self, node: &mut CssColor<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_known_color(&mut self, node: &mut KnownColor, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_rgba(&mut self, node: &mut RGBA, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_lab_color(&mut self, node: &mut LABColor, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_predefined_color(
        &mut self,
        node: &mut PredefinedColor,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_float_color(&mut self, node: &mut FloatColor, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_light_dark(&mut self, node: &mut LightDark<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_system_color(&mut self, node: &mut SystemColor, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_unresolved_color(
        &mut self,
        node: &mut UnresolvedColor<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_css_rule(
        &mut self,
        node: &mut CssRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_length(&mut self, node: &mut Length<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_length_unit(&mut self, node: &mut LengthUnit, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_calc<V>(&mut self, node: &mut Calc<'a, V>, cx: &mut VisitMutContext<'_, 'ghost>)
    where
        V: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_math_function<V>(
        &mut self,
        node: &mut MathFunction<'a, V>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        V: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_rounding_strategy(
        &mut self,
        node: &mut RoundingStrategy,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_resolution(&mut self, node: &mut Resolution, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_ratio(&mut self, node: &mut Ratio, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_angle(&mut self, node: &mut Angle, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_time(&mut self, node: &mut Time, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_condition(
        &mut self,
        node: &mut MediaCondition<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_query_feature<FeatureId>(
        &mut self,
        node: &mut QueryFeature<'a, FeatureId>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        FeatureId: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature_name<FeatureId>(
        &mut self,
        node: &mut MediaFeatureName<'a, FeatureId>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        FeatureId: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature_id(
        &mut self,
        node: &mut MediaFeatureId,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature_value(
        &mut self,
        node: &mut MediaFeatureValue<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature_comparison(
        &mut self,
        node: &mut MediaFeatureComparison,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_operator(&mut self, node: &mut Operator, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_type(&mut self, node: &mut MediaType<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_qualifier(&mut self, node: &mut Qualifier, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_supports_condition(
        &mut self,
        node: &mut SupportsCondition<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_blend_mode(&mut self, node: &mut BlendMode, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_transition(
        &mut self,
        node: &mut Transition<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_timeline(
        &mut self,
        node: &mut ScrollTimeline,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_view_timeline(
        &mut self,
        node: &mut ViewTimeline<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_range(
        &mut self,
        node: &mut AnimationRange<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation(&mut self, node: &mut Animation<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_component(
        &mut self,
        node: &mut AnimationComponent<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_keyword_class(
        &mut self,
        node: &mut AnimationKeywordClass,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_supports_rule(
        &mut self,
        node: &mut SupportsRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_counter_style_rule(
        &mut self,
        node: &mut CounterStyleRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_charset_rule(
        &mut self,
        node: &mut CharsetRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_namespace_rule(
        &mut self,
        node: &mut NamespaceRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_moz_document_rule(
        &mut self,
        node: &mut MozDocumentRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_nesting_rule(
        &mut self,
        node: &mut NestingRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_nested_declarations_rule(
        &mut self,
        node: &mut NestedDeclarationsRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_viewport_rule(
        &mut self,
        node: &mut ViewportRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_custom_media_rule(
        &mut self,
        node: &mut CustomMediaRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_layer_statement_rule(
        &mut self,
        node: &mut LayerStatementRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_layer_block_rule(
        &mut self,
        node: &mut LayerBlockRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scope_rule(
        &mut self,
        node: &mut ScopeRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_starting_style_rule(
        &mut self,
        node: &mut StartingStyleRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_position_try_rule(
        &mut self,
        node: &mut PositionTryRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_unknown_at_rule(
        &mut self,
        node: &mut UnknownAtRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_position(&mut self, node: &mut Position<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_gradient_point(
        &mut self,
        node: &mut WebKitGradientPoint,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_color_stop(
        &mut self,
        node: &mut WebKitColorStop<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_image_set(&mut self, node: &mut ImageSet<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_image_set_option(
        &mut self,
        node: &mut ImageSetOption<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_background_position(
        &mut self,
        node: &mut BackgroundPosition<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_background_repeat(
        &mut self,
        node: &mut BackgroundRepeat,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_background(
        &mut self,
        node: &mut Background<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_box_shadow(&mut self, node: &mut BoxShadow<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_radius(
        &mut self,
        node: &mut BorderRadius<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image_repeat(
        &mut self,
        node: &mut BorderImageRepeat,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image_slice(
        &mut self,
        node: &mut BorderImageSlice<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image(
        &mut self,
        node: &mut BorderImage<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_color(
        &mut self,
        node: &mut BorderColor<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_style(&mut self, node: &mut BorderStyle, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_width(
        &mut self,
        node: &mut BorderWidth<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_block_color(
        &mut self,
        node: &mut BorderBlockColor<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_block_style(
        &mut self,
        node: &mut BorderBlockStyle,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_block_width(
        &mut self,
        node: &mut BorderBlockWidth<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_inline_color(
        &mut self,
        node: &mut BorderInlineColor<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_inline_style(
        &mut self,
        node: &mut BorderInlineStyle,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_inline_width(
        &mut self,
        node: &mut BorderInlineWidth<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_generic_border<S>(
        &mut self,
        node: &mut GenericBorder<'a, S>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        S: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_container_condition(
        &mut self,
        node: &mut ContainerCondition<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_container_size_feature_id(
        &mut self,
        node: &mut ContainerSizeFeatureId,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_style_query(
        &mut self,
        node: &mut StyleQuery<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_state_query(
        &mut self,
        node: &mut ScrollStateQuery<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_state_feature_id(
        &mut self,
        node: &mut ScrollStateFeatureId,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_container(&mut self, node: &mut Container<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_container_rule(
        &mut self,
        node: &mut ContainerRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_face_property(
        &mut self,
        node: &mut FontFaceProperty<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_source(&mut self, node: &mut Source<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_format(
        &mut self,
        node: &mut FontFormat<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_technology(
        &mut self,
        node: &mut FontTechnology,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_face_style(
        &mut self,
        node: &mut FontFaceStyle<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_palette_values_property(
        &mut self,
        node: &mut FontPaletteValuesProperty<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_base_palette(&mut self, node: &mut BasePalette, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_feature_subrule_type(
        &mut self,
        node: &mut FontFeatureSubruleType,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font(&mut self, node: &mut Font<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_face_rule(
        &mut self,
        node: &mut FontFaceRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_url_source(&mut self, node: &mut UrlSource<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_unicode_range(
        &mut self,
        node: &mut UnicodeRange,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_palette_values_rule(
        &mut self,
        node: &mut FontPaletteValuesRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_override_colors(
        &mut self,
        node: &mut OverrideColors<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_feature_values_rule(
        &mut self,
        node: &mut FontFeatureValuesRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_feature_subrule(
        &mut self,
        node: &mut FontFeatureSubrule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_feature_declaration(
        &mut self,
        node: &mut FontFeatureDeclaration<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_family_name(
        &mut self,
        node: &mut FamilyName<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_keyframe_selector(
        &mut self,
        node: &mut KeyframeSelector,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_keyframes_name(
        &mut self,
        node: &mut KeyframesName<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_keyframes_rule(
        &mut self,
        node: &mut KeyframesRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_keyframe(
        &mut self,
        node: &mut Keyframe<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_timeline_range_percentage(
        &mut self,
        node: &mut TimelineRangePercentage,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_aspect_ratio(&mut self, node: &mut AspectRatio, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_overflow(&mut self, node: &mut Overflow, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_inset_block(
        &mut self,
        node: &mut InsetBlock<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_inset_inline(
        &mut self,
        node: &mut InsetInline<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_inset(&mut self, node: &mut Inset<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_flow(&mut self, node: &mut FlexFlow, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_flex(&mut self, node: &mut Flex<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_place_content(
        &mut self,
        node: &mut PlaceContent,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_place_self(&mut self, node: &mut PlaceSelf, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_place_items(&mut self, node: &mut PlaceItems, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_gap(&mut self, node: &mut Gap<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_column_rule(
        &mut self,
        node: &mut ColumnRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_column_width(
        &mut self,
        node: &mut ColumnWidth<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_column_count(&mut self, node: &mut ColumnCount, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_columns(&mut self, node: &mut Columns<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_track_repeat(
        &mut self,
        node: &mut TrackRepeat<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_auto_flow(
        &mut self,
        node: &mut GridAutoFlow,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_template(
        &mut self,
        node: &mut GridTemplate<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_grid(&mut self, node: &mut Grid<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_row(&mut self, node: &mut GridRow<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_column(
        &mut self,
        node: &mut GridColumn<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_area(&mut self, node: &mut GridArea<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_margin_block(
        &mut self,
        node: &mut MarginBlock<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_margin_inline(
        &mut self,
        node: &mut MarginInline<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_margin(&mut self, node: &mut Margin<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_padding_block(
        &mut self,
        node: &mut PaddingBlock<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_padding_inline(
        &mut self,
        node: &mut PaddingInline<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_padding(&mut self, node: &mut Padding<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_margin_block(
        &mut self,
        node: &mut ScrollMarginBlock<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_margin_inline(
        &mut self,
        node: &mut ScrollMarginInline<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_margin(
        &mut self,
        node: &mut ScrollMargin<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_padding_block(
        &mut self,
        node: &mut ScrollPaddingBlock<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_padding_inline(
        &mut self,
        node: &mut ScrollPaddingInline<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_padding(
        &mut self,
        node: &mut ScrollPadding<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_page_margin_box(
        &mut self,
        node: &mut PageMarginBox,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_page_pseudo_class(
        &mut self,
        node: &mut PagePseudoClass,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_page_rule(
        &mut self,
        node: &mut PageRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_page_margin_rule(
        &mut self,
        node: &mut PageMarginRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_page_selector(
        &mut self,
        node: &mut PageSelector<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_parsed_component(
        &mut self,
        node: &mut ParsedComponent<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_multiplier(&mut self, node: &mut Multiplier, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_syntax_string(
        &mut self,
        node: &mut SyntaxString<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_syntax_component_kind(
        &mut self,
        node: &mut SyntaxComponentKind<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_unparsed_property(
        &mut self,
        node: &mut UnparsedProperty<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_custom_property(
        &mut self,
        node: &mut CustomProperty<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_property_rule(
        &mut self,
        node: &mut PropertyRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_syntax_component(
        &mut self,
        node: &mut SyntaxComponent<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_inset_rect(&mut self, node: &mut InsetRect<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_circle_shape(
        &mut self,
        node: &mut CircleShape<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_ellipse_shape(
        &mut self,
        node: &mut EllipseShape<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_polygon(&mut self, node: &mut Polygon<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_point(&mut self, node: &mut Point<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_mask(&mut self, node: &mut Mask<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_border(
        &mut self,
        node: &mut MaskBorder<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_drop_shadow(
        &mut self,
        node: &mut DropShadow<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_default_at_rule(
        &mut self,
        node: &mut DefaultAtRule,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_style_sheet(
        &mut self,
        node: &mut StyleSheet<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_rule(
        &mut self,
        node: &mut MediaRule<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_list(&mut self, node: &mut MediaList<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_query(
        &mut self,
        node: &mut MediaQuery<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_length_value(&mut self, node: &mut LengthValue, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_environment_variable(
        &mut self,
        node: &mut EnvironmentVariable<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_url(&mut self, node: &mut Url<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_variable(&mut self, node: &mut Variable<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_dashed_ident_reference(
        &mut self,
        node: &mut DashedIdentReference<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_function(&mut self, node: &mut Function<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_known_function(
        &mut self,
        node: &mut KnownFunction,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_function_replacement(
        &mut self,
        node: &mut FunctionReplacement,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_import_rule(
        &mut self,
        node: &mut ImportRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_style_rule(
        &mut self,
        mut node: Pin<&mut StyleRule<'a, 'ghost>>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(&mut node, self, cx);
    }
    #[inline]
    fn visit_declaration_block(
        &mut self,
        node: &mut DeclarationBlock<'a, 'ghost>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_transform(
        &mut self,
        node: &mut TextTransform,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_indent(
        &mut self,
        node: &mut TextIndent<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration(
        &mut self,
        node: &mut TextDecoration<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis(
        &mut self,
        node: &mut TextEmphasis<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_position(
        &mut self,
        node: &mut TextEmphasisPosition,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_shadow(
        &mut self,
        node: &mut TextShadow<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_matrix_for_float(
        &mut self,
        node: &mut MatrixForFloat,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_matrix_3_d_for_float(
        &mut self,
        node: &mut Matrix3DForFloat,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_rotate(&mut self, node: &mut Rotate, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_cursor(&mut self, node: &mut Cursor<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_cursor_image(
        &mut self,
        node: &mut CursorImage<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_caret(&mut self, node: &mut Caret<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_list_style(&mut self, node: &mut ListStyle<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_composes(&mut self, node: &mut Composes<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_color_scheme(&mut self, node: &mut ColorScheme, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_property(
        &mut self,
        node: &mut ViewTransitionProperty<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_navigation(&mut self, node: &mut Navigation, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_part_selector(
        &mut self,
        node: &mut ViewTransitionPartSelector<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_rule(
        &mut self,
        node: &mut ViewTransitionRule<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_selector(&mut self, node: &mut Selector<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_selector_component(
        &mut self,
        node: &mut SelectorComponent<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_combinator(&mut self, node: &mut Combinator, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_attr_selector(
        &mut self,
        node: &mut AttrSelector<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_namespace_constraint(
        &mut self,
        node: &mut NamespaceConstraint<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_attr_operation(
        &mut self,
        node: &mut AttrOperation<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_parsed_case_sensitivity(
        &mut self,
        node: &mut ParsedCaseSensitivity,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_attr_selector_operator(
        &mut self,
        node: &mut AttrSelectorOperator,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_nth_type(&mut self, node: &mut NthType, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_nth_selector_data(
        &mut self,
        node: &mut NthSelectorData,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_direction(&mut self, node: &mut Direction, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_pseudo_class(
        &mut self,
        node: &mut PseudoClass<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_class(
        &mut self,
        node: &mut WebKitScrollbarPseudoClass,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_pseudo_element(
        &mut self,
        node: &mut PseudoElement<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_element(
        &mut self,
        node: &mut WebKitScrollbarPseudoElement,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_part_name(
        &mut self,
        node: &mut ViewTransitionPartName<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_span(&mut self, node: &mut Span, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_token_or_value(
        &mut self,
        node: &mut TokenOrValue<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_unit(&mut self, node: &mut Unit, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_token(&mut self, node: &mut Token<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_specifier(&mut self, node: &mut Specifier<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_name(
        &mut self,
        node: &mut AnimationName<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_environment_variable_name(
        &mut self,
        node: &mut EnvironmentVariableName<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_ua_environment_variable(
        &mut self,
        node: &mut UAEnvironmentVariable,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_align_content(
        &mut self,
        node: &mut AlignContent,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_baseline_position(
        &mut self,
        node: &mut BaselinePosition,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_content_distribution(
        &mut self,
        node: &mut ContentDistribution,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_overflow_position(
        &mut self,
        node: &mut OverflowPosition,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_content_position(
        &mut self,
        node: &mut ContentPosition,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_justify_content(
        &mut self,
        node: &mut JustifyContent,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_align_self(&mut self, node: &mut AlignSelf, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_self_position(
        &mut self,
        node: &mut SelfPosition,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_justify_self(&mut self, node: &mut JustifySelf, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_align_items(&mut self, node: &mut AlignItems, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_justify_items(
        &mut self,
        node: &mut JustifyItems,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_legacy_justify(
        &mut self,
        node: &mut LegacyJustify,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_gap_value(&mut self, node: &mut GapValue<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_easing_function(
        &mut self,
        node: &mut EasingFunction,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_step_position(
        &mut self,
        node: &mut StepPosition,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_iteration_count(
        &mut self,
        node: &mut AnimationIterationCount,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_direction(
        &mut self,
        node: &mut AnimationDirection,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_play_state(
        &mut self,
        node: &mut AnimationPlayState,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_fill_mode(
        &mut self,
        node: &mut AnimationFillMode,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_composition(
        &mut self,
        node: &mut AnimationComposition,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_timeline(
        &mut self,
        node: &mut AnimationTimeline<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_axis(&mut self, node: &mut ScrollAxis, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scroller(&mut self, node: &mut Scroller, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_attachment_range(
        &mut self,
        node: &mut AnimationAttachmentRange<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_timeline_range_name(
        &mut self,
        node: &mut TimelineRangeName,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_line_style(&mut self, node: &mut LineStyle, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_side_width(
        &mut self,
        node: &mut BorderSideWidth<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_length_or_number(
        &mut self,
        node: &mut LengthOrNumber<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image_repeat_keyword(
        &mut self,
        node: &mut BorderImageRepeatKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image_side_width(
        &mut self,
        node: &mut BorderImageSideWidth<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_outline_style(
        &mut self,
        node: &mut OutlineStyle,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_display(&mut self, node: &mut Display, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_display_keyword(
        &mut self,
        node: &mut DisplayKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_display_inside(
        &mut self,
        node: &mut DisplayInside,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_display_outside(
        &mut self,
        node: &mut DisplayOutside,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_visibility(&mut self, node: &mut Visibility, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_size(&mut self, node: &mut Size<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_max_size(&mut self, node: &mut MaxSize<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_box_sizing(&mut self, node: &mut BoxSizing, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_overflow_keyword(
        &mut self,
        node: &mut OverflowKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_overflow(
        &mut self,
        node: &mut TextOverflow,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_position_property(
        &mut self,
        node: &mut PositionProperty,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_size_2_d<T>(&mut self, node: &mut Size2D<'a, T>, cx: &mut VisitMutContext<'_, 'ghost>)
    where
        T: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_rect<T>(&mut self, node: &mut Rect<'a, T>, cx: &mut VisitMutContext<'_, 'ghost>)
    where
        T: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_box_decoration_break(
        &mut self,
        node: &mut BoxDecorationBreak,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_z_index(&mut self, node: &mut ZIndex, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_container_type(
        &mut self,
        node: &mut ContainerType,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_container_name_list(
        &mut self,
        node: &mut ContainerNameList<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_filter_list(
        &mut self,
        node: &mut FilterList<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_filter(&mut self, node: &mut Filter<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_direction(
        &mut self,
        node: &mut FlexDirection,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_wrap(&mut self, node: &mut FlexWrap, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_box_orient(&mut self, node: &mut BoxOrient, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_box_direction(
        &mut self,
        node: &mut BoxDirection,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_box_align(&mut self, node: &mut BoxAlign, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_box_pack(&mut self, node: &mut BoxPack, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_box_lines(&mut self, node: &mut BoxLines, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_pack(&mut self, node: &mut FlexPack, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_item_align(
        &mut self,
        node: &mut FlexItemAlign,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_line_pack(
        &mut self,
        node: &mut FlexLinePack,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_weight(&mut self, node: &mut FontWeight, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_absolute_font_weight(
        &mut self,
        node: &mut AbsoluteFontWeight,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_size(&mut self, node: &mut FontSize<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_absolute_font_size(
        &mut self,
        node: &mut AbsoluteFontSize,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_relative_font_size(
        &mut self,
        node: &mut RelativeFontSize,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_stretch(&mut self, node: &mut FontStretch, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_stretch_keyword(
        &mut self,
        node: &mut FontStretchKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_family(
        &mut self,
        node: &mut FontFamily<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_style(&mut self, node: &mut FontStyle, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_font_variant_caps(
        &mut self,
        node: &mut FontVariantCaps,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_line_height(
        &mut self,
        node: &mut LineHeight<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_vertical_align(
        &mut self,
        node: &mut VerticalAlign<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_vertical_align_keyword(
        &mut self,
        node: &mut VerticalAlignKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_track_sizing(
        &mut self,
        node: &mut TrackSizing<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_track_list_item(
        &mut self,
        node: &mut TrackListItem<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_track_size(&mut self, node: &mut TrackSize<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_track_breadth(
        &mut self,
        node: &mut TrackBreadth<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_repeat_count(&mut self, node: &mut RepeatCount, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_auto_flow_direction(
        &mut self,
        node: &mut AutoFlowDirection,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_template_areas(
        &mut self,
        node: &mut GridTemplateAreas<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_line(&mut self, node: &mut GridLine<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_image(&mut self, node: &mut Image<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_gradient(&mut self, node: &mut Gradient<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_gradient(
        &mut self,
        node: &mut WebKitGradient<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_line_direction(
        &mut self,
        node: &mut LineDirection,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_horizontal_position_keyword(
        &mut self,
        node: &mut HorizontalPositionKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_vertical_position_keyword(
        &mut self,
        node: &mut VerticalPositionKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_gradient_item<D>(
        &mut self,
        node: &mut GradientItem<'a, D>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        D: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_dimension_percentage<D>(
        &mut self,
        node: &mut DimensionPercentage<'a, D>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        D: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_position_component<S>(
        &mut self,
        node: &mut PositionComponent<'a, S>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        S: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_ending_shape(
        &mut self,
        node: &mut EndingShape<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_ellipse(&mut self, node: &mut Ellipse<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_shape_extent(&mut self, node: &mut ShapeExtent, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_circle(&mut self, node: &mut Circle<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_gradient_point_component<S>(
        &mut self,
        node: &mut WebKitGradientPointComponent<S>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        S: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_number_or_percentage(
        &mut self,
        node: &mut NumberOrPercentage,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_background_size(
        &mut self,
        node: &mut BackgroundSize<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_length_percentage_or_auto(
        &mut self,
        node: &mut LengthPercentageOrAuto<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_background_repeat_keyword(
        &mut self,
        node: &mut BackgroundRepeatKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_background_attachment(
        &mut self,
        node: &mut BackgroundAttachment,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_background_clip(
        &mut self,
        node: &mut BackgroundClip,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_background_origin(
        &mut self,
        node: &mut BackgroundOrigin,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_list_style_type(
        &mut self,
        node: &mut ListStyleType<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_counter_style(
        &mut self,
        node: &mut CounterStyle<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_symbols_type(&mut self, node: &mut SymbolsType, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_predefined_counter_style(
        &mut self,
        node: &mut PredefinedCounterStyle,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_symbol(&mut self, node: &mut Symbol<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_list_style_position(
        &mut self,
        node: &mut ListStylePosition,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_marker_side(&mut self, node: &mut MarkerSide, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_mode(&mut self, node: &mut MaskMode, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_clip(&mut self, node: &mut MaskClip, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_composite(
        &mut self,
        node: &mut MaskComposite,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_type(&mut self, node: &mut MaskType, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_border_mode(
        &mut self,
        node: &mut MaskBorderMode,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_mask_composite(
        &mut self,
        node: &mut WebKitMaskComposite,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_mask_source_type(
        &mut self,
        node: &mut WebKitMaskSourceType,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_css_wide_keyword(
        &mut self,
        node: &mut CSSWideKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_css_wide_or<T>(
        &mut self,
        node: &mut CSSWideOr<T>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) where
        T: VisitMut<'a, 'ghost>,
    {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_custom_property_name(
        &mut self,
        node: &mut CustomPropertyName<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_clip_path(&mut self, node: &mut ClipPath<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_geometry_box(&mut self, node: &mut GeometryBox, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_basic_shape(
        &mut self,
        node: &mut BasicShape<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_shape_radius(
        &mut self,
        node: &mut ShapeRadius<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_svg_paint(&mut self, node: &mut SVGPaint<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_svg_paint_fallback(
        &mut self,
        node: &mut SVGPaintFallback<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_fill_rule(&mut self, node: &mut FillRule, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_stroke_linecap(
        &mut self,
        node: &mut StrokeLinecap,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_stroke_linejoin(
        &mut self,
        node: &mut StrokeLinejoin,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_stroke_dasharray(
        &mut self,
        node: &mut StrokeDasharray<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_marker(&mut self, node: &mut Marker<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_color_interpolation(
        &mut self,
        node: &mut ColorInterpolation,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_color_rendering(
        &mut self,
        node: &mut ColorRendering,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_shape_rendering(
        &mut self,
        node: &mut ShapeRendering,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_rendering(
        &mut self,
        node: &mut TextRendering,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_image_rendering(
        &mut self,
        node: &mut ImageRendering,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_transform_case(
        &mut self,
        node: &mut TextTransformCase,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_white_space(&mut self, node: &mut WhiteSpace, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_word_break(&mut self, node: &mut WordBreak, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_line_break(&mut self, node: &mut LineBreak, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_hyphens(&mut self, node: &mut Hyphens, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_overflow_wrap(
        &mut self,
        node: &mut OverflowWrap,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_align(&mut self, node: &mut TextAlign, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_align_last(
        &mut self,
        node: &mut TextAlignLast,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_justify(&mut self, node: &mut TextJustify, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_spacing(&mut self, node: &mut Spacing<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration_line(
        &mut self,
        node: &mut TextDecorationLine<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_exclusive_text_decoration_line(
        &mut self,
        node: &mut ExclusiveTextDecorationLine,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_other_text_decoration_line(
        &mut self,
        node: &mut OtherTextDecorationLine,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration_style(
        &mut self,
        node: &mut TextDecorationStyle,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration_thickness(
        &mut self,
        node: &mut TextDecorationThickness<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration_skip_ink(
        &mut self,
        node: &mut TextDecorationSkipInk,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_style(
        &mut self,
        node: &mut TextEmphasisStyle<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_fill_mode(
        &mut self,
        node: &mut TextEmphasisFillMode,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_shape(
        &mut self,
        node: &mut TextEmphasisShape,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_position_horizontal(
        &mut self,
        node: &mut TextEmphasisPositionHorizontal,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_position_vertical(
        &mut self,
        node: &mut TextEmphasisPositionVertical,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_size_adjust(
        &mut self,
        node: &mut TextSizeAdjust,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_text_direction(
        &mut self,
        node: &mut TextDirection,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_unicode_bidi(&mut self, node: &mut UnicodeBidi, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_transform(&mut self, node: &mut Transform<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_transform_style(
        &mut self,
        node: &mut TransformStyle,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_transform_box(
        &mut self,
        node: &mut TransformBox,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_backface_visibility(
        &mut self,
        node: &mut BackfaceVisibility,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_perspective(
        &mut self,
        node: &mut Perspective<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_translate(&mut self, node: &mut Translate<'a>, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_scale(&mut self, node: &mut Scale, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_resize(&mut self, node: &mut Resize, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_cursor_keyword(
        &mut self,
        node: &mut CursorKeyword,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_color_or_auto(
        &mut self,
        node: &mut ColorOrAuto<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_caret_shape(&mut self, node: &mut CaretShape, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_user_select(&mut self, node: &mut UserSelect, cx: &mut VisitMutContext<'_, 'ghost>) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_appearance(
        &mut self,
        node: &mut Appearance<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_print_color_adjust(
        &mut self,
        node: &mut PrintColorAdjust,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_name(
        &mut self,
        node: &mut ViewTransitionName<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_none_or_custom_ident_list(
        &mut self,
        node: &mut NoneOrCustomIdentList<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_group(
        &mut self,
        node: &mut ViewTransitionGroup<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature(
        &mut self,
        node: &mut MediaFeature<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_media_feature_children(node, cx);
    }
    ///Continues traversal of [`MediaFeature`] without redispatching its visitor callback.
    fn visit_media_feature_children(
        &mut self,
        node: &mut MediaFeature<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::MediaFeature);
        VisitMut::visit_mut(node, visitor, cx);
        visitor.leave_node(AstType::MediaFeature);
    }
    #[inline]
    fn visit_container_size_feature(
        &mut self,
        node: &mut ContainerSizeFeature<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_container_size_feature_children(node, cx);
    }
    ///Continues traversal of [`ContainerSizeFeature`] without redispatching its visitor callback.
    fn visit_container_size_feature_children(
        &mut self,
        node: &mut ContainerSizeFeature<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::ContainerSizeFeature);
        VisitMut::visit_mut(node, visitor, cx);
        visitor.leave_node(AstType::ContainerSizeFeature);
    }
    #[inline]
    fn visit_scroll_state_feature(
        &mut self,
        node: &mut ScrollStateFeature<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_scroll_state_feature_children(node, cx);
    }
    ///Continues traversal of [`ScrollStateFeature`] without redispatching its visitor callback.
    fn visit_scroll_state_feature_children(
        &mut self,
        node: &mut ScrollStateFeature<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::ScrollStateFeature);
        VisitMut::visit_mut(node, visitor, cx);
        visitor.leave_node(AstType::ScrollStateFeature);
    }
    #[inline]
    fn visit_selector_list(
        &mut self,
        node: &mut SelectorList<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_selector_list_children(node, cx);
    }
    ///Continues traversal of [`SelectorList`] without redispatching its visitor callback.
    fn visit_selector_list_children(
        &mut self,
        node: &mut SelectorList<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::SelectorList);
        for value_0 in (node).iter_mut() {
            VisitMut::visit_mut(value_0, visitor, cx);
        }
        visitor.leave_node(AstType::SelectorList);
    }
    #[inline]
    fn visit_animation_range_start(
        &mut self,
        node: &mut AnimationRangeStart<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_animation_range_start_children(node, cx);
    }
    ///Continues traversal of [`AnimationRangeStart`] without redispatching its visitor callback.
    fn visit_animation_range_start_children(
        &mut self,
        node: &mut AnimationRangeStart<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::AnimationRangeStart);
        VisitMut::visit_mut(node, visitor, cx);
        visitor.leave_node(AstType::AnimationRangeStart);
    }
    #[inline]
    fn visit_animation_range_end(
        &mut self,
        node: &mut AnimationRangeEnd<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_animation_range_end_children(node, cx);
    }
    ///Continues traversal of [`AnimationRangeEnd`] without redispatching its visitor callback.
    fn visit_animation_range_end_children(
        &mut self,
        node: &mut AnimationRangeEnd<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::AnimationRangeEnd);
        VisitMut::visit_mut(node, visitor, cx);
        visitor.leave_node(AstType::AnimationRangeEnd);
    }
    #[inline]
    fn visit_length_percentage(
        &mut self,
        node: &mut LengthPercentage<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_length_percentage_children(node, cx);
    }
    ///Continues traversal of [`LengthPercentage`] without redispatching its visitor callback.
    fn visit_length_percentage_children(
        &mut self,
        node: &mut LengthPercentage<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::LengthPercentage);
        VisitMut::visit_mut(node, visitor, cx);
        visitor.leave_node(AstType::LengthPercentage);
    }
    #[inline]
    fn visit_angle_percentage(
        &mut self,
        node: &mut AnglePercentage<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        self.visit_angle_percentage_children(node, cx);
    }
    ///Continues traversal of [`AnglePercentage`] without redispatching its visitor callback.
    fn visit_angle_percentage_children(
        &mut self,
        node: &mut AnglePercentage<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::AnglePercentage);
        VisitMut::visit_mut(node, visitor, cx);
        visitor.leave_node(AstType::AnglePercentage);
    }
    #[inline]
    fn visit_declaration(
        &mut self,
        node: &mut Declaration<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_property_id(
        &mut self,
        node: &mut PropertyId<'a>,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
    #[inline]
    fn visit_vendor_prefix(
        &mut self,
        node: &mut VendorPrefix,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut_children(node, self, cx);
    }
}
/// Traversal implemented by CSS AST nodes.
pub trait VisitMut<'a, 'ghost> {
    /// Dispatches this node to its typed visitor callback.
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ghost>,
    );
    /// Continues traversal without redispatching this node's visitor callback.
    #[inline]
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        _visitor: &mut VisitorT,
        _cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
    }
}
macro_rules! impl_leaf_visit_mut {
    ($($ty:ty),+ $(,)?) => {
        $(impl < 'a, 'ghost > VisitMut < 'a, 'ghost > for $ty { fn visit_mut < VisitorT :
        ? Sized + VisitorMut < 'a, 'ghost >> (& mut self, _visitor : & mut VisitorT, _cx
        : & mut VisitMutContext < '_, 'ghost >,) {} })+
    };
}
impl_leaf_visit_mut!(bool, char, f32, i32, u8, u16, u32, usize);
impl<'a, 'ghost, T: ?Sized + VisitMut<'a, 'ghost>> VisitMut<'a, 'ghost>
    for rocketcss_allocator::boxed::Box<'a, T>
{
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        VisitMut::visit_mut(self.as_mut(), visitor, cx);
    }
}
impl<'a, 'ghost, T: VisitMut<'a, 'ghost> + Unpin> VisitMut<'a, 'ghost>
    for rocketcss_allocator::vec::Vec<'a, T>
{
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        for value in self {
            VisitMut::visit_mut(value, visitor, cx);
        }
    }
}
impl<'a, 'ghost, T: VisitMut<'a, 'ghost>> VisitMut<'a, 'ghost> for Option<T> {
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        if let Some(value) = self {
            VisitMut::visit_mut(value, visitor, cx);
        }
    }
}
impl<'a, 'ghost> VisitMut<'a, 'ghost> for &'a str {
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        visitor.visit_str(self, cx);
    }
}
impl<'a, 'ghost> VisitMut<'a, 'ghost> for VendorPrefix {
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        visitor.visit_vendor_prefix(self, cx);
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        visitor: &mut VisitorT,
        _cx: &mut VisitMutContext<'_, 'ghost>,
    ) {
        visitor.enter_node(AstType::VendorPrefix);
        visitor.leave_node(AstType::VendorPrefix);
    }
}
