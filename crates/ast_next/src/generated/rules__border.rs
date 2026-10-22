use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderRadius<'ast>(pub(crate) NodeId<'ast, BorderRadiusData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderRadiusData<'ast> {
    pub bottom_left: Size2D<'ast, LengthPercentage<'ast>>,
    pub bottom_right: Size2D<'ast, LengthPercentage<'ast>>,
    pub top_left: Size2D<'ast, LengthPercentage<'ast>>,
    pub top_right: Size2D<'ast, LengthPercentage<'ast>>,
}
impl<'ast> NodeStorage<'ast> for BorderRadius<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_RADIUS;
    type Data = BorderRadiusData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderRadiusData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderRadiusData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderRadius<'ast> {
    #[inline]
    pub fn bottom_left(self, ast: &Ast<'ast>) -> Size2D<'ast, LengthPercentage<'ast>> {
        ast.data(self).bottom_left
    }
    #[inline]
    pub fn set_bottom_left(self, ast: &mut Ast<'ast>, value: Size2D<'ast, LengthPercentage<'ast>>) {
        let mut data = ast.data(self);
        data.bottom_left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn bottom_right(self, ast: &Ast<'ast>) -> Size2D<'ast, LengthPercentage<'ast>> {
        ast.data(self).bottom_right
    }
    #[inline]
    pub fn set_bottom_right(
        self,
        ast: &mut Ast<'ast>,
        value: Size2D<'ast, LengthPercentage<'ast>>,
    ) {
        let mut data = ast.data(self);
        data.bottom_right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top_left(self, ast: &Ast<'ast>) -> Size2D<'ast, LengthPercentage<'ast>> {
        ast.data(self).top_left
    }
    #[inline]
    pub fn set_top_left(self, ast: &mut Ast<'ast>, value: Size2D<'ast, LengthPercentage<'ast>>) {
        let mut data = ast.data(self);
        data.top_left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top_right(self, ast: &Ast<'ast>) -> Size2D<'ast, LengthPercentage<'ast>> {
        ast.data(self).top_right
    }
    #[inline]
    pub fn set_top_right(self, ast: &mut Ast<'ast>, value: Size2D<'ast, LengthPercentage<'ast>>) {
        let mut data = ast.data(self);
        data.top_right = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderRadius<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderRadius<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_radius(
        &mut self,
        data: BorderRadiusData<'ast>,
        span: Span,
    ) -> BorderRadius<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderImageRepeat<'ast>(pub(crate) NodeId<'ast, BorderImageRepeatData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderImageRepeatData {
    pub horizontal: BorderImageRepeatKeyword,
    pub vertical: BorderImageRepeatKeyword,
}
impl<'ast> NodeStorage<'ast> for BorderImageRepeat<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_IMAGE_REPEAT;
    type Data = BorderImageRepeatData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderImageRepeatData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderImageRepeatData>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderImageRepeat<'ast> {
    #[inline]
    pub fn horizontal(self, ast: &Ast<'ast>) -> BorderImageRepeatKeyword {
        ast.data(self).horizontal
    }
    #[inline]
    pub fn set_horizontal(self, ast: &mut Ast<'ast>, value: BorderImageRepeatKeyword) {
        let mut data = ast.data(self);
        data.horizontal = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn vertical(self, ast: &Ast<'ast>) -> BorderImageRepeatKeyword {
        ast.data(self).vertical
    }
    #[inline]
    pub fn set_vertical(self, ast: &mut Ast<'ast>, value: BorderImageRepeatKeyword) {
        let mut data = ast.data(self);
        data.vertical = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderImageRepeat<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderImageRepeat<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_image_repeat(
        &mut self,
        data: BorderImageRepeatData,
        span: Span,
    ) -> BorderImageRepeat<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderImageSlice<'ast>(pub(crate) NodeId<'ast, BorderImageSliceData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderImageSliceData<'ast> {
    pub fill: bool,
    pub offsets: Rect<'ast, NumberOrPercentage>,
}
impl<'ast> NodeStorage<'ast> for BorderImageSlice<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_IMAGE_SLICE;
    type Data = BorderImageSliceData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderImageSliceData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderImageSliceData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderImageSlice<'ast> {
    #[inline]
    pub fn fill(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).fill
    }
    #[inline]
    pub fn set_fill(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.fill = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn offsets(self, ast: &Ast<'ast>) -> Rect<'ast, NumberOrPercentage> {
        ast.data(self).offsets
    }
    #[inline]
    pub fn set_offsets(self, ast: &mut Ast<'ast>, value: Rect<'ast, NumberOrPercentage>) {
        let mut data = ast.data(self);
        data.offsets = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderImageSlice<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderImageSlice<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_image_slice(
        &mut self,
        data: BorderImageSliceData<'ast>,
        span: Span,
    ) -> BorderImageSlice<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderImage<'ast>(pub(crate) NodeId<'ast, BorderImageData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderImageData<'ast> {
    pub outset: Rect<'ast, LengthOrNumber<'ast>>,
    pub repeat: BorderImageRepeat<'ast>,
    pub slice: BorderImageSlice<'ast>,
    pub source: Image<'ast>,
    pub width: Rect<'ast, BorderImageSideWidth<'ast>>,
}
impl<'ast> NodeStorage<'ast> for BorderImage<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_IMAGE;
    type Data = BorderImageData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderImageData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderImageData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderImage<'ast> {
    #[inline]
    pub fn outset(self, ast: &Ast<'ast>) -> Rect<'ast, LengthOrNumber<'ast>> {
        ast.data(self).outset
    }
    #[inline]
    pub fn set_outset(self, ast: &mut Ast<'ast>, value: Rect<'ast, LengthOrNumber<'ast>>) {
        let mut data = ast.data(self);
        data.outset = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn repeat(self, ast: &Ast<'ast>) -> BorderImageRepeat<'ast> {
        ast.data(self).repeat
    }
    #[inline]
    pub fn set_repeat(self, ast: &mut Ast<'ast>, value: BorderImageRepeat<'ast>) {
        let mut data = ast.data(self);
        data.repeat = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn slice(self, ast: &Ast<'ast>) -> BorderImageSlice<'ast> {
        ast.data(self).slice
    }
    #[inline]
    pub fn set_slice(self, ast: &mut Ast<'ast>, value: BorderImageSlice<'ast>) {
        let mut data = ast.data(self);
        data.slice = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn source(self, ast: &Ast<'ast>) -> Image<'ast> {
        ast.data(self).source
    }
    #[inline]
    pub fn set_source(self, ast: &mut Ast<'ast>, value: Image<'ast>) {
        let mut data = ast.data(self);
        data.source = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn width(self, ast: &Ast<'ast>) -> Rect<'ast, BorderImageSideWidth<'ast>> {
        ast.data(self).width
    }
    #[inline]
    pub fn set_width(self, ast: &mut Ast<'ast>, value: Rect<'ast, BorderImageSideWidth<'ast>>) {
        let mut data = ast.data(self);
        data.width = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderImage<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderImage<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_image(
        &mut self,
        data: BorderImageData<'ast>,
        span: Span,
    ) -> BorderImage<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderColor<'ast>(pub(crate) NodeId<'ast, BorderColorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderColorData<'ast> {
    pub bottom: CssColor<'ast>,
    pub left: CssColor<'ast>,
    pub right: CssColor<'ast>,
    pub top: CssColor<'ast>,
}
impl<'ast> NodeStorage<'ast> for BorderColor<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_COLOR;
    type Data = BorderColorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderColorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderColorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderColor<'ast> {
    #[inline]
    pub fn bottom(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).bottom
    }
    #[inline]
    pub fn set_bottom(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.bottom = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn left(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).left
    }
    #[inline]
    pub fn set_left(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn right(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).right
    }
    #[inline]
    pub fn set_right(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).top
    }
    #[inline]
    pub fn set_top(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.top = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderColor<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderColor<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_color(
        &mut self,
        data: BorderColorData<'ast>,
        span: Span,
    ) -> BorderColor<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderStyle<'ast>(pub(crate) NodeId<'ast, BorderStyleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderStyleData {
    pub bottom: LineStyle,
    pub left: LineStyle,
    pub right: LineStyle,
    pub top: LineStyle,
}
impl<'ast> NodeStorage<'ast> for BorderStyle<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_STYLE;
    type Data = BorderStyleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderStyleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderStyleData>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderStyle<'ast> {
    #[inline]
    pub fn bottom(self, ast: &Ast<'ast>) -> LineStyle {
        ast.data(self).bottom
    }
    #[inline]
    pub fn set_bottom(self, ast: &mut Ast<'ast>, value: LineStyle) {
        let mut data = ast.data(self);
        data.bottom = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn left(self, ast: &Ast<'ast>) -> LineStyle {
        ast.data(self).left
    }
    #[inline]
    pub fn set_left(self, ast: &mut Ast<'ast>, value: LineStyle) {
        let mut data = ast.data(self);
        data.left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn right(self, ast: &Ast<'ast>) -> LineStyle {
        ast.data(self).right
    }
    #[inline]
    pub fn set_right(self, ast: &mut Ast<'ast>, value: LineStyle) {
        let mut data = ast.data(self);
        data.right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top(self, ast: &Ast<'ast>) -> LineStyle {
        ast.data(self).top
    }
    #[inline]
    pub fn set_top(self, ast: &mut Ast<'ast>, value: LineStyle) {
        let mut data = ast.data(self);
        data.top = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderStyle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderStyle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_style(&mut self, data: BorderStyleData, span: Span) -> BorderStyle<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderWidth<'ast>(pub(crate) NodeId<'ast, BorderWidthData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderWidthData<'ast> {
    pub bottom: BorderSideWidth<'ast>,
    pub left: BorderSideWidth<'ast>,
    pub right: BorderSideWidth<'ast>,
    pub top: BorderSideWidth<'ast>,
}
impl<'ast> NodeStorage<'ast> for BorderWidth<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_WIDTH;
    type Data = BorderWidthData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderWidthData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderWidthData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderWidth<'ast> {
    #[inline]
    pub fn bottom(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).bottom
    }
    #[inline]
    pub fn set_bottom(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.bottom = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn left(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).left
    }
    #[inline]
    pub fn set_left(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn right(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).right
    }
    #[inline]
    pub fn set_right(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).top
    }
    #[inline]
    pub fn set_top(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.top = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderWidth<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderWidth<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_width(
        &mut self,
        data: BorderWidthData<'ast>,
        span: Span,
    ) -> BorderWidth<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderBlockColor<'ast>(pub(crate) NodeId<'ast, BorderBlockColorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderBlockColorData<'ast> {
    pub end: CssColor<'ast>,
    pub start: CssColor<'ast>,
}
impl<'ast> NodeStorage<'ast> for BorderBlockColor<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_BLOCK_COLOR;
    type Data = BorderBlockColorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderBlockColorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderBlockColorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderBlockColor<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderBlockColor<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderBlockColor<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_block_color(
        &mut self,
        data: BorderBlockColorData<'ast>,
        span: Span,
    ) -> BorderBlockColor<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderBlockStyle<'ast>(pub(crate) NodeId<'ast, BorderBlockStyleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderBlockStyleData {
    pub end: LineStyle,
    pub start: LineStyle,
}
impl<'ast> NodeStorage<'ast> for BorderBlockStyle<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_BLOCK_STYLE;
    type Data = BorderBlockStyleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderBlockStyleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderBlockStyleData>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderBlockStyle<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> LineStyle {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: LineStyle) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> LineStyle {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: LineStyle) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderBlockStyle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderBlockStyle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_block_style(
        &mut self,
        data: BorderBlockStyleData,
        span: Span,
    ) -> BorderBlockStyle<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderBlockWidth<'ast>(pub(crate) NodeId<'ast, BorderBlockWidthData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderBlockWidthData<'ast> {
    pub end: BorderSideWidth<'ast>,
    pub start: BorderSideWidth<'ast>,
}
impl<'ast> NodeStorage<'ast> for BorderBlockWidth<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_BLOCK_WIDTH;
    type Data = BorderBlockWidthData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderBlockWidthData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderBlockWidthData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderBlockWidth<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderBlockWidth<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderBlockWidth<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_block_width(
        &mut self,
        data: BorderBlockWidthData<'ast>,
        span: Span,
    ) -> BorderBlockWidth<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderInlineColor<'ast>(pub(crate) NodeId<'ast, BorderInlineColorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderInlineColorData<'ast> {
    pub end: CssColor<'ast>,
    pub start: CssColor<'ast>,
}
impl<'ast> NodeStorage<'ast> for BorderInlineColor<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_INLINE_COLOR;
    type Data = BorderInlineColorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderInlineColorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderInlineColorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderInlineColor<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderInlineColor<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderInlineColor<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_inline_color(
        &mut self,
        data: BorderInlineColorData<'ast>,
        span: Span,
    ) -> BorderInlineColor<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderInlineStyle<'ast>(pub(crate) NodeId<'ast, BorderInlineStyleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderInlineStyleData {
    pub end: LineStyle,
    pub start: LineStyle,
}
impl<'ast> NodeStorage<'ast> for BorderInlineStyle<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_INLINE_STYLE;
    type Data = BorderInlineStyleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderInlineStyleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderInlineStyleData>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderInlineStyle<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> LineStyle {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: LineStyle) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> LineStyle {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: LineStyle) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderInlineStyle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderInlineStyle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_inline_style(
        &mut self,
        data: BorderInlineStyleData,
        span: Span,
    ) -> BorderInlineStyle<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BorderInlineWidth<'ast>(pub(crate) NodeId<'ast, BorderInlineWidthData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderInlineWidthData<'ast> {
    pub end: BorderSideWidth<'ast>,
    pub start: BorderSideWidth<'ast>,
}
impl<'ast> NodeStorage<'ast> for BorderInlineWidth<'ast> {
    const KIND: NodeKind = super::kind::KIND_BORDER_INLINE_WIDTH;
    type Data = BorderInlineWidthData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BorderInlineWidthData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BorderInlineWidthData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BorderInlineWidth<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BorderInlineWidth<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BorderInlineWidth<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_border_inline_width(
        &mut self,
        data: BorderInlineWidthData<'ast>,
        span: Span,
    ) -> BorderInlineWidth<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GenericBorder<'ast, S>(pub(crate) NodeId<'ast, GenericBorderData<'ast, S>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GenericBorderData<'ast, S> {
    pub color: CssColor<'ast>,
    pub style: S,
    pub width: BorderSideWidth<'ast>,
}
impl<'ast, S> NodeStorage<'ast> for GenericBorder<'ast, S>
where
    S: Copy,
{
    const KIND: NodeKind = super::kind::KIND_GENERIC_BORDER;
    type Data = GenericBorderData<'ast, S>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GenericBorderData<'ast, S>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GenericBorderData<'ast, S>>) -> Self {
        Self(id)
    }
}
impl<'ast, S> GenericBorder<'ast, S>
where
    S: Copy,
{
    #[inline]
    pub fn color(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).color
    }
    #[inline]
    pub fn set_color(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.color = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn style(self, ast: &Ast<'ast>) -> S {
        ast.data(self).style
    }
    #[inline]
    pub fn set_style(self, ast: &mut Ast<'ast>, value: S) {
        let mut data = ast.data(self);
        data.style = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn width(self, ast: &Ast<'ast>) -> BorderSideWidth<'ast> {
        ast.data(self).width
    }
    #[inline]
    pub fn set_width(self, ast: &mut Ast<'ast>, value: BorderSideWidth<'ast>) {
        let mut data = ast.data(self);
        data.width = value;
        ast.set_data(self, data);
    }
}
impl<'ast, S> GetSpan<'ast> for GenericBorder<'ast, S>
where
    S: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, S> SetSpan<'ast> for GenericBorder<'ast, S>
where
    S: Copy,
{
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_generic_border<S>(
        &mut self,
        data: GenericBorderData<'ast, S>,
        span: Span,
    ) -> GenericBorder<'ast, S>
    where
        S: Copy,
    {
        self.alloc(data, span)
    }
}
