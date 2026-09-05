use crate::*;

use crate::{AstNodeClone, AstNodeStorage, ExtraData, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub enum Transform<'a> {
    Translate(
        (
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, LengthPercentage<'a>>,
        ),
    ),
    TranslateX(NodeId<'a, LengthPercentage<'a>>),
    TranslateY(NodeId<'a, LengthPercentage<'a>>),
    TranslateZ(NodeId<'a, Length<'a>>),
    Translate3d(
        (
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, Length<'a>>,
        ),
    ),
    Scale((NumberOrPercentage, NumberOrPercentage)),
    ScaleX(NumberOrPercentage),
    ScaleY(NumberOrPercentage),
    ScaleZ(NumberOrPercentage),
    Scale3d((NumberOrPercentage, NumberOrPercentage, NumberOrPercentage)),
    Rotate(Angle),
    RotateX(Angle),
    RotateY(Angle),
    RotateZ(Angle),
    Rotate3d((f32, f32, f32, Angle)),
    Skew((Angle, Angle)),
    SkewX(Angle),
    SkewY(Angle),
    Perspective(NodeId<'a, Length<'a>>),
    Matrix(NodeId<'a, MatrixForFloat>),
    Matrix3d(NodeId<'a, Matrix3DForFloat>),
}

// byte 0       variant
// bytes 1..4   scalar subtype/unit tags
// bytes 4..12  first two scalar values or child IDs
// bytes 12..16 first extra slot
//
// extra + 0    third/fourth scalar values or third child ID
impl<'ast> AstNodeStorage<'ast> for Transform<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000f_0006);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let extra = context.extra_slot(payload.extra_start()).bytes();
        let angle =
            |tag, offset| crate::token::decode_angle(tag, f32::from_bits(read_u32(&bytes, offset)));
        match bytes[0] {
            0 => Self::Translate((
                read_node_id(context, &bytes, 4),
                read_node_id(context, &bytes, 8),
            )),
            1 => Self::TranslateX(read_node_id(context, &bytes, 4)),
            2 => Self::TranslateY(read_node_id(context, &bytes, 4)),
            3 => Self::TranslateZ(read_node_id(context, &bytes, 4)),
            4 => Self::Translate3d((
                read_node_id(context, &bytes, 4),
                read_node_id(context, &bytes, 8),
                read_node_id(context, &extra, 0),
            )),
            5 => Self::Scale((
                decode_number_or_percentage(bytes[1], read_u32(&bytes, 4)),
                decode_number_or_percentage(bytes[2], read_u32(&bytes, 8)),
            )),
            6 => Self::ScaleX(decode_number_or_percentage(bytes[1], read_u32(&bytes, 4))),
            7 => Self::ScaleY(decode_number_or_percentage(bytes[1], read_u32(&bytes, 4))),
            8 => Self::ScaleZ(decode_number_or_percentage(bytes[1], read_u32(&bytes, 4))),
            9 => Self::Scale3d((
                decode_number_or_percentage(bytes[1], read_u32(&bytes, 4)),
                decode_number_or_percentage(bytes[2], read_u32(&bytes, 8)),
                decode_number_or_percentage(bytes[3], read_u32(&extra, 0)),
            )),
            10 => Self::Rotate(angle(bytes[1], 4)),
            11 => Self::RotateX(angle(bytes[1], 4)),
            12 => Self::RotateY(angle(bytes[1], 4)),
            13 => Self::RotateZ(angle(bytes[1], 4)),
            14 => Self::Rotate3d((
                f32::from_bits(read_u32(&bytes, 4)),
                f32::from_bits(read_u32(&bytes, 8)),
                f32::from_bits(read_u32(&extra, 0)),
                crate::token::decode_angle(bytes[1], f32::from_bits(read_u32(&extra, 4))),
            )),
            15 => Self::Skew((angle(bytes[1], 4), angle(bytes[2], 8))),
            16 => Self::SkewX(angle(bytes[1], 4)),
            17 => Self::SkewY(angle(bytes[1], 4)),
            18 => Self::Perspective(read_node_id(context, &bytes, 4)),
            19 => Self::Matrix(read_node_id(context, &bytes, 4)),
            20 => Self::Matrix3d(read_node_id(context, &bytes, 4)),
            _ => panic!("invalid encoded Transform variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_transform(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_transform(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for Transform<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Translate((x, y)) => {
                Self::Translate((context.clone_encoded_node(x), context.clone_encoded_node(y)))
            }
            Self::TranslateX(value) => Self::TranslateX(context.clone_encoded_node(value)),
            Self::TranslateY(value) => Self::TranslateY(context.clone_encoded_node(value)),
            Self::TranslateZ(value) => Self::TranslateZ(context.clone_encoded_node(value)),
            Self::Translate3d((x, y, z)) => Self::Translate3d((
                context.clone_encoded_node(x),
                context.clone_encoded_node(y),
                context.clone_encoded_node(z),
            )),
            Self::Scale(value) => Self::Scale(value),
            Self::ScaleX(value) => Self::ScaleX(value),
            Self::ScaleY(value) => Self::ScaleY(value),
            Self::ScaleZ(value) => Self::ScaleZ(value),
            Self::Scale3d(value) => Self::Scale3d(value),
            Self::Rotate(value) => Self::Rotate(value),
            Self::RotateX(value) => Self::RotateX(value),
            Self::RotateY(value) => Self::RotateY(value),
            Self::RotateZ(value) => Self::RotateZ(value),
            Self::Rotate3d(value) => Self::Rotate3d(value),
            Self::Skew(value) => Self::Skew(value),
            Self::SkewX(value) => Self::SkewX(value),
            Self::SkewY(value) => Self::SkewY(value),
            Self::Perspective(value) => Self::Perspective(context.clone_encoded_node(value)),
            Self::Matrix(value) => Self::Matrix(context.clone_encoded_node(value)),
            Self::Matrix3d(value) => Self::Matrix3d(context.clone_encoded_node(value)),
        }
    }
}

