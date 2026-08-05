use crate::{
    Box, CharsetRule, ContainerCondition, CustomMediaRule, FamilyName, FontFeatureSubruleType,
    ImportRule, KeyframeSelector, KeyframesName, MediaList, NamespaceRule, PageMarginBox,
    PageSelector, SelectorList, Span, SupportsCondition, TokenOrValue, Vec, VendorPrefix,
};

use super::{DeclarationId, RadixIdRemap, RuleIdReferences, SelectorValueId};

/// A CSS rule stored in a [`super::StyleSheet`].
#[derive(Debug, PartialEq)]
pub enum CssRule<'ast> {
    Style(StyleRule),
    Media(MediaRule<'ast>),
    Supports(SupportsRule<'ast>),
    StartingStyle(StartingStyleRule),
    LayerStatement(LayerStatementRule<'ast>),
    LayerBlock(LayerBlockRule<'ast>),
    Container(ContainerRule<'ast>),
    Scope(ScopeRule<'ast>),
    MozDocument(MozDocumentRule),
    Unknown(UnknownAtRule<'ast>),
    CounterStyle(CounterStyleRule<'ast>),
    Viewport(ViewportRule),
    PositionTry(PositionTryRule<'ast>),
    FontFace(FontFaceRule),
    FontPaletteValues(FontPaletteValuesRule<'ast>),
    ViewTransition(ViewTransitionRule),
    Import(ImportRule<'ast>),
    Charset(CharsetRule<'ast>),
    Namespace(NamespaceRule<'ast>),
    CustomMedia(CustomMediaRule<'ast>),
    Keyframes(KeyframesRule<'ast>),
    Keyframe(Keyframe<'ast>),
    Page(PageRule<'ast>),
    PageMargin(PageMarginRule),
    PageDeclarations(PageDeclarationsRule),
    Nesting(NestingRule),
    FontFeatureValues(FontFeatureValuesRule<'ast>),
    FontFeatureSubrule(FontFeatureSubrule),
    Property(PropertyRule<'ast>),
    NestedDeclarations(NestedDeclarationsRule),
}

impl CssRule<'_> {
    /// Returns whether this rule owns ordinary CSS property declarations.
    ///
    /// Descriptor families share the physical declaration arena but keep their
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

impl<'ast> RuleIdReferences<CssRule<'ast>> for CssRule<'ast> {
    #[inline]
    fn remap_rule_ids(&mut self, _remaps: &[RadixIdRemap<super::RuleId<CssRule<'ast>>>]) {}
}

#[derive(Debug, PartialEq)]
pub struct StyleRule {
    pub span: Span,
    pub selector_value: SelectorValueId,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq)]
pub struct MediaRule<'ast> {
    pub span: Span,
    pub query: MediaList<'ast>,
}

#[derive(Debug, PartialEq)]
pub struct SupportsRule<'ast> {
    pub span: Span,
    pub condition: SupportsCondition<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartingStyleRule {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct LayerStatementRule<'ast> {
    pub span: Span,
    pub names: Vec<'ast, Vec<'ast, &'ast str>>,
}

#[derive(Debug, PartialEq)]
pub struct LayerBlockRule<'ast> {
    pub span: Span,
    pub name: Option<Vec<'ast, &'ast str>>,
}

#[derive(Debug, PartialEq)]
pub struct ContainerRule<'ast> {
    pub span: Span,
    pub name: Option<&'ast str>,
    pub condition: Option<Box<'ast, ContainerCondition<'ast>>>,
}

#[derive(Debug, PartialEq)]
pub struct ScopeRule<'ast> {
    pub span: Span,
    pub scope_start: Option<Box<'ast, SelectorList<'ast>>>,
    pub scope_end: Option<Box<'ast, SelectorList<'ast>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MozDocumentRule {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct UnknownAtRule<'ast> {
    pub span: Span,
    pub name: &'ast str,
    pub prelude: Vec<'ast, TokenOrValue<'ast>>,
    pub block: Option<Vec<'ast, TokenOrValue<'ast>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CounterStyleRule<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewportRule {
    pub span: Span,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionTryRule<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontFaceRule {
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontPaletteValuesRule<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewTransitionRule {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct KeyframesRule<'ast> {
    pub span: Span,
    pub name: Box<'ast, KeyframesName<'ast>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq)]
pub struct Keyframe<'ast> {
    pub selectors: Vec<'ast, KeyframeSelector>,
}

#[derive(Debug, PartialEq)]
pub struct PageRule<'ast> {
    pub span: Span,
    pub selectors: Vec<'ast, PageSelector<'ast>>,
}

#[derive(Debug, PartialEq)]
pub struct PageMarginRule {
    pub span: Span,
    pub margin_box: PageMarginBox,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageDeclarationsRule {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct NestingRule {
    pub span: Span,
    pub selector_value: SelectorValueId,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureValuesRule<'ast> {
    pub span: Span,
    pub name: Vec<'ast, FamilyName<'ast>>,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureSubrule {
    pub span: Span,
    pub name: FontFeatureSubruleType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropertyRule<'ast> {
    pub span: Span,
    pub name: &'ast str,
    pub syntax: Option<DeclarationId>,
    pub inherits: Option<DeclarationId>,
    pub initial_value: Option<DeclarationId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NestedDeclarationsRule {
    pub span: Span,
}
