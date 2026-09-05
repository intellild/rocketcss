use crate::*;
use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};
#[derive(Debug, PartialEq, Visit)]
pub enum KeyframeSelector {
    Percentage(f32),
    From,
    To,
    TimelineRangePercentage(TimelineRangePercentage),
}

impl ExtraDataCompact<'_> for KeyframeSelector {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        match self {
            Self::Percentage(value) => {
                bytes[0] = 0;
                bytes[4..8].copy_from_slice(&value.to_bits().to_le_bytes());
            }
            Self::From => bytes[0] = 1,
            Self::To => bytes[0] = 2,
            Self::TimelineRangePercentage(value) => {
                bytes[0] = 3;
                bytes[1] = encode_timeline_range_name(value.name);
                bytes[4..8].copy_from_slice(&value.percentage.to_bits().to_le_bytes());
            }
        }
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        let bytes = data.bytes();
        let percentage = f32::from_bits(u32::from_le_bytes(bytes[4..8].try_into().unwrap()));
        match bytes[0] {
            0 => Self::Percentage(percentage),
            1 => Self::From,
            2 => Self::To,
            3 => Self::TimelineRangePercentage(TimelineRangePercentage {
                name: decode_timeline_range_name(bytes[1]),
                percentage,
            }),
            _ => panic!("invalid encoded KeyframeSelector"),
        }
    }
}

impl ExtraDataClone<'_> for KeyframeSelector {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_timeline_range_name(value: TimelineRangeName) -> u8 {
    match value {
        TimelineRangeName::Cover => 0,
        TimelineRangeName::Contain => 1,
        TimelineRangeName::Entry => 2,
        TimelineRangeName::Exit => 3,
        TimelineRangeName::EntryCrossing => 4,
        TimelineRangeName::ExitCrossing => 5,
    }
}

fn decode_timeline_range_name(value: u8) -> TimelineRangeName {
    match value {
        0 => TimelineRangeName::Cover,
        1 => TimelineRangeName::Contain,
        2 => TimelineRangeName::Entry,
        3 => TimelineRangeName::Exit,
        4 => TimelineRangeName::EntryCrossing,
        5 => TimelineRangeName::ExitCrossing,
        _ => panic!("invalid encoded TimelineRangeName"),
    }
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
