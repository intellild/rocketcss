use crate::{
    Box, CharsetRule, ContainerCondition, CustomMediaRule, CustomProperty, Declaration, FamilyName,
    FontFaceProperty, FontFeatureDeclaration, FontFeatureSubruleType, FontPaletteValuesProperty,
    ImportRule, KeyframeSelector, KeyframesName, MediaList, NamespaceRule, PageMarginBox,
    PageSelector, ParsedComponent, SelectorList, Span, SupportsCondition, SyntaxString,
    TokenOrValue, Vec, VendorPrefix, ViewTransitionProperty,
};

use super::{DeclarationId, RadixIdRemap, RuleId, RuleIdReferences, SelectorValueId};

/// One typed descriptor occurrence inside `@property`.
#[derive(Debug, PartialEq)]
pub enum PropertyRuleDescriptor<'ast> {
    Syntax(Box<'ast, SyntaxString<'ast>>),
    Inherits(bool),
    InitialValue(Box<'ast, ParsedComponent<'ast>>),
    Unknown(Box<'ast, CustomProperty<'ast>>),
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

/// Rule payloads stored by the compiler-owned Radix AST.
#[derive(Debug, PartialEq)]
pub enum CssRulePayload<'ast> {
    Style(StyleRulePayload),
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
    Nesting(NestingRulePayload),
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

impl RuleIdReferences for CssRulePayload<'_> {
    #[inline]
    fn remap_rule_ids(&mut self, _remaps: &[RadixIdRemap<RuleId>]) {}
}

#[derive(Debug, PartialEq)]
pub struct StyleRulePayload {
    pub span: Span,
    pub selector_value: SelectorValueId,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq)]
pub struct MediaRulePayload<'ast> {
    pub span: Span,
    pub query: MediaList<'ast>,
}

#[derive(Debug, PartialEq)]
pub struct SupportsRulePayload<'ast> {
    pub span: Span,
    pub condition: SupportsCondition<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartingStyleRulePayload {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct LayerStatementRulePayload<'ast> {
    pub span: Span,
    pub names: Vec<'ast, Vec<'ast, &'ast str>>,
}

#[derive(Debug, PartialEq)]
pub struct LayerBlockRulePayload<'ast> {
    pub span: Span,
    pub name: Option<Vec<'ast, &'ast str>>,
}

#[derive(Debug, PartialEq)]
pub struct ContainerRulePayload<'ast> {
    pub span: Span,
    pub name: Option<&'ast str>,
    pub condition: Option<Box<'ast, ContainerCondition<'ast>>>,
}

#[derive(Debug, PartialEq)]
pub struct ScopeRulePayload<'ast> {
    pub span: Span,
    pub scope_start: Option<Box<'ast, SelectorList<'ast>>>,
    pub scope_end: Option<Box<'ast, SelectorList<'ast>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MozDocumentRulePayload {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct UnknownAtRulePayload<'ast> {
    pub span: Span,
    pub name: &'ast str,
    pub prelude: Vec<'ast, TokenOrValue<'ast>>,
    pub block: Option<Vec<'ast, TokenOrValue<'ast>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CounterStyleRulePayload<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewportRulePayload {
    pub span: Span,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionTryRulePayload<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontFaceRulePayload {
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontPaletteValuesRulePayload<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewTransitionRulePayload {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct KeyframesRulePayload<'ast> {
    pub span: Span,
    pub name: Box<'ast, KeyframesName<'ast>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq)]
pub struct KeyframePayload<'ast> {
    pub selectors: Vec<'ast, KeyframeSelector>,
}

#[derive(Debug, PartialEq)]
pub struct PageRulePayload<'ast> {
    pub span: Span,
    pub selectors: Vec<'ast, PageSelector<'ast>>,
}

#[derive(Debug, PartialEq)]
pub struct PageMarginPayload {
    pub span: Span,
    pub margin_box: PageMarginBox,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageDeclarationsPayload {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct NestingRulePayload {
    pub span: Span,
    pub selector_value: SelectorValueId,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureValuesRulePayload<'ast> {
    pub span: Span,
    pub name: Vec<'ast, FamilyName<'ast>>,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureSubrulePayload {
    pub span: Span,
    pub name: FontFeatureSubruleType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropertyRulePayload<'ast> {
    pub span: Span,
    pub name: &'ast str,
    pub syntax: Option<DeclarationId>,
    pub inherits: Option<DeclarationId>,
    pub initial_value: Option<DeclarationId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NestedDeclarationsPayload {
    pub span: Span,
}
