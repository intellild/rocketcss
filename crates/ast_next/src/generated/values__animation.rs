use crate::*;
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
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum StepPosition {
    Start,
    End,
    JumpNone,
    JumpBoth,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationIterationCount {
    Number(f32),
    Infinite,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AnimationPlayState {
    Running,
    Paused,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AnimationComposition {
    Replace,
    Add,
    Accumulate,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AnimationTimeline<'ast>(pub(crate) NodeId<'ast, AnimationTimelineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationTimelineData<'ast> {
    Auto,
    None,
    DashedIdent(&'ast str),
    Scroll(ScrollTimeline<'ast>),
    View(ViewTimeline<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnimationTimelineKind {
    Auto,
    None,
    DashedIdent,
    Scroll,
    View,
}
impl<'ast> NodeStorage<'ast> for AnimationTimeline<'ast> {
    const KIND: NodeKind = super::kind::KIND_ANIMATION_TIMELINE;
    type Data = AnimationTimelineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, AnimationTimelineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, AnimationTimelineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> AnimationTimeline<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> AnimationTimelineData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> AnimationTimelineKind {
        match ast.data(self) {
            AnimationTimelineData::Auto => AnimationTimelineKind::Auto,
            AnimationTimelineData::None => AnimationTimelineKind::None,
            AnimationTimelineData::DashedIdent { .. } => AnimationTimelineKind::DashedIdent,
            AnimationTimelineData::Scroll { .. } => AnimationTimelineKind::Scroll,
            AnimationTimelineData::View { .. } => AnimationTimelineKind::View,
        }
    }
}
impl<'ast> GetSpan<'ast> for AnimationTimeline<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for AnimationTimeline<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_animation_timeline(
        &mut self,
        data: AnimationTimelineData<'ast>,
        span: Span,
    ) -> AnimationTimeline<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ScrollAxis {
    Block,
    Inline,
    X,
    Y,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Scroller {
    Root,
    Nearest,
    Self_,
}
pub type AnimationRangeStart<'ast> = AnimationAttachmentRange<'ast>;
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AnimationAttachmentRange<'ast>(
    pub(crate) NodeId<'ast, AnimationAttachmentRangeData<'ast>>,
);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationAttachmentRangeData<'ast> {
    Normal,
    LengthPercentage(LengthPercentage<'ast>),
    TimelineRange {
        name: TimelineRangeName,
        offset: LengthPercentage<'ast>,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnimationAttachmentRangeKind {
    Normal,
    LengthPercentage,
    TimelineRange,
}
impl<'ast> NodeStorage<'ast> for AnimationAttachmentRange<'ast> {
    const KIND: NodeKind = super::kind::KIND_ANIMATION_ATTACHMENT_RANGE;
    type Data = AnimationAttachmentRangeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, AnimationAttachmentRangeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, AnimationAttachmentRangeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> AnimationAttachmentRange<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> AnimationAttachmentRangeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> AnimationAttachmentRangeKind {
        match ast.data(self) {
            AnimationAttachmentRangeData::Normal => AnimationAttachmentRangeKind::Normal,
            AnimationAttachmentRangeData::LengthPercentage { .. } => {
                AnimationAttachmentRangeKind::LengthPercentage
            }
            AnimationAttachmentRangeData::TimelineRange { .. } => {
                AnimationAttachmentRangeKind::TimelineRange
            }
        }
    }
}
impl<'ast> GetSpan<'ast> for AnimationAttachmentRange<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for AnimationAttachmentRange<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_animation_attachment_range(
        &mut self,
        data: AnimationAttachmentRangeData<'ast>,
        span: Span,
    ) -> AnimationAttachmentRange<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TimelineRangeName {
    Cover,
    Contain,
    Entry,
    Exit,
    EntryCrossing,
    ExitCrossing,
}
pub type AnimationRangeEnd<'ast> = AnimationAttachmentRange<'ast>;
