use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontWeight {
    Absolute(AbsoluteFontWeight),
    Bolder,
    Lighter,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontSize<'ast>(pub(crate) NodeId<'ast, FontSizeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontSizeData<'ast> {
    Length(LengthPercentage<'ast>),
    Absolute(AbsoluteFontSize),
    Relative(RelativeFontSize),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontSizeKind {
    Length,
    Absolute,
    Relative,
}
impl<'ast> NodeStorage<'ast> for FontSize<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_SIZE;
    type Data = FontSizeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontSizeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontSizeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FontSize<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> FontSizeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> FontSizeKind {
        match ast.data(self) {
            FontSizeData::Length { .. } => FontSizeKind::Length,
            FontSizeData::Absolute { .. } => FontSizeKind::Absolute,
            FontSizeData::Relative { .. } => FontSizeKind::Relative,
        }
    }
}
impl<'ast> GetSpan<'ast> for FontSize<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontSize<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_size(&mut self, data: FontSizeData<'ast>, span: Span) -> FontSize<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AbsoluteFontSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
    XxxLarge,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum RelativeFontSize {
    Smaller,
    Larger,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontStretch {
    Keyword(FontStretchKeyword),
    Percentage(f32),
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FontStretchKeyword {
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontFamily<'ast>(pub(crate) NodeId<'ast, FontFamilyData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontFamilyData<'ast> {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    SystemUi,
    Emoji,
    Math,
    Fangsong,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
    Initial,
    Inherit,
    Unset,
    Default,
    Revert,
    RevertLayer,
    Unparsed(&'ast [TokenOrValue<'ast>]),
    Tombstone,
    Custom(&'ast str),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontFamilyKind {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    SystemUi,
    Emoji,
    Math,
    Fangsong,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
    Initial,
    Inherit,
    Unset,
    Default,
    Revert,
    RevertLayer,
    Unparsed,
    Tombstone,
    Custom,
}
impl<'ast> NodeStorage<'ast> for FontFamily<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_FAMILY;
    type Data = FontFamilyData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontFamilyData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontFamilyData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FontFamily<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> FontFamilyData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> FontFamilyKind {
        match ast.data(self) {
            FontFamilyData::Serif { .. } => FontFamilyKind::Serif,
            FontFamilyData::SansSerif { .. } => FontFamilyKind::SansSerif,
            FontFamilyData::Cursive { .. } => FontFamilyKind::Cursive,
            FontFamilyData::Fantasy { .. } => FontFamilyKind::Fantasy,
            FontFamilyData::Monospace { .. } => FontFamilyKind::Monospace,
            FontFamilyData::SystemUi { .. } => FontFamilyKind::SystemUi,
            FontFamilyData::Emoji { .. } => FontFamilyKind::Emoji,
            FontFamilyData::Math { .. } => FontFamilyKind::Math,
            FontFamilyData::Fangsong { .. } => FontFamilyKind::Fangsong,
            FontFamilyData::UiSerif { .. } => FontFamilyKind::UiSerif,
            FontFamilyData::UiSansSerif { .. } => FontFamilyKind::UiSansSerif,
            FontFamilyData::UiMonospace { .. } => FontFamilyKind::UiMonospace,
            FontFamilyData::UiRounded { .. } => FontFamilyKind::UiRounded,
            FontFamilyData::Initial { .. } => FontFamilyKind::Initial,
            FontFamilyData::Inherit { .. } => FontFamilyKind::Inherit,
            FontFamilyData::Unset { .. } => FontFamilyKind::Unset,
            FontFamilyData::Default { .. } => FontFamilyKind::Default,
            FontFamilyData::Revert { .. } => FontFamilyKind::Revert,
            FontFamilyData::RevertLayer { .. } => FontFamilyKind::RevertLayer,
            FontFamilyData::Unparsed { .. } => FontFamilyKind::Unparsed,
            FontFamilyData::Tombstone { .. } => FontFamilyKind::Tombstone,
            FontFamilyData::Custom { .. } => FontFamilyKind::Custom,
        }
    }
}
impl<'ast> GetSpan<'ast> for FontFamily<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontFamily<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_family(
        &mut self,
        data: FontFamilyData<'ast>,
        span: Span,
    ) -> FontFamily<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique(Angle),
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LineHeight<'ast>(pub(crate) NodeId<'ast, LineHeightData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineHeightData<'ast> {
    Normal,
    Number(f32),
    Length(LengthPercentage<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LineHeightKind {
    Normal,
    Number,
    Length,
}
impl<'ast> NodeStorage<'ast> for LineHeight<'ast> {
    const KIND: NodeKind = super::kind::KIND_LINE_HEIGHT;
    type Data = LineHeightData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, LineHeightData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, LineHeightData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> LineHeight<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> LineHeightData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> LineHeightKind {
        match ast.data(self) {
            LineHeightData::Normal { .. } => LineHeightKind::Normal,
            LineHeightData::Number { .. } => LineHeightKind::Number,
            LineHeightData::Length { .. } => LineHeightKind::Length,
        }
    }
}
impl<'ast> GetSpan<'ast> for LineHeight<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for LineHeight<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_line_height(
        &mut self,
        data: LineHeightData<'ast>,
        span: Span,
    ) -> LineHeight<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VerticalAlign<'ast>(pub(crate) NodeId<'ast, VerticalAlignData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VerticalAlignData<'ast> {
    Keyword(VerticalAlignKeyword),
    Length(LengthPercentage<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VerticalAlignKind {
    Keyword,
    Length,
}
impl<'ast> NodeStorage<'ast> for VerticalAlign<'ast> {
    const KIND: NodeKind = super::kind::KIND_VERTICAL_ALIGN;
    type Data = VerticalAlignData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, VerticalAlignData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, VerticalAlignData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> VerticalAlign<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> VerticalAlignData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> VerticalAlignKind {
        match ast.data(self) {
            VerticalAlignData::Keyword { .. } => VerticalAlignKind::Keyword,
            VerticalAlignData::Length { .. } => VerticalAlignKind::Length,
        }
    }
}
impl<'ast> GetSpan<'ast> for VerticalAlign<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for VerticalAlign<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_vertical_align(
        &mut self,
        data: VerticalAlignData<'ast>,
        span: Span,
    ) -> VerticalAlign<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum VerticalAlignKeyword {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
}
