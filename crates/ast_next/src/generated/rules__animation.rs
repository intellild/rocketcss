use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Transition<'ast>(pub(crate) NodeId<'ast, TransitionData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransitionData<'ast> {
    pub delay: Time,
    pub duration: Time,
    pub property: PropertyId<'ast>,
    pub timing_function: EasingFunction,
}
impl<'ast> NodeStorage<'ast> for Transition<'ast> {
    const KIND: NodeKind = super::kind::KIND_TRANSITION;
    type Data = TransitionData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TransitionData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TransitionData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Transition<'ast> {
    #[inline]
    pub fn delay(self, ast: &Ast<'ast>) -> Time {
        ast.data(self).delay
    }
    #[inline]
    pub fn set_delay(self, ast: &mut Ast<'ast>, value: Time) {
        let mut data = ast.data(self);
        data.delay = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn duration(self, ast: &Ast<'ast>) -> Time {
        ast.data(self).duration
    }
    #[inline]
    pub fn set_duration(self, ast: &mut Ast<'ast>, value: Time) {
        let mut data = ast.data(self);
        data.duration = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn property(self, ast: &Ast<'ast>) -> PropertyId<'ast> {
        ast.data(self).property
    }
    #[inline]
    pub fn set_property(self, ast: &mut Ast<'ast>, value: PropertyId<'ast>) {
        let mut data = ast.data(self);
        data.property = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn timing_function(self, ast: &Ast<'ast>) -> EasingFunction {
        ast.data(self).timing_function
    }
    #[inline]
    pub fn set_timing_function(self, ast: &mut Ast<'ast>, value: EasingFunction) {
        let mut data = ast.data(self);
        data.timing_function = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Transition<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Transition<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_transition(&mut self, data: TransitionData<'ast>, span: Span) -> Transition<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollTimeline<'ast>(pub(crate) NodeId<'ast, ScrollTimelineData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollTimelineData {
    pub axis: ScrollAxis,
    pub scroller: Scroller,
}
impl<'ast> NodeStorage<'ast> for ScrollTimeline<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLL_TIMELINE;
    type Data = ScrollTimelineData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollTimelineData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollTimelineData>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollTimeline<'ast> {
    #[inline]
    pub fn axis(self, ast: &Ast<'ast>) -> ScrollAxis {
        ast.data(self).axis
    }
    #[inline]
    pub fn set_axis(self, ast: &mut Ast<'ast>, value: ScrollAxis) {
        let mut data = ast.data(self);
        data.axis = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn scroller(self, ast: &Ast<'ast>) -> Scroller {
        ast.data(self).scroller
    }
    #[inline]
    pub fn set_scroller(self, ast: &mut Ast<'ast>, value: Scroller) {
        let mut data = ast.data(self);
        data.scroller = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ScrollTimeline<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollTimeline<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scroll_timeline(
        &mut self,
        data: ScrollTimelineData,
        span: Span,
    ) -> ScrollTimeline<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ViewTimeline<'ast>(pub(crate) NodeId<'ast, ViewTimelineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewTimelineData<'ast> {
    pub axis: ScrollAxis,
    pub inset: Size2D<'ast, LengthPercentageOrAuto<'ast>>,
}
impl<'ast> NodeStorage<'ast> for ViewTimeline<'ast> {
    const KIND: NodeKind = super::kind::KIND_VIEW_TIMELINE;
    type Data = ViewTimelineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ViewTimelineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ViewTimelineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ViewTimeline<'ast> {
    #[inline]
    pub fn axis(self, ast: &Ast<'ast>) -> ScrollAxis {
        ast.data(self).axis
    }
    #[inline]
    pub fn set_axis(self, ast: &mut Ast<'ast>, value: ScrollAxis) {
        let mut data = ast.data(self);
        data.axis = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn inset(self, ast: &Ast<'ast>) -> Size2D<'ast, LengthPercentageOrAuto<'ast>> {
        ast.data(self).inset
    }
    #[inline]
    pub fn set_inset(self, ast: &mut Ast<'ast>, value: Size2D<'ast, LengthPercentageOrAuto<'ast>>) {
        let mut data = ast.data(self);
        data.inset = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ViewTimeline<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ViewTimeline<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_view_timeline(
        &mut self,
        data: ViewTimelineData<'ast>,
        span: Span,
    ) -> ViewTimeline<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AnimationRange<'ast>(pub(crate) NodeId<'ast, AnimationRangeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimationRangeData<'ast> {
    pub end: AnimationRangeEnd<'ast>,
    pub start: AnimationRangeStart<'ast>,
}
impl<'ast> NodeStorage<'ast> for AnimationRange<'ast> {
    const KIND: NodeKind = super::kind::KIND_ANIMATION_RANGE;
    type Data = AnimationRangeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, AnimationRangeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, AnimationRangeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> AnimationRange<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> AnimationRangeEnd<'ast> {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: AnimationRangeEnd<'ast>) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> AnimationRangeStart<'ast> {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: AnimationRangeStart<'ast>) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for AnimationRange<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for AnimationRange<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_animation_range(
        &mut self,
        data: AnimationRangeData<'ast>,
        span: Span,
    ) -> AnimationRange<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Animation<'ast>(pub(crate) NodeId<'ast, AnimationData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimationData<'ast> {
    pub components: &'ast [AnimationComponent<'ast>],
}
impl<'ast> NodeStorage<'ast> for Animation<'ast> {
    const KIND: NodeKind = super::kind::KIND_ANIMATION;
    type Data = AnimationData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, AnimationData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, AnimationData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Animation<'ast> {
    #[inline]
    pub fn components(self, ast: &Ast<'ast>) -> &'ast [AnimationComponent<'ast>] {
        ast.data(self).components
    }
    #[inline]
    pub fn set_components(self, ast: &mut Ast<'ast>, value: &'ast [AnimationComponent<'ast>]) {
        let mut data = ast.data(self);
        data.components = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Animation<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Animation<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_animation(&mut self, data: AnimationData<'ast>, span: Span) -> Animation<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AnimationComponent<'ast>(pub(crate) NodeId<'ast, AnimationComponentData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationComponentData<'ast> {
    Name(AnimationName<'ast>),
    Duration(Time),
    TimingFunction(EasingFunction),
    Delay(Time),
    IterationCount(AnimationIterationCount),
    Direction(AnimationDirection),
    FillMode(AnimationFillMode),
    PlayState(AnimationPlayState),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnimationComponentKind {
    Name,
    Duration,
    TimingFunction,
    Delay,
    IterationCount,
    Direction,
    FillMode,
    PlayState,
}
impl<'ast> NodeStorage<'ast> for AnimationComponent<'ast> {
    const KIND: NodeKind = super::kind::KIND_ANIMATION_COMPONENT;
    type Data = AnimationComponentData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, AnimationComponentData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, AnimationComponentData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> AnimationComponent<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> AnimationComponentData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> AnimationComponentKind {
        match ast.data(self) {
            AnimationComponentData::Name { .. } => AnimationComponentKind::Name,
            AnimationComponentData::Duration { .. } => AnimationComponentKind::Duration,
            AnimationComponentData::TimingFunction { .. } => AnimationComponentKind::TimingFunction,
            AnimationComponentData::Delay { .. } => AnimationComponentKind::Delay,
            AnimationComponentData::IterationCount { .. } => AnimationComponentKind::IterationCount,
            AnimationComponentData::Direction { .. } => AnimationComponentKind::Direction,
            AnimationComponentData::FillMode { .. } => AnimationComponentKind::FillMode,
            AnimationComponentData::PlayState { .. } => AnimationComponentKind::PlayState,
        }
    }
}
impl<'ast> GetSpan<'ast> for AnimationComponent<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for AnimationComponent<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_animation_component(
        &mut self,
        data: AnimationComponentData<'ast>,
        span: Span,
    ) -> AnimationComponent<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
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
