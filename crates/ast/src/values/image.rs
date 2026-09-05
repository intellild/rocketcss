use crate::*;

use crate::{
    AstNodeClone, AstNodeStorage, ExtraData, ExtraDataClone, ExtraDataCompact, NodeKind,
    NodePayload,
};

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

impl<'ast> AstNodeClone<'ast> for Image<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Gradient(value) => Self::Gradient(context.clone_encoded_node(value)),
            Self::ImageSet(value) => Self::ImageSet(context.clone_encoded_node(value)),
        }
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

impl<'ast> ExtraDataClone<'ast> for Image<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_in_context(context)
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
        items: Vec<'a, NodeId<'a, GradientItem<'a, LengthValue>>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingLinear {
        direction: LineDirection,
        items: Vec<'a, NodeId<'a, GradientItem<'a, LengthValue>>>,
        vendor_prefix: VendorPrefix,
    },
    Radial {
        items: Vec<'a, NodeId<'a, GradientItem<'a, LengthValue>>>,
        position: NodeId<'a, Position<'a>>,
        shape: NodeId<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    RepeatingRadial {
        items: Vec<'a, NodeId<'a, GradientItem<'a, LengthValue>>>,
        position: NodeId<'a, Position<'a>>,
        shape: NodeId<'a, EndingShape<'a>>,
        vendor_prefix: VendorPrefix,
    },
    Conic {
        angle: Angle,
        items: Vec<'a, NodeId<'a, GradientItem<'a, Angle>>>,
        position: NodeId<'a, Position<'a>>,
    },
    RepeatingConic {
        angle: Angle,
        items: Vec<'a, NodeId<'a, GradientItem<'a, Angle>>>,
        position: NodeId<'a, Position<'a>>,
    },
    WebKitGradient(NodeId<'a, WebKitGradient<'a>>),
}

