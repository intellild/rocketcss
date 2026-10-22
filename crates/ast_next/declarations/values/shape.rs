#[ast]
pub enum ClipPath<'a> {
    None,
    Url(Box<'a, Url<'a>>),
    Shape {
        reference_box: GeometryBox,
        shape: Box<'a, BasicShape<'a>>,
    },
    Box(GeometryBox),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum GeometryBox {
    BorderBox,
    PaddingBox,
    ContentBox,
    MarginBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

#[ast]
pub enum BasicShape<'a> {
    Inset(Box<'a, InsetRect<'a>>),
    Circle(Box<'a, CircleShape<'a>>),
    Ellipse(Box<'a, EllipseShape<'a>>),
    Polygon(Box<'a, Polygon<'a>>),
}

#[ast]
pub enum ShapeRadius<'a> {
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    ClosestSide,
    FarthestSide,
}
