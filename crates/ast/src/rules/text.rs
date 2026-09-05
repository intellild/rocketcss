use crate::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub struct TextTransform {
    pub case: TextTransformCase,
    pub full_size_kana: bool,
    pub full_width: bool,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextIndent<'a> {
    pub each_line: bool,
    pub hanging: bool,
    pub value: NodeId<'a, LengthPercentage<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for TextIndent<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0011_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            each_line: decode_bool(bytes[0]),
            hanging: decode_bool(bytes[1]),
            value: context
                .encoded_node_id_at(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = self.each_line as u8;
        bytes[1] = self.hanging as u8;
        bytes[4..8].copy_from_slice(
            &u32::try_from(self.value.index())
                .expect("AST node ID exceeds four bytes")
                .to_le_bytes(),
        );
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

fn decode_bool(value: u8) -> bool {
    match value {
        0 => false,
        1 => true,
        _ => panic!("invalid encoded bool"),
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextDecoration<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub line: NodeId<'a, TextDecorationLine<'a>>,
    pub style: TextDecorationStyle,
    pub thickness: NodeId<'a, TextDecorationThickness<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasis<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub style: NodeId<'a, TextEmphasisStyle<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasisPosition {
    pub horizontal: TextEmphasisPositionHorizontal,
    pub vertical: TextEmphasisPositionVertical,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextShadow<'a> {
    pub blur: NodeId<'a, Length<'a>>,
    pub color: NodeId<'a, CssColor<'a>>,
    pub spread: NodeId<'a, Length<'a>>,
    pub x_offset: NodeId<'a, Length<'a>>,
    pub y_offset: NodeId<'a, Length<'a>>,
}