// byte 0       variant
// bytes 1..4   vendor/direction/angle tags
// bytes 4..12  direction data, child IDs, or one angle plus one ID
// bytes 12..16 first extra slot
//
// extra + 0    gradient-item range
impl<'ast> AstNodeStorage<'ast> for Gradient<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_000e);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let items = context.extra_slot(payload.extra_start());
        match bytes[0] {
            0 => Self::Linear {
                direction: decode_line_direction(&bytes),
                items: decode_extra_range(items, context),
                vendor_prefix: VendorPrefix::from_bits_retain(bytes[1]),
            },
            1 => Self::RepeatingLinear {
                direction: decode_line_direction(&bytes),
                items: decode_extra_range(items, context),
                vendor_prefix: VendorPrefix::from_bits_retain(bytes[1]),
            },
            2 => Self::Radial {
                items: decode_extra_range(items, context),
                position: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
                shape: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
                vendor_prefix: VendorPrefix::from_bits_retain(bytes[1]),
            },
            3 => Self::RepeatingRadial {
                items: decode_extra_range(items, context),
                position: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
                shape: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
                vendor_prefix: VendorPrefix::from_bits_retain(bytes[1]),
            },
            4 => Self::Conic {
                angle: crate::token::decode_angle(bytes[1], f32::from_bits(read_u32(&bytes, 4))),
                items: decode_extra_range(items, context),
                position: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            },
            5 => Self::RepeatingConic {
                angle: crate::token::decode_angle(bytes[1], f32::from_bits(read_u32(&bytes, 4))),
                items: decode_extra_range(items, context),
                position: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            },
            6 => Self::WebKitGradient(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded Gradient variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_gradient(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_gradient(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for Gradient<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Linear {
                direction,
                items,
                vendor_prefix,
            } => Self::Linear {
                direction,
                items: context.clone_encoded_vec(items),
                vendor_prefix,
            },
            Self::RepeatingLinear {
                direction,
                items,
                vendor_prefix,
            } => Self::RepeatingLinear {
                direction,
                items: context.clone_encoded_vec(items),
                vendor_prefix,
            },
            Self::Radial {
                items,
                position,
                shape,
                vendor_prefix,
            } => Self::Radial {
                items: context.clone_encoded_vec(items),
                position: context.clone_encoded_node(position),
                shape: context.clone_encoded_node(shape),
                vendor_prefix,
            },
            Self::RepeatingRadial {
                items,
                position,
                shape,
                vendor_prefix,
            } => Self::RepeatingRadial {
                items: context.clone_encoded_vec(items),
                position: context.clone_encoded_node(position),
                shape: context.clone_encoded_node(shape),
                vendor_prefix,
            },
            Self::Conic {
                angle,
                items,
                position,
            } => Self::Conic {
                angle,
                items: context.clone_encoded_vec(items),
                position: context.clone_encoded_node(position),
            },
            Self::RepeatingConic {
                angle,
                items,
                position,
            } => Self::RepeatingConic {
                angle,
                items: context.clone_encoded_vec(items),
                position: context.clone_encoded_node(position),
            },
            Self::WebKitGradient(value) => Self::WebKitGradient(context.clone_encoded_node(value)),
        }
    }
}

fn encode_gradient<'ast>(
    value: Gradient<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    let items = match value {
        Gradient::Linear {
            direction,
            items,
            vendor_prefix,
        } => {
            bytes[0] = 0;
            bytes[1] = vendor_prefix.bits();
            encode_line_direction(direction, &mut bytes);
            encode_extra_range(items)
        }
        Gradient::RepeatingLinear {
            direction,
            items,
            vendor_prefix,
        } => {
            bytes[0] = 1;
            bytes[1] = vendor_prefix.bits();
            encode_line_direction(direction, &mut bytes);
            encode_extra_range(items)
        }
        Gradient::Radial {
            items,
            position,
            shape,
            vendor_prefix,
        } => {
            bytes[0] = 2;
            bytes[1] = vendor_prefix.bits();
            write_u32(&mut bytes, 4, node_index(position));
            write_u32(&mut bytes, 8, node_index(shape));
            encode_extra_range(items)
        }
        Gradient::RepeatingRadial {
            items,
            position,
            shape,
            vendor_prefix,
        } => {
            bytes[0] = 3;
            bytes[1] = vendor_prefix.bits();
            write_u32(&mut bytes, 4, node_index(position));
            write_u32(&mut bytes, 8, node_index(shape));
            encode_extra_range(items)
        }
        Gradient::Conic {
            angle,
            items,
            position,
        } => {
            bytes[0] = 4;
            encode_gradient_angle(angle, &mut bytes);
            write_u32(&mut bytes, 8, node_index(position));
            encode_extra_range(items)
        }
        Gradient::RepeatingConic {
            angle,
            items,
            position,
        } => {
            bytes[0] = 5;
            encode_gradient_angle(angle, &mut bytes);
            write_u32(&mut bytes, 8, node_index(position));
            encode_extra_range(items)
        }
        Gradient::WebKitGradient(value) => {
            bytes[0] = 6;
            write_u32(&mut bytes, 4, node_index(value));
            ExtraData::default()
        }
    };
    let extra = match existing_extra {
        Some(index) => {
            context.set_extra_slot(index, items);
            index
        }
        None => context.alloc_extra_slots([items]),
    };
    NodePayload::with_extra(&bytes, extra)
}

fn encode_gradient_angle(value: Angle, bytes: &mut [u8]) {
    let (kind, value) = crate::token::encode_angle(value);
    bytes[1] = kind;
    write_u32(bytes, 4, value.to_bits());
}

