use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Image<'ast>(pub(crate) NodeId<'ast, ImageData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageData<'ast> {
    None,
    Url(Url<'ast>),
    Gradient(Gradient<'ast>),
    ImageSet(ImageSet<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImageKind {
    None,
    Url,
    Gradient,
    ImageSet,
}
impl<'ast> NodeStorage<'ast> for Image<'ast> {
    const KIND: NodeKind = super::kind::KIND_IMAGE;
    type Data = ImageData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ImageData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ImageData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Image<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ImageData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ImageKind {
        match ast.data(self) {
            ImageData::None => ImageKind::None,
            ImageData::Url { .. } => ImageKind::Url,
            ImageData::Gradient { .. } => ImageKind::Gradient,
            ImageData::ImageSet { .. } => ImageKind::ImageSet,
        }
    }
}
impl<'ast> GetSpan<'ast> for Image<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Image<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_image(&mut self, data: ImageData<'ast>, span: Span) -> Image<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Gradient<'ast>(pub(crate) NodeId<'ast, GradientData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GradientData<'ast> {
    Linear {
        direction: LineDirection,
        items: &'ast [GradientItem<'ast, LengthValue<'ast>>],
        vendor_prefix: VendorPrefix,
    },
    RepeatingLinear {
        direction: LineDirection,
        items: &'ast [GradientItem<'ast, LengthValue<'ast>>],
        vendor_prefix: VendorPrefix,
    },
    Radial {
        items: &'ast [GradientItem<'ast, LengthValue<'ast>>],
        position: Position<'ast>,
        shape: EndingShape<'ast>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingRadial {
        items: &'ast [GradientItem<'ast, LengthValue<'ast>>],
        position: Position<'ast>,
        shape: EndingShape<'ast>,
        vendor_prefix: VendorPrefix,
    },
    Conic {
        angle: Angle,
        items: &'ast [GradientItem<'ast, Angle>],
        position: Position<'ast>,
    },
    RepeatingConic {
        angle: Angle,
        items: &'ast [GradientItem<'ast, Angle>],
        position: Position<'ast>,
    },
    WebKitGradient(WebKitGradient<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GradientKind {
    Linear,
    RepeatingLinear,
    Radial,
    RepeatingRadial,
    Conic,
    RepeatingConic,
    WebKitGradient,
}
impl<'ast> NodeStorage<'ast> for Gradient<'ast> {
    const KIND: NodeKind = super::kind::KIND_GRADIENT;
    type Data = GradientData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GradientData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GradientData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Gradient<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> GradientData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> GradientKind {
        match ast.data(self) {
            GradientData::Linear { .. } => GradientKind::Linear,
            GradientData::RepeatingLinear { .. } => GradientKind::RepeatingLinear,
            GradientData::Radial { .. } => GradientKind::Radial,
            GradientData::RepeatingRadial { .. } => GradientKind::RepeatingRadial,
            GradientData::Conic { .. } => GradientKind::Conic,
            GradientData::RepeatingConic { .. } => GradientKind::RepeatingConic,
            GradientData::WebKitGradient { .. } => GradientKind::WebKitGradient,
        }
    }
}
impl<'ast> GetSpan<'ast> for Gradient<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Gradient<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_gradient(&mut self, data: GradientData<'ast>, span: Span) -> Gradient<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WebKitGradient<'ast>(pub(crate) NodeId<'ast, WebKitGradientData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WebKitGradientData<'ast> {
    Linear {
        from: WebKitGradientPoint<'ast>,
        to: WebKitGradientPoint<'ast>,
        stops: &'ast [WebKitColorStop<'ast>],
    },
    Radial {
        from: WebKitGradientPoint<'ast>,
        start_radius: f32,
        to: WebKitGradientPoint<'ast>,
        end_radius: f32,
        stops: &'ast [WebKitColorStop<'ast>],
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WebKitGradientKind {
    Linear,
    Radial,
}
impl<'ast> NodeStorage<'ast> for WebKitGradient<'ast> {
    const KIND: NodeKind = super::kind::KIND_WEB_KIT_GRADIENT;
    type Data = WebKitGradientData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, WebKitGradientData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, WebKitGradientData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> WebKitGradient<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> WebKitGradientData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> WebKitGradientKind {
        match ast.data(self) {
            WebKitGradientData::Linear { .. } => WebKitGradientKind::Linear,
            WebKitGradientData::Radial { .. } => WebKitGradientKind::Radial,
        }
    }
}
impl<'ast> GetSpan<'ast> for WebKitGradient<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for WebKitGradient<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_web_kit_gradient(
        &mut self,
        data: WebKitGradientData<'ast>,
        span: Span,
    ) -> WebKitGradient<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineDirection {
    Angle(Angle),
    Horizontal(HorizontalPositionKeyword),
    Vertical(VerticalPositionKeyword),
    Corner {
        horizontal: HorizontalPositionKeyword,
        vertical: VerticalPositionKeyword,
    },
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum VerticalPositionKeyword {
    Top,
    Bottom,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GradientItem<'ast, D>(pub(crate) NodeId<'ast, GradientItemData<'ast, D>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GradientItemData<'ast, D> {
    ColorStop {
        color: CssColor<'ast>,
        position: Option<DimensionPercentage<'ast, D>>,
    },
    Hint(DimensionPercentage<'ast, D>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GradientItemKind {
    ColorStop,
    Hint,
}
impl<'ast, D> NodeStorage<'ast> for GradientItem<'ast, D>
where
    D: Copy,
{
    const KIND: NodeKind = super::kind::KIND_GRADIENT_ITEM;
    type Data = GradientItemData<'ast, D>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GradientItemData<'ast, D>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GradientItemData<'ast, D>>) -> Self {
        Self(id)
    }
}
impl<'ast, D> GradientItem<'ast, D>
where
    D: Copy,
{
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> GradientItemData<'ast, D> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> GradientItemKind {
        match ast.data(self) {
            GradientItemData::ColorStop { .. } => GradientItemKind::ColorStop,
            GradientItemData::Hint { .. } => GradientItemKind::Hint,
        }
    }
}
impl<'ast, D> GetSpan<'ast> for GradientItem<'ast, D>
where
    D: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, D> SetSpan<'ast> for GradientItem<'ast, D>
where
    D: Copy,
{
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_gradient_item<D>(
        &mut self,
        data: GradientItemData<'ast, D>,
        span: Span,
    ) -> GradientItem<'ast, D>
    where
        D: Copy,
    {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DimensionPercentage<'ast, D>(pub(crate) NodeId<'ast, DimensionPercentageData<'ast, D>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DimensionPercentageData<'ast, D> {
    Dimension(D),
    Percentage(f32),
    Zero,
    Calc(Calc<'ast, DimensionPercentage<'ast, D>>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DimensionPercentageKind {
    Dimension,
    Percentage,
    Zero,
    Calc,
}
impl<'ast, D> NodeStorage<'ast> for DimensionPercentage<'ast, D>
where
    D: Copy,
{
    const KIND: NodeKind = super::kind::KIND_DIMENSION_PERCENTAGE;
    type Data = DimensionPercentageData<'ast, D>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, DimensionPercentageData<'ast, D>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, DimensionPercentageData<'ast, D>>) -> Self {
        Self(id)
    }
}
impl<'ast, D> DimensionPercentage<'ast, D>
where
    D: Copy,
{
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> DimensionPercentageData<'ast, D> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> DimensionPercentageKind {
        match ast.data(self) {
            DimensionPercentageData::Dimension { .. } => DimensionPercentageKind::Dimension,
            DimensionPercentageData::Percentage { .. } => DimensionPercentageKind::Percentage,
            DimensionPercentageData::Zero => DimensionPercentageKind::Zero,
            DimensionPercentageData::Calc { .. } => DimensionPercentageKind::Calc,
        }
    }
}
impl<'ast, D> GetSpan<'ast> for DimensionPercentage<'ast, D>
where
    D: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, D> SetSpan<'ast> for DimensionPercentage<'ast, D>
where
    D: Copy,
{
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_dimension_percentage<D>(
        &mut self,
        data: DimensionPercentageData<'ast, D>,
        span: Span,
    ) -> DimensionPercentage<'ast, D>
    where
        D: Copy,
    {
        self.alloc(data, span)
    }
}
pub type LengthPercentage<'ast> = DimensionPercentage<'ast, LengthValue<'ast>>;
pub type AnglePercentage<'ast> = DimensionPercentage<'ast, Angle>;
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PositionComponent<'ast, S>(pub(crate) NodeId<'ast, PositionComponentData<'ast, S>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositionComponentData<'ast, S> {
    Center,
    Length(LengthPercentage<'ast>),
    Side {
        offset: Option<LengthPercentage<'ast>>,
        side: S,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PositionComponentKind {
    Center,
    Length,
    Side,
}
impl<'ast, S> NodeStorage<'ast> for PositionComponent<'ast, S>
where
    S: Copy,
{
    const KIND: NodeKind = super::kind::KIND_POSITION_COMPONENT;
    type Data = PositionComponentData<'ast, S>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PositionComponentData<'ast, S>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PositionComponentData<'ast, S>>) -> Self {
        Self(id)
    }
}
impl<'ast, S> PositionComponent<'ast, S>
where
    S: Copy,
{
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> PositionComponentData<'ast, S> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> PositionComponentKind {
        match ast.data(self) {
            PositionComponentData::Center => PositionComponentKind::Center,
            PositionComponentData::Length { .. } => PositionComponentKind::Length,
            PositionComponentData::Side { .. } => PositionComponentKind::Side,
        }
    }
}
impl<'ast, S> GetSpan<'ast> for PositionComponent<'ast, S>
where
    S: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, S> SetSpan<'ast> for PositionComponent<'ast, S>
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
    pub fn alloc_position_component<S>(
        &mut self,
        data: PositionComponentData<'ast, S>,
        span: Span,
    ) -> PositionComponent<'ast, S>
    where
        S: Copy,
    {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EndingShape<'ast>(pub(crate) NodeId<'ast, EndingShapeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndingShapeData<'ast> {
    Ellipse(Ellipse<'ast>),
    Circle(Circle<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EndingShapeKind {
    Ellipse,
    Circle,
}
impl<'ast> NodeStorage<'ast> for EndingShape<'ast> {
    const KIND: NodeKind = super::kind::KIND_ENDING_SHAPE;
    type Data = EndingShapeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, EndingShapeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, EndingShapeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> EndingShape<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> EndingShapeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> EndingShapeKind {
        match ast.data(self) {
            EndingShapeData::Ellipse { .. } => EndingShapeKind::Ellipse,
            EndingShapeData::Circle { .. } => EndingShapeKind::Circle,
        }
    }
}
impl<'ast> GetSpan<'ast> for EndingShape<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for EndingShape<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_ending_shape(
        &mut self,
        data: EndingShapeData<'ast>,
        span: Span,
    ) -> EndingShape<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ellipse<'ast>(pub(crate) NodeId<'ast, EllipseData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EllipseData<'ast> {
    Size {
        x: LengthPercentage<'ast>,
        y: LengthPercentage<'ast>,
    },
    Extent(ShapeExtent),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EllipseKind {
    Size,
    Extent,
}
impl<'ast> NodeStorage<'ast> for Ellipse<'ast> {
    const KIND: NodeKind = super::kind::KIND_ELLIPSE;
    type Data = EllipseData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, EllipseData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, EllipseData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Ellipse<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> EllipseData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> EllipseKind {
        match ast.data(self) {
            EllipseData::Size { .. } => EllipseKind::Size,
            EllipseData::Extent { .. } => EllipseKind::Extent,
        }
    }
}
impl<'ast> GetSpan<'ast> for Ellipse<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Ellipse<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_ellipse(&mut self, data: EllipseData<'ast>, span: Span) -> Ellipse<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ShapeExtent {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Circle<'ast>(pub(crate) NodeId<'ast, CircleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CircleData<'ast> {
    Radius(Length<'ast>),
    Extent(ShapeExtent),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CircleKind {
    Radius,
    Extent,
}
impl<'ast> NodeStorage<'ast> for Circle<'ast> {
    const KIND: NodeKind = super::kind::KIND_CIRCLE;
    type Data = CircleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CircleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CircleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Circle<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> CircleData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> CircleKind {
        match ast.data(self) {
            CircleData::Radius { .. } => CircleKind::Radius,
            CircleData::Extent { .. } => CircleKind::Extent,
        }
    }
}
impl<'ast> GetSpan<'ast> for Circle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Circle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_circle(&mut self, data: CircleData<'ast>, span: Span) -> Circle<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WebKitGradientPointComponent<S> {
    Center,
    Number(NumberOrPercentage),
    Side(S),
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BackgroundSize<'ast>(pub(crate) NodeId<'ast, BackgroundSizeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BackgroundSizeData<'ast> {
    Explicit {
        height: LengthPercentageOrAuto<'ast>,
        width: LengthPercentageOrAuto<'ast>,
    },
    Cover,
    Contain,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BackgroundSizeKind {
    Explicit,
    Cover,
    Contain,
}
impl<'ast> NodeStorage<'ast> for BackgroundSize<'ast> {
    const KIND: NodeKind = super::kind::KIND_BACKGROUND_SIZE;
    type Data = BackgroundSizeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, BackgroundSizeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, BackgroundSizeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> BackgroundSize<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> BackgroundSizeData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> BackgroundSizeKind {
        match ast.data(self) {
            BackgroundSizeData::Explicit { .. } => BackgroundSizeKind::Explicit,
            BackgroundSizeData::Cover => BackgroundSizeKind::Cover,
            BackgroundSizeData::Contain => BackgroundSizeKind::Contain,
        }
    }
}
impl<'ast> GetSpan<'ast> for BackgroundSize<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for BackgroundSize<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_background_size(
        &mut self,
        data: BackgroundSizeData<'ast>,
        span: Span,
    ) -> BackgroundSize<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LengthPercentageOrAuto<'ast>(pub(crate) NodeId<'ast, LengthPercentageOrAutoData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthPercentageOrAutoData<'ast> {
    Auto,
    LengthPercentage(LengthPercentage<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LengthPercentageOrAutoKind {
    Auto,
    LengthPercentage,
}
impl<'ast> NodeStorage<'ast> for LengthPercentageOrAuto<'ast> {
    const KIND: NodeKind = super::kind::KIND_LENGTH_PERCENTAGE_OR_AUTO;
    type Data = LengthPercentageOrAutoData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, LengthPercentageOrAutoData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, LengthPercentageOrAutoData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> LengthPercentageOrAuto<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> LengthPercentageOrAutoData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> LengthPercentageOrAutoKind {
        match ast.data(self) {
            LengthPercentageOrAutoData::Auto => LengthPercentageOrAutoKind::Auto,
            LengthPercentageOrAutoData::LengthPercentage { .. } => {
                LengthPercentageOrAutoKind::LengthPercentage
            }
        }
    }
}
impl<'ast> GetSpan<'ast> for LengthPercentageOrAuto<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for LengthPercentageOrAuto<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_length_percentage_or_auto(
        &mut self,
        data: LengthPercentageOrAutoData<'ast>,
        span: Span,
    ) -> LengthPercentageOrAuto<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Border,
    Text,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}
