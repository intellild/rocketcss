use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Cursor<'ast>(pub(crate) NodeId<'ast, CursorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CursorData<'ast> {
    pub images: &'ast [CursorImage<'ast>],
    pub keyword: CursorKeyword,
}
impl<'ast> NodeStorage<'ast> for Cursor<'ast> {
    const KIND: NodeKind = super::kind::KIND_CURSOR;
    type Data = CursorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CursorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CursorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Cursor<'ast> {
    #[inline]
    pub fn images(self, ast: &Ast<'ast>) -> &'ast [CursorImage<'ast>] {
        ast.data(self).images
    }
    #[inline]
    pub fn set_images(self, ast: &mut Ast<'ast>, value: &'ast [CursorImage<'ast>]) {
        let mut data = ast.data(self);
        data.images = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn keyword(self, ast: &Ast<'ast>) -> CursorKeyword {
        ast.data(self).keyword
    }
    #[inline]
    pub fn set_keyword(self, ast: &mut Ast<'ast>, value: CursorKeyword) {
        let mut data = ast.data(self);
        data.keyword = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Cursor<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Cursor<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_cursor(&mut self, data: CursorData<'ast>, span: Span) -> Cursor<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CursorImage<'ast>(pub(crate) NodeId<'ast, CursorImageData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CursorImageData<'ast> {
    pub hotspot: Option<(f32, f32)>,
    pub url: Url<'ast>,
}
impl<'ast> NodeStorage<'ast> for CursorImage<'ast> {
    const KIND: NodeKind = super::kind::KIND_CURSOR_IMAGE;
    type Data = CursorImageData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CursorImageData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CursorImageData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CursorImage<'ast> {
    #[inline]
    pub fn hotspot(self, ast: &Ast<'ast>) -> Option<(f32, f32)> {
        ast.data(self).hotspot
    }
    #[inline]
    pub fn set_hotspot(self, ast: &mut Ast<'ast>, value: Option<(f32, f32)>) {
        let mut data = ast.data(self);
        data.hotspot = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn url(self, ast: &Ast<'ast>) -> Url<'ast> {
        ast.data(self).url
    }
    #[inline]
    pub fn set_url(self, ast: &mut Ast<'ast>, value: Url<'ast>) {
        let mut data = ast.data(self);
        data.url = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for CursorImage<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CursorImage<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_cursor_image(
        &mut self,
        data: CursorImageData<'ast>,
        span: Span,
    ) -> CursorImage<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Caret<'ast>(pub(crate) NodeId<'ast, CaretData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CaretData<'ast> {
    pub color: ColorOrAuto<'ast>,
    pub shape: CaretShape,
}
impl<'ast> NodeStorage<'ast> for Caret<'ast> {
    const KIND: NodeKind = super::kind::KIND_CARET;
    type Data = CaretData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CaretData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CaretData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Caret<'ast> {
    #[inline]
    pub fn color(self, ast: &Ast<'ast>) -> ColorOrAuto<'ast> {
        ast.data(self).color
    }
    #[inline]
    pub fn set_color(self, ast: &mut Ast<'ast>, value: ColorOrAuto<'ast>) {
        let mut data = ast.data(self);
        data.color = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn shape(self, ast: &Ast<'ast>) -> CaretShape {
        ast.data(self).shape
    }
    #[inline]
    pub fn set_shape(self, ast: &mut Ast<'ast>, value: CaretShape) {
        let mut data = ast.data(self);
        data.shape = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Caret<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Caret<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_caret(&mut self, data: CaretData<'ast>, span: Span) -> Caret<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ListStyle<'ast>(pub(crate) NodeId<'ast, ListStyleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ListStyleData<'ast> {
    pub image: Image<'ast>,
    pub list_style_type: ListStyleType<'ast>,
    pub position: ListStylePosition,
}
impl<'ast> NodeStorage<'ast> for ListStyle<'ast> {
    const KIND: NodeKind = super::kind::KIND_LIST_STYLE;
    type Data = ListStyleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ListStyleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ListStyleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ListStyle<'ast> {
    #[inline]
    pub fn image(self, ast: &Ast<'ast>) -> Image<'ast> {
        ast.data(self).image
    }
    #[inline]
    pub fn set_image(self, ast: &mut Ast<'ast>, value: Image<'ast>) {
        let mut data = ast.data(self);
        data.image = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn list_style_type(self, ast: &Ast<'ast>) -> ListStyleType<'ast> {
        ast.data(self).list_style_type
    }
    #[inline]
    pub fn set_list_style_type(self, ast: &mut Ast<'ast>, value: ListStyleType<'ast>) {
        let mut data = ast.data(self);
        data.list_style_type = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn position(self, ast: &Ast<'ast>) -> ListStylePosition {
        ast.data(self).position
    }
    #[inline]
    pub fn set_position(self, ast: &mut Ast<'ast>, value: ListStylePosition) {
        let mut data = ast.data(self);
        data.position = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ListStyle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ListStyle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_list_style(&mut self, data: ListStyleData<'ast>, span: Span) -> ListStyle<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Composes<'ast>(pub(crate) NodeId<'ast, ComposesData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComposesData<'ast> {
    pub from: Option<Specifier<'ast>>,
    pub names: &'ast [&'ast str],
}
impl<'ast> NodeStorage<'ast> for Composes<'ast> {
    const KIND: NodeKind = super::kind::KIND_COMPOSES;
    type Data = ComposesData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ComposesData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ComposesData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Composes<'ast> {
    #[inline]
    pub fn from(self, ast: &Ast<'ast>) -> Option<Specifier<'ast>> {
        ast.data(self).from
    }
    #[inline]
    pub fn set_from(self, ast: &mut Ast<'ast>, value: Option<Specifier<'ast>>) {
        let mut data = ast.data(self);
        data.from = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn names(self, ast: &Ast<'ast>) -> &'ast [&'ast str] {
        ast.data(self).names
    }
    #[inline]
    pub fn set_names(self, ast: &mut Ast<'ast>, value: &'ast [&'ast str]) {
        let mut data = ast.data(self);
        data.names = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Composes<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Composes<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_composes(&mut self, data: ComposesData<'ast>, span: Span) -> Composes<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ColorScheme<'ast>(pub(crate) NodeId<'ast, ColorSchemeData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSchemeData {
    pub dark: bool,
    pub light: bool,
    pub only: bool,
}
impl<'ast> NodeStorage<'ast> for ColorScheme<'ast> {
    const KIND: NodeKind = super::kind::KIND_COLOR_SCHEME;
    type Data = ColorSchemeData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ColorSchemeData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ColorSchemeData>) -> Self {
        Self(id)
    }
}
impl<'ast> ColorScheme<'ast> {
    #[inline]
    pub fn dark(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).dark
    }
    #[inline]
    pub fn set_dark(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.dark = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn light(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).light
    }
    #[inline]
    pub fn set_light(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.light = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn only(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).only
    }
    #[inline]
    pub fn set_only(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.only = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ColorScheme<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ColorScheme<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_color_scheme(&mut self, data: ColorSchemeData, span: Span) -> ColorScheme<'ast> {
        self.alloc(data, span)
    }
}
