use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum TrackSizing<'a> {
    None,
    TrackList {
        items: std::vec::Vec<TrackListItem<'a>>,
        line_names: std::vec::Vec<std::vec::Vec<&'a str>>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum TrackListItem<'a> {
    TrackSize(std::boxed::Box<TrackSize>),
    TrackRepeat(std::boxed::Box<TrackRepeat<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum TrackSize {
    TrackBreadth(std::boxed::Box<TrackBreadth>),
    MinMax {
        max: std::boxed::Box<TrackBreadth>,
        min: std::boxed::Box<TrackBreadth>,
    },
    FitContent(std::boxed::Box<LengthPercentage>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum TrackBreadth {
    Length(std::boxed::Box<LengthPercentage>),
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
        areas: std::vec::Vec<Option<&'a str>>,
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
