use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
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

#[derive(Clone, Copy)]
enum EasingData {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier { x1: f32, x2: f32 },
    Frames(i32),
    Steps { count: i32, position: StepPosition },
}

#[derive(Clone, Copy)]
struct EasingSlot {
    value: EasingData,
    // u32::MAX means no cubic overflow has been allocated yet.
    extra: u32,
}

pub use easing_access::{CubicBezierRead, EasingFunctionRead};
mod easing_access {
    use super::*;
    pub enum EasingFunctionRead<'context, 'storage> {
        Linear,
        Ease,
        EaseIn,
        EaseOut,
        EaseInOut,
        Frames(i32),
        Steps { count: i32, position: StepPosition },
        CubicBezier(CubicBezierRead<'context, 'storage>),
    }
    pub struct CubicBezierRead<'context, 'storage> {
        context: &'context AstContext<'storage>,
        x1: f32,
        x2: f32,
        extra: u32,
    }
    impl CubicBezierRead<'_, '_> {
        pub fn coordinates(&self) -> [f32; 4] {
            // SAFETY: the cubic variant owns an extra slot written as [y1, y2].
            let [y1, y2]: [f32; 2] =
                unsafe { self.context.extra_slot(self.extra as usize).read_value() };
            [self.x1, y1, self.x2, y2]
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn easing_function(
            &self,
            id: NodeId<'_, EasingFunction>,
        ) -> EasingFunctionRead<'_, 'storage> {
            // SAFETY: node_payload checks the owning kind before reading EasingSlot.
            let slot = unsafe { self.node_payload(id).read_value::<EasingSlot>() };
            match slot.value {
                EasingData::Linear => EasingFunctionRead::Linear,
                EasingData::Ease => EasingFunctionRead::Ease,
                EasingData::EaseIn => EasingFunctionRead::EaseIn,
                EasingData::EaseOut => EasingFunctionRead::EaseOut,
                EasingData::EaseInOut => EasingFunctionRead::EaseInOut,
                EasingData::Frames(count) => EasingFunctionRead::Frames(count),
                EasingData::Steps { count, position } => {
                    EasingFunctionRead::Steps { count, position }
                }
                EasingData::CubicBezier { x1, x2 } => {
                    EasingFunctionRead::CubicBezier(CubicBezierRead {
                        context: self,
                        x1,
                        x2,
                        extra: slot.extra,
                    })
                }
            }
        }
    }
}

// SAFETY: this kind stores EasingSlot; cubic overflow stores [f32; 2].
unsafe impl AstNodeStorage<'_> for EasingFunction {
    const KIND: NodeKind = NodeKind::new(0x000b_0001);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
        let slot = unsafe { payload.read_value::<EasingSlot>() };
        match slot.value {
            EasingData::Linear => Self::Linear,
            EasingData::Ease => Self::Ease,
            EasingData::EaseIn => Self::EaseIn,
            EasingData::EaseOut => Self::EaseOut,
            EasingData::EaseInOut => Self::EaseInOut,
            EasingData::Frames(count) => Self::Frames(count),
            EasingData::Steps { count, position } => Self::Steps { count, position },
            EasingData::CubicBezier { x1, x2 } => {
                let [y1, y2] = unsafe {
                    context
                        .extra_slot(slot.extra as usize)
                        .read_value::<[f32; 2]>()
                };
                Self::CubicBezier { x1, x2, y1, y2 }
            }
        }
    }
    fn encode_new(self, context: &mut AstContext<'_>) -> NodePayload {
        encode_easing_function(self, u32::MAX, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'_>,
    ) -> NodePayload {
        let slot = unsafe { current.read_value::<EasingSlot>() };
        encode_easing_function(self, slot.extra, context)
    }
}

impl AstNodeClone<'_> for EasingFunction {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_easing_function(
    value: EasingFunction,
    mut extra: u32,
    context: &mut AstContext<'_>,
) -> NodePayload {
    let value = match value {
        EasingFunction::Linear => EasingData::Linear,
        EasingFunction::Ease => EasingData::Ease,
        EasingFunction::EaseIn => EasingData::EaseIn,
        EasingFunction::EaseOut => EasingData::EaseOut,
        EasingFunction::EaseInOut => EasingData::EaseInOut,
        EasingFunction::Frames(count) => EasingData::Frames(count),
        EasingFunction::Steps { count, position } => EasingData::Steps { count, position },
        EasingFunction::CubicBezier { x1, x2, y1, y2 } => {
            let tail = ExtraData::from_value([y1, y2]);
            if extra == u32::MAX {
                let index = context.alloc_extra_slots([tail]);
                assert!(
                    index < u32::MAX as usize,
                    "easing extra index exceeds available u32 range"
                );
                extra = index as u32;
            } else {
                context.set_extra_slot(extra as usize, tail);
            }
            EasingData::CubicBezier { x1, x2 }
        }
    };
    NodePayload::from_value(EasingSlot { value, extra })
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum StepPosition {
    Start,
    End,
    JumpNone,
    JumpBoth,
}

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub enum AnimationIterationCount {
    Number(f32),
    Infinite,
}

impl_inline_extra!(AnimationIterationCount);

impl ExtraDataClone<'_> for AnimationIterationCount {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

impl_inline_extra!(AnimationDirection);

impl ExtraDataClone<'_> for AnimationDirection {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

impl_inline_extra!(AnimationPlayState);

impl ExtraDataClone<'_> for AnimationPlayState {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl_inline_extra!(AnimationFillMode);

impl ExtraDataClone<'_> for AnimationFillMode {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum AnimationComposition {
    Replace,
    Add,
    Accumulate,
}

