use crate::*;

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
