#[ast]
pub enum ViewTransitionProperty<'a> {
    Navigation(Navigation),
    Types(Box<'a, NoneOrCustomIdentList<'a>>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Navigation {
    None,
    Auto,
}

#[ast]
pub struct ViewTransitionPartSelector<'a> {
    pub classes: Vec<'a, &'a str>,
    pub name: Option<Box<'a, ViewTransitionPartName<'a>>>,
}