fn encode_line_direction(value: LineDirection, bytes: &mut [u8]) {
    match value {
        LineDirection::Angle(value) => {
            bytes[2] = 0;
            let (kind, value) = crate::token::encode_angle(value);
            bytes[3] = kind;
            write_u32(bytes, 4, value.to_bits());
        }
        LineDirection::Horizontal(value) => {
            bytes[2] = 1;
            bytes[3] = encode_horizontal_position(value);
        }
        LineDirection::Vertical(value) => {
            bytes[2] = 2;
            bytes[3] = encode_vertical_position(value);
        }
        LineDirection::Corner {
            horizontal,
            vertical,
        } => {
            bytes[2] = 3;
            bytes[3] = encode_horizontal_position(horizontal);
            bytes[4] = encode_vertical_position(vertical);
        }
    }
}

fn decode_line_direction(bytes: &[u8]) -> LineDirection {
    match bytes[2] {
        0 => LineDirection::Angle(crate::token::decode_angle(
            bytes[3],
            f32::from_bits(read_u32(bytes, 4)),
        )),
        1 => LineDirection::Horizontal(decode_horizontal_position(bytes[3])),
        2 => LineDirection::Vertical(decode_vertical_position(bytes[3])),
        3 => LineDirection::Corner {
            horizontal: decode_horizontal_position(bytes[3]),
            vertical: decode_vertical_position(bytes[4]),
        },
        _ => panic!("invalid encoded LineDirection variant"),
    }
}

fn encode_horizontal_position(value: HorizontalPositionKeyword) -> u8 {
    match value {
        HorizontalPositionKeyword::Left => 0,
        HorizontalPositionKeyword::Right => 1,
    }
}

fn decode_horizontal_position(value: u8) -> HorizontalPositionKeyword {
    match value {
        0 => HorizontalPositionKeyword::Left,
        1 => HorizontalPositionKeyword::Right,
        _ => panic!("invalid encoded horizontal position"),
    }
}

fn encode_vertical_position(value: VerticalPositionKeyword) -> u8 {
    match value {
        VerticalPositionKeyword::Top => 0,
        VerticalPositionKeyword::Bottom => 1,
    }
}

fn decode_vertical_position(value: u8) -> VerticalPositionKeyword {
    match value {
        0 => VerticalPositionKeyword::Top,
        1 => VerticalPositionKeyword::Bottom,
        _ => panic!("invalid encoded vertical position"),
    }
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

// byte 0       variant
// bytes 1..4   reserved
// bytes 4..8   from point ID
// bytes 8..12  to point ID
// bytes 12..16 first extra slot
//
// extra + 0    start/end radii, or reserved
// extra + 1    color-stop range
impl<'ast> AstNodeStorage<'ast> for WebKitGradient<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0003_000f);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let extra = payload.extra_start();
        let from = context.encoded_node_id_at(read_u32(&bytes, 4) as usize);
        let to = context.encoded_node_id_at(read_u32(&bytes, 8) as usize);
        let stops = decode_extra_range(context.extra_slot(extra + 1), context);
        match bytes[0] {
            0 => Self::Linear { from, to, stops },
            1 => {
                let radii = context.extra_slot(extra).bytes();
                Self::Radial {
                    from,
                    start_radius: f32::from_bits(read_u32(&radii, 0)),
                    to,
                    end_radius: f32::from_bits(read_u32(&radii, 4)),
                    stops,
                }
            }
            _ => panic!("invalid encoded WebKitGradient variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_webkit_gradient(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_webkit_gradient(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for WebKitGradient<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Linear { from, to, stops } => Self::Linear {
                from: context.clone_encoded_node(from),
                to: context.clone_encoded_node(to),
                stops: context.clone_encoded_vec(stops),
            },
            Self::Radial {
                from,
                start_radius,
                to,
                end_radius,
                stops,
            } => Self::Radial {
                from: context.clone_encoded_node(from),
                start_radius,
                to: context.clone_encoded_node(to),
                end_radius,
                stops: context.clone_encoded_vec(stops),
            },
        }
    }
}

