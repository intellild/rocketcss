use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum TrackSizing<'a> {
    None,
    TrackList {
        items: Vec<'a, NodeId<'a, TrackListItem<'a>>>,
        line_names: Vec<'a, Vec<'a, AstStr<'a>>>,
    },
}

#[derive(Clone, Copy)]
enum TrackSizingSlot<'a> {
    // Retain an allocated line-name slot across temporary removal of the list.
    None {
        line_names_extra: Option<u32>,
    },
    TrackList {
        items: Vec<'a, NodeId<'a, TrackListItem<'a>>>,
        line_names_extra: u32,
    },
}

pub use track_sizing_access::TrackListRead;

// A transient view; None represents the inline TrackSizing::None variant.
mod track_sizing_access {
    use super::*;
    pub struct TrackListRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        items: Vec<'id, NodeId<'id, TrackListItem<'id>>>,
        line_names_extra: u32,
    }
    impl<'id> TrackListRead<'_, '_, 'id> {
        pub fn items(&self) -> Vec<'id, NodeId<'id, TrackListItem<'id>>> {
            self.items
        }
        pub fn line_names(&self) -> Vec<'id, Vec<'id, AstStr<'id>>> {
            // SAFETY: TrackList owns a native nested line-name range in its extra slot.
            unsafe {
                self.context
                    .extra_slot(self.line_names_extra as usize)
                    .read_value()
            }
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn track_sizing<'id>(
            &self,
            id: NodeId<'id, TrackSizing<'id>>,
        ) -> Option<TrackListRead<'_, 'storage, 'id>> {
            // SAFETY: node_payload validates the TrackSizing kind before the native read.
            match unsafe { self.node_payload(id).read_value::<TrackSizingSlot<'id>>() } {
                TrackSizingSlot::None { .. } => None,
                TrackSizingSlot::TrackList {
                    items,
                    line_names_extra,
                } => Some(TrackListRead {
                    context: self,
                    items,
                    line_names_extra,
                }),
            }
        }
    }
}

// SAFETY: this kind stores TrackSizingSlot and its overflow stores a line-name list range.
unsafe impl<'ast> AstNodeStorage<'ast> for TrackSizing<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0023_0001);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        match unsafe { payload.read_value::<TrackSizingSlot<'ast>>() } {
            TrackSizingSlot::None { .. } => Self::None,
            TrackSizingSlot::TrackList {
                items,
                line_names_extra,
            } => Self::TrackList {
                items,
                line_names: unsafe { context.extra_slot(line_names_extra as usize).read_value() },
            },
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_track_sizing(self, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let extra = match unsafe { current.read_value::<TrackSizingSlot<'ast>>() } {
            TrackSizingSlot::None { line_names_extra } => line_names_extra,
            TrackSizingSlot::TrackList {
                line_names_extra, ..
            } => Some(line_names_extra),
        };
        encode_track_sizing(self, extra, context)
    }
}

impl<'ast> AstNodeClone<'ast> for TrackSizing<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::TrackList { items, line_names } => Self::TrackList {
                items: context.clone_encoded_vec(items),
                line_names: context.clone_encoded_vec(line_names),
            },
        }
    }
}

fn encode_track_sizing<'ast>(
    value: TrackSizing<'ast>,
    existing_extra: Option<u32>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let value = match value {
        TrackSizing::None => TrackSizingSlot::None {
            line_names_extra: existing_extra,
        },
        TrackSizing::TrackList { items, line_names } => {
            let slot = ExtraData::from_value(line_names);
            let extra = match existing_extra {
                Some(extra) => {
                    context.set_extra_slot(extra as usize, slot);
                    extra as usize
                }
                None => context.alloc_extra_slots([slot]),
            };
            TrackSizingSlot::TrackList {
                items,
                line_names_extra: u32::try_from(extra).expect("AST extra index exceeds u32"),
            }
        }
    };
    NodePayload::from_value(value)
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum TrackListItem<'a> {
    TrackSize(NodeId<'a, TrackSize<'a>>),
    TrackRepeat(NodeId<'a, TrackRepeat<'a>>),
}

