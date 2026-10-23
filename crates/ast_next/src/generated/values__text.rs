use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextTransformCase {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum WhiteSpace {
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    BreakSpaces,
    PreLine,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum WordBreak {
    Normal,
    KeepAll,
    BreakAll,
    BreakWord,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum LineBreak {
    Auto,
    Loose,
    Normal,
    Strict,
    Anywhere,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Hyphens {
    None,
    Manual,
    Auto,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum OverflowWrap {
    Normal,
    Anywhere,
    BreakWord,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    MatchParent,
    JustifyAll,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextAlignLast {
    Auto,
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    MatchParent,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextJustify {
    Auto,
    None,
    InterWord,
    InterCharacter,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Spacing<'ast>(pub(crate) NodeId<'ast, SpacingData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpacingData<'ast> {
    Normal,
    Length(Length<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpacingKind {
    Normal,
    Length,
}
impl<'ast> NodeStorage<'ast> for Spacing<'ast> {
    const KIND: NodeKind = super::kind::KIND_SPACING;
    type Data = SpacingData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SpacingData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SpacingData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Spacing<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SpacingData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SpacingKind {
        match ast.data(self) {
            SpacingData::Normal => SpacingKind::Normal,
            SpacingData::Length { .. } => SpacingKind::Length,
        }
    }
}
impl<'ast> GetSpan<'ast> for Spacing<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Spacing<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_spacing(&mut self, data: SpacingData<'ast>, span: Span) -> Spacing<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextDecorationLine<'ast>(pub(crate) NodeId<'ast, TextDecorationLineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextDecorationLineData<'ast> {
    ExclusiveTextDecorationLine(ExclusiveTextDecorationLine),
    Value(&'ast [OtherTextDecorationLine]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextDecorationLineKind {
    ExclusiveTextDecorationLine,
    Value,
}
impl<'ast> NodeStorage<'ast> for TextDecorationLine<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_DECORATION_LINE;
    type Data = TextDecorationLineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextDecorationLineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextDecorationLineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TextDecorationLine<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TextDecorationLineData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TextDecorationLineKind {
        match ast.data(self) {
            TextDecorationLineData::ExclusiveTextDecorationLine { .. } => {
                TextDecorationLineKind::ExclusiveTextDecorationLine
            }
            TextDecorationLineData::Value { .. } => TextDecorationLineKind::Value,
        }
    }
}
impl<'ast> GetSpan<'ast> for TextDecorationLine<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextDecorationLine<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_decoration_line(
        &mut self,
        data: TextDecorationLineData<'ast>,
        span: Span,
    ) -> TextDecorationLine<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ExclusiveTextDecorationLine {
    None,
    SpellingError,
    GrammarError,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum OtherTextDecorationLine {
    Underline,
    Overline,
    LineThrough,
    Blink,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextDecorationThickness<'ast>(
    pub(crate) NodeId<'ast, TextDecorationThicknessData<'ast>>,
);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextDecorationThicknessData<'ast> {
    Auto,
    FromFont,
    LengthPercentage(LengthPercentage<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextDecorationThicknessKind {
    Auto,
    FromFont,
    LengthPercentage,
}
impl<'ast> NodeStorage<'ast> for TextDecorationThickness<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_DECORATION_THICKNESS;
    type Data = TextDecorationThicknessData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextDecorationThicknessData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextDecorationThicknessData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TextDecorationThickness<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TextDecorationThicknessData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TextDecorationThicknessKind {
        match ast.data(self) {
            TextDecorationThicknessData::Auto => TextDecorationThicknessKind::Auto,
            TextDecorationThicknessData::FromFont => TextDecorationThicknessKind::FromFont,
            TextDecorationThicknessData::LengthPercentage { .. } => {
                TextDecorationThicknessKind::LengthPercentage
            }
        }
    }
}
impl<'ast> GetSpan<'ast> for TextDecorationThickness<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextDecorationThickness<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_decoration_thickness(
        &mut self,
        data: TextDecorationThicknessData<'ast>,
        span: Span,
    ) -> TextDecorationThickness<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextDecorationSkipInk {
    Auto,
    None,
    All,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextEmphasisStyle<'ast>(pub(crate) NodeId<'ast, TextEmphasisStyleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextEmphasisStyleData<'ast> {
    None,
    Keyword {
        fill: TextEmphasisFillMode,
        shape: Option<TextEmphasisShape>,
    },
    String(&'ast str),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextEmphasisStyleKind {
    None,
    Keyword,
    String,
}
impl<'ast> NodeStorage<'ast> for TextEmphasisStyle<'ast> {
    const KIND: NodeKind = super::kind::KIND_TEXT_EMPHASIS_STYLE;
    type Data = TextEmphasisStyleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TextEmphasisStyleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TextEmphasisStyleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TextEmphasisStyle<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TextEmphasisStyleData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TextEmphasisStyleKind {
        match ast.data(self) {
            TextEmphasisStyleData::None => TextEmphasisStyleKind::None,
            TextEmphasisStyleData::Keyword { .. } => TextEmphasisStyleKind::Keyword,
            TextEmphasisStyleData::String { .. } => TextEmphasisStyleKind::String,
        }
    }
}
impl<'ast> GetSpan<'ast> for TextEmphasisStyle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TextEmphasisStyle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_text_emphasis_style(
        &mut self,
        data: TextEmphasisStyleData<'ast>,
        span: Span,
    ) -> TextEmphasisStyle<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextEmphasisFillMode {
    Filled,
    Open,
}
use crate::*;
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextEmphasisShape {
    Dot,
    Circle,
    DoubleCircle,
    Triangle,
    Sesame,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextEmphasisPositionHorizontal {
    Left,
    Right,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextEmphasisPositionVertical {
    Over,
    Under,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TextSizeAdjust {
    Auto,
    None,
    Percentage(f32),
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextDirection {
    Ltr,
    Rtl,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Content<'ast>(pub(crate) NodeId<'ast, ContentData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContentData<'ast> {
    pub value: &'ast [TokenOrValue<'ast>],
}
impl<'ast> NodeStorage<'ast> for Content<'ast> {
    const KIND: NodeKind = super::kind::KIND_CONTENT;
    type Data = ContentData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ContentData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ContentData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Content<'ast> {
    #[inline]
    pub fn value(self, ast: &Ast<'ast>) -> &'ast [TokenOrValue<'ast>] {
        ast.data(self).value
    }
    #[inline]
    pub fn set_value(self, ast: &mut Ast<'ast>, value: &'ast [TokenOrValue<'ast>]) {
        let mut data = ast.data(self);
        data.value = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Content<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Content<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_content(&mut self, data: ContentData<'ast>, span: Span) -> Content<'ast> {
        self.alloc(data, span)
    }
}
