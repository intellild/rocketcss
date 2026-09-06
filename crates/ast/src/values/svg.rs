use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum SVGPaint<'a> {
    Url {
        fallback: Option<NodeId<'a, SVGPaintFallback<'a>>>,
        url: NodeId<'a, Url<'a>>,
    },
    Color(NodeId<'a, CssColor<'a>>),
    ContextFill,
    ContextStroke,
    None,
}

impl_inline_node!(SVGPaint<'ast>, 0x00120001);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum SVGPaintFallback<'a> {
    None,
    Color(NodeId<'a, CssColor<'a>>),
}

impl_inline_node!(SVGPaintFallback<'ast>, 0x00120002);

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum StrokeLinecap {
    Butt,
    Round,
    Square,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum StrokeLinejoin {
    Miter,
    MiterClip,
    Round,
    Bevel,
    Arcs,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum StrokeDasharray<'a> {
    None,
    Values(Vec<'a, LengthPercentage<'a>>),
}

impl_inline_node!(StrokeDasharray<'ast>, 0x00120003);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Marker<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
}

impl_inline_node!(Marker<'ast>, 0x00120004);

impl<'ast> AstNodeClone<'ast> for Marker<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ColorInterpolation {
    Auto,
    Srgb,
    Linearrgb,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ColorRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ShapeRendering {
    Auto,
    Optimizespeed,
    Crispedges,
    Geometricprecision,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextRendering {
    Auto,
    Optimizespeed,
    Optimizelegibility,
    Geometricprecision,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ImageRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, DUMMY_SP, DimensionPercentage, Marker, SVGPaint, SVGPaintFallback,
        StrokeDasharray, Url,
    };

    #[test]
    fn svg_native_variants_keep_absent_and_explicit_none_distinct() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let fallback = ast.alloc_node(SVGPaintFallback::None, DUMMY_SP);
        assert_eq!(fallback.index(), 0);
        let text = ast.add_str("paint.svg#fill");
        let url = ast.alloc_node(Url { url: text }, DUMMY_SP);
        let color = ast.alloc_node(crate::CssColor::CurrentColor, DUMMY_SP);
        let paint = ast.alloc_node(SVGPaint::None, DUMMY_SP);
        let empty = ast.alloc_encoded_vec(std::iter::empty());
        let dasharray = ast.alloc_node(StrokeDasharray::None, DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for expected in [
            SVGPaint::Url {
                fallback: None,
                url,
            },
            SVGPaint::Url {
                fallback: Some(fallback),
                url,
            },
            SVGPaint::Color(color),
            SVGPaint::ContextFill,
            SVGPaint::ContextStroke,
            SVGPaint::None,
        ] {
            ast.mutate_node(paint, |value, _| *value = expected);
            assert_eq!(ast.resolve_node(paint), expected);
        }
        for expected in [StrokeDasharray::Values(empty), StrokeDasharray::None] {
            ast.mutate_node(dasharray, |value, _| *value = expected);
            assert_eq!(ast.resolve_node(dasharray), expected);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }

    #[test]
    fn svg_node_codecs_preserve_optional_children_and_compact_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("paint.svg");
        let url = context.alloc_encoded_node(Url { url: text }, DUMMY_SP);
        let fallback = context.alloc_encoded_node(SVGPaintFallback::None, DUMMY_SP);
        let paint = context.alloc_encoded_node(
            SVGPaint::Url {
                fallback: Some(fallback),
                url,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(paint),
            SVGPaint::Url {
                fallback: Some(fallback),
                url,
            }
        );

        let marker = context.alloc_encoded_node(Marker::Url(url), DUMMY_SP);
        assert_eq!(context.encoded_node(marker), Marker::Url(url));

        let values = context.alloc_encoded_vec(
            [
                DimensionPercentage::Percentage(10.0),
                DimensionPercentage::Zero,
            ]
            .into_iter(),
        );
        let dasharray = context.alloc_encoded_node(StrokeDasharray::Values(values), DUMMY_SP);
        assert_eq!(
            context.encoded_node(dasharray),
            StrokeDasharray::Values(values)
        );
    }
}
