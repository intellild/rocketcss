use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ContainerType {
    Normal,
    InlineSize,
    Size,
    ScrollState,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ContainerNameList<'a> {
    None,
    Names(std::vec::Vec<&'a str>),
}
