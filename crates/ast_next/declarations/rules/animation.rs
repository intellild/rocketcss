#[ast]
pub struct Transition<'a> {
    pub delay: Time,
    pub duration: Time,
    pub property: Box<'a, PropertyId<'a>>,
    pub timing_function: Box<'a, EasingFunction>,
}

#[ast]
pub struct ScrollTimeline {
    pub axis: ScrollAxis,
    pub scroller: Scroller,
}

#[ast]
pub struct ViewTimeline<'a> {
    pub axis: ScrollAxis,
    pub inset: Box<'a, Size2D<'a, LengthPercentageOrAuto<'a>>>,
}

#[ast]
pub struct AnimationRange<'a> {
    pub end: Box<'a, AnimationRangeEnd<'a>>,
    pub start: Box<'a, AnimationRangeStart<'a>>,
}

#[ast]
pub struct Animation<'a> {
    /// Components in authored order, so parsing and printing round-trips
    /// losslessly. The `ORDER_VALUES` minify pass sorts them into canonical
    /// order in place.
    pub components: Vec<'a, AnimationComponent<'a>>,
}

#[ast]
pub enum AnimationComponent<'a> {
    Name(Box<'a, AnimationName<'a>>),
    Duration(Time),
    TimingFunction(Box<'a, EasingFunction>),
    Delay(Time),
    IterationCount(AnimationIterationCount),
    Direction(AnimationDirection),
    FillMode(AnimationFillMode),
    PlayState(AnimationPlayState),
}

/// The keyword class an animation component (or a keyframes name colliding
/// with one) belongs to, used to keep shorthand serialization round-trip
/// safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnimationKeywordClass {
    TimingFunction,
    IterationCount,
    Direction,
    FillMode,
    PlayState,
}
