use crate::*;

use crate::{AstNodeStorage, ExtraData, ExtraDataCompact, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub enum Image<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
    Gradient(NodeId<'a, Gradient<'a>>),
    ImageSet(NodeId<'a, ImageSet<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for Image<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let id = read_u32(&bytes, 4) as usize;
        match bytes[0] {
            0 => Self::None,
            1 => Self::Url(context.encoded_node_id_at(id)),
            2 => Self::Gradient(context.encoded_node_id_at(id)),
            3 => Self::ImageSet(context.encoded_node_id_at(id)),
            _ => panic!("invalid encoded Image variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_image(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_image(self)
    }
}

fn encode_image(value: Image<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        Image::None => bytes[0] = 0,
        Image::Url(value) => write_node_id(&mut bytes, 1, value),
        Image::Gradient(value) => write_node_id(&mut bytes, 2, value),
        Image::ImageSet(value) => write_node_id(&mut bytes, 3, value),
    }
    NodePayload::inline(&bytes)
}

impl<'ast> ExtraDataCompact<'ast> for Image<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        inline_payload_as_extra(encode_image(self))
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        Self::decode(NodePayload::inline(&data.bytes()), context)
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Gradient<'a> {
    Linear {
        direction: LineDirection,
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingLinear {
        direction: LineDirection,
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        vendor_prefix: VendorPrefix,
    },
    Radial {
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        position: NodeId<'a, Position<'a>>,
        shape: NodeId<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingRadial {
        items: Vec<'a, GradientItem<'a, LengthValue>>,
        position: NodeId<'a, Position<'a>>,
        shape: NodeId<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    Conic {
        angle: Angle,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: NodeId<'a, Position<'a>>,
    },
    RepeatingConic {
        angle: Angle,
        items: Vec<'a, GradientItem<'a, Angle>>,
        position: NodeId<'a, Position<'a>>,
    },
    WebKitGradient(NodeId<'a, WebKitGradient<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum WebKitGradient<'a> {
    Linear {
        from: NodeId<'a, WebKitGradientPoint>,
        to: NodeId<'a, WebKitGradientPoint>,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
    Radial {
        from: NodeId<'a, WebKitGradientPoint>,
        start_radius: f32,
        to: NodeId<'a, WebKitGradientPoint>,
        end_radius: f32,
        stops: Vec<'a, WebKitColorStop<'a>>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum LineDirection {
    Angle(Angle),
    Horizontal(HorizontalPositionKeyword),
    Vertical(VerticalPositionKeyword),
    Corner {
        horizontal: HorizontalPositionKeyword,
        vertical: VerticalPositionKeyword,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum HorizontalPositionKeyword {
    Left,
    Right,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum VerticalPositionKeyword {
    Top,
    Bottom,
}

#[derive(Debug, PartialEq, Visit)]
pub enum GradientItem<'a, D> {
    ColorStop {
        color: NodeId<'a, CssColor<'a>>,
        position: Option<NodeId<'a, DimensionPercentage<'a, D>>>,
    },
    Hint(NodeId<'a, DimensionPercentage<'a, D>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum DimensionPercentage<'a, D> {
    Dimension(D),
    Percentage(f32),
    /// A unitless zero produced by target-aware minification.
    Zero,
    Calc(NodeId<'a, Calc<'a, DimensionPercentage<'a, D>>>),
}

pub type LengthPercentage<'a> = DimensionPercentage<'a, LengthValue>;
pub type AnglePercentage<'a> = DimensionPercentage<'a, Angle>;

trait DimensionCodec: Sized {
    const NODE_KIND: NodeKind;

    fn encode(self) -> (u8, u32);

    fn decode(kind: u8, value: u32) -> Self;
}

impl DimensionCodec for LengthValue {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0002);

    fn encode(self) -> (u8, u32) {
        (
            crate::length::encode_length_unit(self.unit),
            self.value.to_bits(),
        )
    }

    fn decode(kind: u8, value: u32) -> Self {
        Self {
            unit: crate::length::decode_length_unit(kind),
            value: f32::from_bits(value),
        }
    }
}

impl DimensionCodec for Angle {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0003);

    fn encode(self) -> (u8, u32) {
        let (kind, value) = crate::token::encode_angle(self);
        (kind, value.to_bits())
    }

    fn decode(kind: u8, value: u32) -> Self {
        crate::token::decode_angle(kind, f32::from_bits(value))
    }
}

// Fixed payload layout for `DimensionPercentage<D>`:
//
// byte 0      variant
// byte 1      dimension kind
// bytes 2..4  reserved
// bytes 4..8  dimension/percentage bits or Calc NodeId index
// bytes 8..16 reserved
impl<'ast, D: DimensionCodec> AstNodeStorage<'ast> for DimensionPercentage<'ast, D> {
    const KIND: NodeKind = D::NODE_KIND;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let value = read_u32(&bytes, 4);
        match bytes[0] {
            0 => Self::Dimension(D::decode(bytes[1], value)),
            1 => Self::Percentage(f32::from_bits(value)),
            2 => Self::Zero,
            3 => Self::Calc(context.encoded_node_id_at(value as usize)),
            _ => panic!("invalid encoded DimensionPercentage variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_dimension_percentage(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_dimension_percentage(self)
    }
}

impl<'ast, D: DimensionCodec> ExtraDataCompact<'ast> for DimensionPercentage<'ast, D> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        inline_payload_as_extra(encode_dimension_percentage(self))
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        Self::decode(NodePayload::inline(&data.bytes()), context)
    }
}

fn encode_dimension_percentage<D: DimensionCodec>(
    value: DimensionPercentage<'_, D>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    let data = match value {
        DimensionPercentage::Dimension(value) => {
            bytes[0] = 0;
            let (kind, value) = value.encode();
            bytes[1] = kind;
            value
        }
        DimensionPercentage::Percentage(value) => {
            bytes[0] = 1;
            value.to_bits()
        }
        DimensionPercentage::Zero => {
            bytes[0] = 2;
            0
        }
        DimensionPercentage::Calc(value) => {
            bytes[0] = 3;
            node_index(value)
        }
    };
    write_u32(&mut bytes, 4, data);
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum PositionComponent<'a, S> {
    Center,
    Length(NodeId<'a, LengthPercentage<'a>>),
    Side {
        offset: Option<NodeId<'a, LengthPercentage<'a>>>,
        side: S,
    },
}

trait PositionSideCodec: Sized {
    const NODE_KIND: NodeKind;

    fn encode(self) -> u8;

    fn decode(value: u8) -> Self;
}

impl PositionSideCodec for HorizontalPositionKeyword {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0004);

    fn encode(self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Right => 1,
        }
    }

    fn decode(value: u8) -> Self {
        match value {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("invalid encoded HorizontalPositionKeyword"),
        }
    }
}

impl PositionSideCodec for VerticalPositionKeyword {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0005);

    fn encode(self) -> u8 {
        match self {
            Self::Top => 0,
            Self::Bottom => 1,
        }
    }

    fn decode(value: u8) -> Self {
        match value {
            0 => Self::Top,
            1 => Self::Bottom,
            _ => panic!("invalid encoded VerticalPositionKeyword"),
        }
    }
}

// byte 0 variant, byte 1 side, bytes 4..8 optional offset or length NodeId.
impl<'ast, S: PositionSideCodec> AstNodeStorage<'ast> for PositionComponent<'ast, S> {
    const KIND: NodeKind = S::NODE_KIND;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let id = read_u32(&bytes, 4);
        match bytes[0] {
            0 => Self::Center,
            1 => Self::Length(context.encoded_node_id_at(id as usize)),
            2 => Self::Side {
                offset: (id != u32::MAX).then(|| context.encoded_node_id_at(id as usize)),
                side: S::decode(bytes[1]),
            },
            _ => panic!("invalid encoded PositionComponent variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_position_component(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_position_component(self)
    }
}

impl<'ast, S: PositionSideCodec> ExtraDataCompact<'ast> for PositionComponent<'ast, S> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        inline_payload_as_extra(encode_position_component(self))
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        Self::decode(NodePayload::inline(&data.bytes()), context)
    }
}

fn encode_position_component<S: PositionSideCodec>(value: PositionComponent<'_, S>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    let id = match value {
        PositionComponent::Center => {
            bytes[0] = 0;
            0
        }
        PositionComponent::Length(value) => {
            bytes[0] = 1;
            node_index(value)
        }
        PositionComponent::Side { offset, side } => {
            bytes[0] = 2;
            bytes[1] = side.encode();
            offset.map_or(u32::MAX, node_index)
        }
    };
    write_u32(&mut bytes, 4, id);
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum EndingShape<'a> {
    Ellipse(NodeId<'a, Ellipse<'a>>),
    Circle(NodeId<'a, Circle<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for EndingShape<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_0006);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let id = read_u32(&bytes, 4) as usize;
        match bytes[0] {
            0 => Self::Ellipse(context.encoded_node_id_at(id)),
            1 => Self::Circle(context.encoded_node_id_at(id)),
            _ => panic!("invalid encoded EndingShape variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_ending_shape(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_ending_shape(self)
    }
}

fn encode_ending_shape(value: EndingShape<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        EndingShape::Ellipse(value) => write_node_id(&mut bytes, 0, value),
        EndingShape::Circle(value) => write_node_id(&mut bytes, 1, value),
    }
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum Ellipse<'a> {
    Size {
        x: NodeId<'a, LengthPercentage<'a>>,
        y: NodeId<'a, LengthPercentage<'a>>,
    },
    Extent(ShapeExtent),
}

impl<'ast> AstNodeStorage<'ast> for Ellipse<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_0007);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Size {
                x: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
                y: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            },
            1 => Self::Extent(decode_shape_extent(bytes[1])),
            _ => panic!("invalid encoded Ellipse variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_ellipse(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_ellipse(self)
    }
}

