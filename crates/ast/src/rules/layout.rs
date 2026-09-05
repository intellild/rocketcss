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
    pub track_sizes: Vec<'a, TrackSize<'a>>,
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
    pub auto_columns: Vec<'a, TrackSize<'a>>,
    pub auto_flow: GridAutoFlow,
    pub auto_rows: Vec<'a, TrackSize<'a>>,
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

#[derive(Debug, PartialEq, Visit)]
pub struct MarginBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MarginInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Margin<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct PaddingInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Padding<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMarginInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollMargin<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPaddingInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct ScrollPadding<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
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

fn encode_line_style(value: LineStyle) -> u8 {
    match value {
        LineStyle::None => 0,
        LineStyle::Hidden => 1,
        LineStyle::Inset => 2,
        LineStyle::Groove => 3,
        LineStyle::Outset => 4,
        LineStyle::Ridge => 5,
        LineStyle::Dotted => 6,
        LineStyle::Dashed => 7,
        LineStyle::Solid => 8,
        LineStyle::Double => 9,
    }
}

fn decode_line_style(value: u8) -> LineStyle {
    match value {
        0 => LineStyle::None,
        1 => LineStyle::Hidden,
        2 => LineStyle::Inset,
        3 => LineStyle::Groove,
        4 => LineStyle::Outset,
        5 => LineStyle::Ridge,
        6 => LineStyle::Dotted,
        7 => LineStyle::Dashed,
        8 => LineStyle::Solid,
        9 => LineStyle::Double,
        _ => panic!("invalid encoded LineStyle"),
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, ColumnCount, ColumnRule, ColumnWidth, Columns, DUMMY_SP, Flex, FlexDirection,
        FlexFlow, FlexWrap, Gap, GapValue, Length, LengthPercentageOrAuto, LengthUnit, LengthValue,
        LineStyle,
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
    }
}
