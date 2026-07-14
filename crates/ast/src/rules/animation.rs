use crate::*;

#[derive(Debug, PartialEq)]
pub struct Transition<'a> {
    pub delay: Box<'a, Time>,
    pub duration: Box<'a, Time>,
    pub property: Box<'a, PropertyId<'a>>,
    pub timing_function: Box<'a, EasingFunction>,
}

#[derive(Debug, PartialEq)]
pub struct ScrollTimeline {
    pub axis: ScrollAxis,
    pub scroller: Scroller,
}

#[derive(Debug, PartialEq)]
pub struct ViewTimeline<'a> {
    pub axis: ScrollAxis,
    pub inset: Box<'a, Size2D<'a, LengthPercentageOrAuto<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct AnimationRange<'a> {
    pub end: Box<'a, AnimationRangeEnd<'a>>,
    pub start: Box<'a, AnimationRangeStart<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Animation<'a> {
    pub delay: Box<'a, Time>,
    pub direction: AnimationDirection,
    pub duration: Box<'a, Time>,
    pub fill_mode: AnimationFillMode,
    pub iteration_count: Box<'a, AnimationIterationCount>,
    pub name: Box<'a, AnimationName<'a>>,
    pub play_state: AnimationPlayState,
    pub timeline: Box<'a, AnimationTimeline<'a>>,
    pub timing_function: Box<'a, EasingFunction>,
}
