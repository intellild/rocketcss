use super::*;

use rocketcss_allocator::{boxed::Box, vec::Vec};

#[derive(Debug, PartialEq)]
pub enum MediaCondition<'a> {
    Feature(Box<'a, MediaFeature<'a>>),
    Not(Box<'a, MediaCondition<'a>>),
    Operation {
        conditions: Vec<'a, MediaCondition<'a>>,
        operator: Operator,
    },
    Unknown(Vec<'a, TokenOrValue<'a>>),
}

#[derive(Debug, PartialEq)]
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
        end: Box<'a, MediaFeatureValue<'a>>,
        end_operator: MediaFeatureComparison,
        name: MediaFeatureName<'a, FeatureId>,
        start: Box<'a, MediaFeatureValue<'a>>,
        start_operator: MediaFeatureComparison,
    },
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureName<'a, FeatureId> {
    Standard(FeatureId),
    Custom(&'a str),
    Unknown(&'a str),
}

pub type MediaFeature<'a> = QueryFeature<'a, MediaFeatureId>;

#[derive(Debug, PartialEq)]
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
    WebkitDevicePixelRatio,
    MozDevicePixelRatio,
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureValue<'a> {
    Length(Length<'a>),
    Number(f32),
    Integer(i32),
    Boolean(bool),
    Resolution(Resolution),
    Ratio(Ratio),
    Ident(&'a str),
    Env(Box<'a, EnvironmentVariable<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureComparison {
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum MediaType<'a> {
    All,
    Print,
    Screen,
    Custom(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum Qualifier {
    Only,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum SupportsCondition<'a> {
    Not(Box<'a, SupportsCondition<'a>>),
    And(Vec<'a, SupportsCondition<'a>>),
    Or(Vec<'a, SupportsCondition<'a>>),
    Declaration {
        property_id: Box<'a, PropertyId<'a>>,
        value: &'a str,
    },
    Selector(&'a str),
    Unknown(&'a str),
    /// Unknown syntax whose declaration-like separators are minified directly
    /// by code generation without allocating a replacement string.
    MinifiedUnknown(&'a str),
}
