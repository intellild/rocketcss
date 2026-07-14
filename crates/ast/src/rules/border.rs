use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct BorderRadius<'a> {
    pub bottom_left: Box<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub bottom_right: Box<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub top_left: Box<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub top_right: Box<'a, Size2D<'a, LengthPercentage<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImageRepeat {
    pub horizontal: BorderImageRepeatKeyword,
    pub vertical: BorderImageRepeatKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImageSlice<'a> {
    pub fill: bool,
    pub offsets: Box<'a, Rect<'a, NumberOrPercentage>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImage<'a> {
    pub outset: Box<'a, Rect<'a, LengthOrNumber<'a>>>,
    pub repeat: Box<'a, BorderImageRepeat>,
    pub slice: Box<'a, BorderImageSlice<'a>>,
    pub source: Box<'a, Image<'a>>,
    pub width: Box<'a, Rect<'a, BorderImageSideWidth<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderColor<'a> {
    pub bottom: Box<'a, CssColor<'a>>,
    pub left: Box<'a, CssColor<'a>>,
    pub right: Box<'a, CssColor<'a>>,
    pub top: Box<'a, CssColor<'a>>,
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
    pub bottom: Box<'a, BorderSideWidth<'a>>,
    pub left: Box<'a, BorderSideWidth<'a>>,
    pub right: Box<'a, BorderSideWidth<'a>>,
    pub top: Box<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockColor<'a> {
    pub end: Box<'a, CssColor<'a>>,
    pub start: Box<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockWidth<'a> {
    pub end: Box<'a, BorderSideWidth<'a>>,
    pub start: Box<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineColor<'a> {
    pub end: Box<'a, CssColor<'a>>,
    pub start: Box<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineWidth<'a> {
    pub end: Box<'a, BorderSideWidth<'a>>,
    pub start: Box<'a, BorderSideWidth<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GenericBorder<'a, S> {
    pub color: Box<'a, CssColor<'a>>,
    pub style: S,
    pub width: Box<'a, BorderSideWidth<'a>>,
}
