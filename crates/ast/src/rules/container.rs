use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ContainerCondition<'a> {
    Feature(Box<'a, ContainerSizeFeature<'a>>),
    Not(Box<'a, ContainerCondition<'a>>),
    Operation {
        conditions: Vec<'a, ContainerCondition<'a>>,
        operator: Operator,
    },
    Style(Box<'a, StyleQuery<'a>>),
    ScrollState(Box<'a, ScrollStateQuery<'a>>),
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
    Declaration(Box<'a, Declaration<'a>>),
    Property(Box<'a, PropertyId<'a>>),
    Not(Box<'a, StyleQuery<'a>>),
    Operation {
        conditions: Vec<'a, StyleQuery<'a>>,
        operator: Operator,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum ScrollStateQuery<'a> {
    Feature(Box<'a, ScrollStateFeature<'a>>),
    Not(Box<'a, ScrollStateQuery<'a>>),
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
    pub name: Box<'a, ContainerNameList<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ContainerRule<'a> {
    pub condition: Option<Box<'a, ContainerCondition<'a>>>,
    pub span: Span,
    pub name: Option<&'a str>,
    pub rules: Vec<'a, CssRule<'a>>,
}
