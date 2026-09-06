use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct InsetRect<'a> {
    pub radius: NodeId<'a, BorderRadius<'a>>,
    pub rect: NodeId<'a, Rect<'a, LengthPercentage<'a>>>,
}

impl_inline_node!(InsetRect<'ast>, 0x001d0001);

impl<'ast> AstNodeClone<'ast> for InsetRect<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            radius: context.clone_encoded_node(self.radius),
            rect: context.clone_encoded_node(self.rect),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct CircleShape<'a> {
    pub position: NodeId<'a, Position<'a>>,
    pub radius: NodeId<'a, ShapeRadius<'a>>,
}

impl_inline_node!(CircleShape<'ast>, 0x001d0002);

impl<'ast> AstNodeClone<'ast> for CircleShape<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            position: context.clone_encoded_node(self.position),
            radius: context.clone_encoded_node(self.radius),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct EllipseShape<'a> {
    pub position: NodeId<'a, Position<'a>>,
    pub radius_x: NodeId<'a, ShapeRadius<'a>>,
    pub radius_y: NodeId<'a, ShapeRadius<'a>>,
}

impl_inline_node!(EllipseShape<'ast>, 0x001d0003);

impl<'ast> AstNodeClone<'ast> for EllipseShape<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            position: context.clone_encoded_node(self.position),
            radius_x: context.clone_encoded_node(self.radius_x),
            radius_y: context.clone_encoded_node(self.radius_y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Polygon<'a> {
    pub fill_rule: FillRule,
    pub points: Vec<'a, Point<'a>>,
}

impl_inline_node!(Polygon<'ast>, 0x001d0004);

impl<'ast> AstNodeClone<'ast> for Polygon<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            fill_rule: self.fill_rule,
            points: context.clone_encoded_vec(self.points),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Point<'a> {
    pub x: NodeId<'a, LengthPercentage<'a>>,
    pub y: NodeId<'a, LengthPercentage<'a>>,
}

impl_inline_extra!(Point<'ast>);

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

#[derive(Clone, Copy)]
struct MaskHeader<'a> {
    image: NodeId<'a, Image<'a>>,
    position: NodeId<'a, Position<'a>>,
    size: NodeId<'a, BackgroundSize<'a>>,
    extra: u32,
}

#[derive(Clone, Copy)]
struct MaskFields {
    clip: MaskClip,
    composite: MaskComposite,
    mode: MaskMode,
    origin: GeometryBox,
    repeat: BackgroundRepeat,
}

pub use mask_access::{MaskKeywordsRead, MaskRead};

mod mask_access {
    use super::*;
    pub struct MaskRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: MaskHeader<'id>,
    }
    pub struct MaskKeywordsRead(MaskFields);
    impl MaskKeywordsRead {
        pub fn clip(&self) -> MaskClip {
            self.0.clip
        }
        pub fn composite(&self) -> MaskComposite {
            self.0.composite
        }
        pub fn mode(&self) -> MaskMode {
            self.0.mode
        }
        pub fn origin(&self) -> GeometryBox {
            self.0.origin
        }
        pub fn repeat(&self) -> BackgroundRepeat {
            self.0.repeat
        }
    }
    impl<'id> MaskRead<'_, '_, 'id> {
        pub fn image(&self) -> NodeId<'id, Image<'id>> {
            self.header.image
        }
        pub fn position(&self) -> NodeId<'id, Position<'id>> {
            self.header.position
        }
        pub fn size(&self) -> NodeId<'id, BackgroundSize<'id>> {
            self.header.size
        }
        pub fn keywords(&self) -> MaskKeywordsRead {
            // SAFETY: Mask owns one extra slot written as native MaskFields.
            MaskKeywordsRead(unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            })
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn mask<'id>(&self, id: NodeId<'id, Mask<'id>>) -> MaskRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the owning kind before reading its native header.
            MaskRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

unsafe impl<'ast> AstNodeStorage<'ast> for Mask<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0007);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: MaskHeader<'ast> = unsafe { payload.read_value() };
        let fields: MaskFields = unsafe { context.extra_slot(header.extra as usize).read_value() };
        Self {
            clip: fields.clip,
            composite: fields.composite,
            image: header.image,
            mode: fields.mode,
            origin: fields.origin,
            position: header.position,
            repeat: fields.repeat,
            size: header.size,
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_mask(self, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_mask(
            self,
            Some(unsafe { current.read_value::<MaskHeader>() }.extra as usize),
            context,
        )
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
    let fields = ExtraData::from_value(MaskFields {
        clip: value.clip,
        composite: value.composite,
        mode: value.mode,
        origin: value.origin,
        repeat: value.repeat,
    });
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, fields);
            extra
        }
        None => context.alloc_extra_slots([fields]),
    };
    NodePayload::from_value(MaskHeader {
        image: value.image,
        position: value.position,
        size: value.size,
        extra: u32::try_from(extra).expect("AST extra index exceeds u32"),
    })
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

