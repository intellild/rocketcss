use crate::*;

use crate::{AstNodeClone, AstNodeStorage, ExtraData, ExtraDataClone, NodeKind, NodePayload};

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Position<'a> {
    pub x: NodeId<'a, PositionComponent<'a, HorizontalPositionKeyword>>,
    pub y: NodeId<'a, PositionComponent<'a, VerticalPositionKeyword>>,
}

impl_inline_node!(Position<'ast>, 0x0006_0001);

impl<'ast> AstNodeClone<'ast> for Position<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            x: context.clone_encoded_node(self.x),
            y: context.clone_encoded_node(self.y),
        }
    }
}

impl_inline_extra!(Position<'ast>);

impl<'ast> ExtraDataClone<'ast> for Position<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_in_context(context)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct WebKitGradientPoint {
    pub x: WebKitGradientPointComponent<HorizontalPositionKeyword>,
    pub y: WebKitGradientPointComponent<VerticalPositionKeyword>,
}

// The complete native point fits one payload, including both nested enums.
impl_inline_node!(WebKitGradientPoint, 0x0006_0002);

impl AstNodeClone<'_> for WebKitGradientPoint {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct WebKitColorStop<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub position: f32,
}

impl_inline_extra!(WebKitColorStop<'ast>);

impl<'ast> ExtraDataClone<'ast> for WebKitColorStop<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            color: context.clone_encoded_node(self.color),
            position: self.position,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ImageSet<'a> {
    pub options: Vec<'a, NodeId<'a, ImageSetOption<'a>>>,
    pub vendor_prefix: VendorPrefix,
}

impl_inline_node!(ImageSet<'ast>, 0x0006_0003);

impl<'ast> AstNodeClone<'ast> for ImageSet<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            options: context.clone_encoded_vec(self.options),
            vendor_prefix: self.vendor_prefix,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct ImageSetOption<'a> {
    pub file_type: Option<AstStr<'a>>,
    pub image: NodeId<'a, Image<'a>>,
    pub resolution: Resolution,
}

#[derive(Clone, Copy)]
struct ImageSetOptionHeader<'a> {
    image: NodeId<'a, Image<'a>>,
    resolution: Resolution,
    // u32::MAX means no slot allocated yet. An allocated slot stores Option<AstStr>.
    file_type_extra: u32,
}

impl<'ast> ImageSetOptionHeader<'ast> {
    fn file_type(self, context: &AstContext<'_>) -> Option<AstStr<'ast>> {
        if self.file_type_extra == u32::MAX {
            None
        } else {
            // SAFETY: this header owns a slot written by Option<AstStr>::encode_extra.
            unsafe {
                Option::<AstStr<'ast>>::decode_extra(
                    context.extra_slot(self.file_type_extra as usize),
                )
            }
        }
    }
}

pub use image_set_access::ImageSetOptionRead;

// Transient storage views do not participate in persistent AST visitor generation.
mod image_set_access {
    use super::*;

    pub struct ImageSetOptionRead<'context, 'storage, 'ast> {
        context: &'context AstContext<'storage>,
        header: ImageSetOptionHeader<'ast>,
    }

    impl<'ast> ImageSetOptionRead<'_, '_, 'ast> {
        pub fn image(&self) -> NodeId<'ast, Image<'ast>> {
            self.header.image
        }

        pub fn resolution(&self) -> Resolution {
            self.header.resolution
        }

        pub fn file_type(&self) -> Option<AstStr<'ast>> {
            self.header.file_type(self.context)
        }
    }

    impl<'storage> AstContext<'storage> {
        pub fn image_set_option<'id>(
            &self,
            id: NodeId<'id, ImageSetOption<'id>>,
        ) -> ImageSetOptionRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the kind before reading this header.
            ImageSetOptionRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: KIND identifies the native header and its Option<AstStr> extra slot.
unsafe impl<'ast> AstNodeStorage<'ast> for ImageSetOption<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0006_0004);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        self.image == other.image
            && self.resolution == other.resolution
            && self.file_type.map(|value| context.str(value))
                == other.file_type.map(|value| context.str(value))
    }
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: ImageSetOptionHeader<'ast> = unsafe { payload.read_value() };
        Self {
            image: header.image,
            resolution: header.resolution,
            file_type: header.file_type(context),
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_image_set_option(self, u32::MAX, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: ImageSetOptionHeader<'ast> = unsafe { current.read_value() };
        encode_image_set_option(self, header.file_type_extra, context)
    }
}

