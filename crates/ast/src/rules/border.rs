use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct BorderRadius<'a> {
    pub bottom_left: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub bottom_right: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub top_left: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub top_right: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImageRepeat {
    pub horizontal: BorderImageRepeatKeyword,
    pub vertical: BorderImageRepeatKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImageSlice<'a> {
    pub fill: bool,
    pub offsets: NodeId<'a, Rect<'a, NumberOrPercentage>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImage<'a> {
    pub outset: NodeId<'a, Rect<'a, LengthOrNumber<'a>>>,
    pub repeat: BorderImageRepeat,
    pub slice: NodeId<'a, BorderImageSlice<'a>>,
    pub source: NodeId<'a, Image<'a>>,
    pub width: NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderColor<'a> {
    pub bottom: NodeId<'a, CssColor<'a>>,
    pub left: NodeId<'a, CssColor<'a>>,
    pub right: NodeId<'a, CssColor<'a>>,
    pub top: NodeId<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderStyle {
    pub bottom: LineStyle,
    pub left: LineStyle,
    pub right: LineStyle,
    pub top: LineStyle,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderWidth<'a> {
    pub bottom: NodeId<'a, BorderSideWidth<'a>>,
    pub left: NodeId<'a, BorderSideWidth<'a>>,
    pub right: NodeId<'a, BorderSideWidth<'a>>,
    pub top: NodeId<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockColor<'a> {
    pub end: NodeId<'a, CssColor<'a>>,
    pub start: NodeId<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockWidth<'a> {
    pub end: NodeId<'a, BorderSideWidth<'a>>,
    pub start: NodeId<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineColor<'a> {
    pub end: NodeId<'a, CssColor<'a>>,
    pub start: NodeId<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineWidth<'a> {
    pub end: NodeId<'a, BorderSideWidth<'a>>,
    pub start: NodeId<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GenericBorder<'a, S> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub style: S,
    pub width: NodeId<'a, BorderSideWidth<'a>>,
}
