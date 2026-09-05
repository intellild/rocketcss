use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct InsetRect<'a> {
    pub radius: NodeId<'a, BorderRadius<'a>>,
    pub rect: NodeId<'a, Rect<'a, LengthPercentage<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CircleShape<'a> {
    pub position: NodeId<'a, Position<'a>>,
    pub radius: NodeId<'a, ShapeRadius<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct EllipseShape<'a> {
    pub position: NodeId<'a, Position<'a>>,
    pub radius_x: NodeId<'a, ShapeRadius<'a>>,
    pub radius_y: NodeId<'a, ShapeRadius<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Polygon<'a> {
    pub fill_rule: FillRule,
    pub points: Vec<'a, Point<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Point<'a> {
    pub x: NodeId<'a, LengthPercentage<'a>>,
    pub y: NodeId<'a, LengthPercentage<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Mask<'a> {
    pub clip: MaskClip,
    pub composite: MaskComposite,
    pub image: NodeId<'a, Image<'a>>,
    pub mode: MaskMode,
    pub origin: GeometryBox,
    pub position: NodeId<'a, Position<'a>>,
    pub repeat: BackgroundRepeat,
    pub size: NodeId<'a, BackgroundSize<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MaskBorder<'a> {
    pub mode: MaskBorderMode,
    pub outset: NodeId<'a, Rect<'a, LengthOrNumber<'a>>>,
    pub repeat: BorderImageRepeat,
    pub slice: NodeId<'a, BorderImageSlice<'a>>,
    pub source: NodeId<'a, Image<'a>>,
    pub width: NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct DropShadow<'a> {
    pub blur: NodeId<'a, Length<'a>>,
    pub color: NodeId<'a, CssColor<'a>>,
    pub x_offset: NodeId<'a, Length<'a>>,
    pub y_offset: NodeId<'a, Length<'a>>,
}
