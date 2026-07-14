use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct AspectRatio<'a> {
    pub auto: bool,
    pub ratio: Option<Box<'a, Ratio>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Overflow {
    pub x: OverflowKeyword,
    pub y: OverflowKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct InsetBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct InsetInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Inset<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FlexFlow {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Flex<'a> {
    pub basis: Box<'a, LengthPercentageOrAuto<'a>>,
    pub grow: f32,
    pub shrink: f32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceContent<'a> {
    pub align: Box<'a, AlignContent>,
    pub justify: Box<'a, JustifyContent>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceSelf<'a> {
    pub align: Box<'a, AlignSelf>,
    pub justify: Box<'a, JustifySelf>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceItems<'a> {
    pub align: Box<'a, AlignItems>,
    pub justify: Box<'a, JustifyItems>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Gap<'a> {
    pub column: Box<'a, GapValue<'a>>,
    pub row: Box<'a, GapValue<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TrackRepeat<'a> {
    pub count: Box<'a, RepeatCount>,
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
    pub areas: Box<'a, GridTemplateAreas<'a>>,
    pub columns: Box<'a, TrackSizing<'a>>,
    pub rows: Box<'a, TrackSizing<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Grid<'a> {
    pub areas: Box<'a, GridTemplateAreas<'a>>,
    pub auto_columns: Vec<'a, TrackSize<'a>>,
    pub auto_flow: Box<'a, GridAutoFlow>,
    pub auto_rows: Vec<'a, TrackSize<'a>>,
    pub columns: Box<'a, TrackSizing<'a>>,
    pub rows: Box<'a, TrackSizing<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridRow<'a> {
    pub end: Box<'a, GridLine<'a>>,
    pub start: Box<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridColumn<'a> {
    pub end: Box<'a, GridLine<'a>>,
    pub start: Box<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridArea<'a> {
    pub column_end: Box<'a, GridLine<'a>>,
    pub column_start: Box<'a, GridLine<'a>>,
    pub row_end: Box<'a, GridLine<'a>>,
    pub row_start: Box<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Margin<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Padding<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMargin<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingBlock<'a> {
    pub block_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingInline<'a> {
    pub inline_end: Box<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: Box<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPadding<'a> {
    pub bottom: Box<'a, LengthPercentageOrAuto<'a>>,
    pub left: Box<'a, LengthPercentageOrAuto<'a>>,
    pub right: Box<'a, LengthPercentageOrAuto<'a>>,
    pub top: Box<'a, LengthPercentageOrAuto<'a>>,
}
