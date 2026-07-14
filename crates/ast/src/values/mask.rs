use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MaskMode {
    Luminance,
    Alpha,
    MatchSource,
}

#[derive(Debug, PartialEq, Visit)]
pub enum MaskClip {
    GeometryBox(GeometryBox),
    NoClip,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MaskComposite {
    Add,
    Subtract,
    Intersect,
    Exclude,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MaskType {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MaskBorderMode {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum WebKitMaskComposite {
    Clear,
    Copy,
    SourceOver,
    SourceIn,
    SourceOut,
    SourceAtop,
    DestinationOver,
    DestinationIn,
    DestinationOut,
    DestinationAtop,
    Xor,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum WebKitMaskSourceType {
    Auto,
    Luminance,
    Alpha,
}