#[derive(Clone, Copy)]
struct MaskBorderHeader<'ast> {
    outset: NodeId<'ast, Rect<'ast, LengthOrNumber<'ast>>>,
    slice: NodeId<'ast, BorderImageSlice<'ast>>,
    extra: u32,
    mode: MaskBorderMode,
    repeat: BorderImageRepeat,
}

#[derive(Clone, Copy)]
struct MaskBorderFields<'ast> {
    source: NodeId<'ast, Image<'ast>>,
    width: NodeId<'ast, Rect<'ast, BorderImageSideWidth<'ast>>>,
}

pub use mask_border_access::MaskBorderRead;

mod mask_border_access {
    use super::*;
    pub struct MaskBorderRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: MaskBorderHeader<'id>,
    }
    impl<'id> MaskBorderRead<'_, '_, 'id> {
        pub fn source_and_width(
            &self,
        ) -> (
            NodeId<'id, Image<'id>>,
            NodeId<'id, Rect<'id, BorderImageSideWidth<'id>>>,
        ) {
            // SAFETY: this kind owns one native MaskBorderFields slot.
            let fields: MaskBorderFields<'id> = unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            };
            (fields.source, fields.width)
        }
        pub fn slice(&self) -> NodeId<'id, BorderImageSlice<'id>> {
            self.header.slice
        }
        pub fn outset(&self) -> NodeId<'id, Rect<'id, LengthOrNumber<'id>>> {
            self.header.outset
        }
        pub fn mode(&self) -> MaskBorderMode {
            self.header.mode
        }
        pub fn repeat(&self) -> BorderImageRepeat {
            self.header.repeat
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn mask_border<'id>(
            &self,
            id: NodeId<'id, MaskBorder<'id>>,
        ) -> MaskBorderRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the owning kind before the native header read.
            MaskBorderRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: this kind stores a native header and one typed source/width slot.
