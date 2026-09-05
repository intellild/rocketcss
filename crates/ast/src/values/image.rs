use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum Image<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
    Gradient(NodeId<'a, Gradient<'a>>),
    ImageSet(NodeId<'a, ImageSet<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Gradient<'a> {
    Linear {
        direction: LineDirection,
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingLinear {
        direction: LineDirection,
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    Radial {
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        position: NodeId<'a, Position<'a>>,
        shape: NodeId<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingRadial {
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        position: NodeId<'a, Position<'a>>,
        shape: NodeId<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    Conic {
        angle: Angle,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: NodeId<'a, Position<'a>>,
    },
    RepeatingConic {
        angle: Angle,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: NodeId<'a, Position<'a>>,
    },
    WebKitGradient(NodeId<'a, WebKitGradient<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum WebKitGradient<'a> {
    Linear {
        from: NodeId<'a, WebKitGradientPoint>,
        to: NodeId<'a, WebKitGradientPoint>,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
    Radial {
        from: NodeId<'a, WebKitGradientPoint>,
        start_radius: f32,
        to: NodeId<'a, WebKitGradientPoint>,
        end_radius: f32,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum LineDirection {
    Angle(Angle),
    Horizontal(HorizontalPositionKeyword),
    Vertical(VerticalPositionKeyword),
    Corner {
        horizontal: HorizontalPositionKeyword,
        vertical: VerticalPositionKeyword,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum VerticalPositionKeyword {
    Top,
    Bottom,
}

#[derive(Debug, PartialEq, Visit)]
pub enum GradientItem<'a, D> {
    ColorStop {
        color: NodeId<'a, CssColor<'a>>,
        position: Option<NodeId<'a, DimensionPercentage<'a, D>>>,
    },
    Hint(NodeId<'a, DimensionPercentage<'a, D>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum DimensionPercentage<'a, D> {
    Dimension(D),
    Percentage(f32),
    /// A unitless zero produced by target-aware minification.
    Zero,
    Calc(NodeId<'a, Calc<'a, DimensionPercentage<'a, D>>>),
}

pub type LengthPercentage<'a> = DimensionPercentage<'a, LengthValue>;
pub type AnglePercentage<'a> = DimensionPercentage<'a, Angle>;

#[derive(Debug, PartialEq, Visit)]
pub enum PositionComponent<'a, S> {
    Center,
    Length(NodeId<'a, LengthPercentage<'a>>),
    Side {
        offset: Option<NodeId<'a, LengthPercentage<'a>>>,
        side: S,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum EndingShape<'a> {
    Ellipse(NodeId<'a, Ellipse<'a>>),
    Circle(NodeId<'a, Circle<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Ellipse<'a> {
    Size {
        x: NodeId<'a, LengthPercentage<'a>>,
        y: NodeId<'a, LengthPercentage<'a>>,
    },
    Extent(ShapeExtent),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ShapeExtent {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Circle<'a> {
    Radius(NodeId<'a, Length<'a>>),
    Extent(ShapeExtent),
}

#[derive(Debug, PartialEq, Visit)]
pub enum WebKitGradientPointComponent<S> {
    Center,
    Number(NumberOrPercentage),
    Side(S),
}

#[derive(Debug, PartialEq, Visit)]
pub enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}

#[derive(Debug, PartialEq, Visit)]
pub enum BackgroundSize<'a> {
    Explicit {
        height: NodeId<'a, LengthPercentageOrAuto<'a>>,
        width: NodeId<'a, LengthPercentageOrAuto<'a>>,
    },
    Cover,
    Contain,
}

#[derive(Debug, PartialEq, Visit)]
pub enum LengthPercentageOrAuto<'a> {
    Auto,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Border,
    Text,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}
