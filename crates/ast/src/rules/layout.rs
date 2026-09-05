use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub struct AspectRatio {
    pub auto: bool,
    pub ratio: Option<Ratio>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Overflow {
    pub x: OverflowKeyword,
    pub y: OverflowKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct InsetBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for InsetBlock<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            block_end: read_node_id(&bytes, 0, context),
            block_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.block_end, self.block_start)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_two_ids(self.block_end, self.block_start)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct InsetInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for InsetInline<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            inline_end: read_node_id(&bytes, 0, context),
            inline_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.inline_end, self.inline_start)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_two_ids(self.inline_end, self.inline_start)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Inset<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for Inset<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom: read_node_id(&bytes, 0, context),
            left: read_node_id(&bytes, 4, context),
            right: read_node_id(&bytes, 8, context),
            top: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(self.bottom, self.left, self.right, self.top)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_four_ids(self.bottom, self.left, self.right, self.top)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct FlexFlow {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
}

impl AstNodeStorage<'_> for FlexFlow {
    const KIND: NodeKind = NodeKind::new(0x000a_0004);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        Self {
            direction: decode_flex_direction(bytes[0]),
            wrap: decode_flex_wrap(bytes[1]),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        encode_flex_flow(self)
    }

    fn encode_existing(self, _current: NodePayload, _context: &mut AstContext<'_>) -> NodePayload {
        encode_flex_flow(self)
    }
}

impl AstNodeClone<'_> for FlexFlow {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_flex_flow(value: FlexFlow) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    bytes[0] = encode_flex_direction(value.direction);
    bytes[1] = encode_flex_wrap(value.wrap);
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub struct Flex<'a> {
    pub basis: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub grow: f32,
    pub shrink: f32,
}

impl<'ast> AstNodeStorage<'ast> for Flex<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            basis: read_node_id(&bytes, 0, context),
            grow: f32::from_bits(read_u32(&bytes, 4)),
            shrink: f32::from_bits(read_u32(&bytes, 8)),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_flex(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_flex(self)
    }
}

fn encode_flex(value: Flex<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_node_id(&mut bytes, 0, value.basis);
    write_u32(&mut bytes, 4, value.grow.to_bits());
    write_u32(&mut bytes, 8, value.shrink.to_bits());
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceContent {
    pub align: AlignContent,
    pub justify: JustifyContent,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceSelf {
    pub align: AlignSelf,
    pub justify: JustifySelf,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PlaceItems {
    pub align: AlignItems,
    pub justify: JustifyItems,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Gap<'a> {
    pub column: NodeId<'a, GapValue<'a>>,
    pub row: NodeId<'a, GapValue<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for Gap<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0006);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            column: read_node_id(&bytes, 0, context),
            row: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.column, self.row)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_two_ids(self.column, self.row)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ColumnRule<'a> {
    pub color: Option<NodeId<'a, CssColor<'a>>>,
    pub style: Option<LineStyle>,
    pub width: Option<NodeId<'a, BorderSideWidth<'a>>>,
}

impl<'ast> AstNodeStorage<'ast> for ColumnRule<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0007);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            color: read_optional_node_id(&bytes, 0, context),
            style: (bytes[8] != u8::MAX).then(|| decode_line_style(bytes[8])),
            width: read_optional_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_column_rule(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_column_rule(self)
    }
}

fn encode_column_rule(value: ColumnRule<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_optional_node_id(&mut bytes, 0, value.color);
    write_optional_node_id(&mut bytes, 4, value.width);
    bytes[8] = value.style.map_or(u8::MAX, encode_line_style);
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum ColumnWidth<'a> {
    Auto,
    Length(NodeId<'a, Length<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum ColumnCount {
    Auto,
    Integer(i32),
}

#[derive(Debug, PartialEq, Visit)]
pub struct Columns<'a> {
    pub count: ColumnCount,
    pub width: ColumnWidth<'a>,
}

impl<'ast> AstNodeStorage<'ast> for Columns<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0008);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let count = match bytes[0] {
            0 => ColumnCount::Auto,
            1 => ColumnCount::Integer(read_u32(&bytes, 4) as i32),
            _ => panic!("invalid encoded ColumnCount variant"),
        };
        let width = match bytes[1] {
            0 => ColumnWidth::Auto,
            1 => ColumnWidth::Length(read_node_id(&bytes, 8, context)),
            _ => panic!("invalid encoded ColumnWidth variant"),
        };
        Self { count, width }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_columns(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_columns(self)
    }
}