unsafe impl<'ast> AstNodeStorage<'ast> for MaskBorder<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001d_0005);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: MaskBorderHeader<'ast> = unsafe { payload.read_value() };
        let fields: MaskBorderFields<'ast> =
            unsafe { context.extra_slot(header.extra as usize).read_value() };
        Self {
            mode: header.mode,
            outset: header.outset,
            repeat: header.repeat,
            slice: header.slice,
            source: fields.source,
            width: fields.width,
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        store_mask_border(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: MaskBorderHeader<'ast> = unsafe { current.read_value() };
        store_mask_border(self, Some(header.extra as usize), context)
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

fn store_mask_border<'ast>(
    value: MaskBorder<'ast>,
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let fields = ExtraData::from_value(MaskBorderFields {
        source: value.source,
        width: value.width,
    });
    let extra = match existing {
        Some(index) => {
            context.set_extra_slot(index, fields);
            index
        }
        None => context.alloc_extra_slots([fields]),
    };
    NodePayload::from_value(MaskBorderHeader {
        outset: value.outset,
        slice: value.slice,
        mode: value.mode,
        repeat: value.repeat,
        extra: u32::try_from(extra).expect("extra index exceeds u32"),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct DropShadow<'a> {
    pub blur: NodeId<'a, Length<'a>>,
    pub color: NodeId<'a, CssColor<'a>>,
    pub x_offset: NodeId<'a, Length<'a>>,
    pub y_offset: NodeId<'a, Length<'a>>,
}

impl_inline_node!(DropShadow<'ast>, 0x001d0006);

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

#[cfg(test)]
mod native_mask_tests {
    use super::*;

    #[test]
    fn mask_border_native_overflow_preserves_modes_and_children() {
        use crate::{
            BorderImageRepeat, BorderImageRepeatKeyword, BorderImageSideWidth, BorderImageSlice,
            Image, LengthOrNumber, MaskBorder, MaskBorderMode, NumberOrPercentage, Rect,
        };
        let allocator = rocketcss_common::Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let n = ast.alloc_node(LengthOrNumber::Number(0.0), DUMMY_SP);
        let outset = ast.alloc_node(Rect(n, n, n, n), DUMMY_SP);
        let n = ast.alloc_node(NumberOrPercentage::Number(1.0), DUMMY_SP);
        let offsets = ast.alloc_node(Rect(n, n, n, n), DUMMY_SP);
        let slice = ast.alloc_node(
            BorderImageSlice {
                fill: false,
                offsets,
            },
            DUMMY_SP,
        );
        let n = ast.alloc_node(BorderImageSideWidth::Auto, DUMMY_SP);
        let width = ast.alloc_node(Rect(n, n, n, n), DUMMY_SP);
        let source = ast.alloc_node(Image::None, DUMMY_SP);
        let before = ast.encoded_extra_len();
        let node = ast.alloc_node(
            MaskBorder {
                mode: MaskBorderMode::Alpha,
                outset,
                slice,
                width,
                source,
                repeat: BorderImageRepeat {
                    horizontal: BorderImageRepeatKeyword::Stretch,
                    vertical: BorderImageRepeatKeyword::Repeat,
                },
            },
            DUMMY_SP,
        );
        assert_eq!(ast.encoded_extra_len(), before + 1);
        let checkpoint = ast.node_checkpoint();
        for horizontal in [
            BorderImageRepeatKeyword::Stretch,
            BorderImageRepeatKeyword::Repeat,
            BorderImageRepeatKeyword::Round,
            BorderImageRepeatKeyword::Space,
        ] {
            ast.mutate_node(node, |value, _| value.repeat.horizontal = horizontal);
            ast.mutate_node(slice, |value, _| value.fill = !value.fill);
            let value = ast.resolve_node(node);
            assert_eq!(value.repeat.horizontal, horizontal);
            assert_eq!(
                (value.outset, value.slice, value.width, value.source),
                (outset, slice, width, source)
            );
        }
        for mode in [MaskBorderMode::Luminance, MaskBorderMode::Alpha] {
            for vertical in [
                BorderImageRepeatKeyword::Stretch,
                BorderImageRepeatKeyword::Repeat,
                BorderImageRepeatKeyword::Round,
                BorderImageRepeatKeyword::Space,
            ] {
                ast.mutate_node(node, |value, _| {
                    value.mode = mode;
                    value.repeat.vertical = vertical;
                });
                let value = ast.resolve_node(node);
                assert_eq!(value.mode, mode);
                let view = ast.mask_border(node);
                assert_eq!(view.mode(), mode);
                assert_eq!(view.repeat(), value.repeat);
                assert_eq!(view.source_and_width(), (source, width));
                assert_eq!(view.slice(), slice);
                assert_eq!(view.outset(), outset);
                assert_eq!(value.repeat.vertical, vertical);
                assert_eq!(
                    (value.outset, value.slice, value.width, value.source),
                    (outset, slice, width, source)
                );
            }
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        let clone = ast.clone_node(node);
        let cloned = ast.resolve_node(clone);
        assert_ne!(cloned.slice, slice);
        ast.mutate_node(cloned.slice, |value, _| value.fill = true);
        assert!(!ast.resolve_node(slice).fill);
    }

    #[test]
    fn native_polygon_points_keep_order_and_clone_children() {
        let allocator = rocketcss_common::Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let x = ast.alloc_node(LengthPercentage::Percentage(25.0), DUMMY_SP);
        let y = ast.alloc_node(LengthPercentage::Percentage(75.0), DUMMY_SP);
        let points = ast.alloc_encoded_vec([Point { x, y }, Point { x: y, y: x }].into_iter());
        let before = ast.encoded_extra_len();
        let polygon = ast.alloc_node(
            Polygon {
                fill_rule: FillRule::Evenodd,
                points,
            },
            DUMMY_SP,
        );
        let shape = ast.alloc_node(BasicShape::Polygon(polygon), DUMMY_SP);
        let clip = ast.alloc_node(
            ClipPath::Shape {
                reference_box: GeometryBox::ViewBox,
                shape,
            },
            DUMMY_SP,
        );
        assert_eq!(ast.encoded_extra_len(), before);
        let checkpoint = ast.node_checkpoint();
        for fill_rule in [FillRule::Nonzero, FillRule::Evenodd] {
            ast.mutate_node(polygon, |value, _| value.fill_rule = fill_rule);
            let value = ast.resolve_node(polygon);
            assert_eq!(value.fill_rule, fill_rule);
            assert_eq!(ast.encoded_vec_get(value.points, 0), Some(Point { x, y }));
            assert_eq!(
                ast.encoded_vec_get(value.points, 1),
                Some(Point { x: y, y: x })
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        let cloned = ast.clone_node(clip);
        let ClipPath::Shape {
            shape: cloned_shape,
            reference_box,
        } = ast.resolve_node(cloned)
        else {
            panic!("expected clip shape")
        };
        assert_eq!(reference_box, GeometryBox::ViewBox);
        let BasicShape::Polygon(cloned_polygon) = ast.resolve_node(cloned_shape) else {
            panic!("expected polygon")
        };
        let cloned_points = ast.resolve_node(cloned_polygon).points;
        let cloned_point = ast.encoded_vec_get(cloned_points, 0).unwrap();
        assert_ne!(cloned_point.x, x);
        assert_ne!(cloned_point.y, y);
        ast.mutate_node(cloned_point.x, |value, _| {
            *value = LengthPercentage::Percentage(50.0)
        });
        assert_eq!(ast.resolve_node(x), LengthPercentage::Percentage(25.0));
        assert_eq!(ast.resolve_node(y), LengthPercentage::Percentage(75.0));
    }

    #[test]
    fn mask_native_fields_reuse_overflow_during_variant_changes() {
        let allocator = rocketcss_common::Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let image = ast.alloc_node(Image::None, DUMMY_SP);
        let x = ast.alloc_node(PositionComponent::Center, DUMMY_SP);
        let y = ast.alloc_node(PositionComponent::Center, DUMMY_SP);
        let position = ast.alloc_node(Position { x, y }, DUMMY_SP);
        let size = ast.alloc_node(BackgroundSize::Cover, DUMMY_SP);
        let node = ast.alloc_node(
            Mask {
                clip: MaskClip::NoClip,
                composite: MaskComposite::Add,
                image,
                mode: MaskMode::MatchSource,
                origin: GeometryBox::BorderBox,
                position,
                repeat: BackgroundRepeat {
                    x: BackgroundRepeatKeyword::Repeat,
                    y: BackgroundRepeatKeyword::NoRepeat,
                },
                size,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        let boxes = [
            GeometryBox::BorderBox,
            GeometryBox::PaddingBox,
            GeometryBox::ContentBox,
            GeometryBox::MarginBox,
            GeometryBox::FillBox,
            GeometryBox::StrokeBox,
            GeometryBox::ViewBox,
        ];
        for clip in std::iter::once(MaskClip::NoClip).chain(boxes.map(MaskClip::GeometryBox)) {
            for origin in boxes {
                for mode in [MaskMode::Luminance, MaskMode::Alpha, MaskMode::MatchSource] {
                    for composite in [
                        MaskComposite::Add,
                        MaskComposite::Subtract,
                        MaskComposite::Intersect,
                        MaskComposite::Exclude,
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
                                ast.mutate_node(node, |value, _| {
                                    value.clip = clip;
                                    value.origin = origin;
                                    value.mode = mode;
                                    value.composite = composite;
                                    value.repeat = repeat;
                                });
                                assert_eq!(
                                    ast.resolve_node(node),
                                    Mask {
                                        clip,
                                        origin,
                                        mode,
                                        composite,
                                        repeat,
                                        image,
                                        position,
                                        size,
                                    }
                                );
                                let view = ast.mask(node);
                                assert_eq!(view.image(), image);
                                assert_eq!(view.position(), position);
                                assert_eq!(view.size(), size);
                                let keywords = view.keywords();
                                assert_eq!(keywords.clip(), clip);
                                assert_eq!(keywords.origin(), origin);
                                assert_eq!(keywords.mode(), mode);
                                assert_eq!(keywords.composite(), composite);
                                assert_eq!(keywords.repeat(), repeat);
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }
}
