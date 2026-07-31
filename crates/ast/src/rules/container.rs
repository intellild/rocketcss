use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ContainerCondition<'a> {
    Feature(std::boxed::Box<ContainerSizeFeature<'a>>),
    Not(std::boxed::Box<ContainerCondition<'a>>),
    Operation {
        conditions: std::vec::Vec<ContainerCondition<'a>>,
        operator: Operator,
    },
    Style(std::boxed::Box<StyleQuery<'a>>),
    ScrollState(std::boxed::Box<ScrollStateQuery<'a>>),
    Unknown(std::vec::Vec<TokenOrValue<'a>>),
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
    Declaration(std::boxed::Box<Declaration<'a>>),
    Property(std::boxed::Box<PropertyId<'a>>),
    Not(std::boxed::Box<StyleQuery<'a>>),
    Operation {
        conditions: std::vec::Vec<StyleQuery<'a>>,
        operator: Operator,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum ScrollStateQuery<'a> {
    Feature(std::boxed::Box<ScrollStateFeature<'a>>),
    Not(std::boxed::Box<ScrollStateQuery<'a>>),
    Operation {
        conditions: std::vec::Vec<ScrollStateQuery<'a>>,
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
    pub name: std::boxed::Box<ContainerNameList<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ContainerRule<'a> {
    pub condition: Option<std::boxed::Box<ContainerCondition<'a>>>,
    pub span: Span,
    pub name: Option<Atom<'a>>,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
}
