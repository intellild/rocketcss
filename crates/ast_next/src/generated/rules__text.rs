use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextTransform<'ast>(pub(crate) NodeId<'ast, TextTransformData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextTransformData {
    pub case: TextTransformCase,
    pub full_size_kana: bool,
    pub full_width: bool,
}
impl<'ast> NodeStorage<'ast> for TextTransform<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_TRANSFORM;
    type Data = TextTransformData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextTransformData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextTransformData>) -> Self {
        Self(id)
    }
}
impl<'ast> TextTransform<'ast> {
    #[inline]
    pub fn case(self, ast: &Ast<'ast>) -> TextTransformCase {
        ast.data(self).case
    }
    #[inline]
    pub fn set_case(self, ast: &mut Ast<'ast>, value: TextTransformCase) {
        let mut data = ast.data(self);
        data.case = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn full_size_kana(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).full_size_kana
    }
    #[inline]
    pub fn set_full_size_kana(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.full_size_kana = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn full_width(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).full_width
    }
    #[inline]
    pub fn set_full_width(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.full_width = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for TextTransform<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextTransform<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_transform(
        &mut self,
        data: TextTransformData,
        span: Span,
    ) -> TextTransform<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextIndent<'ast>(pub(crate) NodeId<'ast, TextIndentData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextIndentData<'ast> {
    pub each_line: bool,
    pub hanging: bool,
    pub value: LengthPercentage<'ast>,
}
impl<'ast> NodeStorage<'ast> for TextIndent<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_INDENT;
    type Data = TextIndentData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextIndentData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextIndentData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TextIndent<'ast> {
    #[inline]
    pub fn each_line(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).each_line
    }
    #[inline]
    pub fn set_each_line(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.each_line = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn hanging(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).hanging
    }
    #[inline]
    pub fn set_hanging(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.hanging = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn value(self, ast: &Ast<'ast>) -> LengthPercentage<'ast> {
        ast.data(self).value
    }
    #[inline]
    pub fn set_value(self, ast: &mut Ast<'ast>, value: LengthPercentage<'ast>) {
        let mut data = ast.data(self);
        data.value = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for TextIndent<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextIndent<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_indent(
        &mut self,
        data: TextIndentData<'ast>,
        span: Span,
    ) -> TextIndent<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextDecoration<'ast>(pub(crate) NodeId<'ast, TextDecorationData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextDecorationData<'ast> {
    pub color: CssColor<'ast>,
    pub line: TextDecorationLine<'ast>,
    pub style: TextDecorationStyle,
    pub thickness: TextDecorationThickness<'ast>,
}
impl<'ast> NodeStorage<'ast> for TextDecoration<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_DECORATION;
    type Data = TextDecorationData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextDecorationData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextDecorationData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TextDecoration<'ast> {
    #[inline]
    pub fn color(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).color
    }
    #[inline]
    pub fn set_color(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.color = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn line(self, ast: &Ast<'ast>) -> TextDecorationLine<'ast> {
        ast.data(self).line
    }
    #[inline]
    pub fn set_line(self, ast: &mut Ast<'ast>, value: TextDecorationLine<'ast>) {
        let mut data = ast.data(self);
        data.line = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn style(self, ast: &Ast<'ast>) -> TextDecorationStyle {
        ast.data(self).style
    }
    #[inline]
    pub fn set_style(self, ast: &mut Ast<'ast>, value: TextDecorationStyle) {
        let mut data = ast.data(self);
        data.style = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn thickness(self, ast: &Ast<'ast>) -> TextDecorationThickness<'ast> {
        ast.data(self).thickness
    }
    #[inline]
    pub fn set_thickness(self, ast: &mut Ast<'ast>, value: TextDecorationThickness<'ast>) {
        let mut data = ast.data(self);
        data.thickness = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for TextDecoration<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextDecoration<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_decoration(
        &mut self,
        data: TextDecorationData<'ast>,
        span: Span,
    ) -> TextDecoration<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextEmphasis<'ast>(pub(crate) NodeId<'ast, TextEmphasisData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextEmphasisData<'ast> {
    pub color: CssColor<'ast>,
    pub style: TextEmphasisStyle<'ast>,
}
impl<'ast> NodeStorage<'ast> for TextEmphasis<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_EMPHASIS;
    type Data = TextEmphasisData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextEmphasisData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextEmphasisData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TextEmphasis<'ast> {
    #[inline]
    pub fn color(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).color
    }
    #[inline]
    pub fn set_color(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.color = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn style(self, ast: &Ast<'ast>) -> TextEmphasisStyle<'ast> {
        ast.data(self).style
    }
    #[inline]
    pub fn set_style(self, ast: &mut Ast<'ast>, value: TextEmphasisStyle<'ast>) {
        let mut data = ast.data(self);
        data.style = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for TextEmphasis<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextEmphasis<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_emphasis(
        &mut self,
        data: TextEmphasisData<'ast>,
        span: Span,
    ) -> TextEmphasis<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextEmphasisPosition<'ast>(pub(crate) NodeId<'ast, TextEmphasisPositionData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextEmphasisPositionData {
    pub horizontal: TextEmphasisPositionHorizontal,
    pub vertical: TextEmphasisPositionVertical,
}
impl<'ast> NodeStorage<'ast> for TextEmphasisPosition<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_EMPHASIS_POSITION;
    type Data = TextEmphasisPositionData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextEmphasisPositionData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextEmphasisPositionData>) -> Self {
        Self(id)
    }
}
impl<'ast> TextEmphasisPosition<'ast> {
    #[inline]
    pub fn horizontal(self, ast: &Ast<'ast>) -> TextEmphasisPositionHorizontal {
        ast.data(self).horizontal
    }
    #[inline]
    pub fn set_horizontal(self, ast: &mut Ast<'ast>, value: TextEmphasisPositionHorizontal) {
        let mut data = ast.data(self);
        data.horizontal = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn vertical(self, ast: &Ast<'ast>) -> TextEmphasisPositionVertical {
        ast.data(self).vertical
    }
    #[inline]
    pub fn set_vertical(self, ast: &mut Ast<'ast>, value: TextEmphasisPositionVertical) {
        let mut data = ast.data(self);
        data.vertical = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for TextEmphasisPosition<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextEmphasisPosition<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_emphasis_position(
        &mut self,
        data: TextEmphasisPositionData,
        span: Span,
    ) -> TextEmphasisPosition<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextShadow<'ast>(pub(crate) NodeId<'ast, TextShadowData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextShadowData<'ast> {
    pub blur: Length<'ast>,
    pub color: CssColor<'ast>,
    pub spread: Length<'ast>,
    pub x_offset: Length<'ast>,
    pub y_offset: Length<'ast>,
}
impl<'ast> NodeStorage<'ast> for TextShadow<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_SHADOW;
    type Data = TextShadowData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextShadowData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextShadowData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TextShadow<'ast> {
    #[inline]
    pub fn blur(self, ast: &Ast<'ast>) -> Length<'ast> {
        ast.data(self).blur
    }
    #[inline]
    pub fn set_blur(self, ast: &mut Ast<'ast>, value: Length<'ast>) {
        let mut data = ast.data(self);
        data.blur = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn color(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).color
    }
    #[inline]
    pub fn set_color(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.color = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn spread(self, ast: &Ast<'ast>) -> Length<'ast> {
        ast.data(self).spread
    }
    #[inline]
    pub fn set_spread(self, ast: &mut Ast<'ast>, value: Length<'ast>) {
        let mut data = ast.data(self);
        data.spread = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn x_offset(self, ast: &Ast<'ast>) -> Length<'ast> {
        ast.data(self).x_offset
    }
    #[inline]
    pub fn set_x_offset(self, ast: &mut Ast<'ast>, value: Length<'ast>) {
        let mut data = ast.data(self);
        data.x_offset = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y_offset(self, ast: &Ast<'ast>) -> Length<'ast> {
        ast.data(self).y_offset
    }
    #[inline]
    pub fn set_y_offset(self, ast: &mut Ast<'ast>, value: Length<'ast>) {
        let mut data = ast.data(self);
        data.y_offset = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for TextShadow<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextShadow<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_shadow(
        &mut self,
        data: TextShadowData<'ast>,
        span: Span,
    ) -> TextShadow<'ast> {
        self.alloc(data, span)
    }
}
