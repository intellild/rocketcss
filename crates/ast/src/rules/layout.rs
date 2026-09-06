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

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct InsetBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(InsetBlock<'ast>, 0x000a_0001);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct InsetInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(InsetInline<'ast>, 0x000a_0002);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Inset<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(Inset<'ast>, 0x000a_0003);

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub struct FlexFlow {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
}

impl_inline_node!(FlexFlow, 0x000a_0004);

impl AstNodeClone<'_> for FlexFlow {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Flex<'a> {
    pub basis: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub grow: f32,
    pub shrink: f32,
}

impl_inline_node!(Flex<'ast>, 0x000a_0005);

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

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Gap<'a> {
    pub column: NodeId<'a, GapValue<'a>>,
    pub row: NodeId<'a, GapValue<'a>>,
}

impl_inline_node!(Gap<'ast>, 0x000a_0006);

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub struct ColumnRule<'a> {
    pub color: Option<NodeId<'a, CssColor<'a>>>,
    pub style: Option<LineStyle>,
    pub width: Option<NodeId<'a, BorderSideWidth<'a>>>,
}

impl_inline_node!(ColumnRule<'ast>, 0x000a_0007);

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub enum ColumnWidth<'a> {
    Auto,
    Length(NodeId<'a, Length<'a>>),
}

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub enum ColumnCount {
    Auto,
    Integer(i32),
}

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub struct Columns<'a> {
    pub count: ColumnCount,
    pub width: ColumnWidth<'a>,
}

impl_inline_node!(Columns<'ast>, 0x000a_0008);

#[derive(Debug, PartialEq, Visit)]
pub struct TrackRepeat<'a> {
    pub count: RepeatCount,
    pub line_names: Vec<'a, Vec<'a, AstStr<'a>>>,
    pub track_sizes: Vec<'a, NodeId<'a, TrackSize<'a>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct GridAutoFlow {
    pub dense: bool,
    pub direction: AutoFlowDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
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

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct GridRow<'a> {
    pub end: NodeId<'a, GridLine<'a>>,
    pub start: NodeId<'a, GridLine<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct GridColumn<'a> {
    pub end: NodeId<'a, GridLine<'a>>,
    pub start: NodeId<'a, GridLine<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct GridArea<'a> {
    pub column_end: NodeId<'a, GridLine<'a>>,
    pub column_start: NodeId<'a, GridLine<'a>>,
    pub row_end: NodeId<'a, GridLine<'a>>,
    pub row_start: NodeId<'a, GridLine<'a>>,
}

#[derive(Clone, Copy)]
struct TrackRepeatHeader {
    count: RepeatCount,
    extra: u32,
}
pub use track_repeat_access::TrackRepeatRead;

mod track_repeat_access {
    use super::*;
    pub struct TrackRepeatRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: TrackRepeatHeader,
        marker: std::marker::PhantomData<&'id ()>,
    }
    impl<'id> TrackRepeatRead<'_, '_, 'id> {
        pub fn count(&self) -> RepeatCount {
            self.header.count
        }
        pub fn line_names(&self) -> Vec<'id, Vec<'id, AstStr<'id>>> {
            // SAFETY: the first slot stores the nested line-name range.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            }
        }
        pub fn track_sizes(&self) -> Vec<'id, NodeId<'id, TrackSize<'id>>> {
            // SAFETY: the second slot stores the track-size handle range.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 1)
                    .read_value()
            }
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn track_repeat<'id>(
            &self,
            id: NodeId<'id, TrackRepeat<'id>>,
        ) -> TrackRepeatRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the owning kind before reading its header.
            TrackRepeatRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
                marker: std::marker::PhantomData,
            }
        }
    }
}

// SAFETY: this kind stores TrackRepeatHeader and two independently typed ranges.
unsafe impl<'ast> AstNodeStorage<'ast> for TrackRepeat<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0015);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: TrackRepeatHeader = unsafe { payload.read_value() };
        Self {
            count: header.count,
            line_names: unsafe { context.extra_slot(header.extra as usize).read_value() },
            track_sizes: unsafe { context.extra_slot(header.extra as usize + 1).read_value() },
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_track_repeat(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: TrackRepeatHeader = unsafe { current.read_value() };
        encode_track_repeat(self, Some(header.extra as usize), context)
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
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let slots = [
        ExtraData::from_value(value.line_names),
        ExtraData::from_value(value.track_sizes),
    ];
    let extra = match existing {
        Some(extra) => {
            for (i, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(extra + i, slot);
            }
            extra
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::from_value(TrackRepeatHeader {
        count: value.count,
        extra: u32::try_from(extra).expect("AST extra index exceeds u32"),
    })
}

impl_inline_node!(GridTemplate<'ast>, 0x000a_0016);

impl<'ast> AstNodeClone<'ast> for GridTemplate<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            areas: context.clone_encoded_node(self.areas),
            columns: context.clone_encoded_node(self.columns),
            rows: context.clone_encoded_node(self.rows),
        }
    }
}

#[derive(Clone, Copy)]
struct GridHeader<'a> {
    areas: NodeId<'a, GridTemplateAreas<'a>>,
    columns: NodeId<'a, TrackSizing<'a>>,
    rows: NodeId<'a, TrackSizing<'a>>,
    extra: u32,
}
pub use grid_access::GridRead;

