use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Position<'ast>(pub(crate) NodeId<'ast, PositionData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionData<'ast> {
    pub x: PositionComponent<'ast, HorizontalPositionKeyword>,
    pub y: PositionComponent<'ast, VerticalPositionKeyword>,
}
impl<'ast> NodeStorage<'ast> for Position<'ast> {
    const KIND: NodeKind = super::kind::KIND_POSITION;
    type Data = PositionData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PositionData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PositionData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Position<'ast> {
    #[inline]
    pub fn x(self, ast: &Ast<'ast>) -> PositionComponent<'ast, HorizontalPositionKeyword> {
        ast.data(self).x
    }
    #[inline]
    pub fn set_x(
        self,
        ast: &mut Ast<'ast>,
        value: PositionComponent<'ast, HorizontalPositionKeyword>,
    ) {
        let mut data = ast.data(self);
        data.x = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y(self, ast: &Ast<'ast>) -> PositionComponent<'ast, VerticalPositionKeyword> {
        ast.data(self).y
    }
    #[inline]
    pub fn set_y(
        self,
        ast: &mut Ast<'ast>,
        value: PositionComponent<'ast, VerticalPositionKeyword>,
    ) {
        let mut data = ast.data(self);
        data.y = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Position<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Position<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_position(&mut self, data: PositionData<'ast>, span: Span) -> Position<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WebKitGradientPoint<'ast>(pub(crate) NodeId<'ast, WebKitGradientPointData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WebKitGradientPointData {
    pub x: WebKitGradientPointComponent<HorizontalPositionKeyword>,
    pub y: WebKitGradientPointComponent<VerticalPositionKeyword>,
}
impl<'ast> NodeStorage<'ast> for WebKitGradientPoint<'ast> {
    const KIND: NodeKind = super::kind::KIND_WEB_KIT_GRADIENT_POINT;
    type Data = WebKitGradientPointData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, WebKitGradientPointData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, WebKitGradientPointData>) -> Self {
        Self(id)
    }
}
impl<'ast> WebKitGradientPoint<'ast> {
    #[inline]
    pub fn x(self, ast: &Ast<'ast>) -> WebKitGradientPointComponent<HorizontalPositionKeyword> {
        ast.data(self).x
    }
    #[inline]
    pub fn set_x(
        self,
        ast: &mut Ast<'ast>,
        value: WebKitGradientPointComponent<HorizontalPositionKeyword>,
    ) {
        let mut data = ast.data(self);
        data.x = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y(self, ast: &Ast<'ast>) -> WebKitGradientPointComponent<VerticalPositionKeyword> {
        ast.data(self).y
    }
    #[inline]
    pub fn set_y(
        self,
        ast: &mut Ast<'ast>,
        value: WebKitGradientPointComponent<VerticalPositionKeyword>,
    ) {
        let mut data = ast.data(self);
        data.y = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for WebKitGradientPoint<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for WebKitGradientPoint<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_web_kit_gradient_point(
        &mut self,
        data: WebKitGradientPointData,
        span: Span,
    ) -> WebKitGradientPoint<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WebKitColorStop<'ast>(pub(crate) NodeId<'ast, WebKitColorStopData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WebKitColorStopData<'ast> {
    pub color: CssColor<'ast>,
    pub position: f32,
}
impl<'ast> NodeStorage<'ast> for WebKitColorStop<'ast> {
    const KIND: NodeKind = super::kind::KIND_WEB_KIT_COLOR_STOP;
    type Data = WebKitColorStopData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, WebKitColorStopData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, WebKitColorStopData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> WebKitColorStop<'ast> {
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
    pub fn position(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).position
    }
    #[inline]
    pub fn set_position(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.position = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for WebKitColorStop<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for WebKitColorStop<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_web_kit_color_stop(
        &mut self,
        data: WebKitColorStopData<'ast>,
        span: Span,
    ) -> WebKitColorStop<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ImageSet<'ast>(pub(crate) NodeId<'ast, ImageSetData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageSetData<'ast> {
    pub options: &'ast [ImageSetOption<'ast>],
    pub vendor_prefix: VendorPrefix,
}
impl<'ast> NodeStorage<'ast> for ImageSet<'ast> {
    const KIND: NodeKind = super::kind::KIND_IMAGE_SET;
    type Data = ImageSetData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ImageSetData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ImageSetData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ImageSet<'ast> {
    #[inline]
    pub fn options(self, ast: &Ast<'ast>) -> &'ast [ImageSetOption<'ast>] {
        ast.data(self).options
    }
    #[inline]
    pub fn set_options(self, ast: &mut Ast<'ast>, value: &'ast [ImageSetOption<'ast>]) {
        let mut data = ast.data(self);
        data.options = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn vendor_prefix(self, ast: &Ast<'ast>) -> VendorPrefix {
        ast.data(self).vendor_prefix
    }
    #[inline]
    pub fn set_vendor_prefix(self, ast: &mut Ast<'ast>, value: VendorPrefix) {
        let mut data = ast.data(self);
        data.vendor_prefix = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ImageSet<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ImageSet<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_image_set(&mut self, data: ImageSetData<'ast>, span: Span) -> ImageSet<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ImageSetOption<'ast>(pub(crate) NodeId<'ast, ImageSetOptionData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageSetOptionData<'ast> {
    pub file_type: Option<&'ast str>,
    pub image: Image<'ast>,
    pub resolution: Resolution,
}
impl<'ast> NodeStorage<'ast> for ImageSetOption<'ast> {
    const KIND: NodeKind = super::kind::KIND_IMAGE_SET_OPTION;
    type Data = ImageSetOptionData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ImageSetOptionData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ImageSetOptionData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ImageSetOption<'ast> {
    #[inline]
    pub fn file_type(self, ast: &Ast<'ast>) -> Option<&'ast str> {
        ast.data(self).file_type
    }
    #[inline]
    pub fn set_file_type(self, ast: &mut Ast<'ast>, value: Option<&'ast str>) {
        let mut data = ast.data(self);
        data.file_type = value;
        ast.set_data(self, data);
    }
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
    pub fn resolution(self, ast: &Ast<'ast>) -> Resolution {
        ast.data(self).resolution
    }
    #[inline]
    pub fn set_resolution(self, ast: &mut Ast<'ast>, value: Resolution) {
        let mut data = ast.data(self);
        data.resolution = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ImageSetOption<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ImageSetOption<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_image_set_option(
        &mut self,
        data: ImageSetOptionData<'ast>,
        span: Span,
    ) -> ImageSetOption<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BackgroundPosition<'ast>(pub(crate) NodeId<'ast, BackgroundPositionData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BackgroundPositionData<'ast> {
    pub x: PositionComponent<'ast, HorizontalPositionKeyword>,
    pub y: PositionComponent<'ast, VerticalPositionKeyword>,
}
impl<'ast> NodeStorage<'ast> for BackgroundPosition<'ast> {
    const KIND: NodeKind = super::kind::KIND_BACKGROUND_POSITION;
    type Data = BackgroundPositionData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BackgroundPositionData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BackgroundPositionData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BackgroundPosition<'ast> {
    #[inline]
    pub fn x(self, ast: &Ast<'ast>) -> PositionComponent<'ast, HorizontalPositionKeyword> {
        ast.data(self).x
    }
    #[inline]
    pub fn set_x(
        self,
        ast: &mut Ast<'ast>,
        value: PositionComponent<'ast, HorizontalPositionKeyword>,
    ) {
        let mut data = ast.data(self);
        data.x = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y(self, ast: &Ast<'ast>) -> PositionComponent<'ast, VerticalPositionKeyword> {
        ast.data(self).y
    }
    #[inline]
    pub fn set_y(
        self,
        ast: &mut Ast<'ast>,
        value: PositionComponent<'ast, VerticalPositionKeyword>,
    ) {
        let mut data = ast.data(self);
        data.y = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BackgroundPosition<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BackgroundPosition<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_background_position(
        &mut self,
        data: BackgroundPositionData<'ast>,
        span: Span,
    ) -> BackgroundPosition<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BackgroundRepeat<'ast>(pub(crate) NodeId<'ast, BackgroundRepeatData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BackgroundRepeatData {
    pub x: BackgroundRepeatKeyword,
    pub y: BackgroundRepeatKeyword,
}
impl<'ast> NodeStorage<'ast> for BackgroundRepeat<'ast> {
    const KIND: NodeKind = super::kind::KIND_BACKGROUND_REPEAT;
    type Data = BackgroundRepeatData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BackgroundRepeatData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BackgroundRepeatData>) -> Self {
        Self(id)
    }
}
impl<'ast> BackgroundRepeat<'ast> {
    #[inline]
    pub fn x(self, ast: &Ast<'ast>) -> BackgroundRepeatKeyword {
        ast.data(self).x
    }
    #[inline]
    pub fn set_x(self, ast: &mut Ast<'ast>, value: BackgroundRepeatKeyword) {
        let mut data = ast.data(self);
        data.x = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y(self, ast: &Ast<'ast>) -> BackgroundRepeatKeyword {
        ast.data(self).y
    }
    #[inline]
    pub fn set_y(self, ast: &mut Ast<'ast>, value: BackgroundRepeatKeyword) {
        let mut data = ast.data(self);
        data.y = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BackgroundRepeat<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BackgroundRepeat<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_background_repeat(
        &mut self,
        data: BackgroundRepeatData,
        span: Span,
    ) -> BackgroundRepeat<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Background<'ast>(pub(crate) NodeId<'ast, BackgroundData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BackgroundData<'ast> {
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub color: CssColor<'ast>,
    pub image: Image<'ast>,
    pub origin: BackgroundOrigin,
    pub position: BackgroundPosition<'ast>,
    pub repeat: BackgroundRepeat<'ast>,
    pub size: BackgroundSize<'ast>,
}
impl<'ast> NodeStorage<'ast> for Background<'ast> {
    const KIND: NodeKind = super::kind::KIND_BACKGROUND;
    type Data = BackgroundData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BackgroundData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BackgroundData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Background<'ast> {
    #[inline]
    pub fn attachment(self, ast: &Ast<'ast>) -> BackgroundAttachment {
        ast.data(self).attachment
    }
    #[inline]
    pub fn set_attachment(self, ast: &mut Ast<'ast>, value: BackgroundAttachment) {
        let mut data = ast.data(self);
        data.attachment = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn clip(self, ast: &Ast<'ast>) -> BackgroundClip {
        ast.data(self).clip
    }
    #[inline]
    pub fn set_clip(self, ast: &mut Ast<'ast>, value: BackgroundClip) {
        let mut data = ast.data(self);
        data.clip = value;
        ast.set_data(self, data);
    }
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
    pub fn origin(self, ast: &Ast<'ast>) -> BackgroundOrigin {
        ast.data(self).origin
    }
    #[inline]
    pub fn set_origin(self, ast: &mut Ast<'ast>, value: BackgroundOrigin) {
        let mut data = ast.data(self);
        data.origin = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn position(self, ast: &Ast<'ast>) -> BackgroundPosition<'ast> {
        ast.data(self).position
    }
    #[inline]
    pub fn set_position(self, ast: &mut Ast<'ast>, value: BackgroundPosition<'ast>) {
        let mut data = ast.data(self);
        data.position = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn repeat(self, ast: &Ast<'ast>) -> BackgroundRepeat<'ast> {
        ast.data(self).repeat
    }
    #[inline]
    pub fn set_repeat(self, ast: &mut Ast<'ast>, value: BackgroundRepeat<'ast>) {
        let mut data = ast.data(self);
        data.repeat = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn size(self, ast: &Ast<'ast>) -> BackgroundSize<'ast> {
        ast.data(self).size
    }
    #[inline]
    pub fn set_size(self, ast: &mut Ast<'ast>, value: BackgroundSize<'ast>) {
        let mut data = ast.data(self);
        data.size = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Background<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Background<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_background(&mut self, data: BackgroundData<'ast>, span: Span) -> Background<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BoxShadow<'ast>(pub(crate) NodeId<'ast, BoxShadowData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxShadowData<'ast> {
    pub blur: Length<'ast>,
    pub color: CssColor<'ast>,
    pub inset: bool,
    pub spread: Length<'ast>,
    pub x_offset: Length<'ast>,
    pub y_offset: Length<'ast>,
}
impl<'ast> NodeStorage<'ast> for BoxShadow<'ast> {
    const KIND: NodeKind = super::kind::KIND_BOX_SHADOW;
    type Data = BoxShadowData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BoxShadowData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BoxShadowData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BoxShadow<'ast> {
    #[inline]
    pub fn blur(self, ast: &Ast<'ast>) -> Length<'ast> {
        ast.data(self).blur
    }
    #[inline]
    pub fn set_blur(self, ast: &mut Ast<'ast>, value: Length<'ast>) {
        let mut data = ast.data(self);
        data.blur = value;
        ast.set_data(self, data);
    }
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
    pub fn inset(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).inset
    }
    #[inline]
    pub fn set_inset(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.inset = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn spread(self, ast: &Ast<'ast>) -> Length<'ast> {
        ast.data(self).spread
    }
    #[inline]
    pub fn set_spread(self, ast: &mut Ast<'ast>, value: Length<'ast>) {
        let mut data = ast.data(self);
        data.spread = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn x_offset(self, ast: &Ast<'ast>) -> Length<'ast> {
        ast.data(self).x_offset
    }
    #[inline]
    pub fn set_x_offset(self, ast: &mut Ast<'ast>, value: Length<'ast>) {
        let mut data = ast.data(self);
        data.x_offset = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y_offset(self, ast: &Ast<'ast>) -> Length<'ast> {
        ast.data(self).y_offset
    }
    #[inline]
    pub fn set_y_offset(self, ast: &mut Ast<'ast>, value: Length<'ast>) {
        let mut data = ast.data(self);
        data.y_offset = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for BoxShadow<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BoxShadow<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_box_shadow(&mut self, data: BoxShadowData<'ast>, span: Span) -> BoxShadow<'ast> {
        self.alloc(data, span)
    }
}
