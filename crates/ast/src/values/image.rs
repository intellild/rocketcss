use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum Image<'a> {
    None,
    Url(Box<'a, Url<'a>>),
    Gradient(Box<'a, Gradient<'a>>),
    ImageSet(Box<'a, ImageSet<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Gradient<'a> {
    Linear {
        direction: Box<'a, LineDirection<'a>>,
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingLinear {
        direction: Box<'a, LineDirection<'a>>,
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    Radial {
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        position: Box<'a, Position<'a>>,
        shape: Box<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingRadial {
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        position: Box<'a, Position<'a>>,
        shape: Box<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    Conic {
        angle: Box<'a, Angle>,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: Box<'a, Position<'a>>,
    },
    RepeatingConic {
        angle: Box<'a, Angle>,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: Box<'a, Position<'a>>,
    },
    WebKitGradient(Box<'a, WebKitGradient<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum WebKitGradient<'a> {
    Linear {
        from: Box<'a, WebKitGradientPoint<'a>>,
        to: Box<'a, WebKitGradientPoint<'a>>,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
    Radial {
        from: Box<'a, WebKitGradientPoint<'a>>,
        start_radius: f32,
        to: Box<'a, WebKitGradientPoint<'a>>,
        end_radius: f32,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum LineDirection<'a> {
    Angle(Box<'a, Angle>),
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
        color: Box<'a, CssColor<'a>>,
        position: Option<Box<'a, DimensionPercentage<'a, D>>>,
    },
    Hint(Box<'a, DimensionPercentage<'a, D>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum DimensionPercentage<'a, D> {
    Dimension(Box<'a, D>),
    Percentage(f32),
    /// A unitless zero produced by target-aware minification.
    Zero,
    Calc(Box<'a, Calc<'a, DimensionPercentage<'a, D>>>),
}

pub type LengthPercentage<'a> = DimensionPercentage<'a, LengthValue>;
pub type AnglePercentage<'a> = DimensionPercentage<'a, Angle>;

#[derive(Debug, PartialEq, Visit)]
pub enum PositionComponent<'a, S> {
    Center,
    Length(Box<'a, LengthPercentage<'a>>),
    Side {
        offset: Option<Box<'a, LengthPercentage<'a>>>,
        side: S,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum EndingShape<'a> {
    Ellipse(Box<'a, Ellipse<'a>>),
    Circle(Box<'a, Circle<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Ellipse<'a> {
    Size {
        x: Box<'a, LengthPercentage<'a>>,
        y: Box<'a, LengthPercentage<'a>>,
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
    Radius(Box<'a, Length<'a>>),
    Extent(ShapeExtent),
}

#[derive(Debug, PartialEq, Visit)]
pub enum WebKitGradientPointComponent<'a, S> {
    Center,
    Number(Box<'a, NumberOrPercentage>),
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
        height: Box<'a, LengthPercentageOrAuto<'a>>,
        width: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Cover,
    Contain,
}

#[derive(Debug, PartialEq, Visit)]
pub enum LengthPercentageOrAuto<'a> {
    Auto,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
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
