use crate::*;

#[derive(Debug, PartialEq)]
pub enum ListStyleType<'a> {
    None,
    String(&'a str),
    CounterStyle(Box<'a, CounterStyle<'a>>),
}

#[derive(Debug, PartialEq)]
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

#[derive(CssKeyword, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Symbol<'a> {
    String(&'a str),
    Image(Box<'a, Image<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum MarkerSide {
    MatchSelf,
    MatchParent,
}
