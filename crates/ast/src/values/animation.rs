use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum EasingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier { x1: f32, x2: f32, y1: f32, y2: f32 },
    Frames(i32),
    Steps { count: i32, position: StepPosition },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum StepPosition {
    Start,
    End,
    JumpNone,
    JumpBoth,
}

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationIterationCount {
    Number(f32),
    Infinite,
}

impl ExtraDataCompact<'_> for AnimationIterationCount {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        match self {
            Self::Number(value) => write_u32(&mut bytes, 4, value.to_bits()),
            Self::Infinite => bytes[0] = 1,
        }
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        let bytes = data.bytes();
        match bytes[0] {
            0 => Self::Number(f32::from_bits(read_u32(&bytes, 4))),
            1 => Self::Infinite,
            _ => panic!("invalid encoded AnimationIterationCount variant"),
        }
    }
}

impl ExtraDataClone<'_> for AnimationIterationCount {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

impl ExtraDataCompact<'_> for AnimationDirection {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(encode_animation_direction(self) as u64)
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        decode_animation_direction(data.as_u64() as u8)
    }
}

impl ExtraDataClone<'_> for AnimationDirection {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

impl ExtraDataCompact<'_> for AnimationPlayState {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Running => 0,
            Self::Paused => 1,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Running,
            1 => Self::Paused,
            _ => panic!("invalid encoded AnimationPlayState"),
        }
    }
}

impl ExtraDataClone<'_> for AnimationPlayState {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl ExtraDataCompact<'_> for AnimationFillMode {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(encode_animation_fill_mode(self) as u64)
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        decode_animation_fill_mode(data.as_u64() as u8)
    }
}

impl ExtraDataClone<'_> for AnimationFillMode {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AnimationComposition {
    Replace,
    Add,
    Accumulate,
}

impl ExtraDataCompact<'_> for AnimationComposition {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Replace => 0,
            Self::Add => 1,
            Self::Accumulate => 2,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Replace,
            1 => Self::Add,
            2 => Self::Accumulate,
            _ => panic!("invalid encoded AnimationComposition"),
        }
    }
}

