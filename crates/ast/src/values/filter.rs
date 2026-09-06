use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum FilterList<'a> {
    None,
    Filters(Vec<'a, NodeId<'a, Filter<'a>>>),
}

impl_inline_node!(FilterList<'ast>, 0x0022_0001);

impl<'ast> AstNodeClone<'ast> for FilterList<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Filters(values) => Self::Filters(context.clone_encoded_vec(values)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Filter<'a> {
    Blur(NodeId<'a, Length<'a>>),
    Brightness(NumberOrPercentage),
    Contrast(NumberOrPercentage),
    Grayscale(NumberOrPercentage),
    HueRotate(Angle),
    Invert(NumberOrPercentage),
    Opacity(NumberOrPercentage),
    Saturate(NumberOrPercentage),
    Sepia(NumberOrPercentage),
    DropShadow(NodeId<'a, DropShadow<'a>>),
    Url(NodeId<'a, Url<'a>>),
}

impl_inline_node!(Filter<'ast>, 0x0022_0002);

impl<'ast> AstNodeClone<'ast> for Filter<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Blur(value) => Self::Blur(context.clone_encoded_node(value)),
            Self::DropShadow(value) => Self::DropShadow(context.clone_encoded_node(value)),
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[cfg(test)]
mod native_tests {
    use super::*;
    #[test]
    fn filter_native_values_preserve_units_variants_and_empty_lists() {
        let allocator = rocketcss_common::Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let length = ast.alloc_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 3.0,
            }),
            DUMMY_SP,
        );
        let color = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let shadow = ast.alloc_node(
            DropShadow {
                blur: length,
                color,
                x_offset: length,
                y_offset: length,
            },
            DUMMY_SP,
        );
        let url = ast.add_str("filter.svg#effect");
        let url = ast.alloc_node(Url { url }, DUMMY_SP);
        let node = ast.alloc_node(Filter::Blur(length), DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for value in [
            Filter::Blur(length),
            Filter::Brightness(NumberOrPercentage::Number(0.5)),
            Filter::Contrast(NumberOrPercentage::Percentage(50.0)),
            Filter::Grayscale(NumberOrPercentage::Number(0.25)),
            Filter::HueRotate(Angle::Rad(0.5)),
            Filter::Invert(NumberOrPercentage::Percentage(25.0)),
            Filter::Opacity(NumberOrPercentage::Number(1.0)),
            Filter::Saturate(NumberOrPercentage::Percentage(200.0)),
            Filter::Sepia(NumberOrPercentage::Number(0.75)),
            Filter::DropShadow(shadow),
            Filter::Url(url),
        ] {
            ast.mutate_node(node, |actual, _| *actual = value);
            assert_eq!(ast.resolve_node(node), value);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        let empty = ast.alloc_encoded_vec(std::iter::empty());
        let list = ast.alloc_node(FilterList::None, DUMMY_SP);
        ast.mutate_node(list, |value, _| *value = FilterList::Filters(empty));
        assert_eq!(ast.resolve_node(list), FilterList::Filters(empty));
        ast.mutate_node(list, |value, _| *value = FilterList::None);
        assert_eq!(ast.resolve_node(list), FilterList::None);
    }
}
