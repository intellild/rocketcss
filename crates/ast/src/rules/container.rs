use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ContainerCondition<'a> {
    Feature(NodeId<'a, ContainerSizeFeature<'a>>),
    Not(NodeId<'a, ContainerCondition<'a>>),
    Operation {
        conditions: Vec<'a, NodeId<'a, ContainerCondition<'a>>>,
        operator: Operator,
    },
    Style(NodeId<'a, StyleQuery<'a>>),
    ScrollState(NodeId<'a, ScrollStateQuery<'a>>),
    Unknown(Vec<'a, TokenOrValue<'a>>),
}

pub type ContainerSizeFeature<'a> = QueryFeature<'a, ContainerSizeFeatureId>;

#[repr(u8)]
#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum ContainerSizeFeatureId {
    Width,
    Height,
    InlineSize,
    BlockSize,
    AspectRatio,
    Orientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum StyleQuery<'a> {
    Declaration(#[visit(skip)] DeclarationId<'a>),
    Property(NodeId<'a, PropertyId<'a>>),
    Not(NodeId<'a, StyleQuery<'a>>),
    Operation {
        conditions: Vec<'a, NodeId<'a, StyleQuery<'a>>>,
        operator: Operator,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ScrollStateQuery<'a> {
    Feature(NodeId<'a, ScrollStateFeature<'a>>),
    Not(NodeId<'a, ScrollStateQuery<'a>>),
    Operation {
        conditions: Vec<'a, NodeId<'a, ScrollStateQuery<'a>>>,
        operator: Operator,
    },
}

pub type ScrollStateFeature<'a> = QueryFeature<'a, ScrollStateFeatureId>;

#[repr(u8)]
#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum ScrollStateFeatureId {
    Stuck,
    Snapped,
    Scrollable,
    Scrolled,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Container<'a> {
    pub container_type: ContainerType,
    pub name: NodeId<'a, ContainerNameList<'a>>,
}

impl_inline_node!(Container<'ast>, 0x001a0008);

impl<'ast> AstNodeClone<'ast> for Container<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            container_type: self.container_type,
            name: context.clone_encoded_node(self.name),
        }
    }
}

impl QueryFeatureId for ContainerSizeFeatureId {
    const KIND: NodeKind = NodeKind::new(0x001a_0006);
}

impl QueryFeatureId for ScrollStateFeatureId {
    const KIND: NodeKind = NodeKind::new(0x001a_0007);
}

impl_inline_node!(ContainerCondition<'ast>, 0x001a0005);

impl<'ast> AstNodeClone<'ast> for ContainerCondition<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Feature(value) => Self::Feature(context.clone_encoded_node(value)),
            Self::Not(value) => Self::Not(context.clone_encoded_node(value)),
            Self::Operation {
                conditions,
                operator,
            } => Self::Operation {
                conditions: context.clone_encoded_vec(conditions),
                operator,
            },
            Self::Style(value) => Self::Style(context.clone_encoded_node(value)),
            Self::ScrollState(value) => Self::ScrollState(context.clone_encoded_node(value)),
            Self::Unknown(values) => Self::Unknown(context.clone_encoded_vec(values)),
        }
    }
}

impl_inline_node!(StyleQuery<'ast>, 0x001a000c);

impl<'ast> AstNodeClone<'ast> for StyleQuery<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Declaration(value) => Self::Declaration(value),
            Self::Property(value) => Self::Property(context.clone_encoded_node(value)),
            Self::Not(value) => Self::Not(context.clone_encoded_node(value)),
            Self::Operation {
                conditions,
                operator,
            } => Self::Operation {
                conditions: context.clone_encoded_vec(conditions),
                operator,
            },
        }
    }
}