impl_inline_extra!(AnimationComposition);

impl ExtraDataClone<'_> for AnimationComposition {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum AnimationTimeline<'a> {
    Auto,
    None,
    DashedIdent(AstStr<'a>),
    Scroll(ScrollTimeline),
    View(ViewTimeline<'a>),
}

// SAFETY: this kind stores and reads the native AnimationTimeline value.
unsafe impl<'ast> AstNodeStorage<'ast> for AnimationTimeline<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000b_0003);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::DashedIdent(a), Self::DashedIdent(b)) => context.str(*a) == context.str(*b),
            _ => self == other,
        }
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for AnimationTimeline<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::View(value) => Self::View(ViewTimeline {
                axis: value.axis,
                inset: context.clone_encoded_node(value.inset),
            }),
            value => value,
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum ScrollAxis {
    Block,
    Inline,
    X,
    Y,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum Scroller {
    Root,
    Nearest,
    Self_,
}

pub type AnimationRangeStart<'a> = AnimationAttachmentRange<'a>;

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
#[repr(u8)]
pub enum AnimationAttachmentRange<'a> {
    Normal,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    TimelineRange {
        name: TimelineRangeName,
        offset: NodeId<'a, LengthPercentage<'a>>,
    },
}

impl_inline_extra!(AnimationAttachmentRange<'ast>);

impl<'ast> ExtraDataClone<'ast> for AnimationAttachmentRange<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
            Self::TimelineRange { name, offset } => Self::TimelineRange {
                name,
                offset: context.clone_encoded_node(offset),
            },
            Self::Normal => Self::Normal,
        }
    }
}

