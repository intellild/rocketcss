use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionProperty<'a> {
    Navigation(Navigation),
    Types(std::boxed::Box<NoneOrCustomIdentList<'a>>),
    Custom(std::boxed::Box<CustomProperty<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Navigation {
    None,
    Auto,
}

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub struct ViewTransitionPartSelector<'a> {
    pub classes: std::vec::Vec<&'a str>,
    pub name: Option<std::boxed::Box<ViewTransitionPartName<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ViewTransitionRule<'a> {
    pub span: Span,
    pub properties: std::vec::Vec<ViewTransitionProperty<'a>>,
}
