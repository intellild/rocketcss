use crate::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub struct BorderRadius<'a> {
    pub bottom_left: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub bottom_right: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub top_left: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub top_right: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderRadius<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom_left: read_node_id(&bytes, 0, context),
            bottom_right: read_node_id(&bytes, 4, context),
            top_left: read_node_id(&bytes, 8, context),
            top_right: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(
            self.bottom_left,
            self.bottom_right,
            self.top_left,
            self.top_right,
        )
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.encode_new(_context)
    }
}

impl<'ast> AstNodeClone<'ast> for BorderRadius<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            bottom_left: context.clone_encoded_node(self.bottom_left),
            bottom_right: context.clone_encoded_node(self.bottom_right),
            top_left: context.clone_encoded_node(self.top_left),
            top_right: context.clone_encoded_node(self.top_right),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImageRepeat {
    pub horizontal: BorderImageRepeatKeyword,
    pub vertical: BorderImageRepeatKeyword,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImageSlice<'a> {
    pub fill: bool,
    pub offsets: NodeId<'a, Rect<'a, NumberOrPercentage>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderImageSlice<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0016);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            fill: bytes[0] != 0,
            offsets: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = self.fill as u8;
        write_u32(&mut bytes, 4, node_index(self.offsets));
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for BorderImageSlice<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            fill: self.fill,
            offsets: context.clone_encoded_node(self.offsets),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderImage<'a> {
    pub outset: NodeId<'a, Rect<'a, LengthOrNumber<'a>>>,
    pub repeat: BorderImageRepeat,
    pub slice: NodeId<'a, BorderImageSlice<'a>>,
    pub source: NodeId<'a, Image<'a>>,
    pub width: NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderImage<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0017);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            outset: read_node_id(&bytes, 4, context),
            repeat: BorderImageRepeat {
                horizontal: decode_border_image_repeat(bytes[0]),
                vertical: decode_border_image_repeat(bytes[1]),
            },
            slice: read_node_id(&bytes, 8, context),
            source: read_node_id(
                &context.extra_slot(payload.extra_start()).bytes(),
                0,
                context,
            ),
            width: read_node_id(
                &context.extra_slot(payload.extra_start()).bytes(),
                4,
                context,
            ),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_border_image(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_border_image(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for BorderImage<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            outset: context.clone_encoded_node(self.outset),
            repeat: self.repeat,
            slice: context.clone_encoded_node(self.slice),
            source: context.clone_encoded_node(self.source),
            width: context.clone_encoded_node(self.width),
        }
    }
}

