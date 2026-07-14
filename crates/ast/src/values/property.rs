use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum CSSWideKeyword {
    Initial,
    Inherit,
    Unset,
    Revert,
    RevertLayer,
}

#[derive(Debug, PartialEq, Visit)]
pub enum CustomPropertyName<'a> {
    Custom(&'a str),
    Unknown(&'a str),
}
