#[ast]
pub enum ListStyleType<'a> {
    None,
    String(&'a str),
    CounterStyle(Box<'a, CounterStyle<'a>>),
}

#[ast]
pub enum CounterStyle<'a> {
    Predefined(PredefinedCounterStyle),
    Name(&'a str),
    Symbols {
        symbols: Vec<'a, Symbol<'a>>,
        system: SymbolsType,
    },
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SymbolsType {
    Cyclic,
    Numeric,
    Alphabetic,
    #[default]
    Symbolic,
    Fixed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[ast]
pub enum Symbol<'a> {
    String(&'a str),
    Image(Box<'a, Image<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MarkerSide {
    MatchSelf,
    MatchParent,
}
