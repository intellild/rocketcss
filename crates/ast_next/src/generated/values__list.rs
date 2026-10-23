use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ListStyleType<'ast>(pub(crate) NodeId<'ast, ListStyleTypeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ListStyleTypeData<'ast> {
    None,
    String(&'ast str),
    CounterStyle(CounterStyle<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ListStyleTypeKind {
    None,
    String,
    CounterStyle,
}
impl<'ast> NodeStorage<'ast> for ListStyleType<'ast> {
    const KIND: NodeKind = super::kind::KIND_LIST_STYLE_TYPE;
    type Data = ListStyleTypeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ListStyleTypeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ListStyleTypeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ListStyleType<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ListStyleTypeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ListStyleTypeKind {
        match ast.data(self) {
            ListStyleTypeData::None => ListStyleTypeKind::None,
            ListStyleTypeData::String { .. } => ListStyleTypeKind::String,
            ListStyleTypeData::CounterStyle { .. } => ListStyleTypeKind::CounterStyle,
        }
    }
}
impl<'ast> GetSpan<'ast> for ListStyleType<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ListStyleType<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_list_style_type(
        &mut self,
        data: ListStyleTypeData<'ast>,
        span: Span,
    ) -> ListStyleType<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CounterStyle<'ast>(pub(crate) NodeId<'ast, CounterStyleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CounterStyleData<'ast> {
    Predefined(PredefinedCounterStyle),
    Name(&'ast str),
    Symbols {
        symbols: &'ast [Symbol<'ast>],
        system: SymbolsType,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CounterStyleKind {
    Predefined,
    Name,
    Symbols,
}
impl<'ast> NodeStorage<'ast> for CounterStyle<'ast> {
    const KIND: NodeKind = super::kind::KIND_COUNTER_STYLE;
    type Data = CounterStyleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CounterStyleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CounterStyleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CounterStyle<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> CounterStyleData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> CounterStyleKind {
        match ast.data(self) {
            CounterStyleData::Predefined { .. } => CounterStyleKind::Predefined,
            CounterStyleData::Name { .. } => CounterStyleKind::Name,
            CounterStyleData::Symbols { .. } => CounterStyleKind::Symbols,
        }
    }
}
impl<'ast> GetSpan<'ast> for CounterStyle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CounterStyle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_counter_style(
        &mut self,
        data: CounterStyleData<'ast>,
        span: Span,
    ) -> CounterStyle<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SymbolsType {
    Cyclic,
    Numeric,
    Alphabetic,
    #[default]
    Symbolic,
    Fixed,
}
use crate::*;
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PredefinedCounterStyle {
    Decimal,
    DecimalLeadingZero,
    ArabicIndic,
    Armenian,
    UpperArmenian,
    LowerArmenian,
    Bengali,
    Cambodian,
    Khmer,
    CjkDecimal,
    Devanagari,
    Georgian,
    Gujarati,
    Gurmukhi,
    Hebrew,
    Kannada,
    Lao,
    Malayalam,
    Mongolian,
    Myanmar,
    Oriya,
    Persian,
    LowerRoman,
    UpperRoman,
    Tamil,
    Telugu,
    Thai,
    Tibetan,
    LowerAlpha,
    LowerLatin,
    UpperAlpha,
    UpperLatin,
    LowerGreek,
    Hiragana,
    HiraganaIroha,
    Katakana,
    KatakanaIroha,
    Disc,
    Circle,
    Square,
    DisclosureOpen,
    DisclosureClosed,
    CjkEarthlyBranch,
    CjkHeavenlyStem,
    JapaneseInformal,
    JapaneseFormal,
    KoreanHangulFormal,
    KoreanHanjaInformal,
    KoreanHanjaFormal,
    SimpChineseInformal,
    SimpChineseFormal,
    TradChineseInformal,
    TradChineseFormal,
    EthiopicNumeric,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol<'ast>(pub(crate) NodeId<'ast, SymbolData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolData<'ast> {
    String(&'ast str),
    Image(Image<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    String,
    Image,
}
impl<'ast> NodeStorage<'ast> for Symbol<'ast> {
    const KIND: NodeKind = super::kind::KIND_SYMBOL;
    type Data = SymbolData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SymbolData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SymbolData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Symbol<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SymbolData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SymbolKind {
        match ast.data(self) {
            SymbolData::String { .. } => SymbolKind::String,
            SymbolData::Image { .. } => SymbolKind::Image,
        }
    }
}
impl<'ast> GetSpan<'ast> for Symbol<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Symbol<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_symbol(&mut self, data: SymbolData<'ast>, span: Span) -> Symbol<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ListStylePosition {
    Inside,
    Outside,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MarkerSide {
    MatchSelf,
    MatchParent,
}
