use crate::*;

#[derive(CssKeyword, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum BorderSideWidth<'a> {
    Thin,
    Medium,
    Thick,
    Length(Box<'a, Length<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum LengthOrNumber<'a> {
    Number(f32),
    Length(Box<'a, Length<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[derive(Debug, PartialEq)]
pub enum BorderImageSideWidth<'a> {
    Number(f32),
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    Auto,
}

#[derive(Debug, PartialEq)]
pub enum OutlineStyle {
    Auto,
    LineStyle(LineStyle),
}
