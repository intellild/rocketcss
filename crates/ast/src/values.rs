use super::*;

#[derive(Debug, PartialEq)]
pub enum Image<'a> {
    None,
    Url(Box<'a, Url<'a>>),
    Gradient(Box<'a, Gradient<'a>>),
    ImageSet(Box<'a, ImageSet<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Gradient<'a> {
    Linear {
        direction: Box<'a, LineDirection<'a>>,
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingLinear {
        direction: Box<'a, LineDirection<'a>>,
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    Radial {
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        position: Box<'a, Position<'a>>,
        shape: Box<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingRadial {
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        position: Box<'a, Position<'a>>,
        shape: Box<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    Conic {
        angle: Box<'a, Angle>,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: Box<'a, Position<'a>>,
    },
    RepeatingConic {
        angle: Box<'a, Angle>,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: Box<'a, Position<'a>>,
    },
    WebKitGradient(Box<'a, WebKitGradient<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum WebKitGradient<'a> {
    Linear {
        from: Box<'a, WebKitGradientPoint<'a>>,
        to: Box<'a, WebKitGradientPoint<'a>>,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
    Radial {
        from: Box<'a, WebKitGradientPoint<'a>>,
        start_radius: f32,
        to: Box<'a, WebKitGradientPoint<'a>>,
        end_radius: f32,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum LineDirection<'a> {
    Angle(Box<'a, Angle>),
    Horizontal(HorizontalPositionKeyword),
    Vertical(VerticalPositionKeyword),
    Corner {
        horizontal: HorizontalPositionKeyword,
        vertical: VerticalPositionKeyword,
    },
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum VerticalPositionKeyword {
    Top,
    Bottom,
}

#[derive(Debug, PartialEq)]
pub enum GradientItem<'a, D> {
    ColorStop {
        color: Box<'a, CssColor<'a>>,
        position: Option<Box<'a, DimensionPercentage<'a, D>>>,
    },
    Hint(Box<'a, DimensionPercentage<'a, D>>),
}

#[derive(Debug, PartialEq)]
pub enum DimensionPercentage<'a, D> {
    Dimension(Box<'a, D>),
    Percentage(f32),
    /// A unitless zero produced by target-aware minification.
    Zero,
    Calc(Box<'a, Calc<'a, DimensionPercentage<'a, D>>>),
}

pub type LengthPercentage<'a> = DimensionPercentage<'a, LengthValue>;
pub type AnglePercentage<'a> = DimensionPercentage<'a, Angle>;

#[derive(Debug, PartialEq)]
pub enum PositionComponent<'a, S> {
    Center,
    Length(Box<'a, LengthPercentage<'a>>),
    Side {
        offset: Option<Box<'a, LengthPercentage<'a>>>,
        side: S,
    },
}

#[derive(Debug, PartialEq)]
pub enum EndingShape<'a> {
    Ellipse(Box<'a, Ellipse<'a>>),
    Circle(Box<'a, Circle<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Ellipse<'a> {
    Size {
        x: Box<'a, LengthPercentage<'a>>,
        y: Box<'a, LengthPercentage<'a>>,
    },
    Extent(ShapeExtent),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ShapeExtent {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
}

#[derive(Debug, PartialEq)]
pub enum Circle<'a> {
    Radius(Box<'a, Length<'a>>),
    Extent(ShapeExtent),
}

#[derive(Debug, PartialEq)]
pub enum WebKitGradientPointComponent<'a, S> {
    Center,
    Number(Box<'a, NumberOrPercentage>),
    Side(S),
}

#[derive(Debug, PartialEq)]
pub enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}

#[derive(Debug, PartialEq)]
pub enum BackgroundSize<'a> {
    Explicit {
        height: Box<'a, LengthPercentageOrAuto<'a>>,
        width: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Cover,
    Contain,
}

#[derive(Debug, PartialEq)]
pub enum LengthPercentageOrAuto<'a> {
    Auto,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Border,
    Text,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}

#[derive(Debug, PartialEq)]
pub enum Display<'a> {
    Keyword(DisplayKeyword),
    Pair {
        inside: Box<'a, DisplayInside>,
        is_list_item: bool,
        outside: DisplayOutside,
    },
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum DisplayKeyword {
    None,
    Contents,
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
}

#[derive(Debug, PartialEq)]
pub enum DisplayInside {
    Flow,
    FlowRoot,
    Table,
    Flex { vendor_prefix: VendorPrefix },
    Box { vendor_prefix: VendorPrefix },
    Grid,
    Ruby,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum DisplayOutside {
    Block,
    Inline,
    RunIn,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

#[derive(Debug, PartialEq)]
pub enum Size<'a> {
    Auto,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(Box<'a, LengthPercentage<'a>>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}

#[derive(Debug, PartialEq)]
pub enum MaxSize<'a> {
    None,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(Box<'a, LengthPercentage<'a>>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum OverflowKeyword {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Debug, PartialEq)]
pub enum PositionProperty {
    Static,
    Relative,
    Absolute,
    Sticky(VendorPrefix),
    Fixed,
}

#[derive(Debug, PartialEq)]
pub struct Size2D<'a, T>(pub Box<'a, T>, pub Box<'a, T>);

#[derive(Debug, PartialEq)]
pub struct Rect<'a, T>(
    pub Box<'a, T>,
    pub Box<'a, T>,
    pub Box<'a, T>,
    pub Box<'a, T>,
);

#[derive(CssKeyword, Debug, PartialEq)]
pub enum LineStyle {
    None,
    Hidden,
    Inset,
    Groove,
    Outset,
    Ridge,
    Dotted,
    Dashed,
    Solid,
    Double,
}

#[derive(Debug, PartialEq)]
pub enum BorderSideWidth<'a> {
    Thin,
    Medium,
    Thick,
    Length(Box<'a, Length<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum LengthOrNumber<'a> {
    Number(f32),
    Length(Box<'a, Length<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[derive(Debug, PartialEq)]
pub enum BorderImageSideWidth<'a> {
    Number(f32),
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    Auto,
}

#[derive(Debug, PartialEq)]
pub enum OutlineStyle {
    Auto,
    LineStyle(LineStyle),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, PartialEq)]
pub enum AlignContent {
    Normal,
    BaselinePosition(BaselinePosition),
    ContentDistribution(ContentDistribution),
    ContentPosition {
        overflow: Option<OverflowPosition>,
        value: ContentPosition,
    },
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BaselinePosition {
    First,
    Last,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ContentDistribution {
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum OverflowPosition {
    Safe,
    Unsafe,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ContentPosition {
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
}

#[derive(Debug, PartialEq)]
pub enum JustifyContent {
    Normal,
    ContentDistribution(ContentDistribution),
    ContentPosition {
        overflow: Option<OverflowPosition>,
        value: ContentPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
}

#[derive(Debug, PartialEq)]
pub enum AlignSelf {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum SelfPosition {
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    FlexStart,
    FlexEnd,
}

#[derive(Debug, PartialEq)]
pub enum JustifySelf {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
}

#[derive(Debug, PartialEq)]
pub enum AlignItems {
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
}

#[derive(Debug, PartialEq)]
pub enum JustifyItems {
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
    Legacy(LegacyJustify),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum LegacyJustify {
    Left,
    Right,
    Center,
}

#[derive(Debug, PartialEq)]
pub enum GapValue<'a> {
    Normal,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxOrient {
    Horizontal,
    Vertical,
    InlineAxis,
    BlockAxis,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxDirection {
    Normal,
    Reverse,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxAlign {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxPack {
    Start,
    End,
    Center,
    Justify,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxLines {
    Single,
    Multiple,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexPack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexItemAlign {
    Auto,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexLinePack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
    Stretch,
}

#[derive(Debug, PartialEq)]
pub enum TrackSizing<'a> {
    None,
    TrackList {
        items: Vec<'a, TrackListItem<'a>>,
        line_names: Vec<'a, Vec<'a, &'a str>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum TrackListItem<'a> {
    TrackSize(Box<'a, TrackSize<'a>>),
    TrackRepeat(Box<'a, TrackRepeat<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum TrackSize<'a> {
    TrackBreadth(Box<'a, TrackBreadth<'a>>),
    MinMax {
        max: Box<'a, TrackBreadth<'a>>,
        min: Box<'a, TrackBreadth<'a>>,
    },
    FitContent(Box<'a, LengthPercentage<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum TrackBreadth<'a> {
    Length(Box<'a, LengthPercentage<'a>>),
    Flex(f32),
    MinContent,
    MaxContent,
    Auto,
}

#[derive(Debug, PartialEq)]
pub enum RepeatCount {
    Number(f32),
    AutoFill,
    AutoFit,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum AutoFlowDirection {
    Row,
    Column,
}

#[derive(Debug, PartialEq)]
pub enum GridTemplateAreas<'a> {
    None,
    Areas {
        areas: Vec<'a, Option<&'a str>>,
        columns: u32,
    },
}

#[derive(Debug, PartialEq)]
pub enum GridLine<'a> {
    Auto,
    Area { name: &'a str },
    Line { index: i32, name: Option<&'a str> },
    Span { index: i32, name: Option<&'a str> },
}

#[derive(Debug, PartialEq)]
pub enum FontWeight<'a> {
    Absolute(Box<'a, AbsoluteFontWeight>),
    Bolder,
    Lighter,
}

#[derive(Debug, PartialEq)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}

#[derive(Debug, PartialEq)]
pub enum FontSize<'a> {
    Length(Box<'a, LengthPercentage<'a>>),
    Absolute(AbsoluteFontSize),
    Relative(RelativeFontSize),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum AbsoluteFontSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
    XxxLarge,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum RelativeFontSize {
    Smaller,
    Larger,
}

#[derive(Debug, PartialEq)]
pub enum FontStretch {
    Keyword(FontStretchKeyword),
    Percentage(f32),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FontStretchKeyword {
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Debug, PartialEq)]
pub enum FontFamily<'a> {
    Generic(GenericFontFamily),
    FamilyName(FamilyName<'a>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum GenericFontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    SystemUi,
    Emoji,
    Math,
    Fangsong,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
    Initial,
    Inherit,
    Unset,
    Default,
    Revert,
    RevertLayer,
}

#[derive(Debug, PartialEq)]
pub enum FontStyle<'a> {
    Normal,
    Italic,
    Oblique(Box<'a, Angle>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

#[derive(Debug, PartialEq)]
pub enum LineHeight<'a> {
    Normal,
    Number(f32),
    Length(Box<'a, LengthPercentage<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum VerticalAlign<'a> {
    Keyword(VerticalAlignKeyword),
    Length(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum VerticalAlignKeyword {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
}

#[derive(Debug, PartialEq)]
pub enum EasingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier { x1: f32, x2: f32, y1: f32, y2: f32 },
    Steps { count: i32, position: StepPosition },
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum StepPosition {
    Start,
    End,
    JumpNone,
    JumpBoth,
}

#[derive(Debug, PartialEq)]
pub enum AnimationIterationCount {
    Number(f32),
    Infinite,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum AnimationComposition {
    Replace,
    Add,
    Accumulate,
}

#[derive(Debug, PartialEq)]
pub enum AnimationTimeline<'a> {
    Auto,
    None,
    DashedIdent(&'a str),
    Scroll(Box<'a, ScrollTimeline>),
    View(Box<'a, ViewTimeline<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ScrollAxis {
    Block,
    Inline,
    X,
    Y,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum Scroller {
    Root,
    Nearest,
    Self_,
}

pub type AnimationRangeStart<'a> = AnimationAttachmentRange<'a>;

#[derive(Debug, PartialEq)]
pub enum AnimationAttachmentRange<'a> {
    Normal,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    TimelineRange {
        name: TimelineRangeName,
        offset: Box<'a, LengthPercentage<'a>>,
    },
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TimelineRangeName {
    Cover,
    Contain,
    Entry,
    Exit,
    EntryCrossing,
    ExitCrossing,
}

pub type AnimationRangeEnd<'a> = AnimationAttachmentRange<'a>;

#[derive(Debug, PartialEq)]
pub enum Transform<'a> {
    Translate((Box<'a, LengthPercentage<'a>>, Box<'a, LengthPercentage<'a>>)),
    TranslateX(Box<'a, LengthPercentage<'a>>),
    TranslateY(Box<'a, LengthPercentage<'a>>),
    TranslateZ(Box<'a, Length<'a>>),
    Translate3d(
        (
            Box<'a, LengthPercentage<'a>>,
            Box<'a, LengthPercentage<'a>>,
            Box<'a, Length<'a>>,
        ),
    ),
    Scale((Box<'a, NumberOrPercentage>, Box<'a, NumberOrPercentage>)),
    ScaleX(Box<'a, NumberOrPercentage>),
    ScaleY(Box<'a, NumberOrPercentage>),
    ScaleZ(Box<'a, NumberOrPercentage>),
    Scale3d(
        (
            Box<'a, NumberOrPercentage>,
            Box<'a, NumberOrPercentage>,
            Box<'a, NumberOrPercentage>,
        ),
    ),
    Rotate(Box<'a, Angle>),
    RotateX(Box<'a, Angle>),
    RotateY(Box<'a, Angle>),
    RotateZ(Box<'a, Angle>),
    Rotate3d((f32, f32, f32, Box<'a, Angle>)),
    Skew((Box<'a, Angle>, Box<'a, Angle>)),
    SkewX(Box<'a, Angle>),
    SkewY(Box<'a, Angle>),
    Perspective(Box<'a, Length<'a>>),
    Matrix(Box<'a, MatrixForFloat>),
    Matrix3d(Box<'a, Matrix3DForFloat>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TransformStyle {
    Flat,
    Preserve3d,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TransformBox {
    ContentBox,
    BorderBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}

#[derive(Debug, PartialEq)]
pub enum Perspective<'a> {
    None,
    Length(Box<'a, Length<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Translate<'a> {
    None,
    Xyz {
        x: Box<'a, LengthPercentage<'a>>,
        y: Box<'a, LengthPercentage<'a>>,
        z: Box<'a, Length<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Scale<'a> {
    None,
    Xyz {
        x: Box<'a, NumberOrPercentage>,
        y: Box<'a, NumberOrPercentage>,
        z: Box<'a, NumberOrPercentage>,
    },
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextTransformCase {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum WhiteSpace {
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    BreakSpaces,
    PreLine,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum WordBreak {
    Normal,
    KeepAll,
    BreakAll,
    BreakWord,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum LineBreak {
    Auto,
    Loose,
    Normal,
    Strict,
    Anywhere,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum Hyphens {
    None,
    Manual,
    Auto,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum OverflowWrap {
    Normal,
    Anywhere,
    BreakWord,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    MatchParent,
    JustifyAll,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextAlignLast {
    Auto,
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    MatchParent,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextJustify {
    Auto,
    None,
    InterWord,
    InterCharacter,
}

#[derive(Debug, PartialEq)]
pub enum Spacing<'a> {
    Normal,
    Length(Box<'a, Length<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum TextDecorationLine<'a> {
    ExclusiveTextDecorationLine(ExclusiveTextDecorationLine),
    Value(Vec<'a, OtherTextDecorationLine>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ExclusiveTextDecorationLine {
    None,
    SpellingError,
    GrammarError,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum OtherTextDecorationLine {
    Underline,
    Overline,
    LineThrough,
    Blink,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Debug, PartialEq)]
pub enum TextDecorationThickness<'a> {
    Auto,
    FromFont,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextDecorationSkipInk {
    Auto,
    None,
    All,
}

#[derive(Debug, PartialEq)]
pub enum TextEmphasisStyle<'a> {
    None,
    Keyword {
        fill: TextEmphasisFillMode,
        shape: Option<TextEmphasisShape>,
    },
    String(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextEmphasisFillMode {
    Filled,
    Open,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextEmphasisShape {
    Dot,
    Circle,
    DoubleCircle,
    Triangle,
    Sesame,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextEmphasisPositionHorizontal {
    Left,
    Right,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextEmphasisPositionVertical {
    Over,
    Under,
}

#[derive(Debug, PartialEq)]
pub enum TextSizeAdjust {
    Auto,
    None,
    Percentage(f32),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum CursorKeyword {
    Auto,
    Default,
    None,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}

#[derive(Debug, PartialEq)]
pub enum ColorOrAuto<'a> {
    Auto,
    Color(Box<'a, CssColor<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum CaretShape {
    Auto,
    Bar,
    Block,
    Underscore,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum UserSelect {
    Auto,
    Text,
    None,
    Contain,
    All,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum Appearance<'a> {
    None,
    Auto,
    Textfield,
    MenulistButton,
    Button,
    Checkbox,
    Listbox,
    Menulist,
    Meter,
    ProgressBar,
    PushButton,
    Radio,
    Searchfield,
    SliderHorizontal,
    SquareButton,
    Textarea,
    NonStandard(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum ListStyleType<'a> {
    None,
    String(&'a str),
    CounterStyle(Box<'a, CounterStyle<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum CounterStyle<'a> {
    Predefined(PredefinedCounterStyle),
    Name(&'a str),
    Symbols {
        symbols: Vec<'a, Symbol<'a>>,
        system: SymbolsType,
    },
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SymbolsType {
    Cyclic,
    Numeric,
    Alphabetic,
    #[default]
    Symbolic,
    Fixed,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum PredefinedCounterStyle {
    Decimal,
    DecimalLeadingZero,
    ArabicIndic,
    Armenian,
    UpperArmenian,
    LowerArmenian,
    Bengali,
    Cambodian,
    Khmer,
    CjkDecimal,
    Devanagari,
    Georgian,
    Gujarati,
    Gurmukhi,
    Hebrew,
    Kannada,
    Lao,
    Malayalam,
    Mongolian,
    Myanmar,
    Oriya,
    Persian,
    LowerRoman,
    UpperRoman,
    Tamil,
    Telugu,
    Thai,
    Tibetan,
    LowerAlpha,
    LowerLatin,
    UpperAlpha,
    UpperLatin,
    LowerGreek,
    Hiragana,
    HiraganaIroha,
    Katakana,
    KatakanaIroha,
    Disc,
    Circle,
    Square,
    DisclosureOpen,
    DisclosureClosed,
    CjkEarthlyBranch,
    CjkHeavenlyStem,
    JapaneseInformal,
    JapaneseFormal,
    KoreanHangulFormal,
    KoreanHanjaInformal,
    KoreanHanjaFormal,
    SimpChineseInformal,
    SimpChineseFormal,
    TradChineseInformal,
    TradChineseFormal,
    EthiopicNumeric,
}

#[derive(Debug, PartialEq)]
pub enum Symbol<'a> {
    String(&'a str),
    Image(Box<'a, Image<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum MarkerSide {
    MatchSelf,
    MatchParent,
}

#[derive(Debug, PartialEq)]
pub enum SVGPaint<'a> {
    Url {
        fallback: Option<Box<'a, SVGPaintFallback<'a>>>,
        url: Box<'a, Url<'a>>,
    },
    Color(Box<'a, CssColor<'a>>),
    ContextFill,
    ContextStroke,
    None,
}

#[derive(Debug, PartialEq)]
pub enum SVGPaintFallback<'a> {
    None,
    Color(Box<'a, CssColor<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum StrokeLinecap {
    Butt,
    Round,
    Square,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum StrokeLinejoin {
    Miter,
    MiterClip,
    Round,
    Bevel,
    Arcs,
}

#[derive(Debug, PartialEq)]
pub enum StrokeDasharray<'a> {
    None,
    Values(Vec<'a, LengthPercentage<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Marker<'a> {
    None,
    Url(Box<'a, Url<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ColorInterpolation {
    Auto,
    Srgb,
    Linearrgb,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ColorRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ShapeRendering {
    Auto,
    Optimizespeed,
    Crispedges,
    Geometricprecision,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextRendering {
    Auto,
    Optimizespeed,
    Optimizelegibility,
    Geometricprecision,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ImageRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}

#[derive(Debug, PartialEq)]
pub enum ClipPath<'a> {
    None,
    Url(Box<'a, Url<'a>>),
    Shape {
        reference_box: GeometryBox,
        shape: Box<'a, BasicShape<'a>>,
    },
    Box(GeometryBox),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum GeometryBox {
    BorderBox,
    PaddingBox,
    ContentBox,
    MarginBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

#[derive(Debug, PartialEq)]
pub enum BasicShape<'a> {
    Inset(Box<'a, InsetRect<'a>>),
    Circle(Box<'a, CircleShape<'a>>),
    Ellipse(Box<'a, EllipseShape<'a>>),
    Polygon(Box<'a, Polygon<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum ShapeRadius<'a> {
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    ClosestSide,
    FarthestSide,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum MaskMode {
    Luminance,
    Alpha,
    MatchSource,
}

#[derive(Debug, PartialEq)]
pub enum MaskClip {
    GeometryBox(GeometryBox),
    NoClip,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum MaskComposite {
    Add,
    Subtract,
    Intersect,
    Exclude,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum MaskType {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum MaskBorderMode {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum WebKitMaskComposite {
    Clear,
    Copy,
    SourceOver,
    SourceIn,
    SourceOut,
    SourceAtop,
    DestinationOver,
    DestinationIn,
    DestinationOut,
    DestinationAtop,
    Xor,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum WebKitMaskSourceType {
    Auto,
    Luminance,
    Alpha,
}

#[derive(Debug, PartialEq)]
pub enum FilterList<'a> {
    None,
    Filters(Vec<'a, Filter<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Filter<'a> {
    Blur(Box<'a, Length<'a>>),
    Brightness(Box<'a, NumberOrPercentage>),
    Contrast(Box<'a, NumberOrPercentage>),
    Grayscale(Box<'a, NumberOrPercentage>),
    HueRotate(Box<'a, Angle>),
    Invert(Box<'a, NumberOrPercentage>),
    Opacity(Box<'a, NumberOrPercentage>),
    Saturate(Box<'a, NumberOrPercentage>),
    Sepia(Box<'a, NumberOrPercentage>),
    DropShadow(Box<'a, DropShadow<'a>>),
    Url(Box<'a, Url<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum ZIndex {
    Auto,
    Integer(i32),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ContainerType {
    Normal,
    InlineSize,
    Size,
    ScrollState,
}

#[derive(Debug, PartialEq)]
pub enum ContainerNameList<'a> {
    None,
    Names(Vec<'a, &'a str>),
}

#[derive(Debug, PartialEq)]
pub enum ViewTransitionName<'a> {
    None,
    Auto,
    Custom(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum NoneOrCustomIdentList<'a> {
    None,
    Idents(Vec<'a, &'a str>),
}

#[derive(Debug, PartialEq)]
pub enum ViewTransitionGroup<'a> {
    Normal,
    Contain,
    Nearest,
    Custom(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum PrintColorAdjust {
    Economy,
    Exact,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum CSSWideKeyword {
    Initial,
    Inherit,
    Unset,
    Revert,
    RevertLayer,
}

#[derive(Debug, PartialEq)]
pub enum CustomPropertyName<'a> {
    Custom(&'a str),
    Unknown(&'a str),
}
