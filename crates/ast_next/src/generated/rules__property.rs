use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ParsedComponent<'ast>(pub(crate) NodeId<'ast, ParsedComponentData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParsedComponentData<'ast> {
    Length(Length<'ast>),
    Number(f32),
    Percentage(f32),
    LengthPercentage(LengthPercentage<'ast>),
    String(&'ast str),
    Color(CssColor<'ast>),
    Image(Image<'ast>),
    Url(Url<'ast>),
    Integer(i32),
    Angle(Angle),
    Time(Time),
    Resolution(Resolution),
    TransformFunction(Transform<'ast>),
    TransformList(&'ast [Transform<'ast>]),
    CustomIdent(&'ast str),
    Literal(&'ast str),
    Repeated {
        components: &'ast [ParsedComponent<'ast>],
        multiplier: Multiplier,
    },
    TokenList(&'ast [TokenOrValue<'ast>]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParsedComponentKind {
    Length,
    Number,
    Percentage,
    LengthPercentage,
    String,
    Color,
    Image,
    Url,
    Integer,
    Angle,
    Time,
    Resolution,
    TransformFunction,
    TransformList,
    CustomIdent,
    Literal,
    Repeated,
    TokenList,
}
impl<'ast> NodeStorage<'ast> for ParsedComponent<'ast> {
    const KIND: NodeKind = super::kind::KIND_PARSED_COMPONENT;
    type Data = ParsedComponentData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ParsedComponentData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ParsedComponentData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ParsedComponent<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ParsedComponentData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ParsedComponentKind {
        match ast.data(self) {
            ParsedComponentData::Length { .. } => ParsedComponentKind::Length,
            ParsedComponentData::Number { .. } => ParsedComponentKind::Number,
            ParsedComponentData::Percentage { .. } => ParsedComponentKind::Percentage,
            ParsedComponentData::LengthPercentage { .. } => ParsedComponentKind::LengthPercentage,
            ParsedComponentData::String { .. } => ParsedComponentKind::String,
            ParsedComponentData::Color { .. } => ParsedComponentKind::Color,
            ParsedComponentData::Image { .. } => ParsedComponentKind::Image,
            ParsedComponentData::Url { .. } => ParsedComponentKind::Url,
            ParsedComponentData::Integer { .. } => ParsedComponentKind::Integer,
            ParsedComponentData::Angle { .. } => ParsedComponentKind::Angle,
            ParsedComponentData::Time { .. } => ParsedComponentKind::Time,
            ParsedComponentData::Resolution { .. } => ParsedComponentKind::Resolution,
            ParsedComponentData::TransformFunction { .. } => ParsedComponentKind::TransformFunction,
            ParsedComponentData::TransformList { .. } => ParsedComponentKind::TransformList,
            ParsedComponentData::CustomIdent { .. } => ParsedComponentKind::CustomIdent,
            ParsedComponentData::Literal { .. } => ParsedComponentKind::Literal,
            ParsedComponentData::Repeated { .. } => ParsedComponentKind::Repeated,
            ParsedComponentData::TokenList { .. } => ParsedComponentKind::TokenList,
        }
    }
}
impl<'ast> GetSpan<'ast> for ParsedComponent<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ParsedComponent<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_parsed_component(
        &mut self,
        data: ParsedComponentData<'ast>,
        span: Span,
    ) -> ParsedComponent<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Multiplier {
    None,
    Space,
    Comma,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SyntaxString<'ast>(pub(crate) NodeId<'ast, SyntaxStringData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyntaxStringData<'ast> {
    Components(&'ast [SyntaxComponent<'ast>]),
    Universal,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SyntaxStringKind {
    Components,
    Universal,
}
impl<'ast> NodeStorage<'ast> for SyntaxString<'ast> {
    const KIND: NodeKind = super::kind::KIND_SYNTAX_STRING;
    type Data = SyntaxStringData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SyntaxStringData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SyntaxStringData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> SyntaxString<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SyntaxStringData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SyntaxStringKind {
        match ast.data(self) {
            SyntaxStringData::Components { .. } => SyntaxStringKind::Components,
            SyntaxStringData::Universal => SyntaxStringKind::Universal,
        }
    }
}
impl<'ast> GetSpan<'ast> for SyntaxString<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for SyntaxString<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_syntax_string(
        &mut self,
        data: SyntaxStringData<'ast>,
        span: Span,
    ) -> SyntaxString<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SyntaxComponentKind<'ast>(pub(crate) NodeId<'ast, SyntaxComponentKindData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyntaxComponentKindData<'ast> {
    Length,
    Number,
    Percentage,
    LengthPercentage,
    String,
    Color,
    Image,
    Url,
    Integer,
    Angle,
    Time,
    Resolution,
    TransformFunction,
    TransformList,
    CustomIdent,
    Literal(&'ast str),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SyntaxComponentKindKind {
    Length,
    Number,
    Percentage,
    LengthPercentage,
    String,
    Color,
    Image,
    Url,
    Integer,
    Angle,
    Time,
    Resolution,
    TransformFunction,
    TransformList,
    CustomIdent,
    Literal,
}
impl<'ast> NodeStorage<'ast> for SyntaxComponentKind<'ast> {
    const KIND: NodeKind = super::kind::KIND_SYNTAX_COMPONENT_KIND;
    type Data = SyntaxComponentKindData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SyntaxComponentKindData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SyntaxComponentKindData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> SyntaxComponentKind<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SyntaxComponentKindData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SyntaxComponentKindKind {
        match ast.data(self) {
            SyntaxComponentKindData::Length => SyntaxComponentKindKind::Length,
            SyntaxComponentKindData::Number => SyntaxComponentKindKind::Number,
            SyntaxComponentKindData::Percentage => SyntaxComponentKindKind::Percentage,
            SyntaxComponentKindData::LengthPercentage => SyntaxComponentKindKind::LengthPercentage,
            SyntaxComponentKindData::String => SyntaxComponentKindKind::String,
            SyntaxComponentKindData::Color => SyntaxComponentKindKind::Color,
            SyntaxComponentKindData::Image => SyntaxComponentKindKind::Image,
            SyntaxComponentKindData::Url => SyntaxComponentKindKind::Url,
            SyntaxComponentKindData::Integer => SyntaxComponentKindKind::Integer,
            SyntaxComponentKindData::Angle => SyntaxComponentKindKind::Angle,
            SyntaxComponentKindData::Time => SyntaxComponentKindKind::Time,
            SyntaxComponentKindData::Resolution => SyntaxComponentKindKind::Resolution,
            SyntaxComponentKindData::TransformFunction => {
                SyntaxComponentKindKind::TransformFunction
            }
            SyntaxComponentKindData::TransformList => SyntaxComponentKindKind::TransformList,
            SyntaxComponentKindData::CustomIdent => SyntaxComponentKindKind::CustomIdent,
            SyntaxComponentKindData::Literal { .. } => SyntaxComponentKindKind::Literal,
        }
    }
}
impl<'ast> GetSpan<'ast> for SyntaxComponentKind<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for SyntaxComponentKind<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_syntax_component_kind(
        &mut self,
        data: SyntaxComponentKindData<'ast>,
        span: Span,
    ) -> SyntaxComponentKind<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UnparsedProperty<'ast>(pub(crate) NodeId<'ast, UnparsedPropertyData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnparsedPropertyData<'ast> {
    pub property_id: PropertyId<'ast>,
    pub reason: UnparsedPropertyReason,
    pub raw_value: Option<&'ast str>,
    pub value: &'ast [TokenOrValue<'ast>],
}
impl<'ast> NodeStorage<'ast> for UnparsedProperty<'ast> {
    const KIND: NodeKind = super::kind::KIND_UNPARSED_PROPERTY;
    type Data = UnparsedPropertyData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, UnparsedPropertyData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, UnparsedPropertyData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> UnparsedProperty<'ast> {
    #[inline]
    pub fn property_id(self, ast: &Ast<'ast>) -> PropertyId<'ast> {
        ast.data(self).property_id
    }
    #[inline]
    pub fn set_property_id(self, ast: &mut Ast<'ast>, value: PropertyId<'ast>) {
        let mut data = ast.data(self);
        data.property_id = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn reason(self, ast: &Ast<'ast>) -> UnparsedPropertyReason {
        ast.data(self).reason
    }
    #[inline]
    pub fn set_reason(self, ast: &mut Ast<'ast>, value: UnparsedPropertyReason) {
        let mut data = ast.data(self);
        data.reason = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn raw_value(self, ast: &Ast<'ast>) -> Option<&'ast str> {
        ast.data(self).raw_value
    }
    #[inline]
    pub fn set_raw_value(self, ast: &mut Ast<'ast>, value: Option<&'ast str>) {
        let mut data = ast.data(self);
        data.raw_value = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn value(self, ast: &Ast<'ast>) -> &'ast [TokenOrValue<'ast>] {
        ast.data(self).value
    }
    #[inline]
    pub fn set_value(self, ast: &mut Ast<'ast>, value: &'ast [TokenOrValue<'ast>]) {
        let mut data = ast.data(self);
        data.value = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for UnparsedProperty<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for UnparsedProperty<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_unparsed_property(
        &mut self,
        data: UnparsedPropertyData<'ast>,
        span: Span,
    ) -> UnparsedProperty<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
/// Why a declaration could not be represented by its typed value AST.
///
/// Keeping this decision in the parsed tree lets transforms distinguish
/// unsupported grammar from values whose syntax or semantics are opaque.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnparsedPropertyReason {
    /// RocketCSS recognizes the property, but does not implement its grammar yet.
    UnsupportedGrammar,
    /// The property name is unknown, so its value grammar is also unknown.
    UnknownProperty,
    /// A supported grammar contains a function or comment that cannot be
    /// validated without preserving its original token representation.
    OpaqueValue,
    /// The implemented grammar rejected an otherwise tokenizable value.
    InvalidValue,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CustomProperty<'ast>(pub(crate) NodeId<'ast, CustomPropertyData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CustomPropertyData<'ast> {
    pub name: CustomPropertyName<'ast>,
    pub value: &'ast [TokenOrValue<'ast>],
}
impl<'ast> NodeStorage<'ast> for CustomProperty<'ast> {
    const KIND: NodeKind = super::kind::KIND_CUSTOM_PROPERTY;
    type Data = CustomPropertyData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CustomPropertyData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CustomPropertyData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CustomProperty<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> CustomPropertyName<'ast> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: CustomPropertyName<'ast>) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn value(self, ast: &Ast<'ast>) -> &'ast [TokenOrValue<'ast>] {
        ast.data(self).value
    }
    #[inline]
    pub fn set_value(self, ast: &mut Ast<'ast>, value: &'ast [TokenOrValue<'ast>]) {
        let mut data = ast.data(self);
        data.value = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for CustomProperty<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CustomProperty<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_custom_property(
        &mut self,
        data: CustomPropertyData<'ast>,
        span: Span,
    ) -> CustomProperty<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SyntaxComponent<'ast>(pub(crate) NodeId<'ast, SyntaxComponentData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SyntaxComponentData<'ast> {
    pub kind: SyntaxComponentKind<'ast>,
    pub multiplier: Multiplier,
}
impl<'ast> NodeStorage<'ast> for SyntaxComponent<'ast> {
    const KIND: NodeKind = super::kind::KIND_SYNTAX_COMPONENT;
    type Data = SyntaxComponentData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SyntaxComponentData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SyntaxComponentData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> SyntaxComponent<'ast> {
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SyntaxComponentKind<'ast> {
        ast.data(self).kind
    }
    #[inline]
    pub fn set_kind(self, ast: &mut Ast<'ast>, value: SyntaxComponentKind<'ast>) {
        let mut data = ast.data(self);
        data.kind = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn multiplier(self, ast: &Ast<'ast>) -> Multiplier {
        ast.data(self).multiplier
    }
    #[inline]
    pub fn set_multiplier(self, ast: &mut Ast<'ast>, value: Multiplier) {
        let mut data = ast.data(self);
        data.multiplier = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for SyntaxComponent<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for SyntaxComponent<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_syntax_component(
        &mut self,
        data: SyntaxComponentData<'ast>,
        span: Span,
    ) -> SyntaxComponent<'ast> {
        self.alloc(data, span)
    }
}
