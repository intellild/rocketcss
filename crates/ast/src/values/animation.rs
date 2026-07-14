use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum EasingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier { x1: f32, x2: f32, y1: f32, y2: f32 },
    Steps { count: i32, position: StepPosition },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum StepPosition {
    Start,
    End,
    JumpNone,
    JumpBoth,
}

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationIterationCount {
    Number(f32),
    Infinite,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AnimationComposition {
    Replace,
    Add,
    Accumulate,
}

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationTimeline<'a> {
    Auto,
    None,
    DashedIdent(&'a str),
    Scroll(Box<'a, ScrollTimeline>),
    View(Box<'a, ViewTimeline<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ScrollAxis {
    Block,
    Inline,
    X,
    Y,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Scroller {
    Root,
    Nearest,
    Self_,
}

pub type AnimationRangeStart<'a> = AnimationAttachmentRange<'a>;

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationAttachmentRange<'a> {
    Normal,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    TimelineRange {
        name: TimelineRangeName,
        offset: Box<'a, LengthPercentage<'a>>,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TimelineRangeName {
    Cover,
    Contain,
    Entry,
    Exit,
    EntryCrossing,
    ExitCrossing,
}

pub type AnimationRangeEnd<'a> = AnimationAttachmentRange<'a>;
