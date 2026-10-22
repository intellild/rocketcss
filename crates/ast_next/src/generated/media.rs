use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MediaCondition<'ast>(pub(crate) NodeId<'ast, MediaConditionData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MediaConditionData<'ast> {
    Feature(MediaFeature<'ast>),
    Not(MediaCondition<'ast>),
    Operation {
        conditions: &'ast [MediaCondition<'ast>],
        operator: Operator,
    },
    Unknown(&'ast [TokenOrValue<'ast>]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MediaConditionKind {
    Feature,
    Not,
    Operation,
    Unknown,
}
impl<'ast> NodeStorage<'ast> for MediaCondition<'ast> {
    const KIND: NodeKind = super::kind::KIND_MEDIA_CONDITION;
    type Data = MediaConditionData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MediaConditionData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MediaConditionData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MediaCondition<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> MediaConditionData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> MediaConditionKind {
        match ast.data(self) {
            MediaConditionData::Feature { .. } => MediaConditionKind::Feature,
            MediaConditionData::Not { .. } => MediaConditionKind::Not,
            MediaConditionData::Operation { .. } => MediaConditionKind::Operation,
            MediaConditionData::Unknown { .. } => MediaConditionKind::Unknown,
        }
    }
}
impl<'ast> GetSpan<'ast> for MediaCondition<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MediaCondition<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_media_condition(
        &mut self,
        data: MediaConditionData<'ast>,
        span: Span,
    ) -> MediaCondition<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct QueryFeature<'ast, FeatureId>(
    pub(crate) NodeId<'ast, QueryFeatureData<'ast, FeatureId>>,
);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QueryFeatureData<'ast, FeatureId> {
    Plain {
        name: MediaFeatureName<'ast, FeatureId>,
        value: MediaFeatureValue<'ast>,
    },
    Boolean {
        name: MediaFeatureName<'ast, FeatureId>,
    },
    Range {
        name: MediaFeatureName<'ast, FeatureId>,
        operator: MediaFeatureComparison,
        value: MediaFeatureValue<'ast>,
    },
    Interval {
        end: MediaFeatureValue<'ast>,
        end_operator: MediaFeatureComparison,
        name: MediaFeatureName<'ast, FeatureId>,
        start: MediaFeatureValue<'ast>,
        start_operator: MediaFeatureComparison,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QueryFeatureKind {
    Plain,
    Boolean,
    Range,
    Interval,
}
impl<'ast, FeatureId> NodeStorage<'ast> for QueryFeature<'ast, FeatureId>
where
    FeatureId: Copy,
{
    const KIND: NodeKind = super::kind::KIND_QUERY_FEATURE;
    type Data = QueryFeatureData<'ast, FeatureId>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, QueryFeatureData<'ast, FeatureId>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, QueryFeatureData<'ast, FeatureId>>) -> Self {
        Self(id)
    }
}
impl<'ast, FeatureId> QueryFeature<'ast, FeatureId>
where
    FeatureId: Copy,
{
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> QueryFeatureData<'ast, FeatureId> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> QueryFeatureKind {
        match ast.data(self) {
            QueryFeatureData::Plain { .. } => QueryFeatureKind::Plain,
            QueryFeatureData::Boolean { .. } => QueryFeatureKind::Boolean,
            QueryFeatureData::Range { .. } => QueryFeatureKind::Range,
            QueryFeatureData::Interval { .. } => QueryFeatureKind::Interval,
        }
    }
}
impl<'ast, FeatureId> GetSpan<'ast> for QueryFeature<'ast, FeatureId>
where
    FeatureId: Copy,
{
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast, FeatureId> SetSpan<'ast> for QueryFeature<'ast, FeatureId>
where
    FeatureId: Copy,
{
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_query_feature<FeatureId>(
        &mut self,
        data: QueryFeatureData<'ast, FeatureId>,
        span: Span,
    ) -> QueryFeature<'ast, FeatureId>
    where
        FeatureId: Copy,
    {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MediaFeatureName<'ast, FeatureId> {
    Standard(FeatureId),
    Custom(&'ast str),
    Unknown(&'ast str),
}
pub type MediaFeature<'ast> = QueryFeature<'ast, MediaFeatureId>;
use crate::*;
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MediaFeatureId {
    Width,
    Height,
    AspectRatio,
    Orientation,
    OverflowBlock,
    OverflowInline,
    HorizontalViewportSegments,
    VerticalViewportSegments,
    DisplayMode,
    Resolution,
    Scan,
    Grid,
    Update,
    EnvironmentBlending,
    Color,
    ColorIndex,
    Monochrome,
    ColorGamut,
    DynamicRange,
    InvertedColors,
    Pointer,
    Hover,
    AnyPointer,
    AnyHover,
    NavControls,
    VideoColorGamut,
    VideoDynamicRange,
    Scripting,
    PrefersReducedMotion,
    PrefersReducedTransparency,
    PrefersContrast,
    ForcedColors,
    PrefersColorScheme,
    PrefersReducedData,
    DeviceWidth,
    DeviceHeight,
    DeviceAspectRatio,
    #[css_keyword("-webkit-device-pixel-ratio")]
    WebkitDevicePixelRatio,
    #[css_keyword("-moz-device-pixel-ratio")]
    MozDevicePixelRatio,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MediaFeatureValue<'ast>(pub(crate) NodeId<'ast, MediaFeatureValueData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MediaFeatureValueData<'ast> {
    Length(Length<'ast>),
    Number(f32),
    Integer(i32),
    Boolean(bool),
    Resolution(Resolution),
    Ratio(Ratio<'ast>),
    Ident(&'ast str),
    Env(EnvironmentVariable<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MediaFeatureValueKind {
    Length,
    Number,
    Integer,
    Boolean,
    Resolution,
    Ratio,
    Ident,
    Env,
}
impl<'ast> NodeStorage<'ast> for MediaFeatureValue<'ast> {
    const KIND: NodeKind = super::kind::KIND_MEDIA_FEATURE_VALUE;
    type Data = MediaFeatureValueData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MediaFeatureValueData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MediaFeatureValueData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MediaFeatureValue<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> MediaFeatureValueData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> MediaFeatureValueKind {
        match ast.data(self) {
            MediaFeatureValueData::Length { .. } => MediaFeatureValueKind::Length,
            MediaFeatureValueData::Number { .. } => MediaFeatureValueKind::Number,
            MediaFeatureValueData::Integer { .. } => MediaFeatureValueKind::Integer,
            MediaFeatureValueData::Boolean { .. } => MediaFeatureValueKind::Boolean,
            MediaFeatureValueData::Resolution { .. } => MediaFeatureValueKind::Resolution,
            MediaFeatureValueData::Ratio { .. } => MediaFeatureValueKind::Ratio,
            MediaFeatureValueData::Ident { .. } => MediaFeatureValueKind::Ident,
            MediaFeatureValueData::Env { .. } => MediaFeatureValueKind::Env,
        }
    }
}
impl<'ast> GetSpan<'ast> for MediaFeatureValue<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MediaFeatureValue<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_media_feature_value(
        &mut self,
        data: MediaFeatureValueData<'ast>,
        span: Span,
    ) -> MediaFeatureValue<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MediaFeatureComparison {
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Operator {
    And,
    Or,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy)]
pub enum MediaType<'ast> {
    All,
    Print,
    Screen,
    Custom(&'ast str),
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Qualifier {
    Only,
    Not,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SupportsCondition<'ast>(pub(crate) NodeId<'ast, SupportsConditionData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SupportsConditionData<'ast> {
    Not(SupportsCondition<'ast>),
    And(&'ast [SupportsCondition<'ast>]),
    Or(&'ast [SupportsCondition<'ast>]),
    Declaration {
        property_id: PropertyId<'ast>,
        value: &'ast str,
    },
    Selector(&'ast str),
    Unknown(&'ast str),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SupportsConditionKind {
    Not,
    And,
    Or,
    Declaration,
    Selector,
    Unknown,
}
impl<'ast> NodeStorage<'ast> for SupportsCondition<'ast> {
    const KIND: NodeKind = super::kind::KIND_SUPPORTS_CONDITION;
    type Data = SupportsConditionData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SupportsConditionData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SupportsConditionData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> SupportsCondition<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SupportsConditionData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SupportsConditionKind {
        match ast.data(self) {
            SupportsConditionData::Not { .. } => SupportsConditionKind::Not,
            SupportsConditionData::And { .. } => SupportsConditionKind::And,
            SupportsConditionData::Or { .. } => SupportsConditionKind::Or,
            SupportsConditionData::Declaration { .. } => SupportsConditionKind::Declaration,
            SupportsConditionData::Selector { .. } => SupportsConditionKind::Selector,
            SupportsConditionData::Unknown { .. } => SupportsConditionKind::Unknown,
        }
    }
}
impl<'ast> GetSpan<'ast> for SupportsCondition<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for SupportsCondition<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_supports_condition(
        &mut self,
        data: SupportsConditionData<'ast>,
        span: Span,
    ) -> SupportsCondition<'ast> {
        self.alloc(data, span)
    }
}
