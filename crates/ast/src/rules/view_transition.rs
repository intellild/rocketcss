use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionProperty<'a> {
    Navigation(Navigation),
    Types(NodeId<'a, NoneOrCustomIdentList<'a>>),
    Custom(NodeId<'a, CustomProperty<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Navigation {
    None,
    Auto,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Visit)]
pub struct ViewTransitionPartSelector<'a> {
    pub classes: Vec<'a, &'a str>,
    pub name: Option<NodeId<'a, ViewTransitionPartName<'a>>>,
}

impl<'ast> AstNodeStorage<'ast> for ViewTransitionPartSelector<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            classes: context
                .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
            name: match bytes[0] {
                0 => None,
                1 => Some(context.encoded_node_id_at(read_u32(&bytes, 12) as usize)),
                _ => panic!("invalid encoded ViewTransitionPartSelector name flag"),
            },
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_view_transition_part_selector(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_view_transition_part_selector(self)
    }
}

impl<'ast> AstNodeClone<'ast> for ViewTransitionPartSelector<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            classes: context.clone_encoded_vec(self.classes),
            name: self.name.map(|name| context.clone_encoded_node(name)),
        }
    }
}

fn encode_view_transition_part_selector(value: ViewTransitionPartSelector<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_u32(
        &mut bytes,
        4,
        u32::try_from(value.classes.start_index()).expect("AST range start exceeds four bytes"),
    );
    write_u32(
        &mut bytes,
        8,
        u32::try_from(value.classes.end_index()).expect("AST range end exceeds four bytes"),
    );
    if let Some(name) = value.name {
        bytes[0] = 1;
        write_u32(
            &mut bytes,
            12,
            u32::try_from(name.index()).expect("AST node ID exceeds four bytes"),
        );
    }
    NodePayload::inline(&bytes)
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}
