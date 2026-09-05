use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct InsetRect<'a> {
    pub radius: NodeId<'a, BorderRadius<'a>>,
    pub rect: NodeId<'a, Rect<'a, LengthPercentage<'a>>>,
}

impl<'ast> AstNodeStorage<'ast> for InsetRect<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            radius: read_node_id(&bytes, 0, context),
            rect: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_ids(&[self.radius.index(), self.rect.index()])
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for InsetRect<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            radius: context.clone_encoded_node(self.radius),
            rect: context.clone_encoded_node(self.rect),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct CircleShape<'a> {
    pub position: NodeId<'a, Position<'a>>,
    pub radius: NodeId<'a, ShapeRadius<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for CircleShape<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            position: read_node_id(&bytes, 0, context),
            radius: read_node_id(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_ids(&[self.position.index(), self.radius.index()])
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for CircleShape<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            position: context.clone_encoded_node(self.position),
            radius: context.clone_encoded_node(self.radius),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct EllipseShape<'a> {
    pub position: NodeId<'a, Position<'a>>,
    pub radius_x: NodeId<'a, ShapeRadius<'a>>,
    pub radius_y: NodeId<'a, ShapeRadius<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for EllipseShape<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            position: read_node_id(&bytes, 0, context),
            radius_x: read_node_id(&bytes, 4, context),
            radius_y: read_node_id(&bytes, 8, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_ids(&[
            self.position.index(),
            self.radius_x.index(),
            self.radius_y.index(),
        ])
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for EllipseShape<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            position: context.clone_encoded_node(self.position),
            radius_x: context.clone_encoded_node(self.radius_x),
            radius_y: context.clone_encoded_node(self.radius_y),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Polygon<'a> {
    pub fill_rule: FillRule,
    pub points: Vec<'a, Point<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for Polygon<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            fill_rule: decode_fill_rule(bytes[0]),
            points: context
                .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = encode_fill_rule(self.fill_rule);
        write_range(&mut bytes, self.points);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Polygon<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            fill_rule: self.fill_rule,
            points: context.clone_encoded_vec(self.points),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Point<'a> {
    pub x: NodeId<'a, LengthPercentage<'a>>,
    pub y: NodeId<'a, LengthPercentage<'a>>,
}

impl<'ast> ExtraDataCompact<'ast> for Point<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        ExtraData::from_u64(pack_ids(self.x.index(), self.y.index()))
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let (x, y) = unpack_ids(data.as_u64());
        Self {
            x: context.encoded_node_id_at(x),
            y: context.encoded_node_id_at(y),
        }
    }
}

impl<'ast> ExtraDataClone<'ast> for Point<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            x: context.clone_encoded_node(self.x),
            y: context.clone_encoded_node(self.y),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Mask<'a> {
    pub clip: MaskClip,
    pub composite: MaskComposite,
    pub image: NodeId<'a, Image<'a>>,
    pub mode: MaskMode,
    pub origin: GeometryBox,
    pub position: NodeId<'a, Position<'a>>,
    pub repeat: BackgroundRepeat,
    pub size: NodeId<'a, BackgroundSize<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for Mask<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0007);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let enums = context.extra_slot(payload.extra_start()).bytes();
        Self {
            clip: MaskClip::decode_extra(
                ExtraData::from_u64(u16::from_le_bytes([enums[0], enums[1]]) as u64),
                context,
            ),
            composite: MaskComposite::decode_extra(ExtraData::from_u64(enums[2] as u64), context),
            image: read_node_id(&bytes, 0, context),
            mode: MaskMode::decode_extra(ExtraData::from_u64(enums[3] as u64), context),
            origin: GeometryBox::decode_extra(ExtraData::from_u64(enums[4] as u64), context),
            position: read_node_id(&bytes, 4, context),
            repeat: BackgroundRepeat::decode_extra(
                ExtraData::from_u64(u16::from_le_bytes([enums[5], enums[6]]) as u64),
                context,
            ),
            size: read_node_id(&bytes, 8, context),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_mask(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_mask(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for Mask<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            clip: self.clip,
            composite: self.composite,
            image: context.clone_encoded_node(self.image),
            mode: self.mode,
            origin: self.origin,
            position: context.clone_encoded_node(self.position),
            repeat: self.repeat,
            size: context.clone_encoded_node(self.size),
        }
    }
}

fn encode_mask<'ast>(
    value: Mask<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    write_id(&mut bytes, 0, value.image.index());
    write_id(&mut bytes, 4, value.position.index());
    write_id(&mut bytes, 8, value.size.index());
    let clip = value.clip.encode_extra(context).as_u64() as u16;
    let repeat = value.repeat.encode_extra(context).as_u64() as u16;
    let mut enums = [0; ExtraData::BYTES];
    enums[0..2].copy_from_slice(&clip.to_le_bytes());
    enums[2] = value.composite.encode_extra(context).as_u64() as u8;
    enums[3] = value.mode.encode_extra(context).as_u64() as u8;
    enums[4] = value.origin.encode_extra(context).as_u64() as u8;
    enums[5..7].copy_from_slice(&repeat.to_le_bytes());
    let enums = ExtraData::from_bytes(&enums);
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, enums);
            extra
        }
        None => context.alloc_extra_slots([enums]),
    };
    NodePayload::with_extra(&bytes, extra)
}

