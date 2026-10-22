use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Display<'ast>(pub(crate) NodeId<'ast, DisplayData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DisplayData<'ast> {
    Keyword(DisplayKeyword),
    Pair {
        inside: DisplayInside<'ast>,
        is_list_item: bool,
        outside: DisplayOutside,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DisplayKind {
    Keyword,
    Pair,
}
impl<'ast> NodeStorage<'ast> for Display<'ast> {
    const KIND: NodeKind = super::kind::KIND_DISPLAY;
    type Data = DisplayData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, DisplayData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, DisplayData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Display<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> DisplayData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> DisplayKind {
        match ast.data(self) {
            DisplayData::Keyword { .. } => DisplayKind::Keyword,
            DisplayData::Pair { .. } => DisplayKind::Pair,
        }
    }
}
impl<'ast> GetSpan<'ast> for Display<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Display<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_display(&mut self, data: DisplayData<'ast>, span: Span) -> Display<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
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
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DisplayInside<'ast>(pub(crate) NodeId<'ast, DisplayInsideData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DisplayInsideData {
    Flow,
    FlowRoot,
    Table,
    Flex { vendor_prefix: VendorPrefix },
    Box { vendor_prefix: VendorPrefix },
    Grid,
    Ruby,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DisplayInsideKind {
    Flow,
    FlowRoot,
    Table,
    Flex,
    Box,
    Grid,
    Ruby,
}
impl<'ast> NodeStorage<'ast> for DisplayInside<'ast> {
    const KIND: NodeKind = super::kind::KIND_DISPLAY_INSIDE;
    type Data = DisplayInsideData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, DisplayInsideData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, DisplayInsideData>) -> Self {
        Self(id)
    }
}
impl<'ast> DisplayInside<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> DisplayInsideData {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> DisplayInsideKind {
        match ast.data(self) {
            DisplayInsideData::Flow { .. } => DisplayInsideKind::Flow,
            DisplayInsideData::FlowRoot { .. } => DisplayInsideKind::FlowRoot,
            DisplayInsideData::Table { .. } => DisplayInsideKind::Table,
            DisplayInsideData::Flex { .. } => DisplayInsideKind::Flex,
            DisplayInsideData::Box { .. } => DisplayInsideKind::Box,
            DisplayInsideData::Grid { .. } => DisplayInsideKind::Grid,
            DisplayInsideData::Ruby { .. } => DisplayInsideKind::Ruby,
        }
    }
}
impl<'ast> GetSpan<'ast> for DisplayInside<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for DisplayInside<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_display_inside(
        &mut self,
        data: DisplayInsideData,
        span: Span,
    ) -> DisplayInside<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum DisplayOutside {
    Block,
    Inline,
    RunIn,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Size<'ast>(pub(crate) NodeId<'ast, SizeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SizeData<'ast> {
    Auto,
    LengthPercentage(LengthPercentage<'ast>),
    MathFunction(Function<'ast>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(LengthPercentage<'ast>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SizeKind {
    Auto,
    LengthPercentage,
    MathFunction,
    MinContent,
    MaxContent,
    FitContent,
    FitContentFunction,
    Stretch,
    Contain,
}
impl<'ast> NodeStorage<'ast> for Size<'ast> {
    const KIND: NodeKind = super::kind::KIND_SIZE;
    type Data = SizeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SizeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SizeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Size<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SizeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SizeKind {
        match ast.data(self) {
            SizeData::Auto { .. } => SizeKind::Auto,
            SizeData::LengthPercentage { .. } => SizeKind::LengthPercentage,
            SizeData::MathFunction { .. } => SizeKind::MathFunction,
            SizeData::MinContent { .. } => SizeKind::MinContent,
            SizeData::MaxContent { .. } => SizeKind::MaxContent,
            SizeData::FitContent { .. } => SizeKind::FitContent,
            SizeData::FitContentFunction { .. } => SizeKind::FitContentFunction,
            SizeData::Stretch { .. } => SizeKind::Stretch,
            SizeData::Contain { .. } => SizeKind::Contain,
        }
    }
}
impl<'ast> GetSpan<'ast> for Size<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Size<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_size(&mut self, data: SizeData<'ast>, span: Span) -> Size<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MaxSize<'ast>(pub(crate) NodeId<'ast, MaxSizeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaxSizeData<'ast> {
    None,
    LengthPercentage(LengthPercentage<'ast>),
    MathFunction(Function<'ast>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(LengthPercentage<'ast>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaxSizeKind {
    None,
    LengthPercentage,
    MathFunction,
    MinContent,
    MaxContent,
    FitContent,
    FitContentFunction,
    Stretch,
    Contain,
}
impl<'ast> NodeStorage<'ast> for MaxSize<'ast> {
    const KIND: NodeKind = super::kind::KIND_MAX_SIZE;
    type Data = MaxSizeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MaxSizeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MaxSizeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MaxSize<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> MaxSizeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> MaxSizeKind {
        match ast.data(self) {
            MaxSizeData::None { .. } => MaxSizeKind::None,
            MaxSizeData::LengthPercentage { .. } => MaxSizeKind::LengthPercentage,
            MaxSizeData::MathFunction { .. } => MaxSizeKind::MathFunction,
            MaxSizeData::MinContent { .. } => MaxSizeKind::MinContent,
            MaxSizeData::MaxContent { .. } => MaxSizeKind::MaxContent,
            MaxSizeData::FitContent { .. } => MaxSizeKind::FitContent,
            MaxSizeData::FitContentFunction { .. } => MaxSizeKind::FitContentFunction,
            MaxSizeData::Stretch { .. } => MaxSizeKind::Stretch,
            MaxSizeData::Contain { .. } => MaxSizeKind::Contain,
        }
    }
}
impl<'ast> GetSpan<'ast> for MaxSize<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MaxSize<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_max_size(&mut self, data: MaxSizeData<'ast>, span: Span) -> MaxSize<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum OverflowKeyword {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PositionProperty {
    Static,
    Relative,
    Absolute,
    Sticky(VendorPrefix),
    Fixed,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Size2D<'ast, T>(pub(crate) NodeId<'ast, Size2DData<T>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size2DData<T> {
    pub field_0: T,
    pub field_1: T,
}
impl<'ast, T> NodeStorage<'ast> for Size2D<'ast, T>
where
    T: Copy,
{
    const KIND: NodeKind = super::kind::KIND_SIZE_2_D;
    type Data = Size2DData<T>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, Size2DData<T>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, Size2DData<T>>) -> Self {
        Self(id)
    }
}
impl<'ast, T> Size2D<'ast, T>
where
    T: Copy,
{
    #[inline]
    pub fn field_0(self, ast: &Ast<'ast>) -> T {
        ast.data(self).field_0
    }
    #[inline]
    pub fn set_field_0(self, ast: &mut Ast<'ast>, value: T) {
        let mut data = ast.data(self);
        data.field_0 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn field_1(self, ast: &Ast<'ast>) -> T {
        ast.data(self).field_1
    }
    #[inline]
    pub fn set_field_1(self, ast: &mut Ast<'ast>, value: T) {
        let mut data = ast.data(self);
        data.field_1 = value;
        ast.set_data(self, data);
    }
}
impl<'ast, T> GetSpan<'ast> for Size2D<'ast, T>
where
    T: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, T> SetSpan<'ast> for Size2D<'ast, T>
where
    T: Copy,
{
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_size_2_d<T>(&mut self, data: Size2DData<T>, span: Span) -> Size2D<'ast, T>
    where
        T: Copy,
    {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Rect<'ast, T>(pub(crate) NodeId<'ast, RectData<T>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectData<T> {
    pub field_0: T,
    pub field_1: T,
    pub field_2: T,
    pub field_3: T,
}
impl<'ast, T> NodeStorage<'ast> for Rect<'ast, T>
where
    T: Copy,
{
    const KIND: NodeKind = super::kind::KIND_RECT;
    type Data = RectData<T>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, RectData<T>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, RectData<T>>) -> Self {
        Self(id)
    }
}
impl<'ast, T> Rect<'ast, T>
where
    T: Copy,
{
    #[inline]
    pub fn field_0(self, ast: &Ast<'ast>) -> T {
        ast.data(self).field_0
    }
    #[inline]
    pub fn set_field_0(self, ast: &mut Ast<'ast>, value: T) {
        let mut data = ast.data(self);
        data.field_0 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn field_1(self, ast: &Ast<'ast>) -> T {
        ast.data(self).field_1
    }
    #[inline]
    pub fn set_field_1(self, ast: &mut Ast<'ast>, value: T) {
        let mut data = ast.data(self);
        data.field_1 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn field_2(self, ast: &Ast<'ast>) -> T {
        ast.data(self).field_2
    }
    #[inline]
    pub fn set_field_2(self, ast: &mut Ast<'ast>, value: T) {
        let mut data = ast.data(self);
        data.field_2 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn field_3(self, ast: &Ast<'ast>) -> T {
        ast.data(self).field_3
    }
    #[inline]
    pub fn set_field_3(self, ast: &mut Ast<'ast>, value: T) {
        let mut data = ast.data(self);
        data.field_3 = value;
        ast.set_data(self, data);
    }
}
impl<'ast, T> GetSpan<'ast> for Rect<'ast, T>
where
    T: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, T> SetSpan<'ast> for Rect<'ast, T>
where
    T: Copy,
{
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_rect<T>(&mut self, data: RectData<T>, span: Span) -> Rect<'ast, T>
    where
        T: Copy,
    {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ZIndex {
    Auto,
    Integer(i32),
}
