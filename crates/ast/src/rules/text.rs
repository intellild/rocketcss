use crate::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub struct TextTransform {
    pub case: TextTransformCase,
    pub full_size_kana: bool,
    pub full_width: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct TextIndent<'a> {
    pub each_line: bool,
    pub hanging: bool,
    pub value: NodeId<'a, LengthPercentage<'a>>,
}

impl_inline_node!(TextIndent<'ast>, 0x00110001);

impl<'ast> AstNodeClone<'ast> for TextIndent<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            each_line: self.each_line,
            hanging: self.hanging,
            value: context.clone_encoded_node(self.value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct TextDecoration<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub line: NodeId<'a, TextDecorationLine<'a>>,
    pub style: TextDecorationStyle,
    pub thickness: NodeId<'a, TextDecorationThickness<'a>>,
}

impl_inline_node!(TextDecoration<'ast>, 0x00110002);

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

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct TextEmphasis<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub style: NodeId<'a, TextEmphasisStyle<'a>>,
}

impl_inline_node!(TextEmphasis<'ast>, 0x00110003);

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

#[derive(Clone, Copy)]
struct TextShadowHeader<'ast> {
    blur: NodeId<'ast, Length<'ast>>,
    color: NodeId<'ast, CssColor<'ast>>,
    x_offset: NodeId<'ast, Length<'ast>>,
    extra: u32,
}
#[derive(Clone, Copy)]
struct TextShadowFields<'ast> {
    y_offset: NodeId<'ast, Length<'ast>>,
    spread: NodeId<'ast, Length<'ast>>,
}
pub use text_shadow_access::TextShadowRead;

mod text_shadow_access {
    use super::*;

    pub struct TextShadowRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: TextShadowHeader<'id>,
    }
    impl<'id> TextShadowRead<'_, '_, 'id> {
        pub fn offsets(&self) -> [NodeId<'id, Length<'id>>; 4] {
            // SAFETY: this kind owns one native TextShadowFields slot.
            let fields: TextShadowFields<'id> = unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            };
            [
                self.header.x_offset,
                fields.y_offset,
                self.header.blur,
                fields.spread,
            ]
        }
        pub fn color(&self) -> NodeId<'id, CssColor<'id>> {
            self.header.color
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn text_shadow<'id>(
            &self,
            id: NodeId<'id, TextShadow<'id>>,
        ) -> TextShadowRead<'_, 'storage, 'id> {
            // SAFETY: node_payload validates the owning kind before the header read.
            TextShadowRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: this kind stores the native header and one typed y-offset/spread slot.
unsafe impl<'ast> AstNodeStorage<'ast> for TextShadow<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0011_0004);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: TextShadowHeader<'ast> = unsafe { payload.read_value() };
        let fields: TextShadowFields<'ast> =
            unsafe { context.extra_slot(header.extra as usize).read_value() };
        Self {
            blur: header.blur,
            color: header.color,
            x_offset: header.x_offset,
            y_offset: fields.y_offset,
            spread: fields.spread,
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        store_text_shadow(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: TextShadowHeader<'ast> = unsafe { current.read_value() };
        store_text_shadow(self, Some(header.extra as usize), context)
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

fn store_text_shadow<'ast>(
    value: TextShadow<'ast>,
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let fields = ExtraData::from_value(TextShadowFields {
        y_offset: value.y_offset,
        spread: value.spread,
    });
    let extra = match existing {
        Some(index) => {
            context.set_extra_slot(index, fields);
            index
        }
        None => context.alloc_extra_slots([fields]),
    };
    NodePayload::from_value(TextShadowHeader {
        blur: value.blur,
        color: value.color,
        x_offset: value.x_offset,
        extra: u32::try_from(extra).expect("extra index exceeds u32"),
    })
}

#[cfg(test)]
mod native_tests {
    use super::*;

    #[test]
    fn native_text_fields_preserve_flags_order_and_shadow_overflow() {
        let allocator = rocketcss_common::Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let lengths = [1.0, 2.0, 3.0, 4.0].map(|value| {
            ast.alloc_node(
                Length::Value(LengthValue {
                    unit: LengthUnit::Px,
                    value,
                }),
                DUMMY_SP,
            )
        });
        let color = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let percentage = ast.alloc_node(LengthPercentage::Percentage(25.0), DUMMY_SP);
        let indent = ast.alloc_node(
            TextIndent {
                each_line: false,
                hanging: false,
                value: percentage,
            },
            DUMMY_SP,
        );
        let lines = ast.alloc_encoded_vec(
            [
                OtherTextDecorationLine::Underline,
                OtherTextDecorationLine::Overline,
                OtherTextDecorationLine::Underline,
            ]
            .into_iter(),
        );
        let line = ast.alloc_node(TextDecorationLine::Value(lines), DUMMY_SP);
        let thickness = ast.alloc_node(TextDecorationThickness::FromFont, DUMMY_SP);
        let decoration = ast.alloc_node(
            TextDecoration {
                color,
                line,
                style: TextDecorationStyle::Solid,
                thickness,
            },
            DUMMY_SP,
        );
        let before = ast.encoded_extra_len();
        let shadow = ast.alloc_node(
            TextShadow {
                blur: lengths[0],
                color,
                spread: lengths[1],
                x_offset: lengths[2],
                y_offset: lengths[3],
            },
            DUMMY_SP,
        );
        assert_eq!(ast.encoded_extra_len(), before + 1);
        let checkpoint = ast.node_checkpoint();
        for each_line in [false, true] {
            for hanging in [false, true] {
                let expected = TextIndent {
                    each_line,
                    hanging,
                    value: percentage,
                };
                ast.mutate_node(indent, |value, _| *value = expected);
                assert_eq!(ast.resolve_node(indent), expected);
            }
        }
        for style in [
            TextDecorationStyle::Solid,
            TextDecorationStyle::Double,
            TextDecorationStyle::Dotted,
            TextDecorationStyle::Dashed,
            TextDecorationStyle::Wavy,
        ] {
            ast.mutate_node(decoration, |value, _| value.style = style);
            assert_eq!(
                ast.resolve_node(decoration),
                TextDecoration {
                    color,
                    line,
                    style,
                    thickness
                }
            );
        }
        assert_eq!(
            ast.encoded_vec_get(lines, 0),
            Some(OtherTextDecorationLine::Underline)
        );
        assert_eq!(
            ast.encoded_vec_get(lines, 1),
            Some(OtherTextDecorationLine::Overline)
        );
        assert_eq!(
            ast.encoded_vec_get(lines, 2),
            Some(OtherTextDecorationLine::Underline)
        );
        for (y_offset, spread) in [(lengths[3], lengths[1]), (lengths[1], lengths[3])] {
            ast.mutate_node(shadow, |value, _| {
                value.y_offset = y_offset;
                value.spread = spread;
            });
            assert_eq!(
                ast.resolve_node(shadow),
                TextShadow {
                    blur: lengths[0],
                    color,
                    spread,
                    x_offset: lengths[2],
                    y_offset
                }
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        let cloned = ast.clone_node(shadow);
        let cloned_spread = ast.resolve_node(cloned).spread;
        assert_ne!(cloned_spread, lengths[3]);
        ast.mutate_node(cloned_spread, |value, _| {
            *value = Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 99.0,
            })
        });
        assert_eq!(
            ast.resolve_node(lengths[3]),
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 4.0
            })
        );
    }
}