fn encode_ellipse(value: Ellipse<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        Ellipse::Size { x, y } => {
            bytes[0] = 0;
            write_u32(&mut bytes, 4, node_index(x));
            write_u32(&mut bytes, 8, node_index(y));
        }
        Ellipse::Extent(value) => {
            bytes[0] = 1;
            bytes[1] = encode_shape_extent(value);
        }
    }
    NodePayload::inline(&bytes)
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ShapeExtent {
    ClosestSide,
    FarthestSide,
    ClosestCorner,
    FarthestCorner,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Circle<'a> {
    Radius(NodeId<'a, Length<'a>>),
    Extent(ShapeExtent),
}

impl<'ast> AstNodeStorage<'ast> for Circle<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_0008);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Radius(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            1 => Self::Extent(decode_shape_extent(bytes[1])),
            _ => panic!("invalid encoded Circle variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_circle(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_circle(self)
    }
}

fn encode_circle(value: Circle<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        Circle::Radius(value) => write_node_id(&mut bytes, 0, value),
        Circle::Extent(value) => {
            bytes[0] = 1;
            bytes[1] = encode_shape_extent(value);
        }
    }
    NodePayload::inline(&bytes)
}

fn encode_shape_extent(value: ShapeExtent) -> u8 {
    match value {
        ShapeExtent::ClosestSide => 0,
        ShapeExtent::FarthestSide => 1,
        ShapeExtent::ClosestCorner => 2,
        ShapeExtent::FarthestCorner => 3,
    }
}

