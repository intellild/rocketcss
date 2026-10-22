#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum PageMarginBox {
    TopLeftCorner,
    TopLeft,
    TopCenter,
    TopRight,
    TopRightCorner,
    LeftTop,
    LeftMiddle,
    LeftBottom,
    RightTop,
    RightMiddle,
    RightBottom,
    BottomLeftCorner,
    BottomLeft,
    BottomCenter,
    BottomRight,
    BottomRightCorner,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum PagePseudoClass {
    Left,
    Right,
    First,
    Last,
    Blank,
}

#[ast]
pub struct PageSelector<'a> {
    pub name: Option<&'a str>,
    pub pseudo_classes: Vec<'a, PagePseudoClass>,
}
