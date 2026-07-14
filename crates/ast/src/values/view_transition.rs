use crate::*;

#[derive(Debug, PartialEq)]
pub enum ViewTransitionName<'a> {
    None,
    Auto,
    Custom(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum NoneOrCustomIdentList<'a> {
    None,
    Idents(Vec<'a, &'a str>),
}

#[derive(Debug, PartialEq)]
pub enum ViewTransitionGroup<'a> {
    Normal,
    Contain,
    Nearest,
    Custom(&'a str),
}
