use super::*;

use rocketcss_allocator::boxed::Box;

#[derive(Debug, PartialEq)]
pub enum CssRule<'a> {
    Media(Box<'a, MediaRule<'a>>),
    Import(Box<'a, ImportRule<'a>>),
    Style(Box<'a, StyleRule<'a>>),
    Keyframes(Box<'a, KeyframesRule<'a>>),
    FontFace(Box<'a, FontFaceRule<'a>>),
    FontPaletteValues(Box<'a, FontPaletteValuesRule<'a>>),
    FontFeatureValues(Box<'a, FontFeatureValuesRule<'a>>),
    Page(Box<'a, PageRule<'a>>),
    Supports(Box<'a, SupportsRule<'a>>),
    CounterStyle(Box<'a, CounterStyleRule<'a>>),
    Namespace(Box<'a, NamespaceRule<'a>>),
    MozDocument(Box<'a, MozDocumentRule<'a>>),
    Nesting(Box<'a, NestingRule<'a>>),
    NestedDeclarations(Box<'a, NestedDeclarationsRule<'a>>),
    Viewport(Box<'a, ViewportRule<'a>>),
    CustomMedia(Box<'a, CustomMediaRule<'a>>),
    LayerStatement(Box<'a, LayerStatementRule<'a>>),
    LayerBlock(Box<'a, LayerBlockRule<'a>>),
    Property(Box<'a, PropertyRule<'a>>),
    Container(Box<'a, ContainerRule<'a>>),
    Scope(Box<'a, ScopeRule<'a>>),
    StartingStyle(Box<'a, StartingStyleRule<'a>>),
    ViewTransition(Box<'a, ViewTransitionRule<'a>>),
    PositionTry(Box<'a, PositionTryRule<'a>>),
    Ignored,
    Unknown(Box<'a, UnknownAtRule<'a>>),
    Custom(Box<'a, DefaultAtRule>),
}

impl GetSpan for CssRule<'_> {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Self::Media(rule) => rule.span(),
            Self::Import(rule) => rule.span(),
            Self::Style(rule) => rule.span(),
            Self::Keyframes(rule) => rule.span(),
            Self::FontFace(rule) => rule.span(),
            Self::FontPaletteValues(rule) => rule.span(),
            Self::FontFeatureValues(rule) => rule.span(),
            Self::Page(rule) => rule.span(),
            Self::Supports(rule) => rule.span(),
            Self::CounterStyle(rule) => rule.span(),
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
            Self::Ignored | Self::Custom(_) => DUMMY_SP,
        }
    }
}
