//! SVG value parsing into the flattened AST.

use super::prelude::*;

macro_rules! keyword_parse {
    ($ty:ty, $($name:literal => $variant:expr),+ $(,)?) => {
        impl<'i> Parse<'i> for $ty {
            fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
                let ident = input.expect_ident()?;
                match_ignore_ascii_case!(
                    ident,
                    $( $name => Ok($variant), )+
                    _ => Err(input.new_custom_error(ParserError::InvalidValue)),
                )
            }
        }
    };
}

keyword_parse!(FillRule, "nonzero" => Self::Nonzero, "evenodd" => Self::Evenodd,);
keyword_parse!(
    StrokeLinecap,
    "butt" => Self::Butt,
    "round" => Self::Round,
    "square" => Self::Square,
);
keyword_parse!(
    StrokeLinejoin,
    "miter" => Self::Miter,
    "miter-clip" => Self::MiterClip,
    "round" => Self::Round,
    "bevel" => Self::Bevel,
    "arcs" => Self::Arcs,
);
keyword_parse!(
    ColorInterpolation,
    "auto" => Self::Auto,
    "srgb" => Self::Srgb,
    "linearrgb" => Self::Linearrgb,
);
keyword_parse!(
    ColorRendering,
    "auto" => Self::Auto,
    "optimizespeed" => Self::Optimizespeed,
    "optimizequality" => Self::Optimizequality,
);
keyword_parse!(
    ShapeRendering,
    "auto" => Self::Auto,
    "optimizespeed" => Self::Optimizespeed,
    "crispedges" => Self::Crispedges,
    "geometricprecision" => Self::Geometricprecision,
);
keyword_parse!(
    TextRendering,
    "auto" => Self::Auto,
    "optimizespeed" => Self::Optimizespeed,
    "optimizelegibility" => Self::Optimizelegibility,
    "geometricprecision" => Self::Geometricprecision,
);
keyword_parse!(
    ImageRendering,
    "auto" => Self::Auto,
    "optimizespeed" => Self::Optimizespeed,
    "optimizequality" => Self::Optimizequality,
);

impl<'i> Parse<'i> for SVGPaint<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = DUMMY_SP;
        let state = input.state();
        if let Ok(value) = input.try_parse(parse_svg_paint_keyword) {
            return Ok(value);
        }
        input.reset(&state);

        if let Ok(url) = input.try_parse(parse_url) {
            let fallback = input.try_parse(parse_svg_paint_fallback).ok();
            return Ok(input.ast_mut().alloc_svg_paint(
                SVGPaintData::Url { fallback, url },
                span,
            ));
        }
        input.reset(&state);

        let color = CssColor::parse(input)?;
        Ok(input
            .ast_mut()
            .alloc_svg_paint(SVGPaintData::Color(color), span))
    }
}

fn parse_svg_paint_keyword<'i>(
    input: &mut Compiler<'i>,
) -> Result<SVGPaint<'i>, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    let data = match_ignore_ascii_case!(
        ident,
        "none" => SVGPaintData::None,
        "context-fill" => SVGPaintData::ContextFill,
        "context-stroke" => SVGPaintData::ContextStroke,
        _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
    );
    Ok(input.ast_mut().alloc_svg_paint(data, DUMMY_SP))
}

fn parse_svg_paint_fallback<'i>(
    input: &mut Compiler<'i>,
) -> Result<SVGPaintFallback<'i>, ParseError<'i, ParserError<'i>>> {
    let span = DUMMY_SP;
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(input
            .ast_mut()
            .alloc_svg_paint_fallback(SVGPaintFallbackData::None, span));
    }
    let color = CssColor::parse(input)?;
    Ok(input
        .ast_mut()
        .alloc_svg_paint_fallback(SVGPaintFallbackData::Color(color), span))
}

fn parse_url<'i>(
    input: &mut Compiler<'i>,
) -> Result<Url<'i>, ParseError<'i, ParserError<'i>>> {
    let span = DUMMY_SP;
    let url = match input.next()?.clone() {
        OldToken::UnquotedUrl(url) => url,
        OldToken::Function(name) if name.eq_ignore_ascii_case("url") => input
            .parse_nested_block(|input| {
                let url = match input.next()?.clone() {
                    OldToken::String(url) | OldToken::UnquotedUrl(url) => url,
                    _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
                };
                input.expect_exhausted()?;
                Ok(url)
            })?,
        _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
    };
    Ok(input.ast_mut().alloc_url(UrlData { url }, span))
}

impl<'i> Parse<'i> for StrokeDasharray<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = DUMMY_SP;
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(input
                .ast_mut()
                .alloc_stroke_dasharray(StrokeDasharrayData::None, span));
        }

        let mut values = Vec::new();
        loop {
            if !values.is_empty()
                && input.try_parse(Compiler::expect_comma).is_ok()
                && input.is_exhausted()
            {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            values.push(LengthPercentage::parse(input)?);
            if input.is_exhausted() {
                break;
            }
        }
        let values = input.ast().alloc_slice(&values);
        Ok(input
            .ast_mut()
            .alloc_stroke_dasharray(StrokeDasharrayData::Values(values), span))
    }
}

impl<'i> Parse<'i> for Marker<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = DUMMY_SP;
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(input.ast_mut().alloc_marker(MarkerData::None, span));
        }
        let url = parse_url(input)?;
        Ok(input.ast_mut().alloc_marker(MarkerData::Url(url), span))
    }
}
