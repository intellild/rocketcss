use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TrackSizing<'ast>(pub(crate) NodeId<'ast, TrackSizingData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackSizingData<'ast> {
    None,
    TrackList {
        items: &'ast [TrackListItem<'ast>],
        line_names: &'ast [&'ast [&'ast str]],
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrackSizingKind {
    None,
    TrackList,
}
impl<'ast> NodeStorage<'ast> for TrackSizing<'ast> {
    const KIND: NodeKind = super::kind::KIND_TRACK_SIZING;
    type Data = TrackSizingData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TrackSizingData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TrackSizingData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TrackSizing<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TrackSizingData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TrackSizingKind {
        match ast.data(self) {
            TrackSizingData::None { .. } => TrackSizingKind::None,
            TrackSizingData::TrackList { .. } => TrackSizingKind::TrackList,
        }
    }
}
impl<'ast> GetSpan<'ast> for TrackSizing<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TrackSizing<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_track_sizing(
        &mut self,
        data: TrackSizingData<'ast>,
        span: Span,
    ) -> TrackSizing<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TrackListItem<'ast>(pub(crate) NodeId<'ast, TrackListItemData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackListItemData<'ast> {
    TrackSize(TrackSize<'ast>),
    TrackRepeat(TrackRepeat<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrackListItemKind {
    TrackSize,
    TrackRepeat,
}
impl<'ast> NodeStorage<'ast> for TrackListItem<'ast> {
    const KIND: NodeKind = super::kind::KIND_TRACK_LIST_ITEM;
    type Data = TrackListItemData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TrackListItemData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TrackListItemData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TrackListItem<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TrackListItemData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TrackListItemKind {
        match ast.data(self) {
            TrackListItemData::TrackSize { .. } => TrackListItemKind::TrackSize,
            TrackListItemData::TrackRepeat { .. } => TrackListItemKind::TrackRepeat,
        }
    }
}
impl<'ast> GetSpan<'ast> for TrackListItem<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TrackListItem<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_track_list_item(
        &mut self,
        data: TrackListItemData<'ast>,
        span: Span,
    ) -> TrackListItem<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TrackSize<'ast>(pub(crate) NodeId<'ast, TrackSizeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackSizeData<'ast> {
    TrackBreadth(TrackBreadth<'ast>),
    MinMax {
        max: TrackBreadth<'ast>,
        min: TrackBreadth<'ast>,
    },
    FitContent(LengthPercentage<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrackSizeKind {
    TrackBreadth,
    MinMax,
    FitContent,
}
impl<'ast> NodeStorage<'ast> for TrackSize<'ast> {
    const KIND: NodeKind = super::kind::KIND_TRACK_SIZE;
    type Data = TrackSizeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TrackSizeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TrackSizeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TrackSize<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TrackSizeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TrackSizeKind {
        match ast.data(self) {
            TrackSizeData::TrackBreadth { .. } => TrackSizeKind::TrackBreadth,
            TrackSizeData::MinMax { .. } => TrackSizeKind::MinMax,
            TrackSizeData::FitContent { .. } => TrackSizeKind::FitContent,
        }
    }
}
impl<'ast> GetSpan<'ast> for TrackSize<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TrackSize<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_track_size(&mut self, data: TrackSizeData<'ast>, span: Span) -> TrackSize<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TrackBreadth<'ast>(pub(crate) NodeId<'ast, TrackBreadthData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackBreadthData<'ast> {
    Length(LengthPercentage<'ast>),
    Flex(f32),
    MinContent,
    MaxContent,
    Auto,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrackBreadthKind {
    Length,
    Flex,
    MinContent,
    MaxContent,
    Auto,
}
impl<'ast> NodeStorage<'ast> for TrackBreadth<'ast> {
    const KIND: NodeKind = super::kind::KIND_TRACK_BREADTH;
    type Data = TrackBreadthData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TrackBreadthData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TrackBreadthData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TrackBreadth<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TrackBreadthData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TrackBreadthKind {
        match ast.data(self) {
            TrackBreadthData::Length { .. } => TrackBreadthKind::Length,
            TrackBreadthData::Flex { .. } => TrackBreadthKind::Flex,
            TrackBreadthData::MinContent { .. } => TrackBreadthKind::MinContent,
            TrackBreadthData::MaxContent { .. } => TrackBreadthKind::MaxContent,
            TrackBreadthData::Auto { .. } => TrackBreadthKind::Auto,
        }
    }
}
impl<'ast> GetSpan<'ast> for TrackBreadth<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TrackBreadth<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_track_breadth(
        &mut self,
        data: TrackBreadthData<'ast>,
        span: Span,
    ) -> TrackBreadth<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RepeatCount {
    Number(f32),
    AutoFill,
    AutoFit,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AutoFlowDirection {
    Row,
    Column,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GridTemplateAreas<'ast>(pub(crate) NodeId<'ast, GridTemplateAreasData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridTemplateAreasData<'ast> {
    None,
    Areas {
        areas: &'ast [Option<&'ast str>],
        columns: u32,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GridTemplateAreasKind {
    None,
    Areas,
}
impl<'ast> NodeStorage<'ast> for GridTemplateAreas<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRID_TEMPLATE_AREAS;
    type Data = GridTemplateAreasData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GridTemplateAreasData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GridTemplateAreasData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> GridTemplateAreas<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> GridTemplateAreasData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> GridTemplateAreasKind {
        match ast.data(self) {
            GridTemplateAreasData::None { .. } => GridTemplateAreasKind::None,
            GridTemplateAreasData::Areas { .. } => GridTemplateAreasKind::Areas,
        }
    }
}
impl<'ast> GetSpan<'ast> for GridTemplateAreas<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for GridTemplateAreas<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_grid_template_areas(
        &mut self,
        data: GridTemplateAreasData<'ast>,
        span: Span,
    ) -> GridTemplateAreas<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GridLine<'ast>(pub(crate) NodeId<'ast, GridLineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridLineData<'ast> {
    Auto,
    Area { name: &'ast str },
    Line { index: i32, name: Option<&'ast str> },
    Span { index: i32, name: Option<&'ast str> },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GridLineKind {
    Auto,
    Area,
    Line,
    Span,
}
impl<'ast> NodeStorage<'ast> for GridLine<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRID_LINE;
    type Data = GridLineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GridLineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GridLineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> GridLine<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> GridLineData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> GridLineKind {
        match ast.data(self) {
            GridLineData::Auto { .. } => GridLineKind::Auto,
            GridLineData::Area { .. } => GridLineKind::Area,
            GridLineData::Line { .. } => GridLineKind::Line,
            GridLineData::Span { .. } => GridLineKind::Span,
        }
    }
}
impl<'ast> GetSpan<'ast> for GridLine<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for GridLine<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_grid_line(&mut self, data: GridLineData<'ast>, span: Span) -> GridLine<'ast> {
        self.alloc(data, span)
    }
}