impl<'ast> AstNodeClone<'ast> for ImageSetOption<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            file_type: self.file_type,
            image: context.clone_encoded_node(self.image),
            resolution: self.resolution,
        }
    }
}

fn encode_image_set_option<'ast>(
    value: ImageSetOption<'ast>,
    current: u32,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let file_type_extra = if current != u32::MAX {
        context.set_extra_slot(current as usize, value.file_type.encode_extra());
        current
    } else if value.file_type.is_some() {
        let extra = context.alloc_extra_slots([value.file_type.encode_extra()]);
        assert!(
            extra < u32::MAX as usize,
            "image-set file type index exceeds available u32 range"
        );
        extra as u32
    } else {
        u32::MAX
    };
    NodePayload::from_value(ImageSetOptionHeader {
        image: value.image,
        resolution: value.resolution,
        file_type_extra,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BackgroundPosition<'a> {
    pub x: NodeId<'a, PositionComponent<'a, HorizontalPositionKeyword>>,
    pub y: NodeId<'a, PositionComponent<'a, VerticalPositionKeyword>>,
}

impl_inline_node!(BackgroundPosition<'ast>, 0x0006_0005);

impl<'ast> AstNodeClone<'ast> for BackgroundPosition<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            x: context.clone_encoded_node(self.x),
            y: context.clone_encoded_node(self.y),
        }
    }
}

impl_inline_extra!(BackgroundPosition<'ast>);

impl<'ast> ExtraDataClone<'ast> for BackgroundPosition<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.clone_in_context(context)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BackgroundRepeat {
    pub x: BackgroundRepeatKeyword,
    pub y: BackgroundRepeatKeyword,
}

impl_inline_extra!(BackgroundRepeat);

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

#[derive(Clone, Copy)]
struct BackgroundHeader<'a> {
    color: NodeId<'a, CssColor<'a>>,
    image: NodeId<'a, Image<'a>>,
    position: NodeId<'a, BackgroundPosition<'a>>,
    extra: u32,
}
#[derive(Clone, Copy)]
struct BackgroundKeywords {
    attachment: BackgroundAttachment,
    clip: BackgroundClip,
    origin: BackgroundOrigin,
    repeat: BackgroundRepeat,
}

pub use background_access::{BackgroundKeywordsRead, BackgroundRead};
mod background_access {
    use super::*;
    pub struct BackgroundRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: BackgroundHeader<'id>,
    }
    pub struct BackgroundKeywordsRead(BackgroundKeywords);
    impl BackgroundKeywordsRead {
        pub fn attachment(&self) -> BackgroundAttachment {
            self.0.attachment
        }
        pub fn clip(&self) -> BackgroundClip {
            self.0.clip
        }
        pub fn origin(&self) -> BackgroundOrigin {
            self.0.origin
        }
        pub fn repeat(&self) -> BackgroundRepeat {
            self.0.repeat
        }
    }
    impl<'id> BackgroundRead<'_, '_, 'id> {
        pub fn color(&self) -> NodeId<'id, CssColor<'id>> {
            self.header.color
        }
        pub fn image(&self) -> NodeId<'id, Image<'id>> {
            self.header.image
        }
        pub fn position(&self) -> NodeId<'id, BackgroundPosition<'id>> {
            self.header.position
        }
        pub fn size(&self) -> NodeId<'id, BackgroundSize<'id>> {
            // SAFETY: this kind writes a native size handle to its first extra slot.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            }
        }
        pub fn keywords(&self) -> BackgroundKeywordsRead {
            // SAFETY: the second extra slot is written as BackgroundKeywords.
            BackgroundKeywordsRead(unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 1)
                    .read_value()
            })
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn background<'id>(
            &self,
            id: NodeId<'id, Background<'id>>,
        ) -> BackgroundRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the owning kind before the native header read.
            BackgroundRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: this kind stores BackgroundHeader, then a native size handle and keyword slot.
unsafe impl<'ast> AstNodeStorage<'ast> for Background<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0006_0006);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: BackgroundHeader<'ast> = unsafe { payload.read_value() };
        let size = unsafe { context.extra_slot(header.extra as usize).read_value() };
        let fields: BackgroundKeywords =
            unsafe { context.extra_slot(header.extra as usize + 1).read_value() };
        Self {
            color: header.color,
            image: header.image,
            position: header.position,
            size,
            attachment: fields.attachment,
            clip: fields.clip,
            origin: fields.origin,
            repeat: fields.repeat,
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_background(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: BackgroundHeader<'ast> = unsafe { current.read_value() };
        encode_background(self, Some(header.extra as usize), context)
    }
}

