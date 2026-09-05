use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ClipPath<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
    Shape {
        reference_box: GeometryBox,
        shape: NodeId<'a, BasicShape<'a>>,
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
pub enum BasicShape<'a> {
    Inset(NodeId<'a, InsetRect<'a>>),
    Circle(NodeId<'a, CircleShape<'a>>),
    Ellipse(NodeId<'a, EllipseShape<'a>>),
    Polygon(NodeId<'a, Polygon<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum ShapeRadius<'a> {
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    ClosestSide,
    FarthestSide,
}
