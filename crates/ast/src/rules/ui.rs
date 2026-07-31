use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Cursor<'a> {
    pub images: std::vec::Vec<CursorImage<'a>>,
    pub keyword: CursorKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CursorImage<'a> {
    pub hotspot: Option<(f32, f32)>,
    pub url: std::boxed::Box<Url<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Caret<'a> {
    pub color: std::boxed::Box<ColorOrAuto<'a>>,
    pub shape: CaretShape,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ListStyle<'a> {
    pub image: std::boxed::Box<Image<'a>>,
    pub list_style_type: std::boxed::Box<ListStyleType<'a>>,
    pub position: ListStylePosition,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Composes<'a> {
    pub from: Option<std::boxed::Box<Specifier<'a>>>,
    pub span: Span,
    pub names: std::vec::Vec<&'a str>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ColorScheme {
    pub dark: bool,
    pub light: bool,
    pub only: bool,
}
