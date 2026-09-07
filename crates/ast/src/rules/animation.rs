use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Transition<'a> {
    pub delay: Time,
    pub duration: Time,
    pub property: NodeId<'a, PropertyId<'a>>,
    pub timing_function: NodeId<'a, EasingFunction>,
}

#[derive(Clone, Copy)]
struct TransitionHeader<'a> {
    delay: f32,
    duration: f32,
    property: NodeId<'a, PropertyId<'a>>,
    extra: u32,
}

#[derive(Clone, Copy)]
struct TransitionFields<'a> {
    timing_function: NodeId<'a, EasingFunction>,
    delay_unit: TransitionTimeUnit,
    duration_unit: TransitionTimeUnit,
}

#[derive(Clone, Copy)]
enum TransitionTimeUnit {
    Seconds,
    Milliseconds,
}

impl TransitionTimeUnit {
    fn split(time: Time) -> (Self, f32) {
        match time {
            Time::Seconds(value) => (Self::Seconds, value),
            Time::Milliseconds(value) => (Self::Milliseconds, value),
        }
    }
    fn with_value(self, value: f32) -> Time {
        match self {
            Self::Seconds => Time::Seconds(value),
            Self::Milliseconds => Time::Milliseconds(value),
        }
    }
}

pub use transition_access::TransitionRead;

// A snapshot of native fields, not a persistent AST or visitor target.
mod transition_access {
    use super::*;

    pub struct TransitionRead<'id> {
        header: TransitionHeader<'id>,
        fields: TransitionFields<'id>,
    }

    impl<'id> TransitionRead<'id> {
        pub fn property(&self) -> NodeId<'id, PropertyId<'id>> {
            self.header.property
        }

        pub fn duration(&self) -> Time {
            self.fields.duration_unit.with_value(self.header.duration)
        }

        pub fn timing_function(&self) -> NodeId<'id, EasingFunction> {
            self.fields.timing_function
        }

        /// Codegen omits either unit of zero delay, including negative zero.
        pub fn nonzero_delay(&self) -> Option<Time> {
            (self.header.delay != 0.0).then(|| self.fields.delay_unit.with_value(self.header.delay))
        }
    }

    impl AstContext<'_> {
        pub fn transition<'id>(&self, id: NodeId<'id, Transition<'id>>) -> TransitionRead<'id> {
            // SAFETY: the checked kind owns this header and one TransitionFields slot.
            let header: TransitionHeader<'id> = unsafe { self.node_payload(id).read_value() };
            let fields = unsafe { self.extra_slot(header.extra as usize).read_value() };
            TransitionRead { header, fields }
        }
    }
}

// SAFETY: this kind stores TransitionHeader with one typed TransitionFields slot.
unsafe impl<'ast> AstNodeStorage<'ast> for Transition<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0024_0001);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: TransitionHeader<'ast> = unsafe { payload.read_value() };
        let fields: TransitionFields<'ast> =
            unsafe { context.extra_slot(header.extra as usize).read_value() };
        Self {
            delay: fields.delay_unit.with_value(header.delay),
            duration: fields.duration_unit.with_value(header.duration),
            property: header.property,
            timing_function: fields.timing_function,
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_transition(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: TransitionHeader<'ast> = unsafe { current.read_value() };
        encode_transition(self, Some(header.extra as usize), context)
    }
}

impl<'ast> AstNodeClone<'ast> for Transition<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            delay: self.delay,
            duration: self.duration,
            property: context.clone_encoded_node(self.property),
            timing_function: context.clone_encoded_node(self.timing_function),
        }
    }
}

