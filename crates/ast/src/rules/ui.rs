use crate::*;

#[derive(Debug, PartialEq)]
pub struct Cursor<'a> {
    pub images: Vec<'a, CursorImage<'a>>,
    pub keyword: CursorKeyword,
}

#[derive(Debug, PartialEq)]
pub struct CursorImage<'a> {
    pub hotspot: Option<(f32, f32)>,
    pub url: Box<'a, Url<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Caret<'a> {
    pub color: Box<'a, ColorOrAuto<'a>>,
    pub shape: CaretShape,
}

#[derive(Debug, PartialEq)]
pub struct ListStyle<'a> {
    pub image: Box<'a, Image<'a>>,
    pub list_style_type: Box<'a, ListStyleType<'a>>,
    pub position: ListStylePosition,
}

#[derive(Debug, PartialEq)]
pub struct Composes<'a> {
    pub from: Option<Box<'a, Specifier<'a>>>,
    pub span: Span,
    pub names: Vec<'a, &'a str>,
}

#[derive(Debug, PartialEq)]
pub struct ColorScheme {
    pub dark: bool,
    pub light: bool,
    pub only: bool,
}
