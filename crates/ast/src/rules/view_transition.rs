use crate::*;

#[derive(Debug, PartialEq)]
pub enum ViewTransitionProperty<'a> {
    Navigation(Navigation),
    Types(Box<'a, NoneOrCustomIdentList<'a>>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum Navigation {
    None,
    Auto,
}

#[derive(Debug, PartialEq)]
pub struct ViewTransitionPartSelector<'a> {
    pub classes: Vec<'a, &'a str>,
    pub name: Option<Box<'a, ViewTransitionPartName<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct ViewTransitionRule<'a> {
    pub span: Span,
    pub properties: Vec<'a, ViewTransitionProperty<'a>>,
}
