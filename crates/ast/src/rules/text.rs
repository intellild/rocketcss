use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct TextTransform {
    pub case: TextTransformCase,
    pub full_size_kana: bool,
    pub full_width: bool,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextIndent<'a> {
    pub each_line: bool,
    pub hanging: bool,
    pub value: Box<'a, LengthPercentage<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextDecoration<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub line: Box<'a, TextDecorationLine<'a>>,
    pub style: TextDecorationStyle,
    pub thickness: Box<'a, TextDecorationThickness<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasis<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub style: Box<'a, TextEmphasisStyle<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasisPosition {
    pub horizontal: TextEmphasisPositionHorizontal,
    pub vertical: TextEmphasisPositionVertical,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextShadow<'a> {
    pub blur: Box<'a, Length<'a>>,
    pub color: Box<'a, CssColor<'a>>,
    pub spread: Box<'a, Length<'a>>,
    pub x_offset: Box<'a, Length<'a>>,
    pub y_offset: Box<'a, Length<'a>>,
}
