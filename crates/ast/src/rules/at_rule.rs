use crate::*;
use std::pin::Pin;
#[derive(Debug, PartialEq, Visit)]
pub struct SupportsRule<'a, 'ghost> {
    pub condition: Box<'a, SupportsCondition<'a>>,
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a, 'ghost>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CounterStyleRule<'a, 'ghost> {
    pub declarations: GhostBox<'a, 'ghost, DeclarationBlock<'a, 'ghost>>,
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CharsetRule<'a> {
    pub span: Span,
    pub encoding: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct NamespaceRule<'a> {
    pub span: Span,
    pub prefix: Option<&'a str>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MozDocumentRule<'a, 'ghost> {
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a, 'ghost>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct NestingRule<'a, 'ghost> {
    pub span: Span,
    pub style: Pin<Box<'a, StyleRule<'a, 'ghost>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct NestedDeclarationsRule<'a, 'ghost> {
    pub declarations: GhostBox<'a, 'ghost, DeclarationBlock<'a, 'ghost>>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ViewportRule<'a, 'ghost> {
    pub declarations: GhostBox<'a, 'ghost, DeclarationBlock<'a, 'ghost>>,
    pub span: Span,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CustomMediaRule<'a> {
    pub span: Span,
    pub name: &'a str,
    pub query: MediaList<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct LayerStatementRule<'a> {
    pub span: Span,
    pub names: Vec<'a, Vec<'a, &'a str>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct LayerBlockRule<'a, 'ghost> {
    pub span: Span,
    pub name: Option<Vec<'a, &'a str>>,
    pub rules: Vec<'a, CssRule<'a, 'ghost>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScopeRule<'a, 'ghost> {
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a, 'ghost>>,
    pub scope_end: Option<Box<'a, SelectorList<'a>>>,
    pub scope_start: Option<Box<'a, SelectorList<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct StartingStyleRule<'a, 'ghost> {
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a, 'ghost>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PositionTryRule<'a, 'ghost> {
    pub span: Span,
    pub name: &'a str,
    pub declarations: GhostBox<'a, 'ghost, DeclarationBlock<'a, 'ghost>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct UnknownAtRule<'a> {
    pub block: Option<Vec<'a, TokenOrValue<'a>>>,
    pub span: Span,
    pub name: &'a str,
    pub prelude: Vec<'a, TokenOrValue<'a>>,
}
