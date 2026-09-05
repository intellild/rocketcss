use crate::*;
use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};
#[derive(Debug, PartialEq, Visit)]
pub enum KeyframeSelector {
    Percentage(f32),
    From,
    To,
    TimelineRangePercentage(TimelineRangePercentage),
}

#[derive(Debug, PartialEq, Visit)]
pub enum KeyframesName<'a> {
    Ident(&'a str),
    Custom(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for KeyframesName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0015_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let value =
            context.resolve_string(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as u64);
        match bytes[0] {
            0 => Self::Ident(value),
            1 => Self::Custom(value),
            _ => panic!("invalid encoded KeyframesName variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        let (kind, value) = match self {
            Self::Ident(value) => (0, value),
            Self::Custom(value) => (1, value),
        };
        bytes[0] = kind;
        bytes[4..8].copy_from_slice(&context.store_string(value).to_le_bytes());
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for KeyframesName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct TimelineRangePercentage {
    pub name: TimelineRangeName,
    pub percentage: f32,
}
