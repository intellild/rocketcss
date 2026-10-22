use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MaskMode {
    Luminance,
    Alpha,
    MatchSource,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MaskClip {
    GeometryBox(GeometryBox),
    NoClip,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MaskComposite {
    Add,
    Subtract,
    Intersect,
    Exclude,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MaskType {
    Luminance,
    Alpha,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MaskBorderMode {
    Luminance,
    Alpha,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum WebKitMaskSourceType {
    Auto,
    Luminance,
    Alpha,
}
