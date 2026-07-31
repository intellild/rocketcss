use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(Debug, PartialEq, Visit)]
pub enum BorderSideWidth {
    Thin,
    Medium,
    Thick,
    Length(std::boxed::Box<Length>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum LengthOrNumber {
    Number(f32),
    Length(std::boxed::Box<Length>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[derive(Debug, PartialEq, Visit)]
pub enum BorderImageSideWidth {
    Number(f32),
    LengthPercentage(std::boxed::Box<LengthPercentage>),
    Auto,
}

#[derive(Debug, PartialEq, Visit)]
pub enum OutlineStyle {
    Auto,
    LineStyle(LineStyle),
}
