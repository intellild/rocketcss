use crate::*;

#[derive(Debug, PartialEq, Visit)]
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
    Declaration(#[visit(skip)] DeclarationId<'a>),
    Property(NodeId<'a, PropertyId<'a>>),
    Not(NodeId<'a, StyleQuery<'a>>),
    Operation {
        conditions: Vec<'a, NodeId<'a, StyleQuery<'a>>>,
        operator: Operator,
    },
}

#[derive(Debug, PartialEq, Visit)]
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

impl<'ast> AstNodeStorage<'ast> for Container<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_0008);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            container_type: match bytes[0] {
                0 => ContainerType::Normal,
                1 => ContainerType::InlineSize,
                2 => ContainerType::Size,
                3 => ContainerType::ScrollState,
                _ => panic!("invalid encoded ContainerType"),
            },
            name: read_node_id(&bytes, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = match self.container_type {
            ContainerType::Normal => 0,
            ContainerType::InlineSize => 1,
            ContainerType::Size => 2,
            ContainerType::ScrollState => 3,
        };
        write_u32(
            &mut bytes,
            4,
            u32::try_from(self.name.index()).expect("AST node ID exceeds four bytes"),
        );
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Container<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            container_type: self.container_type,
            name: context.clone_encoded_node(self.name),
        }
    }
}

impl QueryFeatureIdCodec for ContainerSizeFeatureId {
    const KIND: NodeKind = NodeKind::new(0x001a_0006);

    fn encode(self) -> u8 {
        self as u8
    }

    fn decode(value: u8) -> Self {
        match value {
            0 => Self::Width,
            1 => Self::Height,
            2 => Self::InlineSize,
            3 => Self::BlockSize,
            4 => Self::AspectRatio,
            5 => Self::Orientation,
            _ => panic!("invalid encoded ContainerSizeFeatureId"),
        }
    }
}

impl QueryFeatureIdCodec for ScrollStateFeatureId {
    const KIND: NodeKind = NodeKind::new(0x001a_0007);

    fn encode(self) -> u8 {
        self as u8
    }

    fn decode(value: u8) -> Self {
        match value {
            0 => Self::Stuck,
            1 => Self::Snapped,
            2 => Self::Scrollable,
            3 => Self::Scrolled,
            _ => panic!("invalid encoded ScrollStateFeatureId"),
        }
    }
}

impl<'ast> AstNodeStorage<'ast> for ContainerCondition<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Feature(read_node_id(&bytes, context)),
            1 => Self::Not(read_node_id(&bytes, context)),
            2 => Self::Operation {
                conditions: read_range(&bytes, context),
                operator: decode_operator(bytes[1]),
            },
            3 => Self::Style(read_node_id(&bytes, context)),
            4 => Self::ScrollState(read_node_id(&bytes, context)),
            5 => Self::Unknown(read_range(&bytes, context)),
            _ => panic!("invalid encoded ContainerCondition variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_container_condition(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_container_condition(self)
    }
}

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

fn encode_container_condition(value: ContainerCondition<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        ContainerCondition::Feature(value) => write_node_id(&mut bytes, 0, value),
        ContainerCondition::Not(value) => write_node_id(&mut bytes, 1, value),
        ContainerCondition::Operation {
            conditions,
            operator,
        } => {
            write_range(&mut bytes, 2, conditions);
            bytes[1] = encode_operator(operator);
        }
        ContainerCondition::Style(value) => write_node_id(&mut bytes, 3, value),
        ContainerCondition::ScrollState(value) => write_node_id(&mut bytes, 4, value),
        ContainerCondition::Unknown(values) => write_range(&mut bytes, 5, values),
    }
    NodePayload::inline(&bytes)
}

impl<'ast> AstNodeStorage<'ast> for StyleQuery<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_000c);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Declaration(context.encoded_declaration_id_at(read_u32(&bytes, 4) as usize)),
            1 => Self::Property(read_node_id(&bytes, context)),
            2 => Self::Not(read_node_id(&bytes, context)),
            3 => Self::Operation {
                conditions: read_range(&bytes, context),
                operator: decode_operator(bytes[1]),
            },
            _ => panic!("invalid encoded StyleQuery variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_style_query(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_style_query(self)
    }
}

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

fn encode_style_query(value: StyleQuery<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        StyleQuery::Declaration(value) => {
            bytes[0] = 0;
            write_u32(
                &mut bytes,
                4,
                u32::try_from(value.index()).expect("declaration ID exceeds four bytes"),
            );
        }
        StyleQuery::Property(value) => write_node_id(&mut bytes, 1, value),
        StyleQuery::Not(value) => write_node_id(&mut bytes, 2, value),
        StyleQuery::Operation {
            conditions,
            operator,
        } => {
            write_range(&mut bytes, 3, conditions);
            bytes[1] = encode_operator(operator);
        }
    }
    NodePayload::inline(&bytes)
}

impl<'ast> AstNodeStorage<'ast> for ScrollStateQuery<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_0009);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Feature(read_node_id(&bytes, context)),
            1 => Self::Not(read_node_id(&bytes, context)),
            2 => Self::Operation {
                conditions: read_range(&bytes, context),
                operator: decode_operator(bytes[1]),
            },
            _ => panic!("invalid encoded ScrollStateQuery variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_scroll_state_query(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_scroll_state_query(self)
    }
}

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

fn encode_scroll_state_query(value: ScrollStateQuery<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        ScrollStateQuery::Feature(value) => write_node_id(&mut bytes, 0, value),
        ScrollStateQuery::Not(value) => write_node_id(&mut bytes, 1, value),
        ScrollStateQuery::Operation {
            conditions,
            operator,
        } => {
            write_range(&mut bytes, 2, conditions);
            bytes[1] = encode_operator(operator);
        }
    }
    NodePayload::inline(&bytes)
}

fn encode_operator(value: Operator) -> u8 {
    match value {
        Operator::And => 0,
        Operator::Or => 1,
    }
}

fn decode_operator(value: u8) -> Operator {
    match value {
        0 => Operator::And,
        1 => Operator::Or,
        _ => panic!("invalid encoded Operator"),
    }
}

fn write_node_id<T>(bytes: &mut [u8], tag: u8, value: NodeId<'_, T>) {
    bytes[0] = tag;
    write_u32(
        bytes,
        4,
        u32::try_from(value.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn read_node_id<'ast, T>(bytes: &[u8], context: &AstContext<'ast>) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, 4) as usize)
}

fn write_range<T>(bytes: &mut [u8], tag: u8, value: Vec<'_, T>) {
    bytes[0] = tag;
    write_u32(
        bytes,
        4,
        u32::try_from(value.start_index()).expect("AST range start exceeds four bytes"),
    );
    write_u32(
        bytes,
        8,
        u32::try_from(value.end_index()).expect("AST range end exceeds four bytes"),
    );
}

fn read_range<'ast, T>(bytes: &[u8], context: &AstContext<'ast>) -> Vec<'ast, T> {
    context.encoded_vec_range(read_u32(bytes, 4) as usize, read_u32(bytes, 8) as usize)
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use super::*;

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