fn encode_columns(value: Columns<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value.count {
        ColumnCount::Auto => bytes[0] = 0,
        ColumnCount::Integer(value) => {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, value as u32);
        }
    }
    match value.width {
        ColumnWidth::Auto => bytes[1] = 0,
        ColumnWidth::Length(value) => {
            bytes[1] = 1;
            write_node_id(&mut bytes, 8, value);
        }
    }
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub struct TrackRepeat<'a> {
    pub count: RepeatCount,
    pub line_names: Vec<'a, Vec<'a, &'a str>>,
    pub track_sizes: Vec<'a, NodeId<'a, TrackSize<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridAutoFlow {
    pub dense: bool,
    pub direction: AutoFlowDirection,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridTemplate<'a> {
    pub areas: NodeId<'a, GridTemplateAreas<'a>>,
    pub columns: NodeId<'a, TrackSizing<'a>>,
    pub rows: NodeId<'a, TrackSizing<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Grid<'a> {
    pub areas: NodeId<'a, GridTemplateAreas<'a>>,
    pub auto_columns: Vec<'a, NodeId<'a, TrackSize<'a>>>,
    pub auto_flow: GridAutoFlow,
    pub auto_rows: Vec<'a, NodeId<'a, TrackSize<'a>>>,
    pub columns: NodeId<'a, TrackSizing<'a>>,
    pub rows: NodeId<'a, TrackSizing<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridRow<'a> {
    pub end: NodeId<'a, GridLine<'a>>,
    pub start: NodeId<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridColumn<'a> {
    pub end: NodeId<'a, GridLine<'a>>,
    pub start: NodeId<'a, GridLine<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct GridArea<'a> {
    pub column_end: NodeId<'a, GridLine<'a>>,
    pub column_start: NodeId<'a, GridLine<'a>>,
    pub row_end: NodeId<'a, GridLine<'a>>,
    pub row_start: NodeId<'a, GridLine<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for TrackRepeat<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0015);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            count: match bytes[0] {
                0 => RepeatCount::Number(f32::from_bits(read_u32(&bytes, 4))),
                1 => RepeatCount::AutoFill,
                2 => RepeatCount::AutoFit,
                _ => panic!("invalid encoded RepeatCount variant"),
            },
            line_names: decode_range(context.extra_slot(payload.extra_start()), context),
            track_sizes: decode_range(context.extra_slot(payload.extra_start() + 1), context),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_track_repeat(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_track_repeat(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for TrackRepeat<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            count: self.count,
            line_names: context.clone_encoded_vec(self.line_names),
            track_sizes: context.clone_encoded_vec(self.track_sizes),
        }
    }
}

fn encode_track_repeat<'ast>(
    value: TrackRepeat<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    match value.count {
        RepeatCount::Number(value) => write_u32(&mut bytes, 4, value.to_bits()),
        RepeatCount::AutoFill => bytes[0] = 1,
        RepeatCount::AutoFit => bytes[0] = 2,
    }
    let slots = [
        encode_range(value.line_names),
        encode_range(value.track_sizes),
    ];
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

impl<'ast> AstNodeStorage<'ast> for GridTemplate<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0016);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            areas: read_node_id(&bytes, 0, context),
            columns: read_node_id(&bytes, 4, context),
            rows: read_node_id(&bytes, 8, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        write_node_id(&mut bytes, 0, self.areas);
        write_node_id(&mut bytes, 4, self.columns);
        write_node_id(&mut bytes, 8, self.rows);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for GridTemplate<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            areas: context.clone_encoded_node(self.areas),
            columns: context.clone_encoded_node(self.columns),
            rows: context.clone_encoded_node(self.rows),
        }
    }
}

impl<'ast> AstNodeStorage<'ast> for Grid<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0017);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            areas: read_node_id(&bytes, 4, context),
            auto_columns: decode_range(context.extra_slot(payload.extra_start() + 1), context),
            auto_flow: GridAutoFlow {
                dense: bytes[1] != 0,
                direction: decode_auto_flow_direction(bytes[0]),
            },
            auto_rows: decode_range(context.extra_slot(payload.extra_start() + 2), context),
            columns: read_node_id(&bytes, 8, context),
            rows: context.encoded_node_id_at(
                context.extra_slot(payload.extra_start()).as_u64() as u32 as usize
            ),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_grid(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_grid(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for Grid<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            areas: context.clone_encoded_node(self.areas),
            auto_columns: context.clone_encoded_vec(self.auto_columns),
            auto_flow: self.auto_flow,
            auto_rows: context.clone_encoded_vec(self.auto_rows),
            columns: context.clone_encoded_node(self.columns),
            rows: context.clone_encoded_node(self.rows),
        }
    }
}

