use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextTransformCase {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum WhiteSpace {
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    BreakSpaces,
    PreLine,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum WordBreak {
    Normal,
    KeepAll,
    BreakAll,
    BreakWord,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum LineBreak {
    Auto,
    Loose,
    Normal,
    Strict,
    Anywhere,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Hyphens {
    None,
    Manual,
    Auto,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum OverflowWrap {
    Normal,
    Anywhere,
    BreakWord,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextJustify {
    Auto,
    None,
    InterWord,
    InterCharacter,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Spacing<'a> {
    Normal,
    Length(Box<'a, Length<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum TextDecorationLine<'a> {
    ExclusiveTextDecorationLine(ExclusiveTextDecorationLine),
    Value(Vec<'a, OtherTextDecorationLine>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ExclusiveTextDecorationLine {
    None,
    SpellingError,
    GrammarError,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum OtherTextDecorationLine {
    Underline,
    Overline,
    LineThrough,
    Blink,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Debug, PartialEq, Visit)]
pub enum TextDecorationThickness<'a> {
    Auto,
    FromFont,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextDecorationSkipInk {
    Auto,
    None,
    All,
}

#[derive(Debug, PartialEq, Visit)]
pub enum TextEmphasisStyle<'a> {
    None,
    Keyword {
        fill: TextEmphasisFillMode,
        shape: Option<TextEmphasisShape>,
    },
    String(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextEmphasisFillMode {
    Filled,
    Open,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextEmphasisShape {
    Dot,
    Circle,
    DoubleCircle,
    Triangle,
    Sesame,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextEmphasisPositionHorizontal {
    Left,
    Right,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextEmphasisPositionVertical {
    Over,
    Under,
}

#[derive(Debug, PartialEq, Visit)]
pub enum TextSizeAdjust {
    Auto,
    None,
    Percentage(f32),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}
