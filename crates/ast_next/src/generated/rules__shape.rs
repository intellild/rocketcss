use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InsetRect<'ast>(pub(crate) NodeId<'ast, InsetRectData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InsetRectData<'ast> {
    pub radius: BorderRadius<'ast>,
    pub rect: Rect<'ast, LengthPercentage<'ast>>,
}
impl<'ast> NodeStorage<'ast> for InsetRect<'ast> {
    const KIND: NodeKind = super::kind::KIND_INSET_RECT;
    type Data = InsetRectData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, InsetRectData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, InsetRectData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> InsetRect<'ast> {
    #[inline]
    pub fn radius(self, ast: &Ast<'ast>) -> BorderRadius<'ast> {
        ast.data(self).radius
    }
    #[inline]
    pub fn set_radius(self, ast: &mut Ast<'ast>, value: BorderRadius<'ast>) {
        let mut data = ast.data(self);
        data.radius = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn rect(self, ast: &Ast<'ast>) -> Rect<'ast, LengthPercentage<'ast>> {
        ast.data(self).rect
    }
    #[inline]
    pub fn set_rect(self, ast: &mut Ast<'ast>, value: Rect<'ast, LengthPercentage<'ast>>) {
        let mut data = ast.data(self);
        data.rect = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for InsetRect<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for InsetRect<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_inset_rect(&mut self, data: InsetRectData<'ast>, span: Span) -> InsetRect<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CircleShape<'ast>(pub(crate) NodeId<'ast, CircleShapeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleShapeData<'ast> {
    pub position: Position<'ast>,
    pub radius: ShapeRadius<'ast>,
}
impl<'ast> NodeStorage<'ast> for CircleShape<'ast> {
    const KIND: NodeKind = super::kind::KIND_CIRCLE_SHAPE;
    type Data = CircleShapeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CircleShapeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CircleShapeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CircleShape<'ast> {
    #[inline]
    pub fn position(self, ast: &Ast<'ast>) -> Position<'ast> {
        ast.data(self).position
    }
    #[inline]
    pub fn set_position(self, ast: &mut Ast<'ast>, value: Position<'ast>) {
        let mut data = ast.data(self);
        data.position = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn radius(self, ast: &Ast<'ast>) -> ShapeRadius<'ast> {
        ast.data(self).radius
    }
    #[inline]
    pub fn set_radius(self, ast: &mut Ast<'ast>, value: ShapeRadius<'ast>) {
        let mut data = ast.data(self);
        data.radius = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for CircleShape<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CircleShape<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_circle_shape(
        &mut self,
        data: CircleShapeData<'ast>,
        span: Span,
    ) -> CircleShape<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EllipseShape<'ast>(pub(crate) NodeId<'ast, EllipseShapeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EllipseShapeData<'ast> {
    pub position: Position<'ast>,
    pub radius_x: ShapeRadius<'ast>,
    pub radius_y: ShapeRadius<'ast>,
}
impl<'ast> NodeStorage<'ast> for EllipseShape<'ast> {
    const KIND: NodeKind = super::kind::KIND_ELLIPSE_SHAPE;
    type Data = EllipseShapeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, EllipseShapeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, EllipseShapeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> EllipseShape<'ast> {
    #[inline]
    pub fn position(self, ast: &Ast<'ast>) -> Position<'ast> {
        ast.data(self).position
    }
    #[inline]
    pub fn set_position(self, ast: &mut Ast<'ast>, value: Position<'ast>) {
        let mut data = ast.data(self);
        data.position = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn radius_x(self, ast: &Ast<'ast>) -> ShapeRadius<'ast> {
        ast.data(self).radius_x
    }
    #[inline]
    pub fn set_radius_x(self, ast: &mut Ast<'ast>, value: ShapeRadius<'ast>) {
        let mut data = ast.data(self);
        data.radius_x = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn radius_y(self, ast: &Ast<'ast>) -> ShapeRadius<'ast> {
        ast.data(self).radius_y
    }
    #[inline]
    pub fn set_radius_y(self, ast: &mut Ast<'ast>, value: ShapeRadius<'ast>) {
        let mut data = ast.data(self);
        data.radius_y = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for EllipseShape<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for EllipseShape<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_ellipse_shape(
        &mut self,
        data: EllipseShapeData<'ast>,
        span: Span,
    ) -> EllipseShape<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Polygon<'ast>(pub(crate) NodeId<'ast, PolygonData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PolygonData<'ast> {
    pub fill_rule: FillRule,
    pub points: &'ast [Point<'ast>],
}
impl<'ast> NodeStorage<'ast> for Polygon<'ast> {
    const KIND: NodeKind = super::kind::KIND_POLYGON;
    type Data = PolygonData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PolygonData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PolygonData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Polygon<'ast> {
    #[inline]
    pub fn fill_rule(self, ast: &Ast<'ast>) -> FillRule {
        ast.data(self).fill_rule
    }
    #[inline]
    pub fn set_fill_rule(self, ast: &mut Ast<'ast>, value: FillRule) {
        let mut data = ast.data(self);
        data.fill_rule = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn points(self, ast: &Ast<'ast>) -> &'ast [Point<'ast>] {
        ast.data(self).points
    }
    #[inline]
    pub fn set_points(self, ast: &mut Ast<'ast>, value: &'ast [Point<'ast>]) {
        let mut data = ast.data(self);
        data.points = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Polygon<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Polygon<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_polygon(&mut self, data: PolygonData<'ast>, span: Span) -> Polygon<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Point<'ast>(pub(crate) NodeId<'ast, PointData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointData<'ast> {
    pub x: LengthPercentage<'ast>,
    pub y: LengthPercentage<'ast>,
}
impl<'ast> NodeStorage<'ast> for Point<'ast> {
    const KIND: NodeKind = super::kind::KIND_POINT;
    type Data = PointData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PointData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PointData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Point<'ast> {
    #[inline]
    pub fn x(self, ast: &Ast<'ast>) -> LengthPercentage<'ast> {
        ast.data(self).x
    }
    #[inline]
    pub fn set_x(self, ast: &mut Ast<'ast>, value: LengthPercentage<'ast>) {
        let mut data = ast.data(self);
        data.x = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y(self, ast: &Ast<'ast>) -> LengthPercentage<'ast> {
        ast.data(self).y
    }
    #[inline]
    pub fn set_y(self, ast: &mut Ast<'ast>, value: LengthPercentage<'ast>) {
        let mut data = ast.data(self);
        data.y = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Point<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Point<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_point(&mut self, data: PointData<'ast>, span: Span) -> Point<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Mask<'ast>(pub(crate) NodeId<'ast, MaskData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaskData<'ast> {
    pub clip: MaskClip,
    pub composite: MaskComposite,
    pub image: Image<'ast>,
    pub mode: MaskMode,
    pub origin: GeometryBox,
    pub position: Position<'ast>,
    pub repeat: BackgroundRepeat<'ast>,
    pub size: BackgroundSize<'ast>,
}
impl<'ast> NodeStorage<'ast> for Mask<'ast> {
    const KIND: NodeKind = super::kind::KIND_MASK;
    type Data = MaskData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MaskData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MaskData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Mask<'ast> {
    #[inline]
    pub fn clip(self, ast: &Ast<'ast>) -> MaskClip {
        ast.data(self).clip
    }
    #[inline]
    pub fn set_clip(self, ast: &mut Ast<'ast>, value: MaskClip) {
        let mut data = ast.data(self);
        data.clip = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn composite(self, ast: &Ast<'ast>) -> MaskComposite {
        ast.data(self).composite
    }
    #[inline]
    pub fn set_composite(self, ast: &mut Ast<'ast>, value: MaskComposite) {
        let mut data = ast.data(self);
        data.composite = value;
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
    pub fn mode(self, ast: &Ast<'ast>) -> MaskMode {
        ast.data(self).mode
    }
    #[inline]
    pub fn set_mode(self, ast: &mut Ast<'ast>, value: MaskMode) {
        let mut data = ast.data(self);
        data.mode = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn origin(self, ast: &Ast<'ast>) -> GeometryBox {
        ast.data(self).origin
    }
    #[inline]
    pub fn set_origin(self, ast: &mut Ast<'ast>, value: GeometryBox) {
        let mut data = ast.data(self);
        data.origin = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn position(self, ast: &Ast<'ast>) -> Position<'ast> {
        ast.data(self).position
    }
    #[inline]
    pub fn set_position(self, ast: &mut Ast<'ast>, value: Position<'ast>) {
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
impl<'ast> GetSpan<'ast> for Mask<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Mask<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_mask(&mut self, data: MaskData<'ast>, span: Span) -> Mask<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MaskBorder<'ast>(pub(crate) NodeId<'ast, MaskBorderData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaskBorderData<'ast> {
    pub mode: MaskBorderMode,
    pub outset: Rect<'ast, LengthOrNumber<'ast>>,
    pub repeat: BorderImageRepeat<'ast>,
    pub slice: BorderImageSlice<'ast>,
    pub source: Image<'ast>,
    pub width: Rect<'ast, BorderImageSideWidth<'ast>>,
}
impl<'ast> NodeStorage<'ast> for MaskBorder<'ast> {
    const KIND: NodeKind = super::kind::KIND_MASK_BORDER;
    type Data = MaskBorderData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MaskBorderData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MaskBorderData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MaskBorder<'ast> {
    #[inline]
    pub fn mode(self, ast: &Ast<'ast>) -> MaskBorderMode {
        ast.data(self).mode
    }
    #[inline]
    pub fn set_mode(self, ast: &mut Ast<'ast>, value: MaskBorderMode) {
        let mut data = ast.data(self);
        data.mode = value;
        ast.set_data(self, data);
    }
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
impl<'ast> GetSpan<'ast> for MaskBorder<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MaskBorder<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_mask_border(
        &mut self,
        data: MaskBorderData<'ast>,
        span: Span,
    ) -> MaskBorder<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DropShadow<'ast>(pub(crate) NodeId<'ast, DropShadowData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropShadowData<'ast> {
    pub blur: Length<'ast>,
    pub color: CssColor<'ast>,
    pub x_offset: Length<'ast>,
    pub y_offset: Length<'ast>,
}
impl<'ast> NodeStorage<'ast> for DropShadow<'ast> {
    const KIND: NodeKind = super::kind::KIND_DROP_SHADOW;
    type Data = DropShadowData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, DropShadowData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, DropShadowData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> DropShadow<'ast> {
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
impl<'ast> GetSpan<'ast> for DropShadow<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for DropShadow<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_drop_shadow(
        &mut self,
        data: DropShadowData<'ast>,
        span: Span,
    ) -> DropShadow<'ast> {
        self.alloc(data, span)
    }
}
