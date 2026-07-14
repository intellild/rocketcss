use crate::*;

#[derive(Debug, PartialEq)]
pub enum SVGPaint<'a> {
    Url {
        fallback: Option<Box<'a, SVGPaintFallback<'a>>>,
        url: Box<'a, Url<'a>>,
    },
    Color(Box<'a, CssColor<'a>>),
    ContextFill,
    ContextStroke,
    None,
}

#[derive(Debug, PartialEq)]
pub enum SVGPaintFallback<'a> {
    None,
    Color(Box<'a, CssColor<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum StrokeLinecap {
    Butt,
    Round,
    Square,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum StrokeLinejoin {
    Miter,
    MiterClip,
    Round,
    Bevel,
    Arcs,
}

#[derive(Debug, PartialEq)]
pub enum StrokeDasharray<'a> {
    None,
    Values(Vec<'a, LengthPercentage<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Marker<'a> {
    None,
    Url(Box<'a, Url<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ColorInterpolation {
    Auto,
    Srgb,
    Linearrgb,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ColorRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ShapeRendering {
    Auto,
    Optimizespeed,
    Crispedges,
    Geometricprecision,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum TextRendering {
    Auto,
    Optimizespeed,
    Optimizelegibility,
    Geometricprecision,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum ImageRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}
