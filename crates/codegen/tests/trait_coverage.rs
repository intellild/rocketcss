use rs_css_ast::*;
use rs_css_codegen::{Printer, PrinterOptions, PrinterTrait, ToCss};

fn assert_to_css<T: ToCss>() {}

fn serialize_with_printer_trait<T: ToCss, PrinterT: PrinterTrait>(
    value: &T,
    printer: &mut PrinterT,
) -> std::fmt::Result {
    value.to_css(printer)
}

macro_rules! assert_types {
    ($($ty:ty),+ $(,)?) => {
        $(assert_to_css::<$ty>();)+
    };
}

#[test]
fn every_css_ast_node_implements_to_css() {
    assert_types! {
        CssColor<'static>, RGBA, LABColor, PredefinedColor, FloatColor, LightDark<'static>,
        SystemColor, UnresolvedColor<'static>, CssRule<'static>, Length<'static>, LengthUnit,
        Calc<'static, Length<'static>>, MathFunction<'static, Length<'static>>, RoundingStrategy,
        Resolution, Ratio, Angle, Time, MediaCondition<'static>,
        QueryFeature<'static, MediaFeatureId>, MediaFeatureName<'static, MediaFeatureId>,
        MediaFeature<'static>, MediaFeatureId, MediaFeatureValue<'static>, MediaFeatureComparison,
        Operator, MediaType<'static>, Qualifier, SupportsCondition<'static>, BlendMode,
        PropertyId<'static>, Declaration<'static>, VendorPrefix,

        KeyframeSelector<'static>, KeyframesName<'static>, FontFaceProperty<'static>,
        Source<'static>, FontFormat<'static>, FontTechnology, FontFaceStyle<'static>,
        FontPaletteValuesProperty<'static>, BasePalette, FontFeatureSubruleType, PageMarginBox,
        PagePseudoClass, ParsedComponent<'static>, Multiplier, SyntaxString<'static>,
        SyntaxComponentKind<'static>, ContainerCondition<'static>, ContainerSizeFeature<'static>,
        ContainerSizeFeatureId, StyleQuery<'static>, ScrollStateQuery<'static>,
        ScrollStateFeature<'static>, ScrollStateFeatureId, ViewTransitionProperty<'static>,
        Navigation, DefaultAtRule, StyleSheet<'static>, MediaRule<'static>, MediaList<'static>,
        MediaQuery<'static>, LengthValue, EnvironmentVariable<'static>, Url<'static>,
        Variable<'static>, DashedIdentReference<'static>, Function<'static>, ImportRule<'static>,
        StyleRule<'static>, DeclarationBlock<'static>, Position<'static>,
        WebKitGradientPoint<'static>, WebKitColorStop<'static>, ImageSet<'static>,
        ImageSetOption<'static>, BackgroundPosition<'static>, BackgroundRepeat,
        Background<'static>, BoxShadow<'static>, AspectRatio<'static>, Overflow,
        InsetBlock<'static>, InsetInline<'static>, Inset<'static>, BorderRadius<'static>,
        BorderImageRepeat, BorderImageSlice<'static>, BorderImage<'static>,
        BorderColor<'static>, BorderStyle, BorderWidth<'static>, BorderBlockColor<'static>,
        BorderBlockStyle, BorderBlockWidth<'static>, BorderInlineColor<'static>,
        BorderInlineStyle, BorderInlineWidth<'static>, GenericBorder<'static, LineStyle>, FlexFlow,
        Flex<'static>, PlaceContent<'static>, PlaceSelf<'static>, PlaceItems<'static>, Gap<'static>,
        TrackRepeat<'static>, GridAutoFlow, GridTemplate<'static>, Grid<'static>, GridRow<'static>,
        GridColumn<'static>, GridArea<'static>, MarginBlock<'static>, MarginInline<'static>,
        Margin<'static>, PaddingBlock<'static>, PaddingInline<'static>, Padding<'static>,
        ScrollMarginBlock<'static>, ScrollMarginInline<'static>, ScrollMargin<'static>,
        ScrollPaddingBlock<'static>, ScrollPaddingInline<'static>, ScrollPadding<'static>,
        Font<'static>, Transition<'static>, ScrollTimeline, ViewTimeline<'static>,
        AnimationRange<'static>, Animation<'static>, MatrixForFloat, Matrix3DForFloat,
        Rotate<'static>, TextTransform, TextIndent<'static>, TextDecoration<'static>,
        TextEmphasis<'static>, TextEmphasisPosition, TextShadow<'static>, Cursor<'static>,
        CursorImage<'static>, Caret<'static>, ListStyle<'static>, Composes<'static>,
        InsetRect<'static>, CircleShape<'static>, EllipseShape<'static>, Polygon<'static>,
        Point<'static>, Mask<'static>, MaskBorder<'static>, DropShadow<'static>, Container<'static>,
        ColorScheme, UnparsedProperty<'static>, CustomProperty<'static>,
        ViewTransitionPartSelector<'static>, KeyframesRule<'static>, Keyframe<'static>,
        TimelineRangePercentage, FontFaceRule<'static>, UrlSource<'static>, UnicodeRange,
        FontPaletteValuesRule<'static>, OverrideColors<'static>, FontFeatureValuesRule<'static>,
        FontFeatureSubrule<'static>, FontFeatureDeclaration<'static>, FamilyName<'static>,
        PageRule<'static>, PageMarginRule<'static>, PageSelector<'static>, SupportsRule<'static>,
        CounterStyleRule<'static>, NamespaceRule<'static>, MozDocumentRule<'static>,
        NestingRule<'static>, NestedDeclarationsRule<'static>, ViewportRule<'static>,
        CustomMediaRule<'static>, LayerStatementRule<'static>, LayerBlockRule<'static>,
        PropertyRule<'static>, SyntaxComponent<'static>, ContainerRule<'static>, ScopeRule<'static>,
        StartingStyleRule<'static>, ViewTransitionRule<'static>, PositionTryRule<'static>,
        UnknownAtRule<'static>,

        SelectorList<'static>, Selector<'static>, SelectorComponent<'static>, Combinator,
        AttrSelector<'static>, NamespaceConstraint<'static>, AttrOperation<'static>,
        ParsedCaseSensitivity, AttrSelectorOperator, NthType, NthSelectorData, Direction,
        PseudoClass<'static>, WebKitScrollbarPseudoClass, PseudoElement<'static>,
        WebKitScrollbarPseudoElement, ViewTransitionPartName<'static>, TokenOrValue<'static>,
        Unit, Token<'static>, Specifier<'static>, AnimationName<'static>,
        EnvironmentVariableName<'static>, UAEnvironmentVariable,

        Image<'static>, Gradient<'static>, WebKitGradient<'static>, LineDirection<'static>,
        HorizontalPositionKeyword, VerticalPositionKeyword,
        GradientItem<'static, LengthValue>, DimensionPercentage<'static, LengthValue>,
        LengthPercentage<'static>, AnglePercentage<'static>,
        PositionComponent<'static, HorizontalPositionKeyword>, EndingShape<'static>,
        Ellipse<'static>, ShapeExtent, Circle<'static>,
        WebKitGradientPointComponent<'static, HorizontalPositionKeyword>, NumberOrPercentage,
        BackgroundSize<'static>, LengthPercentageOrAuto<'static>, BackgroundRepeatKeyword,
        BackgroundAttachment, BackgroundClip, BackgroundOrigin, Display<'static>, DisplayKeyword,
        DisplayInside, DisplayOutside, Visibility, Size<'static>, MaxSize<'static>, BoxSizing,
        OverflowKeyword, TextOverflow, PositionProperty, Size2D<'static, Length<'static>>,
        Rect<'static, Length<'static>>, LineStyle, BorderSideWidth<'static>,
        LengthOrNumber<'static>, BorderImageRepeatKeyword, BorderImageSideWidth<'static>,
        OutlineStyle, FlexDirection, FlexWrap, AlignContent, BaselinePosition,
        ContentDistribution, OverflowPosition, ContentPosition, JustifyContent, AlignSelf,
        SelfPosition, JustifySelf, AlignItems, JustifyItems, LegacyJustify, GapValue<'static>,
        BoxOrient, BoxDirection, BoxAlign, BoxPack, BoxLines, FlexPack, FlexItemAlign,
        FlexLinePack, TrackSizing<'static>, TrackListItem<'static>, TrackSize<'static>,
        TrackBreadth<'static>, RepeatCount, AutoFlowDirection, GridTemplateAreas<'static>,
        GridLine<'static>, FontWeight<'static>, AbsoluteFontWeight, FontSize<'static>,
        AbsoluteFontSize, RelativeFontSize, FontStretch, FontStretchKeyword, FontFamily<'static>,
        GenericFontFamily, FontStyle<'static>, FontVariantCaps, LineHeight<'static>,
        VerticalAlign<'static>, VerticalAlignKeyword, EasingFunction, StepPosition,
        AnimationIterationCount, AnimationDirection, AnimationPlayState, AnimationFillMode,
        AnimationComposition, AnimationTimeline<'static>, ScrollAxis, Scroller,
        AnimationAttachmentRange<'static>, AnimationRangeStart<'static>, AnimationRangeEnd<'static>,
        TimelineRangeName, Transform<'static>, TransformStyle, TransformBox,
        BackfaceVisibility, Perspective<'static>, Translate<'static>, Scale<'static>,
        TextTransformCase, WhiteSpace, WordBreak, LineBreak, Hyphens, OverflowWrap, TextAlign,
        TextAlignLast, TextJustify, Spacing<'static>, TextDecorationLine<'static>,
        ExclusiveTextDecorationLine, OtherTextDecorationLine, TextDecorationStyle,
        TextDecorationThickness<'static>, TextDecorationSkipInk, TextEmphasisStyle<'static>,
        TextEmphasisFillMode, TextEmphasisShape, TextEmphasisPositionHorizontal,
        TextEmphasisPositionVertical, TextSizeAdjust, TextDirection, UnicodeBidi,
        BoxDecorationBreak, Resize, CursorKeyword, ColorOrAuto<'static>, CaretShape, UserSelect,
        Appearance<'static>, ListStyleType<'static>, CounterStyle<'static>, SymbolsType,
        PredefinedCounterStyle, Symbol<'static>, ListStylePosition, MarkerSide, SVGPaint<'static>,
        SVGPaintFallback<'static>, FillRule, StrokeLinecap, StrokeLinejoin,
        StrokeDasharray<'static>, Marker<'static>, ColorInterpolation, ColorRendering,
        ShapeRendering, TextRendering, ImageRendering, ClipPath<'static>, GeometryBox,
        BasicShape<'static>, ShapeRadius<'static>, MaskMode, MaskClip, MaskComposite, MaskType,
        MaskBorderMode, WebKitMaskComposite, WebKitMaskSourceType, FilterList<'static>,
        Filter<'static>, ZIndex, ContainerType, ContainerNameList<'static>,
        ViewTransitionName<'static>, NoneOrCustomIdentList<'static>, ViewTransitionGroup<'static>,
        PrintColorAdjust, CSSWideKeyword, CustomPropertyName<'static>
    }
}

#[test]
fn to_css_only_depends_on_the_printer_trait() {
    let mut output = String::new();
    let mut printer = Printer::new(&mut output, PrinterOptions::default());
    serialize_with_printer_trait(&CSSWideKeyword::Initial, &mut printer).unwrap();
    assert_eq!(output, "initial");
}
