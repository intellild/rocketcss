use crate::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum LineStyle {
    None,
    Hidden,
    Inset,
    Groove,
    Outset,
    Ridge,
    Dotted,
    Dashed,
    Solid,
    Double,
}

pub(crate) fn encode_line_style(value: LineStyle) -> u8 {
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

pub(crate) fn decode_line_style(value: u8) -> LineStyle {
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

#[derive(Debug, PartialEq, Visit)]
pub enum BorderSideWidth<'a> {
    Thin,
    Medium,
    Thick,
    Length(NodeId<'a, Length<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for BorderSideWidth<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0009_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Thin,
            1 => Self::Medium,
            2 => Self::Thick,
            3 => Self::Length(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded BorderSideWidth variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_border_side_width(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_border_side_width(self)
    }
}

fn encode_border_side_width(value: BorderSideWidth<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        BorderSideWidth::Thin => bytes[0] = 0,
        BorderSideWidth::Medium => bytes[0] = 1,
        BorderSideWidth::Thick => bytes[0] = 2,
        BorderSideWidth::Length(value) => write_node_id(&mut bytes, 3, value),
    }
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum LengthOrNumber<'a> {
    Number(f32),
    Length(NodeId<'a, Length<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for LengthOrNumber<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0009_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Number(f32::from_bits(read_u32(&bytes, 4))),
            1 => Self::Length(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded LengthOrNumber variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_length_or_number(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_length_or_number(self)
    }
}

fn encode_length_or_number(value: LengthOrNumber<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        LengthOrNumber::Number(value) => {
            bytes[0] = 0;
            write_u32(&mut bytes, 4, value.to_bits());
        }
        LengthOrNumber::Length(value) => write_node_id(&mut bytes, 1, value),
    }
    NodePayload::inline(&bytes)
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[derive(Debug, PartialEq, Visit)]
pub enum BorderImageSideWidth<'a> {
    Number(f32),
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    Auto,
}

impl<'ast> AstNodeStorage<'ast> for BorderImageSideWidth<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0009_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Number(f32::from_bits(read_u32(&bytes, 4))),
            1 => Self::LengthPercentage(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            2 => Self::Auto,
            _ => panic!("invalid encoded BorderImageSideWidth variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_border_image_side_width(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_border_image_side_width(self)
    }
}

fn encode_border_image_side_width(value: BorderImageSideWidth<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        BorderImageSideWidth::Number(value) => {
            bytes[0] = 0;
            write_u32(&mut bytes, 4, value.to_bits());
        }
        BorderImageSideWidth::LengthPercentage(value) => write_node_id(&mut bytes, 1, value),
        BorderImageSideWidth::Auto => bytes[0] = 2,
    }
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum OutlineStyle {
    Auto,
    LineStyle(LineStyle),
}

fn write_node_id<T>(bytes: &mut [u8], tag: u8, id: NodeId<'_, T>) {
    bytes[0] = tag;
    write_u32(
        bytes,
        4,
        u32::try_from(id.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact border field is four bytes"),
    )
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, BorderImageSideWidth, BorderSideWidth, DUMMY_SP, DimensionPercentage, Length,
        LengthOrNumber, LengthUnit, LengthValue,
    };

    #[test]
    fn border_scalar_node_codecs_preserve_numbers_and_child_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let length = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 2.0,
            }),
            DUMMY_SP,
        );
        let width = context.alloc_encoded_node(BorderSideWidth::Length(length), DUMMY_SP);
        assert_eq!(context.encoded_node(width), BorderSideWidth::Length(length));

        let outset = context.alloc_encoded_node(LengthOrNumber::Number(1.25), DUMMY_SP);
        assert_eq!(context.encoded_node(outset), LengthOrNumber::Number(1.25));

        let percentage =
            context.alloc_encoded_node(DimensionPercentage::Percentage(40.0), DUMMY_SP);
        let image_width = context
            .alloc_encoded_node(BorderImageSideWidth::LengthPercentage(percentage), DUMMY_SP);
        assert_eq!(
            context.encoded_node(image_width),
            BorderImageSideWidth::LengthPercentage(percentage)
        );
    }
}
