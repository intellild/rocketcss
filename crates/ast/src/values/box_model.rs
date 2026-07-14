use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum Display<'a> {
    Keyword(DisplayKeyword),
    Pair {
        inside: Box<'a, DisplayInside>,
        is_list_item: bool,
        outside: DisplayOutside,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum DisplayKeyword {
    None,
    Contents,
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
}

#[derive(Debug, PartialEq, Visit)]
pub enum DisplayInside {
    Flow,
    FlowRoot,
    Table,
    Flex { vendor_prefix: VendorPrefix },
    Box { vendor_prefix: VendorPrefix },
    Grid,
    Ruby,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum DisplayOutside {
    Block,
    Inline,
    RunIn,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Size<'a> {
    Auto,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(Box<'a, LengthPercentage<'a>>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}

#[derive(Debug, PartialEq, Visit)]
pub enum MaxSize<'a> {
    None,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(Box<'a, LengthPercentage<'a>>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum OverflowKeyword {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Debug, PartialEq, Visit)]
pub enum PositionProperty {
    Static,
    Relative,
    Absolute,
    Sticky(VendorPrefix),
    Fixed,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Size2D<'a, T>(pub Box<'a, T>, pub Box<'a, T>);

#[derive(Debug, PartialEq, Visit)]
pub struct Rect<'a, T>(
    pub Box<'a, T>,
    pub Box<'a, T>,
    pub Box<'a, T>,
    pub Box<'a, T>,
);

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ZIndex {
    Auto,
    Integer(i32),
}
