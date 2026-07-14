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
pub trait VisitorMut<'a> {
    /// Returns the callbacks implemented by this visitor.
    ///
    /// Use `#[rocketcss_visitor::visitor]` on the visitor implementation to
    /// generate a precise static bitset. The default preserves compatibility
    /// by treating every callback as implemented.
    #[inline]
    fn visitor_methods(&self) -> &'static VisitorMethods {
        &VisitorMethods::ALL
    }
    #[inline]
    fn enter_node(&mut self, _kind: AstType) {}
    #[inline]
    fn leave_node(&mut self, _kind: AstType) {}
    #[inline]
    fn visit_str(&mut self, _value: &mut &'a str) {}
    #[inline]
    fn visit_css_color(&mut self, node: &mut CssColor<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_rgba(&mut self, node: &mut RGBA) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_lab_color(&mut self, node: &mut LABColor) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_predefined_color(&mut self, node: &mut PredefinedColor) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_float_color(&mut self, node: &mut FloatColor) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_light_dark(&mut self, node: &mut LightDark<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_system_color(&mut self, node: &mut SystemColor) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_unresolved_color(&mut self, node: &mut UnresolvedColor<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_css_rule(&mut self, node: &mut CssRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_length(&mut self, node: &mut Length<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_length_unit(&mut self, node: &mut LengthUnit) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_calc<V>(&mut self, node: &mut Calc<'a, V>)
    where
        V: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_math_function<V>(&mut self, node: &mut MathFunction<'a, V>)
    where
        V: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_rounding_strategy(&mut self, node: &mut RoundingStrategy) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_resolution(&mut self, node: &mut Resolution) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_ratio(&mut self, node: &mut Ratio) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_angle(&mut self, node: &mut Angle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_time(&mut self, node: &mut Time) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_condition(&mut self, node: &mut MediaCondition<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_query_feature<FeatureId>(&mut self, node: &mut QueryFeature<'a, FeatureId>)
    where
        FeatureId: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_feature_name<FeatureId>(&mut self, node: &mut MediaFeatureName<'a, FeatureId>)
    where
        FeatureId: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_feature_id(&mut self, node: &mut MediaFeatureId) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_feature_value(&mut self, node: &mut MediaFeatureValue<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_feature_comparison(&mut self, node: &mut MediaFeatureComparison) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_operator(&mut self, node: &mut Operator) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_type(&mut self, node: &mut MediaType<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_qualifier(&mut self, node: &mut Qualifier) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_supports_condition(&mut self, node: &mut SupportsCondition<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_blend_mode(&mut self, node: &mut BlendMode) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_transition(&mut self, node: &mut Transition<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_timeline(&mut self, node: &mut ScrollTimeline) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_view_timeline(&mut self, node: &mut ViewTimeline<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_range(&mut self, node: &mut AnimationRange<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation(&mut self, node: &mut Animation<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_supports_rule(&mut self, node: &mut SupportsRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_counter_style_rule(&mut self, node: &mut CounterStyleRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_namespace_rule(&mut self, node: &mut NamespaceRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_moz_document_rule(&mut self, node: &mut MozDocumentRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_nesting_rule(&mut self, node: &mut NestingRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_nested_declarations_rule(&mut self, node: &mut NestedDeclarationsRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_viewport_rule(&mut self, node: &mut ViewportRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_custom_media_rule(&mut self, node: &mut CustomMediaRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_layer_statement_rule(&mut self, node: &mut LayerStatementRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_layer_block_rule(&mut self, node: &mut LayerBlockRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scope_rule(&mut self, node: &mut ScopeRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_starting_style_rule(&mut self, node: &mut StartingStyleRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_position_try_rule(&mut self, node: &mut PositionTryRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_unknown_at_rule(&mut self, node: &mut UnknownAtRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_position(&mut self, node: &mut Position<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_web_kit_gradient_point(&mut self, node: &mut WebKitGradientPoint<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_web_kit_color_stop(&mut self, node: &mut WebKitColorStop<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_image_set(&mut self, node: &mut ImageSet<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_image_set_option(&mut self, node: &mut ImageSetOption<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_background_position(&mut self, node: &mut BackgroundPosition<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_background_repeat(&mut self, node: &mut BackgroundRepeat) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_background(&mut self, node: &mut Background<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_box_shadow(&mut self, node: &mut BoxShadow<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_radius(&mut self, node: &mut BorderRadius<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_image_repeat(&mut self, node: &mut BorderImageRepeat) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_image_slice(&mut self, node: &mut BorderImageSlice<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_image(&mut self, node: &mut BorderImage<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_color(&mut self, node: &mut BorderColor<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_style(&mut self, node: &mut BorderStyle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_width(&mut self, node: &mut BorderWidth<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_block_color(&mut self, node: &mut BorderBlockColor<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_block_style(&mut self, node: &mut BorderBlockStyle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_block_width(&mut self, node: &mut BorderBlockWidth<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_inline_color(&mut self, node: &mut BorderInlineColor<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_inline_style(&mut self, node: &mut BorderInlineStyle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_inline_width(&mut self, node: &mut BorderInlineWidth<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_generic_border<S>(&mut self, node: &mut GenericBorder<'a, S>)
    where
        S: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_container_condition(&mut self, node: &mut ContainerCondition<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_container_size_feature_id(&mut self, node: &mut ContainerSizeFeatureId) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_style_query(&mut self, node: &mut StyleQuery<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_state_query(&mut self, node: &mut ScrollStateQuery<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_state_feature_id(&mut self, node: &mut ScrollStateFeatureId) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_container(&mut self, node: &mut Container<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_container_rule(&mut self, node: &mut ContainerRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_face_property(&mut self, node: &mut FontFaceProperty<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_source(&mut self, node: &mut Source<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_format(&mut self, node: &mut FontFormat<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_technology(&mut self, node: &mut FontTechnology) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_face_style(&mut self, node: &mut FontFaceStyle<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_palette_values_property(&mut self, node: &mut FontPaletteValuesProperty<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_base_palette(&mut self, node: &mut BasePalette) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_feature_subrule_type(&mut self, node: &mut FontFeatureSubruleType) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font(&mut self, node: &mut Font<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_face_rule(&mut self, node: &mut FontFaceRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_url_source(&mut self, node: &mut UrlSource<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_unicode_range(&mut self, node: &mut UnicodeRange) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_palette_values_rule(&mut self, node: &mut FontPaletteValuesRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_override_colors(&mut self, node: &mut OverrideColors<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_feature_values_rule(&mut self, node: &mut FontFeatureValuesRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_feature_subrule(&mut self, node: &mut FontFeatureSubrule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_feature_declaration(&mut self, node: &mut FontFeatureDeclaration<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_family_name(&mut self, node: &mut FamilyName<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_keyframe_selector(&mut self, node: &mut KeyframeSelector<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_keyframes_name(&mut self, node: &mut KeyframesName<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_keyframes_rule(&mut self, node: &mut KeyframesRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_keyframe(&mut self, node: &mut Keyframe<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_timeline_range_percentage(&mut self, node: &mut TimelineRangePercentage) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_aspect_ratio(&mut self, node: &mut AspectRatio<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_overflow(&mut self, node: &mut Overflow) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_inset_block(&mut self, node: &mut InsetBlock<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_inset_inline(&mut self, node: &mut InsetInline<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_inset(&mut self, node: &mut Inset<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_flex_flow(&mut self, node: &mut FlexFlow) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_flex(&mut self, node: &mut Flex<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_place_content(&mut self, node: &mut PlaceContent<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_place_self(&mut self, node: &mut PlaceSelf<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_place_items(&mut self, node: &mut PlaceItems<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_gap(&mut self, node: &mut Gap<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_track_repeat(&mut self, node: &mut TrackRepeat<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_grid_auto_flow(&mut self, node: &mut GridAutoFlow) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_grid_template(&mut self, node: &mut GridTemplate<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_grid(&mut self, node: &mut Grid<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_grid_row(&mut self, node: &mut GridRow<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_grid_column(&mut self, node: &mut GridColumn<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_grid_area(&mut self, node: &mut GridArea<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_margin_block(&mut self, node: &mut MarginBlock<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_margin_inline(&mut self, node: &mut MarginInline<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_margin(&mut self, node: &mut Margin<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_padding_block(&mut self, node: &mut PaddingBlock<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_padding_inline(&mut self, node: &mut PaddingInline<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_padding(&mut self, node: &mut Padding<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_margin_block(&mut self, node: &mut ScrollMarginBlock<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_margin_inline(&mut self, node: &mut ScrollMarginInline<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_margin(&mut self, node: &mut ScrollMargin<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_padding_block(&mut self, node: &mut ScrollPaddingBlock<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_padding_inline(&mut self, node: &mut ScrollPaddingInline<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_padding(&mut self, node: &mut ScrollPadding<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_page_margin_box(&mut self, node: &mut PageMarginBox) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_page_pseudo_class(&mut self, node: &mut PagePseudoClass) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_page_rule(&mut self, node: &mut PageRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_page_margin_rule(&mut self, node: &mut PageMarginRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_page_selector(&mut self, node: &mut PageSelector<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_parsed_component(&mut self, node: &mut ParsedComponent<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_multiplier(&mut self, node: &mut Multiplier) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_syntax_string(&mut self, node: &mut SyntaxString<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_syntax_component_kind(&mut self, node: &mut SyntaxComponentKind<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_unparsed_property(&mut self, node: &mut UnparsedProperty<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_custom_property(&mut self, node: &mut CustomProperty<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_property_rule(&mut self, node: &mut PropertyRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_syntax_component(&mut self, node: &mut SyntaxComponent<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_inset_rect(&mut self, node: &mut InsetRect<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_circle_shape(&mut self, node: &mut CircleShape<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_ellipse_shape(&mut self, node: &mut EllipseShape<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_polygon(&mut self, node: &mut Polygon<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_point(&mut self, node: &mut Point<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_mask(&mut self, node: &mut Mask<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_mask_border(&mut self, node: &mut MaskBorder<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_drop_shadow(&mut self, node: &mut DropShadow<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_default_at_rule(&mut self, node: &mut DefaultAtRule) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_style_sheet(&mut self, node: &mut StyleSheet<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_rule(&mut self, node: &mut MediaRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_list(&mut self, node: &mut MediaList<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_query(&mut self, node: &mut MediaQuery<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_length_value(&mut self, node: &mut LengthValue) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_environment_variable(&mut self, node: &mut EnvironmentVariable<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_url(&mut self, node: &mut Url<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_variable(&mut self, node: &mut Variable<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_dashed_ident_reference(&mut self, node: &mut DashedIdentReference<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_function(&mut self, node: &mut Function<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_function_replacement(&mut self, node: &mut FunctionReplacement) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_import_rule(&mut self, node: &mut ImportRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_style_rule(&mut self, node: &mut StyleRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_declaration_block(&mut self, mut node: Pin<&mut DeclarationBlock<'a>>) {
        VisitMut::visit_mut_children(&mut node, self);
    }
    #[inline]
    fn visit_text_transform(&mut self, node: &mut TextTransform) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_indent(&mut self, node: &mut TextIndent<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_decoration(&mut self, node: &mut TextDecoration<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis(&mut self, node: &mut TextEmphasis<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_position(&mut self, node: &mut TextEmphasisPosition) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_shadow(&mut self, node: &mut TextShadow<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_matrix_for_float(&mut self, node: &mut MatrixForFloat) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_matrix_3_d_for_float(&mut self, node: &mut Matrix3DForFloat) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_rotate(&mut self, node: &mut Rotate<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_cursor(&mut self, node: &mut Cursor<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_cursor_image(&mut self, node: &mut CursorImage<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_caret(&mut self, node: &mut Caret<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_list_style(&mut self, node: &mut ListStyle<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_composes(&mut self, node: &mut Composes<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_color_scheme(&mut self, node: &mut ColorScheme) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_view_transition_property(&mut self, node: &mut ViewTransitionProperty<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_navigation(&mut self, node: &mut Navigation) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_view_transition_part_selector(&mut self, node: &mut ViewTransitionPartSelector<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_view_transition_rule(&mut self, node: &mut ViewTransitionRule<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_selector_component(&mut self, node: &mut SelectorComponent<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_combinator(&mut self, node: &mut Combinator) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_attr_selector(&mut self, node: &mut AttrSelector<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_namespace_constraint(&mut self, node: &mut NamespaceConstraint<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_attr_operation(&mut self, node: &mut AttrOperation<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_parsed_case_sensitivity(&mut self, node: &mut ParsedCaseSensitivity) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_attr_selector_operator(&mut self, node: &mut AttrSelectorOperator) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_nth_type(&mut self, node: &mut NthType) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_nth_selector_data(&mut self, node: &mut NthSelectorData) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_direction(&mut self, node: &mut Direction) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_pseudo_class(&mut self, node: &mut PseudoClass<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_class(&mut self, node: &mut WebKitScrollbarPseudoClass) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_pseudo_element(&mut self, node: &mut PseudoElement<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_element(&mut self, node: &mut WebKitScrollbarPseudoElement) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_view_transition_part_name(&mut self, node: &mut ViewTransitionPartName<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_span(&mut self, node: &mut Span) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_token_or_value(&mut self, node: &mut TokenOrValue<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_unit(&mut self, node: &mut Unit) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_token(&mut self, node: &mut Token<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_specifier(&mut self, node: &mut Specifier<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_name(&mut self, node: &mut AnimationName<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_environment_variable_name(&mut self, node: &mut EnvironmentVariableName<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_ua_environment_variable(&mut self, node: &mut UAEnvironmentVariable) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_align_content(&mut self, node: &mut AlignContent) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_baseline_position(&mut self, node: &mut BaselinePosition) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_content_distribution(&mut self, node: &mut ContentDistribution) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_overflow_position(&mut self, node: &mut OverflowPosition) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_content_position(&mut self, node: &mut ContentPosition) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_justify_content(&mut self, node: &mut JustifyContent) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_align_self(&mut self, node: &mut AlignSelf) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_self_position(&mut self, node: &mut SelfPosition) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_justify_self(&mut self, node: &mut JustifySelf) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_align_items(&mut self, node: &mut AlignItems) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_justify_items(&mut self, node: &mut JustifyItems) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_legacy_justify(&mut self, node: &mut LegacyJustify) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_gap_value(&mut self, node: &mut GapValue<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_easing_function(&mut self, node: &mut EasingFunction) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_step_position(&mut self, node: &mut StepPosition) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_iteration_count(&mut self, node: &mut AnimationIterationCount) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_direction(&mut self, node: &mut AnimationDirection) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_play_state(&mut self, node: &mut AnimationPlayState) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_fill_mode(&mut self, node: &mut AnimationFillMode) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_composition(&mut self, node: &mut AnimationComposition) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_timeline(&mut self, node: &mut AnimationTimeline<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroll_axis(&mut self, node: &mut ScrollAxis) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scroller(&mut self, node: &mut Scroller) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_animation_attachment_range(&mut self, node: &mut AnimationAttachmentRange<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_timeline_range_name(&mut self, node: &mut TimelineRangeName) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_line_style(&mut self, node: &mut LineStyle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_side_width(&mut self, node: &mut BorderSideWidth<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_length_or_number(&mut self, node: &mut LengthOrNumber<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_image_repeat_keyword(&mut self, node: &mut BorderImageRepeatKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_border_image_side_width(&mut self, node: &mut BorderImageSideWidth<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_outline_style(&mut self, node: &mut OutlineStyle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_display(&mut self, node: &mut Display<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_display_keyword(&mut self, node: &mut DisplayKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_display_inside(&mut self, node: &mut DisplayInside) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_display_outside(&mut self, node: &mut DisplayOutside) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_visibility(&mut self, node: &mut Visibility) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_size(&mut self, node: &mut Size<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_max_size(&mut self, node: &mut MaxSize<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_box_sizing(&mut self, node: &mut BoxSizing) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_overflow_keyword(&mut self, node: &mut OverflowKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_overflow(&mut self, node: &mut TextOverflow) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_position_property(&mut self, node: &mut PositionProperty) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_size_2_d<T>(&mut self, node: &mut Size2D<'a, T>)
    where
        T: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_rect<T>(&mut self, node: &mut Rect<'a, T>)
    where
        T: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_box_decoration_break(&mut self, node: &mut BoxDecorationBreak) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_z_index(&mut self, node: &mut ZIndex) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_container_type(&mut self, node: &mut ContainerType) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_container_name_list(&mut self, node: &mut ContainerNameList<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_filter_list(&mut self, node: &mut FilterList<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_filter(&mut self, node: &mut Filter<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_flex_direction(&mut self, node: &mut FlexDirection) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_flex_wrap(&mut self, node: &mut FlexWrap) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_box_orient(&mut self, node: &mut BoxOrient) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_box_direction(&mut self, node: &mut BoxDirection) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_box_align(&mut self, node: &mut BoxAlign) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_box_pack(&mut self, node: &mut BoxPack) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_box_lines(&mut self, node: &mut BoxLines) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_flex_pack(&mut self, node: &mut FlexPack) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_flex_item_align(&mut self, node: &mut FlexItemAlign) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_flex_line_pack(&mut self, node: &mut FlexLinePack) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_weight(&mut self, node: &mut FontWeight<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_absolute_font_weight(&mut self, node: &mut AbsoluteFontWeight) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_size(&mut self, node: &mut FontSize<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_absolute_font_size(&mut self, node: &mut AbsoluteFontSize) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_relative_font_size(&mut self, node: &mut RelativeFontSize) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_stretch(&mut self, node: &mut FontStretch) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_stretch_keyword(&mut self, node: &mut FontStretchKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_family(&mut self, node: &mut FontFamily<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_generic_font_family(&mut self, node: &mut GenericFontFamily) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_style(&mut self, node: &mut FontStyle<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_font_variant_caps(&mut self, node: &mut FontVariantCaps) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_line_height(&mut self, node: &mut LineHeight<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_vertical_align(&mut self, node: &mut VerticalAlign<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_vertical_align_keyword(&mut self, node: &mut VerticalAlignKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_track_sizing(&mut self, node: &mut TrackSizing<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_track_list_item(&mut self, node: &mut TrackListItem<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_track_size(&mut self, node: &mut TrackSize<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_track_breadth(&mut self, node: &mut TrackBreadth<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_repeat_count(&mut self, node: &mut RepeatCount) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_auto_flow_direction(&mut self, node: &mut AutoFlowDirection) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_grid_template_areas(&mut self, node: &mut GridTemplateAreas<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_grid_line(&mut self, node: &mut GridLine<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_image(&mut self, node: &mut Image<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_gradient(&mut self, node: &mut Gradient<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_web_kit_gradient(&mut self, node: &mut WebKitGradient<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_line_direction(&mut self, node: &mut LineDirection<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_horizontal_position_keyword(&mut self, node: &mut HorizontalPositionKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_vertical_position_keyword(&mut self, node: &mut VerticalPositionKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_gradient_item<D>(&mut self, node: &mut GradientItem<'a, D>)
    where
        D: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_dimension_percentage<D>(&mut self, node: &mut DimensionPercentage<'a, D>)
    where
        D: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_position_component<S>(&mut self, node: &mut PositionComponent<'a, S>)
    where
        S: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_ending_shape(&mut self, node: &mut EndingShape<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_ellipse(&mut self, node: &mut Ellipse<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_shape_extent(&mut self, node: &mut ShapeExtent) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_circle(&mut self, node: &mut Circle<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_web_kit_gradient_point_component<S>(
        &mut self,
        node: &mut WebKitGradientPointComponent<'a, S>,
    ) where
        S: VisitMut<'a>,
    {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_number_or_percentage(&mut self, node: &mut NumberOrPercentage) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_background_size(&mut self, node: &mut BackgroundSize<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_length_percentage_or_auto(&mut self, node: &mut LengthPercentageOrAuto<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_background_repeat_keyword(&mut self, node: &mut BackgroundRepeatKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_background_attachment(&mut self, node: &mut BackgroundAttachment) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_background_clip(&mut self, node: &mut BackgroundClip) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_background_origin(&mut self, node: &mut BackgroundOrigin) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_list_style_type(&mut self, node: &mut ListStyleType<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_counter_style(&mut self, node: &mut CounterStyle<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_symbols_type(&mut self, node: &mut SymbolsType) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_predefined_counter_style(&mut self, node: &mut PredefinedCounterStyle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_symbol(&mut self, node: &mut Symbol<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_list_style_position(&mut self, node: &mut ListStylePosition) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_marker_side(&mut self, node: &mut MarkerSide) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_mask_mode(&mut self, node: &mut MaskMode) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_mask_clip(&mut self, node: &mut MaskClip) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_mask_composite(&mut self, node: &mut MaskComposite) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_mask_type(&mut self, node: &mut MaskType) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_mask_border_mode(&mut self, node: &mut MaskBorderMode) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_web_kit_mask_composite(&mut self, node: &mut WebKitMaskComposite) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_web_kit_mask_source_type(&mut self, node: &mut WebKitMaskSourceType) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_css_wide_keyword(&mut self, node: &mut CSSWideKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_custom_property_name(&mut self, node: &mut CustomPropertyName<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_clip_path(&mut self, node: &mut ClipPath<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_geometry_box(&mut self, node: &mut GeometryBox) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_basic_shape(&mut self, node: &mut BasicShape<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_shape_radius(&mut self, node: &mut ShapeRadius<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_svg_paint(&mut self, node: &mut SVGPaint<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_svg_paint_fallback(&mut self, node: &mut SVGPaintFallback<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_fill_rule(&mut self, node: &mut FillRule) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_stroke_linecap(&mut self, node: &mut StrokeLinecap) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_stroke_linejoin(&mut self, node: &mut StrokeLinejoin) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_stroke_dasharray(&mut self, node: &mut StrokeDasharray<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_marker(&mut self, node: &mut Marker<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_color_interpolation(&mut self, node: &mut ColorInterpolation) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_color_rendering(&mut self, node: &mut ColorRendering) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_shape_rendering(&mut self, node: &mut ShapeRendering) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_rendering(&mut self, node: &mut TextRendering) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_image_rendering(&mut self, node: &mut ImageRendering) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_transform_case(&mut self, node: &mut TextTransformCase) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_white_space(&mut self, node: &mut WhiteSpace) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_word_break(&mut self, node: &mut WordBreak) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_line_break(&mut self, node: &mut LineBreak) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_hyphens(&mut self, node: &mut Hyphens) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_overflow_wrap(&mut self, node: &mut OverflowWrap) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_align(&mut self, node: &mut TextAlign) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_align_last(&mut self, node: &mut TextAlignLast) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_justify(&mut self, node: &mut TextJustify) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_spacing(&mut self, node: &mut Spacing<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_decoration_line(&mut self, node: &mut TextDecorationLine<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_exclusive_text_decoration_line(&mut self, node: &mut ExclusiveTextDecorationLine) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_other_text_decoration_line(&mut self, node: &mut OtherTextDecorationLine) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_decoration_style(&mut self, node: &mut TextDecorationStyle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_decoration_thickness(&mut self, node: &mut TextDecorationThickness<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_decoration_skip_ink(&mut self, node: &mut TextDecorationSkipInk) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_style(&mut self, node: &mut TextEmphasisStyle<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_fill_mode(&mut self, node: &mut TextEmphasisFillMode) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_shape(&mut self, node: &mut TextEmphasisShape) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_position_horizontal(
        &mut self,
        node: &mut TextEmphasisPositionHorizontal,
    ) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_position_vertical(&mut self, node: &mut TextEmphasisPositionVertical) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_size_adjust(&mut self, node: &mut TextSizeAdjust) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_text_direction(&mut self, node: &mut TextDirection) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_unicode_bidi(&mut self, node: &mut UnicodeBidi) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_transform(&mut self, node: &mut Transform<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_transform_style(&mut self, node: &mut TransformStyle) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_transform_box(&mut self, node: &mut TransformBox) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_backface_visibility(&mut self, node: &mut BackfaceVisibility) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_perspective(&mut self, node: &mut Perspective<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_translate(&mut self, node: &mut Translate<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_scale(&mut self, node: &mut Scale<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_resize(&mut self, node: &mut Resize) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_cursor_keyword(&mut self, node: &mut CursorKeyword) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_color_or_auto(&mut self, node: &mut ColorOrAuto<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_caret_shape(&mut self, node: &mut CaretShape) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_user_select(&mut self, node: &mut UserSelect) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_appearance(&mut self, node: &mut Appearance<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_print_color_adjust(&mut self, node: &mut PrintColorAdjust) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_view_transition_name(&mut self, node: &mut ViewTransitionName<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_none_or_custom_ident_list(&mut self, node: &mut NoneOrCustomIdentList<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_view_transition_group(&mut self, node: &mut ViewTransitionGroup<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_media_feature(&mut self, node: &mut MediaFeature<'a>) {
        self.visit_media_feature_children(node);
    }
    ///Continues traversal of [`MediaFeature`] without redispatching its visitor callback.
    fn visit_media_feature_children(&mut self, node: &mut MediaFeature<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::MediaFeature);
        }
        VisitMut::visit_mut(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::MediaFeature);
        }
    }
    #[inline]
    fn visit_container_size_feature(&mut self, node: &mut ContainerSizeFeature<'a>) {
        self.visit_container_size_feature_children(node);
    }
    ///Continues traversal of [`ContainerSizeFeature`] without redispatching its visitor callback.
    fn visit_container_size_feature_children(&mut self, node: &mut ContainerSizeFeature<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::ContainerSizeFeature);
        }
        VisitMut::visit_mut(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::ContainerSizeFeature);
        }
    }
    #[inline]
    fn visit_scroll_state_feature(&mut self, node: &mut ScrollStateFeature<'a>) {
        self.visit_scroll_state_feature_children(node);
    }
    ///Continues traversal of [`ScrollStateFeature`] without redispatching its visitor callback.
    fn visit_scroll_state_feature_children(&mut self, node: &mut ScrollStateFeature<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::ScrollStateFeature);
        }
        VisitMut::visit_mut(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::ScrollStateFeature);
        }
    }
    #[inline]
    fn visit_selector_list(&mut self, node: &mut SelectorList<'a>) {
        self.visit_selector_list_children(node);
    }
    ///Continues traversal of [`SelectorList`] without redispatching its visitor callback.
    fn visit_selector_list_children(&mut self, node: &mut SelectorList<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::SelectorList);
        }
        for value_0 in (node).iter_mut() {
            if visitor
                .visitor_methods()
                .contains(VisitorMethods::VISIT_SELECTOR)
            {
                visitor.visit_selector(value_0);
            } else {
                visitor.visit_selector_children(value_0);
            }
        }
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::SelectorList);
        }
    }
    #[inline]
    fn visit_selector(&mut self, node: &mut Selector<'a>) {
        self.visit_selector_children(node);
    }
    ///Continues traversal of [`Selector`] without redispatching its visitor callback.
    fn visit_selector_children(&mut self, node: &mut Selector<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::Selector);
        }
        for value_0 in (node).iter_mut() {
            VisitMut::visit_mut(value_0, visitor);
        }
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::Selector);
        }
    }
    #[inline]
    fn visit_animation_range_start(&mut self, node: &mut AnimationRangeStart<'a>) {
        self.visit_animation_range_start_children(node);
    }
    ///Continues traversal of [`AnimationRangeStart`] without redispatching its visitor callback.
    fn visit_animation_range_start_children(&mut self, node: &mut AnimationRangeStart<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::AnimationRangeStart);
        }
        VisitMut::visit_mut(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::AnimationRangeStart);
        }
    }
    #[inline]
    fn visit_animation_range_end(&mut self, node: &mut AnimationRangeEnd<'a>) {
        self.visit_animation_range_end_children(node);
    }
    ///Continues traversal of [`AnimationRangeEnd`] without redispatching its visitor callback.
    fn visit_animation_range_end_children(&mut self, node: &mut AnimationRangeEnd<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::AnimationRangeEnd);
        }
        VisitMut::visit_mut(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::AnimationRangeEnd);
        }
    }
    #[inline]
    fn visit_length_percentage(&mut self, node: &mut LengthPercentage<'a>) {
        self.visit_length_percentage_children(node);
    }
    ///Continues traversal of [`LengthPercentage`] without redispatching its visitor callback.
    fn visit_length_percentage_children(&mut self, node: &mut LengthPercentage<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::LengthPercentage);
        }
        VisitMut::visit_mut(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::LengthPercentage);
        }
    }
    #[inline]
    fn visit_angle_percentage(&mut self, node: &mut AnglePercentage<'a>) {
        self.visit_angle_percentage_children(node);
    }
    ///Continues traversal of [`AnglePercentage`] without redispatching its visitor callback.
    fn visit_angle_percentage_children(&mut self, node: &mut AnglePercentage<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::AnglePercentage);
        }
        VisitMut::visit_mut(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::AnglePercentage);
        }
    }
    #[inline]
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_property_id(&mut self, node: &mut PropertyId<'a>) {
        VisitMut::visit_mut_children(node, self);
    }
    #[inline]
    fn visit_vendor_prefix(&mut self, node: &mut VendorPrefix) {
        VisitMut::visit_mut_children(node, self);
    }
}
/// Traversal implemented by CSS AST nodes.
pub trait VisitMut<'a> {
    /// Dispatches this node to its typed visitor callback.
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT);
    /// Continues traversal without redispatching this node's visitor callback.
    #[inline]
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, _visitor: &mut VisitorT) {}
}
macro_rules! impl_leaf_visit_mut {
    ($($ty:ty),+ $(,)?) => {
        $(impl < 'a > VisitMut < 'a > for $ty { fn visit_mut < VisitorT : ? Sized +
        VisitorMut < 'a >> (& mut self, _visitor : & mut VisitorT,) {} })+
    };
}
impl_leaf_visit_mut!(bool, char, f32, i32, u8, u16, u32, usize);
impl<'a, T: ?Sized + VisitMut<'a>> VisitMut<'a> for rocketcss_allocator::boxed::Box<'a, T> {
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        VisitMut::visit_mut(self.as_mut(), visitor);
    }
}
impl<'a, T: VisitMut<'a> + Unpin> VisitMut<'a> for rocketcss_allocator::vec::Vec<'a, T> {
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        for value in self {
            VisitMut::visit_mut(value, visitor);
        }
    }
}
impl<'a, T: VisitMut<'a>> VisitMut<'a> for Option<T> {
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        if let Some(value) = self {
            VisitMut::visit_mut(value, visitor);
        }
    }
}
impl<'a> VisitMut<'a> for &'a str {
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::VISIT_STR)
        {
            visitor.visit_str(self);
        }
    }
}
impl<'a> VisitMut<'a> for VendorPrefix {
    fn visit_mut<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::VISIT_VENDOR_PREFIX)
        {
            visitor.visit_vendor_prefix(self);
        } else {
            VisitMut::visit_mut_children(self, visitor);
        }
    }
    fn visit_mut_children<VisitorT: ?Sized + VisitorMut<'a>>(&mut self, visitor: &mut VisitorT) {
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::VendorPrefix);
        }
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::VendorPrefix);
        }
    }
}
