use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub enum CSSWideKeyword {
    Initial,
    Inherit,
    Unset,
    Revert,
    RevertLayer,
}

/// A typed property value or a CSS-wide keyword.
#[derive(Debug, PartialEq, Visit)]
pub enum CSSWideOr<T> {
    Value(T),
    CSSWide(CSSWideKeyword),
}

#[derive(Debug, PartialEq, Visit)]
pub enum CustomPropertyName<'a> {
    Custom(&'a str),
    Unknown(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for CustomPropertyName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0014_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let value =
            context.resolve_string(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as u64);
        match bytes[0] {
            0 => Self::Custom(value),
            1 => Self::Unknown(value),
            _ => panic!("invalid encoded CustomPropertyName variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        let (kind, value) = match self {
            Self::Custom(value) => (0, value),
            Self::Unknown(value) => (1, value),
        };
        bytes[0] = kind;
        bytes[4..8].copy_from_slice(&context.store_string(value).to_le_bytes());
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for CustomPropertyName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}
