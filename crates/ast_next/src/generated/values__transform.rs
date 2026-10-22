use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Transform<'ast>(pub(crate) NodeId<'ast, TransformData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransformData<'ast> {
    Translate((LengthPercentage<'ast>, LengthPercentage<'ast>)),
    TranslateX(LengthPercentage<'ast>),
    TranslateY(LengthPercentage<'ast>),
    TranslateZ(Length<'ast>),
    Translate3d((LengthPercentage<'ast>, LengthPercentage<'ast>, Length<'ast>)),
    Scale((NumberOrPercentage, NumberOrPercentage)),
    ScaleX(NumberOrPercentage),
    ScaleY(NumberOrPercentage),
    ScaleZ(NumberOrPercentage),
    Scale3d((NumberOrPercentage, NumberOrPercentage, NumberOrPercentage)),
    Rotate(Angle),
    RotateX(Angle),
    RotateY(Angle),
    RotateZ(Angle),
    Rotate3d((f32, f32, f32, Angle)),
    Skew((Angle, Angle)),
    SkewX(Angle),
    SkewY(Angle),
    Perspective(Length<'ast>),
    Matrix(MatrixForFloat<'ast>),
    Matrix3d(Matrix3DForFloat<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransformKind {
    Translate,
    TranslateX,
    TranslateY,
    TranslateZ,
    Translate3d,
    Scale,
    ScaleX,
    ScaleY,
    ScaleZ,
    Scale3d,
    Rotate,
    RotateX,
    RotateY,
    RotateZ,
    Rotate3d,
    Skew,
    SkewX,
    SkewY,
    Perspective,
    Matrix,
    Matrix3d,
}
impl<'ast> NodeStorage<'ast> for Transform<'ast> {
    const KIND: NodeKind = super::kind::KIND_TRANSFORM;
    type Data = TransformData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TransformData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TransformData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Transform<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TransformData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TransformKind {
        match ast.data(self) {
            TransformData::Translate { .. } => TransformKind::Translate,
            TransformData::TranslateX { .. } => TransformKind::TranslateX,
            TransformData::TranslateY { .. } => TransformKind::TranslateY,
            TransformData::TranslateZ { .. } => TransformKind::TranslateZ,
            TransformData::Translate3d { .. } => TransformKind::Translate3d,
            TransformData::Scale { .. } => TransformKind::Scale,
            TransformData::ScaleX { .. } => TransformKind::ScaleX,
            TransformData::ScaleY { .. } => TransformKind::ScaleY,
            TransformData::ScaleZ { .. } => TransformKind::ScaleZ,
            TransformData::Scale3d { .. } => TransformKind::Scale3d,
            TransformData::Rotate { .. } => TransformKind::Rotate,
            TransformData::RotateX { .. } => TransformKind::RotateX,
            TransformData::RotateY { .. } => TransformKind::RotateY,
            TransformData::RotateZ { .. } => TransformKind::RotateZ,
            TransformData::Rotate3d { .. } => TransformKind::Rotate3d,
            TransformData::Skew { .. } => TransformKind::Skew,
            TransformData::SkewX { .. } => TransformKind::SkewX,
            TransformData::SkewY { .. } => TransformKind::SkewY,
            TransformData::Perspective { .. } => TransformKind::Perspective,
            TransformData::Matrix { .. } => TransformKind::Matrix,
            TransformData::Matrix3d { .. } => TransformKind::Matrix3d,
        }
    }
}
impl<'ast> GetSpan<'ast> for Transform<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Transform<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_transform(&mut self, data: TransformData<'ast>, span: Span) -> Transform<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TransformStyle {
    Flat,
    Preserve3d,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TransformBox {
    ContentBox,
    BorderBox,
    FillBox,
    StrokeBox,
    ViewBox,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Perspective<'ast>(pub(crate) NodeId<'ast, PerspectiveData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PerspectiveData<'ast> {
    None,
    Length(Length<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PerspectiveKind {
    None,
    Length,
}
impl<'ast> NodeStorage<'ast> for Perspective<'ast> {
    const KIND: NodeKind = super::kind::KIND_PERSPECTIVE;
    type Data = PerspectiveData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PerspectiveData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PerspectiveData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Perspective<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> PerspectiveData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> PerspectiveKind {
        match ast.data(self) {
            PerspectiveData::None { .. } => PerspectiveKind::None,
            PerspectiveData::Length { .. } => PerspectiveKind::Length,
        }
    }
}
impl<'ast> GetSpan<'ast> for Perspective<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Perspective<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_perspective(
        &mut self,
        data: PerspectiveData<'ast>,
        span: Span,
    ) -> Perspective<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Translate<'ast>(pub(crate) NodeId<'ast, TranslateData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TranslateData<'ast> {
    None,
    Xyz {
        x: LengthPercentage<'ast>,
        y: LengthPercentage<'ast>,
        z: Length<'ast>,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TranslateKind {
    None,
    Xyz,
}
impl<'ast> NodeStorage<'ast> for Translate<'ast> {
    const KIND: NodeKind = super::kind::KIND_TRANSLATE;
    type Data = TranslateData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TranslateData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TranslateData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Translate<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TranslateData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TranslateKind {
        match ast.data(self) {
            TranslateData::None { .. } => TranslateKind::None,
            TranslateData::Xyz { .. } => TranslateKind::Xyz,
        }
    }
}
impl<'ast> GetSpan<'ast> for Translate<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Translate<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_translate(&mut self, data: TranslateData<'ast>, span: Span) -> Translate<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scale {
    None,
    Xyz {
        x: NumberOrPercentage,
        y: NumberOrPercentage,
        z: NumberOrPercentage,
    },
}