impl ExtraDataClone<'_> for AnimationComposition {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationTimeline<'a> {
    Auto,
    None,
    DashedIdent(&'a str),
    Scroll(ScrollTimeline),
    View(ViewTimeline<'a>),
}

impl<'ast> ExtraDataCompact<'ast> for AnimationTimeline<'ast> {
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        match self {
            Self::Auto => bytes[0] = 0,
            Self::None => bytes[0] = 1,
            Self::DashedIdent(value) => {
                bytes[0] = 2;
                write_u32(&mut bytes, 4, context.store_string(value));
            }
            Self::Scroll(value) => {
                bytes[0] = 3;
                bytes[1] = encode_scroll_axis(value.axis);
                bytes[2] = encode_scroller(value.scroller);
            }
            Self::View(value) => {
                bytes[0] = 4;
                bytes[1] = encode_scroll_axis(value.axis);
                write_u32(&mut bytes, 4, node_index(value.inset));
            }
        }
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let bytes = data.bytes();
        match bytes[0] {
            0 => Self::Auto,
            1 => Self::None,
            2 => Self::DashedIdent(context.resolve_string(read_u32(&bytes, 4) as u64)),
            3 => Self::Scroll(ScrollTimeline {
                axis: decode_scroll_axis(bytes[1]),
                scroller: decode_scroller(bytes[2]),
            }),
            4 => Self::View(ViewTimeline {
                axis: decode_scroll_axis(bytes[1]),
                inset: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
            }),
            _ => panic!("invalid encoded AnimationTimeline variant"),
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ScrollAxis {
    Block,
    Inline,
    X,
    Y,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Scroller {
    Root,
    Nearest,
    Self_,
}

pub type AnimationRangeStart<'a> = AnimationAttachmentRange<'a>;

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationAttachmentRange<'a> {
    Normal,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    TimelineRange {
        name: TimelineRangeName,
        offset: NodeId<'a, LengthPercentage<'a>>,
    },
}

impl<'ast> ExtraDataCompact<'ast> for AnimationAttachmentRange<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        match self {
            Self::Normal => bytes[0] = 0,
            Self::LengthPercentage(value) => {
                bytes[0] = 1;
                write_u32(&mut bytes, 4, node_index(value));
            }
            Self::TimelineRange { name, offset } => {
                bytes[0] = 2;
                bytes[1] = encode_timeline_range_name(name);
                write_u32(&mut bytes, 4, node_index(offset));
            }
        }
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let bytes = data.bytes();
        match bytes[0] {
            0 => Self::Normal,
            1 => Self::LengthPercentage(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            2 => Self::TimelineRange {
                name: decode_timeline_range_name(bytes[1]),
                offset: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
            },
            _ => panic!("invalid encoded AnimationAttachmentRange variant"),
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TimelineRangeName {
    Cover,
    Contain,
    Entry,
    Exit,
    EntryCrossing,
    ExitCrossing,
}

pub type AnimationRangeEnd<'a> = AnimationAttachmentRange<'a>;

fn encode_animation_direction(value: AnimationDirection) -> u8 {
    match value {
        AnimationDirection::Normal => 0,
        AnimationDirection::Reverse => 1,
        AnimationDirection::Alternate => 2,
        AnimationDirection::AlternateReverse => 3,
    }
}

fn decode_animation_direction(value: u8) -> AnimationDirection {
    match value {
        0 => AnimationDirection::Normal,
        1 => AnimationDirection::Reverse,
        2 => AnimationDirection::Alternate,
        3 => AnimationDirection::AlternateReverse,
        _ => panic!("invalid encoded AnimationDirection"),
    }
}

fn encode_animation_fill_mode(value: AnimationFillMode) -> u8 {
    match value {
        AnimationFillMode::None => 0,
        AnimationFillMode::Forwards => 1,
        AnimationFillMode::Backwards => 2,
        AnimationFillMode::Both => 3,
    }
}

fn decode_animation_fill_mode(value: u8) -> AnimationFillMode {
    match value {
        0 => AnimationFillMode::None,
        1 => AnimationFillMode::Forwards,
        2 => AnimationFillMode::Backwards,
        3 => AnimationFillMode::Both,
        _ => panic!("invalid encoded AnimationFillMode"),
    }
}

fn encode_scroll_axis(value: ScrollAxis) -> u8 {
    match value {
        ScrollAxis::Block => 0,
        ScrollAxis::Inline => 1,
        ScrollAxis::X => 2,
        ScrollAxis::Y => 3,
    }
}

fn decode_scroll_axis(value: u8) -> ScrollAxis {
    match value {
        0 => ScrollAxis::Block,
        1 => ScrollAxis::Inline,
        2 => ScrollAxis::X,
        3 => ScrollAxis::Y,
        _ => panic!("invalid encoded ScrollAxis"),
    }
}

fn encode_scroller(value: Scroller) -> u8 {
    match value {
        Scroller::Root => 0,
        Scroller::Nearest => 1,
        Scroller::Self_ => 2,
    }
}

fn decode_scroller(value: u8) -> Scroller {
    match value {
        0 => Scroller::Root,
        1 => Scroller::Nearest,
        2 => Scroller::Self_,
        _ => panic!("invalid encoded Scroller"),
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

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact animation field is four bytes"),
    )
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AnimationAttachmentRange, AnimationComposition, AnimationDirection, AnimationFillMode,
        AnimationIterationCount, AnimationTimeline, AstContext, DUMMY_SP, DimensionPercentage,
        ScrollAxis, ScrollTimeline, Scroller, TimelineRangeName,
    };

    #[test]
    fn animation_scalar_values_each_use_one_extra_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let counts = context.alloc_encoded_vec(
            [
                AnimationIterationCount::Number(2.5),
                AnimationIterationCount::Infinite,
            ]
            .into_iter(),
        );
        assert_eq!(
            context
                .encoded_vec_iter(counts)
                .collect::<std::vec::Vec<_>>(),
            [
                AnimationIterationCount::Number(2.5),
                AnimationIterationCount::Infinite,
            ]
        );

        let directions =
            context.alloc_encoded_vec([AnimationDirection::AlternateReverse].into_iter());
        assert_eq!(
            context.encoded_vec_get(directions, 0),
            Some(AnimationDirection::AlternateReverse)
        );
        let fills = context.alloc_encoded_vec([AnimationFillMode::Backwards].into_iter());
        assert_eq!(
            context.encoded_vec_get(fills, 0),
            Some(AnimationFillMode::Backwards)
        );
        let compositions =
            context.alloc_encoded_vec([AnimationComposition::Accumulate].into_iter());
        assert_eq!(
            context.encoded_vec_get(compositions, 0),
            Some(AnimationComposition::Accumulate)
        );
    }

    #[test]
    fn animation_timeline_and_attachment_ranges_round_trip() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let timelines = context.alloc_encoded_vec(
            [
                AnimationTimeline::DashedIdent("--progress"),
                AnimationTimeline::Scroll(ScrollTimeline {
                    axis: ScrollAxis::Inline,
                    scroller: Scroller::Nearest,
                }),
            ]
            .into_iter(),
        );
        assert_eq!(
            context.encoded_vec_get(timelines, 0),
            Some(AnimationTimeline::DashedIdent("--progress"))
        );
        assert_eq!(
            context.encoded_vec_get(timelines, 1),
            Some(AnimationTimeline::Scroll(ScrollTimeline {
                axis: ScrollAxis::Inline,
                scroller: Scroller::Nearest,
            }))
        );

        let offset = context.alloc_encoded_node(DimensionPercentage::Percentage(12.5), DUMMY_SP);
        let ranges = context.alloc_encoded_vec(
            [AnimationAttachmentRange::TimelineRange {
                name: TimelineRangeName::EntryCrossing,
                offset,
            }]
            .into_iter(),
        );
        assert_eq!(
            context.encoded_vec_get(ranges, 0),
            Some(AnimationAttachmentRange::TimelineRange {
                name: TimelineRangeName::EntryCrossing,
                offset,
            })
        );
    }
}
