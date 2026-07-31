use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct BorderRadius {
    pub bottom_left: std::boxed::Box<Size2D<LengthPercentage>>,
    pub bottom_right: std::boxed::Box<Size2D<LengthPercentage>>,
    pub top_left: std::boxed::Box<Size2D<LengthPercentage>>,
    pub top_right: std::boxed::Box<Size2D<LengthPercentage>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImageRepeat {
    pub horizontal: BorderImageRepeatKeyword,
    pub vertical: BorderImageRepeatKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImageSlice {
    pub fill: bool,
    pub offsets: std::boxed::Box<Rect<NumberOrPercentage>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImage<'a> {
    pub outset: std::boxed::Box<Rect<LengthOrNumber>>,
    pub repeat: BorderImageRepeat,
    pub slice: std::boxed::Box<BorderImageSlice>,
    pub source: std::boxed::Box<Image<'a>>,
    pub width: std::boxed::Box<Rect<BorderImageSideWidth>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderColor<'a> {
    pub bottom: std::boxed::Box<CssColor<'a>>,
    pub left: std::boxed::Box<CssColor<'a>>,
    pub right: std::boxed::Box<CssColor<'a>>,
    pub top: std::boxed::Box<CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderStyle {
    pub bottom: LineStyle,
    pub left: LineStyle,
    pub right: LineStyle,
    pub top: LineStyle,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderWidth {
    pub bottom: std::boxed::Box<BorderSideWidth>,
    pub left: std::boxed::Box<BorderSideWidth>,
    pub right: std::boxed::Box<BorderSideWidth>,
    pub top: std::boxed::Box<BorderSideWidth>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockColor<'a> {
    pub end: std::boxed::Box<CssColor<'a>>,
    pub start: std::boxed::Box<CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockWidth {
    pub end: std::boxed::Box<BorderSideWidth>,
    pub start: std::boxed::Box<BorderSideWidth>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineColor<'a> {
    pub end: std::boxed::Box<CssColor<'a>>,
    pub start: std::boxed::Box<CssColor<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineWidth {
    pub end: std::boxed::Box<BorderSideWidth>,
    pub start: std::boxed::Box<BorderSideWidth>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GenericBorder<'a, S> {
    pub color: std::boxed::Box<CssColor<'a>>,
    pub style: S,
    pub width: std::boxed::Box<BorderSideWidth>,
}
