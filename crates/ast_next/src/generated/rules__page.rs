use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum PageMarginBox {
    TopLeftCorner,
    TopLeft,
    TopCenter,
    TopRight,
    TopRightCorner,
    LeftTop,
    LeftMiddle,
    LeftBottom,
    RightTop,
    RightMiddle,
    RightBottom,
    BottomLeftCorner,
    BottomLeft,
    BottomCenter,
    BottomRight,
    BottomRightCorner,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum PagePseudoClass {
    Left,
    Right,
    First,
    Last,
    Blank,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PageSelector<'ast>(pub(crate) NodeId<'ast, PageSelectorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageSelectorData<'ast> {
    pub name: Option<&'ast str>,
    pub pseudo_classes: &'ast [PagePseudoClass],
}
impl<'ast> NodeStorage<'ast> for PageSelector<'ast> {
    const KIND: NodeKind = super::kind::KIND_PAGE_SELECTOR;
    type Data = PageSelectorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PageSelectorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PageSelectorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PageSelector<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> Option<&'ast str> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: Option<&'ast str>) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn pseudo_classes(self, ast: &Ast<'ast>) -> &'ast [PagePseudoClass] {
        ast.data(self).pseudo_classes
    }
    #[inline]
    pub fn set_pseudo_classes(self, ast: &mut Ast<'ast>, value: &'ast [PagePseudoClass]) {
        let mut data = ast.data(self);
        data.pseudo_classes = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for PageSelector<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PageSelector<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_page_selector(
        &mut self,
        data: PageSelectorData<'ast>,
        span: Span,
    ) -> PageSelector<'ast> {
        self.alloc(data, span)
    }
}
