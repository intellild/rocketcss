use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum LineStyle {
    None,
    Hidden,
    Inset,
    Groove,
    Outset,
    Ridge,
    Dotted,
    Dashed,
    Solid,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum BorderSideWidth<'a> {
    Thin,
    Medium,
    Thick,
    Length(NodeId<'a, Length<'a>>),
}

impl_inline_node!(BorderSideWidth<'ast>, 0x0009_0001);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum LengthOrNumber<'a> {
    Number(f32),
    Length(NodeId<'a, Length<'a>>),
}

impl_inline_node!(LengthOrNumber<'ast>, 0x0009_0002);

impl<'ast> AstNodeClone<'ast> for LengthOrNumber<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Number(value) => Self::Number(value),
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum BorderImageSideWidth<'a> {
    Number(f32),
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    Auto,
}

impl_inline_node!(BorderImageSideWidth<'ast>, 0x0009_0003);

impl<'ast> AstNodeClone<'ast> for BorderImageSideWidth<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
            value => value,
        }
    }
}

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub enum OutlineStyle {
    Auto,
    LineStyle(LineStyle),
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, BorderImageSideWidth, BorderSideWidth, DUMMY_SP, DimensionPercentage, Length,
        LengthOrNumber, LengthUnit, LengthValue,
    };

    #[test]
    fn border_scalar_node_codecs_preserve_numbers_and_child_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let length = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 2.0,
            }),
            DUMMY_SP,
        );
        let width = context.alloc_encoded_node(BorderSideWidth::Length(length), DUMMY_SP);
        assert_eq!(context.encoded_node(width), BorderSideWidth::Length(length));

        let outset = context.alloc_encoded_node(LengthOrNumber::Number(1.25), DUMMY_SP);
        assert_eq!(context.encoded_node(outset), LengthOrNumber::Number(1.25));

        let percentage =
            context.alloc_encoded_node(DimensionPercentage::Percentage(40.0), DUMMY_SP);
        let image_width = context
            .alloc_encoded_node(BorderImageSideWidth::LengthPercentage(percentage), DUMMY_SP);
        assert_eq!(
            context.encoded_node(image_width),
            BorderImageSideWidth::LengthPercentage(percentage)
        );
    }
}
