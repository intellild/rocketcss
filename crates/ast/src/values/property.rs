use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum CSSWideKeyword {
    Initial,
    Inherit,
    Unset,
    Revert,
    RevertLayer,
}

/// A typed property value or a CSS-wide keyword.
#[derive(Debug, PartialEq, Visit)]
pub enum CSSWideOr<T> {
    Value(T),
    CSSWide(CSSWideKeyword),
}

#[derive(Debug, PartialEq, Visit)]
pub enum CustomPropertyName<'a> {
    Custom(&'a str),
    Unknown(&'a str),
}
