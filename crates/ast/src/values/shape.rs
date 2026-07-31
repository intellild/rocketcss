use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ClipPath<'a> {
    None,
    Url(std::boxed::Box<Url<'a>>),
    Shape {
        reference_box: GeometryBox,
        shape: std::boxed::Box<BasicShape>,
    },
    Box(GeometryBox),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum GeometryBox {
    BorderBox,
    PaddingBox,
    ContentBox,
    MarginBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

#[derive(Debug, PartialEq, Visit)]
pub enum BasicShape {
    Inset(std::boxed::Box<InsetRect>),
    Circle(std::boxed::Box<CircleShape>),
    Ellipse(std::boxed::Box<EllipseShape>),
    Polygon(std::boxed::Box<Polygon>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum ShapeRadius {
    LengthPercentage(std::boxed::Box<LengthPercentage>),
    ClosestSide,
    FarthestSide,
}
