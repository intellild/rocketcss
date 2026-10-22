#[ast]
pub enum Image<'a> {
    None,
    Url(Box<'a, Url<'a>>),
    Gradient(Box<'a, Gradient<'a>>),
    ImageSet(Box<'a, ImageSet<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[ast]
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
        angle: Angle,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: Box<'a, Position<'a>>,
    },
    RepeatingConic {
        angle: Angle,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: Box<'a, Position<'a>>,
    },
    WebKitGradient(Box<'a, WebKitGradient<'a>>),
}

#[ast]
pub enum WebKitGradient<'a> {
    Linear {
        from: Box<'a, WebKitGradientPoint>,
        to: Box<'a, WebKitGradientPoint>,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
    Radial {
        from: Box<'a, WebKitGradientPoint>,
        start_radius: f32,
        to: Box<'a, WebKitGradientPoint>,
        end_radius: f32,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineDirection {
    Angle(Angle),
    Horizontal(HorizontalPositionKeyword),
    Vertical(VerticalPositionKeyword),
    Corner {
        horizontal: HorizontalPositionKeyword,
        vertical: VerticalPositionKeyword,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum VerticalPositionKeyword {
    Top,
    Bottom,
}

#[ast]
pub enum GradientItem<'a, D> {
    ColorStop {
        color: Box<'a, CssColor<'a>>,
        position: Option<Box<'a, DimensionPercentage<'a, D>>>,
    },
    Hint(Box<'a, DimensionPercentage<'a, D>>),
}

#[ast]
pub enum DimensionPercentage<'a, D> {
    Dimension(D),
    Percentage(f32),
    /// A unitless zero produced by target-aware minification.
    Zero,
    Calc(Box<'a, Calc<'a, DimensionPercentage<'a, D>>>),
}

pub type LengthPercentage<'a> = DimensionPercentage<'a, LengthValue>;

pub type AnglePercentage<'a> = DimensionPercentage<'a, Angle>;

#[ast]
pub enum PositionComponent<'a, S> {
    Center,
    Length(Box<'a, LengthPercentage<'a>>),
    Side {
        offset: Option<Box<'a, LengthPercentage<'a>>>,
        side: S,
    },
}

#[ast]
pub enum EndingShape<'a> {
    Ellipse(Box<'a, Ellipse<'a>>),
    Circle(Box<'a, Circle<'a>>),
}

#[ast]
pub enum Ellipse<'a> {
    Size {
        x: Box<'a, LengthPercentage<'a>>,
        y: Box<'a, LengthPercentage<'a>>,
    },
    Extent(ShapeExtent),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ShapeExtent {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
}

#[ast]
pub enum Circle<'a> {
    Radius(Box<'a, Length<'a>>),
    Extent(ShapeExtent),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WebKitGradientPointComponent<S> {
    Center,
    Number(NumberOrPercentage),
    Side(S),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}

#[ast]
pub enum BackgroundSize<'a> {
    Explicit {
        height: Box<'a, LengthPercentageOrAuto<'a>>,
        width: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Cover,
    Contain,
}

#[ast]
pub enum LengthPercentageOrAuto<'a> {
    Auto,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Border,
    Text,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}
