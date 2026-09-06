use crate::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderRadius<'a> {
    pub bottom_left: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub bottom_right: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub top_left: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
    pub top_right: NodeId<'a, Size2D<'a, LengthPercentage<'a>>>,
}

impl_inline_node!(BorderRadius<'ast>, 0x000e_0001);

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

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderImageRepeat {
    pub horizontal: BorderImageRepeatKeyword,
    pub vertical: BorderImageRepeatKeyword,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderImageSlice<'a> {
    pub fill: bool,
    pub offsets: NodeId<'a, Rect<'a, NumberOrPercentage>>,
}

impl_inline_node!(BorderImageSlice<'ast>, 0x000e_0016);

impl<'ast> AstNodeClone<'ast> for BorderImageSlice<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            fill: self.fill,
            offsets: context.clone_encoded_node(self.offsets),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderImage<'a> {
    pub outset: NodeId<'a, Rect<'a, LengthOrNumber<'a>>>,
    pub repeat: BorderImageRepeat,
    pub slice: NodeId<'a, BorderImageSlice<'a>>,
    pub source: NodeId<'a, Image<'a>>,
    pub width: NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>,
}

#[derive(Clone, Copy)]
struct BorderImageHeader<'a> {
    outset: NodeId<'a, Rect<'a, LengthOrNumber<'a>>>,
    slice: NodeId<'a, BorderImageSlice<'a>>,
    repeat: BorderImageRepeat,
    extra: u32,
}

#[derive(Clone, Copy)]
struct BorderImageFields<'a> {
    source: NodeId<'a, Image<'a>>,
    width: NodeId<'a, Rect<'a, BorderImageSideWidth<'a>>>,
}

pub use border_image_access::BorderImageRead;

mod border_image_access {
    use super::*;
    pub struct BorderImageRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: BorderImageHeader<'id>,
    }
    impl<'id> BorderImageRead<'_, '_, 'id> {
        pub fn source_and_width(
            &self,
        ) -> (
            NodeId<'id, Image<'id>>,
            NodeId<'id, Rect<'id, BorderImageSideWidth<'id>>>,
        ) {
            // SAFETY: this kind owns one native BorderImageFields slot.
            let fields: BorderImageFields<'id> = unsafe {
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
        pub fn repeat(&self) -> BorderImageRepeat {
            self.header.repeat
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn border_image<'id>(
            &self,
            id: NodeId<'id, BorderImage<'id>>,
        ) -> BorderImageRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the owning kind before the native header read.
            BorderImageRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: this kind stores BorderImageHeader with a typed BorderImageFields slot.
unsafe impl<'ast> AstNodeStorage<'ast> for BorderImage<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000e_0017);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: BorderImageHeader<'ast> = unsafe { payload.read_value() };
        let fields: BorderImageFields<'ast> =
            unsafe { context.extra_slot(header.extra as usize).read_value() };
        Self {
            outset: header.outset,
            repeat: header.repeat,
            slice: header.slice,
            source: fields.source,
            width: fields.width,
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_border_image(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: BorderImageHeader<'ast> = unsafe { current.read_value() };
        encode_border_image(self, Some(header.extra as usize), context)
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
    let fields = ExtraData::from_value(BorderImageFields {
        source: value.source,
        width: value.width,
    });
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, fields);
            extra
        }
        None => context.alloc_extra_slots([fields]),
    };
    NodePayload::from_value(BorderImageHeader {
        outset: value.outset,
        repeat: value.repeat,
        slice: value.slice,
        extra: u32::try_from(extra).expect("AST extra index exceeds u32"),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderColor<'a> {
    pub bottom: NodeId<'a, CssColor<'a>>,
    pub left: NodeId<'a, CssColor<'a>>,
    pub right: NodeId<'a, CssColor<'a>>,
    pub top: NodeId<'a, CssColor<'a>>,
}

impl_inline_node!(BorderColor<'ast>, 0x000e_0002);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderStyle {
    pub bottom: LineStyle,
    pub left: LineStyle,
    pub right: LineStyle,
    pub top: LineStyle,
}

impl_inline_node!(BorderStyle, 0x000e_0003);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderWidth<'a> {
    pub bottom: NodeId<'a, BorderSideWidth<'a>>,
    pub left: NodeId<'a, BorderSideWidth<'a>>,
    pub right: NodeId<'a, BorderSideWidth<'a>>,
    pub top: NodeId<'a, BorderSideWidth<'a>>,
}

