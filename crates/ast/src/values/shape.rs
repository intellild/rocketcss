use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ClipPath<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
    Shape {
        reference_box: GeometryBox,
        shape: NodeId<'a, BasicShape<'a>>,
    },
    Box(GeometryBox),
}

impl<'ast> AstNodeStorage<'ast> for ClipPath<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001c_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Url(read_node_id(&bytes, context)),
            2 => Self::Shape {
                reference_box: decode_geometry_box(bytes[1]),
                shape: read_node_id(&bytes, context),
            },
            3 => Self::Box(decode_geometry_box(bytes[1])),
            _ => panic!("invalid encoded ClipPath variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_clip_path(self)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ClipPath<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Shape {
                reference_box,
                shape,
            } => Self::Shape {
                reference_box,
                shape: context.clone_encoded_node(shape),
            },
            value => value,
        }
    }
}

fn encode_clip_path(value: ClipPath<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        ClipPath::None => bytes[0] = 0,
        ClipPath::Url(value) => write_tagged_node_id(&mut bytes, 1, value),
        ClipPath::Shape {
            reference_box,
            shape,
        } => {
            bytes[0] = 2;
            bytes[1] = encode_geometry_box(reference_box);
            write_node_id(&mut bytes, shape);
        }
        ClipPath::Box(value) => {
            bytes[0] = 3;
            bytes[1] = encode_geometry_box(value);
        }
    }
    NodePayload::inline(&bytes)
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum GeometryBox {
    BorderBox,
    PaddingBox,
    ContentBox,
    MarginBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

impl ExtraDataCompact<'_> for GeometryBox {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(encode_geometry_box(self) as u64)
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        decode_geometry_box(data.as_u64() as u8)
    }
}

impl ExtraDataClone<'_> for GeometryBox {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum BasicShape<'a> {
    Inset(NodeId<'a, InsetRect<'a>>),
    Circle(NodeId<'a, CircleShape<'a>>),
    Ellipse(NodeId<'a, EllipseShape<'a>>),
    Polygon(NodeId<'a, Polygon<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for BasicShape<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001c_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Inset(read_node_id(&bytes, context)),
            1 => Self::Circle(read_node_id(&bytes, context)),
            2 => Self::Ellipse(read_node_id(&bytes, context)),
            3 => Self::Polygon(read_node_id(&bytes, context)),
            _ => panic!("invalid encoded BasicShape variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let (tag, id) = match self {
            Self::Inset(id) => (0, id.index()),
            Self::Circle(id) => (1, id.index()),
            Self::Ellipse(id) => (2, id.index()),
            Self::Polygon(id) => (3, id.index()),
        };
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = tag;
        write_u32(&mut bytes, 4, id);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for BasicShape<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Inset(value) => Self::Inset(context.clone_encoded_node(value)),
            Self::Circle(value) => Self::Circle(context.clone_encoded_node(value)),
            Self::Ellipse(value) => Self::Ellipse(context.clone_encoded_node(value)),
            Self::Polygon(value) => Self::Polygon(context.clone_encoded_node(value)),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum ShapeRadius<'a> {
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    ClosestSide,
    FarthestSide,
}

impl<'ast> AstNodeStorage<'ast> for ShapeRadius<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001c_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::LengthPercentage(read_node_id(&bytes, context)),
            1 => Self::ClosestSide,
            2 => Self::FarthestSide,
            _ => panic!("invalid encoded ShapeRadius variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::LengthPercentage(value) => write_tagged_node_id(&mut bytes, 0, value),
            Self::ClosestSide => bytes[0] = 1,
            Self::FarthestSide => bytes[0] = 2,
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ShapeRadius<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
            value => value,
        }
    }
}

fn encode_geometry_box(value: GeometryBox) -> u8 {
    match value {
        GeometryBox::BorderBox => 0,
        GeometryBox::PaddingBox => 1,
        GeometryBox::ContentBox => 2,
        GeometryBox::MarginBox => 3,
        GeometryBox::FillBox => 4,
        GeometryBox::StrokeBox => 5,
        GeometryBox::ViewBox => 6,
    }
}

fn decode_geometry_box(value: u8) -> GeometryBox {
    match value {
        0 => GeometryBox::BorderBox,
        1 => GeometryBox::PaddingBox,
        2 => GeometryBox::ContentBox,
        3 => GeometryBox::MarginBox,
        4 => GeometryBox::FillBox,
        5 => GeometryBox::StrokeBox,
        6 => GeometryBox::ViewBox,
        _ => panic!("invalid encoded GeometryBox"),
    }
}

fn write_tagged_node_id<T>(bytes: &mut [u8], tag: u8, value: NodeId<'_, T>) {
    bytes[0] = tag;
    write_node_id(bytes, value);
}

fn write_node_id<T>(bytes: &mut [u8], value: NodeId<'_, T>) {
    write_u32(bytes, 4, value.index());
}

fn read_node_id<'ast, T>(bytes: &[u8], context: &AstContext<'ast>) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, 4) as usize)
}

fn write_u32(bytes: &mut [u8], offset: usize, value: usize) {
    bytes[offset..offset + 4].copy_from_slice(
        &u32::try_from(value)
            .expect("AST node ID exceeds four bytes")
            .to_le_bytes(),
    );
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}
