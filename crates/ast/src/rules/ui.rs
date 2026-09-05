use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct Cursor<'a> {
    pub images: Vec<'a, NodeId<'a, CursorImage<'a>>>,
    pub keyword: CursorKeyword,
}

impl<'ast> AstNodeStorage<'ast> for Cursor<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0021_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            images: context
                .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
            keyword: decode_cursor_keyword(bytes[0]),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = encode_cursor_keyword(self.keyword);
        write_u32(&mut bytes, 4, self.images.start_index());
        write_u32(&mut bytes, 8, self.images.end_index());
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Cursor<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            images: context.clone_encoded_vec(self.images),
            keyword: self.keyword,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct CursorImage<'a> {
    pub hotspot: Option<(f32, f32)>,
    pub url: NodeId<'a, Url<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for CursorImage<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0021_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            hotspot: (bytes[0] != 0).then(|| {
                (
                    f32::from_bits(read_u32(&bytes, 8)),
                    f32::from_bits(read_u32(&bytes, 12)),
                )
            }),
            url: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        write_u32(&mut bytes, 4, node_index(self.url));
        if let Some((x, y)) = self.hotspot {
            bytes[0] = 1;
            write_u32(&mut bytes, 8, x.to_bits());
            write_u32(&mut bytes, 12, y.to_bits());
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for CursorImage<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            hotspot: self.hotspot,
            url: context.clone_encoded_node(self.url),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Caret<'a> {
    pub color: NodeId<'a, ColorOrAuto<'a>>,
    pub shape: CaretShape,
}

impl<'ast> AstNodeStorage<'ast> for Caret<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0021_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            color: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
            shape: decode_caret_shape(bytes[0]),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = encode_caret_shape(self.shape);
        write_u32(&mut bytes, 4, node_index(self.color));
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Caret<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            color: context.clone_encoded_node(self.color),
            shape: self.shape,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ListStyle<'a> {
    pub image: NodeId<'a, Image<'a>>,
    pub list_style_type: NodeId<'a, ListStyleType<'a>>,
    pub position: ListStylePosition,
}

impl<'ast> AstNodeStorage<'ast> for ListStyle<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0021_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            image: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
            list_style_type: context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            position: decode_list_style_position(bytes[0]),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = encode_list_style_position(self.position);
        write_u32(&mut bytes, 4, node_index(self.image));
        write_u32(&mut bytes, 8, node_index(self.list_style_type));
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ListStyle<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            image: context.clone_encoded_node(self.image),
            list_style_type: context.clone_encoded_node(self.list_style_type),
            position: self.position,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Composes<'a> {
    pub from: Option<NodeId<'a, Specifier<'a>>>,
    pub names: Vec<'a, &'a str>,
}

