//! Generated typed visitor API. Regenerate with `cargo run -p rocketcss_ast_tools`.
#![allow(
    clippy::match_same_arms,
    clippy::needless_borrow,
    unused_imports,
    unused_variables
)]
use crate::AstType;
use rocketcss_ast::*;
pub mod color;
pub mod css_rule;
pub mod length;
pub mod media;
pub mod properties;
pub mod rules;
pub mod selector;
pub mod span;
pub mod token;
pub mod values;
pub trait Visit<'a> {
    #[inline]
    fn enter_node(&mut self, _kind: AstType) {}
    #[inline]
    fn leave_node(&mut self, _kind: AstType) {}
    #[inline]
    fn visit_str(&mut self, _value: &&'a str) {}
    #[inline]
    fn visit_css_color(&mut self, node: &CssColor<'a>) {
        color::walk_css_color(self, node);
    }
    #[inline]
    fn visit_rgba(&mut self, node: &RGBA) {
        color::walk_rgba(self, node);
    }
    #[inline]
    fn visit_lab_color(&mut self, node: &LABColor) {
        color::walk_lab_color(self, node);
    }
    #[inline]
    fn visit_predefined_color(&mut self, node: &PredefinedColor) {
        color::walk_predefined_color(self, node);
    }
    #[inline]
    fn visit_float_color(&mut self, node: &FloatColor) {
        color::walk_float_color(self, node);
    }
    #[inline]
    fn visit_light_dark(&mut self, node: &LightDark<'a>) {
        color::walk_light_dark(self, node);
    }
    #[inline]
    fn visit_system_color(&mut self, node: &SystemColor) {
        color::walk_system_color(self, node);
    }
    #[inline]
    fn visit_unresolved_color(&mut self, node: &UnresolvedColor<'a>) {
        color::walk_unresolved_color(self, node);
    }
    #[inline]
    fn visit_css_rule(&mut self, node: &CssRule<'a>) {
        css_rule::walk_css_rule(self, node);
    }
    #[inline]
    fn visit_length(&mut self, node: &Length<'a>) {
        length::walk_length(self, node);
    }
    #[inline]
    fn visit_length_unit(&mut self, node: &LengthUnit) {
        length::walk_length_unit(self, node);
    }
    #[inline]
    fn visit_calc<V>(&mut self, node: &Calc<'a, V>)
    where
        V: VisitNode<'a, Self>,
    {
        length::walk_calc(self, node);
    }
    #[inline]
    fn visit_math_function<V>(&mut self, node: &MathFunction<'a, V>)
    where
        V: VisitNode<'a, Self>,
    {
        length::walk_math_function(self, node);
    }
    #[inline]
    fn visit_rounding_strategy(&mut self, node: &RoundingStrategy) {
        length::walk_rounding_strategy(self, node);
    }
    #[inline]
    fn visit_resolution(&mut self, node: &Resolution) {
        length::walk_resolution(self, node);
    }
    #[inline]
    fn visit_ratio(&mut self, node: &Ratio) {
        length::walk_ratio(self, node);
    }
    #[inline]
    fn visit_angle(&mut self, node: &Angle) {
        length::walk_angle(self, node);
    }
    #[inline]
    fn visit_time(&mut self, node: &Time) {
        length::walk_time(self, node);
    }
    #[inline]
    fn visit_media_condition(&mut self, node: &MediaCondition<'a>) {
        media::walk_media_condition(self, node);
    }
    #[inline]
    fn visit_query_feature<FeatureId>(&mut self, node: &QueryFeature<'a, FeatureId>)
    where
        FeatureId: VisitNode<'a, Self>,
    {
        media::walk_query_feature(self, node);
    }
    #[inline]
    fn visit_media_feature_name<FeatureId>(&mut self, node: &MediaFeatureName<'a, FeatureId>)
    where
        FeatureId: VisitNode<'a, Self>,
    {
        media::walk_media_feature_name(self, node);
    }
    #[inline]
    fn visit_media_feature_id(&mut self, node: &MediaFeatureId) {
        media::walk_media_feature_id(self, node);
    }
    #[inline]
    fn visit_media_feature_value(&mut self, node: &MediaFeatureValue<'a>) {
        media::walk_media_feature_value(self, node);
    }
    #[inline]
    fn visit_media_feature_comparison(&mut self, node: &MediaFeatureComparison) {
        media::walk_media_feature_comparison(self, node);
    }
    #[inline]
    fn visit_operator(&mut self, node: &Operator) {
        media::walk_operator(self, node);
    }
    #[inline]
    fn visit_media_type(&mut self, node: &MediaType<'a>) {
        media::walk_media_type(self, node);
    }
    #[inline]
    fn visit_qualifier(&mut self, node: &Qualifier) {
        media::walk_qualifier(self, node);
    }
    #[inline]
    fn visit_supports_condition(&mut self, node: &SupportsCondition<'a>) {
        media::walk_supports_condition(self, node);
    }
    #[inline]
    fn visit_blend_mode(&mut self, node: &BlendMode) {
        properties::walk_blend_mode(self, node);
    }
    #[inline]
    fn visit_keyframe_selector(&mut self, node: &KeyframeSelector<'a>) {
        rules::walk_keyframe_selector(self, node);
    }
    #[inline]
    fn visit_keyframes_name(&mut self, node: &KeyframesName<'a>) {
        rules::walk_keyframes_name(self, node);
    }
    #[inline]
    fn visit_font_face_property(&mut self, node: &FontFaceProperty<'a>) {
        rules::walk_font_face_property(self, node);
    }
    #[inline]
    fn visit_source(&mut self, node: &Source<'a>) {
        rules::walk_source(self, node);
    }
    #[inline]
    fn visit_font_format(&mut self, node: &FontFormat<'a>) {
        rules::walk_font_format(self, node);
    }
    #[inline]
    fn visit_font_technology(&mut self, node: &FontTechnology) {
        rules::walk_font_technology(self, node);
    }
    #[inline]
    fn visit_font_face_style(&mut self, node: &FontFaceStyle<'a>) {
        rules::walk_font_face_style(self, node);
    }
    #[inline]
    fn visit_font_palette_values_property(&mut self, node: &FontPaletteValuesProperty<'a>) {
        rules::walk_font_palette_values_property(self, node);
    }
    #[inline]
    fn visit_base_palette(&mut self, node: &BasePalette) {
        rules::walk_base_palette(self, node);
    }
    #[inline]
    fn visit_font_feature_subrule_type(&mut self, node: &FontFeatureSubruleType) {
        rules::walk_font_feature_subrule_type(self, node);
    }
    #[inline]
    fn visit_page_margin_box(&mut self, node: &PageMarginBox) {
        rules::walk_page_margin_box(self, node);
    }
    #[inline]
    fn visit_page_pseudo_class(&mut self, node: &PagePseudoClass) {
        rules::walk_page_pseudo_class(self, node);
    }
    #[inline]
    fn visit_parsed_component(&mut self, node: &ParsedComponent<'a>) {
        rules::walk_parsed_component(self, node);
    }
    #[inline]
    fn visit_multiplier(&mut self, node: &Multiplier) {
        rules::walk_multiplier(self, node);
    }
    #[inline]
    fn visit_syntax_string(&mut self, node: &SyntaxString<'a>) {
        rules::walk_syntax_string(self, node);
    }
    #[inline]
    fn visit_syntax_component_kind(&mut self, node: &SyntaxComponentKind<'a>) {
        rules::walk_syntax_component_kind(self, node);
    }
    #[inline]
    fn visit_container_condition(&mut self, node: &ContainerCondition<'a>) {
        rules::walk_container_condition(self, node);
    }
    #[inline]
    fn visit_container_size_feature_id(&mut self, node: &ContainerSizeFeatureId) {
        rules::walk_container_size_feature_id(self, node);
    }
    #[inline]
    fn visit_style_query(&mut self, node: &StyleQuery<'a>) {
        rules::walk_style_query(self, node);
    }
    #[inline]
    fn visit_scroll_state_query(&mut self, node: &ScrollStateQuery<'a>) {
        rules::walk_scroll_state_query(self, node);
    }
    #[inline]
    fn visit_scroll_state_feature_id(&mut self, node: &ScrollStateFeatureId) {
        rules::walk_scroll_state_feature_id(self, node);
    }
    #[inline]
    fn visit_view_transition_property(&mut self, node: &ViewTransitionProperty<'a>) {
        rules::walk_view_transition_property(self, node);
    }
    #[inline]
    fn visit_navigation(&mut self, node: &Navigation) {
        rules::walk_navigation(self, node);
    }
    #[inline]
    fn visit_default_at_rule(&mut self, node: &DefaultAtRule) {
        rules::walk_default_at_rule(self, node);
    }
    #[inline]
    fn visit_style_sheet(&mut self, node: &StyleSheet<'a>) {
        rules::walk_style_sheet(self, node);
    }
    #[inline]
    fn visit_media_rule(&mut self, node: &MediaRule<'a>) {
        rules::walk_media_rule(self, node);
    }
    #[inline]
    fn visit_media_list(&mut self, node: &MediaList<'a>) {
        rules::walk_media_list(self, node);
    }
    #[inline]
    fn visit_media_query(&mut self, node: &MediaQuery<'a>) {
        rules::walk_media_query(self, node);
    }
    #[inline]
    fn visit_length_value(&mut self, node: &LengthValue) {
        rules::walk_length_value(self, node);
    }
    #[inline]
    fn visit_environment_variable(&mut self, node: &EnvironmentVariable<'a>) {
        rules::walk_environment_variable(self, node);
    }
    #[inline]
    fn visit_url(&mut self, node: &Url<'a>) {
        rules::walk_url(self, node);
    }
    #[inline]
    fn visit_variable(&mut self, node: &Variable<'a>) {
        rules::walk_variable(self, node);
    }
    #[inline]
    fn visit_dashed_ident_reference(&mut self, node: &DashedIdentReference<'a>) {
        rules::walk_dashed_ident_reference(self, node);
    }
    #[inline]
    fn visit_function(&mut self, node: &Function<'a>) {
        rules::walk_function(self, node);
    }
    #[inline]
    fn visit_function_replacement(&mut self, node: &FunctionReplacement) {
        rules::walk_function_replacement(self, node);
    }
    #[inline]
    fn visit_import_rule(&mut self, node: &ImportRule<'a>) {
        rules::walk_import_rule(self, node);
    }
    #[inline]
    fn visit_style_rule(&mut self, node: &StyleRule<'a>) {
        rules::walk_style_rule(self, node);
    }
    #[inline]
    fn visit_declaration_block(&mut self, node: &DeclarationBlock<'a>) {
        rules::walk_declaration_block(self, node);
    }
    #[inline]
    fn visit_position(&mut self, node: &Position<'a>) {
        rules::walk_position(self, node);
    }
    #[inline]
    fn visit_web_kit_gradient_point(&mut self, node: &WebKitGradientPoint<'a>) {
        rules::walk_web_kit_gradient_point(self, node);
    }
    #[inline]
    fn visit_web_kit_color_stop(&mut self, node: &WebKitColorStop<'a>) {
        rules::walk_web_kit_color_stop(self, node);
    }
    #[inline]
    fn visit_image_set(&mut self, node: &ImageSet<'a>) {
        rules::walk_image_set(self, node);
    }
    #[inline]
    fn visit_image_set_option(&mut self, node: &ImageSetOption<'a>) {
        rules::walk_image_set_option(self, node);
    }
    #[inline]
    fn visit_background_position(&mut self, node: &BackgroundPosition<'a>) {
        rules::walk_background_position(self, node);
    }
    #[inline]
    fn visit_background_repeat(&mut self, node: &BackgroundRepeat) {
        rules::walk_background_repeat(self, node);
    }
    #[inline]
    fn visit_background(&mut self, node: &Background<'a>) {
        rules::walk_background(self, node);
    }
    #[inline]
    fn visit_box_shadow(&mut self, node: &BoxShadow<'a>) {
        rules::walk_box_shadow(self, node);
    }
    #[inline]
    fn visit_aspect_ratio(&mut self, node: &AspectRatio<'a>) {
        rules::walk_aspect_ratio(self, node);
    }
    #[inline]
    fn visit_overflow(&mut self, node: &Overflow) {
        rules::walk_overflow(self, node);
    }
    #[inline]
    fn visit_inset_block(&mut self, node: &InsetBlock<'a>) {
        rules::walk_inset_block(self, node);
    }
    #[inline]
    fn visit_inset_inline(&mut self, node: &InsetInline<'a>) {
        rules::walk_inset_inline(self, node);
    }
    #[inline]
    fn visit_inset(&mut self, node: &Inset<'a>) {
        rules::walk_inset(self, node);
    }
    #[inline]
    fn visit_border_radius(&mut self, node: &BorderRadius<'a>) {
        rules::walk_border_radius(self, node);
    }
    #[inline]
    fn visit_border_image_repeat(&mut self, node: &BorderImageRepeat) {
        rules::walk_border_image_repeat(self, node);
    }
    #[inline]
    fn visit_border_image_slice(&mut self, node: &BorderImageSlice<'a>) {
        rules::walk_border_image_slice(self, node);
    }
    #[inline]
    fn visit_border_image(&mut self, node: &BorderImage<'a>) {
        rules::walk_border_image(self, node);
    }
    #[inline]
    fn visit_border_color(&mut self, node: &BorderColor<'a>) {
        rules::walk_border_color(self, node);
    }
    #[inline]
    fn visit_border_style(&mut self, node: &BorderStyle) {
        rules::walk_border_style(self, node);
    }
    #[inline]
    fn visit_border_width(&mut self, node: &BorderWidth<'a>) {
        rules::walk_border_width(self, node);
    }
    #[inline]
    fn visit_border_block_color(&mut self, node: &BorderBlockColor<'a>) {
        rules::walk_border_block_color(self, node);
    }
    #[inline]
    fn visit_border_block_style(&mut self, node: &BorderBlockStyle) {
        rules::walk_border_block_style(self, node);
    }
    #[inline]
    fn visit_border_block_width(&mut self, node: &BorderBlockWidth<'a>) {
        rules::walk_border_block_width(self, node);
    }
    #[inline]
    fn visit_border_inline_color(&mut self, node: &BorderInlineColor<'a>) {
        rules::walk_border_inline_color(self, node);
    }
    #[inline]
    fn visit_border_inline_style(&mut self, node: &BorderInlineStyle) {
        rules::walk_border_inline_style(self, node);
    }
    #[inline]
    fn visit_border_inline_width(&mut self, node: &BorderInlineWidth<'a>) {
        rules::walk_border_inline_width(self, node);
    }
    #[inline]
    fn visit_generic_border<S>(&mut self, node: &GenericBorder<'a, S>)
    where
        S: VisitNode<'a, Self>,
    {
        rules::walk_generic_border(self, node);
    }
    #[inline]
    fn visit_flex_flow(&mut self, node: &FlexFlow) {
        rules::walk_flex_flow(self, node);
    }
    #[inline]
    fn visit_flex(&mut self, node: &Flex<'a>) {
        rules::walk_flex(self, node);
    }
    #[inline]
    fn visit_place_content(&mut self, node: &PlaceContent<'a>) {
        rules::walk_place_content(self, node);
    }
    #[inline]
    fn visit_place_self(&mut self, node: &PlaceSelf<'a>) {
        rules::walk_place_self(self, node);
    }
    #[inline]
    fn visit_place_items(&mut self, node: &PlaceItems<'a>) {
        rules::walk_place_items(self, node);
    }
    #[inline]
    fn visit_gap(&mut self, node: &Gap<'a>) {
        rules::walk_gap(self, node);
    }
    #[inline]
    fn visit_track_repeat(&mut self, node: &TrackRepeat<'a>) {
        rules::walk_track_repeat(self, node);
    }
    #[inline]
    fn visit_grid_auto_flow(&mut self, node: &GridAutoFlow) {
        rules::walk_grid_auto_flow(self, node);
    }
    #[inline]
    fn visit_grid_template(&mut self, node: &GridTemplate<'a>) {
        rules::walk_grid_template(self, node);
    }
    #[inline]
    fn visit_grid(&mut self, node: &Grid<'a>) {
        rules::walk_grid(self, node);
    }
    #[inline]
    fn visit_grid_row(&mut self, node: &GridRow<'a>) {
        rules::walk_grid_row(self, node);
    }
    #[inline]
    fn visit_grid_column(&mut self, node: &GridColumn<'a>) {
        rules::walk_grid_column(self, node);
    }
    #[inline]
    fn visit_grid_area(&mut self, node: &GridArea<'a>) {
        rules::walk_grid_area(self, node);
    }
    #[inline]
    fn visit_margin_block(&mut self, node: &MarginBlock<'a>) {
        rules::walk_margin_block(self, node);
    }
    #[inline]
    fn visit_margin_inline(&mut self, node: &MarginInline<'a>) {
        rules::walk_margin_inline(self, node);
    }
    #[inline]
    fn visit_margin(&mut self, node: &Margin<'a>) {
        rules::walk_margin(self, node);
    }
    #[inline]
    fn visit_padding_block(&mut self, node: &PaddingBlock<'a>) {
        rules::walk_padding_block(self, node);
    }
    #[inline]
    fn visit_padding_inline(&mut self, node: &PaddingInline<'a>) {
        rules::walk_padding_inline(self, node);
    }
    #[inline]
    fn visit_padding(&mut self, node: &Padding<'a>) {
        rules::walk_padding(self, node);
    }
    #[inline]
    fn visit_scroll_margin_block(&mut self, node: &ScrollMarginBlock<'a>) {
        rules::walk_scroll_margin_block(self, node);
    }
    #[inline]
    fn visit_scroll_margin_inline(&mut self, node: &ScrollMarginInline<'a>) {
        rules::walk_scroll_margin_inline(self, node);
    }
    #[inline]
    fn visit_scroll_margin(&mut self, node: &ScrollMargin<'a>) {
        rules::walk_scroll_margin(self, node);
    }
    #[inline]
    fn visit_scroll_padding_block(&mut self, node: &ScrollPaddingBlock<'a>) {
        rules::walk_scroll_padding_block(self, node);
    }
    #[inline]
    fn visit_scroll_padding_inline(&mut self, node: &ScrollPaddingInline<'a>) {
        rules::walk_scroll_padding_inline(self, node);
    }
    #[inline]
    fn visit_scroll_padding(&mut self, node: &ScrollPadding<'a>) {
        rules::walk_scroll_padding(self, node);
    }
    #[inline]
    fn visit_font(&mut self, node: &Font<'a>) {
        rules::walk_font(self, node);
    }
    #[inline]
    fn visit_transition(&mut self, node: &Transition<'a>) {
        rules::walk_transition(self, node);
    }
    #[inline]
    fn visit_scroll_timeline(&mut self, node: &ScrollTimeline) {
        rules::walk_scroll_timeline(self, node);
    }
    #[inline]
    fn visit_view_timeline(&mut self, node: &ViewTimeline<'a>) {
        rules::walk_view_timeline(self, node);
    }
    #[inline]
    fn visit_animation_range(&mut self, node: &AnimationRange<'a>) {
        rules::walk_animation_range(self, node);
    }
    #[inline]
    fn visit_animation(&mut self, node: &Animation<'a>) {
        rules::walk_animation(self, node);
    }
    #[inline]
    fn visit_matrix_for_float(&mut self, node: &MatrixForFloat) {
        rules::walk_matrix_for_float(self, node);
    }
    #[inline]
    fn visit_matrix_3_d_for_float(&mut self, node: &Matrix3DForFloat) {
        rules::walk_matrix_3_d_for_float(self, node);
    }
    #[inline]
    fn visit_rotate(&mut self, node: &Rotate<'a>) {
        rules::walk_rotate(self, node);
    }
    #[inline]
    fn visit_text_transform(&mut self, node: &TextTransform) {
        rules::walk_text_transform(self, node);
    }
    #[inline]
    fn visit_text_indent(&mut self, node: &TextIndent<'a>) {
        rules::walk_text_indent(self, node);
    }
    #[inline]
    fn visit_text_decoration(&mut self, node: &TextDecoration<'a>) {
        rules::walk_text_decoration(self, node);
    }
    #[inline]
    fn visit_text_emphasis(&mut self, node: &TextEmphasis<'a>) {
        rules::walk_text_emphasis(self, node);
    }
    #[inline]
    fn visit_text_emphasis_position(&mut self, node: &TextEmphasisPosition) {
        rules::walk_text_emphasis_position(self, node);
    }
    #[inline]
    fn visit_text_shadow(&mut self, node: &TextShadow<'a>) {
        rules::walk_text_shadow(self, node);
    }
    #[inline]
    fn visit_cursor(&mut self, node: &Cursor<'a>) {
        rules::walk_cursor(self, node);
    }
    #[inline]
    fn visit_cursor_image(&mut self, node: &CursorImage<'a>) {
        rules::walk_cursor_image(self, node);
    }
    #[inline]
    fn visit_caret(&mut self, node: &Caret<'a>) {
        rules::walk_caret(self, node);
    }
    #[inline]
    fn visit_list_style(&mut self, node: &ListStyle<'a>) {
        rules::walk_list_style(self, node);
    }
    #[inline]
    fn visit_composes(&mut self, node: &Composes<'a>) {
        rules::walk_composes(self, node);
    }
    #[inline]
    fn visit_inset_rect(&mut self, node: &InsetRect<'a>) {
        rules::walk_inset_rect(self, node);
    }
    #[inline]
    fn visit_circle_shape(&mut self, node: &CircleShape<'a>) {
        rules::walk_circle_shape(self, node);
    }
    #[inline]
    fn visit_ellipse_shape(&mut self, node: &EllipseShape<'a>) {
        rules::walk_ellipse_shape(self, node);
    }
    #[inline]
    fn visit_polygon(&mut self, node: &Polygon<'a>) {
        rules::walk_polygon(self, node);
    }
    #[inline]
    fn visit_point(&mut self, node: &Point<'a>) {
        rules::walk_point(self, node);
    }
    #[inline]
    fn visit_mask(&mut self, node: &Mask<'a>) {
        rules::walk_mask(self, node);
    }
    #[inline]
    fn visit_mask_border(&mut self, node: &MaskBorder<'a>) {
        rules::walk_mask_border(self, node);
    }
    #[inline]
    fn visit_drop_shadow(&mut self, node: &DropShadow<'a>) {
        rules::walk_drop_shadow(self, node);
    }
    #[inline]
    fn visit_container(&mut self, node: &Container<'a>) {
        rules::walk_container(self, node);
    }
    #[inline]
    fn visit_color_scheme(&mut self, node: &ColorScheme) {
        rules::walk_color_scheme(self, node);
    }
    #[inline]
    fn visit_unparsed_property(&mut self, node: &UnparsedProperty<'a>) {
        rules::walk_unparsed_property(self, node);
    }
    #[inline]
    fn visit_custom_property(&mut self, node: &CustomProperty<'a>) {
        rules::walk_custom_property(self, node);
    }
    #[inline]
    fn visit_view_transition_part_selector(&mut self, node: &ViewTransitionPartSelector<'a>) {
        rules::walk_view_transition_part_selector(self, node);
    }
    #[inline]
    fn visit_keyframes_rule(&mut self, node: &KeyframesRule<'a>) {
        rules::walk_keyframes_rule(self, node);
    }
    #[inline]
    fn visit_keyframe(&mut self, node: &Keyframe<'a>) {
        rules::walk_keyframe(self, node);
    }
    #[inline]
    fn visit_timeline_range_percentage(&mut self, node: &TimelineRangePercentage) {
        rules::walk_timeline_range_percentage(self, node);
    }
    #[inline]
    fn visit_font_face_rule(&mut self, node: &FontFaceRule<'a>) {
        rules::walk_font_face_rule(self, node);
    }
    #[inline]
    fn visit_url_source(&mut self, node: &UrlSource<'a>) {
        rules::walk_url_source(self, node);
    }
    #[inline]
    fn visit_unicode_range(&mut self, node: &UnicodeRange) {
        rules::walk_unicode_range(self, node);
    }
    #[inline]
    fn visit_font_palette_values_rule(&mut self, node: &FontPaletteValuesRule<'a>) {
        rules::walk_font_palette_values_rule(self, node);
    }
    #[inline]
    fn visit_override_colors(&mut self, node: &OverrideColors<'a>) {
        rules::walk_override_colors(self, node);
    }
    #[inline]
    fn visit_font_feature_values_rule(&mut self, node: &FontFeatureValuesRule<'a>) {
        rules::walk_font_feature_values_rule(self, node);
    }
    #[inline]
    fn visit_font_feature_subrule(&mut self, node: &FontFeatureSubrule<'a>) {
        rules::walk_font_feature_subrule(self, node);
    }
    #[inline]
    fn visit_font_feature_declaration(&mut self, node: &FontFeatureDeclaration<'a>) {
        rules::walk_font_feature_declaration(self, node);
    }
    #[inline]
    fn visit_family_name(&mut self, node: &FamilyName<'a>) {
        rules::walk_family_name(self, node);
    }
    #[inline]
    fn visit_page_rule(&mut self, node: &PageRule<'a>) {
        rules::walk_page_rule(self, node);
    }
    #[inline]
    fn visit_page_margin_rule(&mut self, node: &PageMarginRule<'a>) {
        rules::walk_page_margin_rule(self, node);
    }
    #[inline]
    fn visit_page_selector(&mut self, node: &PageSelector<'a>) {
        rules::walk_page_selector(self, node);
    }
    #[inline]
    fn visit_supports_rule(&mut self, node: &SupportsRule<'a>) {
        rules::walk_supports_rule(self, node);
    }
    #[inline]
    fn visit_counter_style_rule(&mut self, node: &CounterStyleRule<'a>) {
        rules::walk_counter_style_rule(self, node);
    }
    #[inline]
    fn visit_namespace_rule(&mut self, node: &NamespaceRule<'a>) {
        rules::walk_namespace_rule(self, node);
    }
    #[inline]
    fn visit_moz_document_rule(&mut self, node: &MozDocumentRule<'a>) {
        rules::walk_moz_document_rule(self, node);
    }
    #[inline]
    fn visit_nesting_rule(&mut self, node: &NestingRule<'a>) {
        rules::walk_nesting_rule(self, node);
    }
    #[inline]
    fn visit_nested_declarations_rule(&mut self, node: &NestedDeclarationsRule<'a>) {
        rules::walk_nested_declarations_rule(self, node);
    }
    #[inline]
    fn visit_viewport_rule(&mut self, node: &ViewportRule<'a>) {
        rules::walk_viewport_rule(self, node);
    }
    #[inline]
    fn visit_custom_media_rule(&mut self, node: &CustomMediaRule<'a>) {
        rules::walk_custom_media_rule(self, node);
    }
    #[inline]
    fn visit_layer_statement_rule(&mut self, node: &LayerStatementRule<'a>) {
        rules::walk_layer_statement_rule(self, node);
    }
    #[inline]
    fn visit_layer_block_rule(&mut self, node: &LayerBlockRule<'a>) {
        rules::walk_layer_block_rule(self, node);
    }
    #[inline]
    fn visit_property_rule(&mut self, node: &PropertyRule<'a>) {
        rules::walk_property_rule(self, node);
    }
    #[inline]
    fn visit_syntax_component(&mut self, node: &SyntaxComponent<'a>) {
        rules::walk_syntax_component(self, node);
    }
    #[inline]
    fn visit_container_rule(&mut self, node: &ContainerRule<'a>) {
        rules::walk_container_rule(self, node);
    }
    #[inline]
    fn visit_scope_rule(&mut self, node: &ScopeRule<'a>) {
        rules::walk_scope_rule(self, node);
    }
    #[inline]
    fn visit_starting_style_rule(&mut self, node: &StartingStyleRule<'a>) {
        rules::walk_starting_style_rule(self, node);
    }
    #[inline]
    fn visit_view_transition_rule(&mut self, node: &ViewTransitionRule<'a>) {
        rules::walk_view_transition_rule(self, node);
    }
    #[inline]
    fn visit_position_try_rule(&mut self, node: &PositionTryRule<'a>) {
        rules::walk_position_try_rule(self, node);
    }
    #[inline]
    fn visit_unknown_at_rule(&mut self, node: &UnknownAtRule<'a>) {
        rules::walk_unknown_at_rule(self, node);
    }
    #[inline]
    fn visit_selector_component(&mut self, node: &SelectorComponent<'a>) {
        selector::walk_selector_component(self, node);
    }
    #[inline]
    fn visit_combinator(&mut self, node: &Combinator) {
        selector::walk_combinator(self, node);
    }
    #[inline]
    fn visit_attr_selector(&mut self, node: &AttrSelector<'a>) {
        selector::walk_attr_selector(self, node);
    }
    #[inline]
    fn visit_namespace_constraint(&mut self, node: &NamespaceConstraint<'a>) {
        selector::walk_namespace_constraint(self, node);
    }
    #[inline]
    fn visit_attr_operation(&mut self, node: &AttrOperation<'a>) {
        selector::walk_attr_operation(self, node);
    }
    #[inline]
    fn visit_parsed_case_sensitivity(&mut self, node: &ParsedCaseSensitivity) {
        selector::walk_parsed_case_sensitivity(self, node);
    }
    #[inline]
    fn visit_attr_selector_operator(&mut self, node: &AttrSelectorOperator) {
        selector::walk_attr_selector_operator(self, node);
    }
    #[inline]
    fn visit_nth_type(&mut self, node: &NthType) {
        selector::walk_nth_type(self, node);
    }
    #[inline]
    fn visit_nth_selector_data(&mut self, node: &NthSelectorData) {
        selector::walk_nth_selector_data(self, node);
    }
    #[inline]
    fn visit_direction(&mut self, node: &Direction) {
        selector::walk_direction(self, node);
    }
    #[inline]
    fn visit_pseudo_class(&mut self, node: &PseudoClass<'a>) {
        selector::walk_pseudo_class(self, node);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_class(&mut self, node: &WebKitScrollbarPseudoClass) {
        selector::walk_web_kit_scrollbar_pseudo_class(self, node);
    }
    #[inline]
    fn visit_pseudo_element(&mut self, node: &PseudoElement<'a>) {
        selector::walk_pseudo_element(self, node);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_element(&mut self, node: &WebKitScrollbarPseudoElement) {
        selector::walk_web_kit_scrollbar_pseudo_element(self, node);
    }
    #[inline]
    fn visit_view_transition_part_name(&mut self, node: &ViewTransitionPartName<'a>) {
        selector::walk_view_transition_part_name(self, node);
    }
    #[inline]
    fn visit_span(&mut self, node: &Span) {
        span::walk_span(self, node);
    }
    #[inline]
    fn visit_token_or_value(&mut self, node: &TokenOrValue<'a>) {
        token::walk_token_or_value(self, node);
    }
    #[inline]
    fn visit_unit(&mut self, node: &Unit) {
        token::walk_unit(self, node);
    }
    #[inline]
    fn visit_token(&mut self, node: &Token<'a>) {
        token::walk_token(self, node);
    }
    #[inline]
    fn visit_specifier(&mut self, node: &Specifier<'a>) {
        token::walk_specifier(self, node);
    }
    #[inline]
    fn visit_animation_name(&mut self, node: &AnimationName<'a>) {
        token::walk_animation_name(self, node);
    }
    #[inline]
    fn visit_environment_variable_name(&mut self, node: &EnvironmentVariableName<'a>) {
        token::walk_environment_variable_name(self, node);
    }
    #[inline]
    fn visit_ua_environment_variable(&mut self, node: &UAEnvironmentVariable) {
        token::walk_ua_environment_variable(self, node);
    }
    #[inline]
    fn visit_image(&mut self, node: &Image<'a>) {
        values::walk_image(self, node);
    }
    #[inline]
    fn visit_gradient(&mut self, node: &Gradient<'a>) {
        values::walk_gradient(self, node);
    }
    #[inline]
    fn visit_web_kit_gradient(&mut self, node: &WebKitGradient<'a>) {
        values::walk_web_kit_gradient(self, node);
    }
    #[inline]
    fn visit_line_direction(&mut self, node: &LineDirection<'a>) {
        values::walk_line_direction(self, node);
    }
    #[inline]
    fn visit_horizontal_position_keyword(&mut self, node: &HorizontalPositionKeyword) {
        values::walk_horizontal_position_keyword(self, node);
    }
    #[inline]
    fn visit_vertical_position_keyword(&mut self, node: &VerticalPositionKeyword) {
        values::walk_vertical_position_keyword(self, node);
    }
    #[inline]
    fn visit_gradient_item<D>(&mut self, node: &GradientItem<'a, D>)
    where
        D: VisitNode<'a, Self>,
    {
        values::walk_gradient_item(self, node);
    }
    #[inline]
    fn visit_dimension_percentage<D>(&mut self, node: &DimensionPercentage<'a, D>)
    where
        D: VisitNode<'a, Self>,
    {
        values::walk_dimension_percentage(self, node);
    }
    #[inline]
    fn visit_position_component<S>(&mut self, node: &PositionComponent<'a, S>)
    where
        S: VisitNode<'a, Self>,
    {
        values::walk_position_component(self, node);
    }
    #[inline]
    fn visit_ending_shape(&mut self, node: &EndingShape<'a>) {
        values::walk_ending_shape(self, node);
    }
    #[inline]
    fn visit_ellipse(&mut self, node: &Ellipse<'a>) {
        values::walk_ellipse(self, node);
    }
    #[inline]
    fn visit_shape_extent(&mut self, node: &ShapeExtent) {
        values::walk_shape_extent(self, node);
    }
    #[inline]
    fn visit_circle(&mut self, node: &Circle<'a>) {
        values::walk_circle(self, node);
    }
    #[inline]
    fn visit_web_kit_gradient_point_component<S>(
        &mut self,
        node: &WebKitGradientPointComponent<'a, S>,
    ) where
        S: VisitNode<'a, Self>,
    {
        values::walk_web_kit_gradient_point_component(self, node);
    }
    #[inline]
    fn visit_number_or_percentage(&mut self, node: &NumberOrPercentage) {
        values::walk_number_or_percentage(self, node);
    }
    #[inline]
    fn visit_background_size(&mut self, node: &BackgroundSize<'a>) {
        values::walk_background_size(self, node);
    }
    #[inline]
    fn visit_length_percentage_or_auto(&mut self, node: &LengthPercentageOrAuto<'a>) {
        values::walk_length_percentage_or_auto(self, node);
    }
    #[inline]
    fn visit_background_repeat_keyword(&mut self, node: &BackgroundRepeatKeyword) {
        values::walk_background_repeat_keyword(self, node);
    }
    #[inline]
    fn visit_background_attachment(&mut self, node: &BackgroundAttachment) {
        values::walk_background_attachment(self, node);
    }
    #[inline]
    fn visit_background_clip(&mut self, node: &BackgroundClip) {
        values::walk_background_clip(self, node);
    }
    #[inline]
    fn visit_background_origin(&mut self, node: &BackgroundOrigin) {
        values::walk_background_origin(self, node);
    }
    #[inline]
    fn visit_display(&mut self, node: &Display<'a>) {
        values::walk_display(self, node);
    }
    #[inline]
    fn visit_display_keyword(&mut self, node: &DisplayKeyword) {
        values::walk_display_keyword(self, node);
    }
    #[inline]
    fn visit_display_inside(&mut self, node: &DisplayInside) {
        values::walk_display_inside(self, node);
    }
    #[inline]
    fn visit_display_outside(&mut self, node: &DisplayOutside) {
        values::walk_display_outside(self, node);
    }
    #[inline]
    fn visit_visibility(&mut self, node: &Visibility) {
        values::walk_visibility(self, node);
    }
    #[inline]
    fn visit_size(&mut self, node: &Size<'a>) {
        values::walk_size(self, node);
    }
    #[inline]
    fn visit_max_size(&mut self, node: &MaxSize<'a>) {
        values::walk_max_size(self, node);
    }
    #[inline]
    fn visit_box_sizing(&mut self, node: &BoxSizing) {
        values::walk_box_sizing(self, node);
    }
    #[inline]
    fn visit_overflow_keyword(&mut self, node: &OverflowKeyword) {
        values::walk_overflow_keyword(self, node);
    }
    #[inline]
    fn visit_text_overflow(&mut self, node: &TextOverflow) {
        values::walk_text_overflow(self, node);
    }
    #[inline]
    fn visit_position_property(&mut self, node: &PositionProperty) {
        values::walk_position_property(self, node);
    }
    #[inline]
    fn visit_size_2_d<T>(&mut self, node: &Size2D<'a, T>)
    where
        T: VisitNode<'a, Self>,
    {
        values::walk_size_2_d(self, node);
    }
    #[inline]
    fn visit_rect<T>(&mut self, node: &Rect<'a, T>)
    where
        T: VisitNode<'a, Self>,
    {
        values::walk_rect(self, node);
    }
    #[inline]
    fn visit_line_style(&mut self, node: &LineStyle) {
        values::walk_line_style(self, node);
    }
    #[inline]
    fn visit_border_side_width(&mut self, node: &BorderSideWidth<'a>) {
        values::walk_border_side_width(self, node);
    }
    #[inline]
    fn visit_length_or_number(&mut self, node: &LengthOrNumber<'a>) {
        values::walk_length_or_number(self, node);
    }
    #[inline]
    fn visit_border_image_repeat_keyword(&mut self, node: &BorderImageRepeatKeyword) {
        values::walk_border_image_repeat_keyword(self, node);
    }
    #[inline]
    fn visit_border_image_side_width(&mut self, node: &BorderImageSideWidth<'a>) {
        values::walk_border_image_side_width(self, node);
    }
    #[inline]
    fn visit_outline_style(&mut self, node: &OutlineStyle) {
        values::walk_outline_style(self, node);
    }
    #[inline]
    fn visit_flex_direction(&mut self, node: &FlexDirection) {
        values::walk_flex_direction(self, node);
    }
    #[inline]
    fn visit_flex_wrap(&mut self, node: &FlexWrap) {
        values::walk_flex_wrap(self, node);
    }
    #[inline]
    fn visit_align_content(&mut self, node: &AlignContent) {
        values::walk_align_content(self, node);
    }
    #[inline]
    fn visit_baseline_position(&mut self, node: &BaselinePosition) {
        values::walk_baseline_position(self, node);
    }
    #[inline]
    fn visit_content_distribution(&mut self, node: &ContentDistribution) {
        values::walk_content_distribution(self, node);
    }
    #[inline]
    fn visit_overflow_position(&mut self, node: &OverflowPosition) {
        values::walk_overflow_position(self, node);
    }
    #[inline]
    fn visit_content_position(&mut self, node: &ContentPosition) {
        values::walk_content_position(self, node);
    }
    #[inline]
    fn visit_justify_content(&mut self, node: &JustifyContent) {
        values::walk_justify_content(self, node);
    }
    #[inline]
    fn visit_align_self(&mut self, node: &AlignSelf) {
        values::walk_align_self(self, node);
    }
    #[inline]
    fn visit_self_position(&mut self, node: &SelfPosition) {
        values::walk_self_position(self, node);
    }
    #[inline]
    fn visit_justify_self(&mut self, node: &JustifySelf) {
        values::walk_justify_self(self, node);
    }
    #[inline]
    fn visit_align_items(&mut self, node: &AlignItems) {
        values::walk_align_items(self, node);
    }
    #[inline]
    fn visit_justify_items(&mut self, node: &JustifyItems) {
        values::walk_justify_items(self, node);
    }
    #[inline]
    fn visit_legacy_justify(&mut self, node: &LegacyJustify) {
        values::walk_legacy_justify(self, node);
    }
    #[inline]
    fn visit_gap_value(&mut self, node: &GapValue<'a>) {
        values::walk_gap_value(self, node);
    }
    #[inline]
    fn visit_box_orient(&mut self, node: &BoxOrient) {
        values::walk_box_orient(self, node);
    }
    #[inline]
    fn visit_box_direction(&mut self, node: &BoxDirection) {
        values::walk_box_direction(self, node);
    }
    #[inline]
    fn visit_box_align(&mut self, node: &BoxAlign) {
        values::walk_box_align(self, node);
    }
    #[inline]
    fn visit_box_pack(&mut self, node: &BoxPack) {
        values::walk_box_pack(self, node);
    }
    #[inline]
    fn visit_box_lines(&mut self, node: &BoxLines) {
        values::walk_box_lines(self, node);
    }
    #[inline]
    fn visit_flex_pack(&mut self, node: &FlexPack) {
        values::walk_flex_pack(self, node);
    }
    #[inline]
    fn visit_flex_item_align(&mut self, node: &FlexItemAlign) {
        values::walk_flex_item_align(self, node);
    }
    #[inline]
    fn visit_flex_line_pack(&mut self, node: &FlexLinePack) {
        values::walk_flex_line_pack(self, node);
    }
    #[inline]
    fn visit_track_sizing(&mut self, node: &TrackSizing<'a>) {
        values::walk_track_sizing(self, node);
    }
    #[inline]
    fn visit_track_list_item(&mut self, node: &TrackListItem<'a>) {
        values::walk_track_list_item(self, node);
    }
    #[inline]
    fn visit_track_size(&mut self, node: &TrackSize<'a>) {
        values::walk_track_size(self, node);
    }
    #[inline]
    fn visit_track_breadth(&mut self, node: &TrackBreadth<'a>) {
        values::walk_track_breadth(self, node);
    }
    #[inline]
    fn visit_repeat_count(&mut self, node: &RepeatCount) {
        values::walk_repeat_count(self, node);
    }
    #[inline]
    fn visit_auto_flow_direction(&mut self, node: &AutoFlowDirection) {
        values::walk_auto_flow_direction(self, node);
    }
    #[inline]
    fn visit_grid_template_areas(&mut self, node: &GridTemplateAreas<'a>) {
        values::walk_grid_template_areas(self, node);
    }
    #[inline]
    fn visit_grid_line(&mut self, node: &GridLine<'a>) {
        values::walk_grid_line(self, node);
    }
    #[inline]
    fn visit_font_weight(&mut self, node: &FontWeight<'a>) {
        values::walk_font_weight(self, node);
    }
    #[inline]
    fn visit_absolute_font_weight(&mut self, node: &AbsoluteFontWeight) {
        values::walk_absolute_font_weight(self, node);
    }
    #[inline]
    fn visit_font_size(&mut self, node: &FontSize<'a>) {
        values::walk_font_size(self, node);
    }
    #[inline]
    fn visit_absolute_font_size(&mut self, node: &AbsoluteFontSize) {
        values::walk_absolute_font_size(self, node);
    }
    #[inline]
    fn visit_relative_font_size(&mut self, node: &RelativeFontSize) {
        values::walk_relative_font_size(self, node);
    }
    #[inline]
    fn visit_font_stretch(&mut self, node: &FontStretch) {
        values::walk_font_stretch(self, node);
    }
    #[inline]
    fn visit_font_stretch_keyword(&mut self, node: &FontStretchKeyword) {
        values::walk_font_stretch_keyword(self, node);
    }
    #[inline]
    fn visit_font_family(&mut self, node: &FontFamily<'a>) {
        values::walk_font_family(self, node);
    }
    #[inline]
    fn visit_generic_font_family(&mut self, node: &GenericFontFamily) {
        values::walk_generic_font_family(self, node);
    }
    #[inline]
    fn visit_font_style(&mut self, node: &FontStyle<'a>) {
        values::walk_font_style(self, node);
    }
    #[inline]
    fn visit_font_variant_caps(&mut self, node: &FontVariantCaps) {
        values::walk_font_variant_caps(self, node);
    }
    #[inline]
    fn visit_line_height(&mut self, node: &LineHeight<'a>) {
        values::walk_line_height(self, node);
    }
    #[inline]
    fn visit_vertical_align(&mut self, node: &VerticalAlign<'a>) {
        values::walk_vertical_align(self, node);
    }
    #[inline]
    fn visit_vertical_align_keyword(&mut self, node: &VerticalAlignKeyword) {
        values::walk_vertical_align_keyword(self, node);
    }
    #[inline]
    fn visit_easing_function(&mut self, node: &EasingFunction) {
        values::walk_easing_function(self, node);
    }
    #[inline]
    fn visit_step_position(&mut self, node: &StepPosition) {
        values::walk_step_position(self, node);
    }
    #[inline]
    fn visit_animation_iteration_count(&mut self, node: &AnimationIterationCount) {
        values::walk_animation_iteration_count(self, node);
    }
    #[inline]
    fn visit_animation_direction(&mut self, node: &AnimationDirection) {
        values::walk_animation_direction(self, node);
    }
    #[inline]
    fn visit_animation_play_state(&mut self, node: &AnimationPlayState) {
        values::walk_animation_play_state(self, node);
    }
    #[inline]
    fn visit_animation_fill_mode(&mut self, node: &AnimationFillMode) {
        values::walk_animation_fill_mode(self, node);
    }
    #[inline]
    fn visit_animation_composition(&mut self, node: &AnimationComposition) {
        values::walk_animation_composition(self, node);
    }
    #[inline]
    fn visit_animation_timeline(&mut self, node: &AnimationTimeline<'a>) {
        values::walk_animation_timeline(self, node);
    }
    #[inline]
    fn visit_scroll_axis(&mut self, node: &ScrollAxis) {
        values::walk_scroll_axis(self, node);
    }
    #[inline]
    fn visit_scroller(&mut self, node: &Scroller) {
        values::walk_scroller(self, node);
    }
    #[inline]
    fn visit_animation_attachment_range(&mut self, node: &AnimationAttachmentRange<'a>) {
        values::walk_animation_attachment_range(self, node);
    }
    #[inline]
    fn visit_timeline_range_name(&mut self, node: &TimelineRangeName) {
        values::walk_timeline_range_name(self, node);
    }
    #[inline]
    fn visit_transform(&mut self, node: &Transform<'a>) {
        values::walk_transform(self, node);
    }
    #[inline]
    fn visit_transform_style(&mut self, node: &TransformStyle) {
        values::walk_transform_style(self, node);
    }
    #[inline]
    fn visit_transform_box(&mut self, node: &TransformBox) {
        values::walk_transform_box(self, node);
    }
    #[inline]
    fn visit_backface_visibility(&mut self, node: &BackfaceVisibility) {
        values::walk_backface_visibility(self, node);
    }
    #[inline]
    fn visit_perspective(&mut self, node: &Perspective<'a>) {
        values::walk_perspective(self, node);
    }
    #[inline]
    fn visit_translate(&mut self, node: &Translate<'a>) {
        values::walk_translate(self, node);
    }
    #[inline]
    fn visit_scale(&mut self, node: &Scale<'a>) {
        values::walk_scale(self, node);
    }
    #[inline]
    fn visit_text_transform_case(&mut self, node: &TextTransformCase) {
        values::walk_text_transform_case(self, node);
    }
    #[inline]
    fn visit_white_space(&mut self, node: &WhiteSpace) {
        values::walk_white_space(self, node);
    }
    #[inline]
    fn visit_word_break(&mut self, node: &WordBreak) {
        values::walk_word_break(self, node);
    }
    #[inline]
    fn visit_line_break(&mut self, node: &LineBreak) {
        values::walk_line_break(self, node);
    }
    #[inline]
    fn visit_hyphens(&mut self, node: &Hyphens) {
        values::walk_hyphens(self, node);
    }
    #[inline]
    fn visit_overflow_wrap(&mut self, node: &OverflowWrap) {
        values::walk_overflow_wrap(self, node);
    }
    #[inline]
    fn visit_text_align(&mut self, node: &TextAlign) {
        values::walk_text_align(self, node);
    }
    #[inline]
    fn visit_text_align_last(&mut self, node: &TextAlignLast) {
        values::walk_text_align_last(self, node);
    }
    #[inline]
    fn visit_text_justify(&mut self, node: &TextJustify) {
        values::walk_text_justify(self, node);
    }
    #[inline]
    fn visit_spacing(&mut self, node: &Spacing<'a>) {
        values::walk_spacing(self, node);
    }
    #[inline]
    fn visit_text_decoration_line(&mut self, node: &TextDecorationLine<'a>) {
        values::walk_text_decoration_line(self, node);
    }
    #[inline]
    fn visit_exclusive_text_decoration_line(&mut self, node: &ExclusiveTextDecorationLine) {
        values::walk_exclusive_text_decoration_line(self, node);
    }
    #[inline]
    fn visit_other_text_decoration_line(&mut self, node: &OtherTextDecorationLine) {
        values::walk_other_text_decoration_line(self, node);
    }
    #[inline]
    fn visit_text_decoration_style(&mut self, node: &TextDecorationStyle) {
        values::walk_text_decoration_style(self, node);
    }
    #[inline]
    fn visit_text_decoration_thickness(&mut self, node: &TextDecorationThickness<'a>) {
        values::walk_text_decoration_thickness(self, node);
    }
    #[inline]
    fn visit_text_decoration_skip_ink(&mut self, node: &TextDecorationSkipInk) {
        values::walk_text_decoration_skip_ink(self, node);
    }
    #[inline]
    fn visit_text_emphasis_style(&mut self, node: &TextEmphasisStyle<'a>) {
        values::walk_text_emphasis_style(self, node);
    }
    #[inline]
    fn visit_text_emphasis_fill_mode(&mut self, node: &TextEmphasisFillMode) {
        values::walk_text_emphasis_fill_mode(self, node);
    }
    #[inline]
    fn visit_text_emphasis_shape(&mut self, node: &TextEmphasisShape) {
        values::walk_text_emphasis_shape(self, node);
    }
    #[inline]
    fn visit_text_emphasis_position_horizontal(&mut self, node: &TextEmphasisPositionHorizontal) {
        values::walk_text_emphasis_position_horizontal(self, node);
    }
    #[inline]
    fn visit_text_emphasis_position_vertical(&mut self, node: &TextEmphasisPositionVertical) {
        values::walk_text_emphasis_position_vertical(self, node);
    }
    #[inline]
    fn visit_text_size_adjust(&mut self, node: &TextSizeAdjust) {
        values::walk_text_size_adjust(self, node);
    }
    #[inline]
    fn visit_text_direction(&mut self, node: &TextDirection) {
        values::walk_text_direction(self, node);
    }
    #[inline]
    fn visit_unicode_bidi(&mut self, node: &UnicodeBidi) {
        values::walk_unicode_bidi(self, node);
    }
    #[inline]
    fn visit_box_decoration_break(&mut self, node: &BoxDecorationBreak) {
        values::walk_box_decoration_break(self, node);
    }
    #[inline]
    fn visit_resize(&mut self, node: &Resize) {
        values::walk_resize(self, node);
    }
    #[inline]
    fn visit_cursor_keyword(&mut self, node: &CursorKeyword) {
        values::walk_cursor_keyword(self, node);
    }
    #[inline]
    fn visit_color_or_auto(&mut self, node: &ColorOrAuto<'a>) {
        values::walk_color_or_auto(self, node);
    }
    #[inline]
    fn visit_caret_shape(&mut self, node: &CaretShape) {
        values::walk_caret_shape(self, node);
    }
    #[inline]
    fn visit_user_select(&mut self, node: &UserSelect) {
        values::walk_user_select(self, node);
    }
    #[inline]
    fn visit_appearance(&mut self, node: &Appearance<'a>) {
        values::walk_appearance(self, node);
    }
    #[inline]
    fn visit_list_style_type(&mut self, node: &ListStyleType<'a>) {
        values::walk_list_style_type(self, node);
    }
    #[inline]
    fn visit_counter_style(&mut self, node: &CounterStyle<'a>) {
        values::walk_counter_style(self, node);
    }
    #[inline]
    fn visit_symbols_type(&mut self, node: &SymbolsType) {
        values::walk_symbols_type(self, node);
    }
    #[inline]
    fn visit_predefined_counter_style(&mut self, node: &PredefinedCounterStyle) {
        values::walk_predefined_counter_style(self, node);
    }
    #[inline]
    fn visit_symbol(&mut self, node: &Symbol<'a>) {
        values::walk_symbol(self, node);
    }
    #[inline]
    fn visit_list_style_position(&mut self, node: &ListStylePosition) {
        values::walk_list_style_position(self, node);
    }
    #[inline]
    fn visit_marker_side(&mut self, node: &MarkerSide) {
        values::walk_marker_side(self, node);
    }
    #[inline]
    fn visit_svg_paint(&mut self, node: &SVGPaint<'a>) {
        values::walk_svg_paint(self, node);
    }
    #[inline]
    fn visit_svg_paint_fallback(&mut self, node: &SVGPaintFallback<'a>) {
        values::walk_svg_paint_fallback(self, node);
    }
    #[inline]
    fn visit_fill_rule(&mut self, node: &FillRule) {
        values::walk_fill_rule(self, node);
    }
    #[inline]
    fn visit_stroke_linecap(&mut self, node: &StrokeLinecap) {
        values::walk_stroke_linecap(self, node);
    }
    #[inline]
    fn visit_stroke_linejoin(&mut self, node: &StrokeLinejoin) {
        values::walk_stroke_linejoin(self, node);
    }
    #[inline]
    fn visit_stroke_dasharray(&mut self, node: &StrokeDasharray<'a>) {
        values::walk_stroke_dasharray(self, node);
    }
    #[inline]
    fn visit_marker(&mut self, node: &Marker<'a>) {
        values::walk_marker(self, node);
    }
    #[inline]
    fn visit_color_interpolation(&mut self, node: &ColorInterpolation) {
        values::walk_color_interpolation(self, node);
    }
    #[inline]
    fn visit_color_rendering(&mut self, node: &ColorRendering) {
        values::walk_color_rendering(self, node);
    }
    #[inline]
    fn visit_shape_rendering(&mut self, node: &ShapeRendering) {
        values::walk_shape_rendering(self, node);
    }
    #[inline]
    fn visit_text_rendering(&mut self, node: &TextRendering) {
        values::walk_text_rendering(self, node);
    }
    #[inline]
    fn visit_image_rendering(&mut self, node: &ImageRendering) {
        values::walk_image_rendering(self, node);
    }
    #[inline]
    fn visit_clip_path(&mut self, node: &ClipPath<'a>) {
        values::walk_clip_path(self, node);
    }
    #[inline]
    fn visit_geometry_box(&mut self, node: &GeometryBox) {
        values::walk_geometry_box(self, node);
    }
    #[inline]
    fn visit_basic_shape(&mut self, node: &BasicShape<'a>) {
        values::walk_basic_shape(self, node);
    }
    #[inline]
    fn visit_shape_radius(&mut self, node: &ShapeRadius<'a>) {
        values::walk_shape_radius(self, node);
    }
    #[inline]
    fn visit_mask_mode(&mut self, node: &MaskMode) {
        values::walk_mask_mode(self, node);
    }
    #[inline]
    fn visit_mask_clip(&mut self, node: &MaskClip) {
        values::walk_mask_clip(self, node);
    }
    #[inline]
    fn visit_mask_composite(&mut self, node: &MaskComposite) {
        values::walk_mask_composite(self, node);
    }
    #[inline]
    fn visit_mask_type(&mut self, node: &MaskType) {
        values::walk_mask_type(self, node);
    }
    #[inline]
    fn visit_mask_border_mode(&mut self, node: &MaskBorderMode) {
        values::walk_mask_border_mode(self, node);
    }
    #[inline]
    fn visit_web_kit_mask_composite(&mut self, node: &WebKitMaskComposite) {
        values::walk_web_kit_mask_composite(self, node);
    }
    #[inline]
    fn visit_web_kit_mask_source_type(&mut self, node: &WebKitMaskSourceType) {
        values::walk_web_kit_mask_source_type(self, node);
    }
    #[inline]
    fn visit_filter_list(&mut self, node: &FilterList<'a>) {
        values::walk_filter_list(self, node);
    }
    #[inline]
    fn visit_filter(&mut self, node: &Filter<'a>) {
        values::walk_filter(self, node);
    }
    #[inline]
    fn visit_z_index(&mut self, node: &ZIndex) {
        values::walk_z_index(self, node);
    }
    #[inline]
    fn visit_container_type(&mut self, node: &ContainerType) {
        values::walk_container_type(self, node);
    }
    #[inline]
    fn visit_container_name_list(&mut self, node: &ContainerNameList<'a>) {
        values::walk_container_name_list(self, node);
    }
    #[inline]
    fn visit_view_transition_name(&mut self, node: &ViewTransitionName<'a>) {
        values::walk_view_transition_name(self, node);
    }
    #[inline]
    fn visit_none_or_custom_ident_list(&mut self, node: &NoneOrCustomIdentList<'a>) {
        values::walk_none_or_custom_ident_list(self, node);
    }
    #[inline]
    fn visit_view_transition_group(&mut self, node: &ViewTransitionGroup<'a>) {
        values::walk_view_transition_group(self, node);
    }
    #[inline]
    fn visit_print_color_adjust(&mut self, node: &PrintColorAdjust) {
        values::walk_print_color_adjust(self, node);
    }
    #[inline]
    fn visit_css_wide_keyword(&mut self, node: &CSSWideKeyword) {
        values::walk_css_wide_keyword(self, node);
    }
    #[inline]
    fn visit_custom_property_name(&mut self, node: &CustomPropertyName<'a>) {
        values::walk_custom_property_name(self, node);
    }
    #[inline]
    fn visit_media_feature(&mut self, node: &MediaFeature<'a>) {
        media::walk_media_feature(self, node);
    }
    #[inline]
    fn visit_container_size_feature(&mut self, node: &ContainerSizeFeature<'a>) {
        rules::walk_container_size_feature(self, node);
    }
    #[inline]
    fn visit_scroll_state_feature(&mut self, node: &ScrollStateFeature<'a>) {
        rules::walk_scroll_state_feature(self, node);
    }
    #[inline]
    fn visit_selector_list(&mut self, node: &SelectorList<'a>) {
        selector::walk_selector_list(self, node);
    }
    #[inline]
    fn visit_selector(&mut self, node: &Selector<'a>) {
        selector::walk_selector(self, node);
    }
    #[inline]
    fn visit_length_percentage(&mut self, node: &LengthPercentage<'a>) {
        values::walk_length_percentage(self, node);
    }
    #[inline]
    fn visit_angle_percentage(&mut self, node: &AnglePercentage<'a>) {
        values::walk_angle_percentage(self, node);
    }
    #[inline]
    fn visit_animation_range_start(&mut self, node: &AnimationRangeStart<'a>) {
        values::walk_animation_range_start(self, node);
    }
    #[inline]
    fn visit_animation_range_end(&mut self, node: &AnimationRangeEnd<'a>) {
        values::walk_animation_range_end(self, node);
    }
    #[inline]
    fn visit_declaration(&mut self, node: &Declaration<'a>) {
        walk_declaration(self, node);
    }
    #[inline]
    fn visit_property_id(&mut self, node: &PropertyId<'a>) {
        walk_property_id(self, node);
    }
    #[inline]
    fn visit_vendor_prefix(&mut self, node: &VendorPrefix) {
        walk_vendor_prefix(self, node);
    }
}
#[doc(hidden)]
pub trait VisitNode<'a, VisitorT: ?Sized + Visit<'a>> {
    fn visit_node(&self, visitor: &mut VisitorT);
}
macro_rules! impl_leaf_visit_node {
    ($($ty:ty),+ $(,)?) => {
        $(impl < 'a, VisitorT : ? Sized + Visit < 'a >> VisitNode < 'a, VisitorT > for
        $ty { fn visit_node(& self, _visitor : & mut VisitorT) {} })+
    };
}
impl_leaf_visit_node!(bool, char, f32, i32, u8, u16, u32, usize);
impl<'a, VisitorT, T> VisitNode<'a, VisitorT> for rocketcss_allocator::boxed::Box<'a, T>
where
    VisitorT: ?Sized + Visit<'a>,
    T: ?Sized + VisitNode<'a, VisitorT>,
{
    fn visit_node(&self, visitor: &mut VisitorT) {
        self.as_ref().visit_node(visitor);
    }
}
impl<'a, VisitorT, T> VisitNode<'a, VisitorT> for rocketcss_allocator::vec::Vec<'a, T>
where
    VisitorT: ?Sized + Visit<'a>,
    T: VisitNode<'a, VisitorT>,
{
    fn visit_node(&self, visitor: &mut VisitorT) {
        for value in self {
            value.visit_node(visitor);
        }
    }
}
impl<'a, VisitorT, T> VisitNode<'a, VisitorT> for Option<T>
where
    VisitorT: ?Sized + Visit<'a>,
    T: VisitNode<'a, VisitorT>,
{
    fn visit_node(&self, visitor: &mut VisitorT) {
        if let Some(value) = self {
            value.visit_node(visitor);
        }
    }
}
impl<'a, VisitorT: ?Sized + Visit<'a>> VisitNode<'a, VisitorT> for &'a str {
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_str(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CssColor<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_css_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for RGBA
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_rgba(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LABColor
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_lab_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PredefinedColor
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_predefined_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FloatColor
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_float_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LightDark<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_light_dark(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SystemColor
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_system_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for UnresolvedColor<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_unresolved_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CssRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_css_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Length<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_length(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LengthUnit
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_length_unit(self);
    }
}
impl<'a, V, VisitorT> VisitNode<'a, VisitorT> for Calc<'a, V>
where
    VisitorT: ?Sized + Visit<'a>,
    V: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_calc(self);
    }
}
impl<'a, V, VisitorT> VisitNode<'a, VisitorT> for MathFunction<'a, V>
where
    VisitorT: ?Sized + Visit<'a>,
    V: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_math_function(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for RoundingStrategy
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_rounding_strategy(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Resolution
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_resolution(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Ratio
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_ratio(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Angle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_angle(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Time
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_time(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MediaCondition<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_condition(self);
    }
}
impl<'a, FeatureId, VisitorT> VisitNode<'a, VisitorT> for QueryFeature<'a, FeatureId>
where
    VisitorT: ?Sized + Visit<'a>,
    FeatureId: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_query_feature(self);
    }
}
impl<'a, FeatureId, VisitorT> VisitNode<'a, VisitorT> for MediaFeatureName<'a, FeatureId>
where
    VisitorT: ?Sized + Visit<'a>,
    FeatureId: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_name(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MediaFeatureId
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_id(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MediaFeatureValue<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_value(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MediaFeatureComparison
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_comparison(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Operator
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_operator(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MediaType<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_type(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Qualifier
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_qualifier(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SupportsCondition<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_supports_condition(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BlendMode
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_blend_mode(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for KeyframeSelector<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_keyframe_selector(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for KeyframesName<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_keyframes_name(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFaceProperty<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_face_property(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Source<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_source(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFormat<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_format(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontTechnology
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_technology(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFaceStyle<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_face_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontPaletteValuesProperty<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_palette_values_property(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BasePalette
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_base_palette(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFeatureSubruleType
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_subrule_type(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PageMarginBox
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_page_margin_box(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PagePseudoClass
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_page_pseudo_class(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ParsedComponent<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_parsed_component(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Multiplier
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_multiplier(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SyntaxString<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_syntax_string(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SyntaxComponentKind<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_syntax_component_kind(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ContainerCondition<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_container_condition(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ContainerSizeFeatureId
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_container_size_feature_id(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for StyleQuery<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_style_query(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollStateQuery<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_state_query(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollStateFeatureId
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_state_feature_id(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ViewTransitionProperty<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_property(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Navigation
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_navigation(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for DefaultAtRule
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_default_at_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for StyleSheet<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_style_sheet(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MediaRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MediaList<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_list(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MediaQuery<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_media_query(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LengthValue
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_length_value(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for EnvironmentVariable<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_environment_variable(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Url<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_url(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Variable<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_variable(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for DashedIdentReference<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_dashed_ident_reference(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Function<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_function(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FunctionReplacement
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_function_replacement(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ImportRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_import_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for StyleRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_style_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for DeclarationBlock<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_declaration_block(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Position<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WebKitGradientPoint<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient_point(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WebKitColorStop<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_color_stop(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ImageSet<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_image_set(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ImageSetOption<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_image_set_option(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BackgroundPosition<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_background_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BackgroundRepeat
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_background_repeat(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Background<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_background(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BoxShadow<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_box_shadow(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AspectRatio<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_aspect_ratio(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Overflow
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_overflow(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for InsetBlock<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_inset_block(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for InsetInline<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_inset_inline(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Inset<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_inset(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderRadius<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_radius(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderImageRepeat
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image_repeat(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderImageSlice<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image_slice(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderImage<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderColor<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderStyle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderWidth<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_width(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderBlockColor<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_block_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderBlockStyle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_block_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderBlockWidth<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_block_width(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderInlineColor<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_color(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderInlineStyle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderInlineWidth<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_width(self);
    }
}
impl<'a, S, VisitorT> VisitNode<'a, VisitorT> for GenericBorder<'a, S>
where
    VisitorT: ?Sized + Visit<'a>,
    S: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_generic_border(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FlexFlow
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_flow(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Flex<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_flex(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PlaceContent<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_place_content(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PlaceSelf<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_place_self(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PlaceItems<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_place_items(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Gap<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_gap(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TrackRepeat<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_track_repeat(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GridAutoFlow
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_auto_flow(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GridTemplate<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_template(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Grid<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_grid(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GridRow<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_row(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GridColumn<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_column(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GridArea<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_area(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MarginBlock<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_margin_block(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MarginInline<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_margin_inline(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Margin<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_margin(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PaddingBlock<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_padding_block(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PaddingInline<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_padding_inline(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Padding<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_padding(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollMarginBlock<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin_block(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollMarginInline<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin_inline(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollMargin<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollPaddingBlock<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding_block(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollPaddingInline<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding_inline(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollPadding<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Font<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Transition<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_transition(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollTimeline
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_timeline(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ViewTimeline<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_view_timeline(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationRange<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_range(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Animation<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MatrixForFloat
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_matrix_for_float(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Matrix3DForFloat
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_matrix_3_d_for_float(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Rotate<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_rotate(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextTransform
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_transform(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextIndent<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_indent(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextDecoration<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextEmphasis<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextEmphasisPosition
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextShadow<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_shadow(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Cursor<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_cursor(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CursorImage<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_cursor_image(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Caret<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_caret(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ListStyle<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_list_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Composes<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_composes(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for InsetRect<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_inset_rect(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CircleShape<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_circle_shape(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for EllipseShape<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_ellipse_shape(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Polygon<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_polygon(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Point<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_point(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Mask<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_mask(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MaskBorder<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_border(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for DropShadow<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_drop_shadow(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Container<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_container(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ColorScheme
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_color_scheme(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for UnparsedProperty<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_unparsed_property(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CustomProperty<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_custom_property(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ViewTransitionPartSelector<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_part_selector(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for KeyframesRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_keyframes_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Keyframe<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_keyframe(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TimelineRangePercentage
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_timeline_range_percentage(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFaceRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_face_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for UrlSource<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_url_source(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for UnicodeRange
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_unicode_range(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontPaletteValuesRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_palette_values_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for OverrideColors<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_override_colors(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFeatureValuesRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_values_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFeatureSubrule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_subrule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFeatureDeclaration<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_declaration(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FamilyName<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_family_name(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PageRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_page_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PageMarginRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_page_margin_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PageSelector<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_page_selector(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SupportsRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_supports_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CounterStyleRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_counter_style_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for NamespaceRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_namespace_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MozDocumentRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_moz_document_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for NestingRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_nesting_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for NestedDeclarationsRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_nested_declarations_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ViewportRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_viewport_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CustomMediaRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_custom_media_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LayerStatementRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_layer_statement_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LayerBlockRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_layer_block_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PropertyRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_property_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SyntaxComponent<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_syntax_component(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ContainerRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_container_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScopeRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scope_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for StartingStyleRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_starting_style_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ViewTransitionRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PositionTryRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_position_try_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for UnknownAtRule<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_unknown_at_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SelectorComponent<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_selector_component(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Combinator
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_combinator(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AttrSelector<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_attr_selector(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for NamespaceConstraint<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_namespace_constraint(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AttrOperation<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_attr_operation(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ParsedCaseSensitivity
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_parsed_case_sensitivity(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AttrSelectorOperator
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_attr_selector_operator(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for NthType
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_nth_type(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for NthSelectorData
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_nth_selector_data(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Direction
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_direction(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PseudoClass<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_pseudo_class(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WebKitScrollbarPseudoClass
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_scrollbar_pseudo_class(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PseudoElement<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_pseudo_element(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WebKitScrollbarPseudoElement
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_scrollbar_pseudo_element(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ViewTransitionPartName<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_part_name(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Span
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_span(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TokenOrValue<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_token_or_value(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Unit
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_unit(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Token<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_token(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Specifier<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_specifier(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationName<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_name(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for EnvironmentVariableName<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_environment_variable_name(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for UAEnvironmentVariable
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_ua_environment_variable(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Image<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_image(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Gradient<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_gradient(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WebKitGradient<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LineDirection<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_line_direction(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for HorizontalPositionKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_horizontal_position_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for VerticalPositionKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_vertical_position_keyword(self);
    }
}
impl<'a, D, VisitorT> VisitNode<'a, VisitorT> for GradientItem<'a, D>
where
    VisitorT: ?Sized + Visit<'a>,
    D: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_gradient_item(self);
    }
}
impl<'a, D, VisitorT> VisitNode<'a, VisitorT> for DimensionPercentage<'a, D>
where
    VisitorT: ?Sized + Visit<'a>,
    D: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_dimension_percentage(self);
    }
}
impl<'a, S, VisitorT> VisitNode<'a, VisitorT> for PositionComponent<'a, S>
where
    VisitorT: ?Sized + Visit<'a>,
    S: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_position_component(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for EndingShape<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_ending_shape(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Ellipse<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_ellipse(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ShapeExtent
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_shape_extent(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Circle<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_circle(self);
    }
}
impl<'a, S, VisitorT> VisitNode<'a, VisitorT> for WebKitGradientPointComponent<'a, S>
where
    VisitorT: ?Sized + Visit<'a>,
    S: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient_point_component(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for NumberOrPercentage
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_number_or_percentage(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BackgroundSize<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_background_size(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LengthPercentageOrAuto<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_length_percentage_or_auto(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BackgroundRepeatKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_background_repeat_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BackgroundAttachment
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_background_attachment(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BackgroundClip
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_background_clip(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BackgroundOrigin
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_background_origin(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Display<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_display(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for DisplayKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_display_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for DisplayInside
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_display_inside(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for DisplayOutside
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_display_outside(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Visibility
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_visibility(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Size<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_size(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MaxSize<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_max_size(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BoxSizing
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_box_sizing(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for OverflowKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_overflow_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextOverflow
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_overflow(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PositionProperty
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_position_property(self);
    }
}
impl<'a, T, VisitorT> VisitNode<'a, VisitorT> for Size2D<'a, T>
where
    VisitorT: ?Sized + Visit<'a>,
    T: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_size_2_d(self);
    }
}
impl<'a, T, VisitorT> VisitNode<'a, VisitorT> for Rect<'a, T>
where
    VisitorT: ?Sized + Visit<'a>,
    T: VisitNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_rect(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LineStyle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_line_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderSideWidth<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_side_width(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LengthOrNumber<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_length_or_number(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderImageRepeatKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image_repeat_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BorderImageSideWidth<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_border_image_side_width(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for OutlineStyle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_outline_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FlexDirection
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_direction(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FlexWrap
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_wrap(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AlignContent
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_align_content(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BaselinePosition
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_baseline_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ContentDistribution
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_content_distribution(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for OverflowPosition
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_overflow_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ContentPosition
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_content_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for JustifyContent
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_justify_content(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AlignSelf
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_align_self(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SelfPosition
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_self_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for JustifySelf
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_justify_self(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AlignItems
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_align_items(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for JustifyItems
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_justify_items(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LegacyJustify
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_legacy_justify(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GapValue<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_gap_value(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BoxOrient
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_box_orient(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BoxDirection
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_box_direction(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BoxAlign
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_box_align(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BoxPack
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_box_pack(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BoxLines
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_box_lines(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FlexPack
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_pack(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FlexItemAlign
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_item_align(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FlexLinePack
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_flex_line_pack(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TrackSizing<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_track_sizing(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TrackListItem<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_track_list_item(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TrackSize<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_track_size(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TrackBreadth<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_track_breadth(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for RepeatCount
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_repeat_count(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AutoFlowDirection
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_auto_flow_direction(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GridTemplateAreas<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_template_areas(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GridLine<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_grid_line(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontWeight<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_weight(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AbsoluteFontWeight
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_absolute_font_weight(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontSize<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_size(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AbsoluteFontSize
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_absolute_font_size(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for RelativeFontSize
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_relative_font_size(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontStretch
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_stretch(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontStretchKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_stretch_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontFamily<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_family(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GenericFontFamily
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_generic_font_family(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontStyle<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FontVariantCaps
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_font_variant_caps(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LineHeight<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_line_height(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for VerticalAlign<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_vertical_align(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for VerticalAlignKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_vertical_align_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for EasingFunction
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_easing_function(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for StepPosition
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_step_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationIterationCount
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_iteration_count(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationDirection
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_direction(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationPlayState
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_play_state(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationFillMode
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_fill_mode(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationComposition
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_composition(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationTimeline<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_timeline(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ScrollAxis
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroll_axis(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Scroller
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scroller(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for AnimationAttachmentRange<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_animation_attachment_range(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TimelineRangeName
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_timeline_range_name(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Transform<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_transform(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TransformStyle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_transform_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TransformBox
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_transform_box(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BackfaceVisibility
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_backface_visibility(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Perspective<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_perspective(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Translate<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_translate(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Scale<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_scale(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextTransformCase
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_transform_case(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WhiteSpace
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_white_space(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WordBreak
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_word_break(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for LineBreak
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_line_break(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Hyphens
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_hyphens(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for OverflowWrap
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_overflow_wrap(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextAlign
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_align(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextAlignLast
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_align_last(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextJustify
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_justify(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Spacing<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_spacing(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextDecorationLine<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_line(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ExclusiveTextDecorationLine
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_exclusive_text_decoration_line(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for OtherTextDecorationLine
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_other_text_decoration_line(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextDecorationStyle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextDecorationThickness<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_thickness(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextDecorationSkipInk
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_skip_ink(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextEmphasisStyle<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextEmphasisFillMode
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_fill_mode(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextEmphasisShape
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_shape(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextEmphasisPositionHorizontal
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position_horizontal(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextEmphasisPositionVertical
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position_vertical(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextSizeAdjust
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_size_adjust(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextDirection
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_direction(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for UnicodeBidi
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_unicode_bidi(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BoxDecorationBreak
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_box_decoration_break(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Resize
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_resize(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CursorKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_cursor_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ColorOrAuto<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_color_or_auto(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CaretShape
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_caret_shape(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for UserSelect
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_user_select(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Appearance<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_appearance(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ListStyleType<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_list_style_type(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CounterStyle<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_counter_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SymbolsType
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_symbols_type(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PredefinedCounterStyle
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_predefined_counter_style(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Symbol<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_symbol(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ListStylePosition
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_list_style_position(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MarkerSide
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_marker_side(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SVGPaint<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_svg_paint(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for SVGPaintFallback<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_svg_paint_fallback(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FillRule
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_fill_rule(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for StrokeLinecap
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_stroke_linecap(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for StrokeLinejoin
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_stroke_linejoin(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for StrokeDasharray<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_stroke_dasharray(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Marker<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_marker(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ColorInterpolation
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_color_interpolation(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ColorRendering
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_color_rendering(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ShapeRendering
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_shape_rendering(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for TextRendering
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_text_rendering(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ImageRendering
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_image_rendering(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ClipPath<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_clip_path(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for GeometryBox
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_geometry_box(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for BasicShape<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_basic_shape(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ShapeRadius<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_shape_radius(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MaskMode
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_mode(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MaskClip
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_clip(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MaskComposite
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_composite(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MaskType
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_type(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for MaskBorderMode
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_mask_border_mode(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WebKitMaskComposite
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_mask_composite(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for WebKitMaskSourceType
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_mask_source_type(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for FilterList<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_filter_list(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for Filter<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_filter(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ZIndex
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_z_index(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ContainerType
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_container_type(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ContainerNameList<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_container_name_list(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ViewTransitionName<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_name(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for NoneOrCustomIdentList<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_none_or_custom_ident_list(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for ViewTransitionGroup<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_group(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for PrintColorAdjust
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_print_color_adjust(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CSSWideKeyword
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_css_wide_keyword(self);
    }
}
impl<'a, VisitorT> VisitNode<'a, VisitorT> for CustomPropertyName<'a>
where
    VisitorT: ?Sized + Visit<'a>,
{
    #[inline]
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_custom_property_name(self);
    }
}
impl<'a, VisitorT: ?Sized + Visit<'a>> VisitNode<'a, VisitorT> for Declaration<'a> {
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_declaration(self);
    }
}
impl<'a, VisitorT: ?Sized + Visit<'a>> VisitNode<'a, VisitorT> for PropertyId<'a> {
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_property_id(self);
    }
}
impl<'a, VisitorT: ?Sized + Visit<'a>> VisitNode<'a, VisitorT> for VendorPrefix {
    fn visit_node(&self, visitor: &mut VisitorT) {
        visitor.visit_vendor_prefix(self);
    }
}
pub fn walk_declaration<'a, VisitorT: ?Sized + Visit<'a>>(
    visitor: &mut VisitorT,
    node: &Declaration<'a>,
) {
    visitor.enter_node(AstType::Declaration);
    match node {
        Declaration::BackgroundColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackgroundImage(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackgroundPositionX(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackgroundPositionY(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackgroundPosition(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackgroundSize(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackgroundRepeat(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackgroundAttachment(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackgroundClip(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BackgroundOrigin(value) => VisitNode::visit_node(value, visitor),
        Declaration::Background(value) => VisitNode::visit_node(value, visitor),
        Declaration::BoxShadow(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Opacity(value) => VisitNode::visit_node(value, visitor),
        Declaration::Color(value) => VisitNode::visit_node(value, visitor),
        Declaration::Display(value) => VisitNode::visit_node(value, visitor),
        Declaration::Visibility(value) => VisitNode::visit_node(value, visitor),
        Declaration::Width(value) => VisitNode::visit_node(value, visitor),
        Declaration::Height(value) => VisitNode::visit_node(value, visitor),
        Declaration::MinWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::MinHeight(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaxWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaxHeight(value) => VisitNode::visit_node(value, visitor),
        Declaration::BlockSize(value) => VisitNode::visit_node(value, visitor),
        Declaration::InlineSize(value) => VisitNode::visit_node(value, visitor),
        Declaration::MinBlockSize(value) => VisitNode::visit_node(value, visitor),
        Declaration::MinInlineSize(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaxBlockSize(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaxInlineSize(value) => VisitNode::visit_node(value, visitor),
        Declaration::BoxSizing(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AspectRatio(value) => VisitNode::visit_node(value, visitor),
        Declaration::Overflow(value) => VisitNode::visit_node(value, visitor),
        Declaration::OverflowX(value) => VisitNode::visit_node(value, visitor),
        Declaration::OverflowY(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextOverflow(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Position(value) => VisitNode::visit_node(value, visitor),
        Declaration::Top(value) => VisitNode::visit_node(value, visitor),
        Declaration::Bottom(value) => VisitNode::visit_node(value, visitor),
        Declaration::Left(value) => VisitNode::visit_node(value, visitor),
        Declaration::Right(value) => VisitNode::visit_node(value, visitor),
        Declaration::InsetBlockStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::InsetBlockEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::InsetInlineStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::InsetInlineEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::InsetBlock(value) => VisitNode::visit_node(value, visitor),
        Declaration::InsetInline(value) => VisitNode::visit_node(value, visitor),
        Declaration::Inset(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderSpacing(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderTopColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBottomColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderLeftColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderRightColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockStartColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockEndColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineStartColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineEndColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderTopStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBottomStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderLeftStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderRightStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockStartStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockEndStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineStartStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineEndStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderTopWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBottomWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderLeftWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderRightWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockStartWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockEndWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineStartWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineEndWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderTopLeftRadius(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderTopRightRadius(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderBottomLeftRadius(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderBottomRightRadius(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderStartStartRadius(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderStartEndRadius(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderEndStartRadius(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderEndEndRadius(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderRadius(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderImageSource(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderImageOutset(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderImageRepeat(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderImageWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderImageSlice(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderImage(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::Border(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderTop(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBottom(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderLeft(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderRight(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlock(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderBlockEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInline(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::BorderInlineEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::Outline(value) => VisitNode::visit_node(value, visitor),
        Declaration::OutlineColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::OutlineStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::OutlineWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::FlexDirection(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexWrap(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexFlow(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexGrow(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexShrink(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexBasis(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Flex(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Order(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AlignContent(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::JustifyContent(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::PlaceContent(value) => VisitNode::visit_node(value, visitor),
        Declaration::AlignSelf(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::JustifySelf(value) => VisitNode::visit_node(value, visitor),
        Declaration::PlaceSelf(value) => VisitNode::visit_node(value, visitor),
        Declaration::AlignItems(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::JustifyItems(value) => VisitNode::visit_node(value, visitor),
        Declaration::PlaceItems(value) => VisitNode::visit_node(value, visitor),
        Declaration::RowGap(value) => VisitNode::visit_node(value, visitor),
        Declaration::ColumnGap(value) => VisitNode::visit_node(value, visitor),
        Declaration::Gap(value) => VisitNode::visit_node(value, visitor),
        Declaration::BoxOrient(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxDirection(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxOrdinalGroup(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxAlign(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxFlex(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxFlexGroup(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxPack(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxLines(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexPack(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexOrder(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexAlign(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexItemAlign(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexLinePack(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexPositive(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexNegative(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexPreferredSize(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::GridTemplateColumns(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridTemplateRows(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridAutoColumns(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridAutoRows(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridAutoFlow(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridTemplateAreas(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridTemplate(value) => VisitNode::visit_node(value, visitor),
        Declaration::Grid(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridRowStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridRowEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridColumnStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridColumnEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridRow(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridColumn(value) => VisitNode::visit_node(value, visitor),
        Declaration::GridArea(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginTop(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginBottom(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginLeft(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginRight(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginBlockStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginBlockEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginInlineStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginInlineEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginBlock(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarginInline(value) => VisitNode::visit_node(value, visitor),
        Declaration::Margin(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingTop(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingBottom(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingLeft(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingRight(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingBlockStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingBlockEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingInlineStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingInlineEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingBlock(value) => VisitNode::visit_node(value, visitor),
        Declaration::PaddingInline(value) => VisitNode::visit_node(value, visitor),
        Declaration::Padding(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginTop(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginBottom(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginLeft(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginRight(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginBlockStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginBlockEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginInlineStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginInlineEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginBlock(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMarginInline(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollMargin(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingTop(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingBottom(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingLeft(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingRight(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingBlockStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingBlockEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingInlineStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingInlineEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingBlock(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPaddingInline(value) => VisitNode::visit_node(value, visitor),
        Declaration::ScrollPadding(value) => VisitNode::visit_node(value, visitor),
        Declaration::FontWeight(value) => VisitNode::visit_node(value, visitor),
        Declaration::FontSize(value) => VisitNode::visit_node(value, visitor),
        Declaration::FontStretch(value) => VisitNode::visit_node(value, visitor),
        Declaration::FontFamily(value) => VisitNode::visit_node(value, visitor),
        Declaration::FontStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::FontVariantCaps(value) => VisitNode::visit_node(value, visitor),
        Declaration::LineHeight(value) => VisitNode::visit_node(value, visitor),
        Declaration::Font(value) => VisitNode::visit_node(value, visitor),
        Declaration::VerticalAlign(value) => VisitNode::visit_node(value, visitor),
        Declaration::FontPalette(value) => VisitNode::visit_node(value, visitor),
        Declaration::TransitionProperty(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransitionDuration(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransitionDelay(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransitionTimingFunction(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Transition(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationName(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationDuration(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationTimingFunction(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationIterationCount(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationDirection(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationPlayState(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationDelay(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationFillMode(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationComposition(value) => VisitNode::visit_node(value, visitor),
        Declaration::AnimationTimeline(value) => VisitNode::visit_node(value, visitor),
        Declaration::AnimationRangeStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::AnimationRangeEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::AnimationRange(value) => VisitNode::visit_node(value, visitor),
        Declaration::Animation(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Transform(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransformOrigin(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransformStyle(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransformBox(value) => VisitNode::visit_node(value, visitor),
        Declaration::BackfaceVisibility(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Perspective(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::PerspectiveOrigin(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Translate(value) => VisitNode::visit_node(value, visitor),
        Declaration::Rotate(value) => VisitNode::visit_node(value, visitor),
        Declaration::Scale(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextTransform(value) => VisitNode::visit_node(value, visitor),
        Declaration::WhiteSpace(value) => VisitNode::visit_node(value, visitor),
        Declaration::TabSize(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WordBreak(value) => VisitNode::visit_node(value, visitor),
        Declaration::LineBreak(value) => VisitNode::visit_node(value, visitor),
        Declaration::Hyphens(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::OverflowWrap(value) => VisitNode::visit_node(value, visitor),
        Declaration::WordWrap(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextAlign(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextAlignLast(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextJustify(value) => VisitNode::visit_node(value, visitor),
        Declaration::WordSpacing(value) => VisitNode::visit_node(value, visitor),
        Declaration::LetterSpacing(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextIndent(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextDecorationLine(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextDecorationStyle(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextDecorationColor(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextDecorationThickness(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextDecoration(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextDecorationSkipInk(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextEmphasisStyle(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextEmphasisColor(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextEmphasis(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextEmphasisPosition(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextShadow(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextSizeAdjust(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Direction(value) => VisitNode::visit_node(value, visitor),
        Declaration::UnicodeBidi(value) => VisitNode::visit_node(value, visitor),
        Declaration::BoxDecorationBreak(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Resize(value) => VisitNode::visit_node(value, visitor),
        Declaration::Cursor(value) => VisitNode::visit_node(value, visitor),
        Declaration::CaretColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::CaretShape(value) => VisitNode::visit_node(value, visitor),
        Declaration::Caret(value) => VisitNode::visit_node(value, visitor),
        Declaration::UserSelect(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AccentColor(value) => VisitNode::visit_node(value, visitor),
        Declaration::Appearance(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::ListStyleType(value) => VisitNode::visit_node(value, visitor),
        Declaration::ListStyleImage(value) => VisitNode::visit_node(value, visitor),
        Declaration::ListStylePosition(value) => VisitNode::visit_node(value, visitor),
        Declaration::ListStyle(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarkerSide(value) => VisitNode::visit_node(value, visitor),
        Declaration::Composes(value) => VisitNode::visit_node(value, visitor),
        Declaration::Fill(value) => VisitNode::visit_node(value, visitor),
        Declaration::FillRule(value) => VisitNode::visit_node(value, visitor),
        Declaration::FillOpacity(value) => VisitNode::visit_node(value, visitor),
        Declaration::Stroke(value) => VisitNode::visit_node(value, visitor),
        Declaration::StrokeOpacity(value) => VisitNode::visit_node(value, visitor),
        Declaration::StrokeWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::StrokeLinecap(value) => VisitNode::visit_node(value, visitor),
        Declaration::StrokeLinejoin(value) => VisitNode::visit_node(value, visitor),
        Declaration::StrokeMiterlimit(value) => VisitNode::visit_node(value, visitor),
        Declaration::StrokeDasharray(value) => VisitNode::visit_node(value, visitor),
        Declaration::StrokeDashoffset(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarkerStart(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarkerMid(value) => VisitNode::visit_node(value, visitor),
        Declaration::MarkerEnd(value) => VisitNode::visit_node(value, visitor),
        Declaration::Marker(value) => VisitNode::visit_node(value, visitor),
        Declaration::ColorInterpolation(value) => VisitNode::visit_node(value, visitor),
        Declaration::ColorInterpolationFilters(value) => VisitNode::visit_node(value, visitor),
        Declaration::ColorRendering(value) => VisitNode::visit_node(value, visitor),
        Declaration::ShapeRendering(value) => VisitNode::visit_node(value, visitor),
        Declaration::TextRendering(value) => VisitNode::visit_node(value, visitor),
        Declaration::ImageRendering(value) => VisitNode::visit_node(value, visitor),
        Declaration::ClipPath(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::ClipRule(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskImage(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskMode(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskRepeat(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskPositionX(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskPositionY(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskPosition(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskClip(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskOrigin(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskSize(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskComposite(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskType(value) => VisitNode::visit_node(value, visitor),
        Declaration::Mask(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskBorderSource(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskBorderMode(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskBorderSlice(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskBorderWidth(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskBorderOutset(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskBorderRepeat(value) => VisitNode::visit_node(value, visitor),
        Declaration::MaskBorder(value) => VisitNode::visit_node(value, visitor),
        Declaration::WebKitMaskComposite(value) => VisitNode::visit_node(value, visitor),
        Declaration::WebKitMaskSourceType(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImage(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageSource(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageSlice(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageWidth(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageOutset(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageRepeat(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Filter(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BackdropFilter(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MixBlendMode(value) => VisitNode::visit_node(value, visitor),
        Declaration::ZIndex(value) => VisitNode::visit_node(value, visitor),
        Declaration::ContainerType(value) => VisitNode::visit_node(value, visitor),
        Declaration::ContainerName(value) => VisitNode::visit_node(value, visitor),
        Declaration::Container(value) => VisitNode::visit_node(value, visitor),
        Declaration::ViewTransitionName(value) => VisitNode::visit_node(value, visitor),
        Declaration::ViewTransitionClass(value) => VisitNode::visit_node(value, visitor),
        Declaration::ViewTransitionGroup(value) => VisitNode::visit_node(value, visitor),
        Declaration::ColorScheme(value) => VisitNode::visit_node(value, visitor),
        Declaration::PrintColorAdjust(value, vendor_prefix) => {
            VisitNode::visit_node(value, visitor);
            VisitNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::All(value) => VisitNode::visit_node(value, visitor),
        Declaration::Unparsed(value) => VisitNode::visit_node(value, visitor),
        Declaration::Custom(value) => VisitNode::visit_node(value, visitor),
    }
    visitor.leave_node(AstType::Declaration);
}
pub fn walk_property_id<'a, VisitorT: ?Sized + Visit<'a>>(
    visitor: &mut VisitorT,
    node: &PropertyId<'a>,
) {
    visitor.enter_node(AstType::PropertyId);
    match node {
        PropertyId::BackgroundColor => {}
        PropertyId::BackgroundImage => {}
        PropertyId::BackgroundPositionX => {}
        PropertyId::BackgroundPositionY => {}
        PropertyId::BackgroundPosition => {}
        PropertyId::BackgroundSize => {}
        PropertyId::BackgroundRepeat => {}
        PropertyId::BackgroundAttachment => {}
        PropertyId::BackgroundClip(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BackgroundOrigin => {}
        PropertyId::Background => {}
        PropertyId::BoxShadow(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Opacity => {}
        PropertyId::Color => {}
        PropertyId::Display => {}
        PropertyId::Visibility => {}
        PropertyId::Width => {}
        PropertyId::Height => {}
        PropertyId::MinWidth => {}
        PropertyId::MinHeight => {}
        PropertyId::MaxWidth => {}
        PropertyId::MaxHeight => {}
        PropertyId::BlockSize => {}
        PropertyId::InlineSize => {}
        PropertyId::MinBlockSize => {}
        PropertyId::MinInlineSize => {}
        PropertyId::MaxBlockSize => {}
        PropertyId::MaxInlineSize => {}
        PropertyId::BoxSizing(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AspectRatio => {}
        PropertyId::Overflow => {}
        PropertyId::OverflowX => {}
        PropertyId::OverflowY => {}
        PropertyId::TextOverflow(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Position => {}
        PropertyId::Top => {}
        PropertyId::Bottom => {}
        PropertyId::Left => {}
        PropertyId::Right => {}
        PropertyId::InsetBlockStart => {}
        PropertyId::InsetBlockEnd => {}
        PropertyId::InsetInlineStart => {}
        PropertyId::InsetInlineEnd => {}
        PropertyId::InsetBlock => {}
        PropertyId::InsetInline => {}
        PropertyId::Inset => {}
        PropertyId::BorderSpacing => {}
        PropertyId::BorderTopColor => {}
        PropertyId::BorderBottomColor => {}
        PropertyId::BorderLeftColor => {}
        PropertyId::BorderRightColor => {}
        PropertyId::BorderBlockStartColor => {}
        PropertyId::BorderBlockEndColor => {}
        PropertyId::BorderInlineStartColor => {}
        PropertyId::BorderInlineEndColor => {}
        PropertyId::BorderTopStyle => {}
        PropertyId::BorderBottomStyle => {}
        PropertyId::BorderLeftStyle => {}
        PropertyId::BorderRightStyle => {}
        PropertyId::BorderBlockStartStyle => {}
        PropertyId::BorderBlockEndStyle => {}
        PropertyId::BorderInlineStartStyle => {}
        PropertyId::BorderInlineEndStyle => {}
        PropertyId::BorderTopWidth => {}
        PropertyId::BorderBottomWidth => {}
        PropertyId::BorderLeftWidth => {}
        PropertyId::BorderRightWidth => {}
        PropertyId::BorderBlockStartWidth => {}
        PropertyId::BorderBlockEndWidth => {}
        PropertyId::BorderInlineStartWidth => {}
        PropertyId::BorderInlineEndWidth => {}
        PropertyId::BorderTopLeftRadius(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BorderTopRightRadius(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BorderBottomLeftRadius(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BorderBottomRightRadius(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BorderStartStartRadius => {}
        PropertyId::BorderStartEndRadius => {}
        PropertyId::BorderEndStartRadius => {}
        PropertyId::BorderEndEndRadius => {}
        PropertyId::BorderRadius(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BorderImageSource => {}
        PropertyId::BorderImageOutset => {}
        PropertyId::BorderImageRepeat => {}
        PropertyId::BorderImageWidth => {}
        PropertyId::BorderImageSlice => {}
        PropertyId::BorderImage(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BorderColor => {}
        PropertyId::BorderStyle => {}
        PropertyId::BorderWidth => {}
        PropertyId::BorderBlockColor => {}
        PropertyId::BorderBlockStyle => {}
        PropertyId::BorderBlockWidth => {}
        PropertyId::BorderInlineColor => {}
        PropertyId::BorderInlineStyle => {}
        PropertyId::BorderInlineWidth => {}
        PropertyId::Border => {}
        PropertyId::BorderTop => {}
        PropertyId::BorderBottom => {}
        PropertyId::BorderLeft => {}
        PropertyId::BorderRight => {}
        PropertyId::BorderBlock => {}
        PropertyId::BorderBlockStart => {}
        PropertyId::BorderBlockEnd => {}
        PropertyId::BorderInline => {}
        PropertyId::BorderInlineStart => {}
        PropertyId::BorderInlineEnd => {}
        PropertyId::Outline => {}
        PropertyId::OutlineColor => {}
        PropertyId::OutlineStyle => {}
        PropertyId::OutlineWidth => {}
        PropertyId::FlexDirection(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexWrap(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexFlow(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexGrow(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexShrink(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexBasis(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Flex(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Order(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AlignContent(value) => VisitNode::visit_node(value, visitor),
        PropertyId::JustifyContent(value) => VisitNode::visit_node(value, visitor),
        PropertyId::PlaceContent => {}
        PropertyId::AlignSelf(value) => VisitNode::visit_node(value, visitor),
        PropertyId::JustifySelf => {}
        PropertyId::PlaceSelf => {}
        PropertyId::AlignItems(value) => VisitNode::visit_node(value, visitor),
        PropertyId::JustifyItems => {}
        PropertyId::PlaceItems => {}
        PropertyId::RowGap => {}
        PropertyId::ColumnGap => {}
        PropertyId::Gap => {}
        PropertyId::BoxOrient(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BoxDirection(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BoxOrdinalGroup(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BoxAlign(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BoxFlex(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BoxFlexGroup(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BoxPack(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BoxLines(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexPack(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexOrder(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexAlign(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexItemAlign(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexLinePack(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexPositive(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexNegative(value) => VisitNode::visit_node(value, visitor),
        PropertyId::FlexPreferredSize(value) => VisitNode::visit_node(value, visitor),
        PropertyId::GridTemplateColumns => {}
        PropertyId::GridTemplateRows => {}
        PropertyId::GridAutoColumns => {}
        PropertyId::GridAutoRows => {}
        PropertyId::GridAutoFlow => {}
        PropertyId::GridTemplateAreas => {}
        PropertyId::GridTemplate => {}
        PropertyId::Grid => {}
        PropertyId::GridRowStart => {}
        PropertyId::GridRowEnd => {}
        PropertyId::GridColumnStart => {}
        PropertyId::GridColumnEnd => {}
        PropertyId::GridRow => {}
        PropertyId::GridColumn => {}
        PropertyId::GridArea => {}
        PropertyId::MarginTop => {}
        PropertyId::MarginBottom => {}
        PropertyId::MarginLeft => {}
        PropertyId::MarginRight => {}
        PropertyId::MarginBlockStart => {}
        PropertyId::MarginBlockEnd => {}
        PropertyId::MarginInlineStart => {}
        PropertyId::MarginInlineEnd => {}
        PropertyId::MarginBlock => {}
        PropertyId::MarginInline => {}
        PropertyId::Margin => {}
        PropertyId::PaddingTop => {}
        PropertyId::PaddingBottom => {}
        PropertyId::PaddingLeft => {}
        PropertyId::PaddingRight => {}
        PropertyId::PaddingBlockStart => {}
        PropertyId::PaddingBlockEnd => {}
        PropertyId::PaddingInlineStart => {}
        PropertyId::PaddingInlineEnd => {}
        PropertyId::PaddingBlock => {}
        PropertyId::PaddingInline => {}
        PropertyId::Padding => {}
        PropertyId::ScrollMarginTop => {}
        PropertyId::ScrollMarginBottom => {}
        PropertyId::ScrollMarginLeft => {}
        PropertyId::ScrollMarginRight => {}
        PropertyId::ScrollMarginBlockStart => {}
        PropertyId::ScrollMarginBlockEnd => {}
        PropertyId::ScrollMarginInlineStart => {}
        PropertyId::ScrollMarginInlineEnd => {}
        PropertyId::ScrollMarginBlock => {}
        PropertyId::ScrollMarginInline => {}
        PropertyId::ScrollMargin => {}
        PropertyId::ScrollPaddingTop => {}
        PropertyId::ScrollPaddingBottom => {}
        PropertyId::ScrollPaddingLeft => {}
        PropertyId::ScrollPaddingRight => {}
        PropertyId::ScrollPaddingBlockStart => {}
        PropertyId::ScrollPaddingBlockEnd => {}
        PropertyId::ScrollPaddingInlineStart => {}
        PropertyId::ScrollPaddingInlineEnd => {}
        PropertyId::ScrollPaddingBlock => {}
        PropertyId::ScrollPaddingInline => {}
        PropertyId::ScrollPadding => {}
        PropertyId::FontWeight => {}
        PropertyId::FontSize => {}
        PropertyId::FontStretch => {}
        PropertyId::FontFamily => {}
        PropertyId::FontStyle => {}
        PropertyId::FontVariantCaps => {}
        PropertyId::LineHeight => {}
        PropertyId::Font => {}
        PropertyId::VerticalAlign => {}
        PropertyId::FontPalette => {}
        PropertyId::TransitionProperty(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TransitionDuration(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TransitionDelay(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TransitionTimingFunction(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Transition(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationName(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationDuration(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationTimingFunction(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationIterationCount(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationDirection(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationPlayState(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationDelay(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationFillMode(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AnimationComposition => {}
        PropertyId::AnimationTimeline => {}
        PropertyId::AnimationRangeStart => {}
        PropertyId::AnimationRangeEnd => {}
        PropertyId::AnimationRange => {}
        PropertyId::Animation(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Transform(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TransformOrigin(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TransformStyle(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TransformBox => {}
        PropertyId::BackfaceVisibility(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Perspective(value) => VisitNode::visit_node(value, visitor),
        PropertyId::PerspectiveOrigin(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Translate => {}
        PropertyId::Rotate => {}
        PropertyId::Scale => {}
        PropertyId::TextTransform => {}
        PropertyId::WhiteSpace => {}
        PropertyId::TabSize(value) => VisitNode::visit_node(value, visitor),
        PropertyId::WordBreak => {}
        PropertyId::LineBreak => {}
        PropertyId::Hyphens(value) => VisitNode::visit_node(value, visitor),
        PropertyId::OverflowWrap => {}
        PropertyId::WordWrap => {}
        PropertyId::TextAlign => {}
        PropertyId::TextAlignLast(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextJustify => {}
        PropertyId::WordSpacing => {}
        PropertyId::LetterSpacing => {}
        PropertyId::TextIndent => {}
        PropertyId::TextDecorationLine(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextDecorationStyle(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextDecorationColor(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextDecorationThickness => {}
        PropertyId::TextDecoration(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextDecorationSkipInk(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextEmphasisStyle(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextEmphasisColor(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextEmphasis(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextEmphasisPosition(value) => VisitNode::visit_node(value, visitor),
        PropertyId::TextShadow => {}
        PropertyId::TextSizeAdjust(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Direction => {}
        PropertyId::UnicodeBidi => {}
        PropertyId::BoxDecorationBreak(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Resize => {}
        PropertyId::Cursor => {}
        PropertyId::CaretColor => {}
        PropertyId::CaretShape => {}
        PropertyId::Caret => {}
        PropertyId::UserSelect(value) => VisitNode::visit_node(value, visitor),
        PropertyId::AccentColor => {}
        PropertyId::Appearance(value) => VisitNode::visit_node(value, visitor),
        PropertyId::ListStyleType => {}
        PropertyId::ListStyleImage => {}
        PropertyId::ListStylePosition => {}
        PropertyId::ListStyle => {}
        PropertyId::MarkerSide => {}
        PropertyId::Composes => {}
        PropertyId::Fill => {}
        PropertyId::FillRule => {}
        PropertyId::FillOpacity => {}
        PropertyId::Stroke => {}
        PropertyId::StrokeOpacity => {}
        PropertyId::StrokeWidth => {}
        PropertyId::StrokeLinecap => {}
        PropertyId::StrokeLinejoin => {}
        PropertyId::StrokeMiterlimit => {}
        PropertyId::StrokeDasharray => {}
        PropertyId::StrokeDashoffset => {}
        PropertyId::MarkerStart => {}
        PropertyId::MarkerMid => {}
        PropertyId::MarkerEnd => {}
        PropertyId::Marker => {}
        PropertyId::ColorInterpolation => {}
        PropertyId::ColorInterpolationFilters => {}
        PropertyId::ColorRendering => {}
        PropertyId::ShapeRendering => {}
        PropertyId::TextRendering => {}
        PropertyId::ImageRendering => {}
        PropertyId::ClipPath(value) => VisitNode::visit_node(value, visitor),
        PropertyId::ClipRule => {}
        PropertyId::MaskImage(value) => VisitNode::visit_node(value, visitor),
        PropertyId::MaskMode => {}
        PropertyId::MaskRepeat(value) => VisitNode::visit_node(value, visitor),
        PropertyId::MaskPositionX => {}
        PropertyId::MaskPositionY => {}
        PropertyId::MaskPosition(value) => VisitNode::visit_node(value, visitor),
        PropertyId::MaskClip(value) => VisitNode::visit_node(value, visitor),
        PropertyId::MaskOrigin(value) => VisitNode::visit_node(value, visitor),
        PropertyId::MaskSize(value) => VisitNode::visit_node(value, visitor),
        PropertyId::MaskComposite => {}
        PropertyId::MaskType => {}
        PropertyId::Mask(value) => VisitNode::visit_node(value, visitor),
        PropertyId::MaskBorderSource => {}
        PropertyId::MaskBorderMode => {}
        PropertyId::MaskBorderSlice => {}
        PropertyId::MaskBorderWidth => {}
        PropertyId::MaskBorderOutset => {}
        PropertyId::MaskBorderRepeat => {}
        PropertyId::MaskBorder => {}
        PropertyId::WebKitMaskComposite => {}
        PropertyId::WebKitMaskSourceType(value) => VisitNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImage(value) => VisitNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageSource(value) => VisitNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageSlice(value) => VisitNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageWidth(value) => VisitNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageOutset(value) => VisitNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageRepeat(value) => VisitNode::visit_node(value, visitor),
        PropertyId::Filter(value) => VisitNode::visit_node(value, visitor),
        PropertyId::BackdropFilter(value) => VisitNode::visit_node(value, visitor),
        PropertyId::MixBlendMode => {}
        PropertyId::ZIndex => {}
        PropertyId::ContainerType => {}
        PropertyId::ContainerName => {}
        PropertyId::Container => {}
        PropertyId::ViewTransitionName => {}
        PropertyId::ViewTransitionClass => {}
        PropertyId::ViewTransitionGroup => {}
        PropertyId::ColorScheme => {}
        PropertyId::PrintColorAdjust(value) => VisitNode::visit_node(value, visitor),
        PropertyId::ColumnRule
        | PropertyId::Columns
        | PropertyId::GridColumnGap
        | PropertyId::GridRowGap
        | PropertyId::All
        | PropertyId::Unparsed => {}
        PropertyId::Custom(value) => visitor.visit_str(value),
    }
    visitor.leave_node(AstType::PropertyId);
}
pub fn walk_vendor_prefix<'a, VisitorT: ?Sized + Visit<'a>>(
    visitor: &mut VisitorT,
    _node: &VendorPrefix,
) {
    visitor.enter_node(AstType::VendorPrefix);
    visitor.leave_node(AstType::VendorPrefix);
}
pub mod walk {
    pub use super::color::*;
    pub use super::css_rule::*;
    pub use super::length::*;
    pub use super::media::*;
    pub use super::properties::*;
    pub use super::rules::*;
    pub use super::selector::*;
    pub use super::span::*;
    pub use super::token::*;
    pub use super::values::*;
    pub use super::{walk_declaration, walk_property_id, walk_vendor_prefix};
}
