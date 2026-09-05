use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionProperty<'a> {
    Navigation(Navigation),
    Types(NodeId<'a, NoneOrCustomIdentList<'a>>),
    Custom(NodeId<'a, CustomProperty<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Navigation {
    None,
    Auto,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Visit)]
pub struct ViewTransitionPartSelector<'a> {
    pub classes: Vec<'a, &'a str>,
    pub name: Option<NodeId<'a, ViewTransitionPartName<'a>>>,
}
