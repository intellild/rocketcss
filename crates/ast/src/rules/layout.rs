use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct AspectRatio {
    pub auto: bool,
    pub ratio: Option<Ratio>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Overflow {
    pub x: OverflowKeyword,
    pub y: OverflowKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct InsetBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct InsetInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Inset<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FlexFlow {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Flex<'a> {
    pub basis: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub grow: f32,
    pub shrink: f32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceContent {
    pub align: AlignContent,
    pub justify: JustifyContent,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceSelf {
    pub align: AlignSelf,
    pub justify: JustifySelf,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceItems {
    pub align: AlignItems,
    pub justify: JustifyItems,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Gap<'a> {
    pub column: NodeId<'a, GapValue<'a>>,
    pub row: NodeId<'a, GapValue<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ColumnRule<'a> {
    pub color: Option<NodeId<'a, CssColor<'a>>>,
    pub style: Option<LineStyle>,
    pub width: Option<NodeId<'a, BorderSideWidth<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ColumnWidth<'a> {
    Auto,
    Length(NodeId<'a, Length<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum ColumnCount {
    Auto,
    Integer(i32),
}

#[derive(Debug, PartialEq, Visit)]
pub struct Columns<'a> {
    pub count: ColumnCount,
    pub width: ColumnWidth<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TrackRepeat<'a> {
    pub count: RepeatCount,
    pub line_names: Vec<'a, Vec<'a, &'a str>>,
    pub track_sizes: Vec<'a, TrackSize<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridAutoFlow {
    pub dense: bool,
    pub direction: AutoFlowDirection,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridTemplate<'a> {
    pub areas: NodeId<'a, GridTemplateAreas<'a>>,
    pub columns: NodeId<'a, TrackSizing<'a>>,
    pub rows: NodeId<'a, TrackSizing<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Grid<'a> {
    pub areas: NodeId<'a, GridTemplateAreas<'a>>,
    pub auto_columns: Vec<'a, TrackSize<'a>>,
    pub auto_flow: GridAutoFlow,
    pub auto_rows: Vec<'a, TrackSize<'a>>,
    pub columns: NodeId<'a, TrackSizing<'a>>,
    pub rows: NodeId<'a, TrackSizing<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridRow<'a> {
    pub end: NodeId<'a, GridLine<'a>>,
    pub start: NodeId<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridColumn<'a> {
    pub end: NodeId<'a, GridLine<'a>>,
    pub start: NodeId<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridArea<'a> {
    pub column_end: NodeId<'a, GridLine<'a>>,
    pub column_start: NodeId<'a, GridLine<'a>>,
    pub row_end: NodeId<'a, GridLine<'a>>,
    pub row_start: NodeId<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Margin<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Padding<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMargin<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPadding<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}
