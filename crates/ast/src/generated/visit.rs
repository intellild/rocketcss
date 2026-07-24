//! Generated typed visitor API. Regenerate with `cargo run -p rocketcss_ast_tools`.
#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use crate::*;
/// Typed callbacks invoked while traversing CSS AST nodes.
pub trait Visitor<'a, 'ghost> {
    #[inline]
    fn enter_node(&mut self, _kind: AstType) {}
    #[inline]
    fn leave_node(&mut self, _kind: AstType) {}
    #[inline]
    fn visit_str(&mut self, _value: &&'a str, _cx: &VisitContext<'_, 'ghost>) {}
    #[inline]
    fn visit_css_color(&mut self, node: &CssColor<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_known_color(&mut self, node: &KnownColor, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_rgba(&mut self, node: &RGBA, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_lab_color(&mut self, node: &LABColor, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_predefined_color(&mut self, node: &PredefinedColor, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_float_color(&mut self, node: &FloatColor, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_light_dark(&mut self, node: &LightDark<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_system_color(&mut self, node: &SystemColor, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_unresolved_color(
        &mut self,
        node: &UnresolvedColor<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_css_rule(&mut self, node: &CssRule<'a, 'ghost>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_length(&mut self, node: &Length<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_length_unit(&mut self, node: &LengthUnit, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_calc<V>(&mut self, node: &Calc<'a, V>, cx: &VisitContext<'_, 'ghost>)
    where
        V: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_math_function<V>(&mut self, node: &MathFunction<'a, V>, cx: &VisitContext<'_, 'ghost>)
    where
        V: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_rounding_strategy(&mut self, node: &RoundingStrategy, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_resolution(&mut self, node: &Resolution, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_ratio(&mut self, node: &Ratio, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_angle(&mut self, node: &Angle, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_time(&mut self, node: &Time, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_condition(&mut self, node: &MediaCondition<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_query_feature<FeatureId>(
        &mut self,
        node: &QueryFeature<'a, FeatureId>,
        cx: &VisitContext<'_, 'ghost>,
    ) where
        FeatureId: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature_name<FeatureId>(
        &mut self,
        node: &MediaFeatureName<'a, FeatureId>,
        cx: &VisitContext<'_, 'ghost>,
    ) where
        FeatureId: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature_id(&mut self, node: &MediaFeatureId, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature_value(
        &mut self,
        node: &MediaFeatureValue<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature_comparison(
        &mut self,
        node: &MediaFeatureComparison,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_operator(&mut self, node: &Operator, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_type(&mut self, node: &MediaType<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_qualifier(&mut self, node: &Qualifier, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_supports_condition(
        &mut self,
        node: &SupportsCondition<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_blend_mode(&mut self, node: &BlendMode, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_transition(&mut self, node: &Transition<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_timeline(&mut self, node: &ScrollTimeline, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_view_timeline(&mut self, node: &ViewTimeline<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_range(&mut self, node: &AnimationRange<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation(&mut self, node: &Animation<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_component(
        &mut self,
        node: &AnimationComponent<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_keyword_class(
        &mut self,
        node: &AnimationKeywordClass,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_supports_rule(
        &mut self,
        node: &SupportsRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_counter_style_rule(
        &mut self,
        node: &CounterStyleRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_charset_rule(&mut self, node: &CharsetRule<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_namespace_rule(&mut self, node: &NamespaceRule<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_moz_document_rule(
        &mut self,
        node: &MozDocumentRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_nesting_rule(
        &mut self,
        node: &NestingRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_nested_declarations_rule(
        &mut self,
        node: &NestedDeclarationsRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_viewport_rule(
        &mut self,
        node: &ViewportRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_custom_media_rule(
        &mut self,
        node: &CustomMediaRule<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_layer_statement_rule(
        &mut self,
        node: &LayerStatementRule<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_layer_block_rule(
        &mut self,
        node: &LayerBlockRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scope_rule(&mut self, node: &ScopeRule<'a, 'ghost>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_starting_style_rule(
        &mut self,
        node: &StartingStyleRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_position_try_rule(
        &mut self,
        node: &PositionTryRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_unknown_at_rule(&mut self, node: &UnknownAtRule<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_position(&mut self, node: &Position<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_gradient_point(
        &mut self,
        node: &WebKitGradientPoint,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_color_stop(
        &mut self,
        node: &WebKitColorStop<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_image_set(&mut self, node: &ImageSet<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_image_set_option(&mut self, node: &ImageSetOption<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_background_position(
        &mut self,
        node: &BackgroundPosition<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_background_repeat(&mut self, node: &BackgroundRepeat, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_background(&mut self, node: &Background<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_box_shadow(&mut self, node: &BoxShadow<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_radius(&mut self, node: &BorderRadius<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image_repeat(
        &mut self,
        node: &BorderImageRepeat,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image_slice(
        &mut self,
        node: &BorderImageSlice<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image(&mut self, node: &BorderImage<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_color(&mut self, node: &BorderColor<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_style(&mut self, node: &BorderStyle, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_width(&mut self, node: &BorderWidth<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_block_color(
        &mut self,
        node: &BorderBlockColor<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_block_style(&mut self, node: &BorderBlockStyle, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_block_width(
        &mut self,
        node: &BorderBlockWidth<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_inline_color(
        &mut self,
        node: &BorderInlineColor<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_inline_style(
        &mut self,
        node: &BorderInlineStyle,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_inline_width(
        &mut self,
        node: &BorderInlineWidth<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_generic_border<S>(
        &mut self,
        node: &GenericBorder<'a, S>,
        cx: &VisitContext<'_, 'ghost>,
    ) where
        S: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_container_condition(
        &mut self,
        node: &ContainerCondition<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_container_size_feature_id(
        &mut self,
        node: &ContainerSizeFeatureId,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_style_query(&mut self, node: &StyleQuery<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_state_query(
        &mut self,
        node: &ScrollStateQuery<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_state_feature_id(
        &mut self,
        node: &ScrollStateFeatureId,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_container(&mut self, node: &Container<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_container_rule(
        &mut self,
        node: &ContainerRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_face_property(
        &mut self,
        node: &FontFaceProperty<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_source(&mut self, node: &Source<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_format(&mut self, node: &FontFormat<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_technology(&mut self, node: &FontTechnology, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_face_style(&mut self, node: &FontFaceStyle<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_palette_values_property(
        &mut self,
        node: &FontPaletteValuesProperty<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_base_palette(&mut self, node: &BasePalette, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_feature_subrule_type(
        &mut self,
        node: &FontFeatureSubruleType,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font(&mut self, node: &Font<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_face_rule(&mut self, node: &FontFaceRule<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_url_source(&mut self, node: &UrlSource<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_unicode_range(&mut self, node: &UnicodeRange, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_palette_values_rule(
        &mut self,
        node: &FontPaletteValuesRule<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_override_colors(&mut self, node: &OverrideColors<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_feature_values_rule(
        &mut self,
        node: &FontFeatureValuesRule<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_feature_subrule(
        &mut self,
        node: &FontFeatureSubrule<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_feature_declaration(
        &mut self,
        node: &FontFeatureDeclaration<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_family_name(&mut self, node: &FamilyName<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_keyframe_selector(&mut self, node: &KeyframeSelector, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_keyframes_name(&mut self, node: &KeyframesName<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_keyframes_rule(
        &mut self,
        node: &KeyframesRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_keyframe(&mut self, node: &Keyframe<'a, 'ghost>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_timeline_range_percentage(
        &mut self,
        node: &TimelineRangePercentage,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_aspect_ratio(&mut self, node: &AspectRatio, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_overflow(&mut self, node: &Overflow, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_inset_block(&mut self, node: &InsetBlock<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_inset_inline(&mut self, node: &InsetInline<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_inset(&mut self, node: &Inset<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_flow(&mut self, node: &FlexFlow, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_flex(&mut self, node: &Flex<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_place_content(&mut self, node: &PlaceContent, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_place_self(&mut self, node: &PlaceSelf, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_place_items(&mut self, node: &PlaceItems, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_gap(&mut self, node: &Gap<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_column_rule(&mut self, node: &ColumnRule<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_column_width(&mut self, node: &ColumnWidth<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_column_count(&mut self, node: &ColumnCount, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_columns(&mut self, node: &Columns<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_track_repeat(&mut self, node: &TrackRepeat<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_auto_flow(&mut self, node: &GridAutoFlow, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_template(&mut self, node: &GridTemplate<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_grid(&mut self, node: &Grid<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_row(&mut self, node: &GridRow<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_column(&mut self, node: &GridColumn<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_area(&mut self, node: &GridArea<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_margin_block(&mut self, node: &MarginBlock<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_margin_inline(&mut self, node: &MarginInline<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_margin(&mut self, node: &Margin<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_padding_block(&mut self, node: &PaddingBlock<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_padding_inline(&mut self, node: &PaddingInline<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_padding(&mut self, node: &Padding<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_margin_block(
        &mut self,
        node: &ScrollMarginBlock<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_margin_inline(
        &mut self,
        node: &ScrollMarginInline<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_margin(&mut self, node: &ScrollMargin<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_padding_block(
        &mut self,
        node: &ScrollPaddingBlock<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_padding_inline(
        &mut self,
        node: &ScrollPaddingInline<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_padding(&mut self, node: &ScrollPadding<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_page_margin_box(&mut self, node: &PageMarginBox, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_page_pseudo_class(&mut self, node: &PagePseudoClass, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_page_rule(&mut self, node: &PageRule<'a, 'ghost>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_page_margin_rule(
        &mut self,
        node: &PageMarginRule<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_page_selector(&mut self, node: &PageSelector<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_parsed_component(
        &mut self,
        node: &ParsedComponent<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_multiplier(&mut self, node: &Multiplier, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_syntax_string(&mut self, node: &SyntaxString<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_syntax_component_kind(
        &mut self,
        node: &SyntaxComponentKind<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_unparsed_property(
        &mut self,
        node: &UnparsedProperty<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_custom_property(&mut self, node: &CustomProperty<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_property_rule(&mut self, node: &PropertyRule<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_syntax_component(
        &mut self,
        node: &SyntaxComponent<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_inset_rect(&mut self, node: &InsetRect<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_circle_shape(&mut self, node: &CircleShape<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_ellipse_shape(&mut self, node: &EllipseShape<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_polygon(&mut self, node: &Polygon<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_point(&mut self, node: &Point<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_mask(&mut self, node: &Mask<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_border(&mut self, node: &MaskBorder<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_drop_shadow(&mut self, node: &DropShadow<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_default_at_rule(&mut self, node: &DefaultAtRule, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_style_sheet(&mut self, node: &StyleSheet<'a, 'ghost>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_rule(&mut self, node: &MediaRule<'a, 'ghost>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_list(&mut self, node: &MediaList<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_query(&mut self, node: &MediaQuery<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_length_value(&mut self, node: &LengthValue, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_environment_variable(
        &mut self,
        node: &EnvironmentVariable<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_url(&mut self, node: &Url<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_variable(&mut self, node: &Variable<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_dashed_ident_reference(
        &mut self,
        node: &DashedIdentReference<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_function(&mut self, node: &Function<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_known_function(&mut self, node: &KnownFunction, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_function_replacement(
        &mut self,
        node: &FunctionReplacement,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_import_rule(&mut self, node: &ImportRule<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_style_rule(&mut self, node: &StyleRule<'a, 'ghost>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_declaration_block(
        &mut self,
        node: &DeclarationBlock<'a, 'ghost>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_transform(&mut self, node: &TextTransform, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_indent(&mut self, node: &TextIndent<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration(&mut self, node: &TextDecoration<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis(&mut self, node: &TextEmphasis<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_position(
        &mut self,
        node: &TextEmphasisPosition,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_shadow(&mut self, node: &TextShadow<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_matrix_for_float(&mut self, node: &MatrixForFloat, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_matrix_3_d_for_float(
        &mut self,
        node: &Matrix3DForFloat,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_rotate(&mut self, node: &Rotate, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_cursor(&mut self, node: &Cursor<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_cursor_image(&mut self, node: &CursorImage<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_caret(&mut self, node: &Caret<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_list_style(&mut self, node: &ListStyle<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_composes(&mut self, node: &Composes<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_color_scheme(&mut self, node: &ColorScheme, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_property(
        &mut self,
        node: &ViewTransitionProperty<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_navigation(&mut self, node: &Navigation, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_part_selector(
        &mut self,
        node: &ViewTransitionPartSelector<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_rule(
        &mut self,
        node: &ViewTransitionRule<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_selector(&mut self, node: &Selector<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_selector_component(
        &mut self,
        node: &SelectorComponent<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_combinator(&mut self, node: &Combinator, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_attr_selector(&mut self, node: &AttrSelector<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_namespace_constraint(
        &mut self,
        node: &NamespaceConstraint<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_attr_operation(&mut self, node: &AttrOperation<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_parsed_case_sensitivity(
        &mut self,
        node: &ParsedCaseSensitivity,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_attr_selector_operator(
        &mut self,
        node: &AttrSelectorOperator,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_nth_type(&mut self, node: &NthType, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_nth_selector_data(&mut self, node: &NthSelectorData, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_direction(&mut self, node: &Direction, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_pseudo_class(&mut self, node: &PseudoClass<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_class(
        &mut self,
        node: &WebKitScrollbarPseudoClass,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_pseudo_element(&mut self, node: &PseudoElement<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_element(
        &mut self,
        node: &WebKitScrollbarPseudoElement,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_part_name(
        &mut self,
        node: &ViewTransitionPartName<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_span(&mut self, node: &Span, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_token_or_value(&mut self, node: &TokenOrValue<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_unit(&mut self, node: &Unit, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_token(&mut self, node: &Token<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_specifier(&mut self, node: &Specifier<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_name(&mut self, node: &AnimationName<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_environment_variable_name(
        &mut self,
        node: &EnvironmentVariableName<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_ua_environment_variable(
        &mut self,
        node: &UAEnvironmentVariable,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_align_content(&mut self, node: &AlignContent, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_baseline_position(&mut self, node: &BaselinePosition, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_content_distribution(
        &mut self,
        node: &ContentDistribution,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_overflow_position(&mut self, node: &OverflowPosition, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_content_position(&mut self, node: &ContentPosition, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_justify_content(&mut self, node: &JustifyContent, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_align_self(&mut self, node: &AlignSelf, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_self_position(&mut self, node: &SelfPosition, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_justify_self(&mut self, node: &JustifySelf, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_align_items(&mut self, node: &AlignItems, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_justify_items(&mut self, node: &JustifyItems, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_legacy_justify(&mut self, node: &LegacyJustify, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_gap_value(&mut self, node: &GapValue<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_easing_function(&mut self, node: &EasingFunction, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_step_position(&mut self, node: &StepPosition, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_iteration_count(
        &mut self,
        node: &AnimationIterationCount,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_direction(
        &mut self,
        node: &AnimationDirection,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_play_state(
        &mut self,
        node: &AnimationPlayState,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_fill_mode(
        &mut self,
        node: &AnimationFillMode,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_composition(
        &mut self,
        node: &AnimationComposition,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_timeline(
        &mut self,
        node: &AnimationTimeline<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroll_axis(&mut self, node: &ScrollAxis, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scroller(&mut self, node: &Scroller, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_animation_attachment_range(
        &mut self,
        node: &AnimationAttachmentRange<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_timeline_range_name(
        &mut self,
        node: &TimelineRangeName,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_line_style(&mut self, node: &LineStyle, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_side_width(
        &mut self,
        node: &BorderSideWidth<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_length_or_number(&mut self, node: &LengthOrNumber<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image_repeat_keyword(
        &mut self,
        node: &BorderImageRepeatKeyword,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_border_image_side_width(
        &mut self,
        node: &BorderImageSideWidth<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_outline_style(&mut self, node: &OutlineStyle, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_display(&mut self, node: &Display, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_display_keyword(&mut self, node: &DisplayKeyword, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_display_inside(&mut self, node: &DisplayInside, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_display_outside(&mut self, node: &DisplayOutside, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_visibility(&mut self, node: &Visibility, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_size(&mut self, node: &Size<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_max_size(&mut self, node: &MaxSize<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_box_sizing(&mut self, node: &BoxSizing, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_overflow_keyword(&mut self, node: &OverflowKeyword, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_overflow(&mut self, node: &TextOverflow, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_position_property(&mut self, node: &PositionProperty, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_size_2_d<T>(&mut self, node: &Size2D<'a, T>, cx: &VisitContext<'_, 'ghost>)
    where
        T: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_rect<T>(&mut self, node: &Rect<'a, T>, cx: &VisitContext<'_, 'ghost>)
    where
        T: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_box_decoration_break(
        &mut self,
        node: &BoxDecorationBreak,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_z_index(&mut self, node: &ZIndex, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_container_type(&mut self, node: &ContainerType, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_container_name_list(
        &mut self,
        node: &ContainerNameList<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_filter_list(&mut self, node: &FilterList<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_filter(&mut self, node: &Filter<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_direction(&mut self, node: &FlexDirection, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_wrap(&mut self, node: &FlexWrap, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_box_orient(&mut self, node: &BoxOrient, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_box_direction(&mut self, node: &BoxDirection, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_box_align(&mut self, node: &BoxAlign, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_box_pack(&mut self, node: &BoxPack, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_box_lines(&mut self, node: &BoxLines, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_pack(&mut self, node: &FlexPack, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_item_align(&mut self, node: &FlexItemAlign, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_flex_line_pack(&mut self, node: &FlexLinePack, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_weight(&mut self, node: &FontWeight, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_absolute_font_weight(
        &mut self,
        node: &AbsoluteFontWeight,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_size(&mut self, node: &FontSize<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_absolute_font_size(&mut self, node: &AbsoluteFontSize, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_relative_font_size(&mut self, node: &RelativeFontSize, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_stretch(&mut self, node: &FontStretch, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_stretch_keyword(
        &mut self,
        node: &FontStretchKeyword,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_family(&mut self, node: &FontFamily<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_style(&mut self, node: &FontStyle, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_font_variant_caps(&mut self, node: &FontVariantCaps, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_line_height(&mut self, node: &LineHeight<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_vertical_align(&mut self, node: &VerticalAlign<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_vertical_align_keyword(
        &mut self,
        node: &VerticalAlignKeyword,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_track_sizing(&mut self, node: &TrackSizing<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_track_list_item(&mut self, node: &TrackListItem<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_track_size(&mut self, node: &TrackSize<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_track_breadth(&mut self, node: &TrackBreadth<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_repeat_count(&mut self, node: &RepeatCount, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_auto_flow_direction(
        &mut self,
        node: &AutoFlowDirection,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_template_areas(
        &mut self,
        node: &GridTemplateAreas<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_grid_line(&mut self, node: &GridLine<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_image(&mut self, node: &Image<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_gradient(&mut self, node: &Gradient<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_gradient(&mut self, node: &WebKitGradient<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_line_direction(&mut self, node: &LineDirection, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_horizontal_position_keyword(
        &mut self,
        node: &HorizontalPositionKeyword,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_vertical_position_keyword(
        &mut self,
        node: &VerticalPositionKeyword,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_gradient_item<D>(&mut self, node: &GradientItem<'a, D>, cx: &VisitContext<'_, 'ghost>)
    where
        D: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_dimension_percentage<D>(
        &mut self,
        node: &DimensionPercentage<'a, D>,
        cx: &VisitContext<'_, 'ghost>,
    ) where
        D: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_position_component<S>(
        &mut self,
        node: &PositionComponent<'a, S>,
        cx: &VisitContext<'_, 'ghost>,
    ) where
        S: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_ending_shape(&mut self, node: &EndingShape<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_ellipse(&mut self, node: &Ellipse<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_shape_extent(&mut self, node: &ShapeExtent, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_circle(&mut self, node: &Circle<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_gradient_point_component<S>(
        &mut self,
        node: &WebKitGradientPointComponent<S>,
        cx: &VisitContext<'_, 'ghost>,
    ) where
        S: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_number_or_percentage(
        &mut self,
        node: &NumberOrPercentage,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_background_size(&mut self, node: &BackgroundSize<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_length_percentage_or_auto(
        &mut self,
        node: &LengthPercentageOrAuto<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_background_repeat_keyword(
        &mut self,
        node: &BackgroundRepeatKeyword,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_background_attachment(
        &mut self,
        node: &BackgroundAttachment,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_background_clip(&mut self, node: &BackgroundClip, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_background_origin(&mut self, node: &BackgroundOrigin, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_list_style_type(&mut self, node: &ListStyleType<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_counter_style(&mut self, node: &CounterStyle<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_symbols_type(&mut self, node: &SymbolsType, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_predefined_counter_style(
        &mut self,
        node: &PredefinedCounterStyle,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_symbol(&mut self, node: &Symbol<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_list_style_position(
        &mut self,
        node: &ListStylePosition,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_marker_side(&mut self, node: &MarkerSide, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_mode(&mut self, node: &MaskMode, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_clip(&mut self, node: &MaskClip, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_composite(&mut self, node: &MaskComposite, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_type(&mut self, node: &MaskType, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_mask_border_mode(&mut self, node: &MaskBorderMode, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_mask_composite(
        &mut self,
        node: &WebKitMaskComposite,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_web_kit_mask_source_type(
        &mut self,
        node: &WebKitMaskSourceType,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_css_wide_keyword(&mut self, node: &CSSWideKeyword, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_css_wide_or<T>(&mut self, node: &CSSWideOr<T>, cx: &VisitContext<'_, 'ghost>)
    where
        T: Visit<'a, 'ghost>,
    {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_custom_property_name(
        &mut self,
        node: &CustomPropertyName<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_clip_path(&mut self, node: &ClipPath<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_geometry_box(&mut self, node: &GeometryBox, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_basic_shape(&mut self, node: &BasicShape<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_shape_radius(&mut self, node: &ShapeRadius<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_svg_paint(&mut self, node: &SVGPaint<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_svg_paint_fallback(
        &mut self,
        node: &SVGPaintFallback<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_fill_rule(&mut self, node: &FillRule, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_stroke_linecap(&mut self, node: &StrokeLinecap, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_stroke_linejoin(&mut self, node: &StrokeLinejoin, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_stroke_dasharray(
        &mut self,
        node: &StrokeDasharray<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_marker(&mut self, node: &Marker<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_color_interpolation(
        &mut self,
        node: &ColorInterpolation,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_color_rendering(&mut self, node: &ColorRendering, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_shape_rendering(&mut self, node: &ShapeRendering, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_rendering(&mut self, node: &TextRendering, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_image_rendering(&mut self, node: &ImageRendering, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_transform_case(
        &mut self,
        node: &TextTransformCase,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_white_space(&mut self, node: &WhiteSpace, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_word_break(&mut self, node: &WordBreak, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_line_break(&mut self, node: &LineBreak, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_hyphens(&mut self, node: &Hyphens, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_overflow_wrap(&mut self, node: &OverflowWrap, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_align(&mut self, node: &TextAlign, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_align_last(&mut self, node: &TextAlignLast, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_justify(&mut self, node: &TextJustify, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_spacing(&mut self, node: &Spacing<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration_line(
        &mut self,
        node: &TextDecorationLine<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_exclusive_text_decoration_line(
        &mut self,
        node: &ExclusiveTextDecorationLine,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_other_text_decoration_line(
        &mut self,
        node: &OtherTextDecorationLine,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration_style(
        &mut self,
        node: &TextDecorationStyle,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration_thickness(
        &mut self,
        node: &TextDecorationThickness<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_decoration_skip_ink(
        &mut self,
        node: &TextDecorationSkipInk,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_style(
        &mut self,
        node: &TextEmphasisStyle<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_fill_mode(
        &mut self,
        node: &TextEmphasisFillMode,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_shape(
        &mut self,
        node: &TextEmphasisShape,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_position_horizontal(
        &mut self,
        node: &TextEmphasisPositionHorizontal,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_emphasis_position_vertical(
        &mut self,
        node: &TextEmphasisPositionVertical,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_size_adjust(&mut self, node: &TextSizeAdjust, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_text_direction(&mut self, node: &TextDirection, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_unicode_bidi(&mut self, node: &UnicodeBidi, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_transform(&mut self, node: &Transform<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_transform_style(&mut self, node: &TransformStyle, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_transform_box(&mut self, node: &TransformBox, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_backface_visibility(
        &mut self,
        node: &BackfaceVisibility,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_perspective(&mut self, node: &Perspective<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_translate(&mut self, node: &Translate<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_scale(&mut self, node: &Scale, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_resize(&mut self, node: &Resize, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_cursor_keyword(&mut self, node: &CursorKeyword, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_color_or_auto(&mut self, node: &ColorOrAuto<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_caret_shape(&mut self, node: &CaretShape, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_user_select(&mut self, node: &UserSelect, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_appearance(&mut self, node: &Appearance<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_print_color_adjust(&mut self, node: &PrintColorAdjust, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_name(
        &mut self,
        node: &ViewTransitionName<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_none_or_custom_ident_list(
        &mut self,
        node: &NoneOrCustomIdentList<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_view_transition_group(
        &mut self,
        node: &ViewTransitionGroup<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_media_feature(&mut self, node: &MediaFeature<'a>, cx: &VisitContext<'_, 'ghost>) {
        self.visit_media_feature_children(node, cx);
    }
    ///Continues traversal of [`MediaFeature`] without redispatching its visitor callback.
    fn visit_media_feature_children(
        &mut self,
        node: &MediaFeature<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::MediaFeature);
        Visit::visit(node, visitor, cx);
        visitor.leave_node(AstType::MediaFeature);
    }
    #[inline]
    fn visit_container_size_feature(
        &mut self,
        node: &ContainerSizeFeature<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        self.visit_container_size_feature_children(node, cx);
    }
    ///Continues traversal of [`ContainerSizeFeature`] without redispatching its visitor callback.
    fn visit_container_size_feature_children(
        &mut self,
        node: &ContainerSizeFeature<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::ContainerSizeFeature);
        Visit::visit(node, visitor, cx);
        visitor.leave_node(AstType::ContainerSizeFeature);
    }
    #[inline]
    fn visit_scroll_state_feature(
        &mut self,
        node: &ScrollStateFeature<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        self.visit_scroll_state_feature_children(node, cx);
    }
    ///Continues traversal of [`ScrollStateFeature`] without redispatching its visitor callback.
    fn visit_scroll_state_feature_children(
        &mut self,
        node: &ScrollStateFeature<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::ScrollStateFeature);
        Visit::visit(node, visitor, cx);
        visitor.leave_node(AstType::ScrollStateFeature);
    }
    #[inline]
    fn visit_selector_list(&mut self, node: &SelectorList<'a>, cx: &VisitContext<'_, 'ghost>) {
        self.visit_selector_list_children(node, cx);
    }
    ///Continues traversal of [`SelectorList`] without redispatching its visitor callback.
    fn visit_selector_list_children(
        &mut self,
        node: &SelectorList<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::SelectorList);
        for value_0 in (node).iter() {
            Visit::visit(value_0, visitor, cx);
        }
        visitor.leave_node(AstType::SelectorList);
    }
    #[inline]
    fn visit_animation_range_start(
        &mut self,
        node: &AnimationRangeStart<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        self.visit_animation_range_start_children(node, cx);
    }
    ///Continues traversal of [`AnimationRangeStart`] without redispatching its visitor callback.
    fn visit_animation_range_start_children(
        &mut self,
        node: &AnimationRangeStart<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::AnimationRangeStart);
        Visit::visit(node, visitor, cx);
        visitor.leave_node(AstType::AnimationRangeStart);
    }
    #[inline]
    fn visit_animation_range_end(
        &mut self,
        node: &AnimationRangeEnd<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        self.visit_animation_range_end_children(node, cx);
    }
    ///Continues traversal of [`AnimationRangeEnd`] without redispatching its visitor callback.
    fn visit_animation_range_end_children(
        &mut self,
        node: &AnimationRangeEnd<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::AnimationRangeEnd);
        Visit::visit(node, visitor, cx);
        visitor.leave_node(AstType::AnimationRangeEnd);
    }
    #[inline]
    fn visit_length_percentage(
        &mut self,
        node: &LengthPercentage<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        self.visit_length_percentage_children(node, cx);
    }
    ///Continues traversal of [`LengthPercentage`] without redispatching its visitor callback.
    fn visit_length_percentage_children(
        &mut self,
        node: &LengthPercentage<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::LengthPercentage);
        Visit::visit(node, visitor, cx);
        visitor.leave_node(AstType::LengthPercentage);
    }
    #[inline]
    fn visit_angle_percentage(
        &mut self,
        node: &AnglePercentage<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        self.visit_angle_percentage_children(node, cx);
    }
    ///Continues traversal of [`AnglePercentage`] without redispatching its visitor callback.
    fn visit_angle_percentage_children(
        &mut self,
        node: &AnglePercentage<'a>,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        let visitor = self;
        visitor.enter_node(AstType::AnglePercentage);
        Visit::visit(node, visitor, cx);
        visitor.leave_node(AstType::AnglePercentage);
    }
    #[inline]
    fn visit_declaration(&mut self, node: &Declaration<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_property_id(&mut self, node: &PropertyId<'a>, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
    #[inline]
    fn visit_vendor_prefix(&mut self, node: &VendorPrefix, cx: &VisitContext<'_, 'ghost>) {
        Visit::visit_children(node, self, cx);
    }
}
/// Traversal implemented by CSS AST nodes.
pub trait Visit<'a, 'ghost> {
    /// Dispatches this node to its typed visitor callback.
    fn visit<VisitorT: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ghost>,
    );
    /// Continues traversal without redispatching this node's visitor callback.
    #[inline]
    fn visit_children<VisitorT: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        _visitor: &mut VisitorT,
        _cx: &VisitContext<'_, 'ghost>,
    ) {
    }
}
macro_rules! impl_leaf_visit {
    ($($ty:ty),+ $(,)?) => {
        $(impl < 'a, 'ghost > Visit < 'a, 'ghost > for $ty { fn visit < VisitorT : ?
        Sized + Visitor < 'a, 'ghost >> (& self, _visitor : & mut VisitorT, _cx : &
        VisitContext < '_, 'ghost >,) {} })+
    };
}
impl_leaf_visit!(bool, char, f32, i32, u8, u16, u32, usize);
impl<'a, 'ghost, T: ?Sized + Visit<'a, 'ghost>> Visit<'a, 'ghost>
    for rocketcss_allocator::boxed::Box<'a, T>
{
    fn visit<VisitorT: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        Visit::visit(self.as_ref(), visitor, cx);
    }
}
impl<'a, 'ghost, T: Visit<'a, 'ghost> + Unpin> Visit<'a, 'ghost>
    for rocketcss_allocator::vec::Vec<'a, T>
{
    fn visit<VisitorT: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        for value in self {
            Visit::visit(value, visitor, cx);
        }
    }
}
impl<'a, 'ghost, T: Visit<'a, 'ghost>> Visit<'a, 'ghost> for Option<T> {
    fn visit<VisitorT: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        if let Some(value) = self {
            Visit::visit(value, visitor, cx);
        }
    }
}
impl<'a, 'ghost> Visit<'a, 'ghost> for &'a str {
    fn visit<VisitorT: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        visitor.visit_str(self, cx);
    }
}
impl<'a, 'ghost> Visit<'a, 'ghost> for VendorPrefix {
    fn visit<VisitorT: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        cx: &VisitContext<'_, 'ghost>,
    ) {
        visitor.visit_vendor_prefix(self, cx);
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        visitor: &mut VisitorT,
        _cx: &VisitContext<'_, 'ghost>,
    ) {
        visitor.enter_node(AstType::VendorPrefix);
        visitor.leave_node(AstType::VendorPrefix);
    }
}
