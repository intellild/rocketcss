//! Generated typed visitor API. Regenerate with `cargo run -p rocketcss_ast_tools`.
#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use crate::*;
/// Typed callbacks invoked while traversing CSS AST nodes.
pub trait Visitor<'a> {
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
    fn visit_str(&mut self, _value: &&'a str) {}
    #[inline]
    fn visit_css_color(&mut self, node: &CssColor<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_rgba(&mut self, node: &RGBA) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_lab_color(&mut self, node: &LABColor) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_predefined_color(&mut self, node: &PredefinedColor) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_float_color(&mut self, node: &FloatColor) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_light_dark(&mut self, node: &LightDark<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_system_color(&mut self, node: &SystemColor) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_unresolved_color(&mut self, node: &UnresolvedColor<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_css_rule(&mut self, node: &CssRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_length(&mut self, node: &Length<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_length_unit(&mut self, node: &LengthUnit) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_calc<V>(&mut self, node: &Calc<'a, V>)
    where
        V: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_math_function<V>(&mut self, node: &MathFunction<'a, V>)
    where
        V: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_rounding_strategy(&mut self, node: &RoundingStrategy) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_resolution(&mut self, node: &Resolution) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_ratio(&mut self, node: &Ratio) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_angle(&mut self, node: &Angle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_time(&mut self, node: &Time) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_condition(&mut self, node: &MediaCondition<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_query_feature<FeatureId>(&mut self, node: &QueryFeature<'a, FeatureId>)
    where
        FeatureId: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_feature_name<FeatureId>(&mut self, node: &MediaFeatureName<'a, FeatureId>)
    where
        FeatureId: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_feature_id(&mut self, node: &MediaFeatureId) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_feature_value(&mut self, node: &MediaFeatureValue<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_feature_comparison(&mut self, node: &MediaFeatureComparison) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_operator(&mut self, node: &Operator) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_type(&mut self, node: &MediaType<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_qualifier(&mut self, node: &Qualifier) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_supports_condition(&mut self, node: &SupportsCondition<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_blend_mode(&mut self, node: &BlendMode) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_transition(&mut self, node: &Transition<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_timeline(&mut self, node: &ScrollTimeline) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_view_timeline(&mut self, node: &ViewTimeline<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_range(&mut self, node: &AnimationRange<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation(&mut self, node: &Animation<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_supports_rule(&mut self, node: &SupportsRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_counter_style_rule(&mut self, node: &CounterStyleRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_namespace_rule(&mut self, node: &NamespaceRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_moz_document_rule(&mut self, node: &MozDocumentRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_nesting_rule(&mut self, node: &NestingRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_nested_declarations_rule(&mut self, node: &NestedDeclarationsRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_viewport_rule(&mut self, node: &ViewportRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_custom_media_rule(&mut self, node: &CustomMediaRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_layer_statement_rule(&mut self, node: &LayerStatementRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_layer_block_rule(&mut self, node: &LayerBlockRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scope_rule(&mut self, node: &ScopeRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_starting_style_rule(&mut self, node: &StartingStyleRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_position_try_rule(&mut self, node: &PositionTryRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_unknown_at_rule(&mut self, node: &UnknownAtRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_position(&mut self, node: &Position<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_web_kit_gradient_point(&mut self, node: &WebKitGradientPoint<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_web_kit_color_stop(&mut self, node: &WebKitColorStop<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_image_set(&mut self, node: &ImageSet<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_image_set_option(&mut self, node: &ImageSetOption<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_background_position(&mut self, node: &BackgroundPosition<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_background_repeat(&mut self, node: &BackgroundRepeat) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_background(&mut self, node: &Background<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_box_shadow(&mut self, node: &BoxShadow<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_radius(&mut self, node: &BorderRadius<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_image_repeat(&mut self, node: &BorderImageRepeat) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_image_slice(&mut self, node: &BorderImageSlice<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_image(&mut self, node: &BorderImage<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_color(&mut self, node: &BorderColor<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_style(&mut self, node: &BorderStyle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_width(&mut self, node: &BorderWidth<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_block_color(&mut self, node: &BorderBlockColor<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_block_style(&mut self, node: &BorderBlockStyle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_block_width(&mut self, node: &BorderBlockWidth<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_inline_color(&mut self, node: &BorderInlineColor<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_inline_style(&mut self, node: &BorderInlineStyle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_inline_width(&mut self, node: &BorderInlineWidth<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_generic_border<S>(&mut self, node: &GenericBorder<'a, S>)
    where
        S: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_container_condition(&mut self, node: &ContainerCondition<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_container_size_feature_id(&mut self, node: &ContainerSizeFeatureId) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_style_query(&mut self, node: &StyleQuery<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_state_query(&mut self, node: &ScrollStateQuery<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_state_feature_id(&mut self, node: &ScrollStateFeatureId) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_container(&mut self, node: &Container<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_container_rule(&mut self, node: &ContainerRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_face_property(&mut self, node: &FontFaceProperty<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_source(&mut self, node: &Source<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_format(&mut self, node: &FontFormat<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_technology(&mut self, node: &FontTechnology) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_face_style(&mut self, node: &FontFaceStyle<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_palette_values_property(&mut self, node: &FontPaletteValuesProperty<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_base_palette(&mut self, node: &BasePalette) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_feature_subrule_type(&mut self, node: &FontFeatureSubruleType) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font(&mut self, node: &Font<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_face_rule(&mut self, node: &FontFaceRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_url_source(&mut self, node: &UrlSource<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_unicode_range(&mut self, node: &UnicodeRange) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_palette_values_rule(&mut self, node: &FontPaletteValuesRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_override_colors(&mut self, node: &OverrideColors<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_feature_values_rule(&mut self, node: &FontFeatureValuesRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_feature_subrule(&mut self, node: &FontFeatureSubrule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_feature_declaration(&mut self, node: &FontFeatureDeclaration<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_family_name(&mut self, node: &FamilyName<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_keyframe_selector(&mut self, node: &KeyframeSelector<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_keyframes_name(&mut self, node: &KeyframesName<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_keyframes_rule(&mut self, node: &KeyframesRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_keyframe(&mut self, node: &Keyframe<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_timeline_range_percentage(&mut self, node: &TimelineRangePercentage) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_aspect_ratio(&mut self, node: &AspectRatio<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_overflow(&mut self, node: &Overflow) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_inset_block(&mut self, node: &InsetBlock<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_inset_inline(&mut self, node: &InsetInline<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_inset(&mut self, node: &Inset<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_flex_flow(&mut self, node: &FlexFlow) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_flex(&mut self, node: &Flex<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_place_content(&mut self, node: &PlaceContent<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_place_self(&mut self, node: &PlaceSelf<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_place_items(&mut self, node: &PlaceItems<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_gap(&mut self, node: &Gap<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_track_repeat(&mut self, node: &TrackRepeat<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_grid_auto_flow(&mut self, node: &GridAutoFlow) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_grid_template(&mut self, node: &GridTemplate<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_grid(&mut self, node: &Grid<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_grid_row(&mut self, node: &GridRow<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_grid_column(&mut self, node: &GridColumn<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_grid_area(&mut self, node: &GridArea<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_margin_block(&mut self, node: &MarginBlock<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_margin_inline(&mut self, node: &MarginInline<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_margin(&mut self, node: &Margin<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_padding_block(&mut self, node: &PaddingBlock<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_padding_inline(&mut self, node: &PaddingInline<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_padding(&mut self, node: &Padding<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_margin_block(&mut self, node: &ScrollMarginBlock<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_margin_inline(&mut self, node: &ScrollMarginInline<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_margin(&mut self, node: &ScrollMargin<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_padding_block(&mut self, node: &ScrollPaddingBlock<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_padding_inline(&mut self, node: &ScrollPaddingInline<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_padding(&mut self, node: &ScrollPadding<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_page_margin_box(&mut self, node: &PageMarginBox) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_page_pseudo_class(&mut self, node: &PagePseudoClass) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_page_rule(&mut self, node: &PageRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_page_margin_rule(&mut self, node: &PageMarginRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_page_selector(&mut self, node: &PageSelector<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_parsed_component(&mut self, node: &ParsedComponent<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_multiplier(&mut self, node: &Multiplier) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_syntax_string(&mut self, node: &SyntaxString<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_syntax_component_kind(&mut self, node: &SyntaxComponentKind<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_unparsed_property(&mut self, node: &UnparsedProperty<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_custom_property(&mut self, node: &CustomProperty<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_property_rule(&mut self, node: &PropertyRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_syntax_component(&mut self, node: &SyntaxComponent<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_inset_rect(&mut self, node: &InsetRect<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_circle_shape(&mut self, node: &CircleShape<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_ellipse_shape(&mut self, node: &EllipseShape<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_polygon(&mut self, node: &Polygon<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_point(&mut self, node: &Point<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_mask(&mut self, node: &Mask<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_mask_border(&mut self, node: &MaskBorder<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_drop_shadow(&mut self, node: &DropShadow<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_default_at_rule(&mut self, node: &DefaultAtRule) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_style_sheet(&mut self, node: &StyleSheet<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_rule(&mut self, node: &MediaRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_list(&mut self, node: &MediaList<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_query(&mut self, node: &MediaQuery<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_length_value(&mut self, node: &LengthValue) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_environment_variable(&mut self, node: &EnvironmentVariable<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_url(&mut self, node: &Url<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_variable(&mut self, node: &Variable<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_dashed_ident_reference(&mut self, node: &DashedIdentReference<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_function(&mut self, node: &Function<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_function_replacement(&mut self, node: &FunctionReplacement) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_import_rule(&mut self, node: &ImportRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_style_rule(&mut self, node: &StyleRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_declaration_block(&mut self, node: &DeclarationBlock<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_transform(&mut self, node: &TextTransform) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_indent(&mut self, node: &TextIndent<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_decoration(&mut self, node: &TextDecoration<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis(&mut self, node: &TextEmphasis<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_position(&mut self, node: &TextEmphasisPosition) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_shadow(&mut self, node: &TextShadow<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_matrix_for_float(&mut self, node: &MatrixForFloat) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_matrix_3_d_for_float(&mut self, node: &Matrix3DForFloat) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_rotate(&mut self, node: &Rotate<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_cursor(&mut self, node: &Cursor<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_cursor_image(&mut self, node: &CursorImage<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_caret(&mut self, node: &Caret<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_list_style(&mut self, node: &ListStyle<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_composes(&mut self, node: &Composes<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_color_scheme(&mut self, node: &ColorScheme) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_view_transition_property(&mut self, node: &ViewTransitionProperty<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_navigation(&mut self, node: &Navigation) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_view_transition_part_selector(&mut self, node: &ViewTransitionPartSelector<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_view_transition_rule(&mut self, node: &ViewTransitionRule<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_selector_component(&mut self, node: &SelectorComponent<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_combinator(&mut self, node: &Combinator) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_attr_selector(&mut self, node: &AttrSelector<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_namespace_constraint(&mut self, node: &NamespaceConstraint<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_attr_operation(&mut self, node: &AttrOperation<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_parsed_case_sensitivity(&mut self, node: &ParsedCaseSensitivity) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_attr_selector_operator(&mut self, node: &AttrSelectorOperator) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_nth_type(&mut self, node: &NthType) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_nth_selector_data(&mut self, node: &NthSelectorData) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_direction(&mut self, node: &Direction) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_pseudo_class(&mut self, node: &PseudoClass<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_class(&mut self, node: &WebKitScrollbarPseudoClass) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_pseudo_element(&mut self, node: &PseudoElement<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_element(&mut self, node: &WebKitScrollbarPseudoElement) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_view_transition_part_name(&mut self, node: &ViewTransitionPartName<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_span(&mut self, node: &Span) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_token_or_value(&mut self, node: &TokenOrValue<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_unit(&mut self, node: &Unit) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_token(&mut self, node: &Token<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_specifier(&mut self, node: &Specifier<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_name(&mut self, node: &AnimationName<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_environment_variable_name(&mut self, node: &EnvironmentVariableName<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_ua_environment_variable(&mut self, node: &UAEnvironmentVariable) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_align_content(&mut self, node: &AlignContent) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_baseline_position(&mut self, node: &BaselinePosition) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_content_distribution(&mut self, node: &ContentDistribution) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_overflow_position(&mut self, node: &OverflowPosition) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_content_position(&mut self, node: &ContentPosition) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_justify_content(&mut self, node: &JustifyContent) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_align_self(&mut self, node: &AlignSelf) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_self_position(&mut self, node: &SelfPosition) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_justify_self(&mut self, node: &JustifySelf) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_align_items(&mut self, node: &AlignItems) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_justify_items(&mut self, node: &JustifyItems) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_legacy_justify(&mut self, node: &LegacyJustify) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_gap_value(&mut self, node: &GapValue<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_easing_function(&mut self, node: &EasingFunction) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_step_position(&mut self, node: &StepPosition) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_iteration_count(&mut self, node: &AnimationIterationCount) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_direction(&mut self, node: &AnimationDirection) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_play_state(&mut self, node: &AnimationPlayState) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_fill_mode(&mut self, node: &AnimationFillMode) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_composition(&mut self, node: &AnimationComposition) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_timeline(&mut self, node: &AnimationTimeline<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroll_axis(&mut self, node: &ScrollAxis) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scroller(&mut self, node: &Scroller) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_animation_attachment_range(&mut self, node: &AnimationAttachmentRange<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_timeline_range_name(&mut self, node: &TimelineRangeName) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_line_style(&mut self, node: &LineStyle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_side_width(&mut self, node: &BorderSideWidth<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_length_or_number(&mut self, node: &LengthOrNumber<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_image_repeat_keyword(&mut self, node: &BorderImageRepeatKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_border_image_side_width(&mut self, node: &BorderImageSideWidth<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_outline_style(&mut self, node: &OutlineStyle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_display(&mut self, node: &Display<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_display_keyword(&mut self, node: &DisplayKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_display_inside(&mut self, node: &DisplayInside) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_display_outside(&mut self, node: &DisplayOutside) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_visibility(&mut self, node: &Visibility) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_size(&mut self, node: &Size<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_max_size(&mut self, node: &MaxSize<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_box_sizing(&mut self, node: &BoxSizing) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_overflow_keyword(&mut self, node: &OverflowKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_overflow(&mut self, node: &TextOverflow) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_position_property(&mut self, node: &PositionProperty) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_size_2_d<T>(&mut self, node: &Size2D<'a, T>)
    where
        T: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_rect<T>(&mut self, node: &Rect<'a, T>)
    where
        T: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_box_decoration_break(&mut self, node: &BoxDecorationBreak) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_z_index(&mut self, node: &ZIndex) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_container_type(&mut self, node: &ContainerType) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_container_name_list(&mut self, node: &ContainerNameList<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_filter_list(&mut self, node: &FilterList<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_filter(&mut self, node: &Filter<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_flex_direction(&mut self, node: &FlexDirection) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_flex_wrap(&mut self, node: &FlexWrap) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_box_orient(&mut self, node: &BoxOrient) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_box_direction(&mut self, node: &BoxDirection) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_box_align(&mut self, node: &BoxAlign) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_box_pack(&mut self, node: &BoxPack) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_box_lines(&mut self, node: &BoxLines) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_flex_pack(&mut self, node: &FlexPack) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_flex_item_align(&mut self, node: &FlexItemAlign) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_flex_line_pack(&mut self, node: &FlexLinePack) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_weight(&mut self, node: &FontWeight<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_absolute_font_weight(&mut self, node: &AbsoluteFontWeight) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_size(&mut self, node: &FontSize<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_absolute_font_size(&mut self, node: &AbsoluteFontSize) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_relative_font_size(&mut self, node: &RelativeFontSize) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_stretch(&mut self, node: &FontStretch) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_stretch_keyword(&mut self, node: &FontStretchKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_family(&mut self, node: &FontFamily<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_generic_font_family(&mut self, node: &GenericFontFamily) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_style(&mut self, node: &FontStyle<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_font_variant_caps(&mut self, node: &FontVariantCaps) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_line_height(&mut self, node: &LineHeight<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_vertical_align(&mut self, node: &VerticalAlign<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_vertical_align_keyword(&mut self, node: &VerticalAlignKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_track_sizing(&mut self, node: &TrackSizing<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_track_list_item(&mut self, node: &TrackListItem<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_track_size(&mut self, node: &TrackSize<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_track_breadth(&mut self, node: &TrackBreadth<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_repeat_count(&mut self, node: &RepeatCount) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_auto_flow_direction(&mut self, node: &AutoFlowDirection) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_grid_template_areas(&mut self, node: &GridTemplateAreas<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_grid_line(&mut self, node: &GridLine<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_image(&mut self, node: &Image<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_gradient(&mut self, node: &Gradient<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_web_kit_gradient(&mut self, node: &WebKitGradient<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_line_direction(&mut self, node: &LineDirection<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_horizontal_position_keyword(&mut self, node: &HorizontalPositionKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_vertical_position_keyword(&mut self, node: &VerticalPositionKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_gradient_item<D>(&mut self, node: &GradientItem<'a, D>)
    where
        D: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_dimension_percentage<D>(&mut self, node: &DimensionPercentage<'a, D>)
    where
        D: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_position_component<S>(&mut self, node: &PositionComponent<'a, S>)
    where
        S: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_ending_shape(&mut self, node: &EndingShape<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_ellipse(&mut self, node: &Ellipse<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_shape_extent(&mut self, node: &ShapeExtent) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_circle(&mut self, node: &Circle<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_web_kit_gradient_point_component<S>(
        &mut self,
        node: &WebKitGradientPointComponent<'a, S>,
    ) where
        S: Visit<'a>,
    {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_number_or_percentage(&mut self, node: &NumberOrPercentage) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_background_size(&mut self, node: &BackgroundSize<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_length_percentage_or_auto(&mut self, node: &LengthPercentageOrAuto<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_background_repeat_keyword(&mut self, node: &BackgroundRepeatKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_background_attachment(&mut self, node: &BackgroundAttachment) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_background_clip(&mut self, node: &BackgroundClip) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_background_origin(&mut self, node: &BackgroundOrigin) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_list_style_type(&mut self, node: &ListStyleType<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_counter_style(&mut self, node: &CounterStyle<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_symbols_type(&mut self, node: &SymbolsType) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_predefined_counter_style(&mut self, node: &PredefinedCounterStyle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_symbol(&mut self, node: &Symbol<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_list_style_position(&mut self, node: &ListStylePosition) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_marker_side(&mut self, node: &MarkerSide) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_mask_mode(&mut self, node: &MaskMode) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_mask_clip(&mut self, node: &MaskClip) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_mask_composite(&mut self, node: &MaskComposite) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_mask_type(&mut self, node: &MaskType) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_mask_border_mode(&mut self, node: &MaskBorderMode) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_web_kit_mask_composite(&mut self, node: &WebKitMaskComposite) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_web_kit_mask_source_type(&mut self, node: &WebKitMaskSourceType) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_css_wide_keyword(&mut self, node: &CSSWideKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_custom_property_name(&mut self, node: &CustomPropertyName<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_clip_path(&mut self, node: &ClipPath<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_geometry_box(&mut self, node: &GeometryBox) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_basic_shape(&mut self, node: &BasicShape<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_shape_radius(&mut self, node: &ShapeRadius<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_svg_paint(&mut self, node: &SVGPaint<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_svg_paint_fallback(&mut self, node: &SVGPaintFallback<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_fill_rule(&mut self, node: &FillRule) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_stroke_linecap(&mut self, node: &StrokeLinecap) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_stroke_linejoin(&mut self, node: &StrokeLinejoin) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_stroke_dasharray(&mut self, node: &StrokeDasharray<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_marker(&mut self, node: &Marker<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_color_interpolation(&mut self, node: &ColorInterpolation) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_color_rendering(&mut self, node: &ColorRendering) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_shape_rendering(&mut self, node: &ShapeRendering) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_rendering(&mut self, node: &TextRendering) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_image_rendering(&mut self, node: &ImageRendering) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_transform_case(&mut self, node: &TextTransformCase) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_white_space(&mut self, node: &WhiteSpace) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_word_break(&mut self, node: &WordBreak) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_line_break(&mut self, node: &LineBreak) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_hyphens(&mut self, node: &Hyphens) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_overflow_wrap(&mut self, node: &OverflowWrap) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_align(&mut self, node: &TextAlign) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_align_last(&mut self, node: &TextAlignLast) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_justify(&mut self, node: &TextJustify) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_spacing(&mut self, node: &Spacing<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_decoration_line(&mut self, node: &TextDecorationLine<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_exclusive_text_decoration_line(&mut self, node: &ExclusiveTextDecorationLine) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_other_text_decoration_line(&mut self, node: &OtherTextDecorationLine) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_decoration_style(&mut self, node: &TextDecorationStyle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_decoration_thickness(&mut self, node: &TextDecorationThickness<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_decoration_skip_ink(&mut self, node: &TextDecorationSkipInk) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_style(&mut self, node: &TextEmphasisStyle<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_fill_mode(&mut self, node: &TextEmphasisFillMode) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_shape(&mut self, node: &TextEmphasisShape) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_position_horizontal(&mut self, node: &TextEmphasisPositionHorizontal) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_emphasis_position_vertical(&mut self, node: &TextEmphasisPositionVertical) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_size_adjust(&mut self, node: &TextSizeAdjust) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_text_direction(&mut self, node: &TextDirection) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_unicode_bidi(&mut self, node: &UnicodeBidi) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_transform(&mut self, node: &Transform<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_transform_style(&mut self, node: &TransformStyle) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_transform_box(&mut self, node: &TransformBox) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_backface_visibility(&mut self, node: &BackfaceVisibility) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_perspective(&mut self, node: &Perspective<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_translate(&mut self, node: &Translate<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_scale(&mut self, node: &Scale<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_resize(&mut self, node: &Resize) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_cursor_keyword(&mut self, node: &CursorKeyword) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_color_or_auto(&mut self, node: &ColorOrAuto<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_caret_shape(&mut self, node: &CaretShape) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_user_select(&mut self, node: &UserSelect) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_appearance(&mut self, node: &Appearance<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_print_color_adjust(&mut self, node: &PrintColorAdjust) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_view_transition_name(&mut self, node: &ViewTransitionName<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_none_or_custom_ident_list(&mut self, node: &NoneOrCustomIdentList<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_view_transition_group(&mut self, node: &ViewTransitionGroup<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_media_feature(&mut self, node: &MediaFeature<'a>) {
        self.visit_media_feature_children(node);
    }
    ///Continues traversal of [`MediaFeature`] without redispatching its visitor callback.
    fn visit_media_feature_children(&mut self, node: &MediaFeature<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::MediaFeature);
        }
        Visit::visit(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::MediaFeature);
        }
    }
    #[inline]
    fn visit_container_size_feature(&mut self, node: &ContainerSizeFeature<'a>) {
        self.visit_container_size_feature_children(node);
    }
    ///Continues traversal of [`ContainerSizeFeature`] without redispatching its visitor callback.
    fn visit_container_size_feature_children(&mut self, node: &ContainerSizeFeature<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::ContainerSizeFeature);
        }
        Visit::visit(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::ContainerSizeFeature);
        }
    }
    #[inline]
    fn visit_scroll_state_feature(&mut self, node: &ScrollStateFeature<'a>) {
        self.visit_scroll_state_feature_children(node);
    }
    ///Continues traversal of [`ScrollStateFeature`] without redispatching its visitor callback.
    fn visit_scroll_state_feature_children(&mut self, node: &ScrollStateFeature<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::ScrollStateFeature);
        }
        Visit::visit(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::ScrollStateFeature);
        }
    }
    #[inline]
    fn visit_selector_list(&mut self, node: &SelectorList<'a>) {
        self.visit_selector_list_children(node);
    }
    ///Continues traversal of [`SelectorList`] without redispatching its visitor callback.
    fn visit_selector_list_children(&mut self, node: &SelectorList<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::SelectorList);
        }
        for value_0 in (node).iter() {
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
    fn visit_selector(&mut self, node: &Selector<'a>) {
        self.visit_selector_children(node);
    }
    ///Continues traversal of [`Selector`] without redispatching its visitor callback.
    fn visit_selector_children(&mut self, node: &Selector<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::Selector);
        }
        for value_0 in (node).iter() {
            Visit::visit(value_0, visitor);
        }
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::Selector);
        }
    }
    #[inline]
    fn visit_animation_range_start(&mut self, node: &AnimationRangeStart<'a>) {
        self.visit_animation_range_start_children(node);
    }
    ///Continues traversal of [`AnimationRangeStart`] without redispatching its visitor callback.
    fn visit_animation_range_start_children(&mut self, node: &AnimationRangeStart<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::AnimationRangeStart);
        }
        Visit::visit(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::AnimationRangeStart);
        }
    }
    #[inline]
    fn visit_animation_range_end(&mut self, node: &AnimationRangeEnd<'a>) {
        self.visit_animation_range_end_children(node);
    }
    ///Continues traversal of [`AnimationRangeEnd`] without redispatching its visitor callback.
    fn visit_animation_range_end_children(&mut self, node: &AnimationRangeEnd<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::AnimationRangeEnd);
        }
        Visit::visit(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::AnimationRangeEnd);
        }
    }
    #[inline]
    fn visit_length_percentage(&mut self, node: &LengthPercentage<'a>) {
        self.visit_length_percentage_children(node);
    }
    ///Continues traversal of [`LengthPercentage`] without redispatching its visitor callback.
    fn visit_length_percentage_children(&mut self, node: &LengthPercentage<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::LengthPercentage);
        }
        Visit::visit(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::LengthPercentage);
        }
    }
    #[inline]
    fn visit_angle_percentage(&mut self, node: &AnglePercentage<'a>) {
        self.visit_angle_percentage_children(node);
    }
    ///Continues traversal of [`AnglePercentage`] without redispatching its visitor callback.
    fn visit_angle_percentage_children(&mut self, node: &AnglePercentage<'a>) {
        let visitor = self;
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::ENTER_NODE)
        {
            visitor.enter_node(AstType::AnglePercentage);
        }
        Visit::visit(node, visitor);
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::LEAVE_NODE)
        {
            visitor.leave_node(AstType::AnglePercentage);
        }
    }
    #[inline]
    fn visit_declaration(&mut self, node: &Declaration<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_property_id(&mut self, node: &PropertyId<'a>) {
        Visit::visit_children(node, self);
    }
    #[inline]
    fn visit_vendor_prefix(&mut self, node: &VendorPrefix) {
        Visit::visit_children(node, self);
    }
}
/// Traversal implemented by CSS AST nodes.
pub trait Visit<'a> {
    /// Dispatches this node to its typed visitor callback.
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT);
    /// Continues traversal without redispatching this node's visitor callback.
    #[inline]
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, _visitor: &mut VisitorT) {}
}
macro_rules! impl_leaf_visit {
    ($($ty:ty),+ $(,)?) => {
        $(impl < 'a > Visit < 'a > for $ty { fn visit < VisitorT : ? Sized + Visitor < 'a
        >> (& self, _visitor : & mut VisitorT,) {} })+
    };
}
impl_leaf_visit!(bool, char, f32, i32, u8, u16, u32, usize);
impl<'a, T: ?Sized + Visit<'a>> Visit<'a> for rocketcss_allocator::boxed::Box<'a, T> {
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        Visit::visit(self.as_ref(), visitor);
    }
}
impl<'a, T: Visit<'a> + Unpin> Visit<'a> for rocketcss_allocator::vec::Vec<'a, T> {
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        for value in self {
            Visit::visit(value, visitor);
        }
    }
}
impl<'a, T: Visit<'a>> Visit<'a> for Option<T> {
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        if let Some(value) = self {
            Visit::visit(value, visitor);
        }
    }
}
impl<'a> Visit<'a> for &'a str {
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::VISIT_STR)
        {
            visitor.visit_str(self);
        }
    }
}
impl<'a> Visit<'a> for VendorPrefix {
    fn visit<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
        if visitor
            .visitor_methods()
            .contains(VisitorMethods::VISIT_VENDOR_PREFIX)
        {
            visitor.visit_vendor_prefix(self);
        } else {
            Visit::visit_children(self, visitor);
        }
    }
    fn visit_children<VisitorT: ?Sized + Visitor<'a>>(&self, visitor: &mut VisitorT) {
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
