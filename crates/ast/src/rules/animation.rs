use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Transition<'a> {
    pub delay: Time,
    pub duration: Time,
    pub property: NodeId<'a, PropertyId<'a>>,
    pub timing_function: NodeId<'a, EasingFunction>,
}

impl<'ast> AstNodeStorage<'ast> for Transition<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0024_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let ids = context.extra_slot(payload.extra_start()).as_u64();
        Self {
            delay: crate::token::decode_time(bytes[0], f32::from_bits(read_u32(&bytes, 4))),
            duration: crate::token::decode_time(bytes[1], f32::from_bits(read_u32(&bytes, 8))),
            property: context.encoded_node_id_at(ids as u32 as usize),
            timing_function: context.encoded_node_id_at((ids >> 32) as u32 as usize),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_transition(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_transition(self, Some(current.extra_start()), context)
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
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    let (delay_kind, delay) = crate::token::encode_time(value.delay);
    let (duration_kind, duration) = crate::token::encode_time(value.duration);
    bytes[0] = delay_kind;
    bytes[1] = duration_kind;
    write_u32(&mut bytes, 4, delay.to_bits());
    write_u32(&mut bytes, 8, duration.to_bits());
    let ids = ExtraData::from_u64(
        node_index(value.property) as u64 | (node_index(value.timing_function) as u64) << 32,
    );
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, ids);
            extra
        }
        None => context.alloc_extra_slots([ids]),
    };
    NodePayload::with_extra(&bytes, extra)
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollTimeline {
    pub axis: ScrollAxis,
    pub scroller: Scroller,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ViewTimeline<'a> {
    pub axis: ScrollAxis,
    pub inset: NodeId<'a, Size2D<'a, LengthPercentageOrAuto<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct AnimationRange<'a> {
    pub end: NodeId<'a, AnimationRangeEnd<'a>>,
    pub start: NodeId<'a, AnimationRangeStart<'a>>,
}

impl<'ast> ExtraDataCompact<'ast> for AnimationRange<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        write_u32(&mut bytes, 0, node_index(self.end));
        write_u32(&mut bytes, 4, node_index(self.start));
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let bytes = data.bytes();
        Self {
            end: context.encoded_node_id_at(read_u32(&bytes, 0) as usize),
            start: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Animation<'a> {
    /// Components in authored order, so parsing and printing round-trips
    /// losslessly. The `ORDER_VALUES` minify pass sorts them into canonical
    /// order in place.
    pub components: Vec<'a, AnimationComponent<'a>>,
}

impl<'ast> ExtraDataCompact<'ast> for Animation<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        encode_range(self.components)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        Self {
            components: context.encoded_vec_range(
                data.as_u64() as u32 as usize,
                (data.as_u64() >> 32) as u32 as usize,
            ),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
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

impl<'ast> ExtraDataCompact<'ast> for AnimationComponent<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        match self {
            Self::Name(value) => write_tagged_node_id(&mut bytes, 0, value),
            Self::Duration(value) => write_time(&mut bytes, 1, value),
            Self::TimingFunction(value) => write_tagged_node_id(&mut bytes, 2, value),
            Self::Delay(value) => write_time(&mut bytes, 3, value),
            Self::IterationCount(value) => {
                bytes[0] = 4;
                match value {
                    AnimationIterationCount::Number(value) => {
                        write_u32(&mut bytes, 4, value.to_bits());
                    }
                    AnimationIterationCount::Infinite => bytes[1] = 1,
                }
            }
            Self::Direction(value) => {
                bytes[0] = 5;
                bytes[1] = crate::encode_animation_direction(value);
            }
            Self::FillMode(value) => {
                bytes[0] = 6;
                bytes[1] = crate::encode_animation_fill_mode(value);
            }
            Self::PlayState(value) => {
                bytes[0] = 7;
                bytes[1] = match value {
                    AnimationPlayState::Running => 0,
                    AnimationPlayState::Paused => 1,
                };
            }
        }
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let bytes = data.bytes();
        match bytes[0] {
            0 => Self::Name(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            1 => Self::Duration(read_time(&bytes)),
            2 => Self::TimingFunction(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            3 => Self::Delay(read_time(&bytes)),
            4 => Self::IterationCount(match bytes[1] {
                0 => AnimationIterationCount::Number(f32::from_bits(read_u32(&bytes, 4))),
                1 => AnimationIterationCount::Infinite,
                _ => panic!("invalid encoded AnimationIterationCount variant"),
            }),
            5 => Self::Direction(crate::decode_animation_direction(bytes[1])),
            6 => Self::FillMode(crate::decode_animation_fill_mode(bytes[1])),
            7 => Self::PlayState(match bytes[1] {
                0 => AnimationPlayState::Running,
                1 => AnimationPlayState::Paused,
                _ => panic!("invalid encoded AnimationPlayState"),
            }),
            _ => panic!("invalid encoded AnimationComponent variant"),
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
    pub fn keyword_class(&self) -> Option<AnimationKeywordClass> {
        let name = match self {
            Self::Ident(name) | Self::String(name) => *name,
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

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn write_tagged_node_id<T>(bytes: &mut [u8], tag: u8, id: NodeId<'_, T>) {
    bytes[0] = tag;
    write_u32(bytes, 4, node_index(id));
}

fn write_time(bytes: &mut [u8], tag: u8, value: Time) {
    let (kind, value) = crate::token::encode_time(value);
    bytes[0] = tag;
    bytes[1] = kind;
    write_u32(bytes, 4, value.to_bits());
}

fn read_time(bytes: &[u8]) -> Time {
    crate::token::decode_time(bytes[1], f32::from_bits(read_u32(bytes, 4)))
}

fn encode_range<T>(range: Vec<'_, T>) -> ExtraData {
    let start = u32::try_from(range.start_index()).expect("AST range start exceeds four bytes");
    let end = u32::try_from(range.end_index()).expect("AST range end exceeds four bytes");
    ExtraData::from_u64((end as u64) << 32 | start as u64)
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
        Animation, AnimationComponent, AnimationDirection, AnimationIterationCount, AnimationName,
        AstContext, DUMMY_SP, Time,
    };

    #[test]
    fn animation_and_components_form_nested_one_slot_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let name = context.alloc_encoded_node(AnimationName::Ident("fade"), DUMMY_SP);
        let components = context.alloc_encoded_vec(
            [
                AnimationComponent::Duration(Time::Seconds(1.5)),
                AnimationComponent::IterationCount(AnimationIterationCount::Infinite),
                AnimationComponent::Direction(AnimationDirection::Alternate),
                AnimationComponent::Name(name),
            ]
            .into_iter(),
        );
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
        assert_eq!(
            context.encoded_vec_get(animations, 0),
            Some(Animation { components })
        );
    }
}
