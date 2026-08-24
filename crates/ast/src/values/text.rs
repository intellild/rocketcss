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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub struct TextDecorationLine(u8);

impl TextDecorationLine {
    pub const UNDERLINE: Self = Self(1 << 0);
    pub const OVERLINE: Self = Self(1 << 1);
    pub const LINE_THROUGH: Self = Self(1 << 2);
    pub const BLINK: Self = Self(1 << 3);
    pub const SPELLING_ERROR: Self = Self(1 << 4);
    pub const GRAMMAR_ERROR: Self = Self(1 << 5);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }
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

#[derive(Debug, PartialEq, Visit)]
pub struct Content<'a> {
    pub value: Vec<'a, TokenOrValue<'a>>,
}
