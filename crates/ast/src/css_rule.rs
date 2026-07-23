use super::*;

use rocketcss_allocator::boxed::Box;

#[derive(Debug, PartialEq, Visit)]
pub enum CssRule<'a, 'ghost> {
    Media(Box<'a, MediaRule<'a, 'ghost>>),
    Import(Box<'a, ImportRule<'a>>),
    Style(Ref<'a, 'ghost, StyleRule<'a, 'ghost>>),
    Keyframes(Box<'a, KeyframesRule<'a, 'ghost>>),
    FontFace(Box<'a, FontFaceRule<'a>>),
    FontPaletteValues(Box<'a, FontPaletteValuesRule<'a>>),
    FontFeatureValues(Box<'a, FontFeatureValuesRule<'a>>),
    Page(Box<'a, PageRule<'a, 'ghost>>),
    Supports(Box<'a, SupportsRule<'a, 'ghost>>),
    CounterStyle(Box<'a, CounterStyleRule<'a, 'ghost>>),
    Charset(Box<'a, CharsetRule<'a>>),
    Namespace(Box<'a, NamespaceRule<'a>>),
    MozDocument(Box<'a, MozDocumentRule<'a, 'ghost>>),
    Nesting(Box<'a, NestingRule<'a, 'ghost>>),
    NestedDeclarations(Box<'a, NestedDeclarationsRule<'a, 'ghost>>),
    Viewport(Box<'a, ViewportRule<'a, 'ghost>>),
    CustomMedia(Box<'a, CustomMediaRule<'a>>),
    LayerStatement(Box<'a, LayerStatementRule<'a>>),
    LayerBlock(Box<'a, LayerBlockRule<'a, 'ghost>>),
    Property(Box<'a, PropertyRule<'a>>),
    Container(Box<'a, ContainerRule<'a, 'ghost>>),
    Scope(Box<'a, ScopeRule<'a, 'ghost>>),
    StartingStyle(Box<'a, StartingStyleRule<'a, 'ghost>>),
    ViewTransition(Box<'a, ViewTransitionRule<'a>>),
    PositionTry(Box<'a, PositionTryRule<'a, 'ghost>>),
    Unknown(Box<'a, UnknownAtRule<'a>>),
    Custom(Box<'a, DefaultAtRule>),
}

impl<'ghost> CssRule<'_, 'ghost> {
    #[inline]
    pub fn span(&self, token: &GhostToken<'ghost>) -> Span {
        match self {
            Self::Media(rule) => rule.span(),
            Self::Import(rule) => rule.span(),
            Self::Style(rule) => rule.get(token).span,
            Self::Keyframes(rule) => rule.span(),
            Self::FontFace(rule) => rule.span(),
            Self::FontPaletteValues(rule) => rule.span(),
            Self::FontFeatureValues(rule) => rule.span(),
            Self::Page(rule) => rule.span(),
            Self::Supports(rule) => rule.span(),
            Self::CounterStyle(rule) => rule.span(),
            Self::Charset(rule) => rule.span(),
            Self::Namespace(rule) => rule.span(),
            Self::MozDocument(rule) => rule.span(),
            Self::Nesting(rule) => rule.span(),
            Self::NestedDeclarations(rule) => rule.span(),
            Self::Viewport(rule) => rule.span(),
            Self::CustomMedia(rule) => rule.span(),
            Self::LayerStatement(rule) => rule.span(),
            Self::LayerBlock(rule) => rule.span(),
            Self::Property(rule) => rule.span(),
            Self::Container(rule) => rule.span(),
            Self::Scope(rule) => rule.span(),
            Self::StartingStyle(rule) => rule.span(),
            Self::ViewTransition(rule) => rule.span(),
            Self::PositionTry(rule) => rule.span(),
            Self::Unknown(rule) => rule.span(),
            Self::Custom(_) => DUMMY_SP,
        }
    }
}
