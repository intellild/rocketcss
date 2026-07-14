use crate::*;

#[derive(CssKeyword, Debug, PartialEq)]
pub enum CSSWideKeyword {
    Initial,
    Inherit,
    Unset,
    Revert,
    RevertLayer,
}

#[derive(Debug, PartialEq)]
pub enum CustomPropertyName<'a> {
    Custom(&'a str),
    Unknown(&'a str),
}
