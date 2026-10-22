#[ast]
pub struct TextTransform {
    pub case: TextTransformCase,
    pub full_size_kana: bool,
    pub full_width: bool,
}

#[ast]
pub struct TextIndent<'a> {
    pub each_line: bool,
    pub hanging: bool,
    pub value: Box<'a, LengthPercentage<'a>>,
}

#[ast]
pub struct TextDecoration<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub line: Box<'a, TextDecorationLine<'a>>,
    pub style: TextDecorationStyle,
    pub thickness: Box<'a, TextDecorationThickness<'a>>,
}

#[ast]
pub struct TextEmphasis<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub style: Box<'a, TextEmphasisStyle<'a>>,
}

#[ast]
pub struct TextEmphasisPosition {
    pub horizontal: TextEmphasisPositionHorizontal,
    pub vertical: TextEmphasisPositionVertical,
}

#[ast]
pub struct TextShadow<'a> {
    pub blur: Box<'a, Length<'a>>,
    pub color: Box<'a, CssColor<'a>>,
    pub spread: Box<'a, Length<'a>>,
    pub x_offset: Box<'a, Length<'a>>,
    pub y_offset: Box<'a, Length<'a>>,
}