fn encode_transition<'ast>(
    value: Transition<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let (delay_unit, delay) = TransitionTimeUnit::split(value.delay);
    let (duration_unit, duration) = TransitionTimeUnit::split(value.duration);
    let fields = ExtraData::from_value(TransitionFields {
        timing_function: value.timing_function,
        delay_unit,
        duration_unit,
    });
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, fields);
            extra
        }
        None => context.alloc_extra_slots([fields]),
    };
    NodePayload::from_value(TransitionHeader {
        delay,
        duration,
        property: value.property,
        extra: u32::try_from(extra).expect("AST extra index exceeds u32"),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ScrollTimeline {
    pub axis: ScrollAxis,
    pub scroller: Scroller,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ViewTimeline<'a> {
    pub axis: ScrollAxis,
    pub inset: NodeId<'a, Size2D<'a, LengthPercentageOrAuto<'a>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct AnimationRange<'a> {
    pub end: NodeId<'a, AnimationRangeEnd<'a>>,
    pub start: NodeId<'a, AnimationRangeStart<'a>>,
}

impl_inline_extra!(AnimationRange<'ast>);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Animation<'a> {
    /// Components in authored order, so parsing and printing round-trips
    /// losslessly. The `ORDER_VALUES` minify pass sorts them into canonical
    /// order in place.
    pub components: Vec<'a, AnimationComponent<'a>>,
}

impl_inline_extra!(Animation<'ast>);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum AnimationComponent<'a> {
    Name(NodeId<'a, AnimationName<'a>>),
    Duration(Time),
    TimingFunction(NodeId<'a, EasingFunction>),
    Delay(Time),
    IterationCount(AnimationIterationCount),
    Direction(AnimationDirection),
    FillMode(AnimationFillMode),
    PlayState(AnimationPlayState),
}

#[derive(Clone, Copy)]
enum AnimationComponentSlot<'a> {
    Name(NodeId<'a, AnimationName<'a>>),
    DurationSeconds(f32),
    DurationMilliseconds(f32),
    TimingFunction(NodeId<'a, EasingFunction>),
    DelaySeconds(f32),
    DelayMilliseconds(f32),
    IterationCount(f32),
    Infinite,
    Direction(AnimationDirection),
    FillMode(AnimationFillMode),
    PlayState(AnimationPlayState),
}

// SAFETY: the typed range writes and reads AnimationComponentSlot. Flattening
// nested Time/count variants is necessary to retain one eight-byte slot.
unsafe impl<'ast> ExtraDataCompact<'ast> for AnimationComponent<'ast> {
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(match self {
            Self::Name(v) => AnimationComponentSlot::Name(v),
            Self::Duration(Time::Seconds(v)) => AnimationComponentSlot::DurationSeconds(v),
            Self::Duration(Time::Milliseconds(v)) => {
                AnimationComponentSlot::DurationMilliseconds(v)
            }
            Self::TimingFunction(v) => AnimationComponentSlot::TimingFunction(v),
            Self::Delay(Time::Seconds(v)) => AnimationComponentSlot::DelaySeconds(v),
            Self::Delay(Time::Milliseconds(v)) => AnimationComponentSlot::DelayMilliseconds(v),
            Self::IterationCount(AnimationIterationCount::Number(v)) => {
                AnimationComponentSlot::IterationCount(v)
            }
            Self::IterationCount(AnimationIterationCount::Infinite) => {
                AnimationComponentSlot::Infinite
            }
            Self::Direction(v) => AnimationComponentSlot::Direction(v),
            Self::FillMode(v) => AnimationComponentSlot::FillMode(v),
            Self::PlayState(v) => AnimationComponentSlot::PlayState(v),
        })
    }
    unsafe fn decode_extra(data: ExtraData) -> Self {
        match unsafe { data.read_value::<AnimationComponentSlot<'ast>>() } {
            AnimationComponentSlot::Name(v) => Self::Name(v),
            AnimationComponentSlot::DurationSeconds(v) => Self::Duration(Time::Seconds(v)),
            AnimationComponentSlot::DurationMilliseconds(v) => {
                Self::Duration(Time::Milliseconds(v))
            }
            AnimationComponentSlot::TimingFunction(v) => Self::TimingFunction(v),
            AnimationComponentSlot::DelaySeconds(v) => Self::Delay(Time::Seconds(v)),
            AnimationComponentSlot::DelayMilliseconds(v) => Self::Delay(Time::Milliseconds(v)),
            AnimationComponentSlot::IterationCount(v) => {
                Self::IterationCount(AnimationIterationCount::Number(v))
            }
            AnimationComponentSlot::Infinite => {
                Self::IterationCount(AnimationIterationCount::Infinite)
            }
            AnimationComponentSlot::Direction(v) => Self::Direction(v),
            AnimationComponentSlot::FillMode(v) => Self::FillMode(v),
            AnimationComponentSlot::PlayState(v) => Self::PlayState(v),
        }
    }
}

/// The keyword class an animation component (or a keyframes name colliding
/// with one) belongs to, used to keep shorthand serialization round-trip
/// safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub enum AnimationKeywordClass {
    TimingFunction,
    IterationCount,
    Direction,
    FillMode,
    PlayState,
}

impl AnimationComponent<'_> {
    /// The keyword class of a non-name component.
    pub fn keyword_class(&self) -> Option<AnimationKeywordClass> {
        match self {
            Self::TimingFunction(_) => Some(AnimationKeywordClass::TimingFunction),
            Self::IterationCount(_) => Some(AnimationKeywordClass::IterationCount),
            Self::Direction(_) => Some(AnimationKeywordClass::Direction),
            Self::FillMode(_) => Some(AnimationKeywordClass::FillMode),
            Self::PlayState(_) => Some(AnimationKeywordClass::PlayState),
            Self::Name(_) | Self::Duration(_) | Self::Delay(_) => None,
        }
    }
}

impl AnimationName<'_> {
    /// The keyword class this name collides with on reparse, mirroring the
    /// disambiguation in lightningcss and stylo. Quoted names print without
    /// quotes unless they are CSS-wide keywords or `none`, so they collide
    /// like idents; the `none` name is excluded because fill-mode's initial
    /// value is already `none`.
    pub fn keyword_class(&self, context: &AstContext<'_>) -> Option<AnimationKeywordClass> {
        let name = match self {
            Self::Ident(name) | Self::String(name) => context.str(*name),
            Self::None => return None,
        };
        match_ignore_ascii_case!(
            name,
            "linear" | "ease" | "ease-in" | "ease-out" | "ease-in-out" | "step-start" | "step-end" =>
                Some(AnimationKeywordClass::TimingFunction),
            "infinite" => Some(AnimationKeywordClass::IterationCount),
            "normal" | "reverse" | "alternate" | "alternate-reverse" =>
                Some(AnimationKeywordClass::Direction),
            "forwards" | "backwards" | "both" => Some(AnimationKeywordClass::FillMode),
            "running" | "paused" => Some(AnimationKeywordClass::PlayState),
            _ => None,
        )
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        Animation, AnimationComponent, AnimationDirection, AnimationIterationCount, AnimationName,
        AstContext, DUMMY_SP, Time,
    };

    #[test]
    fn transition_native_fields_preserve_units_and_float_bits_without_growth() {
        use crate::{EasingFunction, PropertyId, Transition};
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let property = ast.alloc_node(PropertyId::Opacity, DUMMY_SP);
        let timing_function = ast.alloc_node(EasingFunction::Ease, DUMMY_SP);
        let node = ast.alloc_node(
            Transition {
                delay: Time::Seconds(0.0),
                duration: Time::Milliseconds(0.0),
                property,
                timing_function,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_0123,
        ] {
            for duration_ms in [false, true] {
                for delay_ms in [false, true] {
                    for special_is_delay in [false, true] {
                        let duration_bits = if special_is_delay {
                            1.25_f32.to_bits()
                        } else {
                            bits
                        };
                        let delay_bits = if special_is_delay {
                            bits
                        } else {
                            2.5_f32.to_bits()
                        };
                        let time = |bits, milliseconds| {
                            let value = f32::from_bits(bits);
                            if milliseconds {
                                Time::Milliseconds(value)
                            } else {
                                Time::Seconds(value)
                            }
                        };
                        ast.mutate_node(node, |value, _| {
                            value.delay = time(delay_bits, delay_ms);
                            value.duration = time(duration_bits, duration_ms);
                        });
                        let value = ast.resolve_node(node);
                        assert_eq!(value.property, property);
                        assert_eq!(value.timing_function, timing_function);
                        let view = ast.transition(node);
                        assert_eq!(view.property(), property);
                        assert_eq!(view.timing_function(), timing_function);
                        let check = |time: Time, bits: u32, milliseconds: bool| {
                            let value = match time {
                                Time::Seconds(value) => {
                                    assert!(!milliseconds);
                                    value
                                }
                                Time::Milliseconds(value) => {
                                    assert!(milliseconds);
                                    value
                                }
                            };
                            assert_eq!(value.to_bits(), bits);
                        };
                        check(value.duration, duration_bits, duration_ms);
                        check(view.duration(), duration_bits, duration_ms);
                        check(value.delay, delay_bits, delay_ms);
                        if f32::from_bits(delay_bits) == 0.0 {
                            assert!(view.nonzero_delay().is_none());
                        } else {
                            check(view.nonzero_delay().unwrap(), delay_bits, delay_ms);
                        }
                        assert_eq!(ast.node_checkpoint(), checkpoint);
                    }
                }
            }
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }

    #[test]
    fn compact_animation_components_preserve_all_variants_and_float_bits() {
        use crate::{AnimationFillMode, AnimationPlayState, EasingFunction};
        use AnimationComponent as C;
        fn assert_same(left: C<'_>, right: C<'_>) {
            assert_eq!(
                std::mem::discriminant(&left),
                std::mem::discriminant(&right)
            );
            match (left, right) {
                (C::Duration(a), C::Duration(b)) | (C::Delay(a), C::Delay(b)) => {
                    assert_eq!(std::mem::discriminant(&a), std::mem::discriminant(&b));
                    let bits = |time| match time {
                        Time::Seconds(v) | Time::Milliseconds(v) => v.to_bits(),
                    };
                    assert_eq!(bits(a), bits(b));
                }
                (
                    C::IterationCount(AnimationIterationCount::Number(a)),
                    C::IterationCount(AnimationIterationCount::Number(b)),
                ) => assert_eq!(a.to_bits(), b.to_bits()),
                (a, b) => assert_eq!(a, b),
            }
        }
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.alloc_node(AnimationName::None, DUMMY_SP);
        let timing = ast.alloc_node(EasingFunction::Linear, DUMMY_SP);
        let mut cases = std::vec![
            C::Name(name),
            C::TimingFunction(timing),
            C::IterationCount(AnimationIterationCount::Infinite)
        ];
        cases.extend(
            [
                AnimationDirection::Normal,
                AnimationDirection::Reverse,
                AnimationDirection::Alternate,
                AnimationDirection::AlternateReverse,
            ]
            .map(C::Direction),
        );
        cases.extend(
            [
                AnimationFillMode::None,
                AnimationFillMode::Forwards,
                AnimationFillMode::Backwards,
                AnimationFillMode::Both,
            ]
            .map(C::FillMode),
        );
        cases.extend([AnimationPlayState::Running, AnimationPlayState::Paused].map(C::PlayState));
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc1_2345,
        ] {
            let value = f32::from_bits(bits);
            cases.extend([
                C::Duration(Time::Seconds(value)),
                C::Duration(Time::Milliseconds(value)),
                C::Delay(Time::Seconds(value)),
                C::Delay(Time::Milliseconds(value)),
                C::IterationCount(AnimationIterationCount::Number(value)),
            ]);
        }
        let before = (ast.encoded_node_len(), ast.encoded_extra_len());
        let range = ast.alloc_encoded_vec(cases.iter().copied());
        assert_eq!(ast.encoded_extra_len(), before.1 + cases.len());
        assert_eq!(ast.encoded_node_len(), before.0);
        let checkpoint = ast.node_checkpoint();
        for (index, value) in cases.iter().copied().enumerate() {
            assert_same(value, ast.vec_get(range, index).unwrap());
        }
        // Every logical variant can replace the same physical slot without allocation.
        for value in cases.into_iter().rev() {
            ast.vec_set(range, 0, value);
            assert_same(value, ast.vec_get(range, 0).unwrap());
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
        assert_eq!(ast.string_pool().extra_len(), 0);
        assert_eq!(ast.string_pool().len(), 0);
    }

    #[test]
    fn animation_and_components_form_nested_one_slot_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let name = context.add_str("fade");
        let name = context.alloc_encoded_node(AnimationName::Ident(name), DUMMY_SP);
        let before = (context.encoded_node_len(), context.encoded_extra_len());
        let components = context.alloc_encoded_vec(
            [
                AnimationComponent::Duration(Time::Seconds(1.5)),
                AnimationComponent::IterationCount(AnimationIterationCount::Infinite),
                AnimationComponent::Direction(AnimationDirection::Alternate),
                AnimationComponent::Name(name),
            ]
            .into_iter(),
        );
        assert_eq!(context.encoded_node_len(), before.0);
        assert_eq!(context.encoded_extra_len(), before.1 + 4);
        assert_eq!(
            context
                .encoded_vec_iter(components)
                .collect::<std::vec::Vec<_>>(),
            [
                AnimationComponent::Duration(Time::Seconds(1.5)),
                AnimationComponent::IterationCount(AnimationIterationCount::Infinite),
                AnimationComponent::Direction(AnimationDirection::Alternate),
                AnimationComponent::Name(name),
            ]
        );

        let animations = context.alloc_encoded_vec([Animation { components }].into_iter());
        let after_animations = context.node_checkpoint();
        assert_eq!(context.encoded_node_len(), before.0);
        assert_eq!(context.encoded_extra_len(), before.1 + 5);
        assert_eq!(
            context.encoded_vec_get(animations, 0),
            Some(Animation { components })
        );
        assert_eq!(context.node_checkpoint(), after_animations);
    }
}
