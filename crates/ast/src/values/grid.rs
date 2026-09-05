use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum TrackSizing<'a> {
    None,
    TrackList {
        items: Vec<'a, TrackListItem<'a>>,
        line_names: Vec<'a, Vec<'a, &'a str>>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum TrackListItem<'a> {
    TrackSize(NodeId<'a, TrackSize<'a>>),
    TrackRepeat(NodeId<'a, TrackRepeat<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum TrackSize<'a> {
    TrackBreadth(NodeId<'a, TrackBreadth<'a>>),
    MinMax {
        max: NodeId<'a, TrackBreadth<'a>>,
        min: NodeId<'a, TrackBreadth<'a>>,
    },
    FitContent(NodeId<'a, LengthPercentage<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum TrackBreadth<'a> {
    Length(NodeId<'a, LengthPercentage<'a>>),
    Flex(f32),
    MinContent,
    MaxContent,
    Auto,
}

#[derive(Debug, PartialEq, Visit)]
pub enum RepeatCount {
    Number(f32),
    AutoFill,
    AutoFit,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AutoFlowDirection {
    Row,
    Column,
}

#[derive(Debug, PartialEq, Visit)]
pub enum GridTemplateAreas<'a> {
    None,
    Areas {
        areas: Vec<'a, Option<&'a str>>,
        columns: u32,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum GridLine<'a> {
    Auto,
    Area { name: &'a str },
    Line { index: i32, name: Option<&'a str> },
    Span { index: i32, name: Option<&'a str> },
}
