use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Url<'ast>(pub(crate) NodeId<'ast, Url<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UrlData<'ast> {
    pub url: &'ast str,
}
impl<'ast> NodeStorage<'ast> for Url<'ast> {
    const KIND: NodeKind = super::kind::KIND_URL;
    type Data = UrlData<'ast>;
}
impl<'ast> Url<'ast> {
    #[inline]
    pub fn url(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self.0).url
    }
    #[inline]
    pub fn set_url(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self.0);
        data.url = value;
        ast.set_data(self.0, data);
    }
}
impl<'ast> GetSpan<'ast> for Url<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self.0)
    }
}
impl<'ast> SetSpan<'ast> for Url<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self.0, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_url(&mut self, data: UrlData<'ast>, span: Span) -> Url<'ast> {
        Url(self.alloc(data, span))
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Token<'ast>(pub(crate) NodeId<'ast, Token<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenData<'ast> {
    Ident(Atom<'ast>),
    Number(f32),
    WhiteSpace(&'ast str),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Ident,
    Number,
    WhiteSpace,
}
impl<'ast> NodeStorage<'ast> for Token<'ast> {
    const KIND: NodeKind = super::kind::KIND_TOKEN;
    type Data = TokenData<'ast>;
}
impl<'ast> Token<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> TokenData<'ast> {
        ast.data(self.0)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> TokenKind {
        match ast.data(self.0) {
            TokenData::Ident { .. } => TokenKind::Ident,
            TokenData::Number { .. } => TokenKind::Number,
            TokenData::WhiteSpace { .. } => TokenKind::WhiteSpace,
        }
    }
}
impl<'ast> GetSpan<'ast> for Token<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self.0)
    }
}
impl<'ast> SetSpan<'ast> for Token<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self.0, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_token(&mut self, data: TokenData<'ast>, span: Span) -> Token<'ast> {
        Token(self.alloc(data, span))
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Function<'ast>(pub(crate) NodeId<'ast, Function<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionData<'ast> {
    pub name: Atom<'ast>,
    pub arguments: &'ast [Token<'ast>],
    pub replacement: Option<Url<'ast>>,
}
impl<'ast> NodeStorage<'ast> for Function<'ast> {
    const KIND: NodeKind = super::kind::KIND_FUNCTION;
    type Data = FunctionData<'ast>;
}
impl<'ast> Function<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> Atom<'ast> {
        ast.data(self.0).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: Atom<'ast>) {
        let mut data = ast.data(self.0);
        data.name = value;
        ast.set_data(self.0, data);
    }
    #[inline]
    pub fn arguments(self, ast: &Ast<'ast>) -> &'ast [Token<'ast>] {
        ast.data(self.0).arguments
    }
    #[inline]
    pub fn set_arguments(self, ast: &mut Ast<'ast>, value: &'ast [Token<'ast>]) {
        let mut data = ast.data(self.0);
        data.arguments = value;
        ast.set_data(self.0, data);
    }
    #[inline]
    pub fn replacement(self, ast: &Ast<'ast>) -> Option<Url<'ast>> {
        ast.data(self.0).replacement
    }
    #[inline]
    pub fn set_replacement(self, ast: &mut Ast<'ast>, value: Option<Url<'ast>>) {
        let mut data = ast.data(self.0);
        data.replacement = value;
        ast.set_data(self.0, data);
    }
}
impl<'ast> GetSpan<'ast> for Function<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self.0)
    }
}
impl<'ast> SetSpan<'ast> for Function<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self.0, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_function(&mut self, data: FunctionData<'ast>, span: Span) -> Function<'ast> {
        Function(self.alloc(data, span))
    }
}
pub type TokenList<'ast> = &'ast [Token<'ast>];
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KnownFunction {
    Calc,
    Var,
    Unknown,
}
