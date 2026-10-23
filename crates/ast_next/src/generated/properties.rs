use crate::*;
/// The parser construction path generated for a known property.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertyParserStrategy {
    Parse,
    Boxed,
    CommaSeparated,
    WhitespaceSeparated,
    Rect,
    TwoValue,
    Custom,
    CssWide,
    CssWideBoxed,
    Unsupported,
}
use crate::*;
/// Coarse parser support used by audits without duplicating property metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertySupport {
    Typed,
    UnsupportedGrammar,
    Custom,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    PlusDarker,
    PlusLighter,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PropertyId<'ast> {
    All,
    BackgroundColor,
    BackgroundImage,
    BackgroundPositionX,
    BackgroundPositionY,
    BackgroundPosition,
    BackgroundSize,
    BackgroundRepeat,
    BackgroundAttachment,
    BackgroundClip(VendorPrefix),
    BackgroundOrigin,
    Background,
    BoxShadow(VendorPrefix),
    Opacity,
    Color,
    Display,
    Visibility,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    BlockSize,
    InlineSize,
    MinBlockSize,
    MinInlineSize,
    MaxBlockSize,
    MaxInlineSize,
    BoxSizing(VendorPrefix),
    AspectRatio,
    Overflow,
    OverflowX,
    OverflowY,
    TextOverflow(VendorPrefix),
    ObjectFit,
    ObjectPosition,
    ScrollbarColor,
    Position,
    Top,
    Bottom,
    Left,
    Right,
    InsetBlockStart,
    InsetBlockEnd,
    InsetInlineStart,
    InsetInlineEnd,
    InsetBlock,
    InsetInline,
    Inset,
    BorderSpacing,
    BorderTopColor,
    BorderBottomColor,
    BorderLeftColor,
    BorderRightColor,
    BorderBlockStartColor,
    BorderBlockEndColor,
    BorderInlineStartColor,
    BorderInlineEndColor,
    BorderTopStyle,
    BorderBottomStyle,
    BorderLeftStyle,
    BorderRightStyle,
    BorderBlockStartStyle,
    BorderBlockEndStyle,
    BorderInlineStartStyle,
    BorderInlineEndStyle,
    BorderTopWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    BorderRightWidth,
    BorderBlockStartWidth,
    BorderBlockEndWidth,
    BorderInlineStartWidth,
    BorderInlineEndWidth,
    BorderTopLeftRadius(VendorPrefix),
    BorderTopRightRadius(VendorPrefix),
    BorderBottomLeftRadius(VendorPrefix),
    BorderBottomRightRadius(VendorPrefix),
    BorderStartStartRadius,
    BorderStartEndRadius,
    BorderEndStartRadius,
    BorderEndEndRadius,
    BorderRadius(VendorPrefix),
    BorderImageSource,
    BorderImageOutset,
    BorderImageRepeat,
    BorderImageWidth,
    BorderImageSlice,
    BorderImage(VendorPrefix),
    BorderColor,
    BorderStyle,
    BorderWidth,
    BorderBlockColor,
    BorderBlockStyle,
    BorderBlockWidth,
    BorderInlineColor,
    BorderInlineStyle,
    BorderInlineWidth,
    Border,
    BorderTop,
    BorderBottom,
    BorderLeft,
    BorderRight,
    BorderBlock,
    BorderBlockStart,
    BorderBlockEnd,
    BorderInline,
    BorderInlineStart,
    BorderInlineEnd,
    Outline,
    OutlineColor,
    OutlineStyle,
    OutlineWidth,
    FlexDirection(VendorPrefix),
    FlexWrap(VendorPrefix),
    FlexFlow(VendorPrefix),
    FlexGrow(VendorPrefix),
    FlexShrink(VendorPrefix),
    FlexBasis(VendorPrefix),
    Flex(VendorPrefix),
    Order(VendorPrefix),
    AlignContent(VendorPrefix),
    JustifyContent(VendorPrefix),
    PlaceContent,
    AlignSelf(VendorPrefix),
    JustifySelf,
    PlaceSelf,
    AlignItems(VendorPrefix),
    JustifyItems,
    PlaceItems,
    RowGap,
    ColumnGap,
    Gap,
    ColumnRule(VendorPrefix),
    ColumnWidth(VendorPrefix),
    ColumnCount(VendorPrefix),
    Columns(VendorPrefix),
    GridColumnGap,
    GridRowGap,
    BoxOrient(VendorPrefix),
    BoxDirection(VendorPrefix),
    BoxOrdinalGroup(VendorPrefix),
    BoxAlign(VendorPrefix),
    BoxFlex(VendorPrefix),
    BoxFlexGroup(VendorPrefix),
    BoxPack(VendorPrefix),
    BoxLines(VendorPrefix),
    FlexPack(VendorPrefix),
    FlexOrder(VendorPrefix),
    FlexAlign(VendorPrefix),
    FlexItemAlign(VendorPrefix),
    FlexLinePack(VendorPrefix),
    FlexPositive(VendorPrefix),
    FlexNegative(VendorPrefix),
    FlexPreferredSize(VendorPrefix),
    GridTemplateColumns,
    GridTemplateRows,
    GridAutoColumns,
    GridAutoRows,
    GridAutoFlow,
    GridTemplateAreas,
    GridTemplate,
    Grid,
    GridRowStart,
    GridRowEnd,
    GridColumnStart,
    GridColumnEnd,
    GridRow,
    GridColumn,
    GridArea,
    MarginTop,
    MarginBottom,
    MarginLeft,
    MarginRight,
    MarginBlockStart,
    MarginBlockEnd,
    MarginInlineStart,
    MarginInlineEnd,
    MarginBlock,
    MarginInline,
    Margin,
    PaddingTop,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    PaddingBlockStart,
    PaddingBlockEnd,
    PaddingInlineStart,
    PaddingInlineEnd,
    PaddingBlock,
    PaddingInline,
    Padding,
    ScrollMarginTop,
    ScrollMarginBottom,
    ScrollMarginLeft,
    ScrollMarginRight,
    ScrollMarginBlockStart,
    ScrollMarginBlockEnd,
    ScrollMarginInlineStart,
    ScrollMarginInlineEnd,
    ScrollMarginBlock,
    ScrollMarginInline,
    ScrollMargin,
    ScrollPaddingTop,
    ScrollPaddingBottom,
    ScrollPaddingLeft,
    ScrollPaddingRight,
    ScrollPaddingBlockStart,
    ScrollPaddingBlockEnd,
    ScrollPaddingInlineStart,
    ScrollPaddingInlineEnd,
    ScrollPaddingBlock,
    ScrollPaddingInline,
    ScrollPadding,
    FontWeight,
    FontSize,
    FontStretch,
    FontFamily,
    FontStyle,
    FontVariantCaps,
    LineHeight,
    Font,
    VerticalAlign,
    FontPalette,
    TransitionProperty(VendorPrefix),
    TransitionDuration(VendorPrefix),
    TransitionDelay(VendorPrefix),
    TransitionTimingFunction(VendorPrefix),
    Transition(VendorPrefix),
    AnimationName(VendorPrefix),
    AnimationDuration(VendorPrefix),
    AnimationTimingFunction(VendorPrefix),
    AnimationIterationCount(VendorPrefix),
    AnimationDirection(VendorPrefix),
    AnimationPlayState(VendorPrefix),
    AnimationDelay(VendorPrefix),
    AnimationFillMode(VendorPrefix),
    AnimationComposition,
    AnimationTimeline,
    AnimationRangeStart,
    AnimationRangeEnd,
    AnimationRange,
    Animation(VendorPrefix),
    Transform(VendorPrefix),
    TransformOrigin(VendorPrefix),
    TransformStyle(VendorPrefix),
    TransformBox,
    BackfaceVisibility(VendorPrefix),
    Perspective(VendorPrefix),
    PerspectiveOrigin(VendorPrefix),
    Translate,
    Rotate,
    Scale,
    TextTransform,
    Content,
    WhiteSpace,
    TabSize(VendorPrefix),
    WordBreak,
    LineBreak,
    Hyphens(VendorPrefix),
    OverflowWrap,
    WordWrap,
    TextAlign,
    TextAlignLast(VendorPrefix),
    TextJustify,
    WordSpacing,
    LetterSpacing,
    TextIndent,
    TextDecorationLine(VendorPrefix),
    TextDecorationStyle(VendorPrefix),
    TextDecorationColor(VendorPrefix),
    TextDecorationThickness,
    TextDecoration(VendorPrefix),
    TextDecorationSkipInk(VendorPrefix),
    TextEmphasisStyle(VendorPrefix),
    TextEmphasisColor(VendorPrefix),
    TextEmphasis(VendorPrefix),
    TextEmphasisPosition(VendorPrefix),
    TextShadow,
    TextSizeAdjust(VendorPrefix),
    Direction,
    UnicodeBidi,
    BoxDecorationBreak(VendorPrefix),
    Resize,
    PointerEvents,
    Float,
    Clear,
    TouchAction,
    ScrollBehavior,
    Cursor,
    CaretColor,
    CaretShape,
    Caret,
    UserSelect(VendorPrefix),
    AccentColor,
    Appearance(VendorPrefix),
    ListStyleType,
    ListStyleImage,
    ListStylePosition,
    ListStyle,
    MarkerSide,
    Composes,
    Fill,
    FillRule,
    FillOpacity,
    Stroke,
    StrokeOpacity,
    StrokeWidth,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeDasharray,
    StrokeDashoffset,
    MarkerStart,
    MarkerMid,
    MarkerEnd,
    Marker,
    ColorInterpolation,
    ColorInterpolationFilters,
    ColorRendering,
    ShapeRendering,
    TextRendering,
    ImageRendering,
    ClipPath(VendorPrefix),
    ClipRule,
    MaskImage(VendorPrefix),
    MaskMode,
    MaskRepeat(VendorPrefix),
    MaskPositionX,
    MaskPositionY,
    MaskPosition(VendorPrefix),
    MaskClip(VendorPrefix),
    MaskOrigin(VendorPrefix),
    MaskSize(VendorPrefix),
    MaskComposite,
    MaskType,
    Mask(VendorPrefix),
    MaskBorderSource,
    MaskBorderMode,
    MaskBorderSlice,
    MaskBorderWidth,
    MaskBorderOutset,
    MaskBorderRepeat,
    MaskBorder,
    WebKitMaskComposite,
    WebKitMaskSourceType(VendorPrefix),
    WebKitMaskBoxImage(VendorPrefix),
    WebKitMaskBoxImageSource(VendorPrefix),
    WebKitMaskBoxImageSlice(VendorPrefix),
    WebKitMaskBoxImageWidth(VendorPrefix),
    WebKitMaskBoxImageOutset(VendorPrefix),
    WebKitMaskBoxImageRepeat(VendorPrefix),
    Filter(VendorPrefix),
    BackdropFilter(VendorPrefix),
    MixBlendMode,
    ZIndex,
    ContainerType,
    ContainerName,
    Container,
    ViewTransitionName,
    ViewTransitionClass,
    ViewTransitionGroup,
    ColorScheme,
    PrintColorAdjust(VendorPrefix),
    Unparsed,
    Custom(&'ast str),
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Declaration<'ast>(pub(crate) NodeId<'ast, DeclarationData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeclarationData<'ast> {
    All(CSSWideKeyword),
    BackgroundColor(CssColor<'ast>),
    BackgroundImage(&'ast [Image<'ast>]),
    BackgroundPositionX(&'ast [PositionComponent<'ast, HorizontalPositionKeyword>]),
    BackgroundPositionY(&'ast [PositionComponent<'ast, VerticalPositionKeyword>]),
    BackgroundPosition(&'ast [BackgroundPosition<'ast>]),
    BackgroundSize(&'ast [BackgroundSize<'ast>]),
    BackgroundRepeat(&'ast [BackgroundRepeat<'ast>]),
    BackgroundAttachment(&'ast [BackgroundAttachment]),
    BackgroundClip(&'ast [BackgroundClip], VendorPrefix),
    BackgroundOrigin(&'ast [BackgroundOrigin]),
    Background(&'ast [Background<'ast>]),
    BoxShadow(&'ast [BoxShadow<'ast>], VendorPrefix),
    Opacity(f32),
    Color(CssColor<'ast>),
    Display(Display<'ast>),
    Visibility(Visibility),
    Width(Size<'ast>),
    Height(Size<'ast>),
    MinWidth(Size<'ast>),
    MinHeight(Size<'ast>),
    MaxWidth(MaxSize<'ast>),
    MaxHeight(MaxSize<'ast>),
    BlockSize(Size<'ast>),
    InlineSize(Size<'ast>),
    MinBlockSize(Size<'ast>),
    MinInlineSize(Size<'ast>),
    MaxBlockSize(MaxSize<'ast>),
    MaxInlineSize(MaxSize<'ast>),
    BoxSizing(BoxSizing, VendorPrefix),
    AspectRatio(AspectRatio<'ast>),
    Overflow(Overflow<'ast>),
    OverflowX(OverflowKeyword),
    OverflowY(OverflowKeyword),
    TextOverflow(TextOverflow, VendorPrefix),
    ObjectFit(ObjectFit),
    ObjectPosition(Position<'ast>),
    ScrollbarColor(ScrollbarColor<'ast>),
    Position(PositionProperty),
    Top(LengthPercentageOrAuto<'ast>),
    Bottom(LengthPercentageOrAuto<'ast>),
    Left(LengthPercentageOrAuto<'ast>),
    Right(LengthPercentageOrAuto<'ast>),
    InsetBlockStart(LengthPercentageOrAuto<'ast>),
    InsetBlockEnd(LengthPercentageOrAuto<'ast>),
    InsetInlineStart(LengthPercentageOrAuto<'ast>),
    InsetInlineEnd(LengthPercentageOrAuto<'ast>),
    InsetBlock(InsetBlock<'ast>),
    InsetInline(InsetInline<'ast>),
    Inset(Inset<'ast>),
    BorderSpacing(Size2D<'ast, Length<'ast>>),
    BorderTopColor(CssColor<'ast>),
    BorderBottomColor(CssColor<'ast>),
    BorderLeftColor(CssColor<'ast>),
    BorderRightColor(CssColor<'ast>),
    BorderBlockStartColor(CssColor<'ast>),
    BorderBlockEndColor(CssColor<'ast>),
    BorderInlineStartColor(CssColor<'ast>),
    BorderInlineEndColor(CssColor<'ast>),
    BorderTopStyle(LineStyle),
    BorderBottomStyle(LineStyle),
    BorderLeftStyle(LineStyle),
    BorderRightStyle(LineStyle),
    BorderBlockStartStyle(LineStyle),
    BorderBlockEndStyle(LineStyle),
    BorderInlineStartStyle(LineStyle),
    BorderInlineEndStyle(LineStyle),
    BorderTopWidth(BorderSideWidth<'ast>),
    BorderBottomWidth(BorderSideWidth<'ast>),
    BorderLeftWidth(BorderSideWidth<'ast>),
    BorderRightWidth(BorderSideWidth<'ast>),
    BorderBlockStartWidth(BorderSideWidth<'ast>),
    BorderBlockEndWidth(BorderSideWidth<'ast>),
    BorderInlineStartWidth(BorderSideWidth<'ast>),
    BorderInlineEndWidth(BorderSideWidth<'ast>),
    BorderTopLeftRadius(Size2D<'ast, LengthPercentage<'ast>>, VendorPrefix),
    BorderTopRightRadius(Size2D<'ast, LengthPercentage<'ast>>, VendorPrefix),
    BorderBottomLeftRadius(Size2D<'ast, LengthPercentage<'ast>>, VendorPrefix),
    BorderBottomRightRadius(Size2D<'ast, LengthPercentage<'ast>>, VendorPrefix),
    BorderStartStartRadius(Size2D<'ast, LengthPercentage<'ast>>),
    BorderStartEndRadius(Size2D<'ast, LengthPercentage<'ast>>),
    BorderEndStartRadius(Size2D<'ast, LengthPercentage<'ast>>),
    BorderEndEndRadius(Size2D<'ast, LengthPercentage<'ast>>),
    BorderRadius(BorderRadius<'ast>, VendorPrefix),
    BorderImageSource(Image<'ast>),
    BorderImageOutset(Rect<'ast, LengthOrNumber<'ast>>),
    BorderImageRepeat(BorderImageRepeat<'ast>),
    BorderImageWidth(Rect<'ast, BorderImageSideWidth<'ast>>),
    BorderImageSlice(BorderImageSlice<'ast>),
    BorderImage(BorderImage<'ast>, VendorPrefix),
    BorderColor(BorderColor<'ast>),
    BorderStyle(BorderStyle<'ast>),
    BorderWidth(BorderWidth<'ast>),
    BorderBlockColor(BorderBlockColor<'ast>),
    BorderBlockStyle(BorderBlockStyle<'ast>),
    BorderBlockWidth(BorderBlockWidth<'ast>),
    BorderInlineColor(BorderInlineColor<'ast>),
    BorderInlineStyle(BorderInlineStyle<'ast>),
    BorderInlineWidth(BorderInlineWidth<'ast>),
    Border(GenericBorder<'ast, LineStyle>),
    BorderTop(GenericBorder<'ast, LineStyle>),
    BorderBottom(GenericBorder<'ast, LineStyle>),
    BorderLeft(GenericBorder<'ast, LineStyle>),
    BorderRight(GenericBorder<'ast, LineStyle>),
    BorderBlock(GenericBorder<'ast, LineStyle>),
    BorderBlockStart(GenericBorder<'ast, LineStyle>),
    BorderBlockEnd(GenericBorder<'ast, LineStyle>),
    BorderInline(GenericBorder<'ast, LineStyle>),
    BorderInlineStart(GenericBorder<'ast, LineStyle>),
    BorderInlineEnd(GenericBorder<'ast, LineStyle>),
    Outline(GenericBorder<'ast, OutlineStyle<'ast>>),
    OutlineColor(CssColor<'ast>),
    OutlineStyle(OutlineStyle<'ast>),
    OutlineWidth(BorderSideWidth<'ast>),
    FlexDirection(FlexDirection, VendorPrefix),
    FlexWrap(FlexWrap, VendorPrefix),
    FlexFlow(FlexFlow<'ast>, VendorPrefix),
    FlexGrow(f32, VendorPrefix),
    FlexShrink(f32, VendorPrefix),
    FlexBasis(LengthPercentageOrAuto<'ast>, VendorPrefix),
    Flex(Flex<'ast>, VendorPrefix),
    Order(f32, VendorPrefix),
    AlignContent(AlignContent, VendorPrefix),
    JustifyContent(JustifyContent, VendorPrefix),
    PlaceContent(PlaceContent<'ast>),
    AlignSelf(AlignSelf, VendorPrefix),
    JustifySelf(JustifySelf),
    PlaceSelf(PlaceSelf<'ast>),
    AlignItems(AlignItems, VendorPrefix),
    JustifyItems(JustifyItems),
    PlaceItems(PlaceItems<'ast>),
    RowGap(GapValue<'ast>),
    ColumnGap(GapValue<'ast>),
    Gap(Gap<'ast>),
    ColumnRule(ColumnRule<'ast>, VendorPrefix),
    ColumnWidth(CSSWideOr<ColumnWidth<'ast>>, VendorPrefix),
    ColumnCount(CSSWideOr<ColumnCount>, VendorPrefix),
    Columns(CSSWideOr<Columns<'ast>>, VendorPrefix),
    GridColumnGap(GapValue<'ast>),
    GridRowGap(GapValue<'ast>),
    BoxOrient(BoxOrient, VendorPrefix),
    BoxDirection(BoxDirection, VendorPrefix),
    BoxOrdinalGroup(f32, VendorPrefix),
    BoxAlign(BoxAlign, VendorPrefix),
    BoxFlex(f32, VendorPrefix),
    BoxFlexGroup(f32, VendorPrefix),
    BoxPack(BoxPack, VendorPrefix),
    BoxLines(BoxLines, VendorPrefix),
    FlexPack(FlexPack, VendorPrefix),
    FlexOrder(f32, VendorPrefix),
    FlexAlign(BoxAlign, VendorPrefix),
    FlexItemAlign(FlexItemAlign, VendorPrefix),
    FlexLinePack(FlexLinePack, VendorPrefix),
    FlexPositive(f32, VendorPrefix),
    FlexNegative(f32, VendorPrefix),
    FlexPreferredSize(LengthPercentageOrAuto<'ast>, VendorPrefix),
    GridTemplateColumns(TrackSizing<'ast>),
    GridTemplateRows(TrackSizing<'ast>),
    GridAutoColumns(&'ast [TrackSize<'ast>]),
    GridAutoRows(&'ast [TrackSize<'ast>]),
    GridAutoFlow(GridAutoFlow<'ast>),
    GridTemplateAreas(GridTemplateAreas<'ast>),
    GridTemplate(GridTemplate<'ast>),
    Grid(Grid<'ast>),
    GridRowStart(GridLine<'ast>),
    GridRowEnd(GridLine<'ast>),
    GridColumnStart(GridLine<'ast>),
    GridColumnEnd(GridLine<'ast>),
    GridRow(GridRow<'ast>),
    GridColumn(GridColumn<'ast>),
    GridArea(GridArea<'ast>),
    MarginTop(LengthPercentageOrAuto<'ast>),
    MarginBottom(LengthPercentageOrAuto<'ast>),
    MarginLeft(LengthPercentageOrAuto<'ast>),
    MarginRight(LengthPercentageOrAuto<'ast>),
    MarginBlockStart(LengthPercentageOrAuto<'ast>),
    MarginBlockEnd(LengthPercentageOrAuto<'ast>),
    MarginInlineStart(LengthPercentageOrAuto<'ast>),
    MarginInlineEnd(LengthPercentageOrAuto<'ast>),
    MarginBlock(MarginBlock<'ast>),
    MarginInline(MarginInline<'ast>),
    Margin(Margin<'ast>),
    PaddingTop(LengthPercentageOrAuto<'ast>),
    PaddingBottom(LengthPercentageOrAuto<'ast>),
    PaddingLeft(LengthPercentageOrAuto<'ast>),
    PaddingRight(LengthPercentageOrAuto<'ast>),
    PaddingBlockStart(LengthPercentageOrAuto<'ast>),
    PaddingBlockEnd(LengthPercentageOrAuto<'ast>),
    PaddingInlineStart(LengthPercentageOrAuto<'ast>),
    PaddingInlineEnd(LengthPercentageOrAuto<'ast>),
    PaddingBlock(PaddingBlock<'ast>),
    PaddingInline(PaddingInline<'ast>),
    Padding(Padding<'ast>),
    ScrollMarginTop(LengthPercentageOrAuto<'ast>),
    ScrollMarginBottom(LengthPercentageOrAuto<'ast>),
    ScrollMarginLeft(LengthPercentageOrAuto<'ast>),
    ScrollMarginRight(LengthPercentageOrAuto<'ast>),
    ScrollMarginBlockStart(LengthPercentageOrAuto<'ast>),
    ScrollMarginBlockEnd(LengthPercentageOrAuto<'ast>),
    ScrollMarginInlineStart(LengthPercentageOrAuto<'ast>),
    ScrollMarginInlineEnd(LengthPercentageOrAuto<'ast>),
    ScrollMarginBlock(ScrollMarginBlock<'ast>),
    ScrollMarginInline(ScrollMarginInline<'ast>),
    ScrollMargin(ScrollMargin<'ast>),
    ScrollPaddingTop(LengthPercentageOrAuto<'ast>),
    ScrollPaddingBottom(LengthPercentageOrAuto<'ast>),
    ScrollPaddingLeft(LengthPercentageOrAuto<'ast>),
    ScrollPaddingRight(LengthPercentageOrAuto<'ast>),
    ScrollPaddingBlockStart(LengthPercentageOrAuto<'ast>),
    ScrollPaddingBlockEnd(LengthPercentageOrAuto<'ast>),
    ScrollPaddingInlineStart(LengthPercentageOrAuto<'ast>),
    ScrollPaddingInlineEnd(LengthPercentageOrAuto<'ast>),
    ScrollPaddingBlock(ScrollPaddingBlock<'ast>),
    ScrollPaddingInline(ScrollPaddingInline<'ast>),
    ScrollPadding(ScrollPadding<'ast>),
    FontWeight(FontWeight),
    FontSize(FontSize<'ast>),
    FontStretch(FontStretch),
    FontFamily(&'ast [FontFamily<'ast>]),
    FontStyle(FontStyle),
    FontVariantCaps(FontVariantCaps),
    LineHeight(LineHeight<'ast>),
    Font(Font<'ast>),
    VerticalAlign(VerticalAlign<'ast>),
    FontPalette(DashedIdentReference<'ast>),
    TransitionProperty(&'ast [PropertyId<'ast>], VendorPrefix),
    TransitionDuration(&'ast [Time], VendorPrefix),
    TransitionDelay(&'ast [Time], VendorPrefix),
    TransitionTimingFunction(&'ast [EasingFunction], VendorPrefix),
    Transition(&'ast [Transition<'ast>], VendorPrefix),
    AnimationName(&'ast [AnimationName<'ast>], VendorPrefix),
    AnimationDuration(&'ast [Time], VendorPrefix),
    AnimationTimingFunction(&'ast [EasingFunction], VendorPrefix),
    AnimationIterationCount(&'ast [AnimationIterationCount], VendorPrefix),
    AnimationDirection(&'ast [AnimationDirection], VendorPrefix),
    AnimationPlayState(&'ast [AnimationPlayState], VendorPrefix),
    AnimationDelay(&'ast [Time], VendorPrefix),
    AnimationFillMode(&'ast [AnimationFillMode], VendorPrefix),
    AnimationComposition(&'ast [AnimationComposition]),
    AnimationTimeline(&'ast [AnimationTimeline<'ast>]),
    AnimationRangeStart(&'ast [AnimationRangeStart<'ast>]),
    AnimationRangeEnd(&'ast [AnimationRangeEnd<'ast>]),
    AnimationRange(&'ast [AnimationRange<'ast>]),
    Animation(&'ast [Animation<'ast>], VendorPrefix),
    Transform(&'ast [Transform<'ast>], VendorPrefix),
    TransformOrigin(Position<'ast>, VendorPrefix),
    TransformStyle(TransformStyle, VendorPrefix),
    TransformBox(TransformBox),
    BackfaceVisibility(BackfaceVisibility, VendorPrefix),
    Perspective(Perspective<'ast>, VendorPrefix),
    PerspectiveOrigin(Position<'ast>, VendorPrefix),
    Translate(Translate<'ast>),
    Rotate(Rotate<'ast>),
    Scale(Scale),
    TextTransform(TextTransform<'ast>),
    Content(Content<'ast>),
    WhiteSpace(WhiteSpace),
    TabSize(LengthOrNumber<'ast>, VendorPrefix),
    WordBreak(WordBreak),
    LineBreak(LineBreak),
    Hyphens(Hyphens, VendorPrefix),
    OverflowWrap(OverflowWrap),
    WordWrap(OverflowWrap),
    TextAlign(TextAlign),
    TextAlignLast(TextAlignLast, VendorPrefix),
    TextJustify(TextJustify),
    WordSpacing(Spacing<'ast>),
    LetterSpacing(Spacing<'ast>),
    TextIndent(TextIndent<'ast>),
    TextDecorationLine(TextDecorationLine<'ast>, VendorPrefix),
    TextDecorationStyle(TextDecorationStyle, VendorPrefix),
    TextDecorationColor(CssColor<'ast>, VendorPrefix),
    TextDecorationThickness(TextDecorationThickness<'ast>),
    TextDecoration(TextDecoration<'ast>, VendorPrefix),
    TextDecorationSkipInk(TextDecorationSkipInk, VendorPrefix),
    TextEmphasisStyle(TextEmphasisStyle<'ast>, VendorPrefix),
    TextEmphasisColor(CssColor<'ast>, VendorPrefix),
    TextEmphasis(TextEmphasis<'ast>, VendorPrefix),
    TextEmphasisPosition(TextEmphasisPosition<'ast>, VendorPrefix),
    TextShadow(&'ast [TextShadow<'ast>]),
    TextSizeAdjust(TextSizeAdjust, VendorPrefix),
    Direction(TextDirection),
    UnicodeBidi(UnicodeBidi),
    BoxDecorationBreak(BoxDecorationBreak, VendorPrefix),
    Resize(Resize),
    PointerEvents(PointerEvents),
    Float(Float),
    Clear(Clear),
    TouchAction(TouchAction),
    ScrollBehavior(ScrollBehavior),
    Cursor(Cursor<'ast>),
    CaretColor(ColorOrAuto<'ast>),
    CaretShape(CaretShape),
    Caret(Caret<'ast>),
    UserSelect(UserSelect, VendorPrefix),
    AccentColor(ColorOrAuto<'ast>),
    Appearance(Appearance<'ast>, VendorPrefix),
    ListStyleType(ListStyleType<'ast>),
    ListStyleImage(Image<'ast>),
    ListStylePosition(ListStylePosition),
    ListStyle(ListStyle<'ast>),
    MarkerSide(MarkerSide),
    Composes(Composes<'ast>),
    Fill(SVGPaint<'ast>),
    FillRule(FillRule),
    FillOpacity(f32),
    Stroke(SVGPaint<'ast>),
    StrokeOpacity(f32),
    StrokeWidth(LengthPercentage<'ast>),
    StrokeLinecap(StrokeLinecap),
    StrokeLinejoin(StrokeLinejoin),
    StrokeMiterlimit(f32),
    StrokeDasharray(StrokeDasharray<'ast>),
    StrokeDashoffset(LengthPercentage<'ast>),
    MarkerStart(Marker<'ast>),
    MarkerMid(Marker<'ast>),
    MarkerEnd(Marker<'ast>),
    Marker(Marker<'ast>),
    ColorInterpolation(ColorInterpolation),
    ColorInterpolationFilters(ColorInterpolation),
    ColorRendering(ColorRendering),
    ShapeRendering(ShapeRendering),
    TextRendering(TextRendering),
    ImageRendering(ImageRendering),
    ClipPath(ClipPath<'ast>, VendorPrefix),
    ClipRule(FillRule),
    MaskImage(&'ast [Image<'ast>], VendorPrefix),
    MaskMode(&'ast [MaskMode]),
    MaskRepeat(&'ast [BackgroundRepeat<'ast>], VendorPrefix),
    MaskPositionX(&'ast [PositionComponent<'ast, HorizontalPositionKeyword>]),
    MaskPositionY(&'ast [PositionComponent<'ast, VerticalPositionKeyword>]),
    MaskPosition(&'ast [Position<'ast>], VendorPrefix),
    MaskClip(&'ast [MaskClip], VendorPrefix),
    MaskOrigin(&'ast [GeometryBox], VendorPrefix),
    MaskSize(&'ast [BackgroundSize<'ast>], VendorPrefix),
    MaskComposite(&'ast [MaskComposite]),
    MaskType(MaskType),
    Mask(&'ast [Mask<'ast>], VendorPrefix),
    MaskBorderSource(Image<'ast>),
    MaskBorderMode(MaskBorderMode),
    MaskBorderSlice(BorderImageSlice<'ast>),
    MaskBorderWidth(Rect<'ast, BorderImageSideWidth<'ast>>),
    MaskBorderOutset(Rect<'ast, LengthOrNumber<'ast>>),
    MaskBorderRepeat(BorderImageRepeat<'ast>),
    MaskBorder(MaskBorder<'ast>),
    WebKitMaskComposite(&'ast [WebKitMaskComposite]),
    WebKitMaskSourceType(&'ast [WebKitMaskSourceType], VendorPrefix),
    WebKitMaskBoxImage(BorderImage<'ast>, VendorPrefix),
    WebKitMaskBoxImageSource(Image<'ast>, VendorPrefix),
    WebKitMaskBoxImageSlice(BorderImageSlice<'ast>, VendorPrefix),
    WebKitMaskBoxImageWidth(Rect<'ast, BorderImageSideWidth<'ast>>, VendorPrefix),
    WebKitMaskBoxImageOutset(Rect<'ast, LengthOrNumber<'ast>>, VendorPrefix),
    WebKitMaskBoxImageRepeat(BorderImageRepeat<'ast>, VendorPrefix),
    Filter(FilterList<'ast>, VendorPrefix),
    BackdropFilter(FilterList<'ast>, VendorPrefix),
    MixBlendMode(BlendMode),
    ZIndex(ZIndex),
    ContainerType(ContainerType),
    ContainerName(ContainerNameList<'ast>),
    Container(Container<'ast>),
    ViewTransitionName(ViewTransitionName<'ast>),
    ViewTransitionClass(NoneOrCustomIdentList<'ast>),
    ViewTransitionGroup(ViewTransitionGroup<'ast>),
    ColorScheme(ColorScheme<'ast>),
    PrintColorAdjust(PrintColorAdjust, VendorPrefix),
    CSSWide(PropertyId<'ast>, CSSWideKeyword),
    Unparsed(UnparsedProperty<'ast>),
    Custom(CustomProperty<'ast>),
    Tombstone,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeclarationKind {
    All,
    BackgroundColor,
    BackgroundImage,
    BackgroundPositionX,
    BackgroundPositionY,
    BackgroundPosition,
    BackgroundSize,
    BackgroundRepeat,
    BackgroundAttachment,
    BackgroundClip,
    BackgroundOrigin,
    Background,
    BoxShadow,
    Opacity,
    Color,
    Display,
    Visibility,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    BlockSize,
    InlineSize,
    MinBlockSize,
    MinInlineSize,
    MaxBlockSize,
    MaxInlineSize,
    BoxSizing,
    AspectRatio,
    Overflow,
    OverflowX,
    OverflowY,
    TextOverflow,
    ObjectFit,
    ObjectPosition,
    ScrollbarColor,
    Position,
    Top,
    Bottom,
    Left,
    Right,
    InsetBlockStart,
    InsetBlockEnd,
    InsetInlineStart,
    InsetInlineEnd,
    InsetBlock,
    InsetInline,
    Inset,
    BorderSpacing,
    BorderTopColor,
    BorderBottomColor,
    BorderLeftColor,
    BorderRightColor,
    BorderBlockStartColor,
    BorderBlockEndColor,
    BorderInlineStartColor,
    BorderInlineEndColor,
    BorderTopStyle,
    BorderBottomStyle,
    BorderLeftStyle,
    BorderRightStyle,
    BorderBlockStartStyle,
    BorderBlockEndStyle,
    BorderInlineStartStyle,
    BorderInlineEndStyle,
    BorderTopWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    BorderRightWidth,
    BorderBlockStartWidth,
    BorderBlockEndWidth,
    BorderInlineStartWidth,
    BorderInlineEndWidth,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    BorderStartStartRadius,
    BorderStartEndRadius,
    BorderEndStartRadius,
    BorderEndEndRadius,
    BorderRadius,
    BorderImageSource,
    BorderImageOutset,
    BorderImageRepeat,
    BorderImageWidth,
    BorderImageSlice,
    BorderImage,
    BorderColor,
    BorderStyle,
    BorderWidth,
    BorderBlockColor,
    BorderBlockStyle,
    BorderBlockWidth,
    BorderInlineColor,
    BorderInlineStyle,
    BorderInlineWidth,
    Border,
    BorderTop,
    BorderBottom,
    BorderLeft,
    BorderRight,
    BorderBlock,
    BorderBlockStart,
    BorderBlockEnd,
    BorderInline,
    BorderInlineStart,
    BorderInlineEnd,
    Outline,
    OutlineColor,
    OutlineStyle,
    OutlineWidth,
    FlexDirection,
    FlexWrap,
    FlexFlow,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    Flex,
    Order,
    AlignContent,
    JustifyContent,
    PlaceContent,
    AlignSelf,
    JustifySelf,
    PlaceSelf,
    AlignItems,
    JustifyItems,
    PlaceItems,
    RowGap,
    ColumnGap,
    Gap,
    ColumnRule,
    ColumnWidth,
    ColumnCount,
    Columns,
    GridColumnGap,
    GridRowGap,
    BoxOrient,
    BoxDirection,
    BoxOrdinalGroup,
    BoxAlign,
    BoxFlex,
    BoxFlexGroup,
    BoxPack,
    BoxLines,
    FlexPack,
    FlexOrder,
    FlexAlign,
    FlexItemAlign,
    FlexLinePack,
    FlexPositive,
    FlexNegative,
    FlexPreferredSize,
    GridTemplateColumns,
    GridTemplateRows,
    GridAutoColumns,
    GridAutoRows,
    GridAutoFlow,
    GridTemplateAreas,
    GridTemplate,
    Grid,
    GridRowStart,
    GridRowEnd,
    GridColumnStart,
    GridColumnEnd,
    GridRow,
    GridColumn,
    GridArea,
    MarginTop,
    MarginBottom,
    MarginLeft,
    MarginRight,
    MarginBlockStart,
    MarginBlockEnd,
    MarginInlineStart,
    MarginInlineEnd,
    MarginBlock,
    MarginInline,
    Margin,
    PaddingTop,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    PaddingBlockStart,
    PaddingBlockEnd,
    PaddingInlineStart,
    PaddingInlineEnd,
    PaddingBlock,
    PaddingInline,
    Padding,
    ScrollMarginTop,
    ScrollMarginBottom,
    ScrollMarginLeft,
    ScrollMarginRight,
    ScrollMarginBlockStart,
    ScrollMarginBlockEnd,
    ScrollMarginInlineStart,
    ScrollMarginInlineEnd,
    ScrollMarginBlock,
    ScrollMarginInline,
    ScrollMargin,
    ScrollPaddingTop,
    ScrollPaddingBottom,
    ScrollPaddingLeft,
    ScrollPaddingRight,
    ScrollPaddingBlockStart,
    ScrollPaddingBlockEnd,
    ScrollPaddingInlineStart,
    ScrollPaddingInlineEnd,
    ScrollPaddingBlock,
    ScrollPaddingInline,
    ScrollPadding,
    FontWeight,
    FontSize,
    FontStretch,
    FontFamily,
    FontStyle,
    FontVariantCaps,
    LineHeight,
    Font,
    VerticalAlign,
    FontPalette,
    TransitionProperty,
    TransitionDuration,
    TransitionDelay,
    TransitionTimingFunction,
    Transition,
    AnimationName,
    AnimationDuration,
    AnimationTimingFunction,
    AnimationIterationCount,
    AnimationDirection,
    AnimationPlayState,
    AnimationDelay,
    AnimationFillMode,
    AnimationComposition,
    AnimationTimeline,
    AnimationRangeStart,
    AnimationRangeEnd,
    AnimationRange,
    Animation,
    Transform,
    TransformOrigin,
    TransformStyle,
    TransformBox,
    BackfaceVisibility,
    Perspective,
    PerspectiveOrigin,
    Translate,
    Rotate,
    Scale,
    TextTransform,
    Content,
    WhiteSpace,
    TabSize,
    WordBreak,
    LineBreak,
    Hyphens,
    OverflowWrap,
    WordWrap,
    TextAlign,
    TextAlignLast,
    TextJustify,
    WordSpacing,
    LetterSpacing,
    TextIndent,
    TextDecorationLine,
    TextDecorationStyle,
    TextDecorationColor,
    TextDecorationThickness,
    TextDecoration,
    TextDecorationSkipInk,
    TextEmphasisStyle,
    TextEmphasisColor,
    TextEmphasis,
    TextEmphasisPosition,
    TextShadow,
    TextSizeAdjust,
    Direction,
    UnicodeBidi,
    BoxDecorationBreak,
    Resize,
    PointerEvents,
    Float,
    Clear,
    TouchAction,
    ScrollBehavior,
    Cursor,
    CaretColor,
    CaretShape,
    Caret,
    UserSelect,
    AccentColor,
    Appearance,
    ListStyleType,
    ListStyleImage,
    ListStylePosition,
    ListStyle,
    MarkerSide,
    Composes,
    Fill,
    FillRule,
    FillOpacity,
    Stroke,
    StrokeOpacity,
    StrokeWidth,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeDasharray,
    StrokeDashoffset,
    MarkerStart,
    MarkerMid,
    MarkerEnd,
    Marker,
    ColorInterpolation,
    ColorInterpolationFilters,
    ColorRendering,
    ShapeRendering,
    TextRendering,
    ImageRendering,
    ClipPath,
    ClipRule,
    MaskImage,
    MaskMode,
    MaskRepeat,
    MaskPositionX,
    MaskPositionY,
    MaskPosition,
    MaskClip,
    MaskOrigin,
    MaskSize,
    MaskComposite,
    MaskType,
    Mask,
    MaskBorderSource,
    MaskBorderMode,
    MaskBorderSlice,
    MaskBorderWidth,
    MaskBorderOutset,
    MaskBorderRepeat,
    MaskBorder,
    WebKitMaskComposite,
    WebKitMaskSourceType,
    WebKitMaskBoxImage,
    WebKitMaskBoxImageSource,
    WebKitMaskBoxImageSlice,
    WebKitMaskBoxImageWidth,
    WebKitMaskBoxImageOutset,
    WebKitMaskBoxImageRepeat,
    Filter,
    BackdropFilter,
    MixBlendMode,
    ZIndex,
    ContainerType,
    ContainerName,
    Container,
    ViewTransitionName,
    ViewTransitionClass,
    ViewTransitionGroup,
    ColorScheme,
    PrintColorAdjust,
    CSSWide,
    Unparsed,
    Custom,
    Tombstone,
}
impl<'ast> NodeStorage<'ast> for Declaration<'ast> {
    const KIND: NodeKind = super::kind::KIND_DECLARATION;
    type Data = DeclarationData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, DeclarationData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, DeclarationData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Declaration<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> DeclarationData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> DeclarationKind {
        match ast.data(self) {
            DeclarationData::All { .. } => DeclarationKind::All,
            DeclarationData::BackgroundColor { .. } => DeclarationKind::BackgroundColor,
            DeclarationData::BackgroundImage { .. } => DeclarationKind::BackgroundImage,
            DeclarationData::BackgroundPositionX { .. } => DeclarationKind::BackgroundPositionX,
            DeclarationData::BackgroundPositionY { .. } => DeclarationKind::BackgroundPositionY,
            DeclarationData::BackgroundPosition { .. } => DeclarationKind::BackgroundPosition,
            DeclarationData::BackgroundSize { .. } => DeclarationKind::BackgroundSize,
            DeclarationData::BackgroundRepeat { .. } => DeclarationKind::BackgroundRepeat,
            DeclarationData::BackgroundAttachment { .. } => DeclarationKind::BackgroundAttachment,
            DeclarationData::BackgroundClip { .. } => DeclarationKind::BackgroundClip,
            DeclarationData::BackgroundOrigin { .. } => DeclarationKind::BackgroundOrigin,
            DeclarationData::Background { .. } => DeclarationKind::Background,
            DeclarationData::BoxShadow { .. } => DeclarationKind::BoxShadow,
            DeclarationData::Opacity { .. } => DeclarationKind::Opacity,
            DeclarationData::Color { .. } => DeclarationKind::Color,
            DeclarationData::Display { .. } => DeclarationKind::Display,
            DeclarationData::Visibility { .. } => DeclarationKind::Visibility,
            DeclarationData::Width { .. } => DeclarationKind::Width,
            DeclarationData::Height { .. } => DeclarationKind::Height,
            DeclarationData::MinWidth { .. } => DeclarationKind::MinWidth,
            DeclarationData::MinHeight { .. } => DeclarationKind::MinHeight,
            DeclarationData::MaxWidth { .. } => DeclarationKind::MaxWidth,
            DeclarationData::MaxHeight { .. } => DeclarationKind::MaxHeight,
            DeclarationData::BlockSize { .. } => DeclarationKind::BlockSize,
            DeclarationData::InlineSize { .. } => DeclarationKind::InlineSize,
            DeclarationData::MinBlockSize { .. } => DeclarationKind::MinBlockSize,
            DeclarationData::MinInlineSize { .. } => DeclarationKind::MinInlineSize,
            DeclarationData::MaxBlockSize { .. } => DeclarationKind::MaxBlockSize,
            DeclarationData::MaxInlineSize { .. } => DeclarationKind::MaxInlineSize,
            DeclarationData::BoxSizing { .. } => DeclarationKind::BoxSizing,
            DeclarationData::AspectRatio { .. } => DeclarationKind::AspectRatio,
            DeclarationData::Overflow { .. } => DeclarationKind::Overflow,
            DeclarationData::OverflowX { .. } => DeclarationKind::OverflowX,
            DeclarationData::OverflowY { .. } => DeclarationKind::OverflowY,
            DeclarationData::TextOverflow { .. } => DeclarationKind::TextOverflow,
            DeclarationData::ObjectFit { .. } => DeclarationKind::ObjectFit,
            DeclarationData::ObjectPosition { .. } => DeclarationKind::ObjectPosition,
            DeclarationData::ScrollbarColor { .. } => DeclarationKind::ScrollbarColor,
            DeclarationData::Position { .. } => DeclarationKind::Position,
            DeclarationData::Top { .. } => DeclarationKind::Top,
            DeclarationData::Bottom { .. } => DeclarationKind::Bottom,
            DeclarationData::Left { .. } => DeclarationKind::Left,
            DeclarationData::Right { .. } => DeclarationKind::Right,
            DeclarationData::InsetBlockStart { .. } => DeclarationKind::InsetBlockStart,
            DeclarationData::InsetBlockEnd { .. } => DeclarationKind::InsetBlockEnd,
            DeclarationData::InsetInlineStart { .. } => DeclarationKind::InsetInlineStart,
            DeclarationData::InsetInlineEnd { .. } => DeclarationKind::InsetInlineEnd,
            DeclarationData::InsetBlock { .. } => DeclarationKind::InsetBlock,
            DeclarationData::InsetInline { .. } => DeclarationKind::InsetInline,
            DeclarationData::Inset { .. } => DeclarationKind::Inset,
            DeclarationData::BorderSpacing { .. } => DeclarationKind::BorderSpacing,
            DeclarationData::BorderTopColor { .. } => DeclarationKind::BorderTopColor,
            DeclarationData::BorderBottomColor { .. } => DeclarationKind::BorderBottomColor,
            DeclarationData::BorderLeftColor { .. } => DeclarationKind::BorderLeftColor,
            DeclarationData::BorderRightColor { .. } => DeclarationKind::BorderRightColor,
            DeclarationData::BorderBlockStartColor { .. } => DeclarationKind::BorderBlockStartColor,
            DeclarationData::BorderBlockEndColor { .. } => DeclarationKind::BorderBlockEndColor,
            DeclarationData::BorderInlineStartColor { .. } => {
                DeclarationKind::BorderInlineStartColor
            }
            DeclarationData::BorderInlineEndColor { .. } => DeclarationKind::BorderInlineEndColor,
            DeclarationData::BorderTopStyle { .. } => DeclarationKind::BorderTopStyle,
            DeclarationData::BorderBottomStyle { .. } => DeclarationKind::BorderBottomStyle,
            DeclarationData::BorderLeftStyle { .. } => DeclarationKind::BorderLeftStyle,
            DeclarationData::BorderRightStyle { .. } => DeclarationKind::BorderRightStyle,
            DeclarationData::BorderBlockStartStyle { .. } => DeclarationKind::BorderBlockStartStyle,
            DeclarationData::BorderBlockEndStyle { .. } => DeclarationKind::BorderBlockEndStyle,
            DeclarationData::BorderInlineStartStyle { .. } => {
                DeclarationKind::BorderInlineStartStyle
            }
            DeclarationData::BorderInlineEndStyle { .. } => DeclarationKind::BorderInlineEndStyle,
            DeclarationData::BorderTopWidth { .. } => DeclarationKind::BorderTopWidth,
            DeclarationData::BorderBottomWidth { .. } => DeclarationKind::BorderBottomWidth,
            DeclarationData::BorderLeftWidth { .. } => DeclarationKind::BorderLeftWidth,
            DeclarationData::BorderRightWidth { .. } => DeclarationKind::BorderRightWidth,
            DeclarationData::BorderBlockStartWidth { .. } => DeclarationKind::BorderBlockStartWidth,
            DeclarationData::BorderBlockEndWidth { .. } => DeclarationKind::BorderBlockEndWidth,
            DeclarationData::BorderInlineStartWidth { .. } => {
                DeclarationKind::BorderInlineStartWidth
            }
            DeclarationData::BorderInlineEndWidth { .. } => DeclarationKind::BorderInlineEndWidth,
            DeclarationData::BorderTopLeftRadius { .. } => DeclarationKind::BorderTopLeftRadius,
            DeclarationData::BorderTopRightRadius { .. } => DeclarationKind::BorderTopRightRadius,
            DeclarationData::BorderBottomLeftRadius { .. } => {
                DeclarationKind::BorderBottomLeftRadius
            }
            DeclarationData::BorderBottomRightRadius { .. } => {
                DeclarationKind::BorderBottomRightRadius
            }
            DeclarationData::BorderStartStartRadius { .. } => {
                DeclarationKind::BorderStartStartRadius
            }
            DeclarationData::BorderStartEndRadius { .. } => DeclarationKind::BorderStartEndRadius,
            DeclarationData::BorderEndStartRadius { .. } => DeclarationKind::BorderEndStartRadius,
            DeclarationData::BorderEndEndRadius { .. } => DeclarationKind::BorderEndEndRadius,
            DeclarationData::BorderRadius { .. } => DeclarationKind::BorderRadius,
            DeclarationData::BorderImageSource { .. } => DeclarationKind::BorderImageSource,
            DeclarationData::BorderImageOutset { .. } => DeclarationKind::BorderImageOutset,
            DeclarationData::BorderImageRepeat { .. } => DeclarationKind::BorderImageRepeat,
            DeclarationData::BorderImageWidth { .. } => DeclarationKind::BorderImageWidth,
            DeclarationData::BorderImageSlice { .. } => DeclarationKind::BorderImageSlice,
            DeclarationData::BorderImage { .. } => DeclarationKind::BorderImage,
            DeclarationData::BorderColor { .. } => DeclarationKind::BorderColor,
            DeclarationData::BorderStyle { .. } => DeclarationKind::BorderStyle,
            DeclarationData::BorderWidth { .. } => DeclarationKind::BorderWidth,
            DeclarationData::BorderBlockColor { .. } => DeclarationKind::BorderBlockColor,
            DeclarationData::BorderBlockStyle { .. } => DeclarationKind::BorderBlockStyle,
            DeclarationData::BorderBlockWidth { .. } => DeclarationKind::BorderBlockWidth,
            DeclarationData::BorderInlineColor { .. } => DeclarationKind::BorderInlineColor,
            DeclarationData::BorderInlineStyle { .. } => DeclarationKind::BorderInlineStyle,
            DeclarationData::BorderInlineWidth { .. } => DeclarationKind::BorderInlineWidth,
            DeclarationData::Border { .. } => DeclarationKind::Border,
            DeclarationData::BorderTop { .. } => DeclarationKind::BorderTop,
            DeclarationData::BorderBottom { .. } => DeclarationKind::BorderBottom,
            DeclarationData::BorderLeft { .. } => DeclarationKind::BorderLeft,
            DeclarationData::BorderRight { .. } => DeclarationKind::BorderRight,
            DeclarationData::BorderBlock { .. } => DeclarationKind::BorderBlock,
            DeclarationData::BorderBlockStart { .. } => DeclarationKind::BorderBlockStart,
            DeclarationData::BorderBlockEnd { .. } => DeclarationKind::BorderBlockEnd,
            DeclarationData::BorderInline { .. } => DeclarationKind::BorderInline,
            DeclarationData::BorderInlineStart { .. } => DeclarationKind::BorderInlineStart,
            DeclarationData::BorderInlineEnd { .. } => DeclarationKind::BorderInlineEnd,
            DeclarationData::Outline { .. } => DeclarationKind::Outline,
            DeclarationData::OutlineColor { .. } => DeclarationKind::OutlineColor,
            DeclarationData::OutlineStyle { .. } => DeclarationKind::OutlineStyle,
            DeclarationData::OutlineWidth { .. } => DeclarationKind::OutlineWidth,
            DeclarationData::FlexDirection { .. } => DeclarationKind::FlexDirection,
            DeclarationData::FlexWrap { .. } => DeclarationKind::FlexWrap,
            DeclarationData::FlexFlow { .. } => DeclarationKind::FlexFlow,
            DeclarationData::FlexGrow { .. } => DeclarationKind::FlexGrow,
            DeclarationData::FlexShrink { .. } => DeclarationKind::FlexShrink,
            DeclarationData::FlexBasis { .. } => DeclarationKind::FlexBasis,
            DeclarationData::Flex { .. } => DeclarationKind::Flex,
            DeclarationData::Order { .. } => DeclarationKind::Order,
            DeclarationData::AlignContent { .. } => DeclarationKind::AlignContent,
            DeclarationData::JustifyContent { .. } => DeclarationKind::JustifyContent,
            DeclarationData::PlaceContent { .. } => DeclarationKind::PlaceContent,
            DeclarationData::AlignSelf { .. } => DeclarationKind::AlignSelf,
            DeclarationData::JustifySelf { .. } => DeclarationKind::JustifySelf,
            DeclarationData::PlaceSelf { .. } => DeclarationKind::PlaceSelf,
            DeclarationData::AlignItems { .. } => DeclarationKind::AlignItems,
            DeclarationData::JustifyItems { .. } => DeclarationKind::JustifyItems,
            DeclarationData::PlaceItems { .. } => DeclarationKind::PlaceItems,
            DeclarationData::RowGap { .. } => DeclarationKind::RowGap,
            DeclarationData::ColumnGap { .. } => DeclarationKind::ColumnGap,
            DeclarationData::Gap { .. } => DeclarationKind::Gap,
            DeclarationData::ColumnRule { .. } => DeclarationKind::ColumnRule,
            DeclarationData::ColumnWidth { .. } => DeclarationKind::ColumnWidth,
            DeclarationData::ColumnCount { .. } => DeclarationKind::ColumnCount,
            DeclarationData::Columns { .. } => DeclarationKind::Columns,
            DeclarationData::GridColumnGap { .. } => DeclarationKind::GridColumnGap,
            DeclarationData::GridRowGap { .. } => DeclarationKind::GridRowGap,
            DeclarationData::BoxOrient { .. } => DeclarationKind::BoxOrient,
            DeclarationData::BoxDirection { .. } => DeclarationKind::BoxDirection,
            DeclarationData::BoxOrdinalGroup { .. } => DeclarationKind::BoxOrdinalGroup,
            DeclarationData::BoxAlign { .. } => DeclarationKind::BoxAlign,
            DeclarationData::BoxFlex { .. } => DeclarationKind::BoxFlex,
            DeclarationData::BoxFlexGroup { .. } => DeclarationKind::BoxFlexGroup,
            DeclarationData::BoxPack { .. } => DeclarationKind::BoxPack,
            DeclarationData::BoxLines { .. } => DeclarationKind::BoxLines,
            DeclarationData::FlexPack { .. } => DeclarationKind::FlexPack,
            DeclarationData::FlexOrder { .. } => DeclarationKind::FlexOrder,
            DeclarationData::FlexAlign { .. } => DeclarationKind::FlexAlign,
            DeclarationData::FlexItemAlign { .. } => DeclarationKind::FlexItemAlign,
            DeclarationData::FlexLinePack { .. } => DeclarationKind::FlexLinePack,
            DeclarationData::FlexPositive { .. } => DeclarationKind::FlexPositive,
            DeclarationData::FlexNegative { .. } => DeclarationKind::FlexNegative,
            DeclarationData::FlexPreferredSize { .. } => DeclarationKind::FlexPreferredSize,
            DeclarationData::GridTemplateColumns { .. } => DeclarationKind::GridTemplateColumns,
            DeclarationData::GridTemplateRows { .. } => DeclarationKind::GridTemplateRows,
            DeclarationData::GridAutoColumns { .. } => DeclarationKind::GridAutoColumns,
            DeclarationData::GridAutoRows { .. } => DeclarationKind::GridAutoRows,
            DeclarationData::GridAutoFlow { .. } => DeclarationKind::GridAutoFlow,
            DeclarationData::GridTemplateAreas { .. } => DeclarationKind::GridTemplateAreas,
            DeclarationData::GridTemplate { .. } => DeclarationKind::GridTemplate,
            DeclarationData::Grid { .. } => DeclarationKind::Grid,
            DeclarationData::GridRowStart { .. } => DeclarationKind::GridRowStart,
            DeclarationData::GridRowEnd { .. } => DeclarationKind::GridRowEnd,
            DeclarationData::GridColumnStart { .. } => DeclarationKind::GridColumnStart,
            DeclarationData::GridColumnEnd { .. } => DeclarationKind::GridColumnEnd,
            DeclarationData::GridRow { .. } => DeclarationKind::GridRow,
            DeclarationData::GridColumn { .. } => DeclarationKind::GridColumn,
            DeclarationData::GridArea { .. } => DeclarationKind::GridArea,
            DeclarationData::MarginTop { .. } => DeclarationKind::MarginTop,
            DeclarationData::MarginBottom { .. } => DeclarationKind::MarginBottom,
            DeclarationData::MarginLeft { .. } => DeclarationKind::MarginLeft,
            DeclarationData::MarginRight { .. } => DeclarationKind::MarginRight,
            DeclarationData::MarginBlockStart { .. } => DeclarationKind::MarginBlockStart,
            DeclarationData::MarginBlockEnd { .. } => DeclarationKind::MarginBlockEnd,
            DeclarationData::MarginInlineStart { .. } => DeclarationKind::MarginInlineStart,
            DeclarationData::MarginInlineEnd { .. } => DeclarationKind::MarginInlineEnd,
            DeclarationData::MarginBlock { .. } => DeclarationKind::MarginBlock,
            DeclarationData::MarginInline { .. } => DeclarationKind::MarginInline,
            DeclarationData::Margin { .. } => DeclarationKind::Margin,
            DeclarationData::PaddingTop { .. } => DeclarationKind::PaddingTop,
            DeclarationData::PaddingBottom { .. } => DeclarationKind::PaddingBottom,
            DeclarationData::PaddingLeft { .. } => DeclarationKind::PaddingLeft,
            DeclarationData::PaddingRight { .. } => DeclarationKind::PaddingRight,
            DeclarationData::PaddingBlockStart { .. } => DeclarationKind::PaddingBlockStart,
            DeclarationData::PaddingBlockEnd { .. } => DeclarationKind::PaddingBlockEnd,
            DeclarationData::PaddingInlineStart { .. } => DeclarationKind::PaddingInlineStart,
            DeclarationData::PaddingInlineEnd { .. } => DeclarationKind::PaddingInlineEnd,
            DeclarationData::PaddingBlock { .. } => DeclarationKind::PaddingBlock,
            DeclarationData::PaddingInline { .. } => DeclarationKind::PaddingInline,
            DeclarationData::Padding { .. } => DeclarationKind::Padding,
            DeclarationData::ScrollMarginTop { .. } => DeclarationKind::ScrollMarginTop,
            DeclarationData::ScrollMarginBottom { .. } => DeclarationKind::ScrollMarginBottom,
            DeclarationData::ScrollMarginLeft { .. } => DeclarationKind::ScrollMarginLeft,
            DeclarationData::ScrollMarginRight { .. } => DeclarationKind::ScrollMarginRight,
            DeclarationData::ScrollMarginBlockStart { .. } => {
                DeclarationKind::ScrollMarginBlockStart
            }
            DeclarationData::ScrollMarginBlockEnd { .. } => DeclarationKind::ScrollMarginBlockEnd,
            DeclarationData::ScrollMarginInlineStart { .. } => {
                DeclarationKind::ScrollMarginInlineStart
            }
            DeclarationData::ScrollMarginInlineEnd { .. } => DeclarationKind::ScrollMarginInlineEnd,
            DeclarationData::ScrollMarginBlock { .. } => DeclarationKind::ScrollMarginBlock,
            DeclarationData::ScrollMarginInline { .. } => DeclarationKind::ScrollMarginInline,
            DeclarationData::ScrollMargin { .. } => DeclarationKind::ScrollMargin,
            DeclarationData::ScrollPaddingTop { .. } => DeclarationKind::ScrollPaddingTop,
            DeclarationData::ScrollPaddingBottom { .. } => DeclarationKind::ScrollPaddingBottom,
            DeclarationData::ScrollPaddingLeft { .. } => DeclarationKind::ScrollPaddingLeft,
            DeclarationData::ScrollPaddingRight { .. } => DeclarationKind::ScrollPaddingRight,
            DeclarationData::ScrollPaddingBlockStart { .. } => {
                DeclarationKind::ScrollPaddingBlockStart
            }
            DeclarationData::ScrollPaddingBlockEnd { .. } => DeclarationKind::ScrollPaddingBlockEnd,
            DeclarationData::ScrollPaddingInlineStart { .. } => {
                DeclarationKind::ScrollPaddingInlineStart
            }
            DeclarationData::ScrollPaddingInlineEnd { .. } => {
                DeclarationKind::ScrollPaddingInlineEnd
            }
            DeclarationData::ScrollPaddingBlock { .. } => DeclarationKind::ScrollPaddingBlock,
            DeclarationData::ScrollPaddingInline { .. } => DeclarationKind::ScrollPaddingInline,
            DeclarationData::ScrollPadding { .. } => DeclarationKind::ScrollPadding,
            DeclarationData::FontWeight { .. } => DeclarationKind::FontWeight,
            DeclarationData::FontSize { .. } => DeclarationKind::FontSize,
            DeclarationData::FontStretch { .. } => DeclarationKind::FontStretch,
            DeclarationData::FontFamily { .. } => DeclarationKind::FontFamily,
            DeclarationData::FontStyle { .. } => DeclarationKind::FontStyle,
            DeclarationData::FontVariantCaps { .. } => DeclarationKind::FontVariantCaps,
            DeclarationData::LineHeight { .. } => DeclarationKind::LineHeight,
            DeclarationData::Font { .. } => DeclarationKind::Font,
            DeclarationData::VerticalAlign { .. } => DeclarationKind::VerticalAlign,
            DeclarationData::FontPalette { .. } => DeclarationKind::FontPalette,
            DeclarationData::TransitionProperty { .. } => DeclarationKind::TransitionProperty,
            DeclarationData::TransitionDuration { .. } => DeclarationKind::TransitionDuration,
            DeclarationData::TransitionDelay { .. } => DeclarationKind::TransitionDelay,
            DeclarationData::TransitionTimingFunction { .. } => {
                DeclarationKind::TransitionTimingFunction
            }
            DeclarationData::Transition { .. } => DeclarationKind::Transition,
            DeclarationData::AnimationName { .. } => DeclarationKind::AnimationName,
            DeclarationData::AnimationDuration { .. } => DeclarationKind::AnimationDuration,
            DeclarationData::AnimationTimingFunction { .. } => {
                DeclarationKind::AnimationTimingFunction
            }
            DeclarationData::AnimationIterationCount { .. } => {
                DeclarationKind::AnimationIterationCount
            }
            DeclarationData::AnimationDirection { .. } => DeclarationKind::AnimationDirection,
            DeclarationData::AnimationPlayState { .. } => DeclarationKind::AnimationPlayState,
            DeclarationData::AnimationDelay { .. } => DeclarationKind::AnimationDelay,
            DeclarationData::AnimationFillMode { .. } => DeclarationKind::AnimationFillMode,
            DeclarationData::AnimationComposition { .. } => DeclarationKind::AnimationComposition,
            DeclarationData::AnimationTimeline { .. } => DeclarationKind::AnimationTimeline,
            DeclarationData::AnimationRangeStart { .. } => DeclarationKind::AnimationRangeStart,
            DeclarationData::AnimationRangeEnd { .. } => DeclarationKind::AnimationRangeEnd,
            DeclarationData::AnimationRange { .. } => DeclarationKind::AnimationRange,
            DeclarationData::Animation { .. } => DeclarationKind::Animation,
            DeclarationData::Transform { .. } => DeclarationKind::Transform,
            DeclarationData::TransformOrigin { .. } => DeclarationKind::TransformOrigin,
            DeclarationData::TransformStyle { .. } => DeclarationKind::TransformStyle,
            DeclarationData::TransformBox { .. } => DeclarationKind::TransformBox,
            DeclarationData::BackfaceVisibility { .. } => DeclarationKind::BackfaceVisibility,
            DeclarationData::Perspective { .. } => DeclarationKind::Perspective,
            DeclarationData::PerspectiveOrigin { .. } => DeclarationKind::PerspectiveOrigin,
            DeclarationData::Translate { .. } => DeclarationKind::Translate,
            DeclarationData::Rotate { .. } => DeclarationKind::Rotate,
            DeclarationData::Scale { .. } => DeclarationKind::Scale,
            DeclarationData::TextTransform { .. } => DeclarationKind::TextTransform,
            DeclarationData::Content { .. } => DeclarationKind::Content,
            DeclarationData::WhiteSpace { .. } => DeclarationKind::WhiteSpace,
            DeclarationData::TabSize { .. } => DeclarationKind::TabSize,
            DeclarationData::WordBreak { .. } => DeclarationKind::WordBreak,
            DeclarationData::LineBreak { .. } => DeclarationKind::LineBreak,
            DeclarationData::Hyphens { .. } => DeclarationKind::Hyphens,
            DeclarationData::OverflowWrap { .. } => DeclarationKind::OverflowWrap,
            DeclarationData::WordWrap { .. } => DeclarationKind::WordWrap,
            DeclarationData::TextAlign { .. } => DeclarationKind::TextAlign,
            DeclarationData::TextAlignLast { .. } => DeclarationKind::TextAlignLast,
            DeclarationData::TextJustify { .. } => DeclarationKind::TextJustify,
            DeclarationData::WordSpacing { .. } => DeclarationKind::WordSpacing,
            DeclarationData::LetterSpacing { .. } => DeclarationKind::LetterSpacing,
            DeclarationData::TextIndent { .. } => DeclarationKind::TextIndent,
            DeclarationData::TextDecorationLine { .. } => DeclarationKind::TextDecorationLine,
            DeclarationData::TextDecorationStyle { .. } => DeclarationKind::TextDecorationStyle,
            DeclarationData::TextDecorationColor { .. } => DeclarationKind::TextDecorationColor,
            DeclarationData::TextDecorationThickness { .. } => {
                DeclarationKind::TextDecorationThickness
            }
            DeclarationData::TextDecoration { .. } => DeclarationKind::TextDecoration,
            DeclarationData::TextDecorationSkipInk { .. } => DeclarationKind::TextDecorationSkipInk,
            DeclarationData::TextEmphasisStyle { .. } => DeclarationKind::TextEmphasisStyle,
            DeclarationData::TextEmphasisColor { .. } => DeclarationKind::TextEmphasisColor,
            DeclarationData::TextEmphasis { .. } => DeclarationKind::TextEmphasis,
            DeclarationData::TextEmphasisPosition { .. } => DeclarationKind::TextEmphasisPosition,
            DeclarationData::TextShadow { .. } => DeclarationKind::TextShadow,
            DeclarationData::TextSizeAdjust { .. } => DeclarationKind::TextSizeAdjust,
            DeclarationData::Direction { .. } => DeclarationKind::Direction,
            DeclarationData::UnicodeBidi { .. } => DeclarationKind::UnicodeBidi,
            DeclarationData::BoxDecorationBreak { .. } => DeclarationKind::BoxDecorationBreak,
            DeclarationData::Resize { .. } => DeclarationKind::Resize,
            DeclarationData::PointerEvents { .. } => DeclarationKind::PointerEvents,
            DeclarationData::Float { .. } => DeclarationKind::Float,
            DeclarationData::Clear { .. } => DeclarationKind::Clear,
            DeclarationData::TouchAction { .. } => DeclarationKind::TouchAction,
            DeclarationData::ScrollBehavior { .. } => DeclarationKind::ScrollBehavior,
            DeclarationData::Cursor { .. } => DeclarationKind::Cursor,
            DeclarationData::CaretColor { .. } => DeclarationKind::CaretColor,
            DeclarationData::CaretShape { .. } => DeclarationKind::CaretShape,
            DeclarationData::Caret { .. } => DeclarationKind::Caret,
            DeclarationData::UserSelect { .. } => DeclarationKind::UserSelect,
            DeclarationData::AccentColor { .. } => DeclarationKind::AccentColor,
            DeclarationData::Appearance { .. } => DeclarationKind::Appearance,
            DeclarationData::ListStyleType { .. } => DeclarationKind::ListStyleType,
            DeclarationData::ListStyleImage { .. } => DeclarationKind::ListStyleImage,
            DeclarationData::ListStylePosition { .. } => DeclarationKind::ListStylePosition,
            DeclarationData::ListStyle { .. } => DeclarationKind::ListStyle,
            DeclarationData::MarkerSide { .. } => DeclarationKind::MarkerSide,
            DeclarationData::Composes { .. } => DeclarationKind::Composes,
            DeclarationData::Fill { .. } => DeclarationKind::Fill,
            DeclarationData::FillRule { .. } => DeclarationKind::FillRule,
            DeclarationData::FillOpacity { .. } => DeclarationKind::FillOpacity,
            DeclarationData::Stroke { .. } => DeclarationKind::Stroke,
            DeclarationData::StrokeOpacity { .. } => DeclarationKind::StrokeOpacity,
            DeclarationData::StrokeWidth { .. } => DeclarationKind::StrokeWidth,
            DeclarationData::StrokeLinecap { .. } => DeclarationKind::StrokeLinecap,
            DeclarationData::StrokeLinejoin { .. } => DeclarationKind::StrokeLinejoin,
            DeclarationData::StrokeMiterlimit { .. } => DeclarationKind::StrokeMiterlimit,
            DeclarationData::StrokeDasharray { .. } => DeclarationKind::StrokeDasharray,
            DeclarationData::StrokeDashoffset { .. } => DeclarationKind::StrokeDashoffset,
            DeclarationData::MarkerStart { .. } => DeclarationKind::MarkerStart,
            DeclarationData::MarkerMid { .. } => DeclarationKind::MarkerMid,
            DeclarationData::MarkerEnd { .. } => DeclarationKind::MarkerEnd,
            DeclarationData::Marker { .. } => DeclarationKind::Marker,
            DeclarationData::ColorInterpolation { .. } => DeclarationKind::ColorInterpolation,
            DeclarationData::ColorInterpolationFilters { .. } => {
                DeclarationKind::ColorInterpolationFilters
            }
            DeclarationData::ColorRendering { .. } => DeclarationKind::ColorRendering,
            DeclarationData::ShapeRendering { .. } => DeclarationKind::ShapeRendering,
            DeclarationData::TextRendering { .. } => DeclarationKind::TextRendering,
            DeclarationData::ImageRendering { .. } => DeclarationKind::ImageRendering,
            DeclarationData::ClipPath { .. } => DeclarationKind::ClipPath,
            DeclarationData::ClipRule { .. } => DeclarationKind::ClipRule,
            DeclarationData::MaskImage { .. } => DeclarationKind::MaskImage,
            DeclarationData::MaskMode { .. } => DeclarationKind::MaskMode,
            DeclarationData::MaskRepeat { .. } => DeclarationKind::MaskRepeat,
            DeclarationData::MaskPositionX { .. } => DeclarationKind::MaskPositionX,
            DeclarationData::MaskPositionY { .. } => DeclarationKind::MaskPositionY,
            DeclarationData::MaskPosition { .. } => DeclarationKind::MaskPosition,
            DeclarationData::MaskClip { .. } => DeclarationKind::MaskClip,
            DeclarationData::MaskOrigin { .. } => DeclarationKind::MaskOrigin,
            DeclarationData::MaskSize { .. } => DeclarationKind::MaskSize,
            DeclarationData::MaskComposite { .. } => DeclarationKind::MaskComposite,
            DeclarationData::MaskType { .. } => DeclarationKind::MaskType,
            DeclarationData::Mask { .. } => DeclarationKind::Mask,
            DeclarationData::MaskBorderSource { .. } => DeclarationKind::MaskBorderSource,
            DeclarationData::MaskBorderMode { .. } => DeclarationKind::MaskBorderMode,
            DeclarationData::MaskBorderSlice { .. } => DeclarationKind::MaskBorderSlice,
            DeclarationData::MaskBorderWidth { .. } => DeclarationKind::MaskBorderWidth,
            DeclarationData::MaskBorderOutset { .. } => DeclarationKind::MaskBorderOutset,
            DeclarationData::MaskBorderRepeat { .. } => DeclarationKind::MaskBorderRepeat,
            DeclarationData::MaskBorder { .. } => DeclarationKind::MaskBorder,
            DeclarationData::WebKitMaskComposite { .. } => DeclarationKind::WebKitMaskComposite,
            DeclarationData::WebKitMaskSourceType { .. } => DeclarationKind::WebKitMaskSourceType,
            DeclarationData::WebKitMaskBoxImage { .. } => DeclarationKind::WebKitMaskBoxImage,
            DeclarationData::WebKitMaskBoxImageSource { .. } => {
                DeclarationKind::WebKitMaskBoxImageSource
            }
            DeclarationData::WebKitMaskBoxImageSlice { .. } => {
                DeclarationKind::WebKitMaskBoxImageSlice
            }
            DeclarationData::WebKitMaskBoxImageWidth { .. } => {
                DeclarationKind::WebKitMaskBoxImageWidth
            }
            DeclarationData::WebKitMaskBoxImageOutset { .. } => {
                DeclarationKind::WebKitMaskBoxImageOutset
            }
            DeclarationData::WebKitMaskBoxImageRepeat { .. } => {
                DeclarationKind::WebKitMaskBoxImageRepeat
            }
            DeclarationData::Filter { .. } => DeclarationKind::Filter,
            DeclarationData::BackdropFilter { .. } => DeclarationKind::BackdropFilter,
            DeclarationData::MixBlendMode { .. } => DeclarationKind::MixBlendMode,
            DeclarationData::ZIndex { .. } => DeclarationKind::ZIndex,
            DeclarationData::ContainerType { .. } => DeclarationKind::ContainerType,
            DeclarationData::ContainerName { .. } => DeclarationKind::ContainerName,
            DeclarationData::Container { .. } => DeclarationKind::Container,
            DeclarationData::ViewTransitionName { .. } => DeclarationKind::ViewTransitionName,
            DeclarationData::ViewTransitionClass { .. } => DeclarationKind::ViewTransitionClass,
            DeclarationData::ViewTransitionGroup { .. } => DeclarationKind::ViewTransitionGroup,
            DeclarationData::ColorScheme { .. } => DeclarationKind::ColorScheme,
            DeclarationData::PrintColorAdjust { .. } => DeclarationKind::PrintColorAdjust,
            DeclarationData::CSSWide { .. } => DeclarationKind::CSSWide,
            DeclarationData::Unparsed { .. } => DeclarationKind::Unparsed,
            DeclarationData::Custom { .. } => DeclarationKind::Custom,
            DeclarationData::Tombstone => DeclarationKind::Tombstone,
        }
    }
}
impl<'ast> GetSpan<'ast> for Declaration<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Declaration<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_declaration(
        &mut self,
        data: DeclarationData<'ast>,
        span: Span,
    ) -> Declaration<'ast> {
        self.alloc(data, span)
    }
}
#[macro_export]
macro_rules! for_each_property {
    ($macro:ident) => {
        $macro! { "all" : All(CSSWideKeyword) [custom : parse_all], "background-color" :
        BackgroundColor(Box < 'a, CssColor < 'a > >) [boxed : CssColor <'i >],
        "background-image" : BackgroundImage(Vec < 'a, Image < 'a > >) [comma_separated :
        Image <'i >], "background-position-x" : BackgroundPositionX(Vec < 'a,
        PositionComponent < 'a, HorizontalPositionKeyword > >) [comma_separated :
        PositionComponent <'i, HorizontalPositionKeyword >], "background-position-y" :
        BackgroundPositionY(Vec < 'a, PositionComponent < 'a, VerticalPositionKeyword >
        >) [comma_separated : PositionComponent <'i, VerticalPositionKeyword >],
        "background-position" : BackgroundPosition(Vec < 'a, BackgroundPosition < 'a > >)
        [comma_separated : BackgroundPosition <'i >], "background-size" :
        BackgroundSize(Vec < 'a, BackgroundSize < 'a > >) [comma_separated :
        BackgroundSize <'i >], "background-repeat" : BackgroundRepeat(Vec < 'a,
        BackgroundRepeat >) [comma_separated : BackgroundRepeat], "background-attachment"
        : BackgroundAttachment(Vec < 'a, BackgroundAttachment >) [comma_separated :
        BackgroundAttachment], "background-clip" : BackgroundClip(Vec < 'a,
        BackgroundClip >, VendorPrefix) [comma_separated : BackgroundClip],
        "background-origin" : BackgroundOrigin(Vec < 'a, BackgroundOrigin >)
        [comma_separated : BackgroundOrigin], "background" : Background(Vec < 'a,
        Background < 'a > >) [custom : parse_background], "box-shadow" : BoxShadow(Vec <
        'a, BoxShadow < 'a > >, VendorPrefix) [unsupported], "opacity" : Opacity(f32)
        [custom : parse_opacity], "color" : Color(Box < 'a, CssColor < 'a > >) [boxed :
        CssColor <'i >], "display" : Display(Display) [parse : Display], "visibility" :
        Visibility(Visibility) [parse : Visibility], "width" : Width(Box < 'a, Size < 'a
        > >) [boxed : Size <'i >], "height" : Height(Box < 'a, Size < 'a > >) [boxed :
        Size <'i >], "min-width" : MinWidth(Box < 'a, Size < 'a > >) [boxed : Size <'i
        >], "min-height" : MinHeight(Box < 'a, Size < 'a > >) [boxed : Size <'i >],
        "max-width" : MaxWidth(Box < 'a, MaxSize < 'a > >) [boxed : MaxSize <'i >],
        "max-height" : MaxHeight(Box < 'a, MaxSize < 'a > >) [boxed : MaxSize <'i >],
        "block-size" : BlockSize(Box < 'a, Size < 'a > >) [boxed : Size <'i >],
        "inline-size" : InlineSize(Box < 'a, Size < 'a > >) [boxed : Size <'i >],
        "min-block-size" : MinBlockSize(Box < 'a, Size < 'a > >) [boxed : Size <'i >],
        "min-inline-size" : MinInlineSize(Box < 'a, Size < 'a > >) [boxed : Size <'i >],
        "max-block-size" : MaxBlockSize(Box < 'a, MaxSize < 'a > >) [boxed : MaxSize <'i
        >], "max-inline-size" : MaxInlineSize(Box < 'a, MaxSize < 'a > >) [boxed :
        MaxSize <'i >], "box-sizing" : BoxSizing(BoxSizing, VendorPrefix) [parse :
        BoxSizing], "aspect-ratio" : AspectRatio(AspectRatio) [parse : AspectRatio],
        "overflow" : Overflow(Overflow) [parse : Overflow], "overflow-x" :
        OverflowX(OverflowKeyword) [parse : OverflowKeyword], "overflow-y" :
        OverflowY(OverflowKeyword) [parse : OverflowKeyword], "text-overflow" :
        TextOverflow(TextOverflow, VendorPrefix) [parse : TextOverflow], "object-fit" :
        ObjectFit(ObjectFit) [parse : ObjectFit], "object-position" : ObjectPosition(Box
        < 'a, Position < 'a > >) [boxed : Position <'i >], "scrollbar-color" :
        ScrollbarColor(ScrollbarColor < 'a >) [parse : ScrollbarColor <'i >], "position"
        : Position(Box < 'a, PositionProperty >) [boxed : PositionProperty], "top" :
        Top(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto
        <'i >], "bottom" : Bottom(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "left" : Left(Box < 'a, LengthPercentageOrAuto <
        'a > >) [boxed : LengthPercentageOrAuto <'i >], "right" : Right(Box < 'a,
        LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "inset-block-start" : InsetBlockStart(Box < 'a, LengthPercentageOrAuto < 'a > >)
        [boxed : LengthPercentageOrAuto <'i >], "inset-block-end" : InsetBlockEnd(Box <
        'a, LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "inset-inline-start" : InsetInlineStart(Box < 'a, LengthPercentageOrAuto < 'a >
        >) [boxed : LengthPercentageOrAuto <'i >], "inset-inline-end" :
        InsetInlineEnd(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "inset-block" : InsetBlock(Box < 'a, InsetBlock <
        'a > >) [two_value : InsetBlock <'i >], "inset-inline" : InsetInline(Box < 'a,
        InsetInline < 'a > >) [two_value : InsetInline <'i >], "inset" : Inset(Box < 'a,
        Inset < 'a > >) [rect : Inset <'i >], "border-spacing" : BorderSpacing(Box < 'a,
        Size2D < 'a, Length < 'a > > >) [two_value : Size2D <'i, Length <'i >>],
        "border-top-color" : BorderTopColor(Box < 'a, CssColor < 'a > >) [boxed :
        CssColor <'i >], "border-bottom-color" : BorderBottomColor(Box < 'a, CssColor <
        'a > >) [boxed : CssColor <'i >], "border-left-color" : BorderLeftColor(Box < 'a,
        CssColor < 'a > >) [boxed : CssColor <'i >], "border-right-color" :
        BorderRightColor(Box < 'a, CssColor < 'a > >) [boxed : CssColor <'i >],
        "border-block-start-color" : BorderBlockStartColor(Box < 'a, CssColor < 'a > >)
        [boxed : CssColor <'i >], "border-block-end-color" : BorderBlockEndColor(Box <
        'a, CssColor < 'a > >) [boxed : CssColor <'i >], "border-inline-start-color" :
        BorderInlineStartColor(Box < 'a, CssColor < 'a > >) [boxed : CssColor <'i >],
        "border-inline-end-color" : BorderInlineEndColor(Box < 'a, CssColor < 'a > >)
        [boxed : CssColor <'i >], "border-top-style" : BorderTopStyle(LineStyle) [parse :
        LineStyle], "border-bottom-style" : BorderBottomStyle(LineStyle) [parse :
        LineStyle], "border-left-style" : BorderLeftStyle(LineStyle) [parse : LineStyle],
        "border-right-style" : BorderRightStyle(LineStyle) [parse : LineStyle],
        "border-block-start-style" : BorderBlockStartStyle(LineStyle) [parse :
        LineStyle], "border-block-end-style" : BorderBlockEndStyle(LineStyle) [parse :
        LineStyle], "border-inline-start-style" : BorderInlineStartStyle(LineStyle)
        [parse : LineStyle], "border-inline-end-style" : BorderInlineEndStyle(LineStyle)
        [parse : LineStyle], "border-top-width" : BorderTopWidth(Box < 'a,
        BorderSideWidth < 'a > >) [boxed : BorderSideWidth <'i >], "border-bottom-width"
        : BorderBottomWidth(Box < 'a, BorderSideWidth < 'a > >) [boxed : BorderSideWidth
        <'i >], "border-left-width" : BorderLeftWidth(Box < 'a, BorderSideWidth < 'a > >)
        [boxed : BorderSideWidth <'i >], "border-right-width" : BorderRightWidth(Box <
        'a, BorderSideWidth < 'a > >) [boxed : BorderSideWidth <'i >],
        "border-block-start-width" : BorderBlockStartWidth(Box < 'a, BorderSideWidth < 'a
        > >) [boxed : BorderSideWidth <'i >], "border-block-end-width" :
        BorderBlockEndWidth(Box < 'a, BorderSideWidth < 'a > >) [boxed : BorderSideWidth
        <'i >], "border-inline-start-width" : BorderInlineStartWidth(Box < 'a,
        BorderSideWidth < 'a > >) [boxed : BorderSideWidth <'i >],
        "border-inline-end-width" : BorderInlineEndWidth(Box < 'a, BorderSideWidth < 'a >
        >) [boxed : BorderSideWidth <'i >], "border-top-left-radius" :
        BorderTopLeftRadius(Box < 'a, Size2D < 'a, LengthPercentage < 'a > > >,
        VendorPrefix) [two_value : Size2D <'i, LengthPercentage <'i >>],
        "border-top-right-radius" : BorderTopRightRadius(Box < 'a, Size2D < 'a,
        LengthPercentage < 'a > > >, VendorPrefix) [two_value : Size2D <'i,
        LengthPercentage <'i >>], "border-bottom-left-radius" :
        BorderBottomLeftRadius(Box < 'a, Size2D < 'a, LengthPercentage < 'a > > >,
        VendorPrefix) [two_value : Size2D <'i, LengthPercentage <'i >>],
        "border-bottom-right-radius" : BorderBottomRightRadius(Box < 'a, Size2D < 'a,
        LengthPercentage < 'a > > >, VendorPrefix) [two_value : Size2D <'i,
        LengthPercentage <'i >>], "border-start-start-radius" :
        BorderStartStartRadius(Box < 'a, Size2D < 'a, LengthPercentage < 'a > > >)
        [two_value : Size2D <'i, LengthPercentage <'i >>], "border-start-end-radius" :
        BorderStartEndRadius(Box < 'a, Size2D < 'a, LengthPercentage < 'a > > >)
        [two_value : Size2D <'i, LengthPercentage <'i >>], "border-end-start-radius" :
        BorderEndStartRadius(Box < 'a, Size2D < 'a, LengthPercentage < 'a > > >)
        [two_value : Size2D <'i, LengthPercentage <'i >>], "border-end-end-radius" :
        BorderEndEndRadius(Box < 'a, Size2D < 'a, LengthPercentage < 'a > > >) [two_value
        : Size2D <'i, LengthPercentage <'i >>], "border-radius" : BorderRadius(Box < 'a,
        BorderRadius < 'a > >, VendorPrefix) [rect : BorderRadius <'i >],
        "border-image-source" : BorderImageSource(Box < 'a, Image < 'a > >) [boxed :
        Image <'i >], "border-image-outset" : BorderImageOutset(Box < 'a, Rect < 'a,
        LengthOrNumber < 'a > > >) [unsupported], "border-image-repeat" :
        BorderImageRepeat(BorderImageRepeat) [unsupported], "border-image-width" :
        BorderImageWidth(Box < 'a, Rect < 'a, BorderImageSideWidth < 'a > > >)
        [unsupported], "border-image-slice" : BorderImageSlice(Box < 'a, BorderImageSlice
        < 'a > >) [unsupported], "border-image" : BorderImage(Box < 'a, BorderImage < 'a
        > >, VendorPrefix) [unsupported], "border-color" : BorderColor(Box < 'a,
        BorderColor < 'a > >) [rect : BorderColor <'i >], "border-style" :
        BorderStyle(Box < 'a, BorderStyle >) [rect : BorderStyle], "border-width" :
        BorderWidth(Box < 'a, BorderWidth < 'a > >) [rect : BorderWidth <'i >],
        "border-block-color" : BorderBlockColor(Box < 'a, BorderBlockColor < 'a > >)
        [two_value : BorderBlockColor <'i >], "border-block-style" : BorderBlockStyle(Box
        < 'a, BorderBlockStyle >) [two_value : BorderBlockStyle], "border-block-width" :
        BorderBlockWidth(Box < 'a, BorderBlockWidth < 'a > >) [two_value :
        BorderBlockWidth <'i >], "border-inline-color" : BorderInlineColor(Box < 'a,
        BorderInlineColor < 'a > >) [two_value : BorderInlineColor <'i >],
        "border-inline-style" : BorderInlineStyle(Box < 'a, BorderInlineStyle >)
        [two_value : BorderInlineStyle], "border-inline-width" : BorderInlineWidth(Box <
        'a, BorderInlineWidth < 'a > >) [two_value : BorderInlineWidth <'i >], "border" :
        Border(Box < 'a, GenericBorder < 'a, LineStyle > >) [boxed : GenericBorder <'i,
        LineStyle >], "border-top" : BorderTop(Box < 'a, GenericBorder < 'a, LineStyle >
        >) [boxed : GenericBorder <'i, LineStyle >], "border-bottom" : BorderBottom(Box <
        'a, GenericBorder < 'a, LineStyle > >) [boxed : GenericBorder <'i, LineStyle >],
        "border-left" : BorderLeft(Box < 'a, GenericBorder < 'a, LineStyle > >) [boxed :
        GenericBorder <'i, LineStyle >], "border-right" : BorderRight(Box < 'a,
        GenericBorder < 'a, LineStyle > >) [boxed : GenericBorder <'i, LineStyle >],
        "border-block" : BorderBlock(Box < 'a, GenericBorder < 'a, LineStyle > >) [boxed
        : GenericBorder <'i, LineStyle >], "border-block-start" : BorderBlockStart(Box <
        'a, GenericBorder < 'a, LineStyle > >) [boxed : GenericBorder <'i, LineStyle >],
        "border-block-end" : BorderBlockEnd(Box < 'a, GenericBorder < 'a, LineStyle > >)
        [boxed : GenericBorder <'i, LineStyle >], "border-inline" : BorderInline(Box <
        'a, GenericBorder < 'a, LineStyle > >) [boxed : GenericBorder <'i, LineStyle >],
        "border-inline-start" : BorderInlineStart(Box < 'a, GenericBorder < 'a, LineStyle
        > >) [boxed : GenericBorder <'i, LineStyle >], "border-inline-end" :
        BorderInlineEnd(Box < 'a, GenericBorder < 'a, LineStyle > >) [boxed :
        GenericBorder <'i, LineStyle >], "outline" : Outline(Box < 'a, GenericBorder <
        'a, OutlineStyle > >) [boxed : GenericBorder <'i, OutlineStyle >],
        "outline-color" : OutlineColor(Box < 'a, CssColor < 'a > >) [boxed : CssColor <'i
        >], "outline-style" : OutlineStyle(OutlineStyle) [parse : OutlineStyle],
        "outline-width" : OutlineWidth(Box < 'a, BorderSideWidth < 'a > >) [boxed :
        BorderSideWidth <'i >], "flex-direction" : FlexDirection(FlexDirection,
        VendorPrefix) [parse : FlexDirection], "flex-wrap" : FlexWrap(FlexWrap,
        VendorPrefix) [parse : FlexWrap], "flex-flow" : FlexFlow(Box < 'a, FlexFlow >,
        VendorPrefix) [whitespace_separated : FlexFlow], "flex-grow" : FlexGrow(f32,
        VendorPrefix) [custom : parse_flex_grow], "flex-shrink" : FlexShrink(f32,
        VendorPrefix) [custom : parse_flex_shrink], "flex-basis" : FlexBasis(Box < 'a,
        LengthPercentageOrAuto < 'a > >, VendorPrefix) [boxed : LengthPercentageOrAuto
        <'i >], "flex" : Flex(Box < 'a, Flex < 'a > >, VendorPrefix) [boxed : Flex <'i
        >], "order" : Order(f32, VendorPrefix) [custom : parse_order], "align-content" :
        AlignContent(AlignContent, VendorPrefix) [parse : AlignContent],
        "justify-content" : JustifyContent(JustifyContent, VendorPrefix) [parse :
        JustifyContent], "place-content" : PlaceContent(PlaceContent) [parse :
        PlaceContent], "align-self" : AlignSelf(AlignSelf, VendorPrefix) [parse :
        AlignSelf], "justify-self" : JustifySelf(JustifySelf) [parse : JustifySelf],
        "place-self" : PlaceSelf(PlaceSelf) [parse : PlaceSelf], "align-items" :
        AlignItems(AlignItems, VendorPrefix) [parse : AlignItems], "justify-items" :
        JustifyItems(JustifyItems) [parse : JustifyItems], "place-items" :
        PlaceItems(PlaceItems) [parse : PlaceItems], "row-gap" : RowGap(Box < 'a,
        GapValue < 'a > >) [boxed : GapValue <'i >], "column-gap" : ColumnGap(Box < 'a,
        GapValue < 'a > >) [boxed : GapValue <'i >], "gap" : Gap(Box < 'a, Gap < 'a > >)
        [two_value : Gap <'i >], "column-rule" : ColumnRule(Box < 'a, ColumnRule < 'a >
        >, VendorPrefix) [boxed : ColumnRule <'i >], "column-width" :
        ColumnWidth(CSSWideOr < ColumnWidth < 'a > >, VendorPrefix) [css_wide :
        ColumnWidth <'i >], "column-count" : ColumnCount(CSSWideOr < ColumnCount >,
        VendorPrefix) [css_wide : ColumnCount], "columns" : Columns(CSSWideOr < Box < 'a,
        Columns < 'a > > >, VendorPrefix) [css_wide_boxed : Columns <'i >],
        "grid-column-gap" : GridColumnGap(Box < 'a, GapValue < 'a > >) [boxed : GapValue
        <'i >], "grid-row-gap" : GridRowGap(Box < 'a, GapValue < 'a > >) [boxed :
        GapValue <'i >], "box-orient" : BoxOrient(BoxOrient, VendorPrefix) [parse :
        BoxOrient], "box-direction" : BoxDirection(BoxDirection, VendorPrefix) [parse :
        BoxDirection], "box-ordinal-group" : BoxOrdinalGroup(f32, VendorPrefix) [custom :
        parse_box_ordinal_group], "box-align" : BoxAlign(BoxAlign, VendorPrefix) [parse :
        BoxAlign], "box-flex" : BoxFlex(f32, VendorPrefix) [custom : parse_box_flex],
        "box-flex-group" : BoxFlexGroup(f32, VendorPrefix) [custom :
        parse_box_flex_group], "box-pack" : BoxPack(BoxPack, VendorPrefix) [parse :
        BoxPack], "box-lines" : BoxLines(BoxLines, VendorPrefix) [parse : BoxLines],
        "flex-pack" : FlexPack(FlexPack, VendorPrefix) [parse : FlexPack], "flex-order" :
        FlexOrder(f32, VendorPrefix) [custom : parse_flex_order], "flex-align" :
        FlexAlign(BoxAlign, VendorPrefix) [parse : BoxAlign], "flex-item-align" :
        FlexItemAlign(FlexItemAlign, VendorPrefix) [parse : FlexItemAlign],
        "flex-line-pack" : FlexLinePack(FlexLinePack, VendorPrefix) [parse :
        FlexLinePack], "flex-positive" : FlexPositive(f32, VendorPrefix) [custom :
        parse_flex_positive], "flex-negative" : FlexNegative(f32, VendorPrefix) [custom :
        parse_flex_negative], "flex-preferred-size" : FlexPreferredSize(Box < 'a,
        LengthPercentageOrAuto < 'a > >, VendorPrefix) [boxed : LengthPercentageOrAuto
        <'i >], "grid-template-columns" : GridTemplateColumns(Box < 'a, TrackSizing < 'a
        > >) [unsupported], "grid-template-rows" : GridTemplateRows(Box < 'a, TrackSizing
        < 'a > >) [unsupported], "grid-auto-columns" : GridAutoColumns(Vec < 'a,
        TrackSize < 'a > >) [unsupported], "grid-auto-rows" : GridAutoRows(Vec < 'a,
        TrackSize < 'a > >) [unsupported], "grid-auto-flow" : GridAutoFlow(GridAutoFlow)
        [unsupported], "grid-template-areas" : GridTemplateAreas(Box < 'a,
        GridTemplateAreas < 'a > >) [unsupported], "grid-template" : GridTemplate(Box <
        'a, GridTemplate < 'a > >) [unsupported], "grid" : Grid(Box < 'a, Grid < 'a > >)
        [unsupported], "grid-row-start" : GridRowStart(Box < 'a, GridLine < 'a > >)
        [unsupported], "grid-row-end" : GridRowEnd(Box < 'a, GridLine < 'a > >)
        [unsupported], "grid-column-start" : GridColumnStart(Box < 'a, GridLine < 'a > >)
        [unsupported], "grid-column-end" : GridColumnEnd(Box < 'a, GridLine < 'a > >)
        [unsupported], "grid-row" : GridRow(Box < 'a, GridRow < 'a > >) [unsupported],
        "grid-column" : GridColumn(Box < 'a, GridColumn < 'a > >) [unsupported],
        "grid-area" : GridArea(Box < 'a, GridArea < 'a > >) [unsupported], "margin-top" :
        MarginTop(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "margin-bottom" : MarginBottom(Box < 'a,
        LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "margin-left" : MarginLeft(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "margin-right" : MarginRight(Box < 'a,
        LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "margin-block-start" : MarginBlockStart(Box < 'a, LengthPercentageOrAuto < 'a >
        >) [boxed : LengthPercentageOrAuto <'i >], "margin-block-end" :
        MarginBlockEnd(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "margin-inline-start" : MarginInlineStart(Box <
        'a, LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "margin-inline-end" : MarginInlineEnd(Box < 'a, LengthPercentageOrAuto < 'a > >)
        [boxed : LengthPercentageOrAuto <'i >], "margin-block" : MarginBlock(Box < 'a,
        MarginBlock < 'a > >) [two_value : MarginBlock <'i >], "margin-inline" :
        MarginInline(Box < 'a, MarginInline < 'a > >) [two_value : MarginInline <'i >],
        "margin" : Margin(Box < 'a, Margin < 'a > >) [rect : Margin <'i >], "padding-top"
        : PaddingTop(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "padding-bottom" : PaddingBottom(Box < 'a,
        LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "padding-left" : PaddingLeft(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "padding-right" : PaddingRight(Box < 'a,
        LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "padding-block-start" : PaddingBlockStart(Box < 'a, LengthPercentageOrAuto < 'a >
        >) [boxed : LengthPercentageOrAuto <'i >], "padding-block-end" :
        PaddingBlockEnd(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "padding-inline-start" : PaddingInlineStart(Box <
        'a, LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "padding-inline-end" : PaddingInlineEnd(Box < 'a, LengthPercentageOrAuto < 'a >
        >) [boxed : LengthPercentageOrAuto <'i >], "padding-block" : PaddingBlock(Box <
        'a, PaddingBlock < 'a > >) [two_value : PaddingBlock <'i >], "padding-inline" :
        PaddingInline(Box < 'a, PaddingInline < 'a > >) [two_value : PaddingInline <'i
        >], "padding" : Padding(Box < 'a, Padding < 'a > >) [rect : Padding <'i >],
        "scroll-margin-top" : ScrollMarginTop(Box < 'a, LengthPercentageOrAuto < 'a > >)
        [boxed : LengthPercentageOrAuto <'i >], "scroll-margin-bottom" :
        ScrollMarginBottom(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-margin-left" : ScrollMarginLeft(Box < 'a,
        LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "scroll-margin-right" : ScrollMarginRight(Box < 'a, LengthPercentageOrAuto < 'a >
        >) [boxed : LengthPercentageOrAuto <'i >], "scroll-margin-block-start" :
        ScrollMarginBlockStart(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-margin-block-end" :
        ScrollMarginBlockEnd(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-margin-inline-start" :
        ScrollMarginInlineStart(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-margin-inline-end" :
        ScrollMarginInlineEnd(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-margin-block" : ScrollMarginBlock(Box <
        'a, ScrollMarginBlock < 'a > >) [two_value : ScrollMarginBlock <'i >],
        "scroll-margin-inline" : ScrollMarginInline(Box < 'a, ScrollMarginInline < 'a >
        >) [two_value : ScrollMarginInline <'i >], "scroll-margin" : ScrollMargin(Box <
        'a, ScrollMargin < 'a > >) [rect : ScrollMargin <'i >], "scroll-padding-top" :
        ScrollPaddingTop(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-padding-bottom" : ScrollPaddingBottom(Box
        < 'a, LengthPercentageOrAuto < 'a > >) [boxed : LengthPercentageOrAuto <'i >],
        "scroll-padding-left" : ScrollPaddingLeft(Box < 'a, LengthPercentageOrAuto < 'a >
        >) [boxed : LengthPercentageOrAuto <'i >], "scroll-padding-right" :
        ScrollPaddingRight(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-padding-block-start" :
        ScrollPaddingBlockStart(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-padding-block-end" :
        ScrollPaddingBlockEnd(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-padding-inline-start" :
        ScrollPaddingInlineStart(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-padding-inline-end" :
        ScrollPaddingInlineEnd(Box < 'a, LengthPercentageOrAuto < 'a > >) [boxed :
        LengthPercentageOrAuto <'i >], "scroll-padding-block" : ScrollPaddingBlock(Box <
        'a, ScrollPaddingBlock < 'a > >) [two_value : ScrollPaddingBlock <'i >],
        "scroll-padding-inline" : ScrollPaddingInline(Box < 'a, ScrollPaddingInline < 'a
        > >) [two_value : ScrollPaddingInline <'i >], "scroll-padding" :
        ScrollPadding(Box < 'a, ScrollPadding < 'a > >) [rect : ScrollPadding <'i >],
        "font-weight" : FontWeight(FontWeight) [parse : FontWeight], "font-size" :
        FontSize(Box < 'a, FontSize < 'a > >) [boxed : FontSize <'i >], "font-stretch" :
        FontStretch(FontStretch) [parse : FontStretch], "font-family" : FontFamily(Vec <
        'a, FontFamily < 'a > >) [custom : parse_font_family], "font-style" :
        FontStyle(FontStyle) [parse : FontStyle], "font-variant-caps" :
        FontVariantCaps(FontVariantCaps) [parse : FontVariantCaps], "line-height" :
        LineHeight(Box < 'a, LineHeight < 'a > >) [boxed : LineHeight <'i >], "font" :
        Font(Box < 'a, Font < 'a > >) [unsupported], "vertical-align" : VerticalAlign(Box
        < 'a, VerticalAlign < 'a > >) [unsupported], "font-palette" : FontPalette(Box <
        'a, DashedIdentReference < 'a > >) [unsupported], "transition-property" :
        TransitionProperty(Vec < 'a, PropertyId < 'a > >, VendorPrefix) [custom :
        parse_transition_property], "transition-duration" : TransitionDuration(Vec < 'a,
        Time >, VendorPrefix) [custom : parse_transition_duration], "transition-delay" :
        TransitionDelay(Vec < 'a, Time >, VendorPrefix) [custom :
        parse_transition_delay], "transition-timing-function" :
        TransitionTimingFunction(Vec < 'a, EasingFunction >, VendorPrefix) [custom :
        parse_transition_timing], "transition" : Transition(Vec < 'a, Transition < 'a >
        >, VendorPrefix) [custom : parse_transition], "animation-name" :
        AnimationName(Vec < 'a, AnimationName < 'a > >, VendorPrefix) [custom :
        parse_animation_name], "animation-duration" : AnimationDuration(Vec < 'a, Time >,
        VendorPrefix) [custom : parse_animation_duration], "animation-timing-function" :
        AnimationTimingFunction(Vec < 'a, EasingFunction >, VendorPrefix) [custom :
        parse_animation_timing], "animation-iteration-count" :
        AnimationIterationCount(Vec < 'a, AnimationIterationCount >, VendorPrefix)
        [custom : parse_animation_iteration], "animation-direction" :
        AnimationDirection(Vec < 'a, AnimationDirection >, VendorPrefix) [custom :
        parse_animation_direction], "animation-play-state" : AnimationPlayState(Vec < 'a,
        AnimationPlayState >, VendorPrefix) [custom : parse_animation_play_state],
        "animation-delay" : AnimationDelay(Vec < 'a, Time >, VendorPrefix) [custom :
        parse_animation_delay], "animation-fill-mode" : AnimationFillMode(Vec < 'a,
        AnimationFillMode >, VendorPrefix) [custom : parse_animation_fill],
        "animation-composition" : AnimationComposition(Vec < 'a, AnimationComposition >)
        [unsupported], "animation-timeline" : AnimationTimeline(Vec < 'a,
        AnimationTimeline < 'a > >) [unsupported], "animation-range-start" :
        AnimationRangeStart(Vec < 'a, AnimationRangeStart < 'a > >) [unsupported],
        "animation-range-end" : AnimationRangeEnd(Vec < 'a, AnimationRangeEnd < 'a > >)
        [unsupported], "animation-range" : AnimationRange(Vec < 'a, AnimationRange < 'a >
        >) [unsupported], "animation" : Animation(Vec < 'a, Animation < 'a > >,
        VendorPrefix) [custom : parse_animation], "transform" : Transform(Vec < 'a,
        Transform < 'a > >, VendorPrefix) [custom : parse_transform], "transform-origin"
        : TransformOrigin(Box < 'a, Position < 'a > >, VendorPrefix) [boxed : Position
        <'i >], "transform-style" : TransformStyle(TransformStyle, VendorPrefix) [parse :
        TransformStyle], "transform-box" : TransformBox(TransformBox) [parse :
        TransformBox], "backface-visibility" : BackfaceVisibility(BackfaceVisibility,
        VendorPrefix) [parse : BackfaceVisibility], "perspective" : Perspective(Box < 'a,
        Perspective < 'a > >, VendorPrefix) [boxed : Perspective <'i >],
        "perspective-origin" : PerspectiveOrigin(Box < 'a, Position < 'a > >,
        VendorPrefix) [boxed : Position <'i >], "translate" : Translate(Box < 'a,
        Translate < 'a > >) [boxed : Translate <'i >], "rotate" : Rotate(Rotate) [parse :
        Rotate], "scale" : Scale(Box < 'a, Scale >) [boxed : Scale], "text-transform" :
        TextTransform(TextTransform) [parse : TextTransform], "content" : Content(Box <
        'a, Content < 'a > >) [unsupported], "white-space" : WhiteSpace(WhiteSpace)
        [parse : WhiteSpace], "tab-size" : TabSize(Box < 'a, LengthOrNumber < 'a > >,
        VendorPrefix) [unsupported], "word-break" : WordBreak(WordBreak) [parse :
        WordBreak], "line-break" : LineBreak(LineBreak) [parse : LineBreak], "hyphens" :
        Hyphens(Hyphens, VendorPrefix) [parse : Hyphens], "overflow-wrap" :
        OverflowWrap(OverflowWrap) [parse : OverflowWrap], "word-wrap" :
        WordWrap(OverflowWrap) [parse : OverflowWrap], "text-align" :
        TextAlign(TextAlign) [parse : TextAlign], "text-align-last" :
        TextAlignLast(TextAlignLast, VendorPrefix) [parse : TextAlignLast],
        "text-justify" : TextJustify(TextJustify) [parse : TextJustify], "word-spacing" :
        WordSpacing(Box < 'a, Spacing < 'a > >) [boxed : Spacing <'i >], "letter-spacing"
        : LetterSpacing(Box < 'a, Spacing < 'a > >) [boxed : Spacing <'i >],
        "text-indent" : TextIndent(Box < 'a, TextIndent < 'a > >) [boxed : TextIndent <'i
        >], "text-decoration-line" : TextDecorationLine(Box < 'a, TextDecorationLine < 'a
        > >, VendorPrefix) [boxed : TextDecorationLine <'i >], "text-decoration-style" :
        TextDecorationStyle(TextDecorationStyle, VendorPrefix) [parse :
        TextDecorationStyle], "text-decoration-color" : TextDecorationColor(Box < 'a,
        CssColor < 'a > >, VendorPrefix) [boxed : CssColor <'i >],
        "text-decoration-thickness" : TextDecorationThickness(Box < 'a,
        TextDecorationThickness < 'a > >) [boxed : TextDecorationThickness <'i >],
        "text-decoration" : TextDecoration(Box < 'a, TextDecoration < 'a > >,
        VendorPrefix) [unsupported], "text-decoration-skip-ink" :
        TextDecorationSkipInk(TextDecorationSkipInk, VendorPrefix) [parse :
        TextDecorationSkipInk], "text-emphasis-style" : TextEmphasisStyle(Box < 'a,
        TextEmphasisStyle < 'a > >, VendorPrefix) [unsupported], "text-emphasis-color" :
        TextEmphasisColor(Box < 'a, CssColor < 'a > >, VendorPrefix) [boxed : CssColor
        <'i >], "text-emphasis" : TextEmphasis(Box < 'a, TextEmphasis < 'a > >,
        VendorPrefix) [unsupported], "text-emphasis-position" :
        TextEmphasisPosition(TextEmphasisPosition, VendorPrefix) [unsupported],
        "text-shadow" : TextShadow(Vec < 'a, TextShadow < 'a > >) [unsupported],
        "text-size-adjust" : TextSizeAdjust(TextSizeAdjust, VendorPrefix) [parse :
        TextSizeAdjust], "direction" : Direction(TextDirection) [parse : TextDirection],
        "unicode-bidi" : UnicodeBidi(UnicodeBidi) [parse : UnicodeBidi],
        "box-decoration-break" : BoxDecorationBreak(BoxDecorationBreak, VendorPrefix)
        [parse : BoxDecorationBreak], "resize" : Resize(Resize) [parse : Resize],
        "pointer-events" : PointerEvents(PointerEvents) [parse : PointerEvents], "float"
        : Float(Float) [parse : Float], "clear" : Clear(Clear) [parse : Clear],
        "touch-action" : TouchAction(TouchAction) [parse : TouchAction],
        "scroll-behavior" : ScrollBehavior(ScrollBehavior) [parse : ScrollBehavior],
        "cursor" : Cursor(Box < 'a, Cursor < 'a > >) [unsupported], "caret-color" :
        CaretColor(Box < 'a, ColorOrAuto < 'a > >) [unsupported], "caret-shape" :
        CaretShape(CaretShape) [unsupported], "caret" : Caret(Box < 'a, Caret < 'a > >)
        [unsupported], "user-select" : UserSelect(UserSelect, VendorPrefix) [parse :
        UserSelect], "accent-color" : AccentColor(Box < 'a, ColorOrAuto < 'a > >) [boxed
        : ColorOrAuto <'i >], "appearance" : Appearance(Box < 'a, Appearance < 'a > >,
        VendorPrefix) [unsupported], "list-style-type" : ListStyleType(Box < 'a,
        ListStyleType < 'a > >) [unsupported], "list-style-image" : ListStyleImage(Box <
        'a, Image < 'a > >) [boxed : Image <'i >], "list-style-position" :
        ListStylePosition(ListStylePosition) [unsupported], "list-style" : ListStyle(Box
        < 'a, ListStyle < 'a > >) [unsupported], "marker-side" : MarkerSide(MarkerSide)
        [unsupported], "composes" : Composes(Box < 'a, Composes < 'a > >) [unsupported],
        "fill" : Fill(Box < 'a, SVGPaint < 'a > >) [boxed : SVGPaint <'i >], "fill-rule"
        : FillRule(FillRule) [parse : FillRule], "fill-opacity" : FillOpacity(f32)
        [custom : parse_fill_opacity], "stroke" : Stroke(Box < 'a, SVGPaint < 'a > >)
        [boxed : SVGPaint <'i >], "stroke-opacity" : StrokeOpacity(f32) [custom :
        parse_stroke_opacity], "stroke-width" : StrokeWidth(Box < 'a, LengthPercentage <
        'a > >) [boxed : LengthPercentage <'i >], "stroke-linecap" :
        StrokeLinecap(StrokeLinecap) [parse : StrokeLinecap], "stroke-linejoin" :
        StrokeLinejoin(StrokeLinejoin) [parse : StrokeLinejoin], "stroke-miterlimit" :
        StrokeMiterlimit(f32) [custom : parse_stroke_miterlimit], "stroke-dasharray" :
        StrokeDasharray(Box < 'a, StrokeDasharray < 'a > >) [boxed : StrokeDasharray <'i
        >], "stroke-dashoffset" : StrokeDashoffset(Box < 'a, LengthPercentage < 'a > >)
        [boxed : LengthPercentage <'i >], "marker-start" : MarkerStart(Box < 'a, Marker <
        'a > >) [boxed : Marker <'i >], "marker-mid" : MarkerMid(Box < 'a, Marker < 'a >
        >) [boxed : Marker <'i >], "marker-end" : MarkerEnd(Box < 'a, Marker < 'a > >)
        [boxed : Marker <'i >], "marker" : Marker(Box < 'a, Marker < 'a > >) [boxed :
        Marker <'i >], "color-interpolation" : ColorInterpolation(ColorInterpolation)
        [parse : ColorInterpolation], "color-interpolation-filters" :
        ColorInterpolationFilters(ColorInterpolation) [parse : ColorInterpolation],
        "color-rendering" : ColorRendering(ColorRendering) [parse : ColorRendering],
        "shape-rendering" : ShapeRendering(ShapeRendering) [parse : ShapeRendering],
        "text-rendering" : TextRendering(TextRendering) [parse : TextRendering],
        "image-rendering" : ImageRendering(ImageRendering) [parse : ImageRendering],
        "clip-path" : ClipPath(Box < 'a, ClipPath < 'a > >, VendorPrefix) [unsupported],
        "clip-rule" : ClipRule(FillRule) [unsupported], "mask-image" : MaskImage(Vec <
        'a, Image < 'a > >, VendorPrefix) [comma_separated : Image <'i >], "mask-mode" :
        MaskMode(Vec < 'a, MaskMode >) [comma_separated : MaskMode], "mask-repeat" :
        MaskRepeat(Vec < 'a, BackgroundRepeat >, VendorPrefix) [comma_separated :
        BackgroundRepeat], "mask-position-x" : MaskPositionX(Vec < 'a, PositionComponent
        < 'a, HorizontalPositionKeyword > >) [comma_separated : PositionComponent <'i,
        HorizontalPositionKeyword >], "mask-position-y" : MaskPositionY(Vec < 'a,
        PositionComponent < 'a, VerticalPositionKeyword > >) [comma_separated :
        PositionComponent <'i, VerticalPositionKeyword >], "mask-position" :
        MaskPosition(Vec < 'a, Position < 'a > >, VendorPrefix) [comma_separated :
        Position <'i >], "mask-clip" : MaskClip(Vec < 'a, MaskClip >, VendorPrefix)
        [comma_separated : MaskClip], "mask-origin" : MaskOrigin(Vec < 'a, GeometryBox >,
        VendorPrefix) [comma_separated : GeometryBox], "mask-size" : MaskSize(Vec < 'a,
        BackgroundSize < 'a > >, VendorPrefix) [comma_separated : BackgroundSize <'i >],
        "mask-composite" : MaskComposite(Vec < 'a, MaskComposite >) [comma_separated :
        MaskComposite], "mask-type" : MaskType(MaskType) [parse : MaskType], "mask" :
        Mask(Vec < 'a, Mask < 'a > >, VendorPrefix) [comma_separated : Mask <'i >],
        "mask-border-source" : MaskBorderSource(Box < 'a, Image < 'a > >) [boxed : Image
        <'i >], "mask-border-mode" : MaskBorderMode(MaskBorderMode) [unsupported],
        "mask-border-slice" : MaskBorderSlice(Box < 'a, BorderImageSlice < 'a > >)
        [unsupported], "mask-border-width" : MaskBorderWidth(Box < 'a, Rect < 'a,
        BorderImageSideWidth < 'a > > >) [unsupported], "mask-border-outset" :
        MaskBorderOutset(Box < 'a, Rect < 'a, LengthOrNumber < 'a > > >) [unsupported],
        "mask-border-repeat" : MaskBorderRepeat(BorderImageRepeat) [unsupported],
        "mask-border" : MaskBorder(Box < 'a, MaskBorder < 'a > >) [unsupported],
        "-webkit-mask-composite" : WebKitMaskComposite(Vec < 'a, WebKitMaskComposite >)
        [comma_separated : WebKitMaskComposite], "mask-source-type" :
        WebKitMaskSourceType(Vec < 'a, WebKitMaskSourceType >, VendorPrefix)
        [comma_separated : WebKitMaskSourceType], "mask-box-image" :
        WebKitMaskBoxImage(Box < 'a, BorderImage < 'a > >, VendorPrefix) [unsupported],
        "mask-box-image-source" : WebKitMaskBoxImageSource(Box < 'a, Image < 'a > >,
        VendorPrefix) [boxed : Image <'i >], "mask-box-image-slice" :
        WebKitMaskBoxImageSlice(Box < 'a, BorderImageSlice < 'a > >, VendorPrefix)
        [unsupported], "mask-box-image-width" : WebKitMaskBoxImageWidth(Box < 'a, Rect <
        'a, BorderImageSideWidth < 'a > > >, VendorPrefix) [unsupported],
        "mask-box-image-outset" : WebKitMaskBoxImageOutset(Box < 'a, Rect < 'a,
        LengthOrNumber < 'a > > >, VendorPrefix) [unsupported], "mask-box-image-repeat" :
        WebKitMaskBoxImageRepeat(BorderImageRepeat, VendorPrefix) [unsupported], "filter"
        : Filter(Box < 'a, FilterList < 'a > >, VendorPrefix) [unsupported],
        "backdrop-filter" : BackdropFilter(Box < 'a, FilterList < 'a > >, VendorPrefix)
        [unsupported], "mix-blend-mode" : MixBlendMode(BlendMode) [unsupported],
        "z-index" : ZIndex(ZIndex) [parse : ZIndex], "container-type" :
        ContainerType(ContainerType) [unsupported], "container-name" : ContainerName(Box
        < 'a, ContainerNameList < 'a > >) [unsupported], "container" : Container(Box <
        'a, Container < 'a > >) [unsupported], "view-transition-name" :
        ViewTransitionName(Box < 'a, ViewTransitionName < 'a > >) [unsupported],
        "view-transition-class" : ViewTransitionClass(Box < 'a, NoneOrCustomIdentList <
        'a > >) [unsupported], "view-transition-group" : ViewTransitionGroup(Box < 'a,
        ViewTransitionGroup < 'a > >) [unsupported], "color-scheme" :
        ColorScheme(ColorScheme) [unsupported], "print-color-adjust" :
        PrintColorAdjust(PrintColorAdjust, VendorPrefix) [unsupported], }
    };
}
#[repr(u32)]
enum KnownPropertyDiscriminant {
    All,
    BackgroundColor,
    BackgroundImage,
    BackgroundPositionX,
    BackgroundPositionY,
    BackgroundPosition,
    BackgroundSize,
    BackgroundRepeat,
    BackgroundAttachment,
    BackgroundClip,
    BackgroundOrigin,
    Background,
    BoxShadow,
    Opacity,
    Color,
    Display,
    Visibility,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    BlockSize,
    InlineSize,
    MinBlockSize,
    MinInlineSize,
    MaxBlockSize,
    MaxInlineSize,
    BoxSizing,
    AspectRatio,
    Overflow,
    OverflowX,
    OverflowY,
    TextOverflow,
    ObjectFit,
    ObjectPosition,
    ScrollbarColor,
    Position,
    Top,
    Bottom,
    Left,
    Right,
    InsetBlockStart,
    InsetBlockEnd,
    InsetInlineStart,
    InsetInlineEnd,
    InsetBlock,
    InsetInline,
    Inset,
    BorderSpacing,
    BorderTopColor,
    BorderBottomColor,
    BorderLeftColor,
    BorderRightColor,
    BorderBlockStartColor,
    BorderBlockEndColor,
    BorderInlineStartColor,
    BorderInlineEndColor,
    BorderTopStyle,
    BorderBottomStyle,
    BorderLeftStyle,
    BorderRightStyle,
    BorderBlockStartStyle,
    BorderBlockEndStyle,
    BorderInlineStartStyle,
    BorderInlineEndStyle,
    BorderTopWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    BorderRightWidth,
    BorderBlockStartWidth,
    BorderBlockEndWidth,
    BorderInlineStartWidth,
    BorderInlineEndWidth,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    BorderStartStartRadius,
    BorderStartEndRadius,
    BorderEndStartRadius,
    BorderEndEndRadius,
    BorderRadius,
    BorderImageSource,
    BorderImageOutset,
    BorderImageRepeat,
    BorderImageWidth,
    BorderImageSlice,
    BorderImage,
    BorderColor,
    BorderStyle,
    BorderWidth,
    BorderBlockColor,
    BorderBlockStyle,
    BorderBlockWidth,
    BorderInlineColor,
    BorderInlineStyle,
    BorderInlineWidth,
    Border,
    BorderTop,
    BorderBottom,
    BorderLeft,
    BorderRight,
    BorderBlock,
    BorderBlockStart,
    BorderBlockEnd,
    BorderInline,
    BorderInlineStart,
    BorderInlineEnd,
    Outline,
    OutlineColor,
    OutlineStyle,
    OutlineWidth,
    FlexDirection,
    FlexWrap,
    FlexFlow,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    Flex,
    Order,
    AlignContent,
    JustifyContent,
    PlaceContent,
    AlignSelf,
    JustifySelf,
    PlaceSelf,
    AlignItems,
    JustifyItems,
    PlaceItems,
    RowGap,
    ColumnGap,
    Gap,
    ColumnRule,
    ColumnWidth,
    ColumnCount,
    Columns,
    GridColumnGap,
    GridRowGap,
    BoxOrient,
    BoxDirection,
    BoxOrdinalGroup,
    BoxAlign,
    BoxFlex,
    BoxFlexGroup,
    BoxPack,
    BoxLines,
    FlexPack,
    FlexOrder,
    FlexAlign,
    FlexItemAlign,
    FlexLinePack,
    FlexPositive,
    FlexNegative,
    FlexPreferredSize,
    GridTemplateColumns,
    GridTemplateRows,
    GridAutoColumns,
    GridAutoRows,
    GridAutoFlow,
    GridTemplateAreas,
    GridTemplate,
    Grid,
    GridRowStart,
    GridRowEnd,
    GridColumnStart,
    GridColumnEnd,
    GridRow,
    GridColumn,
    GridArea,
    MarginTop,
    MarginBottom,
    MarginLeft,
    MarginRight,
    MarginBlockStart,
    MarginBlockEnd,
    MarginInlineStart,
    MarginInlineEnd,
    MarginBlock,
    MarginInline,
    Margin,
    PaddingTop,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    PaddingBlockStart,
    PaddingBlockEnd,
    PaddingInlineStart,
    PaddingInlineEnd,
    PaddingBlock,
    PaddingInline,
    Padding,
    ScrollMarginTop,
    ScrollMarginBottom,
    ScrollMarginLeft,
    ScrollMarginRight,
    ScrollMarginBlockStart,
    ScrollMarginBlockEnd,
    ScrollMarginInlineStart,
    ScrollMarginInlineEnd,
    ScrollMarginBlock,
    ScrollMarginInline,
    ScrollMargin,
    ScrollPaddingTop,
    ScrollPaddingBottom,
    ScrollPaddingLeft,
    ScrollPaddingRight,
    ScrollPaddingBlockStart,
    ScrollPaddingBlockEnd,
    ScrollPaddingInlineStart,
    ScrollPaddingInlineEnd,
    ScrollPaddingBlock,
    ScrollPaddingInline,
    ScrollPadding,
    FontWeight,
    FontSize,
    FontStretch,
    FontFamily,
    FontStyle,
    FontVariantCaps,
    LineHeight,
    Font,
    VerticalAlign,
    FontPalette,
    TransitionProperty,
    TransitionDuration,
    TransitionDelay,
    TransitionTimingFunction,
    Transition,
    AnimationName,
    AnimationDuration,
    AnimationTimingFunction,
    AnimationIterationCount,
    AnimationDirection,
    AnimationPlayState,
    AnimationDelay,
    AnimationFillMode,
    AnimationComposition,
    AnimationTimeline,
    AnimationRangeStart,
    AnimationRangeEnd,
    AnimationRange,
    Animation,
    Transform,
    TransformOrigin,
    TransformStyle,
    TransformBox,
    BackfaceVisibility,
    Perspective,
    PerspectiveOrigin,
    Translate,
    Rotate,
    Scale,
    TextTransform,
    Content,
    WhiteSpace,
    TabSize,
    WordBreak,
    LineBreak,
    Hyphens,
    OverflowWrap,
    WordWrap,
    TextAlign,
    TextAlignLast,
    TextJustify,
    WordSpacing,
    LetterSpacing,
    TextIndent,
    TextDecorationLine,
    TextDecorationStyle,
    TextDecorationColor,
    TextDecorationThickness,
    TextDecoration,
    TextDecorationSkipInk,
    TextEmphasisStyle,
    TextEmphasisColor,
    TextEmphasis,
    TextEmphasisPosition,
    TextShadow,
    TextSizeAdjust,
    Direction,
    UnicodeBidi,
    BoxDecorationBreak,
    Resize,
    PointerEvents,
    Float,
    Clear,
    TouchAction,
    ScrollBehavior,
    Cursor,
    CaretColor,
    CaretShape,
    Caret,
    UserSelect,
    AccentColor,
    Appearance,
    ListStyleType,
    ListStyleImage,
    ListStylePosition,
    ListStyle,
    MarkerSide,
    Composes,
    Fill,
    FillRule,
    FillOpacity,
    Stroke,
    StrokeOpacity,
    StrokeWidth,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeDasharray,
    StrokeDashoffset,
    MarkerStart,
    MarkerMid,
    MarkerEnd,
    Marker,
    ColorInterpolation,
    ColorInterpolationFilters,
    ColorRendering,
    ShapeRendering,
    TextRendering,
    ImageRendering,
    ClipPath,
    ClipRule,
    MaskImage,
    MaskMode,
    MaskRepeat,
    MaskPositionX,
    MaskPositionY,
    MaskPosition,
    MaskClip,
    MaskOrigin,
    MaskSize,
    MaskComposite,
    MaskType,
    Mask,
    MaskBorderSource,
    MaskBorderMode,
    MaskBorderSlice,
    MaskBorderWidth,
    MaskBorderOutset,
    MaskBorderRepeat,
    MaskBorder,
    WebKitMaskComposite,
    WebKitMaskSourceType,
    WebKitMaskBoxImage,
    WebKitMaskBoxImageSource,
    WebKitMaskBoxImageSlice,
    WebKitMaskBoxImageWidth,
    WebKitMaskBoxImageOutset,
    WebKitMaskBoxImageRepeat,
    Filter,
    BackdropFilter,
    MixBlendMode,
    ZIndex,
    ContainerType,
    ContainerName,
    Container,
    ViewTransitionName,
    ViewTransitionClass,
    ViewTransitionGroup,
    ColorScheme,
    PrintColorAdjust,
}
impl<'ast> PropertyId<'ast> {
    /// Resolves a property name while retaining unknown names for lossless parsing.
    pub fn from_name(name: &'ast str) -> Self {
        let property_id = crate::match_ignore_ascii_case!(
            name, "all" => Some(Self::All), "background-color" =>
            Some(Self::BackgroundColor), "background-image" =>
            Some(Self::BackgroundImage), "background-position-x" =>
            Some(Self::BackgroundPositionX), "background-position-y" =>
            Some(Self::BackgroundPositionY), "background-position" =>
            Some(Self::BackgroundPosition), "background-size" =>
            Some(Self::BackgroundSize), "background-repeat" =>
            Some(Self::BackgroundRepeat), "background-attachment" =>
            Some(Self::BackgroundAttachment), "background-clip" =>
            Some(Self::BackgroundClip(VendorPrefix::NONE)), "background-origin" =>
            Some(Self::BackgroundOrigin), "background" => Some(Self::Background),
            "box-shadow" => Some(Self::BoxShadow(VendorPrefix::NONE)), "opacity" =>
            Some(Self::Opacity), "color" => Some(Self::Color), "display" =>
            Some(Self::Display), "visibility" => Some(Self::Visibility), "width" =>
            Some(Self::Width), "height" => Some(Self::Height), "min-width" =>
            Some(Self::MinWidth), "min-height" => Some(Self::MinHeight), "max-width" =>
            Some(Self::MaxWidth), "max-height" => Some(Self::MaxHeight), "block-size" =>
            Some(Self::BlockSize), "inline-size" => Some(Self::InlineSize),
            "min-block-size" => Some(Self::MinBlockSize), "min-inline-size" =>
            Some(Self::MinInlineSize), "max-block-size" => Some(Self::MaxBlockSize),
            "max-inline-size" => Some(Self::MaxInlineSize), "box-sizing" =>
            Some(Self::BoxSizing(VendorPrefix::NONE)), "aspect-ratio" =>
            Some(Self::AspectRatio), "overflow" => Some(Self::Overflow), "overflow-x" =>
            Some(Self::OverflowX), "overflow-y" => Some(Self::OverflowY), "text-overflow"
            => Some(Self::TextOverflow(VendorPrefix::NONE)), "object-fit" =>
            Some(Self::ObjectFit), "object-position" => Some(Self::ObjectPosition),
            "scrollbar-color" => Some(Self::ScrollbarColor), "position" =>
            Some(Self::Position), "top" => Some(Self::Top), "bottom" =>
            Some(Self::Bottom), "left" => Some(Self::Left), "right" => Some(Self::Right),
            "inset-block-start" => Some(Self::InsetBlockStart), "inset-block-end" =>
            Some(Self::InsetBlockEnd), "inset-inline-start" =>
            Some(Self::InsetInlineStart), "inset-inline-end" =>
            Some(Self::InsetInlineEnd), "inset-block" => Some(Self::InsetBlock),
            "inset-inline" => Some(Self::InsetInline), "inset" => Some(Self::Inset),
            "border-spacing" => Some(Self::BorderSpacing), "border-top-color" =>
            Some(Self::BorderTopColor), "border-bottom-color" =>
            Some(Self::BorderBottomColor), "border-left-color" =>
            Some(Self::BorderLeftColor), "border-right-color" =>
            Some(Self::BorderRightColor), "border-block-start-color" =>
            Some(Self::BorderBlockStartColor), "border-block-end-color" =>
            Some(Self::BorderBlockEndColor), "border-inline-start-color" =>
            Some(Self::BorderInlineStartColor), "border-inline-end-color" =>
            Some(Self::BorderInlineEndColor), "border-top-style" =>
            Some(Self::BorderTopStyle), "border-bottom-style" =>
            Some(Self::BorderBottomStyle), "border-left-style" =>
            Some(Self::BorderLeftStyle), "border-right-style" =>
            Some(Self::BorderRightStyle), "border-block-start-style" =>
            Some(Self::BorderBlockStartStyle), "border-block-end-style" =>
            Some(Self::BorderBlockEndStyle), "border-inline-start-style" =>
            Some(Self::BorderInlineStartStyle), "border-inline-end-style" =>
            Some(Self::BorderInlineEndStyle), "border-top-width" =>
            Some(Self::BorderTopWidth), "border-bottom-width" =>
            Some(Self::BorderBottomWidth), "border-left-width" =>
            Some(Self::BorderLeftWidth), "border-right-width" =>
            Some(Self::BorderRightWidth), "border-block-start-width" =>
            Some(Self::BorderBlockStartWidth), "border-block-end-width" =>
            Some(Self::BorderBlockEndWidth), "border-inline-start-width" =>
            Some(Self::BorderInlineStartWidth), "border-inline-end-width" =>
            Some(Self::BorderInlineEndWidth), "border-top-left-radius" =>
            Some(Self::BorderTopLeftRadius(VendorPrefix::NONE)),
            "border-top-right-radius" =>
            Some(Self::BorderTopRightRadius(VendorPrefix::NONE)),
            "border-bottom-left-radius" =>
            Some(Self::BorderBottomLeftRadius(VendorPrefix::NONE)),
            "border-bottom-right-radius" =>
            Some(Self::BorderBottomRightRadius(VendorPrefix::NONE)),
            "border-start-start-radius" => Some(Self::BorderStartStartRadius),
            "border-start-end-radius" => Some(Self::BorderStartEndRadius),
            "border-end-start-radius" => Some(Self::BorderEndStartRadius),
            "border-end-end-radius" => Some(Self::BorderEndEndRadius), "border-radius" =>
            Some(Self::BorderRadius(VendorPrefix::NONE)), "border-image-source" =>
            Some(Self::BorderImageSource), "border-image-outset" =>
            Some(Self::BorderImageOutset), "border-image-repeat" =>
            Some(Self::BorderImageRepeat), "border-image-width" =>
            Some(Self::BorderImageWidth), "border-image-slice" =>
            Some(Self::BorderImageSlice), "border-image" =>
            Some(Self::BorderImage(VendorPrefix::NONE)), "border-color" =>
            Some(Self::BorderColor), "border-style" => Some(Self::BorderStyle),
            "border-width" => Some(Self::BorderWidth), "border-block-color" =>
            Some(Self::BorderBlockColor), "border-block-style" =>
            Some(Self::BorderBlockStyle), "border-block-width" =>
            Some(Self::BorderBlockWidth), "border-inline-color" =>
            Some(Self::BorderInlineColor), "border-inline-style" =>
            Some(Self::BorderInlineStyle), "border-inline-width" =>
            Some(Self::BorderInlineWidth), "border" => Some(Self::Border), "border-top"
            => Some(Self::BorderTop), "border-bottom" => Some(Self::BorderBottom),
            "border-left" => Some(Self::BorderLeft), "border-right" =>
            Some(Self::BorderRight), "border-block" => Some(Self::BorderBlock),
            "border-block-start" => Some(Self::BorderBlockStart), "border-block-end" =>
            Some(Self::BorderBlockEnd), "border-inline" => Some(Self::BorderInline),
            "border-inline-start" => Some(Self::BorderInlineStart), "border-inline-end"
            => Some(Self::BorderInlineEnd), "outline" => Some(Self::Outline),
            "outline-color" => Some(Self::OutlineColor), "outline-style" =>
            Some(Self::OutlineStyle), "outline-width" => Some(Self::OutlineWidth),
            "flex-direction" => Some(Self::FlexDirection(VendorPrefix::NONE)),
            "flex-wrap" => Some(Self::FlexWrap(VendorPrefix::NONE)), "flex-flow" =>
            Some(Self::FlexFlow(VendorPrefix::NONE)), "flex-grow" =>
            Some(Self::FlexGrow(VendorPrefix::NONE)), "flex-shrink" =>
            Some(Self::FlexShrink(VendorPrefix::NONE)), "flex-basis" =>
            Some(Self::FlexBasis(VendorPrefix::NONE)), "flex" =>
            Some(Self::Flex(VendorPrefix::NONE)), "order" =>
            Some(Self::Order(VendorPrefix::NONE)), "align-content" =>
            Some(Self::AlignContent(VendorPrefix::NONE)), "justify-content" =>
            Some(Self::JustifyContent(VendorPrefix::NONE)), "place-content" =>
            Some(Self::PlaceContent), "align-self" =>
            Some(Self::AlignSelf(VendorPrefix::NONE)), "justify-self" =>
            Some(Self::JustifySelf), "place-self" => Some(Self::PlaceSelf), "align-items"
            => Some(Self::AlignItems(VendorPrefix::NONE)), "justify-items" =>
            Some(Self::JustifyItems), "place-items" => Some(Self::PlaceItems), "row-gap"
            => Some(Self::RowGap), "column-gap" => Some(Self::ColumnGap), "gap" =>
            Some(Self::Gap), "column-rule" => Some(Self::ColumnRule(VendorPrefix::NONE)),
            "column-width" => Some(Self::ColumnWidth(VendorPrefix::NONE)), "column-count"
            => Some(Self::ColumnCount(VendorPrefix::NONE)), "columns" =>
            Some(Self::Columns(VendorPrefix::NONE)), "grid-column-gap" =>
            Some(Self::GridColumnGap), "grid-row-gap" => Some(Self::GridRowGap),
            "box-orient" => Some(Self::BoxOrient(VendorPrefix::NONE)), "box-direction" =>
            Some(Self::BoxDirection(VendorPrefix::NONE)), "box-ordinal-group" =>
            Some(Self::BoxOrdinalGroup(VendorPrefix::NONE)), "box-align" =>
            Some(Self::BoxAlign(VendorPrefix::NONE)), "box-flex" =>
            Some(Self::BoxFlex(VendorPrefix::NONE)), "box-flex-group" =>
            Some(Self::BoxFlexGroup(VendorPrefix::NONE)), "box-pack" =>
            Some(Self::BoxPack(VendorPrefix::NONE)), "box-lines" =>
            Some(Self::BoxLines(VendorPrefix::NONE)), "flex-pack" =>
            Some(Self::FlexPack(VendorPrefix::NONE)), "flex-order" =>
            Some(Self::FlexOrder(VendorPrefix::NONE)), "flex-align" =>
            Some(Self::FlexAlign(VendorPrefix::NONE)), "flex-item-align" =>
            Some(Self::FlexItemAlign(VendorPrefix::NONE)), "flex-line-pack" =>
            Some(Self::FlexLinePack(VendorPrefix::NONE)), "flex-positive" =>
            Some(Self::FlexPositive(VendorPrefix::NONE)), "flex-negative" =>
            Some(Self::FlexNegative(VendorPrefix::NONE)), "flex-preferred-size" =>
            Some(Self::FlexPreferredSize(VendorPrefix::NONE)), "grid-template-columns" =>
            Some(Self::GridTemplateColumns), "grid-template-rows" =>
            Some(Self::GridTemplateRows), "grid-auto-columns" =>
            Some(Self::GridAutoColumns), "grid-auto-rows" => Some(Self::GridAutoRows),
            "grid-auto-flow" => Some(Self::GridAutoFlow), "grid-template-areas" =>
            Some(Self::GridTemplateAreas), "grid-template" => Some(Self::GridTemplate),
            "grid" => Some(Self::Grid), "grid-row-start" => Some(Self::GridRowStart),
            "grid-row-end" => Some(Self::GridRowEnd), "grid-column-start" =>
            Some(Self::GridColumnStart), "grid-column-end" => Some(Self::GridColumnEnd),
            "grid-row" => Some(Self::GridRow), "grid-column" => Some(Self::GridColumn),
            "grid-area" => Some(Self::GridArea), "margin-top" => Some(Self::MarginTop),
            "margin-bottom" => Some(Self::MarginBottom), "margin-left" =>
            Some(Self::MarginLeft), "margin-right" => Some(Self::MarginRight),
            "margin-block-start" => Some(Self::MarginBlockStart), "margin-block-end" =>
            Some(Self::MarginBlockEnd), "margin-inline-start" =>
            Some(Self::MarginInlineStart), "margin-inline-end" =>
            Some(Self::MarginInlineEnd), "margin-block" => Some(Self::MarginBlock),
            "margin-inline" => Some(Self::MarginInline), "margin" => Some(Self::Margin),
            "padding-top" => Some(Self::PaddingTop), "padding-bottom" =>
            Some(Self::PaddingBottom), "padding-left" => Some(Self::PaddingLeft),
            "padding-right" => Some(Self::PaddingRight), "padding-block-start" =>
            Some(Self::PaddingBlockStart), "padding-block-end" =>
            Some(Self::PaddingBlockEnd), "padding-inline-start" =>
            Some(Self::PaddingInlineStart), "padding-inline-end" =>
            Some(Self::PaddingInlineEnd), "padding-block" => Some(Self::PaddingBlock),
            "padding-inline" => Some(Self::PaddingInline), "padding" =>
            Some(Self::Padding), "scroll-margin-top" => Some(Self::ScrollMarginTop),
            "scroll-margin-bottom" => Some(Self::ScrollMarginBottom),
            "scroll-margin-left" => Some(Self::ScrollMarginLeft), "scroll-margin-right"
            => Some(Self::ScrollMarginRight), "scroll-margin-block-start" =>
            Some(Self::ScrollMarginBlockStart), "scroll-margin-block-end" =>
            Some(Self::ScrollMarginBlockEnd), "scroll-margin-inline-start" =>
            Some(Self::ScrollMarginInlineStart), "scroll-margin-inline-end" =>
            Some(Self::ScrollMarginInlineEnd), "scroll-margin-block" =>
            Some(Self::ScrollMarginBlock), "scroll-margin-inline" =>
            Some(Self::ScrollMarginInline), "scroll-margin" => Some(Self::ScrollMargin),
            "scroll-padding-top" => Some(Self::ScrollPaddingTop), "scroll-padding-bottom"
            => Some(Self::ScrollPaddingBottom), "scroll-padding-left" =>
            Some(Self::ScrollPaddingLeft), "scroll-padding-right" =>
            Some(Self::ScrollPaddingRight), "scroll-padding-block-start" =>
            Some(Self::ScrollPaddingBlockStart), "scroll-padding-block-end" =>
            Some(Self::ScrollPaddingBlockEnd), "scroll-padding-inline-start" =>
            Some(Self::ScrollPaddingInlineStart), "scroll-padding-inline-end" =>
            Some(Self::ScrollPaddingInlineEnd), "scroll-padding-block" =>
            Some(Self::ScrollPaddingBlock), "scroll-padding-inline" =>
            Some(Self::ScrollPaddingInline), "scroll-padding" =>
            Some(Self::ScrollPadding), "font-weight" => Some(Self::FontWeight),
            "font-size" => Some(Self::FontSize), "font-stretch" =>
            Some(Self::FontStretch), "font-family" => Some(Self::FontFamily),
            "font-style" => Some(Self::FontStyle), "font-variant-caps" =>
            Some(Self::FontVariantCaps), "line-height" => Some(Self::LineHeight), "font"
            => Some(Self::Font), "vertical-align" => Some(Self::VerticalAlign),
            "font-palette" => Some(Self::FontPalette), "transition-property" =>
            Some(Self::TransitionProperty(VendorPrefix::NONE)), "transition-duration" =>
            Some(Self::TransitionDuration(VendorPrefix::NONE)), "transition-delay" =>
            Some(Self::TransitionDelay(VendorPrefix::NONE)), "transition-timing-function"
            => Some(Self::TransitionTimingFunction(VendorPrefix::NONE)), "transition" =>
            Some(Self::Transition(VendorPrefix::NONE)), "animation-name" =>
            Some(Self::AnimationName(VendorPrefix::NONE)), "animation-duration" =>
            Some(Self::AnimationDuration(VendorPrefix::NONE)),
            "animation-timing-function" =>
            Some(Self::AnimationTimingFunction(VendorPrefix::NONE)),
            "animation-iteration-count" =>
            Some(Self::AnimationIterationCount(VendorPrefix::NONE)),
            "animation-direction" => Some(Self::AnimationDirection(VendorPrefix::NONE)),
            "animation-play-state" => Some(Self::AnimationPlayState(VendorPrefix::NONE)),
            "animation-delay" => Some(Self::AnimationDelay(VendorPrefix::NONE)),
            "animation-fill-mode" => Some(Self::AnimationFillMode(VendorPrefix::NONE)),
            "animation-composition" => Some(Self::AnimationComposition),
            "animation-timeline" => Some(Self::AnimationTimeline),
            "animation-range-start" => Some(Self::AnimationRangeStart),
            "animation-range-end" => Some(Self::AnimationRangeEnd), "animation-range" =>
            Some(Self::AnimationRange), "animation" =>
            Some(Self::Animation(VendorPrefix::NONE)), "transform" =>
            Some(Self::Transform(VendorPrefix::NONE)), "transform-origin" =>
            Some(Self::TransformOrigin(VendorPrefix::NONE)), "transform-style" =>
            Some(Self::TransformStyle(VendorPrefix::NONE)), "transform-box" =>
            Some(Self::TransformBox), "backface-visibility" =>
            Some(Self::BackfaceVisibility(VendorPrefix::NONE)), "perspective" =>
            Some(Self::Perspective(VendorPrefix::NONE)), "perspective-origin" =>
            Some(Self::PerspectiveOrigin(VendorPrefix::NONE)), "translate" =>
            Some(Self::Translate), "rotate" => Some(Self::Rotate), "scale" =>
            Some(Self::Scale), "text-transform" => Some(Self::TextTransform), "content"
            => Some(Self::Content), "white-space" => Some(Self::WhiteSpace), "tab-size"
            => Some(Self::TabSize(VendorPrefix::NONE)), "word-break" =>
            Some(Self::WordBreak), "line-break" => Some(Self::LineBreak), "hyphens" =>
            Some(Self::Hyphens(VendorPrefix::NONE)), "overflow-wrap" =>
            Some(Self::OverflowWrap), "word-wrap" => Some(Self::WordWrap), "text-align"
            => Some(Self::TextAlign), "text-align-last" =>
            Some(Self::TextAlignLast(VendorPrefix::NONE)), "text-justify" =>
            Some(Self::TextJustify), "word-spacing" => Some(Self::WordSpacing),
            "letter-spacing" => Some(Self::LetterSpacing), "text-indent" =>
            Some(Self::TextIndent), "text-decoration-line" =>
            Some(Self::TextDecorationLine(VendorPrefix::NONE)), "text-decoration-style"
            => Some(Self::TextDecorationStyle(VendorPrefix::NONE)),
            "text-decoration-color" =>
            Some(Self::TextDecorationColor(VendorPrefix::NONE)),
            "text-decoration-thickness" => Some(Self::TextDecorationThickness),
            "text-decoration" => Some(Self::TextDecoration(VendorPrefix::NONE)),
            "text-decoration-skip-ink" =>
            Some(Self::TextDecorationSkipInk(VendorPrefix::NONE)), "text-emphasis-style"
            => Some(Self::TextEmphasisStyle(VendorPrefix::NONE)), "text-emphasis-color"
            => Some(Self::TextEmphasisColor(VendorPrefix::NONE)), "text-emphasis" =>
            Some(Self::TextEmphasis(VendorPrefix::NONE)), "text-emphasis-position" =>
            Some(Self::TextEmphasisPosition(VendorPrefix::NONE)), "text-shadow" =>
            Some(Self::TextShadow), "text-size-adjust" =>
            Some(Self::TextSizeAdjust(VendorPrefix::NONE)), "direction" =>
            Some(Self::Direction), "unicode-bidi" => Some(Self::UnicodeBidi),
            "box-decoration-break" => Some(Self::BoxDecorationBreak(VendorPrefix::NONE)),
            "resize" => Some(Self::Resize), "pointer-events" =>
            Some(Self::PointerEvents), "float" => Some(Self::Float), "clear" =>
            Some(Self::Clear), "touch-action" => Some(Self::TouchAction),
            "scroll-behavior" => Some(Self::ScrollBehavior), "cursor" =>
            Some(Self::Cursor), "caret-color" => Some(Self::CaretColor), "caret-shape" =>
            Some(Self::CaretShape), "caret" => Some(Self::Caret), "user-select" =>
            Some(Self::UserSelect(VendorPrefix::NONE)), "accent-color" =>
            Some(Self::AccentColor), "appearance" =>
            Some(Self::Appearance(VendorPrefix::NONE)), "list-style-type" =>
            Some(Self::ListStyleType), "list-style-image" => Some(Self::ListStyleImage),
            "list-style-position" => Some(Self::ListStylePosition), "list-style" =>
            Some(Self::ListStyle), "marker-side" => Some(Self::MarkerSide), "composes" =>
            Some(Self::Composes), "fill" => Some(Self::Fill), "fill-rule" =>
            Some(Self::FillRule), "fill-opacity" => Some(Self::FillOpacity), "stroke" =>
            Some(Self::Stroke), "stroke-opacity" => Some(Self::StrokeOpacity),
            "stroke-width" => Some(Self::StrokeWidth), "stroke-linecap" =>
            Some(Self::StrokeLinecap), "stroke-linejoin" => Some(Self::StrokeLinejoin),
            "stroke-miterlimit" => Some(Self::StrokeMiterlimit), "stroke-dasharray" =>
            Some(Self::StrokeDasharray), "stroke-dashoffset" =>
            Some(Self::StrokeDashoffset), "marker-start" => Some(Self::MarkerStart),
            "marker-mid" => Some(Self::MarkerMid), "marker-end" => Some(Self::MarkerEnd),
            "marker" => Some(Self::Marker), "color-interpolation" =>
            Some(Self::ColorInterpolation), "color-interpolation-filters" =>
            Some(Self::ColorInterpolationFilters), "color-rendering" =>
            Some(Self::ColorRendering), "shape-rendering" => Some(Self::ShapeRendering),
            "text-rendering" => Some(Self::TextRendering), "image-rendering" =>
            Some(Self::ImageRendering), "clip-path" =>
            Some(Self::ClipPath(VendorPrefix::NONE)), "clip-rule" =>
            Some(Self::ClipRule), "mask-image" =>
            Some(Self::MaskImage(VendorPrefix::NONE)), "mask-mode" =>
            Some(Self::MaskMode), "mask-repeat" =>
            Some(Self::MaskRepeat(VendorPrefix::NONE)), "mask-position-x" =>
            Some(Self::MaskPositionX), "mask-position-y" => Some(Self::MaskPositionY),
            "mask-position" => Some(Self::MaskPosition(VendorPrefix::NONE)), "mask-clip"
            => Some(Self::MaskClip(VendorPrefix::NONE)), "mask-origin" =>
            Some(Self::MaskOrigin(VendorPrefix::NONE)), "mask-size" =>
            Some(Self::MaskSize(VendorPrefix::NONE)), "mask-composite" =>
            Some(Self::MaskComposite), "mask-type" => Some(Self::MaskType), "mask" =>
            Some(Self::Mask(VendorPrefix::NONE)), "mask-border-source" =>
            Some(Self::MaskBorderSource), "mask-border-mode" =>
            Some(Self::MaskBorderMode), "mask-border-slice" =>
            Some(Self::MaskBorderSlice), "mask-border-width" =>
            Some(Self::MaskBorderWidth), "mask-border-outset" =>
            Some(Self::MaskBorderOutset), "mask-border-repeat" =>
            Some(Self::MaskBorderRepeat), "mask-border" => Some(Self::MaskBorder),
            "-webkit-mask-composite" => Some(Self::WebKitMaskComposite),
            "mask-source-type" => Some(Self::WebKitMaskSourceType(VendorPrefix::NONE)),
            "mask-box-image" => Some(Self::WebKitMaskBoxImage(VendorPrefix::NONE)),
            "mask-box-image-source" =>
            Some(Self::WebKitMaskBoxImageSource(VendorPrefix::NONE)),
            "mask-box-image-slice" =>
            Some(Self::WebKitMaskBoxImageSlice(VendorPrefix::NONE)),
            "mask-box-image-width" =>
            Some(Self::WebKitMaskBoxImageWidth(VendorPrefix::NONE)),
            "mask-box-image-outset" =>
            Some(Self::WebKitMaskBoxImageOutset(VendorPrefix::NONE)),
            "mask-box-image-repeat" =>
            Some(Self::WebKitMaskBoxImageRepeat(VendorPrefix::NONE)), "filter" =>
            Some(Self::Filter(VendorPrefix::NONE)), "backdrop-filter" =>
            Some(Self::BackdropFilter(VendorPrefix::NONE)), "mix-blend-mode" =>
            Some(Self::MixBlendMode), "z-index" => Some(Self::ZIndex), "container-type"
            => Some(Self::ContainerType), "container-name" => Some(Self::ContainerName),
            "container" => Some(Self::Container), "view-transition-name" =>
            Some(Self::ViewTransitionName), "view-transition-class" =>
            Some(Self::ViewTransitionClass), "view-transition-group" =>
            Some(Self::ViewTransitionGroup), "color-scheme" => Some(Self::ColorScheme),
            "print-color-adjust" => Some(Self::PrintColorAdjust(VendorPrefix::NONE)), _
            => None,
        );
        if let Some(property_id) = property_id {
            return property_id;
        }
        if let Some((prefix, unprefixed_name)) = VendorPrefix::split_from_name(name) {
            let property_id = crate::match_ignore_ascii_case!(
                unprefixed_name, "background-clip" => Some(Self::BackgroundClip(prefix)),
                "box-shadow" => Some(Self::BoxShadow(prefix)), "box-sizing" =>
                Some(Self::BoxSizing(prefix)), "text-overflow" =>
                Some(Self::TextOverflow(prefix)), "border-top-left-radius" =>
                Some(Self::BorderTopLeftRadius(prefix)), "border-top-right-radius" =>
                Some(Self::BorderTopRightRadius(prefix)), "border-bottom-left-radius" =>
                Some(Self::BorderBottomLeftRadius(prefix)), "border-bottom-right-radius"
                => Some(Self::BorderBottomRightRadius(prefix)), "border-radius" =>
                Some(Self::BorderRadius(prefix)), "border-image" =>
                Some(Self::BorderImage(prefix)), "flex-direction" =>
                Some(Self::FlexDirection(prefix)), "flex-wrap" =>
                Some(Self::FlexWrap(prefix)), "flex-flow" =>
                Some(Self::FlexFlow(prefix)), "flex-grow" =>
                Some(Self::FlexGrow(prefix)), "flex-shrink" =>
                Some(Self::FlexShrink(prefix)), "flex-basis" =>
                Some(Self::FlexBasis(prefix)), "flex" => Some(Self::Flex(prefix)),
                "order" => Some(Self::Order(prefix)), "align-content" =>
                Some(Self::AlignContent(prefix)), "justify-content" =>
                Some(Self::JustifyContent(prefix)), "align-self" =>
                Some(Self::AlignSelf(prefix)), "align-items" =>
                Some(Self::AlignItems(prefix)), "column-rule" =>
                Some(Self::ColumnRule(prefix)), "column-width" =>
                Some(Self::ColumnWidth(prefix)), "column-count" =>
                Some(Self::ColumnCount(prefix)), "columns" =>
                Some(Self::Columns(prefix)), "box-orient" =>
                Some(Self::BoxOrient(prefix)), "box-direction" =>
                Some(Self::BoxDirection(prefix)), "box-ordinal-group" =>
                Some(Self::BoxOrdinalGroup(prefix)), "box-align" =>
                Some(Self::BoxAlign(prefix)), "box-flex" => Some(Self::BoxFlex(prefix)),
                "box-flex-group" => Some(Self::BoxFlexGroup(prefix)), "box-pack" =>
                Some(Self::BoxPack(prefix)), "box-lines" => Some(Self::BoxLines(prefix)),
                "flex-pack" => Some(Self::FlexPack(prefix)), "flex-order" =>
                Some(Self::FlexOrder(prefix)), "flex-align" =>
                Some(Self::FlexAlign(prefix)), "flex-item-align" =>
                Some(Self::FlexItemAlign(prefix)), "flex-line-pack" =>
                Some(Self::FlexLinePack(prefix)), "flex-positive" =>
                Some(Self::FlexPositive(prefix)), "flex-negative" =>
                Some(Self::FlexNegative(prefix)), "flex-preferred-size" =>
                Some(Self::FlexPreferredSize(prefix)), "transition-property" =>
                Some(Self::TransitionProperty(prefix)), "transition-duration" =>
                Some(Self::TransitionDuration(prefix)), "transition-delay" =>
                Some(Self::TransitionDelay(prefix)), "transition-timing-function" =>
                Some(Self::TransitionTimingFunction(prefix)), "transition" =>
                Some(Self::Transition(prefix)), "animation-name" =>
                Some(Self::AnimationName(prefix)), "animation-duration" =>
                Some(Self::AnimationDuration(prefix)), "animation-timing-function" =>
                Some(Self::AnimationTimingFunction(prefix)), "animation-iteration-count"
                => Some(Self::AnimationIterationCount(prefix)), "animation-direction" =>
                Some(Self::AnimationDirection(prefix)), "animation-play-state" =>
                Some(Self::AnimationPlayState(prefix)), "animation-delay" =>
                Some(Self::AnimationDelay(prefix)), "animation-fill-mode" =>
                Some(Self::AnimationFillMode(prefix)), "animation" =>
                Some(Self::Animation(prefix)), "transform" =>
                Some(Self::Transform(prefix)), "transform-origin" =>
                Some(Self::TransformOrigin(prefix)), "transform-style" =>
                Some(Self::TransformStyle(prefix)), "backface-visibility" =>
                Some(Self::BackfaceVisibility(prefix)), "perspective" =>
                Some(Self::Perspective(prefix)), "perspective-origin" =>
                Some(Self::PerspectiveOrigin(prefix)), "tab-size" =>
                Some(Self::TabSize(prefix)), "hyphens" => Some(Self::Hyphens(prefix)),
                "text-align-last" => Some(Self::TextAlignLast(prefix)),
                "text-decoration-line" => Some(Self::TextDecorationLine(prefix)),
                "text-decoration-style" => Some(Self::TextDecorationStyle(prefix)),
                "text-decoration-color" => Some(Self::TextDecorationColor(prefix)),
                "text-decoration" => Some(Self::TextDecoration(prefix)),
                "text-decoration-skip-ink" => Some(Self::TextDecorationSkipInk(prefix)),
                "text-emphasis-style" => Some(Self::TextEmphasisStyle(prefix)),
                "text-emphasis-color" => Some(Self::TextEmphasisColor(prefix)),
                "text-emphasis" => Some(Self::TextEmphasis(prefix)),
                "text-emphasis-position" => Some(Self::TextEmphasisPosition(prefix)),
                "text-size-adjust" => Some(Self::TextSizeAdjust(prefix)),
                "box-decoration-break" => Some(Self::BoxDecorationBreak(prefix)),
                "user-select" => Some(Self::UserSelect(prefix)), "appearance" =>
                Some(Self::Appearance(prefix)), "clip-path" =>
                Some(Self::ClipPath(prefix)), "mask-image" =>
                Some(Self::MaskImage(prefix)), "mask-repeat" =>
                Some(Self::MaskRepeat(prefix)), "mask-position" =>
                Some(Self::MaskPosition(prefix)), "mask-clip" =>
                Some(Self::MaskClip(prefix)), "mask-origin" =>
                Some(Self::MaskOrigin(prefix)), "mask-size" =>
                Some(Self::MaskSize(prefix)), "mask" => Some(Self::Mask(prefix)),
                "mask-source-type" => Some(Self::WebKitMaskSourceType(prefix)),
                "mask-box-image" => Some(Self::WebKitMaskBoxImage(prefix)),
                "mask-box-image-source" => Some(Self::WebKitMaskBoxImageSource(prefix)),
                "mask-box-image-slice" => Some(Self::WebKitMaskBoxImageSlice(prefix)),
                "mask-box-image-width" => Some(Self::WebKitMaskBoxImageWidth(prefix)),
                "mask-box-image-outset" => Some(Self::WebKitMaskBoxImageOutset(prefix)),
                "mask-box-image-repeat" => Some(Self::WebKitMaskBoxImageRepeat(prefix)),
                "filter" => Some(Self::Filter(prefix)), "backdrop-filter" =>
                Some(Self::BackdropFilter(prefix)), "print-color-adjust" =>
                Some(Self::PrintColorAdjust(prefix)), _ => None,
            );
            if let Some(property_id) = property_id {
                return property_id;
            }
        }
        Self::Custom(name)
    }
    /// Returns the canonical CSS property name.
    pub fn name(&self) -> &'ast str {
        match self {
            Self::All => "all",
            Self::BackgroundColor => "background-color",
            Self::BackgroundImage => "background-image",
            Self::BackgroundPositionX => "background-position-x",
            Self::BackgroundPositionY => "background-position-y",
            Self::BackgroundPosition => "background-position",
            Self::BackgroundSize => "background-size",
            Self::BackgroundRepeat => "background-repeat",
            Self::BackgroundAttachment => "background-attachment",
            Self::BackgroundClip(_) => "background-clip",
            Self::BackgroundOrigin => "background-origin",
            Self::Background => "background",
            Self::BoxShadow(_) => "box-shadow",
            Self::Opacity => "opacity",
            Self::Color => "color",
            Self::Display => "display",
            Self::Visibility => "visibility",
            Self::Width => "width",
            Self::Height => "height",
            Self::MinWidth => "min-width",
            Self::MinHeight => "min-height",
            Self::MaxWidth => "max-width",
            Self::MaxHeight => "max-height",
            Self::BlockSize => "block-size",
            Self::InlineSize => "inline-size",
            Self::MinBlockSize => "min-block-size",
            Self::MinInlineSize => "min-inline-size",
            Self::MaxBlockSize => "max-block-size",
            Self::MaxInlineSize => "max-inline-size",
            Self::BoxSizing(_) => "box-sizing",
            Self::AspectRatio => "aspect-ratio",
            Self::Overflow => "overflow",
            Self::OverflowX => "overflow-x",
            Self::OverflowY => "overflow-y",
            Self::TextOverflow(_) => "text-overflow",
            Self::ObjectFit => "object-fit",
            Self::ObjectPosition => "object-position",
            Self::ScrollbarColor => "scrollbar-color",
            Self::Position => "position",
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
            Self::InsetBlockStart => "inset-block-start",
            Self::InsetBlockEnd => "inset-block-end",
            Self::InsetInlineStart => "inset-inline-start",
            Self::InsetInlineEnd => "inset-inline-end",
            Self::InsetBlock => "inset-block",
            Self::InsetInline => "inset-inline",
            Self::Inset => "inset",
            Self::BorderSpacing => "border-spacing",
            Self::BorderTopColor => "border-top-color",
            Self::BorderBottomColor => "border-bottom-color",
            Self::BorderLeftColor => "border-left-color",
            Self::BorderRightColor => "border-right-color",
            Self::BorderBlockStartColor => "border-block-start-color",
            Self::BorderBlockEndColor => "border-block-end-color",
            Self::BorderInlineStartColor => "border-inline-start-color",
            Self::BorderInlineEndColor => "border-inline-end-color",
            Self::BorderTopStyle => "border-top-style",
            Self::BorderBottomStyle => "border-bottom-style",
            Self::BorderLeftStyle => "border-left-style",
            Self::BorderRightStyle => "border-right-style",
            Self::BorderBlockStartStyle => "border-block-start-style",
            Self::BorderBlockEndStyle => "border-block-end-style",
            Self::BorderInlineStartStyle => "border-inline-start-style",
            Self::BorderInlineEndStyle => "border-inline-end-style",
            Self::BorderTopWidth => "border-top-width",
            Self::BorderBottomWidth => "border-bottom-width",
            Self::BorderLeftWidth => "border-left-width",
            Self::BorderRightWidth => "border-right-width",
            Self::BorderBlockStartWidth => "border-block-start-width",
            Self::BorderBlockEndWidth => "border-block-end-width",
            Self::BorderInlineStartWidth => "border-inline-start-width",
            Self::BorderInlineEndWidth => "border-inline-end-width",
            Self::BorderTopLeftRadius(_) => "border-top-left-radius",
            Self::BorderTopRightRadius(_) => "border-top-right-radius",
            Self::BorderBottomLeftRadius(_) => "border-bottom-left-radius",
            Self::BorderBottomRightRadius(_) => "border-bottom-right-radius",
            Self::BorderStartStartRadius => "border-start-start-radius",
            Self::BorderStartEndRadius => "border-start-end-radius",
            Self::BorderEndStartRadius => "border-end-start-radius",
            Self::BorderEndEndRadius => "border-end-end-radius",
            Self::BorderRadius(_) => "border-radius",
            Self::BorderImageSource => "border-image-source",
            Self::BorderImageOutset => "border-image-outset",
            Self::BorderImageRepeat => "border-image-repeat",
            Self::BorderImageWidth => "border-image-width",
            Self::BorderImageSlice => "border-image-slice",
            Self::BorderImage(_) => "border-image",
            Self::BorderColor => "border-color",
            Self::BorderStyle => "border-style",
            Self::BorderWidth => "border-width",
            Self::BorderBlockColor => "border-block-color",
            Self::BorderBlockStyle => "border-block-style",
            Self::BorderBlockWidth => "border-block-width",
            Self::BorderInlineColor => "border-inline-color",
            Self::BorderInlineStyle => "border-inline-style",
            Self::BorderInlineWidth => "border-inline-width",
            Self::Border => "border",
            Self::BorderTop => "border-top",
            Self::BorderBottom => "border-bottom",
            Self::BorderLeft => "border-left",
            Self::BorderRight => "border-right",
            Self::BorderBlock => "border-block",
            Self::BorderBlockStart => "border-block-start",
            Self::BorderBlockEnd => "border-block-end",
            Self::BorderInline => "border-inline",
            Self::BorderInlineStart => "border-inline-start",
            Self::BorderInlineEnd => "border-inline-end",
            Self::Outline => "outline",
            Self::OutlineColor => "outline-color",
            Self::OutlineStyle => "outline-style",
            Self::OutlineWidth => "outline-width",
            Self::FlexDirection(_) => "flex-direction",
            Self::FlexWrap(_) => "flex-wrap",
            Self::FlexFlow(_) => "flex-flow",
            Self::FlexGrow(_) => "flex-grow",
            Self::FlexShrink(_) => "flex-shrink",
            Self::FlexBasis(_) => "flex-basis",
            Self::Flex(_) => "flex",
            Self::Order(_) => "order",
            Self::AlignContent(_) => "align-content",
            Self::JustifyContent(_) => "justify-content",
            Self::PlaceContent => "place-content",
            Self::AlignSelf(_) => "align-self",
            Self::JustifySelf => "justify-self",
            Self::PlaceSelf => "place-self",
            Self::AlignItems(_) => "align-items",
            Self::JustifyItems => "justify-items",
            Self::PlaceItems => "place-items",
            Self::RowGap => "row-gap",
            Self::ColumnGap => "column-gap",
            Self::Gap => "gap",
            Self::ColumnRule(_) => "column-rule",
            Self::ColumnWidth(_) => "column-width",
            Self::ColumnCount(_) => "column-count",
            Self::Columns(_) => "columns",
            Self::GridColumnGap => "grid-column-gap",
            Self::GridRowGap => "grid-row-gap",
            Self::BoxOrient(_) => "box-orient",
            Self::BoxDirection(_) => "box-direction",
            Self::BoxOrdinalGroup(_) => "box-ordinal-group",
            Self::BoxAlign(_) => "box-align",
            Self::BoxFlex(_) => "box-flex",
            Self::BoxFlexGroup(_) => "box-flex-group",
            Self::BoxPack(_) => "box-pack",
            Self::BoxLines(_) => "box-lines",
            Self::FlexPack(_) => "flex-pack",
            Self::FlexOrder(_) => "flex-order",
            Self::FlexAlign(_) => "flex-align",
            Self::FlexItemAlign(_) => "flex-item-align",
            Self::FlexLinePack(_) => "flex-line-pack",
            Self::FlexPositive(_) => "flex-positive",
            Self::FlexNegative(_) => "flex-negative",
            Self::FlexPreferredSize(_) => "flex-preferred-size",
            Self::GridTemplateColumns => "grid-template-columns",
            Self::GridTemplateRows => "grid-template-rows",
            Self::GridAutoColumns => "grid-auto-columns",
            Self::GridAutoRows => "grid-auto-rows",
            Self::GridAutoFlow => "grid-auto-flow",
            Self::GridTemplateAreas => "grid-template-areas",
            Self::GridTemplate => "grid-template",
            Self::Grid => "grid",
            Self::GridRowStart => "grid-row-start",
            Self::GridRowEnd => "grid-row-end",
            Self::GridColumnStart => "grid-column-start",
            Self::GridColumnEnd => "grid-column-end",
            Self::GridRow => "grid-row",
            Self::GridColumn => "grid-column",
            Self::GridArea => "grid-area",
            Self::MarginTop => "margin-top",
            Self::MarginBottom => "margin-bottom",
            Self::MarginLeft => "margin-left",
            Self::MarginRight => "margin-right",
            Self::MarginBlockStart => "margin-block-start",
            Self::MarginBlockEnd => "margin-block-end",
            Self::MarginInlineStart => "margin-inline-start",
            Self::MarginInlineEnd => "margin-inline-end",
            Self::MarginBlock => "margin-block",
            Self::MarginInline => "margin-inline",
            Self::Margin => "margin",
            Self::PaddingTop => "padding-top",
            Self::PaddingBottom => "padding-bottom",
            Self::PaddingLeft => "padding-left",
            Self::PaddingRight => "padding-right",
            Self::PaddingBlockStart => "padding-block-start",
            Self::PaddingBlockEnd => "padding-block-end",
            Self::PaddingInlineStart => "padding-inline-start",
            Self::PaddingInlineEnd => "padding-inline-end",
            Self::PaddingBlock => "padding-block",
            Self::PaddingInline => "padding-inline",
            Self::Padding => "padding",
            Self::ScrollMarginTop => "scroll-margin-top",
            Self::ScrollMarginBottom => "scroll-margin-bottom",
            Self::ScrollMarginLeft => "scroll-margin-left",
            Self::ScrollMarginRight => "scroll-margin-right",
            Self::ScrollMarginBlockStart => "scroll-margin-block-start",
            Self::ScrollMarginBlockEnd => "scroll-margin-block-end",
            Self::ScrollMarginInlineStart => "scroll-margin-inline-start",
            Self::ScrollMarginInlineEnd => "scroll-margin-inline-end",
            Self::ScrollMarginBlock => "scroll-margin-block",
            Self::ScrollMarginInline => "scroll-margin-inline",
            Self::ScrollMargin => "scroll-margin",
            Self::ScrollPaddingTop => "scroll-padding-top",
            Self::ScrollPaddingBottom => "scroll-padding-bottom",
            Self::ScrollPaddingLeft => "scroll-padding-left",
            Self::ScrollPaddingRight => "scroll-padding-right",
            Self::ScrollPaddingBlockStart => "scroll-padding-block-start",
            Self::ScrollPaddingBlockEnd => "scroll-padding-block-end",
            Self::ScrollPaddingInlineStart => "scroll-padding-inline-start",
            Self::ScrollPaddingInlineEnd => "scroll-padding-inline-end",
            Self::ScrollPaddingBlock => "scroll-padding-block",
            Self::ScrollPaddingInline => "scroll-padding-inline",
            Self::ScrollPadding => "scroll-padding",
            Self::FontWeight => "font-weight",
            Self::FontSize => "font-size",
            Self::FontStretch => "font-stretch",
            Self::FontFamily => "font-family",
            Self::FontStyle => "font-style",
            Self::FontVariantCaps => "font-variant-caps",
            Self::LineHeight => "line-height",
            Self::Font => "font",
            Self::VerticalAlign => "vertical-align",
            Self::FontPalette => "font-palette",
            Self::TransitionProperty(_) => "transition-property",
            Self::TransitionDuration(_) => "transition-duration",
            Self::TransitionDelay(_) => "transition-delay",
            Self::TransitionTimingFunction(_) => "transition-timing-function",
            Self::Transition(_) => "transition",
            Self::AnimationName(_) => "animation-name",
            Self::AnimationDuration(_) => "animation-duration",
            Self::AnimationTimingFunction(_) => "animation-timing-function",
            Self::AnimationIterationCount(_) => "animation-iteration-count",
            Self::AnimationDirection(_) => "animation-direction",
            Self::AnimationPlayState(_) => "animation-play-state",
            Self::AnimationDelay(_) => "animation-delay",
            Self::AnimationFillMode(_) => "animation-fill-mode",
            Self::AnimationComposition => "animation-composition",
            Self::AnimationTimeline => "animation-timeline",
            Self::AnimationRangeStart => "animation-range-start",
            Self::AnimationRangeEnd => "animation-range-end",
            Self::AnimationRange => "animation-range",
            Self::Animation(_) => "animation",
            Self::Transform(_) => "transform",
            Self::TransformOrigin(_) => "transform-origin",
            Self::TransformStyle(_) => "transform-style",
            Self::TransformBox => "transform-box",
            Self::BackfaceVisibility(_) => "backface-visibility",
            Self::Perspective(_) => "perspective",
            Self::PerspectiveOrigin(_) => "perspective-origin",
            Self::Translate => "translate",
            Self::Rotate => "rotate",
            Self::Scale => "scale",
            Self::TextTransform => "text-transform",
            Self::Content => "content",
            Self::WhiteSpace => "white-space",
            Self::TabSize(_) => "tab-size",
            Self::WordBreak => "word-break",
            Self::LineBreak => "line-break",
            Self::Hyphens(_) => "hyphens",
            Self::OverflowWrap => "overflow-wrap",
            Self::WordWrap => "word-wrap",
            Self::TextAlign => "text-align",
            Self::TextAlignLast(_) => "text-align-last",
            Self::TextJustify => "text-justify",
            Self::WordSpacing => "word-spacing",
            Self::LetterSpacing => "letter-spacing",
            Self::TextIndent => "text-indent",
            Self::TextDecorationLine(_) => "text-decoration-line",
            Self::TextDecorationStyle(_) => "text-decoration-style",
            Self::TextDecorationColor(_) => "text-decoration-color",
            Self::TextDecorationThickness => "text-decoration-thickness",
            Self::TextDecoration(_) => "text-decoration",
            Self::TextDecorationSkipInk(_) => "text-decoration-skip-ink",
            Self::TextEmphasisStyle(_) => "text-emphasis-style",
            Self::TextEmphasisColor(_) => "text-emphasis-color",
            Self::TextEmphasis(_) => "text-emphasis",
            Self::TextEmphasisPosition(_) => "text-emphasis-position",
            Self::TextShadow => "text-shadow",
            Self::TextSizeAdjust(_) => "text-size-adjust",
            Self::Direction => "direction",
            Self::UnicodeBidi => "unicode-bidi",
            Self::BoxDecorationBreak(_) => "box-decoration-break",
            Self::Resize => "resize",
            Self::PointerEvents => "pointer-events",
            Self::Float => "float",
            Self::Clear => "clear",
            Self::TouchAction => "touch-action",
            Self::ScrollBehavior => "scroll-behavior",
            Self::Cursor => "cursor",
            Self::CaretColor => "caret-color",
            Self::CaretShape => "caret-shape",
            Self::Caret => "caret",
            Self::UserSelect(_) => "user-select",
            Self::AccentColor => "accent-color",
            Self::Appearance(_) => "appearance",
            Self::ListStyleType => "list-style-type",
            Self::ListStyleImage => "list-style-image",
            Self::ListStylePosition => "list-style-position",
            Self::ListStyle => "list-style",
            Self::MarkerSide => "marker-side",
            Self::Composes => "composes",
            Self::Fill => "fill",
            Self::FillRule => "fill-rule",
            Self::FillOpacity => "fill-opacity",
            Self::Stroke => "stroke",
            Self::StrokeOpacity => "stroke-opacity",
            Self::StrokeWidth => "stroke-width",
            Self::StrokeLinecap => "stroke-linecap",
            Self::StrokeLinejoin => "stroke-linejoin",
            Self::StrokeMiterlimit => "stroke-miterlimit",
            Self::StrokeDasharray => "stroke-dasharray",
            Self::StrokeDashoffset => "stroke-dashoffset",
            Self::MarkerStart => "marker-start",
            Self::MarkerMid => "marker-mid",
            Self::MarkerEnd => "marker-end",
            Self::Marker => "marker",
            Self::ColorInterpolation => "color-interpolation",
            Self::ColorInterpolationFilters => "color-interpolation-filters",
            Self::ColorRendering => "color-rendering",
            Self::ShapeRendering => "shape-rendering",
            Self::TextRendering => "text-rendering",
            Self::ImageRendering => "image-rendering",
            Self::ClipPath(_) => "clip-path",
            Self::ClipRule => "clip-rule",
            Self::MaskImage(_) => "mask-image",
            Self::MaskMode => "mask-mode",
            Self::MaskRepeat(_) => "mask-repeat",
            Self::MaskPositionX => "mask-position-x",
            Self::MaskPositionY => "mask-position-y",
            Self::MaskPosition(_) => "mask-position",
            Self::MaskClip(_) => "mask-clip",
            Self::MaskOrigin(_) => "mask-origin",
            Self::MaskSize(_) => "mask-size",
            Self::MaskComposite => "mask-composite",
            Self::MaskType => "mask-type",
            Self::Mask(_) => "mask",
            Self::MaskBorderSource => "mask-border-source",
            Self::MaskBorderMode => "mask-border-mode",
            Self::MaskBorderSlice => "mask-border-slice",
            Self::MaskBorderWidth => "mask-border-width",
            Self::MaskBorderOutset => "mask-border-outset",
            Self::MaskBorderRepeat => "mask-border-repeat",
            Self::MaskBorder => "mask-border",
            Self::WebKitMaskComposite => "-webkit-mask-composite",
            Self::WebKitMaskSourceType(_) => "mask-source-type",
            Self::WebKitMaskBoxImage(_) => "mask-box-image",
            Self::WebKitMaskBoxImageSource(_) => "mask-box-image-source",
            Self::WebKitMaskBoxImageSlice(_) => "mask-box-image-slice",
            Self::WebKitMaskBoxImageWidth(_) => "mask-box-image-width",
            Self::WebKitMaskBoxImageOutset(_) => "mask-box-image-outset",
            Self::WebKitMaskBoxImageRepeat(_) => "mask-box-image-repeat",
            Self::Filter(_) => "filter",
            Self::BackdropFilter(_) => "backdrop-filter",
            Self::MixBlendMode => "mix-blend-mode",
            Self::ZIndex => "z-index",
            Self::ContainerType => "container-type",
            Self::ContainerName => "container-name",
            Self::Container => "container",
            Self::ViewTransitionName => "view-transition-name",
            Self::ViewTransitionClass => "view-transition-class",
            Self::ViewTransitionGroup => "view-transition-group",
            Self::ColorScheme => "color-scheme",
            Self::PrintColorAdjust(_) => "print-color-adjust",
            Self::Unparsed => "",
            Self::Custom(name) => name,
        }
    }
    /// Returns the vendor prefix associated with this property identifier.
    pub fn vendor_prefix(&self) -> VendorPrefix {
        match self {
            Self::All => VendorPrefix::NONE,
            Self::BackgroundColor => VendorPrefix::NONE,
            Self::BackgroundImage => VendorPrefix::NONE,
            Self::BackgroundPositionX => VendorPrefix::NONE,
            Self::BackgroundPositionY => VendorPrefix::NONE,
            Self::BackgroundPosition => VendorPrefix::NONE,
            Self::BackgroundSize => VendorPrefix::NONE,
            Self::BackgroundRepeat => VendorPrefix::NONE,
            Self::BackgroundAttachment => VendorPrefix::NONE,
            Self::BackgroundClip(prefix) => *prefix,
            Self::BackgroundOrigin => VendorPrefix::NONE,
            Self::Background => VendorPrefix::NONE,
            Self::BoxShadow(prefix) => *prefix,
            Self::Opacity => VendorPrefix::NONE,
            Self::Color => VendorPrefix::NONE,
            Self::Display => VendorPrefix::NONE,
            Self::Visibility => VendorPrefix::NONE,
            Self::Width => VendorPrefix::NONE,
            Self::Height => VendorPrefix::NONE,
            Self::MinWidth => VendorPrefix::NONE,
            Self::MinHeight => VendorPrefix::NONE,
            Self::MaxWidth => VendorPrefix::NONE,
            Self::MaxHeight => VendorPrefix::NONE,
            Self::BlockSize => VendorPrefix::NONE,
            Self::InlineSize => VendorPrefix::NONE,
            Self::MinBlockSize => VendorPrefix::NONE,
            Self::MinInlineSize => VendorPrefix::NONE,
            Self::MaxBlockSize => VendorPrefix::NONE,
            Self::MaxInlineSize => VendorPrefix::NONE,
            Self::BoxSizing(prefix) => *prefix,
            Self::AspectRatio => VendorPrefix::NONE,
            Self::Overflow => VendorPrefix::NONE,
            Self::OverflowX => VendorPrefix::NONE,
            Self::OverflowY => VendorPrefix::NONE,
            Self::TextOverflow(prefix) => *prefix,
            Self::ObjectFit => VendorPrefix::NONE,
            Self::ObjectPosition => VendorPrefix::NONE,
            Self::ScrollbarColor => VendorPrefix::NONE,
            Self::Position => VendorPrefix::NONE,
            Self::Top => VendorPrefix::NONE,
            Self::Bottom => VendorPrefix::NONE,
            Self::Left => VendorPrefix::NONE,
            Self::Right => VendorPrefix::NONE,
            Self::InsetBlockStart => VendorPrefix::NONE,
            Self::InsetBlockEnd => VendorPrefix::NONE,
            Self::InsetInlineStart => VendorPrefix::NONE,
            Self::InsetInlineEnd => VendorPrefix::NONE,
            Self::InsetBlock => VendorPrefix::NONE,
            Self::InsetInline => VendorPrefix::NONE,
            Self::Inset => VendorPrefix::NONE,
            Self::BorderSpacing => VendorPrefix::NONE,
            Self::BorderTopColor => VendorPrefix::NONE,
            Self::BorderBottomColor => VendorPrefix::NONE,
            Self::BorderLeftColor => VendorPrefix::NONE,
            Self::BorderRightColor => VendorPrefix::NONE,
            Self::BorderBlockStartColor => VendorPrefix::NONE,
            Self::BorderBlockEndColor => VendorPrefix::NONE,
            Self::BorderInlineStartColor => VendorPrefix::NONE,
            Self::BorderInlineEndColor => VendorPrefix::NONE,
            Self::BorderTopStyle => VendorPrefix::NONE,
            Self::BorderBottomStyle => VendorPrefix::NONE,
            Self::BorderLeftStyle => VendorPrefix::NONE,
            Self::BorderRightStyle => VendorPrefix::NONE,
            Self::BorderBlockStartStyle => VendorPrefix::NONE,
            Self::BorderBlockEndStyle => VendorPrefix::NONE,
            Self::BorderInlineStartStyle => VendorPrefix::NONE,
            Self::BorderInlineEndStyle => VendorPrefix::NONE,
            Self::BorderTopWidth => VendorPrefix::NONE,
            Self::BorderBottomWidth => VendorPrefix::NONE,
            Self::BorderLeftWidth => VendorPrefix::NONE,
            Self::BorderRightWidth => VendorPrefix::NONE,
            Self::BorderBlockStartWidth => VendorPrefix::NONE,
            Self::BorderBlockEndWidth => VendorPrefix::NONE,
            Self::BorderInlineStartWidth => VendorPrefix::NONE,
            Self::BorderInlineEndWidth => VendorPrefix::NONE,
            Self::BorderTopLeftRadius(prefix) => *prefix,
            Self::BorderTopRightRadius(prefix) => *prefix,
            Self::BorderBottomLeftRadius(prefix) => *prefix,
            Self::BorderBottomRightRadius(prefix) => *prefix,
            Self::BorderStartStartRadius => VendorPrefix::NONE,
            Self::BorderStartEndRadius => VendorPrefix::NONE,
            Self::BorderEndStartRadius => VendorPrefix::NONE,
            Self::BorderEndEndRadius => VendorPrefix::NONE,
            Self::BorderRadius(prefix) => *prefix,
            Self::BorderImageSource => VendorPrefix::NONE,
            Self::BorderImageOutset => VendorPrefix::NONE,
            Self::BorderImageRepeat => VendorPrefix::NONE,
            Self::BorderImageWidth => VendorPrefix::NONE,
            Self::BorderImageSlice => VendorPrefix::NONE,
            Self::BorderImage(prefix) => *prefix,
            Self::BorderColor => VendorPrefix::NONE,
            Self::BorderStyle => VendorPrefix::NONE,
            Self::BorderWidth => VendorPrefix::NONE,
            Self::BorderBlockColor => VendorPrefix::NONE,
            Self::BorderBlockStyle => VendorPrefix::NONE,
            Self::BorderBlockWidth => VendorPrefix::NONE,
            Self::BorderInlineColor => VendorPrefix::NONE,
            Self::BorderInlineStyle => VendorPrefix::NONE,
            Self::BorderInlineWidth => VendorPrefix::NONE,
            Self::Border => VendorPrefix::NONE,
            Self::BorderTop => VendorPrefix::NONE,
            Self::BorderBottom => VendorPrefix::NONE,
            Self::BorderLeft => VendorPrefix::NONE,
            Self::BorderRight => VendorPrefix::NONE,
            Self::BorderBlock => VendorPrefix::NONE,
            Self::BorderBlockStart => VendorPrefix::NONE,
            Self::BorderBlockEnd => VendorPrefix::NONE,
            Self::BorderInline => VendorPrefix::NONE,
            Self::BorderInlineStart => VendorPrefix::NONE,
            Self::BorderInlineEnd => VendorPrefix::NONE,
            Self::Outline => VendorPrefix::NONE,
            Self::OutlineColor => VendorPrefix::NONE,
            Self::OutlineStyle => VendorPrefix::NONE,
            Self::OutlineWidth => VendorPrefix::NONE,
            Self::FlexDirection(prefix) => *prefix,
            Self::FlexWrap(prefix) => *prefix,
            Self::FlexFlow(prefix) => *prefix,
            Self::FlexGrow(prefix) => *prefix,
            Self::FlexShrink(prefix) => *prefix,
            Self::FlexBasis(prefix) => *prefix,
            Self::Flex(prefix) => *prefix,
            Self::Order(prefix) => *prefix,
            Self::AlignContent(prefix) => *prefix,
            Self::JustifyContent(prefix) => *prefix,
            Self::PlaceContent => VendorPrefix::NONE,
            Self::AlignSelf(prefix) => *prefix,
            Self::JustifySelf => VendorPrefix::NONE,
            Self::PlaceSelf => VendorPrefix::NONE,
            Self::AlignItems(prefix) => *prefix,
            Self::JustifyItems => VendorPrefix::NONE,
            Self::PlaceItems => VendorPrefix::NONE,
            Self::RowGap => VendorPrefix::NONE,
            Self::ColumnGap => VendorPrefix::NONE,
            Self::Gap => VendorPrefix::NONE,
            Self::ColumnRule(prefix) => *prefix,
            Self::ColumnWidth(prefix) => *prefix,
            Self::ColumnCount(prefix) => *prefix,
            Self::Columns(prefix) => *prefix,
            Self::GridColumnGap => VendorPrefix::NONE,
            Self::GridRowGap => VendorPrefix::NONE,
            Self::BoxOrient(prefix) => *prefix,
            Self::BoxDirection(prefix) => *prefix,
            Self::BoxOrdinalGroup(prefix) => *prefix,
            Self::BoxAlign(prefix) => *prefix,
            Self::BoxFlex(prefix) => *prefix,
            Self::BoxFlexGroup(prefix) => *prefix,
            Self::BoxPack(prefix) => *prefix,
            Self::BoxLines(prefix) => *prefix,
            Self::FlexPack(prefix) => *prefix,
            Self::FlexOrder(prefix) => *prefix,
            Self::FlexAlign(prefix) => *prefix,
            Self::FlexItemAlign(prefix) => *prefix,
            Self::FlexLinePack(prefix) => *prefix,
            Self::FlexPositive(prefix) => *prefix,
            Self::FlexNegative(prefix) => *prefix,
            Self::FlexPreferredSize(prefix) => *prefix,
            Self::GridTemplateColumns => VendorPrefix::NONE,
            Self::GridTemplateRows => VendorPrefix::NONE,
            Self::GridAutoColumns => VendorPrefix::NONE,
            Self::GridAutoRows => VendorPrefix::NONE,
            Self::GridAutoFlow => VendorPrefix::NONE,
            Self::GridTemplateAreas => VendorPrefix::NONE,
            Self::GridTemplate => VendorPrefix::NONE,
            Self::Grid => VendorPrefix::NONE,
            Self::GridRowStart => VendorPrefix::NONE,
            Self::GridRowEnd => VendorPrefix::NONE,
            Self::GridColumnStart => VendorPrefix::NONE,
            Self::GridColumnEnd => VendorPrefix::NONE,
            Self::GridRow => VendorPrefix::NONE,
            Self::GridColumn => VendorPrefix::NONE,
            Self::GridArea => VendorPrefix::NONE,
            Self::MarginTop => VendorPrefix::NONE,
            Self::MarginBottom => VendorPrefix::NONE,
            Self::MarginLeft => VendorPrefix::NONE,
            Self::MarginRight => VendorPrefix::NONE,
            Self::MarginBlockStart => VendorPrefix::NONE,
            Self::MarginBlockEnd => VendorPrefix::NONE,
            Self::MarginInlineStart => VendorPrefix::NONE,
            Self::MarginInlineEnd => VendorPrefix::NONE,
            Self::MarginBlock => VendorPrefix::NONE,
            Self::MarginInline => VendorPrefix::NONE,
            Self::Margin => VendorPrefix::NONE,
            Self::PaddingTop => VendorPrefix::NONE,
            Self::PaddingBottom => VendorPrefix::NONE,
            Self::PaddingLeft => VendorPrefix::NONE,
            Self::PaddingRight => VendorPrefix::NONE,
            Self::PaddingBlockStart => VendorPrefix::NONE,
            Self::PaddingBlockEnd => VendorPrefix::NONE,
            Self::PaddingInlineStart => VendorPrefix::NONE,
            Self::PaddingInlineEnd => VendorPrefix::NONE,
            Self::PaddingBlock => VendorPrefix::NONE,
            Self::PaddingInline => VendorPrefix::NONE,
            Self::Padding => VendorPrefix::NONE,
            Self::ScrollMarginTop => VendorPrefix::NONE,
            Self::ScrollMarginBottom => VendorPrefix::NONE,
            Self::ScrollMarginLeft => VendorPrefix::NONE,
            Self::ScrollMarginRight => VendorPrefix::NONE,
            Self::ScrollMarginBlockStart => VendorPrefix::NONE,
            Self::ScrollMarginBlockEnd => VendorPrefix::NONE,
            Self::ScrollMarginInlineStart => VendorPrefix::NONE,
            Self::ScrollMarginInlineEnd => VendorPrefix::NONE,
            Self::ScrollMarginBlock => VendorPrefix::NONE,
            Self::ScrollMarginInline => VendorPrefix::NONE,
            Self::ScrollMargin => VendorPrefix::NONE,
            Self::ScrollPaddingTop => VendorPrefix::NONE,
            Self::ScrollPaddingBottom => VendorPrefix::NONE,
            Self::ScrollPaddingLeft => VendorPrefix::NONE,
            Self::ScrollPaddingRight => VendorPrefix::NONE,
            Self::ScrollPaddingBlockStart => VendorPrefix::NONE,
            Self::ScrollPaddingBlockEnd => VendorPrefix::NONE,
            Self::ScrollPaddingInlineStart => VendorPrefix::NONE,
            Self::ScrollPaddingInlineEnd => VendorPrefix::NONE,
            Self::ScrollPaddingBlock => VendorPrefix::NONE,
            Self::ScrollPaddingInline => VendorPrefix::NONE,
            Self::ScrollPadding => VendorPrefix::NONE,
            Self::FontWeight => VendorPrefix::NONE,
            Self::FontSize => VendorPrefix::NONE,
            Self::FontStretch => VendorPrefix::NONE,
            Self::FontFamily => VendorPrefix::NONE,
            Self::FontStyle => VendorPrefix::NONE,
            Self::FontVariantCaps => VendorPrefix::NONE,
            Self::LineHeight => VendorPrefix::NONE,
            Self::Font => VendorPrefix::NONE,
            Self::VerticalAlign => VendorPrefix::NONE,
            Self::FontPalette => VendorPrefix::NONE,
            Self::TransitionProperty(prefix) => *prefix,
            Self::TransitionDuration(prefix) => *prefix,
            Self::TransitionDelay(prefix) => *prefix,
            Self::TransitionTimingFunction(prefix) => *prefix,
            Self::Transition(prefix) => *prefix,
            Self::AnimationName(prefix) => *prefix,
            Self::AnimationDuration(prefix) => *prefix,
            Self::AnimationTimingFunction(prefix) => *prefix,
            Self::AnimationIterationCount(prefix) => *prefix,
            Self::AnimationDirection(prefix) => *prefix,
            Self::AnimationPlayState(prefix) => *prefix,
            Self::AnimationDelay(prefix) => *prefix,
            Self::AnimationFillMode(prefix) => *prefix,
            Self::AnimationComposition => VendorPrefix::NONE,
            Self::AnimationTimeline => VendorPrefix::NONE,
            Self::AnimationRangeStart => VendorPrefix::NONE,
            Self::AnimationRangeEnd => VendorPrefix::NONE,
            Self::AnimationRange => VendorPrefix::NONE,
            Self::Animation(prefix) => *prefix,
            Self::Transform(prefix) => *prefix,
            Self::TransformOrigin(prefix) => *prefix,
            Self::TransformStyle(prefix) => *prefix,
            Self::TransformBox => VendorPrefix::NONE,
            Self::BackfaceVisibility(prefix) => *prefix,
            Self::Perspective(prefix) => *prefix,
            Self::PerspectiveOrigin(prefix) => *prefix,
            Self::Translate => VendorPrefix::NONE,
            Self::Rotate => VendorPrefix::NONE,
            Self::Scale => VendorPrefix::NONE,
            Self::TextTransform => VendorPrefix::NONE,
            Self::Content => VendorPrefix::NONE,
            Self::WhiteSpace => VendorPrefix::NONE,
            Self::TabSize(prefix) => *prefix,
            Self::WordBreak => VendorPrefix::NONE,
            Self::LineBreak => VendorPrefix::NONE,
            Self::Hyphens(prefix) => *prefix,
            Self::OverflowWrap => VendorPrefix::NONE,
            Self::WordWrap => VendorPrefix::NONE,
            Self::TextAlign => VendorPrefix::NONE,
            Self::TextAlignLast(prefix) => *prefix,
            Self::TextJustify => VendorPrefix::NONE,
            Self::WordSpacing => VendorPrefix::NONE,
            Self::LetterSpacing => VendorPrefix::NONE,
            Self::TextIndent => VendorPrefix::NONE,
            Self::TextDecorationLine(prefix) => *prefix,
            Self::TextDecorationStyle(prefix) => *prefix,
            Self::TextDecorationColor(prefix) => *prefix,
            Self::TextDecorationThickness => VendorPrefix::NONE,
            Self::TextDecoration(prefix) => *prefix,
            Self::TextDecorationSkipInk(prefix) => *prefix,
            Self::TextEmphasisStyle(prefix) => *prefix,
            Self::TextEmphasisColor(prefix) => *prefix,
            Self::TextEmphasis(prefix) => *prefix,
            Self::TextEmphasisPosition(prefix) => *prefix,
            Self::TextShadow => VendorPrefix::NONE,
            Self::TextSizeAdjust(prefix) => *prefix,
            Self::Direction => VendorPrefix::NONE,
            Self::UnicodeBidi => VendorPrefix::NONE,
            Self::BoxDecorationBreak(prefix) => *prefix,
            Self::Resize => VendorPrefix::NONE,
            Self::PointerEvents => VendorPrefix::NONE,
            Self::Float => VendorPrefix::NONE,
            Self::Clear => VendorPrefix::NONE,
            Self::TouchAction => VendorPrefix::NONE,
            Self::ScrollBehavior => VendorPrefix::NONE,
            Self::Cursor => VendorPrefix::NONE,
            Self::CaretColor => VendorPrefix::NONE,
            Self::CaretShape => VendorPrefix::NONE,
            Self::Caret => VendorPrefix::NONE,
            Self::UserSelect(prefix) => *prefix,
            Self::AccentColor => VendorPrefix::NONE,
            Self::Appearance(prefix) => *prefix,
            Self::ListStyleType => VendorPrefix::NONE,
            Self::ListStyleImage => VendorPrefix::NONE,
            Self::ListStylePosition => VendorPrefix::NONE,
            Self::ListStyle => VendorPrefix::NONE,
            Self::MarkerSide => VendorPrefix::NONE,
            Self::Composes => VendorPrefix::NONE,
            Self::Fill => VendorPrefix::NONE,
            Self::FillRule => VendorPrefix::NONE,
            Self::FillOpacity => VendorPrefix::NONE,
            Self::Stroke => VendorPrefix::NONE,
            Self::StrokeOpacity => VendorPrefix::NONE,
            Self::StrokeWidth => VendorPrefix::NONE,
            Self::StrokeLinecap => VendorPrefix::NONE,
            Self::StrokeLinejoin => VendorPrefix::NONE,
            Self::StrokeMiterlimit => VendorPrefix::NONE,
            Self::StrokeDasharray => VendorPrefix::NONE,
            Self::StrokeDashoffset => VendorPrefix::NONE,
            Self::MarkerStart => VendorPrefix::NONE,
            Self::MarkerMid => VendorPrefix::NONE,
            Self::MarkerEnd => VendorPrefix::NONE,
            Self::Marker => VendorPrefix::NONE,
            Self::ColorInterpolation => VendorPrefix::NONE,
            Self::ColorInterpolationFilters => VendorPrefix::NONE,
            Self::ColorRendering => VendorPrefix::NONE,
            Self::ShapeRendering => VendorPrefix::NONE,
            Self::TextRendering => VendorPrefix::NONE,
            Self::ImageRendering => VendorPrefix::NONE,
            Self::ClipPath(prefix) => *prefix,
            Self::ClipRule => VendorPrefix::NONE,
            Self::MaskImage(prefix) => *prefix,
            Self::MaskMode => VendorPrefix::NONE,
            Self::MaskRepeat(prefix) => *prefix,
            Self::MaskPositionX => VendorPrefix::NONE,
            Self::MaskPositionY => VendorPrefix::NONE,
            Self::MaskPosition(prefix) => *prefix,
            Self::MaskClip(prefix) => *prefix,
            Self::MaskOrigin(prefix) => *prefix,
            Self::MaskSize(prefix) => *prefix,
            Self::MaskComposite => VendorPrefix::NONE,
            Self::MaskType => VendorPrefix::NONE,
            Self::Mask(prefix) => *prefix,
            Self::MaskBorderSource => VendorPrefix::NONE,
            Self::MaskBorderMode => VendorPrefix::NONE,
            Self::MaskBorderSlice => VendorPrefix::NONE,
            Self::MaskBorderWidth => VendorPrefix::NONE,
            Self::MaskBorderOutset => VendorPrefix::NONE,
            Self::MaskBorderRepeat => VendorPrefix::NONE,
            Self::MaskBorder => VendorPrefix::NONE,
            Self::WebKitMaskComposite => VendorPrefix::NONE,
            Self::WebKitMaskSourceType(prefix) => *prefix,
            Self::WebKitMaskBoxImage(prefix) => *prefix,
            Self::WebKitMaskBoxImageSource(prefix) => *prefix,
            Self::WebKitMaskBoxImageSlice(prefix) => *prefix,
            Self::WebKitMaskBoxImageWidth(prefix) => *prefix,
            Self::WebKitMaskBoxImageOutset(prefix) => *prefix,
            Self::WebKitMaskBoxImageRepeat(prefix) => *prefix,
            Self::Filter(prefix) => *prefix,
            Self::BackdropFilter(prefix) => *prefix,
            Self::MixBlendMode => VendorPrefix::NONE,
            Self::ZIndex => VendorPrefix::NONE,
            Self::ContainerType => VendorPrefix::NONE,
            Self::ContainerName => VendorPrefix::NONE,
            Self::Container => VendorPrefix::NONE,
            Self::ViewTransitionName => VendorPrefix::NONE,
            Self::ViewTransitionClass => VendorPrefix::NONE,
            Self::ViewTransitionGroup => VendorPrefix::NONE,
            Self::ColorScheme => VendorPrefix::NONE,
            Self::PrintColorAdjust(prefix) => *prefix,
            Self::Unparsed | Self::Custom(_) => VendorPrefix::NONE,
        }
    }
    /// Returns the compact discriminant of a known property.
    #[inline]
    pub fn known_id(&self) -> Option<u32> {
        self.known_id_and_prefix().map(|(id, _)| id)
    }
    /// Returns the compact discriminant and vendor prefix of a known property.
    #[inline]
    pub fn known_id_and_prefix(&self) -> Option<(u32, VendorPrefix)> {
        match self {
            Self::All => Some((KnownPropertyDiscriminant::All as u32, VendorPrefix::NONE)),
            Self::BackgroundColor => Some((
                KnownPropertyDiscriminant::BackgroundColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BackgroundImage => Some((
                KnownPropertyDiscriminant::BackgroundImage as u32,
                VendorPrefix::NONE,
            )),
            Self::BackgroundPositionX => Some((
                KnownPropertyDiscriminant::BackgroundPositionX as u32,
                VendorPrefix::NONE,
            )),
            Self::BackgroundPositionY => Some((
                KnownPropertyDiscriminant::BackgroundPositionY as u32,
                VendorPrefix::NONE,
            )),
            Self::BackgroundPosition => Some((
                KnownPropertyDiscriminant::BackgroundPosition as u32,
                VendorPrefix::NONE,
            )),
            Self::BackgroundSize => Some((
                KnownPropertyDiscriminant::BackgroundSize as u32,
                VendorPrefix::NONE,
            )),
            Self::BackgroundRepeat => Some((
                KnownPropertyDiscriminant::BackgroundRepeat as u32,
                VendorPrefix::NONE,
            )),
            Self::BackgroundAttachment => Some((
                KnownPropertyDiscriminant::BackgroundAttachment as u32,
                VendorPrefix::NONE,
            )),
            Self::BackgroundClip(prefix) => {
                Some((KnownPropertyDiscriminant::BackgroundClip as u32, *prefix))
            }
            Self::BackgroundOrigin => Some((
                KnownPropertyDiscriminant::BackgroundOrigin as u32,
                VendorPrefix::NONE,
            )),
            Self::Background => Some((
                KnownPropertyDiscriminant::Background as u32,
                VendorPrefix::NONE,
            )),
            Self::BoxShadow(prefix) => Some((KnownPropertyDiscriminant::BoxShadow as u32, *prefix)),
            Self::Opacity => Some((
                KnownPropertyDiscriminant::Opacity as u32,
                VendorPrefix::NONE,
            )),
            Self::Color => Some((KnownPropertyDiscriminant::Color as u32, VendorPrefix::NONE)),
            Self::Display => Some((
                KnownPropertyDiscriminant::Display as u32,
                VendorPrefix::NONE,
            )),
            Self::Visibility => Some((
                KnownPropertyDiscriminant::Visibility as u32,
                VendorPrefix::NONE,
            )),
            Self::Width => Some((KnownPropertyDiscriminant::Width as u32, VendorPrefix::NONE)),
            Self::Height => Some((KnownPropertyDiscriminant::Height as u32, VendorPrefix::NONE)),
            Self::MinWidth => Some((
                KnownPropertyDiscriminant::MinWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::MinHeight => Some((
                KnownPropertyDiscriminant::MinHeight as u32,
                VendorPrefix::NONE,
            )),
            Self::MaxWidth => Some((
                KnownPropertyDiscriminant::MaxWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::MaxHeight => Some((
                KnownPropertyDiscriminant::MaxHeight as u32,
                VendorPrefix::NONE,
            )),
            Self::BlockSize => Some((
                KnownPropertyDiscriminant::BlockSize as u32,
                VendorPrefix::NONE,
            )),
            Self::InlineSize => Some((
                KnownPropertyDiscriminant::InlineSize as u32,
                VendorPrefix::NONE,
            )),
            Self::MinBlockSize => Some((
                KnownPropertyDiscriminant::MinBlockSize as u32,
                VendorPrefix::NONE,
            )),
            Self::MinInlineSize => Some((
                KnownPropertyDiscriminant::MinInlineSize as u32,
                VendorPrefix::NONE,
            )),
            Self::MaxBlockSize => Some((
                KnownPropertyDiscriminant::MaxBlockSize as u32,
                VendorPrefix::NONE,
            )),
            Self::MaxInlineSize => Some((
                KnownPropertyDiscriminant::MaxInlineSize as u32,
                VendorPrefix::NONE,
            )),
            Self::BoxSizing(prefix) => Some((KnownPropertyDiscriminant::BoxSizing as u32, *prefix)),
            Self::AspectRatio => Some((
                KnownPropertyDiscriminant::AspectRatio as u32,
                VendorPrefix::NONE,
            )),
            Self::Overflow => Some((
                KnownPropertyDiscriminant::Overflow as u32,
                VendorPrefix::NONE,
            )),
            Self::OverflowX => Some((
                KnownPropertyDiscriminant::OverflowX as u32,
                VendorPrefix::NONE,
            )),
            Self::OverflowY => Some((
                KnownPropertyDiscriminant::OverflowY as u32,
                VendorPrefix::NONE,
            )),
            Self::TextOverflow(prefix) => {
                Some((KnownPropertyDiscriminant::TextOverflow as u32, *prefix))
            }
            Self::ObjectFit => Some((
                KnownPropertyDiscriminant::ObjectFit as u32,
                VendorPrefix::NONE,
            )),
            Self::ObjectPosition => Some((
                KnownPropertyDiscriminant::ObjectPosition as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollbarColor => Some((
                KnownPropertyDiscriminant::ScrollbarColor as u32,
                VendorPrefix::NONE,
            )),
            Self::Position => Some((
                KnownPropertyDiscriminant::Position as u32,
                VendorPrefix::NONE,
            )),
            Self::Top => Some((KnownPropertyDiscriminant::Top as u32, VendorPrefix::NONE)),
            Self::Bottom => Some((KnownPropertyDiscriminant::Bottom as u32, VendorPrefix::NONE)),
            Self::Left => Some((KnownPropertyDiscriminant::Left as u32, VendorPrefix::NONE)),
            Self::Right => Some((KnownPropertyDiscriminant::Right as u32, VendorPrefix::NONE)),
            Self::InsetBlockStart => Some((
                KnownPropertyDiscriminant::InsetBlockStart as u32,
                VendorPrefix::NONE,
            )),
            Self::InsetBlockEnd => Some((
                KnownPropertyDiscriminant::InsetBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::InsetInlineStart => Some((
                KnownPropertyDiscriminant::InsetInlineStart as u32,
                VendorPrefix::NONE,
            )),
            Self::InsetInlineEnd => Some((
                KnownPropertyDiscriminant::InsetInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::InsetBlock => Some((
                KnownPropertyDiscriminant::InsetBlock as u32,
                VendorPrefix::NONE,
            )),
            Self::InsetInline => Some((
                KnownPropertyDiscriminant::InsetInline as u32,
                VendorPrefix::NONE,
            )),
            Self::Inset => Some((KnownPropertyDiscriminant::Inset as u32, VendorPrefix::NONE)),
            Self::BorderSpacing => Some((
                KnownPropertyDiscriminant::BorderSpacing as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderTopColor => Some((
                KnownPropertyDiscriminant::BorderTopColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBottomColor => Some((
                KnownPropertyDiscriminant::BorderBottomColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderLeftColor => Some((
                KnownPropertyDiscriminant::BorderLeftColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderRightColor => Some((
                KnownPropertyDiscriminant::BorderRightColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockStartColor => Some((
                KnownPropertyDiscriminant::BorderBlockStartColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockEndColor => Some((
                KnownPropertyDiscriminant::BorderBlockEndColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineStartColor => Some((
                KnownPropertyDiscriminant::BorderInlineStartColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineEndColor => Some((
                KnownPropertyDiscriminant::BorderInlineEndColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderTopStyle => Some((
                KnownPropertyDiscriminant::BorderTopStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBottomStyle => Some((
                KnownPropertyDiscriminant::BorderBottomStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderLeftStyle => Some((
                KnownPropertyDiscriminant::BorderLeftStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderRightStyle => Some((
                KnownPropertyDiscriminant::BorderRightStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockStartStyle => Some((
                KnownPropertyDiscriminant::BorderBlockStartStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockEndStyle => Some((
                KnownPropertyDiscriminant::BorderBlockEndStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineStartStyle => Some((
                KnownPropertyDiscriminant::BorderInlineStartStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineEndStyle => Some((
                KnownPropertyDiscriminant::BorderInlineEndStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderTopWidth => Some((
                KnownPropertyDiscriminant::BorderTopWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBottomWidth => Some((
                KnownPropertyDiscriminant::BorderBottomWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderLeftWidth => Some((
                KnownPropertyDiscriminant::BorderLeftWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderRightWidth => Some((
                KnownPropertyDiscriminant::BorderRightWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockStartWidth => Some((
                KnownPropertyDiscriminant::BorderBlockStartWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockEndWidth => Some((
                KnownPropertyDiscriminant::BorderBlockEndWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineStartWidth => Some((
                KnownPropertyDiscriminant::BorderInlineStartWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineEndWidth => Some((
                KnownPropertyDiscriminant::BorderInlineEndWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderTopLeftRadius(prefix) => Some((
                KnownPropertyDiscriminant::BorderTopLeftRadius as u32,
                *prefix,
            )),
            Self::BorderTopRightRadius(prefix) => Some((
                KnownPropertyDiscriminant::BorderTopRightRadius as u32,
                *prefix,
            )),
            Self::BorderBottomLeftRadius(prefix) => Some((
                KnownPropertyDiscriminant::BorderBottomLeftRadius as u32,
                *prefix,
            )),
            Self::BorderBottomRightRadius(prefix) => Some((
                KnownPropertyDiscriminant::BorderBottomRightRadius as u32,
                *prefix,
            )),
            Self::BorderStartStartRadius => Some((
                KnownPropertyDiscriminant::BorderStartStartRadius as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderStartEndRadius => Some((
                KnownPropertyDiscriminant::BorderStartEndRadius as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderEndStartRadius => Some((
                KnownPropertyDiscriminant::BorderEndStartRadius as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderEndEndRadius => Some((
                KnownPropertyDiscriminant::BorderEndEndRadius as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderRadius(prefix) => {
                Some((KnownPropertyDiscriminant::BorderRadius as u32, *prefix))
            }
            Self::BorderImageSource => Some((
                KnownPropertyDiscriminant::BorderImageSource as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderImageOutset => Some((
                KnownPropertyDiscriminant::BorderImageOutset as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderImageRepeat => Some((
                KnownPropertyDiscriminant::BorderImageRepeat as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderImageWidth => Some((
                KnownPropertyDiscriminant::BorderImageWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderImageSlice => Some((
                KnownPropertyDiscriminant::BorderImageSlice as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderImage(prefix) => {
                Some((KnownPropertyDiscriminant::BorderImage as u32, *prefix))
            }
            Self::BorderColor => Some((
                KnownPropertyDiscriminant::BorderColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderStyle => Some((
                KnownPropertyDiscriminant::BorderStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderWidth => Some((
                KnownPropertyDiscriminant::BorderWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockColor => Some((
                KnownPropertyDiscriminant::BorderBlockColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockStyle => Some((
                KnownPropertyDiscriminant::BorderBlockStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockWidth => Some((
                KnownPropertyDiscriminant::BorderBlockWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineColor => Some((
                KnownPropertyDiscriminant::BorderInlineColor as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineStyle => Some((
                KnownPropertyDiscriminant::BorderInlineStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineWidth => Some((
                KnownPropertyDiscriminant::BorderInlineWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::Border => Some((KnownPropertyDiscriminant::Border as u32, VendorPrefix::NONE)),
            Self::BorderTop => Some((
                KnownPropertyDiscriminant::BorderTop as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBottom => Some((
                KnownPropertyDiscriminant::BorderBottom as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderLeft => Some((
                KnownPropertyDiscriminant::BorderLeft as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderRight => Some((
                KnownPropertyDiscriminant::BorderRight as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlock => Some((
                KnownPropertyDiscriminant::BorderBlock as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockStart => Some((
                KnownPropertyDiscriminant::BorderBlockStart as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderBlockEnd => Some((
                KnownPropertyDiscriminant::BorderBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInline => Some((
                KnownPropertyDiscriminant::BorderInline as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineStart => Some((
                KnownPropertyDiscriminant::BorderInlineStart as u32,
                VendorPrefix::NONE,
            )),
            Self::BorderInlineEnd => Some((
                KnownPropertyDiscriminant::BorderInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::Outline => Some((
                KnownPropertyDiscriminant::Outline as u32,
                VendorPrefix::NONE,
            )),
            Self::OutlineColor => Some((
                KnownPropertyDiscriminant::OutlineColor as u32,
                VendorPrefix::NONE,
            )),
            Self::OutlineStyle => Some((
                KnownPropertyDiscriminant::OutlineStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::OutlineWidth => Some((
                KnownPropertyDiscriminant::OutlineWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::FlexDirection(prefix) => {
                Some((KnownPropertyDiscriminant::FlexDirection as u32, *prefix))
            }
            Self::FlexWrap(prefix) => Some((KnownPropertyDiscriminant::FlexWrap as u32, *prefix)),
            Self::FlexFlow(prefix) => Some((KnownPropertyDiscriminant::FlexFlow as u32, *prefix)),
            Self::FlexGrow(prefix) => Some((KnownPropertyDiscriminant::FlexGrow as u32, *prefix)),
            Self::FlexShrink(prefix) => {
                Some((KnownPropertyDiscriminant::FlexShrink as u32, *prefix))
            }
            Self::FlexBasis(prefix) => Some((KnownPropertyDiscriminant::FlexBasis as u32, *prefix)),
            Self::Flex(prefix) => Some((KnownPropertyDiscriminant::Flex as u32, *prefix)),
            Self::Order(prefix) => Some((KnownPropertyDiscriminant::Order as u32, *prefix)),
            Self::AlignContent(prefix) => {
                Some((KnownPropertyDiscriminant::AlignContent as u32, *prefix))
            }
            Self::JustifyContent(prefix) => {
                Some((KnownPropertyDiscriminant::JustifyContent as u32, *prefix))
            }
            Self::PlaceContent => Some((
                KnownPropertyDiscriminant::PlaceContent as u32,
                VendorPrefix::NONE,
            )),
            Self::AlignSelf(prefix) => Some((KnownPropertyDiscriminant::AlignSelf as u32, *prefix)),
            Self::JustifySelf => Some((
                KnownPropertyDiscriminant::JustifySelf as u32,
                VendorPrefix::NONE,
            )),
            Self::PlaceSelf => Some((
                KnownPropertyDiscriminant::PlaceSelf as u32,
                VendorPrefix::NONE,
            )),
            Self::AlignItems(prefix) => {
                Some((KnownPropertyDiscriminant::AlignItems as u32, *prefix))
            }
            Self::JustifyItems => Some((
                KnownPropertyDiscriminant::JustifyItems as u32,
                VendorPrefix::NONE,
            )),
            Self::PlaceItems => Some((
                KnownPropertyDiscriminant::PlaceItems as u32,
                VendorPrefix::NONE,
            )),
            Self::RowGap => Some((KnownPropertyDiscriminant::RowGap as u32, VendorPrefix::NONE)),
            Self::ColumnGap => Some((
                KnownPropertyDiscriminant::ColumnGap as u32,
                VendorPrefix::NONE,
            )),
            Self::Gap => Some((KnownPropertyDiscriminant::Gap as u32, VendorPrefix::NONE)),
            Self::ColumnRule(prefix) => {
                Some((KnownPropertyDiscriminant::ColumnRule as u32, *prefix))
            }
            Self::ColumnWidth(prefix) => {
                Some((KnownPropertyDiscriminant::ColumnWidth as u32, *prefix))
            }
            Self::ColumnCount(prefix) => {
                Some((KnownPropertyDiscriminant::ColumnCount as u32, *prefix))
            }
            Self::Columns(prefix) => Some((KnownPropertyDiscriminant::Columns as u32, *prefix)),
            Self::GridColumnGap => Some((
                KnownPropertyDiscriminant::GridColumnGap as u32,
                VendorPrefix::NONE,
            )),
            Self::GridRowGap => Some((
                KnownPropertyDiscriminant::GridRowGap as u32,
                VendorPrefix::NONE,
            )),
            Self::BoxOrient(prefix) => Some((KnownPropertyDiscriminant::BoxOrient as u32, *prefix)),
            Self::BoxDirection(prefix) => {
                Some((KnownPropertyDiscriminant::BoxDirection as u32, *prefix))
            }
            Self::BoxOrdinalGroup(prefix) => {
                Some((KnownPropertyDiscriminant::BoxOrdinalGroup as u32, *prefix))
            }
            Self::BoxAlign(prefix) => Some((KnownPropertyDiscriminant::BoxAlign as u32, *prefix)),
            Self::BoxFlex(prefix) => Some((KnownPropertyDiscriminant::BoxFlex as u32, *prefix)),
            Self::BoxFlexGroup(prefix) => {
                Some((KnownPropertyDiscriminant::BoxFlexGroup as u32, *prefix))
            }
            Self::BoxPack(prefix) => Some((KnownPropertyDiscriminant::BoxPack as u32, *prefix)),
            Self::BoxLines(prefix) => Some((KnownPropertyDiscriminant::BoxLines as u32, *prefix)),
            Self::FlexPack(prefix) => Some((KnownPropertyDiscriminant::FlexPack as u32, *prefix)),
            Self::FlexOrder(prefix) => Some((KnownPropertyDiscriminant::FlexOrder as u32, *prefix)),
            Self::FlexAlign(prefix) => Some((KnownPropertyDiscriminant::FlexAlign as u32, *prefix)),
            Self::FlexItemAlign(prefix) => {
                Some((KnownPropertyDiscriminant::FlexItemAlign as u32, *prefix))
            }
            Self::FlexLinePack(prefix) => {
                Some((KnownPropertyDiscriminant::FlexLinePack as u32, *prefix))
            }
            Self::FlexPositive(prefix) => {
                Some((KnownPropertyDiscriminant::FlexPositive as u32, *prefix))
            }
            Self::FlexNegative(prefix) => {
                Some((KnownPropertyDiscriminant::FlexNegative as u32, *prefix))
            }
            Self::FlexPreferredSize(prefix) => {
                Some((KnownPropertyDiscriminant::FlexPreferredSize as u32, *prefix))
            }
            Self::GridTemplateColumns => Some((
                KnownPropertyDiscriminant::GridTemplateColumns as u32,
                VendorPrefix::NONE,
            )),
            Self::GridTemplateRows => Some((
                KnownPropertyDiscriminant::GridTemplateRows as u32,
                VendorPrefix::NONE,
            )),
            Self::GridAutoColumns => Some((
                KnownPropertyDiscriminant::GridAutoColumns as u32,
                VendorPrefix::NONE,
            )),
            Self::GridAutoRows => Some((
                KnownPropertyDiscriminant::GridAutoRows as u32,
                VendorPrefix::NONE,
            )),
            Self::GridAutoFlow => Some((
                KnownPropertyDiscriminant::GridAutoFlow as u32,
                VendorPrefix::NONE,
            )),
            Self::GridTemplateAreas => Some((
                KnownPropertyDiscriminant::GridTemplateAreas as u32,
                VendorPrefix::NONE,
            )),
            Self::GridTemplate => Some((
                KnownPropertyDiscriminant::GridTemplate as u32,
                VendorPrefix::NONE,
            )),
            Self::Grid => Some((KnownPropertyDiscriminant::Grid as u32, VendorPrefix::NONE)),
            Self::GridRowStart => Some((
                KnownPropertyDiscriminant::GridRowStart as u32,
                VendorPrefix::NONE,
            )),
            Self::GridRowEnd => Some((
                KnownPropertyDiscriminant::GridRowEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::GridColumnStart => Some((
                KnownPropertyDiscriminant::GridColumnStart as u32,
                VendorPrefix::NONE,
            )),
            Self::GridColumnEnd => Some((
                KnownPropertyDiscriminant::GridColumnEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::GridRow => Some((
                KnownPropertyDiscriminant::GridRow as u32,
                VendorPrefix::NONE,
            )),
            Self::GridColumn => Some((
                KnownPropertyDiscriminant::GridColumn as u32,
                VendorPrefix::NONE,
            )),
            Self::GridArea => Some((
                KnownPropertyDiscriminant::GridArea as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginTop => Some((
                KnownPropertyDiscriminant::MarginTop as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginBottom => Some((
                KnownPropertyDiscriminant::MarginBottom as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginLeft => Some((
                KnownPropertyDiscriminant::MarginLeft as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginRight => Some((
                KnownPropertyDiscriminant::MarginRight as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginBlockStart => Some((
                KnownPropertyDiscriminant::MarginBlockStart as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginBlockEnd => Some((
                KnownPropertyDiscriminant::MarginBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginInlineStart => Some((
                KnownPropertyDiscriminant::MarginInlineStart as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginInlineEnd => Some((
                KnownPropertyDiscriminant::MarginInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginBlock => Some((
                KnownPropertyDiscriminant::MarginBlock as u32,
                VendorPrefix::NONE,
            )),
            Self::MarginInline => Some((
                KnownPropertyDiscriminant::MarginInline as u32,
                VendorPrefix::NONE,
            )),
            Self::Margin => Some((KnownPropertyDiscriminant::Margin as u32, VendorPrefix::NONE)),
            Self::PaddingTop => Some((
                KnownPropertyDiscriminant::PaddingTop as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingBottom => Some((
                KnownPropertyDiscriminant::PaddingBottom as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingLeft => Some((
                KnownPropertyDiscriminant::PaddingLeft as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingRight => Some((
                KnownPropertyDiscriminant::PaddingRight as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingBlockStart => Some((
                KnownPropertyDiscriminant::PaddingBlockStart as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingBlockEnd => Some((
                KnownPropertyDiscriminant::PaddingBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingInlineStart => Some((
                KnownPropertyDiscriminant::PaddingInlineStart as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingInlineEnd => Some((
                KnownPropertyDiscriminant::PaddingInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingBlock => Some((
                KnownPropertyDiscriminant::PaddingBlock as u32,
                VendorPrefix::NONE,
            )),
            Self::PaddingInline => Some((
                KnownPropertyDiscriminant::PaddingInline as u32,
                VendorPrefix::NONE,
            )),
            Self::Padding => Some((
                KnownPropertyDiscriminant::Padding as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginTop => Some((
                KnownPropertyDiscriminant::ScrollMarginTop as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginBottom => Some((
                KnownPropertyDiscriminant::ScrollMarginBottom as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginLeft => Some((
                KnownPropertyDiscriminant::ScrollMarginLeft as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginRight => Some((
                KnownPropertyDiscriminant::ScrollMarginRight as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginBlockStart => Some((
                KnownPropertyDiscriminant::ScrollMarginBlockStart as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginBlockEnd => Some((
                KnownPropertyDiscriminant::ScrollMarginBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginInlineStart => Some((
                KnownPropertyDiscriminant::ScrollMarginInlineStart as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginInlineEnd => Some((
                KnownPropertyDiscriminant::ScrollMarginInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginBlock => Some((
                KnownPropertyDiscriminant::ScrollMarginBlock as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMarginInline => Some((
                KnownPropertyDiscriminant::ScrollMarginInline as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollMargin => Some((
                KnownPropertyDiscriminant::ScrollMargin as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingTop => Some((
                KnownPropertyDiscriminant::ScrollPaddingTop as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingBottom => Some((
                KnownPropertyDiscriminant::ScrollPaddingBottom as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingLeft => Some((
                KnownPropertyDiscriminant::ScrollPaddingLeft as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingRight => Some((
                KnownPropertyDiscriminant::ScrollPaddingRight as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingBlockStart => Some((
                KnownPropertyDiscriminant::ScrollPaddingBlockStart as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingBlockEnd => Some((
                KnownPropertyDiscriminant::ScrollPaddingBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingInlineStart => Some((
                KnownPropertyDiscriminant::ScrollPaddingInlineStart as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingInlineEnd => Some((
                KnownPropertyDiscriminant::ScrollPaddingInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingBlock => Some((
                KnownPropertyDiscriminant::ScrollPaddingBlock as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPaddingInline => Some((
                KnownPropertyDiscriminant::ScrollPaddingInline as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollPadding => Some((
                KnownPropertyDiscriminant::ScrollPadding as u32,
                VendorPrefix::NONE,
            )),
            Self::FontWeight => Some((
                KnownPropertyDiscriminant::FontWeight as u32,
                VendorPrefix::NONE,
            )),
            Self::FontSize => Some((
                KnownPropertyDiscriminant::FontSize as u32,
                VendorPrefix::NONE,
            )),
            Self::FontStretch => Some((
                KnownPropertyDiscriminant::FontStretch as u32,
                VendorPrefix::NONE,
            )),
            Self::FontFamily => Some((
                KnownPropertyDiscriminant::FontFamily as u32,
                VendorPrefix::NONE,
            )),
            Self::FontStyle => Some((
                KnownPropertyDiscriminant::FontStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::FontVariantCaps => Some((
                KnownPropertyDiscriminant::FontVariantCaps as u32,
                VendorPrefix::NONE,
            )),
            Self::LineHeight => Some((
                KnownPropertyDiscriminant::LineHeight as u32,
                VendorPrefix::NONE,
            )),
            Self::Font => Some((KnownPropertyDiscriminant::Font as u32, VendorPrefix::NONE)),
            Self::VerticalAlign => Some((
                KnownPropertyDiscriminant::VerticalAlign as u32,
                VendorPrefix::NONE,
            )),
            Self::FontPalette => Some((
                KnownPropertyDiscriminant::FontPalette as u32,
                VendorPrefix::NONE,
            )),
            Self::TransitionProperty(prefix) => Some((
                KnownPropertyDiscriminant::TransitionProperty as u32,
                *prefix,
            )),
            Self::TransitionDuration(prefix) => Some((
                KnownPropertyDiscriminant::TransitionDuration as u32,
                *prefix,
            )),
            Self::TransitionDelay(prefix) => {
                Some((KnownPropertyDiscriminant::TransitionDelay as u32, *prefix))
            }
            Self::TransitionTimingFunction(prefix) => Some((
                KnownPropertyDiscriminant::TransitionTimingFunction as u32,
                *prefix,
            )),
            Self::Transition(prefix) => {
                Some((KnownPropertyDiscriminant::Transition as u32, *prefix))
            }
            Self::AnimationName(prefix) => {
                Some((KnownPropertyDiscriminant::AnimationName as u32, *prefix))
            }
            Self::AnimationDuration(prefix) => {
                Some((KnownPropertyDiscriminant::AnimationDuration as u32, *prefix))
            }
            Self::AnimationTimingFunction(prefix) => Some((
                KnownPropertyDiscriminant::AnimationTimingFunction as u32,
                *prefix,
            )),
            Self::AnimationIterationCount(prefix) => Some((
                KnownPropertyDiscriminant::AnimationIterationCount as u32,
                *prefix,
            )),
            Self::AnimationDirection(prefix) => Some((
                KnownPropertyDiscriminant::AnimationDirection as u32,
                *prefix,
            )),
            Self::AnimationPlayState(prefix) => Some((
                KnownPropertyDiscriminant::AnimationPlayState as u32,
                *prefix,
            )),
            Self::AnimationDelay(prefix) => {
                Some((KnownPropertyDiscriminant::AnimationDelay as u32, *prefix))
            }
            Self::AnimationFillMode(prefix) => {
                Some((KnownPropertyDiscriminant::AnimationFillMode as u32, *prefix))
            }
            Self::AnimationComposition => Some((
                KnownPropertyDiscriminant::AnimationComposition as u32,
                VendorPrefix::NONE,
            )),
            Self::AnimationTimeline => Some((
                KnownPropertyDiscriminant::AnimationTimeline as u32,
                VendorPrefix::NONE,
            )),
            Self::AnimationRangeStart => Some((
                KnownPropertyDiscriminant::AnimationRangeStart as u32,
                VendorPrefix::NONE,
            )),
            Self::AnimationRangeEnd => Some((
                KnownPropertyDiscriminant::AnimationRangeEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::AnimationRange => Some((
                KnownPropertyDiscriminant::AnimationRange as u32,
                VendorPrefix::NONE,
            )),
            Self::Animation(prefix) => Some((KnownPropertyDiscriminant::Animation as u32, *prefix)),
            Self::Transform(prefix) => Some((KnownPropertyDiscriminant::Transform as u32, *prefix)),
            Self::TransformOrigin(prefix) => {
                Some((KnownPropertyDiscriminant::TransformOrigin as u32, *prefix))
            }
            Self::TransformStyle(prefix) => {
                Some((KnownPropertyDiscriminant::TransformStyle as u32, *prefix))
            }
            Self::TransformBox => Some((
                KnownPropertyDiscriminant::TransformBox as u32,
                VendorPrefix::NONE,
            )),
            Self::BackfaceVisibility(prefix) => Some((
                KnownPropertyDiscriminant::BackfaceVisibility as u32,
                *prefix,
            )),
            Self::Perspective(prefix) => {
                Some((KnownPropertyDiscriminant::Perspective as u32, *prefix))
            }
            Self::PerspectiveOrigin(prefix) => {
                Some((KnownPropertyDiscriminant::PerspectiveOrigin as u32, *prefix))
            }
            Self::Translate => Some((
                KnownPropertyDiscriminant::Translate as u32,
                VendorPrefix::NONE,
            )),
            Self::Rotate => Some((KnownPropertyDiscriminant::Rotate as u32, VendorPrefix::NONE)),
            Self::Scale => Some((KnownPropertyDiscriminant::Scale as u32, VendorPrefix::NONE)),
            Self::TextTransform => Some((
                KnownPropertyDiscriminant::TextTransform as u32,
                VendorPrefix::NONE,
            )),
            Self::Content => Some((
                KnownPropertyDiscriminant::Content as u32,
                VendorPrefix::NONE,
            )),
            Self::WhiteSpace => Some((
                KnownPropertyDiscriminant::WhiteSpace as u32,
                VendorPrefix::NONE,
            )),
            Self::TabSize(prefix) => Some((KnownPropertyDiscriminant::TabSize as u32, *prefix)),
            Self::WordBreak => Some((
                KnownPropertyDiscriminant::WordBreak as u32,
                VendorPrefix::NONE,
            )),
            Self::LineBreak => Some((
                KnownPropertyDiscriminant::LineBreak as u32,
                VendorPrefix::NONE,
            )),
            Self::Hyphens(prefix) => Some((KnownPropertyDiscriminant::Hyphens as u32, *prefix)),
            Self::OverflowWrap => Some((
                KnownPropertyDiscriminant::OverflowWrap as u32,
                VendorPrefix::NONE,
            )),
            Self::WordWrap => Some((
                KnownPropertyDiscriminant::WordWrap as u32,
                VendorPrefix::NONE,
            )),
            Self::TextAlign => Some((
                KnownPropertyDiscriminant::TextAlign as u32,
                VendorPrefix::NONE,
            )),
            Self::TextAlignLast(prefix) => {
                Some((KnownPropertyDiscriminant::TextAlignLast as u32, *prefix))
            }
            Self::TextJustify => Some((
                KnownPropertyDiscriminant::TextJustify as u32,
                VendorPrefix::NONE,
            )),
            Self::WordSpacing => Some((
                KnownPropertyDiscriminant::WordSpacing as u32,
                VendorPrefix::NONE,
            )),
            Self::LetterSpacing => Some((
                KnownPropertyDiscriminant::LetterSpacing as u32,
                VendorPrefix::NONE,
            )),
            Self::TextIndent => Some((
                KnownPropertyDiscriminant::TextIndent as u32,
                VendorPrefix::NONE,
            )),
            Self::TextDecorationLine(prefix) => Some((
                KnownPropertyDiscriminant::TextDecorationLine as u32,
                *prefix,
            )),
            Self::TextDecorationStyle(prefix) => Some((
                KnownPropertyDiscriminant::TextDecorationStyle as u32,
                *prefix,
            )),
            Self::TextDecorationColor(prefix) => Some((
                KnownPropertyDiscriminant::TextDecorationColor as u32,
                *prefix,
            )),
            Self::TextDecorationThickness => Some((
                KnownPropertyDiscriminant::TextDecorationThickness as u32,
                VendorPrefix::NONE,
            )),
            Self::TextDecoration(prefix) => {
                Some((KnownPropertyDiscriminant::TextDecoration as u32, *prefix))
            }
            Self::TextDecorationSkipInk(prefix) => Some((
                KnownPropertyDiscriminant::TextDecorationSkipInk as u32,
                *prefix,
            )),
            Self::TextEmphasisStyle(prefix) => {
                Some((KnownPropertyDiscriminant::TextEmphasisStyle as u32, *prefix))
            }
            Self::TextEmphasisColor(prefix) => {
                Some((KnownPropertyDiscriminant::TextEmphasisColor as u32, *prefix))
            }
            Self::TextEmphasis(prefix) => {
                Some((KnownPropertyDiscriminant::TextEmphasis as u32, *prefix))
            }
            Self::TextEmphasisPosition(prefix) => Some((
                KnownPropertyDiscriminant::TextEmphasisPosition as u32,
                *prefix,
            )),
            Self::TextShadow => Some((
                KnownPropertyDiscriminant::TextShadow as u32,
                VendorPrefix::NONE,
            )),
            Self::TextSizeAdjust(prefix) => {
                Some((KnownPropertyDiscriminant::TextSizeAdjust as u32, *prefix))
            }
            Self::Direction => Some((
                KnownPropertyDiscriminant::Direction as u32,
                VendorPrefix::NONE,
            )),
            Self::UnicodeBidi => Some((
                KnownPropertyDiscriminant::UnicodeBidi as u32,
                VendorPrefix::NONE,
            )),
            Self::BoxDecorationBreak(prefix) => Some((
                KnownPropertyDiscriminant::BoxDecorationBreak as u32,
                *prefix,
            )),
            Self::Resize => Some((KnownPropertyDiscriminant::Resize as u32, VendorPrefix::NONE)),
            Self::PointerEvents => Some((
                KnownPropertyDiscriminant::PointerEvents as u32,
                VendorPrefix::NONE,
            )),
            Self::Float => Some((KnownPropertyDiscriminant::Float as u32, VendorPrefix::NONE)),
            Self::Clear => Some((KnownPropertyDiscriminant::Clear as u32, VendorPrefix::NONE)),
            Self::TouchAction => Some((
                KnownPropertyDiscriminant::TouchAction as u32,
                VendorPrefix::NONE,
            )),
            Self::ScrollBehavior => Some((
                KnownPropertyDiscriminant::ScrollBehavior as u32,
                VendorPrefix::NONE,
            )),
            Self::Cursor => Some((KnownPropertyDiscriminant::Cursor as u32, VendorPrefix::NONE)),
            Self::CaretColor => Some((
                KnownPropertyDiscriminant::CaretColor as u32,
                VendorPrefix::NONE,
            )),
            Self::CaretShape => Some((
                KnownPropertyDiscriminant::CaretShape as u32,
                VendorPrefix::NONE,
            )),
            Self::Caret => Some((KnownPropertyDiscriminant::Caret as u32, VendorPrefix::NONE)),
            Self::UserSelect(prefix) => {
                Some((KnownPropertyDiscriminant::UserSelect as u32, *prefix))
            }
            Self::AccentColor => Some((
                KnownPropertyDiscriminant::AccentColor as u32,
                VendorPrefix::NONE,
            )),
            Self::Appearance(prefix) => {
                Some((KnownPropertyDiscriminant::Appearance as u32, *prefix))
            }
            Self::ListStyleType => Some((
                KnownPropertyDiscriminant::ListStyleType as u32,
                VendorPrefix::NONE,
            )),
            Self::ListStyleImage => Some((
                KnownPropertyDiscriminant::ListStyleImage as u32,
                VendorPrefix::NONE,
            )),
            Self::ListStylePosition => Some((
                KnownPropertyDiscriminant::ListStylePosition as u32,
                VendorPrefix::NONE,
            )),
            Self::ListStyle => Some((
                KnownPropertyDiscriminant::ListStyle as u32,
                VendorPrefix::NONE,
            )),
            Self::MarkerSide => Some((
                KnownPropertyDiscriminant::MarkerSide as u32,
                VendorPrefix::NONE,
            )),
            Self::Composes => Some((
                KnownPropertyDiscriminant::Composes as u32,
                VendorPrefix::NONE,
            )),
            Self::Fill => Some((KnownPropertyDiscriminant::Fill as u32, VendorPrefix::NONE)),
            Self::FillRule => Some((
                KnownPropertyDiscriminant::FillRule as u32,
                VendorPrefix::NONE,
            )),
            Self::FillOpacity => Some((
                KnownPropertyDiscriminant::FillOpacity as u32,
                VendorPrefix::NONE,
            )),
            Self::Stroke => Some((KnownPropertyDiscriminant::Stroke as u32, VendorPrefix::NONE)),
            Self::StrokeOpacity => Some((
                KnownPropertyDiscriminant::StrokeOpacity as u32,
                VendorPrefix::NONE,
            )),
            Self::StrokeWidth => Some((
                KnownPropertyDiscriminant::StrokeWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::StrokeLinecap => Some((
                KnownPropertyDiscriminant::StrokeLinecap as u32,
                VendorPrefix::NONE,
            )),
            Self::StrokeLinejoin => Some((
                KnownPropertyDiscriminant::StrokeLinejoin as u32,
                VendorPrefix::NONE,
            )),
            Self::StrokeMiterlimit => Some((
                KnownPropertyDiscriminant::StrokeMiterlimit as u32,
                VendorPrefix::NONE,
            )),
            Self::StrokeDasharray => Some((
                KnownPropertyDiscriminant::StrokeDasharray as u32,
                VendorPrefix::NONE,
            )),
            Self::StrokeDashoffset => Some((
                KnownPropertyDiscriminant::StrokeDashoffset as u32,
                VendorPrefix::NONE,
            )),
            Self::MarkerStart => Some((
                KnownPropertyDiscriminant::MarkerStart as u32,
                VendorPrefix::NONE,
            )),
            Self::MarkerMid => Some((
                KnownPropertyDiscriminant::MarkerMid as u32,
                VendorPrefix::NONE,
            )),
            Self::MarkerEnd => Some((
                KnownPropertyDiscriminant::MarkerEnd as u32,
                VendorPrefix::NONE,
            )),
            Self::Marker => Some((KnownPropertyDiscriminant::Marker as u32, VendorPrefix::NONE)),
            Self::ColorInterpolation => Some((
                KnownPropertyDiscriminant::ColorInterpolation as u32,
                VendorPrefix::NONE,
            )),
            Self::ColorInterpolationFilters => Some((
                KnownPropertyDiscriminant::ColorInterpolationFilters as u32,
                VendorPrefix::NONE,
            )),
            Self::ColorRendering => Some((
                KnownPropertyDiscriminant::ColorRendering as u32,
                VendorPrefix::NONE,
            )),
            Self::ShapeRendering => Some((
                KnownPropertyDiscriminant::ShapeRendering as u32,
                VendorPrefix::NONE,
            )),
            Self::TextRendering => Some((
                KnownPropertyDiscriminant::TextRendering as u32,
                VendorPrefix::NONE,
            )),
            Self::ImageRendering => Some((
                KnownPropertyDiscriminant::ImageRendering as u32,
                VendorPrefix::NONE,
            )),
            Self::ClipPath(prefix) => Some((KnownPropertyDiscriminant::ClipPath as u32, *prefix)),
            Self::ClipRule => Some((
                KnownPropertyDiscriminant::ClipRule as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskImage(prefix) => Some((KnownPropertyDiscriminant::MaskImage as u32, *prefix)),
            Self::MaskMode => Some((
                KnownPropertyDiscriminant::MaskMode as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskRepeat(prefix) => {
                Some((KnownPropertyDiscriminant::MaskRepeat as u32, *prefix))
            }
            Self::MaskPositionX => Some((
                KnownPropertyDiscriminant::MaskPositionX as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskPositionY => Some((
                KnownPropertyDiscriminant::MaskPositionY as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskPosition(prefix) => {
                Some((KnownPropertyDiscriminant::MaskPosition as u32, *prefix))
            }
            Self::MaskClip(prefix) => Some((KnownPropertyDiscriminant::MaskClip as u32, *prefix)),
            Self::MaskOrigin(prefix) => {
                Some((KnownPropertyDiscriminant::MaskOrigin as u32, *prefix))
            }
            Self::MaskSize(prefix) => Some((KnownPropertyDiscriminant::MaskSize as u32, *prefix)),
            Self::MaskComposite => Some((
                KnownPropertyDiscriminant::MaskComposite as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskType => Some((
                KnownPropertyDiscriminant::MaskType as u32,
                VendorPrefix::NONE,
            )),
            Self::Mask(prefix) => Some((KnownPropertyDiscriminant::Mask as u32, *prefix)),
            Self::MaskBorderSource => Some((
                KnownPropertyDiscriminant::MaskBorderSource as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskBorderMode => Some((
                KnownPropertyDiscriminant::MaskBorderMode as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskBorderSlice => Some((
                KnownPropertyDiscriminant::MaskBorderSlice as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskBorderWidth => Some((
                KnownPropertyDiscriminant::MaskBorderWidth as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskBorderOutset => Some((
                KnownPropertyDiscriminant::MaskBorderOutset as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskBorderRepeat => Some((
                KnownPropertyDiscriminant::MaskBorderRepeat as u32,
                VendorPrefix::NONE,
            )),
            Self::MaskBorder => Some((
                KnownPropertyDiscriminant::MaskBorder as u32,
                VendorPrefix::NONE,
            )),
            Self::WebKitMaskComposite => Some((
                KnownPropertyDiscriminant::WebKitMaskComposite as u32,
                VendorPrefix::NONE,
            )),
            Self::WebKitMaskSourceType(prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskSourceType as u32,
                *prefix,
            )),
            Self::WebKitMaskBoxImage(prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImage as u32,
                *prefix,
            )),
            Self::WebKitMaskBoxImageSource(prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageSource as u32,
                *prefix,
            )),
            Self::WebKitMaskBoxImageSlice(prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageSlice as u32,
                *prefix,
            )),
            Self::WebKitMaskBoxImageWidth(prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageWidth as u32,
                *prefix,
            )),
            Self::WebKitMaskBoxImageOutset(prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageOutset as u32,
                *prefix,
            )),
            Self::WebKitMaskBoxImageRepeat(prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageRepeat as u32,
                *prefix,
            )),
            Self::Filter(prefix) => Some((KnownPropertyDiscriminant::Filter as u32, *prefix)),
            Self::BackdropFilter(prefix) => {
                Some((KnownPropertyDiscriminant::BackdropFilter as u32, *prefix))
            }
            Self::MixBlendMode => Some((
                KnownPropertyDiscriminant::MixBlendMode as u32,
                VendorPrefix::NONE,
            )),
            Self::ZIndex => Some((KnownPropertyDiscriminant::ZIndex as u32, VendorPrefix::NONE)),
            Self::ContainerType => Some((
                KnownPropertyDiscriminant::ContainerType as u32,
                VendorPrefix::NONE,
            )),
            Self::ContainerName => Some((
                KnownPropertyDiscriminant::ContainerName as u32,
                VendorPrefix::NONE,
            )),
            Self::Container => Some((
                KnownPropertyDiscriminant::Container as u32,
                VendorPrefix::NONE,
            )),
            Self::ViewTransitionName => Some((
                KnownPropertyDiscriminant::ViewTransitionName as u32,
                VendorPrefix::NONE,
            )),
            Self::ViewTransitionClass => Some((
                KnownPropertyDiscriminant::ViewTransitionClass as u32,
                VendorPrefix::NONE,
            )),
            Self::ViewTransitionGroup => Some((
                KnownPropertyDiscriminant::ViewTransitionGroup as u32,
                VendorPrefix::NONE,
            )),
            Self::ColorScheme => Some((
                KnownPropertyDiscriminant::ColorScheme as u32,
                VendorPrefix::NONE,
            )),
            Self::PrintColorAdjust(prefix) => {
                Some((KnownPropertyDiscriminant::PrintColorAdjust as u32, *prefix))
            }
            Self::Unparsed | Self::Custom(_) => None,
        }
    }
    /// Returns the generated parser strategy for this known property.
    pub fn parser_strategy(&self) -> PropertyParserStrategy {
        match self {
            Self::All => PropertyParserStrategy::Custom,
            Self::BackgroundColor => PropertyParserStrategy::Boxed,
            Self::BackgroundImage => PropertyParserStrategy::CommaSeparated,
            Self::BackgroundPositionX => PropertyParserStrategy::CommaSeparated,
            Self::BackgroundPositionY => PropertyParserStrategy::CommaSeparated,
            Self::BackgroundPosition => PropertyParserStrategy::CommaSeparated,
            Self::BackgroundSize => PropertyParserStrategy::CommaSeparated,
            Self::BackgroundRepeat => PropertyParserStrategy::CommaSeparated,
            Self::BackgroundAttachment => PropertyParserStrategy::CommaSeparated,
            Self::BackgroundClip(_) => PropertyParserStrategy::CommaSeparated,
            Self::BackgroundOrigin => PropertyParserStrategy::CommaSeparated,
            Self::Background => PropertyParserStrategy::Custom,
            Self::BoxShadow(_) => PropertyParserStrategy::Unsupported,
            Self::Opacity => PropertyParserStrategy::Custom,
            Self::Color => PropertyParserStrategy::Boxed,
            Self::Display => PropertyParserStrategy::Parse,
            Self::Visibility => PropertyParserStrategy::Parse,
            Self::Width => PropertyParserStrategy::Boxed,
            Self::Height => PropertyParserStrategy::Boxed,
            Self::MinWidth => PropertyParserStrategy::Boxed,
            Self::MinHeight => PropertyParserStrategy::Boxed,
            Self::MaxWidth => PropertyParserStrategy::Boxed,
            Self::MaxHeight => PropertyParserStrategy::Boxed,
            Self::BlockSize => PropertyParserStrategy::Boxed,
            Self::InlineSize => PropertyParserStrategy::Boxed,
            Self::MinBlockSize => PropertyParserStrategy::Boxed,
            Self::MinInlineSize => PropertyParserStrategy::Boxed,
            Self::MaxBlockSize => PropertyParserStrategy::Boxed,
            Self::MaxInlineSize => PropertyParserStrategy::Boxed,
            Self::BoxSizing(_) => PropertyParserStrategy::Parse,
            Self::AspectRatio => PropertyParserStrategy::Parse,
            Self::Overflow => PropertyParserStrategy::Parse,
            Self::OverflowX => PropertyParserStrategy::Parse,
            Self::OverflowY => PropertyParserStrategy::Parse,
            Self::TextOverflow(_) => PropertyParserStrategy::Parse,
            Self::ObjectFit => PropertyParserStrategy::Parse,
            Self::ObjectPosition => PropertyParserStrategy::Boxed,
            Self::ScrollbarColor => PropertyParserStrategy::Parse,
            Self::Position => PropertyParserStrategy::Boxed,
            Self::Top => PropertyParserStrategy::Boxed,
            Self::Bottom => PropertyParserStrategy::Boxed,
            Self::Left => PropertyParserStrategy::Boxed,
            Self::Right => PropertyParserStrategy::Boxed,
            Self::InsetBlockStart => PropertyParserStrategy::Boxed,
            Self::InsetBlockEnd => PropertyParserStrategy::Boxed,
            Self::InsetInlineStart => PropertyParserStrategy::Boxed,
            Self::InsetInlineEnd => PropertyParserStrategy::Boxed,
            Self::InsetBlock => PropertyParserStrategy::TwoValue,
            Self::InsetInline => PropertyParserStrategy::TwoValue,
            Self::Inset => PropertyParserStrategy::Rect,
            Self::BorderSpacing => PropertyParserStrategy::TwoValue,
            Self::BorderTopColor => PropertyParserStrategy::Boxed,
            Self::BorderBottomColor => PropertyParserStrategy::Boxed,
            Self::BorderLeftColor => PropertyParserStrategy::Boxed,
            Self::BorderRightColor => PropertyParserStrategy::Boxed,
            Self::BorderBlockStartColor => PropertyParserStrategy::Boxed,
            Self::BorderBlockEndColor => PropertyParserStrategy::Boxed,
            Self::BorderInlineStartColor => PropertyParserStrategy::Boxed,
            Self::BorderInlineEndColor => PropertyParserStrategy::Boxed,
            Self::BorderTopStyle => PropertyParserStrategy::Parse,
            Self::BorderBottomStyle => PropertyParserStrategy::Parse,
            Self::BorderLeftStyle => PropertyParserStrategy::Parse,
            Self::BorderRightStyle => PropertyParserStrategy::Parse,
            Self::BorderBlockStartStyle => PropertyParserStrategy::Parse,
            Self::BorderBlockEndStyle => PropertyParserStrategy::Parse,
            Self::BorderInlineStartStyle => PropertyParserStrategy::Parse,
            Self::BorderInlineEndStyle => PropertyParserStrategy::Parse,
            Self::BorderTopWidth => PropertyParserStrategy::Boxed,
            Self::BorderBottomWidth => PropertyParserStrategy::Boxed,
            Self::BorderLeftWidth => PropertyParserStrategy::Boxed,
            Self::BorderRightWidth => PropertyParserStrategy::Boxed,
            Self::BorderBlockStartWidth => PropertyParserStrategy::Boxed,
            Self::BorderBlockEndWidth => PropertyParserStrategy::Boxed,
            Self::BorderInlineStartWidth => PropertyParserStrategy::Boxed,
            Self::BorderInlineEndWidth => PropertyParserStrategy::Boxed,
            Self::BorderTopLeftRadius(_) => PropertyParserStrategy::TwoValue,
            Self::BorderTopRightRadius(_) => PropertyParserStrategy::TwoValue,
            Self::BorderBottomLeftRadius(_) => PropertyParserStrategy::TwoValue,
            Self::BorderBottomRightRadius(_) => PropertyParserStrategy::TwoValue,
            Self::BorderStartStartRadius => PropertyParserStrategy::TwoValue,
            Self::BorderStartEndRadius => PropertyParserStrategy::TwoValue,
            Self::BorderEndStartRadius => PropertyParserStrategy::TwoValue,
            Self::BorderEndEndRadius => PropertyParserStrategy::TwoValue,
            Self::BorderRadius(_) => PropertyParserStrategy::Rect,
            Self::BorderImageSource => PropertyParserStrategy::Boxed,
            Self::BorderImageOutset => PropertyParserStrategy::Unsupported,
            Self::BorderImageRepeat => PropertyParserStrategy::Unsupported,
            Self::BorderImageWidth => PropertyParserStrategy::Unsupported,
            Self::BorderImageSlice => PropertyParserStrategy::Unsupported,
            Self::BorderImage(_) => PropertyParserStrategy::Unsupported,
            Self::BorderColor => PropertyParserStrategy::Rect,
            Self::BorderStyle => PropertyParserStrategy::Rect,
            Self::BorderWidth => PropertyParserStrategy::Rect,
            Self::BorderBlockColor => PropertyParserStrategy::TwoValue,
            Self::BorderBlockStyle => PropertyParserStrategy::TwoValue,
            Self::BorderBlockWidth => PropertyParserStrategy::TwoValue,
            Self::BorderInlineColor => PropertyParserStrategy::TwoValue,
            Self::BorderInlineStyle => PropertyParserStrategy::TwoValue,
            Self::BorderInlineWidth => PropertyParserStrategy::TwoValue,
            Self::Border => PropertyParserStrategy::Boxed,
            Self::BorderTop => PropertyParserStrategy::Boxed,
            Self::BorderBottom => PropertyParserStrategy::Boxed,
            Self::BorderLeft => PropertyParserStrategy::Boxed,
            Self::BorderRight => PropertyParserStrategy::Boxed,
            Self::BorderBlock => PropertyParserStrategy::Boxed,
            Self::BorderBlockStart => PropertyParserStrategy::Boxed,
            Self::BorderBlockEnd => PropertyParserStrategy::Boxed,
            Self::BorderInline => PropertyParserStrategy::Boxed,
            Self::BorderInlineStart => PropertyParserStrategy::Boxed,
            Self::BorderInlineEnd => PropertyParserStrategy::Boxed,
            Self::Outline => PropertyParserStrategy::Boxed,
            Self::OutlineColor => PropertyParserStrategy::Boxed,
            Self::OutlineStyle => PropertyParserStrategy::Parse,
            Self::OutlineWidth => PropertyParserStrategy::Boxed,
            Self::FlexDirection(_) => PropertyParserStrategy::Parse,
            Self::FlexWrap(_) => PropertyParserStrategy::Parse,
            Self::FlexFlow(_) => PropertyParserStrategy::WhitespaceSeparated,
            Self::FlexGrow(_) => PropertyParserStrategy::Custom,
            Self::FlexShrink(_) => PropertyParserStrategy::Custom,
            Self::FlexBasis(_) => PropertyParserStrategy::Boxed,
            Self::Flex(_) => PropertyParserStrategy::Boxed,
            Self::Order(_) => PropertyParserStrategy::Custom,
            Self::AlignContent(_) => PropertyParserStrategy::Parse,
            Self::JustifyContent(_) => PropertyParserStrategy::Parse,
            Self::PlaceContent => PropertyParserStrategy::Parse,
            Self::AlignSelf(_) => PropertyParserStrategy::Parse,
            Self::JustifySelf => PropertyParserStrategy::Parse,
            Self::PlaceSelf => PropertyParserStrategy::Parse,
            Self::AlignItems(_) => PropertyParserStrategy::Parse,
            Self::JustifyItems => PropertyParserStrategy::Parse,
            Self::PlaceItems => PropertyParserStrategy::Parse,
            Self::RowGap => PropertyParserStrategy::Boxed,
            Self::ColumnGap => PropertyParserStrategy::Boxed,
            Self::Gap => PropertyParserStrategy::TwoValue,
            Self::ColumnRule(_) => PropertyParserStrategy::Boxed,
            Self::ColumnWidth(_) => PropertyParserStrategy::CssWide,
            Self::ColumnCount(_) => PropertyParserStrategy::CssWide,
            Self::Columns(_) => PropertyParserStrategy::CssWideBoxed,
            Self::GridColumnGap => PropertyParserStrategy::Boxed,
            Self::GridRowGap => PropertyParserStrategy::Boxed,
            Self::BoxOrient(_) => PropertyParserStrategy::Parse,
            Self::BoxDirection(_) => PropertyParserStrategy::Parse,
            Self::BoxOrdinalGroup(_) => PropertyParserStrategy::Custom,
            Self::BoxAlign(_) => PropertyParserStrategy::Parse,
            Self::BoxFlex(_) => PropertyParserStrategy::Custom,
            Self::BoxFlexGroup(_) => PropertyParserStrategy::Custom,
            Self::BoxPack(_) => PropertyParserStrategy::Parse,
            Self::BoxLines(_) => PropertyParserStrategy::Parse,
            Self::FlexPack(_) => PropertyParserStrategy::Parse,
            Self::FlexOrder(_) => PropertyParserStrategy::Custom,
            Self::FlexAlign(_) => PropertyParserStrategy::Parse,
            Self::FlexItemAlign(_) => PropertyParserStrategy::Parse,
            Self::FlexLinePack(_) => PropertyParserStrategy::Parse,
            Self::FlexPositive(_) => PropertyParserStrategy::Custom,
            Self::FlexNegative(_) => PropertyParserStrategy::Custom,
            Self::FlexPreferredSize(_) => PropertyParserStrategy::Boxed,
            Self::GridTemplateColumns => PropertyParserStrategy::Unsupported,
            Self::GridTemplateRows => PropertyParserStrategy::Unsupported,
            Self::GridAutoColumns => PropertyParserStrategy::Unsupported,
            Self::GridAutoRows => PropertyParserStrategy::Unsupported,
            Self::GridAutoFlow => PropertyParserStrategy::Unsupported,
            Self::GridTemplateAreas => PropertyParserStrategy::Unsupported,
            Self::GridTemplate => PropertyParserStrategy::Unsupported,
            Self::Grid => PropertyParserStrategy::Unsupported,
            Self::GridRowStart => PropertyParserStrategy::Unsupported,
            Self::GridRowEnd => PropertyParserStrategy::Unsupported,
            Self::GridColumnStart => PropertyParserStrategy::Unsupported,
            Self::GridColumnEnd => PropertyParserStrategy::Unsupported,
            Self::GridRow => PropertyParserStrategy::Unsupported,
            Self::GridColumn => PropertyParserStrategy::Unsupported,
            Self::GridArea => PropertyParserStrategy::Unsupported,
            Self::MarginTop => PropertyParserStrategy::Boxed,
            Self::MarginBottom => PropertyParserStrategy::Boxed,
            Self::MarginLeft => PropertyParserStrategy::Boxed,
            Self::MarginRight => PropertyParserStrategy::Boxed,
            Self::MarginBlockStart => PropertyParserStrategy::Boxed,
            Self::MarginBlockEnd => PropertyParserStrategy::Boxed,
            Self::MarginInlineStart => PropertyParserStrategy::Boxed,
            Self::MarginInlineEnd => PropertyParserStrategy::Boxed,
            Self::MarginBlock => PropertyParserStrategy::TwoValue,
            Self::MarginInline => PropertyParserStrategy::TwoValue,
            Self::Margin => PropertyParserStrategy::Rect,
            Self::PaddingTop => PropertyParserStrategy::Boxed,
            Self::PaddingBottom => PropertyParserStrategy::Boxed,
            Self::PaddingLeft => PropertyParserStrategy::Boxed,
            Self::PaddingRight => PropertyParserStrategy::Boxed,
            Self::PaddingBlockStart => PropertyParserStrategy::Boxed,
            Self::PaddingBlockEnd => PropertyParserStrategy::Boxed,
            Self::PaddingInlineStart => PropertyParserStrategy::Boxed,
            Self::PaddingInlineEnd => PropertyParserStrategy::Boxed,
            Self::PaddingBlock => PropertyParserStrategy::TwoValue,
            Self::PaddingInline => PropertyParserStrategy::TwoValue,
            Self::Padding => PropertyParserStrategy::Rect,
            Self::ScrollMarginTop => PropertyParserStrategy::Boxed,
            Self::ScrollMarginBottom => PropertyParserStrategy::Boxed,
            Self::ScrollMarginLeft => PropertyParserStrategy::Boxed,
            Self::ScrollMarginRight => PropertyParserStrategy::Boxed,
            Self::ScrollMarginBlockStart => PropertyParserStrategy::Boxed,
            Self::ScrollMarginBlockEnd => PropertyParserStrategy::Boxed,
            Self::ScrollMarginInlineStart => PropertyParserStrategy::Boxed,
            Self::ScrollMarginInlineEnd => PropertyParserStrategy::Boxed,
            Self::ScrollMarginBlock => PropertyParserStrategy::TwoValue,
            Self::ScrollMarginInline => PropertyParserStrategy::TwoValue,
            Self::ScrollMargin => PropertyParserStrategy::Rect,
            Self::ScrollPaddingTop => PropertyParserStrategy::Boxed,
            Self::ScrollPaddingBottom => PropertyParserStrategy::Boxed,
            Self::ScrollPaddingLeft => PropertyParserStrategy::Boxed,
            Self::ScrollPaddingRight => PropertyParserStrategy::Boxed,
            Self::ScrollPaddingBlockStart => PropertyParserStrategy::Boxed,
            Self::ScrollPaddingBlockEnd => PropertyParserStrategy::Boxed,
            Self::ScrollPaddingInlineStart => PropertyParserStrategy::Boxed,
            Self::ScrollPaddingInlineEnd => PropertyParserStrategy::Boxed,
            Self::ScrollPaddingBlock => PropertyParserStrategy::TwoValue,
            Self::ScrollPaddingInline => PropertyParserStrategy::TwoValue,
            Self::ScrollPadding => PropertyParserStrategy::Rect,
            Self::FontWeight => PropertyParserStrategy::Parse,
            Self::FontSize => PropertyParserStrategy::Boxed,
            Self::FontStretch => PropertyParserStrategy::Parse,
            Self::FontFamily => PropertyParserStrategy::Custom,
            Self::FontStyle => PropertyParserStrategy::Parse,
            Self::FontVariantCaps => PropertyParserStrategy::Parse,
            Self::LineHeight => PropertyParserStrategy::Boxed,
            Self::Font => PropertyParserStrategy::Unsupported,
            Self::VerticalAlign => PropertyParserStrategy::Unsupported,
            Self::FontPalette => PropertyParserStrategy::Unsupported,
            Self::TransitionProperty(_) => PropertyParserStrategy::Custom,
            Self::TransitionDuration(_) => PropertyParserStrategy::Custom,
            Self::TransitionDelay(_) => PropertyParserStrategy::Custom,
            Self::TransitionTimingFunction(_) => PropertyParserStrategy::Custom,
            Self::Transition(_) => PropertyParserStrategy::Custom,
            Self::AnimationName(_) => PropertyParserStrategy::Custom,
            Self::AnimationDuration(_) => PropertyParserStrategy::Custom,
            Self::AnimationTimingFunction(_) => PropertyParserStrategy::Custom,
            Self::AnimationIterationCount(_) => PropertyParserStrategy::Custom,
            Self::AnimationDirection(_) => PropertyParserStrategy::Custom,
            Self::AnimationPlayState(_) => PropertyParserStrategy::Custom,
            Self::AnimationDelay(_) => PropertyParserStrategy::Custom,
            Self::AnimationFillMode(_) => PropertyParserStrategy::Custom,
            Self::AnimationComposition => PropertyParserStrategy::Unsupported,
            Self::AnimationTimeline => PropertyParserStrategy::Unsupported,
            Self::AnimationRangeStart => PropertyParserStrategy::Unsupported,
            Self::AnimationRangeEnd => PropertyParserStrategy::Unsupported,
            Self::AnimationRange => PropertyParserStrategy::Unsupported,
            Self::Animation(_) => PropertyParserStrategy::Custom,
            Self::Transform(_) => PropertyParserStrategy::Custom,
            Self::TransformOrigin(_) => PropertyParserStrategy::Boxed,
            Self::TransformStyle(_) => PropertyParserStrategy::Parse,
            Self::TransformBox => PropertyParserStrategy::Parse,
            Self::BackfaceVisibility(_) => PropertyParserStrategy::Parse,
            Self::Perspective(_) => PropertyParserStrategy::Boxed,
            Self::PerspectiveOrigin(_) => PropertyParserStrategy::Boxed,
            Self::Translate => PropertyParserStrategy::Boxed,
            Self::Rotate => PropertyParserStrategy::Parse,
            Self::Scale => PropertyParserStrategy::Boxed,
            Self::TextTransform => PropertyParserStrategy::Parse,
            Self::Content => PropertyParserStrategy::Unsupported,
            Self::WhiteSpace => PropertyParserStrategy::Parse,
            Self::TabSize(_) => PropertyParserStrategy::Unsupported,
            Self::WordBreak => PropertyParserStrategy::Parse,
            Self::LineBreak => PropertyParserStrategy::Parse,
            Self::Hyphens(_) => PropertyParserStrategy::Parse,
            Self::OverflowWrap => PropertyParserStrategy::Parse,
            Self::WordWrap => PropertyParserStrategy::Parse,
            Self::TextAlign => PropertyParserStrategy::Parse,
            Self::TextAlignLast(_) => PropertyParserStrategy::Parse,
            Self::TextJustify => PropertyParserStrategy::Parse,
            Self::WordSpacing => PropertyParserStrategy::Boxed,
            Self::LetterSpacing => PropertyParserStrategy::Boxed,
            Self::TextIndent => PropertyParserStrategy::Boxed,
            Self::TextDecorationLine(_) => PropertyParserStrategy::Boxed,
            Self::TextDecorationStyle(_) => PropertyParserStrategy::Parse,
            Self::TextDecorationColor(_) => PropertyParserStrategy::Boxed,
            Self::TextDecorationThickness => PropertyParserStrategy::Boxed,
            Self::TextDecoration(_) => PropertyParserStrategy::Unsupported,
            Self::TextDecorationSkipInk(_) => PropertyParserStrategy::Parse,
            Self::TextEmphasisStyle(_) => PropertyParserStrategy::Unsupported,
            Self::TextEmphasisColor(_) => PropertyParserStrategy::Boxed,
            Self::TextEmphasis(_) => PropertyParserStrategy::Unsupported,
            Self::TextEmphasisPosition(_) => PropertyParserStrategy::Unsupported,
            Self::TextShadow => PropertyParserStrategy::Unsupported,
            Self::TextSizeAdjust(_) => PropertyParserStrategy::Parse,
            Self::Direction => PropertyParserStrategy::Parse,
            Self::UnicodeBidi => PropertyParserStrategy::Parse,
            Self::BoxDecorationBreak(_) => PropertyParserStrategy::Parse,
            Self::Resize => PropertyParserStrategy::Parse,
            Self::PointerEvents => PropertyParserStrategy::Parse,
            Self::Float => PropertyParserStrategy::Parse,
            Self::Clear => PropertyParserStrategy::Parse,
            Self::TouchAction => PropertyParserStrategy::Parse,
            Self::ScrollBehavior => PropertyParserStrategy::Parse,
            Self::Cursor => PropertyParserStrategy::Unsupported,
            Self::CaretColor => PropertyParserStrategy::Unsupported,
            Self::CaretShape => PropertyParserStrategy::Unsupported,
            Self::Caret => PropertyParserStrategy::Unsupported,
            Self::UserSelect(_) => PropertyParserStrategy::Parse,
            Self::AccentColor => PropertyParserStrategy::Boxed,
            Self::Appearance(_) => PropertyParserStrategy::Unsupported,
            Self::ListStyleType => PropertyParserStrategy::Unsupported,
            Self::ListStyleImage => PropertyParserStrategy::Boxed,
            Self::ListStylePosition => PropertyParserStrategy::Unsupported,
            Self::ListStyle => PropertyParserStrategy::Unsupported,
            Self::MarkerSide => PropertyParserStrategy::Unsupported,
            Self::Composes => PropertyParserStrategy::Unsupported,
            Self::Fill => PropertyParserStrategy::Boxed,
            Self::FillRule => PropertyParserStrategy::Parse,
            Self::FillOpacity => PropertyParserStrategy::Custom,
            Self::Stroke => PropertyParserStrategy::Boxed,
            Self::StrokeOpacity => PropertyParserStrategy::Custom,
            Self::StrokeWidth => PropertyParserStrategy::Boxed,
            Self::StrokeLinecap => PropertyParserStrategy::Parse,
            Self::StrokeLinejoin => PropertyParserStrategy::Parse,
            Self::StrokeMiterlimit => PropertyParserStrategy::Custom,
            Self::StrokeDasharray => PropertyParserStrategy::Boxed,
            Self::StrokeDashoffset => PropertyParserStrategy::Boxed,
            Self::MarkerStart => PropertyParserStrategy::Boxed,
            Self::MarkerMid => PropertyParserStrategy::Boxed,
            Self::MarkerEnd => PropertyParserStrategy::Boxed,
            Self::Marker => PropertyParserStrategy::Boxed,
            Self::ColorInterpolation => PropertyParserStrategy::Parse,
            Self::ColorInterpolationFilters => PropertyParserStrategy::Parse,
            Self::ColorRendering => PropertyParserStrategy::Parse,
            Self::ShapeRendering => PropertyParserStrategy::Parse,
            Self::TextRendering => PropertyParserStrategy::Parse,
            Self::ImageRendering => PropertyParserStrategy::Parse,
            Self::ClipPath(_) => PropertyParserStrategy::Unsupported,
            Self::ClipRule => PropertyParserStrategy::Unsupported,
            Self::MaskImage(_) => PropertyParserStrategy::CommaSeparated,
            Self::MaskMode => PropertyParserStrategy::CommaSeparated,
            Self::MaskRepeat(_) => PropertyParserStrategy::CommaSeparated,
            Self::MaskPositionX => PropertyParserStrategy::CommaSeparated,
            Self::MaskPositionY => PropertyParserStrategy::CommaSeparated,
            Self::MaskPosition(_) => PropertyParserStrategy::CommaSeparated,
            Self::MaskClip(_) => PropertyParserStrategy::CommaSeparated,
            Self::MaskOrigin(_) => PropertyParserStrategy::CommaSeparated,
            Self::MaskSize(_) => PropertyParserStrategy::CommaSeparated,
            Self::MaskComposite => PropertyParserStrategy::CommaSeparated,
            Self::MaskType => PropertyParserStrategy::Parse,
            Self::Mask(_) => PropertyParserStrategy::CommaSeparated,
            Self::MaskBorderSource => PropertyParserStrategy::Boxed,
            Self::MaskBorderMode => PropertyParserStrategy::Unsupported,
            Self::MaskBorderSlice => PropertyParserStrategy::Unsupported,
            Self::MaskBorderWidth => PropertyParserStrategy::Unsupported,
            Self::MaskBorderOutset => PropertyParserStrategy::Unsupported,
            Self::MaskBorderRepeat => PropertyParserStrategy::Unsupported,
            Self::MaskBorder => PropertyParserStrategy::Unsupported,
            Self::WebKitMaskComposite => PropertyParserStrategy::CommaSeparated,
            Self::WebKitMaskSourceType(_) => PropertyParserStrategy::CommaSeparated,
            Self::WebKitMaskBoxImage(_) => PropertyParserStrategy::Unsupported,
            Self::WebKitMaskBoxImageSource(_) => PropertyParserStrategy::Boxed,
            Self::WebKitMaskBoxImageSlice(_) => PropertyParserStrategy::Unsupported,
            Self::WebKitMaskBoxImageWidth(_) => PropertyParserStrategy::Unsupported,
            Self::WebKitMaskBoxImageOutset(_) => PropertyParserStrategy::Unsupported,
            Self::WebKitMaskBoxImageRepeat(_) => PropertyParserStrategy::Unsupported,
            Self::Filter(_) => PropertyParserStrategy::Unsupported,
            Self::BackdropFilter(_) => PropertyParserStrategy::Unsupported,
            Self::MixBlendMode => PropertyParserStrategy::Unsupported,
            Self::ZIndex => PropertyParserStrategy::Parse,
            Self::ContainerType => PropertyParserStrategy::Unsupported,
            Self::ContainerName => PropertyParserStrategy::Unsupported,
            Self::Container => PropertyParserStrategy::Unsupported,
            Self::ViewTransitionName => PropertyParserStrategy::Unsupported,
            Self::ViewTransitionClass => PropertyParserStrategy::Unsupported,
            Self::ViewTransitionGroup => PropertyParserStrategy::Unsupported,
            Self::ColorScheme => PropertyParserStrategy::Unsupported,
            Self::PrintColorAdjust(_) => PropertyParserStrategy::Unsupported,
            Self::Unparsed | Self::Custom(_) => PropertyParserStrategy::Unsupported,
        }
    }
    /// Returns the generated support classification for this property.
    pub fn support_classification(&self) -> PropertySupport {
        match self {
            Self::Custom(_) => PropertySupport::Custom,
            Self::Unparsed => PropertySupport::UnsupportedGrammar,
            _ => self.parser_strategy().support_classification(),
        }
    }
}
impl PropertyParserStrategy {
    #[inline]
    pub const fn support_classification(self) -> PropertySupport {
        match self {
            Self::Unsupported => PropertySupport::UnsupportedGrammar,
            Self::Parse
            | Self::Boxed
            | Self::CommaSeparated
            | Self::WhitespaceSeparated
            | Self::Rect
            | Self::TwoValue
            | Self::Custom
            | Self::CssWide
            | Self::CssWideBoxed => PropertySupport::Typed,
        }
    }
}
impl<'ast> Declaration<'ast> {
    /// Returns the compact discriminant and vendor prefix of a known declaration.
    #[inline]
    pub fn known_id_and_prefix(self, ast: &Ast<'ast>) -> Option<(u32, VendorPrefix)> {
        match self.data(ast) {
            DeclarationData::All(..) => {
                Some((KnownPropertyDiscriminant::All as u32, VendorPrefix::NONE))
            }
            DeclarationData::BackgroundColor(..) => Some((
                KnownPropertyDiscriminant::BackgroundColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackgroundImage(..) => Some((
                KnownPropertyDiscriminant::BackgroundImage as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackgroundPositionX(..) => Some((
                KnownPropertyDiscriminant::BackgroundPositionX as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackgroundPositionY(..) => Some((
                KnownPropertyDiscriminant::BackgroundPositionY as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackgroundPosition(..) => Some((
                KnownPropertyDiscriminant::BackgroundPosition as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackgroundSize(..) => Some((
                KnownPropertyDiscriminant::BackgroundSize as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackgroundRepeat(..) => Some((
                KnownPropertyDiscriminant::BackgroundRepeat as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackgroundAttachment(..) => Some((
                KnownPropertyDiscriminant::BackgroundAttachment as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackgroundClip(_, prefix) => {
                Some((KnownPropertyDiscriminant::BackgroundClip as u32, prefix))
            }
            DeclarationData::BackgroundOrigin(..) => Some((
                KnownPropertyDiscriminant::BackgroundOrigin as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Background(..) => Some((
                KnownPropertyDiscriminant::Background as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BoxShadow(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxShadow as u32, prefix))
            }
            DeclarationData::Opacity(..) => Some((
                KnownPropertyDiscriminant::Opacity as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Color(..) => {
                Some((KnownPropertyDiscriminant::Color as u32, VendorPrefix::NONE))
            }
            DeclarationData::Display(..) => Some((
                KnownPropertyDiscriminant::Display as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Visibility(..) => Some((
                KnownPropertyDiscriminant::Visibility as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Width(..) => {
                Some((KnownPropertyDiscriminant::Width as u32, VendorPrefix::NONE))
            }
            DeclarationData::Height(..) => {
                Some((KnownPropertyDiscriminant::Height as u32, VendorPrefix::NONE))
            }
            DeclarationData::MinWidth(..) => Some((
                KnownPropertyDiscriminant::MinWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MinHeight(..) => Some((
                KnownPropertyDiscriminant::MinHeight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaxWidth(..) => Some((
                KnownPropertyDiscriminant::MaxWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaxHeight(..) => Some((
                KnownPropertyDiscriminant::MaxHeight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BlockSize(..) => Some((
                KnownPropertyDiscriminant::BlockSize as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::InlineSize(..) => Some((
                KnownPropertyDiscriminant::InlineSize as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MinBlockSize(..) => Some((
                KnownPropertyDiscriminant::MinBlockSize as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MinInlineSize(..) => Some((
                KnownPropertyDiscriminant::MinInlineSize as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaxBlockSize(..) => Some((
                KnownPropertyDiscriminant::MaxBlockSize as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaxInlineSize(..) => Some((
                KnownPropertyDiscriminant::MaxInlineSize as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BoxSizing(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxSizing as u32, prefix))
            }
            DeclarationData::AspectRatio(..) => Some((
                KnownPropertyDiscriminant::AspectRatio as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Overflow(..) => Some((
                KnownPropertyDiscriminant::Overflow as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::OverflowX(..) => Some((
                KnownPropertyDiscriminant::OverflowX as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::OverflowY(..) => Some((
                KnownPropertyDiscriminant::OverflowY as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TextOverflow(_, prefix) => {
                Some((KnownPropertyDiscriminant::TextOverflow as u32, prefix))
            }
            DeclarationData::ObjectFit(..) => Some((
                KnownPropertyDiscriminant::ObjectFit as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ObjectPosition(..) => Some((
                KnownPropertyDiscriminant::ObjectPosition as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollbarColor(..) => Some((
                KnownPropertyDiscriminant::ScrollbarColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Position(..) => Some((
                KnownPropertyDiscriminant::Position as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Top(..) => {
                Some((KnownPropertyDiscriminant::Top as u32, VendorPrefix::NONE))
            }
            DeclarationData::Bottom(..) => {
                Some((KnownPropertyDiscriminant::Bottom as u32, VendorPrefix::NONE))
            }
            DeclarationData::Left(..) => {
                Some((KnownPropertyDiscriminant::Left as u32, VendorPrefix::NONE))
            }
            DeclarationData::Right(..) => {
                Some((KnownPropertyDiscriminant::Right as u32, VendorPrefix::NONE))
            }
            DeclarationData::InsetBlockStart(..) => Some((
                KnownPropertyDiscriminant::InsetBlockStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::InsetBlockEnd(..) => Some((
                KnownPropertyDiscriminant::InsetBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::InsetInlineStart(..) => Some((
                KnownPropertyDiscriminant::InsetInlineStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::InsetInlineEnd(..) => Some((
                KnownPropertyDiscriminant::InsetInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::InsetBlock(..) => Some((
                KnownPropertyDiscriminant::InsetBlock as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::InsetInline(..) => Some((
                KnownPropertyDiscriminant::InsetInline as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Inset(..) => {
                Some((KnownPropertyDiscriminant::Inset as u32, VendorPrefix::NONE))
            }
            DeclarationData::BorderSpacing(..) => Some((
                KnownPropertyDiscriminant::BorderSpacing as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderTopColor(..) => Some((
                KnownPropertyDiscriminant::BorderTopColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBottomColor(..) => Some((
                KnownPropertyDiscriminant::BorderBottomColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderLeftColor(..) => Some((
                KnownPropertyDiscriminant::BorderLeftColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderRightColor(..) => Some((
                KnownPropertyDiscriminant::BorderRightColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockStartColor(..) => Some((
                KnownPropertyDiscriminant::BorderBlockStartColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockEndColor(..) => Some((
                KnownPropertyDiscriminant::BorderBlockEndColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineStartColor(..) => Some((
                KnownPropertyDiscriminant::BorderInlineStartColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineEndColor(..) => Some((
                KnownPropertyDiscriminant::BorderInlineEndColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderTopStyle(..) => Some((
                KnownPropertyDiscriminant::BorderTopStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBottomStyle(..) => Some((
                KnownPropertyDiscriminant::BorderBottomStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderLeftStyle(..) => Some((
                KnownPropertyDiscriminant::BorderLeftStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderRightStyle(..) => Some((
                KnownPropertyDiscriminant::BorderRightStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockStartStyle(..) => Some((
                KnownPropertyDiscriminant::BorderBlockStartStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockEndStyle(..) => Some((
                KnownPropertyDiscriminant::BorderBlockEndStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineStartStyle(..) => Some((
                KnownPropertyDiscriminant::BorderInlineStartStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineEndStyle(..) => Some((
                KnownPropertyDiscriminant::BorderInlineEndStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderTopWidth(..) => Some((
                KnownPropertyDiscriminant::BorderTopWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBottomWidth(..) => Some((
                KnownPropertyDiscriminant::BorderBottomWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderLeftWidth(..) => Some((
                KnownPropertyDiscriminant::BorderLeftWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderRightWidth(..) => Some((
                KnownPropertyDiscriminant::BorderRightWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockStartWidth(..) => Some((
                KnownPropertyDiscriminant::BorderBlockStartWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockEndWidth(..) => Some((
                KnownPropertyDiscriminant::BorderBlockEndWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineStartWidth(..) => Some((
                KnownPropertyDiscriminant::BorderInlineStartWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineEndWidth(..) => Some((
                KnownPropertyDiscriminant::BorderInlineEndWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderTopLeftRadius(_, prefix) => Some((
                KnownPropertyDiscriminant::BorderTopLeftRadius as u32,
                prefix,
            )),
            DeclarationData::BorderTopRightRadius(_, prefix) => Some((
                KnownPropertyDiscriminant::BorderTopRightRadius as u32,
                prefix,
            )),
            DeclarationData::BorderBottomLeftRadius(_, prefix) => Some((
                KnownPropertyDiscriminant::BorderBottomLeftRadius as u32,
                prefix,
            )),
            DeclarationData::BorderBottomRightRadius(_, prefix) => Some((
                KnownPropertyDiscriminant::BorderBottomRightRadius as u32,
                prefix,
            )),
            DeclarationData::BorderStartStartRadius(..) => Some((
                KnownPropertyDiscriminant::BorderStartStartRadius as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderStartEndRadius(..) => Some((
                KnownPropertyDiscriminant::BorderStartEndRadius as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderEndStartRadius(..) => Some((
                KnownPropertyDiscriminant::BorderEndStartRadius as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderEndEndRadius(..) => Some((
                KnownPropertyDiscriminant::BorderEndEndRadius as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderRadius(_, prefix) => {
                Some((KnownPropertyDiscriminant::BorderRadius as u32, prefix))
            }
            DeclarationData::BorderImageSource(..) => Some((
                KnownPropertyDiscriminant::BorderImageSource as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderImageOutset(..) => Some((
                KnownPropertyDiscriminant::BorderImageOutset as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderImageRepeat(..) => Some((
                KnownPropertyDiscriminant::BorderImageRepeat as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderImageWidth(..) => Some((
                KnownPropertyDiscriminant::BorderImageWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderImageSlice(..) => Some((
                KnownPropertyDiscriminant::BorderImageSlice as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderImage(_, prefix) => {
                Some((KnownPropertyDiscriminant::BorderImage as u32, prefix))
            }
            DeclarationData::BorderColor(..) => Some((
                KnownPropertyDiscriminant::BorderColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderStyle(..) => Some((
                KnownPropertyDiscriminant::BorderStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderWidth(..) => Some((
                KnownPropertyDiscriminant::BorderWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockColor(..) => Some((
                KnownPropertyDiscriminant::BorderBlockColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockStyle(..) => Some((
                KnownPropertyDiscriminant::BorderBlockStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockWidth(..) => Some((
                KnownPropertyDiscriminant::BorderBlockWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineColor(..) => Some((
                KnownPropertyDiscriminant::BorderInlineColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineStyle(..) => Some((
                KnownPropertyDiscriminant::BorderInlineStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineWidth(..) => Some((
                KnownPropertyDiscriminant::BorderInlineWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Border(..) => {
                Some((KnownPropertyDiscriminant::Border as u32, VendorPrefix::NONE))
            }
            DeclarationData::BorderTop(..) => Some((
                KnownPropertyDiscriminant::BorderTop as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBottom(..) => Some((
                KnownPropertyDiscriminant::BorderBottom as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderLeft(..) => Some((
                KnownPropertyDiscriminant::BorderLeft as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderRight(..) => Some((
                KnownPropertyDiscriminant::BorderRight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlock(..) => Some((
                KnownPropertyDiscriminant::BorderBlock as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockStart(..) => Some((
                KnownPropertyDiscriminant::BorderBlockStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderBlockEnd(..) => Some((
                KnownPropertyDiscriminant::BorderBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInline(..) => Some((
                KnownPropertyDiscriminant::BorderInline as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineStart(..) => Some((
                KnownPropertyDiscriminant::BorderInlineStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BorderInlineEnd(..) => Some((
                KnownPropertyDiscriminant::BorderInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Outline(..) => Some((
                KnownPropertyDiscriminant::Outline as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::OutlineColor(..) => Some((
                KnownPropertyDiscriminant::OutlineColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::OutlineStyle(..) => Some((
                KnownPropertyDiscriminant::OutlineStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::OutlineWidth(..) => Some((
                KnownPropertyDiscriminant::OutlineWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FlexDirection(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexDirection as u32, prefix))
            }
            DeclarationData::FlexWrap(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexWrap as u32, prefix))
            }
            DeclarationData::FlexFlow(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexFlow as u32, prefix))
            }
            DeclarationData::FlexGrow(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexGrow as u32, prefix))
            }
            DeclarationData::FlexShrink(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexShrink as u32, prefix))
            }
            DeclarationData::FlexBasis(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexBasis as u32, prefix))
            }
            DeclarationData::Flex(_, prefix) => {
                Some((KnownPropertyDiscriminant::Flex as u32, prefix))
            }
            DeclarationData::Order(_, prefix) => {
                Some((KnownPropertyDiscriminant::Order as u32, prefix))
            }
            DeclarationData::AlignContent(_, prefix) => {
                Some((KnownPropertyDiscriminant::AlignContent as u32, prefix))
            }
            DeclarationData::JustifyContent(_, prefix) => {
                Some((KnownPropertyDiscriminant::JustifyContent as u32, prefix))
            }
            DeclarationData::PlaceContent(..) => Some((
                KnownPropertyDiscriminant::PlaceContent as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::AlignSelf(_, prefix) => {
                Some((KnownPropertyDiscriminant::AlignSelf as u32, prefix))
            }
            DeclarationData::JustifySelf(..) => Some((
                KnownPropertyDiscriminant::JustifySelf as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PlaceSelf(..) => Some((
                KnownPropertyDiscriminant::PlaceSelf as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::AlignItems(_, prefix) => {
                Some((KnownPropertyDiscriminant::AlignItems as u32, prefix))
            }
            DeclarationData::JustifyItems(..) => Some((
                KnownPropertyDiscriminant::JustifyItems as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PlaceItems(..) => Some((
                KnownPropertyDiscriminant::PlaceItems as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::RowGap(..) => {
                Some((KnownPropertyDiscriminant::RowGap as u32, VendorPrefix::NONE))
            }
            DeclarationData::ColumnGap(..) => Some((
                KnownPropertyDiscriminant::ColumnGap as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Gap(..) => {
                Some((KnownPropertyDiscriminant::Gap as u32, VendorPrefix::NONE))
            }
            DeclarationData::ColumnRule(_, prefix) => {
                Some((KnownPropertyDiscriminant::ColumnRule as u32, prefix))
            }
            DeclarationData::ColumnWidth(_, prefix) => {
                Some((KnownPropertyDiscriminant::ColumnWidth as u32, prefix))
            }
            DeclarationData::ColumnCount(_, prefix) => {
                Some((KnownPropertyDiscriminant::ColumnCount as u32, prefix))
            }
            DeclarationData::Columns(_, prefix) => {
                Some((KnownPropertyDiscriminant::Columns as u32, prefix))
            }
            DeclarationData::GridColumnGap(..) => Some((
                KnownPropertyDiscriminant::GridColumnGap as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridRowGap(..) => Some((
                KnownPropertyDiscriminant::GridRowGap as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BoxOrient(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxOrient as u32, prefix))
            }
            DeclarationData::BoxDirection(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxDirection as u32, prefix))
            }
            DeclarationData::BoxOrdinalGroup(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxOrdinalGroup as u32, prefix))
            }
            DeclarationData::BoxAlign(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxAlign as u32, prefix))
            }
            DeclarationData::BoxFlex(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxFlex as u32, prefix))
            }
            DeclarationData::BoxFlexGroup(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxFlexGroup as u32, prefix))
            }
            DeclarationData::BoxPack(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxPack as u32, prefix))
            }
            DeclarationData::BoxLines(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxLines as u32, prefix))
            }
            DeclarationData::FlexPack(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexPack as u32, prefix))
            }
            DeclarationData::FlexOrder(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexOrder as u32, prefix))
            }
            DeclarationData::FlexAlign(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexAlign as u32, prefix))
            }
            DeclarationData::FlexItemAlign(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexItemAlign as u32, prefix))
            }
            DeclarationData::FlexLinePack(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexLinePack as u32, prefix))
            }
            DeclarationData::FlexPositive(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexPositive as u32, prefix))
            }
            DeclarationData::FlexNegative(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexNegative as u32, prefix))
            }
            DeclarationData::FlexPreferredSize(_, prefix) => {
                Some((KnownPropertyDiscriminant::FlexPreferredSize as u32, prefix))
            }
            DeclarationData::GridTemplateColumns(..) => Some((
                KnownPropertyDiscriminant::GridTemplateColumns as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridTemplateRows(..) => Some((
                KnownPropertyDiscriminant::GridTemplateRows as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridAutoColumns(..) => Some((
                KnownPropertyDiscriminant::GridAutoColumns as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridAutoRows(..) => Some((
                KnownPropertyDiscriminant::GridAutoRows as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridAutoFlow(..) => Some((
                KnownPropertyDiscriminant::GridAutoFlow as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridTemplateAreas(..) => Some((
                KnownPropertyDiscriminant::GridTemplateAreas as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridTemplate(..) => Some((
                KnownPropertyDiscriminant::GridTemplate as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Grid(..) => {
                Some((KnownPropertyDiscriminant::Grid as u32, VendorPrefix::NONE))
            }
            DeclarationData::GridRowStart(..) => Some((
                KnownPropertyDiscriminant::GridRowStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridRowEnd(..) => Some((
                KnownPropertyDiscriminant::GridRowEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridColumnStart(..) => Some((
                KnownPropertyDiscriminant::GridColumnStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridColumnEnd(..) => Some((
                KnownPropertyDiscriminant::GridColumnEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridRow(..) => Some((
                KnownPropertyDiscriminant::GridRow as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridColumn(..) => Some((
                KnownPropertyDiscriminant::GridColumn as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::GridArea(..) => Some((
                KnownPropertyDiscriminant::GridArea as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginTop(..) => Some((
                KnownPropertyDiscriminant::MarginTop as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginBottom(..) => Some((
                KnownPropertyDiscriminant::MarginBottom as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginLeft(..) => Some((
                KnownPropertyDiscriminant::MarginLeft as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginRight(..) => Some((
                KnownPropertyDiscriminant::MarginRight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginBlockStart(..) => Some((
                KnownPropertyDiscriminant::MarginBlockStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginBlockEnd(..) => Some((
                KnownPropertyDiscriminant::MarginBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginInlineStart(..) => Some((
                KnownPropertyDiscriminant::MarginInlineStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginInlineEnd(..) => Some((
                KnownPropertyDiscriminant::MarginInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginBlock(..) => Some((
                KnownPropertyDiscriminant::MarginBlock as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarginInline(..) => Some((
                KnownPropertyDiscriminant::MarginInline as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Margin(..) => {
                Some((KnownPropertyDiscriminant::Margin as u32, VendorPrefix::NONE))
            }
            DeclarationData::PaddingTop(..) => Some((
                KnownPropertyDiscriminant::PaddingTop as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingBottom(..) => Some((
                KnownPropertyDiscriminant::PaddingBottom as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingLeft(..) => Some((
                KnownPropertyDiscriminant::PaddingLeft as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingRight(..) => Some((
                KnownPropertyDiscriminant::PaddingRight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingBlockStart(..) => Some((
                KnownPropertyDiscriminant::PaddingBlockStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingBlockEnd(..) => Some((
                KnownPropertyDiscriminant::PaddingBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingInlineStart(..) => Some((
                KnownPropertyDiscriminant::PaddingInlineStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingInlineEnd(..) => Some((
                KnownPropertyDiscriminant::PaddingInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingBlock(..) => Some((
                KnownPropertyDiscriminant::PaddingBlock as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PaddingInline(..) => Some((
                KnownPropertyDiscriminant::PaddingInline as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Padding(..) => Some((
                KnownPropertyDiscriminant::Padding as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginTop(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginTop as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginBottom(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginBottom as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginLeft(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginLeft as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginRight(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginRight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginBlockStart(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginBlockStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginBlockEnd(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginInlineStart(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginInlineStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginInlineEnd(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginBlock(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginBlock as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMarginInline(..) => Some((
                KnownPropertyDiscriminant::ScrollMarginInline as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollMargin(..) => Some((
                KnownPropertyDiscriminant::ScrollMargin as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingTop(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingTop as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingBottom(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingBottom as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingLeft(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingLeft as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingRight(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingRight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingBlockStart(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingBlockStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingBlockEnd(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingBlockEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingInlineStart(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingInlineStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingInlineEnd(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingInlineEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingBlock(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingBlock as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPaddingInline(..) => Some((
                KnownPropertyDiscriminant::ScrollPaddingInline as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollPadding(..) => Some((
                KnownPropertyDiscriminant::ScrollPadding as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FontWeight(..) => Some((
                KnownPropertyDiscriminant::FontWeight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FontSize(..) => Some((
                KnownPropertyDiscriminant::FontSize as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FontStretch(..) => Some((
                KnownPropertyDiscriminant::FontStretch as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FontFamily(..) => Some((
                KnownPropertyDiscriminant::FontFamily as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FontStyle(..) => Some((
                KnownPropertyDiscriminant::FontStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FontVariantCaps(..) => Some((
                KnownPropertyDiscriminant::FontVariantCaps as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::LineHeight(..) => Some((
                KnownPropertyDiscriminant::LineHeight as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Font(..) => {
                Some((KnownPropertyDiscriminant::Font as u32, VendorPrefix::NONE))
            }
            DeclarationData::VerticalAlign(..) => Some((
                KnownPropertyDiscriminant::VerticalAlign as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FontPalette(..) => Some((
                KnownPropertyDiscriminant::FontPalette as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TransitionProperty(_, prefix) => {
                Some((KnownPropertyDiscriminant::TransitionProperty as u32, prefix))
            }
            DeclarationData::TransitionDuration(_, prefix) => {
                Some((KnownPropertyDiscriminant::TransitionDuration as u32, prefix))
            }
            DeclarationData::TransitionDelay(_, prefix) => {
                Some((KnownPropertyDiscriminant::TransitionDelay as u32, prefix))
            }
            DeclarationData::TransitionTimingFunction(_, prefix) => Some((
                KnownPropertyDiscriminant::TransitionTimingFunction as u32,
                prefix,
            )),
            DeclarationData::Transition(_, prefix) => {
                Some((KnownPropertyDiscriminant::Transition as u32, prefix))
            }
            DeclarationData::AnimationName(_, prefix) => {
                Some((KnownPropertyDiscriminant::AnimationName as u32, prefix))
            }
            DeclarationData::AnimationDuration(_, prefix) => {
                Some((KnownPropertyDiscriminant::AnimationDuration as u32, prefix))
            }
            DeclarationData::AnimationTimingFunction(_, prefix) => Some((
                KnownPropertyDiscriminant::AnimationTimingFunction as u32,
                prefix,
            )),
            DeclarationData::AnimationIterationCount(_, prefix) => Some((
                KnownPropertyDiscriminant::AnimationIterationCount as u32,
                prefix,
            )),
            DeclarationData::AnimationDirection(_, prefix) => {
                Some((KnownPropertyDiscriminant::AnimationDirection as u32, prefix))
            }
            DeclarationData::AnimationPlayState(_, prefix) => {
                Some((KnownPropertyDiscriminant::AnimationPlayState as u32, prefix))
            }
            DeclarationData::AnimationDelay(_, prefix) => {
                Some((KnownPropertyDiscriminant::AnimationDelay as u32, prefix))
            }
            DeclarationData::AnimationFillMode(_, prefix) => {
                Some((KnownPropertyDiscriminant::AnimationFillMode as u32, prefix))
            }
            DeclarationData::AnimationComposition(..) => Some((
                KnownPropertyDiscriminant::AnimationComposition as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::AnimationTimeline(..) => Some((
                KnownPropertyDiscriminant::AnimationTimeline as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::AnimationRangeStart(..) => Some((
                KnownPropertyDiscriminant::AnimationRangeStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::AnimationRangeEnd(..) => Some((
                KnownPropertyDiscriminant::AnimationRangeEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::AnimationRange(..) => Some((
                KnownPropertyDiscriminant::AnimationRange as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Animation(_, prefix) => {
                Some((KnownPropertyDiscriminant::Animation as u32, prefix))
            }
            DeclarationData::Transform(_, prefix) => {
                Some((KnownPropertyDiscriminant::Transform as u32, prefix))
            }
            DeclarationData::TransformOrigin(_, prefix) => {
                Some((KnownPropertyDiscriminant::TransformOrigin as u32, prefix))
            }
            DeclarationData::TransformStyle(_, prefix) => {
                Some((KnownPropertyDiscriminant::TransformStyle as u32, prefix))
            }
            DeclarationData::TransformBox(..) => Some((
                KnownPropertyDiscriminant::TransformBox as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BackfaceVisibility(_, prefix) => {
                Some((KnownPropertyDiscriminant::BackfaceVisibility as u32, prefix))
            }
            DeclarationData::Perspective(_, prefix) => {
                Some((KnownPropertyDiscriminant::Perspective as u32, prefix))
            }
            DeclarationData::PerspectiveOrigin(_, prefix) => {
                Some((KnownPropertyDiscriminant::PerspectiveOrigin as u32, prefix))
            }
            DeclarationData::Translate(..) => Some((
                KnownPropertyDiscriminant::Translate as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Rotate(..) => {
                Some((KnownPropertyDiscriminant::Rotate as u32, VendorPrefix::NONE))
            }
            DeclarationData::Scale(..) => {
                Some((KnownPropertyDiscriminant::Scale as u32, VendorPrefix::NONE))
            }
            DeclarationData::TextTransform(..) => Some((
                KnownPropertyDiscriminant::TextTransform as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Content(..) => Some((
                KnownPropertyDiscriminant::Content as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::WhiteSpace(..) => Some((
                KnownPropertyDiscriminant::WhiteSpace as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TabSize(_, prefix) => {
                Some((KnownPropertyDiscriminant::TabSize as u32, prefix))
            }
            DeclarationData::WordBreak(..) => Some((
                KnownPropertyDiscriminant::WordBreak as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::LineBreak(..) => Some((
                KnownPropertyDiscriminant::LineBreak as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Hyphens(_, prefix) => {
                Some((KnownPropertyDiscriminant::Hyphens as u32, prefix))
            }
            DeclarationData::OverflowWrap(..) => Some((
                KnownPropertyDiscriminant::OverflowWrap as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::WordWrap(..) => Some((
                KnownPropertyDiscriminant::WordWrap as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TextAlign(..) => Some((
                KnownPropertyDiscriminant::TextAlign as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TextAlignLast(_, prefix) => {
                Some((KnownPropertyDiscriminant::TextAlignLast as u32, prefix))
            }
            DeclarationData::TextJustify(..) => Some((
                KnownPropertyDiscriminant::TextJustify as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::WordSpacing(..) => Some((
                KnownPropertyDiscriminant::WordSpacing as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::LetterSpacing(..) => Some((
                KnownPropertyDiscriminant::LetterSpacing as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TextIndent(..) => Some((
                KnownPropertyDiscriminant::TextIndent as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TextDecorationLine(_, prefix) => {
                Some((KnownPropertyDiscriminant::TextDecorationLine as u32, prefix))
            }
            DeclarationData::TextDecorationStyle(_, prefix) => Some((
                KnownPropertyDiscriminant::TextDecorationStyle as u32,
                prefix,
            )),
            DeclarationData::TextDecorationColor(_, prefix) => Some((
                KnownPropertyDiscriminant::TextDecorationColor as u32,
                prefix,
            )),
            DeclarationData::TextDecorationThickness(..) => Some((
                KnownPropertyDiscriminant::TextDecorationThickness as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TextDecoration(_, prefix) => {
                Some((KnownPropertyDiscriminant::TextDecoration as u32, prefix))
            }
            DeclarationData::TextDecorationSkipInk(_, prefix) => Some((
                KnownPropertyDiscriminant::TextDecorationSkipInk as u32,
                prefix,
            )),
            DeclarationData::TextEmphasisStyle(_, prefix) => {
                Some((KnownPropertyDiscriminant::TextEmphasisStyle as u32, prefix))
            }
            DeclarationData::TextEmphasisColor(_, prefix) => {
                Some((KnownPropertyDiscriminant::TextEmphasisColor as u32, prefix))
            }
            DeclarationData::TextEmphasis(_, prefix) => {
                Some((KnownPropertyDiscriminant::TextEmphasis as u32, prefix))
            }
            DeclarationData::TextEmphasisPosition(_, prefix) => Some((
                KnownPropertyDiscriminant::TextEmphasisPosition as u32,
                prefix,
            )),
            DeclarationData::TextShadow(..) => Some((
                KnownPropertyDiscriminant::TextShadow as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TextSizeAdjust(_, prefix) => {
                Some((KnownPropertyDiscriminant::TextSizeAdjust as u32, prefix))
            }
            DeclarationData::Direction(..) => Some((
                KnownPropertyDiscriminant::Direction as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::UnicodeBidi(..) => Some((
                KnownPropertyDiscriminant::UnicodeBidi as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::BoxDecorationBreak(_, prefix) => {
                Some((KnownPropertyDiscriminant::BoxDecorationBreak as u32, prefix))
            }
            DeclarationData::Resize(..) => {
                Some((KnownPropertyDiscriminant::Resize as u32, VendorPrefix::NONE))
            }
            DeclarationData::PointerEvents(..) => Some((
                KnownPropertyDiscriminant::PointerEvents as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Float(..) => {
                Some((KnownPropertyDiscriminant::Float as u32, VendorPrefix::NONE))
            }
            DeclarationData::Clear(..) => {
                Some((KnownPropertyDiscriminant::Clear as u32, VendorPrefix::NONE))
            }
            DeclarationData::TouchAction(..) => Some((
                KnownPropertyDiscriminant::TouchAction as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ScrollBehavior(..) => Some((
                KnownPropertyDiscriminant::ScrollBehavior as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Cursor(..) => {
                Some((KnownPropertyDiscriminant::Cursor as u32, VendorPrefix::NONE))
            }
            DeclarationData::CaretColor(..) => Some((
                KnownPropertyDiscriminant::CaretColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::CaretShape(..) => Some((
                KnownPropertyDiscriminant::CaretShape as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Caret(..) => {
                Some((KnownPropertyDiscriminant::Caret as u32, VendorPrefix::NONE))
            }
            DeclarationData::UserSelect(_, prefix) => {
                Some((KnownPropertyDiscriminant::UserSelect as u32, prefix))
            }
            DeclarationData::AccentColor(..) => Some((
                KnownPropertyDiscriminant::AccentColor as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Appearance(_, prefix) => {
                Some((KnownPropertyDiscriminant::Appearance as u32, prefix))
            }
            DeclarationData::ListStyleType(..) => Some((
                KnownPropertyDiscriminant::ListStyleType as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ListStyleImage(..) => Some((
                KnownPropertyDiscriminant::ListStyleImage as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ListStylePosition(..) => Some((
                KnownPropertyDiscriminant::ListStylePosition as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ListStyle(..) => Some((
                KnownPropertyDiscriminant::ListStyle as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarkerSide(..) => Some((
                KnownPropertyDiscriminant::MarkerSide as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Composes(..) => Some((
                KnownPropertyDiscriminant::Composes as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Fill(..) => {
                Some((KnownPropertyDiscriminant::Fill as u32, VendorPrefix::NONE))
            }
            DeclarationData::FillRule(..) => Some((
                KnownPropertyDiscriminant::FillRule as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::FillOpacity(..) => Some((
                KnownPropertyDiscriminant::FillOpacity as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Stroke(..) => {
                Some((KnownPropertyDiscriminant::Stroke as u32, VendorPrefix::NONE))
            }
            DeclarationData::StrokeOpacity(..) => Some((
                KnownPropertyDiscriminant::StrokeOpacity as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::StrokeWidth(..) => Some((
                KnownPropertyDiscriminant::StrokeWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::StrokeLinecap(..) => Some((
                KnownPropertyDiscriminant::StrokeLinecap as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::StrokeLinejoin(..) => Some((
                KnownPropertyDiscriminant::StrokeLinejoin as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::StrokeMiterlimit(..) => Some((
                KnownPropertyDiscriminant::StrokeMiterlimit as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::StrokeDasharray(..) => Some((
                KnownPropertyDiscriminant::StrokeDasharray as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::StrokeDashoffset(..) => Some((
                KnownPropertyDiscriminant::StrokeDashoffset as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarkerStart(..) => Some((
                KnownPropertyDiscriminant::MarkerStart as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarkerMid(..) => Some((
                KnownPropertyDiscriminant::MarkerMid as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MarkerEnd(..) => Some((
                KnownPropertyDiscriminant::MarkerEnd as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Marker(..) => {
                Some((KnownPropertyDiscriminant::Marker as u32, VendorPrefix::NONE))
            }
            DeclarationData::ColorInterpolation(..) => Some((
                KnownPropertyDiscriminant::ColorInterpolation as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ColorInterpolationFilters(..) => Some((
                KnownPropertyDiscriminant::ColorInterpolationFilters as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ColorRendering(..) => Some((
                KnownPropertyDiscriminant::ColorRendering as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ShapeRendering(..) => Some((
                KnownPropertyDiscriminant::ShapeRendering as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::TextRendering(..) => Some((
                KnownPropertyDiscriminant::TextRendering as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ImageRendering(..) => Some((
                KnownPropertyDiscriminant::ImageRendering as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ClipPath(_, prefix) => {
                Some((KnownPropertyDiscriminant::ClipPath as u32, prefix))
            }
            DeclarationData::ClipRule(..) => Some((
                KnownPropertyDiscriminant::ClipRule as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskImage(_, prefix) => {
                Some((KnownPropertyDiscriminant::MaskImage as u32, prefix))
            }
            DeclarationData::MaskMode(..) => Some((
                KnownPropertyDiscriminant::MaskMode as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskRepeat(_, prefix) => {
                Some((KnownPropertyDiscriminant::MaskRepeat as u32, prefix))
            }
            DeclarationData::MaskPositionX(..) => Some((
                KnownPropertyDiscriminant::MaskPositionX as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskPositionY(..) => Some((
                KnownPropertyDiscriminant::MaskPositionY as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskPosition(_, prefix) => {
                Some((KnownPropertyDiscriminant::MaskPosition as u32, prefix))
            }
            DeclarationData::MaskClip(_, prefix) => {
                Some((KnownPropertyDiscriminant::MaskClip as u32, prefix))
            }
            DeclarationData::MaskOrigin(_, prefix) => {
                Some((KnownPropertyDiscriminant::MaskOrigin as u32, prefix))
            }
            DeclarationData::MaskSize(_, prefix) => {
                Some((KnownPropertyDiscriminant::MaskSize as u32, prefix))
            }
            DeclarationData::MaskComposite(..) => Some((
                KnownPropertyDiscriminant::MaskComposite as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskType(..) => Some((
                KnownPropertyDiscriminant::MaskType as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Mask(_, prefix) => {
                Some((KnownPropertyDiscriminant::Mask as u32, prefix))
            }
            DeclarationData::MaskBorderSource(..) => Some((
                KnownPropertyDiscriminant::MaskBorderSource as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskBorderMode(..) => Some((
                KnownPropertyDiscriminant::MaskBorderMode as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskBorderSlice(..) => Some((
                KnownPropertyDiscriminant::MaskBorderSlice as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskBorderWidth(..) => Some((
                KnownPropertyDiscriminant::MaskBorderWidth as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskBorderOutset(..) => Some((
                KnownPropertyDiscriminant::MaskBorderOutset as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskBorderRepeat(..) => Some((
                KnownPropertyDiscriminant::MaskBorderRepeat as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::MaskBorder(..) => Some((
                KnownPropertyDiscriminant::MaskBorder as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::WebKitMaskComposite(..) => Some((
                KnownPropertyDiscriminant::WebKitMaskComposite as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::WebKitMaskSourceType(_, prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskSourceType as u32,
                prefix,
            )),
            DeclarationData::WebKitMaskBoxImage(_, prefix) => {
                Some((KnownPropertyDiscriminant::WebKitMaskBoxImage as u32, prefix))
            }
            DeclarationData::WebKitMaskBoxImageSource(_, prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageSource as u32,
                prefix,
            )),
            DeclarationData::WebKitMaskBoxImageSlice(_, prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageSlice as u32,
                prefix,
            )),
            DeclarationData::WebKitMaskBoxImageWidth(_, prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageWidth as u32,
                prefix,
            )),
            DeclarationData::WebKitMaskBoxImageOutset(_, prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageOutset as u32,
                prefix,
            )),
            DeclarationData::WebKitMaskBoxImageRepeat(_, prefix) => Some((
                KnownPropertyDiscriminant::WebKitMaskBoxImageRepeat as u32,
                prefix,
            )),
            DeclarationData::Filter(_, prefix) => {
                Some((KnownPropertyDiscriminant::Filter as u32, prefix))
            }
            DeclarationData::BackdropFilter(_, prefix) => {
                Some((KnownPropertyDiscriminant::BackdropFilter as u32, prefix))
            }
            DeclarationData::MixBlendMode(..) => Some((
                KnownPropertyDiscriminant::MixBlendMode as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ZIndex(..) => {
                Some((KnownPropertyDiscriminant::ZIndex as u32, VendorPrefix::NONE))
            }
            DeclarationData::ContainerType(..) => Some((
                KnownPropertyDiscriminant::ContainerType as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ContainerName(..) => Some((
                KnownPropertyDiscriminant::ContainerName as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::Container(..) => Some((
                KnownPropertyDiscriminant::Container as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ViewTransitionName(..) => Some((
                KnownPropertyDiscriminant::ViewTransitionName as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ViewTransitionClass(..) => Some((
                KnownPropertyDiscriminant::ViewTransitionClass as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ViewTransitionGroup(..) => Some((
                KnownPropertyDiscriminant::ViewTransitionGroup as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::ColorScheme(..) => Some((
                KnownPropertyDiscriminant::ColorScheme as u32,
                VendorPrefix::NONE,
            )),
            DeclarationData::PrintColorAdjust(_, prefix) => {
                Some((KnownPropertyDiscriminant::PrintColorAdjust as u32, prefix))
            }
            DeclarationData::CSSWide(property_id, _) => property_id.known_id_and_prefix(),
            DeclarationData::Unparsed(value) => value.property_id(ast).known_id_and_prefix(),
            DeclarationData::Custom(_) | DeclarationData::Tombstone => None,
        }
    }
    /// Returns the typed identity of this declaration.
    #[inline]
    pub fn property_id(self, ast: &Ast<'ast>) -> Option<PropertyId<'ast>> {
        match self.data(ast) {
            DeclarationData::All(..) => Some(PropertyId::All),
            DeclarationData::BackgroundColor(..) => Some(PropertyId::BackgroundColor),
            DeclarationData::BackgroundImage(..) => Some(PropertyId::BackgroundImage),
            DeclarationData::BackgroundPositionX(..) => Some(PropertyId::BackgroundPositionX),
            DeclarationData::BackgroundPositionY(..) => Some(PropertyId::BackgroundPositionY),
            DeclarationData::BackgroundPosition(..) => Some(PropertyId::BackgroundPosition),
            DeclarationData::BackgroundSize(..) => Some(PropertyId::BackgroundSize),
            DeclarationData::BackgroundRepeat(..) => Some(PropertyId::BackgroundRepeat),
            DeclarationData::BackgroundAttachment(..) => Some(PropertyId::BackgroundAttachment),
            DeclarationData::BackgroundClip(_, prefix) => Some(PropertyId::BackgroundClip(prefix)),
            DeclarationData::BackgroundOrigin(..) => Some(PropertyId::BackgroundOrigin),
            DeclarationData::Background(..) => Some(PropertyId::Background),
            DeclarationData::BoxShadow(_, prefix) => Some(PropertyId::BoxShadow(prefix)),
            DeclarationData::Opacity(..) => Some(PropertyId::Opacity),
            DeclarationData::Color(..) => Some(PropertyId::Color),
            DeclarationData::Display(..) => Some(PropertyId::Display),
            DeclarationData::Visibility(..) => Some(PropertyId::Visibility),
            DeclarationData::Width(..) => Some(PropertyId::Width),
            DeclarationData::Height(..) => Some(PropertyId::Height),
            DeclarationData::MinWidth(..) => Some(PropertyId::MinWidth),
            DeclarationData::MinHeight(..) => Some(PropertyId::MinHeight),
            DeclarationData::MaxWidth(..) => Some(PropertyId::MaxWidth),
            DeclarationData::MaxHeight(..) => Some(PropertyId::MaxHeight),
            DeclarationData::BlockSize(..) => Some(PropertyId::BlockSize),
            DeclarationData::InlineSize(..) => Some(PropertyId::InlineSize),
            DeclarationData::MinBlockSize(..) => Some(PropertyId::MinBlockSize),
            DeclarationData::MinInlineSize(..) => Some(PropertyId::MinInlineSize),
            DeclarationData::MaxBlockSize(..) => Some(PropertyId::MaxBlockSize),
            DeclarationData::MaxInlineSize(..) => Some(PropertyId::MaxInlineSize),
            DeclarationData::BoxSizing(_, prefix) => Some(PropertyId::BoxSizing(prefix)),
            DeclarationData::AspectRatio(..) => Some(PropertyId::AspectRatio),
            DeclarationData::Overflow(..) => Some(PropertyId::Overflow),
            DeclarationData::OverflowX(..) => Some(PropertyId::OverflowX),
            DeclarationData::OverflowY(..) => Some(PropertyId::OverflowY),
            DeclarationData::TextOverflow(_, prefix) => Some(PropertyId::TextOverflow(prefix)),
            DeclarationData::ObjectFit(..) => Some(PropertyId::ObjectFit),
            DeclarationData::ObjectPosition(..) => Some(PropertyId::ObjectPosition),
            DeclarationData::ScrollbarColor(..) => Some(PropertyId::ScrollbarColor),
            DeclarationData::Position(..) => Some(PropertyId::Position),
            DeclarationData::Top(..) => Some(PropertyId::Top),
            DeclarationData::Bottom(..) => Some(PropertyId::Bottom),
            DeclarationData::Left(..) => Some(PropertyId::Left),
            DeclarationData::Right(..) => Some(PropertyId::Right),
            DeclarationData::InsetBlockStart(..) => Some(PropertyId::InsetBlockStart),
            DeclarationData::InsetBlockEnd(..) => Some(PropertyId::InsetBlockEnd),
            DeclarationData::InsetInlineStart(..) => Some(PropertyId::InsetInlineStart),
            DeclarationData::InsetInlineEnd(..) => Some(PropertyId::InsetInlineEnd),
            DeclarationData::InsetBlock(..) => Some(PropertyId::InsetBlock),
            DeclarationData::InsetInline(..) => Some(PropertyId::InsetInline),
            DeclarationData::Inset(..) => Some(PropertyId::Inset),
            DeclarationData::BorderSpacing(..) => Some(PropertyId::BorderSpacing),
            DeclarationData::BorderTopColor(..) => Some(PropertyId::BorderTopColor),
            DeclarationData::BorderBottomColor(..) => Some(PropertyId::BorderBottomColor),
            DeclarationData::BorderLeftColor(..) => Some(PropertyId::BorderLeftColor),
            DeclarationData::BorderRightColor(..) => Some(PropertyId::BorderRightColor),
            DeclarationData::BorderBlockStartColor(..) => Some(PropertyId::BorderBlockStartColor),
            DeclarationData::BorderBlockEndColor(..) => Some(PropertyId::BorderBlockEndColor),
            DeclarationData::BorderInlineStartColor(..) => Some(PropertyId::BorderInlineStartColor),
            DeclarationData::BorderInlineEndColor(..) => Some(PropertyId::BorderInlineEndColor),
            DeclarationData::BorderTopStyle(..) => Some(PropertyId::BorderTopStyle),
            DeclarationData::BorderBottomStyle(..) => Some(PropertyId::BorderBottomStyle),
            DeclarationData::BorderLeftStyle(..) => Some(PropertyId::BorderLeftStyle),
            DeclarationData::BorderRightStyle(..) => Some(PropertyId::BorderRightStyle),
            DeclarationData::BorderBlockStartStyle(..) => Some(PropertyId::BorderBlockStartStyle),
            DeclarationData::BorderBlockEndStyle(..) => Some(PropertyId::BorderBlockEndStyle),
            DeclarationData::BorderInlineStartStyle(..) => Some(PropertyId::BorderInlineStartStyle),
            DeclarationData::BorderInlineEndStyle(..) => Some(PropertyId::BorderInlineEndStyle),
            DeclarationData::BorderTopWidth(..) => Some(PropertyId::BorderTopWidth),
            DeclarationData::BorderBottomWidth(..) => Some(PropertyId::BorderBottomWidth),
            DeclarationData::BorderLeftWidth(..) => Some(PropertyId::BorderLeftWidth),
            DeclarationData::BorderRightWidth(..) => Some(PropertyId::BorderRightWidth),
            DeclarationData::BorderBlockStartWidth(..) => Some(PropertyId::BorderBlockStartWidth),
            DeclarationData::BorderBlockEndWidth(..) => Some(PropertyId::BorderBlockEndWidth),
            DeclarationData::BorderInlineStartWidth(..) => Some(PropertyId::BorderInlineStartWidth),
            DeclarationData::BorderInlineEndWidth(..) => Some(PropertyId::BorderInlineEndWidth),
            DeclarationData::BorderTopLeftRadius(_, prefix) => {
                Some(PropertyId::BorderTopLeftRadius(prefix))
            }
            DeclarationData::BorderTopRightRadius(_, prefix) => {
                Some(PropertyId::BorderTopRightRadius(prefix))
            }
            DeclarationData::BorderBottomLeftRadius(_, prefix) => {
                Some(PropertyId::BorderBottomLeftRadius(prefix))
            }
            DeclarationData::BorderBottomRightRadius(_, prefix) => {
                Some(PropertyId::BorderBottomRightRadius(prefix))
            }
            DeclarationData::BorderStartStartRadius(..) => Some(PropertyId::BorderStartStartRadius),
            DeclarationData::BorderStartEndRadius(..) => Some(PropertyId::BorderStartEndRadius),
            DeclarationData::BorderEndStartRadius(..) => Some(PropertyId::BorderEndStartRadius),
            DeclarationData::BorderEndEndRadius(..) => Some(PropertyId::BorderEndEndRadius),
            DeclarationData::BorderRadius(_, prefix) => Some(PropertyId::BorderRadius(prefix)),
            DeclarationData::BorderImageSource(..) => Some(PropertyId::BorderImageSource),
            DeclarationData::BorderImageOutset(..) => Some(PropertyId::BorderImageOutset),
            DeclarationData::BorderImageRepeat(..) => Some(PropertyId::BorderImageRepeat),
            DeclarationData::BorderImageWidth(..) => Some(PropertyId::BorderImageWidth),
            DeclarationData::BorderImageSlice(..) => Some(PropertyId::BorderImageSlice),
            DeclarationData::BorderImage(_, prefix) => Some(PropertyId::BorderImage(prefix)),
            DeclarationData::BorderColor(..) => Some(PropertyId::BorderColor),
            DeclarationData::BorderStyle(..) => Some(PropertyId::BorderStyle),
            DeclarationData::BorderWidth(..) => Some(PropertyId::BorderWidth),
            DeclarationData::BorderBlockColor(..) => Some(PropertyId::BorderBlockColor),
            DeclarationData::BorderBlockStyle(..) => Some(PropertyId::BorderBlockStyle),
            DeclarationData::BorderBlockWidth(..) => Some(PropertyId::BorderBlockWidth),
            DeclarationData::BorderInlineColor(..) => Some(PropertyId::BorderInlineColor),
            DeclarationData::BorderInlineStyle(..) => Some(PropertyId::BorderInlineStyle),
            DeclarationData::BorderInlineWidth(..) => Some(PropertyId::BorderInlineWidth),
            DeclarationData::Border(..) => Some(PropertyId::Border),
            DeclarationData::BorderTop(..) => Some(PropertyId::BorderTop),
            DeclarationData::BorderBottom(..) => Some(PropertyId::BorderBottom),
            DeclarationData::BorderLeft(..) => Some(PropertyId::BorderLeft),
            DeclarationData::BorderRight(..) => Some(PropertyId::BorderRight),
            DeclarationData::BorderBlock(..) => Some(PropertyId::BorderBlock),
            DeclarationData::BorderBlockStart(..) => Some(PropertyId::BorderBlockStart),
            DeclarationData::BorderBlockEnd(..) => Some(PropertyId::BorderBlockEnd),
            DeclarationData::BorderInline(..) => Some(PropertyId::BorderInline),
            DeclarationData::BorderInlineStart(..) => Some(PropertyId::BorderInlineStart),
            DeclarationData::BorderInlineEnd(..) => Some(PropertyId::BorderInlineEnd),
            DeclarationData::Outline(..) => Some(PropertyId::Outline),
            DeclarationData::OutlineColor(..) => Some(PropertyId::OutlineColor),
            DeclarationData::OutlineStyle(..) => Some(PropertyId::OutlineStyle),
            DeclarationData::OutlineWidth(..) => Some(PropertyId::OutlineWidth),
            DeclarationData::FlexDirection(_, prefix) => Some(PropertyId::FlexDirection(prefix)),
            DeclarationData::FlexWrap(_, prefix) => Some(PropertyId::FlexWrap(prefix)),
            DeclarationData::FlexFlow(_, prefix) => Some(PropertyId::FlexFlow(prefix)),
            DeclarationData::FlexGrow(_, prefix) => Some(PropertyId::FlexGrow(prefix)),
            DeclarationData::FlexShrink(_, prefix) => Some(PropertyId::FlexShrink(prefix)),
            DeclarationData::FlexBasis(_, prefix) => Some(PropertyId::FlexBasis(prefix)),
            DeclarationData::Flex(_, prefix) => Some(PropertyId::Flex(prefix)),
            DeclarationData::Order(_, prefix) => Some(PropertyId::Order(prefix)),
            DeclarationData::AlignContent(_, prefix) => Some(PropertyId::AlignContent(prefix)),
            DeclarationData::JustifyContent(_, prefix) => Some(PropertyId::JustifyContent(prefix)),
            DeclarationData::PlaceContent(..) => Some(PropertyId::PlaceContent),
            DeclarationData::AlignSelf(_, prefix) => Some(PropertyId::AlignSelf(prefix)),
            DeclarationData::JustifySelf(..) => Some(PropertyId::JustifySelf),
            DeclarationData::PlaceSelf(..) => Some(PropertyId::PlaceSelf),
            DeclarationData::AlignItems(_, prefix) => Some(PropertyId::AlignItems(prefix)),
            DeclarationData::JustifyItems(..) => Some(PropertyId::JustifyItems),
            DeclarationData::PlaceItems(..) => Some(PropertyId::PlaceItems),
            DeclarationData::RowGap(..) => Some(PropertyId::RowGap),
            DeclarationData::ColumnGap(..) => Some(PropertyId::ColumnGap),
            DeclarationData::Gap(..) => Some(PropertyId::Gap),
            DeclarationData::ColumnRule(_, prefix) => Some(PropertyId::ColumnRule(prefix)),
            DeclarationData::ColumnWidth(_, prefix) => Some(PropertyId::ColumnWidth(prefix)),
            DeclarationData::ColumnCount(_, prefix) => Some(PropertyId::ColumnCount(prefix)),
            DeclarationData::Columns(_, prefix) => Some(PropertyId::Columns(prefix)),
            DeclarationData::GridColumnGap(..) => Some(PropertyId::GridColumnGap),
            DeclarationData::GridRowGap(..) => Some(PropertyId::GridRowGap),
            DeclarationData::BoxOrient(_, prefix) => Some(PropertyId::BoxOrient(prefix)),
            DeclarationData::BoxDirection(_, prefix) => Some(PropertyId::BoxDirection(prefix)),
            DeclarationData::BoxOrdinalGroup(_, prefix) => {
                Some(PropertyId::BoxOrdinalGroup(prefix))
            }
            DeclarationData::BoxAlign(_, prefix) => Some(PropertyId::BoxAlign(prefix)),
            DeclarationData::BoxFlex(_, prefix) => Some(PropertyId::BoxFlex(prefix)),
            DeclarationData::BoxFlexGroup(_, prefix) => Some(PropertyId::BoxFlexGroup(prefix)),
            DeclarationData::BoxPack(_, prefix) => Some(PropertyId::BoxPack(prefix)),
            DeclarationData::BoxLines(_, prefix) => Some(PropertyId::BoxLines(prefix)),
            DeclarationData::FlexPack(_, prefix) => Some(PropertyId::FlexPack(prefix)),
            DeclarationData::FlexOrder(_, prefix) => Some(PropertyId::FlexOrder(prefix)),
            DeclarationData::FlexAlign(_, prefix) => Some(PropertyId::FlexAlign(prefix)),
            DeclarationData::FlexItemAlign(_, prefix) => Some(PropertyId::FlexItemAlign(prefix)),
            DeclarationData::FlexLinePack(_, prefix) => Some(PropertyId::FlexLinePack(prefix)),
            DeclarationData::FlexPositive(_, prefix) => Some(PropertyId::FlexPositive(prefix)),
            DeclarationData::FlexNegative(_, prefix) => Some(PropertyId::FlexNegative(prefix)),
            DeclarationData::FlexPreferredSize(_, prefix) => {
                Some(PropertyId::FlexPreferredSize(prefix))
            }
            DeclarationData::GridTemplateColumns(..) => Some(PropertyId::GridTemplateColumns),
            DeclarationData::GridTemplateRows(..) => Some(PropertyId::GridTemplateRows),
            DeclarationData::GridAutoColumns(..) => Some(PropertyId::GridAutoColumns),
            DeclarationData::GridAutoRows(..) => Some(PropertyId::GridAutoRows),
            DeclarationData::GridAutoFlow(..) => Some(PropertyId::GridAutoFlow),
            DeclarationData::GridTemplateAreas(..) => Some(PropertyId::GridTemplateAreas),
            DeclarationData::GridTemplate(..) => Some(PropertyId::GridTemplate),
            DeclarationData::Grid(..) => Some(PropertyId::Grid),
            DeclarationData::GridRowStart(..) => Some(PropertyId::GridRowStart),
            DeclarationData::GridRowEnd(..) => Some(PropertyId::GridRowEnd),
            DeclarationData::GridColumnStart(..) => Some(PropertyId::GridColumnStart),
            DeclarationData::GridColumnEnd(..) => Some(PropertyId::GridColumnEnd),
            DeclarationData::GridRow(..) => Some(PropertyId::GridRow),
            DeclarationData::GridColumn(..) => Some(PropertyId::GridColumn),
            DeclarationData::GridArea(..) => Some(PropertyId::GridArea),
            DeclarationData::MarginTop(..) => Some(PropertyId::MarginTop),
            DeclarationData::MarginBottom(..) => Some(PropertyId::MarginBottom),
            DeclarationData::MarginLeft(..) => Some(PropertyId::MarginLeft),
            DeclarationData::MarginRight(..) => Some(PropertyId::MarginRight),
            DeclarationData::MarginBlockStart(..) => Some(PropertyId::MarginBlockStart),
            DeclarationData::MarginBlockEnd(..) => Some(PropertyId::MarginBlockEnd),
            DeclarationData::MarginInlineStart(..) => Some(PropertyId::MarginInlineStart),
            DeclarationData::MarginInlineEnd(..) => Some(PropertyId::MarginInlineEnd),
            DeclarationData::MarginBlock(..) => Some(PropertyId::MarginBlock),
            DeclarationData::MarginInline(..) => Some(PropertyId::MarginInline),
            DeclarationData::Margin(..) => Some(PropertyId::Margin),
            DeclarationData::PaddingTop(..) => Some(PropertyId::PaddingTop),
            DeclarationData::PaddingBottom(..) => Some(PropertyId::PaddingBottom),
            DeclarationData::PaddingLeft(..) => Some(PropertyId::PaddingLeft),
            DeclarationData::PaddingRight(..) => Some(PropertyId::PaddingRight),
            DeclarationData::PaddingBlockStart(..) => Some(PropertyId::PaddingBlockStart),
            DeclarationData::PaddingBlockEnd(..) => Some(PropertyId::PaddingBlockEnd),
            DeclarationData::PaddingInlineStart(..) => Some(PropertyId::PaddingInlineStart),
            DeclarationData::PaddingInlineEnd(..) => Some(PropertyId::PaddingInlineEnd),
            DeclarationData::PaddingBlock(..) => Some(PropertyId::PaddingBlock),
            DeclarationData::PaddingInline(..) => Some(PropertyId::PaddingInline),
            DeclarationData::Padding(..) => Some(PropertyId::Padding),
            DeclarationData::ScrollMarginTop(..) => Some(PropertyId::ScrollMarginTop),
            DeclarationData::ScrollMarginBottom(..) => Some(PropertyId::ScrollMarginBottom),
            DeclarationData::ScrollMarginLeft(..) => Some(PropertyId::ScrollMarginLeft),
            DeclarationData::ScrollMarginRight(..) => Some(PropertyId::ScrollMarginRight),
            DeclarationData::ScrollMarginBlockStart(..) => Some(PropertyId::ScrollMarginBlockStart),
            DeclarationData::ScrollMarginBlockEnd(..) => Some(PropertyId::ScrollMarginBlockEnd),
            DeclarationData::ScrollMarginInlineStart(..) => {
                Some(PropertyId::ScrollMarginInlineStart)
            }
            DeclarationData::ScrollMarginInlineEnd(..) => Some(PropertyId::ScrollMarginInlineEnd),
            DeclarationData::ScrollMarginBlock(..) => Some(PropertyId::ScrollMarginBlock),
            DeclarationData::ScrollMarginInline(..) => Some(PropertyId::ScrollMarginInline),
            DeclarationData::ScrollMargin(..) => Some(PropertyId::ScrollMargin),
            DeclarationData::ScrollPaddingTop(..) => Some(PropertyId::ScrollPaddingTop),
            DeclarationData::ScrollPaddingBottom(..) => Some(PropertyId::ScrollPaddingBottom),
            DeclarationData::ScrollPaddingLeft(..) => Some(PropertyId::ScrollPaddingLeft),
            DeclarationData::ScrollPaddingRight(..) => Some(PropertyId::ScrollPaddingRight),
            DeclarationData::ScrollPaddingBlockStart(..) => {
                Some(PropertyId::ScrollPaddingBlockStart)
            }
            DeclarationData::ScrollPaddingBlockEnd(..) => Some(PropertyId::ScrollPaddingBlockEnd),
            DeclarationData::ScrollPaddingInlineStart(..) => {
                Some(PropertyId::ScrollPaddingInlineStart)
            }
            DeclarationData::ScrollPaddingInlineEnd(..) => Some(PropertyId::ScrollPaddingInlineEnd),
            DeclarationData::ScrollPaddingBlock(..) => Some(PropertyId::ScrollPaddingBlock),
            DeclarationData::ScrollPaddingInline(..) => Some(PropertyId::ScrollPaddingInline),
            DeclarationData::ScrollPadding(..) => Some(PropertyId::ScrollPadding),
            DeclarationData::FontWeight(..) => Some(PropertyId::FontWeight),
            DeclarationData::FontSize(..) => Some(PropertyId::FontSize),
            DeclarationData::FontStretch(..) => Some(PropertyId::FontStretch),
            DeclarationData::FontFamily(..) => Some(PropertyId::FontFamily),
            DeclarationData::FontStyle(..) => Some(PropertyId::FontStyle),
            DeclarationData::FontVariantCaps(..) => Some(PropertyId::FontVariantCaps),
            DeclarationData::LineHeight(..) => Some(PropertyId::LineHeight),
            DeclarationData::Font(..) => Some(PropertyId::Font),
            DeclarationData::VerticalAlign(..) => Some(PropertyId::VerticalAlign),
            DeclarationData::FontPalette(..) => Some(PropertyId::FontPalette),
            DeclarationData::TransitionProperty(_, prefix) => {
                Some(PropertyId::TransitionProperty(prefix))
            }
            DeclarationData::TransitionDuration(_, prefix) => {
                Some(PropertyId::TransitionDuration(prefix))
            }
            DeclarationData::TransitionDelay(_, prefix) => {
                Some(PropertyId::TransitionDelay(prefix))
            }
            DeclarationData::TransitionTimingFunction(_, prefix) => {
                Some(PropertyId::TransitionTimingFunction(prefix))
            }
            DeclarationData::Transition(_, prefix) => Some(PropertyId::Transition(prefix)),
            DeclarationData::AnimationName(_, prefix) => Some(PropertyId::AnimationName(prefix)),
            DeclarationData::AnimationDuration(_, prefix) => {
                Some(PropertyId::AnimationDuration(prefix))
            }
            DeclarationData::AnimationTimingFunction(_, prefix) => {
                Some(PropertyId::AnimationTimingFunction(prefix))
            }
            DeclarationData::AnimationIterationCount(_, prefix) => {
                Some(PropertyId::AnimationIterationCount(prefix))
            }
            DeclarationData::AnimationDirection(_, prefix) => {
                Some(PropertyId::AnimationDirection(prefix))
            }
            DeclarationData::AnimationPlayState(_, prefix) => {
                Some(PropertyId::AnimationPlayState(prefix))
            }
            DeclarationData::AnimationDelay(_, prefix) => Some(PropertyId::AnimationDelay(prefix)),
            DeclarationData::AnimationFillMode(_, prefix) => {
                Some(PropertyId::AnimationFillMode(prefix))
            }
            DeclarationData::AnimationComposition(..) => Some(PropertyId::AnimationComposition),
            DeclarationData::AnimationTimeline(..) => Some(PropertyId::AnimationTimeline),
            DeclarationData::AnimationRangeStart(..) => Some(PropertyId::AnimationRangeStart),
            DeclarationData::AnimationRangeEnd(..) => Some(PropertyId::AnimationRangeEnd),
            DeclarationData::AnimationRange(..) => Some(PropertyId::AnimationRange),
            DeclarationData::Animation(_, prefix) => Some(PropertyId::Animation(prefix)),
            DeclarationData::Transform(_, prefix) => Some(PropertyId::Transform(prefix)),
            DeclarationData::TransformOrigin(_, prefix) => {
                Some(PropertyId::TransformOrigin(prefix))
            }
            DeclarationData::TransformStyle(_, prefix) => Some(PropertyId::TransformStyle(prefix)),
            DeclarationData::TransformBox(..) => Some(PropertyId::TransformBox),
            DeclarationData::BackfaceVisibility(_, prefix) => {
                Some(PropertyId::BackfaceVisibility(prefix))
            }
            DeclarationData::Perspective(_, prefix) => Some(PropertyId::Perspective(prefix)),
            DeclarationData::PerspectiveOrigin(_, prefix) => {
                Some(PropertyId::PerspectiveOrigin(prefix))
            }
            DeclarationData::Translate(..) => Some(PropertyId::Translate),
            DeclarationData::Rotate(..) => Some(PropertyId::Rotate),
            DeclarationData::Scale(..) => Some(PropertyId::Scale),
            DeclarationData::TextTransform(..) => Some(PropertyId::TextTransform),
            DeclarationData::Content(..) => Some(PropertyId::Content),
            DeclarationData::WhiteSpace(..) => Some(PropertyId::WhiteSpace),
            DeclarationData::TabSize(_, prefix) => Some(PropertyId::TabSize(prefix)),
            DeclarationData::WordBreak(..) => Some(PropertyId::WordBreak),
            DeclarationData::LineBreak(..) => Some(PropertyId::LineBreak),
            DeclarationData::Hyphens(_, prefix) => Some(PropertyId::Hyphens(prefix)),
            DeclarationData::OverflowWrap(..) => Some(PropertyId::OverflowWrap),
            DeclarationData::WordWrap(..) => Some(PropertyId::WordWrap),
            DeclarationData::TextAlign(..) => Some(PropertyId::TextAlign),
            DeclarationData::TextAlignLast(_, prefix) => Some(PropertyId::TextAlignLast(prefix)),
            DeclarationData::TextJustify(..) => Some(PropertyId::TextJustify),
            DeclarationData::WordSpacing(..) => Some(PropertyId::WordSpacing),
            DeclarationData::LetterSpacing(..) => Some(PropertyId::LetterSpacing),
            DeclarationData::TextIndent(..) => Some(PropertyId::TextIndent),
            DeclarationData::TextDecorationLine(_, prefix) => {
                Some(PropertyId::TextDecorationLine(prefix))
            }
            DeclarationData::TextDecorationStyle(_, prefix) => {
                Some(PropertyId::TextDecorationStyle(prefix))
            }
            DeclarationData::TextDecorationColor(_, prefix) => {
                Some(PropertyId::TextDecorationColor(prefix))
            }
            DeclarationData::TextDecorationThickness(..) => {
                Some(PropertyId::TextDecorationThickness)
            }
            DeclarationData::TextDecoration(_, prefix) => Some(PropertyId::TextDecoration(prefix)),
            DeclarationData::TextDecorationSkipInk(_, prefix) => {
                Some(PropertyId::TextDecorationSkipInk(prefix))
            }
            DeclarationData::TextEmphasisStyle(_, prefix) => {
                Some(PropertyId::TextEmphasisStyle(prefix))
            }
            DeclarationData::TextEmphasisColor(_, prefix) => {
                Some(PropertyId::TextEmphasisColor(prefix))
            }
            DeclarationData::TextEmphasis(_, prefix) => Some(PropertyId::TextEmphasis(prefix)),
            DeclarationData::TextEmphasisPosition(_, prefix) => {
                Some(PropertyId::TextEmphasisPosition(prefix))
            }
            DeclarationData::TextShadow(..) => Some(PropertyId::TextShadow),
            DeclarationData::TextSizeAdjust(_, prefix) => Some(PropertyId::TextSizeAdjust(prefix)),
            DeclarationData::Direction(..) => Some(PropertyId::Direction),
            DeclarationData::UnicodeBidi(..) => Some(PropertyId::UnicodeBidi),
            DeclarationData::BoxDecorationBreak(_, prefix) => {
                Some(PropertyId::BoxDecorationBreak(prefix))
            }
            DeclarationData::Resize(..) => Some(PropertyId::Resize),
            DeclarationData::PointerEvents(..) => Some(PropertyId::PointerEvents),
            DeclarationData::Float(..) => Some(PropertyId::Float),
            DeclarationData::Clear(..) => Some(PropertyId::Clear),
            DeclarationData::TouchAction(..) => Some(PropertyId::TouchAction),
            DeclarationData::ScrollBehavior(..) => Some(PropertyId::ScrollBehavior),
            DeclarationData::Cursor(..) => Some(PropertyId::Cursor),
            DeclarationData::CaretColor(..) => Some(PropertyId::CaretColor),
            DeclarationData::CaretShape(..) => Some(PropertyId::CaretShape),
            DeclarationData::Caret(..) => Some(PropertyId::Caret),
            DeclarationData::UserSelect(_, prefix) => Some(PropertyId::UserSelect(prefix)),
            DeclarationData::AccentColor(..) => Some(PropertyId::AccentColor),
            DeclarationData::Appearance(_, prefix) => Some(PropertyId::Appearance(prefix)),
            DeclarationData::ListStyleType(..) => Some(PropertyId::ListStyleType),
            DeclarationData::ListStyleImage(..) => Some(PropertyId::ListStyleImage),
            DeclarationData::ListStylePosition(..) => Some(PropertyId::ListStylePosition),
            DeclarationData::ListStyle(..) => Some(PropertyId::ListStyle),
            DeclarationData::MarkerSide(..) => Some(PropertyId::MarkerSide),
            DeclarationData::Composes(..) => Some(PropertyId::Composes),
            DeclarationData::Fill(..) => Some(PropertyId::Fill),
            DeclarationData::FillRule(..) => Some(PropertyId::FillRule),
            DeclarationData::FillOpacity(..) => Some(PropertyId::FillOpacity),
            DeclarationData::Stroke(..) => Some(PropertyId::Stroke),
            DeclarationData::StrokeOpacity(..) => Some(PropertyId::StrokeOpacity),
            DeclarationData::StrokeWidth(..) => Some(PropertyId::StrokeWidth),
            DeclarationData::StrokeLinecap(..) => Some(PropertyId::StrokeLinecap),
            DeclarationData::StrokeLinejoin(..) => Some(PropertyId::StrokeLinejoin),
            DeclarationData::StrokeMiterlimit(..) => Some(PropertyId::StrokeMiterlimit),
            DeclarationData::StrokeDasharray(..) => Some(PropertyId::StrokeDasharray),
            DeclarationData::StrokeDashoffset(..) => Some(PropertyId::StrokeDashoffset),
            DeclarationData::MarkerStart(..) => Some(PropertyId::MarkerStart),
            DeclarationData::MarkerMid(..) => Some(PropertyId::MarkerMid),
            DeclarationData::MarkerEnd(..) => Some(PropertyId::MarkerEnd),
            DeclarationData::Marker(..) => Some(PropertyId::Marker),
            DeclarationData::ColorInterpolation(..) => Some(PropertyId::ColorInterpolation),
            DeclarationData::ColorInterpolationFilters(..) => {
                Some(PropertyId::ColorInterpolationFilters)
            }
            DeclarationData::ColorRendering(..) => Some(PropertyId::ColorRendering),
            DeclarationData::ShapeRendering(..) => Some(PropertyId::ShapeRendering),
            DeclarationData::TextRendering(..) => Some(PropertyId::TextRendering),
            DeclarationData::ImageRendering(..) => Some(PropertyId::ImageRendering),
            DeclarationData::ClipPath(_, prefix) => Some(PropertyId::ClipPath(prefix)),
            DeclarationData::ClipRule(..) => Some(PropertyId::ClipRule),
            DeclarationData::MaskImage(_, prefix) => Some(PropertyId::MaskImage(prefix)),
            DeclarationData::MaskMode(..) => Some(PropertyId::MaskMode),
            DeclarationData::MaskRepeat(_, prefix) => Some(PropertyId::MaskRepeat(prefix)),
            DeclarationData::MaskPositionX(..) => Some(PropertyId::MaskPositionX),
            DeclarationData::MaskPositionY(..) => Some(PropertyId::MaskPositionY),
            DeclarationData::MaskPosition(_, prefix) => Some(PropertyId::MaskPosition(prefix)),
            DeclarationData::MaskClip(_, prefix) => Some(PropertyId::MaskClip(prefix)),
            DeclarationData::MaskOrigin(_, prefix) => Some(PropertyId::MaskOrigin(prefix)),
            DeclarationData::MaskSize(_, prefix) => Some(PropertyId::MaskSize(prefix)),
            DeclarationData::MaskComposite(..) => Some(PropertyId::MaskComposite),
            DeclarationData::MaskType(..) => Some(PropertyId::MaskType),
            DeclarationData::Mask(_, prefix) => Some(PropertyId::Mask(prefix)),
            DeclarationData::MaskBorderSource(..) => Some(PropertyId::MaskBorderSource),
            DeclarationData::MaskBorderMode(..) => Some(PropertyId::MaskBorderMode),
            DeclarationData::MaskBorderSlice(..) => Some(PropertyId::MaskBorderSlice),
            DeclarationData::MaskBorderWidth(..) => Some(PropertyId::MaskBorderWidth),
            DeclarationData::MaskBorderOutset(..) => Some(PropertyId::MaskBorderOutset),
            DeclarationData::MaskBorderRepeat(..) => Some(PropertyId::MaskBorderRepeat),
            DeclarationData::MaskBorder(..) => Some(PropertyId::MaskBorder),
            DeclarationData::WebKitMaskComposite(..) => Some(PropertyId::WebKitMaskComposite),
            DeclarationData::WebKitMaskSourceType(_, prefix) => {
                Some(PropertyId::WebKitMaskSourceType(prefix))
            }
            DeclarationData::WebKitMaskBoxImage(_, prefix) => {
                Some(PropertyId::WebKitMaskBoxImage(prefix))
            }
            DeclarationData::WebKitMaskBoxImageSource(_, prefix) => {
                Some(PropertyId::WebKitMaskBoxImageSource(prefix))
            }
            DeclarationData::WebKitMaskBoxImageSlice(_, prefix) => {
                Some(PropertyId::WebKitMaskBoxImageSlice(prefix))
            }
            DeclarationData::WebKitMaskBoxImageWidth(_, prefix) => {
                Some(PropertyId::WebKitMaskBoxImageWidth(prefix))
            }
            DeclarationData::WebKitMaskBoxImageOutset(_, prefix) => {
                Some(PropertyId::WebKitMaskBoxImageOutset(prefix))
            }
            DeclarationData::WebKitMaskBoxImageRepeat(_, prefix) => {
                Some(PropertyId::WebKitMaskBoxImageRepeat(prefix))
            }
            DeclarationData::Filter(_, prefix) => Some(PropertyId::Filter(prefix)),
            DeclarationData::BackdropFilter(_, prefix) => Some(PropertyId::BackdropFilter(prefix)),
            DeclarationData::MixBlendMode(..) => Some(PropertyId::MixBlendMode),
            DeclarationData::ZIndex(..) => Some(PropertyId::ZIndex),
            DeclarationData::ContainerType(..) => Some(PropertyId::ContainerType),
            DeclarationData::ContainerName(..) => Some(PropertyId::ContainerName),
            DeclarationData::Container(..) => Some(PropertyId::Container),
            DeclarationData::ViewTransitionName(..) => Some(PropertyId::ViewTransitionName),
            DeclarationData::ViewTransitionClass(..) => Some(PropertyId::ViewTransitionClass),
            DeclarationData::ViewTransitionGroup(..) => Some(PropertyId::ViewTransitionGroup),
            DeclarationData::ColorScheme(..) => Some(PropertyId::ColorScheme),
            DeclarationData::PrintColorAdjust(_, prefix) => {
                Some(PropertyId::PrintColorAdjust(prefix))
            }
            DeclarationData::CSSWide(property_id, _) => Some(property_id),
            DeclarationData::Unparsed(value) => Some(value.property_id(ast)),
            DeclarationData::Custom(value) => Some(PropertyId::Custom(match value.name(ast) {
                CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
            })),
            DeclarationData::Tombstone => None,
        }
    }
    /// Returns the canonical CSS property name.
    pub fn name(self, ast: &Ast<'ast>) -> &'ast str {
        match self.data(ast) {
            DeclarationData::All(..) => "all",
            DeclarationData::BackgroundColor(..) => "background-color",
            DeclarationData::BackgroundImage(..) => "background-image",
            DeclarationData::BackgroundPositionX(..) => "background-position-x",
            DeclarationData::BackgroundPositionY(..) => "background-position-y",
            DeclarationData::BackgroundPosition(..) => "background-position",
            DeclarationData::BackgroundSize(..) => "background-size",
            DeclarationData::BackgroundRepeat(..) => "background-repeat",
            DeclarationData::BackgroundAttachment(..) => "background-attachment",
            DeclarationData::BackgroundClip(..) => "background-clip",
            DeclarationData::BackgroundOrigin(..) => "background-origin",
            DeclarationData::Background(..) => "background",
            DeclarationData::BoxShadow(..) => "box-shadow",
            DeclarationData::Opacity(..) => "opacity",
            DeclarationData::Color(..) => "color",
            DeclarationData::Display(..) => "display",
            DeclarationData::Visibility(..) => "visibility",
            DeclarationData::Width(..) => "width",
            DeclarationData::Height(..) => "height",
            DeclarationData::MinWidth(..) => "min-width",
            DeclarationData::MinHeight(..) => "min-height",
            DeclarationData::MaxWidth(..) => "max-width",
            DeclarationData::MaxHeight(..) => "max-height",
            DeclarationData::BlockSize(..) => "block-size",
            DeclarationData::InlineSize(..) => "inline-size",
            DeclarationData::MinBlockSize(..) => "min-block-size",
            DeclarationData::MinInlineSize(..) => "min-inline-size",
            DeclarationData::MaxBlockSize(..) => "max-block-size",
            DeclarationData::MaxInlineSize(..) => "max-inline-size",
            DeclarationData::BoxSizing(..) => "box-sizing",
            DeclarationData::AspectRatio(..) => "aspect-ratio",
            DeclarationData::Overflow(..) => "overflow",
            DeclarationData::OverflowX(..) => "overflow-x",
            DeclarationData::OverflowY(..) => "overflow-y",
            DeclarationData::TextOverflow(..) => "text-overflow",
            DeclarationData::ObjectFit(..) => "object-fit",
            DeclarationData::ObjectPosition(..) => "object-position",
            DeclarationData::ScrollbarColor(..) => "scrollbar-color",
            DeclarationData::Position(..) => "position",
            DeclarationData::Top(..) => "top",
            DeclarationData::Bottom(..) => "bottom",
            DeclarationData::Left(..) => "left",
            DeclarationData::Right(..) => "right",
            DeclarationData::InsetBlockStart(..) => "inset-block-start",
            DeclarationData::InsetBlockEnd(..) => "inset-block-end",
            DeclarationData::InsetInlineStart(..) => "inset-inline-start",
            DeclarationData::InsetInlineEnd(..) => "inset-inline-end",
            DeclarationData::InsetBlock(..) => "inset-block",
            DeclarationData::InsetInline(..) => "inset-inline",
            DeclarationData::Inset(..) => "inset",
            DeclarationData::BorderSpacing(..) => "border-spacing",
            DeclarationData::BorderTopColor(..) => "border-top-color",
            DeclarationData::BorderBottomColor(..) => "border-bottom-color",
            DeclarationData::BorderLeftColor(..) => "border-left-color",
            DeclarationData::BorderRightColor(..) => "border-right-color",
            DeclarationData::BorderBlockStartColor(..) => "border-block-start-color",
            DeclarationData::BorderBlockEndColor(..) => "border-block-end-color",
            DeclarationData::BorderInlineStartColor(..) => "border-inline-start-color",
            DeclarationData::BorderInlineEndColor(..) => "border-inline-end-color",
            DeclarationData::BorderTopStyle(..) => "border-top-style",
            DeclarationData::BorderBottomStyle(..) => "border-bottom-style",
            DeclarationData::BorderLeftStyle(..) => "border-left-style",
            DeclarationData::BorderRightStyle(..) => "border-right-style",
            DeclarationData::BorderBlockStartStyle(..) => "border-block-start-style",
            DeclarationData::BorderBlockEndStyle(..) => "border-block-end-style",
            DeclarationData::BorderInlineStartStyle(..) => "border-inline-start-style",
            DeclarationData::BorderInlineEndStyle(..) => "border-inline-end-style",
            DeclarationData::BorderTopWidth(..) => "border-top-width",
            DeclarationData::BorderBottomWidth(..) => "border-bottom-width",
            DeclarationData::BorderLeftWidth(..) => "border-left-width",
            DeclarationData::BorderRightWidth(..) => "border-right-width",
            DeclarationData::BorderBlockStartWidth(..) => "border-block-start-width",
            DeclarationData::BorderBlockEndWidth(..) => "border-block-end-width",
            DeclarationData::BorderInlineStartWidth(..) => "border-inline-start-width",
            DeclarationData::BorderInlineEndWidth(..) => "border-inline-end-width",
            DeclarationData::BorderTopLeftRadius(..) => "border-top-left-radius",
            DeclarationData::BorderTopRightRadius(..) => "border-top-right-radius",
            DeclarationData::BorderBottomLeftRadius(..) => "border-bottom-left-radius",
            DeclarationData::BorderBottomRightRadius(..) => "border-bottom-right-radius",
            DeclarationData::BorderStartStartRadius(..) => "border-start-start-radius",
            DeclarationData::BorderStartEndRadius(..) => "border-start-end-radius",
            DeclarationData::BorderEndStartRadius(..) => "border-end-start-radius",
            DeclarationData::BorderEndEndRadius(..) => "border-end-end-radius",
            DeclarationData::BorderRadius(..) => "border-radius",
            DeclarationData::BorderImageSource(..) => "border-image-source",
            DeclarationData::BorderImageOutset(..) => "border-image-outset",
            DeclarationData::BorderImageRepeat(..) => "border-image-repeat",
            DeclarationData::BorderImageWidth(..) => "border-image-width",
            DeclarationData::BorderImageSlice(..) => "border-image-slice",
            DeclarationData::BorderImage(..) => "border-image",
            DeclarationData::BorderColor(..) => "border-color",
            DeclarationData::BorderStyle(..) => "border-style",
            DeclarationData::BorderWidth(..) => "border-width",
            DeclarationData::BorderBlockColor(..) => "border-block-color",
            DeclarationData::BorderBlockStyle(..) => "border-block-style",
            DeclarationData::BorderBlockWidth(..) => "border-block-width",
            DeclarationData::BorderInlineColor(..) => "border-inline-color",
            DeclarationData::BorderInlineStyle(..) => "border-inline-style",
            DeclarationData::BorderInlineWidth(..) => "border-inline-width",
            DeclarationData::Border(..) => "border",
            DeclarationData::BorderTop(..) => "border-top",
            DeclarationData::BorderBottom(..) => "border-bottom",
            DeclarationData::BorderLeft(..) => "border-left",
            DeclarationData::BorderRight(..) => "border-right",
            DeclarationData::BorderBlock(..) => "border-block",
            DeclarationData::BorderBlockStart(..) => "border-block-start",
            DeclarationData::BorderBlockEnd(..) => "border-block-end",
            DeclarationData::BorderInline(..) => "border-inline",
            DeclarationData::BorderInlineStart(..) => "border-inline-start",
            DeclarationData::BorderInlineEnd(..) => "border-inline-end",
            DeclarationData::Outline(..) => "outline",
            DeclarationData::OutlineColor(..) => "outline-color",
            DeclarationData::OutlineStyle(..) => "outline-style",
            DeclarationData::OutlineWidth(..) => "outline-width",
            DeclarationData::FlexDirection(..) => "flex-direction",
            DeclarationData::FlexWrap(..) => "flex-wrap",
            DeclarationData::FlexFlow(..) => "flex-flow",
            DeclarationData::FlexGrow(..) => "flex-grow",
            DeclarationData::FlexShrink(..) => "flex-shrink",
            DeclarationData::FlexBasis(..) => "flex-basis",
            DeclarationData::Flex(..) => "flex",
            DeclarationData::Order(..) => "order",
            DeclarationData::AlignContent(..) => "align-content",
            DeclarationData::JustifyContent(..) => "justify-content",
            DeclarationData::PlaceContent(..) => "place-content",
            DeclarationData::AlignSelf(..) => "align-self",
            DeclarationData::JustifySelf(..) => "justify-self",
            DeclarationData::PlaceSelf(..) => "place-self",
            DeclarationData::AlignItems(..) => "align-items",
            DeclarationData::JustifyItems(..) => "justify-items",
            DeclarationData::PlaceItems(..) => "place-items",
            DeclarationData::RowGap(..) => "row-gap",
            DeclarationData::ColumnGap(..) => "column-gap",
            DeclarationData::Gap(..) => "gap",
            DeclarationData::ColumnRule(..) => "column-rule",
            DeclarationData::ColumnWidth(..) => "column-width",
            DeclarationData::ColumnCount(..) => "column-count",
            DeclarationData::Columns(..) => "columns",
            DeclarationData::GridColumnGap(..) => "grid-column-gap",
            DeclarationData::GridRowGap(..) => "grid-row-gap",
            DeclarationData::BoxOrient(..) => "box-orient",
            DeclarationData::BoxDirection(..) => "box-direction",
            DeclarationData::BoxOrdinalGroup(..) => "box-ordinal-group",
            DeclarationData::BoxAlign(..) => "box-align",
            DeclarationData::BoxFlex(..) => "box-flex",
            DeclarationData::BoxFlexGroup(..) => "box-flex-group",
            DeclarationData::BoxPack(..) => "box-pack",
            DeclarationData::BoxLines(..) => "box-lines",
            DeclarationData::FlexPack(..) => "flex-pack",
            DeclarationData::FlexOrder(..) => "flex-order",
            DeclarationData::FlexAlign(..) => "flex-align",
            DeclarationData::FlexItemAlign(..) => "flex-item-align",
            DeclarationData::FlexLinePack(..) => "flex-line-pack",
            DeclarationData::FlexPositive(..) => "flex-positive",
            DeclarationData::FlexNegative(..) => "flex-negative",
            DeclarationData::FlexPreferredSize(..) => "flex-preferred-size",
            DeclarationData::GridTemplateColumns(..) => "grid-template-columns",
            DeclarationData::GridTemplateRows(..) => "grid-template-rows",
            DeclarationData::GridAutoColumns(..) => "grid-auto-columns",
            DeclarationData::GridAutoRows(..) => "grid-auto-rows",
            DeclarationData::GridAutoFlow(..) => "grid-auto-flow",
            DeclarationData::GridTemplateAreas(..) => "grid-template-areas",
            DeclarationData::GridTemplate(..) => "grid-template",
            DeclarationData::Grid(..) => "grid",
            DeclarationData::GridRowStart(..) => "grid-row-start",
            DeclarationData::GridRowEnd(..) => "grid-row-end",
            DeclarationData::GridColumnStart(..) => "grid-column-start",
            DeclarationData::GridColumnEnd(..) => "grid-column-end",
            DeclarationData::GridRow(..) => "grid-row",
            DeclarationData::GridColumn(..) => "grid-column",
            DeclarationData::GridArea(..) => "grid-area",
            DeclarationData::MarginTop(..) => "margin-top",
            DeclarationData::MarginBottom(..) => "margin-bottom",
            DeclarationData::MarginLeft(..) => "margin-left",
            DeclarationData::MarginRight(..) => "margin-right",
            DeclarationData::MarginBlockStart(..) => "margin-block-start",
            DeclarationData::MarginBlockEnd(..) => "margin-block-end",
            DeclarationData::MarginInlineStart(..) => "margin-inline-start",
            DeclarationData::MarginInlineEnd(..) => "margin-inline-end",
            DeclarationData::MarginBlock(..) => "margin-block",
            DeclarationData::MarginInline(..) => "margin-inline",
            DeclarationData::Margin(..) => "margin",
            DeclarationData::PaddingTop(..) => "padding-top",
            DeclarationData::PaddingBottom(..) => "padding-bottom",
            DeclarationData::PaddingLeft(..) => "padding-left",
            DeclarationData::PaddingRight(..) => "padding-right",
            DeclarationData::PaddingBlockStart(..) => "padding-block-start",
            DeclarationData::PaddingBlockEnd(..) => "padding-block-end",
            DeclarationData::PaddingInlineStart(..) => "padding-inline-start",
            DeclarationData::PaddingInlineEnd(..) => "padding-inline-end",
            DeclarationData::PaddingBlock(..) => "padding-block",
            DeclarationData::PaddingInline(..) => "padding-inline",
            DeclarationData::Padding(..) => "padding",
            DeclarationData::ScrollMarginTop(..) => "scroll-margin-top",
            DeclarationData::ScrollMarginBottom(..) => "scroll-margin-bottom",
            DeclarationData::ScrollMarginLeft(..) => "scroll-margin-left",
            DeclarationData::ScrollMarginRight(..) => "scroll-margin-right",
            DeclarationData::ScrollMarginBlockStart(..) => "scroll-margin-block-start",
            DeclarationData::ScrollMarginBlockEnd(..) => "scroll-margin-block-end",
            DeclarationData::ScrollMarginInlineStart(..) => "scroll-margin-inline-start",
            DeclarationData::ScrollMarginInlineEnd(..) => "scroll-margin-inline-end",
            DeclarationData::ScrollMarginBlock(..) => "scroll-margin-block",
            DeclarationData::ScrollMarginInline(..) => "scroll-margin-inline",
            DeclarationData::ScrollMargin(..) => "scroll-margin",
            DeclarationData::ScrollPaddingTop(..) => "scroll-padding-top",
            DeclarationData::ScrollPaddingBottom(..) => "scroll-padding-bottom",
            DeclarationData::ScrollPaddingLeft(..) => "scroll-padding-left",
            DeclarationData::ScrollPaddingRight(..) => "scroll-padding-right",
            DeclarationData::ScrollPaddingBlockStart(..) => "scroll-padding-block-start",
            DeclarationData::ScrollPaddingBlockEnd(..) => "scroll-padding-block-end",
            DeclarationData::ScrollPaddingInlineStart(..) => "scroll-padding-inline-start",
            DeclarationData::ScrollPaddingInlineEnd(..) => "scroll-padding-inline-end",
            DeclarationData::ScrollPaddingBlock(..) => "scroll-padding-block",
            DeclarationData::ScrollPaddingInline(..) => "scroll-padding-inline",
            DeclarationData::ScrollPadding(..) => "scroll-padding",
            DeclarationData::FontWeight(..) => "font-weight",
            DeclarationData::FontSize(..) => "font-size",
            DeclarationData::FontStretch(..) => "font-stretch",
            DeclarationData::FontFamily(..) => "font-family",
            DeclarationData::FontStyle(..) => "font-style",
            DeclarationData::FontVariantCaps(..) => "font-variant-caps",
            DeclarationData::LineHeight(..) => "line-height",
            DeclarationData::Font(..) => "font",
            DeclarationData::VerticalAlign(..) => "vertical-align",
            DeclarationData::FontPalette(..) => "font-palette",
            DeclarationData::TransitionProperty(..) => "transition-property",
            DeclarationData::TransitionDuration(..) => "transition-duration",
            DeclarationData::TransitionDelay(..) => "transition-delay",
            DeclarationData::TransitionTimingFunction(..) => "transition-timing-function",
            DeclarationData::Transition(..) => "transition",
            DeclarationData::AnimationName(..) => "animation-name",
            DeclarationData::AnimationDuration(..) => "animation-duration",
            DeclarationData::AnimationTimingFunction(..) => "animation-timing-function",
            DeclarationData::AnimationIterationCount(..) => "animation-iteration-count",
            DeclarationData::AnimationDirection(..) => "animation-direction",
            DeclarationData::AnimationPlayState(..) => "animation-play-state",
            DeclarationData::AnimationDelay(..) => "animation-delay",
            DeclarationData::AnimationFillMode(..) => "animation-fill-mode",
            DeclarationData::AnimationComposition(..) => "animation-composition",
            DeclarationData::AnimationTimeline(..) => "animation-timeline",
            DeclarationData::AnimationRangeStart(..) => "animation-range-start",
            DeclarationData::AnimationRangeEnd(..) => "animation-range-end",
            DeclarationData::AnimationRange(..) => "animation-range",
            DeclarationData::Animation(..) => "animation",
            DeclarationData::Transform(..) => "transform",
            DeclarationData::TransformOrigin(..) => "transform-origin",
            DeclarationData::TransformStyle(..) => "transform-style",
            DeclarationData::TransformBox(..) => "transform-box",
            DeclarationData::BackfaceVisibility(..) => "backface-visibility",
            DeclarationData::Perspective(..) => "perspective",
            DeclarationData::PerspectiveOrigin(..) => "perspective-origin",
            DeclarationData::Translate(..) => "translate",
            DeclarationData::Rotate(..) => "rotate",
            DeclarationData::Scale(..) => "scale",
            DeclarationData::TextTransform(..) => "text-transform",
            DeclarationData::Content(..) => "content",
            DeclarationData::WhiteSpace(..) => "white-space",
            DeclarationData::TabSize(..) => "tab-size",
            DeclarationData::WordBreak(..) => "word-break",
            DeclarationData::LineBreak(..) => "line-break",
            DeclarationData::Hyphens(..) => "hyphens",
            DeclarationData::OverflowWrap(..) => "overflow-wrap",
            DeclarationData::WordWrap(..) => "word-wrap",
            DeclarationData::TextAlign(..) => "text-align",
            DeclarationData::TextAlignLast(..) => "text-align-last",
            DeclarationData::TextJustify(..) => "text-justify",
            DeclarationData::WordSpacing(..) => "word-spacing",
            DeclarationData::LetterSpacing(..) => "letter-spacing",
            DeclarationData::TextIndent(..) => "text-indent",
            DeclarationData::TextDecorationLine(..) => "text-decoration-line",
            DeclarationData::TextDecorationStyle(..) => "text-decoration-style",
            DeclarationData::TextDecorationColor(..) => "text-decoration-color",
            DeclarationData::TextDecorationThickness(..) => "text-decoration-thickness",
            DeclarationData::TextDecoration(..) => "text-decoration",
            DeclarationData::TextDecorationSkipInk(..) => "text-decoration-skip-ink",
            DeclarationData::TextEmphasisStyle(..) => "text-emphasis-style",
            DeclarationData::TextEmphasisColor(..) => "text-emphasis-color",
            DeclarationData::TextEmphasis(..) => "text-emphasis",
            DeclarationData::TextEmphasisPosition(..) => "text-emphasis-position",
            DeclarationData::TextShadow(..) => "text-shadow",
            DeclarationData::TextSizeAdjust(..) => "text-size-adjust",
            DeclarationData::Direction(..) => "direction",
            DeclarationData::UnicodeBidi(..) => "unicode-bidi",
            DeclarationData::BoxDecorationBreak(..) => "box-decoration-break",
            DeclarationData::Resize(..) => "resize",
            DeclarationData::PointerEvents(..) => "pointer-events",
            DeclarationData::Float(..) => "float",
            DeclarationData::Clear(..) => "clear",
            DeclarationData::TouchAction(..) => "touch-action",
            DeclarationData::ScrollBehavior(..) => "scroll-behavior",
            DeclarationData::Cursor(..) => "cursor",
            DeclarationData::CaretColor(..) => "caret-color",
            DeclarationData::CaretShape(..) => "caret-shape",
            DeclarationData::Caret(..) => "caret",
            DeclarationData::UserSelect(..) => "user-select",
            DeclarationData::AccentColor(..) => "accent-color",
            DeclarationData::Appearance(..) => "appearance",
            DeclarationData::ListStyleType(..) => "list-style-type",
            DeclarationData::ListStyleImage(..) => "list-style-image",
            DeclarationData::ListStylePosition(..) => "list-style-position",
            DeclarationData::ListStyle(..) => "list-style",
            DeclarationData::MarkerSide(..) => "marker-side",
            DeclarationData::Composes(..) => "composes",
            DeclarationData::Fill(..) => "fill",
            DeclarationData::FillRule(..) => "fill-rule",
            DeclarationData::FillOpacity(..) => "fill-opacity",
            DeclarationData::Stroke(..) => "stroke",
            DeclarationData::StrokeOpacity(..) => "stroke-opacity",
            DeclarationData::StrokeWidth(..) => "stroke-width",
            DeclarationData::StrokeLinecap(..) => "stroke-linecap",
            DeclarationData::StrokeLinejoin(..) => "stroke-linejoin",
            DeclarationData::StrokeMiterlimit(..) => "stroke-miterlimit",
            DeclarationData::StrokeDasharray(..) => "stroke-dasharray",
            DeclarationData::StrokeDashoffset(..) => "stroke-dashoffset",
            DeclarationData::MarkerStart(..) => "marker-start",
            DeclarationData::MarkerMid(..) => "marker-mid",
            DeclarationData::MarkerEnd(..) => "marker-end",
            DeclarationData::Marker(..) => "marker",
            DeclarationData::ColorInterpolation(..) => "color-interpolation",
            DeclarationData::ColorInterpolationFilters(..) => "color-interpolation-filters",
            DeclarationData::ColorRendering(..) => "color-rendering",
            DeclarationData::ShapeRendering(..) => "shape-rendering",
            DeclarationData::TextRendering(..) => "text-rendering",
            DeclarationData::ImageRendering(..) => "image-rendering",
            DeclarationData::ClipPath(..) => "clip-path",
            DeclarationData::ClipRule(..) => "clip-rule",
            DeclarationData::MaskImage(..) => "mask-image",
            DeclarationData::MaskMode(..) => "mask-mode",
            DeclarationData::MaskRepeat(..) => "mask-repeat",
            DeclarationData::MaskPositionX(..) => "mask-position-x",
            DeclarationData::MaskPositionY(..) => "mask-position-y",
            DeclarationData::MaskPosition(..) => "mask-position",
            DeclarationData::MaskClip(..) => "mask-clip",
            DeclarationData::MaskOrigin(..) => "mask-origin",
            DeclarationData::MaskSize(..) => "mask-size",
            DeclarationData::MaskComposite(..) => "mask-composite",
            DeclarationData::MaskType(..) => "mask-type",
            DeclarationData::Mask(..) => "mask",
            DeclarationData::MaskBorderSource(..) => "mask-border-source",
            DeclarationData::MaskBorderMode(..) => "mask-border-mode",
            DeclarationData::MaskBorderSlice(..) => "mask-border-slice",
            DeclarationData::MaskBorderWidth(..) => "mask-border-width",
            DeclarationData::MaskBorderOutset(..) => "mask-border-outset",
            DeclarationData::MaskBorderRepeat(..) => "mask-border-repeat",
            DeclarationData::MaskBorder(..) => "mask-border",
            DeclarationData::WebKitMaskComposite(..) => "-webkit-mask-composite",
            DeclarationData::WebKitMaskSourceType(..) => "mask-source-type",
            DeclarationData::WebKitMaskBoxImage(..) => "mask-box-image",
            DeclarationData::WebKitMaskBoxImageSource(..) => "mask-box-image-source",
            DeclarationData::WebKitMaskBoxImageSlice(..) => "mask-box-image-slice",
            DeclarationData::WebKitMaskBoxImageWidth(..) => "mask-box-image-width",
            DeclarationData::WebKitMaskBoxImageOutset(..) => "mask-box-image-outset",
            DeclarationData::WebKitMaskBoxImageRepeat(..) => "mask-box-image-repeat",
            DeclarationData::Filter(..) => "filter",
            DeclarationData::BackdropFilter(..) => "backdrop-filter",
            DeclarationData::MixBlendMode(..) => "mix-blend-mode",
            DeclarationData::ZIndex(..) => "z-index",
            DeclarationData::ContainerType(..) => "container-type",
            DeclarationData::ContainerName(..) => "container-name",
            DeclarationData::Container(..) => "container",
            DeclarationData::ViewTransitionName(..) => "view-transition-name",
            DeclarationData::ViewTransitionClass(..) => "view-transition-class",
            DeclarationData::ViewTransitionGroup(..) => "view-transition-group",
            DeclarationData::ColorScheme(..) => "color-scheme",
            DeclarationData::PrintColorAdjust(..) => "print-color-adjust",
            DeclarationData::CSSWide(property_id, _) => property_id.name(),
            DeclarationData::Unparsed(value) => value.property_id(ast).name(),
            DeclarationData::Custom(value) => match value.name(ast) {
                CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => name,
            },
            DeclarationData::Tombstone => "",
        }
    }
    /// Returns the vendor prefix associated with this declaration.
    pub fn vendor_prefix(self, ast: &Ast<'ast>) -> VendorPrefix {
        match self.data(ast) {
            DeclarationData::All(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundColor(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundImage(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundPositionX(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundPositionY(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundPosition(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundSize(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundRepeat(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundAttachment(..) => VendorPrefix::NONE,
            DeclarationData::BackgroundClip(_, prefix) => prefix,
            DeclarationData::BackgroundOrigin(..) => VendorPrefix::NONE,
            DeclarationData::Background(..) => VendorPrefix::NONE,
            DeclarationData::BoxShadow(_, prefix) => prefix,
            DeclarationData::Opacity(..) => VendorPrefix::NONE,
            DeclarationData::Color(..) => VendorPrefix::NONE,
            DeclarationData::Display(..) => VendorPrefix::NONE,
            DeclarationData::Visibility(..) => VendorPrefix::NONE,
            DeclarationData::Width(..) => VendorPrefix::NONE,
            DeclarationData::Height(..) => VendorPrefix::NONE,
            DeclarationData::MinWidth(..) => VendorPrefix::NONE,
            DeclarationData::MinHeight(..) => VendorPrefix::NONE,
            DeclarationData::MaxWidth(..) => VendorPrefix::NONE,
            DeclarationData::MaxHeight(..) => VendorPrefix::NONE,
            DeclarationData::BlockSize(..) => VendorPrefix::NONE,
            DeclarationData::InlineSize(..) => VendorPrefix::NONE,
            DeclarationData::MinBlockSize(..) => VendorPrefix::NONE,
            DeclarationData::MinInlineSize(..) => VendorPrefix::NONE,
            DeclarationData::MaxBlockSize(..) => VendorPrefix::NONE,
            DeclarationData::MaxInlineSize(..) => VendorPrefix::NONE,
            DeclarationData::BoxSizing(_, prefix) => prefix,
            DeclarationData::AspectRatio(..) => VendorPrefix::NONE,
            DeclarationData::Overflow(..) => VendorPrefix::NONE,
            DeclarationData::OverflowX(..) => VendorPrefix::NONE,
            DeclarationData::OverflowY(..) => VendorPrefix::NONE,
            DeclarationData::TextOverflow(_, prefix) => prefix,
            DeclarationData::ObjectFit(..) => VendorPrefix::NONE,
            DeclarationData::ObjectPosition(..) => VendorPrefix::NONE,
            DeclarationData::ScrollbarColor(..) => VendorPrefix::NONE,
            DeclarationData::Position(..) => VendorPrefix::NONE,
            DeclarationData::Top(..) => VendorPrefix::NONE,
            DeclarationData::Bottom(..) => VendorPrefix::NONE,
            DeclarationData::Left(..) => VendorPrefix::NONE,
            DeclarationData::Right(..) => VendorPrefix::NONE,
            DeclarationData::InsetBlockStart(..) => VendorPrefix::NONE,
            DeclarationData::InsetBlockEnd(..) => VendorPrefix::NONE,
            DeclarationData::InsetInlineStart(..) => VendorPrefix::NONE,
            DeclarationData::InsetInlineEnd(..) => VendorPrefix::NONE,
            DeclarationData::InsetBlock(..) => VendorPrefix::NONE,
            DeclarationData::InsetInline(..) => VendorPrefix::NONE,
            DeclarationData::Inset(..) => VendorPrefix::NONE,
            DeclarationData::BorderSpacing(..) => VendorPrefix::NONE,
            DeclarationData::BorderTopColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderBottomColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderLeftColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderRightColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockStartColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockEndColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineStartColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineEndColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderTopStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderBottomStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderLeftStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderRightStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockStartStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockEndStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineStartStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineEndStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderTopWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderBottomWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderLeftWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderRightWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockStartWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockEndWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineStartWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineEndWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderTopLeftRadius(_, prefix) => prefix,
            DeclarationData::BorderTopRightRadius(_, prefix) => prefix,
            DeclarationData::BorderBottomLeftRadius(_, prefix) => prefix,
            DeclarationData::BorderBottomRightRadius(_, prefix) => prefix,
            DeclarationData::BorderStartStartRadius(..) => VendorPrefix::NONE,
            DeclarationData::BorderStartEndRadius(..) => VendorPrefix::NONE,
            DeclarationData::BorderEndStartRadius(..) => VendorPrefix::NONE,
            DeclarationData::BorderEndEndRadius(..) => VendorPrefix::NONE,
            DeclarationData::BorderRadius(_, prefix) => prefix,
            DeclarationData::BorderImageSource(..) => VendorPrefix::NONE,
            DeclarationData::BorderImageOutset(..) => VendorPrefix::NONE,
            DeclarationData::BorderImageRepeat(..) => VendorPrefix::NONE,
            DeclarationData::BorderImageWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderImageSlice(..) => VendorPrefix::NONE,
            DeclarationData::BorderImage(_, prefix) => prefix,
            DeclarationData::BorderColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockWidth(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineColor(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineStyle(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineWidth(..) => VendorPrefix::NONE,
            DeclarationData::Border(..) => VendorPrefix::NONE,
            DeclarationData::BorderTop(..) => VendorPrefix::NONE,
            DeclarationData::BorderBottom(..) => VendorPrefix::NONE,
            DeclarationData::BorderLeft(..) => VendorPrefix::NONE,
            DeclarationData::BorderRight(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlock(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockStart(..) => VendorPrefix::NONE,
            DeclarationData::BorderBlockEnd(..) => VendorPrefix::NONE,
            DeclarationData::BorderInline(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineStart(..) => VendorPrefix::NONE,
            DeclarationData::BorderInlineEnd(..) => VendorPrefix::NONE,
            DeclarationData::Outline(..) => VendorPrefix::NONE,
            DeclarationData::OutlineColor(..) => VendorPrefix::NONE,
            DeclarationData::OutlineStyle(..) => VendorPrefix::NONE,
            DeclarationData::OutlineWidth(..) => VendorPrefix::NONE,
            DeclarationData::FlexDirection(_, prefix) => prefix,
            DeclarationData::FlexWrap(_, prefix) => prefix,
            DeclarationData::FlexFlow(_, prefix) => prefix,
            DeclarationData::FlexGrow(_, prefix) => prefix,
            DeclarationData::FlexShrink(_, prefix) => prefix,
            DeclarationData::FlexBasis(_, prefix) => prefix,
            DeclarationData::Flex(_, prefix) => prefix,
            DeclarationData::Order(_, prefix) => prefix,
            DeclarationData::AlignContent(_, prefix) => prefix,
            DeclarationData::JustifyContent(_, prefix) => prefix,
            DeclarationData::PlaceContent(..) => VendorPrefix::NONE,
            DeclarationData::AlignSelf(_, prefix) => prefix,
            DeclarationData::JustifySelf(..) => VendorPrefix::NONE,
            DeclarationData::PlaceSelf(..) => VendorPrefix::NONE,
            DeclarationData::AlignItems(_, prefix) => prefix,
            DeclarationData::JustifyItems(..) => VendorPrefix::NONE,
            DeclarationData::PlaceItems(..) => VendorPrefix::NONE,
            DeclarationData::RowGap(..) => VendorPrefix::NONE,
            DeclarationData::ColumnGap(..) => VendorPrefix::NONE,
            DeclarationData::Gap(..) => VendorPrefix::NONE,
            DeclarationData::ColumnRule(_, prefix) => prefix,
            DeclarationData::ColumnWidth(_, prefix) => prefix,
            DeclarationData::ColumnCount(_, prefix) => prefix,
            DeclarationData::Columns(_, prefix) => prefix,
            DeclarationData::GridColumnGap(..) => VendorPrefix::NONE,
            DeclarationData::GridRowGap(..) => VendorPrefix::NONE,
            DeclarationData::BoxOrient(_, prefix) => prefix,
            DeclarationData::BoxDirection(_, prefix) => prefix,
            DeclarationData::BoxOrdinalGroup(_, prefix) => prefix,
            DeclarationData::BoxAlign(_, prefix) => prefix,
            DeclarationData::BoxFlex(_, prefix) => prefix,
            DeclarationData::BoxFlexGroup(_, prefix) => prefix,
            DeclarationData::BoxPack(_, prefix) => prefix,
            DeclarationData::BoxLines(_, prefix) => prefix,
            DeclarationData::FlexPack(_, prefix) => prefix,
            DeclarationData::FlexOrder(_, prefix) => prefix,
            DeclarationData::FlexAlign(_, prefix) => prefix,
            DeclarationData::FlexItemAlign(_, prefix) => prefix,
            DeclarationData::FlexLinePack(_, prefix) => prefix,
            DeclarationData::FlexPositive(_, prefix) => prefix,
            DeclarationData::FlexNegative(_, prefix) => prefix,
            DeclarationData::FlexPreferredSize(_, prefix) => prefix,
            DeclarationData::GridTemplateColumns(..) => VendorPrefix::NONE,
            DeclarationData::GridTemplateRows(..) => VendorPrefix::NONE,
            DeclarationData::GridAutoColumns(..) => VendorPrefix::NONE,
            DeclarationData::GridAutoRows(..) => VendorPrefix::NONE,
            DeclarationData::GridAutoFlow(..) => VendorPrefix::NONE,
            DeclarationData::GridTemplateAreas(..) => VendorPrefix::NONE,
            DeclarationData::GridTemplate(..) => VendorPrefix::NONE,
            DeclarationData::Grid(..) => VendorPrefix::NONE,
            DeclarationData::GridRowStart(..) => VendorPrefix::NONE,
            DeclarationData::GridRowEnd(..) => VendorPrefix::NONE,
            DeclarationData::GridColumnStart(..) => VendorPrefix::NONE,
            DeclarationData::GridColumnEnd(..) => VendorPrefix::NONE,
            DeclarationData::GridRow(..) => VendorPrefix::NONE,
            DeclarationData::GridColumn(..) => VendorPrefix::NONE,
            DeclarationData::GridArea(..) => VendorPrefix::NONE,
            DeclarationData::MarginTop(..) => VendorPrefix::NONE,
            DeclarationData::MarginBottom(..) => VendorPrefix::NONE,
            DeclarationData::MarginLeft(..) => VendorPrefix::NONE,
            DeclarationData::MarginRight(..) => VendorPrefix::NONE,
            DeclarationData::MarginBlockStart(..) => VendorPrefix::NONE,
            DeclarationData::MarginBlockEnd(..) => VendorPrefix::NONE,
            DeclarationData::MarginInlineStart(..) => VendorPrefix::NONE,
            DeclarationData::MarginInlineEnd(..) => VendorPrefix::NONE,
            DeclarationData::MarginBlock(..) => VendorPrefix::NONE,
            DeclarationData::MarginInline(..) => VendorPrefix::NONE,
            DeclarationData::Margin(..) => VendorPrefix::NONE,
            DeclarationData::PaddingTop(..) => VendorPrefix::NONE,
            DeclarationData::PaddingBottom(..) => VendorPrefix::NONE,
            DeclarationData::PaddingLeft(..) => VendorPrefix::NONE,
            DeclarationData::PaddingRight(..) => VendorPrefix::NONE,
            DeclarationData::PaddingBlockStart(..) => VendorPrefix::NONE,
            DeclarationData::PaddingBlockEnd(..) => VendorPrefix::NONE,
            DeclarationData::PaddingInlineStart(..) => VendorPrefix::NONE,
            DeclarationData::PaddingInlineEnd(..) => VendorPrefix::NONE,
            DeclarationData::PaddingBlock(..) => VendorPrefix::NONE,
            DeclarationData::PaddingInline(..) => VendorPrefix::NONE,
            DeclarationData::Padding(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginTop(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginBottom(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginLeft(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginRight(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginBlockStart(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginBlockEnd(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginInlineStart(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginInlineEnd(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginBlock(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMarginInline(..) => VendorPrefix::NONE,
            DeclarationData::ScrollMargin(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingTop(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingBottom(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingLeft(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingRight(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingBlockStart(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingBlockEnd(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingInlineStart(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingInlineEnd(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingBlock(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPaddingInline(..) => VendorPrefix::NONE,
            DeclarationData::ScrollPadding(..) => VendorPrefix::NONE,
            DeclarationData::FontWeight(..) => VendorPrefix::NONE,
            DeclarationData::FontSize(..) => VendorPrefix::NONE,
            DeclarationData::FontStretch(..) => VendorPrefix::NONE,
            DeclarationData::FontFamily(..) => VendorPrefix::NONE,
            DeclarationData::FontStyle(..) => VendorPrefix::NONE,
            DeclarationData::FontVariantCaps(..) => VendorPrefix::NONE,
            DeclarationData::LineHeight(..) => VendorPrefix::NONE,
            DeclarationData::Font(..) => VendorPrefix::NONE,
            DeclarationData::VerticalAlign(..) => VendorPrefix::NONE,
            DeclarationData::FontPalette(..) => VendorPrefix::NONE,
            DeclarationData::TransitionProperty(_, prefix) => prefix,
            DeclarationData::TransitionDuration(_, prefix) => prefix,
            DeclarationData::TransitionDelay(_, prefix) => prefix,
            DeclarationData::TransitionTimingFunction(_, prefix) => prefix,
            DeclarationData::Transition(_, prefix) => prefix,
            DeclarationData::AnimationName(_, prefix) => prefix,
            DeclarationData::AnimationDuration(_, prefix) => prefix,
            DeclarationData::AnimationTimingFunction(_, prefix) => prefix,
            DeclarationData::AnimationIterationCount(_, prefix) => prefix,
            DeclarationData::AnimationDirection(_, prefix) => prefix,
            DeclarationData::AnimationPlayState(_, prefix) => prefix,
            DeclarationData::AnimationDelay(_, prefix) => prefix,
            DeclarationData::AnimationFillMode(_, prefix) => prefix,
            DeclarationData::AnimationComposition(..) => VendorPrefix::NONE,
            DeclarationData::AnimationTimeline(..) => VendorPrefix::NONE,
            DeclarationData::AnimationRangeStart(..) => VendorPrefix::NONE,
            DeclarationData::AnimationRangeEnd(..) => VendorPrefix::NONE,
            DeclarationData::AnimationRange(..) => VendorPrefix::NONE,
            DeclarationData::Animation(_, prefix) => prefix,
            DeclarationData::Transform(_, prefix) => prefix,
            DeclarationData::TransformOrigin(_, prefix) => prefix,
            DeclarationData::TransformStyle(_, prefix) => prefix,
            DeclarationData::TransformBox(..) => VendorPrefix::NONE,
            DeclarationData::BackfaceVisibility(_, prefix) => prefix,
            DeclarationData::Perspective(_, prefix) => prefix,
            DeclarationData::PerspectiveOrigin(_, prefix) => prefix,
            DeclarationData::Translate(..) => VendorPrefix::NONE,
            DeclarationData::Rotate(..) => VendorPrefix::NONE,
            DeclarationData::Scale(..) => VendorPrefix::NONE,
            DeclarationData::TextTransform(..) => VendorPrefix::NONE,
            DeclarationData::Content(..) => VendorPrefix::NONE,
            DeclarationData::WhiteSpace(..) => VendorPrefix::NONE,
            DeclarationData::TabSize(_, prefix) => prefix,
            DeclarationData::WordBreak(..) => VendorPrefix::NONE,
            DeclarationData::LineBreak(..) => VendorPrefix::NONE,
            DeclarationData::Hyphens(_, prefix) => prefix,
            DeclarationData::OverflowWrap(..) => VendorPrefix::NONE,
            DeclarationData::WordWrap(..) => VendorPrefix::NONE,
            DeclarationData::TextAlign(..) => VendorPrefix::NONE,
            DeclarationData::TextAlignLast(_, prefix) => prefix,
            DeclarationData::TextJustify(..) => VendorPrefix::NONE,
            DeclarationData::WordSpacing(..) => VendorPrefix::NONE,
            DeclarationData::LetterSpacing(..) => VendorPrefix::NONE,
            DeclarationData::TextIndent(..) => VendorPrefix::NONE,
            DeclarationData::TextDecorationLine(_, prefix) => prefix,
            DeclarationData::TextDecorationStyle(_, prefix) => prefix,
            DeclarationData::TextDecorationColor(_, prefix) => prefix,
            DeclarationData::TextDecorationThickness(..) => VendorPrefix::NONE,
            DeclarationData::TextDecoration(_, prefix) => prefix,
            DeclarationData::TextDecorationSkipInk(_, prefix) => prefix,
            DeclarationData::TextEmphasisStyle(_, prefix) => prefix,
            DeclarationData::TextEmphasisColor(_, prefix) => prefix,
            DeclarationData::TextEmphasis(_, prefix) => prefix,
            DeclarationData::TextEmphasisPosition(_, prefix) => prefix,
            DeclarationData::TextShadow(..) => VendorPrefix::NONE,
            DeclarationData::TextSizeAdjust(_, prefix) => prefix,
            DeclarationData::Direction(..) => VendorPrefix::NONE,
            DeclarationData::UnicodeBidi(..) => VendorPrefix::NONE,
            DeclarationData::BoxDecorationBreak(_, prefix) => prefix,
            DeclarationData::Resize(..) => VendorPrefix::NONE,
            DeclarationData::PointerEvents(..) => VendorPrefix::NONE,
            DeclarationData::Float(..) => VendorPrefix::NONE,
            DeclarationData::Clear(..) => VendorPrefix::NONE,
            DeclarationData::TouchAction(..) => VendorPrefix::NONE,
            DeclarationData::ScrollBehavior(..) => VendorPrefix::NONE,
            DeclarationData::Cursor(..) => VendorPrefix::NONE,
            DeclarationData::CaretColor(..) => VendorPrefix::NONE,
            DeclarationData::CaretShape(..) => VendorPrefix::NONE,
            DeclarationData::Caret(..) => VendorPrefix::NONE,
            DeclarationData::UserSelect(_, prefix) => prefix,
            DeclarationData::AccentColor(..) => VendorPrefix::NONE,
            DeclarationData::Appearance(_, prefix) => prefix,
            DeclarationData::ListStyleType(..) => VendorPrefix::NONE,
            DeclarationData::ListStyleImage(..) => VendorPrefix::NONE,
            DeclarationData::ListStylePosition(..) => VendorPrefix::NONE,
            DeclarationData::ListStyle(..) => VendorPrefix::NONE,
            DeclarationData::MarkerSide(..) => VendorPrefix::NONE,
            DeclarationData::Composes(..) => VendorPrefix::NONE,
            DeclarationData::Fill(..) => VendorPrefix::NONE,
            DeclarationData::FillRule(..) => VendorPrefix::NONE,
            DeclarationData::FillOpacity(..) => VendorPrefix::NONE,
            DeclarationData::Stroke(..) => VendorPrefix::NONE,
            DeclarationData::StrokeOpacity(..) => VendorPrefix::NONE,
            DeclarationData::StrokeWidth(..) => VendorPrefix::NONE,
            DeclarationData::StrokeLinecap(..) => VendorPrefix::NONE,
            DeclarationData::StrokeLinejoin(..) => VendorPrefix::NONE,
            DeclarationData::StrokeMiterlimit(..) => VendorPrefix::NONE,
            DeclarationData::StrokeDasharray(..) => VendorPrefix::NONE,
            DeclarationData::StrokeDashoffset(..) => VendorPrefix::NONE,
            DeclarationData::MarkerStart(..) => VendorPrefix::NONE,
            DeclarationData::MarkerMid(..) => VendorPrefix::NONE,
            DeclarationData::MarkerEnd(..) => VendorPrefix::NONE,
            DeclarationData::Marker(..) => VendorPrefix::NONE,
            DeclarationData::ColorInterpolation(..) => VendorPrefix::NONE,
            DeclarationData::ColorInterpolationFilters(..) => VendorPrefix::NONE,
            DeclarationData::ColorRendering(..) => VendorPrefix::NONE,
            DeclarationData::ShapeRendering(..) => VendorPrefix::NONE,
            DeclarationData::TextRendering(..) => VendorPrefix::NONE,
            DeclarationData::ImageRendering(..) => VendorPrefix::NONE,
            DeclarationData::ClipPath(_, prefix) => prefix,
            DeclarationData::ClipRule(..) => VendorPrefix::NONE,
            DeclarationData::MaskImage(_, prefix) => prefix,
            DeclarationData::MaskMode(..) => VendorPrefix::NONE,
            DeclarationData::MaskRepeat(_, prefix) => prefix,
            DeclarationData::MaskPositionX(..) => VendorPrefix::NONE,
            DeclarationData::MaskPositionY(..) => VendorPrefix::NONE,
            DeclarationData::MaskPosition(_, prefix) => prefix,
            DeclarationData::MaskClip(_, prefix) => prefix,
            DeclarationData::MaskOrigin(_, prefix) => prefix,
            DeclarationData::MaskSize(_, prefix) => prefix,
            DeclarationData::MaskComposite(..) => VendorPrefix::NONE,
            DeclarationData::MaskType(..) => VendorPrefix::NONE,
            DeclarationData::Mask(_, prefix) => prefix,
            DeclarationData::MaskBorderSource(..) => VendorPrefix::NONE,
            DeclarationData::MaskBorderMode(..) => VendorPrefix::NONE,
            DeclarationData::MaskBorderSlice(..) => VendorPrefix::NONE,
            DeclarationData::MaskBorderWidth(..) => VendorPrefix::NONE,
            DeclarationData::MaskBorderOutset(..) => VendorPrefix::NONE,
            DeclarationData::MaskBorderRepeat(..) => VendorPrefix::NONE,
            DeclarationData::MaskBorder(..) => VendorPrefix::NONE,
            DeclarationData::WebKitMaskComposite(..) => VendorPrefix::NONE,
            DeclarationData::WebKitMaskSourceType(_, prefix) => prefix,
            DeclarationData::WebKitMaskBoxImage(_, prefix) => prefix,
            DeclarationData::WebKitMaskBoxImageSource(_, prefix) => prefix,
            DeclarationData::WebKitMaskBoxImageSlice(_, prefix) => prefix,
            DeclarationData::WebKitMaskBoxImageWidth(_, prefix) => prefix,
            DeclarationData::WebKitMaskBoxImageOutset(_, prefix) => prefix,
            DeclarationData::WebKitMaskBoxImageRepeat(_, prefix) => prefix,
            DeclarationData::Filter(_, prefix) => prefix,
            DeclarationData::BackdropFilter(_, prefix) => prefix,
            DeclarationData::MixBlendMode(..) => VendorPrefix::NONE,
            DeclarationData::ZIndex(..) => VendorPrefix::NONE,
            DeclarationData::ContainerType(..) => VendorPrefix::NONE,
            DeclarationData::ContainerName(..) => VendorPrefix::NONE,
            DeclarationData::Container(..) => VendorPrefix::NONE,
            DeclarationData::ViewTransitionName(..) => VendorPrefix::NONE,
            DeclarationData::ViewTransitionClass(..) => VendorPrefix::NONE,
            DeclarationData::ViewTransitionGroup(..) => VendorPrefix::NONE,
            DeclarationData::ColorScheme(..) => VendorPrefix::NONE,
            DeclarationData::PrintColorAdjust(_, prefix) => prefix,
            DeclarationData::CSSWide(property_id, _) => property_id.vendor_prefix(),
            DeclarationData::Unparsed(value) => value.property_id(ast).vendor_prefix(),
            DeclarationData::Custom(_) | DeclarationData::Tombstone => VendorPrefix::NONE,
        }
    }
    /// Returns whether this declaration slot is an in-place tombstone.
    #[inline]
    pub fn is_tombstone(self, ast: &Ast<'ast>) -> bool {
        matches!(self.data(ast), DeclarationData::Tombstone)
    }
}
