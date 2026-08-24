use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Transition<'a> {
    pub delay: Time,
    pub duration: Time,
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
    pub name: Box<'a, AnimationName<'a>>,
    pub duration: Time,
    pub timing_function: Box<'a, EasingFunction>,
    pub iteration_count: AnimationIterationCount,
    pub direction: AnimationDirection,
    pub play_state: AnimationPlayState,
    pub delay: Time,
    pub fill_mode: AnimationFillMode,
    pub timeline: Box<'a, AnimationTimeline<'a>>,
}
