use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Transition<'a> {
    pub delay: Time,
    pub duration: Time,
    pub property: std::boxed::Box<PropertyId<'a>>,
    pub timing_function: std::boxed::Box<EasingFunction>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollTimeline {
    pub axis: ScrollAxis,
    pub scroller: Scroller,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ViewTimeline {
    pub axis: ScrollAxis,
    pub inset: std::boxed::Box<Size2D<LengthPercentageOrAuto>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct AnimationRange {
    pub end: std::boxed::Box<AnimationRangeEnd>,
    pub start: std::boxed::Box<AnimationRangeStart>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Animation<'a> {
    /// Components in authored order, so parsing and printing round-trips
    /// losslessly. The `ORDER_VALUES` minify pass sorts them into canonical
    /// order in place.
    pub components: std::vec::Vec<AnimationComponent<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationComponent<'a> {
    Name(std::boxed::Box<AnimationName<'a>>),
    Duration(Time),
    TimingFunction(std::boxed::Box<EasingFunction>),
    Delay(Time),
    IterationCount(AnimationIterationCount),
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
            Self::Ident(name) => name.as_str(),
            Self::String(name) => name.as_str(),
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
