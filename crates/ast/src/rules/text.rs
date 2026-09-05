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

impl<'ast> AstNodeClone<'ast> for TextIndent<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            each_line: self.each_line,
            hanging: self.hanging,
            value: context.clone_encoded_node(self.value),
        }
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

impl<'ast> AstNodeStorage<'ast> for TextDecoration<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0011_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            color: read_node_id(&bytes, 4, context),
            line: read_node_id(&bytes, 8, context),
            style: decode_decoration_style(bytes[0]),
            thickness: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = encode_decoration_style(self.style);
        write_node_id(&mut bytes, 4, self.color);
        write_node_id(&mut bytes, 8, self.line);
        write_node_id(&mut bytes, 12, self.thickness);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for TextDecoration<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            color: context.clone_encoded_node(self.color),
            line: context.clone_encoded_node(self.line),
            style: self.style,
            thickness: context.clone_encoded_node(self.thickness),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct TextEmphasis<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub style: NodeId<'a, TextEmphasisStyle<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for TextEmphasis<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0011_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            color: read_node_id(&bytes, 0, context),
            style: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        write_node_id(&mut bytes, 0, self.color);
        write_node_id(&mut bytes, 4, self.style);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for TextEmphasis<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            color: context.clone_encoded_node(self.color),
            style: context.clone_encoded_node(self.style),
        }
    }
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

impl<'ast> AstNodeStorage<'ast> for TextShadow<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0011_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let offsets = context.extra_slot(payload.extra_start()).as_u64();
        Self {
            blur: read_node_id(&bytes, 0, context),
            color: read_node_id(&bytes, 4, context),
            spread: context.encoded_node_id_at((offsets >> 32) as u32 as usize),
            x_offset: read_node_id(&bytes, 8, context),
            y_offset: context.encoded_node_id_at(offsets as u32 as usize),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_text_shadow(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_text_shadow(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for TextShadow<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            blur: context.clone_encoded_node(self.blur),
            color: context.clone_encoded_node(self.color),
            spread: context.clone_encoded_node(self.spread),
            x_offset: context.clone_encoded_node(self.x_offset),
            y_offset: context.clone_encoded_node(self.y_offset),
        }
    }
}

fn encode_text_shadow<'ast>(
    value: TextShadow<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    write_node_id(&mut bytes, 0, value.blur);
    write_node_id(&mut bytes, 4, value.color);
    write_node_id(&mut bytes, 8, value.x_offset);
    let offsets =
        ExtraData::from_u64(value.y_offset.index() as u64 | (value.spread.index() as u64) << 32);
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, offsets);
            extra
        }
        None => context.alloc_extra_slots([offsets]),
    };
    NodePayload::with_extra(&bytes, extra)
}

fn write_node_id<T>(bytes: &mut [u8], offset: usize, value: NodeId<'_, T>) {
    bytes[offset..offset + 4].copy_from_slice(
        &u32::try_from(value.index())
            .expect("AST node ID exceeds four bytes")
            .to_le_bytes(),
    );
}

fn read_node_id<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> NodeId<'ast, T> {
    context.encoded_node_id_at(u32::from_le_bytes(
        bytes[offset..offset + 4].try_into().expect("u32 field"),
    ) as usize)
}

fn encode_decoration_style(value: TextDecorationStyle) -> u8 {
    match value {
        TextDecorationStyle::Solid => 0,
        TextDecorationStyle::Double => 1,
        TextDecorationStyle::Dotted => 2,
        TextDecorationStyle::Dashed => 3,
        TextDecorationStyle::Wavy => 4,
    }
}

fn decode_decoration_style(value: u8) -> TextDecorationStyle {
    match value {
        0 => TextDecorationStyle::Solid,
        1 => TextDecorationStyle::Double,
        2 => TextDecorationStyle::Dotted,
        3 => TextDecorationStyle::Dashed,
        4 => TextDecorationStyle::Wavy,
        _ => panic!("invalid encoded TextDecorationStyle"),
    }
}
