use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ContainerCondition<'ast>(pub(crate) NodeId<'ast, ContainerConditionData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContainerConditionData<'ast> {
    Feature(ContainerSizeFeature<'ast>),
    Not(ContainerCondition<'ast>),
    Operation {
        conditions: &'ast [ContainerCondition<'ast>],
        operator: Operator,
    },
    Style(StyleQuery<'ast>),
    ScrollState(ScrollStateQuery<'ast>),
    Unknown(&'ast [TokenOrValue<'ast>]),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerConditionKind {
    Feature,
    Not,
    Operation,
    Style,
    ScrollState,
    Unknown,
}
impl<'ast> NodeStorage<'ast> for ContainerCondition<'ast> {
    const KIND: NodeKind = super::kind::KIND_CONTAINER_CONDITION;
    type Data = ContainerConditionData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ContainerConditionData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ContainerConditionData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ContainerCondition<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ContainerConditionData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ContainerConditionKind {
        match ast.data(self) {
            ContainerConditionData::Feature { .. } => ContainerConditionKind::Feature,
            ContainerConditionData::Not { .. } => ContainerConditionKind::Not,
            ContainerConditionData::Operation { .. } => ContainerConditionKind::Operation,
            ContainerConditionData::Style { .. } => ContainerConditionKind::Style,
            ContainerConditionData::ScrollState { .. } => ContainerConditionKind::ScrollState,
            ContainerConditionData::Unknown { .. } => ContainerConditionKind::Unknown,
        }
    }
}
impl<'ast> GetSpan<'ast> for ContainerCondition<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ContainerCondition<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_container_condition(
        &mut self,
        data: ContainerConditionData<'ast>,
        span: Span,
    ) -> ContainerCondition<'ast> {
        self.alloc(data, span)
    }
}
pub type ContainerSizeFeature<'ast> = QueryFeature<'ast, ContainerSizeFeatureId>;
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerSizeFeatureId {
    Width,
    Height,
    InlineSize,
    BlockSize,
    AspectRatio,
    Orientation,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StyleQuery<'ast>(pub(crate) NodeId<'ast, StyleQueryData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StyleQueryData<'ast> {
    Declaration(Declaration<'ast>),
    Property(PropertyId<'ast>),
    Not(StyleQuery<'ast>),
    Operation {
        conditions: &'ast [StyleQuery<'ast>],
        operator: Operator,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StyleQueryKind {
    Declaration,
    Property,
    Not,
    Operation,
}
impl<'ast> NodeStorage<'ast> for StyleQuery<'ast> {
    const KIND: NodeKind = super::kind::KIND_STYLE_QUERY;
    type Data = StyleQueryData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, StyleQueryData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, StyleQueryData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> StyleQuery<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> StyleQueryData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> StyleQueryKind {
        match ast.data(self) {
            StyleQueryData::Declaration { .. } => StyleQueryKind::Declaration,
            StyleQueryData::Property { .. } => StyleQueryKind::Property,
            StyleQueryData::Not { .. } => StyleQueryKind::Not,
            StyleQueryData::Operation { .. } => StyleQueryKind::Operation,
        }
    }
}
impl<'ast> GetSpan<'ast> for StyleQuery<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for StyleQuery<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_style_query(
        &mut self,
        data: StyleQueryData<'ast>,
        span: Span,
    ) -> StyleQuery<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ScrollStateQuery<'ast>(pub(crate) NodeId<'ast, ScrollStateQueryData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollStateQueryData<'ast> {
    Feature(ScrollStateFeature<'ast>),
    Not(ScrollStateQuery<'ast>),
    Operation {
        conditions: &'ast [ScrollStateQuery<'ast>],
        operator: Operator,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScrollStateQueryKind {
    Feature,
    Not,
    Operation,
}
impl<'ast> NodeStorage<'ast> for ScrollStateQuery<'ast> {
    const KIND: NodeKind = super::kind::KIND_SCROLL_STATE_QUERY;
    type Data = ScrollStateQueryData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ScrollStateQueryData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ScrollStateQueryData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> ScrollStateQuery<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> ScrollStateQueryData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> ScrollStateQueryKind {
        match ast.data(self) {
            ScrollStateQueryData::Feature { .. } => ScrollStateQueryKind::Feature,
            ScrollStateQueryData::Not { .. } => ScrollStateQueryKind::Not,
            ScrollStateQueryData::Operation { .. } => ScrollStateQueryKind::Operation,
        }
    }
}
impl<'ast> GetSpan<'ast> for ScrollStateQuery<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for ScrollStateQuery<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_scroll_state_query(
        &mut self,
        data: ScrollStateQueryData<'ast>,
        span: Span,
    ) -> ScrollStateQuery<'ast> {
        self.alloc(data, span)
    }
}
pub type ScrollStateFeature<'ast> = QueryFeature<'ast, ScrollStateFeatureId>;
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ScrollStateFeatureId {
    Stuck,
    Snapped,
    Scrollable,
    Scrolled,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Container<'ast>(pub(crate) NodeId<'ast, ContainerData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContainerData<'ast> {
    pub container_type: ContainerType,
    pub name: ContainerNameList<'ast>,
}
impl<'ast> NodeStorage<'ast> for Container<'ast> {
    const KIND: NodeKind = super::kind::KIND_CONTAINER;
    type Data = ContainerData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, ContainerData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, ContainerData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Container<'ast> {
    #[inline]
    pub fn container_type(self, ast: &Ast<'ast>) -> ContainerType {
        ast.data(self).container_type
    }
    #[inline]
    pub fn set_container_type(self, ast: &mut Ast<'ast>, value: ContainerType) {
        let mut data = ast.data(self);
        data.container_type = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> ContainerNameList<'ast> {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: ContainerNameList<'ast>) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Container<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Container<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_container(&mut self, data: ContainerData<'ast>, span: Span) -> Container<'ast> {
        self.alloc(data, span)
    }
}