impl<'ast> AstNodeStorage<'ast> for Composes<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0021_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let from = read_u32(&bytes, 0);
        Self {
            from: (from != u32::MAX).then(|| context.encoded_node_id_at(from as usize)),
            names: context
                .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        write_u32(&mut bytes, 0, self.from.map_or(u32::MAX, node_index));
        write_u32(&mut bytes, 4, self.names.start_index());
        write_u32(&mut bytes, 8, self.names.end_index());
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Composes<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            from: self.from.map(|value| context.clone_encoded_node(value)),
            names: context.clone_encoded_vec(self.names),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ColorScheme {
    pub dark: bool,
    pub light: bool,
    pub only: bool,
}

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn write_u32(bytes: &mut [u8], offset: usize, value: impl TryInto<u32>) {
    bytes[offset..offset + 4].copy_from_slice(
        &value
            .try_into()
            .unwrap_or_else(|_| panic!("AST compact value exceeds four bytes"))
            .to_le_bytes(),
    );
}

fn encode_caret_shape(value: CaretShape) -> u8 {
    match value {
        CaretShape::Auto => 0,
        CaretShape::Bar => 1,
        CaretShape::Block => 2,
        CaretShape::Underscore => 3,
    }
}

fn decode_caret_shape(value: u8) -> CaretShape {
    match value {
        0 => CaretShape::Auto,
        1 => CaretShape::Bar,
        2 => CaretShape::Block,
        3 => CaretShape::Underscore,
        _ => panic!("invalid encoded CaretShape"),
    }
}

fn encode_list_style_position(value: ListStylePosition) -> u8 {
    match value {
        ListStylePosition::Inside => 0,
        ListStylePosition::Outside => 1,
    }
}

fn decode_list_style_position(value: u8) -> ListStylePosition {
    match value {
        0 => ListStylePosition::Inside,
        1 => ListStylePosition::Outside,
        _ => panic!("invalid encoded ListStylePosition"),
    }
}

fn encode_cursor_keyword(value: CursorKeyword) -> u8 {
    match value {
        CursorKeyword::Auto => 0,
        CursorKeyword::Default => 1,
        CursorKeyword::None => 2,
        CursorKeyword::ContextMenu => 3,
        CursorKeyword::Help => 4,
        CursorKeyword::Pointer => 5,
        CursorKeyword::Progress => 6,
        CursorKeyword::Wait => 7,
        CursorKeyword::Cell => 8,
        CursorKeyword::Crosshair => 9,
        CursorKeyword::Text => 10,
        CursorKeyword::VerticalText => 11,
        CursorKeyword::Alias => 12,
        CursorKeyword::Copy => 13,
        CursorKeyword::Move => 14,
        CursorKeyword::NoDrop => 15,
        CursorKeyword::NotAllowed => 16,
        CursorKeyword::Grab => 17,
        CursorKeyword::Grabbing => 18,
        CursorKeyword::EResize => 19,
        CursorKeyword::NResize => 20,
        CursorKeyword::NeResize => 21,
        CursorKeyword::NwResize => 22,
        CursorKeyword::SResize => 23,
        CursorKeyword::SeResize => 24,
        CursorKeyword::SwResize => 25,
        CursorKeyword::WResize => 26,
        CursorKeyword::EwResize => 27,
        CursorKeyword::NsResize => 28,
        CursorKeyword::NeswResize => 29,
        CursorKeyword::NwseResize => 30,
        CursorKeyword::ColResize => 31,
        CursorKeyword::RowResize => 32,
        CursorKeyword::AllScroll => 33,
        CursorKeyword::ZoomIn => 34,
        CursorKeyword::ZoomOut => 35,
    }
}

fn decode_cursor_keyword(value: u8) -> CursorKeyword {
    match value {
        0 => CursorKeyword::Auto,
        1 => CursorKeyword::Default,
        2 => CursorKeyword::None,
        3 => CursorKeyword::ContextMenu,
        4 => CursorKeyword::Help,
        5 => CursorKeyword::Pointer,
        6 => CursorKeyword::Progress,
        7 => CursorKeyword::Wait,
        8 => CursorKeyword::Cell,
        9 => CursorKeyword::Crosshair,
        10 => CursorKeyword::Text,
        11 => CursorKeyword::VerticalText,
        12 => CursorKeyword::Alias,
        13 => CursorKeyword::Copy,
        14 => CursorKeyword::Move,
        15 => CursorKeyword::NoDrop,
        16 => CursorKeyword::NotAllowed,
        17 => CursorKeyword::Grab,
        18 => CursorKeyword::Grabbing,
        19 => CursorKeyword::EResize,
        20 => CursorKeyword::NResize,
        21 => CursorKeyword::NeResize,
        22 => CursorKeyword::NwResize,
        23 => CursorKeyword::SResize,
        24 => CursorKeyword::SeResize,
        25 => CursorKeyword::SwResize,
        26 => CursorKeyword::WResize,
        27 => CursorKeyword::EwResize,
        28 => CursorKeyword::NsResize,
        29 => CursorKeyword::NeswResize,
        30 => CursorKeyword::NwseResize,
        31 => CursorKeyword::ColResize,
        32 => CursorKeyword::RowResize,
        33 => CursorKeyword::AllScroll,
        34 => CursorKeyword::ZoomIn,
        35 => CursorKeyword::ZoomOut,
        _ => panic!("invalid encoded CursorKeyword"),
    }
}
