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
    /// Components in authored order, so parsing and printing round-trips
    /// losslessly. The `ORDER_VALUES` minify pass sorts them into canonical
    /// order in place.
    pub components: Vec<'a, AnimationComponent<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationComponent<'a> {
    Name(Box<'a, AnimationName<'a>>),
    Duration(Box<'a, Time>),
    TimingFunction(Box<'a, EasingFunction>),
    Delay(Box<'a, Time>),
    IterationCount(Box<'a, AnimationIterationCount>),
    Direction(AnimationDirection),
    FillMode(AnimationFillMode),
    PlayState(AnimationPlayState),
}

/// The keyword class an animation component (or a keyframes name colliding
/// with one) belongs to, used to keep shorthand serialization round-trip
/// safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub enum AnimationKeywordClass {
    TimingFunction,
    IterationCount,
    Direction,
    FillMode,
    PlayState,
}

impl AnimationComponent<'_> {
    /// The keyword class of a non-name component.
    pub fn keyword_class(&self) -> Option<AnimationKeywordClass> {
        match self {
            Self::TimingFunction(_) => Some(AnimationKeywordClass::TimingFunction),
            Self::IterationCount(_) => Some(AnimationKeywordClass::IterationCount),
            Self::Direction(_) => Some(AnimationKeywordClass::Direction),
            Self::FillMode(_) => Some(AnimationKeywordClass::FillMode),
            Self::PlayState(_) => Some(AnimationKeywordClass::PlayState),
            Self::Name(_) | Self::Duration(_) | Self::Delay(_) => None,
        }
    }
}

impl AnimationName<'_> {
    /// The keyword class this name collides with on reparse, mirroring the
    /// disambiguation in lightningcss and stylo. Quoted names print without
    /// quotes unless they are CSS-wide keywords or `none`, so they collide
    /// like idents; the `none` name is excluded because fill-mode's initial
    /// value is already `none`.
    pub fn keyword_class(&self) -> Option<AnimationKeywordClass> {
        let name = match self {
            Self::Ident(name) | Self::String(name) => *name,
            Self::None => return None,
        };
        match_ignore_ascii_case!(
            name,
            "linear" | "ease" | "ease-in" | "ease-out" | "ease-in-out" | "step-start" | "step-end" =>
                Some(AnimationKeywordClass::TimingFunction),
            "infinite" => Some(AnimationKeywordClass::IterationCount),
            "normal" | "reverse" | "alternate" | "alternate-reverse" =>
                Some(AnimationKeywordClass::Direction),
            "forwards" | "backwards" | "both" => Some(AnimationKeywordClass::FillMode),
            "running" | "paused" => Some(AnimationKeywordClass::PlayState),
            _ => None,
        )
    }
}