fn encode_grid<'ast>(
    value: Grid<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    bytes[0] = encode_auto_flow_direction(value.auto_flow.direction);
    bytes[1] = value.auto_flow.dense as u8;
    write_node_id(&mut bytes, 4, value.areas);
    write_node_id(&mut bytes, 8, value.columns);
    let slots = [
        ExtraData::from_u64(value.rows.index() as u64),
        encode_range(value.auto_columns),
        encode_range(value.auto_rows),
    ];
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

impl<'ast> AstNodeStorage<'ast> for GridRow<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0018);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            end: read_node_id(&bytes, 0, context),
            start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.end, self.start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for GridRow<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            end: context.clone_encoded_node(self.end),
            start: context.clone_encoded_node(self.start),
        }
    }
}

impl<'ast> AstNodeStorage<'ast> for GridColumn<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0019);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            end: read_node_id(&bytes, 0, context),
            start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.end, self.start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for GridColumn<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            end: context.clone_encoded_node(self.end),
            start: context.clone_encoded_node(self.start),
        }
    }
}

impl<'ast> AstNodeStorage<'ast> for GridArea<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_001a);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            column_end: read_node_id(&bytes, 0, context),
            column_start: read_node_id(&bytes, 4, context),
            row_end: read_node_id(&bytes, 8, context),
            row_start: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(
            self.column_end,
            self.column_start,
            self.row_end,
            self.row_start,
        )
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for GridArea<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            column_end: context.clone_encoded_node(self.column_end),
            column_start: context.clone_encoded_node(self.column_start),
            row_end: context.clone_encoded_node(self.row_end),
            row_start: context.clone_encoded_node(self.row_start),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for MarginBlock<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0009);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            block_end: read_node_id(&bytes, 0, context),
            block_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.block_end, self.block_start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for MarginInline<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_000a);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            inline_end: read_node_id(&bytes, 0, context),
            inline_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.inline_end, self.inline_start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Margin<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for Margin<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_000b);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom: read_node_id(&bytes, 0, context),
            left: read_node_id(&bytes, 4, context),
            right: read_node_id(&bytes, 8, context),
            top: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(self.bottom, self.left, self.right, self.top)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for PaddingBlock<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_000c);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            block_end: read_node_id(&bytes, 0, context),
            block_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.block_end, self.block_start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for PaddingInline<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_000d);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            inline_end: read_node_id(&bytes, 0, context),
            inline_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.inline_end, self.inline_start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Padding<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for Padding<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_000e);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom: read_node_id(&bytes, 0, context),
            left: read_node_id(&bytes, 4, context),
            right: read_node_id(&bytes, 8, context),
            top: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(self.bottom, self.left, self.right, self.top)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for ScrollMarginBlock<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_000f);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            block_end: read_node_id(&bytes, 0, context),
            block_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.block_end, self.block_start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for ScrollMarginInline<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0010);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            inline_end: read_node_id(&bytes, 0, context),
            inline_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.inline_end, self.inline_start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMargin<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for ScrollMargin<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0011);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom: read_node_id(&bytes, 0, context),
            left: read_node_id(&bytes, 4, context),
            right: read_node_id(&bytes, 8, context),
            top: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(self.bottom, self.left, self.right, self.top)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for ScrollPaddingBlock<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0012);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            block_end: read_node_id(&bytes, 0, context),
            block_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.block_end, self.block_start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for ScrollPaddingInline<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0013);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            inline_end: read_node_id(&bytes, 0, context),
            inline_start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.inline_end, self.inline_start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPadding<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for ScrollPadding<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0014);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom: read_node_id(&bytes, 0, context),
            left: read_node_id(&bytes, 4, context),
            right: read_node_id(&bytes, 8, context),
            top: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(self.bottom, self.left, self.right, self.top)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
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

fn encode_auto_flow_direction(value: AutoFlowDirection) -> u8 {
    match value {
        AutoFlowDirection::Row => 0,
        AutoFlowDirection::Column => 1,
    }
}

fn decode_auto_flow_direction(value: u8) -> AutoFlowDirection {
    match value {
        0 => AutoFlowDirection::Row,
        1 => AutoFlowDirection::Column,
        _ => panic!("invalid encoded AutoFlowDirection"),
    }
}

fn encode_two_ids<T>(first: NodeId<'_, T>, second: NodeId<'_, T>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_node_id(&mut bytes, 0, first);
    write_node_id(&mut bytes, 4, second);
    NodePayload::inline(&bytes)
}

