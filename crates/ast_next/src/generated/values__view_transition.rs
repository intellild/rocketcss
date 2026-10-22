use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ViewTransitionName<'ast> {
    None,
    Auto,
    Custom(&'ast str),
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NoneOrCustomIdentList<'ast>(pub(crate) NodeId<'ast, NoneOrCustomIdentListData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoneOrCustomIdentListData<'ast> {
    None,
    Idents(&'ast [&'ast str]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoneOrCustomIdentListKind {
    None,
    Idents,
}
impl<'ast> NodeStorage<'ast> for NoneOrCustomIdentList<'ast> {
    const KIND: NodeKind = super::kind::KIND_NONE_OR_CUSTOM_IDENT_LIST;
    type Data = NoneOrCustomIdentListData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, NoneOrCustomIdentListData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, NoneOrCustomIdentListData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> NoneOrCustomIdentList<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> NoneOrCustomIdentListData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> NoneOrCustomIdentListKind {
        match ast.data(self) {
            NoneOrCustomIdentListData::None { .. } => NoneOrCustomIdentListKind::None,
            NoneOrCustomIdentListData::Idents { .. } => NoneOrCustomIdentListKind::Idents,
        }
    }
}
impl<'ast> GetSpan<'ast> for NoneOrCustomIdentList<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for NoneOrCustomIdentList<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_none_or_custom_ident_list(
        &mut self,
        data: NoneOrCustomIdentListData<'ast>,
        span: Span,
    ) -> NoneOrCustomIdentList<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ViewTransitionGroup<'ast> {
    Normal,
    Contain,
    Nearest,
    Custom(&'ast str),
}
