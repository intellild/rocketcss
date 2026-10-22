//! Persistent rule tree: stylesheet root, rule nodes, and declaration blocks.

/// The root of one parsed stylesheet.
#[ast]
pub struct StyleSheet<'a> {
    pub rules: Vec<'a, CssRule<'a>>,
}

/// One authored or synthesized rule with its direct children and declarations.
#[ast]
pub struct CssRule<'a> {
    pub payload: CssRulePayload<'a>,
    pub declarations: Option<DeclarationBlock<'a>>,
    pub children: Vec<'a, CssRule<'a>>,
}

/// One contiguous declaration block owned by a rule.
#[ast]
pub struct DeclarationBlock<'a> {
    pub declarations: Vec<'a, DeclarationPayload<'a>>,
}

/// One typed descriptor occurrence inside `@property`.
#[ast]
pub enum PropertyRuleDescriptor<'a> {
    Syntax(SyntaxString<'a>),
    Inherits(bool),
    InitialValue(ParsedComponent<'a>),
    Unknown(CustomProperty<'a>),
}

/// One heterogeneous occurrence in a declaration block.
#[ast]
pub enum DeclarationPayload<'a> {
    Property(Declaration<'a>),
    FontFace(FontFaceProperty<'a>),
    FontPaletteValues(FontPaletteValuesProperty<'a>),
    ViewTransition(ViewTransitionProperty<'a>),
    FontFeature(FontFeatureDeclaration<'a>),
    PropertyRule(PropertyRuleDescriptor<'a>),
}

/// Rule payloads stored by the flattened tree.
#[ast]
pub enum CssRulePayload<'a> {
    Style(StyleRule<'a>),
    Media(MediaRule<'a>),
    Supports(SupportsRule<'a>),
    StartingStyle(StartingStyleRule),
    LayerStatement(LayerStatementRule<'a>),
    LayerBlock(LayerBlockRule<'a>),
    Container(ContainerRule<'a>),
    Scope(ScopeRule<'a>),
    MozDocument(MozDocumentRule),
    Unknown(UnknownAtRule<'a>),
    CounterStyle(CounterStyleRule<'a>),
    Viewport(ViewportRule),
    PositionTry(PositionTryRule<'a>),
    FontFace(FontFaceRule),
    FontPaletteValues(FontPaletteValuesRule<'a>),
    ViewTransition(ViewTransitionRule),
    Import(ImportRule<'a>),
    Charset(CharsetRule<'a>),
    Namespace(NamespaceRule<'a>),
    CustomMedia(CustomMediaRule<'a>),
    Keyframes(KeyframesRule<'a>),
    Keyframe(Keyframe<'a>),
    Page(PageRule<'a>),
    PageMargin(PageMarginRule),
    PageDeclarations(PageDeclarationsRule),
    Nesting(NestingRule<'a>),
    FontFeatureValues(FontFeatureValuesRule<'a>),
    FontFeatureSubrule(FontFeatureSubruleRule),
    Property(PropertyRule<'a>),
    NestedDeclarations(NestedDeclarationsRule),
}

#[ast]
pub struct StyleRule<'a> {
    pub selectors: SelectorList<'a>,
    pub vendor_prefix: VendorPrefix,
}

#[ast]
pub struct MediaRule<'a> {
    pub query: MediaList<'a>,
}

#[ast]
pub struct SupportsRule<'a> {
    pub condition: SupportsCondition<'a>,
}

#[ast]
pub struct StartingStyleRule {}

#[ast]
pub struct LayerStatementRule<'a> {
    pub names: Vec<'a, Vec<'a, &'a str>>,
}

#[ast]
pub struct LayerBlockRule<'a> {
    pub name: Option<Vec<'a, &'a str>>,
}

#[ast]
pub struct ContainerRule<'a> {
    pub name: Option<&'a str>,
    pub condition: Option<ContainerCondition<'a>>,
}

#[ast]
pub struct ScopeRule<'a> {
    pub scope_start: Option<SelectorList<'a>>,
    pub scope_end: Option<SelectorList<'a>>,
}

#[ast]
pub struct MozDocumentRule {}

#[ast]
pub struct UnknownAtRule<'a> {
    pub name: &'a str,
    pub prelude: Vec<'a, TokenOrValue<'a>>,
    pub block: Option<Vec<'a, TokenOrValue<'a>>>,
}

#[ast]
pub struct CounterStyleRule<'a> {
    pub name: &'a str,
}

#[ast]
pub struct ViewportRule {
    pub vendor_prefix: VendorPrefix,
}

#[ast]
pub struct PositionTryRule<'a> {
    pub name: &'a str,
}

#[ast]
pub struct FontFaceRule {}

#[ast]
pub struct FontPaletteValuesRule<'a> {
    pub name: &'a str,
}

#[ast]
pub struct ViewTransitionRule {}

#[ast]
pub struct KeyframesRule<'a> {
    pub name: KeyframesName<'a>,
    pub vendor_prefix: VendorPrefix,
}

#[ast]
pub struct Keyframe<'a> {
    pub selectors: Vec<'a, KeyframeSelector>,
}

#[ast]
pub struct PageRule<'a> {
    pub selectors: Vec<'a, PageSelector<'a>>,
}

#[ast]
pub struct PageMarginRule {
    pub margin_box: PageMarginBox,
}

#[ast]
pub struct PageDeclarationsRule {}

#[ast]
pub struct NestingRule<'a> {
    pub selectors: SelectorList<'a>,
}

#[ast]
pub struct FontFeatureValuesRule<'a> {
    pub name: Vec<'a, FamilyName<'a>>,
}

#[ast]
pub struct FontFeatureSubruleRule {
    pub name: FontFeatureSubruleType,
}

#[ast]
pub struct PropertyRule<'a> {
    pub name: &'a str,
    pub syntax: Option<SyntaxString<'a>>,
    pub inherits: Option<bool>,
    pub initial_value: Option<ParsedComponent<'a>>,
}

#[ast]
pub struct NestedDeclarationsRule {}
