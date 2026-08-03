use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct CharsetRule<'a> {
    pub span: Span,
    pub encoding: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct NamespaceRule<'a> {
    pub span: Span,
    pub prefix: Option<&'a str>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CustomMediaRule<'a> {
    pub span: Span,
    pub name: &'a str,
    pub query: MediaList<'a>,
}
