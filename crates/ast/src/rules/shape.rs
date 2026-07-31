use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct InsetRect {
    pub radius: std::boxed::Box<BorderRadius>,
    pub rect: std::boxed::Box<Rect<LengthPercentage>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CircleShape {
    pub position: std::boxed::Box<Position>,
    pub radius: std::boxed::Box<ShapeRadius>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct EllipseShape {
    pub position: std::boxed::Box<Position>,
    pub radius_x: std::boxed::Box<ShapeRadius>,
    pub radius_y: std::boxed::Box<ShapeRadius>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Polygon {
    pub fill_rule: FillRule,
    pub points: std::vec::Vec<Point>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Point {
    pub x: std::boxed::Box<LengthPercentage>,
    pub y: std::boxed::Box<LengthPercentage>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Mask<'a> {
    pub clip: MaskClip,
    pub composite: MaskComposite,
    pub image: std::boxed::Box<Image<'a>>,
    pub mode: MaskMode,
    pub origin: GeometryBox,
    pub position: std::boxed::Box<Position>,
    pub repeat: BackgroundRepeat,
    pub size: std::boxed::Box<BackgroundSize>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MaskBorder<'a> {
    pub mode: MaskBorderMode,
    pub outset: std::boxed::Box<Rect<LengthOrNumber>>,
    pub repeat: BorderImageRepeat,
    pub slice: std::boxed::Box<BorderImageSlice>,
    pub source: std::boxed::Box<Image<'a>>,
    pub width: std::boxed::Box<Rect<BorderImageSideWidth>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct DropShadow<'a> {
    pub blur: std::boxed::Box<Length>,
    pub color: std::boxed::Box<CssColor<'a>>,
    pub x_offset: std::boxed::Box<Length>,
    pub y_offset: std::boxed::Box<Length>,
}