#[derive(Debug, PartialEq, Visit)]
pub struct MaskBorder<'a> {
    pub mode: MaskBorderMode,
    pub outset: NodeId<'a, Rect<'a, LengthOrNumber<'a>>>,
    pub repeat: BorderImageRepeat,
    pub slice: NodeId<'a, BorderImageSlice<'a>>,
    pub source: NodeId<'a, Image<'a>>,
    pub width: NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>,
}

impl<'ast> AstNodeStorage<'ast> for MaskBorder<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let first = context.extra_slot(payload.extra_start()).as_u64();
        let second = context.extra_slot(payload.extra_start() + 1).as_u64();
        let (slice, source) = unpack_ids(first);
        let (width, _) = unpack_ids(second);
        Self {
            mode: decode_mask_border_mode(bytes[0]),
            outset: read_node_id(&bytes, 4, context),
            repeat: BorderImageRepeat {
                horizontal: decode_border_image_repeat(bytes[1]),
                vertical: decode_border_image_repeat(bytes[2]),
            },
            slice: context.encoded_node_id_at(slice),
            source: context.encoded_node_id_at(source),
            width: context.encoded_node_id_at(width),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_mask_border(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_mask_border(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for MaskBorder<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            mode: self.mode,
            outset: context.clone_encoded_node(self.outset),
            repeat: self.repeat,
            slice: context.clone_encoded_node(self.slice),
            source: context.clone_encoded_node(self.source),
            width: context.clone_encoded_node(self.width),
        }
    }
}

fn encode_mask_border<'ast>(
    value: MaskBorder<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    bytes[0] = encode_mask_border_mode(value.mode);
    bytes[1] = encode_border_image_repeat(value.repeat.horizontal);
    bytes[2] = encode_border_image_repeat(value.repeat.vertical);
    write_id(&mut bytes, 4, value.outset.index());
    let slots = [
        ExtraData::from_u64(pack_ids(value.slice.index(), value.source.index())),
        ExtraData::from_u64(pack_ids(value.width.index(), 0)),
    ];
    let extra_start = match existing_extra {
        Some(extra_start) => {
            context.set_extra_slot(extra_start, slots[0]);
            context.set_extra_slot(extra_start + 1, slots[1]);
            extra_start
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::with_extra(&bytes, extra_start)
}

#[derive(Debug, PartialEq, Visit)]
pub struct DropShadow<'a> {
    pub blur: NodeId<'a, Length<'a>>,
    pub color: NodeId<'a, CssColor<'a>>,
    pub x_offset: NodeId<'a, Length<'a>>,
    pub y_offset: NodeId<'a, Length<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for DropShadow<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0006);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            blur: read_node_id(&bytes, 0, context),
            color: read_node_id(&bytes, 4, context),
            x_offset: read_node_id(&bytes, 8, context),
            y_offset: read_node_id(&bytes, 12, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_ids(&[
            self.blur.index(),
            self.color.index(),
            self.x_offset.index(),
            self.y_offset.index(),
        ])
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for DropShadow<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            blur: context.clone_encoded_node(self.blur),
            color: context.clone_encoded_node(self.color),
            x_offset: context.clone_encoded_node(self.x_offset),
            y_offset: context.clone_encoded_node(self.y_offset),
        }
    }
}

fn encode_ids(ids: &[usize]) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    for (index, id) in ids.iter().copied().enumerate() {
        write_id(&mut bytes, index * 4, id);
    }
    NodePayload::inline(&bytes)
}

fn read_node_id<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, offset) as usize)
}

fn write_range<T>(bytes: &mut [u8], range: Vec<'_, T>) {
    write_id(bytes, 4, range.start_index());
    write_id(bytes, 8, range.end_index());
}

fn write_id(bytes: &mut [u8], offset: usize, value: usize) {
    bytes[offset..offset + 4].copy_from_slice(
        &u32::try_from(value)
            .expect("AST compact index exceeds four bytes")
            .to_le_bytes(),
    );
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}

fn pack_ids(first: usize, second: usize) -> u64 {
    let first = u32::try_from(first).expect("AST node ID exceeds four bytes");
    let second = u32::try_from(second).expect("AST node ID exceeds four bytes");
    (second as u64) << 32 | first as u64
}

fn unpack_ids(value: u64) -> (usize, usize) {
    (value as u32 as usize, (value >> 32) as u32 as usize)
}

fn encode_fill_rule(value: FillRule) -> u8 {
    match value {
        FillRule::Nonzero => 0,
        FillRule::Evenodd => 1,
    }
}

fn decode_fill_rule(value: u8) -> FillRule {
    match value {
        0 => FillRule::Nonzero,
        1 => FillRule::Evenodd,
        _ => panic!("invalid encoded FillRule"),
    }
}

fn encode_mask_border_mode(value: MaskBorderMode) -> u8 {
    match value {
        MaskBorderMode::Luminance => 0,
        MaskBorderMode::Alpha => 1,
    }
}

fn decode_mask_border_mode(value: u8) -> MaskBorderMode {
    match value {
        0 => MaskBorderMode::Luminance,
        1 => MaskBorderMode::Alpha,
        _ => panic!("invalid encoded MaskBorderMode"),
    }
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
