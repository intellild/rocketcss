use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum Image<'a> {
    None,
    Url(std::boxed::Box<Url<'a>>),
    Gradient(std::boxed::Box<Gradient<'a>>),
    ImageSet(std::boxed::Box<ImageSet<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Gradient<'a> {
    Linear {
        direction: LineDirection,
        items: std::vec::Vec<GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingLinear {
        direction: LineDirection,
        items: std::vec::Vec<GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    Radial {
        items: std::vec::Vec<GradientItem<'a, LengthValue>>,
        position: std::boxed::Box<Position>,
        shape: std::boxed::Box<EndingShape>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingRadial {
        items: std::vec::Vec<GradientItem<'a, LengthValue>>,
        position: std::boxed::Box<Position>,
        shape: std::boxed::Box<EndingShape>,
        vendor_prefix: VendorPrefix,
    },
    Conic {
        angle: Angle,
        items: std::vec::Vec<GradientItem<'a, Angle>>,
        position: std::boxed::Box<Position>,
    },
    RepeatingConic {
        angle: Angle,
        items: std::vec::Vec<GradientItem<'a, Angle>>,
        position: std::boxed::Box<Position>,
    },
    WebKitGradient(std::boxed::Box<WebKitGradient<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum WebKitGradient<'a> {
    Linear {
        from: std::boxed::Box<WebKitGradientPoint>,
        to: std::boxed::Box<WebKitGradientPoint>,
        stops: std::vec::Vec<WebKitColorStop<'a>>,
    },
    Radial {
        from: std::boxed::Box<WebKitGradientPoint>,
        start_radius: f32,
        to: std::boxed::Box<WebKitGradientPoint>,
        end_radius: f32,
        stops: std::vec::Vec<WebKitColorStop<'a>>,
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
        color: std::boxed::Box<CssColor<'a>>,
        position: Option<std::boxed::Box<DimensionPercentage<D>>>,
    },
    Hint(std::boxed::Box<DimensionPercentage<D>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum DimensionPercentage<D> {
    Dimension(D),
    Percentage(f32),
    /// A unitless zero produced by target-aware minification.
    Zero,
    Calc(std::boxed::Box<Calc<DimensionPercentage<D>>>),
}

pub type LengthPercentage = DimensionPercentage<LengthValue>;
pub type AnglePercentage = DimensionPercentage<Angle>;

#[derive(Debug, PartialEq, Visit)]
pub enum PositionComponent<S> {
    Center,
    Length(std::boxed::Box<LengthPercentage>),
    Side {
        offset: Option<std::boxed::Box<LengthPercentage>>,
        side: S,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum EndingShape {
    Ellipse(std::boxed::Box<Ellipse>),
    Circle(std::boxed::Box<Circle>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Ellipse {
    Size {
        x: std::boxed::Box<LengthPercentage>,
        y: std::boxed::Box<LengthPercentage>,
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
pub enum Circle {
    Radius(std::boxed::Box<Length>),
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
pub enum BackgroundSize {
    Explicit {
        height: std::boxed::Box<LengthPercentageOrAuto>,
        width: std::boxed::Box<LengthPercentageOrAuto>,
    },
    Cover,
    Contain,
}

#[derive(Debug, PartialEq, Visit)]
pub enum LengthPercentageOrAuto {
    Auto,
    LengthPercentage(std::boxed::Box<LengthPercentage>),
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
