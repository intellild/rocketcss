use crate::{
    AstStr, CharsetRule, ContainerCondition, CustomMediaRule, CustomProperty, Declaration,
    FamilyName, FontFaceProperty, FontFeatureDeclaration, FontFeatureSubruleType,
    FontPaletteValuesProperty, ImportRule, KeyframeSelector, KeyframesName, MediaList,
    NamespaceRule, NodeId, PageMarginBox, PageSelector, ParsedComponent, SelectorList,
    SupportsCondition, SyntaxString, TokenOrValue, Vec, VendorPrefix, ViewTransitionProperty,
};

use super::{DeclarationId, SelectorValueId};

/// One typed descriptor occurrence inside `@property`.
#[derive(Debug, PartialEq)]
pub enum PropertyRuleDescriptor<'ast> {
    Syntax(NodeId<'ast, SyntaxString<'ast>>),
    Inherits(bool),
    InitialValue(NodeId<'ast, ParsedComponent<'ast>>),
    Unknown(NodeId<'ast, CustomProperty<'ast>>),
}

/// One heterogeneous occurrence in the global authored declaration tape.
#[derive(Debug, PartialEq)]
pub enum DeclarationPayload<'ast> {
    Property(Declaration<'ast>),
    FontFace(FontFaceProperty<'ast>),
    FontPaletteValues(FontPaletteValuesProperty<'ast>),
    ViewTransition(ViewTransitionProperty<'ast>),
    FontFeature(FontFeatureDeclaration<'ast>),
    PropertyRule(PropertyRuleDescriptor<'ast>),
}

impl<'ast> DeclarationPayload<'ast> {
    #[inline]
    pub const fn as_property(&self) -> Option<&Declaration<'ast>> {
        match self {
            Self::Property(declaration) => Some(declaration),
            Self::FontFace(_)
            | Self::FontPaletteValues(_)
            | Self::ViewTransition(_)
            | Self::FontFeature(_)
            | Self::PropertyRule(_) => None,
        }
    }
}

/// Rule payloads stored by the compiler-owned AST.
#[derive(Debug, PartialEq)]
pub enum CssRulePayload<'ast> {
    Style(StyleRulePayload<'ast>),
    Media(MediaRulePayload<'ast>),
    Supports(SupportsRulePayload<'ast>),
    StartingStyle(StartingStyleRulePayload),
    LayerStatement(LayerStatementRulePayload<'ast>),
    LayerBlock(LayerBlockRulePayload<'ast>),
    Container(ContainerRulePayload<'ast>),
    Scope(ScopeRulePayload<'ast>),
    MozDocument(MozDocumentRulePayload),
    Unknown(UnknownAtRulePayload<'ast>),
    CounterStyle(CounterStyleRulePayload<'ast>),
    Viewport(ViewportRulePayload),
    PositionTry(PositionTryRulePayload<'ast>),
    FontFace(FontFaceRulePayload),
    FontPaletteValues(FontPaletteValuesRulePayload<'ast>),
    ViewTransition(ViewTransitionRulePayload),
    Import(ImportRule<'ast>),
    Charset(CharsetRule<'ast>),
    Namespace(NamespaceRule<'ast>),
    CustomMedia(CustomMediaRule<'ast>),
    Keyframes(KeyframesRulePayload<'ast>),
    Keyframe(KeyframePayload<'ast>),
    Page(PageRulePayload<'ast>),
    PageMargin(PageMarginPayload),
    PageDeclarations(PageDeclarationsPayload),
    Nesting(NestingRulePayload<'ast>),
    FontFeatureValues(FontFeatureValuesRulePayload<'ast>),
    FontFeatureSubrule(FontFeatureSubrulePayload),
    Property(PropertyRulePayload<'ast>),
    NestedDeclarations(NestedDeclarationsPayload),
}

impl CssRulePayload<'_> {
    /// Returns whether this rule owns ordinary CSS property declarations.
    ///
    /// Descriptor families share the physical declaration tape but keep their
    /// distinct typed visitor and minifier semantics.
    #[inline]
    pub const fn owns_property_declarations(&self) -> bool {
        matches!(
            self,
            Self::Style(_)
                | Self::CounterStyle(_)
                | Self::Viewport(_)
                | Self::PositionTry(_)
                | Self::Keyframe(_)
                | Self::PageMargin(_)
                | Self::PageDeclarations(_)
                | Self::Nesting(_)
                | Self::NestedDeclarations(_)
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct StyleRulePayload<'ast> {
    pub selector_value: SelectorValueId<'ast>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq)]
pub struct MediaRulePayload<'ast> {
    pub query: MediaList<'ast>,
}

#[derive(Debug, PartialEq)]
pub struct SupportsRulePayload<'ast> {
    pub condition: SupportsCondition<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartingStyleRulePayload;

#[derive(Debug, PartialEq)]
pub struct LayerStatementRulePayload<'ast> {
    pub names: Vec<'ast, Vec<'ast, AstStr<'ast>>>,
}

#[derive(Debug, PartialEq)]
pub struct LayerBlockRulePayload<'ast> {
    pub name: Option<Vec<'ast, AstStr<'ast>>>,
}

#[derive(Debug, PartialEq)]
pub struct ContainerRulePayload<'ast> {
    pub name: Option<AstStr<'ast>>,
    pub condition: Option<NodeId<'ast, ContainerCondition<'ast>>>,
}

#[derive(Debug, PartialEq)]
pub struct ScopeRulePayload<'ast> {
    pub scope_start: Option<SelectorList<'ast>>,
    pub scope_end: Option<SelectorList<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MozDocumentRulePayload;

#[derive(Debug, PartialEq)]
pub struct UnknownAtRulePayload<'ast> {
    pub name: AstStr<'ast>,
    pub prelude: Vec<'ast, TokenOrValue<'ast>>,
    pub block: Option<Vec<'ast, TokenOrValue<'ast>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CounterStyleRulePayload<'ast> {
    pub name: AstStr<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewportRulePayload {
    pub vendor_prefix: VendorPrefix,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionTryRulePayload<'ast> {
    pub name: AstStr<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontFaceRulePayload;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontPaletteValuesRulePayload<'ast> {
    pub name: AstStr<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewTransitionRulePayload;

#[derive(Debug, PartialEq)]
pub struct KeyframesRulePayload<'ast> {
    pub name: NodeId<'ast, KeyframesName<'ast>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq)]
pub struct KeyframePayload<'ast> {
    pub selectors: Vec<'ast, KeyframeSelector>,
}

#[derive(Debug, PartialEq)]
pub struct PageRulePayload<'ast> {
    pub selectors: Vec<'ast, NodeId<'ast, PageSelector<'ast>>>,
}

#[derive(Debug, PartialEq)]
pub struct PageMarginPayload {
    pub margin_box: PageMarginBox,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageDeclarationsPayload;

#[derive(Debug, PartialEq)]
pub struct NestingRulePayload<'ast> {
    pub selector_value: SelectorValueId<'ast>,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureValuesRulePayload<'ast> {
    pub name: Vec<'ast, FamilyName<'ast>>,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureSubrulePayload {
    pub name: FontFeatureSubruleType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropertyRulePayload<'ast> {
    pub name: AstStr<'ast>,
    pub syntax: Option<DeclarationId<'ast>>,
    pub inherits: Option<DeclarationId<'ast>>,
    pub initial_value: Option<DeclarationId<'ast>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NestedDeclarationsPayload;
