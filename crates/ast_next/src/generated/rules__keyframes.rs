use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct KeyframeSelector<'ast>(pub(crate) NodeId<'ast, KeyframeSelectorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyframeSelectorData<'ast> {
    Percentage(f32),
    From,
    To,
    TimelineRangePercentage(TimelineRangePercentage<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyframeSelectorKind {
    Percentage,
    From,
    To,
    TimelineRangePercentage,
}
impl<'ast> NodeStorage<'ast> for KeyframeSelector<'ast> {
    const KIND: NodeKind = super::kind::KIND_KEYFRAME_SELECTOR;
    type Data = KeyframeSelectorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, KeyframeSelectorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, KeyframeSelectorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> KeyframeSelector<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> KeyframeSelectorData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> KeyframeSelectorKind {
        match ast.data(self) {
            KeyframeSelectorData::Percentage { .. } => KeyframeSelectorKind::Percentage,
            KeyframeSelectorData::From { .. } => KeyframeSelectorKind::From,
            KeyframeSelectorData::To { .. } => KeyframeSelectorKind::To,
            KeyframeSelectorData::TimelineRangePercentage { .. } => {
                KeyframeSelectorKind::TimelineRangePercentage
            }
        }
    }
}
impl<'ast> GetSpan<'ast> for KeyframeSelector<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for KeyframeSelector<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_keyframe_selector(
        &mut self,
        data: KeyframeSelectorData<'ast>,
        span: Span,
    ) -> KeyframeSelector<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyframesName<'ast> {
    Ident(&'ast str),
    Custom(&'ast str),
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TimelineRangePercentage<'ast>(pub(crate) NodeId<'ast, TimelineRangePercentageData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimelineRangePercentageData {
    pub name: TimelineRangeName,
    pub percentage: f32,
}
impl<'ast> NodeStorage<'ast> for TimelineRangePercentage<'ast> {
    const KIND: NodeKind = super::kind::KIND_TIMELINE_RANGE_PERCENTAGE;
    type Data = TimelineRangePercentageData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TimelineRangePercentageData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TimelineRangePercentageData>) -> Self {
        Self(id)
    }
}
impl<'ast> TimelineRangePercentage<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> TimelineRangeName {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: TimelineRangeName) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn percentage(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).percentage
    }
    #[inline]
    pub fn set_percentage(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.percentage = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for TimelineRangePercentage<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TimelineRangePercentage<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_timeline_range_percentage(
        &mut self,
        data: TimelineRangePercentageData,
        span: Span,
    ) -> TimelineRangePercentage<'ast> {
        self.alloc(data, span)
    }
}