fn decode_shape_extent(value: u8) -> ShapeExtent {
    match value {
        0 => ShapeExtent::ClosestSide,
        1 => ShapeExtent::FarthestSide,
        2 => ShapeExtent::ClosestCorner,
        3 => ShapeExtent::FarthestCorner,
        _ => panic!("invalid encoded ShapeExtent"),
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum WebKitGradientPointComponent<S> {
    Center,
    Number(NumberOrPercentage),
    Side(S),
}

#[derive(Debug, PartialEq, Visit)]
pub enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}

#[derive(Debug, PartialEq, Visit)]
pub enum BackgroundSize<'a> {
    Explicit {
        height: NodeId<'a, LengthPercentageOrAuto<'a>>,
        width: NodeId<'a, LengthPercentageOrAuto<'a>>,
    },
    Cover,
    Contain,
}

impl<'ast> AstNodeStorage<'ast> for BackgroundSize<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_0009);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Explicit {
                height: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
                width: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            },
            1 => Self::Cover,
            2 => Self::Contain,
            _ => panic!("invalid encoded BackgroundSize variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_background_size(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_background_size(self)
    }
}

fn encode_background_size(value: BackgroundSize<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        BackgroundSize::Explicit { height, width } => {
            bytes[0] = 0;
            write_u32(&mut bytes, 4, node_index(height));
            write_u32(&mut bytes, 8, node_index(width));
        }
        BackgroundSize::Cover => bytes[0] = 1,
        BackgroundSize::Contain => bytes[0] = 2,
    }
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum LengthPercentageOrAuto<'a> {
    Auto,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for LengthPercentageOrAuto<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_000a);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Auto,
            1 => Self::LengthPercentage(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded LengthPercentageOrAuto variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_length_percentage_or_auto(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_length_percentage_or_auto(self)
    }
}