impl_inline_node!(BorderWidth<'ast>, 0x000e_0004);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderBlockColor<'a> {
    pub end: NodeId<'a, CssColor<'a>>,
    pub start: NodeId<'a, CssColor<'a>>,
}

impl_inline_node!(BorderBlockColor<'ast>, 0x000e_0005);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderBlockStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

impl_inline_node!(BorderBlockStyle, 0x000e_0006);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderBlockWidth<'a> {
    pub end: NodeId<'a, BorderSideWidth<'a>>,
    pub start: NodeId<'a, BorderSideWidth<'a>>,
}

impl_inline_node!(BorderBlockWidth<'ast>, 0x000e_0007);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderInlineColor<'a> {
    pub end: NodeId<'a, CssColor<'a>>,
    pub start: NodeId<'a, CssColor<'a>>,
}

impl_inline_node!(BorderInlineColor<'ast>, 0x000e_0008);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderInlineStyle {
    pub end: LineStyle,
    pub start: LineStyle,
}

impl_inline_node!(BorderInlineStyle, 0x000e_0009);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct BorderInlineWidth<'a> {
    pub end: NodeId<'a, BorderSideWidth<'a>>,
    pub start: NodeId<'a, BorderSideWidth<'a>>,
}

impl_inline_node!(BorderInlineWidth<'ast>, 0x000e_000a);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct GenericBorder<'a, S> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub style: S,
    pub width: NodeId<'a, BorderSideWidth<'a>>,
}

impl_inline_node!(GenericBorder<'ast, LineStyle>, 0x000e_000b);

impl_inline_node!(GenericBorder<'ast, OutlineStyle>, 0x000e_000c);

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, BorderBlockStyle, BorderSideWidth, BorderStyle, CssColor, DUMMY_SP,
        GenericBorder, LineStyle, OutlineStyle,
    };

    #[test]
    fn border_image_native_overflow_preserves_children_and_repeat_updates() {
        use crate::{
            BorderImage, BorderImageRepeat, BorderImageRepeatKeyword, BorderImageSideWidth,
            BorderImageSlice, Image, LengthOrNumber, NumberOrPercentage, Rect,
        };
        let allocator = Allocator::new();
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
            BorderImage {
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
        let repeats = [
            BorderImageRepeatKeyword::Stretch,
            BorderImageRepeatKeyword::Repeat,
            BorderImageRepeatKeyword::Round,
            BorderImageRepeatKeyword::Space,
        ];
        for (horizontal, vertical) in repeats
            .into_iter()
            .flat_map(|horizontal| repeats.map(|vertical| (horizontal, vertical)))
        {
            ast.mutate_node(node, |value, _| {
                value.repeat = BorderImageRepeat {
                    horizontal,
                    vertical,
                }
            });
            ast.mutate_node(slice, |value, _| value.fill = !value.fill);
            let value = ast.resolve_node(node);
            assert_eq!(
                value.repeat,
                BorderImageRepeat {
                    horizontal,
                    vertical
                }
            );
            let view = ast.border_image(node);
            assert_eq!(view.repeat(), value.repeat);
            assert_eq!(view.source_and_width(), (source, width));
            assert_eq!(view.slice(), slice);
            assert_eq!(view.outset(), outset);
            assert_eq!(
                (value.outset, value.slice, value.width, value.source),
                (outset, slice, width, source)
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        let clone = ast.clone_node(node);
        let cloned = ast.resolve_node(clone);
        assert_ne!(cloned.slice, slice);
        ast.mutate_node(cloned.slice, |value, _| value.fill = true);
        assert!(!ast.resolve_node(slice).fill);
    }

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
