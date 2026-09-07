use super::*;

keyword_values! {
    FillRule,
    StrokeLinecap,
    StrokeLinejoin,
    ColorInterpolation,
    ColorRendering,
    ShapeRendering,
    TextRendering,
    ImageRendering,
}

impl<'ghost> ToCss<'ghost> for SVGPaint<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Url { fallback, url } => {
                url.to_css(dest, _cx)?;
                if let Some(fallback) = fallback {
                    dest.write_char(' ')?;
                    fallback.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::Color(value) => value.to_css(dest, _cx),
            Self::ContextFill => dest.write_str("context-fill"),
            Self::ContextStroke => dest.write_str("context-stroke"),
            Self::None => dest.write_str("none"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for SVGPaintFallback<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Color(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for StrokeDasharray<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Values(values) => {
                for (index, value) in _cx.ast_context().vec_iter(*values).enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Marker<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest, _cx),
        }
    }
}
