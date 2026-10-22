#[ast]
pub struct CharsetRule<'a> {
    pub encoding: &'a str,
}

#[ast]
pub struct NamespaceRule<'a> {
    pub prefix: Option<&'a str>,
    pub url: &'a str,
}

#[ast]
pub struct CustomMediaRule<'a> {
    pub name: &'a str,
    pub query: MediaList<'a>,
}
