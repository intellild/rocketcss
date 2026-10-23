use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FilterList<'ast>(pub(crate) NodeId<'ast, FilterListData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilterListData<'ast> {
    None,
    Filters(&'ast [Filter<'ast>]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FilterListKind {
    None,
    Filters,
}
impl<'ast> NodeStorage<'ast> for FilterList<'ast> {
    const KIND: NodeKind = super::kind::KIND_FILTER_LIST;
    type Data = FilterListData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FilterListData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FilterListData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FilterList<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> FilterListData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> FilterListKind {
        match ast.data(self) {
            FilterListData::None => FilterListKind::None,
            FilterListData::Filters { .. } => FilterListKind::Filters,
        }
    }
}
impl<'ast> GetSpan<'ast> for FilterList<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FilterList<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_filter_list(
        &mut self,
        data: FilterListData<'ast>,
        span: Span,
    ) -> FilterList<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Filter<'ast>(pub(crate) NodeId<'ast, FilterData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilterData<'ast> {
    Blur(Length<'ast>),
    Brightness(NumberOrPercentage),
    Contrast(NumberOrPercentage),
    Grayscale(NumberOrPercentage),
    HueRotate(Angle),
    Invert(NumberOrPercentage),
    Opacity(NumberOrPercentage),
    Saturate(NumberOrPercentage),
    Sepia(NumberOrPercentage),
    DropShadow(DropShadow<'ast>),
    Url(Url<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FilterKind {
    Blur,
    Brightness,
    Contrast,
    Grayscale,
    HueRotate,
    Invert,
    Opacity,
    Saturate,
    Sepia,
    DropShadow,
    Url,
}
impl<'ast> NodeStorage<'ast> for Filter<'ast> {
    const KIND: NodeKind = super::kind::KIND_FILTER;
    type Data = FilterData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FilterData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FilterData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Filter<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> FilterData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> FilterKind {
        match ast.data(self) {
            FilterData::Blur { .. } => FilterKind::Blur,
            FilterData::Brightness { .. } => FilterKind::Brightness,
            FilterData::Contrast { .. } => FilterKind::Contrast,
            FilterData::Grayscale { .. } => FilterKind::Grayscale,
            FilterData::HueRotate { .. } => FilterKind::HueRotate,
            FilterData::Invert { .. } => FilterKind::Invert,
            FilterData::Opacity { .. } => FilterKind::Opacity,
            FilterData::Saturate { .. } => FilterKind::Saturate,
            FilterData::Sepia { .. } => FilterKind::Sepia,
            FilterData::DropShadow { .. } => FilterKind::DropShadow,
            FilterData::Url { .. } => FilterKind::Url,
        }
    }
}
impl<'ast> GetSpan<'ast> for Filter<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Filter<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_filter(&mut self, data: FilterData<'ast>, span: Span) -> Filter<'ast> {
        self.alloc(data, span)
    }
}