fn encode_webkit_gradient<'ast>(
    value: WebKitGradient<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    let (radii, stops) = match value {
        WebKitGradient::Linear { from, to, stops } => {
            bytes[0] = 0;
            write_u32(&mut bytes, 4, node_index(from));
            write_u32(&mut bytes, 8, node_index(to));
            (ExtraData::default(), encode_extra_range(stops))
        }
        WebKitGradient::Radial {
            from,
            start_radius,
            to,
            end_radius,
            stops,
        } => {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, node_index(from));
            write_u32(&mut bytes, 8, node_index(to));
            let mut radii = [0; ExtraData::BYTES];
            write_u32(&mut radii, 0, start_radius.to_bits());
            write_u32(&mut radii, 4, end_radius.to_bits());
            (ExtraData::from_bytes(&radii), encode_extra_range(stops))
        }
    };
    let extra = match existing_extra {
        Some(index) => {
            context.set_extra_slot(index, radii);
            context.set_extra_slot(index + 1, stops);
            index
        }
        None => context.alloc_extra_slots([radii, stops]),
    };
    NodePayload::with_extra(&bytes, extra)
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
pub enum GradientItem<'a, D: DimensionCodec> {
    ColorStop {
        color: NodeId<'a, CssColor<'a>>,
        position: Option<NodeId<'a, DimensionPercentage<'a, D>>>,
    },
    Hint(NodeId<'a, DimensionPercentage<'a, D>>),
}

// byte 0      variant
// bytes 1..4  reserved
// bytes 4..8  color or hint node ID
// bytes 8..12 optional position node ID
// bytes 12..16 reserved
impl<'ast, D: DimensionCodec> AstNodeStorage<'ast> for GradientItem<'ast, D> {
    const KIND: NodeKind = D::GRADIENT_ITEM_KIND;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => {
                let position = read_u32(&bytes, 8);
                Self::ColorStop {
                    color: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
                    position: (position != u32::MAX)
                        .then(|| context.encoded_node_id_at(position as usize)),
                }
            }
            1 => Self::Hint(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded GradientItem variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_gradient_item(self)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast, D: DimensionCodec> AstNodeClone<'ast> for GradientItem<'ast, D> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::ColorStop { color, position } => Self::ColorStop {
                color: context.clone_encoded_node(color),
                position: position.map(|value| context.clone_encoded_node(value)),
            },
            Self::Hint(value) => Self::Hint(context.clone_encoded_node(value)),
        }
    }
}

fn encode_gradient_item<D: DimensionCodec>(value: GradientItem<'_, D>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        GradientItem::ColorStop { color, position } => {
            bytes[0] = 0;
            write_u32(&mut bytes, 4, node_index(color));
            write_u32(&mut bytes, 8, position.map_or(u32::MAX, node_index));
        }
        GradientItem::Hint(value) => {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, node_index(value));
        }
    }
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum DimensionPercentage<'a, D: DimensionCodec> {
    Dimension(D),
    Percentage(f32),
    /// A unitless zero produced by target-aware minification.
    Zero,
    Calc(NodeId<'a, Calc<'a, DimensionPercentage<'a, D>>>),
}

pub type LengthPercentage<'a> = DimensionPercentage<'a, LengthValue>;
pub type AnglePercentage<'a> = DimensionPercentage<'a, Angle>;

#[doc(hidden)]
pub trait DimensionCodec: Sized {
    const NODE_KIND: NodeKind;
    const CALC_KIND: NodeKind;
    const GRADIENT_ITEM_KIND: NodeKind;
    const MATH_FUNCTION_KIND: NodeKind;

    fn encode(self) -> (u8, u32);

    fn decode(kind: u8, value: u32) -> Self;
}

impl DimensionCodec for LengthValue {
    const NODE_KIND: NodeKind = NodeKind::new(0x0003_0002);
    const CALC_KIND: NodeKind = NodeKind::new(0x0018_0002);
    const GRADIENT_ITEM_KIND: NodeKind = NodeKind::new(0x0003_000c);
    const MATH_FUNCTION_KIND: NodeKind = NodeKind::new(0x0019_0002);

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
    const CALC_KIND: NodeKind = NodeKind::new(0x0018_0003);
    const GRADIENT_ITEM_KIND: NodeKind = NodeKind::new(0x0003_000d);
    const MATH_FUNCTION_KIND: NodeKind = NodeKind::new(0x0019_0003);

