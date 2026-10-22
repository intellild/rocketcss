#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ViewTransitionName<'a> {
    None,
    Auto,
    Custom(&'a str),
}

#[ast]
pub enum NoneOrCustomIdentList<'a> {
    None,
    Idents(Vec<'a, &'a str>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ViewTransitionGroup<'a> {
    Normal,
    Contain,
    Nearest,
    Custom(&'a str),
}
