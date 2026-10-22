use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollbarColor<'ast>(pub(crate) NodeId<'ast, ScrollbarColorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollbarColorData<'ast> {
    Auto,
    Colors(CssColor<'ast>, CssColor<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScrollbarColorKind {
    Auto,
    Colors,
}
impl<'ast> NodeStorage<'ast> for ScrollbarColor<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLLBAR_COLOR;
    type Data = ScrollbarColorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollbarColorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollbarColorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollbarColor<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ScrollbarColorData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ScrollbarColorKind {
        match ast.data(self) {
            ScrollbarColorData::Auto { .. } => ScrollbarColorKind::Auto,
            ScrollbarColorData::Colors { .. } => ScrollbarColorKind::Colors,
        }
    }
}
impl<'ast> GetSpan<'ast> for ScrollbarColor<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollbarColor<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scrollbar_color(
        &mut self,
        data: ScrollbarColorData<'ast>,
        span: Span,
    ) -> ScrollbarColor<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
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
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Float {
    None,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Clear {
    None,
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,
}
use crate::*;
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
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}
use crate::*;
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
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ColorOrAuto<'ast>(pub(crate) NodeId<'ast, ColorOrAutoData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorOrAutoData<'ast> {
    Auto,
    Color(CssColor<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorOrAutoKind {
    Auto,
    Color,
}
impl<'ast> NodeStorage<'ast> for ColorOrAuto<'ast> {
    const KIND: NodeKind = super::kind::KIND_COLOR_OR_AUTO;
    type Data = ColorOrAutoData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ColorOrAutoData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ColorOrAutoData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ColorOrAuto<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ColorOrAutoData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ColorOrAutoKind {
        match ast.data(self) {
            ColorOrAutoData::Auto { .. } => ColorOrAutoKind::Auto,
            ColorOrAutoData::Color { .. } => ColorOrAutoKind::Color,
        }
    }
}
impl<'ast> GetSpan<'ast> for ColorOrAuto<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ColorOrAuto<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_color_or_auto(
        &mut self,
        data: ColorOrAutoData<'ast>,
        span: Span,
    ) -> ColorOrAuto<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum CaretShape {
    Auto,
    Bar,
    Block,
    Underscore,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum UserSelect {
    Auto,
    Text,
    None,
    Contain,
    All,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy)]
pub enum Appearance<'ast> {
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
    NonStandard(&'ast str),
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum PrintColorAdjust {
    Economy,
    Exact,
}