    fn encode(self) -> (u8, u32) {
        let (kind, value) = crate::token::encode_angle(self);
        (kind, value.to_bits())
    }

    fn decode(kind: u8, value: u32) -> Self {
        crate::token::decode_angle(kind, f32::from_bits(value))
    }
}

impl<D: DimensionCodec> crate::length::CalcValueCodec for DimensionPercentage<'_, D> {
    const CALC_KIND: NodeKind = D::CALC_KIND;
    const MATH_FUNCTION_KIND: NodeKind = D::MATH_FUNCTION_KIND;
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

impl<'ast, D: DimensionCodec> AstNodeClone<'ast> for DimensionPercentage<'ast, D> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Dimension(value) => Self::Dimension(value),
            Self::Percentage(value) => Self::Percentage(value),
            Self::Zero => Self::Zero,
            Self::Calc(value) => Self::Calc(context.clone_encoded_node(value)),
        }
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

impl<'ast, D: DimensionCodec> ExtraDataClone<'ast> for DimensionPercentage<'ast, D> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_in_context(context)
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

impl<'ast, S: PositionSideCodec> AstNodeClone<'ast> for PositionComponent<'ast, S> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Center => Self::Center,
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            Self::Side { offset, side } => Self::Side {
                offset: offset.map(|value| context.clone_encoded_node(value)),
                side,
            },
        }
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

impl<'ast, S: PositionSideCodec> ExtraDataClone<'ast> for PositionComponent<'ast, S> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_in_context(context)
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

impl<'ast> AstNodeClone<'ast> for EndingShape<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Ellipse(value) => Self::Ellipse(context.clone_encoded_node(value)),
            Self::Circle(value) => Self::Circle(context.clone_encoded_node(value)),
        }
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

impl<'ast> AstNodeClone<'ast> for Ellipse<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Size { x, y } => Self::Size {
                x: context.clone_encoded_node(x),
                y: context.clone_encoded_node(y),
            },
            Self::Extent(value) => Self::Extent(value),
        }
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

impl<'ast> AstNodeClone<'ast> for Circle<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Radius(value) => Self::Radius(context.clone_encoded_node(value)),
            Self::Extent(value) => Self::Extent(value),
        }
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

impl AstNodeStorage<'_> for NumberOrPercentage {
    const KIND: NodeKind = NodeKind::new(0x0003_000b);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        let value = f32::from_bits(read_u32(&bytes, 4));
        match bytes[0] {
            0 => Self::Number(value),
            1 => Self::Percentage(value),
            _ => panic!("invalid encoded NumberOrPercentage variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        encode_number_or_percentage(self)
    }

    fn encode_existing(self, _current: NodePayload, _context: &mut AstContext<'_>) -> NodePayload {
        encode_number_or_percentage(self)
    }
}

impl AstNodeClone<'_> for NumberOrPercentage {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_number_or_percentage(value: NumberOrPercentage) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    let (kind, value) = match value {
        NumberOrPercentage::Number(value) => (0, value),
        NumberOrPercentage::Percentage(value) => (1, value),
    };
    bytes[0] = kind;
    write_u32(&mut bytes, 4, value.to_bits());
    NodePayload::inline(&bytes)
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

impl<'ast> AstNodeClone<'ast> for BackgroundSize<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Explicit { height, width } => Self::Explicit {
                height: context.clone_encoded_node(height),
                width: context.clone_encoded_node(width),
            },
            value => value,
        }
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

impl<'ast> AstNodeClone<'ast> for LengthPercentageOrAuto<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Auto => Self::Auto,
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
        }
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