fn encode_four_ids<T>(
    first: NodeId<'_, T>,
    second: NodeId<'_, T>,
    third: NodeId<'_, T>,
    fourth: NodeId<'_, T>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_node_id(&mut bytes, 0, first);
    write_node_id(&mut bytes, 4, second);
    write_node_id(&mut bytes, 8, third);
    write_node_id(&mut bytes, 12, fourth);
    NodePayload::inline(&bytes)
}

fn write_node_id<T>(bytes: &mut [u8], offset: usize, id: NodeId<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(id.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn write_optional_node_id<T>(bytes: &mut [u8], offset: usize, id: Option<NodeId<'_, T>>) {
    write_u32(
        bytes,
        offset,
        id.map_or(u32::MAX, |id| {
            u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
        }),
    );
}

fn read_node_id<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, offset) as usize)
}

fn read_optional_node_id<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> Option<NodeId<'ast, T>> {
    let index = read_u32(bytes, offset);
    (index != u32::MAX).then(|| context.encoded_node_id_at(index as usize))
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact layout field is four bytes"),
    )
}

fn encode_flex_direction(value: FlexDirection) -> u8 {
    match value {
        FlexDirection::Row => 0,
        FlexDirection::RowReverse => 1,
        FlexDirection::Column => 2,
        FlexDirection::ColumnReverse => 3,
    }
}

fn decode_flex_direction(value: u8) -> FlexDirection {
    match value {
        0 => FlexDirection::Row,
        1 => FlexDirection::RowReverse,
        2 => FlexDirection::Column,
        3 => FlexDirection::ColumnReverse,
        _ => panic!("invalid encoded FlexDirection"),
    }
}

fn encode_flex_wrap(value: FlexWrap) -> u8 {
    match value {
        FlexWrap::Nowrap => 0,
        FlexWrap::Wrap => 1,
        FlexWrap::WrapReverse => 2,
    }
}

fn decode_flex_wrap(value: u8) -> FlexWrap {
    match value {
        0 => FlexWrap::Nowrap,
        1 => FlexWrap::Wrap,
        2 => FlexWrap::WrapReverse,
        _ => panic!("invalid encoded FlexWrap"),
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, ColumnCount, ColumnRule, ColumnWidth, Columns, DUMMY_SP, Flex, FlexDirection,
        FlexFlow, FlexWrap, Gap, GapValue, Length, LengthPercentageOrAuto, LengthUnit, LengthValue,
        LineStyle, Margin,
    };

    #[test]
    fn layout_node_codecs_preserve_optional_and_scalar_fields() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let basis = context.alloc_encoded_node(LengthPercentageOrAuto::Auto, DUMMY_SP);
        let flex = context.alloc_encoded_node(
            Flex {
                basis,
                grow: 2.0,
                shrink: 0.5,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(flex),
            Flex {
                basis,
                grow: 2.0,
                shrink: 0.5,
            }
        );

        let flow = context.alloc_encoded_node(
            FlexFlow {
                direction: FlexDirection::ColumnReverse,
                wrap: FlexWrap::WrapReverse,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(flow),
            FlexFlow {
                direction: FlexDirection::ColumnReverse,
                wrap: FlexWrap::WrapReverse,
            }
        );

        let rule = context.alloc_encoded_node(
            ColumnRule {
                color: None,
                style: Some(LineStyle::Dashed),
                width: None,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(rule),
            ColumnRule {
                color: None,
                style: Some(LineStyle::Dashed),
                width: None,
            }
        );
    }

    #[test]
    fn layout_aggregate_codecs_preserve_child_order() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.alloc_encoded_node(GapValue::Normal, DUMMY_SP);
        let second = context.alloc_encoded_node(GapValue::Normal, DUMMY_SP);
        let gap = context.alloc_encoded_node(
            Gap {
                column: first,
                row: second,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(gap),
            Gap {
                column: first,
                row: second,
            }
        );

        let length = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Rem,
                value: 3.0,
            }),
            DUMMY_SP,
        );
        let columns = context.alloc_encoded_node(
            Columns {
                count: ColumnCount::Integer(2),
                width: ColumnWidth::Length(length),
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(columns),
            Columns {
                count: ColumnCount::Integer(2),
                width: ColumnWidth::Length(length),
            }
        );

        let edges =
            [(); 4].map(|()| context.alloc_encoded_node(LengthPercentageOrAuto::Auto, DUMMY_SP));
        let margin = context.alloc_encoded_node(
            Margin {
                bottom: edges[0],
                left: edges[1],
                right: edges[2],
                top: edges[3],
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(margin),
            Margin {
                bottom: edges[0],
                left: edges[1],
                right: edges[2],
                top: edges[3],
            }
        );
    }
}
