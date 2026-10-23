use crate::*;
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerType {
    Normal,
    InlineSize,
    Size,
    ScrollState,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ContainerNameList<'ast>(pub(crate) NodeId<'ast, ContainerNameListData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContainerNameListData<'ast> {
    None,
    Names(&'ast [&'ast str]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerNameListKind {
    None,
    Names,
}
impl<'ast> NodeStorage<'ast> for ContainerNameList<'ast> {
    const KIND: NodeKind = super::kind::KIND_CONTAINER_NAME_LIST;
    type Data = ContainerNameListData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ContainerNameListData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ContainerNameListData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ContainerNameList<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ContainerNameListData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ContainerNameListKind {
        match ast.data(self) {
            ContainerNameListData::None => ContainerNameListKind::None,
            ContainerNameListData::Names { .. } => ContainerNameListKind::Names,
        }
    }
}
impl<'ast> GetSpan<'ast> for ContainerNameList<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ContainerNameList<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_container_name_list(
        &mut self,
        data: ContainerNameListData<'ast>,
        span: Span,
    ) -> ContainerNameList<'ast> {
        self.alloc(data, span)
    }
}
