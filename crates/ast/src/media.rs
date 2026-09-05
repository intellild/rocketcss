use super::*;

#[derive(Debug, PartialEq, Visit)]
pub enum MediaCondition<'a> {
    Feature(NodeId<'a, MediaFeature<'a>>),
    Not(NodeId<'a, MediaCondition<'a>>),
    Operation {
        conditions: Vec<'a, MediaCondition<'a>>,
        operator: Operator,
    },
    Unknown(Vec<'a, TokenOrValue<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum QueryFeature<'a, FeatureId> {
    Plain {
        name: MediaFeatureName<'a, FeatureId>,
        value: MediaFeatureValue<'a>,
    },
    Boolean {
        name: MediaFeatureName<'a, FeatureId>,
    },
    Range {
        name: MediaFeatureName<'a, FeatureId>,
        operator: MediaFeatureComparison,
        value: MediaFeatureValue<'a>,
    },
    Interval {
        end: NodeId<'a, MediaFeatureValue<'a>>,
        end_operator: MediaFeatureComparison,
        name: MediaFeatureName<'a, FeatureId>,
        start: NodeId<'a, MediaFeatureValue<'a>>,
        start_operator: MediaFeatureComparison,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum MediaFeatureName<'a, FeatureId> {
    Standard(FeatureId),
    Custom(&'a str),
    Unknown(&'a str),
}

pub type MediaFeature<'a> = QueryFeature<'a, MediaFeatureId>;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MediaFeatureId {
    Width,
    Height,
    AspectRatio,
    Orientation,
    OverflowBlock,
    OverflowInline,
    HorizontalViewportSegments,
    VerticalViewportSegments,
    DisplayMode,
    Resolution,
    Scan,
    Grid,
    Update,
    EnvironmentBlending,
    Color,
    ColorIndex,
    Monochrome,
    ColorGamut,
    DynamicRange,
    InvertedColors,
    Pointer,
    Hover,
    AnyPointer,
    AnyHover,
    NavControls,
    VideoColorGamut,
    VideoDynamicRange,
    Scripting,
    PrefersReducedMotion,
    PrefersReducedTransparency,
    PrefersContrast,
    ForcedColors,
    PrefersColorScheme,
    PrefersReducedData,
    DeviceWidth,
    DeviceHeight,
    DeviceAspectRatio,
    #[css_keyword("-webkit-device-pixel-ratio")]
    WebkitDevicePixelRatio,
    #[css_keyword("-moz-device-pixel-ratio")]
    MozDevicePixelRatio,
}

#[derive(Debug, PartialEq, Visit)]
pub enum MediaFeatureValue<'a> {
    Length(Length<'a>),
    Number(f32),
    Integer(i32),
    Boolean(bool),
    Resolution(Resolution),
    Ratio(Ratio),
    Ident(&'a str),
    Env(NodeId<'a, EnvironmentVariable<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum MediaFeatureComparison {
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Operator {
    And,
    Or,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MediaType<'a> {
    All,
    Print,
    Screen,
    Custom(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Qualifier {
    Only,
    Not,
}

#[derive(Debug, PartialEq, Visit)]
pub enum SupportsCondition<'a> {
    Not(NodeId<'a, SupportsCondition<'a>>),
    And(Vec<'a, SupportsCondition<'a>>),
    Or(Vec<'a, SupportsCondition<'a>>),
    Declaration {
        property_id: NodeId<'a, PropertyId<'a>>,
        value: &'a str,
    },
    Selector(&'a str),
    Unknown(&'a str),
}
