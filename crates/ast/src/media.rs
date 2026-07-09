use super::*;

use rs_css_allocator::{boxed::Box, vec::Vec};

#[derive(Debug, PartialEq)]
pub enum MediaCondition<'a> {
    Feature(Box<'a, QueryFeatureFor_MediaFeatureId<'a>>),
    Not(Box<'a, MediaCondition<'a>>),
    Operation {
        conditions: Vec<'a, MediaCondition<'a>>,
        operator: Operator,
    },
    Unknown(Vec<'a, TokenOrValue<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum QueryFeatureFor_MediaFeatureId<'a> {
    Plain {
        name: Box<'a, MediaFeatureNameFor_MediaFeatureId<'a>>,
        value: Box<'a, MediaFeatureValue<'a>>,
    },
    Boolean {
        name: Box<'a, MediaFeatureNameFor_MediaFeatureId<'a>>,
    },
    Range {
        name: Box<'a, MediaFeatureNameFor_MediaFeatureId<'a>>,
        operator: MediaFeatureComparison,
        value: Box<'a, MediaFeatureValue<'a>>,
    },
    Interval {
        end: Box<'a, MediaFeatureValue<'a>>,
        end_operator: MediaFeatureComparison,
        name: Box<'a, MediaFeatureNameFor_MediaFeatureId<'a>>,
        start: Box<'a, MediaFeatureValue<'a>>,
        start_operator: MediaFeatureComparison,
    },
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureNameFor_MediaFeatureId<'a> {
    MediaFeatureId(MediaFeatureId),
    CssString(&'a str),
    CssString2(&'a str),
}

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
    Length(Box<'a, Length<'a>>),
    Number(f64),
    Integer(f64),
    Boolean(bool),
    Resolution(Box<'a, Resolution>),
    Ratio(Box<'a, Ratio>),
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

pub type MediaType<'a> = &'a str;

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
}
