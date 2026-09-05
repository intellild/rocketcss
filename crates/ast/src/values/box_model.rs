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

#[derive(Debug, PartialEq, Visit)]
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

// Fixed payload layout for `Size` and `MaxSize`:
//
// byte 0      variant
// byte 1      VendorPrefix bits for prefixed keyword variants
// bytes 2..4  reserved
// bytes 4..8  child NodeId for node-bearing variants
// bytes 8..16 reserved
impl<'ast> AstNodeStorage<'ast> for Size<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0008_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let child = read_u32(&bytes, 4) as usize;
        match bytes[0] {
            0 => Self::Auto,
            1 => Self::LengthPercentage(context.encoded_node_id_at(child)),
            2 => Self::MathFunction(context.encoded_node_id_at(child)),
            3 => Self::MinContent {
                vendor_prefix: decode_vendor_prefix(bytes[1]),
            },
            4 => Self::MaxContent {
                vendor_prefix: decode_vendor_prefix(bytes[1]),
            },
            5 => Self::FitContent {
                vendor_prefix: decode_vendor_prefix(bytes[1]),
            },
            6 => Self::FitContentFunction(context.encoded_node_id_at(child)),
            7 => Self::Stretch {
                vendor_prefix: decode_vendor_prefix(bytes[1]),
            },
            8 => Self::Contain,
            _ => panic!("invalid encoded Size variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_size(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_size(self)
    }
}

fn encode_size(value: Size<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        Size::Auto => bytes[0] = 0,
        Size::LengthPercentage(value) => write_node_id(&mut bytes, 1, value),
        Size::MathFunction(value) => write_node_id(&mut bytes, 2, value),
        Size::MinContent { vendor_prefix } => write_vendor_prefix(&mut bytes, 3, vendor_prefix),
        Size::MaxContent { vendor_prefix } => write_vendor_prefix(&mut bytes, 4, vendor_prefix),
        Size::FitContent { vendor_prefix } => write_vendor_prefix(&mut bytes, 5, vendor_prefix),
        Size::FitContentFunction(value) => write_node_id(&mut bytes, 6, value),
        Size::Stretch { vendor_prefix } => write_vendor_prefix(&mut bytes, 7, vendor_prefix),
        Size::Contain => bytes[0] = 8,
    }
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
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

impl<'ast> AstNodeStorage<'ast> for MaxSize<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0008_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let child = read_u32(&bytes, 4) as usize;
        match bytes[0] {
            0 => Self::None,
            1 => Self::LengthPercentage(context.encoded_node_id_at(child)),
            2 => Self::MathFunction(context.encoded_node_id_at(child)),
            3 => Self::MinContent {
                vendor_prefix: decode_vendor_prefix(bytes[1]),
            },
            4 => Self::MaxContent {
                vendor_prefix: decode_vendor_prefix(bytes[1]),
            },
            5 => Self::FitContent {
                vendor_prefix: decode_vendor_prefix(bytes[1]),
            },
            6 => Self::FitContentFunction(context.encoded_node_id_at(child)),
            7 => Self::Stretch {
                vendor_prefix: decode_vendor_prefix(bytes[1]),
            },
            8 => Self::Contain,
            _ => panic!("invalid encoded MaxSize variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_max_size(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_max_size(self)
    }
}

fn encode_max_size(value: MaxSize<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        MaxSize::None => bytes[0] = 0,
        MaxSize::LengthPercentage(value) => write_node_id(&mut bytes, 1, value),
        MaxSize::MathFunction(value) => write_node_id(&mut bytes, 2, value),
        MaxSize::MinContent { vendor_prefix } => write_vendor_prefix(&mut bytes, 3, vendor_prefix),
        MaxSize::MaxContent { vendor_prefix } => write_vendor_prefix(&mut bytes, 4, vendor_prefix),
        MaxSize::FitContent { vendor_prefix } => write_vendor_prefix(&mut bytes, 5, vendor_prefix),
        MaxSize::FitContentFunction(value) => write_node_id(&mut bytes, 6, value),
        MaxSize::Stretch { vendor_prefix } => write_vendor_prefix(&mut bytes, 7, vendor_prefix),
        MaxSize::Contain => bytes[0] = 8,
    }
    NodePayload::inline(&bytes)
}

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

#[derive(Debug, PartialEq, Visit)]
pub enum PositionProperty {
    Static,
    Relative,
    Absolute,
    Sticky(VendorPrefix),
    Fixed,
}

impl AstNodeStorage<'_> for PositionProperty {
    const KIND: NodeKind = NodeKind::new(0x0008_000d);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Static,
            1 => Self::Relative,
            2 => Self::Absolute,
            3 => Self::Sticky(decode_vendor_prefix(bytes[1])),
            4 => Self::Fixed,
            _ => panic!("invalid encoded PositionProperty variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Static => bytes[0] = 0,
            Self::Relative => bytes[0] = 1,
            Self::Absolute => bytes[0] = 2,
            Self::Sticky(prefix) => write_vendor_prefix(&mut bytes, 3, prefix),
            Self::Fixed => bytes[0] = 4,
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        self.encode_new(context)
    }
}

impl AstNodeClone<'_> for PositionProperty {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Size2D<'a, T>(pub NodeId<'a, T>, pub NodeId<'a, T>);

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

impl<'ast, T: Size2DKind> AstNodeStorage<'ast> for Size2D<'ast, T> {
    const KIND: NodeKind = T::KIND;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self(
            context.encoded_node_id_at(read_u32(&bytes, 0) as usize),
            context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
        )
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_size_2d(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_size_2d(self)
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

fn encode_size_2d<T>(value: Size2D<'_, T>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_u32(&mut bytes, 0, node_index(value.0));
    write_u32(&mut bytes, 4, node_index(value.1));
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub struct Rect<'a, T>(
    pub NodeId<'a, T>,
    pub NodeId<'a, T>,
    pub NodeId<'a, T>,
    pub NodeId<'a, T>,
);

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

impl<'ast, T: RectKind> AstNodeStorage<'ast> for Rect<'ast, T> {
    const KIND: NodeKind = T::KIND;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self(
            context.encoded_node_id_at(read_u32(&bytes, 0) as usize),
            context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
            context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            context.encoded_node_id_at(read_u32(&bytes, 12) as usize),
        )
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_rect(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_rect(self)
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

fn encode_rect<T>(value: Rect<'_, T>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_u32(&mut bytes, 0, node_index(value.0));
    write_u32(&mut bytes, 4, node_index(value.1));
    write_u32(&mut bytes, 8, node_index(value.2));
    write_u32(&mut bytes, 12, node_index(value.3));
    NodePayload::inline(&bytes)
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

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn write_node_id<T>(bytes: &mut [u8], tag: u8, id: NodeId<'_, T>) {
    bytes[0] = tag;
    write_u32(bytes, 4, node_index(id));
}

fn write_vendor_prefix(bytes: &mut [u8], tag: u8, prefix: VendorPrefix) {
    bytes[0] = tag;
    bytes[1] = prefix.bits();
}

fn decode_vendor_prefix(bits: u8) -> VendorPrefix {
    VendorPrefix::from_bits_retain(bits)
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact box-model field is four bytes"),
    )
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
