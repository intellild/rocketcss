use crate::*;
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CSSWideKeyword {
    Initial,
    Inherit,
    Unset,
    Revert,
    RevertLayer,
}
use crate::*;
/// A typed property value or a CSS-wide keyword.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CSSWideOr<T> {
    Value(T),
    CSSWide(CSSWideKeyword),
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CustomPropertyName<'ast> {
    Custom(&'ast str),
    Unknown(&'ast str),
}