fn encode_border_image<'ast>(
    value: BorderImage<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    bytes[0] = encode_border_image_repeat(value.repeat.horizontal);
    bytes[1] = encode_border_image_repeat(value.repeat.vertical);
    write_u32(&mut bytes, 4, node_index(value.outset));
    write_u32(&mut bytes, 8, node_index(value.slice));
    let mut extra = [0; ExtraData::BYTES];
    write_u32(&mut extra, 0, node_index(value.source));
    write_u32(&mut extra, 4, node_index(value.width));
    let extra = ExtraData::from_bytes(&extra);
    let extra_start = match existing_extra {
        Some(extra_start) => {
            context.set_extra_slot(extra_start, extra);
            extra_start
        }
        None => context.alloc_extra_slots([extra]),
    };
    NodePayload::with_extra(&bytes, extra_start)
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderColor<'a> {
    pub bottom: NodeId<'a, CssColor<'a>>,
    pub left: NodeId<'a, CssColor<'a>>,
    pub right: NodeId<'a, CssColor<'a>>,
    pub top: NodeId<'a, CssColor<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderColor<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom: read_node_id(&bytes, 0, context),
            left: read_node_id(&bytes, 4, context),
            right: read_node_id(&bytes, 8, context),
            top: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(self.bottom, self.left, self.right, self.top)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.encode_new(_context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderStyle {
    pub bottom: LineStyle,
    pub left: LineStyle,
    pub right: LineStyle,
    pub top: LineStyle,
}

impl AstNodeStorage<'_> for BorderStyle {
    const KIND: NodeKind = NodeKind::new(0x000e_0003);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom: decode_line_style(bytes[0]),
            left: decode_line_style(bytes[1]),
            right: decode_line_style(bytes[2]),
            top: decode_line_style(bytes[3]),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        NodePayload::inline(&[
            encode_line_style(self.bottom),
            encode_line_style(self.left),
            encode_line_style(self.right),
            encode_line_style(self.top),
        ])
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderWidth<'a> {
    pub bottom: NodeId<'a, BorderSideWidth<'a>>,
    pub left: NodeId<'a, BorderSideWidth<'a>>,
    pub right: NodeId<'a, BorderSideWidth<'a>>,
    pub top: NodeId<'a, BorderSideWidth<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderWidth<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            bottom: read_node_id(&bytes, 0, context),
            left: read_node_id(&bytes, 4, context),
            right: read_node_id(&bytes, 8, context),
            top: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_four_ids(self.bottom, self.left, self.right, self.top)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.encode_new(_context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockColor<'a> {
    pub end: NodeId<'a, CssColor<'a>>,
    pub start: NodeId<'a, CssColor<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderBlockColor<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            end: read_node_id(&bytes, 0, context),
            start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.end, self.start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

impl AstNodeStorage<'_> for BorderBlockStyle {
    const KIND: NodeKind = NodeKind::new(0x000e_0006);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        Self {
            end: decode_line_style(bytes[0]),
            start: decode_line_style(bytes[1]),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        NodePayload::inline(&[encode_line_style(self.end), encode_line_style(self.start)])
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderBlockWidth<'a> {
    pub end: NodeId<'a, BorderSideWidth<'a>>,
    pub start: NodeId<'a, BorderSideWidth<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderBlockWidth<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0007);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            end: read_node_id(&bytes, 0, context),
            start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.end, self.start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineColor<'a> {
    pub end: NodeId<'a, CssColor<'a>>,
    pub start: NodeId<'a, CssColor<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderInlineColor<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0008);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            end: read_node_id(&bytes, 0, context),
            start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.end, self.start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

impl AstNodeStorage<'_> for BorderInlineStyle {
    const KIND: NodeKind = NodeKind::new(0x000e_0009);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        Self {
            end: decode_line_style(bytes[0]),
            start: decode_line_style(bytes[1]),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        NodePayload::inline(&[encode_line_style(self.end), encode_line_style(self.start)])
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BorderInlineWidth<'a> {
    pub end: NodeId<'a, BorderSideWidth<'a>>,
    pub start: NodeId<'a, BorderSideWidth<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for BorderInlineWidth<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_000a);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            end: read_node_id(&bytes, 0, context),
            start: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_two_ids(self.end, self.start)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct GenericBorder<'a, S> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub style: S,
    pub width: NodeId<'a, BorderSideWidth<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for GenericBorder<'ast, LineStyle> {
    const KIND: NodeKind = NodeKind::new(0x000e_000b);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            color: read_node_id(&bytes, 0, context),
            style: decode_line_style(bytes[8]),
            width: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = encode_generic_border_ids(self.color, self.width);
        bytes[8] = encode_line_style(self.style);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeStorage<'ast> for GenericBorder<'ast, OutlineStyle> {
    const KIND: NodeKind = NodeKind::new(0x000e_000c);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            color: read_node_id(&bytes, 0, context),
            style: match bytes[8] {
                0 => OutlineStyle::Auto,
                1..=10 => OutlineStyle::LineStyle(decode_line_style(bytes[8] - 1)),
                _ => panic!("invalid encoded OutlineStyle"),
            },
            width: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = encode_generic_border_ids(self.color, self.width);
        bytes[8] = match self.style {
            OutlineStyle::Auto => 0,
            OutlineStyle::LineStyle(value) => encode_line_style(value) + 1,
        };
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

fn encode_generic_border_ids<C, W>(
    color: NodeId<'_, C>,
    width: NodeId<'_, W>,
) -> [u8; NodePayload::INLINE_BYTES] {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_node_id(&mut bytes, 0, color);
    write_node_id(&mut bytes, 4, width);
    bytes
}

fn encode_two_ids<T>(first: NodeId<'_, T>, second: NodeId<'_, T>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_node_id(&mut bytes, 0, first);
    write_node_id(&mut bytes, 4, second);
    NodePayload::inline(&bytes)
}

fn encode_four_ids<T>(
    first: NodeId<'_, T>,
    second: NodeId<'_, T>,
    third: NodeId<'_, T>,
    fourth: NodeId<'_, T>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_node_id(&mut bytes, 0, first);
    write_node_id(&mut bytes, 4, second);
    write_node_id(&mut bytes, 8, third);
    write_node_id(&mut bytes, 12, fourth);
    NodePayload::inline(&bytes)
}

fn write_node_id<T>(bytes: &mut [u8], offset: usize, id: NodeId<'_, T>) {
    bytes[offset..offset + 4].copy_from_slice(
        &u32::try_from(id.index())
            .expect("AST node ID exceeds four bytes")
            .to_le_bytes(),
    );
}

fn read_node_id<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> NodeId<'ast, T> {
    context.encoded_node_id_at(
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize,
    )
}

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn encode_border_image_repeat(value: BorderImageRepeatKeyword) -> u8 {
    match value {
        BorderImageRepeatKeyword::Stretch => 0,
        BorderImageRepeatKeyword::Repeat => 1,
        BorderImageRepeatKeyword::Round => 2,
        BorderImageRepeatKeyword::Space => 3,
    }
}

fn decode_border_image_repeat(value: u8) -> BorderImageRepeatKeyword {
    match value {
        0 => BorderImageRepeatKeyword::Stretch,
        1 => BorderImageRepeatKeyword::Repeat,
        2 => BorderImageRepeatKeyword::Round,
        3 => BorderImageRepeatKeyword::Space,
        _ => panic!("invalid encoded BorderImageRepeatKeyword"),
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, BorderBlockStyle, BorderSideWidth, BorderStyle, CssColor, DUMMY_SP,
        GenericBorder, LineStyle, OutlineStyle,
    };

    #[test]
    fn border_aggregate_codecs_preserve_order_and_style_domains() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let style = context.alloc_encoded_node(
            BorderStyle {
                bottom: LineStyle::Dashed,
                left: LineStyle::Dotted,
                right: LineStyle::Double,
                top: LineStyle::Solid,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(style),
            BorderStyle {
                bottom: LineStyle::Dashed,
                left: LineStyle::Dotted,
                right: LineStyle::Double,
                top: LineStyle::Solid,
            }
        );

        let block = context.alloc_encoded_node(
            BorderBlockStyle {
                end: LineStyle::Groove,
                start: LineStyle::Ridge,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(block),
            BorderBlockStyle {
                end: LineStyle::Groove,
                start: LineStyle::Ridge,
            }
        );

        let color = context.alloc_encoded_node(CssColor::CurrentColor, DUMMY_SP);
        let width = context.alloc_encoded_node(BorderSideWidth::Medium, DUMMY_SP);
        let border = context.alloc_encoded_node(
            GenericBorder {
                color,
                style: OutlineStyle::LineStyle(LineStyle::Double),
                width,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(border),
            GenericBorder {
                color,
                style: OutlineStyle::LineStyle(LineStyle::Double),
                width,
            }
        );
    }
}
