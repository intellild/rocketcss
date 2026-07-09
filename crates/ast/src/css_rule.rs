use super::*;

use rs_css_allocator::boxed::Box;

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
    Ignored,
    Unknown(Box<'a, UnknownAtRule<'a>>),
    Custom(Box<'a, DefaultAtRule>),
}
