use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxOrient {
    Horizontal,
    Vertical,
    InlineAxis,
    BlockAxis,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxDirection {
    Normal,
    Reverse,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxAlign {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxPack {
    Start,
    End,
    Center,
    Justify,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxLines {
    Single,
    Multiple,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexPack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexItemAlign {
    Auto,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FlexLinePack {
    Start,
    End,
    Center,
    Justify,
    Distribute,
    Stretch,
}
