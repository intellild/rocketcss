#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}

#[ast]
pub enum ScrollbarColor<'a> {
    Auto,
    Colors(Box<'a, CssColor<'a>>, Box<'a, CssColor<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum PointerEvents {
    Auto,
    None,
    VisiblePainted,
    VisibleFill,
    VisibleStroke,
    Visible,
    Painted,
    Fill,
    Stroke,
    All,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Float {
    None,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Clear {
    None,
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TouchAction {
    Auto,
    None,
    Manipulation,
    PanX,
    PanY,
    PanLeft,
    PanRight,
    PanUp,
    PanDown,
    PinchZoom,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum CursorKeyword {
    Auto,
    Default,
    None,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}

#[ast]
pub enum ColorOrAuto<'a> {
    Auto,
    Color(Box<'a, CssColor<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum CaretShape {
    Auto,
    Bar,
    Block,
    Underscore,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum UserSelect {
    Auto,
    Text,
    None,
    Contain,
    All,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy)]
pub enum Appearance<'a> {
    None,
    Auto,
    Textfield,
    MenulistButton,
    Button,
    Checkbox,
    Listbox,
    Menulist,
    Meter,
    ProgressBar,
    PushButton,
    Radio,
    Searchfield,
    SliderHorizontal,
    SquareButton,
    Textarea,
    NonStandard(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum PrintColorAdjust {
    Economy,
    Exact,
}
