use crate::*;

#[derive(Debug, PartialEq)]
pub struct InsetRect<'a> {
    pub radius: Box<'a, BorderRadius<'a>>,
    pub rect: Box<'a, Rect<'a, LengthPercentage<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct CircleShape<'a> {
    pub position: Box<'a, Position<'a>>,
    pub radius: Box<'a, ShapeRadius<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct EllipseShape<'a> {
    pub position: Box<'a, Position<'a>>,
    pub radius_x: Box<'a, ShapeRadius<'a>>,
    pub radius_y: Box<'a, ShapeRadius<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Polygon<'a> {
    pub fill_rule: FillRule,
    pub points: Vec<'a, Point<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Point<'a> {
    pub x: Box<'a, LengthPercentage<'a>>,
    pub y: Box<'a, LengthPercentage<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Mask<'a> {
    pub clip: Box<'a, MaskClip>,
    pub composite: MaskComposite,
    pub image: Box<'a, Image<'a>>,
    pub mode: MaskMode,
    pub origin: GeometryBox,
    pub position: Box<'a, Position<'a>>,
    pub repeat: Box<'a, BackgroundRepeat>,
    pub size: Box<'a, BackgroundSize<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct MaskBorder<'a> {
    pub mode: MaskBorderMode,
    pub outset: Box<'a, Rect<'a, LengthOrNumber<'a>>>,
    pub repeat: Box<'a, BorderImageRepeat>,
    pub slice: Box<'a, BorderImageSlice<'a>>,
    pub source: Box<'a, Image<'a>>,
    pub width: Box<'a, Rect<'a, BorderImageSideWidth<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct DropShadow<'a> {
    pub blur: Box<'a, Length<'a>>,
    pub color: Box<'a, CssColor<'a>>,
    pub x_offset: Box<'a, Length<'a>>,
    pub y_offset: Box<'a, Length<'a>>,
}
