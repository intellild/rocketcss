use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ViewTransitionProperty<'ast>(pub(crate) NodeId<'ast, ViewTransitionPropertyData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewTransitionPropertyData<'ast> {
    Navigation(Navigation),
    Types(NoneOrCustomIdentList<'ast>),
    Custom(CustomProperty<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ViewTransitionPropertyKind {
    Navigation,
    Types,
    Custom,
}
impl<'ast> NodeStorage<'ast> for ViewTransitionProperty<'ast> {
    const KIND: NodeKind = super::kind::KIND_VIEW_TRANSITION_PROPERTY;
    type Data = ViewTransitionPropertyData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ViewTransitionPropertyData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ViewTransitionPropertyData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ViewTransitionProperty<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ViewTransitionPropertyData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ViewTransitionPropertyKind {
        match ast.data(self) {
            ViewTransitionPropertyData::Navigation { .. } => ViewTransitionPropertyKind::Navigation,
            ViewTransitionPropertyData::Types { .. } => ViewTransitionPropertyKind::Types,
            ViewTransitionPropertyData::Custom { .. } => ViewTransitionPropertyKind::Custom,
        }
    }
}
impl<'ast> GetSpan<'ast> for ViewTransitionProperty<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ViewTransitionProperty<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_view_transition_property(
        &mut self,
        data: ViewTransitionPropertyData<'ast>,
        span: Span,
    ) -> ViewTransitionProperty<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Navigation {
    None,
    Auto,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ViewTransitionPartSelector<'ast>(
    pub(crate) NodeId<'ast, ViewTransitionPartSelectorData<'ast>>,
);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewTransitionPartSelectorData<'ast> {
    pub classes: &'ast [&'ast str],
    pub name: Option<ViewTransitionPartName<'ast>>,
}
impl<'ast> NodeStorage<'ast> for ViewTransitionPartSelector<'ast> {
    const KIND: NodeKind = super::kind::KIND_VIEW_TRANSITION_PART_SELECTOR;
    type Data = ViewTransitionPartSelectorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ViewTransitionPartSelectorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ViewTransitionPartSelectorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ViewTransitionPartSelector<'ast> {
    #[inline]
    pub fn classes(self, ast: &Ast<'ast>) -> &'ast [&'ast str] {
        ast.data(self).classes
    }
    #[inline]
    pub fn set_classes(self, ast: &mut Ast<'ast>, value: &'ast [&'ast str]) {
        let mut data = ast.data(self);
        data.classes = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> Option<ViewTransitionPartName<'ast>> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: Option<ViewTransitionPartName<'ast>>) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for ViewTransitionPartSelector<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ViewTransitionPartSelector<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_view_transition_part_selector(
        &mut self,
        data: ViewTransitionPartSelectorData<'ast>,
        span: Span,
    ) -> ViewTransitionPartSelector<'ast> {
        self.alloc(data, span)
    }
}
