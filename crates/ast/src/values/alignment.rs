use crate::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub enum AlignContent {
    Normal,
    BaselinePosition(BaselinePosition),
    ContentDistribution(ContentDistribution),
    ContentPosition {
        overflow: Option<OverflowPosition>,
        value: ContentPosition,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BaselinePosition {
    First,
    Last,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ContentDistribution {
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum OverflowPosition {
    Safe,
    Unsafe,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ContentPosition {
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
}

#[derive(Debug, PartialEq, Visit)]
pub enum JustifyContent {
    Normal,
    ContentDistribution(ContentDistribution),
    ContentPosition {
        overflow: Option<OverflowPosition>,
        value: ContentPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum AlignSelf {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum SelfPosition {
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    FlexStart,
    FlexEnd,
}

#[derive(Debug, PartialEq, Visit)]
pub enum JustifySelf {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum AlignItems {
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum JustifyItems {
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
    Legacy(LegacyJustify),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum LegacyJustify {
    Left,
    Right,
    Center,
}

#[derive(Debug, PartialEq, Visit)]
pub enum GapValue<'a> {
    Normal,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for GapValue<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000d_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Normal,
            1 => {
                Self::LengthPercentage(context.encoded_node_id_at(u32::from_le_bytes(
                    bytes[4..8].try_into().unwrap(),
                ) as usize))
            }
            _ => panic!("invalid encoded GapValue variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_gap_value(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_gap_value(self)
    }
}

fn encode_gap_value(value: GapValue<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        GapValue::Normal => bytes[0] = 0,
        GapValue::LengthPercentage(value) => {
            bytes[0] = 1;
            bytes[4..8].copy_from_slice(
                &u32::try_from(value.index())
                    .expect("AST node ID exceeds four bytes")
                    .to_le_bytes(),
            );
        }
    }
    NodePayload::inline(&bytes)
}