fn encode_background<'ast>(
    value: Background<'ast>,
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let slots = [
        ExtraData::from_value(value.size),
        ExtraData::from_value(BackgroundKeywords {
            attachment: value.attachment,
            clip: value.clip,
            origin: value.origin,
            repeat: value.repeat,
        }),
    ];
    let extra = match existing {
        Some(extra) => {
            for (i, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(extra + i, slot);
            }
            extra
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::from_value(BackgroundHeader {
        color: value.color,
        image: value.image,
        position: value.position,
        extra: u32::try_from(extra).expect("AST extra index exceeds u32"),
    })
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

#[derive(Clone, Copy)]
struct BoxShadowHeader<'a> {
    blur: NodeId<'a, Length<'a>>,
    color: NodeId<'a, CssColor<'a>>,
    x_offset: NodeId<'a, Length<'a>>,
    extra: u32,
}
pub use box_shadow_access::BoxShadowRead;

mod box_shadow_access {
    use super::*;

    pub struct BoxShadowRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: BoxShadowHeader<'id>,
    }
    impl<'id> BoxShadowRead<'_, '_, 'id> {
        pub fn offsets(&self) -> [NodeId<'id, Length<'id>>; 4] {
            // SAFETY: the first slot stores the native y-offset/spread handle pair.
            let (y, spread) = unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            };
            [self.header.x_offset, y, self.header.blur, spread]
        }
        pub fn color(&self) -> NodeId<'id, CssColor<'id>> {
            self.header.color
        }
        pub fn inset(&self) -> bool {
            // SAFETY: the second slot is written as a native bool.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 1)
                    .read_value()
            }
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn box_shadow<'id>(
            &self,
            id: NodeId<'id, BoxShadow<'id>>,
        ) -> BoxShadowRead<'_, 'storage, 'id> {
            // SAFETY: node_payload validates the owning kind before the header read.
            BoxShadowRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: this kind stores BoxShadowHeader, a native handle pair, and a bool slot.
unsafe impl<'ast> AstNodeStorage<'ast> for BoxShadow<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0006_0007);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: BoxShadowHeader<'ast> = unsafe { payload.read_value() };
        let (y_offset, spread) = unsafe { context.extra_slot(header.extra as usize).read_value() };
        let inset = unsafe { context.extra_slot(header.extra as usize + 1).read_value() };
        Self {
            blur: header.blur,
            color: header.color,
            x_offset: header.x_offset,
            y_offset,
            spread,
            inset,
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_box_shadow(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: BoxShadowHeader<'ast> = unsafe { current.read_value() };
        encode_box_shadow(self, Some(header.extra as usize), context)
    }
}

impl<'ast> AstNodeClone<'ast> for BoxShadow<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            blur: context.clone_encoded_node(self.blur),
            color: context.clone_encoded_node(self.color),
            inset: self.inset,
            spread: context.clone_encoded_node(self.spread),
            x_offset: context.clone_encoded_node(self.x_offset),
            y_offset: context.clone_encoded_node(self.y_offset),
        }
    }
}

