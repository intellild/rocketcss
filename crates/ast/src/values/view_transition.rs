use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionName<'a> {
    None,
    Auto,
    Custom(&'a str),
}

#[derive(Debug, PartialEq, Visit)]
pub enum NoneOrCustomIdentList<'a> {
    None,
    Idents(Vec<'a, &'a str>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionGroup<'a> {
    Normal,
    Contain,
    Nearest,
    Custom(&'a str),
}
