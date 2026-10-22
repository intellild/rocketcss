#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LineStyle {
    None,
    Hidden,
    Inset,
    Groove,
    Outset,
    Ridge,
    Dotted,
    Dashed,
    Solid,
    Double,
}

#[ast]
pub enum BorderSideWidth<'a> {
    Thin,
    Medium,
    Thick,
    Length(Box<'a, Length<'a>>),
}

#[ast]
pub enum LengthOrNumber<'a> {
    Number(f32),
    Length(Box<'a, Length<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[ast]
pub enum BorderImageSideWidth<'a> {
    Number(f32),
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    Auto,
}

#[ast]
pub enum OutlineStyle {
    Auto,
    LineStyle(LineStyle),
}
