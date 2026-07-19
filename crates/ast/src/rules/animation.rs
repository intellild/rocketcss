use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Transition<'a> {
    pub delay: Box<'a, Time>,
    pub duration: Box<'a, Time>,
    pub property: Box<'a, PropertyId<'a>>,
    pub timing_function: Box<'a, EasingFunction>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollTimeline {
    pub axis: ScrollAxis,
    pub scroller: Scroller,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ViewTimeline<'a> {
    pub axis: ScrollAxis,
    pub inset: Box<'a, Size2D<'a, LengthPercentageOrAuto<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct AnimationRange<'a> {
    pub end: Box<'a, AnimationRangeEnd<'a>>,
    pub start: Box<'a, AnimationRangeStart<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Animation<'a> {
    /// Each component is `Some` only when it was explicitly present in the
    /// shorthand, so serialization preserves authored defaults while printing
    /// them in canonical order.
    pub delay: Option<Box<'a, Time>>,
    pub direction: Option<AnimationDirection>,
    pub duration: Option<Box<'a, Time>>,
    pub fill_mode: Option<AnimationFillMode>,
    pub iteration_count: Option<Box<'a, AnimationIterationCount>>,
    pub name: Option<Box<'a, AnimationName<'a>>>,
    pub play_state: Option<AnimationPlayState>,
    pub timeline: Option<Box<'a, AnimationTimeline<'a>>>,
    pub timing_function: Option<Box<'a, EasingFunction>>,
}
