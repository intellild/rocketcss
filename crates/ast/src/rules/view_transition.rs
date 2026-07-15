use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionProperty<'a> {
    Navigation(Navigation),
    Types(Box<'a, NoneOrCustomIdentList<'a>>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Navigation {
    None,
    Auto,
}

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub struct ViewTransitionPartSelector<'a> {
    pub classes: Vec<'a, &'a str>,
    pub name: Option<Box<'a, ViewTransitionPartName<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ViewTransitionRule<'a> {
    pub span: Span,
    pub properties: Vec<'a, ViewTransitionProperty<'a>>,
}
