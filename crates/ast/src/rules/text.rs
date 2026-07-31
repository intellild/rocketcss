use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct TextTransform {
    pub case: TextTransformCase,
    pub full_size_kana: bool,
    pub full_width: bool,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextIndent {
    pub each_line: bool,
    pub hanging: bool,
    pub value: std::boxed::Box<LengthPercentage>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextDecoration<'a> {
    pub color: std::boxed::Box<CssColor<'a>>,
    pub line: std::boxed::Box<TextDecorationLine>,
    pub style: TextDecorationStyle,
    pub thickness: std::boxed::Box<TextDecorationThickness>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasis<'a> {
    pub color: std::boxed::Box<CssColor<'a>>,
    pub style: std::boxed::Box<TextEmphasisStyle<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasisPosition {
    pub horizontal: TextEmphasisPositionHorizontal,
    pub vertical: TextEmphasisPositionVertical,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextShadow<'a> {
    pub blur: std::boxed::Box<Length>,
    pub color: std::boxed::Box<CssColor<'a>>,
    pub spread: std::boxed::Box<Length>,
    pub x_offset: std::boxed::Box<Length>,
    pub y_offset: std::boxed::Box<Length>,
}
