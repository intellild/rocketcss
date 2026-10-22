#[ast]
pub struct AspectRatio {
    pub auto: bool,
    pub ratio: Option<Ratio>,
}

#[ast]
pub struct Overflow {
    pub x: OverflowKeyword,
    pub y: OverflowKeyword,
}

#[ast]
pub struct InsetBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct InsetInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct Inset<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct FlexFlow {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
}

#[ast]
pub struct Flex<'a> {
    pub basis: Box<'a, LengthPercentageOrAuto<'a>>,
    pub grow: f32,
    pub shrink: f32,
}

#[ast]
pub struct PlaceContent {
    pub align: AlignContent,
    pub justify: JustifyContent,
}

#[ast]
pub struct PlaceSelf {
    pub align: AlignSelf,
    pub justify: JustifySelf,
}

#[ast]
pub struct PlaceItems {
    pub align: AlignItems,
    pub justify: JustifyItems,
}

#[ast]
pub struct Gap<'a> {
    pub column: Box<'a, GapValue<'a>>,
    pub row: Box<'a, GapValue<'a>>,
}

#[ast]
pub struct ColumnRule<'a> {
    pub color: Option<Box<'a, CssColor<'a>>>,
    pub style: Option<LineStyle>,
    pub width: Option<Box<'a, BorderSideWidth<'a>>>,
}

#[ast]
pub enum ColumnWidth<'a> {
    Auto,
    Length(Box<'a, Length<'a>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColumnCount {
    Auto,
    Integer(i32),
}

#[ast]
pub struct Columns<'a> {
    pub count: ColumnCount,
    pub width: ColumnWidth<'a>,
}

#[ast]
pub struct TrackRepeat<'a> {
    pub count: RepeatCount,
    pub line_names: Vec<'a, Vec<'a, &'a str>>,
    pub track_sizes: Vec<'a, TrackSize<'a>>,
}

#[ast]
pub struct GridAutoFlow {
    pub dense: bool,
    pub direction: AutoFlowDirection,
}

#[ast]
pub struct GridTemplate<'a> {
    pub areas: Box<'a, GridTemplateAreas<'a>>,
    pub columns: Box<'a, TrackSizing<'a>>,
    pub rows: Box<'a, TrackSizing<'a>>,
}

#[ast]
pub struct Grid<'a> {
    pub areas: Box<'a, GridTemplateAreas<'a>>,
    pub auto_columns: Vec<'a, TrackSize<'a>>,
    pub auto_flow: GridAutoFlow,
    pub auto_rows: Vec<'a, TrackSize<'a>>,
    pub columns: Box<'a, TrackSizing<'a>>,
    pub rows: Box<'a, TrackSizing<'a>>,
}

#[ast]
pub struct GridRow<'a> {
    pub end: Box<'a, GridLine<'a>>,
    pub start: Box<'a, GridLine<'a>>,
}

#[ast]
pub struct GridColumn<'a> {
    pub end: Box<'a, GridLine<'a>>,
    pub start: Box<'a, GridLine<'a>>,
}

#[ast]
pub struct GridArea<'a> {
    pub column_end: Box<'a, GridLine<'a>>,
    pub column_start: Box<'a, GridLine<'a>>,
    pub row_end: Box<'a, GridLine<'a>>,
    pub row_start: Box<'a, GridLine<'a>>,
}

#[ast]
pub struct MarginBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct MarginInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct Margin<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct PaddingBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct PaddingInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct Padding<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct ScrollMarginBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct ScrollMarginInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct ScrollMargin<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct ScrollPaddingBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct ScrollPaddingInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[ast]
pub struct ScrollPadding<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}