mod grid_access {
    use super::*;
    pub struct GridRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: GridHeader<'id>,
    }
    impl<'id> GridRead<'_, '_, 'id> {
        pub fn rows(&self) -> NodeId<'id, TrackSizing<'id>> {
            self.header.rows
        }
        pub fn columns(&self) -> NodeId<'id, TrackSizing<'id>> {
            self.header.columns
        }
        pub fn areas(&self) -> NodeId<'id, GridTemplateAreas<'id>> {
            self.header.areas
        }
        pub fn auto_columns(&self) -> Vec<'id, NodeId<'id, TrackSize<'id>>> {
            // SAFETY: slot zero stores the auto-column range.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            }
        }
        pub fn auto_rows(&self) -> Vec<'id, NodeId<'id, TrackSize<'id>>> {
            // SAFETY: slot one stores the auto-row range.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 1)
                    .read_value()
            }
        }
        pub fn auto_flow(&self) -> GridAutoFlow {
            // SAFETY: slot two stores native GridAutoFlow, including its bool.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 2)
                    .read_value()
            }
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn grid<'id>(&self, id: NodeId<'id, Grid<'id>>) -> GridRead<'_, 'storage, 'id> {
            // SAFETY: node_payload validates the owning kind before reading the header.
            GridRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: this kind stores GridHeader, two native ranges and native GridAutoFlow.
unsafe impl<'ast> AstNodeStorage<'ast> for Grid<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000a_0017);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: GridHeader<'ast> = unsafe { payload.read_value() };
        Self {
            areas: header.areas,
            columns: header.columns,
            rows: header.rows,
            auto_columns: unsafe { context.extra_slot(header.extra as usize).read_value() },
            auto_rows: unsafe { context.extra_slot(header.extra as usize + 1).read_value() },
            auto_flow: unsafe { context.extra_slot(header.extra as usize + 2).read_value() },
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_grid(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: GridHeader<'ast> = unsafe { current.read_value() };
        encode_grid(self, Some(header.extra as usize), context)
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
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let slots = [
        ExtraData::from_value(value.auto_columns),
        ExtraData::from_value(value.auto_rows),
        ExtraData::from_value(value.auto_flow),
    ];
    let extra = match existing {
        Some(extra) => {
            for (i, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(extra + i, slot);
            }
            extra
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::from_value(GridHeader {
        areas: value.areas,
        columns: value.columns,
        rows: value.rows,
        extra: u32::try_from(extra).expect("AST extra index exceeds u32"),
    })
}

impl_inline_node!(GridRow<'ast>, 0x000a_0018);

impl<'ast> AstNodeClone<'ast> for GridRow<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            end: context.clone_encoded_node(self.end),
            start: context.clone_encoded_node(self.start),
        }
    }
}

impl_inline_node!(GridColumn<'ast>, 0x000a_0019);

impl<'ast> AstNodeClone<'ast> for GridColumn<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            end: context.clone_encoded_node(self.end),
            start: context.clone_encoded_node(self.start),
        }
    }
}