fn encode_transform<'ast>(
    value: Transform<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    let mut extra = [0; ExtraData::BYTES];
    match value {
        Transform::Translate((x, y)) => {
            bytes[0] = 0;
            write_id_at(&mut bytes, 4, x);
            write_id_at(&mut bytes, 8, y);
        }
        Transform::TranslateX(value) => write_node_id(&mut bytes, 1, value),
        Transform::TranslateY(value) => write_node_id(&mut bytes, 2, value),
        Transform::TranslateZ(value) => write_node_id(&mut bytes, 3, value),
        Transform::Translate3d((x, y, z)) => {
            bytes[0] = 4;
            write_id_at(&mut bytes, 4, x);
            write_id_at(&mut bytes, 8, y);
            write_id_at(&mut extra, 0, z);
        }
        Transform::Scale((x, y)) => {
            bytes[0] = 5;
            encode_number_or_percentage(x, &mut bytes, 1, 4);
            encode_number_or_percentage(y, &mut bytes, 2, 8);
        }
        Transform::ScaleX(value) => {
            bytes[0] = 6;
            encode_number_or_percentage(value, &mut bytes, 1, 4);
        }
        Transform::ScaleY(value) => {
            bytes[0] = 7;
            encode_number_or_percentage(value, &mut bytes, 1, 4);
        }
        Transform::ScaleZ(value) => {
            bytes[0] = 8;
            encode_number_or_percentage(value, &mut bytes, 1, 4);
        }
        Transform::Scale3d((x, y, z)) => {
            bytes[0] = 9;
            encode_number_or_percentage(x, &mut bytes, 1, 4);
            encode_number_or_percentage(y, &mut bytes, 2, 8);
            let (tag, value) = split_number_or_percentage(z);
            bytes[3] = tag;
            write_u32(&mut extra, 0, value.to_bits());
        }
        Transform::Rotate(value) => encode_transform_angle(&mut bytes, 10, value),
        Transform::RotateX(value) => encode_transform_angle(&mut bytes, 11, value),
        Transform::RotateY(value) => encode_transform_angle(&mut bytes, 12, value),
        Transform::RotateZ(value) => encode_transform_angle(&mut bytes, 13, value),
        Transform::Rotate3d((x, y, z, angle)) => {
            bytes[0] = 14;
            write_u32(&mut bytes, 4, x.to_bits());
            write_u32(&mut bytes, 8, y.to_bits());
            write_u32(&mut extra, 0, z.to_bits());
            let (kind, value) = crate::token::encode_angle(angle);
            bytes[1] = kind;
            write_u32(&mut extra, 4, value.to_bits());
        }
        Transform::Skew((x, y)) => {
            bytes[0] = 15;
            let (x_kind, x) = crate::token::encode_angle(x);
            let (y_kind, y) = crate::token::encode_angle(y);
            bytes[1] = x_kind;
            bytes[2] = y_kind;
            write_u32(&mut bytes, 4, x.to_bits());
            write_u32(&mut bytes, 8, y.to_bits());
        }
        Transform::SkewX(value) => encode_transform_angle(&mut bytes, 16, value),
        Transform::SkewY(value) => encode_transform_angle(&mut bytes, 17, value),
        Transform::Perspective(value) => write_node_id(&mut bytes, 18, value),
        Transform::Matrix(value) => write_node_id(&mut bytes, 19, value),
        Transform::Matrix3d(value) => write_node_id(&mut bytes, 20, value),
    }
    let slot = ExtraData::from_bytes(&extra);
    let extra = match existing_extra {
        Some(index) => {
            context.set_extra_slot(index, slot);
            index
        }
        None => context.alloc_extra_slots([slot]),
    };
    NodePayload::with_extra(&bytes, extra)
}