impl_inline_node!(TrackListItem<'ast>, 0x0023_0002);

impl<'ast> AstNodeClone<'ast> for TrackListItem<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::TrackSize(value) => Self::TrackSize(context.clone_encoded_node(value)),
            Self::TrackRepeat(value) => Self::TrackRepeat(context.clone_encoded_node(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum TrackSize<'a> {
    TrackBreadth(NodeId<'a, TrackBreadth<'a>>),
    MinMax {
        max: NodeId<'a, TrackBreadth<'a>>,
        min: NodeId<'a, TrackBreadth<'a>>,
    },
    FitContent(NodeId<'a, LengthPercentage<'a>>),
}

impl_inline_node!(TrackSize<'ast>, 0x0023_0003);

impl<'ast> AstNodeClone<'ast> for TrackSize<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::TrackBreadth(value) => Self::TrackBreadth(context.clone_encoded_node(value)),
            Self::MinMax { max, min } => Self::MinMax {
                max: context.clone_encoded_node(max),
                min: context.clone_encoded_node(min),
            },
            Self::FitContent(value) => Self::FitContent(context.clone_encoded_node(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum TrackBreadth<'a> {
    Length(NodeId<'a, LengthPercentage<'a>>),
    Flex(f32),
    MinContent,
    MaxContent,
    Auto,
}

impl_inline_node!(TrackBreadth<'ast>, 0x0023_0004);

impl<'ast> AstNodeClone<'ast> for TrackBreadth<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub enum RepeatCount {
    Number(f32),
    AutoFill,
    AutoFit,
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum AutoFlowDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum GridTemplateAreas<'a> {
    None,
    Areas {
        areas: Vec<'a, Option<AstStr<'a>>>,
        columns: u32,
    },
}

impl_inline_node!(GridTemplateAreas<'ast>, 0x0023_0005);

impl<'ast> AstNodeClone<'ast> for GridTemplateAreas<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Areas { areas, columns } => Self::Areas {
                areas: context.clone_encoded_vec(areas),
                columns,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum GridLine<'a> {
    Auto,
    Area {
        name: AstStr<'a>,
    },
    Line {
        index: i32,
        name: Option<AstStr<'a>>,
    },
    Span {
        index: i32,
        name: Option<AstStr<'a>>,
    },
}

// Flatten optional-name variants so the full value fits a native payload.
#[derive(Clone, Copy)]
enum GridLineSlot<'a> {
    Auto,
    Area { name: AstStr<'a> },
    Line { index: i32 },
    NamedLine { index: i32, name: AstStr<'a> },
    Span { index: i32 },
    NamedSpan { index: i32, name: AstStr<'a> },
}

// SAFETY: this KIND always writes and reads GridLineSlot.
unsafe impl<'ast> AstNodeStorage<'ast> for GridLine<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0023_0006);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Area { name: a }, Self::Area { name: b }) => context.str(*a) == context.str(*b),
            (Self::Line { index: a, name: an }, Self::Line { index: b, name: bn })
            | (Self::Span { index: a, name: an }, Self::Span { index: b, name: bn }) => {
                a == b && an.map(|name| context.str(name)) == bn.map(|name| context.str(name))
            }
            _ => self == other,
        }
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        match unsafe { payload.read_value::<GridLineSlot<'ast>>() } {
            GridLineSlot::Auto => Self::Auto,
            GridLineSlot::Area { name } => Self::Area { name },
            GridLineSlot::Line { index } => Self::Line { index, name: None },
            GridLineSlot::NamedLine { index, name } => Self::Line {
                index,
                name: Some(name),
            },
            GridLineSlot::Span { index } => Self::Span { index, name: None },
            GridLineSlot::NamedSpan { index, name } => Self::Span {
                index,
                name: Some(name),
            },
        }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        self.into_payload()
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.into_payload()
    }
}
impl GridLine<'_> {
    fn into_payload(self) -> NodePayload {
        NodePayload::from_value(match self {
            Self::Auto => GridLineSlot::Auto,
            Self::Area { name } => GridLineSlot::Area { name },
            Self::Line { index, name: None } => GridLineSlot::Line { index },
            Self::Line {
                index,
                name: Some(name),
            } => GridLineSlot::NamedLine { index, name },
            Self::Span { index, name: None } => GridLineSlot::Span { index },
            Self::Span {
                index,
                name: Some(name),
            } => GridLineSlot::NamedSpan { index, name },
        })
    }
}

impl<'ast> AstNodeClone<'ast> for GridLine<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;
    use rocketcss_common::Allocator;

    #[test]
    fn track_sizing_reuses_native_overflow_and_clones_nested_ranges() {
        assert_eq!(std::mem::size_of::<TrackSizingSlot<'_>>(), 16);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.add_str("区域");
        let second = context.add_str("区域");
        assert_ne!(first, second);
        let names = context.alloc_encoded_vec([first, second, AstStr::EMPTY].into_iter());
        let line_names = context.alloc_encoded_vec([names].into_iter());
        let items = context.alloc_encoded_vec(std::iter::empty());
        let before = context.encoded_extra_len();
        let node = context.alloc_encoded_node(TrackSizing::None, DUMMY_SP);
        assert_eq!(context.encoded_extra_len(), before);
        context.mutate_encoded_node(node, |value, _| {
            *value = TrackSizing::TrackList { items, line_names }
        });
        assert_eq!(context.encoded_extra_len(), before + 1);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for _ in 0..3 {
            context.mutate_encoded_node(node, |value, _| *value = TrackSizing::None);
            assert_eq!(context.encoded_node(node), TrackSizing::None);
            assert!(context.track_sizing(node).is_none());
            assert_eq!(context.node_checkpoint(), checkpoint);
            context.mutate_encoded_node(node, |value, _| {
                *value = TrackSizing::TrackList { items, line_names };
            });
            let view = context.track_sizing(node).unwrap();
            assert_eq!(view.items(), items);
            assert_eq!(view.line_names(), line_names);
            assert_eq!(context.node_checkpoint(), checkpoint);
            context.mutate_encoded_node(node, |_, _| {});
            let TrackSizing::TrackList {
                line_names: stored, ..
            } = context.encoded_node(node)
            else {
                panic!()
            };
            assert_eq!(context.vec_get(stored, 0), Some(names));
            assert_eq!(context.vec_get(names, 1), Some(second));
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        let cloned = context.clone_encoded_node(node);
        let TrackSizing::TrackList {
            line_names: cloned_names,
            ..
        } = context.encoded_node(cloned)
        else {
            panic!()
        };
        assert_ne!(cloned_names, line_names);
        let cloned_names = context.vec_get(cloned_names, 0).unwrap();
        assert_ne!(cloned_names, names);
        assert_eq!(context.vec_get(cloned_names, 0), Some(first));
        assert_eq!(context.vec_get(cloned_names, 1), Some(second));
        assert_eq!(context.string_pool().extra_len(), bytes);
    }

    #[test]
    fn grid_line_native_slots_preserve_optional_names_and_indices() {
        assert_eq!(std::mem::size_of::<GridLineSlot<'_>>(), 16);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.add_str("区域");
        let second = context.add_str("区域");
        let node = context.alloc_encoded_node(GridLine::Area { name: first }, DUMMY_SP);
        let equal = context.alloc_encoded_node(GridLine::Area { name: second }, DUMMY_SP);
        assert!(context.nodes_eq(node, equal));
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for index in [i32::MIN, -1, 0, 1, i32::MAX] {
            for name in [None, Some(AstStr::EMPTY), Some(first), Some(second)] {
                for value in [
                    GridLine::Line { index, name },
                    GridLine::Span { index, name },
                ] {
                    context.mutate_encoded_node(node, |stored, _| *stored = value);
                    assert_eq!(context.encoded_node(node), value);
                }
            }
        }
        context.mutate_encoded_node(node, |stored, _| *stored = GridLine::Auto);
        assert_eq!(context.encoded_node(node), GridLine::Auto);
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }
}
