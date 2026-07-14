use crate::*;

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ContainerType {
    Normal,
    InlineSize,
    Size,
    ScrollState,
}

#[derive(Debug, PartialEq)]
pub enum ContainerNameList<'a> {
    None,
    Names(Vec<'a, &'a str>),
}