fn encode_extra_range<T>(range: Vec<'_, T>) -> ExtraData {
    let start = u32::try_from(range.start_index()).expect("AST range exceeds four bytes");
    let end = u32::try_from(range.end_index()).expect("AST range exceeds four bytes");
    ExtraData::from_u64((end as u64) << 32 | start as u64)
}

fn decode_extra_range<'ast, T>(data: ExtraData, context: &AstContext<'ast>) -> Vec<'ast, T> {
    context.encoded_vec_range(
        data.as_u64() as u32 as usize,
        (data.as_u64() >> 32) as u32 as usize,
    )
}

fn inline_payload_as_extra(payload: NodePayload) -> ExtraData {
    ExtraData::from_bytes(&payload.bytes()[..ExtraData::BYTES])
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, BackgroundSize, Circle, CssColor, DUMMY_SP, DimensionPercentage, Ellipse,
        EndingShape, Gradient, GradientItem, HorizontalPositionKeyword, Image,
        LengthPercentageOrAuto, LineDirection, PositionComponent, ShapeExtent, Url, VendorPrefix,
        VerticalPositionKeyword,
    };

    #[test]
    fn gradient_codec_deep_clones_promoted_item_nodes() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let color = context.alloc_encoded_node(CssColor::CurrentColor, DUMMY_SP);
        let position = context.alloc_encoded_node(DimensionPercentage::Percentage(25.0), DUMMY_SP);
        let item = context.alloc_encoded_node(
            GradientItem::ColorStop {
                color,
                position: Some(position),
            },
            DUMMY_SP,
        );
        let items = context.alloc_encoded_vec([item].into_iter());
        let before = context.encoded_extra_len();
        let gradient = context.alloc_encoded_node(
            Gradient::Linear {
                direction: LineDirection::Corner {
                    horizontal: HorizontalPositionKeyword::Right,
                    vertical: VerticalPositionKeyword::Top,
                },
                items,
                vendor_prefix: VendorPrefix::WEBKIT,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 1);
        assert_eq!(
            context.encoded_node(gradient),
            Gradient::Linear {
                direction: LineDirection::Corner {
                    horizontal: HorizontalPositionKeyword::Right,
                    vertical: VerticalPositionKeyword::Top,
                },
                items,
                vendor_prefix: VendorPrefix::WEBKIT,
            }
        );

        let cloned = context.clone_encoded_node(gradient);
        let Gradient::Linear {
            items: cloned_items,
            ..
        } = context.encoded_node(cloned)
        else {
            panic!("expected linear gradient")
        };
        assert_ne!(cloned_items, items);
        assert_ne!(context.encoded_vec_get(cloned_items, 0), Some(item));
    }

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

impl ExtraDataCompact<'_> for BackgroundAttachment {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Scroll => 0,
            Self::Fixed => 1,
            Self::Local => 2,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Scroll,
            1 => Self::Fixed,
            2 => Self::Local,
            _ => panic!("invalid encoded BackgroundAttachment"),
        }
    }
}

impl ExtraDataClone<'_> for BackgroundAttachment {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Border,
    Text,
}

impl ExtraDataCompact<'_> for BackgroundClip {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::BorderBox => 0,
            Self::PaddingBox => 1,
            Self::ContentBox => 2,
            Self::Border => 3,
            Self::Text => 4,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::BorderBox,
            1 => Self::PaddingBox,
            2 => Self::ContentBox,
            3 => Self::Border,
            4 => Self::Text,
            _ => panic!("invalid encoded BackgroundClip"),
        }
    }
}

impl ExtraDataClone<'_> for BackgroundClip {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}

impl ExtraDataCompact<'_> for BackgroundOrigin {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::BorderBox => 0,
            Self::PaddingBox => 1,
            Self::ContentBox => 2,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::BorderBox,
            1 => Self::PaddingBox,
            2 => Self::ContentBox,
            _ => panic!("invalid encoded BackgroundOrigin"),
        }
    }
}

impl ExtraDataClone<'_> for BackgroundOrigin {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}
