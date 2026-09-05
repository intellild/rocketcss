use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ContainerType {
    Normal,
    InlineSize,
    Size,
    ScrollState,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ContainerNameList<'a> {
    None,
    Names(Vec<'a, &'a str>),
}

impl<'ast> AstNodeStorage<'ast> for ContainerNameList<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001e_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Names(
                context
                    .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
            ),
            _ => panic!("invalid encoded ContainerNameList variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::None => bytes[0] = 0,
            Self::Names(values) => {
                bytes[0] = 1;
                write_u32(&mut bytes, 4, values.start_index());
                write_u32(&mut bytes, 8, values.end_index());
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ContainerNameList<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Names(values) => Self::Names(context.clone_encoded_vec(values)),
        }
    }
}

fn write_u32(bytes: &mut [u8], offset: usize, value: usize) {
    bytes[offset..offset + 4].copy_from_slice(
        &u32::try_from(value)
            .expect("AST compact index exceeds four bytes")
            .to_le_bytes(),
    );
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}
