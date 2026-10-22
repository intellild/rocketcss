#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerType {
    Normal,
    InlineSize,
    Size,
    ScrollState,
}

#[ast]
pub enum ContainerNameList<'a> {
    None,
    Names(Vec<'a, &'a str>),
}
