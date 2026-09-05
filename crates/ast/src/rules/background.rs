use crate::*;

use crate::{AstNodeStorage, ExtraData, ExtraDataCompact, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub struct Position<'a> {
    pub x: NodeId<'a, PositionComponent<'a, HorizontalPositionKeyword>>,
    pub y: NodeId<'a, PositionComponent<'a, VerticalPositionKeyword>>,
}

impl<'ast> AstNodeStorage<'ast> for Position<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0006_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        decode_position(&payload.bytes(), context)
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_position(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_position(self)
    }
}

impl<'ast> ExtraDataCompact<'ast> for Position<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        payload_as_extra(encode_position(self))
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        decode_position(&data.bytes(), context)
    }
}

fn encode_position(value: Position<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_u32(&mut bytes, 0, node_index(value.x));
    write_u32(&mut bytes, 4, node_index(value.y));
    NodePayload::inline(&bytes)
}

fn decode_position<'ast>(bytes: &[u8], context: &AstContext<'ast>) -> Position<'ast> {
    Position {
        x: context.encoded_node_id_at(read_u32(bytes, 0) as usize),
        y: context.encoded_node_id_at(read_u32(bytes, 4) as usize),
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct WebKitGradientPoint {
    pub x: WebKitGradientPointComponent<HorizontalPositionKeyword>,
    pub y: WebKitGradientPointComponent<VerticalPositionKeyword>,
}

impl AstNodeStorage<'_> for WebKitGradientPoint {
    const KIND: NodeKind = NodeKind::new(0x0006_0002);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        Self {
            x: decode_gradient_point_component::<HorizontalPositionKeyword>(&bytes, 0),
            y: decode_gradient_point_component::<VerticalPositionKeyword>(&bytes, 8),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        encode_webkit_gradient_point(self)
    }

    fn encode_existing(self, _current: NodePayload, _context: &mut AstContext<'_>) -> NodePayload {
        encode_webkit_gradient_point(self)
    }
}

fn encode_webkit_gradient_point(value: WebKitGradientPoint) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    encode_gradient_point_component(value.x, &mut bytes, 0);
    encode_gradient_point_component(value.y, &mut bytes, 8);
    NodePayload::inline(&bytes)
}

trait GradientPointSide: Sized {
    fn encode(self) -> u8;

    fn decode(value: u8) -> Self;
}

impl GradientPointSide for HorizontalPositionKeyword {
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
            _ => panic!("invalid encoded horizontal gradient point side"),
        }
    }
}

impl GradientPointSide for VerticalPositionKeyword {
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
            _ => panic!("invalid encoded vertical gradient point side"),
        }
    }
}

fn encode_gradient_point_component<S: GradientPointSide>(
    value: WebKitGradientPointComponent<S>,
    bytes: &mut [u8],
    offset: usize,
) {
    match value {
        WebKitGradientPointComponent::Center => bytes[offset] = 0,
        WebKitGradientPointComponent::Number(value) => {
            bytes[offset] = 1;
            let (kind, value) = match value {
                NumberOrPercentage::Number(value) => (0, value),
                NumberOrPercentage::Percentage(value) => (1, value),
            };
            bytes[offset + 1] = kind;
            write_u32(bytes, offset + 4, value.to_bits());
        }
        WebKitGradientPointComponent::Side(value) => {
            bytes[offset] = 2;
            bytes[offset + 1] = value.encode();
        }
    }
}

fn decode_gradient_point_component<S: GradientPointSide>(
    bytes: &[u8],
    offset: usize,
) -> WebKitGradientPointComponent<S> {
    match bytes[offset] {
        0 => WebKitGradientPointComponent::Center,
        1 => WebKitGradientPointComponent::Number(match bytes[offset + 1] {
            0 => NumberOrPercentage::Number(f32::from_bits(read_u32(bytes, offset + 4))),
            1 => NumberOrPercentage::Percentage(f32::from_bits(read_u32(bytes, offset + 4))),
            _ => panic!("invalid encoded gradient point number variant"),
        }),
        2 => WebKitGradientPointComponent::Side(S::decode(bytes[offset + 1])),
        _ => panic!("invalid encoded WebKitGradientPointComponent variant"),
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct WebKitColorStop<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub position: f32,
}

impl<'ast> ExtraDataCompact<'ast> for WebKitColorStop<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        write_u32(&mut bytes, 0, node_index(self.color));
        write_u32(&mut bytes, 4, self.position.to_bits());
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let bytes = data.bytes();
        Self {
            color: context.encoded_node_id_at(read_u32(&bytes, 0) as usize),
            position: f32::from_bits(read_u32(&bytes, 4)),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct ImageSet<'a> {
    pub options: Vec<'a, ImageSetOption<'a>>,
    pub vendor_prefix: VendorPrefix,
}

impl<'ast> AstNodeStorage<'ast> for ImageSet<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0006_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            options: context
                .encoded_vec_range(read_u32(&bytes, 0) as usize, read_u32(&bytes, 4) as usize),
            vendor_prefix: VendorPrefix::from_bits_retain(bytes[8]),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_image_set(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_image_set(self)
    }
}

