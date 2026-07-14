use crate::*;
use std::pin::Pin;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum PagePseudoClass {
    Left,
    Right,
    First,
    Last,
    Blank,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PageRule<'a> {
    pub declarations: Pin<Box<'a, DeclarationBlock<'a>>>,
    pub span: Span,
    pub rules: Vec<'a, PageMarginRule<'a>>,
    pub selectors: Vec<'a, PageSelector<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PageMarginRule<'a> {
    pub declarations: Pin<Box<'a, DeclarationBlock<'a>>>,
    pub span: Span,
    pub margin_box: PageMarginBox,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PageSelector<'a> {
    pub name: Option<&'a str>,
    pub pseudo_classes: Vec<'a, PagePseudoClass>,
}
