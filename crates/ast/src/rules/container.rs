use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ContainerCondition<'a> {
    Feature(NodeId<'a, ContainerSizeFeature<'a>>),
    Not(NodeId<'a, ContainerCondition<'a>>),
    Operation {
        conditions: Vec<'a, ContainerCondition<'a>>,
        operator: Operator,
    },
    Style(NodeId<'a, StyleQuery<'a>>),
    ScrollState(NodeId<'a, ScrollStateQuery<'a>>),
    Unknown(Vec<'a, TokenOrValue<'a>>),
}

pub type ContainerSizeFeature<'a> = QueryFeature<'a, ContainerSizeFeatureId>;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ContainerSizeFeatureId {
    Width,
    Height,
    InlineSize,
    BlockSize,
    AspectRatio,
    Orientation,
}

#[derive(Debug, PartialEq, Visit)]
pub enum StyleQuery<'a> {
    Declaration(NodeId<'a, Declaration<'a>>),
    Property(NodeId<'a, PropertyId<'a>>),
    Not(NodeId<'a, StyleQuery<'a>>),
    Operation {
        conditions: Vec<'a, StyleQuery<'a>>,
        operator: Operator,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum ScrollStateQuery<'a> {
    Feature(NodeId<'a, ScrollStateFeature<'a>>),
    Not(NodeId<'a, ScrollStateQuery<'a>>),
    Operation {
        conditions: Vec<'a, ScrollStateQuery<'a>>,
        operator: Operator,
    },
}

pub type ScrollStateFeature<'a> = QueryFeature<'a, ScrollStateFeatureId>;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ScrollStateFeatureId {
    Stuck,
    Snapped,
    Scrollable,
    Scrolled,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Container<'a> {
    pub container_type: ContainerType,
    pub name: NodeId<'a, ContainerNameList<'a>>,
}
