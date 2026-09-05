use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum TrackSizing<'a> {
    None,
    TrackList {
        items: Vec<'a, NodeId<'a, TrackListItem<'a>>>,
        line_names: Vec<'a, Vec<'a, &'a str>>,
    },
}

impl<'ast> AstNodeStorage<'ast> for TrackSizing<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0023_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::TrackList {
                items: decode_range(context.extra_slot(payload.extra_start()), context),
                line_names: decode_range(context.extra_slot(payload.extra_start() + 1), context),
            },
            _ => panic!("invalid encoded TrackSizing variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_track_sizing(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_track_sizing(self, Some(current.extra_start()), context)
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
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    let slots = match value {
        TrackSizing::None => [ExtraData::from_u64(0), ExtraData::from_u64(0)],
        TrackSizing::TrackList { items, line_names } => {
            bytes[0] = 1;
            [encode_range(items), encode_range(line_names)]
        }
    };
    let extra_start = match existing_extra {
        Some(extra_start) => {
            for (offset, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(extra_start + offset, slot);
            }
            extra_start
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::with_extra(&bytes, extra_start)
}

#[derive(Debug, PartialEq, Visit)]
pub enum TrackListItem<'a> {
    TrackSize(NodeId<'a, TrackSize<'a>>),
    TrackRepeat(NodeId<'a, TrackRepeat<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for TrackListItem<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0023_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::TrackSize(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            1 => Self::TrackRepeat(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded TrackListItem variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        let value = match self {
            Self::TrackSize(value) => value.index(),
            Self::TrackRepeat(value) => {
                bytes[0] = 1;
                value.index()
            }
        };
        write_u32(&mut bytes, 4, value);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for TrackListItem<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::TrackSize(value) => Self::TrackSize(context.clone_encoded_node(value)),
            Self::TrackRepeat(value) => Self::TrackRepeat(context.clone_encoded_node(value)),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum TrackSize<'a> {
    TrackBreadth(NodeId<'a, TrackBreadth<'a>>),
    MinMax {
        max: NodeId<'a, TrackBreadth<'a>>,
        min: NodeId<'a, TrackBreadth<'a>>,
    },
    FitContent(NodeId<'a, LengthPercentage<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for TrackSize<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0023_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::TrackBreadth(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            1 => Self::MinMax {
                max: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
                min: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            },
            2 => Self::FitContent(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded TrackSize variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::TrackBreadth(value) => write_u32(&mut bytes, 4, value.index()),
            Self::MinMax { max, min } => {
                bytes[0] = 1;
                write_u32(&mut bytes, 4, max.index());
                write_u32(&mut bytes, 8, min.index());
            }
            Self::FitContent(value) => {
                bytes[0] = 2;
                write_u32(&mut bytes, 4, value.index());
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

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

#[derive(Debug, PartialEq, Visit)]
pub enum TrackBreadth<'a> {
    Length(NodeId<'a, LengthPercentage<'a>>),
    Flex(f32),
    MinContent,
    MaxContent,
    Auto,
}

impl<'ast> AstNodeStorage<'ast> for TrackBreadth<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0023_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Length(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            1 => Self::Flex(f32::from_bits(read_u32(&bytes, 4))),
            2 => Self::MinContent,
            3 => Self::MaxContent,
            4 => Self::Auto,
            _ => panic!("invalid encoded TrackBreadth variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Length(value) => write_u32(&mut bytes, 4, value.index()),
            Self::Flex(value) => {
                bytes[0] = 1;
                write_u32(&mut bytes, 4, value.to_bits());
            }
            Self::MinContent => bytes[0] = 2,
            Self::MaxContent => bytes[0] = 3,
            Self::Auto => bytes[0] = 4,
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for TrackBreadth<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum RepeatCount {
    Number(f32),
    AutoFill,
    AutoFit,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AutoFlowDirection {
    Row,
    Column,
}

#[derive(Debug, PartialEq, Visit)]
pub enum GridTemplateAreas<'a> {
    None,
    Areas {
        areas: Vec<'a, Option<&'a str>>,
        columns: u32,
    },
}

impl<'ast> AstNodeStorage<'ast> for GridTemplateAreas<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0023_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Areas {
                areas: context
                    .encoded_vec_range(read_u32(&bytes, 8) as usize, read_u32(&bytes, 12) as usize),
                columns: read_u32(&bytes, 4),
            },
            _ => panic!("invalid encoded GridTemplateAreas variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        if let Self::Areas { areas, columns } = self {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, columns);
            write_u32(&mut bytes, 8, areas.start_index());
            write_u32(&mut bytes, 12, areas.end_index());
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

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

#[derive(Debug, PartialEq, Visit)]
pub enum GridLine<'a> {
    Auto,
    Area { name: &'a str },
    Line { index: i32, name: Option<&'a str> },
    Span { index: i32, name: Option<&'a str> },
}

impl<'ast> AstNodeStorage<'ast> for GridLine<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0023_0006);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let name = || {
            let value = read_u32(&bytes, 8);
            (value != u32::MAX).then(|| context.resolve_string(value as u64))
        };
        match bytes[0] {
            0 => Self::Auto,
            1 => Self::Area {
                name: context.resolve_string(read_u32(&bytes, 8) as u64),
            },
            2 => Self::Line {
                index: read_u32(&bytes, 4) as i32,
                name: name(),
            },
            3 => Self::Span {
                index: read_u32(&bytes, 4) as i32,
                name: name(),
            },
            _ => panic!("invalid encoded GridLine variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Auto => {}
            Self::Area { name } => {
                bytes[0] = 1;
                write_u32(&mut bytes, 8, context.store_string(name));
            }
            Self::Line { index, name } => {
                bytes[0] = 2;
                write_u32(&mut bytes, 4, index as u32);
                write_optional_string(&mut bytes, 8, name, context);
            }
            Self::Span { index, name } => {
                bytes[0] = 3;
                write_u32(&mut bytes, 4, index as u32);
                write_optional_string(&mut bytes, 8, name, context);
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for GridLine<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

fn encode_range<T>(range: Vec<'_, T>) -> ExtraData {
    let start = u32::try_from(range.start_index()).expect("AST range start exceeds four bytes");
    let end = u32::try_from(range.end_index()).expect("AST range end exceeds four bytes");
    ExtraData::from_u64((end as u64) << 32 | start as u64)
}

fn decode_range<'ast, T>(data: ExtraData, context: &AstContext<'ast>) -> Vec<'ast, T> {
    context.encoded_vec_range(
        data.as_u64() as u32 as usize,
        (data.as_u64() >> 32) as usize,
    )
}

fn write_optional_string<'ast>(
    bytes: &mut [u8],
    offset: usize,
    value: Option<&'ast str>,
    context: &mut AstContext<'ast>,
) {
    write_u32(
        bytes,
        offset,
        value.map_or(u32::MAX, |value| context.store_string(value)),
    );
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn write_u32(bytes: &mut [u8], offset: usize, value: impl TryInto<u32>) {
    bytes[offset..offset + 4].copy_from_slice(
        &value
            .try_into()
            .unwrap_or_else(|_| panic!("AST compact value exceeds four bytes"))
            .to_le_bytes(),
    );
}
