use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StyleSheet<'ast>(pub(crate) NodeId<'ast, StyleSheetData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StyleSheetData<'ast> {
    pub rules: &'ast [CssRule<'ast>],
}
impl<'ast> NodeStorage<'ast> for StyleSheet<'ast> {
    const KIND: NodeKind = super::kind::KIND_STYLE_SHEET;
    type Data = StyleSheetData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, StyleSheetData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, StyleSheetData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> StyleSheet<'ast> {
    #[inline]
    pub fn rules(self, ast: &Ast<'ast>) -> &'ast [CssRule<'ast>] {
        ast.data(self).rules
    }
    #[inline]
    pub fn set_rules(self, ast: &mut Ast<'ast>, value: &'ast [CssRule<'ast>]) {
        let mut data = ast.data(self);
        data.rules = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for StyleSheet<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for StyleSheet<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_style_sheet(
        &mut self,
        data: StyleSheetData<'ast>,
        span: Span,
    ) -> StyleSheet<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CssRule<'ast>(pub(crate) NodeId<'ast, CssRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssRuleData<'ast> {
    pub payload: CssRulePayload<'ast>,
    pub declarations: Option<DeclarationBlock<'ast>>,
    pub children: &'ast [CssRule<'ast>],
}
impl<'ast> NodeStorage<'ast> for CssRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_CSS_RULE;
    type Data = CssRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CssRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CssRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CssRule<'ast> {
    #[inline]
    pub fn payload(self, ast: &Ast<'ast>) -> CssRulePayload<'ast> {
        ast.data(self).payload
    }
    #[inline]
    pub fn set_payload(self, ast: &mut Ast<'ast>, value: CssRulePayload<'ast>) {
        let mut data = ast.data(self);
        data.payload = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn declarations(self, ast: &Ast<'ast>) -> Option<DeclarationBlock<'ast>> {
        ast.data(self).declarations
    }
    #[inline]
    pub fn set_declarations(self, ast: &mut Ast<'ast>, value: Option<DeclarationBlock<'ast>>) {
        let mut data = ast.data(self);
        data.declarations = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn children(self, ast: &Ast<'ast>) -> &'ast [CssRule<'ast>] {
        ast.data(self).children
    }
    #[inline]
    pub fn set_children(self, ast: &mut Ast<'ast>, value: &'ast [CssRule<'ast>]) {
        let mut data = ast.data(self);
        data.children = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for CssRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CssRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_css_rule(&mut self, data: CssRuleData<'ast>, span: Span) -> CssRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DeclarationBlock<'ast>(pub(crate) NodeId<'ast, DeclarationBlockData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeclarationBlockData<'ast> {
    pub declarations: &'ast [DeclarationPayload<'ast>],
}
impl<'ast> NodeStorage<'ast> for DeclarationBlock<'ast> {
    const KIND: NodeKind = super::kind::KIND_DECLARATION_BLOCK;
    type Data = DeclarationBlockData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, DeclarationBlockData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, DeclarationBlockData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> DeclarationBlock<'ast> {
    #[inline]
    pub fn declarations(self, ast: &Ast<'ast>) -> &'ast [DeclarationPayload<'ast>] {
        ast.data(self).declarations
    }
    #[inline]
    pub fn set_declarations(self, ast: &mut Ast<'ast>, value: &'ast [DeclarationPayload<'ast>]) {
        let mut data = ast.data(self);
        data.declarations = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for DeclarationBlock<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for DeclarationBlock<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_declaration_block(
        &mut self,
        data: DeclarationBlockData<'ast>,
        span: Span,
    ) -> DeclarationBlock<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PropertyRuleDescriptor<'ast>(pub(crate) NodeId<'ast, PropertyRuleDescriptorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PropertyRuleDescriptorData<'ast> {
    Syntax(SyntaxString<'ast>),
    Inherits(bool),
    InitialValue(ParsedComponent<'ast>),
    Unknown(CustomProperty<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertyRuleDescriptorKind {
    Syntax,
    Inherits,
    InitialValue,
    Unknown,
}
impl<'ast> NodeStorage<'ast> for PropertyRuleDescriptor<'ast> {
    const KIND: NodeKind = super::kind::KIND_PROPERTY_RULE_DESCRIPTOR;
    type Data = PropertyRuleDescriptorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PropertyRuleDescriptorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PropertyRuleDescriptorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PropertyRuleDescriptor<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> PropertyRuleDescriptorData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> PropertyRuleDescriptorKind {
        match ast.data(self) {
            PropertyRuleDescriptorData::Syntax { .. } => PropertyRuleDescriptorKind::Syntax,
            PropertyRuleDescriptorData::Inherits { .. } => PropertyRuleDescriptorKind::Inherits,
            PropertyRuleDescriptorData::InitialValue { .. } => {
                PropertyRuleDescriptorKind::InitialValue
            }
            PropertyRuleDescriptorData::Unknown { .. } => PropertyRuleDescriptorKind::Unknown,
        }
    }
}
impl<'ast> GetSpan<'ast> for PropertyRuleDescriptor<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PropertyRuleDescriptor<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_property_rule_descriptor(
        &mut self,
        data: PropertyRuleDescriptorData<'ast>,
        span: Span,
    ) -> PropertyRuleDescriptor<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DeclarationPayload<'ast>(pub(crate) NodeId<'ast, DeclarationPayloadData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeclarationPayloadData<'ast> {
    Property(Declaration<'ast>),
    FontFace(FontFaceProperty<'ast>),
    FontPaletteValues(FontPaletteValuesProperty<'ast>),
    ViewTransition(ViewTransitionProperty<'ast>),
    FontFeature(FontFeatureDeclaration<'ast>),
    PropertyRule(PropertyRuleDescriptor<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeclarationPayloadKind {
    Property,
    FontFace,
    FontPaletteValues,
    ViewTransition,
    FontFeature,
    PropertyRule,
}
impl<'ast> NodeStorage<'ast> for DeclarationPayload<'ast> {
    const KIND: NodeKind = super::kind::KIND_DECLARATION_PAYLOAD;
    type Data = DeclarationPayloadData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, DeclarationPayloadData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, DeclarationPayloadData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> DeclarationPayload<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> DeclarationPayloadData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> DeclarationPayloadKind {
        match ast.data(self) {
            DeclarationPayloadData::Property { .. } => DeclarationPayloadKind::Property,
            DeclarationPayloadData::FontFace { .. } => DeclarationPayloadKind::FontFace,
            DeclarationPayloadData::FontPaletteValues { .. } => {
                DeclarationPayloadKind::FontPaletteValues
            }
            DeclarationPayloadData::ViewTransition { .. } => DeclarationPayloadKind::ViewTransition,
            DeclarationPayloadData::FontFeature { .. } => DeclarationPayloadKind::FontFeature,
            DeclarationPayloadData::PropertyRule { .. } => DeclarationPayloadKind::PropertyRule,
        }
    }
}
impl<'ast> GetSpan<'ast> for DeclarationPayload<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for DeclarationPayload<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_declaration_payload(
        &mut self,
        data: DeclarationPayloadData<'ast>,
        span: Span,
    ) -> DeclarationPayload<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CssRulePayload<'ast>(pub(crate) NodeId<'ast, CssRulePayloadData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CssRulePayloadData<'ast> {
    Style(StyleRule<'ast>),
    Media(MediaRule<'ast>),
    Supports(SupportsRule<'ast>),
    StartingStyle(StartingStyleRule<'ast>),
    LayerStatement(LayerStatementRule<'ast>),
    LayerBlock(LayerBlockRule<'ast>),
    Container(ContainerRule<'ast>),
    Scope(ScopeRule<'ast>),
    MozDocument(MozDocumentRule<'ast>),
    Unknown(UnknownAtRule<'ast>),
    CounterStyle(CounterStyleRule<'ast>),
    Viewport(ViewportRule<'ast>),
    PositionTry(PositionTryRule<'ast>),
    FontFace(FontFaceRule<'ast>),
    FontPaletteValues(FontPaletteValuesRule<'ast>),
    ViewTransition(ViewTransitionRule<'ast>),
    Import(ImportRule<'ast>),
    Charset(CharsetRule<'ast>),
    Namespace(NamespaceRule<'ast>),
    CustomMedia(CustomMediaRule<'ast>),
    Keyframes(KeyframesRule<'ast>),
    Keyframe(Keyframe<'ast>),
    Page(PageRule<'ast>),
    PageMargin(PageMarginRule<'ast>),
    PageDeclarations(PageDeclarationsRule<'ast>),
    Nesting(NestingRule<'ast>),
    FontFeatureValues(FontFeatureValuesRule<'ast>),
    FontFeatureSubrule(FontFeatureSubruleRule<'ast>),
    Property(PropertyRule<'ast>),
    NestedDeclarations(NestedDeclarationsRule<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CssRulePayloadKind {
    Style,
    Media,
    Supports,
    StartingStyle,
    LayerStatement,
    LayerBlock,
    Container,
    Scope,
    MozDocument,
    Unknown,
    CounterStyle,
    Viewport,
    PositionTry,
    FontFace,
    FontPaletteValues,
    ViewTransition,
    Import,
    Charset,
    Namespace,
    CustomMedia,
    Keyframes,
    Keyframe,
    Page,
    PageMargin,
    PageDeclarations,
    Nesting,
    FontFeatureValues,
    FontFeatureSubrule,
    Property,
    NestedDeclarations,
}
impl<'ast> NodeStorage<'ast> for CssRulePayload<'ast> {
    const KIND: NodeKind = super::kind::KIND_CSS_RULE_PAYLOAD;
    type Data = CssRulePayloadData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CssRulePayloadData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CssRulePayloadData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CssRulePayload<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> CssRulePayloadData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> CssRulePayloadKind {
        match ast.data(self) {
            CssRulePayloadData::Style { .. } => CssRulePayloadKind::Style,
            CssRulePayloadData::Media { .. } => CssRulePayloadKind::Media,
            CssRulePayloadData::Supports { .. } => CssRulePayloadKind::Supports,
            CssRulePayloadData::StartingStyle { .. } => CssRulePayloadKind::StartingStyle,
            CssRulePayloadData::LayerStatement { .. } => CssRulePayloadKind::LayerStatement,
            CssRulePayloadData::LayerBlock { .. } => CssRulePayloadKind::LayerBlock,
            CssRulePayloadData::Container { .. } => CssRulePayloadKind::Container,
            CssRulePayloadData::Scope { .. } => CssRulePayloadKind::Scope,
            CssRulePayloadData::MozDocument { .. } => CssRulePayloadKind::MozDocument,
            CssRulePayloadData::Unknown { .. } => CssRulePayloadKind::Unknown,
            CssRulePayloadData::CounterStyle { .. } => CssRulePayloadKind::CounterStyle,
            CssRulePayloadData::Viewport { .. } => CssRulePayloadKind::Viewport,
            CssRulePayloadData::PositionTry { .. } => CssRulePayloadKind::PositionTry,
            CssRulePayloadData::FontFace { .. } => CssRulePayloadKind::FontFace,
            CssRulePayloadData::FontPaletteValues { .. } => CssRulePayloadKind::FontPaletteValues,
            CssRulePayloadData::ViewTransition { .. } => CssRulePayloadKind::ViewTransition,
            CssRulePayloadData::Import { .. } => CssRulePayloadKind::Import,
            CssRulePayloadData::Charset { .. } => CssRulePayloadKind::Charset,
            CssRulePayloadData::Namespace { .. } => CssRulePayloadKind::Namespace,
            CssRulePayloadData::CustomMedia { .. } => CssRulePayloadKind::CustomMedia,
            CssRulePayloadData::Keyframes { .. } => CssRulePayloadKind::Keyframes,
            CssRulePayloadData::Keyframe { .. } => CssRulePayloadKind::Keyframe,
            CssRulePayloadData::Page { .. } => CssRulePayloadKind::Page,
            CssRulePayloadData::PageMargin { .. } => CssRulePayloadKind::PageMargin,
            CssRulePayloadData::PageDeclarations { .. } => CssRulePayloadKind::PageDeclarations,
            CssRulePayloadData::Nesting { .. } => CssRulePayloadKind::Nesting,
            CssRulePayloadData::FontFeatureValues { .. } => CssRulePayloadKind::FontFeatureValues,
            CssRulePayloadData::FontFeatureSubrule { .. } => CssRulePayloadKind::FontFeatureSubrule,
            CssRulePayloadData::Property { .. } => CssRulePayloadKind::Property,
            CssRulePayloadData::NestedDeclarations { .. } => CssRulePayloadKind::NestedDeclarations,
        }
    }
}
impl<'ast> GetSpan<'ast> for CssRulePayload<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CssRulePayload<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_css_rule_payload(
        &mut self,
        data: CssRulePayloadData<'ast>,
        span: Span,
    ) -> CssRulePayload<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StyleRule<'ast>(pub(crate) NodeId<'ast, StyleRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StyleRuleData<'ast> {
    pub selectors: SelectorList<'ast>,
    pub vendor_prefix: VendorPrefix,
}
impl<'ast> NodeStorage<'ast> for StyleRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_STYLE_RULE;
    type Data = StyleRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, StyleRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, StyleRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> StyleRule<'ast> {
    #[inline]
    pub fn selectors(self, ast: &Ast<'ast>) -> SelectorList<'ast> {
        ast.data(self).selectors
    }
    #[inline]
    pub fn set_selectors(self, ast: &mut Ast<'ast>, value: SelectorList<'ast>) {
        let mut data = ast.data(self);
        data.selectors = value;
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
impl<'ast> GetSpan<'ast> for StyleRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for StyleRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_style_rule(&mut self, data: StyleRuleData<'ast>, span: Span) -> StyleRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MediaRule<'ast>(pub(crate) NodeId<'ast, MediaRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MediaRuleData<'ast> {
    pub query: MediaList<'ast>,
}
impl<'ast> NodeStorage<'ast> for MediaRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_MEDIA_RULE;
    type Data = MediaRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MediaRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MediaRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> MediaRule<'ast> {
    #[inline]
    pub fn query(self, ast: &Ast<'ast>) -> MediaList<'ast> {
        ast.data(self).query
    }
    #[inline]
    pub fn set_query(self, ast: &mut Ast<'ast>, value: MediaList<'ast>) {
        let mut data = ast.data(self);
        data.query = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for MediaRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MediaRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_media_rule(&mut self, data: MediaRuleData<'ast>, span: Span) -> MediaRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SupportsRule<'ast>(pub(crate) NodeId<'ast, SupportsRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SupportsRuleData<'ast> {
    pub condition: SupportsCondition<'ast>,
}
impl<'ast> NodeStorage<'ast> for SupportsRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_SUPPORTS_RULE;
    type Data = SupportsRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SupportsRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SupportsRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> SupportsRule<'ast> {
    #[inline]
    pub fn condition(self, ast: &Ast<'ast>) -> SupportsCondition<'ast> {
        ast.data(self).condition
    }
    #[inline]
    pub fn set_condition(self, ast: &mut Ast<'ast>, value: SupportsCondition<'ast>) {
        let mut data = ast.data(self);
        data.condition = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for SupportsRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for SupportsRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_supports_rule(
        &mut self,
        data: SupportsRuleData<'ast>,
        span: Span,
    ) -> SupportsRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StartingStyleRule<'ast>(pub(crate) NodeId<'ast, StartingStyleRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StartingStyleRuleData {}
impl<'ast> NodeStorage<'ast> for StartingStyleRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_STARTING_STYLE_RULE;
    type Data = StartingStyleRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, StartingStyleRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, StartingStyleRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> StartingStyleRule<'ast> {}
impl<'ast> GetSpan<'ast> for StartingStyleRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for StartingStyleRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_starting_style_rule(
        &mut self,
        data: StartingStyleRuleData,
        span: Span,
    ) -> StartingStyleRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LayerStatementRule<'ast>(pub(crate) NodeId<'ast, LayerStatementRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayerStatementRuleData<'ast> {
    pub names: &'ast [&'ast [&'ast str]],
}
impl<'ast> NodeStorage<'ast> for LayerStatementRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_LAYER_STATEMENT_RULE;
    type Data = LayerStatementRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, LayerStatementRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, LayerStatementRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> LayerStatementRule<'ast> {
    #[inline]
    pub fn names(self, ast: &Ast<'ast>) -> &'ast [&'ast [&'ast str]] {
        ast.data(self).names
    }
    #[inline]
    pub fn set_names(self, ast: &mut Ast<'ast>, value: &'ast [&'ast [&'ast str]]) {
        let mut data = ast.data(self);
        data.names = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for LayerStatementRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for LayerStatementRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_layer_statement_rule(
        &mut self,
        data: LayerStatementRuleData<'ast>,
        span: Span,
    ) -> LayerStatementRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LayerBlockRule<'ast>(pub(crate) NodeId<'ast, LayerBlockRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayerBlockRuleData<'ast> {
    pub name: Option<&'ast [&'ast str]>,
}
impl<'ast> NodeStorage<'ast> for LayerBlockRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_LAYER_BLOCK_RULE;
    type Data = LayerBlockRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, LayerBlockRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, LayerBlockRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> LayerBlockRule<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> Option<&'ast [&'ast str]> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: Option<&'ast [&'ast str]>) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for LayerBlockRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for LayerBlockRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_layer_block_rule(
        &mut self,
        data: LayerBlockRuleData<'ast>,
        span: Span,
    ) -> LayerBlockRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ContainerRule<'ast>(pub(crate) NodeId<'ast, ContainerRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContainerRuleData<'ast> {
    pub name: Option<&'ast str>,
    pub condition: Option<ContainerCondition<'ast>>,
}
impl<'ast> NodeStorage<'ast> for ContainerRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_CONTAINER_RULE;
    type Data = ContainerRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ContainerRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ContainerRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ContainerRule<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> Option<&'ast str> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: Option<&'ast str>) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn condition(self, ast: &Ast<'ast>) -> Option<ContainerCondition<'ast>> {
        ast.data(self).condition
    }
    #[inline]
    pub fn set_condition(self, ast: &mut Ast<'ast>, value: Option<ContainerCondition<'ast>>) {
        let mut data = ast.data(self);
        data.condition = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ContainerRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ContainerRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_container_rule(
        &mut self,
        data: ContainerRuleData<'ast>,
        span: Span,
    ) -> ContainerRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScopeRule<'ast>(pub(crate) NodeId<'ast, ScopeRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScopeRuleData<'ast> {
    pub scope_start: Option<SelectorList<'ast>>,
    pub scope_end: Option<SelectorList<'ast>>,
}
impl<'ast> NodeStorage<'ast> for ScopeRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCOPE_RULE;
    type Data = ScopeRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScopeRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScopeRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScopeRule<'ast> {
    #[inline]
    pub fn scope_start(self, ast: &Ast<'ast>) -> Option<SelectorList<'ast>> {
        ast.data(self).scope_start
    }
    #[inline]
    pub fn set_scope_start(self, ast: &mut Ast<'ast>, value: Option<SelectorList<'ast>>) {
        let mut data = ast.data(self);
        data.scope_start = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn scope_end(self, ast: &Ast<'ast>) -> Option<SelectorList<'ast>> {
        ast.data(self).scope_end
    }
    #[inline]
    pub fn set_scope_end(self, ast: &mut Ast<'ast>, value: Option<SelectorList<'ast>>) {
        let mut data = ast.data(self);
        data.scope_end = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ScopeRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScopeRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scope_rule(&mut self, data: ScopeRuleData<'ast>, span: Span) -> ScopeRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MozDocumentRule<'ast>(pub(crate) NodeId<'ast, MozDocumentRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MozDocumentRuleData {}
impl<'ast> NodeStorage<'ast> for MozDocumentRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_MOZ_DOCUMENT_RULE;
    type Data = MozDocumentRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MozDocumentRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MozDocumentRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> MozDocumentRule<'ast> {}
impl<'ast> GetSpan<'ast> for MozDocumentRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MozDocumentRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_moz_document_rule(
        &mut self,
        data: MozDocumentRuleData,
        span: Span,
    ) -> MozDocumentRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UnknownAtRule<'ast>(pub(crate) NodeId<'ast, UnknownAtRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnknownAtRuleData<'ast> {
    pub name: &'ast str,
    pub prelude: &'ast [TokenOrValue<'ast>],
    pub block: Option<&'ast [TokenOrValue<'ast>]>,
}
impl<'ast> NodeStorage<'ast> for UnknownAtRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_UNKNOWN_AT_RULE;
    type Data = UnknownAtRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, UnknownAtRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, UnknownAtRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> UnknownAtRule<'ast> {
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
    pub fn prelude(self, ast: &Ast<'ast>) -> &'ast [TokenOrValue<'ast>] {
        ast.data(self).prelude
    }
    #[inline]
    pub fn set_prelude(self, ast: &mut Ast<'ast>, value: &'ast [TokenOrValue<'ast>]) {
        let mut data = ast.data(self);
        data.prelude = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn block(self, ast: &Ast<'ast>) -> Option<&'ast [TokenOrValue<'ast>]> {
        ast.data(self).block
    }
    #[inline]
    pub fn set_block(self, ast: &mut Ast<'ast>, value: Option<&'ast [TokenOrValue<'ast>]>) {
        let mut data = ast.data(self);
        data.block = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for UnknownAtRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for UnknownAtRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_unknown_at_rule(
        &mut self,
        data: UnknownAtRuleData<'ast>,
        span: Span,
    ) -> UnknownAtRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CounterStyleRule<'ast>(pub(crate) NodeId<'ast, CounterStyleRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CounterStyleRuleData<'ast> {
    pub name: &'ast str,
}
impl<'ast> NodeStorage<'ast> for CounterStyleRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_COUNTER_STYLE_RULE;
    type Data = CounterStyleRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CounterStyleRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CounterStyleRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CounterStyleRule<'ast> {
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
}
impl<'ast> GetSpan<'ast> for CounterStyleRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CounterStyleRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_counter_style_rule(
        &mut self,
        data: CounterStyleRuleData<'ast>,
        span: Span,
    ) -> CounterStyleRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ViewportRule<'ast>(pub(crate) NodeId<'ast, ViewportRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportRuleData {
    pub vendor_prefix: VendorPrefix,
}
impl<'ast> NodeStorage<'ast> for ViewportRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_VIEWPORT_RULE;
    type Data = ViewportRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ViewportRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ViewportRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> ViewportRule<'ast> {
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
impl<'ast> GetSpan<'ast> for ViewportRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ViewportRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_viewport_rule(
        &mut self,
        data: ViewportRuleData,
        span: Span,
    ) -> ViewportRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PositionTryRule<'ast>(pub(crate) NodeId<'ast, PositionTryRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionTryRuleData<'ast> {
    pub name: &'ast str,
}
impl<'ast> NodeStorage<'ast> for PositionTryRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_POSITION_TRY_RULE;
    type Data = PositionTryRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PositionTryRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PositionTryRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PositionTryRule<'ast> {
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
}
impl<'ast> GetSpan<'ast> for PositionTryRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PositionTryRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_position_try_rule(
        &mut self,
        data: PositionTryRuleData<'ast>,
        span: Span,
    ) -> PositionTryRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontFaceRule<'ast>(pub(crate) NodeId<'ast, FontFaceRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontFaceRuleData {}
impl<'ast> NodeStorage<'ast> for FontFaceRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_FACE_RULE;
    type Data = FontFaceRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontFaceRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontFaceRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> FontFaceRule<'ast> {}
impl<'ast> GetSpan<'ast> for FontFaceRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontFaceRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_face_rule(
        &mut self,
        data: FontFaceRuleData,
        span: Span,
    ) -> FontFaceRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontPaletteValuesRule<'ast>(pub(crate) NodeId<'ast, FontPaletteValuesRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontPaletteValuesRuleData<'ast> {
    pub name: &'ast str,
}
impl<'ast> NodeStorage<'ast> for FontPaletteValuesRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_PALETTE_VALUES_RULE;
    type Data = FontPaletteValuesRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontPaletteValuesRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontPaletteValuesRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FontPaletteValuesRule<'ast> {
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
}
impl<'ast> GetSpan<'ast> for FontPaletteValuesRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontPaletteValuesRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_palette_values_rule(
        &mut self,
        data: FontPaletteValuesRuleData<'ast>,
        span: Span,
    ) -> FontPaletteValuesRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ViewTransitionRule<'ast>(pub(crate) NodeId<'ast, ViewTransitionRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewTransitionRuleData {}
impl<'ast> NodeStorage<'ast> for ViewTransitionRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_VIEW_TRANSITION_RULE;
    type Data = ViewTransitionRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ViewTransitionRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ViewTransitionRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> ViewTransitionRule<'ast> {}
impl<'ast> GetSpan<'ast> for ViewTransitionRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ViewTransitionRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_view_transition_rule(
        &mut self,
        data: ViewTransitionRuleData,
        span: Span,
    ) -> ViewTransitionRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct KeyframesRule<'ast>(pub(crate) NodeId<'ast, KeyframesRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyframesRuleData<'ast> {
    pub name: KeyframesName<'ast>,
    pub vendor_prefix: VendorPrefix,
}
impl<'ast> NodeStorage<'ast> for KeyframesRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_KEYFRAMES_RULE;
    type Data = KeyframesRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, KeyframesRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, KeyframesRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> KeyframesRule<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> KeyframesName<'ast> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: KeyframesName<'ast>) {
        let mut data = ast.data(self);
        data.name = value;
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
impl<'ast> GetSpan<'ast> for KeyframesRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for KeyframesRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_keyframes_rule(
        &mut self,
        data: KeyframesRuleData<'ast>,
        span: Span,
    ) -> KeyframesRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Keyframe<'ast>(pub(crate) NodeId<'ast, KeyframeData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyframeData<'ast> {
    pub selectors: &'ast [KeyframeSelector<'ast>],
}
impl<'ast> NodeStorage<'ast> for Keyframe<'ast> {
    const KIND: NodeKind = super::kind::KIND_KEYFRAME;
    type Data = KeyframeData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, KeyframeData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, KeyframeData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Keyframe<'ast> {
    #[inline]
    pub fn selectors(self, ast: &Ast<'ast>) -> &'ast [KeyframeSelector<'ast>] {
        ast.data(self).selectors
    }
    #[inline]
    pub fn set_selectors(self, ast: &mut Ast<'ast>, value: &'ast [KeyframeSelector<'ast>]) {
        let mut data = ast.data(self);
        data.selectors = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Keyframe<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Keyframe<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_keyframe(&mut self, data: KeyframeData<'ast>, span: Span) -> Keyframe<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PageRule<'ast>(pub(crate) NodeId<'ast, PageRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageRuleData<'ast> {
    pub selectors: &'ast [PageSelector<'ast>],
}
impl<'ast> NodeStorage<'ast> for PageRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_PAGE_RULE;
    type Data = PageRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PageRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PageRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PageRule<'ast> {
    #[inline]
    pub fn selectors(self, ast: &Ast<'ast>) -> &'ast [PageSelector<'ast>] {
        ast.data(self).selectors
    }
    #[inline]
    pub fn set_selectors(self, ast: &mut Ast<'ast>, value: &'ast [PageSelector<'ast>]) {
        let mut data = ast.data(self);
        data.selectors = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PageRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PageRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_page_rule(&mut self, data: PageRuleData<'ast>, span: Span) -> PageRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PageMarginRule<'ast>(pub(crate) NodeId<'ast, PageMarginRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageMarginRuleData {
    pub margin_box: PageMarginBox,
}
impl<'ast> NodeStorage<'ast> for PageMarginRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_PAGE_MARGIN_RULE;
    type Data = PageMarginRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PageMarginRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PageMarginRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> PageMarginRule<'ast> {
    #[inline]
    pub fn margin_box(self, ast: &Ast<'ast>) -> PageMarginBox {
        ast.data(self).margin_box
    }
    #[inline]
    pub fn set_margin_box(self, ast: &mut Ast<'ast>, value: PageMarginBox) {
        let mut data = ast.data(self);
        data.margin_box = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PageMarginRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PageMarginRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_page_margin_rule(
        &mut self,
        data: PageMarginRuleData,
        span: Span,
    ) -> PageMarginRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PageDeclarationsRule<'ast>(pub(crate) NodeId<'ast, PageDeclarationsRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageDeclarationsRuleData {}
impl<'ast> NodeStorage<'ast> for PageDeclarationsRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_PAGE_DECLARATIONS_RULE;
    type Data = PageDeclarationsRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PageDeclarationsRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PageDeclarationsRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> PageDeclarationsRule<'ast> {}
impl<'ast> GetSpan<'ast> for PageDeclarationsRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PageDeclarationsRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_page_declarations_rule(
        &mut self,
        data: PageDeclarationsRuleData,
        span: Span,
    ) -> PageDeclarationsRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NestingRule<'ast>(pub(crate) NodeId<'ast, NestingRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NestingRuleData<'ast> {
    pub selectors: SelectorList<'ast>,
}
impl<'ast> NodeStorage<'ast> for NestingRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_NESTING_RULE;
    type Data = NestingRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, NestingRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, NestingRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> NestingRule<'ast> {
    #[inline]
    pub fn selectors(self, ast: &Ast<'ast>) -> SelectorList<'ast> {
        ast.data(self).selectors
    }
    #[inline]
    pub fn set_selectors(self, ast: &mut Ast<'ast>, value: SelectorList<'ast>) {
        let mut data = ast.data(self);
        data.selectors = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for NestingRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for NestingRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_nesting_rule(
        &mut self,
        data: NestingRuleData<'ast>,
        span: Span,
    ) -> NestingRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontFeatureValuesRule<'ast>(pub(crate) NodeId<'ast, FontFeatureValuesRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontFeatureValuesRuleData<'ast> {
    pub name: &'ast [FamilyName<'ast>],
}
impl<'ast> NodeStorage<'ast> for FontFeatureValuesRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_FEATURE_VALUES_RULE;
    type Data = FontFeatureValuesRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontFeatureValuesRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontFeatureValuesRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FontFeatureValuesRule<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> &'ast [FamilyName<'ast>] {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: &'ast [FamilyName<'ast>]) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for FontFeatureValuesRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontFeatureValuesRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_feature_values_rule(
        &mut self,
        data: FontFeatureValuesRuleData<'ast>,
        span: Span,
    ) -> FontFeatureValuesRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontFeatureSubruleRule<'ast>(pub(crate) NodeId<'ast, FontFeatureSubruleRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontFeatureSubruleRuleData {
    pub name: FontFeatureSubruleType,
}
impl<'ast> NodeStorage<'ast> for FontFeatureSubruleRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_FEATURE_SUBRULE_RULE;
    type Data = FontFeatureSubruleRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontFeatureSubruleRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontFeatureSubruleRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> FontFeatureSubruleRule<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> FontFeatureSubruleType {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: FontFeatureSubruleType) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for FontFeatureSubruleRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontFeatureSubruleRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_feature_subrule_rule(
        &mut self,
        data: FontFeatureSubruleRuleData,
        span: Span,
    ) -> FontFeatureSubruleRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PropertyRule<'ast>(pub(crate) NodeId<'ast, PropertyRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PropertyRuleData<'ast> {
    pub name: &'ast str,
    pub syntax: Option<SyntaxString<'ast>>,
    pub inherits: Option<bool>,
    pub initial_value: Option<ParsedComponent<'ast>>,
}
impl<'ast> NodeStorage<'ast> for PropertyRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_PROPERTY_RULE;
    type Data = PropertyRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PropertyRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PropertyRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PropertyRule<'ast> {
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
    pub fn syntax(self, ast: &Ast<'ast>) -> Option<SyntaxString<'ast>> {
        ast.data(self).syntax
    }
    #[inline]
    pub fn set_syntax(self, ast: &mut Ast<'ast>, value: Option<SyntaxString<'ast>>) {
        let mut data = ast.data(self);
        data.syntax = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn inherits(self, ast: &Ast<'ast>) -> Option<bool> {
        ast.data(self).inherits
    }
    #[inline]
    pub fn set_inherits(self, ast: &mut Ast<'ast>, value: Option<bool>) {
        let mut data = ast.data(self);
        data.inherits = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn initial_value(self, ast: &Ast<'ast>) -> Option<ParsedComponent<'ast>> {
        ast.data(self).initial_value
    }
    #[inline]
    pub fn set_initial_value(self, ast: &mut Ast<'ast>, value: Option<ParsedComponent<'ast>>) {
        let mut data = ast.data(self);
        data.initial_value = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PropertyRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PropertyRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_property_rule(
        &mut self,
        data: PropertyRuleData<'ast>,
        span: Span,
    ) -> PropertyRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NestedDeclarationsRule<'ast>(pub(crate) NodeId<'ast, NestedDeclarationsRuleData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NestedDeclarationsRuleData {}
impl<'ast> NodeStorage<'ast> for NestedDeclarationsRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_NESTED_DECLARATIONS_RULE;
    type Data = NestedDeclarationsRuleData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, NestedDeclarationsRuleData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, NestedDeclarationsRuleData>) -> Self {
        Self(id)
    }
}
impl<'ast> NestedDeclarationsRule<'ast> {}
impl<'ast> GetSpan<'ast> for NestedDeclarationsRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for NestedDeclarationsRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_nested_declarations_rule(
        &mut self,
        data: NestedDeclarationsRuleData,
        span: Span,
    ) -> NestedDeclarationsRule<'ast> {
        self.alloc(data, span)
    }
}
