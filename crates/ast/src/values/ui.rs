use crate::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ScrollbarColor<'a> {
    Auto,
    Colors(NodeId<'a, CssColor<'a>>, NodeId<'a, CssColor<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Float {
    None,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Clear {
    None,
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
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

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ColorOrAuto<'a> {
    Auto,
    Color(NodeId<'a, CssColor<'a>>),
}

impl_inline_node!(ColorOrAuto<'ast>, 0x00130001);

impl<'ast> AstNodeClone<'ast> for ColorOrAuto<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Auto => Self::Auto,
            Self::Color(value) => Self::Color(context.clone_encoded_node(value)),
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum CaretShape {
    Auto,
    Bar,
    Block,
    Underscore,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum UserSelect {
    Auto,
    Text,
    None,
    Contain,
    All,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
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
    NonStandard(AstStr<'a>),
}

// SAFETY: this KIND always publishes and reads the same native Copy type.
unsafe impl<'ast> AstNodeStorage<'ast> for Appearance<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0013_0002);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::NonStandard(a), Self::NonStandard(b)) => context.str(*a) == context.str(*b),
            _ => self == other,
        }
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast> AstNodeClone<'ast> for Appearance<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum PrintColorAdjust {
    Economy,
    Exact,
}
