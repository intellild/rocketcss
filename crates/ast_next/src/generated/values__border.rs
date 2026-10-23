use crate::*;
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LineStyle {
    None,
    Hidden,
    Inset,
    Groove,
    Outset,
    Ridge,
    Dotted,
    Dashed,
    Solid,
    Double,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderSideWidth<'ast>(pub(crate) NodeId<'ast, BorderSideWidthData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BorderSideWidthData<'ast> {
    Thin,
    Medium,
    Thick,
    Length(Length<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BorderSideWidthKind {
    Thin,
    Medium,
    Thick,
    Length,
}
impl<'ast> NodeStorage<'ast> for BorderSideWidth<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_SIDE_WIDTH;
    type Data = BorderSideWidthData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderSideWidthData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderSideWidthData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderSideWidth<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> BorderSideWidthData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> BorderSideWidthKind {
        match ast.data(self) {
            BorderSideWidthData::Thin => BorderSideWidthKind::Thin,
            BorderSideWidthData::Medium => BorderSideWidthKind::Medium,
            BorderSideWidthData::Thick => BorderSideWidthKind::Thick,
            BorderSideWidthData::Length { .. } => BorderSideWidthKind::Length,
        }
    }
}
impl<'ast> GetSpan<'ast> for BorderSideWidth<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderSideWidth<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_side_width(
        &mut self,
        data: BorderSideWidthData<'ast>,
        span: Span,
    ) -> BorderSideWidth<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LengthOrNumber<'ast>(pub(crate) NodeId<'ast, LengthOrNumberData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthOrNumberData<'ast> {
    Number(f32),
    Length(Length<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LengthOrNumberKind {
    Number,
    Length,
}
impl<'ast> NodeStorage<'ast> for LengthOrNumber<'ast> {
    const KIND: NodeKind = super::kind::KIND_LENGTH_OR_NUMBER;
    type Data = LengthOrNumberData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, LengthOrNumberData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, LengthOrNumberData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> LengthOrNumber<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> LengthOrNumberData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> LengthOrNumberKind {
        match ast.data(self) {
            LengthOrNumberData::Number { .. } => LengthOrNumberKind::Number,
            LengthOrNumberData::Length { .. } => LengthOrNumberKind::Length,
        }
    }
}
impl<'ast> GetSpan<'ast> for LengthOrNumber<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for LengthOrNumber<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_length_or_number(
        &mut self,
        data: LengthOrNumberData<'ast>,
        span: Span,
    ) -> LengthOrNumber<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderImageSideWidth<'ast>(pub(crate) NodeId<'ast, BorderImageSideWidthData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BorderImageSideWidthData<'ast> {
    Number(f32),
    LengthPercentage(LengthPercentage<'ast>),
    Auto,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BorderImageSideWidthKind {
    Number,
    LengthPercentage,
    Auto,
}
impl<'ast> NodeStorage<'ast> for BorderImageSideWidth<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_IMAGE_SIDE_WIDTH;
    type Data = BorderImageSideWidthData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderImageSideWidthData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderImageSideWidthData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderImageSideWidth<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> BorderImageSideWidthData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> BorderImageSideWidthKind {
        match ast.data(self) {
            BorderImageSideWidthData::Number { .. } => BorderImageSideWidthKind::Number,
            BorderImageSideWidthData::LengthPercentage { .. } => {
                BorderImageSideWidthKind::LengthPercentage
            }
            BorderImageSideWidthData::Auto => BorderImageSideWidthKind::Auto,
        }
    }
}
impl<'ast> GetSpan<'ast> for BorderImageSideWidth<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderImageSideWidth<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_image_side_width(
        &mut self,
        data: BorderImageSideWidthData<'ast>,
        span: Span,
    ) -> BorderImageSideWidth<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutlineStyle<'ast>(pub(crate) NodeId<'ast, OutlineStyleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutlineStyleData {
    Auto,
    LineStyle(LineStyle),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OutlineStyleKind {
    Auto,
    LineStyle,
}
impl<'ast> NodeStorage<'ast> for OutlineStyle<'ast> {
    const KIND: NodeKind = super::kind::KIND_OUTLINE_STYLE;
    type Data = OutlineStyleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, OutlineStyleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, OutlineStyleData>) -> Self {
        Self(id)
    }
}
impl<'ast> OutlineStyle<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> OutlineStyleData {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> OutlineStyleKind {
        match ast.data(self) {
            OutlineStyleData::Auto => OutlineStyleKind::Auto,
            OutlineStyleData::LineStyle { .. } => OutlineStyleKind::LineStyle,
        }
    }
}
impl<'ast> GetSpan<'ast> for OutlineStyle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for OutlineStyle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_outline_style(
        &mut self,
        data: OutlineStyleData,
        span: Span,
    ) -> OutlineStyle<'ast> {
        self.alloc(data, span)
    }
}
