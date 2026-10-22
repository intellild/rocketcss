#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EasingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier { x1: f32, x2: f32, y1: f32, y2: f32 },
    Frames(i32),
    Steps { count: i32, position: StepPosition },
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum StepPosition {
    Start,
    End,
    JumpNone,
    JumpBoth,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationIterationCount {
    Number(f32),
    Infinite,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AnimationComposition {
    Replace,
    Add,
    Accumulate,
}

#[ast]
pub enum AnimationTimeline<'a> {
    Auto,
    None,
    DashedIdent(&'a str),
    Scroll(ScrollTimeline),
    View(ViewTimeline<'a>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ScrollAxis {
    Block,
    Inline,
    X,
    Y,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Scroller {
    Root,
    Nearest,
    Self_,
}

pub type AnimationRangeStart<'a> = AnimationAttachmentRange<'a>;

#[ast]
pub enum AnimationAttachmentRange<'a> {
    Normal,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    TimelineRange {
        name: TimelineRangeName,
        offset: Box<'a, LengthPercentage<'a>>,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TimelineRangeName {
    Cover,
    Contain,
    Entry,
    Exit,
    EntryCrossing,
    ExitCrossing,
}

pub type AnimationRangeEnd<'a> = AnimationAttachmentRange<'a>;
