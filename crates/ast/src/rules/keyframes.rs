use crate::*;
#[derive(Debug, PartialEq, Visit)]
pub enum KeyframeSelector {
    Percentage(f32),
    From,
    To,
    TimelineRangePercentage(TimelineRangePercentage),
}

#[derive(Debug, PartialEq, Visit)]
pub enum KeyframesName<'a> {
    Ident(&'a str),
    Custom(&'a str),
}

#[derive(Debug, PartialEq, Visit)]
pub struct KeyframesRule<'a, 'ghost> {
    pub keyframes: Vec<'a, Keyframe<'a, 'ghost>>,
    pub span: Span,
    pub name: Box<'a, KeyframesName<'a>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Keyframe<'a, 'ghost> {
    pub declarations: GhostBox<'a, 'ghost, DeclarationBlock<'a, 'ghost>>,
    pub selectors: Vec<'a, KeyframeSelector>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TimelineRangePercentage {
    pub name: TimelineRangeName,
    pub percentage: f32,
}
