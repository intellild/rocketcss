use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
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

impl<'ast> AstNodeStorage<'ast> for SVGPaint<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0012_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Url {
                fallback: read_optional_node_id(&bytes, 4, context),
                url: read_node_id(&bytes, 8, context),
            },
            1 => Self::Color(read_node_id(&bytes, 4, context)),
            2 => Self::ContextFill,
            3 => Self::ContextStroke,
            4 => Self::None,
            _ => panic!("invalid encoded SVGPaint variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Url { fallback, url } => {
                bytes[0] = 0;
                write_optional_node_id(&mut bytes, 4, fallback);
                write_node_id(&mut bytes, 8, url);
            }
            Self::Color(value) => {
                bytes[0] = 1;
                write_node_id(&mut bytes, 4, value);
            }
            Self::ContextFill => bytes[0] = 2,
            Self::ContextStroke => bytes[0] = 3,
            Self::None => bytes[0] = 4,
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum SVGPaintFallback<'a> {
    None,
    Color(NodeId<'a, CssColor<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for SVGPaintFallback<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0012_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Color(read_node_id(&bytes, 4, context)),
            _ => panic!("invalid encoded SVGPaintFallback variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::None => bytes[0] = 0,
            Self::Color(value) => {
                bytes[0] = 1;
                write_node_id(&mut bytes, 4, value);
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(Debug, PartialEq, Visit)]
pub enum StrokeDasharray<'a> {
    None,
    Values(Vec<'a, LengthPercentage<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for StrokeDasharray<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0012_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Values(
                context
                    .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
            ),
            _ => panic!("invalid encoded StrokeDasharray variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::None => bytes[0] = 0,
            Self::Values(values) => {
                bytes[0] = 1;
                write_u32(
                    &mut bytes,
                    4,
                    u32::try_from(values.start_index()).expect("AST range exceeds four bytes"),
                );
                write_u32(
                    &mut bytes,
                    8,
                    u32::try_from(values.end_index()).expect("AST range exceeds four bytes"),
                );
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum Marker<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for Marker<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0012_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::None,
            1 => Self::Url(read_node_id(&bytes, 4, context)),
            _ => panic!("invalid encoded Marker variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::None => bytes[0] = 0,
            Self::Url(value) => {
                bytes[0] = 1;
                write_node_id(&mut bytes, 4, value);
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for Marker<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
        }
    }
}

fn write_node_id<T>(bytes: &mut [u8], offset: usize, value: NodeId<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(value.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn write_optional_node_id<T>(bytes: &mut [u8], offset: usize, value: Option<NodeId<'_, T>>) {
    write_u32(
        bytes,
        offset,
        value.map_or(u32::MAX, |value| {
            u32::try_from(value.index()).expect("AST node ID exceeds four bytes")
        }),
    );
}

fn read_node_id<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, offset) as usize)
}

fn read_optional_node_id<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> Option<NodeId<'ast, T>> {
    let value = read_u32(bytes, offset);
    (value != u32::MAX).then(|| context.encoded_node_id_at(value as usize))
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
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
    fn svg_node_codecs_preserve_optional_children_and_compact_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let url = context.alloc_encoded_node(Url { url: "paint.svg" }, DUMMY_SP);
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
