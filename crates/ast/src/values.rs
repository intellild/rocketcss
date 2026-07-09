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
        items: Vec<'a, Box<'a, GradientItemFor_DimensionPercentageFor_LengthValue<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    RepeatingLinear {
        direction: Box<'a, LineDirection<'a>>,
        items: Vec<'a, Box<'a, GradientItemFor_DimensionPercentageFor_LengthValue<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Radial {
        items: Vec<'a, Box<'a, GradientItemFor_DimensionPercentageFor_LengthValue<'a>>>,
        position: Box<'a, Position<'a>>,
        shape: Box<'a, EndingShape<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    RepeatingRadial {
        items: Vec<'a, Box<'a, GradientItemFor_DimensionPercentageFor_LengthValue<'a>>>,
        position: Box<'a, Position<'a>>,
        shape: Box<'a, EndingShape<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Conic {
        angle: Box<'a, Angle>,
        items: Vec<'a, Box<'a, GradientItemFor_DimensionPercentageFor_Angle<'a>>>,
        position: Box<'a, Position<'a>>,
    },
    RepeatingConic {
        angle: Box<'a, Angle>,
        items: Vec<'a, Box<'a, GradientItemFor_DimensionPercentageFor_Angle<'a>>>,
        position: Box<'a, Position<'a>>,
    },
    Value(()),
}

