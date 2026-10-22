use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClipPath<'ast>(pub(crate) NodeId<'ast, ClipPathData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClipPathData<'ast> {
    None,
    Url(Url<'ast>),
    Shape {
        reference_box: GeometryBox,
        shape: BasicShape<'ast>,
    },
    Box(GeometryBox),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClipPathKind {
    None,
    Url,
    Shape,
    Box,
}
impl<'ast> NodeStorage<'ast> for ClipPath<'ast> {
    const KIND: NodeKind = super::kind::KIND_CLIP_PATH;
    type Data = ClipPathData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ClipPathData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ClipPathData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ClipPath<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ClipPathData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ClipPathKind {
        match ast.data(self) {
            ClipPathData::None { .. } => ClipPathKind::None,
            ClipPathData::Url { .. } => ClipPathKind::Url,
            ClipPathData::Shape { .. } => ClipPathKind::Shape,
            ClipPathData::Box { .. } => ClipPathKind::Box,
        }
    }
}
impl<'ast> GetSpan<'ast> for ClipPath<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ClipPath<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_clip_path(&mut self, data: ClipPathData<'ast>, span: Span) -> ClipPath<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum GeometryBox {
    BorderBox,
    PaddingBox,
    ContentBox,
    MarginBox,
    FillBox,
    StrokeBox,
    ViewBox,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BasicShape<'ast>(pub(crate) NodeId<'ast, BasicShapeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BasicShapeData<'ast> {
    Inset(InsetRect<'ast>),
    Circle(CircleShape<'ast>),
    Ellipse(EllipseShape<'ast>),
    Polygon(Polygon<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BasicShapeKind {
    Inset,
    Circle,
    Ellipse,
    Polygon,
}
impl<'ast> NodeStorage<'ast> for BasicShape<'ast> {
    const KIND: NodeKind = super::kind::KIND_BASIC_SHAPE;
    type Data = BasicShapeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BasicShapeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BasicShapeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BasicShape<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> BasicShapeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> BasicShapeKind {
        match ast.data(self) {
            BasicShapeData::Inset { .. } => BasicShapeKind::Inset,
            BasicShapeData::Circle { .. } => BasicShapeKind::Circle,
            BasicShapeData::Ellipse { .. } => BasicShapeKind::Ellipse,
            BasicShapeData::Polygon { .. } => BasicShapeKind::Polygon,
        }
    }
}
impl<'ast> GetSpan<'ast> for BasicShape<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BasicShape<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_basic_shape(
        &mut self,
        data: BasicShapeData<'ast>,
        span: Span,
    ) -> BasicShape<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ShapeRadius<'ast>(pub(crate) NodeId<'ast, ShapeRadiusData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeRadiusData<'ast> {
    LengthPercentage(LengthPercentage<'ast>),
    ClosestSide,
    FarthestSide,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeRadiusKind {
    LengthPercentage,
    ClosestSide,
    FarthestSide,
}
impl<'ast> NodeStorage<'ast> for ShapeRadius<'ast> {
    const KIND: NodeKind = super::kind::KIND_SHAPE_RADIUS;
    type Data = ShapeRadiusData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ShapeRadiusData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ShapeRadiusData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ShapeRadius<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ShapeRadiusData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ShapeRadiusKind {
        match ast.data(self) {
            ShapeRadiusData::LengthPercentage { .. } => ShapeRadiusKind::LengthPercentage,
            ShapeRadiusData::ClosestSide { .. } => ShapeRadiusKind::ClosestSide,
            ShapeRadiusData::FarthestSide { .. } => ShapeRadiusKind::FarthestSide,
        }
    }
}
impl<'ast> GetSpan<'ast> for ShapeRadius<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ShapeRadius<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_shape_radius(
        &mut self,
        data: ShapeRadiusData<'ast>,
        span: Span,
    ) -> ShapeRadius<'ast> {
        self.alloc(data, span)
    }
}
