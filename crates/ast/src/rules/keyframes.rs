use crate::*;
use std::pin::Pin;

#[derive(Debug, PartialEq, Visit)]
pub enum KeyframeSelector<'a> {
    Percentage(f32),
    From,
    To,
    TimelineRangePercentage(Box<'a, TimelineRangePercentage>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum KeyframesName<'a> {
    Ident(&'a str),
    Custom(&'a str),
}

#[derive(Debug, PartialEq, Visit)]
pub struct KeyframesRule<'a> {
    pub keyframes: Vec<'a, Keyframe<'a>>,
    pub span: Span,
    pub name: Box<'a, KeyframesName<'a>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Keyframe<'a> {
    pub declarations: Pin<Box<'a, DeclarationBlock<'a>>>,
    pub selectors: Vec<'a, KeyframeSelector<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TimelineRangePercentage {
    pub name: TimelineRangeName,
    pub percentage: f32,
}
