use rocketcss_ast::*;
use rocketcss_codegen::{Printer, PrinterOptions, PrinterTrait, ToCss, ToCssContext};
use rocketcss_common::GhostToken;

fn assert_to_css<T>()
where
    T: for<'ghost> ToCss<'ghost>,
{
}

fn assert_branded_to_css<T: ToCss<'static>>() {}

fn serialize_with_printer_trait<'ghost, T: ToCss<'ghost>, PrinterT: PrinterTrait>(
    value: &T,
    printer: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> std::fmt::Result {
    value.to_css(printer, cx)
}

macro_rules! assert_types {
    ($($ty:ty),+ $(,)?) => {
        $(assert_to_css::<$ty>();)+
    };
}

macro_rules! assert_ghost_types {
    ($($ty:ty),+ $(,)?) => {
        $(assert_branded_to_css::<$ty>();)+
    };
}

#[test]
fn every_css_ast_node_implements_to_css() {
    assert_types! {
        CssColor<'static>, RGBA, LABColor, PredefinedColor, FloatColor, LightDark<'static>,
        SystemColor, UnresolvedColor<'static>, Length, LengthUnit,
        Calc<Length>, MathFunction<Length>, RoundingStrategy,
        Resolution, Ratio, Angle, Time, MediaCondition<'static>,
        QueryFeature<'static, MediaFeatureId>, MediaFeatureName<'static, MediaFeatureId>,
        MediaFeature<'static>, MediaFeatureId, MediaFeatureValue<'static>, MediaFeatureComparison,
        Operator, MediaType<'static>, Qualifier, SupportsCondition<'static>, BlendMode,
        PropertyId<'static>, Declaration<'static>, VendorPrefix,

        KeyframeSelector, KeyframesName<'static>, FontFaceProperty<'static>,
        Source<'static>, FontFormat<'static>, FontTechnology, FontFaceStyle,
        FontPaletteValuesProperty<'static>, BasePalette, FontFeatureSubruleType, PageMarginBox,
        PagePseudoClass, ParsedComponent<'static>, Multiplier, SyntaxString,
        SyntaxComponentKind, ContainerCondition<'static>, ContainerSizeFeature<'static>,
        ContainerSizeFeatureId, StyleQuery<'static>, ScrollStateQuery<'static>,
        ScrollStateFeature<'static>, ScrollStateFeatureId, ViewTransitionProperty<'static>,
        Navigation, DefaultAtRule, MediaList<'static>,
        MediaQuery<'static>, LengthValue, EnvironmentVariable<'static>, Url<'static>,
        Variable<'static>, DashedIdentReference<'static>, Function<'static>, ImportRule<'static>,
        Position,
        WebKitGradientPoint, WebKitColorStop<'static>, ImageSet<'static>,
        ImageSetOption<'static>, BackgroundPosition, BackgroundRepeat,
        Background<'static>, BoxShadow<'static>, AspectRatio, Overflow,
        InsetBlock, InsetInline, Inset, BorderRadius,
        BorderImageRepeat, BorderImageSlice, BorderImage<'static>,
        BorderColor<'static>, BorderStyle, BorderWidth, BorderBlockColor<'static>,
        BorderBlockStyle, BorderBlockWidth, BorderInlineColor<'static>,
        BorderInlineStyle, BorderInlineWidth, GenericBorder<'static, LineStyle>, FlexFlow,
        Flex, PlaceContent, PlaceSelf, PlaceItems, Gap,
        TrackRepeat<'static>, GridAutoFlow, GridTemplate<'static>, Grid<'static>, GridRow<'static>,
        GridColumn<'static>, GridArea<'static>, MarginBlock, MarginInline,
        Margin, PaddingBlock, PaddingInline, Padding,
        ScrollMarginBlock, ScrollMarginInline, ScrollMargin,
        ScrollPaddingBlock, ScrollPaddingInline, ScrollPadding,
        Font<'static>, Transition<'static>, ScrollTimeline, ViewTimeline,
        AnimationRange, Animation<'static>, MatrixForFloat, Matrix3DForFloat,
        Rotate, TextTransform, TextIndent, TextDecoration<'static>,
        TextEmphasis<'static>, TextEmphasisPosition, TextShadow<'static>, Cursor<'static>,
        CursorImage<'static>, Caret<'static>, ListStyle<'static>, Composes<'static>,
        InsetRect, CircleShape, EllipseShape, Polygon,
        Point, Mask<'static>, MaskBorder<'static>, DropShadow<'static>, Container<'static>,
        ColorScheme, UnparsedProperty<'static>, CustomProperty<'static>,
        ViewTransitionPartSelector<'static>,
        TimelineRangePercentage, FontFaceRule<'static>, UrlSource<'static>, UnicodeRange,
        FontPaletteValuesRule<'static>, OverrideColors<'static>, FontFeatureValuesRule<'static>,
        FontFeatureSubrule<'static>, FontFeatureDeclaration<'static>, FamilyName<'static>,
        PageSelector<'static>, CharsetRule<'static>, NamespaceRule<'static>,
        CustomMediaRule<'static>, LayerStatementRule<'static>,
        PropertyRule<'static>, SyntaxComponent,
        ViewTransitionRule<'static>,
        UnknownAtRule<'static>,

        SelectorList<'static>, Selector<'static>, SelectorComponent<'static>, Combinator,
        AttrSelector<'static>, NamespaceConstraint<'static>, AttrOperation<'static>,
        ParsedCaseSensitivity, AttrSelectorOperator, NthType, NthSelectorData, Direction,
        PseudoClass<'static>, WebKitScrollbarPseudoClass, PseudoElement<'static>,
        WebKitScrollbarPseudoElement, ViewTransitionPartName<'static>, TokenOrValue<'static>,
        Unit, Token<'static>, Specifier<'static>, AnimationName<'static>,
        EnvironmentVariableName<'static>, UAEnvironmentVariable,

        Image<'static>, Gradient<'static>, WebKitGradient<'static>, LineDirection,
        HorizontalPositionKeyword, VerticalPositionKeyword,
        GradientItem<'static, LengthValue>, DimensionPercentage<LengthValue>,
        LengthPercentage, AnglePercentage,
        PositionComponent<HorizontalPositionKeyword>, EndingShape,
        Ellipse, ShapeExtent, Circle,
        WebKitGradientPointComponent<HorizontalPositionKeyword>, NumberOrPercentage,
        BackgroundSize, LengthPercentageOrAuto, BackgroundRepeatKeyword,
        BackgroundAttachment, BackgroundClip, BackgroundOrigin, Display, DisplayKeyword,
        DisplayInside, DisplayOutside, Visibility, Size<'static>, MaxSize<'static>, BoxSizing,
        OverflowKeyword, TextOverflow, PositionProperty, Size2D<Length>,
        Rect<Length>, LineStyle, BorderSideWidth,
        LengthOrNumber, BorderImageRepeatKeyword, BorderImageSideWidth,
        OutlineStyle, FlexDirection, FlexWrap, AlignContent, BaselinePosition,
        ContentDistribution, OverflowPosition, ContentPosition, JustifyContent, AlignSelf,
        SelfPosition, JustifySelf, AlignItems, JustifyItems, LegacyJustify, GapValue,
        BoxOrient, BoxDirection, BoxAlign, BoxPack, BoxLines, FlexPack, FlexItemAlign,
        FlexLinePack, TrackSizing<'static>, TrackListItem<'static>, TrackSize,
        TrackBreadth, RepeatCount, AutoFlowDirection, GridTemplateAreas<'static>,
        GridLine<'static>, FontWeight, AbsoluteFontWeight, FontSize,
        AbsoluteFontSize, RelativeFontSize, FontStretch, FontStretchKeyword, FontFamily<'static>,
        FontStyle, FontVariantCaps, LineHeight,
        VerticalAlign, VerticalAlignKeyword, EasingFunction, StepPosition,
        AnimationIterationCount, AnimationDirection, AnimationPlayState, AnimationFillMode,
        AnimationComposition, AnimationTimeline<'static>, ScrollAxis, Scroller,
        AnimationAttachmentRange, AnimationRangeStart, AnimationRangeEnd,
        TimelineRangeName, Transform, TransformStyle, TransformBox,
        BackfaceVisibility, Perspective, Translate, Scale,
        TextTransformCase, WhiteSpace, WordBreak, LineBreak, Hyphens, OverflowWrap, TextAlign,
        TextAlignLast, TextJustify, Spacing, TextDecorationLine,
        ExclusiveTextDecorationLine, OtherTextDecorationLine, TextDecorationStyle,
        TextDecorationThickness, TextDecorationSkipInk, TextEmphasisStyle<'static>,
        TextEmphasisFillMode, TextEmphasisShape, TextEmphasisPositionHorizontal,
        TextEmphasisPositionVertical, TextSizeAdjust, TextDirection, UnicodeBidi,
        BoxDecorationBreak, Resize, CursorKeyword, ColorOrAuto<'static>, CaretShape, UserSelect,
        Appearance<'static>, ListStyleType<'static>, CounterStyle<'static>, SymbolsType,
        PredefinedCounterStyle, Symbol<'static>, ListStylePosition, MarkerSide, SVGPaint<'static>,
        SVGPaintFallback<'static>, FillRule, StrokeLinecap, StrokeLinejoin,
        StrokeDasharray, Marker<'static>, ColorInterpolation, ColorRendering,
        ShapeRendering, TextRendering, ImageRendering, ClipPath<'static>, GeometryBox,
        BasicShape, ShapeRadius, MaskMode, MaskClip, MaskComposite, MaskType,
        MaskBorderMode, WebKitMaskComposite, WebKitMaskSourceType, FilterList<'static>,
        Filter<'static>, ZIndex, ContainerType, ContainerNameList<'static>,
        ViewTransitionName<'static>, NoneOrCustomIdentList<'static>, ViewTransitionGroup<'static>,
        PrintColorAdjust, CSSWideKeyword, CustomPropertyName<'static>
    }

    assert_ghost_types! {
        CssRule<'static>, StyleSheet<'static>,
        DeclarationBlockRef<'static, 'static>,
        MediaRule<'static>, StyleRule<'static>,
        KeyframesRule<'static>, Keyframe,
        PageRule<'static>, PageMarginRule,
        SupportsRule<'static>, CounterStyleRule<'static>,
        MozDocumentRule<'static>, NestingRule<'static>,
        NestedDeclarationsRule, ViewportRule,
        LayerBlockRule<'static>, ContainerRule<'static>,
        ScopeRule<'static>, StartingStyleRule<'static>,
        PositionTryRule<'static>,
    }
}

#[test]
fn to_css_only_depends_on_the_printer_trait() {
    GhostToken::scope(|token| {
        let mut output = String::new();
        let mut printer = Printer::new(&mut output, PrinterOptions::default());
        serialize_with_printer_trait(
            &CSSWideKeyword::Initial,
            &mut printer,
            &ToCssContext::new(&token),
        )
        .unwrap();
        assert_eq!(output, "initial");
    });
}
