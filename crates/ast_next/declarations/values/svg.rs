#[ast]
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

#[ast]
pub enum SVGPaintFallback<'a> {
    None,
    Color(Box<'a, CssColor<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum StrokeLinecap {
    Butt,
    Round,
    Square,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum StrokeLinejoin {
    Miter,
    MiterClip,
    Round,
    Bevel,
    Arcs,
}

#[ast]
pub enum StrokeDasharray<'a> {
    None,
    Values(Vec<'a, LengthPercentage<'a>>),
}

#[ast]
pub enum Marker<'a> {
    None,
    Url(Box<'a, Url<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ColorInterpolation {
    Auto,
    Srgb,
    Linearrgb,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ColorRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ShapeRendering {
    Auto,
    Optimizespeed,
    Crispedges,
    Geometricprecision,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TextRendering {
    Auto,
    Optimizespeed,
    Optimizelegibility,
    Geometricprecision,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ImageRendering {
    Auto,
    Optimizespeed,
    Optimizequality,
}
