use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MediaList<'ast>(pub(crate) NodeId<'ast, MediaListData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MediaListData<'ast> {
    pub media_queries: &'ast [MediaQuery<'ast>],
}
impl<'ast> NodeStorage<'ast> for MediaList<'ast> {
    const KIND: NodeKind = super::kind::KIND_MEDIA_LIST;
    type Data = MediaListData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MediaListData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MediaListData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MediaList<'ast> {
    #[inline]
    pub fn media_queries(self, ast: &Ast<'ast>) -> &'ast [MediaQuery<'ast>] {
        ast.data(self).media_queries
    }
    #[inline]
    pub fn set_media_queries(self, ast: &mut Ast<'ast>, value: &'ast [MediaQuery<'ast>]) {
        let mut data = ast.data(self);
        data.media_queries = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for MediaList<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MediaList<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_media_list(&mut self, data: MediaListData<'ast>, span: Span) -> MediaList<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MediaQuery<'ast>(pub(crate) NodeId<'ast, MediaQueryData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MediaQueryData<'ast> {
    pub condition: Option<MediaCondition<'ast>>,
    pub media_type: MediaType<'ast>,
    pub qualifier: Option<Qualifier>,
}
impl<'ast> NodeStorage<'ast> for MediaQuery<'ast> {
    const KIND: NodeKind = super::kind::KIND_MEDIA_QUERY;
    type Data = MediaQueryData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MediaQueryData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MediaQueryData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MediaQuery<'ast> {
    #[inline]
    pub fn condition(self, ast: &Ast<'ast>) -> Option<MediaCondition<'ast>> {
        ast.data(self).condition
    }
    #[inline]
    pub fn set_condition(self, ast: &mut Ast<'ast>, value: Option<MediaCondition<'ast>>) {
        let mut data = ast.data(self);
        data.condition = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn media_type(self, ast: &Ast<'ast>) -> MediaType<'ast> {
        ast.data(self).media_type
    }
    #[inline]
    pub fn set_media_type(self, ast: &mut Ast<'ast>, value: MediaType<'ast>) {
        let mut data = ast.data(self);
        data.media_type = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn qualifier(self, ast: &Ast<'ast>) -> Option<Qualifier> {
        ast.data(self).qualifier
    }
    #[inline]
    pub fn set_qualifier(self, ast: &mut Ast<'ast>, value: Option<Qualifier>) {
        let mut data = ast.data(self);
        data.qualifier = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for MediaQuery<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MediaQuery<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_media_query(
        &mut self,
        data: MediaQueryData<'ast>,
        span: Span,
    ) -> MediaQuery<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LengthValue<'ast>(pub(crate) NodeId<'ast, LengthValueData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LengthValueData {
    pub unit: LengthUnit,
    pub value: f32,
}
impl<'ast> NodeStorage<'ast> for LengthValue<'ast> {
    const KIND: NodeKind = super::kind::KIND_LENGTH_VALUE;
    type Data = LengthValueData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, LengthValueData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, LengthValueData>) -> Self {
        Self(id)
    }
}
impl<'ast> LengthValue<'ast> {
    #[inline]
    pub fn unit(self, ast: &Ast<'ast>) -> LengthUnit {
        ast.data(self).unit
    }
    #[inline]
    pub fn set_unit(self, ast: &mut Ast<'ast>, value: LengthUnit) {
        let mut data = ast.data(self);
        data.unit = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn value(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).value
    }
    #[inline]
    pub fn set_value(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.value = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for LengthValue<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for LengthValue<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_length_value(&mut self, data: LengthValueData, span: Span) -> LengthValue<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EnvironmentVariable<'ast>(pub(crate) NodeId<'ast, EnvironmentVariableData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnvironmentVariableData<'ast> {
    pub fallback: Option<&'ast [TokenOrValue<'ast>]>,
    pub indices: &'ast [i32],
    pub name: EnvironmentVariableName<'ast>,
}
impl<'ast> NodeStorage<'ast> for EnvironmentVariable<'ast> {
    const KIND: NodeKind = super::kind::KIND_ENVIRONMENT_VARIABLE;
    type Data = EnvironmentVariableData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, EnvironmentVariableData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, EnvironmentVariableData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> EnvironmentVariable<'ast> {
    #[inline]
    pub fn fallback(self, ast: &Ast<'ast>) -> Option<&'ast [TokenOrValue<'ast>]> {
        ast.data(self).fallback
    }
    #[inline]
    pub fn set_fallback(self, ast: &mut Ast<'ast>, value: Option<&'ast [TokenOrValue<'ast>]>) {
        let mut data = ast.data(self);
        data.fallback = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn indices(self, ast: &Ast<'ast>) -> &'ast [i32] {
        ast.data(self).indices
    }
    #[inline]
    pub fn set_indices(self, ast: &mut Ast<'ast>, value: &'ast [i32]) {
        let mut data = ast.data(self);
        data.indices = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> EnvironmentVariableName<'ast> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: EnvironmentVariableName<'ast>) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for EnvironmentVariable<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for EnvironmentVariable<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_environment_variable(
        &mut self,
        data: EnvironmentVariableData<'ast>,
        span: Span,
    ) -> EnvironmentVariable<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Url<'ast>(pub(crate) NodeId<'ast, UrlData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UrlData<'ast> {
    pub url: &'ast str,
}
impl<'ast> NodeStorage<'ast> for Url<'ast> {
    const KIND: NodeKind = super::kind::KIND_URL;
    type Data = UrlData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, UrlData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, UrlData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Url<'ast> {
    #[inline]
    pub fn url(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self).url
    }
    #[inline]
    pub fn set_url(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self);
        data.url = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Url<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Url<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_url(&mut self, data: UrlData<'ast>, span: Span) -> Url<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Variable<'ast>(pub(crate) NodeId<'ast, VariableData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VariableData<'ast> {
    pub fallback: Option<&'ast [TokenOrValue<'ast>]>,
    pub name: DashedIdentReference<'ast>,
}
impl<'ast> NodeStorage<'ast> for Variable<'ast> {
    const KIND: NodeKind = super::kind::KIND_VARIABLE;
    type Data = VariableData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, VariableData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, VariableData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Variable<'ast> {
    #[inline]
    pub fn fallback(self, ast: &Ast<'ast>) -> Option<&'ast [TokenOrValue<'ast>]> {
        ast.data(self).fallback
    }
    #[inline]
    pub fn set_fallback(self, ast: &mut Ast<'ast>, value: Option<&'ast [TokenOrValue<'ast>]>) {
        let mut data = ast.data(self);
        data.fallback = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> DashedIdentReference<'ast> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: DashedIdentReference<'ast>) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Variable<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Variable<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_variable(&mut self, data: VariableData<'ast>, span: Span) -> Variable<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DashedIdentReference<'ast>(pub(crate) NodeId<'ast, DashedIdentReferenceData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DashedIdentReferenceData<'ast> {
    pub from: Option<Specifier<'ast>>,
    pub ident: &'ast str,
}
impl<'ast> NodeStorage<'ast> for DashedIdentReference<'ast> {
    const KIND: NodeKind = super::kind::KIND_DASHED_IDENT_REFERENCE;
    type Data = DashedIdentReferenceData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, DashedIdentReferenceData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, DashedIdentReferenceData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> DashedIdentReference<'ast> {
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
    pub fn ident(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self).ident
    }
    #[inline]
    pub fn set_ident(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self);
        data.ident = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for DashedIdentReference<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for DashedIdentReference<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_dashed_ident_reference(
        &mut self,
        data: DashedIdentReferenceData<'ast>,
        span: Span,
    ) -> DashedIdentReference<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Function<'ast>(pub(crate) NodeId<'ast, FunctionData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionData<'ast> {
    pub arguments: &'ast [TokenOrValue<'ast>],
    pub flags: FunctionFlags,
    pub kind: KnownFunction,
    pub name: &'ast str,
    pub replacement: Option<FunctionReplacement<'ast>>,
}
impl<'ast> NodeStorage<'ast> for Function<'ast> {
    const KIND: NodeKind = super::kind::KIND_FUNCTION;
    type Data = FunctionData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FunctionData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FunctionData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Function<'ast> {
    #[inline]
    pub fn arguments(self, ast: &Ast<'ast>) -> &'ast [TokenOrValue<'ast>] {
        ast.data(self).arguments
    }
    #[inline]
    pub fn set_arguments(self, ast: &mut Ast<'ast>, value: &'ast [TokenOrValue<'ast>]) {
        let mut data = ast.data(self);
        data.arguments = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn flags(self, ast: &Ast<'ast>) -> FunctionFlags {
        ast.data(self).flags
    }
    #[inline]
    pub fn set_flags(self, ast: &mut Ast<'ast>, value: FunctionFlags) {
        let mut data = ast.data(self);
        data.flags = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> KnownFunction {
        ast.data(self).kind
    }
    #[inline]
    pub fn set_kind(self, ast: &mut Ast<'ast>, value: KnownFunction) {
        let mut data = ast.data(self);
        data.kind = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn replacement(self, ast: &Ast<'ast>) -> Option<FunctionReplacement<'ast>> {
        ast.data(self).replacement
    }
    #[inline]
    pub fn set_replacement(self, ast: &mut Ast<'ast>, value: Option<FunctionReplacement<'ast>>) {
        let mut data = ast.data(self);
        data.replacement = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Function<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Function<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_function(&mut self, data: FunctionData<'ast>, span: Span) -> Function<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// A function name recognized by RocketCSS.
///
/// The original function name remains on [`Function`] so parsing and code
/// generation stay lossless. This enum gives downstream passes a shared,
/// ASCII case-insensitive identity without repeating string matching.
#[repr(u8)]
pub enum KnownFunction {
    Abs,
    Calc,
    Clamp,
    Color,
    ColorMix,
    Constant,
    ConicGradient,
    CubicBezier,
    Env,
    Frames,
    Hsl,
    Hsla,
    Hwb,
    Hypot,
    Lab,
    Lch,
    Linear,
    LinearGradient,
    Local,
    Matrix,
    Matrix3d,
    Max,
    Min,
    Mod,
    RadialGradient,
    Rem,
    RepeatingConicGradient,
    RepeatingLinearGradient,
    RepeatingRadialGradient,
    Rgb,
    Rgba,
    Rotate,
    RotateX,
    RotateY,
    Rotate3d,
    RotateZ,
    Round,
    Scale,
    ScaleX,
    ScaleY,
    ScaleZ,
    Scale3d,
    Sign,
    Steps,
    Translate,
    TranslateY,
    TranslateZ,
    Translate3d,
    Url,
    Var,
    #[default]
    Unknown,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FunctionReplacement<'ast>(pub(crate) NodeId<'ast, FunctionReplacementData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionReplacementData<'ast> {
    GrayAlpha {
        alpha: f32,
        lightness: f32,
    },
    Number(f32),
    Dimension {
        unit: Unit<'ast>,
        value: f32,
    },
    Percentage(f32),
    Rgb {
        blue: u8,
        green: u8,
        red: u8,
    },
    Rgba {
        alpha: f32,
        blue: u8,
        green: u8,
        red: u8,
        use_hex: bool,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FunctionReplacementKind {
    GrayAlpha,
    Number,
    Dimension,
    Percentage,
    Rgb,
    Rgba,
}
impl<'ast> NodeStorage<'ast> for FunctionReplacement<'ast> {
    const KIND: NodeKind = super::kind::KIND_FUNCTION_REPLACEMENT;
    type Data = FunctionReplacementData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FunctionReplacementData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FunctionReplacementData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FunctionReplacement<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> FunctionReplacementData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> FunctionReplacementKind {
        match ast.data(self) {
            FunctionReplacementData::GrayAlpha { .. } => FunctionReplacementKind::GrayAlpha,
            FunctionReplacementData::Number { .. } => FunctionReplacementKind::Number,
            FunctionReplacementData::Dimension { .. } => FunctionReplacementKind::Dimension,
            FunctionReplacementData::Percentage { .. } => FunctionReplacementKind::Percentage,
            FunctionReplacementData::Rgb { .. } => FunctionReplacementKind::Rgb,
            FunctionReplacementData::Rgba { .. } => FunctionReplacementKind::Rgba,
        }
    }
}
impl<'ast> GetSpan<'ast> for FunctionReplacement<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FunctionReplacement<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_function_replacement(
        &mut self,
        data: FunctionReplacementData<'ast>,
        span: Span,
    ) -> FunctionReplacement<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ImportRule<'ast>(pub(crate) NodeId<'ast, ImportRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImportRuleData<'ast> {
    pub layer: Option<&'ast [&'ast str]>,
    pub media: Option<MediaList<'ast>>,
    pub supports: Option<SupportsCondition<'ast>>,
    pub url: &'ast str,
}
impl<'ast> NodeStorage<'ast> for ImportRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_IMPORT_RULE;
    type Data = ImportRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ImportRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ImportRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ImportRule<'ast> {
    #[inline]
    pub fn layer(self, ast: &Ast<'ast>) -> Option<&'ast [&'ast str]> {
        ast.data(self).layer
    }
    #[inline]
    pub fn set_layer(self, ast: &mut Ast<'ast>, value: Option<&'ast [&'ast str]>) {
        let mut data = ast.data(self);
        data.layer = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn media(self, ast: &Ast<'ast>) -> Option<MediaList<'ast>> {
        ast.data(self).media
    }
    #[inline]
    pub fn set_media(self, ast: &mut Ast<'ast>, value: Option<MediaList<'ast>>) {
        let mut data = ast.data(self);
        data.media = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn supports(self, ast: &Ast<'ast>) -> Option<SupportsCondition<'ast>> {
        ast.data(self).supports
    }
    #[inline]
    pub fn set_supports(self, ast: &mut Ast<'ast>, value: Option<SupportsCondition<'ast>>) {
        let mut data = ast.data(self);
        data.supports = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn url(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self).url
    }
    #[inline]
    pub fn set_url(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self);
        data.url = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ImportRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ImportRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_import_rule(
        &mut self,
        data: ImportRuleData<'ast>,
        span: Span,
    ) -> ImportRule<'ast> {
        self.alloc(data, span)
    }
}
