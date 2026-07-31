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
pub struct InsetBlock {
    pub block_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub block_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct InsetInline {
    pub inline_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub inline_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Inset {
    pub bottom: std::boxed::Box<LengthPercentageOrAuto>,
    pub left: std::boxed::Box<LengthPercentageOrAuto>,
    pub right: std::boxed::Box<LengthPercentageOrAuto>,
    pub top: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FlexFlow {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Flex {
    pub basis: std::boxed::Box<LengthPercentageOrAuto>,
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
pub struct Gap {
    pub column: std::boxed::Box<GapValue>,
    pub row: std::boxed::Box<GapValue>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ColumnRule<'a> {
    pub color: Option<std::boxed::Box<CssColor<'a>>>,
    pub style: Option<LineStyle>,
    pub width: Option<std::boxed::Box<BorderSideWidth>>,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ColumnWidth {
    Auto,
    Length(std::boxed::Box<Length>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum ColumnCount {
    Auto,
    Integer(i32),
}

#[derive(Debug, PartialEq, Visit)]
pub struct Columns {
    pub count: ColumnCount,
    pub width: ColumnWidth,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TrackRepeat<'a> {
    pub count: RepeatCount,
    pub line_names: std::vec::Vec<std::vec::Vec<&'a str>>,
    pub track_sizes: std::vec::Vec<TrackSize>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridAutoFlow {
    pub dense: bool,
    pub direction: AutoFlowDirection,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridTemplate<'a> {
    pub areas: std::boxed::Box<GridTemplateAreas<'a>>,
    pub columns: std::boxed::Box<TrackSizing<'a>>,
    pub rows: std::boxed::Box<TrackSizing<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Grid<'a> {
    pub areas: std::boxed::Box<GridTemplateAreas<'a>>,
    pub auto_columns: std::vec::Vec<TrackSize>,
    pub auto_flow: GridAutoFlow,
    pub auto_rows: std::vec::Vec<TrackSize>,
    pub columns: std::boxed::Box<TrackSizing<'a>>,
    pub rows: std::boxed::Box<TrackSizing<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridRow<'a> {
    pub end: std::boxed::Box<GridLine<'a>>,
    pub start: std::boxed::Box<GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridColumn<'a> {
    pub end: std::boxed::Box<GridLine<'a>>,
    pub start: std::boxed::Box<GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridArea<'a> {
    pub column_end: std::boxed::Box<GridLine<'a>>,
    pub column_start: std::boxed::Box<GridLine<'a>>,
    pub row_end: std::boxed::Box<GridLine<'a>>,
    pub row_start: std::boxed::Box<GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginBlock {
    pub block_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub block_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginInline {
    pub inline_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub inline_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Margin {
    pub bottom: std::boxed::Box<LengthPercentageOrAuto>,
    pub left: std::boxed::Box<LengthPercentageOrAuto>,
    pub right: std::boxed::Box<LengthPercentageOrAuto>,
    pub top: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingBlock {
    pub block_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub block_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingInline {
    pub inline_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub inline_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Padding {
    pub bottom: std::boxed::Box<LengthPercentageOrAuto>,
    pub left: std::boxed::Box<LengthPercentageOrAuto>,
    pub right: std::boxed::Box<LengthPercentageOrAuto>,
    pub top: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginBlock {
    pub block_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub block_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginInline {
    pub inline_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub inline_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMargin {
    pub bottom: std::boxed::Box<LengthPercentageOrAuto>,
    pub left: std::boxed::Box<LengthPercentageOrAuto>,
    pub right: std::boxed::Box<LengthPercentageOrAuto>,
    pub top: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingBlock {
    pub block_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub block_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingInline {
    pub inline_end: std::boxed::Box<LengthPercentageOrAuto>,
    pub inline_start: std::boxed::Box<LengthPercentageOrAuto>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPadding {
    pub bottom: std::boxed::Box<LengthPercentageOrAuto>,
    pub left: std::boxed::Box<LengthPercentageOrAuto>,
    pub right: std::boxed::Box<LengthPercentageOrAuto>,
    pub top: std::boxed::Box<LengthPercentageOrAuto>,
}