impl_inline_node!(AnimationAttachmentRange<'ast>, 0x000b_0002);

impl<'ast> AstNodeClone<'ast> for AnimationAttachmentRange<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_extra(context)
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum TimelineRangeName {
    Cover,
    Contain,
    Entry,
    Exit,
    EntryCrossing,
    ExitCrossing,
}

pub type AnimationRangeEnd<'a> = AnimationAttachmentRange<'a>;

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AnimationAttachmentRange, AnimationComposition, AnimationDirection, AnimationFillMode,
        AnimationIterationCount, AnimationTimeline, AstContext, DUMMY_SP, DimensionPercentage,
        EasingFunction, ScrollAxis, ScrollTimeline, Scroller, StepPosition, TimelineRangeName,
    };

    #[test]
    fn easing_native_storage_allocates_only_for_cubic_and_preserves_float_bits() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let before = ast.encoded_extra_len();
        let node = ast.alloc_node(EasingFunction::Linear, DUMMY_SP);
        for value in [
            EasingFunction::Ease,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
            EasingFunction::EaseInOut,
            EasingFunction::Frames(i32::MIN),
            EasingFunction::Steps {
                count: i32::MAX,
                position: StepPosition::JumpNone,
            },
        ] {
            ast.mutate_node(node, |stored, _| *stored = value);
            assert_eq!(ast.resolve_node(node), value);
        }
        assert_eq!(ast.encoded_extra_len(), before);
        for bits in [0, 0x8000_0000, 1, 0x7f80_0000, 0x7fc0_0123] {
            let f = f32::from_bits(bits);
            ast.mutate_node(node, |stored, _| {
                *stored = EasingFunction::CubicBezier {
                    x1: f,
                    x2: f,
                    y1: f,
                    y2: f,
                }
            });
            let EasingFunction::CubicBezier { x1, x2, y1, y2 } = ast.resolve_node(node) else {
                panic!()
            };
            assert_eq!(
                [x1.to_bits(), x2.to_bits(), y1.to_bits(), y2.to_bits()],
                [bits; 4]
            );
            let super::EasingFunctionRead::CubicBezier(view) = ast.easing_function(node) else {
                panic!()
            };
            assert_eq!(view.coordinates().map(f32::to_bits), [bits; 4]);
            assert_eq!(ast.encoded_extra_len(), before + 1);
        }
    }

    #[test]
    fn easing_function_codec_reuses_one_fixed_overflow_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let before = context.encoded_extra_len();
        let easing = context.alloc_encoded_node(
            EasingFunction::CubicBezier {
                x1: 0.1,
                x2: 0.2,
                y1: 0.3,
                y2: 0.4,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 1);
        assert_eq!(
            context.encoded_node(easing),
            EasingFunction::CubicBezier {
                x1: 0.1,
                x2: 0.2,
                y1: 0.3,
                y2: 0.4,
            }
        );

        context.mutate_encoded_node(easing, |value, _| {
            *value = EasingFunction::Steps {
                count: 4,
                position: StepPosition::JumpBoth,
            };
        });
        assert_eq!(context.encoded_extra_len(), before + 1);
        assert_eq!(
            context.encoded_node(easing),
            EasingFunction::Steps {
                count: 4,
                position: StepPosition::JumpBoth,
            }
        );
        let checkpoint = context.node_checkpoint();
        context.mutate_encoded_node(easing, |value, _| {
            *value = EasingFunction::CubicBezier {
                x1: 0.1,
                y1: 0.3,
                x2: 0.2,
                y2: 0.4,
            };
        });
        assert_eq!(context.node_checkpoint(), checkpoint);
    }

    #[test]
    fn easing_variant_cycles_preserve_fields_and_reuse_cubic_tail() {
        use super::{EasingFunctionRead, EasingSlot};

        assert_eq!(std::mem::size_of::<EasingSlot>(), 16);
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let node = ast.alloc_node(EasingFunction::Linear, DUMMY_SP);
        assert_eq!(ast.encoded_extra_len(), 0);
        let inline = [
            EasingFunction::Linear,
            EasingFunction::Ease,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
            EasingFunction::EaseInOut,
        ]
        .into_iter()
        .chain(
            [i32::MIN, -1, 0, 1, i32::MAX]
                .into_iter()
                .flat_map(|count| {
                    [
                        EasingFunction::Frames(count),
                        EasingFunction::Steps {
                            count,
                            position: StepPosition::Start,
                        },
                        EasingFunction::Steps {
                            count,
                            position: StepPosition::End,
                        },
                        EasingFunction::Steps {
                            count,
                            position: StepPosition::JumpNone,
                        },
                        EasingFunction::Steps {
                            count,
                            position: StepPosition::JumpBoth,
                        },
                    ]
                }),
        )
        .collect::<std::vec::Vec<_>>();
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_1234,
        ] {
            for position in 0..4 {
                let mut expected = [0.125_f32, 0.25, 0.5, 0.75].map(f32::to_bits);
                expected[position] = bits;
                let [x1, y1, x2, y2] = expected.map(f32::from_bits);
                for &value in &inline {
                    ast.mutate_node(node, |stored, _| *stored = value);
                    assert_eq!(ast.resolve_node(node), value);
                    let read = match ast.easing_function(node) {
                        EasingFunctionRead::Linear => EasingFunction::Linear,
                        EasingFunctionRead::Ease => EasingFunction::Ease,
                        EasingFunctionRead::EaseIn => EasingFunction::EaseIn,
                        EasingFunctionRead::EaseOut => EasingFunction::EaseOut,
                        EasingFunctionRead::EaseInOut => EasingFunction::EaseInOut,
                        EasingFunctionRead::Frames(count) => EasingFunction::Frames(count),
                        EasingFunctionRead::Steps { count, position } => {
                            EasingFunction::Steps { count, position }
                        }
                        EasingFunctionRead::CubicBezier(_) => panic!("expected inline easing"),
                    };
                    assert_eq!(read, value);
                    ast.mutate_node(node, |stored, _| {
                        *stored = EasingFunction::CubicBezier { x1, y1, x2, y2 }
                    });
                    assert_eq!(ast.encoded_extra_len(), 1);
                    let checkpoint = ast.node_checkpoint();
                    let EasingFunction::CubicBezier { x1, y1, x2, y2 } = ast.resolve_node(node)
                    else {
                        panic!("expected cubic easing");
                    };
                    assert_eq!([x1, y1, x2, y2].map(f32::to_bits), expected);
                    let EasingFunctionRead::CubicBezier(view) = ast.easing_function(node) else {
                        panic!("expected cubic view");
                    };
                    assert_eq!(view.coordinates().map(f32::to_bits), expected);
                    assert_eq!(ast.node_checkpoint(), checkpoint);
                    assert_eq!(ast.string_pool().extra_len(), 0);
                }
            }
        }
    }

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
        let name = context.add_str("--progress");
        let named = context.alloc_encoded_node(AnimationTimeline::DashedIdent(name), DUMMY_SP);
        let scrolling = context.alloc_encoded_node(
            AnimationTimeline::Scroll(ScrollTimeline {
                axis: ScrollAxis::Inline,
                scroller: Scroller::Nearest,
            }),
            DUMMY_SP,
        );
        let timelines = context.alloc_encoded_vec([named, scrolling].into_iter());
        assert_eq!(
            context.encoded_node(context.encoded_vec_get(timelines, 0).unwrap()),
            AnimationTimeline::DashedIdent(name)
        );
        assert_eq!(
            context.encoded_node(context.encoded_vec_get(timelines, 1).unwrap()),
            AnimationTimeline::Scroll(ScrollTimeline {
                axis: ScrollAxis::Inline,
                scroller: Scroller::Nearest,
            })
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
