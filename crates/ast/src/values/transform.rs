use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

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

fn encode_number_or_percentage(
    value: NumberOrPercentage,
    bytes: &mut [u8],
    tag_offset: usize,
    value_offset: usize,
) {
    let (tag, value) = match value {
        NumberOrPercentage::Number(value) => (0, value),
        NumberOrPercentage::Percentage(value) => (1, value),
    };
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
        AstContext, DUMMY_SP, DimensionPercentage, Length, LengthUnit, LengthValue,
        NumberOrPercentage, Perspective, Scale, Translate,
    };

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