impl_inline_node!(GridArea<'ast>, 0x000a_001a);

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

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct MarginBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(MarginBlock<'ast>, 0x000a_0009);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct MarginInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(MarginInline<'ast>, 0x000a_000a);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Margin<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(Margin<'ast>, 0x000a_000b);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct PaddingBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(PaddingBlock<'ast>, 0x000a_000c);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct PaddingInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(PaddingInline<'ast>, 0x000a_000d);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Padding<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(Padding<'ast>, 0x000a_000e);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ScrollMarginBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(ScrollMarginBlock<'ast>, 0x000a_000f);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ScrollMarginInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(ScrollMarginInline<'ast>, 0x000a_0010);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ScrollMargin<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(ScrollMargin<'ast>, 0x000a_0011);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ScrollPaddingBlock<'a> {
    pub block_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub block_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(ScrollPaddingBlock<'ast>, 0x000a_0012);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ScrollPaddingInline<'a> {
    pub inline_end: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub inline_start: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(ScrollPaddingInline<'ast>, 0x000a_0013);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ScrollPadding<'a> {
    pub bottom: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub left: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub right: NodeId<'a, LengthPercentageOrAuto<'a>>,
    pub top: NodeId<'a, LengthPercentageOrAuto<'a>>,
}

impl_inline_node!(ScrollPadding<'ast>, 0x000a_0014);

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, ColumnCount, ColumnRule, ColumnWidth, Columns, DUMMY_SP, Flex, FlexDirection,
        FlexFlow, FlexWrap, Gap, GapValue, Length, LengthPercentageOrAuto, LengthUnit, LengthValue,
        LineStyle, Margin,
    };

    #[test]
    fn grid_and_repeat_native_overflow_reuses_typed_ranges() {
        use crate::{
            AutoFlowDirection, Grid, GridAutoFlow, GridTemplateAreas, RepeatCount, TrackRepeat,
            TrackSizing,
        };
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let line_names = ast.alloc_encoded_vec(std::iter::empty());
        let tracks = ast.alloc_encoded_vec(std::iter::empty());
        let before = ast.encoded_extra_len();
        let repeat = ast.alloc_node(
            TrackRepeat {
                count: RepeatCount::AutoFill,
                line_names,
                track_sizes: tracks,
            },
            DUMMY_SP,
        );
        assert_eq!(ast.encoded_extra_len(), before + 2);
        let areas = ast.alloc_node(GridTemplateAreas::None, DUMMY_SP);
        let columns = ast.alloc_node(TrackSizing::None, DUMMY_SP);
        let rows = ast.alloc_node(TrackSizing::None, DUMMY_SP);
        let grid = ast.alloc_node(
            Grid {
                areas,
                columns,
                rows,
                auto_columns: tracks,
                auto_rows: tracks,
                auto_flow: GridAutoFlow {
                    dense: false,
                    direction: AutoFlowDirection::Row,
                },
            },
            DUMMY_SP,
        );
        assert_eq!(ast.encoded_extra_len(), before + 5);
        let checkpoint = ast.node_checkpoint();
        for count in [
            RepeatCount::AutoFill,
            RepeatCount::AutoFit,
            RepeatCount::Number(-0.0),
        ] {
            ast.mutate_node(repeat, |value, _| value.count = count);
            let value = ast.resolve_node(repeat);
            assert_eq!(value.count, count);
            if let RepeatCount::Number(value) = value.count {
                assert_eq!(value.to_bits(), (-0.0f32).to_bits());
            }
            assert_eq!(value.line_names, line_names);
            assert_eq!(value.track_sizes, tracks);
            let view = ast.track_repeat(repeat);
            assert_eq!(view.count(), count);
            if let RepeatCount::Number(value) = view.count() {
                assert_eq!(value.to_bits(), (-0.0f32).to_bits());
            }
            assert_eq!(view.line_names(), line_names);
            assert_eq!(view.track_sizes(), tracks);
            ast.mutate_node(grid, |value, _| {
                value.auto_flow.dense = !value.auto_flow.dense;
                value.auto_flow.direction = AutoFlowDirection::Column;
            });
            let value = ast.resolve_node(grid);
            assert_eq!(value.auto_flow.direction, AutoFlowDirection::Column);
            assert_eq!(
                (value.areas, value.columns, value.rows),
                (areas, columns, rows)
            );
            assert_eq!(value.auto_columns, tracks);
            assert_eq!(value.auto_rows, tracks);
            let view = ast.grid(grid);
            assert_eq!(
                (view.areas(), view.columns(), view.rows()),
                (areas, columns, rows)
            );
            assert_eq!(view.auto_flow(), value.auto_flow);
            assert_eq!(view.auto_rows(), tracks);
            assert_eq!(view.auto_columns(), tracks);
        }
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_1234,
        ] {
            ast.mutate_node(repeat, |value, _| {
                value.count = RepeatCount::Number(f32::from_bits(bits))
            });
            for count in [
                ast.resolve_node(repeat).count,
                ast.track_repeat(repeat).count(),
            ] {
                let RepeatCount::Number(value) = count else {
                    panic!("expected number");
                };
                assert_eq!(value.to_bits(), bits);
            }
            assert_eq!(ast.track_repeat(repeat).line_names(), line_names);
            assert_eq!(ast.track_repeat(repeat).track_sizes(), tracks);
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }

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
