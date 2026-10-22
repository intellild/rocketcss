#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxOrient {
    Horizontal,
    Vertical,
    InlineAxis,
    BlockAxis,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxDirection {
    Normal,
    Reverse,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxAlign {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxPack {
    Start,
    End,
    Center,
    Justify,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxLines {
    Single,
    Multiple,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexPack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexItemAlign {
    Auto,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexLinePack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
    Stretch,
}