fn encode_image_set(value: ImageSet<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_range(&mut bytes, 0, value.options);
    bytes[8] = value.vendor_prefix.bits();
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub struct ImageSetOption<'a> {
    pub file_type: Option<&'a str>,
    pub image: NodeId<'a, Image<'a>>,
    pub resolution: Resolution,
}

// byte 0 file type presence, byte 1 resolution kind, bytes 4..8 image ID,
// bytes 8..12 resolution value, bytes 12..16 compact file-type string ID.
impl<'ast> AstNodeStorage<'ast> for ImageSetOption<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0006_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            file_type: match bytes[0] {
                0 => None,
                1 => Some(context.resolve_string(read_u32(&bytes, 12) as u64)),
                _ => panic!("invalid encoded ImageSetOption file type flag"),
            },
            image: context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
            resolution: crate::token::decode_resolution(
                bytes[1],
                f32::from_bits(read_u32(&bytes, 8)),
            ),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_image_set_option(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_image_set_option(self, context)
    }
}

fn encode_image_set_option<'ast>(
    value: ImageSetOption<'ast>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    if let Some(file_type) = value.file_type {
        bytes[0] = 1;
        let file_type = context.store_string(file_type);
        write_u32(&mut bytes, 12, file_type);
    }
    let (resolution_kind, resolution) = crate::token::encode_resolution(value.resolution);
    bytes[1] = resolution_kind;
    write_u32(&mut bytes, 4, node_index(value.image));
    write_u32(&mut bytes, 8, resolution.to_bits());
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub struct BackgroundPosition<'a> {
    pub x: NodeId<'a, PositionComponent<'a, HorizontalPositionKeyword>>,
    pub y: NodeId<'a, PositionComponent<'a, VerticalPositionKeyword>>,
}

impl<'ast> AstNodeStorage<'ast> for BackgroundPosition<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0006_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        decode_background_position(&payload.bytes(), context)
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_background_position(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_background_position(self)
    }
}

impl<'ast> ExtraDataCompact<'ast> for BackgroundPosition<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        payload_as_extra(encode_background_position(self))
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        decode_background_position(&data.bytes(), context)
    }
}

fn encode_background_position(value: BackgroundPosition<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    write_u32(&mut bytes, 0, node_index(value.x));
    write_u32(&mut bytes, 4, node_index(value.y));
    NodePayload::inline(&bytes)
}

fn decode_background_position<'ast>(
    bytes: &[u8],
    context: &AstContext<'ast>,
) -> BackgroundPosition<'ast> {
    BackgroundPosition {
        x: context.encoded_node_id_at(read_u32(bytes, 0) as usize),
        y: context.encoded_node_id_at(read_u32(bytes, 4) as usize),
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct BackgroundRepeat {
    pub x: BackgroundRepeatKeyword,
    pub y: BackgroundRepeatKeyword,
}

impl ExtraDataCompact<'_> for BackgroundRepeat {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_bytes(&[
            encode_background_repeat_keyword(self.x),
            encode_background_repeat_keyword(self.y),
        ])
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        let bytes = data.bytes();
        Self {
            x: decode_background_repeat_keyword(bytes[0]),
            y: decode_background_repeat_keyword(bytes[1]),
        }
    }
}

fn encode_background_repeat_keyword(value: BackgroundRepeatKeyword) -> u8 {
    match value {
        BackgroundRepeatKeyword::Repeat => 0,
        BackgroundRepeatKeyword::Space => 1,
        BackgroundRepeatKeyword::Round => 2,
        BackgroundRepeatKeyword::NoRepeat => 3,
    }
}

fn decode_background_repeat_keyword(value: u8) -> BackgroundRepeatKeyword {
    match value {
        0 => BackgroundRepeatKeyword::Repeat,
        1 => BackgroundRepeatKeyword::Space,
        2 => BackgroundRepeatKeyword::Round,
        3 => BackgroundRepeatKeyword::NoRepeat,
        _ => panic!("invalid encoded BackgroundRepeatKeyword"),
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Background<'a> {
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub color: NodeId<'a, CssColor<'a>>,
    pub image: NodeId<'a, Image<'a>>,
    pub origin: BackgroundOrigin,
    pub position: NodeId<'a, BackgroundPosition<'a>>,
    pub repeat: BackgroundRepeat,
    pub size: NodeId<'a, BackgroundSize<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct BoxShadow<'a> {
    pub blur: NodeId<'a, Length<'a>>,
    pub color: NodeId<'a, CssColor<'a>>,
    pub inset: bool,
    pub spread: NodeId<'a, Length<'a>>,
    pub x_offset: NodeId<'a, Length<'a>>,
    pub y_offset: NodeId<'a, Length<'a>>,
}

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact background field is four bytes"),
    )
}

