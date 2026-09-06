use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub enum Display {
    Keyword(DisplayKeyword),
    Pair {
        inside: DisplayInside,
        is_list_item: bool,
        outside: DisplayOutside,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum DisplayKeyword {
    None,
    Contents,
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
}

#[derive(Debug, PartialEq, Visit)]
pub enum DisplayInside {
    Flow,
    FlowRoot,
    Table,
    Flex { vendor_prefix: VendorPrefix },
    Box { vendor_prefix: VendorPrefix },
    Grid,
    Ruby,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum DisplayOutside {
    Block,
    Inline,
    RunIn,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Size<'a> {
    Auto,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    MathFunction(NodeId<'a, Function<'a>>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(NodeId<'a, LengthPercentage<'a>>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}

impl_inline_node!(Size<'ast>, 0x00080001);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum MaxSize<'a> {
    None,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    MathFunction(NodeId<'a, Function<'a>>),
    MinContent { vendor_prefix: VendorPrefix },
    MaxContent { vendor_prefix: VendorPrefix },
    FitContent { vendor_prefix: VendorPrefix },
    FitContentFunction(NodeId<'a, LengthPercentage<'a>>),
    Stretch { vendor_prefix: VendorPrefix },
    Contain,
}

impl_inline_node!(MaxSize<'ast>, 0x00080002);

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum OverflowKeyword {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum PositionProperty {
    Static,
    Relative,
    Absolute,
    Sticky(VendorPrefix),
    Fixed,
}

impl_inline_node!(PositionProperty, 0x0008000d);

impl AstNodeClone<'_> for PositionProperty {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Size2D<'a, T>(pub NodeId<'a, T>, pub NodeId<'a, T>);

impl<T> Copy for Size2D<'_, T> {}
impl<T> Clone for Size2D<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

trait Size2DKind {
    const KIND: NodeKind;
}

impl Size2DKind for LengthPercentage<'_> {
    const KIND: NodeKind = NodeKind::new(0x0008_0003);
}

impl Size2DKind for LengthPercentageOrAuto<'_> {
    const KIND: NodeKind = NodeKind::new(0x0008_0004);
}

impl Size2DKind for Length<'_> {
    const KIND: NodeKind = NodeKind::new(0x0008_0005);
}

impl Size2DKind for FontWeight {
    const KIND: NodeKind = NodeKind::new(0x0008_0006);
}

impl Size2DKind for FontStretch {
    const KIND: NodeKind = NodeKind::new(0x0008_0007);
}

impl Size2DKind for Angle {
    const KIND: NodeKind = NodeKind::new(0x0008_0008);
}

// SAFETY: each supported child type has a distinct kind and stores this native handle aggregate.
unsafe impl<'ast, T: Size2DKind> AstNodeStorage<'ast> for Size2D<'ast, T> {
    const KIND: NodeKind = T::KIND;
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

impl<'ast, T> AstNodeClone<'ast> for Size2D<'ast, T>
where
    T: Size2DKind + AstNodeClone<'ast>,
{
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self(
            context.clone_encoded_node(self.0),
            context.clone_encoded_node(self.1),
        )
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Rect<'a, T>(
    pub NodeId<'a, T>,
    pub NodeId<'a, T>,
    pub NodeId<'a, T>,
    pub NodeId<'a, T>,
);

impl<T> Copy for Rect<'_, T> {}
impl<T> Clone for Rect<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

trait RectKind {
    const KIND: NodeKind;
}

impl RectKind for LengthPercentage<'_> {
    const KIND: NodeKind = NodeKind::new(0x0008_0009);
}

impl RectKind for LengthOrNumber<'_> {
    const KIND: NodeKind = NodeKind::new(0x0008_000a);
}

impl RectKind for BorderImageSideWidth<'_> {
    const KIND: NodeKind = NodeKind::new(0x0008_000b);
}

impl RectKind for NumberOrPercentage {
    const KIND: NodeKind = NodeKind::new(0x0008_000c);
}

// SAFETY: each supported child type has a distinct kind and stores this native handle aggregate.
unsafe impl<'ast, T: RectKind> AstNodeStorage<'ast> for Rect<'ast, T> {
    const KIND: NodeKind = T::KIND;
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

impl<'ast, T> AstNodeClone<'ast> for Rect<'ast, T>
where
    T: RectKind + AstNodeClone<'ast>,
{
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self(
            context.clone_encoded_node(self.0),
            context.clone_encoded_node(self.1),
            context.clone_encoded_node(self.2),
            context.clone_encoded_node(self.3),
        )
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
}

#[derive(Debug, PartialEq, Visit)]
pub enum ZIndex {
    Auto,
    Integer(i32),
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        Angle, AstContext, DUMMY_SP, DimensionPercentage, LengthUnit, LengthValue, MaxSize, Rect,
        Size, Size2D, VendorPrefix,
    };

    #[test]
    fn size_codecs_preserve_variants_prefixes_and_child_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let length = context.alloc_encoded_node(
            DimensionPercentage::Dimension(LengthValue {
                unit: LengthUnit::Cqw,
                value: 12.5,
            }),
            DUMMY_SP,
        );

        let size = context.alloc_encoded_node(Size::LengthPercentage(length), DUMMY_SP);
        assert_eq!(context.encoded_node(size), Size::LengthPercentage(length));

        let max_size = context.alloc_encoded_node(
            MaxSize::FitContent {
                vendor_prefix: VendorPrefix::WEBKIT | VendorPrefix::MOZ,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(max_size),
            MaxSize::FitContent {
                vendor_prefix: VendorPrefix::WEBKIT | VendorPrefix::MOZ,
            }
        );
    }

    #[test]
    fn fixed_arity_id_aggregate_codecs_preserve_order() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let values = [1.0, 2.0, 3.0, 4.0].map(|value| {
            context.alloc_encoded_node(DimensionPercentage::Percentage(value), DUMMY_SP)
        });

        let pair = context.alloc_encoded_node(Size2D(values[0], values[1]), DUMMY_SP);
        assert_eq!(context.encoded_node(pair), Size2D(values[0], values[1]));

        let rect =
            context.alloc_encoded_node(Rect(values[0], values[1], values[2], values[3]), DUMMY_SP);
        assert_eq!(
            context.encoded_node(rect),
            Rect(values[0], values[1], values[2], values[3])
        );
    }

    #[test]
    fn cloning_an_id_aggregate_deep_clones_its_children() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.alloc_encoded_node(Angle::Deg(15.0), DUMMY_SP);
        let second = context.alloc_encoded_node(Angle::Turn(0.5), DUMMY_SP);
        let pair = context.alloc_encoded_node(Size2D(first, second), DUMMY_SP);

        let cloned = context.clone_encoded_node(pair);
        assert_ne!(pair, cloned);
        let Size2D(cloned_first, cloned_second) = context.encoded_node(cloned);
        assert_ne!(first, cloned_first);
        assert_ne!(second, cloned_second);
        assert_eq!(context.encoded_node(cloned_first), Angle::Deg(15.0));
        assert_eq!(context.encoded_node(cloned_second), Angle::Turn(0.5));
    }
}
