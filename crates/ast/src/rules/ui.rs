use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Cursor<'a> {
    pub images: Vec<'a, CursorImage<'a>>,
    pub keyword: CursorKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CursorImage<'a> {
    pub hotspot: Option<(f32, f32)>,
    pub url: NodeId<'a, Url<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Caret<'a> {
    pub color: NodeId<'a, ColorOrAuto<'a>>,
    pub shape: CaretShape,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ListStyle<'a> {
    pub image: NodeId<'a, Image<'a>>,
    pub list_style_type: NodeId<'a, ListStyleType<'a>>,
    pub position: ListStylePosition,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Composes<'a> {
    pub from: Option<NodeId<'a, Specifier<'a>>>,
    pub names: Vec<'a, &'a str>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ColorScheme {
    pub dark: bool,
    pub light: bool,
    pub only: bool,
}