#[derive(Debug, PartialEq)]
pub enum LineDirection<'a> {
    Angle(Box<'a, Angle>),
    Horizontal(Box<'a, HorizontalPositionKeyword>),
    Vertical(Box<'a, VerticalPositionKeyword>),
    Corner {
        horizontal: Box<'a, HorizontalPositionKeyword>,
        vertical: Box<'a, VerticalPositionKeyword>,
    },
}

#[derive(Debug, PartialEq)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum VerticalPositionKeyword {
    Top,
    Bottom,
}

#[derive(Debug, PartialEq)]
pub enum GradientItemFor_DimensionPercentageFor_LengthValue<'a> {
    ColorStop {
        color: Box<'a, CssColor<'a>>,
        position: Option<Box<'a, DimensionPercentageFor_LengthValue<'a>>>,
    },
    Hint(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum DimensionPercentageFor_LengthValue<'a> {
    Dimension(Box<'a, LengthValue<'a>>),
    Percentage(f64),
    Calc(Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum CalcFor_DimensionPercentageFor_LengthValue<'a> {
    Value(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Number(f64),
    Sum(
        (
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
        ),
    ),
    Product((f64, Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>)),
    Function(Box<'a, MathFunctionFor_DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum MathFunctionFor_DimensionPercentageFor_LengthValue<'a> {
    Calc(Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>),
    Min(Vec<'a, Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>>),
    Max(Vec<'a, Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>>),
    Clamp(
        (
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
        ),
    ),
    Round(
        (
            Box<'a, RoundingStrategy>,
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
        ),
    ),
    Rem(
        (
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
        ),
    ),
    Mod(
        (
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>,
        ),
    ),
    Abs(Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>),
    Sign(Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>),
    Hypot(Vec<'a, Box<'a, CalcFor_DimensionPercentageFor_LengthValue<'a>>>),
}

#[derive(Debug, PartialEq)]
pub enum PositionComponentFor_HorizontalPositionKeyword<'a> {
    Center,
    Length(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Side {
        offset: Option<Box<'a, DimensionPercentageFor_LengthValue<'a>>>,
        side: Box<'a, HorizontalPositionKeyword>,
    },
}

#[derive(Debug, PartialEq)]
pub enum PositionComponentFor_VerticalPositionKeyword<'a> {
    Center,
    Length(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Side {
        offset: Option<Box<'a, DimensionPercentageFor_LengthValue<'a>>>,
        side: Box<'a, VerticalPositionKeyword>,
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
        x: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
        y: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    },
    Extent(Box<'a, ShapeExtent>),
}

#[derive(Debug, PartialEq)]
pub enum ShapeExtent {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
}

#[derive(Debug, PartialEq)]
pub enum Circle<'a> {
    Radius(Box<'a, Length<'a>>),
    Extent(Box<'a, ShapeExtent>),
}

#[derive(Debug, PartialEq)]
pub enum GradientItemFor_DimensionPercentageFor_Angle<'a> {
    ColorStop {
        color: Box<'a, CssColor<'a>>,
        position: Option<Box<'a, DimensionPercentageFor_Angle<'a>>>,
    },
    Hint(Box<'a, DimensionPercentageFor_Angle<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum DimensionPercentageFor_Angle<'a> {
    Dimension(Box<'a, Angle>),
    Percentage(f64),
    Calc(Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum CalcFor_DimensionPercentageFor_Angle<'a> {
    Value(Box<'a, DimensionPercentageFor_Angle<'a>>),
    Number(f64),
    Sum(
        (
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
        ),
    ),
    Product((f64, Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>)),
    Function(Box<'a, MathFunctionFor_DimensionPercentageFor_Angle<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum MathFunctionFor_DimensionPercentageFor_Angle<'a> {
    Calc(Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>),
    Min(Vec<'a, Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>>),
    Max(Vec<'a, Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>>),
    Clamp(
        (
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
        ),
    ),
    Round(
        (
            Box<'a, RoundingStrategy>,
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
        ),
    ),
    Rem(
        (
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
        ),
    ),
    Mod(
        (
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
            Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>,
        ),
    ),
    Abs(Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>),
    Sign(Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>),
    Hypot(Vec<'a, Box<'a, CalcFor_DimensionPercentageFor_Angle<'a>>>),
}

#[derive(Debug, PartialEq)]
pub enum WebKitGradientPointComponentFor_HorizontalPositionKeyword<'a> {
    Center,
    Number(Box<'a, NumberOrPercentage>),
    Side(Box<'a, HorizontalPositionKeyword>),
}

#[derive(Debug, PartialEq)]
pub enum NumberOrPercentage {
    Number(f64),
    Percentage(f64),
}

#[derive(Debug, PartialEq)]
pub enum WebKitGradientPointComponentFor_VerticalPositionKeyword<'a> {
    Center,
    Number(Box<'a, NumberOrPercentage>),
    Side(Box<'a, VerticalPositionKeyword>),
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
    LengthPercentage(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(Debug, PartialEq)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

#[derive(Debug, PartialEq)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Border,
    Text,
}

#[derive(Debug, PartialEq)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}

#[derive(Debug, PartialEq)]
pub enum Display<'a> {
    Keyword(Box<'a, DisplayKeyword>),
    Pair {
        inside: Box<'a, DisplayInside<'a>>,
        is_list_item: bool,
        outside: Box<'a, DisplayOutside>,
    },
}

#[derive(Debug, PartialEq)]
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
pub enum DisplayInside<'a> {
    Flow,
    FlowRoot,
    Table,
    Flex {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Box {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Grid,
    Ruby,
}

#[derive(Debug, PartialEq)]
pub enum DisplayOutside {
    Block,
    Inline,
    RunIn,
}

#[derive(Debug, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

#[derive(Debug, PartialEq)]
pub enum Size<'a> {
    Auto,
    LengthPercentage(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    MinContent {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    MaxContent {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    FitContent {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    FitContentFunction(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Stretch {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Contain,
}

#[derive(Debug, PartialEq)]
pub enum MaxSize<'a> {
    None,
    LengthPercentage(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    MinContent {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    MaxContent {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    FitContent {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    FitContentFunction(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Stretch {
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Contain,
}

#[derive(Debug, PartialEq)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(Debug, PartialEq)]
pub enum OverflowKeyword {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

#[derive(Debug, PartialEq)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Debug, PartialEq)]
pub enum Position2<'a> {
    Static,
    Relative,
    Absolute,
    Sticky(Box<'a, VendorPrefix<'a>>),
    Fixed,
}

#[derive(Debug, PartialEq)]
pub struct Size2DFor_Length<'a>(pub Box<'a, Length<'a>>, pub Box<'a, Length<'a>>);

#[derive(Debug, PartialEq)]
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
pub struct Size2DFor_DimensionPercentageFor_LengthValue<'a>(
    pub Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    pub Box<'a, DimensionPercentageFor_LengthValue<'a>>,
);

#[derive(Debug, PartialEq)]
pub struct RectFor_LengthOrNumber<'a>(
    pub Box<'a, LengthOrNumber<'a>>,
    pub Box<'a, LengthOrNumber<'a>>,
    pub Box<'a, LengthOrNumber<'a>>,
    pub Box<'a, LengthOrNumber<'a>>,
);

#[derive(Debug, PartialEq)]
pub enum LengthOrNumber<'a> {
    Number(f64),
    Length(Box<'a, Length<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[derive(Debug, PartialEq)]
pub struct RectFor_BorderImageSideWidth<'a>(
    pub Box<'a, BorderImageSideWidth<'a>>,
    pub Box<'a, BorderImageSideWidth<'a>>,
    pub Box<'a, BorderImageSideWidth<'a>>,
    pub Box<'a, BorderImageSideWidth<'a>>,
);

#[derive(Debug, PartialEq)]
pub enum BorderImageSideWidth<'a> {
    Number(f64),
    LengthPercentage(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Auto,
}

#[derive(Debug, PartialEq)]
pub struct RectFor_NumberOrPercentage<'a>(
    pub Box<'a, NumberOrPercentage>,
    pub Box<'a, NumberOrPercentage>,
    pub Box<'a, NumberOrPercentage>,
    pub Box<'a, NumberOrPercentage>,
);

#[derive(Debug, PartialEq)]
pub enum OutlineStyle<'a> {
    Auto,
    LineStyle(Box<'a, LineStyle>),
}

#[derive(Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, PartialEq)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, PartialEq)]
pub enum AlignContent<'a> {
    Normal,
    BaselinePosition(Box<'a, BaselinePosition>),
    ContentDistribution(Box<'a, ContentDistribution>),
    ContentPosition {
        overflow: Option<Box<'a, OverflowPosition>>,
        value: Box<'a, ContentPosition>,
    },
}

#[derive(Debug, PartialEq)]
pub enum BaselinePosition {
    First,
    Last,
}

#[derive(Debug, PartialEq)]
pub enum ContentDistribution {
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
}

#[derive(Debug, PartialEq)]
pub enum OverflowPosition {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq)]
pub enum ContentPosition {
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
}

#[derive(Debug, PartialEq)]
pub enum JustifyContent<'a> {
    Normal,
    ContentDistribution(Box<'a, ContentDistribution>),
    ContentPosition {
        overflow: Option<Box<'a, OverflowPosition>>,
        value: Box<'a, ContentPosition>,
    },
    Left {
        overflow: Option<Box<'a, OverflowPosition>>,
    },
    Right {
        overflow: Option<Box<'a, OverflowPosition>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum AlignSelf<'a> {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(Box<'a, BaselinePosition>),
    SelfPosition {
        overflow: Option<Box<'a, OverflowPosition>>,
        value: Box<'a, SelfPosition>,
    },
}

#[derive(Debug, PartialEq)]
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
pub enum JustifySelf<'a> {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(Box<'a, BaselinePosition>),
    SelfPosition {
        overflow: Option<Box<'a, OverflowPosition>>,
        value: Box<'a, SelfPosition>,
    },
    Left {
        overflow: Option<Box<'a, OverflowPosition>>,
    },
    Right {
        overflow: Option<Box<'a, OverflowPosition>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum AlignItems<'a> {
    Normal,
    Stretch,
    BaselinePosition(Box<'a, BaselinePosition>),
    SelfPosition {
        overflow: Option<Box<'a, OverflowPosition>>,
        value: Box<'a, SelfPosition>,
    },
}

#[derive(Debug, PartialEq)]
pub enum JustifyItems<'a> {
    Normal,
    Stretch,
    BaselinePosition(Box<'a, BaselinePosition>),
    SelfPosition {
        overflow: Option<Box<'a, OverflowPosition>>,
        value: Box<'a, SelfPosition>,
    },
    Left {
        overflow: Option<Box<'a, OverflowPosition>>,
    },
    Right {
        overflow: Option<Box<'a, OverflowPosition>>,
    },
    Legacy(Box<'a, LegacyJustify>),
}

#[derive(Debug, PartialEq)]
pub enum LegacyJustify {
    Left,
    Right,
    Center,
}

#[derive(Debug, PartialEq)]
pub enum GapValue<'a> {
    Normal,
    LengthPercentage(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum BoxOrient {
    Horizontal,
    Vertical,
    InlineAxis,
    BlockAxis,
}

#[derive(Debug, PartialEq)]
pub enum BoxDirection {
    Normal,
    Reverse,
}

#[derive(Debug, PartialEq)]
pub enum BoxAlign {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[derive(Debug, PartialEq)]
pub enum BoxPack {
    Start,
    End,
    Center,
    Justify,
}

#[derive(Debug, PartialEq)]
pub enum BoxLines {
    Single,
    Multiple,
}

#[derive(Debug, PartialEq)]
pub enum FlexPack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
}

#[derive(Debug, PartialEq)]
pub enum FlexItemAlign {
    Auto,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[derive(Debug, PartialEq)]
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
        items: Vec<'a, Box<'a, TrackListItem<'a>>>,
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
    FitContent(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum TrackBreadth<'a> {
    Length(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Flex(f64),
    MinContent,
    MaxContent,
    Auto,
}

#[derive(Debug, PartialEq)]
pub enum RepeatCount {
    Number(f64),
    AutoFill,
    AutoFit,
}

#[derive(Debug, PartialEq)]
pub enum AutoFlowDirection {
    Row,
    Column,
}

#[derive(Debug, PartialEq)]
pub enum GridTemplateAreas<'a> {
    None,
    Areas {
        areas: Vec<'a, Option<&'a str>>,
        columns: f64,
    },
}

#[derive(Debug, PartialEq)]
pub enum GridLine<'a> {
    Auto,
    Area { name: &'a str },
    Line { index: f64, name: Option<&'a str> },
    Span { index: f64, name: Option<&'a str> },
}

#[derive(Debug, PartialEq)]
pub enum FontWeight<'a> {
    Absolute(Box<'a, AbsoluteFontWeight>),
    Bolder,
    Lighter,
}

#[derive(Debug, PartialEq)]
pub enum AbsoluteFontWeight {
    Weight(f64),
    Normal,
    Bold,
}

#[derive(Debug, PartialEq)]
pub enum FontSize<'a> {
    Length(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Absolute(Box<'a, AbsoluteFontSize>),
    Relative(Box<'a, RelativeFontSize>),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum RelativeFontSize {
    Smaller,
    Larger,
}

#[derive(Debug, PartialEq)]
pub enum FontStretch<'a> {
    Keyword(Box<'a, FontStretchKeyword>),
    Percentage(f64),
}

#[derive(Debug, PartialEq)]
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
    GenericFontFamily(Box<'a, GenericFontFamily>),
    CssString(&'a str),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    Number(f64),
    Length(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum VerticalAlign<'a> {
    Keyword(Box<'a, VerticalAlignKeyword>),
    Length(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
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
pub enum EasingFunction<'a> {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier {
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
    },
    Steps {
        count: f64,
        position: Option<Box<'a, StepPosition>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum StepPosition {
    Start,
    End,
    JumpNone,
    JumpBoth,
}

#[derive(Debug, PartialEq)]
pub enum AnimationIterationCount {
    Number(f64),
    Infinite,
}

#[derive(Debug, PartialEq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Debug, PartialEq)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

#[derive(Debug, PartialEq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Debug, PartialEq)]
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
    Scroll(Box<'a, ScrollTimeline<'a>>),
    View(Box<'a, ViewTimeline<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum ScrollAxis {
    Block,
    Inline,
    X,
    Y,
}

#[derive(Debug, PartialEq)]
pub enum Scroller {
    Root,
    Nearest,
    Self_,
}

#[derive(Debug, PartialEq)]
pub struct Size2DFor_LengthPercentageOrAuto<'a>(
    pub Box<'a, LengthPercentageOrAuto<'a>>,
    pub Box<'a, LengthPercentageOrAuto<'a>>,
);

pub type AnimationRangeStart<'a> = AnimationAttachmentRange<'a>;

#[derive(Debug, PartialEq)]
pub enum AnimationAttachmentRange<'a> {
    Normal,
    DimensionPercentageForLengthValue(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    Object {
        name: Box<'a, TimelineRangeName>,
        offset: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    },
}

#[derive(Debug, PartialEq)]
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
    Translate(
        (
            Box<'a, DimensionPercentageFor_LengthValue<'a>>,
            Box<'a, DimensionPercentageFor_LengthValue<'a>>,
        ),
    ),
    TranslateX(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    TranslateY(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    TranslateZ(Box<'a, Length<'a>>),
    Translate3d(
        (
            Box<'a, DimensionPercentageFor_LengthValue<'a>>,
            Box<'a, DimensionPercentageFor_LengthValue<'a>>,
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
    Rotate3d((f64, f64, f64, Box<'a, Angle>)),
    Skew((Box<'a, Angle>, Box<'a, Angle>)),
    SkewX(Box<'a, Angle>),
    SkewY(Box<'a, Angle>),
    Perspective(Box<'a, Length<'a>>),
    Matrix(Box<'a, MatrixForFloat>),
    Matrix3d(Box<'a, Matrix3DForFloat>),
}

#[derive(Debug, PartialEq)]
pub enum TransformStyle {
    Flat,
    Preserve3d,
}

#[derive(Debug, PartialEq)]
pub enum TransformBox {
    ContentBox,
    BorderBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

#[derive(Debug, PartialEq)]
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
    Object {
        x: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
        y: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
        z: Box<'a, Length<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Scale<'a> {
    None,
    Object {
        x: Box<'a, NumberOrPercentage>,
        y: Box<'a, NumberOrPercentage>,
        z: Box<'a, NumberOrPercentage>,
    },
}

#[derive(Debug, PartialEq)]
pub enum TextTransformCase {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(Debug, PartialEq)]
pub enum WhiteSpace {
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    BreakSpaces,
    PreLine,
}

#[derive(Debug, PartialEq)]
pub enum WordBreak {
    Normal,
    KeepAll,
    BreakAll,
    BreakWord,
}

#[derive(Debug, PartialEq)]
pub enum LineBreak {
    Auto,
    Loose,
    Normal,
    Strict,
    Anywhere,
}

#[derive(Debug, PartialEq)]
pub enum Hyphens {
    None,
    Manual,
    Auto,
}

#[derive(Debug, PartialEq)]
pub enum OverflowWrap {
    Normal,
    Anywhere,
    BreakWord,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    ExclusiveTextDecorationLine(Box<'a, ExclusiveTextDecorationLine>),
    Value(Vec<'a, Box<'a, OtherTextDecorationLine>>),
}

#[derive(Debug, PartialEq)]
pub enum ExclusiveTextDecorationLine {
    None,
    SpellingError,
    GrammarError,
}

#[derive(Debug, PartialEq)]
pub enum OtherTextDecorationLine {
    Underline,
    Overline,
    LineThrough,
    Blink,
}

#[derive(Debug, PartialEq)]
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
    LengthPercentage(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum TextDecorationSkipInk {
    Auto,
    None,
    All,
}

#[derive(Debug, PartialEq)]
pub enum TextEmphasisStyle<'a> {
    None,
    Keyword {
        fill: Box<'a, TextEmphasisFillMode>,
        shape: Option<Box<'a, TextEmphasisShape>>,
    },
    String(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum TextEmphasisFillMode {
    Filled,
    Open,
}

#[derive(Debug, PartialEq)]
pub enum TextEmphasisShape {
    Dot,
    Circle,
    DoubleCircle,
    Triangle,
    Sesame,
}

#[derive(Debug, PartialEq)]
pub enum TextEmphasisPositionHorizontal {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum TextEmphasisPositionVertical {
    Over,
    Under,
}

#[derive(Debug, PartialEq)]
pub enum TextSizeAdjust {
    Auto,
    None,
    Percentage(f64),
}

#[derive(Debug, PartialEq)]
pub enum Direction2 {
    Ltr,
    Rtl,
}

#[derive(Debug, PartialEq)]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}

#[derive(Debug, PartialEq)]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}

#[derive(Debug, PartialEq)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum CaretShape {
    Auto,
    Bar,
    Block,
    Underscore,
}

#[derive(Debug, PartialEq)]
pub enum UserSelect {
    Auto,
    Text,
    None,
    Contain,
    All,
}

pub type Appearance<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub enum ListStyleType<'a> {
    None,
    String(&'a str),
    CounterStyle(Box<'a, CounterStyle<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum CounterStyle<'a> {
    Predefined(Box<'a, PredefinedCounterStyle>),
    Name(&'a str),
    Symbols {
        symbols: Vec<'a, Box<'a, Symbol<'a>>>,
        system: Option<()>,
    },
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum SymbolsType {
    Cyclic,
    Numeric,
    Alphabetic,
    Symbolic,
    Fixed,
}

#[derive(Debug, PartialEq)]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

#[derive(Debug, PartialEq)]
pub enum StrokeLinecap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, PartialEq)]
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
    Values(Vec<'a, Box<'a, DimensionPercentageFor_LengthValue<'a>>>),
}

#[derive(Debug, PartialEq)]
pub enum Marker<'a> {
    None,
    Url(Box<'a, Url<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum ColorInterpolation {
    Auto,
    Srgb,
    Linearrgb,
}

#[derive(Debug, PartialEq)]
pub enum ColorRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}

#[derive(Debug, PartialEq)]
pub enum ShapeRendering {
    Auto,
    Optimizespeed,
    Crispedges,
    Geometricprecision,
}

#[derive(Debug, PartialEq)]
pub enum TextRendering {
    Auto,
    Optimizespeed,
    Optimizelegibility,
    Geometricprecision,
}

#[derive(Debug, PartialEq)]
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
        reference_box: Box<'a, GeometryBox>,
        shape: Box<'a, BasicShape<'a>>,
    },
    Box(Box<'a, GeometryBox>),
}

#[derive(Debug, PartialEq)]
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
    Circle(Box<'a, Circle2<'a>>),
    Ellipse(Box<'a, Ellipse2<'a>>),
    Polygon(Box<'a, Polygon<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct RectFor_DimensionPercentageFor_LengthValue<'a>(
    pub Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    pub Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    pub Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    pub Box<'a, DimensionPercentageFor_LengthValue<'a>>,
);

#[derive(Debug, PartialEq)]
pub enum ShapeRadius<'a> {
    LengthPercentage(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    ClosestSide,
    FarthestSide,
}

#[derive(Debug, PartialEq)]
pub enum MaskMode {
    Luminance,
    Alpha,
    MatchSource,
}

#[derive(Debug, PartialEq)]
pub enum MaskClip<'a> {
    GeometryBox(Box<'a, GeometryBox>),
    NoClip,
}

#[derive(Debug, PartialEq)]
pub enum MaskComposite {
    Add,
    Subtract,
    Intersect,
    Exclude,
}

#[derive(Debug, PartialEq)]
pub enum MaskType {
    Luminance,
    Alpha,
}

#[derive(Debug, PartialEq)]
pub enum MaskBorderMode {
    Luminance,
    Alpha,
}

#[derive(Debug, PartialEq)]
pub enum WebKitMaskComposite<'a> {
    Value(&'a str),
    SourceOver,
    SourceIn,
    SourceOut,
    Xor,
}

#[derive(Debug, PartialEq)]
pub enum WebKitMaskSourceType {
    Auto,
    Luminance,
    Alpha,
}

#[derive(Debug, PartialEq)]
pub enum FilterList<'a> {
    None,
    Filters(Vec<'a, Box<'a, Filter<'a>>>),
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
    Integer(f64),
}

#[derive(Debug, PartialEq)]
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
    CssString(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum NoneOrCustomIdentList<'a> {
    None,
    Value(Vec<'a, &'a str>),
}

#[derive(Debug, PartialEq)]
pub enum ViewTransitionGroup<'a> {
    Normal,
    Contain,
    Nearest,
    CssString(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum PrintColorAdjust {
    Economy,
    Exact,
}

#[derive(Debug, PartialEq)]
pub enum CSSWideKeyword {
    Initial,
    Inherit,
    Unset,
    Revert,
    RevertLayer,
}

#[derive(Debug, PartialEq)]
pub enum CustomPropertyName<'a> {
    CssString(&'a str),
    CssString2(&'a str),
}
