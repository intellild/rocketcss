use crate::*;

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxOrient {
    Horizontal,
    Vertical,
    InlineAxis,
    BlockAxis,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxDirection {
    Normal,
    Reverse,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxAlign {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxPack {
    Start,
    End,
    Center,
    Justify,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum BoxLines {
    Single,
    Multiple,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexPack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexItemAlign {
    Auto,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FlexLinePack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
    Stretch,
}
