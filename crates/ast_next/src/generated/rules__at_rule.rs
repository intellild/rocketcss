use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CharsetRule<'ast>(pub(crate) NodeId<'ast, CharsetRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharsetRuleData<'ast> {
    pub encoding: &'ast str,
}
impl<'ast> NodeStorage<'ast> for CharsetRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_CHARSET_RULE;
    type Data = CharsetRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CharsetRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CharsetRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CharsetRule<'ast> {
    #[inline]
    pub fn encoding(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self).encoding
    }
    #[inline]
    pub fn set_encoding(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self);
        data.encoding = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for CharsetRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CharsetRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_charset_rule(
        &mut self,
        data: CharsetRuleData<'ast>,
        span: Span,
    ) -> CharsetRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NamespaceRule<'ast>(pub(crate) NodeId<'ast, NamespaceRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NamespaceRuleData<'ast> {
    pub prefix: Option<&'ast str>,
    pub url: &'ast str,
}
impl<'ast> NodeStorage<'ast> for NamespaceRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_NAMESPACE_RULE;
    type Data = NamespaceRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, NamespaceRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, NamespaceRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> NamespaceRule<'ast> {
    #[inline]
    pub fn prefix(self, ast: &Ast<'ast>) -> Option<&'ast str> {
        ast.data(self).prefix
    }
    #[inline]
    pub fn set_prefix(self, ast: &mut Ast<'ast>, value: Option<&'ast str>) {
        let mut data = ast.data(self);
        data.prefix = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn url(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self).url
    }
    #[inline]
    pub fn set_url(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self);
        data.url = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for NamespaceRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for NamespaceRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_namespace_rule(
        &mut self,
        data: NamespaceRuleData<'ast>,
        span: Span,
    ) -> NamespaceRule<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CustomMediaRule<'ast>(pub(crate) NodeId<'ast, CustomMediaRuleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CustomMediaRuleData<'ast> {
    pub name: &'ast str,
    pub query: MediaList<'ast>,
}
impl<'ast> NodeStorage<'ast> for CustomMediaRule<'ast> {
    const KIND: NodeKind = super::kind::KIND_CUSTOM_MEDIA_RULE;
    type Data = CustomMediaRuleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CustomMediaRuleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CustomMediaRuleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CustomMediaRule<'ast> {
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
impl<'ast> GetSpan<'ast> for CustomMediaRule<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CustomMediaRule<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_custom_media_rule(
        &mut self,
        data: CustomMediaRuleData<'ast>,
        span: Span,
    ) -> CustomMediaRule<'ast> {
        self.alloc(data, span)
    }
}
