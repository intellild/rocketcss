use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct CharsetRule<'a> {
    pub encoding: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct NamespaceRule<'a> {
    pub prefix: Option<&'a str>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CustomMediaRule<'a> {
    pub name: &'a str,
    pub query: MediaList<'a>,
}
