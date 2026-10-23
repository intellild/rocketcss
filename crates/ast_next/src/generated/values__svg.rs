use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SVGPaint<'ast>(pub(crate) NodeId<'ast, SVGPaintData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVGPaintData<'ast> {
    Url {
        fallback: Option<SVGPaintFallback<'ast>>,
        url: Url<'ast>,
    },
    Color(CssColor<'ast>),
    ContextFill,
    ContextStroke,
    None,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVGPaintKind {
    Url,
    Color,
    ContextFill,
    ContextStroke,
    None,
}
impl<'ast> NodeStorage<'ast> for SVGPaint<'ast> {
    const KIND: NodeKind = super::kind::KIND_SVG_PAINT;
    type Data = SVGPaintData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SVGPaintData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SVGPaintData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> SVGPaint<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SVGPaintData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SVGPaintKind {
        match ast.data(self) {
            SVGPaintData::Url { .. } => SVGPaintKind::Url,
            SVGPaintData::Color { .. } => SVGPaintKind::Color,
            SVGPaintData::ContextFill => SVGPaintKind::ContextFill,
            SVGPaintData::ContextStroke => SVGPaintKind::ContextStroke,
            SVGPaintData::None => SVGPaintKind::None,
        }
    }
}
impl<'ast> GetSpan<'ast> for SVGPaint<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for SVGPaint<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_svg_paint(&mut self, data: SVGPaintData<'ast>, span: Span) -> SVGPaint<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SVGPaintFallback<'ast>(pub(crate) NodeId<'ast, SVGPaintFallbackData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVGPaintFallbackData<'ast> {
    None,
    Color(CssColor<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVGPaintFallbackKind {
    None,
    Color,
}
impl<'ast> NodeStorage<'ast> for SVGPaintFallback<'ast> {
    const KIND: NodeKind = super::kind::KIND_SVG_PAINT_FALLBACK;
    type Data = SVGPaintFallbackData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SVGPaintFallbackData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SVGPaintFallbackData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> SVGPaintFallback<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SVGPaintFallbackData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SVGPaintFallbackKind {
        match ast.data(self) {
            SVGPaintFallbackData::None => SVGPaintFallbackKind::None,
            SVGPaintFallbackData::Color { .. } => SVGPaintFallbackKind::Color,
        }
    }
}
impl<'ast> GetSpan<'ast> for SVGPaintFallback<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for SVGPaintFallback<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_svg_paint_fallback(
        &mut self,
        data: SVGPaintFallbackData<'ast>,
        span: Span,
    ) -> SVGPaintFallback<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum StrokeLinecap {
    Butt,
    Round,
    Square,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum StrokeLinejoin {
    Miter,
    MiterClip,
    Round,
    Bevel,
    Arcs,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StrokeDasharray<'ast>(pub(crate) NodeId<'ast, StrokeDasharrayData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StrokeDasharrayData<'ast> {
    None,
    Values(&'ast [LengthPercentage<'ast>]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StrokeDasharrayKind {
    None,
    Values,
}
impl<'ast> NodeStorage<'ast> for StrokeDasharray<'ast> {
    const KIND: NodeKind = super::kind::KIND_STROKE_DASHARRAY;
    type Data = StrokeDasharrayData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, StrokeDasharrayData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, StrokeDasharrayData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> StrokeDasharray<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> StrokeDasharrayData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> StrokeDasharrayKind {
        match ast.data(self) {
            StrokeDasharrayData::None => StrokeDasharrayKind::None,
            StrokeDasharrayData::Values { .. } => StrokeDasharrayKind::Values,
        }
    }
}
impl<'ast> GetSpan<'ast> for StrokeDasharray<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for StrokeDasharray<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_stroke_dasharray(
        &mut self,
        data: StrokeDasharrayData<'ast>,
        span: Span,
    ) -> StrokeDasharray<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Marker<'ast>(pub(crate) NodeId<'ast, MarkerData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MarkerData<'ast> {
    None,
    Url(Url<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MarkerKind {
    None,
    Url,
}
impl<'ast> NodeStorage<'ast> for Marker<'ast> {
    const KIND: NodeKind = super::kind::KIND_MARKER;
    type Data = MarkerData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MarkerData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MarkerData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Marker<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> MarkerData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> MarkerKind {
        match ast.data(self) {
            MarkerData::None => MarkerKind::None,
            MarkerData::Url { .. } => MarkerKind::Url,
        }
    }
}
impl<'ast> GetSpan<'ast> for Marker<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Marker<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_marker(&mut self, data: MarkerData<'ast>, span: Span) -> Marker<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ColorInterpolation {
    Auto,
    Srgb,
    Linearrgb,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ColorRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ShapeRendering {
    Auto,
    Optimizespeed,
    Crispedges,
    Geometricprecision,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextRendering {
    Auto,
    Optimizespeed,
    Optimizelegibility,
    Geometricprecision,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ImageRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}
