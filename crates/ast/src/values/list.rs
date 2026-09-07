use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ListStyleType<'a> {
    None,
    String(AstStr<'a>),
    CounterStyle(NodeId<'a, CounterStyle<'a>>),
}

// SAFETY: this kind writes and reads the same native ListStyleType value.
unsafe impl<'ast> AstNodeStorage<'ast> for ListStyleType<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0020_0001);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::String(a), Self::String(b)) => context.str(*a) == context.str(*b),
            _ => self == other,
        }
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ListStyleType<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::CounterStyle(value) => Self::CounterStyle(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum CounterStyle<'a> {
    Predefined(PredefinedCounterStyle),
    Name(AstStr<'a>),
    Symbols {
        symbols: Vec<'a, NodeId<'a, Symbol<'a>>>,
        system: SymbolsType,
    },
}

// SAFETY: this kind writes and reads the same native CounterStyle value.
unsafe impl<'ast> AstNodeStorage<'ast> for CounterStyle<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0020_0002);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Name(a), Self::Name(b)) => context.str(*a) == context.str(*b),
            _ => self == other,
        }
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for CounterStyle<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Symbols { symbols, system } => Self::Symbols {
                symbols: context.clone_encoded_vec(symbols),
                system,
            },
            value => value,
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Visit)]
pub enum SymbolsType {
    Cyclic,
    Numeric,
    Alphabetic,
    #[default]
    Symbolic,
    Fixed,
}

#[repr(u8)]
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Visit)]
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

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Symbol<'a> {
    String(AstStr<'a>),
    Image(NodeId<'a, Image<'a>>),
}

// SAFETY: this kind writes and reads the same native Symbol value.
unsafe impl<'ast> AstNodeStorage<'ast> for Symbol<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0020_0003);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::String(a), Self::String(b)) => context.str(*a) == context.str(*b),
            _ => self == other,
        }
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Symbol<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::String(value) => Self::String(value),
            Self::Image(value) => Self::Image(context.clone_encoded_node(value)),
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MarkerSide {
    MatchSelf,
    MatchParent,
}