impl_inline_node!(ScrollStateQuery<'ast>, 0x001a0009);

impl<'ast> AstNodeClone<'ast> for ScrollStateQuery<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Feature(value) => Self::Feature(context.clone_encoded_node(value)),
            Self::Not(value) => Self::Not(context.clone_encoded_node(value)),
            Self::Operation {
                conditions,
                operator,
            } => Self::Operation {
                conditions: context.clone_encoded_vec(conditions),
                operator,
            },
        }
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use super::*;

    #[test]
    fn native_container_queries_keep_typed_ranges_and_reuse_nodes() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let property = ast.alloc_node(PropertyId::Width, DUMMY_SP);
        let style_child = ast.alloc_node(StyleQuery::Property(property), DUMMY_SP);
        let styles = ast.alloc_encoded_vec([style_child].into_iter());
        let style = ast.alloc_node(StyleQuery::Not(style_child), DUMMY_SP);
        let feature = ast.alloc_node(
            QueryFeature::Boolean {
                name: MediaFeatureName::Standard(ScrollStateFeatureId::Scrolled),
            },
            DUMMY_SP,
        );
        let scroll_child = ast.alloc_node(ScrollStateQuery::Feature(feature), DUMMY_SP);
        let scrolls = ast.alloc_encoded_vec([scroll_child].into_iter());
        let scroll = ast.alloc_node(ScrollStateQuery::Not(scroll_child), DUMMY_SP);
        let child = ast.alloc_node(ContainerCondition::Style(style), DUMMY_SP);
        let conditions = ast.alloc_encoded_vec([child].into_iter());
        let unknown = ast.alloc_encoded_vec(std::iter::empty());
        let root = ast.alloc_node(ContainerCondition::Not(child), DUMMY_SP);
        let name = ast.alloc_node(ContainerNameList::None, DUMMY_SP);
        let container = ast.alloc_node(
            Container {
                container_type: ContainerType::Normal,
                name,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for operator in [Operator::And, Operator::Or] {
            let expected = StyleQuery::Operation {
                conditions: styles,
                operator,
            };
            ast.mutate_node(style, |value, _| *value = expected);
            assert_eq!(ast.resolve_node(style), expected);
            let expected = ScrollStateQuery::Operation {
                conditions: scrolls,
                operator,
            };
            ast.mutate_node(scroll, |value, _| *value = expected);
            assert_eq!(ast.resolve_node(scroll), expected);
            for expected in [
                ContainerCondition::Operation {
                    conditions,
                    operator,
                },
                ContainerCondition::ScrollState(scroll),
                ContainerCondition::Unknown(unknown),
                ContainerCondition::Style(style),
                ContainerCondition::Not(child),
            ] {
                ast.mutate_node(root, |value, _| *value = expected);
                assert_eq!(ast.resolve_node(root), expected);
            }
        }
        for container_type in [
            ContainerType::Normal,
            ContainerType::InlineSize,
            ContainerType::Size,
            ContainerType::ScrollState,
        ] {
            ast.mutate_node(container, |value, _| value.container_type = container_type);
            assert_eq!(
                ast.resolve_node(container),
                Container {
                    container_type,
                    name
                }
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        let cloned = ast.clone_node(style);
        let StyleQuery::Operation {
            conditions: cloned_styles,
            ..
        } = ast.resolve_node(cloned)
        else {
            panic!("expected style operation")
        };
        let cloned_child = ast.encoded_vec_get(cloned_styles, 0).unwrap();
        let StyleQuery::Property(cloned_property) = ast.resolve_node(cloned_child) else {
            panic!("expected style property")
        };
        assert_ne!(cloned_property, property);
        ast.mutate_node(cloned_property, |value, _| *value = PropertyId::Height);
        assert_eq!(ast.resolve_node(property), PropertyId::Width);
    }

    #[test]
    fn container_condition_codec_deep_clones_query_tree() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let value = context.alloc_encoded_node(MediaFeatureValue::Number(48.0), DUMMY_SP);
        let feature = context.alloc_encoded_node(
            QueryFeature::Plain {
                name: MediaFeatureName::Standard(ContainerSizeFeatureId::Width),
                value,
            },
            DUMMY_SP,
        );
        let child = context.alloc_encoded_node(ContainerCondition::Feature(feature), DUMMY_SP);
        let conditions = context.alloc_encoded_vec([child].into_iter());
        let condition = context.alloc_encoded_node(
            ContainerCondition::Operation {
                conditions,
                operator: Operator::Or,
            },
            DUMMY_SP,
        );

        let cloned = context.clone_encoded_node(condition);
        let ContainerCondition::Operation {
            conditions: cloned_conditions,
            operator: Operator::Or,
        } = context.encoded_node(cloned)
        else {
            panic!("expected cloned container operation")
        };
        assert_ne!(cloned_conditions, conditions);
        assert_ne!(context.encoded_vec_get(cloned_conditions, 0), Some(child));
    }
}
