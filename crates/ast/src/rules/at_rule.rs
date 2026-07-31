use crate::*;
#[derive(Debug, PartialEq, Visit)]
pub struct SupportsRule<'a> {
    pub condition: std::boxed::Box<SupportsCondition<'a>>,
    pub span: Span,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CounterStyleRule<'a> {
    #[visit(with = visit_declaration_block_id, with_mut = visit_declaration_block_id_mut)]
    pub declarations: DeclarationBlockId,
    pub span: Span,
    pub name: Atom<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CharsetRule<'a> {
    pub span: Span,
    pub encoding: Atom<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct NamespaceRule<'a> {
    pub span: Span,
    pub prefix: Option<Atom<'a>>,
    pub url: Atom<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MozDocumentRule<'a> {
    pub span: Span,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
    #[visit(skip)]
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> MozDocumentRule<'a> {
    pub fn new(span: Span, rules: RuleListId) -> Self {
        Self {
            span,
            rules,
            marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct NestingRule<'a> {
    pub span: Span,
    pub style: StyleRule<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct NestedDeclarationsRule {
    #[visit(with = visit_declaration_block_id, with_mut = visit_declaration_block_id_mut)]
    pub declarations: DeclarationBlockId,
    pub span: Span,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ViewportRule {
    #[visit(with = visit_declaration_block_id, with_mut = visit_declaration_block_id_mut)]
    pub declarations: DeclarationBlockId,
    pub span: Span,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CustomMediaRule<'a> {
    pub span: Span,
    pub name: Atom<'a>,
    pub query: MediaList<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct LayerStatementRule<'a> {
    pub span: Span,
    pub names: std::vec::Vec<std::vec::Vec<Atom<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct LayerBlockRule<'a> {
    pub span: Span,
    pub name: Option<std::vec::Vec<Atom<'a>>>,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScopeRule<'a> {
    pub span: Span,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
    pub scope_end: Option<std::boxed::Box<SelectorList<'a>>>,
    pub scope_start: Option<std::boxed::Box<SelectorList<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct StartingStyleRule<'a> {
    pub span: Span,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
    #[visit(skip)]
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> StartingStyleRule<'a> {
    pub fn new(span: Span, rules: RuleListId) -> Self {
        Self {
            span,
            rules,
            marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct PositionTryRule<'a> {
    pub span: Span,
    pub name: Atom<'a>,
    #[visit(with = visit_declaration_block_id, with_mut = visit_declaration_block_id_mut)]
    pub declarations: DeclarationBlockId,
}

#[derive(Debug, PartialEq, Visit)]
pub struct UnknownAtRule<'a> {
    pub block: Option<std::vec::Vec<TokenOrValue<'a>>>,
    pub span: Span,
    pub name: Atom<'a>,
    pub prelude: std::vec::Vec<TokenOrValue<'a>>,
}
