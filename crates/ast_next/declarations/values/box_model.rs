#[ast]
pub enum Display {
    Keyword(DisplayKeyword),
    Pair {
        inside: DisplayInside,
        is_list_item: bool,
        outside: DisplayOutside,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[ast]
pub enum DisplayInside {
    Flow,
    FlowRoot,
    Table,
    Flex { vendor_prefix: VendorPrefix },
    Box { vendor_prefix: VendorPrefix },
    Grid,
    Ruby,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum DisplayOutside {
    Block,
    Inline,
    RunIn,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

#[ast]
pub enum Size<'a> {
    Auto,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    MathFunction(Box<'a, Function<'a>>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(Box<'a, LengthPercentage<'a>>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}

#[ast]
pub enum MaxSize<'a> {
    None,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
    MathFunction(Box<'a, Function<'a>>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(Box<'a, LengthPercentage<'a>>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum OverflowKeyword {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PositionProperty {
    Static,
    Relative,
    Absolute,
    Sticky(VendorPrefix),
    Fixed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[ast]
pub struct Size2D<'a, T>(pub Box<'a, T>, pub Box<'a, T>);
#[ast]
pub struct Rect<'a, T>(
    pub Box<'a, T>,
    pub Box<'a, T>,
    pub Box<'a, T>,
    pub Box<'a, T>,
);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ZIndex {
    Auto,
    Integer(i32),
}
