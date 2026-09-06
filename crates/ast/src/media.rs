use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum MediaCondition<'a> {
    Feature(NodeId<'a, MediaFeature<'a>>),
    Not(NodeId<'a, MediaCondition<'a>>),
    Operation {
        conditions: Vec<'a, NodeId<'a, MediaCondition<'a>>>,
        operator: Operator,
    },
    Unknown(Vec<'a, TokenOrValue<'a>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum QueryFeature<'a, FeatureId> {
    Plain {
        name: MediaFeatureName<'a, FeatureId>,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    Boolean {
        name: MediaFeatureName<'a, FeatureId>,
    },
    Range {
        name: MediaFeatureName<'a, FeatureId>,
        operator: MediaFeatureComparison,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    Interval {
        end: NodeId<'a, MediaFeatureValue<'a>>,
        end_operator: MediaFeatureComparison,
        name: MediaFeatureName<'a, FeatureId>,
        start: NodeId<'a, MediaFeatureValue<'a>>,
        start_operator: MediaFeatureComparison,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum MediaFeatureName<'a, FeatureId> {
    Standard(FeatureId),
    Custom(AstStr<'a>),
    Unknown(AstStr<'a>),
}

pub type MediaFeature<'a> = QueryFeature<'a, MediaFeatureId>;

#[repr(u8)]
#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum MediaFeatureId {
    Width,
    Height,
    AspectRatio,
    Orientation,
    OverflowBlock,
    OverflowInline,
    HorizontalViewportSegments,
    VerticalViewportSegments,
    DisplayMode,
    Resolution,
    Scan,
    Grid,
    Update,
    EnvironmentBlending,
    Color,
    ColorIndex,
    Monochrome,
    ColorGamut,
    DynamicRange,
    InvertedColors,
    Pointer,
    Hover,
    AnyPointer,
    AnyHover,
    NavControls,
    VideoColorGamut,
    VideoDynamicRange,
    Scripting,
    PrefersReducedMotion,
    PrefersReducedTransparency,
    PrefersContrast,
    ForcedColors,
    PrefersColorScheme,
    PrefersReducedData,
    DeviceWidth,
    DeviceHeight,
    DeviceAspectRatio,
    #[css_keyword("-webkit-device-pixel-ratio")]
    WebkitDevicePixelRatio,
    #[css_keyword("-moz-device-pixel-ratio")]
    MozDevicePixelRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum MediaFeatureValue<'a> {
    Length(NodeId<'a, Length<'a>>),
    Number(f32),
    Integer(i32),
    Boolean(bool),
    Resolution(Resolution),
    Ratio(Ratio),
    Ident(AstStr<'a>),
    Env(NodeId<'a, EnvironmentVariable<'a>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum MediaFeatureComparison {
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum Operator {
    And,
    Or,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum MediaType<'a> {
    All,
    Print,
    Screen,
    Custom(AstStr<'a>),
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum Qualifier {
    Only,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum SupportsCondition<'a> {
    Not(NodeId<'a, SupportsCondition<'a>>),
    And(Vec<'a, NodeId<'a, SupportsCondition<'a>>>),
    Or(Vec<'a, NodeId<'a, SupportsCondition<'a>>>),
    Declaration {
        property_id: NodeId<'a, PropertyId<'a>>,
        value: AstStr<'a>,
    },
    Selector(AstStr<'a>),
    Unknown(AstStr<'a>),
}

impl_inline_node!(MediaCondition<'ast>, 0x001a_0001);

impl<'ast> AstNodeClone<'ast> for MediaCondition<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Feature(value) => Self::Feature(context.clone_encoded_node(value)),
            Self::Not(value) => Self::Not(context.clone_encoded_node(value)),
            Self::Operation {
                conditions,
                operator,
            } => Self::Operation {
                conditions: context.clone_encoded_vec(conditions),
                operator,
            },
            Self::Unknown(values) => Self::Unknown(context.clone_encoded_vec(values)),
        }
    }
}

pub trait QueryFeatureId: Copy + PartialEq + feature_access::Sealed {
    const KIND: NodeKind;
}
impl QueryFeatureId for MediaFeatureId {
    const KIND: NodeKind = NodeKind::new(0x001a_0002);
}

// Flatten the name and predicate tags to keep ordinary queries inline. Only
// interval queries with a string name exceed the payload and spill that range.
#[repr(u8)]
#[derive(Clone, Copy)]
enum QueryFeatureSlot<'a, F> {
    StandardPlain {
        name: F,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    StandardBoolean {
        name: F,
    },
    StandardRange {
        operator: MediaFeatureComparison,
        name: F,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    StandardInterval {
        start_operator: MediaFeatureComparison,
        end_operator: MediaFeatureComparison,
        name: F,
        start: NodeId<'a, MediaFeatureValue<'a>>,
        end: NodeId<'a, MediaFeatureValue<'a>>,
    },
    CustomPlain {
        name: AstStr<'a>,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    CustomBoolean {
        name: AstStr<'a>,
    },
    CustomRange {
        operator: MediaFeatureComparison,
        name: AstStr<'a>,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    CustomInterval {
        start_operator: MediaFeatureComparison,
        end_operator: MediaFeatureComparison,
        extra: u32,
        start: NodeId<'a, MediaFeatureValue<'a>>,
        end: NodeId<'a, MediaFeatureValue<'a>>,
    },
    UnknownPlain {
        name: AstStr<'a>,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    UnknownBoolean {
        name: AstStr<'a>,
    },
    UnknownRange {
        operator: MediaFeatureComparison,
        name: AstStr<'a>,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    UnknownInterval {
        start_operator: MediaFeatureComparison,
        end_operator: MediaFeatureComparison,
        extra: u32,
        start: NodeId<'a, MediaFeatureValue<'a>>,
        end: NodeId<'a, MediaFeatureValue<'a>>,
    },
}

pub use feature_access::{QueryFeaturePredicate, QueryFeatureRead};

// These borrowed field views are not persistent AST or visitor targets.
mod feature_access {
    use super::*;

    pub trait Sealed {}
    impl Sealed for MediaFeatureId {}
    impl Sealed for ContainerSizeFeatureId {}
    impl Sealed for ScrollStateFeatureId {}

    pub enum QueryFeaturePredicate<'id> {
        Boolean,
        Plain(NodeId<'id, MediaFeatureValue<'id>>),
        Range {
            operator: MediaFeatureComparison,
            value: NodeId<'id, MediaFeatureValue<'id>>,
        },
        Interval {
            start: NodeId<'id, MediaFeatureValue<'id>>,
            start_operator: MediaFeatureComparison,
            end: NodeId<'id, MediaFeatureValue<'id>>,
            end_operator: MediaFeatureComparison,
        },
    }

    pub struct QueryFeatureRead<'context, 'storage, 'id, F> {
        context: &'context AstContext<'storage>,
        slot: QueryFeatureSlot<'id, F>,
    }

    impl<'id, F: QueryFeatureId> QueryFeatureRead<'_, '_, 'id, F> {
        pub fn name(&self) -> MediaFeatureName<'id, F> {
            use QueryFeatureSlot as S;
            match self.slot {
                S::StandardPlain { name, .. }
                | S::StandardBoolean { name }
                | S::StandardRange { name, .. }
                | S::StandardInterval { name, .. } => MediaFeatureName::Standard(name),
                S::CustomPlain { name, .. }
                | S::CustomBoolean { name }
                | S::CustomRange { name, .. } => MediaFeatureName::Custom(name),
                S::UnknownPlain { name, .. }
                | S::UnknownBoolean { name }
                | S::UnknownRange { name, .. } => MediaFeatureName::Unknown(name),
                S::CustomInterval { extra, .. } => {
                    // SAFETY: this interval variant owns one AstStr overflow slot.
                    MediaFeatureName::Custom(unsafe {
                        self.context.extra_slot(extra as usize).read_value()
                    })
                }
                S::UnknownInterval { extra, .. } => {
                    // SAFETY: this interval variant owns one AstStr overflow slot.
                    MediaFeatureName::Unknown(unsafe {
                        self.context.extra_slot(extra as usize).read_value()
                    })
                }
            }
        }

        pub fn predicate(&self) -> QueryFeaturePredicate<'id> {
            use QueryFeatureSlot as S;
            match self.slot {
                S::StandardBoolean { .. } | S::CustomBoolean { .. } | S::UnknownBoolean { .. } => {
                    QueryFeaturePredicate::Boolean
                }
                S::StandardPlain { value, .. }
                | S::CustomPlain { value, .. }
                | S::UnknownPlain { value, .. } => QueryFeaturePredicate::Plain(value),
                S::StandardRange {
                    operator, value, ..
                }
                | S::CustomRange {
                    operator, value, ..
                }
                | S::UnknownRange {
                    operator, value, ..
                } => QueryFeaturePredicate::Range { operator, value },
                S::StandardInterval {
                    start,
                    start_operator,
                    end,
                    end_operator,
                    ..
                }
                | S::CustomInterval {
                    start,
                    start_operator,
                    end,
                    end_operator,
                    ..
                }
                | S::UnknownInterval {
                    start,
                    start_operator,
                    end,
                    end_operator,
                    ..
                } => QueryFeaturePredicate::Interval {
                    start,
                    start_operator,
                    end,
                    end_operator,
                },
            }
        }
    }

    impl<'storage> AstContext<'storage> {
        pub fn query_feature<'id, F: QueryFeatureId>(
            &self,
            id: NodeId<'id, QueryFeature<'id, F>>,
        ) -> QueryFeatureRead<'_, 'storage, 'id, F> {
            // SAFETY: the sealed feature ID selects the checked kind and native slot type.
            QueryFeatureRead {
                context: self,
                slot: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: each FeatureId owns a distinct KIND with its native slot type;
// the only overflow is an AstStr written and read as AstStr.
unsafe impl<'ast, F: QueryFeatureId> AstNodeStorage<'ast> for QueryFeature<'ast, F> {
    const KIND: NodeKind = F::KIND;
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        let mut other = *other;
        let name = match self {
            Self::Plain { name, .. }
            | Self::Boolean { name }
            | Self::Range { name, .. }
            | Self::Interval { name, .. } => name,
        };
        let other_name = match &mut other {
            Self::Plain { name, .. }
            | Self::Boolean { name }
            | Self::Range { name, .. }
            | Self::Interval { name, .. } => name,
        };
        match (*name, *other_name) {
            (MediaFeatureName::Custom(a), MediaFeatureName::Custom(b))
            | (MediaFeatureName::Unknown(a), MediaFeatureName::Unknown(b))
                if context.str(a) == context.str(b) =>
            {
                *other_name = *name
            }
            _ => {}
        }
        *self == other
    }
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        use QueryFeatureSlot as S;
        match unsafe { payload.read_value::<S<'ast, F>>() } {
            S::StandardPlain { name, value } => Self::Plain {
                name: MediaFeatureName::Standard(name),
                value,
            },
            S::StandardBoolean { name } => Self::Boolean {
                name: MediaFeatureName::Standard(name),
            },
            S::StandardRange {
                operator,
                name,
                value,
            } => Self::Range {
                name: MediaFeatureName::Standard(name),
                operator,
                value,
            },
            S::StandardInterval {
                start_operator,
                end_operator,
                name,
                start,
                end,
            } => Self::Interval {
                name: MediaFeatureName::Standard(name),
                start_operator,
                end_operator,
                start,
                end,
            },
            S::CustomPlain { name, value } => Self::Plain {
                name: MediaFeatureName::Custom(name),
                value,
            },
            S::CustomBoolean { name } => Self::Boolean {
                name: MediaFeatureName::Custom(name),
            },
            S::CustomRange {
                operator,
                name,
                value,
            } => Self::Range {
                name: MediaFeatureName::Custom(name),
                operator,
                value,
            },
            S::CustomInterval {
                start_operator,
                end_operator,
                extra,
                start,
                end,
            } => Self::Interval {
                name: MediaFeatureName::Custom(unsafe {
                    context.extra_slot(extra as usize).read_value()
                }),
                start_operator,
                end_operator,
                start,
                end,
            },
            S::UnknownPlain { name, value } => Self::Plain {
                name: MediaFeatureName::Unknown(name),
                value,
            },
            S::UnknownBoolean { name } => Self::Boolean {
                name: MediaFeatureName::Unknown(name),
            },
            S::UnknownRange {
                operator,
                name,
                value,
            } => Self::Range {
                name: MediaFeatureName::Unknown(name),
                operator,
                value,
            },
            S::UnknownInterval {
                start_operator,
                end_operator,
                extra,
                start,
                end,
            } => Self::Interval {
                name: MediaFeatureName::Unknown(unsafe {
                    context.extra_slot(extra as usize).read_value()
                }),
                start_operator,
                end_operator,
                start,
                end,
            },
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        query_payload(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        query_payload(self, Some(unsafe { current.read_value() }), context)
    }
}
fn query_payload<'ast, F: QueryFeatureId>(
    value: QueryFeature<'ast, F>,
    current: Option<QueryFeatureSlot<'ast, F>>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    use QueryFeatureSlot as S;
    NodePayload::from_value(match value {
        QueryFeature::Plain {
            name: MediaFeatureName::Standard(name),
            value,
        } => S::StandardPlain { name, value },
        QueryFeature::Boolean {
            name: MediaFeatureName::Standard(name),
        } => S::StandardBoolean { name },
        QueryFeature::Range {
            name: MediaFeatureName::Standard(name),
            operator,
            value,
        } => S::StandardRange {
            name,
            operator,
            value,
        },
        QueryFeature::Interval {
            name: MediaFeatureName::Standard(name),
            start_operator,
            end_operator,
            start,
            end,
        } => S::StandardInterval {
            name,
            start_operator,
            end_operator,
            start,
            end,
        },
        QueryFeature::Plain {
            name: MediaFeatureName::Custom(name),
            value,
        } => S::CustomPlain { name, value },
        QueryFeature::Boolean {
            name: MediaFeatureName::Custom(name),
        } => S::CustomBoolean { name },
        QueryFeature::Range {
            name: MediaFeatureName::Custom(name),
            operator,
            value,
        } => S::CustomRange {
            name,
            operator,
            value,
        },
        QueryFeature::Interval {
            name: MediaFeatureName::Custom(name),
            start_operator,
            end_operator,
            start,
            end,
        } => S::CustomInterval {
            extra: match current {
                Some(S::CustomInterval { extra, .. }) => {
                    context.set_extra_slot(extra as usize, ExtraData::from_value(name));
                    extra
                }
                _ => u32::try_from(context.alloc_extra_slots([ExtraData::from_value(name)]))
                    .expect("query overflow index exceeds u32"),
            },
            start_operator,
            end_operator,
            start,
            end,
        },
        QueryFeature::Plain {
            name: MediaFeatureName::Unknown(name),
            value,
        } => S::UnknownPlain { name, value },
        QueryFeature::Boolean {
            name: MediaFeatureName::Unknown(name),
        } => S::UnknownBoolean { name },
        QueryFeature::Range {
            name: MediaFeatureName::Unknown(name),
            operator,
            value,
        } => S::UnknownRange {
            name,
            operator,
            value,
        },
        QueryFeature::Interval {
            name: MediaFeatureName::Unknown(name),
            start_operator,
            end_operator,
            start,
            end,
        } => S::UnknownInterval {
            extra: match current {
                Some(S::UnknownInterval { extra, .. }) => {
                    context.set_extra_slot(extra as usize, ExtraData::from_value(name));
                    extra
                }
                _ => u32::try_from(context.alloc_extra_slots([ExtraData::from_value(name)]))
                    .expect("query overflow index exceeds u32"),
            },
            start_operator,
            end_operator,
            start,
            end,
        },
    })
}

impl<'ast, FeatureId: QueryFeatureId> AstNodeClone<'ast> for QueryFeature<'ast, FeatureId> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Plain { name, value } => Self::Plain {
                name,
                value: context.clone_encoded_node(value),
            },
            Self::Boolean { name } => Self::Boolean { name },
            Self::Range {
                name,
                operator,
                value,
            } => Self::Range {
                name,
                operator,
                value: context.clone_encoded_node(value),
            },
            Self::Interval {
                end,
                end_operator,
                name,
                start,
                start_operator,
            } => Self::Interval {
                end: context.clone_encoded_node(end),
                end_operator,
                name,
                start: context.clone_encoded_node(start),
                start_operator,
            },
        }
    }
}

// SAFETY: this KIND always stores and reads the same native Copy type.
unsafe impl<'ast> AstNodeStorage<'ast> for MediaFeatureValue<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_0003);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Ident(a), Self::Ident(b)) => context.str(*a) == context.str(*b),
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
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast> AstNodeClone<'ast> for MediaFeatureValue<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            Self::Env(value) => Self::Env(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

// SAFETY: this KIND always stores and reads the same native Copy type.
unsafe impl<'ast> AstNodeStorage<'ast> for SupportsCondition<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_0004);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Selector(a), Self::Selector(b)) | (Self::Unknown(a), Self::Unknown(b)) => {
                context.str(*a) == context.str(*b)
            }
            (
                Self::Declaration {
                    property_id: a,
                    value: av,
                },
                Self::Declaration {
                    property_id: b,
                    value: bv,
                },
            ) => a == b && context.str(*av) == context.str(*bv),
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
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast> AstNodeClone<'ast> for SupportsCondition<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Not(value) => Self::Not(context.clone_encoded_node(value)),
            Self::And(values) => Self::And(context.clone_encoded_vec(values)),
            Self::Or(values) => Self::Or(context.clone_encoded_vec(values)),
            Self::Declaration { property_id, value } => Self::Declaration {
                property_id: context.clone_encoded_node(property_id),
                value,
            },
            Self::Selector(value) => Self::Selector(value),
            Self::Unknown(value) => Self::Unknown(value),
        }
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{AstContext, DUMMY_SP, Length, LengthUnit, LengthValue};

    use super::*;

    #[test]
    fn query_feature_names_compare_contents_across_all_storage_forms() {
        fn check<'ast, F: QueryFeatureId + std::fmt::Debug + 'ast>(
            context: &mut AstContext<'ast>,
            strings: [AstStr<'ast>; 4],
        ) {
            let start = context.alloc_node(MediaFeatureValue::Integer(1), DUMMY_SP);
            let end = context.alloc_node(MediaFeatureValue::Integer(2), DUMMY_SP);
            for variant in 0..4 {
                let make = |name| match variant {
                    0 => QueryFeature::<F>::Boolean { name },
                    1 => QueryFeature::Plain { name, value: start },
                    2 => QueryFeature::Range {
                        name,
                        value: start,
                        operator: MediaFeatureComparison::GreaterThan,
                    },
                    _ => QueryFeature::Interval {
                        name,
                        start,
                        end,
                        start_operator: MediaFeatureComparison::LessThan,
                        end_operator: MediaFeatureComparison::LessThanEqual,
                    },
                };
                let custom = strings
                    .map(|text| context.alloc_node(make(MediaFeatureName::Custom(text)), DUMMY_SP));
                let unknown = strings.map(|text| {
                    context.alloc_node(make(MediaFeatureName::Unknown(text)), DUMMY_SP)
                });
                let checkpoint = context.node_checkpoint();
                let bytes = context.string_pool().extra_len();
                let interned = context.string_pool().len();
                for _ in 0..3 {
                    for ids in [custom, unknown] {
                        assert!(context.nodes_eq(ids[0], ids[1]));
                        assert!(context.nodes_eq(ids[0], ids[2]));
                        assert!(!context.nodes_eq(ids[0], ids[3]));
                    }
                    assert!(!context.nodes_eq(custom[0], unknown[1]));
                }
                assert_eq!(context.node_checkpoint(), checkpoint);
                assert_eq!(context.string_pool().extra_len(), bytes);
                assert_eq!(context.string_pool().len(), interned);
                if variant == 3 {
                    context.mutate_node(custom[1], |node, _| {
                        let QueryFeature::Interval { end_operator, .. } = node else {
                            unreachable!()
                        };
                        *end_operator = MediaFeatureComparison::LessThan;
                    });
                    assert!(!context.nodes_eq(custom[0], custom[1]));
                }
            }
        }

        let allocator = Allocator::new();
        let mut context = AstContext::with_source_in(&allocator, "name name", Default::default());
        let first = context.string_pool().source_range(0, 4);
        let second = context.string_pool().source_range(5, 9);
        let extra = context.add_str("name");
        let other = context.add_str("other");
        context.add_str(&"é".repeat(8192));
        let strings = [first, second, extra, other];
        check::<MediaFeatureId>(&mut context, strings);
        check::<crate::ContainerSizeFeatureId>(&mut context, strings);
        check::<crate::ScrollStateFeatureId>(&mut context, strings);
    }

    #[test]
    fn query_slots_preserve_names_predicates_and_reuse_interval_overflow() {
        assert_eq!(
            std::mem::size_of::<QueryFeatureSlot<'_, MediaFeatureId>>(),
            16
        );
        assert_eq!(
            std::mem::size_of::<QueryFeatureSlot<'_, ContainerSizeFeatureId>>(),
            16
        );
        assert_eq!(
            std::mem::size_of::<QueryFeatureSlot<'_, ScrollStateFeatureId>>(),
            16
        );
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("--feature");
        let duplicate = context.add_str("--feature");
        let start = context.alloc_encoded_node(MediaFeatureValue::Integer(1), DUMMY_SP);
        let end = context.alloc_encoded_node(MediaFeatureValue::Integer(3), DUMMY_SP);
        for name in [
            MediaFeatureName::Standard(MediaFeatureId::Width),
            MediaFeatureName::Custom(text),
            MediaFeatureName::Unknown(text),
        ] {
            for expected in [
                QueryFeature::Boolean { name },
                QueryFeature::Plain { name, value: start },
                QueryFeature::Range {
                    name,
                    operator: MediaFeatureComparison::GreaterThan,
                    value: start,
                },
                QueryFeature::Interval {
                    name,
                    start,
                    end,
                    start_operator: MediaFeatureComparison::LessThan,
                    end_operator: MediaFeatureComparison::LessThanEqual,
                },
            ] {
                let before = context.encoded_extra_len();
                let node = context.alloc_encoded_node(expected, DUMMY_SP);
                let extra = usize::from(matches!(
                    expected,
                    QueryFeature::Interval {
                        name: MediaFeatureName::Custom(_) | MediaFeatureName::Unknown(_),
                        ..
                    }
                ));
                assert_eq!(context.encoded_extra_len(), before + extra);
                let checkpoint = context.node_checkpoint();
                let bytes = context.string_pool().extra_len();
                for _ in 0..4 {
                    assert_eq!(context.encoded_node(node), expected);
                    context.mutate_encoded_node(node, |stored, _| *stored = expected);
                }
                assert_eq!(context.node_checkpoint(), checkpoint);
                assert_eq!(context.string_pool().extra_len(), bytes);
            }
        }
        let left = context.alloc_encoded_node(
            QueryFeature::<MediaFeatureId>::Boolean {
                name: MediaFeatureName::Custom(text),
            },
            DUMMY_SP,
        );
        let right = context.alloc_encoded_node(
            QueryFeature::<MediaFeatureId>::Boolean {
                name: MediaFeatureName::Custom(duplicate),
            },
            DUMMY_SP,
        );
        let unknown = context.alloc_encoded_node(
            QueryFeature::<MediaFeatureId>::Boolean {
                name: MediaFeatureName::Unknown(duplicate),
            },
            DUMMY_SP,
        );
        assert!(context.nodes_eq(left, right));
        assert!(!context.nodes_eq(left, unknown));
    }

    #[test]
    fn native_media_strings_compare_contents_and_reuse_storage() {
        assert!(std::mem::size_of::<MediaCondition<'_>>() <= 16);
        assert!(std::mem::size_of::<MediaFeatureValue<'_>>() <= 16);
        assert!(std::mem::size_of::<SupportsCondition<'_>>() <= 16);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.add_str("λ-value");
        let second = context.add_str("λ-value");
        assert_ne!(first, second);
        let left = context.alloc_encoded_node(MediaFeatureValue::Ident(first), DUMMY_SP);
        let right = context.alloc_encoded_node(MediaFeatureValue::Ident(second), DUMMY_SP);
        assert!(context.nodes_eq(left, right));
        let a = context.alloc_encoded_node(SupportsCondition::Unknown(first), DUMMY_SP);
        let b = context.alloc_encoded_node(SupportsCondition::Unknown(second), DUMMY_SP);
        assert!(context.nodes_eq(a, b));
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for value in [first, second, AstStr::EMPTY, first] {
            context.mutate_encoded_node(a, |node, _| *node = SupportsCondition::Selector(value));
            assert_eq!(context.encoded_node(a), SupportsCondition::Selector(value));
            context.mutate_encoded_node(left, |node, _| *node = MediaFeatureValue::Ident(value));
            assert_eq!(context.encoded_node(left), MediaFeatureValue::Ident(value));
        }
        for denominator in [None, Some(-0.0), Some(2.0)] {
            context.mutate_encoded_node(left, |node, _| {
                *node = MediaFeatureValue::Ratio(Ratio {
                    numerator: -0.0,
                    denominator,
                });
            });
            let MediaFeatureValue::Ratio(ratio) = context.encoded_node(left) else {
                panic!("expected ratio")
            };
            assert_eq!(ratio.numerator.to_bits(), (-0.0f32).to_bits());
            assert_eq!(
                ratio.denominator.map(f32::to_bits),
                denominator.map(f32::to_bits)
            );
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }

    #[test]
    fn media_query_codec_deep_clones_condition_tree() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let length = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 640.0,
            }),
            DUMMY_SP,
        );
        let value = context.alloc_encoded_node(MediaFeatureValue::Length(length), DUMMY_SP);
        let feature = context.alloc_encoded_node(
            QueryFeature::Range {
                name: MediaFeatureName::Standard(MediaFeatureId::Width),
                operator: MediaFeatureComparison::GreaterThanEqual,
                value,
            },
            DUMMY_SP,
        );
        let child = context.alloc_encoded_node(MediaCondition::Feature(feature), DUMMY_SP);
        let conditions = context.alloc_encoded_vec([child].into_iter());
        let condition = context.alloc_encoded_node(
            MediaCondition::Operation {
                conditions,
                operator: Operator::And,
            },
            DUMMY_SP,
        );
        let query = context.alloc_encoded_node(
            crate::MediaQuery {
                condition: Some(condition),
                media_type: MediaType::Screen,
                qualifier: Some(Qualifier::Only),
            },
            DUMMY_SP,
        );

        let cloned = context.clone_encoded_node(query);
        let cloned_condition = context
            .encoded_node(cloned)
            .condition
            .expect("cloned media condition");
        assert_ne!(cloned_condition, condition);
        let MediaCondition::Operation {
            conditions: cloned_conditions,
            operator: Operator::And,
        } = context.encoded_node(cloned_condition)
        else {
            panic!("expected cloned media operation")
        };
        assert_ne!(cloned_conditions, conditions);
        assert_ne!(context.encoded_vec_get(cloned_conditions, 0), Some(child));
    }

    #[test]
    fn supports_condition_codec_clones_node_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("(display:grid)");
        let child = context.alloc_encoded_node(SupportsCondition::Unknown(text), DUMMY_SP);
        let values = context.alloc_encoded_vec([child].into_iter());
        let condition = context.alloc_encoded_node(SupportsCondition::And(values), DUMMY_SP);
        let cloned = context.clone_encoded_node(condition);
        let SupportsCondition::And(cloned_values) = context.encoded_node(cloned) else {
            panic!("expected cloned supports operation")
        };
        assert_ne!(cloned_values, values);
        assert_ne!(context.encoded_vec_get(cloned_values, 0), Some(child));
    }
}
