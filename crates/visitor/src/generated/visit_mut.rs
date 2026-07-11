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
pub trait VisitMut<'a> {
    #[inline]
    fn enter_node(&mut self, _kind: AstType) {}
    #[inline]
    fn leave_node(&mut self, _kind: AstType) {}
    #[inline]
    fn visit_str(&mut self, _value: &mut &'a str) {}
    #[inline]
    fn visit_css_color(&mut self, node: &mut CssColor<'a>) {
        color::walk_css_color(self, node);
    }
    #[inline]
    fn visit_rgba(&mut self, node: &mut RGBA) {
        color::walk_rgba(self, node);
    }
    #[inline]
    fn visit_lab_color(&mut self, node: &mut LABColor) {
        color::walk_lab_color(self, node);
    }
    #[inline]
    fn visit_predefined_color(&mut self, node: &mut PredefinedColor) {
        color::walk_predefined_color(self, node);
    }
    #[inline]
    fn visit_float_color(&mut self, node: &mut FloatColor) {
        color::walk_float_color(self, node);
    }
    #[inline]
    fn visit_light_dark(&mut self, node: &mut LightDark<'a>) {
        color::walk_light_dark(self, node);
    }
    #[inline]
    fn visit_system_color(&mut self, node: &mut SystemColor) {
        color::walk_system_color(self, node);
    }
    #[inline]
    fn visit_unresolved_color(&mut self, node: &mut UnresolvedColor<'a>) {
        color::walk_unresolved_color(self, node);
    }
    #[inline]
    fn visit_css_rule(&mut self, node: &mut CssRule<'a>) {
        css_rule::walk_css_rule(self, node);
    }
    #[inline]
    fn visit_length(&mut self, node: &mut Length<'a>) {
        length::walk_length(self, node);
    }
    #[inline]
    fn visit_length_unit(&mut self, node: &mut LengthUnit) {
        length::walk_length_unit(self, node);
    }
    #[inline]
    fn visit_calc<V>(&mut self, node: &mut Calc<'a, V>)
    where
        V: VisitMutNode<'a, Self>,
    {
        length::walk_calc(self, node);
    }
    #[inline]
    fn visit_math_function<V>(&mut self, node: &mut MathFunction<'a, V>)
    where
        V: VisitMutNode<'a, Self>,
    {
        length::walk_math_function(self, node);
    }
    #[inline]
    fn visit_rounding_strategy(&mut self, node: &mut RoundingStrategy) {
        length::walk_rounding_strategy(self, node);
    }
    #[inline]
    fn visit_resolution(&mut self, node: &mut Resolution) {
        length::walk_resolution(self, node);
    }
    #[inline]
    fn visit_ratio(&mut self, node: &mut Ratio) {
        length::walk_ratio(self, node);
    }
    #[inline]
    fn visit_angle(&mut self, node: &mut Angle) {
        length::walk_angle(self, node);
    }
    #[inline]
    fn visit_time(&mut self, node: &mut Time) {
        length::walk_time(self, node);
    }
    #[inline]
    fn visit_media_condition(&mut self, node: &mut MediaCondition<'a>) {
        media::walk_media_condition(self, node);
    }
    #[inline]
    fn visit_query_feature<FeatureId>(&mut self, node: &mut QueryFeature<'a, FeatureId>)
    where
        FeatureId: VisitMutNode<'a, Self>,
    {
        media::walk_query_feature(self, node);
    }
    #[inline]
    fn visit_media_feature_name<FeatureId>(&mut self, node: &mut MediaFeatureName<'a, FeatureId>)
    where
        FeatureId: VisitMutNode<'a, Self>,
    {
        media::walk_media_feature_name(self, node);
    }
    #[inline]
    fn visit_media_feature_id(&mut self, node: &mut MediaFeatureId) {
        media::walk_media_feature_id(self, node);
    }
    #[inline]
    fn visit_media_feature_value(&mut self, node: &mut MediaFeatureValue<'a>) {
        media::walk_media_feature_value(self, node);
    }
    #[inline]
    fn visit_media_feature_comparison(&mut self, node: &mut MediaFeatureComparison) {
        media::walk_media_feature_comparison(self, node);
    }
    #[inline]
    fn visit_operator(&mut self, node: &mut Operator) {
        media::walk_operator(self, node);
    }
    #[inline]
    fn visit_media_type(&mut self, node: &mut MediaType<'a>) {
        media::walk_media_type(self, node);
    }
    #[inline]
    fn visit_qualifier(&mut self, node: &mut Qualifier) {
        media::walk_qualifier(self, node);
    }
    #[inline]
    fn visit_supports_condition(&mut self, node: &mut SupportsCondition<'a>) {
        media::walk_supports_condition(self, node);
    }
    #[inline]
    fn visit_blend_mode(&mut self, node: &mut BlendMode) {
        properties::walk_blend_mode(self, node);
    }
    #[inline]
    fn visit_keyframe_selector(&mut self, node: &mut KeyframeSelector<'a>) {
        rules::walk_keyframe_selector(self, node);
    }
    #[inline]
    fn visit_keyframes_name(&mut self, node: &mut KeyframesName<'a>) {
        rules::walk_keyframes_name(self, node);
    }
    #[inline]
    fn visit_font_face_property(&mut self, node: &mut FontFaceProperty<'a>) {
        rules::walk_font_face_property(self, node);
    }
    #[inline]
    fn visit_source(&mut self, node: &mut Source<'a>) {
        rules::walk_source(self, node);
    }
    #[inline]
    fn visit_font_format(&mut self, node: &mut FontFormat<'a>) {
        rules::walk_font_format(self, node);
    }
    #[inline]
    fn visit_font_technology(&mut self, node: &mut FontTechnology) {
        rules::walk_font_technology(self, node);
    }
    #[inline]
    fn visit_font_face_style(&mut self, node: &mut FontFaceStyle<'a>) {
        rules::walk_font_face_style(self, node);
    }
    #[inline]
    fn visit_font_palette_values_property(&mut self, node: &mut FontPaletteValuesProperty<'a>) {
        rules::walk_font_palette_values_property(self, node);
    }
    #[inline]
    fn visit_base_palette(&mut self, node: &mut BasePalette) {
        rules::walk_base_palette(self, node);
    }
    #[inline]
    fn visit_font_feature_subrule_type(&mut self, node: &mut FontFeatureSubruleType) {
        rules::walk_font_feature_subrule_type(self, node);
    }
    #[inline]
    fn visit_page_margin_box(&mut self, node: &mut PageMarginBox) {
        rules::walk_page_margin_box(self, node);
    }
    #[inline]
    fn visit_page_pseudo_class(&mut self, node: &mut PagePseudoClass) {
        rules::walk_page_pseudo_class(self, node);
    }
    #[inline]
    fn visit_parsed_component(&mut self, node: &mut ParsedComponent<'a>) {
        rules::walk_parsed_component(self, node);
    }
    #[inline]
    fn visit_multiplier(&mut self, node: &mut Multiplier) {
        rules::walk_multiplier(self, node);
    }
    #[inline]
    fn visit_syntax_string(&mut self, node: &mut SyntaxString<'a>) {
        rules::walk_syntax_string(self, node);
    }
    #[inline]
    fn visit_syntax_component_kind(&mut self, node: &mut SyntaxComponentKind<'a>) {
        rules::walk_syntax_component_kind(self, node);
    }
    #[inline]
    fn visit_container_condition(&mut self, node: &mut ContainerCondition<'a>) {
        rules::walk_container_condition(self, node);
    }
    #[inline]
    fn visit_container_size_feature_id(&mut self, node: &mut ContainerSizeFeatureId) {
        rules::walk_container_size_feature_id(self, node);
    }
    #[inline]
    fn visit_style_query(&mut self, node: &mut StyleQuery<'a>) {
        rules::walk_style_query(self, node);
    }
    #[inline]
    fn visit_scroll_state_query(&mut self, node: &mut ScrollStateQuery<'a>) {
        rules::walk_scroll_state_query(self, node);
    }
    #[inline]
    fn visit_scroll_state_feature_id(&mut self, node: &mut ScrollStateFeatureId) {
        rules::walk_scroll_state_feature_id(self, node);
    }
    #[inline]
    fn visit_view_transition_property(&mut self, node: &mut ViewTransitionProperty<'a>) {
        rules::walk_view_transition_property(self, node);
    }
    #[inline]
    fn visit_navigation(&mut self, node: &mut Navigation) {
        rules::walk_navigation(self, node);
    }
    #[inline]
    fn visit_default_at_rule(&mut self, node: &mut DefaultAtRule) {
        rules::walk_default_at_rule(self, node);
    }
    #[inline]
    fn visit_style_sheet(&mut self, node: &mut StyleSheet<'a>) {
        rules::walk_style_sheet(self, node);
    }
    #[inline]
    fn visit_media_rule(&mut self, node: &mut MediaRule<'a>) {
        rules::walk_media_rule(self, node);
    }
    #[inline]
    fn visit_media_list(&mut self, node: &mut MediaList<'a>) {
        rules::walk_media_list(self, node);
    }
    #[inline]
    fn visit_media_query(&mut self, node: &mut MediaQuery<'a>) {
        rules::walk_media_query(self, node);
    }
    #[inline]
    fn visit_length_value(&mut self, node: &mut LengthValue) {
        rules::walk_length_value(self, node);
    }
    #[inline]
    fn visit_environment_variable(&mut self, node: &mut EnvironmentVariable<'a>) {
        rules::walk_environment_variable(self, node);
    }
    #[inline]
    fn visit_url(&mut self, node: &mut Url<'a>) {
        rules::walk_url(self, node);
    }
    #[inline]
    fn visit_variable(&mut self, node: &mut Variable<'a>) {
        rules::walk_variable(self, node);
    }
    #[inline]
    fn visit_dashed_ident_reference(&mut self, node: &mut DashedIdentReference<'a>) {
        rules::walk_dashed_ident_reference(self, node);
    }
    #[inline]
    fn visit_function(&mut self, node: &mut Function<'a>) {
        rules::walk_function(self, node);
    }
    #[inline]
    fn visit_import_rule(&mut self, node: &mut ImportRule<'a>) {
        rules::walk_import_rule(self, node);
    }
    #[inline]
    fn visit_style_rule(&mut self, node: &mut StyleRule<'a>) {
        rules::walk_style_rule(self, node);
    }
    #[inline]
    fn visit_declaration_block(&mut self, node: &mut DeclarationBlock<'a>) {
        rules::walk_declaration_block(self, node);
    }
    #[inline]
    fn visit_position(&mut self, node: &mut Position<'a>) {
        rules::walk_position(self, node);
    }
    #[inline]
    fn visit_web_kit_gradient_point(&mut self, node: &mut WebKitGradientPoint<'a>) {
        rules::walk_web_kit_gradient_point(self, node);
    }
    #[inline]
    fn visit_web_kit_color_stop(&mut self, node: &mut WebKitColorStop<'a>) {
        rules::walk_web_kit_color_stop(self, node);
    }
    #[inline]
    fn visit_image_set(&mut self, node: &mut ImageSet<'a>) {
        rules::walk_image_set(self, node);
    }
    #[inline]
    fn visit_image_set_option(&mut self, node: &mut ImageSetOption<'a>) {
        rules::walk_image_set_option(self, node);
    }
    #[inline]
    fn visit_background_position(&mut self, node: &mut BackgroundPosition<'a>) {
        rules::walk_background_position(self, node);
    }
    #[inline]
    fn visit_background_repeat(&mut self, node: &mut BackgroundRepeat) {
        rules::walk_background_repeat(self, node);
    }
    #[inline]
    fn visit_background(&mut self, node: &mut Background<'a>) {
        rules::walk_background(self, node);
    }
    #[inline]
    fn visit_box_shadow(&mut self, node: &mut BoxShadow<'a>) {
        rules::walk_box_shadow(self, node);
    }
    #[inline]
    fn visit_aspect_ratio(&mut self, node: &mut AspectRatio<'a>) {
        rules::walk_aspect_ratio(self, node);
    }
    #[inline]
    fn visit_overflow(&mut self, node: &mut Overflow) {
        rules::walk_overflow(self, node);
    }
    #[inline]
    fn visit_inset_block(&mut self, node: &mut InsetBlock<'a>) {
        rules::walk_inset_block(self, node);
    }
    #[inline]
    fn visit_inset_inline(&mut self, node: &mut InsetInline<'a>) {
        rules::walk_inset_inline(self, node);
    }
    #[inline]
    fn visit_inset(&mut self, node: &mut Inset<'a>) {
        rules::walk_inset(self, node);
    }
    #[inline]
    fn visit_border_radius(&mut self, node: &mut BorderRadius<'a>) {
        rules::walk_border_radius(self, node);
    }
    #[inline]
    fn visit_border_image_repeat(&mut self, node: &mut BorderImageRepeat) {
        rules::walk_border_image_repeat(self, node);
    }
    #[inline]
    fn visit_border_image_slice(&mut self, node: &mut BorderImageSlice<'a>) {
        rules::walk_border_image_slice(self, node);
    }
    #[inline]
    fn visit_border_image(&mut self, node: &mut BorderImage<'a>) {
        rules::walk_border_image(self, node);
    }
    #[inline]
    fn visit_border_color(&mut self, node: &mut BorderColor<'a>) {
        rules::walk_border_color(self, node);
    }
    #[inline]
    fn visit_border_style(&mut self, node: &mut BorderStyle) {
        rules::walk_border_style(self, node);
    }
    #[inline]
    fn visit_border_width(&mut self, node: &mut BorderWidth<'a>) {
        rules::walk_border_width(self, node);
    }
    #[inline]
    fn visit_border_block_color(&mut self, node: &mut BorderBlockColor<'a>) {
        rules::walk_border_block_color(self, node);
    }
    #[inline]
    fn visit_border_block_style(&mut self, node: &mut BorderBlockStyle) {
        rules::walk_border_block_style(self, node);
    }
    #[inline]
    fn visit_border_block_width(&mut self, node: &mut BorderBlockWidth<'a>) {
        rules::walk_border_block_width(self, node);
    }
    #[inline]
    fn visit_border_inline_color(&mut self, node: &mut BorderInlineColor<'a>) {
        rules::walk_border_inline_color(self, node);
    }
    #[inline]
    fn visit_border_inline_style(&mut self, node: &mut BorderInlineStyle) {
        rules::walk_border_inline_style(self, node);
    }
    #[inline]
    fn visit_border_inline_width(&mut self, node: &mut BorderInlineWidth<'a>) {
        rules::walk_border_inline_width(self, node);
    }
    #[inline]
    fn visit_generic_border<S>(&mut self, node: &mut GenericBorder<'a, S>)
    where
        S: VisitMutNode<'a, Self>,
    {
        rules::walk_generic_border(self, node);
    }
    #[inline]
    fn visit_flex_flow(&mut self, node: &mut FlexFlow) {
        rules::walk_flex_flow(self, node);
    }
    #[inline]
    fn visit_flex(&mut self, node: &mut Flex<'a>) {
        rules::walk_flex(self, node);
    }
    #[inline]
    fn visit_place_content(&mut self, node: &mut PlaceContent<'a>) {
        rules::walk_place_content(self, node);
    }
    #[inline]
    fn visit_place_self(&mut self, node: &mut PlaceSelf<'a>) {
        rules::walk_place_self(self, node);
    }
    #[inline]
    fn visit_place_items(&mut self, node: &mut PlaceItems<'a>) {
        rules::walk_place_items(self, node);
    }
    #[inline]
    fn visit_gap(&mut self, node: &mut Gap<'a>) {
        rules::walk_gap(self, node);
    }
    #[inline]
    fn visit_track_repeat(&mut self, node: &mut TrackRepeat<'a>) {
        rules::walk_track_repeat(self, node);
    }
    #[inline]
    fn visit_grid_auto_flow(&mut self, node: &mut GridAutoFlow) {
        rules::walk_grid_auto_flow(self, node);
    }
    #[inline]
    fn visit_grid_template(&mut self, node: &mut GridTemplate<'a>) {
        rules::walk_grid_template(self, node);
    }
    #[inline]
    fn visit_grid(&mut self, node: &mut Grid<'a>) {
        rules::walk_grid(self, node);
    }
    #[inline]
    fn visit_grid_row(&mut self, node: &mut GridRow<'a>) {
        rules::walk_grid_row(self, node);
    }
    #[inline]
    fn visit_grid_column(&mut self, node: &mut GridColumn<'a>) {
        rules::walk_grid_column(self, node);
    }
    #[inline]
    fn visit_grid_area(&mut self, node: &mut GridArea<'a>) {
        rules::walk_grid_area(self, node);
    }
    #[inline]
    fn visit_margin_block(&mut self, node: &mut MarginBlock<'a>) {
        rules::walk_margin_block(self, node);
    }
    #[inline]
    fn visit_margin_inline(&mut self, node: &mut MarginInline<'a>) {
        rules::walk_margin_inline(self, node);
    }
    #[inline]
    fn visit_margin(&mut self, node: &mut Margin<'a>) {
        rules::walk_margin(self, node);
    }
    #[inline]
    fn visit_padding_block(&mut self, node: &mut PaddingBlock<'a>) {
        rules::walk_padding_block(self, node);
    }
    #[inline]
    fn visit_padding_inline(&mut self, node: &mut PaddingInline<'a>) {
        rules::walk_padding_inline(self, node);
    }
    #[inline]
    fn visit_padding(&mut self, node: &mut Padding<'a>) {
        rules::walk_padding(self, node);
    }
    #[inline]
    fn visit_scroll_margin_block(&mut self, node: &mut ScrollMarginBlock<'a>) {
        rules::walk_scroll_margin_block(self, node);
    }
    #[inline]
    fn visit_scroll_margin_inline(&mut self, node: &mut ScrollMarginInline<'a>) {
        rules::walk_scroll_margin_inline(self, node);
    }
    #[inline]
    fn visit_scroll_margin(&mut self, node: &mut ScrollMargin<'a>) {
        rules::walk_scroll_margin(self, node);
    }
    #[inline]
    fn visit_scroll_padding_block(&mut self, node: &mut ScrollPaddingBlock<'a>) {
        rules::walk_scroll_padding_block(self, node);
    }
    #[inline]
    fn visit_scroll_padding_inline(&mut self, node: &mut ScrollPaddingInline<'a>) {
        rules::walk_scroll_padding_inline(self, node);
    }
    #[inline]
    fn visit_scroll_padding(&mut self, node: &mut ScrollPadding<'a>) {
        rules::walk_scroll_padding(self, node);
    }
    #[inline]
    fn visit_font(&mut self, node: &mut Font<'a>) {
        rules::walk_font(self, node);
    }
    #[inline]
    fn visit_transition(&mut self, node: &mut Transition<'a>) {
        rules::walk_transition(self, node);
    }
    #[inline]
    fn visit_scroll_timeline(&mut self, node: &mut ScrollTimeline) {
        rules::walk_scroll_timeline(self, node);
    }
    #[inline]
    fn visit_view_timeline(&mut self, node: &mut ViewTimeline<'a>) {
        rules::walk_view_timeline(self, node);
    }
    #[inline]
    fn visit_animation_range(&mut self, node: &mut AnimationRange<'a>) {
        rules::walk_animation_range(self, node);
    }
    #[inline]
    fn visit_animation(&mut self, node: &mut Animation<'a>) {
        rules::walk_animation(self, node);
    }
    #[inline]
    fn visit_matrix_for_float(&mut self, node: &mut MatrixForFloat) {
        rules::walk_matrix_for_float(self, node);
    }
    #[inline]
    fn visit_matrix_3_d_for_float(&mut self, node: &mut Matrix3DForFloat) {
        rules::walk_matrix_3_d_for_float(self, node);
    }
    #[inline]
    fn visit_rotate(&mut self, node: &mut Rotate<'a>) {
        rules::walk_rotate(self, node);
    }
    #[inline]
    fn visit_text_transform(&mut self, node: &mut TextTransform) {
        rules::walk_text_transform(self, node);
    }
    #[inline]
    fn visit_text_indent(&mut self, node: &mut TextIndent<'a>) {
        rules::walk_text_indent(self, node);
    }
    #[inline]
    fn visit_text_decoration(&mut self, node: &mut TextDecoration<'a>) {
        rules::walk_text_decoration(self, node);
    }
    #[inline]
    fn visit_text_emphasis(&mut self, node: &mut TextEmphasis<'a>) {
        rules::walk_text_emphasis(self, node);
    }
    #[inline]
    fn visit_text_emphasis_position(&mut self, node: &mut TextEmphasisPosition) {
        rules::walk_text_emphasis_position(self, node);
    }
    #[inline]
    fn visit_text_shadow(&mut self, node: &mut TextShadow<'a>) {
        rules::walk_text_shadow(self, node);
    }
    #[inline]
    fn visit_cursor(&mut self, node: &mut Cursor<'a>) {
        rules::walk_cursor(self, node);
    }
    #[inline]
    fn visit_cursor_image(&mut self, node: &mut CursorImage<'a>) {
        rules::walk_cursor_image(self, node);
    }
    #[inline]
    fn visit_caret(&mut self, node: &mut Caret<'a>) {
        rules::walk_caret(self, node);
    }
    #[inline]
    fn visit_list_style(&mut self, node: &mut ListStyle<'a>) {
        rules::walk_list_style(self, node);
    }
    #[inline]
    fn visit_composes(&mut self, node: &mut Composes<'a>) {
        rules::walk_composes(self, node);
    }
    #[inline]
    fn visit_inset_rect(&mut self, node: &mut InsetRect<'a>) {
        rules::walk_inset_rect(self, node);
    }
    #[inline]
    fn visit_circle_shape(&mut self, node: &mut CircleShape<'a>) {
        rules::walk_circle_shape(self, node);
    }
    #[inline]
    fn visit_ellipse_shape(&mut self, node: &mut EllipseShape<'a>) {
        rules::walk_ellipse_shape(self, node);
    }
    #[inline]
    fn visit_polygon(&mut self, node: &mut Polygon<'a>) {
        rules::walk_polygon(self, node);
    }
    #[inline]
    fn visit_point(&mut self, node: &mut Point<'a>) {
        rules::walk_point(self, node);
    }
    #[inline]
    fn visit_mask(&mut self, node: &mut Mask<'a>) {
        rules::walk_mask(self, node);
    }
    #[inline]
    fn visit_mask_border(&mut self, node: &mut MaskBorder<'a>) {
        rules::walk_mask_border(self, node);
    }
    #[inline]
    fn visit_drop_shadow(&mut self, node: &mut DropShadow<'a>) {
        rules::walk_drop_shadow(self, node);
    }
    #[inline]
    fn visit_container(&mut self, node: &mut Container<'a>) {
        rules::walk_container(self, node);
    }
    #[inline]
    fn visit_color_scheme(&mut self, node: &mut ColorScheme) {
        rules::walk_color_scheme(self, node);
    }
    #[inline]
    fn visit_unparsed_property(&mut self, node: &mut UnparsedProperty<'a>) {
        rules::walk_unparsed_property(self, node);
    }
    #[inline]
    fn visit_custom_property(&mut self, node: &mut CustomProperty<'a>) {
        rules::walk_custom_property(self, node);
    }
    #[inline]
    fn visit_view_transition_part_selector(&mut self, node: &mut ViewTransitionPartSelector<'a>) {
        rules::walk_view_transition_part_selector(self, node);
    }
    #[inline]
    fn visit_keyframes_rule(&mut self, node: &mut KeyframesRule<'a>) {
        rules::walk_keyframes_rule(self, node);
    }
    #[inline]
    fn visit_keyframe(&mut self, node: &mut Keyframe<'a>) {
        rules::walk_keyframe(self, node);
    }
    #[inline]
    fn visit_timeline_range_percentage(&mut self, node: &mut TimelineRangePercentage) {
        rules::walk_timeline_range_percentage(self, node);
    }
    #[inline]
    fn visit_font_face_rule(&mut self, node: &mut FontFaceRule<'a>) {
        rules::walk_font_face_rule(self, node);
    }
    #[inline]
    fn visit_url_source(&mut self, node: &mut UrlSource<'a>) {
        rules::walk_url_source(self, node);
    }
    #[inline]
    fn visit_unicode_range(&mut self, node: &mut UnicodeRange) {
        rules::walk_unicode_range(self, node);
    }
    #[inline]
    fn visit_font_palette_values_rule(&mut self, node: &mut FontPaletteValuesRule<'a>) {
        rules::walk_font_palette_values_rule(self, node);
    }
    #[inline]
    fn visit_override_colors(&mut self, node: &mut OverrideColors<'a>) {
        rules::walk_override_colors(self, node);
    }
    #[inline]
    fn visit_font_feature_values_rule(&mut self, node: &mut FontFeatureValuesRule<'a>) {
        rules::walk_font_feature_values_rule(self, node);
    }
    #[inline]
    fn visit_font_feature_subrule(&mut self, node: &mut FontFeatureSubrule<'a>) {
        rules::walk_font_feature_subrule(self, node);
    }
    #[inline]
    fn visit_font_feature_declaration(&mut self, node: &mut FontFeatureDeclaration<'a>) {
        rules::walk_font_feature_declaration(self, node);
    }
    #[inline]
    fn visit_family_name(&mut self, node: &mut FamilyName<'a>) {
        rules::walk_family_name(self, node);
    }
    #[inline]
    fn visit_page_rule(&mut self, node: &mut PageRule<'a>) {
        rules::walk_page_rule(self, node);
    }
    #[inline]
    fn visit_page_margin_rule(&mut self, node: &mut PageMarginRule<'a>) {
        rules::walk_page_margin_rule(self, node);
    }
    #[inline]
    fn visit_page_selector(&mut self, node: &mut PageSelector<'a>) {
        rules::walk_page_selector(self, node);
    }
    #[inline]
    fn visit_supports_rule(&mut self, node: &mut SupportsRule<'a>) {
        rules::walk_supports_rule(self, node);
    }
    #[inline]
    fn visit_counter_style_rule(&mut self, node: &mut CounterStyleRule<'a>) {
        rules::walk_counter_style_rule(self, node);
    }
    #[inline]
    fn visit_namespace_rule(&mut self, node: &mut NamespaceRule<'a>) {
        rules::walk_namespace_rule(self, node);
    }
    #[inline]
    fn visit_moz_document_rule(&mut self, node: &mut MozDocumentRule<'a>) {
        rules::walk_moz_document_rule(self, node);
    }
    #[inline]
    fn visit_nesting_rule(&mut self, node: &mut NestingRule<'a>) {
        rules::walk_nesting_rule(self, node);
    }
    #[inline]
    fn visit_nested_declarations_rule(&mut self, node: &mut NestedDeclarationsRule<'a>) {
        rules::walk_nested_declarations_rule(self, node);
    }
    #[inline]
    fn visit_viewport_rule(&mut self, node: &mut ViewportRule<'a>) {
        rules::walk_viewport_rule(self, node);
    }
    #[inline]
    fn visit_custom_media_rule(&mut self, node: &mut CustomMediaRule<'a>) {
        rules::walk_custom_media_rule(self, node);
    }
    #[inline]
    fn visit_layer_statement_rule(&mut self, node: &mut LayerStatementRule<'a>) {
        rules::walk_layer_statement_rule(self, node);
    }
    #[inline]
    fn visit_layer_block_rule(&mut self, node: &mut LayerBlockRule<'a>) {
        rules::walk_layer_block_rule(self, node);
    }
    #[inline]
    fn visit_property_rule(&mut self, node: &mut PropertyRule<'a>) {
        rules::walk_property_rule(self, node);
    }
    #[inline]
    fn visit_syntax_component(&mut self, node: &mut SyntaxComponent<'a>) {
        rules::walk_syntax_component(self, node);
    }
    #[inline]
    fn visit_container_rule(&mut self, node: &mut ContainerRule<'a>) {
        rules::walk_container_rule(self, node);
    }
    #[inline]
    fn visit_scope_rule(&mut self, node: &mut ScopeRule<'a>) {
        rules::walk_scope_rule(self, node);
    }
    #[inline]
    fn visit_starting_style_rule(&mut self, node: &mut StartingStyleRule<'a>) {
        rules::walk_starting_style_rule(self, node);
    }
    #[inline]
    fn visit_view_transition_rule(&mut self, node: &mut ViewTransitionRule<'a>) {
        rules::walk_view_transition_rule(self, node);
    }
    #[inline]
    fn visit_position_try_rule(&mut self, node: &mut PositionTryRule<'a>) {
        rules::walk_position_try_rule(self, node);
    }
    #[inline]
    fn visit_unknown_at_rule(&mut self, node: &mut UnknownAtRule<'a>) {
        rules::walk_unknown_at_rule(self, node);
    }
    #[inline]
    fn visit_selector_component(&mut self, node: &mut SelectorComponent<'a>) {
        selector::walk_selector_component(self, node);
    }
    #[inline]
    fn visit_combinator(&mut self, node: &mut Combinator) {
        selector::walk_combinator(self, node);
    }
    #[inline]
    fn visit_attr_selector(&mut self, node: &mut AttrSelector<'a>) {
        selector::walk_attr_selector(self, node);
    }
    #[inline]
    fn visit_namespace_constraint(&mut self, node: &mut NamespaceConstraint<'a>) {
        selector::walk_namespace_constraint(self, node);
    }
    #[inline]
    fn visit_attr_operation(&mut self, node: &mut AttrOperation<'a>) {
        selector::walk_attr_operation(self, node);
    }
    #[inline]
    fn visit_parsed_case_sensitivity(&mut self, node: &mut ParsedCaseSensitivity) {
        selector::walk_parsed_case_sensitivity(self, node);
    }
    #[inline]
    fn visit_attr_selector_operator(&mut self, node: &mut AttrSelectorOperator) {
        selector::walk_attr_selector_operator(self, node);
    }
    #[inline]
    fn visit_nth_type(&mut self, node: &mut NthType) {
        selector::walk_nth_type(self, node);
    }
    #[inline]
    fn visit_nth_selector_data(&mut self, node: &mut NthSelectorData) {
        selector::walk_nth_selector_data(self, node);
    }
    #[inline]
    fn visit_direction(&mut self, node: &mut Direction) {
        selector::walk_direction(self, node);
    }
    #[inline]
    fn visit_pseudo_class(&mut self, node: &mut PseudoClass<'a>) {
        selector::walk_pseudo_class(self, node);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_class(&mut self, node: &mut WebKitScrollbarPseudoClass) {
        selector::walk_web_kit_scrollbar_pseudo_class(self, node);
    }
    #[inline]
    fn visit_pseudo_element(&mut self, node: &mut PseudoElement<'a>) {
        selector::walk_pseudo_element(self, node);
    }
    #[inline]
    fn visit_web_kit_scrollbar_pseudo_element(&mut self, node: &mut WebKitScrollbarPseudoElement) {
        selector::walk_web_kit_scrollbar_pseudo_element(self, node);
    }
    #[inline]
    fn visit_view_transition_part_name(&mut self, node: &mut ViewTransitionPartName<'a>) {
        selector::walk_view_transition_part_name(self, node);
    }
    #[inline]
    fn visit_span(&mut self, node: &mut Span) {
        span::walk_span(self, node);
    }
    #[inline]
    fn visit_token_or_value(&mut self, node: &mut TokenOrValue<'a>) {
        token::walk_token_or_value(self, node);
    }
    #[inline]
    fn visit_unit(&mut self, node: &mut Unit) {
        token::walk_unit(self, node);
    }
    #[inline]
    fn visit_token(&mut self, node: &mut Token<'a>) {
        token::walk_token(self, node);
    }
    #[inline]
    fn visit_specifier(&mut self, node: &mut Specifier<'a>) {
        token::walk_specifier(self, node);
    }
    #[inline]
    fn visit_animation_name(&mut self, node: &mut AnimationName<'a>) {
        token::walk_animation_name(self, node);
    }
    #[inline]
    fn visit_environment_variable_name(&mut self, node: &mut EnvironmentVariableName<'a>) {
        token::walk_environment_variable_name(self, node);
    }
    #[inline]
    fn visit_ua_environment_variable(&mut self, node: &mut UAEnvironmentVariable) {
        token::walk_ua_environment_variable(self, node);
    }
    #[inline]
    fn visit_image(&mut self, node: &mut Image<'a>) {
        values::walk_image(self, node);
    }
    #[inline]
    fn visit_gradient(&mut self, node: &mut Gradient<'a>) {
        values::walk_gradient(self, node);
    }
    #[inline]
    fn visit_web_kit_gradient(&mut self, node: &mut WebKitGradient<'a>) {
        values::walk_web_kit_gradient(self, node);
    }
    #[inline]
    fn visit_line_direction(&mut self, node: &mut LineDirection<'a>) {
        values::walk_line_direction(self, node);
    }
    #[inline]
    fn visit_horizontal_position_keyword(&mut self, node: &mut HorizontalPositionKeyword) {
        values::walk_horizontal_position_keyword(self, node);
    }
    #[inline]
    fn visit_vertical_position_keyword(&mut self, node: &mut VerticalPositionKeyword) {
        values::walk_vertical_position_keyword(self, node);
    }
    #[inline]
    fn visit_gradient_item<D>(&mut self, node: &mut GradientItem<'a, D>)
    where
        D: VisitMutNode<'a, Self>,
    {
        values::walk_gradient_item(self, node);
    }
    #[inline]
    fn visit_dimension_percentage<D>(&mut self, node: &mut DimensionPercentage<'a, D>)
    where
        D: VisitMutNode<'a, Self>,
    {
        values::walk_dimension_percentage(self, node);
    }
    #[inline]
    fn visit_position_component<S>(&mut self, node: &mut PositionComponent<'a, S>)
    where
        S: VisitMutNode<'a, Self>,
    {
        values::walk_position_component(self, node);
    }
    #[inline]
    fn visit_ending_shape(&mut self, node: &mut EndingShape<'a>) {
        values::walk_ending_shape(self, node);
    }
    #[inline]
    fn visit_ellipse(&mut self, node: &mut Ellipse<'a>) {
        values::walk_ellipse(self, node);
    }
    #[inline]
    fn visit_shape_extent(&mut self, node: &mut ShapeExtent) {
        values::walk_shape_extent(self, node);
    }
    #[inline]
    fn visit_circle(&mut self, node: &mut Circle<'a>) {
        values::walk_circle(self, node);
    }
    #[inline]
    fn visit_web_kit_gradient_point_component<S>(
        &mut self,
        node: &mut WebKitGradientPointComponent<'a, S>,
    ) where
        S: VisitMutNode<'a, Self>,
    {
        values::walk_web_kit_gradient_point_component(self, node);
    }
    #[inline]
    fn visit_number_or_percentage(&mut self, node: &mut NumberOrPercentage) {
        values::walk_number_or_percentage(self, node);
    }
    #[inline]
    fn visit_background_size(&mut self, node: &mut BackgroundSize<'a>) {
        values::walk_background_size(self, node);
    }
    #[inline]
    fn visit_length_percentage_or_auto(&mut self, node: &mut LengthPercentageOrAuto<'a>) {
        values::walk_length_percentage_or_auto(self, node);
    }
    #[inline]
    fn visit_background_repeat_keyword(&mut self, node: &mut BackgroundRepeatKeyword) {
        values::walk_background_repeat_keyword(self, node);
    }
    #[inline]
    fn visit_background_attachment(&mut self, node: &mut BackgroundAttachment) {
        values::walk_background_attachment(self, node);
    }
    #[inline]
    fn visit_background_clip(&mut self, node: &mut BackgroundClip) {
        values::walk_background_clip(self, node);
    }
    #[inline]
    fn visit_background_origin(&mut self, node: &mut BackgroundOrigin) {
        values::walk_background_origin(self, node);
    }
    #[inline]
    fn visit_display(&mut self, node: &mut Display<'a>) {
        values::walk_display(self, node);
    }
    #[inline]
    fn visit_display_keyword(&mut self, node: &mut DisplayKeyword) {
        values::walk_display_keyword(self, node);
    }
    #[inline]
    fn visit_display_inside(&mut self, node: &mut DisplayInside) {
        values::walk_display_inside(self, node);
    }
    #[inline]
    fn visit_display_outside(&mut self, node: &mut DisplayOutside) {
        values::walk_display_outside(self, node);
    }
    #[inline]
    fn visit_visibility(&mut self, node: &mut Visibility) {
        values::walk_visibility(self, node);
    }
    #[inline]
    fn visit_size(&mut self, node: &mut Size<'a>) {
        values::walk_size(self, node);
    }
    #[inline]
    fn visit_max_size(&mut self, node: &mut MaxSize<'a>) {
        values::walk_max_size(self, node);
    }
    #[inline]
    fn visit_box_sizing(&mut self, node: &mut BoxSizing) {
        values::walk_box_sizing(self, node);
    }
    #[inline]
    fn visit_overflow_keyword(&mut self, node: &mut OverflowKeyword) {
        values::walk_overflow_keyword(self, node);
    }
    #[inline]
    fn visit_text_overflow(&mut self, node: &mut TextOverflow) {
        values::walk_text_overflow(self, node);
    }
    #[inline]
    fn visit_position_property(&mut self, node: &mut PositionProperty) {
        values::walk_position_property(self, node);
    }
    #[inline]
    fn visit_size_2_d<T>(&mut self, node: &mut Size2D<'a, T>)
    where
        T: VisitMutNode<'a, Self>,
    {
        values::walk_size_2_d(self, node);
    }
    #[inline]
    fn visit_rect<T>(&mut self, node: &mut Rect<'a, T>)
    where
        T: VisitMutNode<'a, Self>,
    {
        values::walk_rect(self, node);
    }
    #[inline]
    fn visit_line_style(&mut self, node: &mut LineStyle) {
        values::walk_line_style(self, node);
    }
    #[inline]
    fn visit_border_side_width(&mut self, node: &mut BorderSideWidth<'a>) {
        values::walk_border_side_width(self, node);
    }
    #[inline]
    fn visit_length_or_number(&mut self, node: &mut LengthOrNumber<'a>) {
        values::walk_length_or_number(self, node);
    }
    #[inline]
    fn visit_border_image_repeat_keyword(&mut self, node: &mut BorderImageRepeatKeyword) {
        values::walk_border_image_repeat_keyword(self, node);
    }
    #[inline]
    fn visit_border_image_side_width(&mut self, node: &mut BorderImageSideWidth<'a>) {
        values::walk_border_image_side_width(self, node);
    }
    #[inline]
    fn visit_outline_style(&mut self, node: &mut OutlineStyle) {
        values::walk_outline_style(self, node);
    }
    #[inline]
    fn visit_flex_direction(&mut self, node: &mut FlexDirection) {
        values::walk_flex_direction(self, node);
    }
    #[inline]
    fn visit_flex_wrap(&mut self, node: &mut FlexWrap) {
        values::walk_flex_wrap(self, node);
    }
    #[inline]
    fn visit_align_content(&mut self, node: &mut AlignContent) {
        values::walk_align_content(self, node);
    }
    #[inline]
    fn visit_baseline_position(&mut self, node: &mut BaselinePosition) {
        values::walk_baseline_position(self, node);
    }
    #[inline]
    fn visit_content_distribution(&mut self, node: &mut ContentDistribution) {
        values::walk_content_distribution(self, node);
    }
    #[inline]
    fn visit_overflow_position(&mut self, node: &mut OverflowPosition) {
        values::walk_overflow_position(self, node);
    }
    #[inline]
    fn visit_content_position(&mut self, node: &mut ContentPosition) {
        values::walk_content_position(self, node);
    }
    #[inline]
    fn visit_justify_content(&mut self, node: &mut JustifyContent) {
        values::walk_justify_content(self, node);
    }
    #[inline]
    fn visit_align_self(&mut self, node: &mut AlignSelf) {
        values::walk_align_self(self, node);
    }
    #[inline]
    fn visit_self_position(&mut self, node: &mut SelfPosition) {
        values::walk_self_position(self, node);
    }
    #[inline]
    fn visit_justify_self(&mut self, node: &mut JustifySelf) {
        values::walk_justify_self(self, node);
    }
    #[inline]
    fn visit_align_items(&mut self, node: &mut AlignItems) {
        values::walk_align_items(self, node);
    }
    #[inline]
    fn visit_justify_items(&mut self, node: &mut JustifyItems) {
        values::walk_justify_items(self, node);
    }
    #[inline]
    fn visit_legacy_justify(&mut self, node: &mut LegacyJustify) {
        values::walk_legacy_justify(self, node);
    }
    #[inline]
    fn visit_gap_value(&mut self, node: &mut GapValue<'a>) {
        values::walk_gap_value(self, node);
    }
    #[inline]
    fn visit_box_orient(&mut self, node: &mut BoxOrient) {
        values::walk_box_orient(self, node);
    }
    #[inline]
    fn visit_box_direction(&mut self, node: &mut BoxDirection) {
        values::walk_box_direction(self, node);
    }
    #[inline]
    fn visit_box_align(&mut self, node: &mut BoxAlign) {
        values::walk_box_align(self, node);
    }
    #[inline]
    fn visit_box_pack(&mut self, node: &mut BoxPack) {
        values::walk_box_pack(self, node);
    }
    #[inline]
    fn visit_box_lines(&mut self, node: &mut BoxLines) {
        values::walk_box_lines(self, node);
    }
    #[inline]
    fn visit_flex_pack(&mut self, node: &mut FlexPack) {
        values::walk_flex_pack(self, node);
    }
    #[inline]
    fn visit_flex_item_align(&mut self, node: &mut FlexItemAlign) {
        values::walk_flex_item_align(self, node);
    }
    #[inline]
    fn visit_flex_line_pack(&mut self, node: &mut FlexLinePack) {
        values::walk_flex_line_pack(self, node);
    }
    #[inline]
    fn visit_track_sizing(&mut self, node: &mut TrackSizing<'a>) {
        values::walk_track_sizing(self, node);
    }
    #[inline]
    fn visit_track_list_item(&mut self, node: &mut TrackListItem<'a>) {
        values::walk_track_list_item(self, node);
    }
    #[inline]
    fn visit_track_size(&mut self, node: &mut TrackSize<'a>) {
        values::walk_track_size(self, node);
    }
    #[inline]
    fn visit_track_breadth(&mut self, node: &mut TrackBreadth<'a>) {
        values::walk_track_breadth(self, node);
    }
    #[inline]
    fn visit_repeat_count(&mut self, node: &mut RepeatCount) {
        values::walk_repeat_count(self, node);
    }
    #[inline]
    fn visit_auto_flow_direction(&mut self, node: &mut AutoFlowDirection) {
        values::walk_auto_flow_direction(self, node);
    }
    #[inline]
    fn visit_grid_template_areas(&mut self, node: &mut GridTemplateAreas<'a>) {
        values::walk_grid_template_areas(self, node);
    }
    #[inline]
    fn visit_grid_line(&mut self, node: &mut GridLine<'a>) {
        values::walk_grid_line(self, node);
    }
    #[inline]
    fn visit_font_weight(&mut self, node: &mut FontWeight<'a>) {
        values::walk_font_weight(self, node);
    }
    #[inline]
    fn visit_absolute_font_weight(&mut self, node: &mut AbsoluteFontWeight) {
        values::walk_absolute_font_weight(self, node);
    }
    #[inline]
    fn visit_font_size(&mut self, node: &mut FontSize<'a>) {
        values::walk_font_size(self, node);
    }
    #[inline]
    fn visit_absolute_font_size(&mut self, node: &mut AbsoluteFontSize) {
        values::walk_absolute_font_size(self, node);
    }
    #[inline]
    fn visit_relative_font_size(&mut self, node: &mut RelativeFontSize) {
        values::walk_relative_font_size(self, node);
    }
    #[inline]
    fn visit_font_stretch(&mut self, node: &mut FontStretch) {
        values::walk_font_stretch(self, node);
    }
    #[inline]
    fn visit_font_stretch_keyword(&mut self, node: &mut FontStretchKeyword) {
        values::walk_font_stretch_keyword(self, node);
    }
    #[inline]
    fn visit_font_family(&mut self, node: &mut FontFamily<'a>) {
        values::walk_font_family(self, node);
    }
    #[inline]
    fn visit_generic_font_family(&mut self, node: &mut GenericFontFamily) {
        values::walk_generic_font_family(self, node);
    }
    #[inline]
    fn visit_font_style(&mut self, node: &mut FontStyle<'a>) {
        values::walk_font_style(self, node);
    }
    #[inline]
    fn visit_font_variant_caps(&mut self, node: &mut FontVariantCaps) {
        values::walk_font_variant_caps(self, node);
    }
    #[inline]
    fn visit_line_height(&mut self, node: &mut LineHeight<'a>) {
        values::walk_line_height(self, node);
    }
    #[inline]
    fn visit_vertical_align(&mut self, node: &mut VerticalAlign<'a>) {
        values::walk_vertical_align(self, node);
    }
    #[inline]
    fn visit_vertical_align_keyword(&mut self, node: &mut VerticalAlignKeyword) {
        values::walk_vertical_align_keyword(self, node);
    }
    #[inline]
    fn visit_easing_function(&mut self, node: &mut EasingFunction) {
        values::walk_easing_function(self, node);
    }
    #[inline]
    fn visit_step_position(&mut self, node: &mut StepPosition) {
        values::walk_step_position(self, node);
    }
    #[inline]
    fn visit_animation_iteration_count(&mut self, node: &mut AnimationIterationCount) {
        values::walk_animation_iteration_count(self, node);
    }
    #[inline]
    fn visit_animation_direction(&mut self, node: &mut AnimationDirection) {
        values::walk_animation_direction(self, node);
    }
    #[inline]
    fn visit_animation_play_state(&mut self, node: &mut AnimationPlayState) {
        values::walk_animation_play_state(self, node);
    }
    #[inline]
    fn visit_animation_fill_mode(&mut self, node: &mut AnimationFillMode) {
        values::walk_animation_fill_mode(self, node);
    }
    #[inline]
    fn visit_animation_composition(&mut self, node: &mut AnimationComposition) {
        values::walk_animation_composition(self, node);
    }
    #[inline]
    fn visit_animation_timeline(&mut self, node: &mut AnimationTimeline<'a>) {
        values::walk_animation_timeline(self, node);
    }
    #[inline]
    fn visit_scroll_axis(&mut self, node: &mut ScrollAxis) {
        values::walk_scroll_axis(self, node);
    }
    #[inline]
    fn visit_scroller(&mut self, node: &mut Scroller) {
        values::walk_scroller(self, node);
    }
    #[inline]
    fn visit_animation_attachment_range(&mut self, node: &mut AnimationAttachmentRange<'a>) {
        values::walk_animation_attachment_range(self, node);
    }
    #[inline]
    fn visit_timeline_range_name(&mut self, node: &mut TimelineRangeName) {
        values::walk_timeline_range_name(self, node);
    }
    #[inline]
    fn visit_transform(&mut self, node: &mut Transform<'a>) {
        values::walk_transform(self, node);
    }
    #[inline]
    fn visit_transform_style(&mut self, node: &mut TransformStyle) {
        values::walk_transform_style(self, node);
    }
    #[inline]
    fn visit_transform_box(&mut self, node: &mut TransformBox) {
        values::walk_transform_box(self, node);
    }
    #[inline]
    fn visit_backface_visibility(&mut self, node: &mut BackfaceVisibility) {
        values::walk_backface_visibility(self, node);
    }
    #[inline]
    fn visit_perspective(&mut self, node: &mut Perspective<'a>) {
        values::walk_perspective(self, node);
    }
    #[inline]
    fn visit_translate(&mut self, node: &mut Translate<'a>) {
        values::walk_translate(self, node);
    }
    #[inline]
    fn visit_scale(&mut self, node: &mut Scale<'a>) {
        values::walk_scale(self, node);
    }
    #[inline]
    fn visit_text_transform_case(&mut self, node: &mut TextTransformCase) {
        values::walk_text_transform_case(self, node);
    }
    #[inline]
    fn visit_white_space(&mut self, node: &mut WhiteSpace) {
        values::walk_white_space(self, node);
    }
    #[inline]
    fn visit_word_break(&mut self, node: &mut WordBreak) {
        values::walk_word_break(self, node);
    }
    #[inline]
    fn visit_line_break(&mut self, node: &mut LineBreak) {
        values::walk_line_break(self, node);
    }
    #[inline]
    fn visit_hyphens(&mut self, node: &mut Hyphens) {
        values::walk_hyphens(self, node);
    }
    #[inline]
    fn visit_overflow_wrap(&mut self, node: &mut OverflowWrap) {
        values::walk_overflow_wrap(self, node);
    }
    #[inline]
    fn visit_text_align(&mut self, node: &mut TextAlign) {
        values::walk_text_align(self, node);
    }
    #[inline]
    fn visit_text_align_last(&mut self, node: &mut TextAlignLast) {
        values::walk_text_align_last(self, node);
    }
    #[inline]
    fn visit_text_justify(&mut self, node: &mut TextJustify) {
        values::walk_text_justify(self, node);
    }
    #[inline]
    fn visit_spacing(&mut self, node: &mut Spacing<'a>) {
        values::walk_spacing(self, node);
    }
    #[inline]
    fn visit_text_decoration_line(&mut self, node: &mut TextDecorationLine<'a>) {
        values::walk_text_decoration_line(self, node);
    }
    #[inline]
    fn visit_exclusive_text_decoration_line(&mut self, node: &mut ExclusiveTextDecorationLine) {
        values::walk_exclusive_text_decoration_line(self, node);
    }
    #[inline]
    fn visit_other_text_decoration_line(&mut self, node: &mut OtherTextDecorationLine) {
        values::walk_other_text_decoration_line(self, node);
    }
    #[inline]
    fn visit_text_decoration_style(&mut self, node: &mut TextDecorationStyle) {
        values::walk_text_decoration_style(self, node);
    }
    #[inline]
    fn visit_text_decoration_thickness(&mut self, node: &mut TextDecorationThickness<'a>) {
        values::walk_text_decoration_thickness(self, node);
    }
    #[inline]
    fn visit_text_decoration_skip_ink(&mut self, node: &mut TextDecorationSkipInk) {
        values::walk_text_decoration_skip_ink(self, node);
    }
    #[inline]
    fn visit_text_emphasis_style(&mut self, node: &mut TextEmphasisStyle<'a>) {
        values::walk_text_emphasis_style(self, node);
    }
    #[inline]
    fn visit_text_emphasis_fill_mode(&mut self, node: &mut TextEmphasisFillMode) {
        values::walk_text_emphasis_fill_mode(self, node);
    }
    #[inline]
    fn visit_text_emphasis_shape(&mut self, node: &mut TextEmphasisShape) {
        values::walk_text_emphasis_shape(self, node);
    }
    #[inline]
    fn visit_text_emphasis_position_horizontal(
        &mut self,
        node: &mut TextEmphasisPositionHorizontal,
    ) {
        values::walk_text_emphasis_position_horizontal(self, node);
    }
    #[inline]
    fn visit_text_emphasis_position_vertical(&mut self, node: &mut TextEmphasisPositionVertical) {
        values::walk_text_emphasis_position_vertical(self, node);
    }
    #[inline]
    fn visit_text_size_adjust(&mut self, node: &mut TextSizeAdjust) {
        values::walk_text_size_adjust(self, node);
    }
    #[inline]
    fn visit_text_direction(&mut self, node: &mut TextDirection) {
        values::walk_text_direction(self, node);
    }
    #[inline]
    fn visit_unicode_bidi(&mut self, node: &mut UnicodeBidi) {
        values::walk_unicode_bidi(self, node);
    }
    #[inline]
    fn visit_box_decoration_break(&mut self, node: &mut BoxDecorationBreak) {
        values::walk_box_decoration_break(self, node);
    }
    #[inline]
    fn visit_resize(&mut self, node: &mut Resize) {
        values::walk_resize(self, node);
    }
    #[inline]
    fn visit_cursor_keyword(&mut self, node: &mut CursorKeyword) {
        values::walk_cursor_keyword(self, node);
    }
    #[inline]
    fn visit_color_or_auto(&mut self, node: &mut ColorOrAuto<'a>) {
        values::walk_color_or_auto(self, node);
    }
    #[inline]
    fn visit_caret_shape(&mut self, node: &mut CaretShape) {
        values::walk_caret_shape(self, node);
    }
    #[inline]
    fn visit_user_select(&mut self, node: &mut UserSelect) {
        values::walk_user_select(self, node);
    }
    #[inline]
    fn visit_appearance(&mut self, node: &mut Appearance<'a>) {
        values::walk_appearance(self, node);
    }
    #[inline]
    fn visit_list_style_type(&mut self, node: &mut ListStyleType<'a>) {
        values::walk_list_style_type(self, node);
    }
    #[inline]
    fn visit_counter_style(&mut self, node: &mut CounterStyle<'a>) {
        values::walk_counter_style(self, node);
    }
    #[inline]
    fn visit_symbols_type(&mut self, node: &mut SymbolsType) {
        values::walk_symbols_type(self, node);
    }
    #[inline]
    fn visit_predefined_counter_style(&mut self, node: &mut PredefinedCounterStyle) {
        values::walk_predefined_counter_style(self, node);
    }
    #[inline]
    fn visit_symbol(&mut self, node: &mut Symbol<'a>) {
        values::walk_symbol(self, node);
    }
    #[inline]
    fn visit_list_style_position(&mut self, node: &mut ListStylePosition) {
        values::walk_list_style_position(self, node);
    }
    #[inline]
    fn visit_marker_side(&mut self, node: &mut MarkerSide) {
        values::walk_marker_side(self, node);
    }
    #[inline]
    fn visit_svg_paint(&mut self, node: &mut SVGPaint<'a>) {
        values::walk_svg_paint(self, node);
    }
    #[inline]
    fn visit_svg_paint_fallback(&mut self, node: &mut SVGPaintFallback<'a>) {
        values::walk_svg_paint_fallback(self, node);
    }
    #[inline]
    fn visit_fill_rule(&mut self, node: &mut FillRule) {
        values::walk_fill_rule(self, node);
    }
    #[inline]
    fn visit_stroke_linecap(&mut self, node: &mut StrokeLinecap) {
        values::walk_stroke_linecap(self, node);
    }
    #[inline]
    fn visit_stroke_linejoin(&mut self, node: &mut StrokeLinejoin) {
        values::walk_stroke_linejoin(self, node);
    }
    #[inline]
    fn visit_stroke_dasharray(&mut self, node: &mut StrokeDasharray<'a>) {
        values::walk_stroke_dasharray(self, node);
    }
    #[inline]
    fn visit_marker(&mut self, node: &mut Marker<'a>) {
        values::walk_marker(self, node);
    }
    #[inline]
    fn visit_color_interpolation(&mut self, node: &mut ColorInterpolation) {
        values::walk_color_interpolation(self, node);
    }
    #[inline]
    fn visit_color_rendering(&mut self, node: &mut ColorRendering) {
        values::walk_color_rendering(self, node);
    }
    #[inline]
    fn visit_shape_rendering(&mut self, node: &mut ShapeRendering) {
        values::walk_shape_rendering(self, node);
    }
    #[inline]
    fn visit_text_rendering(&mut self, node: &mut TextRendering) {
        values::walk_text_rendering(self, node);
    }
    #[inline]
    fn visit_image_rendering(&mut self, node: &mut ImageRendering) {
        values::walk_image_rendering(self, node);
    }
    #[inline]
    fn visit_clip_path(&mut self, node: &mut ClipPath<'a>) {
        values::walk_clip_path(self, node);
    }
    #[inline]
    fn visit_geometry_box(&mut self, node: &mut GeometryBox) {
        values::walk_geometry_box(self, node);
    }
    #[inline]
    fn visit_basic_shape(&mut self, node: &mut BasicShape<'a>) {
        values::walk_basic_shape(self, node);
    }
    #[inline]
    fn visit_shape_radius(&mut self, node: &mut ShapeRadius<'a>) {
        values::walk_shape_radius(self, node);
    }
    #[inline]
    fn visit_mask_mode(&mut self, node: &mut MaskMode) {
        values::walk_mask_mode(self, node);
    }
    #[inline]
    fn visit_mask_clip(&mut self, node: &mut MaskClip) {
        values::walk_mask_clip(self, node);
    }
    #[inline]
    fn visit_mask_composite(&mut self, node: &mut MaskComposite) {
        values::walk_mask_composite(self, node);
    }
    #[inline]
    fn visit_mask_type(&mut self, node: &mut MaskType) {
        values::walk_mask_type(self, node);
    }
    #[inline]
    fn visit_mask_border_mode(&mut self, node: &mut MaskBorderMode) {
        values::walk_mask_border_mode(self, node);
    }
    #[inline]
    fn visit_web_kit_mask_composite(&mut self, node: &mut WebKitMaskComposite) {
        values::walk_web_kit_mask_composite(self, node);
    }
    #[inline]
    fn visit_web_kit_mask_source_type(&mut self, node: &mut WebKitMaskSourceType) {
        values::walk_web_kit_mask_source_type(self, node);
    }
    #[inline]
    fn visit_filter_list(&mut self, node: &mut FilterList<'a>) {
        values::walk_filter_list(self, node);
    }
    #[inline]
    fn visit_filter(&mut self, node: &mut Filter<'a>) {
        values::walk_filter(self, node);
    }
    #[inline]
    fn visit_z_index(&mut self, node: &mut ZIndex) {
        values::walk_z_index(self, node);
    }
    #[inline]
    fn visit_container_type(&mut self, node: &mut ContainerType) {
        values::walk_container_type(self, node);
    }
    #[inline]
    fn visit_container_name_list(&mut self, node: &mut ContainerNameList<'a>) {
        values::walk_container_name_list(self, node);
    }
    #[inline]
    fn visit_view_transition_name(&mut self, node: &mut ViewTransitionName<'a>) {
        values::walk_view_transition_name(self, node);
    }
    #[inline]
    fn visit_none_or_custom_ident_list(&mut self, node: &mut NoneOrCustomIdentList<'a>) {
        values::walk_none_or_custom_ident_list(self, node);
    }
    #[inline]
    fn visit_view_transition_group(&mut self, node: &mut ViewTransitionGroup<'a>) {
        values::walk_view_transition_group(self, node);
    }
    #[inline]
    fn visit_print_color_adjust(&mut self, node: &mut PrintColorAdjust) {
        values::walk_print_color_adjust(self, node);
    }
    #[inline]
    fn visit_css_wide_keyword(&mut self, node: &mut CSSWideKeyword) {
        values::walk_css_wide_keyword(self, node);
    }
    #[inline]
    fn visit_custom_property_name(&mut self, node: &mut CustomPropertyName<'a>) {
        values::walk_custom_property_name(self, node);
    }
    #[inline]
    fn visit_media_feature(&mut self, node: &mut MediaFeature<'a>) {
        media::walk_media_feature(self, node);
    }
    #[inline]
    fn visit_container_size_feature(&mut self, node: &mut ContainerSizeFeature<'a>) {
        rules::walk_container_size_feature(self, node);
    }
    #[inline]
    fn visit_scroll_state_feature(&mut self, node: &mut ScrollStateFeature<'a>) {
        rules::walk_scroll_state_feature(self, node);
    }
    #[inline]
    fn visit_selector_list(&mut self, node: &mut SelectorList<'a>) {
        selector::walk_selector_list(self, node);
    }
    #[inline]
    fn visit_selector(&mut self, node: &mut Selector<'a>) {
        selector::walk_selector(self, node);
    }
    #[inline]
    fn visit_length_percentage(&mut self, node: &mut LengthPercentage<'a>) {
        values::walk_length_percentage(self, node);
    }
    #[inline]
    fn visit_angle_percentage(&mut self, node: &mut AnglePercentage<'a>) {
        values::walk_angle_percentage(self, node);
    }
    #[inline]
    fn visit_animation_range_start(&mut self, node: &mut AnimationRangeStart<'a>) {
        values::walk_animation_range_start(self, node);
    }
    #[inline]
    fn visit_animation_range_end(&mut self, node: &mut AnimationRangeEnd<'a>) {
        values::walk_animation_range_end(self, node);
    }
    #[inline]
    fn visit_declaration(&mut self, node: &mut Declaration<'a>) {
        walk_declaration(self, node);
    }
    #[inline]
    fn visit_property_id(&mut self, node: &mut PropertyId<'a>) {
        walk_property_id(self, node);
    }
    #[inline]
    fn visit_vendor_prefix(&mut self, node: &mut VendorPrefix) {
        walk_vendor_prefix(self, node);
    }
}
#[doc(hidden)]
pub trait VisitMutNode<'a, VisitorT: ?Sized + VisitMut<'a>> {
    fn visit_node(&mut self, visitor: &mut VisitorT);
}
macro_rules! impl_leaf_visit_mut_node {
    ($($ty:ty),+ $(,)?) => {
        $(impl < 'a, VisitorT : ? Sized + VisitMut < 'a >> VisitMutNode < 'a, VisitorT >
        for $ty { fn visit_node(& mut self, _visitor : & mut VisitorT) {} })+
    };
}
impl_leaf_visit_mut_node!(bool, char, f32, i32, u8, u16, u32, usize);
impl<'a, VisitorT, T> VisitMutNode<'a, VisitorT> for rocketcss_allocator::boxed::Box<'a, T>
where
    VisitorT: ?Sized + VisitMut<'a>,
    T: ?Sized + VisitMutNode<'a, VisitorT>,
{
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        self.as_mut().visit_node(visitor);
    }
}
impl<'a, VisitorT, T> VisitMutNode<'a, VisitorT> for rocketcss_allocator::vec::Vec<'a, T>
where
    VisitorT: ?Sized + VisitMut<'a>,
    T: VisitMutNode<'a, VisitorT>,
{
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        for value in self {
            value.visit_node(visitor);
        }
    }
}
impl<'a, VisitorT, T> VisitMutNode<'a, VisitorT> for Option<T>
where
    VisitorT: ?Sized + VisitMut<'a>,
    T: VisitMutNode<'a, VisitorT>,
{
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        if let Some(value) = self {
            value.visit_node(visitor);
        }
    }
}
impl<'a, VisitorT: ?Sized + VisitMut<'a>> VisitMutNode<'a, VisitorT> for &'a str {
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_str(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CssColor<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_css_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for RGBA
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_rgba(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LABColor
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_lab_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PredefinedColor
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_predefined_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FloatColor
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_float_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LightDark<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_light_dark(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SystemColor
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_system_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for UnresolvedColor<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unresolved_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CssRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_css_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Length<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_length(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LengthUnit
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_length_unit(self);
    }
}
impl<'a, V, VisitorT> VisitMutNode<'a, VisitorT> for Calc<'a, V>
where
    VisitorT: ?Sized + VisitMut<'a>,
    V: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_calc(self);
    }
}
impl<'a, V, VisitorT> VisitMutNode<'a, VisitorT> for MathFunction<'a, V>
where
    VisitorT: ?Sized + VisitMut<'a>,
    V: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_math_function(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for RoundingStrategy
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_rounding_strategy(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Resolution
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_resolution(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Ratio
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_ratio(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Angle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_angle(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Time
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_time(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MediaCondition<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_condition(self);
    }
}
impl<'a, FeatureId, VisitorT> VisitMutNode<'a, VisitorT> for QueryFeature<'a, FeatureId>
where
    VisitorT: ?Sized + VisitMut<'a>,
    FeatureId: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_query_feature(self);
    }
}
impl<'a, FeatureId, VisitorT> VisitMutNode<'a, VisitorT> for MediaFeatureName<'a, FeatureId>
where
    VisitorT: ?Sized + VisitMut<'a>,
    FeatureId: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_name(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MediaFeatureId
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_id(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MediaFeatureValue<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_value(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MediaFeatureComparison
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_feature_comparison(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Operator
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_operator(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MediaType<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_type(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Qualifier
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_qualifier(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SupportsCondition<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_supports_condition(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BlendMode
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_blend_mode(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for KeyframeSelector<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_keyframe_selector(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for KeyframesName<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_keyframes_name(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFaceProperty<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_face_property(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Source<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_source(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFormat<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_format(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontTechnology
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_technology(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFaceStyle<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_face_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontPaletteValuesProperty<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_palette_values_property(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BasePalette
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_base_palette(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFeatureSubruleType
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_subrule_type(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PageMarginBox
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_margin_box(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PagePseudoClass
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_pseudo_class(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ParsedComponent<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_parsed_component(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Multiplier
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_multiplier(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SyntaxString<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_syntax_string(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SyntaxComponentKind<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_syntax_component_kind(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ContainerCondition<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container_condition(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ContainerSizeFeatureId
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container_size_feature_id(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for StyleQuery<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_style_query(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollStateQuery<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_state_query(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollStateFeatureId
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_state_feature_id(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ViewTransitionProperty<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_property(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Navigation
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_navigation(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for DefaultAtRule
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_default_at_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for StyleSheet<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_style_sheet(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MediaRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MediaList<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_list(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MediaQuery<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_media_query(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LengthValue
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_length_value(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for EnvironmentVariable<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_environment_variable(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Url<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_url(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Variable<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_variable(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for DashedIdentReference<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_dashed_ident_reference(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Function<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_function(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ImportRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_import_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for StyleRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_style_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for DeclarationBlock<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_declaration_block(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Position<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WebKitGradientPoint<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient_point(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WebKitColorStop<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_color_stop(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ImageSet<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_image_set(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ImageSetOption<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_image_set_option(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BackgroundPosition<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BackgroundRepeat
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_repeat(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Background<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BoxShadow<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_shadow(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AspectRatio<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_aspect_ratio(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Overflow
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_overflow(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for InsetBlock<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_inset_block(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for InsetInline<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_inset_inline(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Inset<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_inset(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderRadius<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_radius(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderImageRepeat
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_image_repeat(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderImageSlice<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_image_slice(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderImage<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_image(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderColor<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderStyle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderWidth<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_width(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderBlockColor<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_block_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderBlockStyle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_block_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderBlockWidth<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_block_width(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderInlineColor<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_color(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderInlineStyle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderInlineWidth<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_inline_width(self);
    }
}
impl<'a, S, VisitorT> VisitMutNode<'a, VisitorT> for GenericBorder<'a, S>
where
    VisitorT: ?Sized + VisitMut<'a>,
    S: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_generic_border(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FlexFlow
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex_flow(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Flex<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PlaceContent<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_place_content(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PlaceSelf<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_place_self(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PlaceItems<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_place_items(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Gap<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_gap(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TrackRepeat<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_track_repeat(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GridAutoFlow
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_auto_flow(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GridTemplate<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_template(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Grid<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GridRow<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_row(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GridColumn<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_column(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GridArea<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_area(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MarginBlock<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_margin_block(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MarginInline<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_margin_inline(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Margin<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_margin(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PaddingBlock<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_padding_block(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PaddingInline<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_padding_inline(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Padding<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_padding(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollMarginBlock<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin_block(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollMarginInline<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin_inline(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollMargin<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_margin(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollPaddingBlock<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding_block(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollPaddingInline<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding_inline(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollPadding<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_padding(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Font<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Transition<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_transition(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollTimeline
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_timeline(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ViewTimeline<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_timeline(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationRange<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_range(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Animation<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MatrixForFloat
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_matrix_for_float(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Matrix3DForFloat
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_matrix_3_d_for_float(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Rotate<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_rotate(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextTransform
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_transform(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextIndent<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_indent(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextDecoration<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextEmphasis<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextEmphasisPosition
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextShadow<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_shadow(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Cursor<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_cursor(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CursorImage<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_cursor_image(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Caret<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_caret(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ListStyle<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_list_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Composes<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_composes(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for InsetRect<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_inset_rect(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CircleShape<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_circle_shape(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for EllipseShape<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_ellipse_shape(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Polygon<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_polygon(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Point<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_point(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Mask<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MaskBorder<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask_border(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for DropShadow<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_drop_shadow(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Container<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ColorScheme
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_color_scheme(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for UnparsedProperty<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unparsed_property(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CustomProperty<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_custom_property(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ViewTransitionPartSelector<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_part_selector(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for KeyframesRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_keyframes_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Keyframe<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_keyframe(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TimelineRangePercentage
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_timeline_range_percentage(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFaceRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_face_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for UrlSource<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_url_source(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for UnicodeRange
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unicode_range(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontPaletteValuesRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_palette_values_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for OverrideColors<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_override_colors(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFeatureValuesRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_values_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFeatureSubrule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_subrule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFeatureDeclaration<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_feature_declaration(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FamilyName<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_family_name(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PageRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PageMarginRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_margin_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PageSelector<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_page_selector(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SupportsRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_supports_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CounterStyleRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_counter_style_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for NamespaceRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_namespace_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MozDocumentRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_moz_document_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for NestingRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_nesting_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for NestedDeclarationsRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_nested_declarations_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ViewportRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_viewport_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CustomMediaRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_custom_media_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LayerStatementRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_layer_statement_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LayerBlockRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_layer_block_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PropertyRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_property_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SyntaxComponent<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_syntax_component(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ContainerRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScopeRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scope_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for StartingStyleRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_starting_style_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ViewTransitionRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PositionTryRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_position_try_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for UnknownAtRule<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unknown_at_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SelectorComponent<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_selector_component(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Combinator
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_combinator(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AttrSelector<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_attr_selector(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for NamespaceConstraint<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_namespace_constraint(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AttrOperation<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_attr_operation(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ParsedCaseSensitivity
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_parsed_case_sensitivity(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AttrSelectorOperator
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_attr_selector_operator(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for NthType
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_nth_type(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for NthSelectorData
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_nth_selector_data(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Direction
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_direction(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PseudoClass<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_pseudo_class(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WebKitScrollbarPseudoClass
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_scrollbar_pseudo_class(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PseudoElement<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_pseudo_element(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WebKitScrollbarPseudoElement
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_scrollbar_pseudo_element(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ViewTransitionPartName<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_part_name(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Span
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_span(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TokenOrValue<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_token_or_value(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Unit
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unit(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Token<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_token(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Specifier<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_specifier(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationName<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_name(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for EnvironmentVariableName<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_environment_variable_name(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for UAEnvironmentVariable
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_ua_environment_variable(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Image<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_image(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Gradient<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_gradient(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WebKitGradient<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LineDirection<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_line_direction(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for HorizontalPositionKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_horizontal_position_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for VerticalPositionKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_vertical_position_keyword(self);
    }
}
impl<'a, D, VisitorT> VisitMutNode<'a, VisitorT> for GradientItem<'a, D>
where
    VisitorT: ?Sized + VisitMut<'a>,
    D: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_gradient_item(self);
    }
}
impl<'a, D, VisitorT> VisitMutNode<'a, VisitorT> for DimensionPercentage<'a, D>
where
    VisitorT: ?Sized + VisitMut<'a>,
    D: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_dimension_percentage(self);
    }
}
impl<'a, S, VisitorT> VisitMutNode<'a, VisitorT> for PositionComponent<'a, S>
where
    VisitorT: ?Sized + VisitMut<'a>,
    S: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_position_component(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for EndingShape<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_ending_shape(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Ellipse<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_ellipse(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ShapeExtent
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_shape_extent(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Circle<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_circle(self);
    }
}
impl<'a, S, VisitorT> VisitMutNode<'a, VisitorT> for WebKitGradientPointComponent<'a, S>
where
    VisitorT: ?Sized + VisitMut<'a>,
    S: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_gradient_point_component(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for NumberOrPercentage
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_number_or_percentage(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BackgroundSize<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_size(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LengthPercentageOrAuto<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_length_percentage_or_auto(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BackgroundRepeatKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_repeat_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BackgroundAttachment
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_attachment(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BackgroundClip
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_clip(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BackgroundOrigin
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_background_origin(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Display<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_display(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for DisplayKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_display_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for DisplayInside
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_display_inside(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for DisplayOutside
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_display_outside(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Visibility
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_visibility(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Size<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_size(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MaxSize<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_max_size(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BoxSizing
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_sizing(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for OverflowKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_overflow_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextOverflow
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_overflow(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PositionProperty
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_position_property(self);
    }
}
impl<'a, T, VisitorT> VisitMutNode<'a, VisitorT> for Size2D<'a, T>
where
    VisitorT: ?Sized + VisitMut<'a>,
    T: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_size_2_d(self);
    }
}
impl<'a, T, VisitorT> VisitMutNode<'a, VisitorT> for Rect<'a, T>
where
    VisitorT: ?Sized + VisitMut<'a>,
    T: VisitMutNode<'a, VisitorT>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_rect(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LineStyle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_line_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderSideWidth<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_side_width(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LengthOrNumber<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_length_or_number(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderImageRepeatKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_image_repeat_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BorderImageSideWidth<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_border_image_side_width(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for OutlineStyle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_outline_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FlexDirection
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex_direction(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FlexWrap
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex_wrap(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AlignContent
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_align_content(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BaselinePosition
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_baseline_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ContentDistribution
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_content_distribution(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for OverflowPosition
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_overflow_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ContentPosition
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_content_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for JustifyContent
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_justify_content(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AlignSelf
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_align_self(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SelfPosition
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_self_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for JustifySelf
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_justify_self(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AlignItems
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_align_items(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for JustifyItems
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_justify_items(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LegacyJustify
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_legacy_justify(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GapValue<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_gap_value(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BoxOrient
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_orient(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BoxDirection
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_direction(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BoxAlign
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_align(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BoxPack
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_pack(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BoxLines
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_lines(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FlexPack
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex_pack(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FlexItemAlign
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex_item_align(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FlexLinePack
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_flex_line_pack(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TrackSizing<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_track_sizing(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TrackListItem<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_track_list_item(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TrackSize<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_track_size(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TrackBreadth<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_track_breadth(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for RepeatCount
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_repeat_count(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AutoFlowDirection
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_auto_flow_direction(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GridTemplateAreas<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_template_areas(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GridLine<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_grid_line(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontWeight<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_weight(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AbsoluteFontWeight
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_absolute_font_weight(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontSize<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_size(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AbsoluteFontSize
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_absolute_font_size(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for RelativeFontSize
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_relative_font_size(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontStretch
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_stretch(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontStretchKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_stretch_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontFamily<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_family(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GenericFontFamily
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_generic_font_family(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontStyle<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FontVariantCaps
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_font_variant_caps(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LineHeight<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_line_height(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for VerticalAlign<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_vertical_align(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for VerticalAlignKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_vertical_align_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for EasingFunction
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_easing_function(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for StepPosition
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_step_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationIterationCount
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_iteration_count(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationDirection
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_direction(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationPlayState
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_play_state(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationFillMode
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_fill_mode(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationComposition
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_composition(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationTimeline<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_timeline(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ScrollAxis
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroll_axis(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Scroller
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scroller(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for AnimationAttachmentRange<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_animation_attachment_range(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TimelineRangeName
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_timeline_range_name(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Transform<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_transform(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TransformStyle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_transform_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TransformBox
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_transform_box(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BackfaceVisibility
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_backface_visibility(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Perspective<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_perspective(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Translate<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_translate(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Scale<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_scale(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextTransformCase
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_transform_case(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WhiteSpace
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_white_space(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WordBreak
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_word_break(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for LineBreak
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_line_break(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Hyphens
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_hyphens(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for OverflowWrap
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_overflow_wrap(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextAlign
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_align(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextAlignLast
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_align_last(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextJustify
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_justify(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Spacing<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_spacing(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextDecorationLine<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_line(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ExclusiveTextDecorationLine
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_exclusive_text_decoration_line(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for OtherTextDecorationLine
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_other_text_decoration_line(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextDecorationStyle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextDecorationThickness<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_thickness(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextDecorationSkipInk
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_decoration_skip_ink(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextEmphasisStyle<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextEmphasisFillMode
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_fill_mode(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextEmphasisShape
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_shape(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextEmphasisPositionHorizontal
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position_horizontal(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextEmphasisPositionVertical
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_emphasis_position_vertical(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextSizeAdjust
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_size_adjust(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextDirection
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_direction(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for UnicodeBidi
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_unicode_bidi(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BoxDecorationBreak
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_box_decoration_break(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Resize
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_resize(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CursorKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_cursor_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ColorOrAuto<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_color_or_auto(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CaretShape
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_caret_shape(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for UserSelect
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_user_select(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Appearance<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_appearance(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ListStyleType<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_list_style_type(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CounterStyle<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_counter_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SymbolsType
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_symbols_type(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PredefinedCounterStyle
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_predefined_counter_style(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Symbol<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_symbol(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ListStylePosition
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_list_style_position(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MarkerSide
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_marker_side(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SVGPaint<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_svg_paint(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for SVGPaintFallback<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_svg_paint_fallback(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FillRule
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_fill_rule(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for StrokeLinecap
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_stroke_linecap(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for StrokeLinejoin
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_stroke_linejoin(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for StrokeDasharray<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_stroke_dasharray(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Marker<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_marker(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ColorInterpolation
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_color_interpolation(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ColorRendering
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_color_rendering(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ShapeRendering
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_shape_rendering(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for TextRendering
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_text_rendering(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ImageRendering
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_image_rendering(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ClipPath<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_clip_path(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for GeometryBox
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_geometry_box(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for BasicShape<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_basic_shape(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ShapeRadius<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_shape_radius(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MaskMode
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask_mode(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MaskClip
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask_clip(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MaskComposite
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask_composite(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MaskType
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask_type(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for MaskBorderMode
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_mask_border_mode(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WebKitMaskComposite
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_mask_composite(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for WebKitMaskSourceType
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_web_kit_mask_source_type(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for FilterList<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_filter_list(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for Filter<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_filter(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ZIndex
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_z_index(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ContainerType
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container_type(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ContainerNameList<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_container_name_list(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ViewTransitionName<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_name(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for NoneOrCustomIdentList<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_none_or_custom_ident_list(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for ViewTransitionGroup<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_view_transition_group(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for PrintColorAdjust
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_print_color_adjust(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CSSWideKeyword
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_css_wide_keyword(self);
    }
}
impl<'a, VisitorT> VisitMutNode<'a, VisitorT> for CustomPropertyName<'a>
where
    VisitorT: ?Sized + VisitMut<'a>,
{
    #[inline]
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_custom_property_name(self);
    }
}
impl<'a, VisitorT: ?Sized + VisitMut<'a>> VisitMutNode<'a, VisitorT> for Declaration<'a> {
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_declaration(self);
    }
}
impl<'a, VisitorT: ?Sized + VisitMut<'a>> VisitMutNode<'a, VisitorT> for PropertyId<'a> {
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_property_id(self);
    }
}
impl<'a, VisitorT: ?Sized + VisitMut<'a>> VisitMutNode<'a, VisitorT> for VendorPrefix {
    fn visit_node(&mut self, visitor: &mut VisitorT) {
        visitor.visit_vendor_prefix(self);
    }
}
pub fn walk_declaration<'a, VisitorT: ?Sized + VisitMut<'a>>(
    visitor: &mut VisitorT,
    node: &mut Declaration<'a>,
) {
    visitor.enter_node(AstType::Declaration);
    match node {
        Declaration::BackgroundColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackgroundImage(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackgroundPositionX(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackgroundPositionY(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackgroundPosition(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackgroundSize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackgroundRepeat(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackgroundAttachment(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackgroundClip(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BackgroundOrigin(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Background(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BoxShadow(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Opacity(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Color(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Display(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Visibility(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Width(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Height(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MinWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MinHeight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaxWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaxHeight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BlockSize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::InlineSize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MinBlockSize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MinInlineSize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaxBlockSize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaxInlineSize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BoxSizing(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AspectRatio(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Overflow(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::OverflowX(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::OverflowY(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextOverflow(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Position(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Top(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Bottom(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Left(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Right(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::InsetBlockStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::InsetBlockEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::InsetInlineStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::InsetInlineEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::InsetBlock(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::InsetInline(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Inset(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderSpacing(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderTopColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBottomColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderLeftColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderRightColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockStartColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockEndColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineStartColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineEndColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderTopStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBottomStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderLeftStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderRightStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockStartStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockEndStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineStartStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineEndStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderTopWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBottomWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderLeftWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderRightWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockStartWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockEndWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineStartWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineEndWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderTopLeftRadius(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderTopRightRadius(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderBottomLeftRadius(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderBottomRightRadius(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderStartStartRadius(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderStartEndRadius(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderEndStartRadius(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderEndEndRadius(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderRadius(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderImageSource(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderImageOutset(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderImageRepeat(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderImageWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderImageSlice(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderImage(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BorderColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Border(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderTop(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBottom(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderLeft(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderRight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlock(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderBlockEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInline(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BorderInlineEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Outline(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::OutlineColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::OutlineStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::OutlineWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FlexDirection(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexWrap(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexFlow(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexGrow(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexShrink(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexBasis(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Flex(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Order(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AlignContent(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::JustifyContent(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::PlaceContent(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::AlignSelf(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::JustifySelf(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PlaceSelf(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::AlignItems(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::JustifyItems(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PlaceItems(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::RowGap(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ColumnGap(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Gap(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BoxOrient(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxDirection(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxOrdinalGroup(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxAlign(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxFlex(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxFlexGroup(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxPack(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BoxLines(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexPack(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexOrder(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexAlign(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexItemAlign(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexLinePack(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexPositive(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexNegative(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::FlexPreferredSize(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::GridTemplateColumns(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridTemplateRows(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridAutoColumns(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridAutoRows(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridAutoFlow(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridTemplateAreas(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridTemplate(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Grid(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridRowStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridRowEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridColumnStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridColumnEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridRow(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridColumn(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::GridArea(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginTop(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginBottom(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginLeft(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginRight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginBlockStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginBlockEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginInlineStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginInlineEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginBlock(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarginInline(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Margin(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingTop(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingBottom(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingLeft(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingRight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingBlockStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingBlockEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingInlineStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingInlineEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingBlock(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PaddingInline(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Padding(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginTop(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginBottom(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginLeft(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginRight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginBlockStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginBlockEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginInlineStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginInlineEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginBlock(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMarginInline(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollMargin(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingTop(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingBottom(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingLeft(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingRight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingBlockStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingBlockEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingInlineStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingInlineEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingBlock(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPaddingInline(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ScrollPadding(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FontWeight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FontSize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FontStretch(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FontFamily(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FontStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FontVariantCaps(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::LineHeight(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Font(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::VerticalAlign(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FontPalette(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TransitionProperty(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransitionDuration(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransitionDelay(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransitionTimingFunction(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Transition(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationName(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationDuration(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationTimingFunction(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationIterationCount(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationDirection(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationPlayState(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationDelay(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationFillMode(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AnimationComposition(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::AnimationTimeline(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::AnimationRangeStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::AnimationRangeEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::AnimationRange(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Animation(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Transform(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransformOrigin(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransformStyle(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TransformBox(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BackfaceVisibility(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Perspective(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::PerspectiveOrigin(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Translate(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Rotate(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Scale(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextTransform(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::WhiteSpace(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TabSize(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WordBreak(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::LineBreak(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Hyphens(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::OverflowWrap(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::WordWrap(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextAlign(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextAlignLast(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextJustify(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::WordSpacing(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::LetterSpacing(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextIndent(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextDecorationLine(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextDecorationStyle(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextDecorationColor(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextDecorationThickness(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextDecoration(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextDecorationSkipInk(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextEmphasisStyle(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextEmphasisColor(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextEmphasis(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextEmphasisPosition(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::TextShadow(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextSizeAdjust(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Direction(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::UnicodeBidi(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::BoxDecorationBreak(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Resize(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Cursor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::CaretColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::CaretShape(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Caret(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::UserSelect(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::AccentColor(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Appearance(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::ListStyleType(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ListStyleImage(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ListStylePosition(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ListStyle(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarkerSide(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Composes(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Fill(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FillRule(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::FillOpacity(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Stroke(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::StrokeOpacity(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::StrokeWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::StrokeLinecap(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::StrokeLinejoin(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::StrokeMiterlimit(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::StrokeDasharray(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::StrokeDashoffset(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarkerStart(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarkerMid(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MarkerEnd(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Marker(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ColorInterpolation(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ColorInterpolationFilters(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ColorRendering(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ShapeRendering(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::TextRendering(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ImageRendering(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ClipPath(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::ClipRule(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskImage(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskMode(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskRepeat(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskPositionX(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskPositionY(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskPosition(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskClip(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskOrigin(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskSize(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskComposite(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskType(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Mask(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MaskBorderSource(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskBorderMode(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskBorderSlice(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskBorderWidth(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskBorderOutset(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskBorderRepeat(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::MaskBorder(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::WebKitMaskComposite(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::WebKitMaskSourceType(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImage(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageSource(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageSlice(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageWidth(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageOutset(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::WebKitMaskBoxImageRepeat(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::Filter(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::BackdropFilter(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::MixBlendMode(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ZIndex(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ContainerType(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ContainerName(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Container(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ViewTransitionName(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ViewTransitionClass(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ViewTransitionGroup(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::ColorScheme(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::PrintColorAdjust(value, vendor_prefix) => {
            VisitMutNode::visit_node(value, visitor);
            VisitMutNode::visit_node(vendor_prefix, visitor);
        }
        Declaration::All(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Unparsed(value) => VisitMutNode::visit_node(value, visitor),
        Declaration::Custom(value) => VisitMutNode::visit_node(value, visitor),
    }
    visitor.leave_node(AstType::Declaration);
}
pub fn walk_property_id<'a, VisitorT: ?Sized + VisitMut<'a>>(
    visitor: &mut VisitorT,
    node: &mut PropertyId<'a>,
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
        PropertyId::BackgroundClip(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BackgroundOrigin => {}
        PropertyId::Background => {}
        PropertyId::BoxShadow(value) => VisitMutNode::visit_node(value, visitor),
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
        PropertyId::BoxSizing(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AspectRatio => {}
        PropertyId::Overflow => {}
        PropertyId::OverflowX => {}
        PropertyId::OverflowY => {}
        PropertyId::TextOverflow(value) => VisitMutNode::visit_node(value, visitor),
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
        PropertyId::BorderTopLeftRadius(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BorderTopRightRadius(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BorderBottomLeftRadius(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BorderBottomRightRadius(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BorderStartStartRadius => {}
        PropertyId::BorderStartEndRadius => {}
        PropertyId::BorderEndStartRadius => {}
        PropertyId::BorderEndEndRadius => {}
        PropertyId::BorderRadius(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BorderImageSource => {}
        PropertyId::BorderImageOutset => {}
        PropertyId::BorderImageRepeat => {}
        PropertyId::BorderImageWidth => {}
        PropertyId::BorderImageSlice => {}
        PropertyId::BorderImage(value) => VisitMutNode::visit_node(value, visitor),
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
        PropertyId::FlexDirection(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexWrap(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexFlow(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexGrow(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexShrink(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexBasis(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Flex(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Order(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AlignContent(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::JustifyContent(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::PlaceContent => {}
        PropertyId::AlignSelf(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::JustifySelf => {}
        PropertyId::PlaceSelf => {}
        PropertyId::AlignItems(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::JustifyItems => {}
        PropertyId::PlaceItems => {}
        PropertyId::RowGap => {}
        PropertyId::ColumnGap => {}
        PropertyId::Gap => {}
        PropertyId::BoxOrient(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BoxDirection(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BoxOrdinalGroup(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BoxAlign(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BoxFlex(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BoxFlexGroup(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BoxPack(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BoxLines(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexPack(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexOrder(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexAlign(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexItemAlign(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexLinePack(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexPositive(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexNegative(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::FlexPreferredSize(value) => VisitMutNode::visit_node(value, visitor),
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
        PropertyId::TransitionProperty(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TransitionDuration(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TransitionDelay(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TransitionTimingFunction(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Transition(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationName(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationDuration(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationTimingFunction(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationIterationCount(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationDirection(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationPlayState(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationDelay(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationFillMode(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AnimationComposition => {}
        PropertyId::AnimationTimeline => {}
        PropertyId::AnimationRangeStart => {}
        PropertyId::AnimationRangeEnd => {}
        PropertyId::AnimationRange => {}
        PropertyId::Animation(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Transform(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TransformOrigin(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TransformStyle(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TransformBox => {}
        PropertyId::BackfaceVisibility(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Perspective(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::PerspectiveOrigin(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Translate => {}
        PropertyId::Rotate => {}
        PropertyId::Scale => {}
        PropertyId::TextTransform => {}
        PropertyId::WhiteSpace => {}
        PropertyId::TabSize(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::WordBreak => {}
        PropertyId::LineBreak => {}
        PropertyId::Hyphens(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::OverflowWrap => {}
        PropertyId::WordWrap => {}
        PropertyId::TextAlign => {}
        PropertyId::TextAlignLast(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextJustify => {}
        PropertyId::WordSpacing => {}
        PropertyId::LetterSpacing => {}
        PropertyId::TextIndent => {}
        PropertyId::TextDecorationLine(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextDecorationStyle(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextDecorationColor(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextDecorationThickness => {}
        PropertyId::TextDecoration(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextDecorationSkipInk(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextEmphasisStyle(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextEmphasisColor(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextEmphasis(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextEmphasisPosition(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::TextShadow => {}
        PropertyId::TextSizeAdjust(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Direction => {}
        PropertyId::UnicodeBidi => {}
        PropertyId::BoxDecorationBreak(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Resize => {}
        PropertyId::Cursor => {}
        PropertyId::CaretColor => {}
        PropertyId::CaretShape => {}
        PropertyId::Caret => {}
        PropertyId::UserSelect(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::AccentColor => {}
        PropertyId::Appearance(value) => VisitMutNode::visit_node(value, visitor),
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
        PropertyId::ClipPath(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::ClipRule => {}
        PropertyId::MaskImage(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::MaskMode => {}
        PropertyId::MaskRepeat(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::MaskPositionX => {}
        PropertyId::MaskPositionY => {}
        PropertyId::MaskPosition(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::MaskClip(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::MaskOrigin(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::MaskSize(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::MaskComposite => {}
        PropertyId::MaskType => {}
        PropertyId::Mask(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::MaskBorderSource => {}
        PropertyId::MaskBorderMode => {}
        PropertyId::MaskBorderSlice => {}
        PropertyId::MaskBorderWidth => {}
        PropertyId::MaskBorderOutset => {}
        PropertyId::MaskBorderRepeat => {}
        PropertyId::MaskBorder => {}
        PropertyId::WebKitMaskComposite => {}
        PropertyId::WebKitMaskSourceType(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImage(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageSource(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageSlice(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageWidth(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageOutset(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::WebKitMaskBoxImageRepeat(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::Filter(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::BackdropFilter(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::MixBlendMode => {}
        PropertyId::ZIndex => {}
        PropertyId::ContainerType => {}
        PropertyId::ContainerName => {}
        PropertyId::Container => {}
        PropertyId::ViewTransitionName => {}
        PropertyId::ViewTransitionClass => {}
        PropertyId::ViewTransitionGroup => {}
        PropertyId::ColorScheme => {}
        PropertyId::PrintColorAdjust(value) => VisitMutNode::visit_node(value, visitor),
        PropertyId::All | PropertyId::Unparsed => {}
        PropertyId::Custom(value) => visitor.visit_str(value),
    }
    visitor.leave_node(AstType::PropertyId);
}
pub fn walk_vendor_prefix<'a, VisitorT: ?Sized + VisitMut<'a>>(
    visitor: &mut VisitorT,
    _node: &mut VendorPrefix,
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
