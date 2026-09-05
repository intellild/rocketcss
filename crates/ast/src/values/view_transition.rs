use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionName<'a> {
    None,
    Auto,
    Custom(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for ViewTransitionName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0016_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Auto,
            2 => Self::Custom(context.resolve_string(read_u32(&bytes, 4) as u64)),
            _ => panic!("invalid encoded ViewTransitionName variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_string_enum(self, context, |value| match value {
            Self::None => (0, None),
            Self::Auto => (1, None),
            Self::Custom(value) => (2, Some(value)),
        })
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ViewTransitionName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum NoneOrCustomIdentList<'a> {
    None,
    Idents(Vec<'a, &'a str>),
}

impl<'ast> AstNodeStorage<'ast> for NoneOrCustomIdentList<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0016_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Idents(
                context
                    .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
            ),
            _ => panic!("invalid encoded NoneOrCustomIdentList variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::None => bytes[0] = 0,
            Self::Idents(values) => {
                bytes[0] = 1;
                write_u32(
                    &mut bytes,
                    4,
                    u32::try_from(values.start_index()).expect("AST range exceeds four bytes"),
                );
                write_u32(
                    &mut bytes,
                    8,
                    u32::try_from(values.end_index()).expect("AST range exceeds four bytes"),
                );
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for NoneOrCustomIdentList<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Idents(values) => Self::Idents(context.clone_encoded_vec(values)),
        }
    }
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

#[derive(Debug, PartialEq, Visit)]
pub enum ViewTransitionGroup<'a> {
    Normal,
    Contain,
    Nearest,
    Custom(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for ViewTransitionGroup<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0016_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Normal,
            1 => Self::Contain,
            2 => Self::Nearest,
            3 => Self::Custom(context.resolve_string(read_u32(&bytes, 4) as u64)),
            _ => panic!("invalid encoded ViewTransitionGroup variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_string_enum(self, context, |value| match value {
            Self::Normal => (0, None),
            Self::Contain => (1, None),
            Self::Nearest => (2, None),
            Self::Custom(value) => (3, Some(value)),
        })
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ViewTransitionGroup<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

fn encode_string_enum<'ast, T>(
    value: T,
    context: &mut AstContext<'ast>,
    classify: impl FnOnce(T) -> (u8, Option<&'ast str>),
) -> NodePayload {
    let (tag, string) = classify(value);
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    bytes[0] = tag;
    if let Some(string) = string {
        write_u32(&mut bytes, 4, context.store_string(string));
    }
    NodePayload::inline(&bytes)
}