fn write_range<T>(bytes: &mut [u8], offset: usize, range: Vec<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(range.start_index()).expect("AST range start exceeds four bytes"),
    );
    write_u32(
        bytes,
        offset + 4,
        u32::try_from(range.end_index()).expect("AST range end exceeds four bytes"),
    );
}

fn payload_as_extra(payload: NodePayload) -> ExtraData {
    ExtraData::from_bytes(&payload.bytes()[..ExtraData::BYTES])
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, BackgroundPosition, BackgroundRepeat, BackgroundRepeatKeyword, CssColor,
        DUMMY_SP, DimensionPercentage, HorizontalPositionKeyword, Image, ImageSet, ImageSetOption,
        Position, PositionComponent, Resolution, Url, VendorPrefix, VerticalPositionKeyword,
        WebKitColorStop, WebKitGradientPoint, WebKitGradientPointComponent,
    };

    #[test]
    fn position_codecs_share_the_same_inline_id_layout_in_nodes_and_lists() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let length = context.alloc_encoded_node(DimensionPercentage::Percentage(10.0), DUMMY_SP);
        let x = context.alloc_encoded_node(PositionComponent::Length(length), DUMMY_SP);
        let y = context.alloc_encoded_node(
            PositionComponent::<VerticalPositionKeyword>::Center,
            DUMMY_SP,
        );
        let position = context.alloc_encoded_node(Position { x, y }, DUMMY_SP);
        assert_eq!(context.encoded_node(position), Position { x, y });

        let values = context.alloc_encoded_vec([BackgroundPosition { x, y }].into_iter());
        assert_eq!(
            context.encoded_vec_get(values, 0),
            Some(BackgroundPosition { x, y })
        );
    }

    #[test]
    fn image_set_codecs_keep_ranges_prefixes_strings_and_resolution() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let url = context.alloc_encoded_node(Url { url: "image.avif" }, DUMMY_SP);
        let image = context.alloc_encoded_node(Image::Url(url), DUMMY_SP);
        let option = context.alloc_encoded_node(
            ImageSetOption {
                file_type: Some("image/avif"),
                image,
                resolution: Resolution::Dppx(2.0),
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(option),
            ImageSetOption {
                file_type: Some("image/avif"),
                image,
                resolution: Resolution::Dppx(2.0),
            }
        );

        let mut option_values = allocator.vec();
        option_values.push(ImageSetOption {
            file_type: Some("image/avif"),
            image,
            resolution: Resolution::Dppx(2.0),
        });
        let options = context.alloc_vec(option_values);
        let image_set = context.alloc_encoded_node(
            ImageSet {
                options,
                vendor_prefix: VendorPrefix::WEBKIT,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(image_set),
            ImageSet {
                options,
                vendor_prefix: VendorPrefix::WEBKIT,
            }
        );
    }

    #[test]
    fn background_list_elements_each_use_one_extra_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let color = context.alloc_encoded_node(CssColor::CurrentColor, DUMMY_SP);
        let stops = context.alloc_encoded_vec(
            [WebKitColorStop {
                color,
                position: 0.75,
            }]
            .into_iter(),
        );
        assert_eq!(
            context.encoded_vec_get(stops, 0),
            Some(WebKitColorStop {
                color,
                position: 0.75,
            })
        );

        let repeats = context.alloc_encoded_vec(
            [BackgroundRepeat {
                x: BackgroundRepeatKeyword::Round,
                y: BackgroundRepeatKeyword::NoRepeat,
            }]
            .into_iter(),
        );
        assert_eq!(
            context.encoded_vec_get(repeats, 0),
            Some(BackgroundRepeat {
                x: BackgroundRepeatKeyword::Round,
                y: BackgroundRepeatKeyword::NoRepeat,
            })
        );
    }

    #[test]
    fn webkit_gradient_point_codec_preserves_component_variants() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let point = WebKitGradientPoint {
            x: WebKitGradientPointComponent::Side(HorizontalPositionKeyword::Right),
            y: WebKitGradientPointComponent::Number(crate::NumberOrPercentage::Percentage(25.0)),
        };
        let id = context.alloc_encoded_node(point, DUMMY_SP);
        assert_eq!(
            context.encoded_node(id),
            WebKitGradientPoint {
                x: WebKitGradientPointComponent::Side(HorizontalPositionKeyword::Right),
                y: WebKitGradientPointComponent::Number(crate::NumberOrPercentage::Percentage(
                    25.0
                )),
            }
        );
    }
}
