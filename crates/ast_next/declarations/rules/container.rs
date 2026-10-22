#[ast]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerSizeFeatureId {
    Width,
    Height,
    InlineSize,
    BlockSize,
    AspectRatio,
    Orientation,
}

#[ast]
pub enum StyleQuery<'a> {
    Declaration(Box<'a, Declaration<'a>>),
    Property(Box<'a, PropertyId<'a>>),
    Not(Box<'a, StyleQuery<'a>>),
    Operation {
        conditions: Vec<'a, StyleQuery<'a>>,
        operator: Operator,
    },
}

#[ast]
pub enum ScrollStateQuery<'a> {
    Feature(Box<'a, ScrollStateFeature<'a>>),
    Not(Box<'a, ScrollStateQuery<'a>>),
    Operation {
        conditions: Vec<'a, ScrollStateQuery<'a>>,
        operator: Operator,
    },
}

pub type ScrollStateFeature<'a> = QueryFeature<'a, ScrollStateFeatureId>;

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ScrollStateFeatureId {
    Stuck,
    Snapped,
    Scrollable,
    Scrolled,
}

#[ast]
pub struct Container<'a> {
    pub container_type: ContainerType,
    pub name: Box<'a, ContainerNameList<'a>>,
}
