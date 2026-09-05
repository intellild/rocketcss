use crate::*;

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

#[derive(Debug, PartialEq, Visit)]
pub enum SVGPaintFallback<'a> {
    None,
    Color(NodeId<'a, CssColor<'a>>),
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

#[derive(Debug, PartialEq, Visit)]
pub enum Marker<'a> {
    None,
    Url(NodeId<'a, Url<'a>>),
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
