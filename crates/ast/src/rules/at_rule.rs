use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct CharsetRule<'a> {
    pub encoding: AstStr<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct NamespaceRule<'a> {
    pub prefix: Option<AstStr<'a>>,
    pub url: AstStr<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CustomMediaRule<'a> {
    pub name: AstStr<'a>,
    pub query: MediaList<'a>,
}
