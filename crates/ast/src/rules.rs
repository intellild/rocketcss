use super::*;

#[derive(Debug, PartialEq)]
pub enum KeyframeSelector<'a> {
    Percentage(f64),
    From,
    To,
    TimelineRangePercentage(Box<'a, TimelineRangePercentage>),
}

#[derive(Debug, PartialEq)]
pub enum KeyframesName<'a> {
    Ident(&'a str),
    Custom(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum FontFaceProperty<'a> {
    Source(Vec<'a, Source<'a>>),
    FontFamily(Box<'a, FontFamily<'a>>),
    FontStyle(Box<'a, FontStyle2<'a>>),
    FontWeight(Box<'a, Size2DFor_FontWeight<'a>>),
    FontStretch(Box<'a, Size2DFor_FontStretch<'a>>),
    UnicodeRange(Vec<'a, UnicodeRange>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Source<'a> {
    Url(Box<'a, UrlSource<'a>>),
    Local(Box<'a, FontFamily<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum FontFormat<'a> {
    Woff,
    Woff2,
    Truetype,
    Opentype,
    EmbeddedOpentype,
    Collection,
    Svg,
    String(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum FontTechnology {
    FeaturesOpentype,
    FeaturesAat,
    FeaturesGraphite,
    ColorColrv0,
    ColorColrv1,
    ColorSvg,
    ColorSbix,
    ColorCbdt,
    Variations,
    Palettes,
    Incremental,
}

#[derive(Debug, PartialEq)]
pub enum FontStyle2<'a> {
    Normal,
    Italic,
    Oblique(Box<'a, Size2DFor_Angle<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct Size2DFor_Angle<'a>(pub Box<'a, Angle>, pub Box<'a, Angle>);

#[derive(Debug, PartialEq)]
pub struct Size2DFor_FontWeight<'a>(pub Box<'a, FontWeight<'a>>, pub Box<'a, FontWeight<'a>>);

#[derive(Debug, PartialEq)]
pub struct Size2DFor_FontStretch<'a>(pub Box<'a, FontStretch>, pub Box<'a, FontStretch>);

#[derive(Debug, PartialEq)]
pub enum FontPaletteValuesProperty<'a> {
    FontFamily(Box<'a, FontFamily<'a>>),
    BasePalette(Box<'a, BasePalette>),
    OverrideColors(Vec<'a, OverrideColors<'a>>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum BasePalette {
    Light,
    Dark,
    Integer(f64),
}

#[derive(Debug, PartialEq)]
pub enum FontFeatureSubruleType {
    Stylistic,
    HistoricalForms,
    Styleset,
    CharacterVariant,
    Swash,
    Ornaments,
    Annotation,
}

#[derive(Debug, PartialEq)]
pub enum PageMarginBox {
    TopLeftCorner,
    TopLeft,
    TopCenter,
    TopRight,
    TopRightCorner,
    LeftTop,
    LeftMiddle,
    LeftBottom,
    RightTop,
    RightMiddle,
    RightBottom,
    BottomLeftCorner,
    BottomLeft,
    BottomCenter,
    BottomRight,
    BottomRightCorner,
}

#[derive(Debug, PartialEq)]
pub enum PagePseudoClass {
    Left,
    Right,
    First,
    Last,
    Blank,
}

#[derive(Debug, PartialEq)]
pub enum ParsedComponent<'a> {
    Length(Box<'a, Length<'a>>),
    Number(f64),
    Percentage(f64),
    LengthPercentage(Box<'a, DimensionPercentageFor_LengthValue<'a>>),
    String(&'a str),
    Color(Box<'a, CssColor<'a>>),
    Image(Box<'a, Image<'a>>),
    Url(Box<'a, Url<'a>>),
    Integer(f64),
    Angle(Box<'a, Angle>),
    Time(Box<'a, Time>),
    Resolution(Box<'a, Resolution>),
    TransformFunction(Box<'a, Transform<'a>>),
    TransformList(Vec<'a, Transform<'a>>),
    CustomIdent(&'a str),
    Literal(&'a str),
    Repeated(()),
    TokenList(Vec<'a, TokenOrValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Multiplier {
    None,
    Space,
    Comma,
}

#[derive(Debug, PartialEq)]
pub enum SyntaxString<'a> {
    Components(Vec<'a, SyntaxComponent<'a>>),
    Universal,
}

#[derive(Debug, PartialEq)]
pub enum SyntaxComponentKind<'a> {
    Length,
    Number,
    Percentage,
    LengthPercentage,
    String,
    Color,
    Image,
    Url,
    Integer,
    Angle,
    Time,
    Resolution,
    TransformFunction,
    TransformList,
    CustomIdent,
    Literal(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum ContainerCondition<'a> {
    Feature(Box<'a, QueryFeatureFor_ContainerSizeFeatureId<'a>>),
    Not(Box<'a, ContainerCondition<'a>>),
    Operation {
        conditions: Vec<'a, ContainerCondition<'a>>,
        operator: Operator,
    },
    Style(Box<'a, StyleQuery<'a>>),
    ScrollState(Box<'a, ScrollStateQuery<'a>>),
    Unknown(Vec<'a, TokenOrValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum QueryFeatureFor_ContainerSizeFeatureId<'a> {
    Plain {
        name: Box<'a, MediaFeatureNameFor_ContainerSizeFeatureId<'a>>,
        value: Box<'a, MediaFeatureValue<'a>>,
    },
    Boolean {
        name: Box<'a, MediaFeatureNameFor_ContainerSizeFeatureId<'a>>,
    },
    Range {
        name: Box<'a, MediaFeatureNameFor_ContainerSizeFeatureId<'a>>,
        operator: MediaFeatureComparison,
        value: Box<'a, MediaFeatureValue<'a>>,
    },
    Interval {
        end: Box<'a, MediaFeatureValue<'a>>,
        end_operator: MediaFeatureComparison,
        name: Box<'a, MediaFeatureNameFor_ContainerSizeFeatureId<'a>>,
        start: Box<'a, MediaFeatureValue<'a>>,
        start_operator: MediaFeatureComparison,
    },
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureNameFor_ContainerSizeFeatureId<'a> {
    ContainerSizeFeatureId(ContainerSizeFeatureId),
    CssString(&'a str),
    CssString2(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum ContainerSizeFeatureId {
    Width,
    Height,
    InlineSize,
    BlockSize,
    AspectRatio,
    Orientation,
}

#[derive(Debug, PartialEq)]
pub enum StyleQuery<'a> {
    Declaration(Box<'a, Declaration<'a>>),
    Property(Box<'a, PropertyId<'a>>),
    Not(Box<'a, StyleQuery<'a>>),
    Operation {
        conditions: Vec<'a, StyleQuery<'a>>,
        operator: Operator,
    },
}

#[derive(Debug, PartialEq)]
pub enum ScrollStateQuery<'a> {
    Feature(Box<'a, QueryFeatureFor_ScrollStateFeatureId<'a>>),
    Not(Box<'a, ScrollStateQuery<'a>>),
    Operation {
        conditions: Vec<'a, ScrollStateQuery<'a>>,
        operator: Operator,
    },
}

#[derive(Debug, PartialEq)]
pub enum QueryFeatureFor_ScrollStateFeatureId<'a> {
    Plain {
        name: Box<'a, MediaFeatureNameFor_ScrollStateFeatureId<'a>>,
        value: Box<'a, MediaFeatureValue<'a>>,
    },
    Boolean {
        name: Box<'a, MediaFeatureNameFor_ScrollStateFeatureId<'a>>,
    },
    Range {
        name: Box<'a, MediaFeatureNameFor_ScrollStateFeatureId<'a>>,
        operator: MediaFeatureComparison,
        value: Box<'a, MediaFeatureValue<'a>>,
    },
    Interval {
        end: Box<'a, MediaFeatureValue<'a>>,
        end_operator: MediaFeatureComparison,
        name: Box<'a, MediaFeatureNameFor_ScrollStateFeatureId<'a>>,
        start: Box<'a, MediaFeatureValue<'a>>,
        start_operator: MediaFeatureComparison,
    },
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureNameFor_ScrollStateFeatureId<'a> {
    ScrollStateFeatureId(ScrollStateFeatureId),
    CssString(&'a str),
    CssString2(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum ScrollStateFeatureId {
    Stuck,
    Snapped,
    Scrollable,
    Scrolled,
}

#[derive(Debug, PartialEq)]
pub enum ViewTransitionProperty<'a> {
    Object {
        property: &'a str,
        value: Navigation,
    },
    Object2 {
        property: &'a str,
        value: Box<'a, NoneOrCustomIdentList<'a>>,
    },
    Object3 {
        property: &'a str,
        value: Box<'a, CustomProperty<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Navigation {
    None,
    Auto,
}

pub type DefaultAtRule = ();

#[derive(Debug, PartialEq)]
pub struct StyleSheet<'a> {
    pub license_comments: Vec<'a, &'a str>,
    pub rules: Vec<'a, CssRule<'a>>,
    pub source_map_urls: Vec<'a, Option<&'a str>>,
    pub sources: Vec<'a, &'a str>,
}

#[derive(Debug, PartialEq)]
pub struct MediaRule<'a> {
    pub span: Span,
    pub query: Box<'a, MediaList<'a>>,
    pub rules: Vec<'a, CssRule<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct MediaList<'a> {
    pub media_queries: Vec<'a, MediaQuery<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct MediaQuery<'a> {
    pub condition: Option<Box<'a, MediaCondition<'a>>>,
    pub media_type: Box<'a, MediaType<'a>>,
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, PartialEq)]
pub struct LengthValue {
    pub unit: LengthUnit,
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub struct EnvironmentVariable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub indices: Option<Vec<'a, f64>>,
    pub name: Box<'a, EnvironmentVariableName<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Url<'a> {
    pub span: Span,
    pub url: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Variable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub name: Box<'a, DashedIdentReference<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct DashedIdentReference<'a> {
    pub from: Option<Box<'a, Specifier<'a>>>,
    pub ident: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    pub arguments: Vec<'a, TokenOrValue<'a>>,
    pub name: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct ImportRule<'a> {
    pub layer: Option<Vec<'a, &'a str>>,
    pub span: Span,
    pub media: Option<Box<'a, MediaList<'a>>>,
    pub supports: Option<Box<'a, SupportsCondition<'a>>>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct StyleRule<'a> {
    pub declarations: Option<Box<'a, DeclarationBlock<'a>>>,
    pub span: Span,
    pub rules: Option<Vec<'a, CssRule<'a>>>,
    pub selectors: Box<'a, SelectorList<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct DeclarationBlock<'a> {
    pub declarations: Option<Vec<'a, Declaration<'a>>>,
    pub important_declarations: Option<Vec<'a, Declaration<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct Position<'a> {
    pub x: Box<'a, PositionComponentFor_HorizontalPositionKeyword<'a>>,
    pub y: Box<'a, PositionComponentFor_VerticalPositionKeyword<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct WebKitGradientPoint<'a> {
    pub x: Box<'a, WebKitGradientPointComponentFor_HorizontalPositionKeyword<'a>>,
    pub y: Box<'a, WebKitGradientPointComponentFor_VerticalPositionKeyword<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct WebKitColorStop<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub position: f64,
}

#[derive(Debug, PartialEq)]
pub struct ImageSet<'a> {
    pub options: Vec<'a, ImageSetOption<'a>>,
    pub vendor_prefix: VendorPrefix<'a>,
}

#[derive(Debug, PartialEq)]
pub struct ImageSetOption<'a> {
    pub file_type: Option<&'a str>,
    pub image: Box<'a, Image<'a>>,
    pub resolution: Box<'a, Resolution>,
}

#[derive(Debug, PartialEq)]
pub struct BackgroundPosition<'a> {
    pub x: Box<'a, PositionComponentFor_HorizontalPositionKeyword<'a>>,
    pub y: Box<'a, PositionComponentFor_VerticalPositionKeyword<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BackgroundRepeat {
    pub x: BackgroundRepeatKeyword,
    pub y: BackgroundRepeatKeyword,
}

#[derive(Debug, PartialEq)]
pub struct Background<'a> {
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub color: Box<'a, CssColor<'a>>,
    pub image: Box<'a, Image<'a>>,
    pub origin: BackgroundOrigin,
    pub position: Box<'a, BackgroundPosition<'a>>,
    pub repeat: Box<'a, BackgroundRepeat>,
    pub size: Box<'a, BackgroundSize<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BoxShadow<'a> {
    pub blur: Box<'a, Length<'a>>,
    pub color: Box<'a, CssColor<'a>>,
    pub inset: bool,
    pub spread: Box<'a, Length<'a>>,
    pub x_offset: Box<'a, Length<'a>>,
    pub y_offset: Box<'a, Length<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct AspectRatio<'a> {
    pub auto: bool,
    pub ratio: Option<Box<'a, Ratio>>,
}

#[derive(Debug, PartialEq)]
pub struct Overflow {
    pub x: OverflowKeyword,
    pub y: OverflowKeyword,
}

#[derive(Debug, PartialEq)]
pub struct InsetBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct InsetInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Inset<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderRadius<'a> {
    pub bottom_left: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
    pub bottom_right: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
    pub top_left: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
    pub top_right: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderImageRepeat {
    pub horizontal: BorderImageRepeatKeyword,
    pub vertical: BorderImageRepeatKeyword,
}

#[derive(Debug, PartialEq)]
pub struct BorderImageSlice<'a> {
    pub fill: bool,
    pub offsets: Box<'a, RectFor_NumberOrPercentage<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderImage<'a> {
    pub outset: Box<'a, RectFor_LengthOrNumber<'a>>,
    pub repeat: Box<'a, BorderImageRepeat>,
    pub slice: Box<'a, BorderImageSlice<'a>>,
    pub source: Box<'a, Image<'a>>,
    pub width: Box<'a, RectFor_BorderImageSideWidth<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderColor<'a> {
    pub bottom: Box<'a, CssColor<'a>>,
    pub left: Box<'a, CssColor<'a>>,
    pub right: Box<'a, CssColor<'a>>,
    pub top: Box<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderStyle {
    pub bottom: LineStyle,
    pub left: LineStyle,
    pub right: LineStyle,
    pub top: LineStyle,
}

#[derive(Debug, PartialEq)]
pub struct BorderWidth<'a> {
    pub bottom: Box<'a, BorderSideWidth<'a>>,
    pub left: Box<'a, BorderSideWidth<'a>>,
    pub right: Box<'a, BorderSideWidth<'a>>,
    pub top: Box<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderBlockColor<'a> {
    pub end: Box<'a, CssColor<'a>>,
    pub start: Box<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderBlockStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

#[derive(Debug, PartialEq)]
pub struct BorderBlockWidth<'a> {
    pub end: Box<'a, BorderSideWidth<'a>>,
    pub start: Box<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderInlineColor<'a> {
    pub end: Box<'a, CssColor<'a>>,
    pub start: Box<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct BorderInlineStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

#[derive(Debug, PartialEq)]
pub struct BorderInlineWidth<'a> {
    pub end: Box<'a, BorderSideWidth<'a>>,
    pub start: Box<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct GenericBorderFor_LineStyle<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub style: LineStyle,
    pub width: Box<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct GenericBorderFor_OutlineStyleAnd_11<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub style: Box<'a, OutlineStyle>,
    pub width: Box<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct FlexFlow {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
}

#[derive(Debug, PartialEq)]
pub struct Flex<'a> {
    pub basis: Box<'a, LengthPercentageOrAuto<'a>>,
    pub grow: f64,
    pub shrink: f64,
}

#[derive(Debug, PartialEq)]
pub struct PlaceContent<'a> {
    pub align: Box<'a, AlignContent>,
    pub justify: Box<'a, JustifyContent>,
}

#[derive(Debug, PartialEq)]
pub struct PlaceSelf<'a> {
    pub align: Box<'a, AlignSelf>,
    pub justify: Box<'a, JustifySelf>,
}

#[derive(Debug, PartialEq)]
pub struct PlaceItems<'a> {
    pub align: Box<'a, AlignItems>,
    pub justify: Box<'a, JustifyItems>,
}

#[derive(Debug, PartialEq)]
pub struct Gap<'a> {
    pub column: Box<'a, GapValue<'a>>,
    pub row: Box<'a, GapValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct TrackRepeat<'a> {
    pub count: Box<'a, RepeatCount>,
    pub line_names: Vec<'a, Vec<'a, &'a str>>,
    pub track_sizes: Vec<'a, TrackSize<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct GridAutoFlow {
    pub dense: bool,
    pub direction: AutoFlowDirection,
}

#[derive(Debug, PartialEq)]
pub struct GridTemplate<'a> {
    pub areas: Box<'a, GridTemplateAreas<'a>>,
    pub columns: Box<'a, TrackSizing<'a>>,
    pub rows: Box<'a, TrackSizing<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Grid<'a> {
    pub areas: Box<'a, GridTemplateAreas<'a>>,
    pub auto_columns: Vec<'a, TrackSize<'a>>,
    pub auto_flow: Box<'a, GridAutoFlow>,
    pub auto_rows: Vec<'a, TrackSize<'a>>,
    pub columns: Box<'a, TrackSizing<'a>>,
    pub rows: Box<'a, TrackSizing<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct GridRow<'a> {
    pub end: Box<'a, GridLine<'a>>,
    pub start: Box<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct GridColumn<'a> {
    pub end: Box<'a, GridLine<'a>>,
    pub start: Box<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct GridArea<'a> {
    pub column_end: Box<'a, GridLine<'a>>,
    pub column_start: Box<'a, GridLine<'a>>,
    pub row_end: Box<'a, GridLine<'a>>,
    pub row_start: Box<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct MarginBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct MarginInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Margin<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct PaddingBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct PaddingInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Padding<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ScrollMarginBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ScrollMarginInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ScrollMargin<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ScrollPaddingBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ScrollPaddingInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ScrollPadding<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Font<'a> {
    pub family: Vec<'a, FontFamily<'a>>,
    pub line_height: Box<'a, LineHeight<'a>>,
    pub size: Box<'a, FontSize<'a>>,
    pub stretch: Box<'a, FontStretch>,
    pub style: Box<'a, FontStyle<'a>>,
    pub variant_caps: FontVariantCaps,
    pub weight: Box<'a, FontWeight<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Transition<'a> {
    pub delay: Box<'a, Time>,
    pub duration: Box<'a, Time>,
    pub property: Box<'a, PropertyId<'a>>,
    pub timing_function: Box<'a, EasingFunction>,
}

#[derive(Debug, PartialEq)]
pub struct ScrollTimeline {
    pub axis: ScrollAxis,
    pub scroller: Scroller,
}

#[derive(Debug, PartialEq)]
pub struct ViewTimeline<'a> {
    pub axis: ScrollAxis,
    pub inset: Box<'a, Size2DFor_LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct AnimationRange<'a> {
    pub end: Box<'a, AnimationRangeEnd<'a>>,
    pub start: Box<'a, AnimationRangeStart<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Animation<'a> {
    pub delay: Box<'a, Time>,
    pub direction: AnimationDirection,
    pub duration: Box<'a, Time>,
    pub fill_mode: AnimationFillMode,
    pub iteration_count: Box<'a, AnimationIterationCount>,
    pub name: Box<'a, AnimationName<'a>>,
    pub play_state: AnimationPlayState,
    pub timeline: Box<'a, AnimationTimeline<'a>>,
    pub timing_function: Box<'a, EasingFunction>,
}

#[derive(Debug, PartialEq)]
pub struct MatrixForFloat {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

#[derive(Debug, PartialEq)]
pub struct Matrix3DForFloat {
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m14: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m24: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
    pub m34: f64,
    pub m41: f64,
    pub m42: f64,
    pub m43: f64,
    pub m44: f64,
}

#[derive(Debug, PartialEq)]
pub struct Rotate<'a> {
    pub angle: Box<'a, Angle>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, PartialEq)]
pub struct TextTransform {
    pub case: TextTransformCase,
    pub full_size_kana: bool,
    pub full_width: bool,
}

#[derive(Debug, PartialEq)]
pub struct TextIndent<'a> {
    pub each_line: bool,
    pub hanging: bool,
    pub value: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct TextDecoration<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub line: Box<'a, TextDecorationLine<'a>>,
    pub style: TextDecorationStyle,
    pub thickness: Box<'a, TextDecorationThickness<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct TextEmphasis<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub style: Box<'a, TextEmphasisStyle<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct TextEmphasisPosition {
    pub horizontal: TextEmphasisPositionHorizontal,
    pub vertical: TextEmphasisPositionVertical,
}

#[derive(Debug, PartialEq)]
pub struct TextShadow<'a> {
    pub blur: Box<'a, Length<'a>>,
    pub color: Box<'a, CssColor<'a>>,
    pub spread: Box<'a, Length<'a>>,
    pub x_offset: Box<'a, Length<'a>>,
    pub y_offset: Box<'a, Length<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Cursor<'a> {
    pub images: Vec<'a, CursorImage<'a>>,
    pub keyword: CursorKeyword,
}

#[derive(Debug, PartialEq)]
pub struct CursorImage<'a> {
    pub hotspot: Option<(f64, f64)>,
    pub url: Box<'a, Url<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Caret<'a> {
    pub color: Box<'a, ColorOrAuto<'a>>,
    pub shape: CaretShape,
}

#[derive(Debug, PartialEq)]
pub struct ListStyle<'a> {
    pub image: Box<'a, Image<'a>>,
    pub list_style_type: Box<'a, ListStyleType<'a>>,
    pub position: ListStylePosition,
}

#[derive(Debug, PartialEq)]
pub struct Composes<'a> {
    pub from: Option<Box<'a, Specifier<'a>>>,
    pub span: Span,
    pub names: Vec<'a, &'a str>,
}

#[derive(Debug, PartialEq)]
pub struct InsetRect<'a> {
    pub radius: Box<'a, BorderRadius<'a>>,
    pub rect: Box<'a, RectFor_DimensionPercentageFor_LengthValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Circle2<'a> {
    pub position: Box<'a, Position<'a>>,
    pub radius: Box<'a, ShapeRadius<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Ellipse2<'a> {
    pub position: Box<'a, Position<'a>>,
    pub radius_x: Box<'a, ShapeRadius<'a>>,
    pub radius_y: Box<'a, ShapeRadius<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Polygon<'a> {
    pub fill_rule: FillRule,
    pub points: Vec<'a, Point<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Point<'a> {
    pub x: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    pub y: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Mask<'a> {
    pub clip: Box<'a, MaskClip>,
    pub composite: MaskComposite,
    pub image: Box<'a, Image<'a>>,
    pub mode: MaskMode,
    pub origin: GeometryBox,
    pub position: Box<'a, Position<'a>>,
    pub repeat: Box<'a, BackgroundRepeat>,
    pub size: Box<'a, BackgroundSize<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct MaskBorder<'a> {
    pub mode: MaskBorderMode,
    pub outset: Box<'a, RectFor_LengthOrNumber<'a>>,
    pub repeat: Box<'a, BorderImageRepeat>,
    pub slice: Box<'a, BorderImageSlice<'a>>,
    pub source: Box<'a, Image<'a>>,
    pub width: Box<'a, RectFor_BorderImageSideWidth<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct DropShadow<'a> {
    pub blur: Box<'a, Length<'a>>,
    pub color: Box<'a, CssColor<'a>>,
    pub x_offset: Box<'a, Length<'a>>,
    pub y_offset: Box<'a, Length<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Container<'a> {
    pub container_type: ContainerType,
    pub name: Box<'a, ContainerNameList<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ColorScheme {
    pub dark: bool,
    pub light: bool,
    pub only: bool,
}

#[derive(Debug, PartialEq)]
pub struct UnparsedProperty<'a> {
    pub property_id: Box<'a, PropertyId<'a>>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct CustomProperty<'a> {
    pub name: Box<'a, CustomPropertyName<'a>>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct AttrOperation<'a> {
    pub case_sensitivity: Option<()>,
    pub operator: AttrSelectorOperator,
    pub value: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct ViewTransitionPartSelector<'a> {
    pub classes: Vec<'a, &'a str>,
    pub name: Option<Box<'a, ViewTransitionPartName<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct KeyframesRule<'a> {
    pub keyframes: Vec<'a, Keyframe<'a>>,
    pub span: Span,
    pub name: Box<'a, KeyframesName<'a>>,
    pub vendor_prefix: VendorPrefix<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Keyframe<'a> {
    pub declarations: Box<'a, DeclarationBlock<'a>>,
    pub selectors: Vec<'a, KeyframeSelector<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct TimelineRangePercentage {
    pub name: TimelineRangeName,
    pub percentage: f64,
}

#[derive(Debug, PartialEq)]
pub struct FontFaceRule<'a> {
    pub span: Span,
    pub properties: Vec<'a, FontFaceProperty<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct UrlSource<'a> {
    pub format: Option<Box<'a, FontFormat<'a>>>,
    pub tech: Vec<'a, FontTechnology>,
    pub url: Box<'a, Url<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct UnicodeRange {
    pub end: f64,
    pub start: f64,
}

#[derive(Debug, PartialEq)]
pub struct FontPaletteValuesRule<'a> {
    pub span: Span,
    pub name: &'a str,
    pub properties: Vec<'a, FontPaletteValuesProperty<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct OverrideColors<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub index: f64,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureValuesRule<'a> {
    pub span: Span,
    pub name: Vec<'a, &'a str>,
    pub rules: (),
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureSubrule {
    pub declarations: (),
    pub span: Span,
    pub name: FontFeatureSubruleType,
}

#[derive(Debug, PartialEq)]
pub struct PageRule<'a> {
    pub declarations: Box<'a, DeclarationBlock<'a>>,
    pub span: Span,
    pub rules: Vec<'a, PageMarginRule<'a>>,
    pub selectors: Vec<'a, PageSelector<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct PageMarginRule<'a> {
    pub declarations: Box<'a, DeclarationBlock<'a>>,
    pub span: Span,
    pub margin_box: PageMarginBox,
}

#[derive(Debug, PartialEq)]
pub struct PageSelector<'a> {
    pub name: Option<&'a str>,
    pub pseudo_classes: Vec<'a, PagePseudoClass>,
}

#[derive(Debug, PartialEq)]
pub struct SupportsRule<'a> {
    pub condition: Box<'a, SupportsCondition<'a>>,
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct CounterStyleRule<'a> {
    pub declarations: Box<'a, DeclarationBlock<'a>>,
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct NamespaceRule<'a> {
    pub span: Span,
    pub prefix: Option<&'a str>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct MozDocumentRule<'a> {
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct NestingRule<'a> {
    pub span: Span,
    pub style: Box<'a, StyleRule<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct NestedDeclarationsRule<'a> {
    pub declarations: Box<'a, DeclarationBlock<'a>>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ViewportRule<'a> {
    pub declarations: Box<'a, DeclarationBlock<'a>>,
    pub span: Span,
    pub vendor_prefix: VendorPrefix<'a>,
}

#[derive(Debug, PartialEq)]
pub struct CustomMediaRule<'a> {
    pub span: Span,
    pub name: &'a str,
    pub query: Box<'a, MediaList<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct LayerStatementRule<'a> {
    pub span: Span,
    pub names: Vec<'a, Vec<'a, &'a str>>,
}

#[derive(Debug, PartialEq)]
pub struct LayerBlockRule<'a> {
    pub span: Span,
    pub name: Option<Vec<'a, &'a str>>,
    pub rules: Vec<'a, CssRule<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct PropertyRule<'a> {
    pub inherits: bool,
    pub initial_value: Option<Box<'a, ParsedComponent<'a>>>,
    pub span: Span,
    pub name: &'a str,
    pub syntax: Box<'a, SyntaxString<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct SyntaxComponent<'a> {
    pub kind: Box<'a, SyntaxComponentKind<'a>>,
    pub multiplier: Multiplier,
}

#[derive(Debug, PartialEq)]
pub struct ContainerRule<'a> {
    pub condition: Option<Box<'a, ContainerCondition<'a>>>,
    pub span: Span,
    pub name: Option<&'a str>,
    pub rules: Vec<'a, CssRule<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ScopeRule<'a> {
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a>>,
    pub scope_end: Option<Box<'a, SelectorList<'a>>>,
    pub scope_start: Option<Box<'a, SelectorList<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct StartingStyleRule<'a> {
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct ViewTransitionRule<'a> {
    pub span: Span,
    pub properties: Vec<'a, ViewTransitionProperty<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct UnknownAtRule<'a> {
    pub block: Option<Vec<'a, TokenOrValue<'a>>>,
    pub span: Span,
    pub name: &'a str,
    pub prelude: Vec<'a, TokenOrValue<'a>>,
}
