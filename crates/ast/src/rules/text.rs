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
    pub value: NodeId<'a, LengthPercentage<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextDecoration<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub line: NodeId<'a, TextDecorationLine<'a>>,
    pub style: TextDecorationStyle,
    pub thickness: NodeId<'a, TextDecorationThickness<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasis<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub style: NodeId<'a, TextEmphasisStyle<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasisPosition {
    pub horizontal: TextEmphasisPositionHorizontal,
    pub vertical: TextEmphasisPositionVertical,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextShadow<'a> {
    pub blur: NodeId<'a, Length<'a>>,
    pub color: NodeId<'a, CssColor<'a>>,
    pub spread: NodeId<'a, Length<'a>>,
    pub x_offset: NodeId<'a, Length<'a>>,
    pub y_offset: NodeId<'a, Length<'a>>,
}
