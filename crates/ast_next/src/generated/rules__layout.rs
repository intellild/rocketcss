use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AspectRatio<'ast>(pub(crate) NodeId<'ast, AspectRatioData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AspectRatioData<'ast> {
    pub auto: bool,
    pub ratio: Option<Ratio<'ast>>,
}
impl<'ast> NodeStorage<'ast> for AspectRatio<'ast> {
    const KIND: NodeKind = super::kind::KIND_ASPECT_RATIO;
    type Data = AspectRatioData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, AspectRatioData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, AspectRatioData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> AspectRatio<'ast> {
    #[inline]
    pub fn auto(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).auto
    }
    #[inline]
    pub fn set_auto(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.auto = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn ratio(self, ast: &Ast<'ast>) -> Option<Ratio<'ast>> {
        ast.data(self).ratio
    }
    #[inline]
    pub fn set_ratio(self, ast: &mut Ast<'ast>, value: Option<Ratio<'ast>>) {
        let mut data = ast.data(self);
        data.ratio = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for AspectRatio<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for AspectRatio<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_aspect_ratio(
        &mut self,
        data: AspectRatioData<'ast>,
        span: Span,
    ) -> AspectRatio<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Overflow<'ast>(pub(crate) NodeId<'ast, OverflowData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverflowData {
    pub x: OverflowKeyword,
    pub y: OverflowKeyword,
}
impl<'ast> NodeStorage<'ast> for Overflow<'ast> {
    const KIND: NodeKind = super::kind::KIND_OVERFLOW;
    type Data = OverflowData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, OverflowData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, OverflowData>) -> Self {
        Self(id)
    }
}
impl<'ast> Overflow<'ast> {
    #[inline]
    pub fn x(self, ast: &Ast<'ast>) -> OverflowKeyword {
        ast.data(self).x
    }
    #[inline]
    pub fn set_x(self, ast: &mut Ast<'ast>, value: OverflowKeyword) {
        let mut data = ast.data(self);
        data.x = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y(self, ast: &Ast<'ast>) -> OverflowKeyword {
        ast.data(self).y
    }
    #[inline]
    pub fn set_y(self, ast: &mut Ast<'ast>, value: OverflowKeyword) {
        let mut data = ast.data(self);
        data.y = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Overflow<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Overflow<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_overflow(&mut self, data: OverflowData, span: Span) -> Overflow<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InsetBlock<'ast>(pub(crate) NodeId<'ast, InsetBlockData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InsetBlockData<'ast> {
    pub block_end: LengthPercentageOrAuto<'ast>,
    pub block_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for InsetBlock<'ast> {
    const KIND: NodeKind = super::kind::KIND_INSET_BLOCK;
    type Data = InsetBlockData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, InsetBlockData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, InsetBlockData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> InsetBlock<'ast> {
    #[inline]
    pub fn block_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_end
    }
    #[inline]
    pub fn set_block_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn block_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_start
    }
    #[inline]
    pub fn set_block_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for InsetBlock<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for InsetBlock<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_inset_block(
        &mut self,
        data: InsetBlockData<'ast>,
        span: Span,
    ) -> InsetBlock<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InsetInline<'ast>(pub(crate) NodeId<'ast, InsetInlineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InsetInlineData<'ast> {
    pub inline_end: LengthPercentageOrAuto<'ast>,
    pub inline_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for InsetInline<'ast> {
    const KIND: NodeKind = super::kind::KIND_INSET_INLINE;
    type Data = InsetInlineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, InsetInlineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, InsetInlineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> InsetInline<'ast> {
    #[inline]
    pub fn inline_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_end
    }
    #[inline]
    pub fn set_inline_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn inline_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_start
    }
    #[inline]
    pub fn set_inline_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for InsetInline<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for InsetInline<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_inset_inline(
        &mut self,
        data: InsetInlineData<'ast>,
        span: Span,
    ) -> InsetInline<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Inset<'ast>(pub(crate) NodeId<'ast, InsetData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InsetData<'ast> {
    pub bottom: LengthPercentageOrAuto<'ast>,
    pub left: LengthPercentageOrAuto<'ast>,
    pub right: LengthPercentageOrAuto<'ast>,
    pub top: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for Inset<'ast> {
    const KIND: NodeKind = super::kind::KIND_INSET;
    type Data = InsetData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, InsetData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, InsetData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Inset<'ast> {
    #[inline]
    pub fn bottom(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).bottom
    }
    #[inline]
    pub fn set_bottom(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.bottom = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn left(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).left
    }
    #[inline]
    pub fn set_left(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn right(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).right
    }
    #[inline]
    pub fn set_right(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).top
    }
    #[inline]
    pub fn set_top(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.top = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Inset<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Inset<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_inset(&mut self, data: InsetData<'ast>, span: Span) -> Inset<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FlexFlow<'ast>(pub(crate) NodeId<'ast, FlexFlowData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexFlowData {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
}
impl<'ast> NodeStorage<'ast> for FlexFlow<'ast> {
    const KIND: NodeKind = super::kind::KIND_FLEX_FLOW;
    type Data = FlexFlowData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FlexFlowData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FlexFlowData>) -> Self {
        Self(id)
    }
}
impl<'ast> FlexFlow<'ast> {
    #[inline]
    pub fn direction(self, ast: &Ast<'ast>) -> FlexDirection {
        ast.data(self).direction
    }
    #[inline]
    pub fn set_direction(self, ast: &mut Ast<'ast>, value: FlexDirection) {
        let mut data = ast.data(self);
        data.direction = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn wrap(self, ast: &Ast<'ast>) -> FlexWrap {
        ast.data(self).wrap
    }
    #[inline]
    pub fn set_wrap(self, ast: &mut Ast<'ast>, value: FlexWrap) {
        let mut data = ast.data(self);
        data.wrap = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for FlexFlow<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FlexFlow<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_flex_flow(&mut self, data: FlexFlowData, span: Span) -> FlexFlow<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Flex<'ast>(pub(crate) NodeId<'ast, FlexData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexData<'ast> {
    pub basis: LengthPercentageOrAuto<'ast>,
    pub grow: f32,
    pub shrink: f32,
}
impl<'ast> NodeStorage<'ast> for Flex<'ast> {
    const KIND: NodeKind = super::kind::KIND_FLEX;
    type Data = FlexData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FlexData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FlexData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Flex<'ast> {
    #[inline]
    pub fn basis(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).basis
    }
    #[inline]
    pub fn set_basis(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.basis = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn grow(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).grow
    }
    #[inline]
    pub fn set_grow(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.grow = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn shrink(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).shrink
    }
    #[inline]
    pub fn set_shrink(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.shrink = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Flex<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Flex<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_flex(&mut self, data: FlexData<'ast>, span: Span) -> Flex<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PlaceContent<'ast>(pub(crate) NodeId<'ast, PlaceContentData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaceContentData {
    pub align: AlignContent,
    pub justify: JustifyContent,
}
impl<'ast> NodeStorage<'ast> for PlaceContent<'ast> {
    const KIND: NodeKind = super::kind::KIND_PLACE_CONTENT;
    type Data = PlaceContentData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PlaceContentData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PlaceContentData>) -> Self {
        Self(id)
    }
}
impl<'ast> PlaceContent<'ast> {
    #[inline]
    pub fn align(self, ast: &Ast<'ast>) -> AlignContent {
        ast.data(self).align
    }
    #[inline]
    pub fn set_align(self, ast: &mut Ast<'ast>, value: AlignContent) {
        let mut data = ast.data(self);
        data.align = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn justify(self, ast: &Ast<'ast>) -> JustifyContent {
        ast.data(self).justify
    }
    #[inline]
    pub fn set_justify(self, ast: &mut Ast<'ast>, value: JustifyContent) {
        let mut data = ast.data(self);
        data.justify = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PlaceContent<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PlaceContent<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_place_content(
        &mut self,
        data: PlaceContentData,
        span: Span,
    ) -> PlaceContent<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PlaceSelf<'ast>(pub(crate) NodeId<'ast, PlaceSelfData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaceSelfData {
    pub align: AlignSelf,
    pub justify: JustifySelf,
}
impl<'ast> NodeStorage<'ast> for PlaceSelf<'ast> {
    const KIND: NodeKind = super::kind::KIND_PLACE_SELF;
    type Data = PlaceSelfData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PlaceSelfData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PlaceSelfData>) -> Self {
        Self(id)
    }
}
impl<'ast> PlaceSelf<'ast> {
    #[inline]
    pub fn align(self, ast: &Ast<'ast>) -> AlignSelf {
        ast.data(self).align
    }
    #[inline]
    pub fn set_align(self, ast: &mut Ast<'ast>, value: AlignSelf) {
        let mut data = ast.data(self);
        data.align = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn justify(self, ast: &Ast<'ast>) -> JustifySelf {
        ast.data(self).justify
    }
    #[inline]
    pub fn set_justify(self, ast: &mut Ast<'ast>, value: JustifySelf) {
        let mut data = ast.data(self);
        data.justify = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PlaceSelf<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PlaceSelf<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_place_self(&mut self, data: PlaceSelfData, span: Span) -> PlaceSelf<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PlaceItems<'ast>(pub(crate) NodeId<'ast, PlaceItemsData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaceItemsData {
    pub align: AlignItems,
    pub justify: JustifyItems,
}
impl<'ast> NodeStorage<'ast> for PlaceItems<'ast> {
    const KIND: NodeKind = super::kind::KIND_PLACE_ITEMS;
    type Data = PlaceItemsData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PlaceItemsData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PlaceItemsData>) -> Self {
        Self(id)
    }
}
impl<'ast> PlaceItems<'ast> {
    #[inline]
    pub fn align(self, ast: &Ast<'ast>) -> AlignItems {
        ast.data(self).align
    }
    #[inline]
    pub fn set_align(self, ast: &mut Ast<'ast>, value: AlignItems) {
        let mut data = ast.data(self);
        data.align = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn justify(self, ast: &Ast<'ast>) -> JustifyItems {
        ast.data(self).justify
    }
    #[inline]
    pub fn set_justify(self, ast: &mut Ast<'ast>, value: JustifyItems) {
        let mut data = ast.data(self);
        data.justify = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PlaceItems<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PlaceItems<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_place_items(&mut self, data: PlaceItemsData, span: Span) -> PlaceItems<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Gap<'ast>(pub(crate) NodeId<'ast, GapData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GapData<'ast> {
    pub column: GapValue<'ast>,
    pub row: GapValue<'ast>,
}
impl<'ast> NodeStorage<'ast> for Gap<'ast> {
    const KIND: NodeKind = super::kind::KIND_GAP;
    type Data = GapData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GapData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GapData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Gap<'ast> {
    #[inline]
    pub fn column(self, ast: &Ast<'ast>) -> GapValue<'ast> {
        ast.data(self).column
    }
    #[inline]
    pub fn set_column(self, ast: &mut Ast<'ast>, value: GapValue<'ast>) {
        let mut data = ast.data(self);
        data.column = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn row(self, ast: &Ast<'ast>) -> GapValue<'ast> {
        ast.data(self).row
    }
    #[inline]
    pub fn set_row(self, ast: &mut Ast<'ast>, value: GapValue<'ast>) {
        let mut data = ast.data(self);
        data.row = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Gap<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Gap<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_gap(&mut self, data: GapData<'ast>, span: Span) -> Gap<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ColumnRule<'ast>(pub(crate) NodeId<'ast, ColumnRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColumnRuleData<'ast> {
    pub color: Option<CssColor<'ast>>,
    pub style: Option<LineStyle>,
    pub width: Option<BorderSideWidth<'ast>>,
}
impl<'ast> NodeStorage<'ast> for ColumnRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_COLUMN_RULE;
    type Data = ColumnRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ColumnRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ColumnRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ColumnRule<'ast> {
    #[inline]
    pub fn color(self, ast: &Ast<'ast>) -> Option<CssColor<'ast>> {
        ast.data(self).color
    }
    #[inline]
    pub fn set_color(self, ast: &mut Ast<'ast>, value: Option<CssColor<'ast>>) {
        let mut data = ast.data(self);
        data.color = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn style(self, ast: &Ast<'ast>) -> Option<LineStyle> {
        ast.data(self).style
    }
    #[inline]
    pub fn set_style(self, ast: &mut Ast<'ast>, value: Option<LineStyle>) {
        let mut data = ast.data(self);
        data.style = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn width(self, ast: &Ast<'ast>) -> Option<BorderSideWidth<'ast>> {
        ast.data(self).width
    }
    #[inline]
    pub fn set_width(self, ast: &mut Ast<'ast>, value: Option<BorderSideWidth<'ast>>) {
        let mut data = ast.data(self);
        data.width = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ColumnRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ColumnRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_column_rule(
        &mut self,
        data: ColumnRuleData<'ast>,
        span: Span,
    ) -> ColumnRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ColumnWidth<'ast>(pub(crate) NodeId<'ast, ColumnWidthData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnWidthData<'ast> {
    Auto,
    Length(Length<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColumnWidthKind {
    Auto,
    Length,
}
impl<'ast> NodeStorage<'ast> for ColumnWidth<'ast> {
    const KIND: NodeKind = super::kind::KIND_COLUMN_WIDTH;
    type Data = ColumnWidthData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ColumnWidthData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ColumnWidthData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ColumnWidth<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ColumnWidthData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ColumnWidthKind {
        match ast.data(self) {
            ColumnWidthData::Auto => ColumnWidthKind::Auto,
            ColumnWidthData::Length { .. } => ColumnWidthKind::Length,
        }
    }
}
impl<'ast> GetSpan<'ast> for ColumnWidth<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ColumnWidth<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_column_width(
        &mut self,
        data: ColumnWidthData<'ast>,
        span: Span,
    ) -> ColumnWidth<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColumnCount {
    Auto,
    Integer(i32),
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Columns<'ast>(pub(crate) NodeId<'ast, ColumnsData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColumnsData<'ast> {
    pub count: ColumnCount,
    pub width: ColumnWidth<'ast>,
}
impl<'ast> NodeStorage<'ast> for Columns<'ast> {
    const KIND: NodeKind = super::kind::KIND_COLUMNS;
    type Data = ColumnsData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ColumnsData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ColumnsData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Columns<'ast> {
    #[inline]
    pub fn count(self, ast: &Ast<'ast>) -> ColumnCount {
        ast.data(self).count
    }
    #[inline]
    pub fn set_count(self, ast: &mut Ast<'ast>, value: ColumnCount) {
        let mut data = ast.data(self);
        data.count = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn width(self, ast: &Ast<'ast>) -> ColumnWidth<'ast> {
        ast.data(self).width
    }
    #[inline]
    pub fn set_width(self, ast: &mut Ast<'ast>, value: ColumnWidth<'ast>) {
        let mut data = ast.data(self);
        data.width = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Columns<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Columns<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_columns(&mut self, data: ColumnsData<'ast>, span: Span) -> Columns<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TrackRepeat<'ast>(pub(crate) NodeId<'ast, TrackRepeatData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackRepeatData<'ast> {
    pub count: RepeatCount,
    pub line_names: &'ast [&'ast [&'ast str]],
    pub track_sizes: &'ast [TrackSize<'ast>],
}
impl<'ast> NodeStorage<'ast> for TrackRepeat<'ast> {
    const KIND: NodeKind = super::kind::KIND_TRACK_REPEAT;
    type Data = TrackRepeatData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, TrackRepeatData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, TrackRepeatData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> TrackRepeat<'ast> {
    #[inline]
    pub fn count(self, ast: &Ast<'ast>) -> RepeatCount {
        ast.data(self).count
    }
    #[inline]
    pub fn set_count(self, ast: &mut Ast<'ast>, value: RepeatCount) {
        let mut data = ast.data(self);
        data.count = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn line_names(self, ast: &Ast<'ast>) -> &'ast [&'ast [&'ast str]] {
        ast.data(self).line_names
    }
    #[inline]
    pub fn set_line_names(self, ast: &mut Ast<'ast>, value: &'ast [&'ast [&'ast str]]) {
        let mut data = ast.data(self);
        data.line_names = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn track_sizes(self, ast: &Ast<'ast>) -> &'ast [TrackSize<'ast>] {
        ast.data(self).track_sizes
    }
    #[inline]
    pub fn set_track_sizes(self, ast: &mut Ast<'ast>, value: &'ast [TrackSize<'ast>]) {
        let mut data = ast.data(self);
        data.track_sizes = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for TrackRepeat<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for TrackRepeat<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_track_repeat(
        &mut self,
        data: TrackRepeatData<'ast>,
        span: Span,
    ) -> TrackRepeat<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GridAutoFlow<'ast>(pub(crate) NodeId<'ast, GridAutoFlowData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridAutoFlowData {
    pub dense: bool,
    pub direction: AutoFlowDirection,
}
impl<'ast> NodeStorage<'ast> for GridAutoFlow<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRID_AUTO_FLOW;
    type Data = GridAutoFlowData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GridAutoFlowData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GridAutoFlowData>) -> Self {
        Self(id)
    }
}
impl<'ast> GridAutoFlow<'ast> {
    #[inline]
    pub fn dense(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).dense
    }
    #[inline]
    pub fn set_dense(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.dense = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn direction(self, ast: &Ast<'ast>) -> AutoFlowDirection {
        ast.data(self).direction
    }
    #[inline]
    pub fn set_direction(self, ast: &mut Ast<'ast>, value: AutoFlowDirection) {
        let mut data = ast.data(self);
        data.direction = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for GridAutoFlow<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for GridAutoFlow<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_grid_auto_flow(
        &mut self,
        data: GridAutoFlowData,
        span: Span,
    ) -> GridAutoFlow<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GridTemplate<'ast>(pub(crate) NodeId<'ast, GridTemplateData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridTemplateData<'ast> {
    pub areas: GridTemplateAreas<'ast>,
    pub columns: TrackSizing<'ast>,
    pub rows: TrackSizing<'ast>,
}
impl<'ast> NodeStorage<'ast> for GridTemplate<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRID_TEMPLATE;
    type Data = GridTemplateData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GridTemplateData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GridTemplateData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> GridTemplate<'ast> {
    #[inline]
    pub fn areas(self, ast: &Ast<'ast>) -> GridTemplateAreas<'ast> {
        ast.data(self).areas
    }
    #[inline]
    pub fn set_areas(self, ast: &mut Ast<'ast>, value: GridTemplateAreas<'ast>) {
        let mut data = ast.data(self);
        data.areas = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn columns(self, ast: &Ast<'ast>) -> TrackSizing<'ast> {
        ast.data(self).columns
    }
    #[inline]
    pub fn set_columns(self, ast: &mut Ast<'ast>, value: TrackSizing<'ast>) {
        let mut data = ast.data(self);
        data.columns = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn rows(self, ast: &Ast<'ast>) -> TrackSizing<'ast> {
        ast.data(self).rows
    }
    #[inline]
    pub fn set_rows(self, ast: &mut Ast<'ast>, value: TrackSizing<'ast>) {
        let mut data = ast.data(self);
        data.rows = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for GridTemplate<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for GridTemplate<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_grid_template(
        &mut self,
        data: GridTemplateData<'ast>,
        span: Span,
    ) -> GridTemplate<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Grid<'ast>(pub(crate) NodeId<'ast, GridData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridData<'ast> {
    pub areas: GridTemplateAreas<'ast>,
    pub auto_columns: &'ast [TrackSize<'ast>],
    pub auto_flow: GridAutoFlow<'ast>,
    pub auto_rows: &'ast [TrackSize<'ast>],
    pub columns: TrackSizing<'ast>,
    pub rows: TrackSizing<'ast>,
}
impl<'ast> NodeStorage<'ast> for Grid<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRID;
    type Data = GridData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GridData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GridData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Grid<'ast> {
    #[inline]
    pub fn areas(self, ast: &Ast<'ast>) -> GridTemplateAreas<'ast> {
        ast.data(self).areas
    }
    #[inline]
    pub fn set_areas(self, ast: &mut Ast<'ast>, value: GridTemplateAreas<'ast>) {
        let mut data = ast.data(self);
        data.areas = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn auto_columns(self, ast: &Ast<'ast>) -> &'ast [TrackSize<'ast>] {
        ast.data(self).auto_columns
    }
    #[inline]
    pub fn set_auto_columns(self, ast: &mut Ast<'ast>, value: &'ast [TrackSize<'ast>]) {
        let mut data = ast.data(self);
        data.auto_columns = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn auto_flow(self, ast: &Ast<'ast>) -> GridAutoFlow<'ast> {
        ast.data(self).auto_flow
    }
    #[inline]
    pub fn set_auto_flow(self, ast: &mut Ast<'ast>, value: GridAutoFlow<'ast>) {
        let mut data = ast.data(self);
        data.auto_flow = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn auto_rows(self, ast: &Ast<'ast>) -> &'ast [TrackSize<'ast>] {
        ast.data(self).auto_rows
    }
    #[inline]
    pub fn set_auto_rows(self, ast: &mut Ast<'ast>, value: &'ast [TrackSize<'ast>]) {
        let mut data = ast.data(self);
        data.auto_rows = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn columns(self, ast: &Ast<'ast>) -> TrackSizing<'ast> {
        ast.data(self).columns
    }
    #[inline]
    pub fn set_columns(self, ast: &mut Ast<'ast>, value: TrackSizing<'ast>) {
        let mut data = ast.data(self);
        data.columns = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn rows(self, ast: &Ast<'ast>) -> TrackSizing<'ast> {
        ast.data(self).rows
    }
    #[inline]
    pub fn set_rows(self, ast: &mut Ast<'ast>, value: TrackSizing<'ast>) {
        let mut data = ast.data(self);
        data.rows = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Grid<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Grid<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_grid(&mut self, data: GridData<'ast>, span: Span) -> Grid<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GridRow<'ast>(pub(crate) NodeId<'ast, GridRowData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridRowData<'ast> {
    pub end: GridLine<'ast>,
    pub start: GridLine<'ast>,
}
impl<'ast> NodeStorage<'ast> for GridRow<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRID_ROW;
    type Data = GridRowData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GridRowData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GridRowData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> GridRow<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> GridLine<'ast> {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: GridLine<'ast>) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> GridLine<'ast> {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: GridLine<'ast>) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for GridRow<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for GridRow<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_grid_row(&mut self, data: GridRowData<'ast>, span: Span) -> GridRow<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GridColumn<'ast>(pub(crate) NodeId<'ast, GridColumnData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridColumnData<'ast> {
    pub end: GridLine<'ast>,
    pub start: GridLine<'ast>,
}
impl<'ast> NodeStorage<'ast> for GridColumn<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRID_COLUMN;
    type Data = GridColumnData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GridColumnData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GridColumnData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> GridColumn<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> GridLine<'ast> {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: GridLine<'ast>) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> GridLine<'ast> {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: GridLine<'ast>) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for GridColumn<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for GridColumn<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_grid_column(
        &mut self,
        data: GridColumnData<'ast>,
        span: Span,
    ) -> GridColumn<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GridArea<'ast>(pub(crate) NodeId<'ast, GridAreaData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridAreaData<'ast> {
    pub column_end: GridLine<'ast>,
    pub column_start: GridLine<'ast>,
    pub row_end: GridLine<'ast>,
    pub row_start: GridLine<'ast>,
}
impl<'ast> NodeStorage<'ast> for GridArea<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRID_AREA;
    type Data = GridAreaData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GridAreaData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GridAreaData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> GridArea<'ast> {
    #[inline]
    pub fn column_end(self, ast: &Ast<'ast>) -> GridLine<'ast> {
        ast.data(self).column_end
    }
    #[inline]
    pub fn set_column_end(self, ast: &mut Ast<'ast>, value: GridLine<'ast>) {
        let mut data = ast.data(self);
        data.column_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn column_start(self, ast: &Ast<'ast>) -> GridLine<'ast> {
        ast.data(self).column_start
    }
    #[inline]
    pub fn set_column_start(self, ast: &mut Ast<'ast>, value: GridLine<'ast>) {
        let mut data = ast.data(self);
        data.column_start = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn row_end(self, ast: &Ast<'ast>) -> GridLine<'ast> {
        ast.data(self).row_end
    }
    #[inline]
    pub fn set_row_end(self, ast: &mut Ast<'ast>, value: GridLine<'ast>) {
        let mut data = ast.data(self);
        data.row_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn row_start(self, ast: &Ast<'ast>) -> GridLine<'ast> {
        ast.data(self).row_start
    }
    #[inline]
    pub fn set_row_start(self, ast: &mut Ast<'ast>, value: GridLine<'ast>) {
        let mut data = ast.data(self);
        data.row_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for GridArea<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for GridArea<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_grid_area(&mut self, data: GridAreaData<'ast>, span: Span) -> GridArea<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MarginBlock<'ast>(pub(crate) NodeId<'ast, MarginBlockData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarginBlockData<'ast> {
    pub block_end: LengthPercentageOrAuto<'ast>,
    pub block_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for MarginBlock<'ast> {
    const KIND: NodeKind = super::kind::KIND_MARGIN_BLOCK;
    type Data = MarginBlockData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MarginBlockData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MarginBlockData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MarginBlock<'ast> {
    #[inline]
    pub fn block_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_end
    }
    #[inline]
    pub fn set_block_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn block_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_start
    }
    #[inline]
    pub fn set_block_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for MarginBlock<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MarginBlock<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_margin_block(
        &mut self,
        data: MarginBlockData<'ast>,
        span: Span,
    ) -> MarginBlock<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MarginInline<'ast>(pub(crate) NodeId<'ast, MarginInlineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarginInlineData<'ast> {
    pub inline_end: LengthPercentageOrAuto<'ast>,
    pub inline_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for MarginInline<'ast> {
    const KIND: NodeKind = super::kind::KIND_MARGIN_INLINE;
    type Data = MarginInlineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MarginInlineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MarginInlineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MarginInline<'ast> {
    #[inline]
    pub fn inline_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_end
    }
    #[inline]
    pub fn set_inline_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn inline_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_start
    }
    #[inline]
    pub fn set_inline_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for MarginInline<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MarginInline<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_margin_inline(
        &mut self,
        data: MarginInlineData<'ast>,
        span: Span,
    ) -> MarginInline<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Margin<'ast>(pub(crate) NodeId<'ast, MarginData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarginData<'ast> {
    pub bottom: LengthPercentageOrAuto<'ast>,
    pub left: LengthPercentageOrAuto<'ast>,
    pub right: LengthPercentageOrAuto<'ast>,
    pub top: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for Margin<'ast> {
    const KIND: NodeKind = super::kind::KIND_MARGIN;
    type Data = MarginData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MarginData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MarginData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Margin<'ast> {
    #[inline]
    pub fn bottom(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).bottom
    }
    #[inline]
    pub fn set_bottom(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.bottom = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn left(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).left
    }
    #[inline]
    pub fn set_left(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn right(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).right
    }
    #[inline]
    pub fn set_right(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).top
    }
    #[inline]
    pub fn set_top(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.top = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Margin<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Margin<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_margin(&mut self, data: MarginData<'ast>, span: Span) -> Margin<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PaddingBlock<'ast>(pub(crate) NodeId<'ast, PaddingBlockData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaddingBlockData<'ast> {
    pub block_end: LengthPercentageOrAuto<'ast>,
    pub block_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for PaddingBlock<'ast> {
    const KIND: NodeKind = super::kind::KIND_PADDING_BLOCK;
    type Data = PaddingBlockData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PaddingBlockData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PaddingBlockData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PaddingBlock<'ast> {
    #[inline]
    pub fn block_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_end
    }
    #[inline]
    pub fn set_block_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn block_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_start
    }
    #[inline]
    pub fn set_block_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PaddingBlock<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PaddingBlock<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_padding_block(
        &mut self,
        data: PaddingBlockData<'ast>,
        span: Span,
    ) -> PaddingBlock<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PaddingInline<'ast>(pub(crate) NodeId<'ast, PaddingInlineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaddingInlineData<'ast> {
    pub inline_end: LengthPercentageOrAuto<'ast>,
    pub inline_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for PaddingInline<'ast> {
    const KIND: NodeKind = super::kind::KIND_PADDING_INLINE;
    type Data = PaddingInlineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PaddingInlineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PaddingInlineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PaddingInline<'ast> {
    #[inline]
    pub fn inline_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_end
    }
    #[inline]
    pub fn set_inline_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn inline_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_start
    }
    #[inline]
    pub fn set_inline_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PaddingInline<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PaddingInline<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_padding_inline(
        &mut self,
        data: PaddingInlineData<'ast>,
        span: Span,
    ) -> PaddingInline<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Padding<'ast>(pub(crate) NodeId<'ast, PaddingData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaddingData<'ast> {
    pub bottom: LengthPercentageOrAuto<'ast>,
    pub left: LengthPercentageOrAuto<'ast>,
    pub right: LengthPercentageOrAuto<'ast>,
    pub top: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for Padding<'ast> {
    const KIND: NodeKind = super::kind::KIND_PADDING;
    type Data = PaddingData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PaddingData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PaddingData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Padding<'ast> {
    #[inline]
    pub fn bottom(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).bottom
    }
    #[inline]
    pub fn set_bottom(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.bottom = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn left(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).left
    }
    #[inline]
    pub fn set_left(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn right(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).right
    }
    #[inline]
    pub fn set_right(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).top
    }
    #[inline]
    pub fn set_top(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.top = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Padding<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Padding<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_padding(&mut self, data: PaddingData<'ast>, span: Span) -> Padding<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollMarginBlock<'ast>(pub(crate) NodeId<'ast, ScrollMarginBlockData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollMarginBlockData<'ast> {
    pub block_end: LengthPercentageOrAuto<'ast>,
    pub block_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for ScrollMarginBlock<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLL_MARGIN_BLOCK;
    type Data = ScrollMarginBlockData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollMarginBlockData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollMarginBlockData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollMarginBlock<'ast> {
    #[inline]
    pub fn block_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_end
    }
    #[inline]
    pub fn set_block_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn block_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_start
    }
    #[inline]
    pub fn set_block_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ScrollMarginBlock<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollMarginBlock<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scroll_margin_block(
        &mut self,
        data: ScrollMarginBlockData<'ast>,
        span: Span,
    ) -> ScrollMarginBlock<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollMarginInline<'ast>(pub(crate) NodeId<'ast, ScrollMarginInlineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollMarginInlineData<'ast> {
    pub inline_end: LengthPercentageOrAuto<'ast>,
    pub inline_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for ScrollMarginInline<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLL_MARGIN_INLINE;
    type Data = ScrollMarginInlineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollMarginInlineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollMarginInlineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollMarginInline<'ast> {
    #[inline]
    pub fn inline_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_end
    }
    #[inline]
    pub fn set_inline_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn inline_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_start
    }
    #[inline]
    pub fn set_inline_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ScrollMarginInline<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollMarginInline<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scroll_margin_inline(
        &mut self,
        data: ScrollMarginInlineData<'ast>,
        span: Span,
    ) -> ScrollMarginInline<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollMargin<'ast>(pub(crate) NodeId<'ast, ScrollMarginData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollMarginData<'ast> {
    pub bottom: LengthPercentageOrAuto<'ast>,
    pub left: LengthPercentageOrAuto<'ast>,
    pub right: LengthPercentageOrAuto<'ast>,
    pub top: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for ScrollMargin<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLL_MARGIN;
    type Data = ScrollMarginData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollMarginData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollMarginData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollMargin<'ast> {
    #[inline]
    pub fn bottom(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).bottom
    }
    #[inline]
    pub fn set_bottom(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.bottom = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn left(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).left
    }
    #[inline]
    pub fn set_left(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn right(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).right
    }
    #[inline]
    pub fn set_right(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).top
    }
    #[inline]
    pub fn set_top(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.top = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ScrollMargin<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollMargin<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scroll_margin(
        &mut self,
        data: ScrollMarginData<'ast>,
        span: Span,
    ) -> ScrollMargin<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollPaddingBlock<'ast>(pub(crate) NodeId<'ast, ScrollPaddingBlockData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollPaddingBlockData<'ast> {
    pub block_end: LengthPercentageOrAuto<'ast>,
    pub block_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for ScrollPaddingBlock<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLL_PADDING_BLOCK;
    type Data = ScrollPaddingBlockData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollPaddingBlockData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollPaddingBlockData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollPaddingBlock<'ast> {
    #[inline]
    pub fn block_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_end
    }
    #[inline]
    pub fn set_block_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn block_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).block_start
    }
    #[inline]
    pub fn set_block_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.block_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ScrollPaddingBlock<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollPaddingBlock<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scroll_padding_block(
        &mut self,
        data: ScrollPaddingBlockData<'ast>,
        span: Span,
    ) -> ScrollPaddingBlock<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollPaddingInline<'ast>(pub(crate) NodeId<'ast, ScrollPaddingInlineData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollPaddingInlineData<'ast> {
    pub inline_end: LengthPercentageOrAuto<'ast>,
    pub inline_start: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for ScrollPaddingInline<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLL_PADDING_INLINE;
    type Data = ScrollPaddingInlineData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollPaddingInlineData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollPaddingInlineData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollPaddingInline<'ast> {
    #[inline]
    pub fn inline_end(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_end
    }
    #[inline]
    pub fn set_inline_end(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn inline_start(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).inline_start
    }
    #[inline]
    pub fn set_inline_start(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.inline_start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ScrollPaddingInline<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollPaddingInline<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scroll_padding_inline(
        &mut self,
        data: ScrollPaddingInlineData<'ast>,
        span: Span,
    ) -> ScrollPaddingInline<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollPadding<'ast>(pub(crate) NodeId<'ast, ScrollPaddingData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollPaddingData<'ast> {
    pub bottom: LengthPercentageOrAuto<'ast>,
    pub left: LengthPercentageOrAuto<'ast>,
    pub right: LengthPercentageOrAuto<'ast>,
    pub top: LengthPercentageOrAuto<'ast>,
}
impl<'ast> NodeStorage<'ast> for ScrollPadding<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLL_PADDING;
    type Data = ScrollPaddingData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollPaddingData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollPaddingData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollPadding<'ast> {
    #[inline]
    pub fn bottom(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).bottom
    }
    #[inline]
    pub fn set_bottom(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.bottom = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn left(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).left
    }
    #[inline]
    pub fn set_left(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.left = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn right(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).right
    }
    #[inline]
    pub fn set_right(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.right = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn top(self, ast: &Ast<'ast>) -> LengthPercentageOrAuto<'ast> {
        ast.data(self).top
    }
    #[inline]
    pub fn set_top(self, ast: &mut Ast<'ast>, value: LengthPercentageOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.top = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ScrollPadding<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollPadding<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scroll_padding(
        &mut self,
        data: ScrollPaddingData<'ast>,
        span: Span,
    ) -> ScrollPadding<'ast> {
        self.alloc(data, span)
    }
}