fn encode_box_shadow<'ast>(
    value: BoxShadow<'ast>,
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let slots = [
        ExtraData::from_value((value.y_offset, value.spread)),
        ExtraData::from_value(value.inset),
    ];
    let extra = match existing {
        Some(extra) => {
            for (i, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(extra + i, slot);
            }
            extra
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::from_value(BoxShadowHeader {
        blur: value.blur,
        color: value.color,
        x_offset: value.x_offset,
        extra: u32::try_from(extra).expect("AST extra index exceeds u32"),
    })
}

#[cfg(test)]
mod storage_tests {
    use super::ImageSetOptionHeader;
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, Background, BackgroundAttachment, BackgroundClip, BackgroundOrigin,
        BackgroundPosition, BackgroundRepeat, BackgroundRepeatKeyword, BackgroundSize, CssColor,
        DUMMY_SP, DimensionPercentage, HorizontalPositionKeyword, Image, ImageSet, ImageSetOption,
        LengthPercentageOrAuto, Position, PositionComponent, Resolution, Url, VendorPrefix,
        VerticalPositionKeyword, WebKitColorStop, WebKitGradientPoint,
        WebKitGradientPointComponent,
    };

    #[test]
    fn box_shadow_native_fields_preserve_boolean_and_handle_changes() {
        use crate::{BoxShadow, Length, LengthUnit, LengthValue};
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let length = ast.alloc_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 1.0,
            }),
            DUMMY_SP,
        );
        let other = ast.alloc_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 2.0,
            }),
            DUMMY_SP,
        );
        let color = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let node = ast.alloc_node(
            BoxShadow {
                blur: length,
                color,
                inset: false,
                spread: other,
                x_offset: length,
                y_offset: other,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for inset in [true, false, true] {
            ast.mutate_node(node, |value, _| {
                value.inset = inset;
                value.y_offset = length;
                value.spread = other;
            });
            let value = ast.resolve_node(node);
            assert_eq!(value.inset, inset);
            assert_eq!(value.y_offset, length);
            assert_eq!(value.spread, other);
            assert_eq!(value.color, color);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }

    #[test]
    fn image_set_type_ranges_preserve_optional_empty_and_reuse_storage() {
        assert_eq!(std::mem::size_of::<ImageSetOptionHeader<'_>>(), 16);
        assert_eq!(std::mem::size_of::<ImageSet<'_>>(), 12);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let image = context.alloc_encoded_node(Image::None, DUMMY_SP);
        let text = context.add_str("image/avif");
        let duplicate = context.add_str("image/avif");
        let node = context.alloc_encoded_node(
            ImageSetOption {
                image,
                resolution: Resolution::Dpi(-0.0),
                file_type: None,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), 0);
        context.mutate_encoded_node(node, |value, _| {
            value.file_type = Some(crate::AstStr::EMPTY)
        });
        assert_eq!(context.encoded_extra_len(), 1);
        let second_image = context.alloc_encoded_node(Image::None, DUMMY_SP);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for file_type in [
            Some(text),
            None,
            Some(duplicate),
            Some(crate::AstStr::EMPTY),
        ] {
            for bits in [
                0,
                0x8000_0000,
                1,
                0x7f7f_ffff,
                0x7f80_0000,
                0xff80_0000,
                0x7fc0_1234,
            ] {
                let f = f32::from_bits(bits);
                for resolution in [Resolution::Dpi(f), Resolution::Dpcm(f), Resolution::Dppx(f)] {
                    for image in [image, second_image] {
                        context.mutate_encoded_node(node, |value, _| {
                            value.file_type = file_type;
                            value.resolution = resolution;
                            value.image = image;
                        });
                        let value = context.encoded_node(node);
                        assert_eq!(value.file_type, file_type);
                        assert_eq!(value.image, image);
                        let view = context.image_set_option(node);
                        assert_eq!(view.image(), image);
                        assert_eq!(view.file_type(), file_type);
                        for actual in [view.resolution(), value.resolution] {
                            assert_eq!(
                                std::mem::discriminant(&actual),
                                std::mem::discriminant(&resolution)
                            );
                            let (Resolution::Dpi(f) | Resolution::Dpcm(f) | Resolution::Dppx(f)) =
                                actual;
                            assert_eq!(f.to_bits(), bits);
                        }
                        assert_eq!(context.node_checkpoint(), checkpoint);
                    }
                }
            }
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
        context.mutate_encoded_node(node, |value, _| value.file_type = None);
        assert_eq!(context.encoded_node(node).file_type, None);
        assert_eq!(context.encoded_extra_len(), 1);
    }

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
        let text = context.add_str("image.avif");
        let url = context.alloc_encoded_node(Url { url: text }, DUMMY_SP);
        let image = context.alloc_encoded_node(Image::Url(url), DUMMY_SP);
        let mime = context.add_str("image/avif");
        let option = context.alloc_encoded_node(
            ImageSetOption {
                file_type: Some(mime),
                image,
                resolution: Resolution::Dppx(2.0),
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(option),
            ImageSetOption {
                file_type: Some(mime),
                image,
                resolution: Resolution::Dppx(2.0),
            }
        );

        let options = context.alloc_encoded_vec([option].into_iter());
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
    fn background_codec_uses_and_reuses_two_fixed_overflow_slots() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let color = context.alloc_encoded_node(CssColor::CurrentColor, DUMMY_SP);
        let image = context.alloc_encoded_node(Image::None, DUMMY_SP);
        let x = context.alloc_encoded_node(
            PositionComponent::<HorizontalPositionKeyword>::Center,
            DUMMY_SP,
        );
        let y = context.alloc_encoded_node(
            PositionComponent::<VerticalPositionKeyword>::Center,
            DUMMY_SP,
        );
        let position = context.alloc_encoded_node(BackgroundPosition { x, y }, DUMMY_SP);
        let width = context.alloc_encoded_node(LengthPercentageOrAuto::Auto, DUMMY_SP);
        let height = context.alloc_encoded_node(LengthPercentageOrAuto::Auto, DUMMY_SP);
        let size = context.alloc_encoded_node(BackgroundSize::Explicit { height, width }, DUMMY_SP);
        let before = context.encoded_extra_len();
        let background = context.alloc_encoded_node(
            Background {
                attachment: BackgroundAttachment::Fixed,
                clip: BackgroundClip::Text,
                color,
                image,
                origin: BackgroundOrigin::ContentBox,
                position,
                repeat: BackgroundRepeat {
                    x: BackgroundRepeatKeyword::Round,
                    y: BackgroundRepeatKeyword::Space,
                },
                size,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 2);

        let checkpoint = context.node_checkpoint();
        for attachment in [
            BackgroundAttachment::Scroll,
            BackgroundAttachment::Fixed,
            BackgroundAttachment::Local,
        ] {
            for clip in [
                BackgroundClip::BorderBox,
                BackgroundClip::PaddingBox,
                BackgroundClip::ContentBox,
                BackgroundClip::Border,
                BackgroundClip::Text,
            ] {
                for origin in [
                    BackgroundOrigin::BorderBox,
                    BackgroundOrigin::PaddingBox,
                    BackgroundOrigin::ContentBox,
                ] {
                    let repeats = [
                        BackgroundRepeatKeyword::Repeat,
                        BackgroundRepeatKeyword::Space,
                        BackgroundRepeatKeyword::Round,
                        BackgroundRepeatKeyword::NoRepeat,
                    ];
                    for x in repeats {
                        for y in repeats {
                            let repeat = BackgroundRepeat { x, y };
                            context.mutate_encoded_node(background, |value, _| {
                                value.attachment = attachment;
                                value.clip = clip;
                                value.origin = origin;
                                value.repeat = repeat;
                            });
                            assert_eq!(
                                context.encoded_node(background),
                                Background {
                                    attachment,
                                    clip,
                                    origin,
                                    repeat,
                                    color,
                                    image,
                                    position,
                                    size,
                                }
                            );
                            let view = context.background(background);
                            assert_eq!(view.color(), color);
                            assert_eq!(view.image(), image);
                            assert_eq!(view.position(), position);
                            assert_eq!(view.size(), size);
                            let keywords = view.keywords();
                            assert_eq!(keywords.attachment(), attachment);
                            assert_eq!(keywords.clip(), clip);
                            assert_eq!(keywords.origin(), origin);
                            assert_eq!(keywords.repeat(), repeat);
                            assert_eq!(context.node_checkpoint(), checkpoint);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn native_webkit_gradient_point_preserves_all_component_variants() {
        use crate::NumberOrPercentage;
        use WebKitGradientPointComponent::{Center, Number, Side};
        fn check<S: std::fmt::Debug + PartialEq>(
            actual: WebKitGradientPointComponent<S>,
            expected: WebKitGradientPointComponent<S>,
        ) {
            match (actual, expected) {
                (Number(NumberOrPercentage::Number(a)), Number(NumberOrPercentage::Number(b)))
                | (
                    Number(NumberOrPercentage::Percentage(a)),
                    Number(NumberOrPercentage::Percentage(b)),
                ) => assert_eq!(a.to_bits(), b.to_bits()),
                (actual, expected) => assert_eq!(actual, expected),
            }
        }
        assert_eq!(std::mem::size_of::<WebKitGradientPoint>(), 16);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let id = context.alloc_encoded_node(
            WebKitGradientPoint {
                x: Center,
                y: Center,
            },
            DUMMY_SP,
        );
        let checkpoint = context.node_checkpoint();
        for bits in [0, 0x8000_0000, 1, 0x7f80_0000, 0xff80_0000, 0x7fc0_0123] {
            let value = f32::from_bits(bits);
            for x in [
                Center,
                Number(NumberOrPercentage::Number(value)),
                Number(NumberOrPercentage::Percentage(value)),
                Side(HorizontalPositionKeyword::Left),
                Side(HorizontalPositionKeyword::Right),
            ] {
                for y in [
                    Center,
                    Number(NumberOrPercentage::Number(value)),
                    Number(NumberOrPercentage::Percentage(value)),
                    Side(VerticalPositionKeyword::Top),
                    Side(VerticalPositionKeyword::Bottom),
                ] {
                    context
                        .mutate_encoded_node(id, |point, _| *point = WebKitGradientPoint { x, y });
                    let actual = context.encoded_node(id);
                    check(actual.x, x);
                    check(actual.y, y);
                    assert_eq!(context.node_checkpoint(), checkpoint);
                }
            }
        }
    }
}