fn encode_transform_angle(bytes: &mut [u8], tag: u8, angle: Angle) {
    let (kind, value) = crate::token::encode_angle(angle);
    bytes[0] = tag;
    bytes[1] = kind;
    write_u32(bytes, 4, value.to_bits());
}

fn split_number_or_percentage(value: NumberOrPercentage) -> (u8, f32) {
    match value {
        NumberOrPercentage::Number(value) => (0, value),
        NumberOrPercentage::Percentage(value) => (1, value),
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TransformStyle {
    Flat,
    Preserve3d,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TransformBox {
    ContentBox,
    BorderBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Perspective<'a> {
    None,
    Length(NodeId<'a, Length<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for Perspective<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000f_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Length(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded Perspective variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::None => bytes[0] = 0,
            Self::Length(value) => write_node_id(&mut bytes, 1, value),
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum Translate<'a> {
    None,
    Xyz {
        x: NodeId<'a, LengthPercentage<'a>>,
        y: NodeId<'a, LengthPercentage<'a>>,
        z: NodeId<'a, Length<'a>>,
    },
}

impl<'ast> AstNodeStorage<'ast> for Translate<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000f_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Xyz {
                x: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
                y: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
                z: context.encoded_node_id_at(read_u32(&bytes, 12) as usize),
            },
            _ => panic!("invalid encoded Translate variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::None => bytes[0] = 0,
            Self::Xyz { x, y, z } => {
                bytes[0] = 1;
                write_id_at(&mut bytes, 4, x);
                write_id_at(&mut bytes, 8, y);
                write_id_at(&mut bytes, 12, z);
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum Scale {
    None,
    Xyz {
        x: NumberOrPercentage,
        y: NumberOrPercentage,
        z: NumberOrPercentage,
    },
}

impl AstNodeStorage<'_> for Scale {
    const KIND: NodeKind = NodeKind::new(0x000f_0005);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Xyz {
                x: decode_number_or_percentage(bytes[1], read_u32(&bytes, 4)),
                y: decode_number_or_percentage(bytes[2], read_u32(&bytes, 8)),
                z: decode_number_or_percentage(bytes[3], read_u32(&bytes, 12)),
            },
            _ => panic!("invalid encoded Scale variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::None => bytes[0] = 0,
            Self::Xyz { x, y, z } => {
                bytes[0] = 1;
                encode_number_or_percentage(x, &mut bytes, 1, 4);
                encode_number_or_percentage(y, &mut bytes, 2, 8);
                encode_number_or_percentage(z, &mut bytes, 3, 12);
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        self.encode_new(context)
    }
}

impl AstNodeClone<'_> for Scale {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn write_node_id<T>(bytes: &mut [u8], tag: u8, id: NodeId<'_, T>) {
    bytes[0] = tag;
    write_id_at(bytes, 4, id);
}

fn write_id_at<T>(bytes: &mut [u8], offset: usize, id: NodeId<'_, T>) {
    bytes[offset..offset + 4].copy_from_slice(
        &u32::try_from(id.index())
            .expect("AST node ID exceeds four bytes")
            .to_le_bytes(),
    );
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact transform field is four bytes"),
    )
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_node_id<'ast, T>(
    context: &AstContext<'ast>,
    bytes: &[u8],
    offset: usize,
) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, offset) as usize)
}

fn encode_number_or_percentage(
    value: NumberOrPercentage,
    bytes: &mut [u8],
    tag_offset: usize,
    value_offset: usize,
) {
    let (tag, value) = split_number_or_percentage(value);
    bytes[tag_offset] = tag;
    bytes[value_offset..value_offset + 4].copy_from_slice(&value.to_bits().to_le_bytes());
}

fn decode_number_or_percentage(tag: u8, value: u32) -> NumberOrPercentage {
    let value = f32::from_bits(value);
    match tag {
        0 => NumberOrPercentage::Number(value),
        1 => NumberOrPercentage::Percentage(value),
        _ => panic!("invalid encoded NumberOrPercentage variant"),
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        Angle, AstContext, DUMMY_SP, DimensionPercentage, Length, LengthUnit, LengthValue,
        NumberOrPercentage, Perspective, Scale, Transform, Translate,
    };

    #[test]
    fn transform_codec_reuses_overflow_and_deep_clones_child_nodes() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let x = context.alloc_encoded_node(DimensionPercentage::Percentage(10.0), DUMMY_SP);
        let y = context.alloc_encoded_node(DimensionPercentage::Percentage(20.0), DUMMY_SP);
        let z = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 30.0,
            }),
            DUMMY_SP,
        );
        let before = context.encoded_extra_len();
        let transform = context.alloc_encoded_node(Transform::Translate3d((x, y, z)), DUMMY_SP);
        assert_eq!(context.encoded_extra_len(), before + 1);
        assert_eq!(
            context.encoded_node(transform),
            Transform::Translate3d((x, y, z))
        );

        let cloned = context.clone_encoded_node(transform);
        let Transform::Translate3d((cloned_x, cloned_y, cloned_z)) = context.encoded_node(cloned)
        else {
            panic!("expected translate3d")
        };
        assert_ne!((cloned_x, cloned_y, cloned_z), (x, y, z));

        context.mutate_encoded_node(transform, |value, _| {
            *value = Transform::Rotate3d((1.0, 2.0, 3.0, Angle::Turn(0.25)));
        });
        assert_eq!(context.encoded_extra_len(), before + 2);
        assert_eq!(
            context.encoded_node(transform),
            Transform::Rotate3d((1.0, 2.0, 3.0, Angle::Turn(0.25)))
        );
    }

    #[test]
    fn transform_property_node_codecs_preserve_variants_and_child_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let x = context.alloc_encoded_node(DimensionPercentage::Percentage(10.0), DUMMY_SP);
        let y = context.alloc_encoded_node(DimensionPercentage::Percentage(20.0), DUMMY_SP);
        let z = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 30.0,
            }),
            DUMMY_SP,
        );
        let translate = context.alloc_encoded_node(Translate::Xyz { x, y, z }, DUMMY_SP);
        assert_eq!(context.encoded_node(translate), Translate::Xyz { x, y, z });

        let perspective = context.alloc_encoded_node(Perspective::Length(z), DUMMY_SP);
        assert_eq!(context.encoded_node(perspective), Perspective::Length(z));

        let scale = context.alloc_encoded_node(
            Scale::Xyz {
                x: NumberOrPercentage::Number(1.0),
                y: NumberOrPercentage::Percentage(50.0),
                z: NumberOrPercentage::Number(-0.0),
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(scale),
            Scale::Xyz {
                x: NumberOrPercentage::Number(1.0),
                y: NumberOrPercentage::Percentage(50.0),
                z: NumberOrPercentage::Number(-0.0),
            }
        );
    }
}
