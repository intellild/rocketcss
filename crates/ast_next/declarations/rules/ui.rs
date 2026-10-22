#[ast]
pub struct Cursor<'a> {
    pub images: Vec<'a, CursorImage<'a>>,
    pub keyword: CursorKeyword,
}

#[ast]
pub struct CursorImage<'a> {
    pub hotspot: Option<(f32, f32)>,
    pub url: Box<'a, Url<'a>>,
}

#[ast]
pub struct Caret<'a> {
    pub color: Box<'a, ColorOrAuto<'a>>,
    pub shape: CaretShape,
}

#[ast]
pub struct ListStyle<'a> {
    pub image: Box<'a, Image<'a>>,
    pub list_style_type: Box<'a, ListStyleType<'a>>,
    pub position: ListStylePosition,
}

#[ast]
pub struct Composes<'a> {
    pub from: Option<Box<'a, Specifier<'a>>>,
    pub names: Vec<'a, &'a str>,
}

#[ast]
pub struct ColorScheme {
    pub dark: bool,
    pub light: bool,
    pub only: bool,
}