fn encode_length_percentage_or_auto(value: LengthPercentageOrAuto<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        LengthPercentageOrAuto::Auto => bytes[0] = 0,
        LengthPercentageOrAuto::LengthPercentage(value) => {
            write_node_id(&mut bytes, 1, value);
        }
    }
    NodePayload::inline(&bytes)
}

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn write_node_id<T>(bytes: &mut [u8; NodePayload::INLINE_BYTES], tag: u8, id: NodeId<'_, T>) {
    bytes[0] = tag;
    write_u32(bytes, 4, node_index(id));
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact image field is four bytes"),
    )
}

fn inline_payload_as_extra(payload: NodePayload) -> ExtraData {
    ExtraData::from_bytes(&payload.bytes()[..ExtraData::BYTES])
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, BackgroundSize, Circle, DUMMY_SP, DimensionPercentage, Ellipse, EndingShape,
        HorizontalPositionKeyword, Image, LengthPercentageOrAuto, PositionComponent, ShapeExtent,
        Url, VerticalPositionKeyword,
    };

    #[test]
    fn image_and_dimension_codecs_round_trip_compact_variants() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let url = context.alloc_encoded_node(Url { url: "asset.webp" }, DUMMY_SP);
        let image = context.alloc_encoded_node(Image::Url(url), DUMMY_SP);
        assert_eq!(context.encoded_node(image), Image::Url(url));

        let length = context.alloc_encoded_node(
            DimensionPercentage::Dimension(crate::LengthValue {
                unit: crate::LengthUnit::Cqw,
                value: 2.5,
            }),
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(length),
            DimensionPercentage::Dimension(crate::LengthValue {
                unit: crate::LengthUnit::Cqw,
                value: 2.5,
            })
        );

        let angle = context.alloc_encoded_node(
            DimensionPercentage::<crate::Angle>::Percentage(33.0),
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(angle),
            DimensionPercentage::<crate::Angle>::Percentage(33.0)
        );
    }

    #[test]
    fn position_and_shape_codecs_preserve_typed_child_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let offset = context.alloc_encoded_node(DimensionPercentage::Percentage(25.0), DUMMY_SP);
        let horizontal = context.alloc_encoded_node(
            PositionComponent::Side {
                offset: Some(offset),
                side: HorizontalPositionKeyword::Right,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(horizontal),
            PositionComponent::Side {
                offset: Some(offset),
                side: HorizontalPositionKeyword::Right,
            }
        );

        let vertical = context.alloc_encoded_node(
            PositionComponent::<VerticalPositionKeyword>::Center,
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(vertical),
            PositionComponent::<VerticalPositionKeyword>::Center
        );

        let ellipse = context.alloc_encoded_node(
            Ellipse::Size {
                x: offset,
                y: offset,
            },
            DUMMY_SP,
        );
        let shape = context.alloc_encoded_node(EndingShape::Ellipse(ellipse), DUMMY_SP);
        assert_eq!(context.encoded_node(shape), EndingShape::Ellipse(ellipse));

        let circle = context.alloc_encoded_node(Circle::Extent(ShapeExtent::ClosestSide), DUMMY_SP);
        assert_eq!(
            context.encoded_node(circle),
            Circle::Extent(ShapeExtent::ClosestSide)
        );
    }

    #[test]
    fn background_size_mutation_reuses_the_same_node_identity() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let auto = context.alloc_encoded_node(LengthPercentageOrAuto::Auto, DUMMY_SP);
        let size = context.alloc_encoded_node(
            BackgroundSize::Explicit {
                height: auto,
                width: auto,
            },
            DUMMY_SP,
        );
        context.mutate_encoded_node(size, |value, _| *value = BackgroundSize::Cover);
        assert_eq!(context.encoded_node(size), BackgroundSize::Cover);
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundRepeatKeyword {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Border,
    Text,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}
