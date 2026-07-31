use super::*;

#[derive(Debug, PartialEq, Visit)]
pub enum MediaCondition<'a> {
    Feature(std::boxed::Box<MediaFeature<'a>>),
    Not(std::boxed::Box<MediaCondition<'a>>),
    Operation {
        conditions: std::vec::Vec<MediaCondition<'a>>,
        operator: Operator,
    },
    Unknown(std::vec::Vec<TokenOrValue<'a>>),
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
        end: std::boxed::Box<MediaFeatureValue<'a>>,
        end_operator: MediaFeatureComparison,
        name: MediaFeatureName<'a, FeatureId>,
        start: std::boxed::Box<MediaFeatureValue<'a>>,
        start_operator: MediaFeatureComparison,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum MediaFeatureName<'a, FeatureId> {
    Standard(FeatureId),
    Custom(Atom<'a>),
    Unknown(Atom<'a>),
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
    Length(Length),
    Number(f32),
    Integer(i32),
    Boolean(bool),
    Resolution(Resolution),
    Ratio(Ratio),
    Ident(Atom<'a>),
    Env(std::boxed::Box<EnvironmentVariable<'a>>),
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
    Custom(Atom<'a>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Qualifier {
    Only,
    Not,
}

#[derive(Debug, PartialEq, Visit)]
pub enum SupportsCondition<'a> {
    Not(std::boxed::Box<SupportsCondition<'a>>),
    And(std::vec::Vec<SupportsCondition<'a>>),
    Or(std::vec::Vec<SupportsCondition<'a>>),
    Declaration {
        property_id: std::boxed::Box<PropertyId<'a>>,
        value: &'a str,
    },
    Selector(&'a str),
    Unknown(&'a str),
}
